const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  const source = path.join(root, relativePath);
  if (fs.existsSync(source)) {
    return fs.readFileSync(source, "utf8");
  }

  if (
    relativePath.startsWith("tools/launch/runtime-template/pages/") &&
    relativePath.endsWith(".html")
  ) {
    const htmlSource = source.replace(/\.html$/, ".html");
    if (fs.existsSync(htmlSource)) {
      return fs.readFileSync(htmlSource, "utf8");
    }
  }

  return fs.readFileSync(source, "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

test("Type-Safe API package lane is exposed in the dx-check panel view model", () => {
  const dxCheckReceipt = read("core/src/ecosystem/dx_check_receipt.rs");

  for (const expected of [
    'const TYPE_SAFE_API_PACKAGE_ID: &str = "api/trpc";',
    'const TYPE_SAFE_API_OFFICIAL_NAME: &str = "Type-Safe API";',
    'const TYPE_SAFE_API_UPSTREAM_PACKAGE: &str = "@trpc/server";',
    'const TYPE_SAFE_API_UPSTREAM_VERSION: &str = "11.17.0";',
    'const TYPE_SAFE_API_SOURCE_MIRROR: &str = "G:/WWW/inspirations/trpc";',
    'const TYPE_SAFE_API_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";',
    "TYPE_SAFE_API_PACKAGE_RECEIPT_PATH",
    "type_safe_api_package_present",
    "type_safe_api_receipt_present",
    "type_safe_api_receipt_stale",
    "type_safe_api_missing_receipt",
    "type_safe_api_blocked_surface",
    "type_safe_api_unsupported_surface",
    "type_safe_api_hash_manifest_present",
    "type_safe_api_hash_mismatch",
    "type_safe_api_receipt_hash_refresh_current",
    "type_safe_api_receipt_hash_refresh_stale",
    "type_safe_api_receipt_hash_refresh_missing",
    "rows.extend(type_safe_api_package_lane_row(root, package_status));",
    "fn type_safe_api_package_lane_row(",
    "package_lane_visibility_entry(package_status, TYPE_SAFE_API_PACKAGE_ID)",
    "let receipt_hash_refresh = package_lane_hash_refresh(package);",
    "receipt_hash_refresh_counts(package)",
    "status_vocabulary: type_safe_api_status_vocabulary(package)",
    "metrics: type_safe_api_metric_rows(",
    "type_safe_api_next_action(status, refresh_stale, refresh_missing)",
    "fn type_safe_api_missing_receipt_row(next_action: &str) -> DxCheckPanelPackageLaneRow",
    "fn type_safe_api_metric_rows(",
    "fn type_safe_api_status_vocabulary(package: &serde_json::Value) -> Vec<String>",
    "fn type_safe_api_selected_surfaces(",
    'value_at(visibility, &["unsupported_surfaces"])',
    'reason: json_text(unsupported, &["reason"]).map(str::to_string)',
    'app_owned_boundary: json_text(unsupported, &["app_owned_boundary"])',
    "dx_check_latest_panel_exposes_type_safe_api_unsupported_surface_context",
  ]) {
    assert.ok(
      dxCheckReceipt.includes(expected),
      `${expected} missing from dx-check receipt panel reader`,
    );
  }

  assert.match(
    dxCheckReceipt,
    /fn type_safe_api_next_action\(\s*status: &str,\s*refresh_stale: u64,\s*refresh_missing: u64,\s*\) -> &'static str/,
  );
});

test("Type-Safe API docs describe check-panel helper freshness metrics", () => {
  const packageDoc = read("docs/packages/api-trpc.md");

  for (const expected of [
    "check_panel.view_model.package_lane_rows",
    "type-safe-api:receipt-hash-refresh",
    "receipt_hash_refresh",
    "type_safe_api_receipt_hash_refresh_current",
    "type_safe_api_receipt_hash_refresh_stale",
    "type_safe_api_receipt_hash_refresh_missing",
    "unsupported requested Type-Safe API surfaces",
    "app-owned boundary",
    "trpc-websocket-subscriptions",
  ]) {
    assert.ok(
      packageDoc.includes(expected),
      `${expected} missing from Type-Safe API package docs`,
    );
  }
});

test("Type-Safe API package-lane row survives generated starter materialization", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-trpc-package-lane-"));
  const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");

  const result = JSON.parse(
    execFileSync(process.execPath, [materializer, dir], {
      cwd: root,
      encoding: "utf8",
    }),
  );
  const launch = fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8");
  const manifest = JSON.parse(
    fs.readFileSync(path.join(dir, "public", "preview-.dx/build-cache/manifest.json"), "utf8"),
  );

  assert.equal(result.ok, true);
  assert.equal(result.noNodeModules, true);
  assert.ok(!fs.existsSync(path.join(dir, "node_modules")));
  assert.match(launch, /data-dx-check-package-lane-template="api\/trpc"/);
  assert.match(launch, /data-dx-check-package-lane-row="api\/trpc"/);
  assert.match(launch, /data-dx-check-package-lane-name="Type-Safe API"/);
  assert.match(launch, /data-dx-check-package-lane-status="missing"/);
  assert.match(launch, /data-dx-check-package-lane-receipt-status="missing-receipt"/);
  assert.match(
    launch,
    /data-dx-check-package-lane-receipt-path="examples\/template\/\.dx\/forge\/receipts\/2026-05-22-api-trpc-dashboard-workflow\.json"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-helper="examples\/template\/type-safe-api-receipt-hashes\.ts"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-zed="type-safe-api:receipt-hash-refresh"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-current-metric="type_safe_api_receipt_hash_refresh_current"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-stale-metric="type_safe_api_receipt_hash_refresh_stale"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-missing-metric="type_safe_api_receipt_hash_refresh_missing"/,
  );

  const launchRoute = manifest.routes.find((route) => route.route === "/");
  assert.ok(launchRoute, "expected materialized /launch route metadata");
  assert.ok(launchRoute.forgePackages.includes("api/trpc"));

  assert.ok(
    Array.isArray(manifest.sourceGuardRunbookFixtures),
    "generated preview manifest must expose source-guard runbook fixtures",
  );
  const typeSafeApiRunbookFixture = manifest.sourceGuardRunbookFixtures.find(
    (fixture) => fixture.packageId === "api/trpc",
  );
  assert.ok(
    typeSafeApiRunbookFixture,
    "generated preview manifest must expose the Type-Safe API source-guard runbook fixture",
  );
  assert.equal(typeSafeApiRunbookFixture.officialPackageName, "Type-Safe API");
  assert.equal(typeSafeApiRunbookFixture.upstreamPackage, "@trpc/server");
  assert.equal(typeSafeApiRunbookFixture.upstreamVersion, "11.17.0");
  assert.equal(typeSafeApiRunbookFixture.sourceMirror, "G:/WWW/inspirations/trpc");
  assert.equal(typeSafeApiRunbookFixture.route, "/");
  assert.equal(
    typeSafeApiRunbookFixture.fixture,
    "docs/packages/api-trpc.source-guard-runbook.json",
  );
  assert.equal(
    typeSafeApiRunbookFixture.guardId,
    "type-safe-api-unsupported-surface-context",
  );
  assert.equal(typeSafeApiRunbookFixture.honestyLabel, "SOURCE-ONLY");
  assert.equal(typeSafeApiRunbookFixture.runtimeProof, false);
  assert.equal(
    typeSafeApiRunbookFixture.zedVisibility,
    "type-safe-api:receipt-hash-refresh",
  );
  assert.ok(
    launchRoute.sourceGuardRunbookFixtures.includes(
      "docs/packages/api-trpc.source-guard-runbook.json",
    ),
    "the materialized /launch route must point at the Type-Safe API runbook fixture",
  );

  const checkPanel = manifest.editContract.editableSurfaces.find(
    (surface) => surface.id === "launch-runtime-dx-check-panel",
  );
  assert.ok(checkPanel, "expected dx-check panel edit surface");
  assert.equal(checkPanel.sourceFile, "pages/index.html");
  assert.ok(
    checkPanel.packageIds.includes("api/trpc"),
    "generated dx-check panel package scope must include Type-Safe API",
  );
  assert.ok(
    checkPanel.stateMarkers.includes("data-dx-check-package-lane-hash-refresh-helper"),
  );
  assert.ok(
    checkPanel.stateMarkers.includes("data-dx-check-package-lane-hash-refresh-zed"),
  );

  for (const source of [
    read("docs/packages/api-trpc.md"),
    read("DX.md"),
    read("TODO.md"),
    read("CHANGELOG.md"),
  ]) {
    assert.match(source, /[Gg]enerated-starter materialization guard for Type-Safe API/);
    assert.match(source, /sourceGuardRunbookFixtures/);
    assert.match(source, /api-trpc\.source-guard-runbook\.json/);
    assert.match(source, /without\s+claiming\s+live Type-Safe API runtime proof/);
  }
});

test("Type-Safe API static launch package-lane markers are Studio discoverable", () => {
  const launchShell = read("examples/template/template-shell.tsx");
  const normalizedTemplateShell = launchShell.replace(/\r\n/g, "\n");
  const runtimeLaunch = read("tools/launch/runtime-template/pages/index.html");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const materializer = read("tools/launch/materialize-www-template.ts");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const packageDoc = read("docs/packages/api-trpc.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  for (const expected of [
    'package_id: "api/trpc"',
    'official_package_name: "Type-Safe API"',
    'upstream_package: "@trpc/server"',
    'upstream_version: "11.17.0"',
    'source_mirror: "G:/WWW/inspirations/trpc"',
    'package_receipt_path:\n      "examples/template/.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json"',
    'hash_refresh_status: "current"',
    'hash_refresh_helper: "examples/template/type-safe-api-receipt-hashes.ts"',
    'hash_refresh_zed: "type-safe-api:receipt-hash-refresh"',
    "hash_refresh_tracked_files: 7",
    'hash_refresh_current_file_list:\n      "core/src/ecosystem/forge_trpc.rs|examples/template/trpc-launch-health.tsx|examples/template/trpc-launch-contract.ts|examples/dashboard/src/components/TrpcDashboardWorkflow.tsx|examples/dashboard/src/lib/trpcDashboardWorkflow.ts|docs/packages/api-trpc.source-guard-runbook.json|tools/launch/materialize-www-template.ts"',
    'hash_refresh_stale_file_list: ""',
    'hash_refresh_missing_file_list: ""',
    'hash_refresh_stale_mirror_file_list: ""',
    'hash_refresh_missing_mirror_file_list: ""',
    'hash_refresh_metric_current: "type_safe_api_receipt_hash_refresh_current"',
    'hash_refresh_metric_stale: "type_safe_api_receipt_hash_refresh_stale"',
    'hash_refresh_metric_missing: "type_safe_api_receipt_hash_refresh_missing"',
    'data-dx-check-package-lane-hash-refresh-status={packageLane.hash_refresh_status ?? "missing"}',
    'data-dx-check-package-lane-hash-refresh-helper={packageLane.hash_refresh_helper ?? ""}',
    'data-dx-check-package-lane-hash-refresh-json-command={packageLane.hash_refresh_json_command ?? ""}',
    'data-dx-check-package-lane-hash-refresh-zed={packageLane.hash_refresh_zed ?? ""}',
    'data-dx-check-package-lane-hash-refresh-current-file-list={packageLane.hash_refresh_current_file_list ?? ""}',
    'data-dx-check-package-lane-hash-refresh-missing-file-list={packageLane.hash_refresh_missing_file_list ?? ""}',
    'data-dx-check-package-lane-hash-refresh-stale-mirror-file-list={packageLane.hash_refresh_stale_mirror_file_list ?? ""}',
    'data-dx-check-package-lane-hash-refresh-missing-mirror-file-list={packageLane.hash_refresh_missing_mirror_file_list ?? ""}',
  ]) {
    assert.ok(
      normalizedTemplateShell.includes(expected),
      `${expected} missing from Type-Safe API launch shell package template`,
    );
  }

  for (const expected of [
    'data-dx-check-package-lane-template="api/trpc"',
    'data-dx-check-package-lane-row="api/trpc"',
    'data-dx-check-package-lane-name="Type-Safe API"',
    'data-dx-check-package-lane-status="missing"',
    'data-dx-check-package-lane-receipt-status="missing-receipt"',
    'data-dx-check-package-lane-upstream-package="@trpc/server"',
    'data-dx-check-package-lane-source-mirror="G:/WWW/inspirations/trpc"',
    'data-dx-check-package-lane-receipt-path="examples/template/.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json"',
    'data-dx-check-package-lane-hash-refresh-status="current"',
    'data-dx-check-package-lane-hash-refresh-helper="examples/template/type-safe-api-receipt-hashes.ts"',
    'data-dx-check-package-lane-hash-refresh-json-command="node tools/launch/run-template-receipt-helper.js examples/template/type-safe-api-receipt-hashes.ts --check --json"',
    'data-dx-check-package-lane-hash-refresh-zed="type-safe-api:receipt-hash-refresh"',
    'data-dx-check-package-lane-hash-refresh-tracked-files="7"',
    'data-dx-check-package-lane-hash-refresh-stale-files="0"',
    'data-dx-check-package-lane-hash-refresh-missing-files="0"',
    'data-dx-check-package-lane-hash-refresh-current-file-list="core/src/ecosystem/forge_trpc.rs|examples/template/trpc-launch-health.tsx|examples/template/trpc-launch-contract.ts|examples/dashboard/src/components/TrpcDashboardWorkflow.tsx|examples/dashboard/src/lib/trpcDashboardWorkflow.ts|docs/packages/api-trpc.source-guard-runbook.json|tools/launch/materialize-www-template.ts"',
    'data-dx-check-package-lane-hash-refresh-stale-file-list=""',
    'data-dx-check-package-lane-hash-refresh-missing-file-list=""',
    'data-dx-check-package-lane-hash-refresh-stale-mirror-file-list=""',
    'data-dx-check-package-lane-hash-refresh-missing-mirror-file-list=""',
    'data-dx-check-package-lane-hash-refresh-current-metric="type_safe_api_receipt_hash_refresh_current"',
    'data-dx-check-package-lane-hash-refresh-stale-metric="type_safe_api_receipt_hash_refresh_stale"',
    'data-dx-check-package-lane-hash-refresh-missing-metric="type_safe_api_receipt_hash_refresh_missing"',
    'data-dx-package="api/trpc"',
  ]) {
    assert.ok(
      runtimeLaunch.includes(expected),
      `${expected} missing from static launch runtime page`,
    );
  }

  assert.match(
    editContract,
    /id: "dx-check-health-panel"[\s\S]*packageIds: \[[\s\S]*"api\/trpc"/,
  );
  assert.match(
    materializer,
    /"launch-runtime-dx-check-panel"[\s\S]*\[[\s\S]*"api\/trpc"/,
  );
  assert.match(
    studioManifest,
    /"dx-check-health-panel"[\s\S]*&\[[\s\S]*"api\/trpc"/,
  );
  assert.match(
    studioManifest,
    /"package": "api\/trpc"[\s\S]*"front_facing_name": "Type-Safe API Dashboard"[\s\S]*"data-dx-check-package-lane-hash-refresh-zed"/,
  );
  for (const source of [editContract, materializer, studioManifest]) {
    for (const marker of [
      "data-dx-check-package-lane-hash-refresh-current-file-list",
      "data-dx-check-package-lane-hash-refresh-missing-file-list",
      "data-dx-check-package-lane-hash-refresh-stale-mirror-file-list",
      "data-dx-check-package-lane-hash-refresh-missing-mirror-file-list",
    ]) {
      assert.match(source, new RegExp(marker), `${marker} missing from marker registry`);
    }
  }

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /Type-Safe API static launch package-lane template/);
    assert.match(source, /Type-Safe API static helper metric markers/);
    assert.match(source, /static helper path array markers/);
    assert.match(source, /data-dx-check-package-lane-template="api\/trpc"/);
    assert.match(source, /type_safe_api_receipt_hash_refresh_current/);
    assert.match(source, /type-safe-api:receipt-hash-refresh/);
    assert.match(source, /without claiming\s+live tRPC route execution/);
  }
});

test("Type-Safe API source-guard runbook fixture publishes unsupported-surface proof command", () => {
  const fixture = readJson("docs/packages/api-trpc.source-guard-runbook.json");
  const packageDoc = read("docs/packages/api-trpc.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(
    fixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(fixture.route, "/");
  assert.equal(fixture.package.official_package_name, "Type-Safe API");
  assert.equal(fixture.package.package_id, "api/trpc");
  assert.equal(fixture.package.upstream_package, "@trpc/server");
  assert.equal(fixture.package.upstream_version, "11.17.0");
  assert.equal(fixture.package.source_mirror, "G:/WWW/inspirations/trpc");

  assert.equal(
    fixture.guard.id,
    "type-safe-api-unsupported-surface-context",
  );
  assert.equal(
    fixture.guard.guard_file,
    "core/src/ecosystem/dx_check_receipt.rs",
  );
  assert.equal(
    fixture.guard.command,
    "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_type_safe_api_unsupported_surface_context --lib",
  );
  assert.equal(fixture.guard.execution_policy, "source-only");
  assert.equal(fixture.guard.writes_files, false);
  assert.equal(fixture.guard.starts_server, false);
  assert.equal(fixture.guard.runs_package_install, false);
  assert.equal(fixture.guard.runs_full_build, false);
  assert.equal(fixture.guard.node_modules_required, false);

  for (const proof of [
    "Type-Safe API unsupported requested surfaces remain visible in check_panel.view_model.package_lane_rows",
    'selected_surfaces status="unsupported-surface" for trpc-websocket-subscriptions',
    "reason and app_owned_boundary stay attached to the unsupported requested surface",
    "type_safe_api_unsupported_surface increments without claiming subscription runtime support",
    "docs/packages/api-trpc.source-guard-runbook.json",
    "without claiming live tRPC subscriptions",
  ]) {
    assert.ok(fixture.guard.proves.includes(proof), `missing proof ${proof}`);
  }

  assert.equal(fixture.runbook.index_field, "source_guard_runbook_index");
  assert.equal(fixture.runbook.default_action, "show-source-only-runbook");
  assert.equal(
    fixture.runbook.contract.id,
    "type-safe-api-unsupported-surface-context",
  );
  assert.equal(
    fixture.runbook.contract.evidence_field,
    "core/src/ecosystem/dx_check_receipt.rs",
  );
  assert.equal(
    fixture.runbook.command.command,
    "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_type_safe_api_unsupported_surface_context --lib",
  );
  assert.ok(
    fixture.preview_manifest,
    "Type-Safe API runbook fixture must describe its preview-manifest exposure",
  );
  assert.equal(fixture.preview_manifest.generated_file, "public/preview-.dx/build-cache/manifest.json");
  assert.equal(
    fixture.preview_manifest.materializer,
    "tools/launch/materialize-www-template.ts",
  );
  assert.equal(fixture.preview_manifest.field, "sourceGuardRunbookFixtures");
  assert.equal(
    fixture.preview_manifest.route_field,
    "routes[].sourceGuardRunbookFixtures",
  );
  assert.equal(
    fixture.preview_manifest.fixture_path,
    "docs/packages/api-trpc.source-guard-runbook.json",
  );
  assert.equal(fixture.preview_manifest.guard_id, "type-safe-api-unsupported-surface-context");
  assert.equal(fixture.preview_manifest.package_id, "api/trpc");
  assert.equal(fixture.preview_manifest.honesty_label, "SOURCE-ONLY");
  assert.equal(fixture.preview_manifest.runtime_proof, false);
  assert.equal(
    fixture.preview_manifest.zed_visibility,
    "type-safe-api:receipt-hash-refresh",
  );

  for (const api of [
    "initTRPC.context().create()",
    "createCallerFactory",
    "fetchRequestHandler",
    "createTRPCClient",
    "httpBatchLink",
    "createTRPCReact",
  ]) {
    assert.ok(fixture.upstream_public_apis.includes(api), `missing API ${api}`);
  }

  for (const marker of [
    "check_panel.view_model.package_lane_rows",
    "type_safe_api_unsupported_surface",
    "type-safe-api-unsupported-surface",
    "trpc-websocket-subscriptions",
    "reason",
    "app_owned_boundary",
  ]) {
    assert.ok(
      fixture.zed_dx_studio_markers.includes(marker),
      `fixture must expose ${marker}`,
    );
  }

  assert.equal(
    fixture.receipt.path,
    "examples/template/.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json",
  );
  assert.equal(
    fixture.receipt.hash_helper,
    "examples/template/type-safe-api-receipt-hashes.ts",
  );
  assert.equal(
    fixture.receipt.zed_visibility,
    "type-safe-api:receipt-hash-refresh",
  );
  assert.equal(fixture.honesty_label, "SOURCE-ONLY");
  assert.equal(fixture.runtime_proof, false);
  assert.match(
    fixture.runtime_limitations.join("\n"),
    /live tRPC subscriptions/,
  );
  assert.match(
    fixture.app_owned_boundaries.join("\n"),
    /Subscription transport, connection authorization, stream fan-out, retry policy, and hosted runtime limits stay app-owned/,
  );

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /api-trpc\.source-guard-runbook\.json/);
    assert.match(source, /type-safe-api-unsupported-surface-context/);
    assert.match(source, /without claiming live tRPC subscriptions/);
  }
});
