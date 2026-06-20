const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const threeRoot = path.resolve(root, "..", "..", "WWW/inspirations/three.js");
const fiberRoot = path.resolve(
  root,
  "..",
  "..",
  "WWW/inspirations/react-three-fiber",
);
const dreiRoot = path.resolve(root, "..", "..", "WWW/inspirations/drei");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function readMirror(base, relativePath) {
  return fs.readFileSync(path.join(base, relativePath), "utf8");
}

function escaped(marker) {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

test("3D Scene System package-lane row exposes dx-style evidence in the DX check panel", () => {
  const runbookFixturePath =
    "docs/packages/3d-scene-system.source-guard-runbook.json";
  assert.ok(
    fs.existsSync(path.join(root, runbookFixturePath)),
    "3D Scene System source-guard runbook fixture should exist",
  );

  const threePackage = JSON.parse(readMirror(threeRoot, "package.json"));
  const fiberPackage = JSON.parse(
    readMirror(fiberRoot, "packages/fiber/package.json"),
  );
  const webglRenderer = readMirror(
    threeRoot,
    "src/renderers/WebGLRenderer.js",
  );
  const fiberRenderer = readMirror(
    fiberRoot,
    "packages/fiber/src/core/renderer.tsx",
  );
  const dreiBounds = readMirror(dreiRoot, "src/core/Bounds.tsx");
  const reader = read("core/src/ecosystem/dx_check_receipt.rs");
  const launchShell = read("examples/template/template-shell.tsx");
  const packageDoc = read("docs/packages/3d-scene-system.md");
  const runtimeLaunch = read("tools/launch/runtime-template/pages/index.html");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const materializer = read("tools/launch/materialize-www-template.ts");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const frameworkDocs = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const runbookFixture = readJson(runbookFixturePath);
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(threePackage.name, "three");
  assert.equal(threePackage.version, "0.184.0");
  assert.equal(fiberPackage.name, "@react-three/fiber");
  assert.equal(fiberPackage.version, "9.6.1");
  assert.match(webglRenderer, /class WebGLRenderer/);
  assert.match(fiberRenderer, /export function createRoot/);
  assert.match(dreiBounds, /export function Bounds/);

  for (const marker of [
    'THREE_SCENE_SYSTEM_PACKAGE_ID: &str = "3d/launch-scene"',
    'THREE_SCENE_SYSTEM_OFFICIAL_NAME: &str = "3D Scene System"',
    'THREE_SCENE_SYSTEM_UPSTREAM_PACKAGE: &str = "three + @react-three/fiber + @react-three/drei"',
    'THREE_SCENE_SYSTEM_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json"',
    "THREE_SCENE_SYSTEM_PACKAGE_RECEIPT_PATH: &str =",
    '".dx/forge/receipts/3d-launch-scene-dashboard-workflow.json"',
    "rows.extend(three_scene_system_package_lane_row(root, package_status));",
    "fn three_scene_system_package_lane_row(",
    "fn three_scene_system_metric_rows(",
    "fn three_scene_system_status_vocabulary(",
    "fn three_scene_system_next_action(",
    "three_scene_system_hash_manifest_present",
    "three_scene_system_hash_mismatch",
    "three_scene_system_receipt_hash_refresh_current",
    "three_scene_system_receipt_hash_refresh_stale",
    "three_scene_system_receipt_hash_refresh_missing",
    "three_scene_system_dx_style_compatibility_present",
    "three_scene_system_dx_style_compatibility_missing",
    "count_sha256_file_hash_mismatches(root, package)",
    "let receipt_hash_refresh = package_lane_hash_refresh(package);",
    "let (refresh_current, refresh_stale, refresh_missing) = receipt_hash_refresh_counts(package);",
    "|| refresh_stale > 0",
    "|| refresh_missing > 0",
    "dx_style_compatibility_is_present(package)",
    "dx_check_latest_panel_exposes_three_scene_system_package_lane_style_row",
  ]) {
    assert.match(reader, escaped(marker), `missing check-panel marker ${marker}`);
  }

  for (const marker of [
    'data-dx-check-package-lane-template="3d/launch-scene"',
    'data-dx-check-package-lane-row="3d/launch-scene"',
    'data-dx-check-package-lane-name="3D Scene System"',
    'data-dx-check-package-lane-status="missing"',
    'data-dx-check-package-lane-receipt-status="missing-receipt"',
    'data-dx-check-package-lane-upstream-package="three + @react-three/fiber + @react-three/drei"',
    'data-dx-check-package-lane-source-mirror="G:/WWW/inspirations/three.js; G:/WWW/inspirations/react-three-fiber; G:/WWW/inspirations/drei"',
    'data-dx-check-package-lane-receipt-path=".dx/forge/receipts/3d-launch-scene-dashboard-workflow.json"',
    'data-dx-check-package-lane-dx-style-status="missing"',
    'data-dx-style-surface="launch-scene"',
    'data-dx-token-scope="3d/launch-scene"',
  ]) {
    assert.match(
      runtimeLaunch,
      escaped(marker),
      `missing static launch package-lane marker ${marker}`,
    );
  }

  assert.match(
    launchShell,
    /package_id: "3d\/launch-scene"[\s\S]*official_package_name: "3D Scene System"[\s\S]*upstream_package: "three \+ @react-three\/fiber \+ @react-three\/drei"/,
  );
  assert.match(
    launchShell,
    /data-dx-check-package-lane-template=\{packageLane\.package_id\}/,
  );
  assert.match(
    launchShell,
    /data-dx-check-package-lane-row=\{packageLane\.package_id\}/,
  );
  assert.match(
    editContract,
    /id: "dx-check-health-panel"[\s\S]*packageIds: \[[\s\S]*"3d\/launch-scene"/,
  );
  assert.match(
    materializer,
    /"launch-runtime-dx-check-panel"[\s\S]*\[[\s\S]*"3d\/launch-scene"/,
  );
  assert.match(
    studioManifest,
    /"dx-check-health-panel"[\s\S]*&\[[\s\S]*"3d\/launch-scene"/,
  );
  assert.match(
    studioManifest,
    /"package": "3d\/launch-scene"[\s\S]*"front_facing_name": "3D Scene System"[\s\S]*"data-dx-check-package-lane-row"/,
  );
  assert.match(
    studioManifest,
    /studio_source_guard_with_fixture\(\s*"three-scene-system-lower-dx-check-helper-freshness"/,
  );
  assert.match(
    studioManifest,
    /cargo test -q -p dx-www-compiler three_scene_system_hash_refresh_stale_helper_keeps_source_hash_clean --lib/,
  );
  assert.match(
    studioManifest,
    /source_guard_contract_with_fixture\(\s*"three-scene-system-lower-dx-check-helper-freshness"[\s\S]*three_scene_system_receipt_hash_refresh_stale[\s\S]*docs\/packages\/3d-scene-system\.source-guard-runbook\.json/,
  );
  assert.match(
    studioManifest,
    /source_guard_command_with_fixture\(\s*"cargo test -q -p dx-www-compiler three_scene_system_hash_refresh_stale_helper_keeps_source_hash_clean --lib"[\s\S]*3D Scene System[\s\S]*docs\/packages\/3d-scene-system\.source-guard-runbook\.json/,
  );
  assert.match(
    studioManifest,
    /"source_guard_id": "three-scene-system-lower-dx-check-helper-freshness"[\s\S]*"package_id": "3d\/launch-scene"[\s\S]*"fixture_path": "docs\/packages\/3d-scene-system\.source-guard-runbook\.json"/,
  );

  assert.match(packageDoc, /static `\/launch` runtime fixture/);
  assert.match(packageDoc, /data-dx-check-package-lane-row="3d\/launch-scene"/);
  assert.match(packageDoc, /three-scene-system-lower-dx-check-helper-freshness/);
  assert.match(
    packageDoc,
    /docs\/packages\/3d-scene-system\.source-guard-runbook\.json/,
  );
  assert.match(
    frameworkDocs,
    /Lane 17 uses the official front-facing package name `3D Scene System`[\s\S]*docs\/packages\/3d-scene-system\.source-guard-runbook\.json/,
  );

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /DX Studio\/check-panel 3D Scene System package row/);
    assert.match(source, /DX Studio package-scoped filter/);
    assert.match(source, /generated-starter materialization guard for 3D Scene System/);
    assert.match(source, /static `\/launch` package-lane marker/);
    assert.match(source, /three_scene_system_receipt_hash_refresh_current/);
    assert.match(source, /three_scene_system_receipt_hash_refresh_stale/);
    assert.match(source, /three_scene_system_receipt_hash_refresh_missing/);
    assert.match(source, /three_scene_system_dx_style_compatibility_present/);
    assert.match(source, /three_scene_system_dx_style_compatibility_missing/);
    assert.match(source, /three-scene-system-lower-dx-check-helper-freshness/);
    assert.match(source, /without claiming live browser\/WebGL proof/);
  }

  assert.equal(
    runbookFixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(runbookFixture.route, "/");
  assert.equal(runbookFixture.package.official_package_name, "3D Scene System");
  assert.equal(runbookFixture.package.package_id, "3d/launch-scene");
  assert.equal(
    runbookFixture.package.upstream_package,
    "three + @react-three/fiber + @react-three/drei",
  );
  assert.equal(
    runbookFixture.package.upstream_version,
    "three 0.184.0; @react-three/fiber 9.6.1; @react-three/drei local mirror",
  );
  assert.deepEqual(runbookFixture.package.source_mirrors, [
    "G:/WWW/inspirations/three.js",
    "G:/WWW/inspirations/react-three-fiber",
    "G:/WWW/inspirations/drei",
  ]);
  assert.deepEqual(runbookFixture.selected_surfaces, [
    "launch-scene-dashboard-workflow",
    "dx-style-compatible-launch-scene",
    "generated-starter-materialization",
    "receipt-hash-refresh",
    "three-scene-system-source-guard-runbook",
    "three-scene-system-preview-manifest-materializer",
  ]);
  assert.equal(
    runbookFixture.guard.id,
    "three-scene-system-lower-dx-check-helper-freshness",
  );
  assert.equal(
    runbookFixture.guard.guard_file,
    "core/src/ecosystem/project_check/three_scene_system_dx_check.rs",
  );
  assert.equal(
    runbookFixture.guard.command,
    "cargo test -q -p dx-www-compiler three_scene_system_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
  );
  assert.equal(runbookFixture.guard.execution_policy, "source-only");
  assert.equal(runbookFixture.guard.starts_server, false);
  assert.equal(runbookFixture.guard.runs_package_install, false);
  assert.equal(runbookFixture.guard.runs_full_build, false);
  assert.equal(runbookFixture.guard.node_modules_required, false);
  assert.ok(
    runbookFixture.guard.proves.includes(
      "three_scene_system_receipt_hash_refresh_stale",
    ),
  );
  assert.ok(
    runbookFixture.guard.proves.includes(
      "three_scene_system_hash_mismatch stays byte-derived",
    ),
  );
  assert.equal(
    runbookFixture.runbook.command.command,
    runbookFixture.guard.command,
  );
  assert.equal(runbookFixture.runbook.command.starts_server, false);
  assert.equal(runbookFixture.runbook.command.runs_package_install, false);
  assert.equal(runbookFixture.runbook.command.runs_full_build, false);
  assert.ok(runbookFixture.upstream_public_apis.includes("WebGLRenderer"));
  assert.ok(runbookFixture.upstream_public_apis.includes("createRoot"));
  assert.ok(runbookFixture.upstream_public_apis.includes("Bounds.fit"));
  assert.equal(
    runbookFixture.receipt.zed_visibility,
    "3d-scene-system:receipt-hash-refresh",
  );
  assert.equal(
    runbookFixture.receipt.source_guard_runbook_fixture,
    "docs/packages/3d-scene-system.source-guard-runbook.json",
  );
  assert.equal(runbookFixture.receipt.tracked_by_receipt_hash_helper, true);
  assert.deepEqual(runbookFixture.preview_manifest, {
    generated_file: "public/preview-.dx/build-cache/manifest.json",
    materializer: "tools/launch/materialize-www-template.ts",
    hash_backed_by: "examples/template/3d-scene-system-receipt-hashes.ts",
    hash_backed_files: ["tools/launch/materialize-www-template.ts"],
    selected_surface: "three-scene-system-preview-manifest-materializer",
    root_field: "sourceGuardRunbookFixtures",
    route_field: "routes[].sourceGuardRunbookFixtures",
    fixture: "docs/packages/3d-scene-system.source-guard-runbook.json",
    guard_id: "three-scene-system-lower-dx-check-helper-freshness",
    route: "/",
    runtime_proof: false,
    zed_visibility: "3d-scene-system:receipt-hash-refresh",
  });
  assert.equal(runbookFixture.honesty_label, "LOCK-BACKED SOURCE-OWNED");
  assert.equal(runbookFixture.runtime_proof, false);
  assert.ok(
    runbookFixture.runtime_limitations.some((limitation) =>
      limitation.includes("LOCK-BACKED SOURCE-OWNED"),
    ),
  );
});

test("3D Scene System package-scoped dx-check row survives generated starter materialization", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-three-package-lane-"));
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
  assert.match(launch, /data-dx-check-package-lane-row="3d\/launch-scene"/);
  assert.match(launch, /data-dx-check-package-lane-name="3D Scene System"/);
  assert.match(
    launch,
    /data-dx-check-package-lane-receipt-path="\.dx\/forge\/receipts\/3d-launch-scene-dashboard-workflow\.json"/,
  );
  assert.match(launch, /data-dx-style-surface="launch-scene"/);
  assert.match(launch, /data-dx-token-scope="3d\/launch-scene"/);

  const launchRoute = manifest.routes.find((route) => route.route === "/");
  assert.ok(launchRoute, "expected materialized /launch route metadata");
  assert.ok(launchRoute.forgePackages.includes("3d/launch-scene"));
  assert.deepEqual(
    manifest.sourceGuardRunbookFixtures.find(
      (fixture) => fixture.packageId === "3d/launch-scene",
    ),
    {
      packageId: "3d/launch-scene",
      officialPackageName: "3D Scene System",
      upstreamPackage: "three + @react-three/fiber + @react-three/drei",
      upstreamVersion:
        "three 0.184.0; @react-three/fiber 9.6.1; @react-three/drei local mirror",
      sourceMirror:
        "G:/WWW/inspirations/three.js; G:/WWW/inspirations/react-three-fiber; G:/WWW/inspirations/drei",
      route: "/",
      fixture: "docs/packages/3d-scene-system.source-guard-runbook.json",
      guardId: "three-scene-system-lower-dx-check-helper-freshness",
      schema: "dx.forge.package.source_guard_runbook_fixture",
      honestyLabel: "SOURCE-ONLY",
      runtimeProof: false,
      zedVisibility: "3d-scene-system:receipt-hash-refresh",
    },
    "generated preview manifest should expose the 3D Scene System runbook fixture",
  );
  assert.ok(
    launchRoute.sourceGuardRunbookFixtures.includes(
      "docs/packages/3d-scene-system.source-guard-runbook.json",
    ),
    "generated /launch metadata should link the 3D Scene System runbook fixture",
  );

  const checkPanel = manifest.editContract.editableSurfaces.find(
    (surface) => surface.id === "launch-runtime-dx-check-panel",
  );
  assert.ok(checkPanel, "expected dx-check panel edit surface");
  assert.equal(checkPanel.sourceFile, "pages/index.html");
  assert.ok(
    checkPanel.packageIds.includes("3d/launch-scene"),
    "generated dx-check panel package scope must include 3D Scene System",
  );

  for (const marker of [
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
});
