const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function assertIncludesAll(values, expectedValues) {
  for (const expectedValue of expectedValues) {
    assert.ok(
      values.includes(expectedValue),
      `expected ${JSON.stringify(values)} to include ${expectedValue}`,
    );
  }
}

function assertDxStyleCompatibility(value) {
  assert.equal(value.schema, "dx.forge.package.dx_style_compatibility");
  assert.equal(value.status, "present");
  assert.equal(
    value.token_source ?? value.tokenSource,
    "examples/template/styles/globals.css",
  );
  assert.equal(
    value.generated_css ?? value.generatedCss,
    "examples/template/styles/globals.css",
  );
  assert.equal(value.runtime_proof ?? value.runtimeProof, false);
}

test("Realtime App Database publishes dx-style compatibility for visible launch and dashboard surfaces", () => {
  const upstreamPackage = fs.readFileSync(
    path.resolve(
      root,
      "..",
      "..",
      "WWW/inspirations/instantdb/client/packages/react/package.json",
    ),
    "utf8",
  );
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
  const upstreamSyncTableExample = fs.readFileSync(
    path.resolve(
      root,
      "..",
      "..",
      "WWW/inspirations/instantdb/client/sandbox/react-nextjs/pages/play/sync-table.tsx",
    ),
    "utf8",
  );

  assert.match(upstreamPackage, /"name": "@instantdb\/react"/);
  assert.match(upstreamPackage, /"version": "0\.0\.0"/);
  assert.match(upstreamReactIndex, /export \{/);
  for (const api of ["lookup", "init", "SyncTableCallbackEventType"]) {
    assert.match(upstreamReactIndex, new RegExp(api));
  }
  for (const api of [
    "_syncTableExperimental",
    "transact",
    "StoreInterfaceStoreName",
  ]) {
    assert.match(upstreamCoreIndex, new RegExp(api));
  }
  assert.match(upstreamSyncTableExample, /db\.core\._syncTableExperimental/);

  const launchStatus = read("examples/template/instantdb-status.tsx");
  const runtimeLaunch = read("tools/launch/runtime-template/pages/index.html");
  const dashboardWorkflow = read(
    "examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx",
  );

  for (const source of [launchStatus, runtimeLaunch, dashboardWorkflow]) {
    assert.match(source, /data-dx-style-surface="realtime-app-database"/);
    assert.doesNotMatch(source, /style=\{\{/);
  }
  assert.match(launchStatus, /bg-card/);
  assert.match(launchStatus, /text-card-foreground/);
  assert.doesNotMatch(launchStatus, /#[0-9a-fA-F]{3,8}/);

  const receipt = readJson(
    "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
  );
  assertDxStyleCompatibility(receipt.dx_style_compatibility);
  assertIncludesAll(receipt.dx_style_compatibility.visible_surfaces, [
    "instantdb-runtime-dashboard-workflow",
    "dashboard-instantdb-workflow",
  ]);
  assertIncludesAll(receipt.dx_style_compatibility.source_files, [
    "examples/template/instantdb-status.tsx",
    "tools/launch/runtime-template/pages/index.html",
    "examples/dashboard/src/components/InstantDbDashboardWorkflow.tsx",
  ]);
  assertIncludesAll(receipt.dx_style_compatibility.data_dx_markers, [
    'data-dx-style-surface="realtime-app-database"',
  ]);
  assert.match(
    receipt.dx_style_compatibility.runtime_limitations.join("\n"),
    /SOURCE-ONLY/,
  );
  assert.match(
    receipt.dx_style_compatibility.runtime_limitations.join("\n"),
    /No live browser style proof was run/,
  );

  const status = readJson("examples/template/.dx/forge/package-status.json");
  const realtimeRow = status.package_lane_visibility.find(
    (entry) => entry.package_id === "instantdb/react",
  );
  assert.ok(realtimeRow, "missing Realtime App Database package-status row");
  assertDxStyleCompatibility(realtimeRow.dx_style_compatibility);
  assertIncludesAll(realtimeRow.dx_check_metrics, [
    "realtime_app_database_dx_style_compatibility_present",
    "realtime_app_database_dx_style_compatibility_missing",
  ]);
  assertIncludesAll(status.dx_check_metrics, [
    "realtime_app_database_dx_style_compatibility_present",
    "realtime_app_database_dx_style_compatibility_missing",
  ]);
  for (const surfaceId of [
    "instantdb-runtime-dashboard-workflow",
    "dashboard-instantdb-workflow",
  ]) {
    const surface = realtimeRow.selected_surfaces.find(
      (entry) => entry.surface_id === surfaceId,
    );
    assert.ok(surface, `missing selected surface ${surfaceId}`);
    assertIncludesAll(surface.source_markers, [
      'data-dx-style-surface="realtime-app-database"',
    ]);
  }

  const readModel = read("examples/template/forge-package-status-read-model.ts");
  assert.match(readModel, /dxStyleCompatibility:\s*\{/);
  assert.match(
    readModel,
    /realtime_app_database_dx_style_compatibility_present/,
  );
  assert.match(
    readModel,
    /realtime_app_database_dx_style_compatibility_missing/,
  );
  assert.match(readModel, /data-dx-style-surface="realtime-app-database"/);

  const packageCatalog = read("examples/template/package-catalog.ts");
  assert.match(packageCatalog, /dxStyleCompatibility:\s*\{/);
  assert.match(packageCatalog, /data-dx-style-surface="realtime-app-database"/);

  const checker = read(
    "core/src/ecosystem/project_check/realtime_app_database_dx_check.rs",
  );
  assert.match(
    checker,
    /realtime_app_database_dx_style_compatibility_present/,
  );
  assert.match(
    checker,
    /realtime_app_database_dx_style_compatibility_missing/,
  );
  assert.match(
    checker,
    /realtime-app-database-missing-dx-style-compatibility/,
  );
  assert.match(checker, /fn dx_style_compatibility_is_present/);

  const docs = read("docs/packages/instantdb-react.md");
  assert.match(docs, /## DX-Style Compatibility/);
  assert.match(docs, /data-dx-style-surface="realtime-app-database"/);
  assert.match(
    docs,
    /realtime_app_database_dx_style_compatibility_present/,
  );
  assert.match(
    docs,
    /realtime-app-database-missing-dx-style-compatibility/,
  );
});
