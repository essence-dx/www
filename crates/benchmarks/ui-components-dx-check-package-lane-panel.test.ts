const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const shadcnUpstream = path.resolve(root, "..", "..", "WWW/inspirations/shadcn-ui");
const radixUpstream = path.resolve(root, "..", "..", "WWW/inspirations/radix-primitives");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function readUpstream(base, relativePath) {
  return fs.readFileSync(path.join(base, relativePath), "utf8");
}

test("UI Components package-lane row exposes helper freshness in the DX check panel", () => {
  const shadcnPackage = JSON.parse(readUpstream(shadcnUpstream, "package.json"));
  const shadcnButton = readUpstream(
    shadcnUpstream,
    "apps/v4/registry/new-york-v4/ui/button.tsx",
  );
  const radixSlotPackage = JSON.parse(
    readUpstream(radixUpstream, "packages/react/slot/package.json"),
  );
  const radixSlot = readUpstream(
    radixUpstream,
    "packages/react/slot/src/slot.tsx",
  );
  const reader = read("core/src/ecosystem/dx_check_receipt.rs");
  const launchShell = read("examples/template/template-shell.tsx");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const materializer = read("tools/launch/materialize-www-template.ts");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const runtimeLaunchPage = read("tools/launch/runtime-template/pages/index.html");
  const packageDoc = read("docs/packages/ui-components.md");
  const frameworkStructure = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(shadcnPackage.name, "ui");
  assert.equal(shadcnPackage.version, "0.0.1");
  assert.match(shadcnButton, /function Button/);
  assert.match(shadcnButton, /buttonVariants/);
  assert.equal(radixSlotPackage.name, "@radix-ui/react-slot");
  assert.equal(radixSlotPackage.version, "1.2.4");
  assert.match(radixSlot, /export function createSlot/);
  assert.match(radixSlot, /Slot as Root/);

  for (const marker of [
    'UI_COMPONENTS_PACKAGE_ID: &str = "shadcn/ui/button"',
    'UI_COMPONENTS_OFFICIAL_NAME: &str = "UI Components"',
    'UI_COMPONENTS_UPSTREAM_PACKAGE: &str = "shadcn-ui"',
    'UI_COMPONENTS_UPSTREAM_VERSION: &str = "0.0.1"',
    'UI_COMPONENTS_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json"',
    "UI_COMPONENTS_PACKAGE_RECEIPT_PATH: &str",
    "rows.extend(ui_components_package_lane_row(root, package_status))",
    "fn ui_components_package_lane_row(",
    "fn ui_components_hash_refresh_row",
    "let receipt_hash_refresh = ui_components_hash_refresh_row(package)",
    "ui_components_hash_manifest_present",
    "ui_components_hash_mismatch",
    "ui_components_receipt_hash_refresh_current",
    "ui_components_receipt_hash_refresh_stale",
    "ui_components_receipt_hash_refresh_missing",
    "receipt_hash_refresh_counts(package)",
    "ui_components_next_action(status, receipt_hash_refresh.as_ref())",
    "dx_check_latest_panel_exposes_ui_components_package_lane_hash_refresh_row",
  ]) {
    assert.match(reader, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "type DxCheckPanelPackageLaneHashRefreshRow = {",
    "receipt_hash_refresh?: DxCheckPanelPackageLaneHashRefreshRow | null;",
    "data-dx-check-package-lane-hash-refresh-status",
    "data-dx-check-package-lane-hash-refresh-helper",
    "data-dx-check-package-lane-hash-refresh-json-command",
    "data-dx-check-package-lane-hash-refresh-zed",
    "data-dx-check-package-lane-hash-refresh-current-metric",
    "data-dx-check-package-lane-hash-refresh-stale-metric",
    "data-dx-check-package-lane-hash-refresh-missing-metric",
    "{packageLane.receipt_hash_refresh ?",
  ]) {
    assert.match(launchShell, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const source of [editContract, materializer, studioManifest]) {
    assert.match(source, /data-dx-check-package-lane-hash-refresh-status/);
    assert.match(source, /data-dx-check-package-lane-hash-refresh-helper/);
    assert.match(source, /data-dx-check-package-lane-hash-refresh-json-command/);
    assert.match(source, /data-dx-check-package-lane-hash-refresh-zed/);
    assert.match(source, /data-dx-check-package-lane-hash-refresh-current-metric/);
    assert.match(source, /data-dx-check-package-lane-hash-refresh-stale-metric/);
    assert.match(source, /data-dx-check-package-lane-hash-refresh-missing-metric/);
  }

  assert.match(
    editContract,
    /id: "dx-check-health-panel"[\s\S]*packageIds: \[[\s\S]*"shadcn\/ui\/button"/,
    "source DX Studio dx-check panel package filter must include UI Components",
  );
  assert.match(
    materializer,
    /"launch-runtime-dx-check-panel"[\s\S]*\[[\s\S]*"shadcn\/ui\/button"/,
    "generated DX Studio dx-check panel package filter must include UI Components",
  );
  assert.match(
    studioManifest,
    /fn studio_dx_check_edit_surface\(\)[\s\S]*&\[[\s\S]*"shadcn\/ui\/button"/,
    "Rust Studio manifest dx-check panel package filter must include UI Components",
  );
  assert.match(
    studioManifest,
    /studio_source_guard(?:_with_fixture)?\(\s*"ui-components-generated-starter-materialization"/,
    "Rust Studio manifest must publish the UI Components generated-starter source guard",
  );
  assert.match(
    studioManifest,
    /"ui-components-generated-starter-materialization"[\s\S]*"benchmarks\/ui-components-dx-check-package-lane-panel\.test\.ts"/,
    "UI Components source guard must point at the focused lane benchmark",
  );
  assert.match(
    studioManifest,
    /"ui-components-generated-starter-materialization"[\s\S]*"dx run --test \.\\\\benchmarks\\\\ui-components-dx-check-package-lane-panel\.test\.ts"/,
    "UI Components source guard must expose the exact lightweight command",
  );
  assert.match(
    studioManifest,
    /"ui-components-generated-starter-materialization"[\s\S]*"UI Components generated-starter materialization guard"/,
  );
  assert.match(
    studioManifest,
    /"ui-components-generated-starter-materialization"[\s\S]*"ui-components:receipt-hash-refresh"/,
  );
  assert.match(
    studioManifest,
    /"ui-components-generated-starter-materialization"[\s\S]*"data-dx-token-scope=\\\"shadcn\/ui\/button\\\""/,
  );
  assert.match(
    studioManifest,
    /"\/" => guards\.extend\(\[[\s\S]*"ui-components-generated-starter-materialization"/,
    "the /launch source guard runbook must list the UI Components generated-starter guard",
  );
  assert.match(
    studioManifest,
    /"ui-components-generated-starter-materialization"[\s\S]*"The generated starter preserves the UI Components package-lane row, helper freshness markers, and package-scoped dx-check panel without claiming browser UI runtime proof\."/,
  );
  assert.match(
    studioManifest,
    /"dx run --test \.\\\\benchmarks\\\\ui-components-dx-check-package-lane-panel\.test\.ts"[\s\S]*"Validate the source-only UI Components package-lane row, generated starter materialization, helper freshness markers, and dx-check panel package scope\."/,
  );

  assert.match(
    launchShell,
    /package_id: "shadcn\/ui\/button"[\s\S]*official_package_name: "UI Components"[\s\S]*hash_refresh_status: "current"[\s\S]*examples\/template\/ui-components-receipt-hashes\.ts[\s\S]*node tools\/launch\/run-template-receipt-helper\.js examples\/template\/ui-components-receipt-hashes\.ts --check --json[\s\S]*ui-components:receipt-hash-refresh[\s\S]*hash_refresh_tracked_files: 6[\s\S]*hash_refresh_metric_current: "ui_components_receipt_hash_refresh_current"[\s\S]*hash_refresh_metric_stale: "ui_components_receipt_hash_refresh_stale"[\s\S]*hash_refresh_metric_missing: "ui_components_receipt_hash_refresh_missing"/,
  );
  assert.match(
    runtimeLaunchPage,
    /data-dx-check-package-lane-template="shadcn\/ui\/button"[\s\S]*data-dx-check-package-lane-name="UI Components"[\s\S]*data-dx-check-package-lane-hash-refresh-status="current"[\s\S]*examples\/template\/ui-components-receipt-hashes\.ts[\s\S]*node tools\/launch\/run-template-receipt-helper\.js examples\/template\/ui-components-receipt-hashes\.ts --check --json[\s\S]*ui-components:receipt-hash-refresh[\s\S]*data-dx-check-package-lane-hash-refresh-tracked-files="6"[\s\S]*data-dx-check-package-lane-hash-refresh-current-metric="ui_components_receipt_hash_refresh_current"[\s\S]*data-dx-check-package-lane-hash-refresh-stale-metric="ui_components_receipt_hash_refresh_stale"[\s\S]*data-dx-check-package-lane-hash-refresh-missing-metric="ui_components_receipt_hash_refresh_missing"/,
  );

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /DX Studio\/check-panel UI Components package row|static \/launch UI Components package-lane fixture/);
    assert.match(source, /receipt_hash_refresh/);
    assert.match(source, /ui-components:receipt-hash-refresh/);
    assert.match(source, /ui_components_receipt_hash_refresh_current/);
    assert.match(source, /ui_components_receipt_hash_refresh_stale/);
    assert.match(source, /ui_components_receipt_hash_refresh_missing/);
    assert.match(source, /without claiming browser UI runtime proof/);
  }

  assert.match(
    frameworkStructure,
    /Lane 20 uses the official front-facing package name `UI Components`/,
  );
  assert.match(frameworkStructure, /ui-components-generated-starter-materialization/);
  assert.match(
    frameworkStructure,
    /dx run --test \.\\benchmarks\\ui-components-dx-check-package-lane-panel\.test\.ts/,
  );
  assert.match(frameworkStructure, /source_guard_runbook_index/);
  assert.match(frameworkStructure, /without claiming browser UI runtime proof/);
});

test("UI Components source-guard runbook fixture mirrors the Studio manifest", () => {
  const fixture = readJson("docs/packages/ui-components.source-guard-runbook.json");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const packageDoc = read("docs/packages/ui-components.md");
  const frameworkStructure = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(
    fixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(fixture.route, "/");
  assert.equal(fixture.package.official_package_name, "UI Components");
  assert.equal(fixture.package.package_id, "shadcn/ui/button");
  assert.equal(fixture.package.upstream_package, "shadcn-ui");
  assert.equal(fixture.package.upstream_version, "0.0.1");
  assert.deepEqual(fixture.package.source_mirrors, [
    "G:/WWW/inspirations/shadcn-ui",
    "G:/WWW/inspirations/radix-primitives",
  ]);
  assert.equal(
    fixture.package.based_on,
    "shadcn-ui v4 registry plus Radix Primitives",
  );

  assert.equal(
    fixture.guard.id,
    "ui-components-generated-starter-materialization",
  );
  assert.equal(
    fixture.guard.guard_file,
    "benchmarks/ui-components-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(
    fixture.guard.command,
    "dx run --test .\\benchmarks\\ui-components-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(fixture.guard.execution_policy, "source-only");
  assert.equal(fixture.guard.writes_files, false);
  assert.equal(fixture.guard.starts_server, false);
  assert.equal(fixture.guard.runs_package_install, false);
  assert.equal(fixture.guard.runs_full_build, false);
  assert.equal(fixture.guard.node_modules_required, false);

  for (const proof of [
    "UI Components generated-starter materialization guard",
    'data-dx-check-package-lane-row="shadcn/ui/button"',
    'data-dx-token-scope="shadcn/ui/button"',
    "ui-components:receipt-hash-refresh",
    "without claiming browser UI runtime proof",
    "shadcn/ui/button source-only Studio discovery",
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
    "benchmarks/ui-components-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(
    fixture.runbook.command.purpose,
    "Validate the source-only UI Components package-lane row, generated starter materialization, helper freshness markers, and dx-check panel package scope.",
  );
  assert.ok(
    fixture.preview_manifest,
    "UI Components runbook fixture must describe its preview-manifest exposure",
  );
  assert.equal(fixture.preview_manifest.generated_file, "public/preview-manifest.json");
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
    "docs/packages/ui-components.source-guard-runbook.json",
  );
  assert.equal(fixture.preview_manifest.runtime_proof, false);
  assert.equal(
    fixture.preview_manifest.zed_visibility,
    "ui-components:receipt-hash-refresh",
  );
  assert.match(
    studioManifest,
    /fn source_guard_fixture_paths_for_route\(route: &str\)[\s\S]*"source_guard_id": "ui-components-generated-starter-materialization"[\s\S]*"fixture_path": "docs\/packages\/ui-components\.source-guard-runbook\.json"/,
    "Studio source_guard_runbook_index fixture_paths must link the UI Components runbook fixture",
  );

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
      `fixture must expose ${marker}`,
    );
  }

  assert.equal(fixture.honesty_label, "SOURCE-ONLY");
  assert.equal(fixture.runtime_proof, false);
  assert.match(fixture.runtime_limitations.join("\n"), /browser UI runtime proof/);

  for (const source of [
    studioManifest,
    packageDoc,
    frameworkStructure,
    dx,
    todo,
    changelog,
  ]) {
    assert.match(source, /ui-components\.source-guard-runbook\.json/);
    assert.match(source, /ui-components-generated-starter-materialization/);
    assert.match(source, /without claiming browser UI runtime proof/);
  }
});

test("UI Components package-lane row survives generated starter materialization", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-ui-components-package-lane-"));
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
      fs.readFileSync(path.join(dir, "public", "preview-manifest.json"), "utf8"),
    );

    assert.equal(result.ok, true);
    assert.equal(result.noNodeModules, true);
    assert.ok(!fs.existsSync(path.join(dir, "node_modules")));
    assert.match(launch, /data-dx-check-package-lane-template="shadcn\/ui\/button"/);
    assert.match(launch, /data-dx-check-package-lane-row="shadcn\/ui\/button"/);
    assert.match(launch, /data-dx-check-package-lane-name="UI Components"/);
    assert.match(
      launch,
      /data-dx-check-package-lane-upstream-package="shadcn-ui"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-source-mirror="G:\/WWW\/inspirations\/shadcn-ui; G:\/WWW\/inspirations\/radix-primitives"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-receipt-path="examples\/template\/\.dx\/forge\/receipts\/2026-05-22-shadcn-dashboard-controls\.json"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-hash-refresh-helper="examples\/template\/ui-components-receipt-hashes\.ts"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-hash-refresh-json-command="node tools\/launch\/run-template-receipt-helper\.js examples\/template\/ui-components-receipt-hashes\.ts --check --json"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-hash-refresh-zed="ui-components:receipt-hash-refresh"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-template="shadcn\/ui\/button"[\s\S]*data-dx-check-package-lane-hash-refresh-tracked-files="6"/,
    );
    assert.match(launch, /data-dx-check-package-lane-hash-refresh-current-metric="ui_components_receipt_hash_refresh_current"/);
    assert.match(launch, /data-dx-check-package-lane-hash-refresh-stale-metric="ui_components_receipt_hash_refresh_stale"/);
    assert.match(launch, /data-dx-check-package-lane-hash-refresh-missing-metric="ui_components_receipt_hash_refresh_missing"/);
    assert.match(launch, /data-dx-style-surface="ui-components"/);
    assert.match(launch, /data-dx-token-scope="shadcn\/ui\/button"/);
    assert.match(launch, /data-dx-package="shadcn\/ui\/button"/);

    const rootRoute = manifest.routes.find((route) => route.route === "/");
    assert.ok(rootRoute, "expected generated root route metadata");
    assert.ok(
      rootRoute.forgePackages.includes("3d/launch-scene"),
      "generated root route should stay scoped to the 3D landing scene package",
    );
    assert.ok(
      rootRoute.forgePackages.includes("shadcn/ui/button"),
      "generated root route package scope must include UI Components",
    );

    const launchRoute = manifest.routes.find((route) => route.route === "/");
    assert.ok(launchRoute, "expected generated / route metadata");
    assert.ok(
      launchRoute.forgePackages.includes("shadcn/ui/button"),
      "generated / route package scope must include UI Components",
    );
    assert.ok(
      launchRoute.dataDxMarkers.includes("data-dx-check-package-lane-template"),
      "generated /launch manifest must expose the package-lane template marker",
    );
    assert.ok(
      launchRoute.dataDxMarkers.includes("data-dx-check-package-lane-hash-refresh-current-metric"),
      "generated /launch manifest must expose UI Components hash-refresh metrics",
    );
    assert.ok(
      Array.isArray(manifest.sourceGuardRunbookFixtures),
      "generated preview manifest must expose source-guard runbook fixtures",
    );
    const uiRunbookFixture = manifest.sourceGuardRunbookFixtures.find(
      (fixture) => fixture.packageId === "shadcn/ui/button",
    );
    assert.ok(
      uiRunbookFixture,
      "generated preview manifest must expose the UI Components source-guard runbook fixture",
    );
    assert.equal(uiRunbookFixture.officialPackageName, "UI Components");
    assert.equal(uiRunbookFixture.upstreamPackage, "shadcn-ui");
    assert.equal(uiRunbookFixture.upstreamVersion, "0.0.1");
    assert.equal(
      uiRunbookFixture.fixture,
      "docs/packages/ui-components.source-guard-runbook.json",
    );
    assert.equal(
      uiRunbookFixture.guardId,
      "ui-components-generated-starter-materialization",
    );
    assert.equal(uiRunbookFixture.route, "/");
    assert.equal(uiRunbookFixture.honestyLabel, "SOURCE-ONLY");
    assert.equal(uiRunbookFixture.runtimeProof, false);
    assert.equal(uiRunbookFixture.zedVisibility, "ui-components:receipt-hash-refresh");
    assert.ok(
      launchRoute.sourceGuardRunbookFixtures.includes(
        "docs/packages/ui-components.source-guard-runbook.json",
      ),
      "generated /launch route must link the UI Components runbook fixture path",
    );

    const checkPanel = manifest.editContract.editableSurfaces.find(
      (surface) => surface.id === "launch-runtime-dx-check-panel",
    );
    assert.ok(checkPanel, "expected dx-check panel edit surface");
    assert.equal(checkPanel.sourceFile, "pages/index.html");
    assert.ok(
      checkPanel.packageIds.includes("shadcn/ui/button"),
      "generated dx-check panel package scope must include UI Components",
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
      read("docs/packages/ui-components.md"),
      read("DX.md"),
      read("TODO.md"),
      read("CHANGELOG.md"),
    ]) {
      assert.match(source, /UI Components generated-starter materialization guard/);
      assert.match(source, /UI Components Studio source-guard\/runbook entry/);
      assert.match(source, /without claiming browser UI runtime proof/);
    }
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});
