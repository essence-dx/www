const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath = path.join(
  root,
  "examples/template/data-fetching-cache-receipt-hashes.ts",
);
const runbookFixturePath =
  "docs/packages/data-fetching-cache.source-guard-runbook.json";
const previewManifestMaterializerPath =
  "tools/launch/materialize-www-template.ts";
const forgeSourcePath = "core/src/ecosystem/forge_tanstack_query.rs";
const launchShellPath = "examples/template/template-shell.tsx";
const studioEditContractPath =
  "examples/template/dx-studio-edit-contract.ts";
const templateRouteContractPath =
  "examples/template/template-route-contract.ts";
const packageCatalogPath = "examples/template/package-catalog.ts";
const launchRuntimePagePath = "tools/launch/runtime-template/pages/index.html";
const dashboardWorkflowLibPath =
  "examples/dashboard/src/lib/queryDashboardWorkflow.ts";
const launchRuntimeAssetPath =
  "tools/launch/runtime-template/assets/launch-runtime.ts";
const templateQueryCacheModel =
  "examples/template/components/template-app/dashboard-query-cache.ts";
const serverQueryCacheReadiness =
  "examples/template/server/query-cache/readiness.ts";
const appRouteQueryCacheReadiness =
  "examples/template/app/api/query-cache/readiness/route.ts";

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

function readModelExport(source, exportName) {
  const start = source.indexOf(`export const ${exportName} = {`);
  assert.notEqual(start, -1, `${exportName} export is missing`);
  const nextExport = source.indexOf("\n\nexport const ", start + 1);
  return source.slice(start, nextExport === -1 ? undefined : nextExport);
}

test("Data Fetching & Cache receipt hash helper refreshes receipt, package-status, and read model hashes", () => {
  assert.ok(
    fs.existsSync(helperPath),
    "Data Fetching & Cache hash helper is missing",
  );

  const fixtureRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-data-fetching-hashes-"),
  );
  try {
    const selectedFiles = [
      forgeSourcePath,
      "examples/template/query-cache-status.tsx",
      "examples/template/query-dashboard-read-model.ts",
      launchShellPath,
      studioEditContractPath,
      templateRouteContractPath,
      packageCatalogPath,
      launchRuntimePagePath,
      dashboardWorkflowLibPath,
      "examples/dashboard/src/components/QueryDashboardWorkflow.tsx",
      runbookFixturePath,
      previewManifestMaterializerPath,
      launchRuntimeAssetPath,
      templateQueryCacheModel,
      serverQueryCacheReadiness,
      appRouteQueryCacheReadiness,
    ];
    for (const selectedFile of selectedFiles) {
      const selectedFilePath = path.join(fixtureRoot, selectedFile);
      fs.mkdirSync(path.dirname(selectedFilePath), { recursive: true });
      fs.writeFileSync(
        selectedFilePath,
        `export const dataFetchingHashFixture = ${JSON.stringify(
          selectedFile,
        )};\n`,
      );
    }

    const receiptPath =
      "examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json";
    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.package_dashboard_workflow_receipt",
      package_id: "tanstack/query",
      package_name: "Data Fetching & Cache",
      upstream_package: "@tanstack/react-query",
      upstream_version: "5.100.10",
      source_mirror: "G:/WWW/inspirations/tanstack-query",
      hash_algorithm: "sha256",
      file_hashes: Object.fromEntries(
        selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
      ),
      dx_check_visibility: {
        schema: "dx.forge.package.dx_check_visibility",
        monitored_surfaces: [
          {
            id: "query-dashboard-workflow",
            status: "present",
            hash_algorithm: "sha256",
            file_hashes: Object.fromEntries(
              selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
            ),
          },
        ],
      },
    });

    const packageStatusPath =
      "examples/template/.dx/forge/package-status.json";
    writeJson(path.join(fixtureRoot, packageStatusPath), {
      package_lane_visibility: [
        {
          official_package_name: "Data Fetching & Cache",
          package_id: "tanstack/query",
          package_receipt_path: receiptPath,
          selected_surfaces: [
            {
              surface_id: "data-fetching-cache-query-dashboard-workflow",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
              ),
            },
            {
              surface_id: "data-fetching-cache-starter-dashboard-workflow",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
              ),
            },
          ],
        },
      ],
      zed_receipt_surfaces: [
        "data-fetching-cache-query-dashboard-workflow",
        "data-fetching-cache-starter-dashboard-workflow",
      ],
    });

    const readModelPath =
      "examples/template/forge-package-status-read-model.ts";
    const absoluteReadModelPath = path.join(fixtureRoot, readModelPath);
    fs.mkdirSync(path.dirname(absoluteReadModelPath), { recursive: true });
    fs.writeFileSync(
      absoluteReadModelPath,
      [
        "export const unrelatedPackageVisibility = {",
        "  fileHashes: {",
        `    "${selectedFiles[0]}":`,
        '      "stale-unrelated-copy",',
        "  },",
        "};",
        "",
        "export const dataFetchingCachePackageVisibility = {",
        '  packageId: "tanstack/query",',
        `  packageReceiptPath: "${receiptPath}",`,
        "  selectedSurfaces: [",
        "    {",
        "      fileHashes: {",
        ...selectedFiles.flatMap((selectedFile) => [
          `        "${selectedFile}":`,
          '          "stale",',
        ]),
        "      },",
        "    },",
        "    {",
        "      fileHashes: {",
        ...selectedFiles.flatMap((selectedFile) => [
          `        "${selectedFile}":`,
          '          "stale",',
        ]),
        "      },",
        "    },",
        "  ],",
        "  statusVocabulary: [],",
        "};",
        "",
      ].join("\n"),
    );

    const stale = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    const staleReport = JSON.parse(stale.stdout);
    assert.equal(staleReport.schema, "dx.forge.package.receipt_hash_refresh");
    assert.equal(staleReport.package_id, "tanstack/query");
    assert.equal(staleReport.official_package_name, "Data Fetching & Cache");
    assert.equal(staleReport.status, "missing");
    assert.ok(staleReport.stale_file_count > 0);
    assert.ok(staleReport.missing_file_count > 0);
    assert.equal(staleReport.runtime_execution, false);
    assert.equal(staleReport.secret_access, false);
    assert.equal(
      staleReport.zed_visibility,
      "data-fetching-cache:receipt-hash-refresh",
    );

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);
    assert.match(write.stdout, /Data Fetching & Cache receipt hashes updated/);

    const fresh = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.equal(fresh.status, 0, fresh.stdout + fresh.stderr);
    const freshReport = JSON.parse(fresh.stdout);
    assert.equal(freshReport.status, "current");
    assert.equal(freshReport.tracked_file_count, selectedFiles.length);
    assert.equal(freshReport.source_guard_runbook_fixture, runbookFixturePath);
    assert.equal(
      freshReport.preview_manifest_materializer,
      previewManifestMaterializerPath,
    );
    assert.ok(
      freshReport.tracked_files.includes(previewManifestMaterializerPath),
      "Data Fetching & Cache helper report must track the preview-manifest materializer",
    );
    assert.ok(
      freshReport.files.some(
        (entry) => entry.path === previewManifestMaterializerPath,
      ),
      "Data Fetching & Cache helper report must include materializer file status",
    );
    assert.equal(freshReport.stale_file_count, 0);
    assert.equal(freshReport.missing_file_count, 0);

    const refreshedReceipt = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, receiptPath), "utf8"),
    );
    const refreshedStatus = JSON.parse(
      fs.readFileSync(path.join(fixtureRoot, packageStatusPath), "utf8"),
    );
    const readModelText = fs.readFileSync(absoluteReadModelPath, "utf8");
    const dataFetchingReadModel = readModelExport(
      readModelText,
      "dataFetchingCachePackageVisibility",
    );

    for (const selectedFile of selectedFiles) {
      const refreshedHash = refreshedReceipt.file_hashes[selectedFile];
      assert.match(refreshedHash, /^[a-f0-9]{64}$/);
      assert.equal(
        refreshedReceipt.dx_check_visibility.monitored_surfaces[0].file_hashes[
          selectedFile
        ],
        refreshedHash,
      );
      for (const surface of refreshedStatus.package_lane_visibility[0]
        .selected_surfaces) {
        if (
          Object.prototype.hasOwnProperty.call(
            surface.file_hashes ?? {},
            selectedFile,
          )
        ) {
          assert.equal(surface.file_hashes[selectedFile], refreshedHash);
        }
      }
      assert.match(
        dataFetchingReadModel,
        new RegExp(refreshedHash),
        `${selectedFile} should be refreshed in the Data Fetching read model`,
      );
    }
    assert.match(readModelText, /stale-unrelated-copy/);
    assert.doesNotMatch(dataFetchingReadModel, /stale-unrelated-copy/);

    assert.deepEqual(
      refreshedStatus.package_lane_visibility[0].receipt_hash_refresh,
      {
        schema: "dx.forge.package.receipt_hash_refresh",
        status: "current",
        helper_path:
          "examples/template/data-fetching-cache-receipt-hashes.ts",
        check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/data-fetching-cache-receipt-hashes.ts --check",
        write_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/data-fetching-cache-receipt-hashes.ts --write",
        json_check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/data-fetching-cache-receipt-hashes.ts --check --json",
        source_guard_runbook_fixture: runbookFixturePath,
        preview_manifest_materializer: previewManifestMaterializerPath,
        receipt_path: receiptPath,
        hash_algorithm: "sha256",
        tracked_file_count: selectedFiles.length,
        tracked_files: selectedFiles,
        stale_file_count: 0,
        missing_file_count: 0,
        runtime_execution: false,
        secret_access: false,
        zed_visibility: "data-fetching-cache:receipt-hash-refresh",
        runtime_limitations: [
          "SOURCE-ONLY: this helper checks local Data Fetching & Cache receipt hash freshness only.",
          "SOURCE-OWNED ROUTE: the App Router readiness route executes the template cache model and dry-run cache action receipts without node_modules or network calls.",
          "ADAPTER-BOUNDARY: live QueryClient execution, production fetchers, persistence, broadcast sync, dependency installation, and browser proof stay app-owned.",
        ],
      },
    );
    assert.ok(
      refreshedStatus.zed_receipt_surfaces.includes(
        "data-fetching-cache:receipt-hash-refresh",
      ),
    );
    assert.match(readModelText, /receiptHashRefresh: \{/);
    assert.match(readModelText, /data-fetching-cache:receipt-hash-refresh/);
    assert.match(
      dataFetchingReadModel,
      /sourceGuardRunbookFixture:\s*\n\s*"docs\/packages\/data-fetching-cache\.source-guard-runbook\.json"/,
    );
    assert.match(
      dataFetchingReadModel,
      /previewManifestMaterializer:\s*\n\s*"tools\/launch\/materialize-www-template\.ts"/,
    );
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("Data Fetching & Cache current receipt tracks the source-guard runbook fixture", () => {
  const receiptPath =
    "examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json";
  const receipt = JSON.parse(fs.readFileSync(path.join(root, receiptPath), "utf8"));
  const packageStatus = JSON.parse(
    fs.readFileSync(
      path.join(root, "examples/template/.dx/forge/package-status.json"),
      "utf8",
    ),
  );
  const readModelText = fs.readFileSync(
    path.join(root, "examples/template/forge-package-status-read-model.ts"),
    "utf8",
  );
  const packageDoc = fs.readFileSync(
    path.join(root, "docs/packages/tanstack-query.md"),
    "utf8",
  );
  const helperReportResult = runHelper(["--check", "--json"]);

  assert.equal(helperReportResult.status, 0, helperReportResult.stdout + helperReportResult.stderr);
  const helperReport = JSON.parse(helperReportResult.stdout);
  assert.equal(helperReport.source_guard_runbook_fixture, runbookFixturePath);
  assert.equal(
    helperReport.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  assert.ok(
    Object.prototype.hasOwnProperty.call(receipt.file_hashes, runbookFixturePath),
    "Data Fetching & Cache receipt must hash the package-owned runbook fixture",
  );
  assert.ok(
    Object.prototype.hasOwnProperty.call(
      receipt.file_hashes,
      previewManifestMaterializerPath,
    ),
    "Data Fetching & Cache receipt must hash the shared preview-manifest materializer",
  );
  assert.ok(
    receipt.source_files.includes(previewManifestMaterializerPath),
    "Data Fetching & Cache receipt must list the preview-manifest materializer as source",
  );
  assert.ok(
    receipt.dx_check_visibility.monitored_surfaces.some(
      (surface) =>
        surface.id === "data-fetching-cache-preview-manifest-materializer" &&
        surface.file_hashes?.[previewManifestMaterializerPath] ===
          receipt.file_hashes[previewManifestMaterializerPath],
    ),
    "Data Fetching & Cache receipt must monitor the materializer hash",
  );
  assert.equal(helperReport.tracked_file_count, Object.keys(receipt.file_hashes).length);
  assert.ok(
    helperReport.tracked_files.includes(previewManifestMaterializerPath),
    "Data Fetching & Cache helper must list the materializer in tracked_files",
  );
  assert.ok(
    helperReport.files.some(
      (entry) => entry.path === previewManifestMaterializerPath,
    ),
    "Data Fetching & Cache helper must report materializer file freshness",
  );

  const visibility = packageStatus.package_lane_visibility.find(
    (row) => row.package_id === "tanstack/query",
  );
  assert.ok(visibility, "Data Fetching & Cache package-status row missing");
  assert.equal(
    visibility.receipt_hash_refresh.source_guard_runbook_fixture,
    runbookFixturePath,
  );
  assert.equal(
    visibility.receipt_hash_refresh.preview_manifest_materializer,
    previewManifestMaterializerPath,
  );
  for (const surface of visibility.selected_surfaces) {
    assert.equal(
      surface.file_hashes[runbookFixturePath],
      receipt.file_hashes[runbookFixturePath],
      `${surface.surface_id} should mirror the runbook fixture hash`,
    );
    assert.equal(
      surface.file_hashes[previewManifestMaterializerPath],
      receipt.file_hashes[previewManifestMaterializerPath],
      `${surface.surface_id} should mirror the materializer hash`,
    );
  }

  const dataFetchingReadModel = readModelExport(
    readModelText,
    "dataFetchingCachePackageVisibility",
  );
  assert.match(
    dataFetchingReadModel,
    /sourceGuardRunbookFixture:\s*\n\s*"docs\/packages\/data-fetching-cache\.source-guard-runbook\.json"/,
  );
  assert.match(
    dataFetchingReadModel,
    /previewManifestMaterializer:\s*\n\s*"tools\/launch\/materialize-www-template\.ts"/,
  );
  assert.match(packageDoc, /source-guard runbook fixture is hash-tracked/);
  assert.match(packageDoc, /preview_manifest_materializer/);
});

test("Data Fetching & Cache docs publish the hash refresh command without claiming runtime proof", () => {
  const packageDoc = fs.readFileSync(
    path.join(root, "docs/packages/tanstack-query.md"),
    "utf8",
  );

  assert.match(
    packageDoc,
    /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/data-fetching-cache-receipt-hashes\.ts --check/,
  );
  assert.match(packageDoc, /--write/);
  assert.match(packageDoc, /does not run live QueryClient execution/i);
});
