const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json";
const previewManifestMaterializerPath =
  "tools/launch/materialize-www-template.ts";
const lockBackedBackendPlatformSourceFiles = [
  "examples/template/lib/supabase/metadata.ts",
  "examples/template/lib/supabase/README.md",
  "examples/template/lib/supabase/schema.sql",
  "examples/template/server/supabase/readiness.ts",
  "examples/template/app/api/supabase/readiness/route.ts",
];
const lockBackedBackendPlatformSurfaceFiles = [
  "lib/supabase/metadata.ts",
  "lib/supabase/README.md",
  "lib/supabase/schema.sql",
  "server/supabase/readiness.ts",
  "app/api/supabase/readiness/route.ts",
];
const statusVocabulary = [
  "present",
  "stale",
  "missing-receipt",
  "blocked",
  "unsupported-surface",
];

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function sha256(relativePath) {
  return crypto.createHash("sha256").update(read(relativePath)).digest("hex");
}

test("Backend Platform Client receipt hashes are consumed by the shared package-status read model", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const receipt = readJson(receiptPath);
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const packageDoc = read("docs/packages/supabase-client.md");
  const profileWorkflow = read(
    "examples/template/supabase-profile-workflow.tsx",
  );
  const dataStatus = read("examples/template/data-status.tsx");
  const backendReadModelBlock = readModel.slice(
    readModel.indexOf("export const backendPlatformClientPackageVisibility"),
    readModel.indexOf("const webAssemblyBridgeFileHashes"),
  );

  const backendPlatformVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "supabase/client",
  );

  assert.ok(
    backendPlatformVisibility,
    "Backend Platform Client visibility row is missing",
  );
  assert.equal(
    backendPlatformVisibility.official_package_name,
    "Backend Platform Client",
  );
  assert.equal(
    backendPlatformVisibility.upstream_package,
    "@supabase/ssr + @supabase/supabase-js",
  );
  assert.equal(
    backendPlatformVisibility.upstream_version,
    "@supabase/ssr latest; @supabase/supabase-js ^2",
  );
  assert.equal(
    backendPlatformVisibility.source_mirror,
    "G:/WWW/inspirations/supabase",
  );
  assert.equal(backendPlatformVisibility.status, "present");
  assert.equal(backendPlatformVisibility.receipt_status, "present");
  assert.equal(backendPlatformVisibility.package_receipt_path, receiptPath);
  assert.deepEqual(backendPlatformVisibility.status_vocabulary, statusVocabulary);
  assert.deepEqual(
    receipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    statusVocabulary,
  );

  for (const surfaceId of [
    "supabase-profile-workflow",
    "supabase-schema-query-workflow",
  ]) {
    assert.ok(
      backendPlatformVisibility.selected_surfaces.some(
        (surface) =>
          surface.surface_id === surfaceId && surface.receipt_path === receiptPath,
      ),
      `${surfaceId} missing from Backend Platform Client visibility row`,
    );
  }

  const profileSurface = backendPlatformVisibility.selected_surfaces.find(
    (surface) => surface.surface_id === "supabase-profile-workflow",
  );
  const materializerSurface = backendPlatformVisibility.selected_surfaces.find(
    (surface) =>
      surface.surface_id === "backend-platform-client-preview-manifest-materializer",
  );
  const lockBackedSourceSurface =
    backendPlatformVisibility.selected_surfaces.find(
      (surface) =>
        surface.surface_id === "backend-platform-client-lock-backed-source",
    );

  assert.ok(profileSurface, "Supabase profile workflow surface is missing");
  assert.equal(profileSurface.hash_algorithm, "sha256");
  const profileSurfaceHashes = { ...receipt.file_hashes };
  delete profileSurfaceHashes[previewManifestMaterializerPath];
  for (const selectedFile of lockBackedBackendPlatformSourceFiles) {
    delete profileSurfaceHashes[selectedFile];
  }
  assert.deepEqual(profileSurface.file_hashes, profileSurfaceHashes);
  assert.ok(
    materializerSurface,
    "Backend Platform Client preview-manifest materializer surface is missing",
  );
  assert.equal(materializerSurface.surface_type, "preview_manifest_materializer");
  assert.equal(materializerSurface.hash_algorithm, "sha256");
  assert.deepEqual(materializerSurface.files, [previewManifestMaterializerPath]);
  assert.deepEqual(materializerSurface.file_hashes, {
    [previewManifestMaterializerPath]:
      receipt.file_hashes[previewManifestMaterializerPath],
  });
  assert.ok(
    lockBackedSourceSurface,
    "Backend Platform Client lock-backed source surface is missing",
  );
  assert.equal(lockBackedSourceSurface.surface_type, "lock_backed_source");
  assert.equal(lockBackedSourceSurface.hash_algorithm, "sha256");
  assert.equal(lockBackedSourceSurface.runtime_proof, false);
  assert.deepEqual(
    lockBackedSourceSurface.files,
    lockBackedBackendPlatformSurfaceFiles,
  );
  assert.deepEqual(
    lockBackedSourceSurface.file_hashes,
    Object.fromEntries(
      lockBackedBackendPlatformSourceFiles.map((selectedFile) => [
        selectedFile,
        receipt.file_hashes[selectedFile],
      ]),
    ),
  );
  for (const marker of [
    "dx.www.template.supabase_readiness",
    "runtimeProof: false",
    "networkCalls: false",
    "hostedCredentials: false",
    "readBackendPlatformClientReadiness",
    "createBackendPlatformClientReadinessResponse",
  ]) {
    assert.ok(
      lockBackedSourceSurface.source_markers.includes(marker),
      `${marker} missing from Backend Platform Client lock-backed source markers`,
    );
  }

  const selectedSurfaceFiles = new Set(
    backendPlatformVisibility.selected_surfaces.flatMap(
      (surface) => surface.files ?? [],
    ),
  );
  const selectedSurfaceHashes = Object.assign(
    {},
    ...backendPlatformVisibility.selected_surfaces.map(
      (surface) => surface.file_hashes ?? {},
    ),
  );
  assert.deepEqual(selectedSurfaceHashes, receipt.file_hashes);

  for (const filePath of receipt.files) {
    assert.ok(
      selectedSurfaceFiles.has(filePath) ||
        selectedSurfaceFiles.has(
          filePath.replace(/^examples\/template\//, ""),
        ),
      `${filePath} missing from Backend Platform Client file list`,
    );
    assert.equal(
      selectedSurfaceHashes[filePath],
      sha256(filePath),
      `${filePath} hash is stale in Backend Platform Client visibility row`,
    );
  }

  for (const marker of [
    'data-dx-package="supabase/client"',
    'data-dx-component="supabase-profile-workflow"',
    'data-dx-component="supabase-schema-query-workflow"',
    'data-dx-dashboard-workflow="account-profile-settings"',
    'data-dx-dashboard-workflow="supabase-schema-query"',
    'data-dx-supabase-action="load-profile-fixture"',
    'data-dx-supabase-action="prepare-profile-upsert"',
    'data-dx-supabase-action="run-local-schema-query"',
    'data-dx-style-surface="backend-platform-client"',
    'data-dx-token-scope="supabase/client"',
    "data-dx-supabase-query-operation",
    "data-dx-supabase-receipt-path",
  ]) {
    const markers = backendPlatformVisibility.selected_surfaces.flatMap(
      (surface) => surface.source_markers,
    );

    assert.ok(
      markers.includes(marker),
      `${marker} missing from Backend Platform Client visibility markers`,
    );
  }

  assert.match(
    profileWorkflow,
    /data-dx-style-surface="backend-platform-client"/,
  );
  assert.match(profileWorkflow, /data-dx-token-scope="supabase\/client"/);
  assert.match(dataStatus, /data-dx-style-surface="backend-platform-client"/);
  assert.match(dataStatus, /data-dx-token-scope="supabase\/client"/);

  for (const metric of [
    "backend_platform_client_package_present",
    "backend_platform_client_receipt_present",
    "backend_platform_client_receipt_stale",
    "backend_platform_client_missing_receipt",
    "backend_platform_client_blocked_surface",
    "backend_platform_client_unsupported_surface",
    "backend_platform_client_hash_manifest_present",
    "backend_platform_client_hash_mismatch",
    "backend_platform_client_receipt_hash_refresh_current",
    "backend_platform_client_receipt_hash_refresh_stale",
    "backend_platform_client_receipt_hash_refresh_missing",
    "backend_platform_client_dx_style_compatibility_present",
    "backend_platform_client_dx_style_compatibility_missing",
  ]) {
    assert.ok(
      backendPlatformVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from Backend Platform Client visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  const dxStyle = backendPlatformVisibility.dx_style_compatibility;
  assert.ok(dxStyle, "Backend Platform Client dx-style metadata is missing");
  assert.equal(dxStyle.schema, "dx.forge.package.dx_style_compatibility");
  assert.equal(dxStyle.status, "present");
  assert.equal(
    dxStyle.token_source,
    "examples/template/styles/globals.css",
  );
  assert.equal(
    dxStyle.generated_css,
    "examples/template/styles/globals.css",
  );
  assert.deepEqual(dxStyle.visible_surfaces, [
    "supabase-profile-workflow",
    "supabase-schema-query-workflow",
  ]);
  assert.deepEqual(dxStyle.source_files, [
    "examples/template/supabase-profile-workflow.tsx",
    "examples/template/data-status.tsx",
    "examples/template/styles/globals.css",
  ]);
  assert.deepEqual(dxStyle.data_dx_markers, [
    'data-dx-style-surface="backend-platform-client"',
    'data-dx-token-scope="supabase/client"',
  ]);
  assert.equal(dxStyle.receipt_path, receiptPath);
  assert.equal(dxStyle.runtime_proof, false);
  assert.ok(
    dxStyle.runtime_limitations.some((limitation) =>
      limitation.startsWith("SOURCE-ONLY:"),
    ),
  );
  assert.ok(
    dxStyle.runtime_limitations.some((limitation) =>
      limitation.startsWith("ADAPTER-BOUNDARY:"),
    ),
  );
  assert.deepEqual(receipt.dx_style_compatibility, dxStyle);
  assert.match(backendReadModelBlock, /dxStyleCompatibility/);

  const hashRefresh = backendPlatformVisibility.receipt_hash_refresh;
  assert.ok(hashRefresh, "Backend Platform Client hash refresh status is missing");
  assert.equal(hashRefresh.schema, "dx.forge.package.receipt_hash_refresh");
  assert.equal(hashRefresh.status, "current");
  assert.equal(
    hashRefresh.helper_path,
    "examples/template/backend-platform-client-receipt-hashes.ts",
  );
  assert.equal(
    hashRefresh.check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/backend-platform-client-receipt-hashes.ts --check",
  );
  assert.equal(
    hashRefresh.write_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/backend-platform-client-receipt-hashes.ts --write",
  );
  assert.equal(
    hashRefresh.json_check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/backend-platform-client-receipt-hashes.ts --check --json",
  );
  assert.equal(hashRefresh.receipt_path, receiptPath);
  assert.equal(hashRefresh.hash_algorithm, "sha256");
  assert.equal(
    hashRefresh.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.deepEqual(hashRefresh.tracked_files, receipt.files);
  assert.equal(hashRefresh.tracked_file_count, Object.keys(receipt.file_hashes).length);
  assert.equal(hashRefresh.tracked_file_count, 12);
  assert.deepEqual(hashRefresh.current_files, hashRefresh.tracked_files);
  assert.deepEqual(hashRefresh.stale_files, []);
  assert.deepEqual(hashRefresh.missing_files, []);
  assert.deepEqual(hashRefresh.stale_mirror_files, []);
  assert.deepEqual(hashRefresh.missing_mirror_files, []);
  assert.equal(hashRefresh.mirror_problem_count, 0);
  assert.equal(hashRefresh.stale_file_count, 0);
  assert.equal(hashRefresh.missing_file_count, 0);
  assert.equal(hashRefresh.runtime_execution, false);
  assert.equal(hashRefresh.secret_access, false);
  assert.equal(
    hashRefresh.zed_visibility,
    "backend-platform-client:receipt-hash-refresh",
  );

  const helper = spawnSync(
    process.execPath,
    [
      "examples/template/backend-platform-client-receipt-hashes.ts",
      "--check",
      "--json",
    ],
    {
      cwd: root,
      encoding: "utf8",
    },
  );
  assert.equal(helper.status, 0, helper.stdout + helper.stderr);
  const helperReport = JSON.parse(helper.stdout);
  assert.equal(helperReport.schema, hashRefresh.schema);
  assert.equal(helperReport.package_id, "supabase/client");
  assert.equal(helperReport.official_package_name, "Backend Platform Client");
  assert.equal(helperReport.status, hashRefresh.status);
  assert.equal(
    helperReport.preview_manifest_materializer,
    hashRefresh.preview_manifest_materializer,
  );
  assert.deepEqual(helperReport.tracked_files, hashRefresh.tracked_files);
  assert.equal(helperReport.tracked_file_count, hashRefresh.tracked_file_count);
  assert.equal(helperReport.stale_file_count, hashRefresh.stale_file_count);
  assert.equal(helperReport.missing_file_count, hashRefresh.missing_file_count);
  assert.equal(helperReport.runtime_execution, false);
  assert.equal(helperReport.secret_access, false);

  assert.match(readModel, /export const backendPlatformClientPackageVisibility/);
  assert.match(readModel, /receiptHashRefresh/);
  assert.match(readModel, /previewManifestMaterializer/);
  assert.match(readModel, /backend-platform-client-preview-manifest-materializer/);
  assert.match(readModel, /backend-platform-client-lock-backed-source/);
  assert.match(readModel, /app\/api\/supabase\/readiness\/route\.ts/);
  assert.match(readModel, /backend-platform-client:receipt-hash-refresh/);
  assert.match(statusSource, /backendPlatformClientPackageVisibility/);
  assert.match(statusSource, /backendPlatformClientVisibility/);
  assert.ok(
    status.zed_receipt_surfaces.includes(
      "backend-platform-client:supabase-profile-workflow",
    ),
    "Backend Platform Client profile Zed receipt surface is missing",
  );
  assert.ok(
    status.zed_receipt_surfaces.includes(
      "backend-platform-client:supabase-schema-query-workflow",
    ),
    "Backend Platform Client schema-query Zed receipt surface is missing",
  );
  assert.ok(
    status.zed_receipt_surfaces.includes(
      "backend-platform-client:receipt-hash-refresh",
    ),
    "Backend Platform Client receipt hash refresh Zed surface is missing",
  );
  assert.ok(
    status.zed_receipt_surfaces.includes(
      "backend-platform-client:dx-style-compatibility",
    ),
    "Backend Platform Client dx-style Zed receipt surface is missing",
  );
  assert.match(packageDoc, /package-status read model/);
  assert.match(packageDoc, /dx-style compatibility/);
  assert.match(packageDoc, /backend_platform_client_dx_style_compatibility_present/);
  assert.match(packageDoc, /receipt_hash_refresh/);
  assert.match(packageDoc, /backend_platform_client_hash_mismatch/);
});
