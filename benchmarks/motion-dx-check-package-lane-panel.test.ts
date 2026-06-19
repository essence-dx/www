const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const upstream = path.resolve(root, "..", "..", "WWW/inspirations/motion");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function readUpstream(relativePath) {
  return fs.readFileSync(path.join(upstream, relativePath), "utf8");
}

test("Motion & Animation package lane exposes helper freshness in the dx-check panel", () => {
  const upstreamPackage = JSON.parse(
    readUpstream("packages/framer-motion/package.json"),
  );
  const upstreamReact = readUpstream("packages/motion/src/react.ts");
  const upstreamIndex = readUpstream("packages/framer-motion/src/index.ts");
  const reader = read("core/src/ecosystem/dx_check_receipt.rs");
  const launchShell = read("examples/template/template-shell.tsx");
  const runtimeLaunchPage = read("tools/launch/runtime-template/pages/index.html");
  const packageDoc = read("docs/packages/animation-motion.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(upstreamPackage.name, "framer-motion");
  assert.equal(upstreamPackage.version, "12.38.0");
  assert.match(upstreamReact, /export \{ motion, m \} from "framer-motion"/);
  assert.match(upstreamIndex, /export \{ MotionConfig \}/);
  assert.match(upstreamIndex, /export \{ LazyMotion \}/);
  assert.match(upstreamIndex, /export \{ Reorder \}/);
  assert.match(upstreamIndex, /export \{ useScroll/);
  assert.match(upstreamIndex, /export \{ useReducedMotion \}/);

  for (const expected of [
    'const MOTION_ANIMATION_PACKAGE_ID: &str = "animation/motion";',
    'const MOTION_ANIMATION_OFFICIAL_NAME: &str = "Motion & Animation";',
    'const MOTION_ANIMATION_UPSTREAM_PACKAGE: &str = "motion";',
    'const MOTION_ANIMATION_UPSTREAM_VERSION: &str = "12.38.0";',
    'const MOTION_ANIMATION_SOURCE_MIRROR: &str = "G:/WWW/inspirations/motion";',
    'const MOTION_ANIMATION_PACKAGE_STATUS_PATH: &str = ".dx/forge/package-status.json";',
    "MOTION_ANIMATION_PACKAGE_RECEIPT_PATH",
    "motion_animation_package_present",
    "motion_animation_receipt_present",
    "motion_animation_receipt_stale",
    "motion_animation_missing_receipt",
    "motion_animation_blocked_surface",
    "motion_animation_unsupported_surface",
    "motion_animation_hash_manifest_present",
    "motion_animation_hash_mismatch",
    "motion_animation_receipt_hash_refresh_current",
    "motion_animation_receipt_hash_refresh_stale",
    "motion_animation_receipt_hash_refresh_missing",
    "dx_check_latest_panel_exposes_motion_animation_package_lane_hash_refresh_row",
    "stale_helper_motion_animation",
    "rows.extend(motion_animation_package_lane_row(root, package_status));",
    "fn motion_animation_package_lane_row(",
    "package_lane_visibility_entry(package_status, MOTION_ANIMATION_PACKAGE_ID)",
    "let receipt_hash_refresh = package_lane_hash_refresh(package);",
    "receipt_hash_refresh_counts(package)",
    'metric_value("motion_animation_hash_mismatch"), 0',
    "metrics: motion_animation_metric_rows(",
    "motion_animation_next_action(status, refresh_stale, refresh_missing)",
    "motion-animation:receipt-hash-refresh",
  ]) {
    assert.ok(
      reader.includes(expected),
      `${expected} missing from dx-check receipt panel reader`,
    );
  }

  assert.match(
    reader,
    /fn motion_animation_next_action\(\s*status: &str,\s*refresh_stale: u64,\s*refresh_missing: u64,\s*\) -> &'static str/,
  );

  for (const source of [launchShell, runtimeLaunchPage]) {
    assert.match(
      source,
      /data-dx-check-package-lane-template="animation\/motion"|package_id: "animation\/motion"/,
    );
    assert.match(
      source,
      /data-dx-check-package-lane-name="Motion & Animation"|official_package_name: "Motion & Animation"/,
    );
    assert.match(
      source,
      /data-dx-check-package-lane-hash-refresh-status="current"|hash_refresh_status: "current"/,
    );
    assert.match(source, /examples\/template\/motion-receipt-hashes\.ts/);
    assert.match(
      source,
      /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/motion-receipt-hashes\.ts --check --json/,
    );
    assert.match(source, /motion-animation:receipt-hash-refresh/);
    assert.match(
      source,
      /data-dx-check-package-lane-hash-refresh-tracked-files="4"|hash_refresh_tracked_files: 4/,
    );
    assert.match(
      source,
      /data-dx-check-package-lane-hash-refresh-current-metric="motion_animation_receipt_hash_refresh_current"|hash_refresh_metric_current: "motion_animation_receipt_hash_refresh_current"/,
    );
    assert.match(
      source,
      /data-dx-check-package-lane-hash-refresh-stale-metric="motion_animation_receipt_hash_refresh_stale"|hash_refresh_metric_stale: "motion_animation_receipt_hash_refresh_stale"/,
    );
    assert.match(
      source,
      /data-dx-check-package-lane-hash-refresh-missing-metric="motion_animation_receipt_hash_refresh_missing"|hash_refresh_metric_missing: "motion_animation_receipt_hash_refresh_missing"/,
    );
  }

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(
      source,
      /DX Studio\/check-panel Motion package row|static \/launch Motion & Animation package-lane fixture/,
    );
    assert.match(source, /motion_animation_hash_manifest_present/);
    assert.match(source, /motion_animation_hash_mismatch/);
    assert.match(source, /motion_animation_receipt_hash_refresh_current/);
    assert.match(source, /motion_animation_receipt_hash_refresh_stale/);
    assert.match(source, /motion_animation_receipt_hash_refresh_missing/);
    assert.match(source, /receiptHashRefresh|receipt_hash_refresh/);
    assert.match(source, /stale-helper-only Motion check-panel fixture/);
    assert.match(source, /receipt_hash_refresh\.stale_file_count/);
    assert.match(source, /without claiming live Motion browser animation proof/);
  }
});

test("Motion & Animation package-lane fixture survives generated starter materialization", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-motion-package-lane-"));
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
    assert.match(launch, /data-dx-check-package-lane-template="animation\/motion"/);
    assert.match(launch, /data-dx-check-package-lane-row="animation\/motion"/);
    assert.match(launch, /data-dx-check-package-lane-name="Motion & Animation"/);
    assert.match(launch, /data-dx-check-package-lane-status="missing"/);
    assert.match(launch, /data-dx-check-package-lane-receipt-status="missing-receipt"/);
    assert.match(launch, /data-dx-check-package-lane-upstream-package="motion"/);
    assert.match(
      launch,
      /data-dx-check-package-lane-source-mirror="G:\/WWW\/inspirations\/motion"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-receipt-path="examples\/template\/\.dx\/forge\/receipts\/2026-05-22-animation-motion-dashboard-workflow\.json"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-hash-refresh-helper="examples\/template\/motion-receipt-hashes\.ts"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-hash-refresh-json-command="node tools\/launch\/run-template-receipt-helper\.js examples\/template\/motion-receipt-hashes\.ts --check --json"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-hash-refresh-zed="motion-animation:receipt-hash-refresh"/,
    );
    assert.match(launch, /data-dx-check-package-lane-hash-refresh-tracked-files="4"/);
    assert.match(
      launch,
      /data-dx-check-package-lane-hash-refresh-current-metric="motion_animation_receipt_hash_refresh_current"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-hash-refresh-stale-metric="motion_animation_receipt_hash_refresh_stale"/,
    );
    assert.match(
      launch,
      /data-dx-check-package-lane-hash-refresh-missing-metric="motion_animation_receipt_hash_refresh_missing"/,
    );
    assert.match(launch, /data-dx-style-surface="motion-animation"/);
    assert.match(launch, /data-dx-token-scope="animation\/motion"/);
    assert.match(launch, /data-dx-package="animation\/motion"/);

    const rootRoute = manifest.routes.find((route) => route.route === "/");
    assert.ok(rootRoute, "expected generated root route metadata");
    assert.ok(
      rootRoute.forgePackages.includes("animation/motion"),
      "generated root route package scope must include Motion & Animation",
    );

    const launchRoute = manifest.routes.find((route) => route.route === "/");
    assert.ok(launchRoute, "expected generated / route metadata");
    assert.ok(
      launchRoute.forgePackages.includes("animation/motion"),
      "generated / route package scope must include Motion & Animation",
    );
    assert.ok(
      launchRoute.dataDxMarkers.includes(
        "data-dx-check-package-lane-hash-refresh-current-metric",
      ),
      "generated /launch manifest must expose Motion & Animation helper metrics",
    );
    assert.ok(
      manifest.sourceGuardRunbookFixtures.some(
        (fixture) =>
          fixture.packageId === "animation/motion" &&
          fixture.fixture === "docs/packages/motion-animation.source-guard-runbook.json" &&
          fixture.guardId === "motion-animation-generated-starter-materialization" &&
          fixture.honestyLabel === "SOURCE-ONLY" &&
          fixture.runtimeProof === false,
      ),
      "generated preview manifest must expose the Motion & Animation runbook fixture object",
    );
    assert.ok(
      launchRoute.sourceGuardRunbookFixtures.includes(
        "docs/packages/motion-animation.source-guard-runbook.json",
      ),
      "generated /launch route must link the Motion & Animation runbook fixture path",
    );

    const checkPanel = manifest.editContract.editableSurfaces.find(
      (surface) => surface.id === "launch-runtime-dx-check-panel",
    );
    assert.ok(checkPanel, "expected dx-check panel edit surface");
    assert.equal(checkPanel.sourceFile, "pages/index.html");
    assert.ok(
      checkPanel.packageIds.includes("animation/motion"),
      "generated dx-check panel package scope must include Motion & Animation",
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
      read("docs/packages/animation-motion.md"),
      read("DX.md"),
      read("TODO.md"),
      read("CHANGELOG.md"),
    ]) {
      assert.match(source, /generated-starter materialization guard for Motion & Animation/);
      assert.match(source, /without claiming live Motion browser animation proof/);
    }
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("Motion & Animation Studio source guard runbook exposes generated-starter command", () => {
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const frameworkDoc = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const packageDoc = read("docs/packages/animation-motion.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.match(
    studioManifest,
    /studio_source_guard_with_fixture\(\s*"motion-animation-generated-starter-materialization",\s*&\["\/"\],\s*"benchmarks\/motion-dx-check-package-lane-panel\.test\.ts",\s*"dx run --test \.\\\\benchmarks\\\\motion-dx-check-package-lane-panel\.test\.ts"[\s\S]*"docs\/packages\/motion-animation\.source-guard-runbook\.json",\s*\)/,
  );
  assert.match(
    studioManifest,
    /"motion-animation-generated-starter-materialization"[\s\S]*"Motion & Animation generated-starter materialization guard"[\s\S]*"data-dx-check-package-lane-row=\\\"animation\/motion\\\""[\s\S]*"data-dx-token-scope=\\\"animation\/motion\\\""[\s\S]*"motion-animation:receipt-hash-refresh"[\s\S]*"animation\/motion source-only Studio discovery"/,
  );
  assert.match(
    studioManifest,
    /"motion-animation-generated-starter-materialization"[\s\S]*"The generated starter preserves the Motion & Animation package-lane row, helper freshness markers, and package-scoped dx-check panel without live Motion browser animation proof\."/,
  );
  assert.match(
    studioManifest,
    /"dx run --test \.\\\\benchmarks\\\\motion-dx-check-package-lane-panel\.test\.ts"[\s\S]*"Validate the source-only Motion & Animation package-lane row, generated starter materialization, helper freshness markers, and dx-check panel package scope without live Motion browser animation proof\."/,
  );
  assert.match(
    studioManifest,
    /"\/" => guards\.extend\(\[[\s\S]*"motion-animation-generated-starter-materialization"/,
  );
  assert.match(
    studioManifest,
    /"source_guard_id": "motion-animation-generated-starter-materialization",[\s\S]*"package_id": "animation\/motion",[\s\S]*"fixture_path": "docs\/packages\/motion-animation\.source-guard-runbook\.json",[\s\S]*"schema": "dx\.forge\.package\.source_guard_runbook_fixture"/,
  );
  assert.match(
    studioManifest,
    /source_guard_contract_with_fixture\(\s*"motion-animation-generated-starter-materialization",[\s\S]*"benchmarks\/motion-dx-check-package-lane-panel\.test\.ts",\s*"docs\/packages\/motion-animation\.source-guard-runbook\.json",\s*\)/,
  );
  assert.match(
    studioManifest,
    /source_guard_command_with_fixture\(\s*"dx run --test \.\\\\benchmarks\\\\motion-dx-check-package-lane-panel\.test\.ts",[\s\S]*"docs\/packages\/motion-animation\.source-guard-runbook\.json",\s*\)/,
  );
  assert.match(frameworkDoc, /Lane 13 uses the official front-facing package name `Motion & Animation`/);
  assert.match(frameworkDoc, /motion-animation-generated-starter-materialization/);
  assert.match(frameworkDoc, /source_guard_index/);
  assert.match(frameworkDoc, /source_guard_runbook_index/);
  assert.match(frameworkDoc, /without claiming live Motion browser animation proof/);

  for (const source of [packageDoc, frameworkDoc, dx, todo, changelog]) {
    assert.match(source, /Motion & Animation Studio source-guard\/runbook entry/);
    assert.match(source, /motion-animation-generated-starter-materialization/);
    assert.match(
      source,
      /dx run --test \.\\benchmarks\\motion-dx-check-package-lane-panel\.test\.ts/,
    );
    assert.match(source, /without claiming live Motion browser animation proof/);
  }
});

test("Motion & Animation source-guard runbook fixture mirrors the Studio manifest", () => {
  const fixture = readJson("docs/packages/motion-animation.source-guard-runbook.json");
  const runbookFixturePath = "docs/packages/motion-animation.source-guard-runbook.json";
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const frameworkDoc = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const packageDoc = read("docs/packages/animation-motion.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(
    fixture.schema,
    "dx.forge.package.source_guard_runbook_fixture",
  );
  assert.equal(fixture.fixture_path, runbookFixturePath);
  assert.equal(fixture.route, "/");
  assert.equal(fixture.package.official_package_name, "Motion & Animation");
  assert.equal(fixture.package.package_id, "animation/motion");
  assert.equal(fixture.package.upstream_package, "motion");
  assert.equal(fixture.package.upstream_version, "12.38.0");
  assert.equal(fixture.package.source_mirror, "G:/WWW/inspirations/motion");
  assert.match(fixture.package.based_on, /Motion React public animation APIs/);

  assert.equal(
    fixture.guard.id,
    "motion-animation-generated-starter-materialization",
  );
  assert.deepEqual(fixture.guard.routes, ["/"]);
  assert.equal(
    fixture.guard.guard_file,
    "benchmarks/motion-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(
    fixture.guard.command,
    "dx run --test .\\benchmarks\\motion-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(fixture.guard.execution_policy, "source-only");
  assert.equal(fixture.guard.writes_files, false);
  assert.equal(fixture.guard.starts_server, false);
  assert.equal(fixture.guard.runs_package_install, false);
  assert.equal(fixture.guard.runs_full_build, false);
  assert.equal(fixture.guard.node_modules_required, false);
  assert.equal(fixture.guard.fixture_path, runbookFixturePath);

  for (const proof of [
    "Motion & Animation generated-starter materialization guard",
    'data-dx-check-package-lane-row="animation/motion"',
    'data-dx-token-scope="animation/motion"',
    "motion-animation:receipt-hash-refresh",
    "docs/packages/motion-animation.source-guard-runbook.json",
    "without claiming live Motion browser animation proof",
    "animation/motion source-only Studio discovery",
  ]) {
    assert.ok(fixture.guard.proves.includes(proof), `missing proof ${proof}`);
    assert.ok(
      studioManifest.includes(proof) ||
        studioManifest.includes(proof.replaceAll('"', '\\"')),
      `Studio manifest missing ${proof}`,
    );
  }

  assert.equal(fixture.runbook.index_field, "source_guard_runbook_index");
  assert.equal(fixture.runbook.default_action, "show-source-only-runbook");
  assert.deepEqual(fixture.runbook.fixture_paths, [
    {
      source_guard_id: "motion-animation-generated-starter-materialization",
      package_id: "animation/motion",
      fixture_path: runbookFixturePath,
      schema: "dx.forge.package.source_guard_runbook_fixture",
    },
  ]);
  assert.equal(
    fixture.runbook.contract.id,
    "motion-animation-generated-starter-materialization",
  );
  assert.equal(
    fixture.runbook.contract.evidence_field,
    "benchmarks/motion-dx-check-package-lane-panel.test.ts",
  );
  assert.equal(fixture.runbook.contract.fixture_path, runbookFixturePath);
  assert.equal(
    fixture.runbook.command.purpose,
    "Validate the source-only Motion & Animation package-lane row, generated starter materialization, helper freshness markers, and dx-check panel package scope without live Motion browser animation proof.",
  );
  assert.equal(fixture.runbook.command.fixture_path, runbookFixturePath);

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

  assert.equal(
    fixture.receipt.path,
    "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
  );
  assert.equal(
    fixture.receipt.hash_helper,
    "examples/template/motion-receipt-hashes.ts",
  );
  assert.equal(
    fixture.receipt.hash_helper_json_command,
    "node tools/launch/run-template-receipt-helper.js examples/template/motion-receipt-hashes.ts --check --json",
  );
  assert.equal(
    fixture.receipt.zed_visibility,
    "motion-animation:receipt-hash-refresh",
  );
  assert.equal(fixture.honesty_label, "SOURCE-ONLY");
  assert.equal(fixture.runtime_proof, false);
  assert.match(
    fixture.runtime_limitations.join("\n"),
    /live Motion browser animation proof/,
  );

  for (const source of [
    studioManifest,
    frameworkDoc,
    packageDoc,
    dx,
    todo,
    changelog,
  ]) {
    assert.match(source, /motion-animation\.source-guard-runbook\.json/);
    assert.match(source, /motion-animation-generated-starter-materialization/);
    assert.match(source, /without claiming live Motion browser animation proof/);
  }
});
