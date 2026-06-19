const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceMirror = "G:/WWW/inspirations/drizzle-orm";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(sourceMirror, relativePath), "utf8");
}

function escaped(marker) {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

test("Database ORM package-lane row exposes DX check-panel style visibility", () => {
  const runbookFixturePath =
    "docs/packages/database-orm.source-guard-runbook.json";
  assert.ok(
    fs.existsSync(path.join(root, runbookFixturePath)),
    "Database ORM source-guard runbook fixture should exist",
  );

  const upstreamPackage = JSON.parse(readMirror("drizzle-orm/package.json"));
  const upstreamDb = readMirror("drizzle-orm/src/sqlite-core/db.ts");
  const upstreamDriver = readMirror("drizzle-orm/src/better-sqlite3/driver.ts");
  const reader = read("core/src/ecosystem/dx_check_receipt.rs");
  const launchShell = read("examples/template/template-shell.tsx");
  const runtimeLaunch = read("tools/launch/runtime-template/pages/index.html");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const materializer = read("tools/launch/materialize-www-template.ts");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const packageDoc = read("docs/packages/db-drizzle-sqlite.md");
  const frameworkDocs = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const runbookFixture = readJson(runbookFixturePath);
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(upstreamPackage.name, "drizzle-orm");
  assert.equal(upstreamPackage.version, "0.45.3");
  assert.match(upstreamDb, /export const withReplicas = </);
  assert.match(upstreamDb, /const selectDistinct: Q\['selectDistinct'\]/);
  assert.match(upstreamDriver, /export function drizzle/);

  for (const marker of [
    'DATABASE_ORM_PACKAGE_ID: &str = "db/drizzle-sqlite"',
    'DATABASE_ORM_OFFICIAL_NAME: &str = "Database ORM"',
    'DATABASE_ORM_UPSTREAM_PACKAGE: &str = "drizzle-orm"',
    'DATABASE_ORM_UPSTREAM_VERSION: &str = "0.45.3"',
    'DATABASE_ORM_SOURCE_MIRROR: &str = "G:/WWW/inspirations/drizzle-orm"',
    'DATABASE_ORM_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json"',
    'DATABASE_ORM_PACKAGE_RECEIPT_PATH: &str =',
    "rows.extend(database_orm_package_lane_row(root, package_status));",
    "fn database_orm_package_lane_row(",
    "fn database_orm_metric_rows(",
    "fn database_orm_status_vocabulary(",
    "fn database_orm_next_action(",
    "dx_style_compatibility_missing: u64",
    "database_orm_hash_manifest_present",
    "database_orm_hash_mismatch",
    "database_orm_receipt_hash_refresh_current",
    "database_orm_receipt_hash_refresh_stale",
    "database_orm_receipt_hash_refresh_missing",
    "database_orm_dx_style_compatibility_present",
    "database_orm_dx_style_compatibility_missing",
    "pub mirror_problem_count: u64",
    'mirror_problem_count: json_u64(refresh, &["mirror_problem_count"]).unwrap_or_else',
    "stale_helper_database_orm",
    "count_sha256_file_hash_mismatches(root, package)",
    "receipt_hash_refresh: receipt_hash_refresh.clone()",
    "dx_check_latest_panel_exposes_database_orm_package_lane_style_row",
  ]) {
    assert.match(reader, escaped(marker), `missing check-panel marker ${marker}`);
  }
  assert.match(
    reader,
    /database_orm_next_action\(\s*status,\s*receipt_hash_refresh\.as_ref\(\),\s*dx_style_compatibility_missing,\s*\)/,
  );

  assert.match(
    launchShell,
    /data-dx-check-package-lane-dx-style-status=\{dxStyleCompatibilityStatus\(packageLane\)\}/,
  );
  assert.match(runtimeLaunch, /data-dx-check-package-lane-count="0"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-template="db\/drizzle-sqlite"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-row="db\/drizzle-sqlite"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-name="Database ORM"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-status="missing"/);
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-receipt-status="missing-receipt"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-upstream-package="drizzle-orm"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-source-mirror="G:\/WWW\/inspirations\/drizzle-orm"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-receipt-path="examples\/template\/\.dx\/forge\/receipts\/2026-05-22-db-drizzle-sqlite-dashboard-workflow\.json"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-dx-style-status="present"/,
  );

  for (const source of [editContract, materializer]) {
    assert.match(source, /"db\/drizzle-sqlite"/);
    assert.match(source, /"data-dx-check-package-lane-row"/);
    assert.match(source, /"data-dx-check-package-lane-dx-style-status"/);
  }
  assert.match(
    editContract,
    /id: "dx-check-health-panel"[\s\S]*packageIds: \[[\s\S]*"db\/drizzle-sqlite"/,
  );
  assert.match(
    materializer,
    /"launch-runtime-dx-check-panel"[\s\S]*\[[\s\S]*"db\/drizzle-sqlite"/,
  );
  assert.match(
    studioManifest,
    /"package": "db\/drizzle-sqlite"[\s\S]*"front_facing_name": "Database ORM"[\s\S]*"data-dx-check-package-lane-row"/,
  );
  assert.match(
    studioManifest,
    /studio_source_guard_with_fixture\(\s*"database-orm-lower-dx-check-helper-freshness"/,
  );
  assert.match(
    studioManifest,
    /cargo test -q -p dx-www-compiler database_orm_hash_refresh_stale_helper_keeps_source_hash_clean --lib/,
  );
  assert.match(
    studioManifest,
    /source_guard_contract_with_fixture\(\s*"database-orm-lower-dx-check-helper-freshness"[\s\S]*database_orm_receipt_hash_refresh_stale[\s\S]*docs\/packages\/database-orm\.source-guard-runbook\.json/,
  );
  assert.match(
    studioManifest,
    /source_guard_command_with_fixture\(\s*"cargo test -q -p dx-www-compiler database_orm_hash_refresh_stale_helper_keeps_source_hash_clean --lib"[\s\S]*Database ORM[\s\S]*docs\/packages\/database-orm\.source-guard-runbook\.json/,
  );
  assert.match(studioManifest, /docs\/packages\/database-orm\.source-guard-runbook\.json/);

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /DX Studio\/check-panel Database ORM package row/);
    assert.match(source, /database_orm_hash_manifest_present/);
    assert.match(source, /database_orm_hash_mismatch/);
    assert.match(source, /database_orm_receipt_hash_refresh_current/);
    assert.match(source, /database_orm_receipt_hash_refresh_stale/);
    assert.match(source, /database_orm_receipt_hash_refresh_missing/);
    assert.match(source, /receipt_hash_refresh helper freshness/);
    assert.match(source, /mirror_problem_count/);
    assert.match(source, /stale-helper-only Database ORM check-panel fixture/);
    assert.match(source, /database_orm_dx_style_compatibility_present/);
    assert.match(source, /database_orm_dx_style_compatibility_missing/);
    assert.match(source, /without claiming live SQLite read proof/);
  }
  assert.match(
    packageDoc,
    /database-orm-lower-dx-check-helper-freshness/,
  );
  assert.match(
    packageDoc,
    /docs\/packages\/database-orm\.source-guard-runbook\.json/,
  );
  assert.match(
    frameworkDocs,
    /Lane 9 uses the official front-facing package name `Database ORM`[\s\S]*docs\/packages\/database-orm\.source-guard-runbook\.json/,
  );

  assert.equal(
    runbookFixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(runbookFixture.route, "/");
  assert.equal(runbookFixture.package.official_package_name, "Database ORM");
  assert.equal(runbookFixture.package.package_id, "db/drizzle-sqlite");
  assert.equal(runbookFixture.package.upstream_package, "drizzle-orm");
  assert.equal(runbookFixture.package.upstream_version, "0.45.3");
  assert.deepEqual(runbookFixture.package.source_mirrors, [
    "G:/WWW/inspirations/drizzle-orm",
  ]);
  assert.deepEqual(runbookFixture.selected_surfaces, [
    "drizzle-replica-routing",
    "drizzle-launch-dashboard-workflow",
    "database-orm-source-guard-runbook",
    "database-orm-preview-manifest-materializer",
    "database-orm-lock-backed-source",
    "receipt-hash-refresh",
  ]);
  assert.equal(
    runbookFixture.guard.id,
    "database-orm-lower-dx-check-helper-freshness",
  );
  assert.equal(
    runbookFixture.guard.guard_file,
    "core/src/ecosystem/project_check/database_orm_dx_check.rs",
  );
  assert.equal(
    runbookFixture.guard.command,
    "cargo test -q -p dx-www-compiler database_orm_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
  );
  assert.equal(runbookFixture.guard.execution_policy, "source-only");
  assert.equal(runbookFixture.guard.starts_server, false);
  assert.equal(runbookFixture.guard.runs_package_install, false);
  assert.equal(runbookFixture.guard.runs_full_build, false);
  assert.equal(runbookFixture.guard.node_modules_required, false);
  assert.ok(
    runbookFixture.guard.proves.includes(
      "database_orm_receipt_hash_refresh_stale",
    ),
  );
  assert.ok(
    runbookFixture.guard.proves.includes(
      "database_orm_hash_mismatch stays byte-derived",
    ),
  );
  assert.equal(
    runbookFixture.runbook.command.command,
    runbookFixture.guard.command,
  );
  assert.equal(runbookFixture.runbook.command.starts_server, false);
  assert.equal(runbookFixture.runbook.command.runs_package_install, false);
  assert.equal(runbookFixture.runbook.command.runs_full_build, false);
  assert.ok(runbookFixture.upstream_public_apis.includes("withReplicas"));
  assert.ok(runbookFixture.upstream_public_apis.includes("selectDistinct"));
  assert.ok(runbookFixture.upstream_public_apis.includes("$count"));
  assert.equal(
    runbookFixture.receipt.zed_visibility,
    "database-orm:receipt-hash-refresh",
  );
  assert.equal(
    runbookFixture.receipt.source_guard_runbook_fixture,
    runbookFixturePath,
  );
  assert.equal(
    runbookFixture.receipt.preview_manifest_materializer,
    "tools/launch/materialize-www-template.ts",
  );
  assert.equal(runbookFixture.receipt.tracked_by_receipt_hash_helper, true);
  assert.equal(runbookFixture.receipt.tracked_file_count, 12);
  for (const trackedFile of [
    "examples/template/db/drizzle/schema.ts",
    "examples/template/db/drizzle/metadata.ts",
    "examples/template/db/drizzle/README.md",
    "examples/template/server/database-orm/readiness.ts",
    "examples/template/app/api/database-orm/readiness/route.ts",
  ]) {
    assert.ok(
      runbookFixture.receipt.tracked_files.includes(trackedFile),
      `${trackedFile} should be tracked as Database ORM lock-backed source`,
    );
  }
  assert.deepEqual(runbookFixture.preview_manifest, {
    generated_file: "public/preview-manifest.json",
    materializer: "tools/launch/materialize-www-template.ts",
    root_field: "sourceGuardRunbookFixtures",
    route_field: "routes[].sourceGuardRunbookFixtures",
    fixture: runbookFixturePath,
    runtime_proof: false,
  });
  assert.equal(runbookFixture.honesty_label, "SOURCE-ONLY");
  assert.equal(runbookFixture.runtime_proof, false);
  assert.ok(
    runbookFixture.runtime_limitations.some((limitation) =>
      limitation.includes("SOURCE-ONLY"),
    ),
  );
});

test("Database ORM runbook fixture is exposed from generated preview-manifest metadata", () => {
  const runbookFixturePath =
    "docs/packages/database-orm.source-guard-runbook.json";
  const guardId = "database-orm-lower-dx-check-helper-freshness";
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-database-orm-preview-"));
  const materializer = path.join(
    root,
    "tools",
    "launch",
    "materialize-www-template.ts",
  );

  try {
    const result = JSON.parse(
      execFileSync(process.execPath, [materializer, dir], {
        cwd: root,
        encoding: "utf8",
      }),
    );
    const manifest = JSON.parse(
      fs.readFileSync(path.join(dir, "public", "preview-manifest.json"), "utf8"),
    );
    const launch = fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8");
    const fixture = readJson(runbookFixturePath);

    assert.equal(result.ok, true);
    assert.equal(result.noNodeModules, true);
    assert.ok(!fs.existsSync(path.join(dir, "node_modules")));
    assert.ok(
      Array.isArray(manifest.sourceGuardRunbookFixtures),
      "generated preview manifest must expose source-guard runbook fixtures",
    );

    const databaseOrmFixture = manifest.sourceGuardRunbookFixtures.find(
      (entry) => entry.packageId === "db/drizzle-sqlite",
    );
    assert.ok(
      databaseOrmFixture,
      "generated preview manifest must expose the Database ORM runbook fixture",
    );
    assert.equal(databaseOrmFixture.officialPackageName, "Database ORM");
    assert.equal(databaseOrmFixture.upstreamPackage, "drizzle-orm");
    assert.equal(databaseOrmFixture.upstreamVersion, "0.45.3");
    assert.equal(
      databaseOrmFixture.sourceMirror,
      "G:/WWW/inspirations/drizzle-orm",
    );
    assert.equal(databaseOrmFixture.fixture, runbookFixturePath);
    assert.equal(databaseOrmFixture.guardId, guardId);
    assert.equal(
      databaseOrmFixture.schema,
      "dx.forge.package.source_guard_runbook_fixture",
    );
    assert.equal(databaseOrmFixture.route, "/");
    assert.equal(databaseOrmFixture.honestyLabel, "SOURCE-ONLY");
    assert.equal(databaseOrmFixture.runtimeProof, false);
    assert.equal(
      databaseOrmFixture.zedVisibility,
      "database-orm:receipt-hash-refresh",
    );

    const launchRoute = manifest.routes.find((entry) => entry.route === "/");
    assert.ok(launchRoute, "expected generated /launch route metadata");
    assert.ok(
      launchRoute.forgePackages.includes("db/drizzle-sqlite"),
      "generated /launch route package scope must include Database ORM",
    );
    assert.ok(
      launchRoute.sourceGuardRunbookFixtures.includes(runbookFixturePath),
      "generated /launch route must link the Database ORM source-guard runbook fixture",
    );
    assert.match(launch, /data-dx-check-package-lane-template="db\/drizzle-sqlite"/);
    assert.match(launch, /data-dx-check-package-lane-row="db\/drizzle-sqlite"/);
    assert.match(launch, /data-dx-check-package-lane-name="Database ORM"/);
    assert.deepEqual(fixture.preview_manifest, {
      generated_file: "public/preview-manifest.json",
      materializer: "tools/launch/materialize-www-template.ts",
      root_field: "sourceGuardRunbookFixtures",
      route_field: "routes[].sourceGuardRunbookFixtures",
      fixture: runbookFixturePath,
      runtime_proof: false,
    });
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});
