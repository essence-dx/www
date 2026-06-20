const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperRelativePath =
  "examples/template/3d-scene-system-receipt-hashes.ts";
const helperPath = path.join(root, helperRelativePath);
const receiptPath =
  "examples/template/.dx/forge/receipts/3d-launch-scene-dashboard-workflow.json";
const generatedReceiptPath =
  ".dx/forge/receipts/3d-launch-scene-dashboard-workflow.json";
const packageStatusPath =
  "examples/template/.dx/forge/package-status.json";
const readModelPath =
  "examples/template/forge-package-status-read-model.ts";
const sourceGuardRunbookFixturePath =
  "docs/packages/3d-scene-system.source-guard-runbook.json";
const sourceGuardRunbookSurfaceId =
  "three-scene-system-source-guard-runbook";
const previewManifestMaterializerPath =
  "tools/launch/materialize-www-template.ts";
const previewManifestMaterializerSurfaceId =
  "three-scene-system-preview-manifest-materializer";
const nextTemplateSourceSurfaceId = "three-scene-system-next-template-source";
const zedVisibility = "3d-scene-system:receipt-hash-refresh";
const dxCheckMetrics = [
  "three_scene_system_receipt_present",
  "three_scene_system_receipt_stale",
  "three_scene_system_missing_receipt",
  "three_scene_system_blocked_surface",
  "three_scene_system_unsupported_surface",
  "three_scene_system_hash_manifest_present",
  "three_scene_system_hash_mismatch",
  "three_scene_system_receipt_hash_refresh_current",
  "three_scene_system_receipt_hash_refresh_stale",
  "three_scene_system_receipt_hash_refresh_missing",
  "three_scene_system_dx_style_compatibility_present",
  "three_scene_system_dx_style_compatibility_missing",
];

const nextTemplateSourceFiles = [
  "examples/template/components/scene/launch-scene.tsx",
  "examples/template/lib/scene/index.ts",
  "examples/template/lib/scene/types.ts",
  "examples/template/lib/scene/preset.ts",
  "examples/template/lib/scene/interaction.ts",
  "examples/template/lib/scene/dashboard-workflow.ts",
  "examples/template/lib/scene/dashboard-controls.ts",
  "examples/template/lib/scene/frame-sample.ts",
  "examples/template/lib/scene/capability-report.ts",
  "examples/template/lib/scene/viewport-report.ts",
  "examples/template/lib/scene/bounds-report.ts",
  "examples/template/lib/scene/raycast-report.ts",
  "examples/template/lib/scene/preview-readiness.ts",
  "examples/template/lib/scene/performance-monitor.ts",
  "examples/template/lib/scene/renderer-handoff.ts",
  "examples/template/lib/scene/r3f-renderer-adapter.ts",
  "examples/template/lib/scene/webgl-runtime.ts",
  "examples/template/lib/scene/metadata.ts",
  "examples/template/lib/scene/README.md",
];

const selectedFiles = [
  "examples/template/launch-scene.tsx",
  "examples/template/scene/dashboard-workflow.ts",
  "examples/template/scene/dashboard-controls.ts",
  "examples/template/scene/frame-sample.ts",
  "examples/template/scene/capability-report.ts",
  "examples/template/scene/viewport-report.ts",
  "examples/template/scene/bounds-report.ts",
  "examples/template/scene/raycast-report.ts",
  "examples/template/scene/preset.ts",
  "examples/template/scene/metadata.ts",
  "docs/packages/3d-scene-system.md",
  sourceGuardRunbookFixturePath,
  previewManifestMaterializerPath,
  "tools/launch/runtime-template/pages/index.html",
  "tools/launch/runtime-template/assets/launch-runtime.ts",
  ...nextTemplateSourceFiles,
];

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function runHelper(args, cwd = root) {
  return spawnSync(process.execPath, [helperPath, ...args], {
    cwd,
    encoding: "utf8",
  });
}

function writeJson(filePath, value) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`);
}

function writeFixtureFile(fixtureRoot, relativePath) {
  const filePath = path.join(fixtureRoot, relativePath);
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(
    filePath,
    `export const threeSceneHashFixture = ${JSON.stringify(relativePath)};\n`,
  );
}

test("3D Scene System exposes receipt hash freshness through package-status and read model", () => {
  assert.ok(fs.existsSync(helperPath), "3D Scene System hash helper is missing");

  const helperSource = read(helperRelativePath);
  const receipt = readJson(receiptPath);
  const status = readJson(packageStatusPath);
  const readModel = read(readModelPath);
  const packageDoc = read("docs/packages/3d-scene-system.md");

  assert.match(helperSource, /OFFICIAL_PACKAGE_NAME = "3D Scene System"/);
  assert.match(helperSource, /PACKAGE_ID = "3d\/launch-scene"/);
  assert.match(helperSource, /SOURCE_GUARD_RUNBOOK_FIXTURE/);
  assert.match(helperSource, /PREVIEW_MANIFEST_MATERIALIZER/);
  assert.match(helperSource, /NEXT_TEMPLATE_SOURCE_FILES/);
  assert.match(helperSource, /three-scene-system-next-template-source/);
  assert.match(
    helperSource,
    /UPSTREAM_PACKAGE = "three \+ @react-three\/fiber \+ @react-three\/drei"/,
  );
  assert.match(
    helperSource,
    /SOURCE_MIRROR = "G:\/WWW\/inspirations\/three\.js; G:\/WWW\/inspirations\/react-three-fiber; G:\/WWW\/inspirations\/drei"/,
  );
  assert.doesNotMatch(
    helperSource,
    /new THREE\.WebGLRenderer|createRoot\(|fetch\(|process\.env|npm install|pnpm install|yarn add/,
  );

  assert.equal(receipt.package_id, "3d/launch-scene");
  assert.equal(receipt.official_package_name, "3D Scene System");
  assert.equal(
    receipt.upstream_package,
    "three + @react-three/fiber + @react-three/drei",
  );
  assert.equal(
    receipt.upstream_version,
    "three 0.184.0; @react-three/fiber 9.6.1; @react-three/drei local mirror",
  );
  assert.equal(
    receipt.source_mirror,
    "G:/WWW/inspirations/three.js; G:/WWW/inspirations/react-three-fiber; G:/WWW/inspirations/drei",
  );
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.equal(
    receipt.source_guard_runbook_fixture,
    sourceGuardRunbookFixturePath,
  );
  assert.equal(
    receipt.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.deepEqual(Object.keys(receipt.file_hashes).sort(), selectedFiles.sort());

  const visibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "3d/launch-scene",
  );
  assert.ok(visibility, "3D Scene System package-status row is missing");
  assert.equal(visibility.official_package_name, "3D Scene System");
  assert.equal(
    visibility.upstream_package,
    "three + @react-three/fiber + @react-three/drei",
  );
  assert.equal(visibility.source_hashes.algorithm, "sha256");
  assert.deepEqual(
    Object.keys(visibility.source_hashes.files).sort(),
    selectedFiles.sort(),
  );

  const hashRefresh = visibility.receipt_hash_refresh;
  assert.ok(hashRefresh, "3D Scene System receipt_hash_refresh is missing");
  assert.equal(hashRefresh.schema, "dx.forge.package.receipt_hash_refresh");
  assert.equal(hashRefresh.status, "current");
  assert.equal(hashRefresh.helper_path, helperRelativePath);
  assert.equal(
    hashRefresh.check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/3d-scene-system-receipt-hashes.ts --check",
  );
  assert.equal(
    hashRefresh.write_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/3d-scene-system-receipt-hashes.ts --write",
  );
  assert.equal(
    hashRefresh.json_check_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/3d-scene-system-receipt-hashes.ts --check --json",
  );
  assert.equal(hashRefresh.receipt_path, generatedReceiptPath);
  assert.equal(
    hashRefresh.source_guard_runbook_fixture,
    sourceGuardRunbookFixturePath,
  );
  assert.equal(
    hashRefresh.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.ok(
    hashRefresh.tracked_files.includes(previewManifestMaterializerPath),
    "3D Scene System receipt_hash_refresh should expose the preview-manifest materializer",
  );
  assert.equal(hashRefresh.hash_algorithm, "sha256");
  assert.equal(hashRefresh.tracked_file_count, selectedFiles.length);
  assert.equal(hashRefresh.stale_file_count, 0);
  assert.equal(hashRefresh.missing_file_count, 0);
  assert.equal(hashRefresh.runtime_execution, false);
  assert.equal(hashRefresh.secret_access, false);
  assert.equal(hashRefresh.zed_visibility, zedVisibility);

  assert.ok(
    status.zed_receipt_surfaces.includes(zedVisibility),
    "3D Scene System helper is missing from Zed receipt surfaces",
  );
  for (const metric of dxCheckMetrics) {
    assert.ok(
      visibility.dx_check_metrics.includes(metric),
      `${metric} missing from 3D Scene System visibility metrics`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status root metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }
  assert.match(readModel, /export const threeDSceneSystemPackageVisibility/);
  assert.match(readModel, /receiptHashRefresh/);
  assert.match(readModel, /sourceGuardRunbookFixture/);
  assert.match(readModel, /previewManifestMaterializer/);
  assert.match(
    readModel,
    /docs\/packages\/3d-scene-system\.source-guard-runbook\.json/,
  );
  assert.match(readModel, /tools\/launch\/materialize-www-template\.ts/);
  assert.match(readModel, /3d-scene-system:receipt-hash-refresh/);
  assert.match(readModel, /three-scene-system-next-template-source/);
  const sourceGuardRunbookSurface = visibility.selected_surfaces.find(
    (surface) => surface.surface_id === sourceGuardRunbookSurfaceId,
  );
  assert.ok(
    sourceGuardRunbookSurface,
    "3D Scene System source-guard runbook surface is missing",
  );
  assert.deepEqual(sourceGuardRunbookSurface.files, [
    sourceGuardRunbookFixturePath,
  ]);
  assert.equal(
    sourceGuardRunbookSurface.file_hashes[sourceGuardRunbookFixturePath],
    receipt.file_hashes[sourceGuardRunbookFixturePath],
  );
  const previewManifestMaterializerSurface = visibility.selected_surfaces.find(
    (surface) => surface.surface_id === previewManifestMaterializerSurfaceId,
  );
  assert.ok(
    previewManifestMaterializerSurface,
    "3D Scene System preview-manifest materializer surface is missing",
  );
  assert.deepEqual(previewManifestMaterializerSurface.files, [
    previewManifestMaterializerPath,
    "public/preview-.dx/build-cache/manifest.json",
  ]);
  assert.equal(
    previewManifestMaterializerSurface.file_hashes[
      previewManifestMaterializerPath
    ],
    receipt.file_hashes[previewManifestMaterializerPath],
  );
  const nextTemplateSourceSurface = visibility.selected_surfaces.find(
    (surface) => surface.surface_id === nextTemplateSourceSurfaceId,
  );
  assert.ok(
    nextTemplateSourceSurface,
    "3D Scene System Next-shaped source surface is missing",
  );
  assert.deepEqual(nextTemplateSourceSurface.files, nextTemplateSourceFiles);
  for (const sourceFile of nextTemplateSourceFiles) {
    assert.equal(
      nextTemplateSourceSurface.file_hashes[sourceFile],
      receipt.file_hashes[sourceFile],
    );
  }
  assert.match(packageDoc, /3d-scene-system-receipt-hashes\.ts --check/);
  assert.match(packageDoc, /receipt_hash_refresh/);

  const helper = runHelper(["--check", "--json"]);
  assert.equal(helper.status, 0, helper.stdout + helper.stderr);
  const report = JSON.parse(helper.stdout);
  assert.equal(report.schema, hashRefresh.schema);
  assert.equal(report.package_id, "3d/launch-scene");
  assert.equal(report.official_package_name, "3D Scene System");
  assert.equal(
    report.upstream_package,
    "three + @react-three/fiber + @react-three/drei",
  );
  assert.equal(
    report.upstream_version,
    "three 0.184.0; @react-three/fiber 9.6.1; @react-three/drei local mirror",
  );
  assert.equal(
    report.source_mirror,
    "G:/WWW/inspirations/three.js; G:/WWW/inspirations/react-three-fiber; G:/WWW/inspirations/drei",
  );
  assert.equal(report.status, "current");
  assert.equal(report.tracked_file_count, selectedFiles.length);
  assert.equal(
    report.source_guard_runbook_fixture,
    sourceGuardRunbookFixturePath,
  );
  assert.equal(
    report.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.deepEqual([...report.tracked_files].sort(), [...selectedFiles].sort());
  assert.equal(report.stale_file_count, 0);
  assert.equal(report.missing_file_count, 0);
  assert.equal(report.runtime_execution, false);
  assert.equal(report.secret_access, false);
  assert.equal(report.runs_package_install, false);
  assert.equal(report.zed_visibility, zedVisibility);
});

test("3D Scene System receipt hash helper refreshes selected surfaces without runtime proof", () => {
  assert.ok(fs.existsSync(helperPath), "3D Scene System hash helper is missing");

  const fixtureRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-three-hashes-"));
  try {
    for (const selectedFile of selectedFiles) {
      writeFixtureFile(fixtureRoot, selectedFile);
    }

    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.receipt",
      package_id: "3d/launch-scene",
      package_name: "3D Scene System",
      official_package_name: "3D Scene System",
      upstream_package: "three + @react-three/fiber + @react-three/drei",
      upstream_version:
        "three 0.184.0; @react-three/fiber 9.6.1; @react-three/drei local mirror",
      source_mirror:
        "G:/WWW/inspirations/three.js; G:/WWW/inspirations/react-three-fiber; G:/WWW/inspirations/drei",
      source_guard_runbook_fixture: sourceGuardRunbookFixturePath,
      preview_manifest_materializer: previewManifestMaterializerPath,
      hash_algorithm: "sha256",
      file_hashes: Object.fromEntries(
        selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
      ),
      runtime_execution: false,
    });

    writeJson(path.join(fixtureRoot, packageStatusPath), {
      zed_receipt_surfaces: [
        "3d-scene-system:launch-scene-dashboard-workflow",
      ],
      package_lane_visibility: [
        {
          official_package_name: "3D Scene System",
          package_id: "3d/launch-scene",
          upstream_package: "three + @react-three/fiber + @react-three/drei",
          upstream_version:
            "three 0.184.0; @react-three/fiber 9.6.1; @react-three/drei local mirror",
          source_mirror:
            "G:/WWW/inspirations/three.js; G:/WWW/inspirations/react-three-fiber; G:/WWW/inspirations/drei",
          package_receipt_path: generatedReceiptPath,
          selected_surfaces: [
            {
              surface_id: "launch-scene-dashboard-workflow",
              receipt_path: generatedReceiptPath,
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
              ),
            },
          ],
        },
      ],
    });

    const absoluteReadModelPath = path.join(fixtureRoot, readModelPath);
    fs.mkdirSync(path.dirname(absoluteReadModelPath), { recursive: true });
    fs.writeFileSync(
      absoluteReadModelPath,
      [
        "export const motionAnimationPackageVisibility = {",
        "  receiptHashRefresh: {",
        '    zedVisibility: "motion-animation:receipt-hash-refresh",',
        "  },",
        "};",
        "",
        "export const threeDSceneSystemPackageVisibility = {",
        '  packageId: "3d/launch-scene",',
        `  packageReceiptPath: "${generatedReceiptPath}",`,
        "  statusVocabulary: [],",
        "  selectedSurfaces: [",
        "    {",
        "      fileHashes: {",
        ...selectedFiles.flatMap((selectedFile) => [
          `        "${selectedFile}":`,
          '          "stale",',
        ]),
        "      },",
        "    },",
        "  ],",
        "  dxCheckMetrics: [],",
        "};",
        "",
      ].join("\n"),
    );

    const stale = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    const staleReport = JSON.parse(stale.stdout);
    assert.equal(staleReport.package_id, "3d/launch-scene");
    assert.equal(staleReport.status, "missing");
    assert.equal(staleReport.runtime_execution, false);
    assert.equal(staleReport.secret_access, false);
    assert.equal(staleReport.runs_package_install, false);
    assert.equal(staleReport.zed_visibility, zedVisibility);

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);
    assert.match(write.stdout, /3D Scene System receipt hashes updated/);

    const fresh = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.equal(fresh.status, 0, fresh.stdout + fresh.stderr);
    const freshReport = JSON.parse(fresh.stdout);
    assert.equal(freshReport.schema, "dx.forge.package.receipt_hash_refresh");
    assert.equal(freshReport.status, "current");
    assert.equal(freshReport.tracked_file_count, selectedFiles.length);
    assert.equal(freshReport.stale_file_count, 0);
    assert.equal(freshReport.missing_file_count, 0);

    const refreshedReceipt = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, receiptPath), "utf8"),
    );
    const refreshedStatus = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, packageStatusPath), "utf8"),
    );
    const readModelText = fs.readFileSync(absoluteReadModelPath, "utf8");
    const visibility = refreshedStatus.package_lane_visibility[0];

    assert.ok(refreshedStatus.zed_receipt_surfaces.includes(zedVisibility));
    assert.equal(visibility.receipt_hash_refresh.zed_visibility, zedVisibility);
    assert.equal(
      visibility.receipt_hash_refresh.source_guard_runbook_fixture,
      sourceGuardRunbookFixturePath,
    );
    assert.equal(
      visibility.receipt_hash_refresh.preview_manifest_materializer,
      previewManifestMaterializerPath,
    );
    assert.ok(
      visibility.receipt_hash_refresh.tracked_files.includes(
        previewManifestMaterializerPath,
      ),
    );
    assert.equal(visibility.source_hashes.algorithm, "sha256");
    const refreshedRunbookSurface = visibility.selected_surfaces.find(
      (surface) => surface.surface_id === sourceGuardRunbookSurfaceId,
    );
    assert.ok(
      refreshedRunbookSurface,
      "write mode should add the source-guard runbook selected surface",
    );
    assert.deepEqual(refreshedRunbookSurface.files, [
      sourceGuardRunbookFixturePath,
    ]);
    const refreshedMaterializerSurface = visibility.selected_surfaces.find(
      (surface) => surface.surface_id === previewManifestMaterializerSurfaceId,
    );
    assert.ok(
      refreshedMaterializerSurface,
      "write mode should add the preview-manifest materializer selected surface",
    );
    assert.deepEqual(refreshedMaterializerSurface.files, [
      previewManifestMaterializerPath,
      "public/preview-.dx/build-cache/manifest.json",
    ]);
    assert.equal(
      refreshedMaterializerSurface.file_hashes[previewManifestMaterializerPath],
      refreshedReceipt.file_hashes[previewManifestMaterializerPath],
    );
    const refreshedNextTemplateSourceSurface = visibility.selected_surfaces.find(
      (surface) => surface.surface_id === nextTemplateSourceSurfaceId,
    );
    assert.ok(
      refreshedNextTemplateSourceSurface,
      "write mode should add the Next-shaped source selected surface",
    );
    assert.deepEqual(
      refreshedNextTemplateSourceSurface.files,
      nextTemplateSourceFiles,
    );
    for (const sourceFile of nextTemplateSourceFiles) {
      assert.equal(
        refreshedNextTemplateSourceSurface.file_hashes[sourceFile],
        refreshedReceipt.file_hashes[sourceFile],
      );
    }
    assert.match(readModelText, /sourceGuardRunbookFixture/);
    assert.match(readModelText, /previewManifestMaterializer/);
    assert.match(readModelText, /three-scene-system-next-template-source/);
    assert.match(
      readModelText,
      /docs\/packages\/3d-scene-system\.source-guard-runbook\.json/,
    );
    assert.match(
      readModelText,
      /tools\/launch\/materialize-www-template\.ts/,
    );
    for (const metric of dxCheckMetrics) {
      assert.ok(
        visibility.dx_check_metrics.includes(metric),
        `${metric} was not written to the fixture visibility row`,
      );
      assert.ok(
        refreshedStatus.dx_check_metrics.includes(metric),
        `${metric} was not written to package-status root metrics`,
      );
      assert.match(readModelText, new RegExp(metric));
    }
    assert.match(
      readModelText,
      /motion-animation:receipt-hash-refresh/,
      "3D Scene System helper must not rewrite neighboring package lanes",
    );

    for (const selectedFile of selectedFiles) {
      const refreshedHash = refreshedReceipt.file_hashes[selectedFile];
      assert.match(refreshedHash, /^[a-f0-9]{64}$/);
      assert.equal(visibility.source_hashes.files[selectedFile], refreshedHash);
      assert.ok(
        visibility.selected_surfaces.some(
          (surface) => surface.file_hashes?.[selectedFile] === refreshedHash,
        ),
        `${selectedFile} hash was not mirrored into a selected surface`,
      );
      assert.match(readModelText, new RegExp(refreshedHash));
    }

    fs.appendFileSync(
      path.join(fixtureRoot, previewManifestMaterializerPath),
      "\nexport const mutatedThreeScenePreviewMaterializer = true;\n",
    );
    const materializerStale = runHelper([
      "--root",
      fixtureRoot,
      "--check",
      "--json",
    ]);
    assert.notEqual(
      materializerStale.status,
      0,
      materializerStale.stdout + materializerStale.stderr,
    );
    const materializerStaleReport = JSON.parse(materializerStale.stdout);
    assert.equal(materializerStaleReport.status, "stale");
    assert.equal(materializerStaleReport.missing_file_count, 0);
    assert.deepEqual(
      [...new Set(materializerStaleReport.stale_files)].sort(),
      [previewManifestMaterializerPath],
    );
    assert.equal(materializerStaleReport.runtime_execution, false);
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});
