const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

test("Reactive Store dx-check package-lane row is Studio and runtime discoverable", () => {
  const runbookFixturePath =
    "docs/packages/reactive-store.source-guard-runbook.json";
  assert.ok(
    fs.existsSync(path.join(root, runbookFixturePath)),
    "Reactive Store source-guard runbook fixture should exist",
  );

  const launchShell = read("examples/template/template-shell.tsx");
  const runtimeLaunch = read("tools/launch/runtime-template/pages/index.html");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const materializer = read("tools/launch/materialize-www-template.ts");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const dxCheckReceipt = read("core/src/ecosystem/dx_check_receipt.rs");
  const packageDocs = read("docs/packages/reactive-store.md");
  const frameworkDocs = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const runbookFixture = readJson(runbookFixturePath);

  assert.match(launchShell, /type DxCheckPanelPackageLaneRow = \{/);
  assert.match(launchShell, /package_lane_rows: DxCheckPanelPackageLaneRow\[\]/);
  assert.match(
    launchShell,
    /data-dx-check-package-lane-count=\{viewModel\.package_lane_rows\.length\}/,
  );
  assert.match(launchShell, /viewModel\.package_lane_rows\.map\(\(packageLane\) =>/);
  assert.match(
    launchShell,
    /data-dx-check-package-lane-row=\{packageLane\.package_id\}/,
  );
  assert.match(
    launchShell,
    /data-dx-check-package-lane-name=\{packageLane\.official_package_name\}/,
  );
  assert.match(
    launchShell,
    /data-dx-check-package-lane-upstream-package=\{packageLane\.upstream_package\}/,
  );
  assert.match(
    launchShell,
    /data-dx-check-package-lane-source-mirror=\{packageLane\.source_mirror\}/,
  );

  assert.match(runtimeLaunch, /data-dx-check-package-lane-count="0"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-template="reactive\/store"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-row="reactive\/store"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-name="Reactive Store"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-status="missing"/);
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-receipt-status="missing-receipt"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-upstream-package="@tanstack\/store"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-source-mirror="G:\/WWW\/inspirations\/tanstack-store"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-receipt-path="\.dx\/forge\/receipts\/packages\/reactive-store\.json"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-template="reactive\/store"[\s\S]*data-dx-check-package-lane-hash-refresh-status="current"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-template="reactive\/store"[\s\S]*data-dx-check-package-lane-hash-refresh-helper="examples\/template\/reactive-store-receipt-hashes\.ts"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-template="reactive\/store"[\s\S]*data-dx-check-package-lane-hash-refresh-json-command="node tools\/launch\/run-template-receipt-helper\.js examples\/template\/reactive-store-receipt-hashes\.ts --check --json"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-template="reactive\/store"[\s\S]*data-dx-check-package-lane-hash-refresh-zed="reactive-store:receipt-hash-refresh"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-template="reactive\/store"[\s\S]*data-dx-check-package-lane-hash-refresh-stale-file-list=""/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-template="reactive\/store"[\s\S]*data-dx-check-package-lane-hash-refresh-current-metric="reactive_store_receipt_hash_refresh_current"/,
  );
  assert.match(
    launchShell,
    /package_id: "reactive\/store"[\s\S]*official_package_name: "Reactive Store"[\s\S]*hash_refresh_helper:\s*"examples\/template\/reactive-store-receipt-hashes\.ts"/,
  );
  assert.match(
    launchShell,
    /package_id: "reactive\/store"[\s\S]*hash_refresh_stale_file_list: ""/,
  );

  for (const source of [editContract, materializer]) {
    assert.match(source, /"data-dx-check-package-lane-count"/);
    assert.match(source, /"data-dx-check-package-lane-row"/);
    assert.match(source, /"data-dx-check-package-lane-status"/);
    assert.match(source, /"data-dx-check-package-lane-receipt-status"/);
  }

  assert.match(
    studioManifest,
    /"check_package_lane_count_marker": "data-dx-check-package-lane-count"/,
  );
  assert.match(
    studioManifest,
    /"forge_packages": \[[\s\S]*"reactive\/store"/,
  );
  assert.match(studioManifest, /studio_marker\(\s*"data-dx-check-package-lane-row"/);
  assert.match(studioManifest, /check_panel\.view_model\.package_lane_rows/);
  assert.match(
    studioManifest,
    /studio_edit_surface\(\s*"dx-check-health-panel"[\s\S]*&\[[\s\S]*"reactive\/store"/,
  );
  assert.match(
    studioManifest,
    /"package": "reactive\/store"[\s\S]*"front_facing_name": "Reactive Store"[\s\S]*"data-dx-check-package-lane-row"/,
  );
  assert.match(
    editContract,
    /id: "dx-check-health-panel"[\s\S]*packageIds: \[[\s\S]*"reactive\/store"/,
  );
  assert.match(
    materializer,
    /"launch-runtime-dx-check-panel"[\s\S]*\[[\s\S]*"reactive\/store"/,
  );
  assert.match(
    studioManifest,
    /studio_source_guard_with_fixture\(\s*"reactive-store-lower-dx-check-helper-freshness"/,
  );
  assert.match(
    studioManifest,
    /"reactive-store-lower-dx-check-helper-freshness"/,
  );
  assert.match(
    studioManifest,
    /cargo test -q -p dx-www-compiler reactive_store_hash_refresh_stale_helper_keeps_source_hash_clean --lib/,
  );
  assert.match(
    studioManifest,
    /source_guard_contract_with_fixture\(\s*"reactive-store-lower-dx-check-helper-freshness"[\s\S]*reactive_store_receipt_hash_refresh_stale/,
  );
  assert.match(
    studioManifest,
    /source_guard_command\(\s*"cargo test -q -p dx-www-compiler reactive_store_hash_refresh_stale_helper_keeps_source_hash_clean --lib"[\s\S]*Reactive Store/,
  );
  assert.match(studioManifest, /docs\/packages\/reactive-store\.source-guard-runbook\.json/);
  assert.match(
    studioManifest,
    /fn source_guard_fixture_paths_for_route\(route: &str\)[\s\S]*"source_guard_id": "reactive-store-lower-dx-check-helper-freshness"[\s\S]*"package_id": "reactive\/store"[\s\S]*"fixture_path": "docs\/packages\/reactive-store\.source-guard-runbook\.json"/,
    "Reactive Store source_guard_runbook_index fixture_paths must link the package-owned fixture",
  );
  assert.match(
    studioManifest,
    /source_guard_contract_with_fixture\(\s*"reactive-store-lower-dx-check-helper-freshness"[\s\S]*"docs\/packages\/reactive-store\.source-guard-runbook\.json"/,
    "Reactive Store runbook contract must carry structured fixture metadata",
  );
  assert.match(
    packageDocs,
    /reactive-store-lower-dx-check-helper-freshness/,
  );
  assert.match(
    packageDocs,
    /docs\/packages\/reactive-store\.source-guard-runbook\.json/,
  );
  assert.match(
    frameworkDocs,
    /Lane 4 uses the official front-facing package name `Reactive Store`[\s\S]*docs\/packages\/reactive-store\.source-guard-runbook\.json/,
  );

  assert.equal(
    runbookFixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(runbookFixture.route, "/");
  assert.equal(
    runbookFixture.package.official_package_name,
    "Reactive Store",
  );
  assert.equal(runbookFixture.package.package_id, "reactive/store");
  assert.equal(runbookFixture.package.upstream_package, "@tanstack/store");
  assert.equal(runbookFixture.package.upstream_version, "0.11.0");
  assert.deepEqual(runbookFixture.package.source_mirrors, [
    "G:/WWW/inspirations/tanstack-store",
  ]);
  assert.equal(
    runbookFixture.guard.id,
    "reactive-store-lower-dx-check-helper-freshness",
  );
  assert.deepEqual(runbookFixture.guard.routes, ["/"]);
  assert.equal(
    runbookFixture.guard.command,
    "cargo test -q -p dx-www-compiler reactive_store_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
  );
  assert.equal(runbookFixture.guard.execution_policy, "source-only");
  assert.equal(runbookFixture.guard.starts_server, false);
  assert.equal(runbookFixture.guard.runs_package_install, false);
  assert.equal(runbookFixture.guard.runs_full_build, false);
  assert.equal(runbookFixture.guard.node_modules_required, false);
  assert.ok(
    runbookFixture.guard.proves.includes(
      "reactive_store_receipt_hash_refresh_stale",
    ),
  );
  assert.ok(
    runbookFixture.guard.proves.includes(
      "docs/packages/reactive-store.source-guard-runbook.json",
    ),
  );
  assert.equal(
    runbookFixture.runbook.command.command,
    runbookFixture.guard.command,
  );
  assert.equal(runbookFixture.runbook.command.starts_server, false);
  assert.equal(runbookFixture.runbook.command.runs_package_install, false);
  assert.equal(runbookFixture.runbook.command.runs_full_build, false);
  assert.ok(
    runbookFixture.upstream_public_apis.includes("createStoreContext"),
  );
  assert.equal(runbookFixture.receipt.zed_visibility, "reactive-store:receipt-hash-refresh");
  assert.equal(runbookFixture.honesty_label, "SOURCE-OWNED TEMPLATE STORE");
  assert.equal(runbookFixture.runtime_proof, false);
  assert.ok(
    runbookFixture.runtime_limitations.some((limitation) =>
      limitation.includes("SOURCE-OWNED TEMPLATE STORE"),
    ),
  );
  assert.match(packageDocs, /Static launch preview/);
  assert.match(packageDocs, /data-dx-check-package-lane-row="reactive\/store"/);
  assert.match(packageDocs, /Studio package-scoped discovery/);
  assert.match(
    dxCheckReceipt,
    /const REACTIVE_STORE_PACKAGE_STATUS_PATH: &str = "\.dx\/forge\/package-status\.json";/,
  );
  assert.match(dxCheckReceipt, /"reactive_store_receipt_hash_refresh_current"/);
  assert.match(dxCheckReceipt, /"reactive_store_receipt_hash_refresh_stale"/);
  assert.match(dxCheckReceipt, /"reactive_store_receipt_hash_refresh_missing"/);
  assert.match(
    dxCheckReceipt,
    /package_lane_visibility_entry\(status, REACTIVE_STORE_PACKAGE_ID\)/,
  );
  assert.match(
    dxCheckReceipt,
    /let receipt_hash_refresh = package_lane_hash_refresh\(hash_refresh_source\);/,
  );
  assert.match(
    dxCheckReceipt,
    /let \(refresh_current, refresh_stale, refresh_missing\) =\s*receipt_hash_refresh_counts\(hash_refresh_source\);/,
  );
  assert.match(dxCheckReceipt, /receipt_hash_refresh,/);
  assert.match(
    dxCheckReceipt,
    /fn dx_check_latest_panel_exposes_reactive_store_package_lane_hash_refresh_row\(\)/,
  );
  assert.match(dxCheckReceipt, /stale_helper_reactive_store/);
  assert.match(dxCheckReceipt, /reactive_store_receipt_hash_refresh_stale/);
  assert.match(
    dxCheckReceipt,
    /helper_stale_metric_value\("reactive_store_hash_mismatch"\),\s*0/,
  );
  assert.match(packageDocs, /receipt_hash_refresh helper freshness/);
  assert.match(
    packageDocs,
    /cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_reactive_store_package_lane_hash_refresh_row --lib/,
  );
});

test("Reactive Store source-guard fixture is exposed in generated preview manifest", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-reactive-store-preview-"));
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

    assert.equal(result.ok, true);
    assert.equal(result.noNodeModules, true);
    assert.ok(!fs.existsSync(path.join(dir, "node_modules")));

    assert.ok(
      Array.isArray(manifest.sourceGuardRunbookFixtures),
      "generated preview manifest must expose source-guard runbook fixtures",
    );
    const reactiveStoreRunbookFixture = manifest.sourceGuardRunbookFixtures.find(
      (fixture) => fixture.packageId === "reactive/store",
    );
    assert.ok(
      reactiveStoreRunbookFixture,
      "generated preview manifest must expose the Reactive Store runbook fixture object",
    );
    assert.equal(reactiveStoreRunbookFixture.officialPackageName, "Reactive Store");
    assert.equal(reactiveStoreRunbookFixture.upstreamPackage, "@tanstack/store");
    assert.equal(reactiveStoreRunbookFixture.upstreamVersion, "0.11.0");
    assert.equal(
      reactiveStoreRunbookFixture.sourceMirror,
      "G:/WWW/inspirations/tanstack-store",
    );
    assert.equal(reactiveStoreRunbookFixture.route, "/");
    assert.equal(
      reactiveStoreRunbookFixture.fixture,
      "docs/packages/reactive-store.source-guard-runbook.json",
    );
    assert.equal(
      reactiveStoreRunbookFixture.guardId,
      "reactive-store-lower-dx-check-helper-freshness",
    );
    assert.equal(reactiveStoreRunbookFixture.honestyLabel, "SOURCE-ONLY");
    assert.equal(reactiveStoreRunbookFixture.runtimeProof, false);
    assert.equal(
      reactiveStoreRunbookFixture.zedVisibility,
      "reactive-store:receipt-hash-refresh",
    );

    const launchRoute = manifest.routes.find((route) => route.route === "/");
    assert.ok(launchRoute, "expected generated /launch route metadata");
    assert.ok(
      launchRoute.forgePackages.includes("reactive/store"),
      "generated /launch route package scope must include Reactive Store",
    );
    assert.ok(
      launchRoute.sourceGuardRunbookFixtures.includes(
        "docs/packages/reactive-store.source-guard-runbook.json",
      ),
      "generated /launch route must link the Reactive Store runbook fixture path",
    );

    const checkPanel = manifest.editContract.editableSurfaces.find(
      (surface) => surface.id === "launch-runtime-dx-check-panel",
    );
    assert.ok(checkPanel, "expected dx-check panel edit surface");
    assert.ok(
      checkPanel.packageIds.includes("reactive/store"),
      "generated dx-check panel package scope must include Reactive Store",
    );

    const packageDocs = read("docs/packages/reactive-store.md");
    assert.match(packageDocs, /Generated `public\/preview-manifest\.json`/);
    assert.match(packageDocs, /sourceGuardRunbookFixtures/);
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});
