const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath = "core/src/ecosystem/project_check/realtime_app_database_dx_check.rs";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("Realtime App Database has a focused Rust dx-check visibility emitter", () => {
  const upstreamReactIndex = fs.readFileSync(
    path.resolve(
      root,
      "..",
      "..",
      "WWW/inspirations/instantdb/client/packages/react/src/index.ts",
    ),
    "utf8",
  );
  const upstreamCoreIndex = fs.readFileSync(
    path.resolve(
      root,
      "..",
      "..",
      "WWW/inspirations/instantdb/client/packages/core/src/index.ts",
    ),
    "utf8",
  );
  const helper = read(helperPath);
  const projectCheck = read("core/src/ecosystem/project_check.rs");
  const packageDoc = read("docs/packages/instantdb-react.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.match(upstreamReactIndex, /SyncTableCallbackEventType/);
  assert.match(upstreamCoreIndex, /_syncTableExperimental/);

  assert.match(helper, /REALTIME_APP_DATABASE_PACKAGE_ID: &str = "instantdb\/react"/);
  assert.match(helper, /REALTIME_APP_DATABASE_OFFICIAL_NAME: &str = "Realtime App Database"/);
  assert.match(
    helper,
    /REALTIME_APP_DATABASE_PACKAGE_STATUS: &str = "\.dx\/forge\/package-status\.json"/,
  );
  assert.match(
    helper,
    /REALTIME_APP_DATABASE_DASHBOARD_RECEIPT: &str =\s*"\.dx\/forge\/receipts\/2026-05-22-instantdb-realtime-dashboard\.json"/,
  );

  for (const metric of [
    "realtime_app_database_package_present",
    "realtime_app_database_receipt_present",
    "realtime_app_database_receipt_stale",
    "realtime_app_database_missing_receipt",
    "realtime_app_database_blocked_surface",
    "realtime_app_database_unsupported_surface",
  ]) {
    assert.match(helper, new RegExp(metric));
  }

  for (const finding of [
    "realtime-app-database-missing-package-status",
    "realtime-app-database-missing-receipt",
    "realtime-app-database-stale-receipt",
    "realtime-app-database-blocked-surface",
    "realtime-app-database-unsupported-surface",
  ]) {
    assert.match(helper, new RegExp(finding));
  }

  assert.match(projectCheck, /mod realtime_app_database_dx_check;/);
  assert.match(
    projectCheck,
    /use realtime_app_database_dx_check::forge_realtime_app_database_package_metrics;/,
  );
  assert.match(projectCheck, /forge_realtime_app_database_package_metrics\(root, &manifest\)/);
  assert.match(projectCheck, /dx_check_reports_realtime_app_database_package_status_visibility/);
  assert.match(
    projectCheck,
    /realtime_app_database_hash_mismatch_flips_when_selected_file_changes/,
  );
  assert.match(projectCheck, /realtime-app-database-hash-mismatch/);

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /Rust dx-check emitter/);
    assert.match(source, /realtime_app_database_package_present/);
    assert.match(source, /realtime-app-database-missing-receipt/);
  }
});
