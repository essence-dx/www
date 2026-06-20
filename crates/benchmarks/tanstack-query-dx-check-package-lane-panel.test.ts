const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const upstream = path.resolve(root, "..", "..", "WWW/inspirations/tanstack-query");
const dataFetchingRunbookFixture =
  "docs/packages/data-fetching-cache.source-guard-runbook.json";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readUpstream(relativePath) {
  return fs.readFileSync(path.join(upstream, relativePath), "utf8");
}

function escaped(marker) {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

test("Data Fetching & Cache package-lane row exposes helper freshness in the dx-check panel", () => {
  const upstreamPackage = JSON.parse(
    readUpstream("packages/react-query/package.json"),
  );
  const upstreamQueryClient = readUpstream(
    "packages/query-core/src/queryClient.ts",
  );
  const reader = read("core/src/ecosystem/dx_check_receipt.rs");
  const packageDoc = read("docs/packages/tanstack-query.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");
  const packageStatus = JSON.parse(
    read("examples/template/.dx/forge/package-status.json"),
  );
  const dataFetchingRow = packageStatus.package_lane_visibility.find(
    (row) => row.package_id === "tanstack/query",
  );

  assert.equal(upstreamPackage.name, "@tanstack/react-query");
  assert.equal(upstreamPackage.version, "5.100.10");
  for (const api of [
    "setQueryDefaults",
    "getQueryDefaults",
    "invalidateQueries",
    "ensureQueryData",
    "prefetchQuery",
    "cancelQueries",
  ]) {
    assert.match(upstreamQueryClient, new RegExp(`\\b${api}\\b`));
  }

  assert.ok(dataFetchingRow, "Data Fetching & Cache package-status row missing");
  assert.equal(dataFetchingRow.official_package_name, "Data Fetching & Cache");
  assert.equal(dataFetchingRow.upstream_package, "@tanstack/react-query");
  assert.equal(dataFetchingRow.upstream_version, "5.100.10");
  assert.equal(
    dataFetchingRow.receipt_hash_refresh.zed_visibility,
    "data-fetching-cache:receipt-hash-refresh",
  );

  for (const marker of [
    'const DATA_FETCHING_CACHE_PACKAGE_ID: &str = "tanstack/query";',
    'const DATA_FETCHING_CACHE_OFFICIAL_NAME: &str = "Data Fetching & Cache";',
    'const DATA_FETCHING_CACHE_UPSTREAM_PACKAGE: &str = "@tanstack/react-query";',
    'const DATA_FETCHING_CACHE_UPSTREAM_VERSION: &str = "5.100.10";',
    'const DATA_FETCHING_CACHE_SOURCE_MIRROR: &str = "G:/WWW/inspirations/tanstack-query";',
    'const DATA_FETCHING_CACHE_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";',
    "DATA_FETCHING_CACHE_PACKAGE_RECEIPT_PATH",
    "data_fetching_cache_receipt_hash_refresh_current",
    "data_fetching_cache_receipt_hash_refresh_stale",
    "data_fetching_cache_receipt_hash_refresh_missing",
    "rows.extend(data_fetching_cache_package_lane_row(root, package_status));",
    "fn data_fetching_cache_package_lane_row(",
    "package_lane_visibility_entry(package_status, DATA_FETCHING_CACHE_PACKAGE_ID)",
    "let receipt_hash_refresh = package_lane_hash_refresh(package);",
    "let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);",
    "metrics: data_fetching_cache_metric_rows(",
    "data_fetching_cache_next_action(status, refresh_stale, refresh_missing)",
    "fn data_fetching_cache_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow",
    "fn data_fetching_cache_metric_rows(",
    "fn data_fetching_cache_status_vocabulary(package: &serde_json::Value) -> Vec<String>",
    "data-fetching-cache:receipt-hash-refresh",
    "dx_check_latest_panel_exposes_data_fetching_cache_package_lane_hash_refresh_row",
  ]) {
    assert.match(reader, escaped(marker), `missing check-panel marker ${marker}`);
  }

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /DX Studio\/check-panel Data Fetching & Cache package row/);
    assert.match(source, /receipt_hash_refresh|receiptHashRefresh/);
    assert.match(source, /data-fetching-cache:receipt-hash-refresh/);
    assert.match(source, /data_fetching_cache_receipt_hash_refresh_current/);
    assert.match(source, /data_fetching_cache_receipt_hash_refresh_stale/);
    assert.match(source, /data_fetching_cache_receipt_hash_refresh_missing/);
    assert.match(source, /without claiming live QueryClient execution/);
  }
});

test("Data Fetching & Cache static launch package-lane template is Studio discoverable", () => {
  const launchShell = read("examples/template/template-shell.tsx");
  const normalizedTemplateShell = launchShell.replace(/\r\n/g, "\n");
  const runtimePage = read("tools/launch/runtime-template/pages/index.html");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const materializerSource = read("tools/launch/materialize-www-template.ts");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const docs = read("docs/packages/tanstack-query.md");
  const frameworkDoc = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  for (const expected of [
    'package_id: "tanstack/query"',
    'official_package_name: "Data Fetching & Cache"',
    'upstream_package: "@tanstack/react-query"',
    'upstream_version: "5.100.10"',
    'source_mirror: "G:/WWW/inspirations/tanstack-query"',
    'package_receipt_path:\n      "examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json"',
    'dx_style_surface: "data-fetching-cache"',
    'token_scope: "tanstack/query"',
    'hash_refresh_status: "current"',
    'hash_refresh_helper:\n      "examples/template/data-fetching-cache-receipt-hashes.ts"',
    'hash_refresh_zed: "data-fetching-cache:receipt-hash-refresh"',
    'hash_refresh_tracked_files: 13',
    'hash_refresh_metric_current:\n      "data_fetching_cache_receipt_hash_refresh_current"',
    'hash_refresh_metric_stale:\n      "data_fetching_cache_receipt_hash_refresh_stale"',
    'hash_refresh_metric_missing:\n      "data_fetching_cache_receipt_hash_refresh_missing"',
  ]) {
    assert.ok(
      normalizedTemplateShell.includes(expected),
      `${expected} missing from Data Fetching & Cache launch shell template`,
    );
  }

  for (const expected of [
    'data-dx-check-package-lane-template="tanstack/query"',
    'data-dx-check-package-lane-row="tanstack/query"',
    'data-dx-check-package-lane-name="Data Fetching &amp; Cache"',
    'data-dx-check-package-lane-status="missing"',
    'data-dx-check-package-lane-receipt-status="missing-receipt"',
    'data-dx-check-package-lane-upstream-package="@tanstack/react-query"',
    'data-dx-check-package-lane-source-mirror="G:/WWW/inspirations/tanstack-query"',
    'data-dx-check-package-lane-receipt-path="examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json"',
    'data-dx-check-package-lane-dx-style-status="present"',
    'data-dx-check-package-lane-hash-refresh-status="current"',
    'data-dx-check-package-lane-hash-refresh-helper="examples/template/data-fetching-cache-receipt-hashes.ts"',
    'data-dx-check-package-lane-hash-refresh-json-command="node tools/launch/run-template-receipt-helper.js examples/template/data-fetching-cache-receipt-hashes.ts --check --json"',
    'data-dx-check-package-lane-hash-refresh-zed="data-fetching-cache:receipt-hash-refresh"',
    'data-dx-check-package-lane-hash-refresh-tracked-files="13"',
    'data-dx-check-package-lane-hash-refresh-stale-files="0"',
    'data-dx-check-package-lane-hash-refresh-missing-files="0"',
    'data-dx-check-package-lane-hash-refresh-current-metric="data_fetching_cache_receipt_hash_refresh_current"',
    'data-dx-check-package-lane-hash-refresh-stale-metric="data_fetching_cache_receipt_hash_refresh_stale"',
    'data-dx-check-package-lane-hash-refresh-missing-metric="data_fetching_cache_receipt_hash_refresh_missing"',
    'data-dx-style-surface="data-fetching-cache"',
    'data-dx-token-scope="tanstack/query"',
    'data-dx-package="tanstack/query"',
  ]) {
    assert.ok(
      runtimePage.includes(expected),
      `${expected} missing from static launch runtime page`,
    );
  }

  assert.match(
    editContract,
    /id: "dx-check-health-panel"[\s\S]*packageIds: \[[\s\S]*"tanstack\/query"/,
  );
  assert.match(
    materializerSource,
    /"launch-runtime-dx-check-panel"[\s\S]*\[[\s\S]*"tanstack\/query"/,
  );
  assert.match(
    studioManifest,
    /"dx-check-health-panel"[\s\S]*&\[[\s\S]*"tanstack\/query"/,
  );

  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-data-fetching-package-lane-"));
  try {
    const result = JSON.parse(
      execFileSync(
        process.execPath,
        [path.join(root, "tools", "launch", "materialize-www-template.ts"), dir],
        {
          cwd: root,
          encoding: "utf8",
        },
      ),
    );
    const launch = fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8");
    const manifest = JSON.parse(
      fs.readFileSync(path.join(dir, "public", "preview-.dx/build-cache/manifest.json"), "utf8"),
    );

    assert.equal(result.ok, true);
    assert.equal(result.noNodeModules, true);
    assert.ok(!fs.existsSync(path.join(dir, "node_modules")));
    assert.match(launch, /data-dx-check-package-lane-row="tanstack\/query"/);
    assert.match(
      launch,
      /data-dx-check-package-lane-hash-refresh-zed="data-fetching-cache:receipt-hash-refresh"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-hash-refresh-current-metric="data_fetching_cache_receipt_hash_refresh_current"/,
    );
    assert.match(launch, /data-dx-style-surface="data-fetching-cache"/);

    const rootRoute = manifest.routes.find((entry) => entry.route === "/");
    assert.ok(rootRoute, "expected generated / route metadata");
    assert.ok(
      rootRoute.forgePackages.includes("3d/launch-scene"),
      "generated / route should stay scoped to the 3D landing scene package",
    );
    assert.ok(
      rootRoute.forgePackages.includes("tanstack/query"),
      "generated / route package scope must include Data Fetching & Cache",
    );

    const launchRouteMetadata = manifest.routes.find((entry) => entry.route === "/");
    assert.ok(launchRouteMetadata, "expected generated / route metadata");
    assert.ok(
      launchRouteMetadata.forgePackages.includes("tanstack/query"),
      "generated / route package scope must include Data Fetching & Cache",
    );

    const globalRunbookFixture = manifest.sourceGuardRunbookFixtures.find(
      (fixture) => fixture.fixture === dataFetchingRunbookFixture,
    );
    assert.ok(
      globalRunbookFixture,
      "generated preview manifest must expose the Data Fetching & Cache source-guard runbook fixture",
    );
    assert.equal(globalRunbookFixture.packageId, "tanstack/query");
    assert.equal(globalRunbookFixture.officialPackageName, "Data Fetching & Cache");
    assert.equal(globalRunbookFixture.upstreamPackage, "@tanstack/react-query");
    assert.equal(globalRunbookFixture.upstreamVersion, "5.100.10");
    assert.equal(globalRunbookFixture.sourceMirror, "G:/WWW/inspirations/tanstack-query");
    assert.equal(
      globalRunbookFixture.guardId,
      "data-fetching-cache-generated-starter-materialization",
    );
    assert.equal(globalRunbookFixture.honestyLabel, "SOURCE-ONLY");
    assert.equal(globalRunbookFixture.runtimeProof, false);
    assert.equal(
      globalRunbookFixture.zedVisibility,
      "data-fetching-cache:receipt-hash-refresh",
    );

    const launchRoute = manifest.routes.find((entry) => entry.route === "/");
    assert.ok(
      launchRoute.sourceGuardRunbookFixtures.includes(dataFetchingRunbookFixture),
      "generated / route must point to the Data Fetching & Cache runbook fixture",
    );

    const checkPanel = manifest.editContract.editableSurfaces.find(
      (surface) => surface.id === "launch-runtime-dx-check-panel",
    );
    assert.ok(checkPanel, "expected generated dx-check panel edit surface");
    assert.ok(checkPanel.packageIds.includes("tanstack/query"));
    for (const marker of [
      "data-dx-check-package-lane-template",
      "data-dx-check-package-lane-row",
      "data-dx-check-package-lane-hash-refresh-zed",
      "data-dx-check-package-lane-hash-refresh-current-metric",
      "data-dx-style-surface",
      "data-dx-token-scope",
    ]) {
      assert.ok(
        checkPanel.stateMarkers.includes(marker),
        `generated dx-check panel must expose ${marker}`,
      );
    }
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }

  for (const source of [docs, frameworkDoc, dx, todo, changelog]) {
    assert.match(source, /Data Fetching & Cache static \/ package-lane template/);
    assert.match(source, /data-dx-check-package-lane-template="tanstack\/query"/);
    assert.match(source, /sourceGuardRunbookFixtures/);
    assert.match(source, /docs\/packages\/data-fetching-cache\.source-guard-runbook\.json/);
    assert.match(source, /data_fetching_cache_receipt_hash_refresh_current/);
    assert.match(source, /without claiming live QueryClient execution/);
  }
});

test("Data Fetching & Cache generated-starter guard is published in the Studio source runbook", () => {
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const frameworkDoc = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const packageDoc = read("docs/packages/tanstack-query.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  for (const expected of [
    "data-fetching-cache-generated-starter-materialization",
    "Data Fetching & Cache generated-starter materialization guard",
    'data-dx-check-package-lane-row=\\"tanstack/query\\"',
    'data-dx-token-scope=\\"tanstack/query\\"',
    "data-fetching-cache:receipt-hash-refresh",
    "tanstack/query source-only Studio discovery",
    "without live QueryClient execution proof",
    String.raw`dx run --test .\\benchmarks\\tanstack-query-dx-check-package-lane-panel.test.ts`,
  ]) {
    assert.ok(
      studioManifest.includes(expected),
      `${expected} missing from Studio source-guard manifest`,
    );
  }

  assert.match(
    studioManifest,
    /source_guard_contract\(\s*"data-fetching-cache-generated-starter-materialization"[\s\S]*benchmarks\/tanstack-query-dx-check-package-lane-panel\.test\.ts/,
  );
  assert.match(
    studioManifest,
    /source_guard_command\(\s*"dx run --test \.\\\\benchmarks\\\\tanstack-query-dx-check-package-lane-panel\.test\.ts"[\s\S]*Data Fetching & Cache package-lane row/,
  );
  assert.match(
    studioManifest,
    /"\/" => guards\.extend\(\[[\s\S]*"data-fetching-cache-generated-starter-materialization"/,
  );

  for (const source of [frameworkDoc, packageDoc, dx, todo, changelog]) {
    assert.match(source, /Data Fetching & Cache Studio source-guard\/runbook entry/);
    assert.match(source, /data-fetching-cache-generated-starter-materialization/);
    assert.match(
      source,
      /dx run --test \.\\benchmarks\\tanstack-query-dx-check-package-lane-panel\.test\.ts/,
    );
    assert.match(source, /without claiming live QueryClient execution/);
  }
});

test("Data Fetching & Cache source-guard runbook fixture mirrors the Studio contract", () => {
  const fixturePath = "docs/packages/data-fetching-cache.source-guard-runbook.json";
  const fixture = JSON.parse(read(fixturePath));
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const frameworkDoc = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const packageDoc = read("docs/packages/tanstack-query.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(
    fixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(fixture.route, "/");
  assert.equal(fixture.package.official_package_name, "Data Fetching & Cache");
  assert.equal(fixture.package.package_id, "tanstack/query");
  assert.equal(fixture.package.upstream_package, "@tanstack/react-query");
  assert.equal(fixture.package.upstream_version, "5.100.10");
  assert.deepEqual(fixture.package.source_mirrors, [
    "G:/WWW/inspirations/tanstack-query",
  ]);
  assert.deepEqual(fixture.selected_surfaces, [
    "query-dashboard-workflow",
    "starter-dashboard-workflow",
    "template-dashboard-cache-readiness",
    "app-router-cache-readiness-route",
    "receipt-hash-refresh",
  ]);

  for (const api of [
    "QueryClient",
    "QueryClientProvider",
    "useQuery",
    "useMutation",
    "setQueryDefaults",
    "getQueryDefaults",
    "invalidateQueries",
    "ensureQueryData",
    "prefetchQuery",
    "cancelQueries",
  ]) {
    assert.ok(
      fixture.upstream_public_apis.includes(api),
      `${api} missing from Data Fetching & Cache runbook fixture`,
    );
  }

  assert.equal(
    fixture.guard.id,
    "data-fetching-cache-generated-starter-materialization",
  );
  assert.deepEqual(fixture.guard.routes, ["/"]);
  assert.equal(
    fixture.guard.guard_file,
    "benchmarks/tanstack-query-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(
    fixture.guard.command,
    "dx run --test .\\benchmarks\\tanstack-query-dx-check-package-lane-panel.test.ts",
  );
  for (const expected of [
    "Data Fetching & Cache generated-starter materialization guard",
    'data-dx-check-package-lane-row="tanstack/query"',
    'data-dx-token-scope="tanstack/query"',
    "data-fetching-cache:receipt-hash-refresh",
    "docs/packages/data-fetching-cache.source-guard-runbook.json",
    "without live QueryClient execution proof",
    "tanstack/query source-only Studio discovery",
  ]) {
    assert.ok(
      fixture.guard.proves.includes(expected),
      `${expected} missing from Data Fetching & Cache runbook fixture proof list`,
    );
  }
  assert.equal(fixture.guard.execution_policy, "source-only");
  assert.equal(fixture.guard.writes_files, false);
  assert.equal(fixture.guard.starts_server, false);
  assert.equal(fixture.guard.runs_package_install, false);
  assert.equal(fixture.guard.runs_full_build, false);
  assert.equal(fixture.guard.node_modules_required, false);

  assert.equal(fixture.runbook.index_field, "source_guard_runbook_index");
  assert.equal(fixture.runbook.default_action, "show-source-only-runbook");
  assert.equal(
    fixture.runbook.contract.id,
    "data-fetching-cache-generated-starter-materialization",
  );
  assert.equal(
    fixture.runbook.contract.evidence_field,
    "benchmarks/tanstack-query-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(
    fixture.runbook.command.command,
    "dx run --test .\\benchmarks\\tanstack-query-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(fixture.runbook.command.scope, "source-only");
  assert.equal(fixture.runbook.command.starts_server, false);
  assert.equal(fixture.runbook.command.runs_package_install, false);
  assert.equal(fixture.runbook.command.runs_full_build, false);
  assert.equal(fixture.runbook.command.writes_files, false);
  assert.equal(fixture.runbook.command.node_modules_required, false);
  assert.equal(fixture.preview_manifest.generated_file, "public/preview-.dx/build-cache/manifest.json");
  assert.equal(fixture.preview_manifest.root_field, "sourceGuardRunbookFixtures");
  assert.equal(
    fixture.preview_manifest.route_field,
    "routes[].sourceGuardRunbookFixtures",
  );
  assert.equal(fixture.preview_manifest.route, "/");
  assert.equal(fixture.preview_manifest.fixture_path, fixturePath);
  assert.equal(
    fixture.preview_manifest.guard_id,
    "data-fetching-cache-generated-starter-materialization",
  );
  assert.equal(fixture.preview_manifest.honesty_label, "SOURCE-ONLY");
  assert.equal(fixture.preview_manifest.runtime_proof, false);

  for (const marker of [
    "data-dx-check-package-lane-row",
    "data-dx-check-package-lane-hash-refresh-helper",
    "data-dx-check-package-lane-hash-refresh-json-command",
    "data-dx-check-package-lane-hash-refresh-zed",
    "data-dx-style-surface",
    "data-dx-token-scope",
    "data-dx-package",
  ]) {
    assert.ok(
      fixture.zed_dx_studio_markers.includes(marker),
      `${marker} missing from Data Fetching & Cache runbook fixture`,
    );
  }

  assert.equal(
    fixture.receipt.path,
    "examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json",
  );
  assert.equal(
    fixture.receipt.hash_helper,
    "examples/template/data-fetching-cache-receipt-hashes.ts",
  );
  assert.equal(
    fixture.receipt.hash_helper_json_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/data-fetching-cache-receipt-hashes.ts --check --json",
  );
  assert.equal(
    fixture.receipt.zed_visibility,
    "data-fetching-cache:receipt-hash-refresh",
  );
  assert.equal(fixture.honesty_label, "SOURCE-ONLY");
  assert.equal(fixture.runtime_proof, false);

  for (const source of [
    studioManifest,
    frameworkDoc,
    packageDoc,
    dx,
    todo,
    changelog,
  ]) {
    assert.match(source, /docs\/packages\/data-fetching-cache\.source-guard-runbook\.json/);
    assert.match(source, /data-fetching-cache-generated-starter-materialization/);
    assert.match(source, /without claiming live QueryClient execution/);
  }
});
