const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceMirror = "G:/WWW/inspirations/instantdb";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(sourceMirror, relativePath), "utf8");
}

function escaped(marker) {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

function sourceSection(source, marker, label) {
  const start = source.indexOf(marker);
  assert.notEqual(start, -1, `missing ${label}`);
  const rest = source.slice(start);
  const end = ["\n  },", "\n    ),"]
    .map((candidate) => rest.indexOf(candidate))
    .filter((index) => index >= 0)
    .sort((a, b) => a - b)[0];
  assert.notEqual(end, -1, `missing end of ${label}`);
  return rest.slice(0, end);
}

test("Realtime App Database package-lane row exposes dx-style check-panel visibility", () => {
  const upstreamPackage = JSON.parse(
    readMirror("client/packages/react/package.json"),
  );
  const upstreamReact = readMirror("client/packages/react/src/index.ts");
  const upstreamCore = readMirror("client/packages/core/src/index.ts");
  const upstreamReactCommon = readMirror(
    "client/packages/react-common/src/InstantReactAbstractDatabase.tsx",
  );
  const syncTableSandbox = readMirror(
    "client/sandbox/react-nextjs/pages/play/sync-table.tsx",
  );
  const reader = read("core/src/ecosystem/dx_check_receipt.rs");
  const launchShell = read("examples/template/template-shell.tsx");
  const runtimePage = read("tools/launch/runtime-template/pages/index.html");
  const packageDoc = read("docs/packages/instantdb-react.md");
  const runbookFixture = JSON.parse(
    read("docs/packages/instantdb-react.source-guard-runbook.json"),
  );
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(upstreamPackage.name, "@instantdb/react");
  assert.equal(upstreamPackage.version, "0.0.0");
  assert.match(upstreamReact, /export \{\s*id,\s*tx,\s*lookup,\s*init,/);
  assert.match(upstreamReact, /SyncTableCallbackEventType/);
  assert.match(upstreamCore, /createInstantRouteHandler/);
  assert.match(upstreamCore, /StoreInterfaceStoreName/);
  assert.match(upstreamReactCommon, /useQuery = </);
  assert.match(upstreamReactCommon, /transact = \(/);
  assert.match(syncTableSandbox, /db\.core\._syncTableExperimental/);

  for (const marker of [
    'REALTIME_APP_DATABASE_PACKAGE_ID: &str = "instantdb/react"',
    'REALTIME_APP_DATABASE_OFFICIAL_NAME: &str = "Realtime App Database"',
    'REALTIME_APP_DATABASE_UPSTREAM_PACKAGE: &str = "@instantdb/react"',
    'REALTIME_APP_DATABASE_UPSTREAM_VERSION: &str = "0.0.0"',
    'REALTIME_APP_DATABASE_SOURCE_MIRROR: &str = "G:/WWW/inspirations/instantdb"',
    'REALTIME_APP_DATABASE_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json"',
    "REALTIME_APP_DATABASE_PACKAGE_RECEIPT_PATH: &str =",
    "REALTIME_APP_DATABASE_METRICS: [&str; 13]",
    "rows.extend(realtime_app_database_package_lane_row(root, package_status));",
    "fn realtime_app_database_package_lane_row(",
    "fn realtime_app_database_missing_receipt_row(next_action: &str)",
    "fn realtime_app_database_metric_rows(",
    "fn realtime_app_database_status_vocabulary(",
    "fn realtime_app_database_next_action(",
    "dx_style_compatibility_missing: u64",
    "realtime_app_database_hash_manifest_present",
    "realtime_app_database_hash_mismatch",
    "realtime_app_database_receipt_hash_refresh_current",
    "realtime_app_database_receipt_hash_refresh_stale",
    "realtime_app_database_receipt_hash_refresh_missing",
    "realtime_app_database_dx_style_compatibility_present",
    "realtime_app_database_dx_style_compatibility_missing",
    "receipt_hash_refresh_counts(package)",
    "data-dx-style-surface=\\\"realtime-app-database\\\"",
    "dx_check_latest_panel_exposes_realtime_app_database_package_lane_style_row",
    "dx_check_latest_panel_exposes_realtime_app_database_package_lane_hash_refresh_row",
    'stale_helper_realtime_app_database["package_lane_visibility"][0]["receipt_hash_refresh"]',
    '["status"] = serde_json::json!("stale")',
    '["stale_file_count"] = serde_json::json!(1)',
  ]) {
    assert.match(reader, escaped(marker), `missing check-panel marker ${marker}`);
  }

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /DX Studio\/check-panel Realtime App Database package row/);
    assert.match(source, /realtime_app_database_hash_manifest_present/);
    assert.match(source, /realtime_app_database_hash_mismatch/);
    assert.match(source, /realtime_app_database_receipt_hash_refresh_current/);
    assert.match(source, /realtime_app_database_receipt_hash_refresh_stale/);
    assert.match(source, /realtime_app_database_receipt_hash_refresh_missing/);
    assert.match(source, /realtime-app-database:receipt-hash-refresh/);
    assert.match(source, /realtime_app_database_dx_style_compatibility_present/);
    assert.match(source, /realtime_app_database_dx_style_compatibility_missing/);
    assert.match(
      source,
      /cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_realtime_app_database_package_lane_hash_refresh_row --lib/,
    );
    assert.match(source, /without claiming hosted Instant runtime proof/);
  }

  assert.equal(
    runbookFixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(runbookFixture.route, "/");
  assert.equal(
    runbookFixture.package.official_package_name,
    "Realtime App Database",
  );
  assert.equal(runbookFixture.package.package_id, "instantdb/react");
  assert.equal(runbookFixture.package.upstream_package, "@instantdb/react");
  assert.equal(runbookFixture.package.upstream_version, "0.0.0");
  assert.equal(
    runbookFixture.package.source_mirror,
    "G:/WWW/inspirations/instantdb",
  );
  assert.ok(
    runbookFixture.selected_surfaces.includes("check-panel-helper-freshness"),
  );
  assert.ok(runbookFixture.selected_surfaces.includes("receipt-hash-refresh"));
  for (const api of [
    "init",
    "db.useQuery",
    "db.transact",
    "db.storage.uploadFile",
    "db.streams.createWriteStream",
    "_syncTableExperimental",
    "SyncTableCallbackEventType",
    "StoreInterfaceStoreName",
    "createInstantRouteHandler",
  ]) {
    assert.ok(
      runbookFixture.upstream_public_apis.includes(api),
      `expected runbook fixture to cite upstream API ${api}`,
    );
  }
  assert.equal(
    runbookFixture.guard.id,
    "realtime-app-database-check-panel-helper-freshness",
  );
  assert.equal(
    runbookFixture.guard.command,
    "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_realtime_app_database_package_lane_hash_refresh_row --lib",
  );
  assert.ok(
    runbookFixture.guard.proves.includes(
      "realtime_app_database_receipt_hash_refresh_stale",
    ),
  );
  assert.equal(
    runbookFixture.receipt.hash_helper_json_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/realtime-app-database-receipt-hashes.ts --check --json",
  );
  assert.equal(
    runbookFixture.source_guard_fixture_paths[0].source_guard_id,
    "realtime-app-database-check-panel-helper-freshness",
  );
  assert.equal(
    runbookFixture.source_guard_fixture_paths[0].package_id,
    "instantdb/react",
  );
  assert.equal(
    runbookFixture.source_guard_fixture_paths[0].fixture_path,
    "docs/packages/instantdb-react.source-guard-runbook.json",
  );
  assert.equal(runbookFixture.honesty_label, "SOURCE-ONLY");
  assert.equal(runbookFixture.runtime_proof, false);

  for (const marker of [
    "realtime-app-database-check-panel-helper-freshness",
    "docs/packages/instantdb-react.source-guard-runbook.json",
    "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_realtime_app_database_package_lane_hash_refresh_row --lib",
  ]) {
    assert.match(studioManifest, escaped(marker), `missing Studio marker ${marker}`);
  }

  assert.match(launchShell, /package_id: "instantdb\/react"/);
  assert.match(launchShell, /official_package_name: "Realtime App Database"/);
  assert.match(launchShell, /upstream_package: "@instantdb\/react"/);
  assert.match(launchShell, /source_mirror: "G:\/WWW\/inspirations\/instantdb"/);
  assert.match(
    launchShell,
    /package_receipt_path:\s*"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-instantdb-realtime-dashboard\.json"/,
  );
  assert.match(launchShell, /dx_style_status: "present"/);
  assert.match(launchShell, /dx_style_surface: "realtime-app-database"/);
  assert.match(launchShell, /token_scope: "instantdb\/react"/);
  assert.match(launchShell, /hash_refresh_status: "current"/);
  assert.match(
    launchShell,
    /hash_refresh_helper:\s*"examples\/template\/realtime-app-database-receipt-hashes\.ts"/,
  );
  assert.match(
    launchShell,
    /hash_refresh_json_command:\s*"node tools\/launch\/run-template-receipt-helper\.js examples\/template\/realtime-app-database-receipt-hashes\.ts --check --json"/,
  );
  assert.match(launchShell, /hash_refresh_zed: "realtime-app-database:receipt-hash-refresh"/);
  assert.match(launchShell, /hash_refresh_tracked_files: 6/);
  assert.match(
    launchShell,
    /hash_refresh_metric_current:\s*"realtime_app_database_receipt_hash_refresh_current"/,
  );

  for (const marker of [
    'data-dx-check-package-lane-template="instantdb/react"',
    'data-dx-check-package-lane-row="instantdb/react"',
    'data-dx-check-package-lane-name="Realtime App Database"',
    'data-dx-check-package-lane-status="missing"',
    'data-dx-check-package-lane-receipt-status="missing-receipt"',
    'data-dx-check-package-lane-upstream-package="@instantdb/react"',
    'data-dx-check-package-lane-source-mirror="G:/WWW/inspirations/instantdb"',
    'data-dx-check-package-lane-receipt-path="examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json"',
    'data-dx-check-package-lane-dx-style-status="present"',
    'data-dx-check-package-lane-hash-refresh-status="current"',
    'data-dx-check-package-lane-hash-refresh-helper="examples/template/realtime-app-database-receipt-hashes.ts"',
    'data-dx-check-package-lane-hash-refresh-json-command="node tools/launch/run-template-receipt-helper.js examples/template/realtime-app-database-receipt-hashes.ts --check --json"',
    'data-dx-check-package-lane-hash-refresh-zed="realtime-app-database:receipt-hash-refresh"',
    'data-dx-check-package-lane-hash-refresh-tracked-files="6"',
    'data-dx-check-package-lane-hash-refresh-stale-files="0"',
    'data-dx-check-package-lane-hash-refresh-missing-files="0"',
    'data-dx-check-package-lane-hash-refresh-current-metric="realtime_app_database_receipt_hash_refresh_current"',
    'data-dx-check-package-lane-hash-refresh-stale-metric="realtime_app_database_receipt_hash_refresh_stale"',
    'data-dx-check-package-lane-hash-refresh-missing-metric="realtime_app_database_receipt_hash_refresh_missing"',
    'data-dx-style-surface="realtime-app-database"',
    'data-dx-token-scope="instantdb/react"',
    'data-dx-package="instantdb/react"',
  ]) {
    assert.match(runtimePage, escaped(marker), `missing static marker ${marker}`);
  }
});

test("Realtime App Database package-lane row survives generated starter materialization", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-instantdb-package-lane-"));
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
    const generatedLaunchPath = ["index.html"]
      .map((name) => path.join(dir, "pages", name))
      .find((candidate) => fs.existsSync(candidate));
    assert.ok(generatedLaunchPath, "expected generated launch page");
    const launch = fs.readFileSync(generatedLaunchPath, "utf8");
    const manifest = JSON.parse(
      fs.readFileSync(path.join(dir, "public", "preview-.dx/build-cache/manifest.json"), "utf8"),
    );
    const sourceContract = read("examples/template/dx-studio-edit-contract.ts");
    const materializerSource = read("tools/launch/materialize-www-template.ts");
    const studioManifest = read("dx-www/src/cli/studio_manifest.rs");

    assert.equal(result.ok, true);
    assert.equal(result.noNodeModules, true);
    assert.ok(!fs.existsSync(path.join(dir, "node_modules")));

    for (const marker of [
      'data-dx-check-package-lane-template="instantdb/react"',
      'data-dx-check-package-lane-row="instantdb/react"',
      'data-dx-check-package-lane-name="Realtime App Database"',
      'data-dx-check-package-lane-status="missing"',
      'data-dx-check-package-lane-receipt-status="missing-receipt"',
      'data-dx-check-package-lane-upstream-package="@instantdb/react"',
      'data-dx-check-package-lane-source-mirror="G:/WWW/inspirations/instantdb"',
      'data-dx-check-package-lane-receipt-path="examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json"',
      'data-dx-check-package-lane-dx-style-status="present"',
      'data-dx-check-package-lane-hash-refresh-status="current"',
      'data-dx-check-package-lane-hash-refresh-helper="examples/template/realtime-app-database-receipt-hashes.ts"',
      'data-dx-check-package-lane-hash-refresh-json-command="node tools/launch/run-template-receipt-helper.js examples/template/realtime-app-database-receipt-hashes.ts --check --json"',
      'data-dx-check-package-lane-hash-refresh-zed="realtime-app-database:receipt-hash-refresh"',
      'data-dx-check-package-lane-hash-refresh-tracked-files="6"',
      'data-dx-check-package-lane-hash-refresh-stale-files="0"',
      'data-dx-check-package-lane-hash-refresh-missing-files="0"',
      'data-dx-check-package-lane-hash-refresh-current-metric="realtime_app_database_receipt_hash_refresh_current"',
      'data-dx-check-package-lane-hash-refresh-stale-metric="realtime_app_database_receipt_hash_refresh_stale"',
      'data-dx-check-package-lane-hash-refresh-missing-metric="realtime_app_database_receipt_hash_refresh_missing"',
      'data-dx-style-surface="realtime-app-database"',
      'data-dx-token-scope="instantdb/react"',
      'data-dx-package="instantdb/react"',
    ]) {
      assert.match(
        launch,
        escaped(marker),
        `missing generated Realtime App Database marker ${marker}`,
      );
    }

    const homeRoute = manifest.routes.find((entry) => entry.route === "/");
    assert.ok(homeRoute, "expected generated / route metadata");
    assert.ok(
      homeRoute.forgePackages.includes("instantdb/react"),
      "generated / route package scope must include Realtime App Database",
    );

    for (const routePath of ["/dashboard", "/"]) {
      const route = manifest.routes.find((entry) => entry.route === routePath);
      assert.ok(route, `expected generated ${routePath} route metadata`);
      assert.ok(
        route.forgePackages.includes("instantdb/react"),
        `generated ${routePath} route package scope must include Realtime App Database`,
      );
    }

    const checkPanel = manifest.editContract.editableSurfaces.find(
      (surface) => surface.id === "launch-runtime-dx-check-panel",
    );
    assert.ok(checkPanel, "expected generated dx-check panel edit surface");
    assert.equal(checkPanel.sourceFile, "pages/index.html");
    assert.ok(
      checkPanel.packageIds.includes("instantdb/react"),
      "generated dx-check panel package scope must include Realtime App Database",
    );

    for (const marker of [
      "data-dx-check-package-lane-template",
      "data-dx-check-package-lane-row",
      "data-dx-check-package-lane-dx-style-status",
      "data-dx-style-surface",
      "data-dx-token-scope",
      "data-dx-check-package-lane-hash-refresh-status",
      "data-dx-check-package-lane-hash-refresh-helper",
      "data-dx-check-package-lane-hash-refresh-json-command",
      "data-dx-check-package-lane-hash-refresh-zed",
      "data-dx-check-package-lane-hash-refresh-tracked-files",
      "data-dx-check-package-lane-hash-refresh-stale-files",
      "data-dx-check-package-lane-hash-refresh-missing-files",
      "data-dx-check-package-lane-hash-refresh-current-metric",
      "data-dx-check-package-lane-hash-refresh-stale-metric",
      "data-dx-check-package-lane-hash-refresh-missing-metric",
    ]) {
      assert.ok(
        checkPanel.stateMarkers.includes(marker),
        `generated dx-check panel must expose ${marker}`,
      );
    }

    const sourceDxCheckPanel = sourceSection(
      sourceContract,
      'id: "dx-check-health-panel"',
      "source Realtime App Database dx-check panel package scope",
    );
    assert.match(sourceDxCheckPanel, /"instantdb\/react"/);

    const materializedDxCheckPanel = sourceSection(
      materializerSource,
      '"launch-runtime-dx-check-panel"',
      "materialized Realtime App Database dx-check panel package scope",
    );
    assert.match(materializedDxCheckPanel, /"instantdb\/react"/);

    assert.match(
      studioManifest,
      /fn studio_dx_check_edit_surface\(\)[\s\S]*"instantdb\/react"/,
      "Rust Studio manifest dx-check panel package scope must include Realtime App Database",
    );

    for (const source of [
      read("docs/packages/instantdb-react.md"),
      read("DX.md"),
      read("TODO.md"),
      read("CHANGELOG.md"),
    ]) {
      assert.match(source, /Realtime App Database generated-starter materialization guard/);
      assert.match(source, /without claiming hosted Instant runtime proof/);
    }
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});
