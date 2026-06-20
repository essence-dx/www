const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const upstream = path.resolve(root, "..", "..", "WWW/inspirations/supabase");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function readUpstream(relativePath) {
  return fs.readFileSync(path.join(upstream, relativePath), "utf8");
}

function escaped(marker) {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

test("Backend Platform Client package-lane row exposes receipt helper freshness in dx-check panel", () => {
  const upstreamAccountForm = readUpstream(
    "examples/user-management/nextjs-user-management/app/account/account-form.tsx",
  );
  const upstreamBrowserClient = readUpstream(
    "examples/user-management/nextjs-user-management/lib/supabase/client.ts",
  );
  const upstreamServerClient = readUpstream(
    "examples/user-management/nextjs-user-management/lib/supabase/server.ts",
  );
  const reader = read("core/src/ecosystem/dx_check_receipt.rs");
  const launchShell = read("examples/template/template-shell.tsx");
  const runtimeLaunchPage = read("tools/launch/runtime-template/pages/index.html");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const materializer = read("tools/launch/materialize-www-template.ts");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const packageDoc = read("docs/packages/supabase-client.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.match(upstreamBrowserClient, /createBrowserClient/);
  assert.match(upstreamServerClient, /createServerClient/);
  assert.match(upstreamAccountForm, /\.from\('profiles'\)/);
  assert.match(upstreamAccountForm, /\.select\(`full_name, username, website, avatar_url`\)/);
  assert.match(upstreamAccountForm, /\.upsert\(\{/);

  for (const marker of [
    'BACKEND_PLATFORM_CLIENT_PACKAGE_ID: &str = "supabase/client"',
    'BACKEND_PLATFORM_CLIENT_OFFICIAL_NAME: &str = "Backend Platform Client"',
    'BACKEND_PLATFORM_CLIENT_UPSTREAM_PACKAGE: &str = "@supabase/ssr + @supabase/supabase-js"',
    'BACKEND_PLATFORM_CLIENT_SOURCE_MIRROR: &str = "G:/WWW/inspirations/supabase"',
    'BACKEND_PLATFORM_CLIENT_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json"',
    "rows.extend(backend_platform_client_package_lane_row(root, package_status));",
    "fn backend_platform_client_package_lane_row(",
    "fn backend_platform_client_metric_rows(",
    "fn backend_platform_client_next_action(status: &str)",
    "let receipt_hash_refresh = package_lane_hash_refresh(package);",
    "backend_platform_client_hash_manifest_present",
    "backend_platform_client_hash_mismatch",
    "backend_platform_client_receipt_hash_refresh_current",
    "backend_platform_client_receipt_hash_refresh_stale",
    "backend_platform_client_receipt_hash_refresh_missing",
    "backend-platform-client:receipt-hash-refresh",
    "dx_check_latest_panel_exposes_backend_platform_client_package_lane_hash_refresh_row",
  ]) {
    assert.match(reader, escaped(marker), `missing check-panel marker ${marker}`);
  }
  assert.match(
    reader,
    /let \(refresh_current, refresh_stale, refresh_missing\) = receipt_hash_refresh_counts\(package\);/,
  );
  assert.match(
    reader,
    /metric_value\("backend_platform_client_receipt_hash_refresh_current"\)/,
  );
  assert.match(
    reader,
    /helper_stale_metric_value\("backend_platform_client_receipt_hash_refresh_stale"\)/,
  );
  assert.match(
    reader,
    /helper_stale_metric_value\("backend_platform_client_hash_mismatch"\),\s*0/,
  );

  for (const marker of [
    "type DxCheckPanelPackageLaneHashRefreshRow = {",
    "receipt_hash_refresh?: DxCheckPanelPackageLaneHashRefreshRow | null;",
    'package_id: "supabase/client"',
    'official_package_name: "Backend Platform Client"',
    "examples/template/backend-platform-client-receipt-hashes.ts",
    'hash_refresh_zed: "backend-platform-client:receipt-hash-refresh"',
    "data-dx-check-package-lane-hash-refresh-status",
    "data-dx-check-package-lane-hash-refresh-helper",
    "data-dx-check-package-lane-hash-refresh-json-command",
    "data-dx-check-package-lane-hash-refresh-zed",
    "data-dx-check-package-lane-hash-refresh-tracked-files",
    "data-dx-check-package-lane-hash-refresh-stale-files",
    "data-dx-check-package-lane-hash-refresh-missing-files",
  ]) {
    assert.match(launchShell, escaped(marker), `missing template-shell marker ${marker}`);
  }

  for (const marker of [
    'data-dx-check-package-lane-template="supabase/client"',
    'data-dx-check-package-lane-row="supabase/client"',
    'data-dx-check-package-lane-name="Backend Platform Client"',
    'data-dx-check-package-lane-upstream-package="@supabase/ssr + @supabase/supabase-js"',
    'data-dx-check-package-lane-source-mirror="G:/WWW/inspirations/supabase"',
    'data-dx-check-package-lane-receipt-path="examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json"',
    'data-dx-check-package-lane-hash-refresh-helper="examples/template/backend-platform-client-receipt-hashes.ts"',
    'data-dx-check-package-lane-hash-refresh-json-command="node tools/launch/run-template-receipt-helper.js examples/template/backend-platform-client-receipt-hashes.ts --check --json"',
    'data-dx-check-package-lane-hash-refresh-zed="backend-platform-client:receipt-hash-refresh"',
    'data-dx-check-package-lane-hash-refresh-tracked-files="12"',
    'data-dx-check-package-lane-hash-refresh-stale-files="0"',
    'data-dx-check-package-lane-hash-refresh-missing-files="0"',
  ]) {
    assert.match(runtimeLaunchPage, escaped(marker), `missing static launch marker ${marker}`);
  }

  for (const source of [editContract, materializer, studioManifest]) {
    assert.match(
      source,
      /dx-check-health-panel[\s\S]*supabase\/client/,
      "dx-check panel package filter must include Backend Platform Client",
    );
  }

  for (const source of [editContract, materializer, studioManifest]) {
    assert.match(source, /data-dx-check-package-lane-hash-refresh-status/);
    assert.match(source, /data-dx-check-package-lane-hash-refresh-helper/);
    assert.match(source, /data-dx-check-package-lane-hash-refresh-json-command/);
    assert.match(source, /data-dx-check-package-lane-hash-refresh-zed/);
    assert.match(source, /data-dx-check-package-lane-hash-refresh-tracked-files/);
    assert.match(source, /data-dx-check-package-lane-hash-refresh-stale-files/);
    assert.match(source, /data-dx-check-package-lane-hash-refresh-missing-files/);
  }

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /DX Studio\/check-panel Backend Platform Client package row/);
    assert.match(source, /static \/launch Backend Platform Client package-lane row/);
    assert.match(source, /backend_platform_client_hash_manifest_present/);
    assert.match(source, /backend_platform_client_receipt_hash_refresh_current/);
    assert.match(source, /backend_platform_client_receipt_hash_refresh_stale/);
    assert.match(source, /backend_platform_client_receipt_hash_refresh_missing/);
    assert.match(source, /backend-platform-client:receipt-hash-refresh/);
    assert.match(source, /without claiming hosted Supabase runtime proof/);
  }
});

test("Backend Platform Client source-guard runbook fixture mirrors the Studio runbook", () => {
  const fixture = readJson(
    "docs/packages/backend-platform-client.source-guard-runbook.json",
  );
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const packageDoc = read("docs/packages/supabase-client.md");
  const frameworkStructure = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(
    fixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(fixture.route, "/");
  assert.equal(fixture.package.official_package_name, "Backend Platform Client");
  assert.equal(fixture.package.package_id, "supabase/client");
  assert.equal(
    fixture.package.upstream_package,
    "@supabase/ssr + @supabase/supabase-js",
  );
  assert.equal(
    fixture.package.source_mirror,
    "G:/WWW/inspirations/supabase",
  );
  assert.equal(
    fixture.package.based_on,
    "Supabase user-management profile workflow plus SSR client helpers",
  );

  assert.equal(
    fixture.guard.id,
    "backend-platform-client-dx-style-rust-check-output",
  );
  assert.equal(
    fixture.guard.guard_file,
    "core/src/ecosystem/project_check/backend_platform_client_dx_check.rs",
  );
  assert.equal(
    fixture.guard.command,
    "cargo test -q -p dx-www-compiler backend_platform_client_dx_style_missing_metric_and_finding_flip --lib",
  );
  assert.equal(
    fixture.guard.lightweight_guard_file,
    "benchmarks/supabase-dx-check-output.test.ts",
  );
  assert.equal(
    fixture.guard.lightweight_command,
    "dx run --test .\\benchmarks\\supabase-dx-check-output.test.ts",
  );
  assert.equal(fixture.guard.execution_policy, "source-only");
  assert.equal(fixture.guard.writes_files, false);
  assert.equal(fixture.guard.starts_server, false);
  assert.equal(fixture.guard.runs_package_install, false);
  assert.equal(fixture.guard.runs_full_build, false);
  assert.equal(fixture.guard.node_modules_required, false);

  for (const proof of [
    "Backend Platform Client Rust dx-style check output",
    "backend_platform_client_dx_style_compatibility_present",
    "backend_platform_client_dx_style_compatibility_missing",
    "backend-platform-client-missing-dx-style-compatibility",
    "docs/packages/backend-platform-client.source-guard-runbook.json",
    "without claiming hosted Supabase runtime proof",
    "supabase/client source-only Studio discovery",
  ]) {
    assert.ok(fixture.guard.proves.includes(proof), `missing proof ${proof}`);
    assert.ok(
      studioManifest.includes(proof) ||
        studioManifest.includes(proof.replaceAll('"', '\\"')),
      `Studio manifest missing ${proof}`,
    );
  }

  assert.equal(
    fixture.runbook.contract.evidence_field,
    "cargo test -q -p dx-www-compiler backend_platform_client_dx_style_missing_metric_and_finding_flip --lib",
  );
  assert.equal(
    fixture.runbook.command.purpose,
    "Validate the source-only Backend Platform Client dx-style metric and missing-metadata finding flip without hosted Supabase runtime proof.",
  );
  assert.equal(
    fixture.runbook.lightweight_command.purpose,
    "Validate the Backend Platform Client Rust dx-check output contract with the lightweight package-owned Node guard.",
  );
  assert.equal(
    fixture.helper_freshness_guard.id,
    "backend-platform-client-lower-dx-check-helper-freshness",
  );
  assert.equal(
    fixture.helper_freshness_guard.command,
    "cargo test -q -p dx-www-compiler backend_platform_client_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
  );
  assert.ok(
    fixture.helper_freshness_guard.proves.includes(
      "backend_platform_client_receipt_hash_refresh_stale",
    ),
    "fixture must prove stale helper freshness is surfaced",
  );
  assert.ok(
    fixture.helper_freshness_guard.proves.includes(
      "backend_platform_client_hash_mismatch stays byte-derived",
    ),
    "fixture must prove helper drift does not fake a source hash mismatch",
  );
  assert.ok(
    fixture.source_guard_fixture_paths.some(
      (entry) =>
        entry.source_guard_id ===
          "backend-platform-client-lower-dx-check-helper-freshness" &&
        entry.fixture_path ===
          "docs/packages/backend-platform-client.source-guard-runbook.json",
    ),
    "fixture must publish the helper-freshness guard as a targetable fixture path",
  );
  assert.equal(fixture.preview_manifest.generated_file, "public/preview-.dx/build-cache/manifest.json");
  assert.equal(fixture.preview_manifest.root_field, "sourceGuardRunbookFixtures");
  assert.equal(fixture.preview_manifest.route_field, "routes[].sourceGuardRunbookFixtures");
  assert.equal(fixture.preview_manifest.route, "/");
  assert.equal(
    fixture.preview_manifest.fixture_path,
    "docs/packages/backend-platform-client.source-guard-runbook.json",
  );
  assert.equal(
    fixture.preview_manifest.guard_id,
    "backend-platform-client-lower-dx-check-helper-freshness",
  );
  assert.equal(fixture.preview_manifest.honesty_label, "SOURCE-ONLY");
  assert.equal(fixture.preview_manifest.runtime_proof, false);
  assert.match(
    studioManifest,
    /studio_source_guard_with_fixture\(\s*"backend-platform-client-lower-dx-check-helper-freshness"/,
  );
  assert.match(
    studioManifest,
    /source_guard_contract_with_fixture\(\s*"backend-platform-client-lower-dx-check-helper-freshness"/,
  );
  assert.match(
    studioManifest,
    /source_guard_command_with_fixture\(\s*"cargo test -q -p dx-www-compiler backend_platform_client_hash_refresh_stale_helper_keeps_source_hash_clean --lib"/,
  );

  for (const marker of [
    "data-dx-style-surface",
    "data-dx-token-scope",
    "data-dx-package",
    "backend_platform_client_dx_style_compatibility_present",
    "backend_platform_client_dx_style_compatibility_missing",
    "backend-platform-client:dx-style-compatibility",
  ]) {
    assert.ok(
      fixture.zed_dx_studio_markers.includes(marker),
      `fixture must expose ${marker}`,
    );
  }

  assert.equal(fixture.receipt.hash_helper, "examples/template/backend-platform-client-receipt-hashes.ts");
  assert.equal(fixture.honesty_label, "SOURCE-ONLY");
  assert.equal(fixture.runtime_proof, false);
  assert.match(fixture.runtime_limitations.join("\n"), /hosted Supabase runtime proof/);

  for (const source of [
    studioManifest,
    packageDoc,
    frameworkStructure,
    dx,
    todo,
    changelog,
  ]) {
    assert.match(source, /backend-platform-client\.source-guard-runbook\.json/);
    assert.match(source, /backend-platform-client-dx-style-rust-check-output/);
    assert.match(source, /backend-platform-client-lower-dx-check-helper-freshness/);
    assert.match(source, /backend_platform_client_hash_refresh_stale_helper_keeps_source_hash_clean/);
    assert.match(source, /without claiming hosted Supabase runtime proof/);
  }

  for (const source of [packageDoc, frameworkStructure, dx, todo, changelog]) {
    assert.match(source, /sourceGuardRunbookFixtures/);
    assert.match(source, /public\/preview-manifest\.json/);
  }
});

test("Backend Platform Client package-lane row survives generated starter materialization", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-supabase-package-lane-"));
  const materializer = path.join(
    root,
    "tools",
    "launch",
    "materialize-www-template.ts",
  );

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
  assert.match(launch, /data-dx-check-package-lane-template="supabase\/client"/);
  assert.match(launch, /data-dx-check-package-lane-row="supabase\/client"/);
  assert.match(launch, /data-dx-check-package-lane-name="Backend Platform Client"/);
  assert.match(launch, /data-dx-check-package-lane-status="missing"/);
  assert.match(
    launch,
    /data-dx-check-package-lane-receipt-status="missing-receipt"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-receipt-path="examples\/template\/\.dx\/forge\/receipts\/2026-05-22-supabase-client-dashboard-workflow\.json"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-helper="examples\/template\/backend-platform-client-receipt-hashes\.ts"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-json-command="node tools\/launch\/run-template-receipt-helper\.js examples\/template\/backend-platform-client-receipt-hashes\.ts --check --json"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-zed="backend-platform-client:receipt-hash-refresh"/,
  );
  assert.match(launch, /data-dx-check-package-lane-hash-refresh-tracked-files="12"/);
  assert.match(launch, /data-dx-check-package-lane-hash-refresh-stale-files="0"/);
  assert.match(launch, /data-dx-check-package-lane-hash-refresh-missing-files="0"/);
  assert.match(launch, /data-dx-package="supabase\/client"/);

  const backendPlatformRunbookFixture = manifest.sourceGuardRunbookFixtures.find(
    (entry) => entry.packageId === "supabase/client",
  );
  assert.ok(
    backendPlatformRunbookFixture,
    "generated preview manifest must expose the Backend Platform Client source-guard runbook fixture",
  );
  assert.equal(
    backendPlatformRunbookFixture.officialPackageName,
    "Backend Platform Client",
  );
  assert.equal(
    backendPlatformRunbookFixture.fixture,
    "docs/packages/backend-platform-client.source-guard-runbook.json",
  );
  assert.equal(
    backendPlatformRunbookFixture.guardId,
    "backend-platform-client-lower-dx-check-helper-freshness",
  );
  assert.equal(
    backendPlatformRunbookFixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(backendPlatformRunbookFixture.honestyLabel, "SOURCE-ONLY");
  assert.equal(backendPlatformRunbookFixture.runtimeProof, false);
  assert.equal(
    backendPlatformRunbookFixture.zedVisibility,
    "backend-platform-client:receipt-hash-refresh",
  );

  const rootRoute = manifest.routes.find((route) => route.route === "/");
  assert.ok(rootRoute, "expected materialized / route metadata");
  assert.ok(
    rootRoute.forgePackages.includes("supabase/client"),
    "generated / route package scope must include Backend Platform Client",
  );

  const launchRoute = manifest.routes.find((route) => route.route === "/");
  assert.ok(launchRoute, "expected materialized / route metadata");
  assert.ok(
    launchRoute.forgePackages.includes("supabase/client"),
    "generated / route package scope must include Backend Platform Client",
  );
  assert.ok(
    launchRoute.sourceGuardRunbookFixtures.includes(
      "docs/packages/backend-platform-client.source-guard-runbook.json",
    ),
    "generated launch route must link the Backend Platform Client runbook fixture",
  );

  const checkPanel = manifest.editContract.editableSurfaces.find(
    (surface) => surface.id === "launch-runtime-dx-check-panel",
  );
  assert.ok(checkPanel, "expected dx-check panel edit surface");
  assert.equal(checkPanel.sourceFile, "pages/index.html");
  assert.ok(
    checkPanel.packageIds.includes("supabase/client"),
    "generated dx-check panel package scope must include Backend Platform Client",
  );
  assert.ok(
    checkPanel.stateMarkers.includes("data-dx-check-package-lane-template"),
  );
  assert.ok(
    checkPanel.stateMarkers.includes("data-dx-check-package-lane-row"),
  );
  assert.ok(
    checkPanel.stateMarkers.includes("data-dx-check-package-lane-hash-refresh-helper"),
  );
  assert.ok(
    checkPanel.stateMarkers.includes("data-dx-check-package-lane-hash-refresh-json-command"),
  );
  assert.ok(
    checkPanel.stateMarkers.includes("data-dx-check-package-lane-hash-refresh-zed"),
  );

  for (const source of [
    read("docs/packages/supabase-client.md"),
    read("DX.md"),
    read("TODO.md"),
    read("CHANGELOG.md"),
  ]) {
    assert.match(
      source,
      /Backend Platform Client generated-starter materialization guard|generated-starter materialization guard for Backend Platform Client/i,
    );
    assert.match(source, /sourceGuardRunbookFixtures/);
    assert.match(source, /public\/preview-manifest\.json/);
    assert.match(source, /without claiming hosted Supabase runtime proof/);
  }
});
