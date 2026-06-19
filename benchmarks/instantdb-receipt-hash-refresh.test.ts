const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath = path.join(
  root,
  "examples/template/realtime-app-database-receipt-hashes.ts",
);

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

test("Realtime App Database receipt hash helper refreshes receipt, package-status, and read model hashes", () => {
  assert.ok(
    fs.existsSync(helperPath),
    "Realtime App Database hash helper is missing",
  );

  const fixtureRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-realtime-app-database-hashes-"),
  );
  try {
    const sourceGuardRunbookFixture =
      "docs/packages/instantdb-react.source-guard-runbook.json";
    const selectedFiles = [
      "core/src/ecosystem/forge_instantdb.rs",
      "examples/template/instantdb-status.tsx",
      "tools/launch/runtime-template/pages/index.html",
      "tools/launch/runtime-template/assets/launch-runtime.ts",
      "examples/dashboard/src/lib/instantdbDashboard.ts",
      "examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx",
      sourceGuardRunbookFixture,
    ];

    for (const selectedFile of selectedFiles) {
      const selectedFilePath = path.join(fixtureRoot, selectedFile);
      fs.mkdirSync(path.dirname(selectedFilePath), { recursive: true });
      if (selectedFile === sourceGuardRunbookFixture) {
        writeJson(selectedFilePath, {
          schema: "dx.forge.package.source_guard_runbook_fixture",
          fixture_path: sourceGuardRunbookFixture,
          package: {
            official_package_name: "Realtime App Database",
            package_id: "instantdb/react",
            upstream_package: "@instantdb/react",
          },
          receipt: {
            hash_helper:
              "examples/template/realtime-app-database-receipt-hashes.ts",
            zed_visibility: "realtime-app-database:receipt-hash-refresh",
          },
          runtime_proof: false,
        });
      } else {
        fs.writeFileSync(
          selectedFilePath,
          `export const realtimeAppDatabaseHashFixture = ${JSON.stringify(
            selectedFile,
          )};\n`,
        );
      }
    }

    const receiptPath =
      "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json";
    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.package_dashboard_workflow_receipt",
      package_id: "instantdb/react",
      package_name: "Realtime App Database",
      upstream_package: "@instantdb/react",
      upstream_version: "0.0.0",
      source_mirror: "G:/WWW/inspirations/instantdb",
      hash_algorithm: "sha256",
      file_hashes: Object.fromEntries(
        selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
      ),
      dx_check_visibility: {
        schema: "dx.forge.package.dx_check_visibility",
        selected_surfaces: [
          {
            surface_id: "instantdb-runtime-dashboard-workflow",
            status: "present",
            hash_algorithm: "sha256",
            file_hashes: Object.fromEntries(
              selectedFiles.slice(2, 4).map((selectedFile) => [
                selectedFile,
                "stale",
              ]),
            ),
          },
          {
            surface_id: "dashboard-instantdb-workflow",
            status: "present",
            hash_algorithm: "sha256",
            file_hashes: Object.fromEntries(
              selectedFiles.slice(4).map((selectedFile) => [
                selectedFile,
                "stale",
              ]),
            ),
          },
          {
            surface_id: "realtime-app-database-source-guard-runbook",
            status: "present",
            hash_algorithm: "sha256",
            file_hashes: {
              [sourceGuardRunbookFixture]: "stale",
            },
          },
        ],
      },
    });

    const packageStatusPath =
      "examples/template/.dx/forge/package-status.json";
    writeJson(path.join(fixtureRoot, packageStatusPath), {
      zed_receipt_surfaces: [
        "instantdb-runtime-dashboard-workflow",
        "dashboard-instantdb-workflow",
      ],
      package_lane_visibility: [
        {
          official_package_name: "Realtime App Database",
          package_id: "instantdb/react",
          package_receipt_path: receiptPath,
          selected_surfaces: [
            {
              surface_id: "instantdb-runtime-dashboard-workflow",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                selectedFiles.slice(2, 4).map((selectedFile) => [
                  selectedFile,
                  "stale",
                ]),
              ),
            },
            {
              surface_id: "dashboard-instantdb-workflow",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                selectedFiles.slice(4).map((selectedFile) => [
                  selectedFile,
                  "stale",
                ]),
              ),
            },
            {
              surface_id: "sync-table-events",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                selectedFiles.slice(0, 2).map((selectedFile) => [
                  selectedFile,
                  "stale",
                ]),
              ),
            },
            {
              surface_id: "realtime-app-database-source-guard-runbook",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                [sourceGuardRunbookFixture]: "stale",
              },
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
        "export const unrelatedPackageVisibility = {",
        "  fileHashes: {",
        `    "${selectedFiles[0]}":`,
        '      "stale-unrelated-copy",',
        "  },",
        "};",
        "",
        "const realtimeAppDatabaseFileHashes = {",
        ...selectedFiles.flatMap((selectedFile) => [
          `  "${selectedFile}":`,
          '    "stale",',
        ]),
        "} as const;",
        "",
        "export const realtimeAppDatabasePackageVisibility = {",
        '  packageId: "instantdb/react",',
        `  packageReceiptPath: "${receiptPath}",`,
        "  selectedSurfaces: [",
        "    {",
        "      fileHashes: {",
        ...selectedFiles.slice(2, 4).flatMap((selectedFile) => [
          `        "${selectedFile}":`,
          `          realtimeAppDatabaseFileHashes["${selectedFile}"],`,
        ]),
        "      },",
        "    },",
        "    {",
        '      surfaceId: "realtime-app-database-source-guard-runbook",',
        "      fileHashes: {",
        `        "${sourceGuardRunbookFixture}":`,
        `          realtimeAppDatabaseFileHashes["${sourceGuardRunbookFixture}"],`,
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
    assert.equal(staleReport.package_id, "instantdb/react");
    assert.equal(staleReport.official_package_name, "Realtime App Database");
    assert.equal(staleReport.upstream_package, "@instantdb/react");
    assert.equal(staleReport.upstream_version, "0.0.0");
    assert.equal(staleReport.source_mirror, "G:/WWW/inspirations/instantdb");
    assert.equal(staleReport.status, "missing");
    assert.ok(staleReport.stale_file_count > 0);
    assert.ok(staleReport.missing_file_count > 0);
    assert.equal(staleReport.runtime_execution, false);
    assert.equal(staleReport.secret_access, false);
    assert.equal(
      staleReport.zed_visibility,
      "realtime-app-database:receipt-hash-refresh",
    );

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);
    assert.match(write.stdout, /Realtime App Database receipt hashes updated/);

    const fresh = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.equal(fresh.status, 0, fresh.stdout + fresh.stderr);
    const freshReport = JSON.parse(fresh.stdout);
    assert.equal(freshReport.status, "current");
    assert.equal(freshReport.tracked_file_count, selectedFiles.length);
    assert.equal(
      freshReport.source_guard_runbook_fixture,
      sourceGuardRunbookFixture,
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
    const readModelBlock = readModelExport(
      readModelText,
      "realtimeAppDatabasePackageVisibility",
    );

    for (const selectedFile of selectedFiles) {
      const refreshedHash = refreshedReceipt.file_hashes[selectedFile];
      assert.match(refreshedHash, /^[a-f0-9]{64}$/);
      assert.match(readModelText, new RegExp(refreshedHash));
    }
    assert.match(readModelText, /stale-unrelated-copy/);
    assert.doesNotMatch(readModelBlock, /stale-unrelated-copy/);

    assert.deepEqual(
      refreshedStatus.package_lane_visibility[0].receipt_hash_refresh,
      {
        schema: "dx.forge.package.receipt_hash_refresh",
        status: "current",
        helper_path:
          "examples/template/realtime-app-database-receipt-hashes.ts",
        check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/realtime-app-database-receipt-hashes.ts --check",
        write_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/realtime-app-database-receipt-hashes.ts --write",
        json_check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/realtime-app-database-receipt-hashes.ts --check --json",
        source_guard_runbook_fixture: sourceGuardRunbookFixture,
        receipt_path: receiptPath,
        package_status_path:
          "examples/template/.dx/forge/package-status.json",
        read_model_path:
          "examples/template/forge-package-status-read-model.ts",
        hash_algorithm: "sha256",
        tracked_file_count: selectedFiles.length,
        tracked_files: selectedFiles,
        current_files: selectedFiles,
        stale_files: [],
        missing_files: [],
        stale_mirror_files: [],
        missing_mirror_files: [],
        mirror_problem_count: 0,
        stale_file_count: 0,
        missing_file_count: 0,
        runtime_execution: false,
        secret_access: false,
        zed_visibility: "realtime-app-database:receipt-hash-refresh",
        runtime_limitations: [
          "SOURCE-ONLY: this helper checks local Realtime App Database receipt hash freshness only.",
          "ADAPTER-BOUNDARY: hosted Instant app provisioning, rules, auth policy, storage, streams, Sync Table runtime validation, dependency installation, and browser proof stay app-owned.",
        ],
      },
    );
    assert.ok(
      refreshedStatus.zed_receipt_surfaces.includes(
        "realtime-app-database:receipt-hash-refresh",
      ),
    );
    assert.ok(
      refreshedStatus.package_lane_visibility[0].selected_surfaces.some(
        (surface) =>
          surface.surface_id === "realtime-app-database-source-guard-runbook" &&
          surface.file_hashes?.[sourceGuardRunbookFixture],
      ),
    );
    assert.match(readModelText, /receiptHashRefresh: \{/);
    assert.match(
      readModelText,
      /sourceGuardRunbookFixture: "docs\/packages\/instantdb-react\.source-guard-runbook\.json"/,
    );
    assert.match(readModelText, /realtime-app-database:receipt-hash-refresh/);
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("Realtime App Database runbook fixture is hash-backed in live receipt metadata", () => {
  const sourceGuardRunbookFixture =
    "docs/packages/instantdb-react.source-guard-runbook.json";
  const receiptPath =
    "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json";
  const packageStatusPath =
    "examples/template/.dx/forge/package-status.json";
  const readModelPath =
    "examples/template/forge-package-status-read-model.ts";

  const receipt = JSON.parse(
    fs.readFileSync(path.join(root, receiptPath), "utf8"),
  );
  const packageStatus = JSON.parse(
    fs.readFileSync(path.join(root, packageStatusPath), "utf8"),
  );
  const readModelText = fs.readFileSync(path.join(root, readModelPath), "utf8");
  const realtimeStatus = packageStatus.package_lane_visibility.find(
    (entry) => entry.package_id === "instantdb/react",
  );

  assert.equal(receipt.file_hashes[sourceGuardRunbookFixture]?.length, 64);
  assert.ok(
    receipt.dx_check_visibility.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "realtime-app-database-source-guard-runbook" &&
        surface.file_hashes?.[sourceGuardRunbookFixture],
    ),
  );
  assert.equal(
    realtimeStatus.receipt_hash_refresh.source_guard_runbook_fixture,
    sourceGuardRunbookFixture,
  );
  assert.ok(
    realtimeStatus.selected_surfaces.some(
      (surface) =>
        surface.surface_id === "realtime-app-database-source-guard-runbook" &&
        surface.file_hashes?.[sourceGuardRunbookFixture],
    ),
  );
  assert.match(
    readModelText,
    /sourceGuardRunbookFixture: "docs\/packages\/instantdb-react\.source-guard-runbook\.json"/,
  );
  assert.match(
    readModelText,
    /"docs\/packages\/instantdb-react\.source-guard-runbook\.json":/,
  );
});

test("Realtime App Database docs publish the hash refresh command without claiming runtime proof", () => {
  const packageDoc = fs.readFileSync(
    path.join(root, "docs/packages/instantdb-react.md"),
    "utf8",
  );

  assert.match(
    packageDoc,
    /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/realtime-app-database-receipt-hashes\.ts --check/,
  );
  assert.match(packageDoc, /--write/);
  assert.match(packageDoc, /does not run hosted Instant runtime proof/i);
});
