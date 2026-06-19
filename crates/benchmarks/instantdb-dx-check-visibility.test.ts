const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const { spawnSync } = require("node:child_process");
const { test } = require("node:test");
const fs = require("node:fs");
const path = require("node:path");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(mirror, relativePath), "utf8");
}

function sha256(relativePath) {
  return crypto.createHash("sha256").update(read(relativePath)).digest("hex");
}

function readHashRefreshReport() {
  const helperPath = path.join(
    root,
    "examples/template/realtime-app-database-receipt-hashes.ts",
  );
  const result = spawnSync(
    process.execPath,
    [helperPath, "--check", "--json"],
    {
      cwd: root,
      encoding: "utf8",
    },
  );
  assert.equal(result.status, 0, result.stdout + result.stderr);
  return JSON.parse(result.stdout);
}

const statusVocabulary = [
  "present",
  "stale",
  "missing-receipt",
  "blocked",
  "unsupported-surface",
];

function assertStatusVocabulary(source, label) {
  for (const status of statusVocabulary) {
    assert.match(
      source,
      new RegExp(`["']${status}["']`),
      `${label} should expose dx-check status ${status}`,
    );
  }
}

test("Realtime App Database publishes dx-check package visibility without runtime overclaim", () => {
  const upstreamReactPackage = JSON.parse(
    readMirror("client/packages/react/package.json"),
  );
  const upstreamReactIndex = readMirror("client/packages/react/src/index.ts");
  const upstreamCoreIndex = readMirror("client/packages/core/src/index.ts");
  const upstreamSyncTableSandbox = readMirror(
    "client/sandbox/react-nextjs/pages/play/sync-table.tsx",
  );

  const slice = read("core/src/ecosystem/forge_instantdb.rs");
  const catalog = read("examples/template/package-catalog.ts");
  const dashboardModel = read("examples/dashboard/src/lib/instantdbDashboard.ts");
  const dashboardWorkflow = read(
    "examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx",
  );
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
  );
  const packageStatus = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = read("examples/template/forge-package-status-read-model.ts");
  const statusSource = read("examples/template/forge-package-status.ts");
  const packageDoc = read("docs/packages/instantdb-react.md");
  const cli = read("dx-www/src/cli/mod.rs");

  assert.equal(upstreamReactPackage.name, "@instantdb/react");
  assert.equal(upstreamReactPackage.version, "0.0.0");
  assert.match(upstreamReactIndex, /SyncTableCallbackEventType/);
  assert.match(upstreamCoreIndex, /_syncTableExperimental/);
  assert.match(upstreamCoreIndex, /type StoreInterfaceStoreName/);
  assert.match(upstreamSyncTableSandbox, /db\.core\._syncTableExperimental/);

  assert.match(slice, /officialPackageName: "Realtime App Database"/);
  assert.match(slice, /upstreamVersion: "0\.0\.0"/);
  assert.match(slice, /selectedSurfaces: \[/);
  assert.match(slice, /dxCheckVisibility: \{/);
  assert.match(slice, /currentStatus: "present"/);
  assert.match(slice, /receiptPath: "examples\/template\/\.dx\/forge\/receipts\/2026-05-22-instantdb-realtime-dashboard\.json"/);
  assert.match(slice, /honestyLabel: "ADAPTER-BOUNDARY"/);
  assertStatusVocabulary(slice, "Forge metadata template");

  assert.match(catalog, /officialName: "Realtime App Database"/);
  assert.match(catalog, /upstreamPackage: "@instantdb\/react"/);
  assert.match(catalog, /upstreamVersion: "0\.0\.0"/);
  assert.match(catalog, /selectedSurfaces: \[/);
  assert.match(catalog, /dxCheckVisibility: \{/);
  assert.match(catalog, /honestyLabel: "ADAPTER-BOUNDARY"/);
  assertStatusVocabulary(catalog, "launch catalog");

  assert.match(dashboardModel, /instantDashboardDxCheckVisibility/);
  assert.match(dashboardModel, /currentStatus: 'present'/);
  assert.match(dashboardModel, /status: 'blocked'/);
  assert.match(dashboardModel, /status: 'unsupported-surface'/);
  assert.match(dashboardWorkflow, /data-dx-instant-dashboard-dx-check-status/);
  assert.match(dashboardWorkflow, /data-dx-instant-dashboard-dx-check-schema/);

  assert.equal(receipt.package_id, "instantdb/react");
  assert.equal(receipt.package_name, "Realtime App Database");
  assert.equal(receipt.honesty_label, "ADAPTER-BOUNDARY");
  assert.deepEqual(receipt.dx_check_visibility.status_vocabulary, statusVocabulary);
  assert.equal(receipt.dx_check_visibility.current_status, "present");
  assert.equal(receipt.dx_check_visibility.receipt_status, "present");
  assert.ok(
    receipt.dx_check_visibility.selected_surfaces.some(
      (surface) => surface.surface_id === "instantdb-runtime-dashboard-workflow",
    ),
    "receipt should monitor the runtime dashboard workflow surface",
  );
  assert.ok(
    receipt.runtime_limitations.some((limitation) => limitation.includes("SOURCE-ONLY")),
    "receipt should keep Sync Table runtime proof honest",
  );

  const realtimeVisibility = packageStatus.package_lane_visibility.find(
    (entry) => entry.package_id === "instantdb/react",
  );
  assert.ok(realtimeVisibility, "Realtime App Database visibility row is missing");
  assert.equal(realtimeVisibility.official_package_name, "Realtime App Database");
  assert.equal(realtimeVisibility.upstream_package, "@instantdb/react");
  assert.equal(realtimeVisibility.upstream_version, "0.0.0");
  assert.deepEqual(realtimeVisibility.status_vocabulary, statusVocabulary);
  assert.ok(
    realtimeVisibility.selected_surfaces.some(
      (surface) => surface.surface_id === "sync-table-events",
    ),
    "Sync Table events surface should be visible to dx-check",
  );

  for (const metric of [
    "realtime_app_database_receipt_present",
    "realtime_app_database_receipt_stale",
    "realtime_app_database_missing_receipt",
    "realtime_app_database_blocked_surface",
    "realtime_app_database_unsupported_surface",
  ]) {
    assert.ok(
      realtimeVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from Realtime App Database visibility row`,
    );
    assert.ok(
      packageStatus.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  assert.match(readModel, /export const realtimeAppDatabasePackageVisibility/);
  assert.match(statusSource, /realtimeAppDatabasePackageVisibility/);
  assert.match(packageDoc, /## dx-check Visibility/);
  assert.match(packageDoc, /ADAPTER-BOUNDARY/);
  assert.match(packageDoc, /SOURCE-ONLY/);
  assert.match(cli, /"package_id": "instantdb\/react"[\s\S]*"dx_check_visibility": \{/);
  assert.match(cli, /"package_id": "instantdb\/react"[\s\S]*"honesty_label": "ADAPTER-BOUNDARY"/);
});

test("Realtime App Database receipt hash helper stays current on source-owned runtime assets", () => {
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
  );
  const packageStatus = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = read("examples/template/forge-package-status-read-model.ts");
  const report = readHashRefreshReport();
  const trackedFiles = Object.keys(receipt.file_hashes);
  const realtimeVisibility = packageStatus.package_lane_visibility.find(
    (entry) => entry.package_id === "instantdb/react",
  );

  assert.ok(realtimeVisibility, "Realtime App Database visibility row is missing");
  assert.equal(report.status, "current");
  assert.equal(report.tracked_file_count, trackedFiles.length);
  assert.equal(report.stale_file_count, 0);
  assert.equal(report.missing_file_count, 0);
  assert.deepEqual(report.tracked_files, trackedFiles);
  assert.deepEqual(report.current_files, trackedFiles);
  assert.deepEqual(report.stale_files, []);
  assert.deepEqual(report.missing_files, []);
  assert.deepEqual(report.stale_mirror_files, []);
  assert.deepEqual(report.missing_mirror_files, []);
  assert.equal(report.mirror_problem_count, 0);
  assert.equal(report.runtime_execution, false);
  assert.equal(report.secret_access, false);
  assert.ok(
    trackedFiles.includes("tools/launch/runtime-template/assets/launch-runtime.ts"),
    "Realtime App Database should track the source-owned launch runtime asset",
  );
  assert.ok(
    !trackedFiles.includes("tools/launch/runtime-template/assets/launch-runtime.js"),
    "Realtime App Database must not track the generated launch runtime JS asset",
  );

  for (const [filePath, expectedHash] of Object.entries(receipt.file_hashes)) {
    assert.equal(expectedHash, sha256(filePath), `${filePath} has a stale hash`);
  }

  assert.deepEqual(realtimeVisibility.receipt_hash_refresh, {
    schema: report.schema,
    status: report.status,
    helper_path: report.helper_path,
    check_command: report.check_command,
    write_command: report.write_command,
    json_check_command: report.json_check_command,
    source_guard_runbook_fixture: report.source_guard_runbook_fixture,
    receipt_path: report.receipt_path,
    package_status_path: report.package_status_path,
    read_model_path: report.read_model_path,
    hash_algorithm: report.hash_algorithm,
    tracked_file_count: report.tracked_file_count,
    tracked_files: report.tracked_files,
    current_files: report.current_files,
    stale_files: report.stale_files,
    missing_files: report.missing_files,
    stale_mirror_files: report.stale_mirror_files,
    missing_mirror_files: report.missing_mirror_files,
    mirror_problem_count: report.mirror_problem_count,
    stale_file_count: report.stale_file_count,
    missing_file_count: report.missing_file_count,
    runtime_execution: report.runtime_execution,
    secret_access: report.secret_access,
    zed_visibility: report.zed_visibility,
    runtime_limitations: report.runtime_limitations,
  });
  assert.doesNotMatch(
    JSON.stringify({ receipt, realtimeVisibility }),
    /tools\/launch\/runtime-template\/assets\/launch-runtime\.js/,
  );
  assert.match(readModel, /tools\\launch\\runtime-template\\assets\\/launch-runtime\.ts/);
});
