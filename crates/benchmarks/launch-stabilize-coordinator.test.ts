import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const http = require("node:http");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const {
  LAUNCH_COORDINATOR_SCHEMA,
  buildLaunchCoordinatorReport,
  runLaunchCoordinatorCli,
} = require("../tools/launch-stabilize/coordinator.cjs");

test("Lane 14 launch coordinator keeps the launch score honest", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: repoRoot,
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: [
      " M dx-www/src/cli/mod.rs",
      "?? .dx/template-app-browser-preview/pages/index.html",
      "?? examples/template/.dx/forge/cache/package/status.json",
      "",
    ].join("\n"),
  });
  const byId = new Map(report.checks.map((entry) => [entry.id, entry]));

  assert.equal(report.schema, LAUNCH_COORDINATOR_SCHEMA);
  assert.equal(report.lane, 14);
  assert.equal(report.laneName, "Final Launch Coordinator");
  assert.equal(report.featureImplementation, false);
  assert.equal(report.status, "blocked");
  assert.equal(report.launchScore, 40);
  assert.equal(report.auditFloorScore, 40);
  assert.equal(report.executionMode, "launch-stabilize-read-only");
  assert.equal(report.generatedAt, "2026-05-23T00:00:00.000Z");
  assert.equal(report.architecture.dxRuntimeAuthoritative, true);
  assert.equal(report.architecture.publicTurbopackDependency, false);
  assert.equal(report.architecture.nodeModulesRequired, false);
  assert.deepEqual(
    report.blockers.map((entry) => entry.id),
    [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "generated-preview-noise",
      "dirty-worktree-lane-risk",
      "giant-cli-mod-risk",
    ],
  );

  assert.equal(byId.get("minimal-source-layout").status, "passed");
  assert.equal(byId.get("required-template-route-sources").status, "passed");
  assert.equal(byId.get("required-template-route-sources").ownerLane, 10);
  assert.equal(byId.get("no-template-node-modules").status, "passed");
  assert.equal(byId.get("no-template-fake-artifacts").status, "passed");
  assert.deepEqual(byId.get("no-template-fake-artifacts").details.artifacts, []);
  assert.equal(byId.get("merge-conflict-markers").status, "passed");
  assert.equal(byId.get("live-route-probe").status, "skipped");
  assert.equal(byId.get("live-route-probe").blocking, false);
  assert.equal(byId.get("dx-style-compile-proof").status, "blocked");
  assert.equal(byId.get("dx-build-tiny-app-proof").ownerLane, 2);
  assert.equal(byId.get("server-data-json-proof").ownerLane, 3);
  assert.equal(byId.get("generated-preview-noise").ownerLane, 7);
  assert.equal(byId.get("dirty-worktree-lane-risk").ownerLane, 14);
  assert.equal(byId.get("dirty-worktree-owner-coverage").status, "passed");
  assert.equal(byId.get("dirty-worktree-owner-coverage").details.unclassifiedCount, 0);
  assert.equal(byId.get("dirty-worktree-owner-coverage").ownerLane, 14);
  assert.equal(byId.get("public-schema-version-noise").status, "passed");
  assert.equal(byId.get("public-schema-version-noise").ownerLane, 14);
  assert.equal(byId.get("launch-status-overclaims").status, "passed");
  assert.equal(byId.get("launch-status-overclaims").ownerLane, 14);
  assert.equal(byId.get("giant-cli-mod-risk").ownerLane, 6);
  assert.doesNotMatch(JSON.stringify(report), /\bdx(?:\.[A-Za-z][A-Za-z0-9_-]*)+\.v1\b/);
});

test("Lane 14 launch coordinator drops route-source blockers once real route files are restored", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: repoRoot,
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: "",
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });

  assert.equal(report.status, "passing");
  assert.equal(report.launchScore, 100);
  assert.deepEqual(report.blockers.map((entry) => entry.id), []);
});

test("Lane 14 launch coordinator blocks missing template route source files", () => {
  const fixtureRoot = makeCoordinatorFixtureRoot({
  missingRouteSources: [
    "examples/template/app/page.tsx",
  ],
  sourceOnlyRouteShadows: [
    "examples/template/app/page.tsx.source-only",
  ],
  });
  const report = buildLaunchCoordinatorReport({
    cwd: fixtureRoot,
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: "",
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });
  const routeSources = report.checks.find(
    (entry) => entry.id === "required-template-route-sources",
  );

  assert.equal(routeSources.status, "blocked");
  assert.equal(routeSources.ownerLane, 10);
  assert.deepEqual(routeSources.details.missing, [
    "examples/template/app/page.tsx",
  ]);
  assert.deepEqual(routeSources.details.sourceOnlyShadows, [
    "examples/template/app/page.tsx.source-only",
  ]);
  assert.equal(report.ownerLaneSummary[0].ownerLane, 10);
  assert.deepEqual(report.ownerLaneSummary[0].checkIds, [
    "required-template-route-sources",
  ]);
});

test("Lane 14 launch coordinator separates generated preview harnesses from source fake artifacts", () => {
  const fixtureRoot = makeCoordinatorFixtureRoot();
  fs.mkdirSync(path.join(fixtureRoot, ".dx/template-app-browser-preview"), {
    recursive: true,
  });
  fs.writeFileSync(
    path.join(fixtureRoot, ".dx/template-app-browser-preview/server.cjs"),
    "module.exports = {}\n",
  );
  fs.writeFileSync(
    path.join(fixtureRoot, "examples/template/source-fake.cjs"),
    "module.exports = {}\n",
  );

  const report = buildLaunchCoordinatorReport({
    cwd: fixtureRoot,
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: "",
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });
  const fakeArtifacts = report.checks.find(
    (entry) => entry.id === "no-template-fake-artifacts",
  );

  assert.equal(fakeArtifacts.status, "blocked");
  assert.deepEqual(fakeArtifacts.details.artifacts, [
    "examples/template/source-fake.cjs",
  ]);
});

test("Lane 14 launch coordinator classifies dx-style tool and scratch-noise dirty files", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: makeCoordinatorFixtureRoot(),
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: [
      " D -",
      "?? tools/dx-style/",
      "?? tools/style/",
      "",
    ].join("\n"),
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });
  const ownerCoverage = report.checks.find(
    (entry) => entry.id === "dirty-worktree-owner-coverage",
  );
  const dirtyRisk = report.checks.find((entry) => entry.id === "dirty-worktree-lane-risk");
  const ownerHints = new Map(
    dirtyRisk.details.ownerHints.map((entry) => [entry.bucket, entry]),
  );

  assert.equal(ownerCoverage.status, "passed");
  assert.deepEqual(ownerCoverage.details.unclassifiedEntries, []);
  assert.deepEqual(ownerHints.get("generated-preview-noise"), {
    ownerLane: 7,
    bucket: "generated-preview-noise",
    count: 1,
    samplePaths: ["-"],
  });
  assert.deepEqual(ownerHints.get("dx-style-runtime-integration"), {
    ownerLane: 7,
    bucket: "dx-style-runtime-integration",
    count: 2,
    samplePaths: ["tools/dx-style/", "tools/style/"],
  });
});

test("Lane 14 launch coordinator blocks supplied git diff whitespace failures", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: makeCoordinatorFixtureRoot(),
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: "",
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
    gitDiffCheckReport: {
      command: "git diff --check",
      exitCode: 2,
      stdout: "dx-www/src/dev/axum_server.rs:42: trailing whitespace\n",
      stderr: "",
    },
  });
  const diffCheck = report.checks.find((entry) => entry.id === "git-diff-whitespace");

  assert.equal(diffCheck.status, "blocked");
  assert.equal(diffCheck.blocking, true);
  assert.equal(diffCheck.ownerLane, 14);
  assert.equal(diffCheck.evidence, "git diff --check failed with exit code 2");
  assert.deepEqual(diffCheck.details.outputLines, [
    "dx-www/src/dev/axum_server.rs:42: trailing whitespace",
  ]);
  assert.ok(report.blockers.some((entry) => entry.id === "git-diff-whitespace"));
});

test("Lane 14 strict final readiness requires git diff proof", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: makeCoordinatorFixtureRoot(),
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: "",
    requireGitDiffCheck: true,
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
    routeProbeReport: {
      schema: "dx.www.launchStabilize.routeProbe",
      format: 1,
      status: "passed",
      passedCount: 5,
      adapterBoundaryCount: 0,
      failedCount: 0,
      routes: [
        { path: "/", status: "passed", actualStatus: 200 },
        { path: "/", status: "passed", actualStatus: 200 },
        { path: "/dashboard", status: "passed", actualStatus: 200 },
        { path: "/_dx/hot-reload/version", status: "passed", actualStatus: 200 },
        { path: "/api/trpc/health", status: "passed", actualStatus: 200 },
      ],
    },
  });
  const diffCheck = report.checks.find((entry) => entry.id === "git-diff-whitespace");

  assert.equal(report.status, "blocked");
  assert.equal(report.hundredPercentReady, false);
  assert.equal(diffCheck.status, "blocked");
  assert.equal(diffCheck.ownerLane, 14);
  assert.equal(diffCheck.evidence, "strict final launch requires git diff --check proof");
  assert.deepEqual(diffCheck.details, {
    supplied: false,
    requiredCommand: "git diff --check",
  });
  assert.deepEqual(report.blockers.map((entry) => entry.id), ["git-diff-whitespace"]);
});

test("Lane 14 launch coordinator blocks final signoff on dirty worktree risk", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: repoRoot,
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: [
      " M dx-www/src/cli/mod.rs",
      " M dx-www/src/build/source_engine/route_output.rs",
      "?? tools/launch-stabilize/coordinator.cjs",
      "?? benchmarks/launch-stabilize-coordinator.test.ts",
      " M examples/template/.dx/forge/cache/package/status.json",
      "?? .dx/template-app-browser-preview/pages/index.html",
      "",
    ].join("\n"),
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });
  const dirtyRisk = report.checks.find((entry) => entry.id === "dirty-worktree-lane-risk");

  assert.equal(dirtyRisk.status, "blocked");
  assert.equal(dirtyRisk.blocking, true);
  assert.equal(dirtyRisk.details.totalCount, 6);
  assert.equal(dirtyRisk.details.generatedNoiseCount, 2);
  assert.equal(dirtyRisk.details.lane14OutputCount, 2);
  assert.equal(dirtyRisk.details.sourceOrOtherCount, 2);
  assert.deepEqual(dirtyRisk.details.sourceOrOtherEntries, [
    { status: "M", path: "dx-www/src/cli/mod.rs" },
    { status: "M", path: "dx-www/src/build/source_engine/route_output.rs" },
  ]);
  assert.deepEqual(dirtyRisk.details.ownerHints, [
    {
      ownerLane: 7,
      bucket: "generated-preview-noise",
      count: 2,
      samplePaths: [
        "examples/template/.dx/forge/cache/package/status.json",
        ".dx/template-app-browser-preview/pages/index.html",
      ],
    },
    {
      ownerLane: 14,
      bucket: "final-launch-coordinator",
      count: 2,
      samplePaths: [
        "tools/launch-stabilize/coordinator.cjs",
        "benchmarks/launch-stabilize-coordinator.test.ts",
      ],
    },
    {
      ownerLane: 13,
      bucket: "cli-architecture-split",
      count: 1,
      samplePaths: ["dx-www/src/cli/mod.rs"],
    },
    {
      ownerLane: 2,
      bucket: "dx-build-end-to-end",
      count: 1,
      samplePaths: ["dx-www/src/build/source_engine/route_output.rs"],
    },
  ]);
  assert.match(dirtyRisk.evidence, /6 git status entries/);
  assert.deepEqual(report.blockers.map((entry) => entry.id), [
    "generated-preview-noise",
    "dirty-worktree-lane-risk",
  ]);
});

test("Lane 14 launch coordinator blocks unassigned dirty owner coverage", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: makeCoordinatorFixtureRoot(),
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: [
      " M dx-www/src/cli/mod.rs",
      "?? experiments/unknown-worker-output.ts",
      "",
    ].join("\n"),
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });
  const ownerCoverage = report.checks.find(
    (entry) => entry.id === "dirty-worktree-owner-coverage",
  );

  assert.equal(ownerCoverage.status, "blocked");
  assert.equal(ownerCoverage.ownerLane, 14);
  assert.equal(ownerCoverage.blocking, true);
  assert.match(ownerCoverage.evidence, /1 dirty status entry has no lane owner/);
  assert.deepEqual(ownerCoverage.details.unclassifiedEntries, [
    { status: "??", path: "experiments/unknown-worker-output.ts" },
  ]);
  assert.ok(report.blockers.some((entry) => entry.id === "dirty-worktree-owner-coverage"));
});

test("Lane 14 launch coordinator emits a prioritized lane cleanup plan", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: makeCoordinatorFixtureRoot(),
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: [
      " M .dx/template-app-browser-preview/pages/index.html",
      " M .dx/template-app-browser-preview/pages/index.html",
      " M .dx/template-app-browser-preview/styles/globals.css",
      " M examples/template/app/page.tsx",
      " M examples/template/components/template-app/dashboard-page.tsx",
      " M dx-www/src/cli/mod.rs",
      " M dx-www/src/build/source_engine/route_output.rs",
      "",
    ].join("\n"),
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });

  assert.deepEqual(report.laneCleanupPlan.slice(0, 4).map((entry) => ({
    ownerLane: entry.ownerLane,
    dirtyCount: entry.dirtyCount,
    dirtyBuckets: entry.dirtyBuckets.map((bucket) => bucket.bucket),
    blockingCheckIds: entry.blockingCheckIds,
  })), [
    {
      ownerLane: 7,
      dirtyCount: 3,
      dirtyBuckets: ["generated-preview-noise"],
      blockingCheckIds: ["generated-preview-noise"],
    },
    {
      ownerLane: 10,
      dirtyCount: 2,
      dirtyBuckets: ["default-template-product-quality"],
      blockingCheckIds: [],
    },
    {
      ownerLane: 2,
      dirtyCount: 1,
      dirtyBuckets: ["dx-build-end-to-end"],
      blockingCheckIds: [],
    },
    {
      ownerLane: 13,
      dirtyCount: 1,
      dirtyBuckets: ["cli-architecture-split"],
      blockingCheckIds: [],
    },
  ]);
  const coordinatorPlan = report.laneCleanupPlan.find((entry) => entry.ownerLane === 14);

  assert.deepEqual(coordinatorPlan.blockingCheckIds, ["dirty-worktree-lane-risk"]);
  assert.deepEqual(coordinatorPlan.nonBlockingCheckIds, ["live-route-probe"]);
  assert.equal(coordinatorPlan.dirtyCount, 0);
});

test("Lane 14 launch coordinator gives actionable dirty owner hints", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: makeCoordinatorFixtureRoot(),
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: [
      " M related-crates/style/src/core/engine/parity.rs",
      "?? benchmarks/route-handler-readiness-interpreters.test.ts",
      " M core/src/delivery/route_handler_compat.rs",
      "?? dx-www/tests/source_resolver_compat.rs",
      " M dx-www/src/diagnostics.rs",
      " M examples/template/.dx/forge/package-status.json",
      " M docs/packages/authentication.md",
      " M examples/template/components/template-app/dashboard-page.tsx",
      "",
    ].join("\n"),
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });
  const dirtyRisk = report.checks.find((entry) => entry.id === "dirty-worktree-lane-risk");
  const ownerHints = new Map(
    dirtyRisk.details.ownerHints.map((entry) => [entry.bucket, entry]),
  );

  assert.deepEqual(ownerHints.get("dx-style-runtime-integration"), {
    ownerLane: 7,
    bucket: "dx-style-runtime-integration",
    count: 1,
    samplePaths: ["related-crates/style/src/core/engine/parity.rs"],
  });
  assert.deepEqual(ownerHints.get("route-handler-execution"), {
    ownerLane: 3,
    bucket: "route-handler-execution",
    count: 2,
    samplePaths: [
      "benchmarks/route-handler-readiness-interpreters.test.ts",
      "core/src/delivery/route_handler_compat.rs",
    ],
  });
  assert.deepEqual(ownerHints.get("resolver-module-linker"), {
    ownerLane: 6,
    bucket: "resolver-module-linker",
    count: 1,
    samplePaths: ["dx-www/tests/source_resolver_compat.rs"],
  });
  assert.deepEqual(ownerHints.get("diagnostics-error-ux"), {
    ownerLane: 9,
    bucket: "diagnostics-error-ux",
    count: 1,
    samplePaths: ["dx-www/src/diagnostics.rs"],
  });
  assert.deepEqual(ownerHints.get("forge-package-template-truth"), {
    ownerLane: 11,
    bucket: "forge-package-template-truth",
    count: 2,
    samplePaths: [
      "examples/template/.dx/forge/package-status.json",
      "docs/packages/authentication.md",
    ],
  });
  assert.deepEqual(ownerHints.get("default-template-product-quality"), {
    ownerLane: 10,
    bucket: "default-template-product-quality",
    count: 1,
    samplePaths: ["examples/template/components/template-app/dashboard-page.tsx"],
  });
  assert.equal(ownerHints.has("unclassified-source-or-doc"), false);
});

test("Lane 14 launch coordinator classifies current launch handoff dirty files", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: makeCoordinatorFixtureRoot(),
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: [
      " M Cargo.toml",
      " M benchmarks/automations-launch-visible-proof.test.ts",
      " M benchmarks/forge-golden-path-launch-proof.test.ts",
      " M benchmarks/instantdb-cursors-slice.test.ts",
      " M benchmarks/supabase-launch-visible-proof.test.ts",
      " M benchmarks/wasm-bindgen-slice.test.ts",
      " M benchmarks/zod-launch-visible-proof.test.ts",
      " M integrations/n8n-nodes-base/dx-node-source-.dx/build-cache/manifest.json",
      " M benchmarks/launch-live-runtime-guard.test.ts",
      " M benchmarks/launch-runtime-materializer.test.ts",
      " M benchmarks/launch-scene-runtime.test.ts",
      "?? tools/launch/run-template-receipt-helper.js",
      " M docs/dx-www-developer-contract.md",
      "?? benchmarks/dx-www-parser-launch-extensions.test.ts",
      "?? benchmarks/public-v1-error-wording.test.ts",
      " M dx-www/src/parser/mod.rs",
      " M dx-www/src/production/mod.rs",
      "?? benchmarks/app-api-route-handler-extensions.test.ts",
      "?? dx-www/tests/route_handler_instant.rs",
      " M examples/dashboard/src/components/AutomationWorkflowPanel.tsx",
      " M examples/dashboard/src/lib/wasmBindgenDashboard.ts",
      " M ../forge/README.md",
      "?? benchmarks/flow-forge-schema-format.test.ts",
      "?? benchmarks/flow-forge-workspace-boundary.test.ts",
      " M ../forge/tests/integration.rs",
      "?? benchmarks/launch-readme-overclaim-guard.test.ts",
      "?? dx-www/tests/dx_build_tiny_app.rs",
      "",
    ].join("\n"),
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });
  const ownerCoverage = report.checks.find(
    (entry) => entry.id === "dirty-worktree-owner-coverage",
  );
  const dirtyRisk = report.checks.find((entry) => entry.id === "dirty-worktree-lane-risk");
  const ownerHints = new Map(
    dirtyRisk.details.ownerHints.map((entry) => [entry.bucket, entry]),
  );

  assert.equal(ownerCoverage.status, "passed");
  assert.deepEqual(ownerCoverage.details.unclassifiedEntries, []);
  assert.deepEqual(ownerHints.get("compile-test-gate"), {
    ownerLane: 1,
    bucket: "compile-test-gate",
    count: 1,
    samplePaths: ["Cargo.toml"],
  });
  assert.deepEqual(ownerHints.get("forge-package-template-truth"), {
    ownerLane: 11,
    bucket: "forge-package-template-truth",
    count: 9,
    samplePaths: [
      "benchmarks/automations-launch-visible-proof.test.ts",
      "benchmarks/forge-golden-path-launch-proof.test.ts",
      "benchmarks/instantdb-cursors-slice.test.ts",
      "benchmarks/supabase-launch-visible-proof.test.ts",
      "benchmarks/wasm-bindgen-slice.test.ts",
    ],
  });
  assert.deepEqual(ownerHints.get("default-template-product-quality"), {
    ownerLane: 10,
    bucket: "default-template-product-quality",
    count: 4,
    samplePaths: [
      "benchmarks/launch-live-runtime-guard.test.ts",
      "benchmarks/launch-runtime-materializer.test.ts",
      "benchmarks/launch-scene-runtime.test.ts",
      "tools/launch/run-template-receipt-helper.js",
    ],
  });
  assert.deepEqual(ownerHints.get("public-framework-contract"), {
    ownerLane: 14,
    bucket: "public-framework-contract",
    count: 5,
    samplePaths: [
      "docs/dx-www-developer-contract.md",
      "benchmarks/dx-www-parser-launch-extensions.test.ts",
      "benchmarks/public-v1-error-wording.test.ts",
      "dx-www/src/parser/mod.rs",
      "dx-www/src/production/mod.rs",
    ],
  });
  assert.deepEqual(ownerHints.get("route-handler-execution"), {
    ownerLane: 3,
    bucket: "route-handler-execution",
    count: 2,
    samplePaths: [
      "benchmarks/app-api-route-handler-extensions.test.ts",
      "dx-www/tests/route_handler_instant.rs",
    ],
  });
  assert.deepEqual(ownerHints.get("flow-forge-integration"), {
    ownerLane: 11,
    bucket: "flow-forge-integration",
    count: 4,
    samplePaths: [
      "../forge/README.md",
      "benchmarks/flow-forge-schema-format.test.ts",
      "benchmarks/flow-forge-workspace-boundary.test.ts",
      "../forge/tests/integration.rs",
    ],
  });
  assert.deepEqual(ownerHints.get("launch-status-handoff"), {
    ownerLane: 14,
    bucket: "launch-status-handoff",
    count: 1,
    samplePaths: ["benchmarks/launch-readme-overclaim-guard.test.ts"],
  });
  assert.deepEqual(ownerHints.get("dx-build-end-to-end"), {
    ownerLane: 2,
    bucket: "dx-build-end-to-end",
    count: 1,
    samplePaths: ["dx-www/tests/dx_build_tiny_app.rs"],
  });
});

test("Lane 14 launch coordinator classifies fresh launch gate and route proof outputs", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: makeCoordinatorFixtureRoot(),
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: [
      " M benchmarks/trpc-launch-runtime-proof.test.ts",
      "?? .dx/postcss-compat-target/",
      "?? benchmarks/launch-readiness-gate.test.ts",
      "?? tools/launch/launch-readiness-gate.js",
      "?? tools/launch/readiness-gate/",
      "",
    ].join("\n"),
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });
  const dirtyRisk = report.checks.find((entry) => entry.id === "dirty-worktree-lane-risk");
  const coverage = report.checks.find((entry) => entry.id === "dirty-worktree-owner-coverage");
  const ownerHints = new Map(
    dirtyRisk.details.ownerHints.map((entry) => [entry.bucket, entry]),
  );

  assert.deepEqual(ownerHints.get("route-handler-execution"), {
    ownerLane: 3,
    bucket: "route-handler-execution",
    count: 1,
    samplePaths: ["benchmarks/trpc-launch-runtime-proof.test.ts"],
  });
  assert.deepEqual(ownerHints.get("dx-style-runtime-integration"), {
    ownerLane: 7,
    bucket: "dx-style-runtime-integration",
    count: 1,
    samplePaths: [".dx/postcss-compat-target/"],
  });
  assert.deepEqual(ownerHints.get("compile-test-gate"), {
    ownerLane: 1,
    bucket: "compile-test-gate",
    count: 3,
    samplePaths: [
      "benchmarks/launch-readiness-gate.test.ts",
      "tools/launch/launch-readiness-gate.js",
      "tools/launch/readiness-gate/",
    ],
  });
  assert.equal(coverage.status, "passed");
});

test("Lane 14 launch coordinator classifies conversion proof static parity outputs", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: makeCoordinatorFixtureRoot(),
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: [
      " M examples/conversion-proof/.dx/vercel-landing/launch-runtime.js",
      " M examples/conversion-proof/.dx/vercel-landing/preview-.dx/build-cache/manifest.json",
      " M examples/conversion-proof/pages/backend.html",
      " M examples/conversion-proof/pages/index.html",
      " M examples/conversion-proof/pages/ui.html",
      " M examples/conversion-proof/public/launch-runtime.js",
      " M examples/conversion-proof/public/preview-.dx/build-cache/manifest.json",
      " M benchmarks/dx-www-conversion-proof.test.ts",
      " M examples/conversion-proof/.dx/forge/source-.dx/build-cache/manifest.json",
      " M examples/conversion-proof/forge/route-discovery/conversion-routes.json",
      "",
    ].join("\n"),
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });
  const coverage = report.checks.find((entry) => entry.id === "dirty-worktree-owner-coverage");
  const dirtyRisk = report.checks.find((entry) => entry.id === "dirty-worktree-lane-risk");
  const ownerHints = new Map(
    dirtyRisk.details.ownerHints.map((entry) => [entry.bucket, entry]),
  );

  assert.equal(coverage.status, "passed");
  assert.deepEqual(ownerHints.get("conversion-proof-static-parity"), {
    ownerLane: 10,
    bucket: "conversion-proof-static-parity",
    count: 10,
    samplePaths: [
      "examples/conversion-proof/.dx/vercel-landing/launch-runtime.js",
      "examples/conversion-proof/.dx/vercel-landing/preview-.dx/build-cache/manifest.json",
      "examples/conversion-proof/pages/backend.html",
      "examples/conversion-proof/pages/index.html",
      "examples/conversion-proof/pages/ui.html",
    ],
  });
});

test("Lane 14 launch coordinator classifies launch docs and foundation dirty files", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: makeCoordinatorFixtureRoot(),
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: [
      " M README.md",
      " M DX.md",
      " M TODO.md",
      " M CHANGELOG.md",
      " M docs/next-rust-merge-checkpoint.md",
      " M vendor/next-rust/README.md",
      " M dx-www/src/next_rust.rs",
      " M Cargo.lock",
      " M dx-www/Cargo.toml",
      " M dx-www/src/lib.rs",
      " M dx-www/src/main.rs",
      " M core/src/delivery/mod.rs",
      "",
    ].join("\n"),
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });
  const dirtyRisk = report.checks.find((entry) => entry.id === "dirty-worktree-lane-risk");
  const ownerHints = new Map(
    dirtyRisk.details.ownerHints.map((entry) => [entry.bucket, entry]),
  );

  assert.deepEqual(ownerHints.get("launch-status-handoff"), {
    ownerLane: 14,
    bucket: "launch-status-handoff",
    count: 4,
    samplePaths: ["README.md", "DX.md", "TODO.md", "CHANGELOG.md"],
  });
  assert.deepEqual(ownerHints.get("next-rust-reference-boundary"), {
    ownerLane: 14,
    bucket: "next-rust-reference-boundary",
    count: 3,
    samplePaths: [
      "docs/next-rust-merge-checkpoint.md",
      "vendor/next-rust/README.md",
      "dx-www/src/next_rust.rs",
    ],
  });
  assert.deepEqual(ownerHints.get("compile-test-gate"), {
    ownerLane: 1,
    bucket: "compile-test-gate",
    count: 5,
    samplePaths: [
      "Cargo.lock",
      "dx-www/Cargo.toml",
      "dx-www/src/lib.rs",
      "dx-www/src/main.rs",
      "core/src/delivery/mod.rs",
    ],
  });
  assert.equal(ownerHints.has("unclassified-source-or-doc"), false);
});

test("Lane 14 launch coordinator removes the active Next/Turbopack build-brain target", () => {
  const coordinatorSource = fs.readFileSync(
    path.join(repoRoot, "tools/launch-stabilize/coordinator.cjs"),
    "utf8",
  );
  const report = buildLaunchCoordinatorReport({
    cwd: makeCoordinatorFixtureRoot(),
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: [
      " M docs/next-rust-merge-checkpoint.md",
      " M vendor/next-rust/README.md",
      " M dx-www/src/next_rust.rs",
      " M tools/build-graph/dx-graph.ts",
      " M tools/build-graph/index.js",
      " M tools/build-graph/scanner.ts",
      "",
    ].join("\n"),
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });
  const dirtyRisk = report.checks.find((entry) => entry.id === "dirty-worktree-lane-risk");
  const ownerHints = new Map(
    dirtyRisk.details.ownerHints.map((entry) => [entry.bucket, entry]),
  );
  const removedBucket = "next-turbopack-" + "build-brain-adapter";
  const removedFunction = "isNextTurbopack" + "BuildBrainPath";

  assert.doesNotMatch(coordinatorSource, new RegExp(removedBucket));
  assert.doesNotMatch(coordinatorSource, new RegExp(removedFunction));
  assert.equal(ownerHints.has(removedBucket), false);
  assert.deepEqual(ownerHints.get("next-rust-reference-boundary"), {
    ownerLane: 14,
    bucket: "next-rust-reference-boundary",
    count: 3,
    samplePaths: [
      "docs/next-rust-merge-checkpoint.md",
      "vendor/next-rust/README.md",
      "dx-www/src/next_rust.rs",
    ],
  });
  assert.deepEqual(ownerHints.get("dx-build-end-to-end"), {
    ownerLane: 2,
    bucket: "dx-build-end-to-end",
    count: 3,
    samplePaths: [
      "tools/build-graph/dx-graph.ts",
      "tools/build-graph/index.js",
      "tools/build-graph/scanner.ts",
    ],
  });
});

test("Lane 14 launch coordinator classifies public contract and runtime support dirty files", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: makeCoordinatorFixtureRoot(),
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: [
      " M docs/DX_WWW_FRAMEWORK_STRUCTURE.md",
      " M benchmarks/public-framework-contract.test.ts",
      " M benchmarks/public-framework-tools.test.ts",
      " M core/src/delivery/server_contract.rs",
      " M core/src/ecosystem/dx_check_receipt.rs",
      " M core/src/ecosystem/dx_style_receipts.rs",
      " M dx-www/src/project.rs",
      " M dx-www/src/router/matcher.rs",
      " M dx-www/src/parser/style.rs",
      " M dx-www/src/data/mod.rs",
      " M dx-www/tests/source_build_engine.rs",
      "",
    ].join("\n"),
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });
  const dirtyRisk = report.checks.find((entry) => entry.id === "dirty-worktree-lane-risk");
  const ownerHints = new Map(
    dirtyRisk.details.ownerHints.map((entry) => [entry.bucket, entry]),
  );

  assert.deepEqual(ownerHints.get("public-framework-contract"), {
    ownerLane: 14,
    bucket: "public-framework-contract",
    count: 7,
    samplePaths: [
      "docs/DX_WWW_FRAMEWORK_STRUCTURE.md",
      "benchmarks/public-framework-contract.test.ts",
      "benchmarks/public-framework-tools.test.ts",
      "core/src/delivery/server_contract.rs",
      "core/src/ecosystem/dx_check_receipt.rs",
    ],
  });
  assert.deepEqual(ownerHints.get("app-router-runtime-parity"), {
    ownerLane: 4,
    bucket: "app-router-runtime-parity",
    count: 1,
    samplePaths: ["dx-www/src/router/matcher.rs"],
  });
  assert.deepEqual(ownerHints.get("dx-style-runtime-integration"), {
    ownerLane: 7,
    bucket: "dx-style-runtime-integration",
    count: 1,
    samplePaths: ["dx-www/src/parser/style.rs"],
  });
  assert.deepEqual(ownerHints.get("server-data-safe-loaders"), {
    ownerLane: 5,
    bucket: "server-data-safe-loaders",
    count: 1,
    samplePaths: ["dx-www/src/data/mod.rs"],
  });
  assert.deepEqual(ownerHints.get("dx-build-end-to-end"), {
    ownerLane: 2,
    bucket: "dx-build-end-to-end",
    count: 1,
    samplePaths: ["dx-www/tests/source_build_engine.rs"],
  });
  assert.equal(ownerHints.has("unclassified-source-or-doc"), false);
});

test("Lane 14 launch coordinator classifies router normalization and dev feedback proofs", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: makeCoordinatorFixtureRoot(),
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: [
      " M dx-www/src/api/mod.rs",
      "?? benchmarks/dx-api-router-http-method-detection.test.ts",
      "?? benchmarks/dx-dev-feedback-contract.test.ts",
      "?? benchmarks/dx-router-request-normalization.test.ts",
      "",
    ].join("\n"),
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });
  const dirtyRisk = report.checks.find((entry) => entry.id === "dirty-worktree-lane-risk");
  const ownerCoverage = report.checks.find(
    (entry) => entry.id === "dirty-worktree-owner-coverage",
  );
  const ownerHints = new Map(
    dirtyRisk.details.ownerHints.map((entry) => [entry.bucket, entry]),
  );

  assert.equal(ownerCoverage.status, "passed");
  assert.deepEqual(ownerCoverage.details.unclassifiedEntries, []);
  assert.deepEqual(ownerHints.get("route-handler-execution"), {
    ownerLane: 3,
    bucket: "route-handler-execution",
    count: 2,
    samplePaths: [
      "dx-www/src/api/mod.rs",
      "benchmarks/dx-api-router-http-method-detection.test.ts",
    ],
  });
  assert.deepEqual(ownerHints.get("app-router-runtime-parity"), {
    ownerLane: 4,
    bucket: "app-router-runtime-parity",
    count: 1,
    samplePaths: ["benchmarks/dx-router-request-normalization.test.ts"],
  });
  assert.deepEqual(ownerHints.get("dev-server-hmr"), {
    ownerLane: 8,
    bucket: "dev-server-hmr",
    count: 1,
    samplePaths: ["benchmarks/dx-dev-feedback-contract.test.ts"],
  });
  assert.equal(ownerHints.has("unclassified-source-or-doc"), false);
});

test("Lane 14 launch coordinator classifies package proof and tooling dirty files", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: makeCoordinatorFixtureRoot(),
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: [
      " M benchmarks/automations-dx-check-package-lane-panel.test.ts",
      " M benchmarks/automations-receipt-hash-refresh.test.ts",
      " M benchmarks/better-auth-dashboard-workflow.test.ts",
      " M benchmarks/better-auth-live-runtime.test.ts",
      " M benchmarks/instantdb-dx-check-visibility.test.ts",
      " M benchmarks/stripe-payment-launch-proof.test.ts",
      " M benchmarks/three-scene-package-doc.test.ts",
      " M benchmarks/vercel-ai-launch-visible-proof.test.ts",
      " M benchmarks/zod-dashboard-settings-workflow.test.ts",
      " M benchmarks/drizzle-launch-proof.test.ts",
      "?? benchmarks/forge-package-row-maturity-classification.test.ts",
      " M benchmarks/fumadocs-dashboard-workflow.test.ts",
      "?? benchmarks/launch-package-maturity-classification.test.ts",
      " M benchmarks/next-intl-dashboard-workflow.test.ts",
      " M benchmarks/next-intl-launch-package-lane-template.test.ts",
      "?? benchmarks/authentication-lock-backed-template.test.ts",
      "?? benchmarks/lane7-3d-lock-promotion.test.ts",
      "?? benchmarks/template-forms-validation-receipt-wiring.test.ts",
      " M examples/dashboard/src/components/BetterAuthAccountWorkflow.tsx",
      " M examples/dashboard/src/lib/forge/auth/better-auth/dashboard.ts",
      " M benchmarks/instantdb-route-handler-slice.test.ts",
      "?? benchmarks/lane4-forge-materialization-cache.test.ts",
      "?? benchmarks/lane4-runtime-safe-readiness-route.test.ts",
      " M benchmarks/nextjs-compatibility-map.test.ts",
      "?? benchmarks/next-custom-transforms-receipt.test.ts",
      " M tools/build-graph/dx-graph.ts",
      " M tools/build-graph/index.js",
      " M tools/build-graph/scanner.ts",
      "?? tools/build-graph/asset-references.ts",
      "?? tools/build-graph/vendor-root.ts",
      "?? tools/vendor/",
      " M benchmarks/fixtures/build-graph/minimal-app/styles/generated.css",
      "?? benchmarks/mdx-docs-source-build-contract.test.ts",
      "?? benchmarks/source-build-image-metadata-source-guard.test.ts",
      "?? benchmarks/default-www-template-contract.test.ts",
      "?? benchmarks/lane7-default-template-surface.test.ts",
      "?? benchmarks/template-readiness-execution-proof.test.ts",
      " M tools/launch/materialize-www-template.ts",
      "?? benchmarks/launch-docs-honesty.test.ts",
      "?? dx-www/tests/diagnostics_cli.rs",
      "?? tools/launch/launch-compile-gate.js",
      "?? tools/launch/launch-route-smoke.js",
      "?? .dx/receipts/next-rust/",
      "",
    ].join("\n"),
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });
  const dirtyRisk = report.checks.find((entry) => entry.id === "dirty-worktree-lane-risk");
  const ownerHints = new Map(
    dirtyRisk.details.ownerHints.map((entry) => [entry.bucket, entry]),
  );

  assert.deepEqual(ownerHints.get("forge-package-template-truth"), {
    ownerLane: 11,
    bucket: "forge-package-template-truth",
    count: 21,
    samplePaths: [
      "benchmarks/automations-dx-check-package-lane-panel.test.ts",
      "benchmarks/automations-receipt-hash-refresh.test.ts",
      "benchmarks/better-auth-dashboard-workflow.test.ts",
      "benchmarks/better-auth-live-runtime.test.ts",
      "benchmarks/instantdb-dx-check-visibility.test.ts",
    ],
  });
  assert.deepEqual(ownerHints.get("route-handler-execution"), {
    ownerLane: 3,
    bucket: "route-handler-execution",
    count: 1,
    samplePaths: ["benchmarks/instantdb-route-handler-slice.test.ts"],
  });
  assert.deepEqual(ownerHints.get("app-router-runtime-parity"), {
    ownerLane: 4,
    bucket: "app-router-runtime-parity",
    count: 2,
    samplePaths: [
      "benchmarks/lane4-forge-materialization-cache.test.ts",
      "benchmarks/lane4-runtime-safe-readiness-route.test.ts",
    ],
  });
  assert.deepEqual(ownerHints.get("next-rust-reference-boundary"), {
    ownerLane: 14,
    bucket: "next-rust-reference-boundary",
    count: 3,
    samplePaths: [
      "benchmarks/nextjs-compatibility-map.test.ts",
      "benchmarks/next-custom-transforms-receipt.test.ts",
      "tools/vendor/",
    ],
  });
  assert.deepEqual(ownerHints.get("dx-build-end-to-end"), {
    ownerLane: 2,
    bucket: "dx-build-end-to-end",
    count: 8,
    samplePaths: [
      "tools/build-graph/dx-graph.ts",
      "tools/build-graph/index.js",
      "tools/build-graph/scanner.ts",
      "tools/build-graph/asset-references.ts",
      "tools/build-graph/vendor-root.ts",
    ],
  });
  assert.deepEqual(ownerHints.get("default-template-product-quality"), {
    ownerLane: 10,
    bucket: "default-template-product-quality",
    count: 3,
    samplePaths: [
      "benchmarks/default-www-template-contract.test.ts",
      "benchmarks/lane7-default-template-surface.test.ts",
      "tools/launch/materialize-www-template.ts",
    ],
  });
  assert.deepEqual(ownerHints.get("launch-status-handoff"), {
    ownerLane: 14,
    bucket: "launch-status-handoff",
    count: 1,
    samplePaths: ["benchmarks/launch-docs-honesty.test.ts"],
  });
  assert.deepEqual(ownerHints.get("diagnostics-error-ux"), {
    ownerLane: 9,
    bucket: "diagnostics-error-ux",
    count: 1,
    samplePaths: ["dx-www/tests/diagnostics_cli.rs"],
  });
  assert.deepEqual(ownerHints.get("compile-test-gate"), {
    ownerLane: 1,
    bucket: "compile-test-gate",
    count: 1,
    samplePaths: ["tools/launch/launch-compile-gate.js"],
  });
  assert.deepEqual(ownerHints.get("final-launch-coordinator"), {
    ownerLane: 14,
    bucket: "final-launch-coordinator",
    count: 1,
    samplePaths: ["tools/launch/launch-route-smoke.js"],
  });
  assert.deepEqual(ownerHints.get("generated-preview-noise"), {
    ownerLane: 7,
    bucket: "generated-preview-noise",
    count: 1,
    samplePaths: [".dx/receipts/next-rust/"],
  });
  assert.equal(ownerHints.has("unclassified-source-or-doc"), false);
});

test("Lane 14 launch coordinator blocks public schema version suffix noise from fixtures", () => {
  const fixtureRoot = makeCoordinatorFixtureRoot();
  const schemaFile = path.join(
    fixtureRoot,
    "dx-www/src/cli/app_route_handler_receipt.rs",
  );
  fs.writeFileSync(schemaFile, 'const SCHEMA: &str = "dx.app.route.handler.v1";\n');

  const report = buildLaunchCoordinatorReport({
    cwd: fixtureRoot,
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: "",
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });
  const schemaNoise = report.checks.find((entry) => entry.id === "public-schema-version-noise");

  assert.equal(schemaNoise.status, "blocked");
  assert.equal(schemaNoise.blocking, true);
  assert.equal(schemaNoise.ownerLane, 14);
  assert.ok(schemaNoise.details.violationCount > 0);
  assert.deepEqual(schemaNoise.details.violations, [
    {
      id: "public-schema-version-suffix",
      file: "dx-www/src/cli/app_route_handler_receipt.rs",
      line: 1,
      column: 23,
      matchLength: "dx.app.route.handler.v1".length,
    },
  ]);
  assert.doesNotMatch(
    JSON.stringify(schemaNoise),
    /\bdx(?:\.[A-Za-z][A-Za-z0-9_-]*)+\.v1\b/,
  );
});

test("Lane 14 launch coordinator scans launch-sensitive source roots for public schema suffix noise", () => {
  const fixtureRoot = makeCoordinatorFixtureRoot();
  const extraSchemaFile = path.join(
    fixtureRoot,
    "dx-www/src/dev/unlisted_schema.rs",
  );
  fs.mkdirSync(path.dirname(extraSchemaFile), { recursive: true });
  fs.writeFileSync(
    extraSchemaFile,
    'const SCHEMA: &str = "dx.unlisted.launch.contract.v1";\n',
  );

  const report = buildLaunchCoordinatorReport({
    cwd: fixtureRoot,
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: "",
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });
  const schemaNoise = report.checks.find((entry) => entry.id === "public-schema-version-noise");

  assert.equal(schemaNoise.status, "blocked");
  assert.equal(schemaNoise.details.violationCount, 1);
  assert.ok(
    schemaNoise.details.scannedFiles.includes("dx-www/src/dev/unlisted_schema.rs"),
  );
  assert.deepEqual(schemaNoise.details.violations, [
    {
      id: "public-schema-version-suffix",
      file: "dx-www/src/dev/unlisted_schema.rs",
      line: 1,
      column: 23,
      matchLength: "dx.unlisted.launch.contract.v1".length,
    },
  ]);
});

test("Lane 14 launch coordinator blocks launch status overclaims", () => {
  const fixtureRoot = makeCoordinatorFixtureRoot();
  const statusFile = path.join(fixtureRoot, "docs/launch-status.md");
  fs.writeFileSync(statusFile, "production merge ready: true\n");

  const report = buildLaunchCoordinatorReport({
    cwd: fixtureRoot,
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: "",
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
  });
  const overclaims = report.checks.find((entry) => entry.id === "launch-status-overclaims");

  assert.equal(overclaims.status, "blocked");
  assert.equal(overclaims.blocking, true);
  assert.equal(overclaims.ownerLane, 14);
  assert.deepEqual(report.blockers.map((entry) => entry.id), [
    "launch-status-overclaims",
  ]);
  assert.deepEqual(overclaims.details.violations, [
    {
      id: "production-merge-ready-overclaim",
      file: "docs/launch-status.md",
      line: 1,
      column: 1,
      matchLength: "production merge ready: true".length,
    },
  ]);
});

test("Lane 14 launch coordinator can attach live route probe evidence", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: repoRoot,
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: "",
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
    routeProbeReport: {
      schema: "dx.www.launchStabilize.routeProbe",
      format: 1,
      status: "adapter-boundary",
      passedCount: 5,
      adapterBoundaryCount: 1,
      failedCount: 0,
    routes: [
      { path: "/", status: "passed", actualStatus: 200 },
      {
        path: "/api/provider-boundary",
        status: "adapter-boundary",
        actualStatus: 501,
      },
      ],
    },
  });
  const routeProbe = report.checks.find((entry) => entry.id === "live-route-probe");

  assert.equal(routeProbe.status, "adapter-boundary");
  assert.equal(routeProbe.blocking, false);
  assert.equal(routeProbe.ownerLane, 14);
  assert.equal(routeProbe.details.passedCount, 5);
  assert.equal(routeProbe.details.adapterBoundaryCount, 1);
  assert.equal(report.hundredPercentReady, false);
  assert.deepEqual(report.readinessGaps.map((entry) => entry.id), [
    "live-route-probe",
  ]);
  assert.equal(
    report.readinessGaps.find((entry) => entry.id === "live-route-probe").blocking,
    false,
  );
  assert.deepEqual(report.blockers.map((entry) => entry.id), []);
});

test("Lane 14 launch coordinator scores nonblocking gaps below final 100", () => {
  const fixtureRoot = makeCoordinatorFixtureRoot();
  const report = buildLaunchCoordinatorReport({
    cwd: fixtureRoot,
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: "",
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
    ],
    routeProbeReport: {
      schema: "dx.www.launchStabilize.routeProbe",
      format: 1,
      status: "adapter-boundary",
      passedCount: 5,
      adapterBoundaryCount: 1,
      failedCount: 0,
    routes: [
      { path: "/", status: "passed", actualStatus: 200 },
      {
        path: "/api/provider-boundary",
        status: "adapter-boundary",
        actualStatus: 501,
      },
      ],
    },
  });

  assert.equal(report.status, "passing");
  assert.equal(report.launchScore, 100);
  assert.equal(report.hundredPercentReady, false);
  assert.equal(report.finalReadinessScore, 92);
  assert.deepEqual(report.blockers, []);
  assert.deepEqual(report.readinessGaps.map((entry) => entry.id), ["live-route-probe"]);
  assert.deepEqual(report.readinessSummary, {
    blockerCount: 0,
    nonBlockingGapCount: 1,
    totalGapCount: 1,
    finalReadinessScore: 92,
  });
  assert.deepEqual(report.ownerLaneSummary, [
    {
      ownerLane: 14,
      blockerCount: 0,
      nonBlockingGapCount: 1,
      totalGapCount: 1,
      checkIds: ["live-route-probe"],
      nextActions: ["keep adapter-boundary 501 routes visible until route-owner lanes replace them"],
    },
  ]);
});

test("Lane 14 coordinator CLI can require strict 100 readiness", async () => {
  const fixtureRoot = makeCoordinatorFixtureRoot();
  const { baseUrl, close } = await startFixtureServer((request, response) => {
    const statusByPath = new Map([
      ["/", 200],
      ["/", 200],
      ["/dashboard", 200],
      ["/_dx/hot-reload/version", 200],
      ["/api/trpc/health", 200],
      ["/api/database-api/readiness", 200],
    ]);
    response.writeHead(statusByPath.get(new URL(request.url, baseUrl).pathname) || 404, {
      "content-type": "application/json",
    });
    response.end(JSON.stringify({ ok: true }));
  });
  const stdout = [];
  const stderr = [];

  try {
    const result = await runLaunchCoordinatorCli({
      cwd: fixtureRoot,
      generatedAt: "2026-05-23T00:00:00.000Z",
      argv: [
        "--probe-live-routes",
        "--include-database-readiness",
        "--base-url",
        baseUrl,
        "--strict-final",
        "--proven-check",
        "dx-style-compile-proof",
        "--proven-check",
        "dx-build-tiny-app-proof",
        "--proven-check",
        "server-data-json-proof",
      ],
      stdout: { write: (chunk) => stdout.push(String(chunk)) },
      stderr: { write: (chunk) => stderr.push(String(chunk)) },
    });
    const report = JSON.parse(stdout.join(""));
    const routeProbe = report.checks.find((entry) => entry.id === "live-route-probe");
    const diffCheck = report.checks.find((entry) => entry.id === "git-diff-whitespace");

    assert.equal(stderr.join(""), "");
    assert.equal(result.exitCode, 1);
    assert.equal(report.status, "blocked");
    assert.equal(report.hundredPercentReady, false);
    assert.equal(report.finalReadinessScore, 88);
    assert.equal(diffCheck.status, "blocked");
    assert.equal(routeProbe.status, "passed");
    assert.equal(routeProbe.blocking, false);
    assert.deepEqual(report.blockers.map((entry) => entry.id), ["git-diff-whitespace"]);
    assert.deepEqual(report.readinessGaps.map((entry) => entry.id), ["git-diff-whitespace"]);
  } finally {
    await close();
  }
});

test("Lane 14 coordinator CLI rejects unknown proof ids", async () => {
  const stdout = [];
  const stderr = [];

  const result = await runLaunchCoordinatorCli({
    cwd: makeCoordinatorFixtureRoot(),
    generatedAt: "2026-05-23T00:00:00.000Z",
    argv: ["--proven-check", "dx-build-proofish"],
    stdout: { write: (chunk) => stdout.push(String(chunk)) },
    stderr: { write: (chunk) => stderr.push(String(chunk)) },
  });

  assert.equal(result.exitCode, 1);
  assert.equal(result.report, null);
  assert.equal(stdout.join(""), "");
  assert.match(stderr.join(""), /Unknown launch coordinator proof id: dx-build-proofish/);
});

test("Lane 14 coordinator CLI can probe the running launch routes", async () => {
  const { baseUrl, close } = await startFixtureServer((request, response) => {
    const statusByPath = new Map([
      ["/", 200],
      ["/", 200],
      ["/dashboard", 200],
      ["/_dx/hot-reload/version", 200],
      ["/api/trpc/health", 200],
      ["/api/database-api/readiness", 200],
    ]);
    response.writeHead(statusByPath.get(new URL(request.url, baseUrl).pathname) || 404, {
      "content-type": "application/json",
    });
    response.end(JSON.stringify({ ok: true }));
  });
  const writes = [];

  try {
    const result = await runLaunchCoordinatorCli({
      cwd: repoRoot,
      generatedAt: "2026-05-23T00:00:00.000Z",
      argv: [
        "--probe-live-routes",
        "--include-database-readiness",
        "--base-url",
        baseUrl,
      ],
      stdout: { write: (chunk) => writes.push(String(chunk)) },
      stderr: { write: (chunk) => writes.push(String(chunk)) },
    });
    const report = JSON.parse(writes.join(""));
    const routeProbe = report.checks.find((entry) => entry.id === "live-route-probe");

    assert.equal(result.exitCode, 1);
    assert.equal(report.schema, LAUNCH_COORDINATOR_SCHEMA);
    assert.equal(routeProbe.status, "passed");
    assert.equal(routeProbe.blocking, false);
    assert.equal(routeProbe.details.baseUrl, baseUrl);
    assert.equal(routeProbe.details.passedCount, 6);
    assert.equal(routeProbe.details.adapterBoundaryCount, 0);
    assert.equal(routeProbe.details.failedCount, 0);
  } finally {
    await close();
  }
});

test("Lane 14 coordinator CLI can try launch route base URL candidates", async () => {
  const closed = await startFixtureServer((_request, response) => {
    response.writeHead(200, { "content-type": "text/plain" });
    response.end("closing");
  });
  await closed.close();

  const { baseUrl, close } = await startFixtureServer((request, response) => {
    const statusByPath = new Map([
      ["/", 200],
      ["/", 200],
      ["/dashboard", 200],
      ["/_dx/hot-reload/version", 200],
      ["/api/trpc/health", 200],
      ["/api/database-api/readiness", 200],
    ]);
    response.writeHead(statusByPath.get(new URL(request.url, baseUrl).pathname) || 404, {
      "content-type": "application/json",
    });
    response.end(JSON.stringify({ ok: true }));
  });
  const writes = [];

  try {
    const result = await runLaunchCoordinatorCli({
      cwd: repoRoot,
      generatedAt: "2026-05-23T00:00:00.000Z",
      argv: [
        "--probe-live-routes",
        "--include-database-readiness",
        "--base-url-candidates",
        `${closed.baseUrl},${baseUrl}`,
      ],
      stdout: { write: (chunk) => writes.push(String(chunk)) },
      stderr: { write: (chunk) => writes.push(String(chunk)) },
    });
    const report = JSON.parse(writes.join(""));
    const routeProbe = report.checks.find((entry) => entry.id === "live-route-probe");

    assert.equal(result.exitCode, 1);
    assert.equal(routeProbe.status, "passed");
    assert.equal(routeProbe.details.baseUrl, baseUrl);
    assert.equal(routeProbe.details.candidateAttempts.length, 2);
    assert.equal(routeProbe.details.candidateAttempts[0].baseUrl, closed.baseUrl);
    assert.equal(routeProbe.details.candidateAttempts[0].connectivity.status, "unreachable");
    assert.equal(routeProbe.details.candidateAttempts[1].status, "passed");
  } finally {
    await close();
  }
});

test("Lane 14 coordinator reports unreachable live server distinctly", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: repoRoot,
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: "",
    routeProbeReport: {
      schema: "dx.www.launchStabilize.routeProbe",
      format: 1,
      status: "failed",
      passedCount: 0,
      adapterBoundaryCount: 0,
      failedCount: 5,
      connectivity: {
        status: "unreachable",
        failureKind: "connection-refused",
        failedConnectionCount: 5,
        evidence: "server did not accept TCP connections",
      },
      routes: [],
    },
  });
  const routeProbe = report.checks.find((entry) => entry.id === "live-route-probe");

  assert.equal(routeProbe.status, "failed");
  assert.equal(routeProbe.blocking, true);
  assert.match(routeProbe.evidence, /live launch server is unreachable/);
  assert.equal(routeProbe.details.connectivity.status, "unreachable");
  assert.equal(routeProbe.details.connectivity.failureKind, "connection-refused");
});

test("Lane 14 coordinator summarizes mixed live route probe failures", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: makeCoordinatorFixtureRoot(),
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: "",
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
    routeProbeReport: {
      schema: "dx.www.launchStabilize.routeProbe",
      format: 1,
      status: "failed",
      baseUrl: "http://127.0.0.1:3000",
      passedCount: 2,
      adapterBoundaryCount: 0,
      failedCount: 3,
      connectivity: {
        status: "mixed",
        failureKind: "timeout",
        failedConnectionCount: 3,
        responseCount: 2,
        evidence: "2 route probes received HTTP responses and 3 connection attempts failed",
      },
      routes: [
        {
          path: "/",
          label: "home page",
          status: "failed",
          actualStatus: null,
          elapsedMs: 5006,
          evidence: "request failed: timed out after 5000ms",
        },
        {
          path: "/login",
          label: "login page",
          status: "failed",
          actualStatus: null,
          elapsedMs: 5004,
          evidence: "request failed: timed out after 5000ms",
        },
        {
          path: "/dashboard",
          label: "dashboard page",
          status: "failed",
          actualStatus: null,
          elapsedMs: 5002,
          evidence: "request failed: timed out after 5000ms",
        },
        {
          path: "/_dx/hot-reload/version",
          label: "DX hot reload version",
          status: "passed",
          actualStatus: 200,
          elapsedMs: 12,
          evidence: "received expected HTTP 200",
        },
        {
          path: "/api/trpc/health",
          label: "DX source-safe tRPC health",
          status: "passed",
          actualStatus: 200,
          elapsedMs: 9,
          evidence: "received expected HTTP 200",
        },
      ],
    },
  });
  const routeProbe = report.checks.find((entry) => entry.id === "live-route-probe");

  assert.equal(routeProbe.status, "failed");
  assert.equal(routeProbe.blocking, true);
  assert.match(routeProbe.evidence, /3 live launch route probes failed: \/, \/login, \/dashboard/);
  assert.match(routeProbe.evidence, /2 route probes received HTTP responses and 3 connection attempts failed/);
  assert.deepEqual(routeProbe.details.failedRoutes, [
    {
      path: "/",
      label: "home page",
      actualStatus: null,
      elapsedMs: 5006,
      evidence: "request failed: timed out after 5000ms",
    },
    {
      path: "/login",
      label: "login page",
      actualStatus: null,
      elapsedMs: 5004,
      evidence: "request failed: timed out after 5000ms",
    },
    {
      path: "/dashboard",
      label: "dashboard page",
      actualStatus: null,
      elapsedMs: 5002,
      evidence: "request failed: timed out after 5000ms",
    },
  ]);
  assert.deepEqual(routeProbe.details.passedRoutes, [
    "/_dx/hot-reload/version",
    "/api/trpc/health",
  ]);
  assert.deepEqual(report.blockers.map((entry) => entry.id), ["live-route-probe"]);
});

test("Lane 14 cleanup plan hands failed live route probes to the dev server lane", () => {
  const report = buildLaunchCoordinatorReport({
    cwd: makeCoordinatorFixtureRoot(),
    generatedAt: "2026-05-23T00:00:00.000Z",
    gitStatusText: "",
    provenCheckIds: [
      "dx-style-compile-proof",
      "dx-build-tiny-app-proof",
      "server-data-json-proof",
      "giant-cli-mod-risk",
    ],
    routeProbeReport: {
      schema: "dx.www.launchStabilize.routeProbe",
      format: 1,
      status: "failed",
      baseUrl: "http://127.0.0.1:3000",
      passedCount: 0,
      adapterBoundaryCount: 0,
      failedCount: 5,
      connectivity: {
        status: "unreachable",
        failureKind: "connection-refused",
        failedConnectionCount: 5,
        responseCount: 0,
        evidence: "server did not accept TCP connections for 5 route probes",
      },
      routes: [],
    },
  });
  const routeProbe = report.checks.find((entry) => entry.id === "live-route-probe");
  const devServerPlan = report.laneCleanupPlan.find((entry) => entry.ownerLane === 8);
  const coordinatorPlan = report.laneCleanupPlan.find((entry) => entry.ownerLane === 14);

  assert.equal(routeProbe.ownerLane, 14);
  assert.equal(routeProbe.handoffLane, 8);
  assert.deepEqual(devServerPlan.blockingCheckIds, ["live-route-probe"]);
  assert.deepEqual(devServerPlan.nextActions, [
    "restore the Rust/Axum dev server and route responses on port 3000 before final launch signoff",
  ]);
  assert.equal(coordinatorPlan, undefined);
});

function startFixtureServer(handler) {
  return new Promise((resolve, reject) => {
    const server = http.createServer(handler);
    server.once("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      resolve({
        baseUrl: `http://127.0.0.1:${address.port}`,
        close: () =>
          new Promise((closeResolve, closeReject) => {
            server.close((error) => (error ? closeReject(error) : closeResolve()));
          }),
      });
    });
  });
}

function makeCoordinatorFixtureRoot({
  missingRouteSources = [],
  sourceOnlyRouteShadows = [],
} = {}) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "dx-www-launch-coordinator-"));
  const directories = [
    "examples/template/app",
    "examples/template/components",
    "examples/template/server",
    "examples/template/styles",
    "docs",
    "vendor/next-rust",
    "dx-www/src/cli",
    "dx-www/src",
    "core/src/delivery",
    "core/src/ecosystem",
    "tools/next-rust-merge",
    "benchmarks",
  ];
  const files = [
    "docs/next-rust-merge-checkpoint.md",
    "vendor/next-rust/README.md",
    "dx-www/src/next_rust.rs",
    "dx-www/src/diagnostics.rs",
    "dx-www/src/cli/app_route_handler_receipt.rs",
    "dx-www/src/cli/mod.rs",
    "core/src/delivery/server_contract.rs",
    "core/src/ecosystem/dx_check_receipt.rs",
    "examples/template/app/page.tsx",
    "examples/template/app/dashboard/page.tsx",
    "examples/template/app/login/page.tsx",
    "examples/template/app/logout/page.tsx",
    "tools/next-rust-merge/coordinator-checks.cjs",
    "benchmarks/next-rust-merge-coordinator.test.ts",
    "benchmarks/next-rust-merge-audit-comparison.test.ts",
    "benchmarks/next-rust-vendor-boundary.test.ts",
    "benchmarks/next-rust-schema-status-noise.test.ts",
    "benchmarks/next-rust-giant-cli-mod.test.ts",
  ];

  for (const directory of directories) {
    fs.mkdirSync(path.join(root, directory), { recursive: true });
  }
  for (const file of files) {
    if (missingRouteSources.includes(file)) continue;
    const filePath = path.join(root, file);
    fs.mkdirSync(path.dirname(filePath), { recursive: true });
    fs.writeFileSync(filePath, "fixture\n");
  }
  for (const file of sourceOnlyRouteShadows) {
    fs.mkdirSync(path.dirname(path.join(root, file)), { recursive: true });
    fs.writeFileSync(path.join(root, file), "source-only shadow\n");
  }

  return root;
}
