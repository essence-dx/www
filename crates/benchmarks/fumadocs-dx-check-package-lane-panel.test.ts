const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceRoot = "G:\\WWW\\inspirations\\fumadocs";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(sourceRoot, relativePath), "utf8");
}

function escaped(marker) {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

function sliceAfter(source, marker, length = 1600) {
  const index = source.indexOf(marker);
  assert.notEqual(index, -1, `missing source marker ${marker}`);
  return source.slice(index, index + length);
}

test("Documentation System package-lane row exposes hash-backed DX check-panel visibility", () => {
  const upstreamPackage = JSON.parse(readMirror("packages/core/package.json"));
  const loader = readMirror("packages/core/src/source/loader.ts");
  const llms = readMirror("packages/core/src/source/llms.ts");
  const searchServer = readMirror("packages/core/src/search/orama/create-server.ts");
  const openapi = readMirror("packages/openapi/src/server/index.tsx");
  const reader = read("core/src/ecosystem/dx_check_receipt.rs");
  const launchShell = read("examples/template/template-shell.tsx");
  const runtimeLaunchPage = read("tools/launch/runtime-template/pages/index.html");
  const materializer = read("tools/launch/materialize-www-template.ts");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const packageDoc = read("docs/packages/content-fumadocs-next.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(upstreamPackage.name, "fumadocs-core");
  assert.equal(upstreamPackage.version, "16.8.12");
  assert.match(loader, /export function loader/);
  assert.match(llms, /export function llms/);
  assert.match(searchServer, /export function createFromSource/);
  assert.match(openapi, /export function createOpenAPI/);

  for (const marker of [
    'DOCUMENTATION_SYSTEM_PACKAGE_ID: &str = "content/fumadocs-next"',
    'DOCUMENTATION_SYSTEM_OFFICIAL_NAME: &str = "Documentation System"',
    'DOCUMENTATION_SYSTEM_UPSTREAM_PACKAGE: &str = "fumadocs"',
    'DOCUMENTATION_SYSTEM_UPSTREAM_VERSION: &str = "16.8.12"',
    'DOCUMENTATION_SYSTEM_SOURCE_MIRROR: &str = "G:/WWW/inspirations/fumadocs"',
    'DOCUMENTATION_SYSTEM_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json"',
    'DOCUMENTATION_SYSTEM_PACKAGE_RECEIPT_PATH: &str =',
    "rows.extend(documentation_system_package_lane_row(root, package_status));",
    "fn documentation_system_package_lane_row(",
    "fn documentation_system_metric_rows(",
    "fn documentation_system_status_vocabulary(",
    "fn documentation_system_next_action(status: &str)",
    'DOCUMENTATION_SYSTEM_METRICS: [&str; 13]',
    "documentation_system_hash_manifest_present",
    "documentation_system_hash_mismatch",
    "documentation_system_receipt_hash_refresh_current",
    "documentation_system_receipt_hash_refresh_stale",
    "documentation_system_receipt_hash_refresh_missing",
    "documentation_system_dx_style_compatibility_present",
    "documentation_system_dx_style_compatibility_missing",
    "count_sha256_file_hash_mismatches(root, package)",
    "let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);",
    "|| refresh_stale > 0",
    "dx_check_latest_panel_exposes_documentation_system_package_lane_hash_row",
  ]) {
    assert.match(reader, escaped(marker), `missing check-panel marker ${marker}`);
  }

  for (const source of [launchShell, runtimeLaunchPage]) {
    assert.match(
      source,
      /data-dx-check-package-lane-template="content\/fumadocs-next"|package_id: "content\/fumadocs-next"/,
    );
    assert.match(
      source,
      /data-dx-check-package-lane-name="Documentation System"|official_package_name: "Documentation System"/,
    );
    assert.match(
      source,
      /data-dx-check-package-lane-hash-refresh-status="current"|hash_refresh_status: "current"/,
    );
    assert.match(
      source,
      /examples\/template\/documentation-system-receipt-hashes\.ts/,
    );
    assert.match(
      source,
      /documentation-system:receipt-hash-refresh/,
    );
    assert.match(
      source,
      /data-dx-style-surface="documentation-system"|dx_style_surface: "documentation-system"/,
    );
    assert.match(
      source,
      /data-dx-token-scope="content\/fumadocs-next"|token_scope: "content\/fumadocs-next"/,
    );
  }

  for (const [source, surfaceMarker, markerListMarker] of [
    [
      materializer,
      '"launch-runtime-dx-check-panel"',
      '"launch-runtime-dx-check-panel"',
    ],
    [
      editContract,
      'id: "dx-check-health-panel"',
      "const dxCheckPanelStateMarkers",
    ],
    [
      studioManifest,
      "fn studio_dx_check_edit_surface()",
      "fn studio_dx_check_edit_surface()",
    ],
  ]) {
    const checkPanelSlice = sliceAfter(source, surfaceMarker, 5200);
    assert.match(checkPanelSlice, /content\/fumadocs-next/);
    const checkPanelMarkers = sliceAfter(source, markerListMarker, 5200);
    assert.match(
      checkPanelMarkers,
      /data-dx-check-package-lane-hash-refresh-current-metric/,
    );
  }

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /DX Studio\/check-panel Documentation System package row/);
    assert.match(source, /documentation_system_hash_manifest_present/);
    assert.match(source, /documentation_system_hash_mismatch/);
    assert.match(source, /documentation_system_receipt_hash_refresh_current/);
    assert.match(source, /documentation_system_receipt_hash_refresh_stale/);
    assert.match(source, /documentation_system_receipt_hash_refresh_missing/);
    assert.match(source, /without claiming live Fumadocs renderer runtime proof/);
  }
});

test("Documentation System generated-starter guard is published in the Studio runbook", () => {
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const frameworkDoc = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const packageDoc = read("docs/packages/content-fumadocs-next.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  const guardSlice = sliceAfter(
    studioManifest,
    '"documentation-system-generated-starter-materialization"',
    5200,
  );
  assert.match(
    guardSlice,
    /dx run --test \.\\\\benchmarks\\\\fumadocs-dx-check-package-lane-panel\.test\.ts/,
  );
  assert.match(
    guardSlice,
    /Documentation System generated-starter materialization guard/,
  );
  assert.match(
    guardSlice,
    /data-dx-check-package-lane-row=\\"content\/fumadocs-next\\"/,
  );
  assert.match(guardSlice, /data-dx-token-scope=\\"content\/fumadocs-next\\"/);
  assert.match(guardSlice, /documentation-system:receipt-hash-refresh/);
  assert.match(
    guardSlice,
    /examples\/template\/documentation-system-receipt-hashes\.ts/,
  );
  assert.match(
    guardSlice,
    /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/documentation-system-receipt-hashes\.ts --check --json/,
  );
  assert.match(guardSlice, /without claiming live Fumadocs renderer runtime proof/);
  assert.match(guardSlice, /content\/fumadocs-next source-only Studio discovery/);

  const launchGuardIds = sliceAfter(
    studioManifest,
    '"/" => guards.extend',
    1400,
  );
  assert.match(
    launchGuardIds,
    /"documentation-system-generated-starter-materialization"/,
  );

  const launchCommands = sliceAfter(
    studioManifest,
    '"/" => commands.extend',
    18000,
  );
  assert.match(
    launchCommands,
    /dx run --test \.\\\\benchmarks\\\\fumadocs-dx-check-package-lane-panel\.test\.ts/,
  );
  assert.match(
    launchCommands,
    /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/documentation-system-receipt-hashes\.ts --check --json/,
  );

  const launchContracts = sliceAfter(
    studioManifest,
    '"/" => contracts.extend',
    30000,
  );
  assert.match(
    launchContracts,
    /documentation-system-generated-starter-materialization/,
  );
  assert.match(
    launchContracts,
    /The generated starter preserves the Documentation System package-lane row/,
  );

  for (const source of [frameworkDoc, packageDoc, dx, todo, changelog]) {
    assert.match(source, /Documentation System Studio source-guard\/runbook entry/);
    assert.match(source, /documentation-system-generated-starter-materialization/);
    assert.match(
      source,
      /dx run --test \.\\benchmarks\\fumadocs-dx-check-package-lane-panel\.test\.ts/,
    );
    assert.match(
      source,
      /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/documentation-system-receipt-hashes\.ts --check --json/,
    );
    assert.match(source, /without claiming live Fumadocs renderer runtime proof/);
  }
});

test("Documentation System source-guard runbook fixture is package-owned and machine readable", () => {
  const fixturePath = "docs/packages/content-fumadocs-next.source-guard-runbook.json";
  const fixture = JSON.parse(read(fixturePath));
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const frameworkDoc = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const packageDoc = read("docs/packages/content-fumadocs-next.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(fixture.schema, "dx.forge.package.source_guard_runbook_fixture");
  assert.equal(fixture.route, "/");
  assert.equal(fixture.package.official_package_name, "Documentation System");
  assert.equal(fixture.package.package_id, "content/fumadocs-next");
  assert.equal(fixture.package.upstream_package, "fumadocs");
  assert.equal(fixture.package.upstream_version, "fumadocs-core@16.8.12");
  assert.equal(fixture.package.source_mirror, "G:/WWW/inspirations/fumadocs");
  assert.equal(fixture.guard.id, "documentation-system-generated-starter-materialization");
  assert.equal(
    fixture.guard.command,
    "dx run --test .\\benchmarks\\fumadocs-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(fixture.guard.fixture_path, fixturePath);
  assert.equal(fixture.guard.execution_policy, "source-only");
  assert.equal(fixture.guard.starts_server, false);
  assert.equal(fixture.guard.runs_package_install, false);
  assert.equal(fixture.guard.runs_full_build, false);
  assert.equal(fixture.guard.node_modules_required, false);
  assert.equal(fixture.honesty_label, "SOURCE-ONLY");
  assert.equal(fixture.runtime_proof, false);

  for (const item of [
    "docs-help-changelog-workflow",
    "generated-starter-materialization",
    "preview-manifest-materializer",
    "receipt-hash-refresh",
    "dx-style-compatibility",
  ]) {
    assert.ok(
      fixture.selected_surfaces.includes(item),
      `fixture must list selected surface ${item}`,
    );
  }

  for (const api of [
    "loader",
    "llms",
    "createFromSource",
    "useDocsSearch",
    "createOpenAPI",
    "loaderPlugin",
    "proxyUrl",
  ]) {
    assert.ok(
      fixture.upstream_public_apis.includes(api),
      `fixture must cite upstream API ${api}`,
    );
  }

  for (const metric of [
    "documentation_system_receipt_hash_refresh_current",
    "documentation_system_receipt_hash_refresh_stale",
    "documentation_system_receipt_hash_refresh_missing",
    "documentation_system_dx_style_compatibility_present",
    "documentation_system_dx_style_compatibility_missing",
  ]) {
    assert.ok(fixture.dx_check_metrics.includes(metric), `fixture must list ${metric}`);
  }

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
      `fixture must list marker ${marker}`,
    );
  }

  assert.equal(
    fixture.runbook.helper_command.command,
    "node tools/launch/run-template-receipt-helper.js examples/template/documentation-system-receipt-hashes.ts --check --json",
  );
  assert.equal(fixture.runbook.helper_command.scope, "source-only");
  assert.equal(fixture.receipt.hash_helper, "examples/template/documentation-system-receipt-hashes.ts");
  assert.equal(fixture.receipt.source_guard_runbook_fixture, fixturePath);
  assert.equal(
    fixture.receipt.preview_manifest_materializer,
    "tools/launch/materialize-www-template.ts",
  );
  assert.equal(fixture.receipt.tracked_by_receipt_hash_helper, true);
  assert.ok(fixture.receipt.hash_tracked_files.includes(fixturePath));
  assert.ok(
    fixture.receipt.hash_tracked_files.includes(
      "tools/launch/materialize-www-template.ts",
    ),
  );
  assert.equal(fixture.receipt.zed_visibility, "documentation-system:receipt-hash-refresh");
  assert.equal(fixture.preview_manifest.generated_file, "public/preview-.dx/build-cache/manifest.json");
  assert.equal(
    fixture.preview_manifest.materializer,
    "tools/launch/materialize-www-template.ts",
  );
  assert.equal(fixture.preview_manifest.root_field, "sourceGuardRunbookFixtures");
  assert.equal(
    fixture.preview_manifest.route_field,
    "routes[].sourceGuardRunbookFixtures",
  );
  assert.equal(fixture.preview_manifest.fixture, fixturePath);
  assert.equal(fixture.preview_manifest.tracked_by_receipt_hash_helper, true);
  assert.equal(fixture.preview_manifest.runtime_proof, false);
  assert.match(
    fixture.runtime_limitations.join("\n"),
    /live Fumadocs renderer proof.*stay app-owned/,
  );

  const guardFixtureSlice = sliceAfter(
    studioManifest,
    '"source_guard_id": "documentation-system-generated-starter-materialization"',
    900,
  );
  assert.match(guardFixtureSlice, /package_id": "content\/fumadocs-next/);
  assert.match(guardFixtureSlice, escaped(fixturePath));
  assert.match(guardFixtureSlice, /source_guard_runbook_fixture/);

  for (const source of [frameworkDoc, packageDoc, dx, todo, changelog]) {
    assert.match(source, escaped(fixturePath));
    assert.match(source, /package-owned Documentation System runbook fixture/);
    assert.match(source, /without claiming live Fumadocs renderer runtime proof/);
  }
});

test("Documentation System package-lane fixture survives generated starter materialization", () => {
  const runbookFixturePath = "docs/packages/content-fumadocs-next.source-guard-runbook.json";
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-fumadocs-package-lane-"));
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
    const launch = fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8");
    const manifest = JSON.parse(
      fs.readFileSync(path.join(dir, "public", "preview-.dx/build-cache/manifest.json"), "utf8"),
    );

    assert.equal(result.ok, true);
    assert.equal(result.noNodeModules, true);
    assert.ok(!fs.existsSync(path.join(dir, "node_modules")));
    assert.ok(
      Array.isArray(manifest.sourceGuardRunbookFixtures),
      "generated preview manifest must expose source-guard runbook fixtures",
    );
    const documentationSystemFixture = manifest.sourceGuardRunbookFixtures.find(
      (fixture) => fixture.packageId === "content/fumadocs-next",
    );
    assert.ok(
      documentationSystemFixture,
      "generated preview manifest must expose the Documentation System runbook fixture",
    );
    assert.equal(documentationSystemFixture.officialPackageName, "Documentation System");
    assert.equal(documentationSystemFixture.upstreamPackage, "fumadocs");
    assert.equal(documentationSystemFixture.upstreamVersion, "16.8.12");
    assert.equal(documentationSystemFixture.sourceMirror, "G:/WWW/inspirations/fumadocs");
    assert.equal(documentationSystemFixture.fixture, runbookFixturePath);
    assert.equal(
      documentationSystemFixture.guardId,
      "documentation-system-generated-starter-materialization",
    );
    assert.equal(
      documentationSystemFixture.schema,
      "dx.forge.package.source_guard_runbook_fixture",
    );
    assert.equal(documentationSystemFixture.route, "/");
    assert.equal(documentationSystemFixture.honestyLabel, "SOURCE-ONLY");
    assert.equal(documentationSystemFixture.runtimeProof, false);
    assert.equal(
      documentationSystemFixture.zedVisibility,
      "documentation-system:receipt-hash-refresh",
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-template="content\/fumadocs-next"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-row="content\/fumadocs-next"/,
    );
    assert.match(launch, /data-dx-check-package-lane-name="Documentation System"/);
    assert.match(launch, /data-dx-check-package-lane-status="missing"/);
    assert.match(launch, /data-dx-check-package-lane-receipt-status="missing-receipt"/);
    assert.match(launch, /data-dx-check-package-lane-upstream-package="fumadocs"/);
    assert.match(
      launch,
      /data-dx-check-package-lane-source-mirror="G:\/WWW\/inspirations\/fumadocs"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-receipt-path="examples\/template\/\.dx\/forge\/receipts\/2026-05-22-content-fumadocs-dashboard-workflow\.json"/,
    );
    assert.match(launch, /data-dx-check-package-lane-dx-style-status="present"/);
    assert.match(launch, /data-dx-check-package-lane-hash-refresh-status="current"/);
    assert.match(
      launch,
      /data-dx-check-package-lane-hash-refresh-helper="examples\/template\/documentation-system-receipt-hashes\.ts"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-hash-refresh-json-command="node tools\/launch\/run-template-receipt-helper\.js examples\/template\/documentation-system-receipt-hashes\.ts --check --json"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-hash-refresh-zed="documentation-system:receipt-hash-refresh"/,
    );
    assert.match(launch, /data-dx-check-package-lane-hash-refresh-tracked-files="8"/);
    assert.match(launch, /data-dx-check-package-lane-hash-refresh-stale-files="0"/);
    assert.match(launch, /data-dx-check-package-lane-hash-refresh-missing-files="0"/);
    assert.match(
      launch,
      /data-dx-check-package-lane-hash-refresh-current-metric="documentation_system_receipt_hash_refresh_current"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-hash-refresh-stale-metric="documentation_system_receipt_hash_refresh_stale"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-hash-refresh-missing-metric="documentation_system_receipt_hash_refresh_missing"/,
    );
    assert.match(launch, /data-dx-style-surface="documentation-system"/);
    assert.match(launch, /data-dx-token-scope="content\/fumadocs-next"/);
    assert.match(launch, /data-dx-package="content\/fumadocs-next"/);

    const dashboardRoute = manifest.routes.find((route) => route.route === "/dashboard");
    assert.ok(dashboardRoute, "expected generated dashboard route metadata");
    assert.ok(
      dashboardRoute.forgePackages.includes("content/fumadocs-next"),
      "generated dashboard route package scope must include Documentation System",
    );

    const launchRoute = manifest.routes.find((route) => route.route === "/");
    assert.ok(launchRoute, "expected generated /launch route metadata");
    assert.ok(
      launchRoute.forgePackages.includes("content/fumadocs-next"),
      "generated /launch route package scope must include Documentation System",
    );
    assert.ok(
      launchRoute.sourceGuardRunbookFixtures.includes(runbookFixturePath),
      "generated /launch route must expose the Documentation System runbook fixture",
    );
    assert.ok(
      launchRoute.dataDxMarkers.includes(
        "data-dx-check-package-lane-hash-refresh-current-metric",
      ),
      "generated /launch manifest must expose Documentation System helper metrics",
    );

    const checkPanel = manifest.editContract.editableSurfaces.find(
      (surface) => surface.id === "launch-runtime-dx-check-panel",
    );
    assert.ok(checkPanel, "expected dx-check panel edit surface");
    assert.equal(checkPanel.sourceFile, "pages/index.html");
    assert.ok(
      checkPanel.packageIds.includes("content/fumadocs-next"),
      "generated dx-check panel package scope must include Documentation System",
    );

    for (const marker of [
      "data-dx-check-package-lane-template",
      "data-dx-check-package-lane-row",
      "data-dx-check-package-lane-hash-refresh-helper",
      "data-dx-check-package-lane-hash-refresh-json-command",
      "data-dx-check-package-lane-hash-refresh-zed",
      "data-dx-check-package-lane-hash-refresh-current-metric",
      "data-dx-check-package-lane-hash-refresh-stale-metric",
      "data-dx-check-package-lane-hash-refresh-missing-metric",
      "data-dx-style-surface",
      "data-dx-token-scope",
    ]) {
      assert.ok(
        checkPanel.stateMarkers.includes(marker),
        `generated dx-check panel must expose ${marker}`,
      );
    }

    for (const source of [
      read("docs/packages/content-fumadocs-next.md"),
      read("DX.md"),
      read("TODO.md"),
      read("CHANGELOG.md"),
    ]) {
      assert.match(
        source,
        /generated-starter materialization guard for Documentation System/,
      );
      assert.match(source, /without claiming live Fumadocs renderer runtime proof/);
    }
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});
