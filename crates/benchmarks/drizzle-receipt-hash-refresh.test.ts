const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath = path.join(
  root,
  "examples/template/database-orm-receipt-hashes.ts",
);
const helperRelativePath =
  "examples/template/database-orm-receipt-hashes.ts";
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json";
const packageStatusPath =
  "examples/template/.dx/forge/package-status.json";
const readModelPath =
  "examples/template/forge-package-status-read-model.ts";
const previewManifestMaterializerPath =
  "tools/launch/materialize-www-template.ts";
const previewManifestMaterializerSurfaceId =
  "database-orm-preview-manifest-materializer";
const mirrorDriftFixturePath =
  "docs/packages/database-orm.mirror-drift.fixture.json";
const selectedDatabaseOrmSourceFiles = [
  "core/src/ecosystem/forge_drizzle.rs",
  "examples/template/drizzle-query-proof.tsx",
  "examples/template/data-status.tsx",
  "tools/launch/runtime-template/pages/index.html",
  "tools/launch/runtime-template/assets/launch-runtime.ts",
  "docs/packages/database-orm.source-guard-runbook.json",
  "examples/template/db/drizzle/schema.ts",
  "examples/template/db/drizzle/metadata.ts",
  "examples/template/db/drizzle/README.md",
  "examples/template/server/database-orm/readiness.ts",
  "examples/template/app/api/database-orm/readiness/route.ts",
];
const lockBackedDatabaseOrmSourceFiles = [
  "examples/template/db/drizzle/schema.ts",
  "examples/template/db/drizzle/metadata.ts",
  "examples/template/db/drizzle/README.md",
  "examples/template/server/database-orm/readiness.ts",
  "examples/template/app/api/database-orm/readiness/route.ts",
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

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function copyFixtureFile(tempRoot, relativePath) {
  const target = path.join(tempRoot, relativePath);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(path.join(root, relativePath), target);
}

function materializerDriftFixture() {
  const tempRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-database-orm-receipt-drift-"),
  );
  const receipt = readJson(receiptPath);
  for (const relativePath of Object.keys(receipt.file_hashes)) {
    copyFixtureFile(tempRoot, relativePath);
  }
  copyFixtureFile(tempRoot, receiptPath);
  copyFixtureFile(tempRoot, packageStatusPath);
  copyFixtureFile(tempRoot, readModelPath);

  const refresh = runHelper(["--root", tempRoot, "--write"]);
  if (refresh.status !== 0) {
    throw new Error(refresh.stdout + refresh.stderr);
  }

  fs.appendFileSync(
    path.join(tempRoot, previewManifestMaterializerPath),
    "\n// Database ORM helper drift fixture.\n",
  );

  return tempRoot;
}

function currentDatabaseOrmFixture() {
  const tempRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-database-orm-current-"),
  );
  const receipt = readJson(receiptPath);
  for (const relativePath of Object.keys(receipt.file_hashes)) {
    copyFixtureFile(tempRoot, relativePath);
  }
  copyFixtureFile(tempRoot, receiptPath);
  copyFixtureFile(tempRoot, packageStatusPath);
  copyFixtureFile(tempRoot, readModelPath);

  const refresh = runHelper(["--root", tempRoot, "--write"]);
  if (refresh.status !== 0) {
    throw new Error(refresh.stdout + refresh.stderr);
  }

  return tempRoot;
}

function removeReadModelSurfaceBlock(text, surfaceId) {
  const idIndex = text.indexOf(`surfaceId: "${surfaceId}"`);
  assert.notEqual(idIndex, -1, `${surfaceId} read-model block is missing`);

  const blockStart = text.lastIndexOf("    {", idIndex);
  assert.notEqual(blockStart, -1, `${surfaceId} read-model block start is missing`);

  const blockEndMarker = "\n    },";
  const blockEnd = text.indexOf(blockEndMarker, idIndex);
  assert.notEqual(blockEnd, -1, `${surfaceId} read-model block end is missing`);

  return text.slice(0, blockStart) + text.slice(blockEnd + blockEndMarker.length);
}

test("Database ORM receipt hash helper refreshes receipt, package-status, and read model hashes", () => {
  assert.ok(fs.existsSync(helperPath), "Database ORM hash helper is missing");

  const fixtureRoot = fs.mkdtempSync(
    path.join(os.tmpdir(), "dx-database-orm-hashes-"),
  );
  try {
    const selectedFiles = [
      "core/src/ecosystem/forge_drizzle.rs",
      "examples/template/drizzle-query-proof.tsx",
      "examples/template/data-status.tsx",
      "tools/launch/runtime-template/pages/index.html",
      "tools/launch/runtime-template/assets/launch-runtime.ts",
      "docs/packages/database-orm.source-guard-runbook.json",
      "tools/launch/materialize-www-template.ts",
      ...lockBackedDatabaseOrmSourceFiles,
    ];
    for (const selectedFile of selectedFiles) {
      const selectedFilePath = path.join(fixtureRoot, selectedFile);
      fs.mkdirSync(path.dirname(selectedFilePath), { recursive: true });
      const contents = selectedFile.endsWith(".json")
        ? JSON.stringify(
            {
              schema: "dx.forge.package.source_guard_runbook_fixture",
              package: {
                official_package_name: "Database ORM",
                package_id: "db/drizzle-sqlite",
              },
            },
            null,
            2,
          ) + "\n"
        : `export const databaseOrmHashFixture = ${JSON.stringify(selectedFile)};\n`;
      fs.writeFileSync(selectedFilePath, contents);
    }

    const receiptPath =
      "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json";
    writeJson(path.join(fixtureRoot, receiptPath), {
      schema: "dx.forge.package_dashboard_workflow_receipt",
      package_id: "db/drizzle-sqlite",
      official_package_name: "Database ORM",
      upstream_package: "drizzle-orm",
      upstream_version: "0.45.3",
      source_mirror: "G:/WWW/inspirations/drizzle-orm",
      hash_algorithm: "sha256",
      file_hashes: Object.fromEntries(
        selectedFiles.map((selectedFile) => [selectedFile, "stale"]),
      ),
      dx_check_visibility: {
        schema: "dx.forge.package.dx_check_visibility",
        monitored_surfaces: [
          {
            id: "drizzle-replica-routing",
            hash_algorithm: "sha256",
            file_hashes: {
              "core/src/ecosystem/forge_drizzle.rs": "stale",
            },
          },
          {
            id: "drizzle-launch-dashboard-workflow",
            hash_algorithm: "sha256",
            file_hashes: Object.fromEntries(
              selectedFiles
                .filter((selectedFile) => selectedFile !== "core/src/ecosystem/forge_drizzle.rs")
                .map((selectedFile) => [selectedFile, "stale"]),
            ),
          },
          {
            id: "database-orm-source-guard-runbook",
            hash_algorithm: "sha256",
            file_hashes: {
              "docs/packages/database-orm.source-guard-runbook.json": "stale",
            },
          },
          {
            id: "database-orm-preview-manifest-materializer",
            hash_algorithm: "sha256",
            file_hashes: {
              "tools/launch/materialize-www-template.ts": "stale",
            },
          },
          {
            id: "database-orm-lock-backed-source",
            hash_algorithm: "sha256",
            file_hashes: Object.fromEntries(
              lockBackedDatabaseOrmSourceFiles.map((selectedFile) => [
                selectedFile,
                "stale",
              ]),
            ),
          },
        ],
      },
    });

    const packageStatusPath =
      "examples/template/.dx/forge/package-status.json";
    writeJson(path.join(fixtureRoot, packageStatusPath), {
      zed_receipt_surfaces: [
        "database-orm:drizzle-replica-routing",
        "database-orm:drizzle-launch-dashboard-workflow",
      ],
      package_lane_visibility: [
        {
          official_package_name: "Database ORM",
          package_id: "db/drizzle-sqlite",
          upstream_package: "drizzle-orm",
          upstream_version: "0.45.3",
          source_mirror: "G:/WWW/inspirations/drizzle-orm",
          package_receipt_path: receiptPath,
          selected_surfaces: [
            {
              surface_id: "drizzle-replica-routing",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                "core/src/ecosystem/forge_drizzle.rs": "stale",
              },
            },
            {
              surface_id: "drizzle-launch-dashboard-workflow",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                selectedFiles
                  .filter((selectedFile) => selectedFile !== "core/src/ecosystem/forge_drizzle.rs")
                  .map((selectedFile) => [selectedFile, "stale"]),
              ),
            },
            {
              surface_id: "database-orm-source-guard-runbook",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                "docs/packages/database-orm.source-guard-runbook.json": "stale",
              },
            },
            {
              surface_id: "database-orm-preview-manifest-materializer",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: {
                "tools/launch/materialize-www-template.ts": "stale",
              },
            },
            {
              surface_id: "database-orm-lock-backed-source",
              receipt_path: receiptPath,
              hash_algorithm: "sha256",
              file_hashes: Object.fromEntries(
                lockBackedDatabaseOrmSourceFiles.map((selectedFile) => [
                  selectedFile,
                  "stale",
                ]),
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
        "export const unrelatedPackageVisibility = {",
        "  fileHashes: {",
        `    "${selectedFiles[0]}":`,
        '      "stale-unrelated-copy",',
        "  },",
        "};",
        "",
        "export const databaseOrmPackageVisibility = {",
        '  packageId: "db/drizzle-sqlite",',
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
        '        "docs/packages/database-orm.source-guard-runbook.json":',
        '          "stale",',
        "      },",
        "    },",
        "  ],",
        "  statusVocabulary: [],",
        "};",
        "",
        "export const paymentsPackageVisibility = {",
        "  receiptHashRefresh: {",
        '    schema: "dx.forge.package.receipt_hash_refresh",',
        '    helperPath: "examples/template/payments-receipt-hashes.ts",',
        '    zedVisibility: "payments:receipt-hash-refresh",',
        "  },",
        "};",
        "",
      ].join("\n"),
    );

    const stale = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    const staleReport = JSON.parse(stale.stdout);
    assert.equal(staleReport.package_id, "db/drizzle-sqlite");
    assert.equal(staleReport.official_package_name, "Database ORM");
    assert.equal(staleReport.upstream_package, "drizzle-orm");
    assert.equal(staleReport.upstream_version, "0.45.3");
    assert.equal(staleReport.source_mirror, "G:/WWW/inspirations/drizzle-orm");
    assert.equal(staleReport.status, "stale");
    assert.equal(staleReport.runtime_execution, false);
    assert.equal(staleReport.secret_access, false);
    assert.equal(staleReport.runs_package_install, false);
    assert.equal(staleReport.zed_visibility, "database-orm:receipt-hash-refresh");
    assert.equal(
      staleReport.source_guard_runbook_fixture,
      "docs/packages/database-orm.source-guard-runbook.json",
    );
    assert.equal(
      staleReport.preview_manifest_materializer,
      "tools/launch/materialize-www-template.ts",
    );

    const write = runHelper(["--root", fixtureRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);
    assert.match(write.stdout, /Database ORM receipt hashes updated/);

    const fresh = runHelper(["--root", fixtureRoot, "--check", "--json"]);
    assert.equal(fresh.status, 0, fresh.stdout + fresh.stderr);
    const freshReport = JSON.parse(fresh.stdout);
    assert.equal(freshReport.schema, "dx.forge.package.receipt_hash_refresh");
    assert.equal(freshReport.status, "current");
    assert.equal(freshReport.tracked_file_count, selectedFiles.length);
    for (const selectedFile of lockBackedDatabaseOrmSourceFiles) {
      assert.ok(
        freshReport.tracked_files.includes(selectedFile),
        `${selectedFile} should be tracked as Database ORM lock-backed source`,
      );
    }
    assert.equal(freshReport.stale_file_count, 0);
    assert.equal(freshReport.missing_file_count, 0);

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
    assert.equal(
      refreshedReceipt.dx_check_visibility.monitored_surfaces[0].file_hashes[
        "core/src/ecosystem/forge_drizzle.rs"
      ],
      refreshedReceipt.file_hashes["core/src/ecosystem/forge_drizzle.rs"],
    );
    assert.equal(
      refreshedStatus.package_lane_visibility[0].selected_surfaces[1].file_hashes[
        "tools/launch/runtime-template/pages/index.html"
      ],
      refreshedReceipt.file_hashes[
        "tools/launch/runtime-template/pages/index.html"
      ],
    );
    assert.ok(
      refreshedStatus.package_lane_visibility[0].selected_surfaces.some(
        (surface) =>
          surface.surface_id === "database-orm-lock-backed-source" &&
          lockBackedDatabaseOrmSourceFiles.every(
            (selectedFile) =>
              surface.file_hashes[selectedFile] ===
              refreshedReceipt.file_hashes[selectedFile],
          ),
      ),
      "Database ORM lock-backed source surface should mirror route, readiness, schema, metadata, and README hashes",
    );
    assert.deepEqual(
      refreshedStatus.package_lane_visibility[0].receipt_hash_refresh,
      {
        schema: "dx.forge.package.receipt_hash_refresh",
        status: "current",
        helper_path:
          "examples/template/database-orm-receipt-hashes.ts",
        check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/database-orm-receipt-hashes.ts --check",
        write_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/database-orm-receipt-hashes.ts --write",
        json_check_command:
          "node tools/launch/run-template-receipt-helper.js examples/template/database-orm-receipt-hashes.ts --check --json",
        source_guard_runbook_fixture:
          "docs/packages/database-orm.source-guard-runbook.json",
        preview_manifest_materializer:
          "tools/launch/materialize-www-template.ts",
        receipt_path: receiptPath,
        hash_algorithm: "sha256",
        tracked_file_count: selectedFiles.length,
        stale_file_count: 0,
        missing_file_count: 0,
        tracked_files: selectedFiles,
        current_files: selectedFiles,
        stale_files: [],
        missing_files: [],
        stale_mirror_files: [],
        missing_mirror_files: [],
        mirror_problem_count: 0,
        runtime_execution: false,
        secret_access: false,
        zed_visibility: "database-orm:receipt-hash-refresh",
        runtime_limitations: [
          "SOURCE-ONLY: this helper checks local Database ORM receipt hash freshness only.",
          "ADAPTER-BOUNDARY: SQLite files, better-sqlite3 runtime installation, migration rollout, authorization, and replica health stay app-owned.",
        ],
      },
    );
    assert.match(readModelText, /receiptHashRefresh: \{/);
    assert.match(
      readModelText,
      /sourceGuardRunbookFixture: "docs\/packages\/database-orm\.source-guard-runbook\.json"/,
    );
    assert.match(
      readModelText,
      /previewManifestMaterializer: "tools\/launch\/materialize-www-template\.ts"/,
    );
    assert.match(
      readModelText,
      /zedVisibility: "database-orm:receipt-hash-refresh"/,
    );
    assert.match(readModelText, /database-orm-lock-backed-source/);
    assert.match(readModelText, /app\/api\/database-orm\/readiness\/route\.ts/);
    assert.match(readModelText, /server\/database-orm\/readiness\.ts/);
    assert.match(readModelText, /db\/drizzle\/metadata\.ts/);
    assert.match(readModelText, /db\/drizzle\/README\.md/);
    assert.match(
      readModelText,
      /payments-receipt-hashes\.ts/,
      "Database ORM helper must not rewrite the next package lane's hash refresh block",
    );
    assert.deepEqual(refreshedStatus.zed_receipt_surfaces, [
      "database-orm:drizzle-replica-routing",
      "database-orm:drizzle-launch-dashboard-workflow",
      "database-orm:receipt-hash-refresh",
    ]);
  } finally {
    fs.rmSync(fixtureRoot, { recursive: true, force: true });
  }
});

test("Database ORM helper recreates missing preview-manifest materializer surfaces", () => {
  const tempRoot = currentDatabaseOrmFixture();
  try {
    const absoluteReceiptPath = path.join(tempRoot, receiptPath);
    const receipt = JSON.parse(fs.readFileSync(absoluteReceiptPath, "utf8"));
    receipt.dx_check_visibility.monitored_surfaces =
      receipt.dx_check_visibility.monitored_surfaces.filter(
        (surface) => surface.id !== previewManifestMaterializerSurfaceId,
      );
    writeJson(absoluteReceiptPath, receipt);

    const absolutePackageStatusPath = path.join(tempRoot, packageStatusPath);
    const status = JSON.parse(
      fs.readFileSync(absolutePackageStatusPath, "utf8"),
    );
    const visibility = status.package_lane_visibility.find(
      (entry) => entry.package_id === "db/drizzle-sqlite",
    );
    visibility.selected_surfaces = visibility.selected_surfaces.filter(
      (surface) =>
        surface.surface_id !== previewManifestMaterializerSurfaceId,
    );
    writeJson(absolutePackageStatusPath, status);

    const absoluteReadModelPath = path.join(tempRoot, readModelPath);
    const readModelWithoutPreviewSurface = removeReadModelSurfaceBlock(
      fs.readFileSync(absoluteReadModelPath, "utf8"),
      previewManifestMaterializerSurfaceId,
    );
    fs.writeFileSync(absoluteReadModelPath, readModelWithoutPreviewSurface);

    const stale = runHelper(["--root", tempRoot, "--check", "--json"]);
    assert.notEqual(stale.status, 0, stale.stdout + stale.stderr);
    const staleReport = JSON.parse(stale.stdout);
    assert.equal(staleReport.status, "missing");
    assert.ok(
      staleReport.missing_mirror_files.includes(previewManifestMaterializerPath),
      "missing preview-manifest mirrors should be attributed to the materializer",
    );

    const write = runHelper(["--root", tempRoot, "--write"]);
    assert.equal(write.status, 0, write.stdout + write.stderr);

    const refreshedReceipt = JSON.parse(
      fs.readFileSync(absoluteReceiptPath, "utf8"),
    );
    const materializerHash =
      refreshedReceipt.file_hashes[previewManifestMaterializerPath];
    const receiptSurface =
      refreshedReceipt.dx_check_visibility.monitored_surfaces.find(
        (surface) => surface.id === previewManifestMaterializerSurfaceId,
      );
    assert.equal(
      receiptSurface.file_hashes[previewManifestMaterializerPath],
      materializerHash,
    );
    assert.ok(
      receiptSurface.source_markers.includes(
        "DATABASE_ORM_SOURCE_GUARD_RUNBOOK_FIXTURE",
      ),
    );

    const refreshedStatus = JSON.parse(
      fs.readFileSync(absolutePackageStatusPath, "utf8"),
    );
    const refreshedVisibility = refreshedStatus.package_lane_visibility.find(
      (entry) => entry.package_id === "db/drizzle-sqlite",
    );
    const packageStatusSurface = refreshedVisibility.selected_surfaces.find(
      (surface) =>
        surface.surface_id === previewManifestMaterializerSurfaceId,
    );
    assert.equal(packageStatusSurface.official_package_name, "Database ORM");
    assert.equal(
      packageStatusSurface.file_hashes[previewManifestMaterializerPath],
      materializerHash,
    );
    assert.ok(
      packageStatusSurface.source_markers.includes(
        "database-orm:receipt-hash-refresh",
      ),
    );

    const refreshedReadModel = fs.readFileSync(absoluteReadModelPath, "utf8");
    assert.match(refreshedReadModel, /database-orm-preview-manifest-materializer/);
    assert.match(refreshedReadModel, new RegExp(materializerHash));

    const fresh = runHelper(["--root", tempRoot, "--check", "--json"]);
    assert.equal(fresh.status, 0, fresh.stdout + fresh.stderr);
    assert.equal(JSON.parse(fresh.stdout).status, "current");
  } finally {
    fs.rmSync(tempRoot, { recursive: true, force: true });
  }
});

test("Database ORM helper attributes materializer drift while Drizzle sources stay current", () => {
  const tempRoot = materializerDriftFixture();
  try {
    const helper = runHelper(["--root", tempRoot, "--check", "--json"]);
    assert.equal(helper.status, 1, helper.stdout + helper.stderr);
    const helperReport = JSON.parse(helper.stdout);

    assert.equal(helperReport.status, "stale");
    assert.deepEqual(helperReport.stale_files, [previewManifestMaterializerPath]);
    assert.deepEqual(helperReport.missing_files, []);
    assert.ok(
      helperReport.stale_mirror_files.includes(previewManifestMaterializerPath),
      "materializer drift must be attributed as stale mirror evidence",
    );

    for (const relativePath of selectedDatabaseOrmSourceFiles) {
      assert.ok(
        helperReport.current_files.includes(relativePath),
        `${relativePath} should remain current when only the materializer drifts`,
      );
      assert.ok(
        !helperReport.stale_files.includes(relativePath),
        `${relativePath} should not be reported stale for a materializer-only drift`,
      );
    }
  } finally {
    fs.rmSync(tempRoot, { recursive: true, force: true });
  }
});

test("Database ORM helper writes a mirror-only drift fixture without mutating selected source hashes", () => {
  const tempRoot = currentDatabaseOrmFixture();
  try {
    const writeFixture = runHelper([
      "--root",
      tempRoot,
      "--write-mirror-drift-fixture",
      "--mirror-drift-fixture",
      mirrorDriftFixturePath,
    ]);
    assert.equal(writeFixture.status, 0, writeFixture.stdout + writeFixture.stderr);
    assert.match(writeFixture.stdout, /Database ORM mirror drift fixture written/);

    const fixture = JSON.parse(
      fs.readFileSync(path.join(tempRoot, mirrorDriftFixturePath), "utf8"),
    );
    assert.equal(
      fixture.schema,
      "dx.forge.package.receipt_hash_refresh.mirror_drift_fixture",
    );
    assert.equal(fixture.package.official_package_name, "Database ORM");
    assert.equal(fixture.package.package_id, "db/drizzle-sqlite");
    assert.equal(fixture.package.upstream_package, "drizzle-orm");
    assert.equal(fixture.honesty_label, "SOURCE-ONLY");
    assert.equal(fixture.runtime_proof, false);
    assert.equal(fixture.selected_source_hashes_clean, true);
    assert.equal(fixture.receipt_hash_refresh.status, "stale");
    assert.equal(fixture.receipt_hash_refresh.stale_file_count, 3);
    assert.deepEqual(fixture.receipt_hash_refresh.stale_files, []);
    assert.deepEqual(fixture.receipt_hash_refresh.missing_files, []);
    assert.deepEqual(fixture.receipt_hash_refresh.stale_mirror_files, [
      previewManifestMaterializerPath,
    ]);
    assert.deepEqual(fixture.receipt_hash_refresh.missing_mirror_files, []);
    assert.equal(fixture.receipt_hash_refresh.mirror_problem_count, 3);
    for (const relativePath of selectedDatabaseOrmSourceFiles) {
      assert.ok(
        fixture.receipt_hash_refresh.current_files.includes(relativePath),
        `${relativePath} should remain current in the mirror-only fixture`,
      );
    }
    assert.equal(
      fixture.check_panel_expectations.metrics.database_orm_hash_mismatch,
      0,
    );
    assert.equal(
      fixture.check_panel_expectations.metrics
        .database_orm_receipt_hash_refresh_stale,
      1,
    );

    const helperReport = runHelper(["--root", tempRoot, "--check", "--json"]);
    assert.equal(helperReport.status, 0, helperReport.stdout + helperReport.stderr);
    assert.equal(JSON.parse(helperReport.stdout).status, "current");
  } finally {
    fs.rmSync(tempRoot, { recursive: true, force: true });
  }
});

test("Database ORM checked-in mirror drift fixture is source-only check-panel evidence", () => {
  const fixture = readJson(mirrorDriftFixturePath);

  assert.equal(
    fixture.schema,
    "dx.forge.package.receipt_hash_refresh.mirror_drift_fixture",
  );
  assert.equal(fixture.fixture_kind, "mirror-only-drift");
  assert.equal(fixture.package.official_package_name, "Database ORM");
  assert.equal(fixture.package.package_id, "db/drizzle-sqlite");
  assert.equal(fixture.package.upstream_package, "drizzle-orm");
  assert.equal(fixture.package.source_mirror, "G:/WWW/inspirations/drizzle-orm");
  assert.equal(fixture.honesty_label, "SOURCE-ONLY");
  assert.equal(fixture.runtime_proof, false);
  assert.equal(fixture.selected_source_hashes_clean, true);
  assert.equal(fixture.receipt_hash_refresh.status, "stale");
  assert.deepEqual(fixture.receipt_hash_refresh.stale_files, []);
  assert.deepEqual(fixture.receipt_hash_refresh.missing_files, []);
  assert.deepEqual(fixture.receipt_hash_refresh.stale_mirror_files, [
    previewManifestMaterializerPath,
  ]);
  assert.equal(fixture.receipt_hash_refresh.mirror_problem_count, 3);
  assert.equal(fixture.receipt_hash_refresh.mirror_problem_details.length, 3);
  for (const relativePath of selectedDatabaseOrmSourceFiles) {
    assert.ok(
      fixture.receipt_hash_refresh.current_files.includes(relativePath),
      `${relativePath} should stay current in the checked-in fixture`,
    );
  }
  assert.equal(
    fixture.check_panel_expectations.metrics.database_orm_hash_mismatch,
    0,
  );
  assert.equal(
    fixture.check_panel_expectations.metrics
      .database_orm_receipt_hash_refresh_stale,
    1,
  );
});

test("Database ORM docs publish the hash refresh command without claiming runtime proof", () => {
  const packageDoc = fs.readFileSync(
    path.join(root, "docs/packages/db-drizzle-sqlite.md"),
    "utf8",
  );

  assert.match(
    packageDoc,
    /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/database-orm-receipt-hashes\.ts --check/,
  );
  assert.match(packageDoc, /--write/);
  assert.match(packageDoc, /database-orm\.mirror-drift\.fixture\.json/);
  assert.match(
    packageDoc,
    /does not run live SQLite reads, install better-sqlite3, or read database secrets/i,
  );
});
