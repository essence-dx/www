const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath = path.join(
  root,
  "examples/template/backend-platform-client-receipt-hashes.ts",
);
const runbookFixturePath =
  "docs/packages/backend-platform-client.source-guard-runbook.json";
const previewManifestMaterializerPath =
  "tools/launch/materialize-www-template.ts";
const lockBackedBackendPlatformSourceFiles = [
  "examples/template/lib/supabase/metadata.ts",
  "examples/template/lib/supabase/README.md",
  "examples/template/lib/supabase/schema.sql",
  "examples/template/server/supabase/readiness.ts",
  "examples/template/app/api/supabase/readiness/route.ts",
];

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

test("Backend Platform Client receipt hash helper refreshes receipt, package-status, and read model hashes", () => {
  assert.ok(
    fs.existsSync(helperPath),
    "Backend Platform Client hash helper is missing",
  );

  const fixtureRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-backend-platform-client-hashes-"),
  );
  try {
    const selectedFiles = [
      "examples/template/supabase-profile-workflow.tsx",
      runbookFixturePath,
      previewManifestMaterializerPath,
      ...lockBackedBackendPlatformSourceFiles,
    ];
    for (const selectedFile of selectedFiles) {
      const selectedFilePath = path.join(fixtureRoot, selectedFile);
      fs.mkdirSync(path.dirname(selectedFilePath), { recursive: true });
      fs.writeFileSync(
        selectedFilePath,
        `export const backendPlatformClientHashFixture = ${JSON.stringify(
          selectedFile,
        )};\n`,
      );
    }

    const receiptPath =
      "examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json";
    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.package_dashboard_workflow_receipt",
      package_id: "supabase/client",
      package_name: "Backend Platform Client",
      official_dx_package_name: "Backend Platform Client",
      upstream_package: "@supabase/ssr + @supabase/supabase-js",
      upstream_version: "@supabase/ssr latest; @supabase/supabase-js ^2",
      hash_algorithm: "sha256",
      file_hashes: Object.fromEntries(
        selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
      ),
    });

    const packageStatusPath =
      "examples/template/.dx/forge/package-status.json";
    writeJson(path.join(fixtureRoot, packageStatusPath), {
      package_lane_visibility: [
        {
          official_package_name: "Backend Platform Client",
          package_id: "supabase/client",
          package_receipt_path: receiptPath,
          selected_surfaces: [
            {
              surface_id: "supabase-profile-workflow",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
              ),
            },
          ],
        },
      ],
    });

    const readModelPath =
      "examples/template/forge-package-status-read-model.ts";
    const absoluteReadModelPath = path.join(fixtureRoot, readModelPath);
    fs.mkdirSync(path.dirname(absoluteReadModelPath), { recursive: true });
    fs.writeFileSync(
      absoluteReadModelPath,
      [
        "export const backendPlatformClientPackageVisibility = {",
        "  selectedSurfaces: [{",
        "    fileHashes: {",
        ...selectedFiles.flatMap((selectedFile) => [
          `      "${selectedFile}":`,
          '        "stale",',
        ]),
        "    },",
        "  }],",
        "} as const;",
        "",
      ].join("\n"),
    );

    const stale = runHelper(["--root", fixtureRoot, "--check"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    assert.match(stale.stdout + stale.stderr, /stale/i);
    assert.match(stale.stdout + stale.stderr, /supabase-profile-workflow\.tsx/);

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);
    assert.match(write.stdout, /updated/i);

    const fresh = runHelper(["--root", fixtureRoot, "--check"]);
    assert.equal(fresh.status, 0, fresh.stdout + fresh.stderr);
    assert.match(
      fresh.stdout,
      /Backend Platform Client receipt hashes are current/,
    );

    const refreshedReceipt = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, receiptPath), "utf8"),
    );
    const refreshedStatus = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, packageStatusPath), "utf8"),
    );
    const readModelText = fs.readFileSync(absoluteReadModelPath, "utf8");
    for (const selectedFile of selectedFiles) {
      const refreshedHash = refreshedReceipt.file_hashes[selectedFile];
      assert.match(refreshedHash, /^[a-f0-9]{64}$/);
      assert.match(readModelText, new RegExp(refreshedHash));
    }
    const selectedSurfaceHashes = Object.assign(
      {},
      ...refreshedStatus.package_lane_visibility[0].selected_surfaces.map(
        (surface) => surface.file_hashes ?? {},
      ),
    );
    for (const selectedFile of selectedFiles) {
      assert.equal(selectedSurfaceHashes[selectedFile], refreshedReceipt.file_hashes[selectedFile]);
    }
    assert.ok(
      refreshedStatus.package_lane_visibility[0].selected_surfaces.some(
        (surface) =>
          surface.surface_id === "backend-platform-client-lock-backed-source" &&
          lockBackedBackendPlatformSourceFiles.every(
            (selectedFile) =>
              surface.file_hashes[selectedFile] ===
              refreshedReceipt.file_hashes[selectedFile],
          ),
      ),
      "Backend Platform Client lock-backed source surface should mirror readiness route, server helper, metadata, schema, and README hashes",
    );
    assert.equal(
      refreshedStatus.package_lane_visibility[0].receipt_hash_refresh
        .preview_manifest_materializer,
      previewManifestMaterializerPath,
    );
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("Backend Platform Client docs publish the hash refresh command without claiming runtime proof", () => {
  const packageDoc = fs.readFileSync(
    path.join(root, "docs/packages/supabase-client.md"),
    "utf8",
  );

  assert.match(
    packageDoc,
    /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/backend-platform-client-receipt-hashes\.ts --check/,
  );
  assert.match(packageDoc, /--write/);
  assert.match(packageDoc, /backend-platform-client\.source-guard-runbook\.json/);
  assert.match(packageDoc, /source-guard runbook fixture drift/i);
  assert.match(packageDoc, /does not contact hosted Supabase/i);
});

test("Backend Platform Client receipt helper tracks the source-guard runbook fixture and preview materializer", () => {
  const receipt = JSON.parse(
    fs.readFileSync(
      path.join(
        root,
        "examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json",
      ),
      "utf8",
    ),
  );
  const packageStatus = JSON.parse(
    fs.readFileSync(
      path.join(root, "examples/template/.dx/forge/package-status.json"),
      "utf8",
    ),
  );
  const readModel = fs.readFileSync(
    path.join(root, "examples/template/forge-package-status-read-model.ts"),
    "utf8",
  );

  const report = runHelper(["--check", "--json"]);
  assert.equal(report.status, 0, report.stdout + report.stderr);
  const helperReport = JSON.parse(report.stdout);

  const receiptHashPaths = Object.keys(receipt.file_hashes);
  assert.ok(
    receiptHashPaths.includes(runbookFixturePath),
    "receipt must hash the Backend Platform Client source-guard runbook fixture",
  );
  assert.ok(
    receiptHashPaths.includes(previewManifestMaterializerPath),
    "receipt must hash the Backend Platform Client preview-manifest materializer",
  );
  assert.equal(helperReport.source_guard_runbook_fixture, runbookFixturePath);
  assert.equal(
    helperReport.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.equal(helperReport.tracked_file_count, receiptHashPaths.length);
  assert.equal(helperReport.tracked_file_count, 12);
  for (const selectedFile of lockBackedBackendPlatformSourceFiles) {
    assert.ok(
      receiptHashPaths.includes(selectedFile),
      `${selectedFile} should be hash-backed in the Backend Platform Client receipt`,
    );
    assert.ok(
      helperReport.tracked_files.includes(selectedFile),
      `${selectedFile} should be tracked as Backend Platform Client lock-backed source`,
    );
  }
  assert.ok(
    helperReport.tracked_files.includes(previewManifestMaterializerPath),
    "helper report must list the preview-manifest materializer as tracked",
  );
  assert.ok(
    helperReport.files.some(
      (entry) => entry.path === previewManifestMaterializerPath,
    ),
    "helper report must include materializer freshness details",
  );

  const visibility = packageStatus.package_lane_visibility.find(
    (entry) => entry.package_id === "supabase/client",
  );
  assert.ok(visibility, "Backend Platform Client package-status row is missing");
  assert.equal(
    visibility.receipt_hash_refresh.source_guard_runbook_fixture,
    runbookFixturePath,
  );
  assert.equal(
    visibility.receipt_hash_refresh.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.equal(visibility.receipt_hash_refresh.tracked_file_count, 12);
  assert.ok(
    visibility.receipt_hash_refresh.tracked_files.includes(
      previewManifestMaterializerPath,
    ),
    "package-status receipt_hash_refresh must list the materializer",
  );
  assert.ok(
    visibility.selected_surfaces.some((surface) =>
      Object.prototype.hasOwnProperty.call(
        surface.file_hashes || {},
        runbookFixturePath,
      ),
    ),
    "package-status selected surfaces must mirror the runbook fixture hash",
  );
  assert.ok(
    visibility.selected_surfaces.some((surface) =>
      Object.prototype.hasOwnProperty.call(
        surface.file_hashes || {},
        previewManifestMaterializerPath,
      ),
    ),
    "package-status selected surfaces must mirror the materializer hash",
  );
  assert.ok(
    visibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "backend-platform-client-lock-backed-source" &&
        lockBackedBackendPlatformSourceFiles.every((selectedFile) =>
          Object.prototype.hasOwnProperty.call(
            surface.file_hashes || {},
            selectedFile,
          ),
        ),
    ),
    "package-status selected surfaces must mirror Backend Platform Client lock-backed readiness source hashes",
  );

  assert.match(
    readModel,
    /sourceGuardRunbookFixture:\s*\n?\s*"docs\/packages\/backend-platform-client\.source-guard-runbook\.json"/,
  );
  assert.match(
    readModel,
    /previewManifestMaterializer:\s*\n?\s*"tools\/launch\/materialize-www-template\.ts"/,
  );
  assert.match(readModel, /trackedFileCount: 12/);
  assert.match(readModel, /backend-platform-client-lock-backed-source/);
  assert.match(readModel, /app\/api\/supabase\/readiness\/route\.ts/);
  assert.match(readModel, /server\/supabase\/readiness\.ts/);
  assert.match(readModel, /lib\/supabase\/metadata\.ts/);
  assert.match(readModel, /lib\/supabase\/schema\.sql/);
  assert.match(
    readModel,
    /"docs\/packages\/backend-platform-client\.source-guard-runbook\.json":\s*\n\s*"[a-f0-9]{64}"/,
  );
  assert.match(
    readModel,
    /"tools\/launch\/materialize-www-template\.ts":\s*\n\s*"[a-f0-9]{64}"/,
  );
});
