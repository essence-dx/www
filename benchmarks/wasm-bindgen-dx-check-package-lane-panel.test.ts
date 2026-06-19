const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceMirror = "G:/WWW/inspirations/wasm-bindgen";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(sourceMirror, relativePath), "utf8");
}

function escaped(marker) {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

test("WebAssembly Bridge package-lane row exposes DX check-panel style visibility", () => {
  const upstreamCargo = readMirror("Cargo.toml");
  const generatedWebTarget = readMirror(
    "crates/cli/tests/reference/targets-target-web.js",
  );
  const reader = read("core/src/ecosystem/dx_check_receipt.rs");
  const packageDoc = read("docs/packages/wasm-bindgen.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.match(upstreamCargo, /name = "wasm-bindgen"/);
  assert.match(upstreamCargo, /version = "0\.2\.121"/);
  assert.match(generatedWebTarget, /async function __wbg_load/);
  assert.match(generatedWebTarget, /WebAssembly\.instantiateStreaming/);
  assert.match(generatedWebTarget, /function initSync/);
  assert.match(generatedWebTarget, /export \{ initSync, __wbg_init as default \}/);

  for (const marker of [
    'WEBASSEMBLY_BRIDGE_PACKAGE_ID: &str = "wasm/bindgen"',
    'WEBASSEMBLY_BRIDGE_OFFICIAL_NAME: &str = "WebAssembly Bridge"',
    'WEBASSEMBLY_BRIDGE_UPSTREAM_PACKAGE: &str = "wasm-bindgen"',
    'WEBASSEMBLY_BRIDGE_UPSTREAM_VERSION: &str = "0.2.121"',
    'WEBASSEMBLY_BRIDGE_SOURCE_MIRROR: &str = "G:/WWW/inspirations/wasm-bindgen"',
    'WEBASSEMBLY_BRIDGE_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json"',
    "WEBASSEMBLY_BRIDGE_PACKAGE_RECEIPT_PATH: &str =",
    "WEBASSEMBLY_BRIDGE_METRICS: [&str; 13]",
    "rows.extend(webassembly_bridge_package_lane_row(root, package_status));",
    "fn webassembly_bridge_package_lane_row(",
    "fn webassembly_bridge_missing_receipt_row(next_action: &str)",
    "fn webassembly_bridge_metric_rows(",
    "fn webassembly_bridge_status_vocabulary(",
    "fn webassembly_bridge_next_action(",
    "webassembly_bridge_hash_manifest_present",
    "webassembly_bridge_hash_mismatch",
    "webassembly_bridge_receipt_hash_refresh_current",
    "webassembly_bridge_receipt_hash_refresh_stale",
    "webassembly_bridge_receipt_hash_refresh_missing",
    "let receipt_hash_refresh = package_lane_hash_refresh(package)",
    "let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package)",
    "dx_check_latest_panel_exposes_webassembly_bridge_package_lane_hash_refresh_row",
    "webassembly_bridge_dx_style_compatibility_present",
    "webassembly_bridge_dx_style_compatibility_missing",
    "count_sha256_file_hash_mismatches(root, package)",
    "dx_style_compatibility_is_present(package)",
    "dx_check_latest_panel_exposes_webassembly_bridge_package_lane_style_row",
  ]) {
    assert.match(reader, escaped(marker), `missing check-panel marker ${marker}`);
  }

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /DX Studio\/check-panel WebAssembly Bridge package row/);
    assert.match(source, /webassembly_bridge_hash_manifest_present/);
    assert.match(source, /webassembly_bridge_hash_mismatch/);
    assert.match(source, /webassembly_bridge_receipt_hash_refresh_current/);
    assert.match(source, /webassembly_bridge_receipt_hash_refresh_stale/);
    assert.match(source, /webassembly_bridge_receipt_hash_refresh_missing/);
    assert.match(source, /receipt_hash_refresh path arrays/);
    assert.match(source, /webassembly_bridge_dx_style_compatibility_present/);
    assert.match(source, /webassembly_bridge_dx_style_compatibility_missing/);
    assert.match(source, /without claiming live generated-Wasm or browser style proof/);
  }
});

test("WebAssembly Bridge static launch package-lane template is Studio-discoverable", () => {
  const upstreamCargo = readMirror("Cargo.toml");
  const generatedWebTarget = readMirror(
    "crates/cli/tests/reference/targets-target-web.js",
  );
  const launchShell = read("examples/template/template-shell.tsx");
  const runtimePage = read("tools/launch/runtime-template/pages/index.html");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const materializer = read("tools/launch/materialize-www-template.ts");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const packageDoc = read("docs/packages/wasm-bindgen.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.match(upstreamCargo, /name = "wasm-bindgen"/);
  assert.match(upstreamCargo, /version = "0\.2\.121"/);
  assert.match(generatedWebTarget, /async function __wbg_load/);
  assert.match(generatedWebTarget, /WebAssembly\.instantiateStreaming/);
  assert.match(generatedWebTarget, /function initSync/);
  assert.match(generatedWebTarget, /export \{ initSync, __wbg_init as default \}/);

  const wwwTemplateStart = launchShell.indexOf('package_id: "wasm/bindgen"');
  assert.notEqual(wwwTemplateStart, -1, "missing template-shell template row");
  const wwwTemplate = launchShell.slice(
    wwwTemplateStart,
    launchShell.indexOf("next_action:", wwwTemplateStart) + 400,
  );
  for (const marker of [
    'official_package_name: "WebAssembly Bridge"',
    'upstream_package: "wasm-bindgen"',
    'upstream_version: "0.2.121"',
    'source_mirror: "G:/WWW/inspirations/wasm-bindgen"',
    'receipt_status: "missing-receipt"',
    'package_receipt_path:',
    "2026-05-22-wasm-bindgen-dashboard-workflow.json",
    'dx_style_status: "present"',
    'dx_style_surface: "theme-token"',
    'token_scope: "wasm/bindgen"',
  ]) {
    assert.match(wwwTemplate, escaped(marker), `missing shell marker ${marker}`);
  }

  for (const marker of [
    'data-dx-check-package-lane-template="wasm/bindgen"',
    'data-dx-check-package-lane-row="wasm/bindgen"',
    'data-dx-check-package-lane-name="WebAssembly Bridge"',
    'data-dx-check-package-lane-status="missing"',
    'data-dx-check-package-lane-receipt-status="missing-receipt"',
    'data-dx-check-package-lane-upstream-package="wasm-bindgen"',
    'data-dx-check-package-lane-source-mirror="G:/WWW/inspirations/wasm-bindgen"',
    'data-dx-check-package-lane-receipt-path="examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json"',
    'data-dx-check-package-lane-dx-style-status="present"',
    'data-dx-style-surface="theme-token"',
    'data-dx-token-scope="wasm/bindgen"',
    'data-dx-package="wasm/bindgen"',
    "webassembly_bridge_hash_manifest_present",
    "webassembly_bridge_hash_mismatch",
    "webassembly_bridge_dx_style_compatibility_present",
    "webassembly_bridge_dx_style_compatibility_missing",
    "live generated-Wasm runtime proof remains app-owned",
  ]) {
    assert.match(runtimePage, escaped(marker), `missing runtime marker ${marker}`);
  }

  const editContractPanel = editContract.slice(
    editContract.indexOf('id: "dx-check-health-panel"'),
    editContract.indexOf("operations:", editContract.indexOf('id: "dx-check-health-panel"')),
  );
  const materializerPanel = materializer.slice(
    materializer.indexOf('"launch-runtime-dx-check-panel"'),
    materializer.indexOf('"pages/index.html"', materializer.indexOf('"launch-runtime-dx-check-panel"')),
  );
  const manifestPanel = studioManifest.slice(
    studioManifest.indexOf("fn studio_dx_check_edit_surface()"),
    studioManifest.indexOf("&[\"move_reorder_section\"", studioManifest.indexOf("fn studio_dx_check_edit_surface()")),
  );

  assert.match(editContractPanel, /"wasm\/bindgen"/);
  assert.match(materializerPanel, /"wasm\/bindgen"/);
  assert.match(manifestPanel, /"wasm\/bindgen"/);

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /static `\/` WebAssembly Bridge package-lane template/);
    assert.match(source, /data-dx-check-package-lane-template="wasm\/bindgen"/);
    assert.match(source, /data-dx-style-surface="theme-token"/);
    assert.match(source, /live generated-Wasm runtime proof remains app-owned/);
  }
});

test("WebAssembly Bridge package-lane row survives generated starter materialization", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-wasm-bindgen-package-lane-"));
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
    const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
    const frameworkStructure = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");

    assert.equal(result.ok, true);
    assert.equal(result.noNodeModules, true);
    assert.ok(!fs.existsSync(path.join(dir, "node_modules")));

    for (const marker of [
      'data-dx-check-package-lane-template="wasm/bindgen"',
      'data-dx-check-package-lane-row="wasm/bindgen"',
      'data-dx-check-package-lane-name="WebAssembly Bridge"',
      'data-dx-check-package-lane-status="missing"',
      'data-dx-check-package-lane-receipt-status="missing-receipt"',
      'data-dx-check-package-lane-upstream-package="wasm-bindgen"',
      'data-dx-check-package-lane-source-mirror="G:/WWW/inspirations/wasm-bindgen"',
      'data-dx-check-package-lane-receipt-path="examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json"',
      'data-dx-check-package-lane-dx-style-status="present"',
      'data-dx-style-surface="theme-token"',
      'data-dx-token-scope="wasm/bindgen"',
      'data-dx-package="wasm/bindgen"',
      "webassembly_bridge_hash_manifest_present",
      "webassembly_bridge_hash_mismatch",
      "webassembly_bridge_dx_style_compatibility_present",
      "webassembly_bridge_dx_style_compatibility_missing",
      "live generated-Wasm runtime proof remains app-owned",
    ]) {
      assert.match(
        launch,
        escaped(marker),
        `missing generated WebAssembly Bridge marker ${marker}`,
      );
    }

    for (const routePath of ["/"]) {
      const route = manifest.routes.find((entry) => entry.route === routePath);
      assert.ok(route, `expected generated ${routePath} route metadata`);
      assert.ok(
        route.forgePackages.includes("wasm/bindgen"),
        `generated ${routePath} route package scope must include WebAssembly Bridge`,
      );
    }

    assert.ok(
      Array.isArray(manifest.sourceGuardRunbookFixtures),
      "generated preview manifest must expose source-guard runbook fixtures",
    );
    const wasmRunbookFixture = manifest.sourceGuardRunbookFixtures.find(
      (fixture) => fixture.packageId === "wasm/bindgen",
    );
    assert.ok(
      wasmRunbookFixture,
      "generated preview manifest must expose WebAssembly Bridge source-guard runbook fixture",
    );
    assert.equal(wasmRunbookFixture.officialPackageName, "WebAssembly Bridge");
    assert.equal(wasmRunbookFixture.upstreamPackage, "wasm-bindgen");
    assert.equal(wasmRunbookFixture.upstreamVersion, "0.2.121");
    assert.equal(
      wasmRunbookFixture.sourceMirror,
      "G:/WWW/inspirations/wasm-bindgen",
    );
    assert.equal(
      wasmRunbookFixture.fixture,
      "docs/packages/wasm-bindgen.source-guard-runbook.json",
    );
    assert.equal(
      wasmRunbookFixture.guardId,
      "webassembly-bridge-generated-starter-materialization",
    );
    assert.equal(wasmRunbookFixture.route, "/");
    assert.equal(wasmRunbookFixture.honestyLabel, "SOURCE-ONLY");
    assert.equal(wasmRunbookFixture.runtimeProof, false);
    assert.equal(
      wasmRunbookFixture.zedVisibility,
      "webassembly-bridge:source-guard-runbook",
    );
    const launchRoute = manifest.routes.find((entry) => entry.route === "/");
    assert.ok(
      launchRoute.sourceGuardRunbookFixtures.includes(
        "docs/packages/wasm-bindgen.source-guard-runbook.json",
      ),
      "generated / route must link the WebAssembly Bridge source-guard runbook fixture",
    );

    const checkPanel = manifest.editContract.editableSurfaces.find(
      (surface) => surface.id === "launch-runtime-dx-check-panel",
    );
    assert.ok(checkPanel, "expected generated dx-check panel edit surface");
    assert.equal(checkPanel.sourceFile, "pages/index.html");
    assert.ok(
      checkPanel.packageIds.includes("wasm/bindgen"),
      "generated dx-check panel package scope must include WebAssembly Bridge",
    );

    for (const marker of [
      "data-dx-check-package-lane-template",
      "data-dx-check-package-lane-row",
      "data-dx-check-package-lane-dx-style-status",
      "data-dx-style-surface",
      "data-dx-token-scope",
    ]) {
      assert.ok(
        checkPanel.stateMarkers.includes(marker),
        `generated dx-check panel must expose ${marker}`,
      );
    }

    assert.match(
      studioManifest,
      /studio_source_guard_with_fixture\(\s*"webassembly-bridge-generated-starter-materialization"/,
      "Rust Studio manifest must publish the WebAssembly Bridge generated-starter source guard",
    );
    assert.match(
      studioManifest,
      /"webassembly-bridge-generated-starter-materialization"[\s\S]*"benchmarks\/wasm-bindgen-dx-check-package-lane-panel\.test\.ts"/,
      "WebAssembly Bridge source guard must point at the focused lane benchmark",
    );
    assert.match(
      studioManifest,
      /"webassembly-bridge-generated-starter-materialization"[\s\S]*"dx run --test \.\\\\benchmarks\\\\wasm-bindgen-dx-check-package-lane-panel\.test\.ts"/,
      "WebAssembly Bridge source guard must expose the exact lightweight command",
    );
    assert.match(
      studioManifest,
      /"webassembly-bridge-generated-starter-materialization"[\s\S]*"WebAssembly Bridge generated-starter materialization guard"/,
    );
    assert.match(
      studioManifest,
      /"webassembly-bridge-generated-starter-materialization"[\s\S]*"data-dx-token-scope=\\\"wasm\/bindgen\\\""/,
    );
    assert.match(
      studioManifest,
      /"webassembly-bridge-generated-starter-materialization"[\s\S]*"docs\/packages\/wasm-bindgen\.source-guard-runbook\.json"/,
      "WebAssembly Bridge source guard must link its package-owned runbook fixture",
    );
    assert.match(
      studioManifest,
      /"\/" => guards\.extend\(\[[\s\S]*"webassembly-bridge-generated-starter-materialization"/,
      "the / source guard runbook must list the WebAssembly Bridge generated-starter guard",
    );
    assert.match(
      studioManifest,
      /"webassembly-bridge-generated-starter-materialization"[\s\S]*"The generated starter preserves the WebAssembly Bridge package-lane row, provenance, dx-style markers, and package-scoped dx-check panel without live generated-Wasm runtime proof\."/,
    );
    assert.match(
      studioManifest,
      /"dx run --test \.\\\\benchmarks\\\\wasm-bindgen-dx-check-package-lane-panel\.test\.ts"[\s\S]*"Validate the source-only WebAssembly Bridge package-lane row, generated starter materialization, provenance, dx-style markers, and dx-check panel package scope without live generated-Wasm runtime proof\."/,
    );

    assert.match(
      frameworkStructure,
      /Lane 18 uses the official front-facing package name `WebAssembly Bridge`/,
    );
    assert.match(frameworkStructure, /webassembly-bridge-generated-starter-materialization/);
    assert.match(
      frameworkStructure,
      /dx run --test \.\\benchmarks\\wasm-bindgen-dx-check-package-lane-panel\.test\.ts/,
    );
    assert.match(frameworkStructure, /source_guard_runbook_index/);
    assert.match(frameworkStructure, /wasm-bindgen\.source-guard-runbook\.json/);
    assert.match(frameworkStructure, /without claiming live generated-Wasm runtime proof/);

    for (const source of [
      read("docs/packages/wasm-bindgen.md"),
      read("DX.md"),
      read("TODO.md"),
      read("CHANGELOG.md"),
    ]) {
      assert.match(source, /WebAssembly Bridge generated-starter materialization guard/);
      assert.match(source, /WebAssembly Bridge Studio source-guard\/runbook entry/);
      assert.match(source, /wasm-bindgen\.source-guard-runbook\.json/);
      assert.match(source, /without claiming live generated-Wasm runtime proof/);
    }
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("WebAssembly Bridge source-guard runbook fixture mirrors the Studio manifest", () => {
  const fixture = readJson("docs/packages/wasm-bindgen.source-guard-runbook.json");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const packageDoc = read("docs/packages/wasm-bindgen.md");
  const frameworkDoc = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(
    fixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(fixture.route, "/");
  assert.equal(fixture.package.official_package_name, "WebAssembly Bridge");
  assert.equal(fixture.package.package_id, "wasm/bindgen");
  assert.equal(fixture.package.upstream_package, "wasm-bindgen");
  assert.equal(fixture.package.upstream_version, "0.2.121");
  assert.equal(
    fixture.package.source_mirror,
    "G:/WWW/inspirations/wasm-bindgen",
  );

  assert.equal(
    fixture.guard.id,
    "webassembly-bridge-generated-starter-materialization",
  );
  assert.equal(
    fixture.guard.guard_file,
    "benchmarks/wasm-bindgen-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(
    fixture.guard.command,
    "dx run --test .\\benchmarks\\wasm-bindgen-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(fixture.guard.execution_policy, "source-only");
  assert.equal(fixture.guard.writes_files, false);
  assert.equal(fixture.guard.starts_server, false);
  assert.equal(fixture.guard.runs_package_install, false);
  assert.equal(fixture.guard.runs_full_build, false);
  assert.equal(fixture.guard.node_modules_required, false);

  for (const proof of [
    "WebAssembly Bridge generated-starter materialization guard",
    'data-dx-check-package-lane-row="wasm/bindgen"',
    'data-dx-token-scope="wasm/bindgen"',
    "docs/packages/wasm-bindgen.source-guard-runbook.json",
    "without claiming live generated-Wasm runtime proof",
    "wasm-bindgen 0.2.121 source-only Studio discovery",
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
    "benchmarks/wasm-bindgen-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(
    fixture.runbook.command.purpose,
    "Validate the source-only WebAssembly Bridge package-lane row, generated starter materialization, provenance, dx-style markers, and dx-check panel package scope without live generated-Wasm runtime proof.",
  );

  for (const marker of [
    "data-dx-check-package-lane-row",
    "data-dx-check-package-lane-dx-style-status",
    "data-dx-style-surface",
    "data-dx-token-scope",
    "data-dx-package",
    "webassembly_bridge_hash_manifest_present",
    "webassembly_bridge_hash_mismatch",
    "webassembly_bridge_dx_style_compatibility_present",
    "webassembly_bridge_dx_style_compatibility_missing",
  ]) {
    assert.ok(
      fixture.zed_dx_studio_markers.includes(marker),
      `fixture must expose ${marker}`,
    );
  }

  assert.equal(
    fixture.receipt.path,
    "examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
  );
  assert.equal(
    fixture.receipt.hash_receipt_guard,
    "benchmarks/wasm-bindgen-hash-receipt.test.ts",
  );
  assert.equal(
    fixture.preview_manifest.generated_file,
    "public/preview-manifest.json",
  );
  assert.equal(
    fixture.preview_manifest.materializer,
    "tools/launch/materialize-www-template.ts",
  );
  assert.equal(fixture.preview_manifest.root_field, "sourceGuardRunbookFixtures");
  assert.equal(
    fixture.preview_manifest.route_field,
    "routes[].sourceGuardRunbookFixtures",
  );
  assert.equal(
    fixture.preview_manifest.fixture,
    "docs/packages/wasm-bindgen.source-guard-runbook.json",
  );
  assert.equal(fixture.preview_manifest.runtime_proof, false);
  assert.equal(
    fixture.studio_manifest.source_guard_entry,
    "source_guard_index[].fixture_path",
  );
  assert.equal(
    fixture.studio_manifest.runbook_fixture_paths,
    "source_guard_runbook_index[].fixture_paths[]",
  );
  assert.equal(
    fixture.studio_manifest.contract_entry,
    "source_guard_runbook_index[].contracts[].fixture_path",
  );
  assert.equal(
    fixture.studio_manifest.command_entry,
    "source_guard_runbook_index[].commands[].fixture_path",
  );
  assert.equal(
    fixture.studio_manifest.fixture_path,
    "docs/packages/wasm-bindgen.source-guard-runbook.json",
  );
  assert.equal(fixture.honesty_label, "SOURCE-ONLY");
  assert.equal(fixture.runtime_proof, false);
  assert.match(fixture.runtime_limitations.join("\n"), /generated-Wasm runtime proof/);

  assert.match(
    studioManifest,
    /"source_guard_id": "webassembly-bridge-generated-starter-materialization"[\s\S]*"package_id": "wasm\/bindgen"[\s\S]*"fixture_path": "docs\/packages\/wasm-bindgen\.source-guard-runbook\.json"[\s\S]*"schema": "dx\.forge\.package\.source_guard_runbook_fixture"/,
  );
  assert.match(
    studioManifest,
    /source_guard_contract_with_fixture\(\s*"webassembly-bridge-generated-starter-materialization",\s*"The generated starter preserves the WebAssembly Bridge package-lane row, provenance, dx-style markers, and package-scoped dx-check panel without live generated-Wasm runtime proof\.",\s*"benchmarks\/wasm-bindgen-dx-check-package-lane-panel\.test\.ts",\s*"docs\/packages\/wasm-bindgen\.source-guard-runbook\.json",\s*\)/,
  );
  assert.match(
    studioManifest,
    /source_guard_command_with_fixture\(\s*"dx run --test \.\\\\benchmarks\\\\wasm-bindgen-dx-check-package-lane-panel\.test\.ts",\s*"Validate the source-only WebAssembly Bridge package-lane row, generated starter materialization, provenance, dx-style markers, and dx-check panel package scope without live generated-Wasm runtime proof\.",\s*"docs\/packages\/wasm-bindgen\.source-guard-runbook\.json",\s*\)/,
  );

  for (const source of [
    studioManifest,
    packageDoc,
    frameworkDoc,
    dx,
    todo,
    changelog,
  ]) {
    assert.match(source, /wasm-bindgen\.source-guard-runbook\.json/);
    assert.match(source, /webassembly-bridge-generated-starter-materialization/);
    assert.match(source, /without claiming live generated-Wasm runtime proof/);
  }
});

test("WebAssembly Bridge helper-freshness proof is a Studio source guard", () => {
  const fixture = readJson("docs/packages/wasm-bindgen.source-guard-runbook.json");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const packageDoc = read("docs/packages/wasm-bindgen.md");
  const frameworkDoc = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  const guardId = "webassembly-bridge-lower-dx-check-helper-freshness";
  const guardFile = "core/src/ecosystem/project_check/wasm_bindgen_dx_check.rs";
  const guardCommand =
    "cargo test -q -p dx-www-compiler webassembly_bridge_package_metrics_reports_helper_freshness_from_path_arrays --lib";
  const fixturePath = "docs/packages/wasm-bindgen.source-guard-runbook.json";

  assert.equal(fixture.helper_freshness_guard.id, guardId);
  assert.deepEqual(fixture.helper_freshness_guard.routes, ["/"]);
  assert.equal(fixture.helper_freshness_guard.guard_file, guardFile);
  assert.equal(fixture.helper_freshness_guard.command, guardCommand);
  assert.equal(fixture.helper_freshness_guard.execution_policy, "source-only");
  assert.equal(fixture.helper_freshness_guard.runtime_proof, false);
  assert.equal(fixture.helper_freshness_guard.writes_files, false);
  assert.equal(fixture.helper_freshness_guard.starts_server, false);
  assert.equal(fixture.helper_freshness_guard.runs_package_install, false);
  assert.equal(fixture.helper_freshness_guard.runs_full_build, false);
  assert.equal(fixture.helper_freshness_guard.node_modules_required, false);

  for (const proof of [
    "WebAssembly Bridge lower dx-check helper freshness fixture",
    "webassembly_bridge_receipt_hash_refresh_current",
    "webassembly_bridge_receipt_hash_refresh_stale",
    "webassembly_bridge_receipt_hash_refresh_missing",
    "webassembly_bridge_hash_mismatch stays byte-derived",
    fixturePath,
    "without claiming live generated-Wasm runtime proof",
    "wasm-bindgen 0.2.121 source-only Studio discovery",
  ]) {
    assert.ok(
      fixture.helper_freshness_guard.proves.includes(proof),
      `missing helper-freshness proof ${proof}`,
    );
    assert.ok(
      studioManifest.includes(proof) ||
        studioManifest.includes(proof.replaceAll("\\", "\\\\").replaceAll('"', '\\"')),
      `Studio manifest missing helper-freshness proof ${proof}`,
    );
  }

  assert.match(
    studioManifest,
    /studio_source_guard_with_fixture\(\s*"webassembly-bridge-lower-dx-check-helper-freshness",\s*&\["\/"\],\s*"core\/src\/ecosystem\/project_check\/wasm_bindgen_dx_check\.rs",\s*"cargo test -q -p dx-www-compiler webassembly_bridge_package_metrics_reports_helper_freshness_from_path_arrays --lib"/,
  );
  assert.match(
    studioManifest,
    /"source_guard_id": "webassembly-bridge-lower-dx-check-helper-freshness"[\s\S]*"package_id": "wasm\/bindgen"[\s\S]*"fixture_path": "docs\/packages\/wasm-bindgen\.source-guard-runbook\.json"/,
  );
  assert.match(
    studioManifest,
    /source_guard_contract_with_fixture\(\s*"webassembly-bridge-lower-dx-check-helper-freshness",\s*"The lower-level WebAssembly Bridge dx-check producer reports helper freshness path arrays while keeping webassembly_bridge_hash_mismatch byte-derived\.",\s*"cargo test -q -p dx-www-compiler webassembly_bridge_package_metrics_reports_helper_freshness_from_path_arrays --lib",\s*"docs\/packages\/wasm-bindgen\.source-guard-runbook\.json",\s*\)/,
  );
  assert.match(
    studioManifest,
    /source_guard_command_with_fixture\(\s*"cargo test -q -p dx-www-compiler webassembly_bridge_package_metrics_reports_helper_freshness_from_path_arrays --lib",\s*"Validate the source-only WebAssembly Bridge lower dx-check helper freshness metrics and path-array attribution without live generated-Wasm runtime proof\.",\s*"docs\/packages\/wasm-bindgen\.source-guard-runbook\.json",\s*\)/,
  );

  for (const source of [
    packageDoc,
    frameworkDoc,
    dx,
    todo,
    changelog,
  ]) {
    assert.match(source, /WebAssembly Bridge lower dx-check helper freshness guard/);
    assert.match(
      source,
      /webassembly_bridge_hash_mismatch stays byte-derived/,
    );
    assert.match(source, /without live generated-Wasm runtime proof/);
  }
});

test("WebAssembly Bridge check-panel helper-freshness proof is a Studio source guard", () => {
  const fixture = readJson("docs/packages/wasm-bindgen.source-guard-runbook.json");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const packageDoc = read("docs/packages/wasm-bindgen.md");
  const frameworkDoc = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  const guardId = "webassembly-bridge-check-panel-helper-freshness";
  const guardFile = "core/src/ecosystem/dx_check_receipt.rs";
  const guardCommand =
    "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_webassembly_bridge_package_lane_hash_refresh_row --lib";
  const fixturePath = "docs/packages/wasm-bindgen.source-guard-runbook.json";

  assert.equal(fixture.check_panel_helper_freshness_guard.id, guardId);
  assert.deepEqual(fixture.check_panel_helper_freshness_guard.routes, ["/"]);
  assert.equal(fixture.check_panel_helper_freshness_guard.guard_file, guardFile);
  assert.equal(fixture.check_panel_helper_freshness_guard.command, guardCommand);
  assert.equal(fixture.check_panel_helper_freshness_guard.execution_policy, "source-only");
  assert.equal(fixture.check_panel_helper_freshness_guard.runtime_proof, false);
  assert.equal(fixture.check_panel_helper_freshness_guard.writes_files, false);
  assert.equal(fixture.check_panel_helper_freshness_guard.starts_server, false);
  assert.equal(fixture.check_panel_helper_freshness_guard.runs_package_install, false);
  assert.equal(fixture.check_panel_helper_freshness_guard.runs_full_build, false);
  assert.equal(fixture.check_panel_helper_freshness_guard.node_modules_required, false);

  for (const proof of [
    "WebAssembly Bridge check-panel helper freshness fixture",
    "webassembly_bridge_receipt_hash_refresh_current",
    "webassembly_bridge_receipt_hash_refresh_stale",
    "webassembly_bridge_receipt_hash_refresh_missing",
    "webassembly_bridge_hash_mismatch stays byte-derived",
    fixturePath,
    "without claiming live generated-Wasm runtime proof",
    "wasm-bindgen 0.2.121 source-only Studio discovery",
  ]) {
    assert.ok(
      fixture.check_panel_helper_freshness_guard.proves.includes(proof),
      `missing check-panel helper-freshness proof ${proof}`,
    );
    assert.ok(
      studioManifest.includes(proof) ||
        studioManifest.includes(proof.replaceAll("\\", "\\\\").replaceAll('"', '\\"')),
      `Studio manifest missing check-panel helper-freshness proof ${proof}`,
    );
  }

  assert.match(
    studioManifest,
    /studio_source_guard_with_fixture\(\s*"webassembly-bridge-check-panel-helper-freshness",\s*&\["\/"\],\s*"core\/src\/ecosystem\/dx_check_receipt\.rs",\s*"cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_webassembly_bridge_package_lane_hash_refresh_row --lib"/,
  );
  assert.match(
    studioManifest,
    /"source_guard_id": "webassembly-bridge-check-panel-helper-freshness"[\s\S]*"package_id": "wasm\/bindgen"[\s\S]*"fixture_path": "docs\/packages\/wasm-bindgen\.source-guard-runbook\.json"/,
  );
  assert.match(
    studioManifest,
    /source_guard_contract_with_fixture\(\s*"webassembly-bridge-check-panel-helper-freshness",\s*"The DX Studio\/check-panel WebAssembly Bridge row reports helper freshness path arrays while keeping webassembly_bridge_hash_mismatch byte-derived\.",\s*"cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_webassembly_bridge_package_lane_hash_refresh_row --lib",\s*"docs\/packages\/wasm-bindgen\.source-guard-runbook\.json",\s*\)/,
  );
  assert.match(
    studioManifest,
    /source_guard_command_with_fixture\(\s*"cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_webassembly_bridge_package_lane_hash_refresh_row --lib",\s*"Validate the source-only WebAssembly Bridge check-panel helper freshness metrics and path-array attribution without live generated-Wasm runtime proof\.",\s*"docs\/packages\/wasm-bindgen\.source-guard-runbook\.json",\s*\)/,
  );

  for (const source of [
    packageDoc,
    frameworkDoc,
    dx,
    todo,
    changelog,
  ]) {
    assert.match(source, /WebAssembly Bridge check-panel helper freshness guard/);
    assert.match(
      source,
      /webassembly_bridge_hash_mismatch stays byte-derived/,
    );
    assert.match(source, /without live generated-Wasm runtime proof/);
  }
});
