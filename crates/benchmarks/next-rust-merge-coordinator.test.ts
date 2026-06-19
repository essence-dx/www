import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const {
  CHECK_SCHEMA,
  COORDINATOR_CHECKS,
  commandText,
  coordinatorCheckSummary,
  heavyCommandViolations,
} = require("../tools/next-rust-merge/coordinator-checks.cjs");
const {
  DEFAULT_COORDINATOR_RECEIPT_PATH,
  ARGUMENT_ERROR_SCHEMA,
  FAILURES_SCHEMA,
  RUN_SCHEMA,
  buildCoordinatorArgumentError,
  compactCoordinatorFailures,
  parseArgs,
  resolveCoordinatorReceiptPath,
  resolveCoordinatorChecks,
  runCoordinatorChecks,
  writeCoordinatorReport,
} = require("../tools/next-rust-merge/coordinator-runner.cjs");

const requiredCheckIds = [
  "vendor-boundary",
  "turbo-tasks-adapter",
  "turbopack-core-map",
  "turbopack-core-graph",
  "dx-style-drift-fixture",
  "dev-hot-reload-protocol",
  "next-custom-transforms",
  "default-www-template",
  "www-template-forge-reality",
  "build-readiness-gate",
  "schema-status-noise",
  "giant-cli-mod",
  "merge-conflict-markers",
  "app-router-server-data",
];

test("Lane 14 coordinator registry covers the merge risks without feature implementation", () => {
  const summary = coordinatorCheckSummary();

  assert.equal(summary.schema, CHECK_SCHEMA);
  assert.equal(summary.lane, 14);
  assert.equal(summary.laneName, "Final Coordinator");
  assert.equal(summary.featureImplementation, false);

  const ids = new Set(COORDINATOR_CHECKS.map((entry) => entry.id));
  assert.equal(ids.size, COORDINATOR_CHECKS.length, "coordinator check ids must be unique");
  for (const id of requiredCheckIds) {
    assert.ok(ids.has(id), `missing coordinator check ${id}`);
  }
});

test("Lane 14 coordinator checks stay lightweight and source-only", () => {
  for (const entry of COORDINATOR_CHECKS) {
    assert.equal(entry.heavy, false, `${entry.id} must not be a heavy check`);
    assert.equal(entry.publicTurbopackDependency, false, `${entry.id} must not publish Turbopack`);
    assert.equal(entry.requiresReactCore, false, `${entry.id} must not require React core`);
    assert.equal(entry.requiresNodeModules, false, `${entry.id} must not require node_modules`);
    assert.deepEqual(heavyCommandViolations(entry), [], `${entry.id} uses a heavy command`);
    assert.match(entry.boundary, /boundary|receipt|contract|gate|model|map/);
    assert.ok(entry.proves.length > 0, `${entry.id} should state what it proves`);
  }
});

test("Lane 14 coordinator declares receipt side effects before checks run", () => {
  const summary = coordinatorCheckSummary();
  const sideEffectsById = new Map(
    summary.checks.map((entry) => [entry.id, entry.sideEffects]),
  );

  assert.deepEqual(summary.sideEffectSummary, {
    workspaceWriteCheckCount: 0,
    tempFixtureWriteCheckCount: 5,
    readOnlyCheckCount: COORDINATOR_CHECKS.length - 5,
    receiptWritingCheckIds: [
      "vendor-boundary",
      "turbo-tasks-adapter",
      "turbopack-core-map",
      "turbopack-core-graph",
      "build-readiness-gate",
    ],
  });

  assert.deepEqual(sideEffectsById.get("vendor-boundary"), {
    workspaceWrites: false,
    tempFixtureWrites: true,
    writesReceipts: true,
    receiptPaths: [
      ".dx/receipts/next-rust/vendor-boundary.json",
      ".dx/receipts/next-rust/vendor-boundary-consumer.json",
    ],
    note: "writes only temp-fixture vendor receipts when run through the benchmark",
  });
  assert.equal(sideEffectsById.get("dx-style-drift-fixture").writesReceipts, false);
  assert.equal(sideEffectsById.get("default-www-template").workspaceWrites, false);
});

test("Lane 14 coordinator side-effect contract stays split from the check registry", () => {
  const sideEffectModulePath = path.join(
    repoRoot,
    "tools",
    "next-rust-merge",
    "coordinator-side-effects.cjs",
  );
  assert.ok(fs.existsSync(sideEffectModulePath), "missing side-effect contract module");

  const {
    coordinatorSideEffectSummary,
    readOnlySideEffects,
    tempReceiptSideEffects,
  } = require("../tools/next-rust-merge/coordinator-side-effects.cjs");
  const registrySource = fs.readFileSync(
    path.join(repoRoot, "tools", "next-rust-merge", "coordinator-checks.cjs"),
    "utf8",
  );

  assert.match(registrySource, /require\("\.\/coordinator-side-effects\.cjs"\)/);
  assert.doesNotMatch(registrySource, /function tempReceiptSideEffects/);
  assert.doesNotMatch(registrySource, /function coordinatorSideEffectSummary/);
  assert.deepEqual(readOnlySideEffects("fixture check is read-only"), {
    workspaceWrites: false,
    tempFixtureWrites: false,
    writesReceipts: false,
    receiptPaths: [],
    note: "fixture check is read-only",
  });
  const receiptPaths = [".dx/receipts/example.json"];
  const tempSideEffects = tempReceiptSideEffects({
    receiptPaths,
    note: "temp only",
  });

  receiptPaths.push(".dx/receipts/other.json");
  assert.deepEqual(tempSideEffects, {
    workspaceWrites: false,
    tempFixtureWrites: true,
    writesReceipts: true,
    receiptPaths: [".dx/receipts/example.json"],
    note: "temp only",
  });
  assert.deepEqual(
    coordinatorSideEffectSummary([
      {
        id: "read-only",
        sideEffects: readOnlySideEffects("read-only fixture"),
      },
      {
        id: "temp",
        sideEffects: tempReceiptSideEffects({
          receiptPaths: [".dx/receipts/example.json"],
          note: "temp only",
        }),
      },
    ]),
    {
      workspaceWriteCheckCount: 0,
      tempFixtureWriteCheckCount: 1,
      readOnlyCheckCount: 1,
      receiptWritingCheckIds: ["temp"],
    },
  );
});

test("Lane 14 coordinator scores the Lane 3 core concept map validation", () => {
  const entry = COORDINATOR_CHECKS.find(
    (candidate) => candidate.id === "turbopack-core-map",
  );

  assert.ok(entry, "missing Lane 3 Turbopack core map check");
  assert.equal(entry.lane, 3);
  assert.equal(entry.blocking, true);
  assert.equal(entry.boundary, "adapter-boundary");
  assert.equal(entry.publicTurbopackDependency, false);
  assert.equal(entry.requiresReactCore, false);
  assert.equal(entry.requiresNodeModules, false);
  assert.deepEqual(entry.command, [
    "node",
    "--test",
    "benchmarks/dx-build-graph-core-map.test.ts",
  ]);
  assert.ok(
    entry.proves.includes("vendor-backed core concept validation"),
    "Lane 3 coordinator check should score the validation guard",
  );
  assert.deepEqual(entry.healthContract, {
    schema: "dx.build.graph.turbopackCoreConceptMapStatus",
    provider:
      "tools/build-graph/turbopack-core-status.ts#createTurbopackCoreConceptMapStatus",
    consumerSnapshotField: "coreConceptMapStatus",
    statusField: "status",
    scoreField: "score",
  });
});

test("Lane 14 coordinator run preserves Lane 3 health contract metadata", () => {
  const entry = COORDINATOR_CHECKS.find(
    (candidate) => candidate.id === "turbopack-core-map",
  );

  const report = runCoordinatorChecks({
    checks: [entry],
    cwd: repoRoot,
    clock: () => 10,
    runCommand: () => ({ status: 0, stdout: "ok", stderr: "", signal: null }),
    startedAt: "2026-05-23T00:00:00.000Z",
  });

  assert.equal(report.status, "passing");
  assert.equal(report.checks[0].id, "turbopack-core-map");
  assert.deepEqual(report.checks[0].healthContract, entry.healthContract);
  assert.deepEqual(report.checks[0].sideEffects, entry.sideEffects);
  assert.equal(
    report.checks[0].healthContract.schema,
    "dx.build.graph.turbopackCoreConceptMapStatus",
  );
});

test("Lane 14 coordinator commands point at existing tiny benchmark files", () => {
  for (const entry of COORDINATOR_CHECKS) {
    assert.equal(entry.command[0], "node");
    assert.equal(entry.command[1], "--test");
    assert.equal(entry.command.length, 3, `${entry.id} should be one focused node --test command`);

    const benchmarkPath = path.join(repoRoot, entry.command[2]);
    assert.ok(fs.existsSync(benchmarkPath), `${entry.id} target does not exist: ${entry.command[2]}`);
    assert.match(commandText(entry), /^node --test benchmarks\//);
  }
});

test("Lane 14 coordinator scores merge conflict marker detection", () => {
  const entry = COORDINATOR_CHECKS.find(
    (candidate) => candidate.id === "merge-conflict-markers",
  );

  assert.ok(entry, "missing merge conflict marker coordinator check");
  assert.equal(entry.lane, 14);
  assert.equal(entry.blocking, true);
  assert.equal(entry.boundary, "coordinator source-only gate");
  assert.deepEqual(entry.command, [
    "node",
    "--test",
    "benchmarks/next-rust-conflict-markers.test.ts",
  ]);
  assert.deepEqual(entry.sideEffects, {
    workspaceWrites: false,
    tempFixtureWrites: false,
    writesReceipts: false,
    receiptPaths: [],
    note: "no workspace writes",
  });
  assert.ok(
    entry.proves.includes("no unresolved merge conflict markers in merge-sensitive source surfaces"),
  );
});

test("Lane 14 coordinator runner emits an honest lightweight scorecard", () => {
  const checks = [
    coordinatorFixtureCheck("passing-source-guard", true),
    coordinatorFixtureCheck("warning-source-guard", false),
    coordinatorFixtureCheck("blocking-source-guard", true),
  ];
  const outcomes = new Map([
    ["passing-source-guard", { status: 0, stdout: "ok", stderr: "", signal: null }],
    ["warning-source-guard", { status: 1, stdout: "", stderr: "non-blocking drift", signal: null }],
    ["blocking-source-guard", { status: 1, stdout: "", stderr: "blocking drift", signal: null }],
  ]);
  let tick = 0;

  const report = runCoordinatorChecks({
    checks,
    cwd: repoRoot,
    clock: () => (tick += 7),
    runCommand: (command) => outcomes.get(command[2]),
    startedAt: "2026-05-23T00:00:00.000Z",
  });

  assert.equal(report.schema, RUN_SCHEMA);
  assert.equal(report.lane, 14);
  assert.equal(report.featureImplementation, false);
  assert.equal(report.status, "blocked");
  assert.equal(report.score, 84);
  assert.deepEqual(report.totals, {
    total: 3,
    passed: 1,
    failed: 2,
    blockingFailed: 1,
    nonBlockingFailed: 1,
  });
  assert.equal(report.architecture.dxRuntimeAuthoritative, true);
  assert.equal(report.architecture.publicTurbopackDependency, false);
  assert.equal(report.architecture.reactRequiredCore, false);
  assert.equal(report.architecture.nodeModulesRequired, false);
  assert.equal(report.checks[0].failureSummary, null);
  assert.equal(report.checks[1].failureSummary, "non-blocking drift");
  assert.equal(report.checks[1].stderrPreview, "non-blocking drift");
  assert.equal(report.checks[2].failureSummary, "blocking drift");
  assert.equal(report.checks[2].blocking, true);
});

test("Lane 14 coordinator reports distinguish executed proof from preflight plans", () => {
  const contractModulePath = path.join(
    repoRoot,
    "tools",
    "next-rust-merge",
    "coordinator-report-contract.cjs",
  );
  assert.ok(fs.existsSync(contractModulePath), "missing coordinator report contract module");

  const {
    coordinatorArchitecture,
    coordinatorProofContract,
    coordinatorReceiptWritePolicy,
  } = require("../tools/next-rust-merge/coordinator-report-contract.cjs");
  const runChecks = [
    coordinatorFixtureCheck("read-only-source-guard", true),
    {
      ...coordinatorFixtureCheck("receipt-source-guard", true),
      sideEffects: {
        workspaceWrites: false,
        tempFixtureWrites: true,
        writesReceipts: true,
        receiptPaths: [".dx/receipts/example.json"],
        note: "fixture receipt",
      },
    },
  ];
  let tick = 0;
  const runReport = runCoordinatorChecks({
    checks: runChecks,
    cwd: repoRoot,
    clock: () => (tick += 3),
    runCommand: () => ({ status: 0, stdout: "ok", stderr: "", signal: null }),
    startedAt: "2026-05-23T00:00:00.000Z",
  });

  assert.deepEqual(coordinatorArchitecture(), {
    dxRuntimeAuthoritative: true,
    publicTurbopackDependency: false,
    reactRequiredCore: false,
    nodeModulesRequired: false,
    nodeNapiFoundation: false,
  });
  assert.deepEqual(coordinatorProofContract("preflight"), {
    executionMode: "preflight",
    proofLevel: "declared-source-plan-not-executed",
    score: null,
    scoreReason: "not scored because checks were not executed",
  });
  assert.deepEqual(
    coordinatorReceiptWritePolicy({
      checks: runChecks,
      executionMode: "preflight",
      writesDefaultScorecard: false,
    }),
    {
      writesDefaultScorecard: false,
      executesChecks: false,
      mayWriteTempReceipts: false,
      receiptWritingCheckIds: ["receipt-source-guard"],
      note: "preflight does not run checks or write temp fixture receipts",
    },
  );
  assert.equal(runReport.executionMode, "run");
  assert.equal(runReport.proofLevel, "executed-lightweight-source-checks");
  assert.deepEqual(runReport.receiptWritePolicy, {
    writesDefaultScorecard: false,
    executesChecks: true,
    mayWriteTempReceipts: true,
    receiptWritingCheckIds: ["receipt-source-guard"],
    note: "running checks may write declared temp fixture receipts",
  });

  const {
    buildCoordinatorPreflightReport,
  } = require("../tools/next-rust-merge/coordinator-preflight.cjs");
  const preflight = buildCoordinatorPreflightReport({
    checks: runChecks,
    generatedAt: "2026-05-23T00:00:00.000Z",
  });

  assert.equal(preflight.executionMode, "preflight");
  assert.equal(preflight.proofLevel, "declared-source-plan-not-executed");
  assert.equal(preflight.score, null);
  assert.equal(preflight.scoreReason, "not scored because checks were not executed");
  assert.deepEqual(preflight.receiptWritePolicy, {
    writesDefaultScorecard: false,
    executesChecks: false,
    mayWriteTempReceipts: false,
    receiptWritingCheckIds: ["receipt-source-guard"],
    note: "preflight does not run checks or write temp fixture receipts",
  });
});

test("Lane 14 readiness audit scorecard caps optimistic coordinator runs", () => {
  const auditModulePath = path.join(
    repoRoot,
    "tools",
    "next-rust-merge",
    "coordinator-readiness-audit.cjs",
  );
  assert.ok(fs.existsSync(auditModulePath), "missing readiness audit scorecard module");

  const {
    READINESS_AUDIT_SCHEMA,
    buildReadinessAuditReport,
  } = require("../tools/next-rust-merge/coordinator-readiness-audit.cjs");
  const coordinatorReport = {
    schema: RUN_SCHEMA,
    lane: 14,
    laneName: "Final Coordinator",
    featureImplementation: false,
    executionMode: "run",
    proofLevel: "executed-lightweight-source-checks",
    status: "blocked",
    score: 88,
    totals: {
      total: 12,
      passed: 11,
      failed: 1,
      blockingFailed: 1,
      nonBlockingFailed: 0,
    },
    architecture: {
      dxRuntimeAuthoritative: true,
      publicTurbopackDependency: false,
      reactRequiredCore: false,
      nodeModulesRequired: false,
      nodeNapiFoundation: false,
    },
    checks: [],
  };
  const report = buildReadinessAuditReport({
    coordinatorReport,
    generatedAt: "2026-05-23T00:00:00.000Z",
  });

  assert.equal(report.schema, READINESS_AUDIT_SCHEMA);
  assert.equal(report.lane, 14);
  assert.equal(report.laneName, "Final Coordinator");
  assert.equal(report.featureImplementation, false);
  assert.equal(report.status, "blocked");
  assert.equal(report.executionMode, "audit");
  assert.equal(report.proofLevel, "audit-reconciled-source-scorecard");
  assert.equal(report.productionMergeReady, false);
  assert.equal(report.auditedOverallScore, 46);
  assert.equal(report.auditedDxBuildGraphIntegrationScore, 24);
  assert.equal(report.coordinatorScore, 88);
  assert.equal(report.reconciledScore, 46);
  assert.match(report.scoreReason, /readiness audit/);
  assert.equal(report.coordinatorEvidence.executionMode, "run");
  assert.equal(report.coordinatorEvidence.score, 88);
  assert.deepEqual(report.architecture, coordinatorReport.architecture);

  const gapIds = report.criticalGaps.map((entry) => entry.id);
  assert.deepEqual(gapIds, [
    "dx-build-graph-integration",
    "giant-cli-mod",
    "dx-style-drift-fixture",
    "missing-app-server-data",
    "hmr-polling-only",
    "default-www-template",
    "build-readiness-gate",
    "docs-status-overclaims",
    "schema-status-noise",
  ]);
  assert.equal(report.criticalGaps[0].score, 24);
  assert.equal(report.criticalGaps[0].severity, "blocking");
  assert.equal(report.criticalGaps[2].lane, 5);
  assert.equal(report.criticalGaps[3].lane, 12);
  assert.equal(report.criticalGaps[4].lane, 8);
  assert.equal(report.criticalGaps[5].lane, 13);
  assert.equal(report.criticalGaps[6].lane, 14);
  assert.equal(report.criticalGaps.every((entry) => !entry.schemaName), true);
});

test("Lane 14 readiness audit can consume an explicit audit file", () => {
  const {
    buildReadinessAuditReport,
    readReadinessAuditInput,
  } = require("../tools/next-rust-merge/coordinator-readiness-audit.cjs");
  const auditDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-next-rust-audit-"));
  const auditPath = path.join(auditDir, "audit.json");
  const auditFile = {
    source: "coordinator test fixture",
    overallScore: 41,
    dxBuildGraphIntegrationScore: 19,
    criticalGaps: [
      {
        id: "dx-build-graph-integration",
        lane: "2-11",
        severity: "blocking",
        score: 19,
        status: "under-proven",
        evidence: "fixture still lacks DX build graph integration proof",
        nextAction: "land one executable DX build graph integration proof",
      },
    ],
    nextHighestImpactAction: "fixture action",
  };
  fs.writeFileSync(auditPath, `${JSON.stringify(auditFile, null, 2)}\n`);

  const auditInput = readReadinessAuditInput(auditPath, { cwd: repoRoot });
  const report = buildReadinessAuditReport({
    coordinatorReport: {
      schema: RUN_SCHEMA,
      executionMode: "run",
      proofLevel: "executed-lightweight-source-checks",
      status: "passing",
      score: 96,
      totals: { total: 1, passed: 1, failed: 0, blockingFailed: 0, nonBlockingFailed: 0 },
      architecture: {
        dxRuntimeAuthoritative: true,
        publicTurbopackDependency: false,
        reactRequiredCore: false,
        nodeModulesRequired: false,
        nodeNapiFoundation: false,
      },
      checks: [],
    },
    auditInput,
    generatedAt: "2026-05-23T00:00:00.000Z",
  });

  assert.equal(report.auditSource, auditPath);
  assert.equal(report.auditedOverallScore, 41);
  assert.equal(report.auditedDxBuildGraphIntegrationScore, 19);
  assert.equal(report.coordinatorScore, 96);
  assert.equal(report.reconciledScore, 41);
  assert.deepEqual(report.criticalGaps.map((entry) => entry.id), [
    "dx-build-graph-integration",
  ]);
  assert.equal(report.criticalGaps[0].schemaName, undefined);
  assert.equal(report.nextHighestImpactAction, "fixture action");
});

test("Lane 14 readiness audit baseline is a checked source contract", () => {
  const {
    DEFAULT_READINESS_AUDIT_BASELINE_PATH,
    readDefaultReadinessAuditInput,
  } = require("../tools/next-rust-merge/coordinator-readiness-audit.cjs");
  const baselinePath = path.join(repoRoot, DEFAULT_READINESS_AUDIT_BASELINE_PATH);

  assert.equal(
    DEFAULT_READINESS_AUDIT_BASELINE_PATH,
    "tools/next-rust-merge/readiness-audit-baseline.json",
  );
  assert.ok(fs.existsSync(baselinePath), "missing source-owned audit baseline");

  const baseline = readDefaultReadinessAuditInput({ cwd: repoRoot });
  assert.equal(baseline.source, baselinePath);
  assert.equal(baseline.overallScore, 46);
  assert.equal(baseline.dxBuildGraphIntegrationScore, 24);
  assert.deepEqual(
    baseline.criticalGaps.map((entry) => entry.id),
    [
      "dx-build-graph-integration",
      "giant-cli-mod",
      "dx-style-drift-fixture",
      "missing-app-server-data",
      "hmr-polling-only",
      "default-www-template",
      "build-readiness-gate",
      "docs-status-overclaims",
      "schema-status-noise",
    ],
  );

  const baselineText = fs.readFileSync(baselinePath, "utf8");
  assert.doesNotMatch(baselineText, /\.v1\b/);
  assert.doesNotMatch(baselineText, /full Next\.js parity/i);
});

test("Lane 14 coordinator CLI can audit against the checked baseline", () => {
  const result = spawnSync(
    process.execPath,
    [
      "tools/next-rust-merge/coordinator-runner.cjs",
      "--audit",
      "--audit-baseline",
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      windowsHide: true,
    },
  );

  assert.equal(result.status, 1);
  assert.equal(result.stderr.trim(), "");

  const report = JSON.parse(result.stdout);
  assert.equal(report.schema, "dx.nextRustMerge.readinessAudit");
  assert.equal(
    report.auditSource,
    path.join(repoRoot, "tools/next-rust-merge/readiness-audit-baseline.json"),
  );
  assert.equal(report.auditedOverallScore, 46);
  assert.equal(report.auditedDxBuildGraphIntegrationScore, 24);
  assert.equal(report.reconciledScore, 46);
  assert.equal(report.coordinatorEvidence.executionMode, "preflight");
});

test("Lane 14 coordinator CLI emits readiness audit from an explicit audit file", () => {
  const auditDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-next-rust-audit-cli-"));
  const auditPath = path.join(auditDir, "audit.json");
  fs.writeFileSync(
    auditPath,
    `${JSON.stringify({
      source: "coordinator CLI fixture",
      overallScore: 39,
      dxBuildGraphIntegrationScore: 18,
      criticalGaps: [
        {
          id: "missing-app-server-data",
          lane: 12,
          severity: "blocking",
          status: "unresolved",
          evidence: "fixture missing server-data.json",
          nextAction: "write source-owned server data",
        },
      ],
      nextHighestImpactAction: "fix fixture App Router output",
    })}\n`,
  );

  const result = spawnSync(
    process.execPath,
    [
      "tools/next-rust-merge/coordinator-runner.cjs",
      "--audit",
      "--audit-file",
      auditPath,
      "--json",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      windowsHide: true,
    },
  );

  assert.equal(result.status, 1);
  assert.equal(result.stderr.trim(), "");

  const report = JSON.parse(result.stdout);
  assert.equal(report.schema, "dx.nextRustMerge.readinessAudit");
  assert.equal(report.auditSource, auditPath);
  assert.equal(report.auditedOverallScore, 39);
  assert.equal(report.auditedDxBuildGraphIntegrationScore, 18);
  assert.equal(report.reconciledScore, 39);
  assert.deepEqual(report.criticalGaps.map((entry) => entry.id), [
    "missing-app-server-data",
  ]);
  assert.equal(report.coordinatorEvidence.executionMode, "preflight");
});

test("Lane 14 coordinator CLI emits readiness audit without executing checks", () => {
  const result = spawnSync(
    process.execPath,
    ["tools/next-rust-merge/coordinator-runner.cjs", "--audit", "--json"],
    {
      cwd: repoRoot,
      encoding: "utf8",
      windowsHide: true,
    },
  );

  assert.equal(result.status, 1);
  assert.equal(result.stderr.trim(), "");
  assert.equal(result.stdout.includes("TAP version"), false);

  const report = JSON.parse(result.stdout);
  assert.equal(report.schema, "dx.nextRustMerge.readinessAudit");
  assert.equal(report.status, "blocked");
  assert.equal(report.reconciledScore, 46);
  assert.equal(report.coordinatorEvidence.executionMode, "preflight");
  assert.equal(report.coordinatorEvidence.score, null);
  assert.equal(report.receiptWritePolicy.executesChecks, false);
  assert.equal(report.receiptWritePolicy.mayWriteTempReceipts, false);
});

test("Lane 14 coordinator can emit compact failure-only scorecards", () => {
  const checks = [
    coordinatorFixtureCheck("passing-source-guard", true),
    coordinatorFixtureCheck("warning-source-guard", false),
    coordinatorFixtureCheck("blocking-source-guard", true),
  ];
  const outcomes = new Map([
    ["passing-source-guard", { status: 0, stdout: "ok", stderr: "", signal: null }],
    ["warning-source-guard", { status: 1, stdout: "", stderr: "non-blocking drift", signal: null }],
    ["blocking-source-guard", { status: 1, stdout: "", stderr: "blocking drift", signal: null }],
  ]);
  let tick = 0;

  const report = runCoordinatorChecks({
    checks,
    cwd: repoRoot,
    clock: () => (tick += 5),
    runCommand: (command) => outcomes.get(command[2]),
    startedAt: "2026-05-23T00:00:00.000Z",
  });
  const compact = compactCoordinatorFailures(report);

  assert.equal(compact.schema, FAILURES_SCHEMA);
  assert.equal(compact.generatedFromSchema, RUN_SCHEMA);
  assert.equal(compact.lane, 14);
  assert.equal(compact.featureImplementation, false);
  assert.equal(compact.status, "blocked");
  assert.equal(compact.score, 84);
  assert.equal(compact.failureCount, 2);
  assert.equal(compact.blockingFailureCount, 1);
  assert.deepEqual(
    compact.failures.map((entry) => entry.id),
    ["warning-source-guard", "blocking-source-guard"],
  );
  assert.equal(compact.failures[0].failureSummary, "non-blocking drift");
  assert.equal(compact.failures[1].failureSummary, "blocking drift");
  assert.equal(compact.failures[1].blocking, true);
  assert.deepEqual(compact.failures[1].sideEffects, {
    workspaceWrites: false,
    tempFixtureWrites: false,
    writesReceipts: false,
    receiptPaths: [],
    note: "fixture check is read-only",
  });
  assert.equal(compact.architecture.dxRuntimeAuthoritative, true);
  assert.equal(compact.architecture.publicTurbopackDependency, false);
  assert.equal(compact.architecture.reactRequiredCore, false);
  assert.equal(compact.architecture.nodeModulesRequired, false);
  assert.equal("stdoutPreview" in compact.failures[0], false);
  assert.equal("stderrPreview" in compact.failures[0], false);
});

test("Lane 14 coordinator preflight reports receipt writes without running checks", () => {
  const preflightModulePath = path.join(
    repoRoot,
    "tools",
    "next-rust-merge",
    "coordinator-preflight.cjs",
  );
  assert.ok(fs.existsSync(preflightModulePath), "missing coordinator preflight module");

  const {
    PREFLIGHT_SCHEMA,
    buildCoordinatorPreflightReport,
  } = require("../tools/next-rust-merge/coordinator-preflight.cjs");
  const checks = COORDINATOR_CHECKS.slice(0, 2);
  const report = buildCoordinatorPreflightReport({
    checks,
    generatedAt: "2026-05-23T00:00:00.000Z",
  });

  assert.equal(report.schema, PREFLIGHT_SCHEMA);
  assert.equal(report.lane, 14);
  assert.equal(report.laneName, "Final Coordinator");
  assert.equal(report.featureImplementation, false);
  assert.equal(report.status, "not-run");
  assert.equal(report.generatedAt, "2026-05-23T00:00:00.000Z");
  assert.deepEqual(report.totals, {
    total: 2,
    blocking: 2,
    nonBlocking: 0,
    receiptWriting: 2,
    readOnly: 0,
  });
  assert.deepEqual(report.sideEffectSummary, {
    workspaceWriteCheckCount: 0,
    tempFixtureWriteCheckCount: 2,
    readOnlyCheckCount: 0,
    receiptWritingCheckIds: ["vendor-boundary", "turbo-tasks-adapter"],
  });
  assert.deepEqual(report.architecture, {
    dxRuntimeAuthoritative: true,
    publicTurbopackDependency: false,
    reactRequiredCore: false,
    nodeModulesRequired: false,
    nodeNapiFoundation: false,
  });
  assert.deepEqual(
    report.checks.map((entry) => [entry.id, entry.status, entry.willRun]),
    [
      ["vendor-boundary", "not-run", false],
      ["turbo-tasks-adapter", "not-run", false],
    ],
  );
  assert.equal(report.checks[0].command, "node --test benchmarks/next-rust-vendor-boundary.test.ts");
  assert.deepEqual(report.checks[0].sideEffects, checks[0].sideEffects);
  assert.equal("stdoutPreview" in report.checks[0], false);
  assert.equal("durationMs" in report.checks[0], false);
});

test("Lane 14 coordinator CLI preflight does not execute selected checks", () => {
  const result = spawnSync(
    process.execPath,
    [
      "tools/next-rust-merge/coordinator-runner.cjs",
      "--preflight",
      "--json",
      "--only",
      "vendor-boundary",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      windowsHide: true,
    },
  );

  assert.equal(result.status, 0);
  assert.equal(result.stderr.trim(), "");

  const report = JSON.parse(result.stdout);
  assert.equal(report.schema, "dx.nextRustMerge.coordinatorPreflight");
  assert.equal(report.status, "not-run");
  assert.deepEqual(report.totals, {
    total: 1,
    blocking: 1,
    nonBlocking: 0,
    receiptWriting: 1,
    readOnly: 0,
  });
  assert.equal(report.checks[0].id, "vendor-boundary");
  assert.equal(report.checks[0].status, "not-run");
  assert.equal(report.checks[0].willRun, false);
  assert.equal(result.stdout.includes("TAP version"), false);
});

test("Lane 14 coordinator write mode persists only to an explicit caller path", () => {
  const args = parseArgs([
    "--run",
    "--json",
    "--failures-only",
    "--only",
    "vendor-boundary",
    "--write",
    "custom/scorecard.json",
  ]);
  assert.deepEqual(args, {
    audit: false,
    auditBaseline: false,
    auditFile: null,
    compareAuditBaseline: false,
    confirmedOpenOnly: false,
    run: true,
    json: true,
    preflight: false,
    failuresOnly: true,
    only: "vendor-boundary",
    writePath: "custom/scorecard.json",
    writeDefault: false,
  });

  const outputDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-next-rust-scorecard-"));
  const targetPath = path.join(outputDir, "nested", "scorecard.json");
  const report = {
    schema: RUN_SCHEMA,
    lane: 14,
    laneName: "Final Coordinator",
    featureImplementation: false,
    status: "warning",
    score: 96,
    totals: {
      total: 1,
      passed: 0,
      failed: 1,
      blockingFailed: 0,
      nonBlockingFailed: 1,
    },
    architecture: {
      dxRuntimeAuthoritative: true,
      publicTurbopackDependency: false,
      reactRequiredCore: false,
      nodeModulesRequired: false,
      nodeNapiFoundation: false,
    },
    checks: [],
  };

  const written = writeCoordinatorReport(report, targetPath, { cwd: repoRoot });
  const persisted = JSON.parse(fs.readFileSync(targetPath, "utf8"));

  assert.equal(written.receiptPath, targetPath);
  assert.equal(persisted.receiptPath, targetPath);
  assert.equal(persisted.schema, RUN_SCHEMA);
  assert.equal(persisted.score, 96);
  assert.equal(fs.existsSync(path.join(repoRoot, ".dx", "receipts", "next-rust", "coordinator-run.json")), false);
});

test("Lane 14 coordinator default receipt path is conventional but opt-in", () => {
  assert.equal(
    DEFAULT_COORDINATOR_RECEIPT_PATH,
    ".dx/receipts/next-rust/coordinator-scorecard.json",
  );
  assert.equal(resolveCoordinatorReceiptPath(), null);
  assert.equal(
    resolveCoordinatorReceiptPath({ writeDefault: true }),
    DEFAULT_COORDINATOR_RECEIPT_PATH,
  );
  assert.equal(
    resolveCoordinatorReceiptPath({
      writePath: "custom/scorecard.json",
      writeDefault: true,
    }),
    "custom/scorecard.json",
  );

  const withoutWrite = parseArgs(["--run", "--json"]);
  assert.equal(withoutWrite.writePath, null);
  assert.equal(withoutWrite.writeDefault, false);
  assert.equal(withoutWrite.failuresOnly, false);
  assert.equal(withoutWrite.confirmedOpenOnly, false);

  const withDefault = parseArgs(["--run", "--json", "--write-default"]);
  assert.equal(withDefault.writePath, DEFAULT_COORDINATOR_RECEIPT_PATH);
  assert.equal(withDefault.writeDefault, true);
  assert.equal(withDefault.failuresOnly, false);

  const failuresOnly = parseArgs(["--run", "--json", "--failures-only"]);
  assert.equal(failuresOnly.writePath, null);
  assert.equal(failuresOnly.failuresOnly, true);

  const confirmedOpenOnly = parseArgs([
    "--compare-audit-baseline",
    "--json",
    "--confirmed-open-only",
  ]);
  assert.equal(confirmedOpenOnly.confirmedOpenOnly, true);
});

test("Lane 14 coordinator CLI emits compact confirmed-open audit gaps", () => {
  const result = spawnSync(
    process.execPath,
    [
      "tools/next-rust-merge/coordinator-runner.cjs",
      "--compare-audit-baseline",
      "--json",
      "--confirmed-open-only",
      "--only",
      "giant-cli-mod",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      windowsHide: true,
    },
  );

  assert.equal(result.status, 1);
  assert.equal(result.stderr.trim(), "");

  const report = JSON.parse(result.stdout);
  assert.equal(report.schema, "dx.nextRustMerge.auditComparisonOpenGaps");
  assert.equal(report.generatedFromSchema, "dx.nextRustMerge.auditComparison");
  assert.equal(report.status, "needs-review");
  assert.equal(report.confirmedOpenCount, 1);
  assert.equal(report.unmappedBlockingFailureCount, 0);
  assert.deepEqual(report.openGaps.map((entry) => entry.id), ["giant-cli-mod"]);
  assert.equal(report.openGaps[0].confirmsOpenWhenPassed, true);
  assert.equal(report.openGaps[0].passedCheckIds[0], "giant-cli-mod");
  assert.deepEqual(report.openGaps[0].owner, {
    lane: "cross-cutting",
    action: "extract any touched CLI behavior into small modules before adding more CLI code",
    requiresFeatureImplementation: true,
    coordinatorRole: "track-and-score",
  });
  assert.equal("gaps" in report, false);
  assert.equal("checks" in report, false);
});

test("Lane 14 coordinator rejects unknown --only ids instead of running zero checks", () => {
  let capturedError;
  assert.throws(
    () => {
      resolveCoordinatorChecks({
        checks: COORDINATOR_CHECKS,
        only: "missing-lane-check",
      });
    },
    (error) =>
      error.code === "DX_NEXT_RUST_UNKNOWN_CHECK" &&
      error.unknownCheckId === "missing-lane-check" &&
      error.availableCheckIds.includes("vendor-boundary") &&
      error.availableCheckIds.includes("build-readiness-gate"),
  );

  try {
    resolveCoordinatorChecks({
      checks: COORDINATOR_CHECKS,
      only: "missing-lane-check",
    });
  } catch (error) {
    capturedError = error;
  }

  assert.ok(capturedError, "unknown check should throw a structured error");
  const argumentError = buildCoordinatorArgumentError(capturedError);

  assert.equal(argumentError.schema, ARGUMENT_ERROR_SCHEMA);
  assert.equal(argumentError.status, "argument-error");
  assert.equal(argumentError.exitCode, 2);
  assert.equal(argumentError.requestedCheckId, "missing-lane-check");
  assert.equal(argumentError.architecture.dxRuntimeAuthoritative, true);
  assert.equal(argumentError.architecture.publicTurbopackDependency, false);
  assert.equal(argumentError.architecture.reactRequiredCore, false);
  assert.equal(argumentError.architecture.nodeModulesRequired, false);
});

test("Lane 14 coordinator CLI reports unknown --only ids as JSON errors", () => {
  const result = spawnSync(
    process.execPath,
    [
      "tools/next-rust-merge/coordinator-runner.cjs",
      "--run",
      "--json",
      "--only",
      "missing-lane-check",
    ],
    {
      cwd: repoRoot,
      encoding: "utf8",
      windowsHide: true,
    },
  );

  assert.equal(result.status, 2);
  assert.equal(result.stderr.trim(), "");

  const report = JSON.parse(result.stdout);
  assert.equal(report.schema, ARGUMENT_ERROR_SCHEMA);
  assert.equal(report.status, "argument-error");
  assert.equal(report.requestedCheckId, "missing-lane-check");
  assert.match(report.message, /Unknown coordinator check id/);
  assert.ok(report.availableCheckIds.includes("vendor-boundary"));
  assert.ok(report.availableCheckIds.includes("build-readiness-gate"));
});

function coordinatorFixtureCheck(id, blocking) {
  return {
    id,
    lane: 14,
    boundary: "coordinator source-only gate",
    proves: ["scorecard behavior"],
    blocking,
    heavy: false,
    publicTurbopackDependency: false,
    requiresReactCore: false,
    requiresNodeModules: false,
    sideEffects: {
      workspaceWrites: false,
      tempFixtureWrites: false,
      writesReceipts: false,
      receiptPaths: [],
      note: "fixture check is read-only",
    },
    command: ["node", "--test", id],
  };
}
