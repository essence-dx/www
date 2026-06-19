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

test("State Management dx-check package-lane rows are Studio and runtime discoverable", () => {
  const launchShell = read("examples/template/template-shell.tsx");
  const runtimeLaunch = read("tools/launch/runtime-template/pages/index.html");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const materializer = read("tools/launch/materialize-www-template.ts");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const dxCheckReceipt = read("core/src/ecosystem/dx_check_receipt.rs");
  const packageDocs = read("docs/packages/state-zustand.md");
  const frameworkStructure = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");
  const runbookFixture = readJson(
    "docs/packages/state-zustand.source-guard-runbook.json",
  );

  assert.match(launchShell, /type DxCheckPanelPackageLaneHashRefreshRow = \{/);
  assert.match(launchShell, /current_files: string\[\];/);
  assert.match(launchShell, /stale_files: string\[\];/);
  assert.match(launchShell, /missing_files: string\[\];/);
  assert.match(launchShell, /stale_mirror_files: string\[\];/);
  assert.match(launchShell, /missing_mirror_files: string\[\];/);
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
  assert.match(launchShell, /function dxStyleCompatibilityStatus\(/);
  assert.match(
    launchShell,
    /data-dx-check-package-lane-dx-style-status=\{dxStyleCompatibilityStatus\(packageLane\)\}/,
  );
  assert.match(
    launchShell,
    /package_id: "state\/zustand"[\s\S]*hash_refresh_helper:\s*"examples\/template\/state-management-receipt-hashes\.ts"/,
  );
  assert.match(
    launchShell,
    /hash_refresh_json_command:\s*"node tools\/launch\/run-template-receipt-helper\.js examples\/template\/state-management-receipt-hashes\.ts --check --json"/,
  );
  assert.match(
    launchShell,
    /hash_refresh_zed:\s*"state-management:receipt-hash-refresh"/,
  );
  assert.match(
    launchShell,
    /hash_refresh_metric_current:\s*"state_management_receipt_hash_refresh_current"/,
  );
  assert.match(
    launchShell,
    /hash_refresh_metric_stale:\s*"state_management_receipt_hash_refresh_stale"/,
  );
  assert.match(
    launchShell,
    /hash_refresh_metric_missing:\s*"state_management_receipt_hash_refresh_missing"/,
  );
  assert.match(
    launchShell,
    /hash_refresh_stale_file_list:\s*""/,
  );

  assert.match(runtimeLaunch, /data-dx-check-package-lane-count="0"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-row="state\/zustand"/);
  assert.match(runtimeLaunch, /data-dx-check-package-lane-name="State Management"/);
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-upstream-package="zustand"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-source-mirror="G:\/WWW\/inspirations\/zustand"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-dx-style-status="present"/,
  );
  assert.match(runtimeLaunch, /data-dx-style-surface="state-management"/);
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-hash-refresh-helper="examples\/template\/state-management-receipt-hashes\.ts"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-hash-refresh-json-command="node tools\/launch\/run-template-receipt-helper\.js examples\/template\/state-management-receipt-hashes\.ts --check --json"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-hash-refresh-zed="state-management:receipt-hash-refresh"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-hash-refresh-current-metric="state_management_receipt_hash_refresh_current"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-hash-refresh-stale-metric="state_management_receipt_hash_refresh_stale"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-hash-refresh-missing-metric="state_management_receipt_hash_refresh_missing"/,
  );
  assert.match(
    runtimeLaunch,
    /data-dx-check-package-lane-hash-refresh-stale-file-list=""/,
  );

  for (const source of [editContract, materializer]) {
    assert.match(source, /"data-dx-check-package-lane-count"/);
    assert.match(source, /"data-dx-check-package-lane-row"/);
    assert.match(source, /"data-dx-check-package-lane-status"/);
    assert.match(source, /"data-dx-check-package-lane-receipt-status"/);
    assert.match(source, /"data-dx-check-package-lane-dx-style-status"/);
  }

  assert.match(
    studioManifest,
    /"check_package_lane_count_marker": "data-dx-check-package-lane-count"/,
  );
  assert.match(studioManifest, /studio_marker\(\s*"data-dx-check-package-lane-row"/);
  assert.match(studioManifest, /check_panel\.view_model\.package_lane_rows/);
  assert.match(
    studioManifest,
    /"check_package_lane_dx_style_status_marker": "data-dx-check-package-lane-dx-style-status"/,
  );
  assert.match(
    studioManifest,
    /"check_package_lane_hash_refresh_stale_file_list_marker": "data-dx-check-package-lane-hash-refresh-stale-file-list"/,
  );
  assert.match(
    studioManifest,
    /studio_marker\(\s*"data-dx-check-package-lane-dx-style-status"/,
  );
  assert.match(
    studioManifest,
    new RegExp(
      String.raw`"data-dx-check-package-lane-row",\s*"dx-check-package-lane-row",\s*&\["/"\],\s*"check_panel\.view_model\.package_lane_rows"`,
    ),
  );
  assert.match(
    studioManifest,
    /"state-management-generated-starter-materialization"/,
  );
  assert.ok(
    studioManifest.includes(
      '"dx run --test .\\\\benchmarks\\\\zustand-launch-materialized.test.ts"',
    ),
    "Studio manifest must expose the exact State Management generated-starter guard command",
  );
  assert.match(
    studioManifest,
    /"State Management generated-starter materialization guard"/,
  );
  assert.match(
    studioManifest,
    /"data-dx-check-package-lane-row=\\"state\/zustand\\""/,
  );
  assert.match(
    studioManifest,
    /"state-management:receipt-hash-refresh"/,
  );
  assert.match(
    studioManifest,
    /"state\/zustand source-only Studio discovery"/,
  );
  assert.match(
    studioManifest,
    /"docs\/packages\/state-zustand\.source-guard-runbook\.json"/,
  );
  assert.match(
    studioManifest,
    /"The generated starter preserves the State Management package-lane row, helper freshness markers, and package-scoped dx-check panel without browser storage or visual runtime proof\."/,
  );
  assert.match(
    studioManifest,
    /"state-management-check-panel-stale-helper-attribution"/,
  );
  assert.match(
    studioManifest,
    /"State Management check-panel stale helper attribution guard"/,
  );
  assert.ok(
    studioManifest.includes(
      '"cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_state_management_stale_helper_file_attribution --lib"',
    ),
    "Studio manifest must expose the exact State Management stale-helper Cargo guard command",
  );
  assert.match(
    studioManifest,
    /source_guard_contract_with_fixture\(\s*"state-management-check-panel-stale-helper-attribution"/s,
  );
  assert.match(
    studioManifest,
    /source_guard_command_with_fixture\(\s*"cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_state_management_stale_helper_file_attribution --lib"/s,
  );
  assert.match(
    packageDocs,
    /state-management-generated-starter-materialization/,
  );

  assert.match(dxCheckReceipt, /"state_management_dx_style_compatibility_present"/);
  assert.match(dxCheckReceipt, /"state_management_dx_style_compatibility_missing"/);
  assert.match(dxCheckReceipt, /"state_management_receipt_hash_refresh_current"/);
  assert.match(dxCheckReceipt, /"state_management_receipt_hash_refresh_stale"/);
  assert.match(dxCheckReceipt, /"state_management_receipt_hash_refresh_missing"/);
  assert.match(dxCheckReceipt, /pub stale_files: Vec<String>/);
  assert.match(dxCheckReceipt, /stale_files: json_string_array\(refresh, &\["stale_files"\]\)/);
  assert.match(
    dxCheckReceipt,
    /fn dx_check_latest_panel_exposes_state_management_stale_helper_file_attribution\(\)/,
  );
  assert.match(
    dxCheckReceipt,
    /stale_helper_state_management\["package_lane_visibility"\]\[0\]\["receipt_hash_refresh"\]\["stale_files"\]/,
  );
  assert.match(dxCheckReceipt, /"tools\/launch\/materialize-www-template\.ts"/);
  assert.match(
    dxCheckReceipt,
    /state_management_metric_rows\([^)]*refresh_current[^)]*refresh_stale[^)]*refresh_missing[^)]*dx_style_compatibility_present[^)]*dx_style_compatibility_missing/s,
  );
  assert.match(dxCheckReceipt, /package_lane_hash_refresh\(&package\)/);

  assert.match(packageDocs, /data-dx-check-package-lane-row/);
  assert.match(packageDocs, /data-dx-check-package-lane-dx-style-status/);
  assert.match(packageDocs, /state_management_receipt_hash_refresh_current/);
  assert.match(packageDocs, /state-management:receipt-hash-refresh/);
  assert.match(packageDocs, /package_lane_rows/);
  assert.match(packageDocs, /data-dx-check-package-lane-hash-refresh-stale-file-list/);

  assert.equal(
    runbookFixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(runbookFixture.route, "/");
  assert.equal(
    runbookFixture.package.official_package_name,
    "State Management",
  );
  assert.equal(runbookFixture.package.package_id, "state/zustand");
  assert.equal(runbookFixture.package.upstream_package, "zustand");
  assert.equal(runbookFixture.package.upstream_version, "5.0.13");
  assert.equal(runbookFixture.package.source_mirror, "G:/WWW/inspirations/zustand");
  assert.deepEqual(runbookFixture.selected_surfaces, [
    "launch-dashboard-state-workflow",
    "launch-dashboard-state-shell",
    "generated-starter-materialization",
    "check-panel-stale-helper-attribution",
    "receipt-hash-refresh",
    "studio-manifest-source",
  ]);
  assert.deepEqual(runbookFixture.upstream_public_apis, [
    "createStore",
    "create",
    "useStore",
    "persist",
    "createJSONStorage",
    "onHydrate",
    "onFinishHydration",
    "subscribeWithSelector",
  ]);
  assert.deepEqual(runbookFixture.inspected_upstream_files, [
    "package.json",
    "src/vanilla.ts",
    "src/react.ts",
    "src/middleware/persist.ts",
    "src/middleware/subscribeWithSelector.ts",
  ]);
  assert.equal(
    runbookFixture.guard.id,
    "state-management-generated-starter-materialization",
  );
  assert.equal(
    runbookFixture.guard.guard_file,
    "benchmarks/zustand-launch-materialized.test.ts",
  );
  assert.equal(
    runbookFixture.guard.command,
    "dx run --test .\\benchmarks\\zustand-launch-materialized.test.ts",
  );
  assert.equal(runbookFixture.guard.execution_policy, "source-only");
  assert.equal(runbookFixture.guard.writes_files, false);
  assert.equal(runbookFixture.guard.starts_server, false);
  assert.equal(runbookFixture.guard.runs_package_install, false);
  assert.equal(runbookFixture.guard.runs_full_build, false);
  assert.equal(runbookFixture.guard.node_modules_required, false);

  for (const proof of [
    "State Management generated-starter materialization guard",
    'data-dx-check-package-lane-row="state/zustand"',
    'data-dx-token-scope="state/zustand"',
    "state-management:receipt-hash-refresh",
    "docs/packages/state-zustand.source-guard-runbook.json",
    "without claiming browser storage or visual runtime proof",
    "state/zustand source-only Studio discovery",
  ]) {
    assert.ok(runbookFixture.guard.proves.includes(proof), `missing proof ${proof}`);
    assert.ok(
      studioManifest.includes(proof) ||
        studioManifest.includes(proof.replaceAll('"', '\\"')),
      `Studio manifest missing ${proof}`,
    );
  }

  assert.equal(runbookFixture.runbook.index_field, "source_guard_runbook_index");
  assert.equal(runbookFixture.runbook.default_action, "show-source-only-runbook");
  assert.equal(
    runbookFixture.runbook.contract.id,
    "state-management-generated-starter-materialization",
  );
  assert.equal(
    runbookFixture.runbook.contract.evidence_field,
    "benchmarks/zustand-launch-materialized.test.ts",
  );
  assert.equal(
    runbookFixture.runbook.command.command,
    "dx run --test .\\benchmarks\\zustand-launch-materialized.test.ts",
  );
  assert.equal(runbookFixture.runbook.command.starts_server, false);
  assert.equal(runbookFixture.runbook.command.runs_package_install, false);
  assert.equal(runbookFixture.runbook.command.runs_full_build, false);
  assert.equal(runbookFixture.runbook.command.writes_files, false);
  assert.equal(runbookFixture.runbook.command.node_modules_required, false);

  for (const marker of [
    "data-dx-check-package-lane-row",
    "data-dx-check-package-lane-hash-refresh-helper",
    "data-dx-check-package-lane-hash-refresh-json-command",
    "data-dx-check-package-lane-hash-refresh-zed",
    "data-dx-check-package-lane-hash-refresh-stale-file-list",
    "data-dx-style-surface",
    "data-dx-token-scope",
    "data-dx-package",
  ]) {
    assert.ok(
      runbookFixture.zed_dx_studio_markers.includes(marker),
      `fixture must expose ${marker}`,
    );
  }

  assert.equal(
    runbookFixture.receipt.path,
    "examples/template/.dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json",
  );
  assert.equal(
    runbookFixture.receipt.hash_helper,
    "examples/template/state-management-receipt-hashes.ts",
  );
  assert.equal(
    runbookFixture.receipt.hash_helper_json_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/state-management-receipt-hashes.ts --check --json",
  );
  assert.equal(
    runbookFixture.receipt.studio_manifest_source,
    "dx-www/src/cli/studio_manifest.rs",
  );
  assert.equal(
    runbookFixture.studio_manifest.source_file,
    "dx-www/src/cli/studio_manifest.rs",
  );
  assert.ok(
    runbookFixture.studio_manifest.published_source_guards.includes(
      "state-management-check-panel-stale-helper-attribution",
    ),
    "State Management runbook must hash-back the Studio source-guard declaration surface",
  );
  assert.equal(
    runbookFixture.receipt.stale_helper_attribution.source_guard_id,
    "state-management-check-panel-stale-helper-attribution",
  );
  assert.equal(
    runbookFixture.receipt.stale_helper_attribution.rust_fixture,
    "dx_check_latest_panel_exposes_state_management_stale_helper_file_attribution",
  );
  assert.equal(
    runbookFixture.receipt.stale_helper_attribution.command,
    "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_state_management_stale_helper_file_attribution --lib",
  );
  assert.ok(
    runbookFixture.receipt.stale_helper_attribution.proves.includes(
      "receipt_hash_refresh.stale_files carries tools/launch/materialize-www-template.ts",
    ),
  );
  assert.ok(Array.isArray(runbookFixture.helper_freshness_guards));
  const staleHelperGuard = runbookFixture.helper_freshness_guards.find(
    (guard) => guard.id === "state-management-check-panel-stale-helper-attribution",
  );
  assert.ok(staleHelperGuard, "State Management stale-helper guard missing from runbook fixture");
  assert.equal(staleHelperGuard.guard_file, "core/src/ecosystem/dx_check_receipt.rs");
  assert.equal(
    staleHelperGuard.command,
    "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_state_management_stale_helper_file_attribution --lib",
  );
  assert.equal(staleHelperGuard.execution_policy, "source-only");
  assert.equal(staleHelperGuard.starts_server, false);
  assert.equal(staleHelperGuard.runs_package_install, false);
  assert.equal(staleHelperGuard.runs_full_build, false);
  assert.equal(staleHelperGuard.writes_files, false);
  assert.ok(Array.isArray(runbookFixture.source_guard_fixture_paths));
  assert.deepEqual(
    runbookFixture.source_guard_fixture_paths.find(
      (entry) =>
        entry.source_guard_id ===
        "state-management-check-panel-stale-helper-attribution",
    ),
    {
      source_guard_id: "state-management-check-panel-stale-helper-attribution",
      package_id: "state/zustand",
      fixture_path: "docs/packages/state-zustand.source-guard-runbook.json",
      schema: "dx.forge.package.source_guard_runbook_fixture",
    },
  );
  assert.equal(
    runbookFixture.receipt.zed_visibility,
    "state-management:receipt-hash-refresh",
  );
  assert.equal(runbookFixture.honesty_label, "SOURCE-ONLY");
  assert.equal(runbookFixture.runtime_proof, false);
  assert.match(runbookFixture.runtime_limitations.join("\n"), /browser storage/);

  for (const source of [
    packageDocs,
    frameworkStructure,
    dx,
    todo,
    changelog,
  ]) {
    assert.match(source, /state-zustand\.source-guard-runbook\.json/);
    assert.match(source, /state-management-generated-starter-materialization/);
    assert.match(source, /without claiming browser storage or visual runtime proof/);
  }
});
