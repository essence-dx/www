const assert = require("node:assert/strict");
const { execFileSync, spawnSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const {
  createDxBuildGraphConsumerSnapshotFromReceipt,
  createTurboTasksAdapterDiffConsumerSummary,
  createTurboTasksAdapterStatusFromReceipt,
  diffTurboTasksAdapterPlans,
  scanDxBuildGraph,
  writeDxBuildGraphReceipt,
} = require("../tools/build-graph");

const repoRoot = path.resolve(__dirname, "..");
const fixtureRoot = path.join(
  __dirname,
  "fixtures",
  "build-graph",
  "minimal-app",
);

test("Turbo Tasks source study stays reference-only and adapter-boundary", () => {
  const report = scanDxBuildGraph(fixtureRoot, {
    changedPaths: ["styles/generated.css"],
  });
  const route = nodeByPath(report, "app/page.tsx");
  const component = nodeByPath(report, "components/LaunchPanel.tsx");
  const css = nodeByPath(report, "styles/generated.css");

  assert.equal(report.turboTasksAdapter.schema, "dx.build.graph.turboTasksAdapter");
  assert.equal(report.turboTasksAdapter.positioning.adapterOnly, true);
  assert.equal(report.turboTasksAdapter.positioning.turbopackPublicDependency, false);
  assert.equal(report.turboTasksAdapter.positioning.nodeNapiFoundation, false);
  assert.equal(report.turboTasksAdapter.positioning.dxRuntimeAuthoritative, true);
  assert.equal(report.turboTasksAdapter.upstream.copiedCode, false);
  assert.deepEqual(report.turboTasksAdapter.upstream.crates, [
    "turbo-tasks",
    "turbo-tasks-backend",
    "turbo-tasks-fs",
    "turbo-persistence",
  ]);
  assert.deepEqual(report.turboTasksAdapter.invalidation.rebuildNodeIds, [
    css.id,
    component.id,
    route.id,
  ]);
  assert.equal(
    report.turboTasksAdapter.parallelism.scheduler,
    "dx-owned-topological-levels",
  );
  assert.equal(report.turboTasksAdapter.persistence.mode, "source-receipt-plan");
  assert.equal(report.turboTasksAdapter.persistence.turboPersistencePublicDependency, false);
  assert.ok(
    report.turboTasksAdapter.tasks.every(
      (task) => task.adapterBoundary === true && task.publicArchitecture === false,
    ),
  );
});

test("Turbo Tasks adapter diff/status remain usable without runtime execution", () => {
  const previous = scanDxBuildGraph(fixtureRoot, {
    changedPaths: ["styles/generated.css"],
  });
  const current = scanDxBuildGraph(fixtureRoot, {
    changedPaths: ["components/LaunchPanel.tsx"],
  });
  const diff = diffTurboTasksAdapterPlans(
    previous.turboTasksAdapter,
    current.turboTasksAdapter,
  );
  const status = createTurboTasksAdapterStatusFromReceipt(current);
  const snapshot = createDxBuildGraphConsumerSnapshotFromReceipt(current, null);

  assert.equal(diff.schema, "dx.build.graph.turboTasksAdapterDiff");
  assert.equal(diff.boundary.adapterOnly, true);
  assert.equal(diff.boundary.publicArchitecture, false);
  assert.equal(diff.boundary.turboPersistencePublicDependency, false);
  assert.equal(status.schema, "dx.build.graph.turboTasksAdapterStatus");
  assert.equal(status.architecture.publicTurbopackDependency, false);
  assert.equal(status.architecture.dxRuntimeAuthoritative, true);
  assert.equal(snapshot.turboTasksAdapter.schema, "dx.build.graph.turboTasksAdapter");
  assert.equal(snapshot.turboTasksAdapter.boundary.adapterOnly, true);
  assert.equal(snapshot.turboTasksAdapter.boundary.publicArchitecture, false);
  assert.equal(status.evidence.referenceOnly, true);
  assert.equal(status.evidence.runtimeExecutionPath, false);
  assert.equal(status.evidence.taskCacheExecutorSchema, null);
});

test("graph CLI exposes reference-only Turbo Tasks status and rejects execution flags", () => {
  const tmpRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-turbo-reference-"));
  const receiptPath = path.join(tmpRoot, "graph.json");
  writeDxBuildGraphReceipt(fixtureRoot, receiptPath, {
    changedPaths: ["styles/generated.css"],
  });

  const status = JSON.parse(
    execFileSync(
      process.execPath,
      [
        path.join(repoRoot, "tools/build-graph/dx-graph.ts"),
        "--project",
        fixtureRoot,
        "--turbo-tasks-status",
        "--json",
      ],
      { encoding: "utf8" },
    ),
  );
  assert.equal(status.schema, "dx.build.graph.turboTasksAdapterStatus");
  assert.equal(status.architecture.publicTurbopackDependency, false);
  assert.equal(status.architecture.dxRuntimeAuthoritative, true);

  const executionAttempt = spawnSync(
    process.execPath,
    [
      path.join(repoRoot, "tools/build-graph/dx-graph.ts"),
      "--project",
      fixtureRoot,
      "--changed",
      "styles/generated.css",
      "--execute-turbo-tasks",
      "--json",
    ],
    { encoding: "utf8" },
  );
  assert.notEqual(executionAttempt.status, 0);
  assert.match(
    executionAttempt.stderr,
    /Turbo Tasks execution is out of scope for DX-WWW/,
  );

  const diffSummary = createTurboTasksAdapterDiffConsumerSummary(
    writeDiffReceipt(tmpRoot, receiptPath),
  );
  assert.equal(diffSummary.boundary.publicArchitecture, false);
  assert.equal(diffSummary.boundary.turboPersistencePublicDependency, false);
});

test("out-of-scope Turbo Tasks execution modules are not exported or kept active", () => {
  const graphIndex = fs.readFileSync(
    path.join(repoRoot, "tools/build-graph/index.js"),
    "utf8",
  );
  const graphCli = fs.readFileSync(
    path.join(repoRoot, "tools/build-graph/dx-graph.ts"),
    "utf8",
  );

  for (const relativePath of [
    "tools/build-graph/turbo-tasks-executor.ts",
    "tools/build-graph/turbo-tasks-execution-reader.ts",
    "tools/build-graph/turbo-tasks-execution-handoff.ts",
    "tools/build-graph/turbo-tasks-execution-read-model.ts",
    "tools/build-graph/turbo-tasks-execution-panel.ts",
    "tools/build-graph/turbo-tasks-zed-handoff.ts",
    "tools/build-graph/turbo-tasks-zed-panel.ts",
  ]) {
    assert.ok(
      !fs.existsSync(path.join(repoRoot, relativePath)),
      `${relativePath} must not remain an active build-graph execution artifact`,
    );
  }

  assert.doesNotMatch(graphIndex, /turbo-tasks-executor|turbo-tasks-execution|turbo-tasks-zed/);
  assert.doesNotMatch(graphCli, /executeTurboTasksAdapterPlan|createTurboTasksExecutor|writeTurboTasksExecution|readTurboTasksZed/);
  assert.match(graphCli, /Turbo Tasks execution is out of scope for DX-WWW/);
});

function nodeByPath(report, relativePath) {
  const normalized = relativePath.replace(/\\/g, "/");
  const node = report.graph.nodes.find((candidate) => candidate.path === normalized);
  assert.ok(node, `expected graph node for ${relativePath}`);
  return node;
}

function writeDiffReceipt(tmpRoot, previousReceiptPath) {
  const currentReceiptPath = path.join(tmpRoot, "current.json");
  const diffReceiptPath = path.join(tmpRoot, "turbo-tasks-diff.json");
  execFileSync(
    process.execPath,
    [
      path.join(repoRoot, "tools/build-graph/dx-graph.ts"),
      "--project",
      fixtureRoot,
      "--changed",
      "components/LaunchPanel.tsx",
      "--write",
      currentReceiptPath,
      "--diff-against",
      previousReceiptPath,
      "--write-diff",
      diffReceiptPath,
      "--json",
    ],
    { encoding: "utf8" },
  );
  return diffReceiptPath;
}
