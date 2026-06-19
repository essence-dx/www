import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const { RUN_SCHEMA } = require("../tools/next-rust-merge/coordinator-runner.cjs");
const { COORDINATOR_CHECKS } = require("../tools/next-rust-merge/coordinator-checks.cjs");
const {
  AUDIT_COMPARISON_SCHEMA,
  AUDIT_COMPARISON_OPEN_GAPS_SCHEMA,
  DEFAULT_AUDIT_GAP_CHECK_MAP,
  DEFAULT_AUDIT_GAP_CHECK_MAP_PATH,
  compactAuditComparisonOpenGaps,
  compareAuditBaselineToCoordinatorRun,
  normalizeAuditGapCheckMap,
  readDefaultAuditGapCheckMap,
} = require("../tools/next-rust-merge/coordinator-audit-comparison.cjs");

test("Lane 14 audit vocabulary removes Turbopack adoption as an active target", () => {
  const scopedFiles = [
    "tools/next-rust-merge/coordinator-readiness-audit.cjs",
    "tools/next-rust-merge/coordinator-audit-comparison.cjs",
    "tools/next-rust-merge/audit-gap-check-map.json",
    "tools/next-rust-merge/readiness-audit-baseline.json",
  ];

  for (const relativePath of scopedFiles) {
    const source = fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
    assert.doesNotMatch(source, /build-brain-adoption/);
    assert.doesNotMatch(source, /buildBrainAdoptionScore/);
    assert.doesNotMatch(source, /real Next\/Turbopack Rust build-brain adoption/i);
    assert.doesNotMatch(source, /land executable build-brain proofs/i);
    assert.match(source, /dx-build-graph-integration|dxBuildGraphIntegrationScore/);
  }
});

test("Lane 14 audit gap check map covers every baseline gap explicitly", () => {
  const mapPath = path.join(repoRoot, DEFAULT_AUDIT_GAP_CHECK_MAP_PATH);
  const baselinePath = path.join(
    repoRoot,
    "tools/next-rust-merge/readiness-audit-baseline.json",
  );
  const mapping = readDefaultAuditGapCheckMap({ cwd: repoRoot });
  const baseline = JSON.parse(fs.readFileSync(baselinePath, "utf8"));
  const baselineGapIds = baseline.criticalGaps.map((entry) => entry.id).sort();
  const mappedGapIds = mapping.gapMappings.map((entry) => entry.id).sort();
  const registeredCheckIds = new Set(COORDINATOR_CHECKS.map((entry) => entry.id));

  assert.equal(
    DEFAULT_AUDIT_GAP_CHECK_MAP_PATH,
    "tools/next-rust-merge/audit-gap-check-map.json",
  );
  assert.ok(fs.existsSync(mapPath), "missing audit gap check map");
  assert.deepEqual(mappedGapIds, baselineGapIds);
  assert.equal(mapping.source, mapPath);
  assert.doesNotMatch(fs.readFileSync(mapPath, "utf8"), /\.v1\b/);
  assert.deepEqual(mapping.gapMappings.find((entry) => entry.id === "schema-status-noise").checkIds, [
    "schema-status-noise",
  ]);
  assert.deepEqual(mapping.gapMappings.find((entry) => entry.id === "giant-cli-mod").checkIds, [
    "giant-cli-mod",
  ]);
  assert.equal(
    mapping.gapMappings.find((entry) => entry.id === "giant-cli-mod").confirmsOpenWhenPassed,
    true,
  );

  for (const entry of mapping.gapMappings) {
    const hasChecks = Array.isArray(entry.checkIds) && entry.checkIds.length > 0;
    const hasUntrackedReason =
      typeof entry.untrackedReason === "string" &&
      entry.untrackedReason.trim().length > 0;

    assert.notEqual(
      hasChecks,
      hasUntrackedReason,
      `${entry.id} should have checkIds or untrackedReason, not both`,
    );
    assert.equal(typeof entry.reason, "string", `${entry.id} needs a reason`);
    for (const checkId of entry.checkIds || []) {
      assert.ok(
        registeredCheckIds.has(checkId),
        `${entry.id} references unregistered coordinator check ${checkId}`,
      );
    }
  }
});

test("Lane 14 inline audit gap map stays aligned with the checked source map", () => {
  const baselinePath = path.join(
    repoRoot,
    "tools/next-rust-merge/readiness-audit-baseline.json",
  );
  const checkedMapping = readDefaultAuditGapCheckMap({ cwd: repoRoot });
  const inlineMapping = normalizeAuditGapCheckMap(DEFAULT_AUDIT_GAP_CHECK_MAP);
  const baseline = JSON.parse(fs.readFileSync(baselinePath, "utf8"));
  const expectedGapIds = baseline.criticalGaps.map((entry) => entry.id).sort();

  assert.deepEqual(
    inlineMapping.gapMappings.map((entry) => entry.id).sort(),
    expectedGapIds,
  );
  assert.deepEqual(
    inlineMapping.gapMappings.map((entry) => entry.id).sort(),
    checkedMapping.gapMappings.map((entry) => entry.id).sort(),
  );
  assert.deepEqual(
    inlineMapping.byId.get("dx-build-graph-integration").checkIds,
    [
      "turbo-tasks-adapter",
      "turbopack-core-map",
      "turbopack-core-graph",
      "next-custom-transforms",
    ],
  );
  assert.deepEqual(inlineMapping.byId.get("dx-style-drift-fixture").checkIds, [
    "dx-style-drift-fixture",
  ]);
  assert.deepEqual(inlineMapping.byId.get("default-www-template").checkIds, [
    "default-www-template",
  ]);
  assert.deepEqual(inlineMapping.byId.get("build-readiness-gate").checkIds, [
    "build-readiness-gate",
  ]);
  assert.equal(inlineMapping.byId.get("giant-cli-mod").confirmsOpenWhenPassed, true);
});

test("Lane 14 compares the readiness audit baseline against latest coordinator evidence", () => {
  const report = compareAuditBaselineToCoordinatorRun({
    auditInput: {
      source: "test baseline",
      overallScore: 46,
      dxBuildGraphIntegrationScore: 24,
      criticalGaps: [
        gap("dx-build-graph-integration", "2-11", "blocking"),
        gap("hmr-polling-only", 8, "blocking"),
        gap("dx-style-drift-fixture", 5, "blocking"),
        gap("missing-app-server-data", 12, "blocking"),
        gap("default-www-template", 13, "blocking"),
        gap("build-readiness-gate", 14, "blocking"),
        gap("giant-cli-mod", "cross-cutting", "risk"),
      ],
      nextHighestImpactAction: "keep the baseline honest",
    },
    coordinatorReport: coordinatorRunFixture(),
    generatedAt: "2026-05-23T00:00:00.000Z",
  });

  assert.equal(report.schema, AUDIT_COMPARISON_SCHEMA);
  assert.equal(report.lane, 14);
  assert.equal(report.laneName, "Final Coordinator");
  assert.equal(report.featureImplementation, false);
  assert.equal(report.status, "needs-review");
  assert.equal(report.executionMode, "audit-comparison");
  assert.equal(report.proofLevel, "read-only-baseline-run-comparison");
  assert.equal(report.generatedAt, "2026-05-23T00:00:00.000Z");
  assert.equal(report.baselineSource, "test baseline");
  assert.equal(report.coordinatorEvidence.schema, RUN_SCHEMA);
  assert.equal(report.coordinatorEvidence.score, 52);
  assert.equal(report.receiptWritePolicy.writesDefaultScorecard, false);
  assert.equal("receiptPath" in report, false);
  assert.deepEqual(report.counts, {
    totalGaps: 7,
    confirmedOpen: 6,
    possiblyStale: 1,
    partialEvidence: 0,
    untracked: 0,
    intentionallyUntracked: 0,
    missingCoordinatorChecks: 0,
    unmappedBlockingFailures: 0,
  });
  assert.deepEqual(report.unmappedBlockingFailures, []);

  const byId = new Map(report.gaps.map((entry) => [entry.id, entry]));
  assert.equal(byId.get("dx-build-graph-integration").comparisonStatus, "possibly-stale");
  assert.deepEqual(byId.get("dx-build-graph-integration").passedCheckIds, [
    "turbo-tasks-adapter",
    "turbopack-core-map",
    "turbopack-core-graph",
    "next-custom-transforms",
  ]);
  assert.equal(byId.get("hmr-polling-only").comparisonStatus, "confirmed-open");
  assert.deepEqual(byId.get("hmr-polling-only").failedCheckIds, [
    "dev-hot-reload-protocol",
  ]);
  assert.equal(byId.get("dx-style-drift-fixture").comparisonStatus, "confirmed-open");
  assert.deepEqual(byId.get("dx-style-drift-fixture").failedCheckIds, [
    "dx-style-drift-fixture",
  ]);
  assert.equal(byId.get("missing-app-server-data").comparisonStatus, "confirmed-open");
  assert.deepEqual(byId.get("missing-app-server-data").failedCheckIds, [
    "app-router-server-data",
  ]);
  assert.equal(byId.get("default-www-template").comparisonStatus, "confirmed-open");
  assert.deepEqual(byId.get("default-www-template").failedCheckIds, [
    "default-www-template",
  ]);
  assert.equal(byId.get("build-readiness-gate").comparisonStatus, "confirmed-open");
  assert.deepEqual(byId.get("build-readiness-gate").failedCheckIds, [
    "build-readiness-gate",
  ]);
  assert.equal(byId.get("giant-cli-mod").comparisonStatus, "confirmed-open");
  assert.deepEqual(byId.get("giant-cli-mod").passedCheckIds, ["giant-cli-mod"]);
  assert.equal(byId.get("giant-cli-mod").confirmsOpenWhenPassed, true);
});

test("Lane 14 compacts audit comparison to confirmed-open gaps only", () => {
  const report = compareAuditBaselineToCoordinatorRun({
    auditInput: {
      source: "test baseline",
      overallScore: 46,
      dxBuildGraphIntegrationScore: 24,
      criticalGaps: [
        gap("dx-build-graph-integration", "2-11", "blocking"),
        gap("hmr-polling-only", 8, "blocking"),
        gap("dx-style-drift-fixture", 5, "blocking"),
        gap("missing-app-server-data", 12, "blocking"),
        gap("default-www-template", 13, "blocking"),
        gap("build-readiness-gate", 14, "blocking"),
        gap("giant-cli-mod", "cross-cutting", "risk"),
      ],
      nextHighestImpactAction: "keep the baseline honest",
    },
    coordinatorReport: coordinatorRunFixture(),
    generatedAt: "2026-05-23T00:00:00.000Z",
  });
  const compact = compactAuditComparisonOpenGaps(report);

  assert.equal(compact.schema, AUDIT_COMPARISON_OPEN_GAPS_SCHEMA);
  assert.equal(compact.generatedFromSchema, AUDIT_COMPARISON_SCHEMA);
  assert.equal(compact.lane, 14);
  assert.equal(compact.featureImplementation, false);
  assert.equal(compact.status, "needs-review");
  assert.equal(compact.baselineScore, 46);
  assert.equal(compact.coordinatorEvidence.score, 52);
  assert.equal(compact.confirmedOpenCount, 6);
  assert.equal(compact.possiblyStaleCount, 1);
  assert.equal(compact.partialEvidenceCount, 0);
  assert.equal(compact.unmappedBlockingFailureCount, 0);
  assert.deepEqual(
    compact.openGaps.map((entry) => entry.id),
    [
      "hmr-polling-only",
      "dx-style-drift-fixture",
      "missing-app-server-data",
      "default-www-template",
      "build-readiness-gate",
      "giant-cli-mod",
    ],
  );
  assert.deepEqual(compact.openGaps[0], {
    id: "hmr-polling-only",
    lane: 8,
    severity: "blocking",
    baselineStatus: "under-proven",
    baselineScore: null,
    comparisonStatus: "confirmed-open",
    failedCheckIds: ["dev-hot-reload-protocol"],
    passedCheckIds: [],
    confirmsOpenWhenPassed: false,
    mappingReason:
      "Tracks the Lane 8 hot reload source guard that distinguishes polling/overlay from real HMR.",
    evidence: "hmr-polling-only fixture evidence",
    nextAction: "hmr-polling-only fixture action",
    owner: {
      lane: 8,
      action: "hmr-polling-only fixture action",
      requiresFeatureImplementation: true,
      coordinatorRole: "track-and-score",
    },
  });

  assert.deepEqual(
    compact.openGaps.find((entry) => entry.id === "build-readiness-gate").owner,
    {
      lane: 14,
      action: "build-readiness-gate fixture action",
      requiresFeatureImplementation: false,
      coordinatorRole: "own-and-score",
    },
  );
  assert.deepEqual(
    compact.openGaps.find((entry) => entry.id === "giant-cli-mod").owner,
    {
      lane: "cross-cutting",
      action: "giant-cli-mod fixture action",
      requiresFeatureImplementation: true,
      coordinatorRole: "track-and-score",
    },
  );
});

test("Lane 14 coordinator CLI compares the checked baseline without writing score noise", () => {
  const result = spawnSync(
    process.execPath,
    [
      "tools/next-rust-merge/coordinator-runner.cjs",
      "--compare-audit-baseline",
      "--json",
      "--only",
      "next-compatibility-map",
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
  assert.equal(report.schema, AUDIT_COMPARISON_SCHEMA);
  assert.equal(report.status, "needs-review");
  assert.equal(report.executionMode, "audit-comparison");
  assert.equal(
    report.baselineSource,
    path.join(repoRoot, "tools/next-rust-merge/readiness-audit-baseline.json"),
  );
  assert.equal(report.coordinatorEvidence.executionMode, "run");
  assert.equal(report.coordinatorEvidence.totals.total, 1);
  assert.equal(report.receiptWritePolicy.writesDefaultScorecard, false);
  assert.equal("receiptPath" in report, false);
  assert.equal(report.counts.untracked, 0);
  assert.equal(report.counts.intentionallyUntracked, 0);
  assert.equal(report.counts.partialEvidence, 8);
  assert.equal(report.counts.missingCoordinatorChecks, 11);
  assert.equal(report.counts.unmappedBlockingFailures, 0);
  assert.deepEqual(report.unmappedBlockingFailures, []);
  assert.equal(
    report.gaps.find((entry) => entry.id === "giant-cli-mod").comparisonStatus,
    "partial-evidence",
  );
});

function gap(id, lane, severity) {
  return {
    id,
    lane,
    severity,
    status: "under-proven",
    evidence: `${id} fixture evidence`,
    nextAction: `${id} fixture action`,
  };
}

function coordinatorRunFixture() {
  return {
    schema: RUN_SCHEMA,
    lane: 14,
    laneName: "Final Coordinator",
    featureImplementation: false,
    executionMode: "run",
    proofLevel: "executed-lightweight-source-checks",
    status: "blocked",
    score: 52,
    totals: {
      total: 10,
      passed: 5,
      failed: 5,
      blockingFailed: 5,
      nonBlockingFailed: 0,
    },
    receiptWritePolicy: {
      writesDefaultScorecard: false,
      executesChecks: true,
      mayWriteTempReceipts: false,
      receiptWritingCheckIds: [],
      note: "fixture does not write receipts",
    },
    checks: [
      check("turbo-tasks-adapter", "passed"),
      check("turbopack-core-map", "passed"),
      check("turbopack-core-graph", "passed"),
      check("next-custom-transforms", "passed"),
      check("giant-cli-mod", "passed"),
      check("app-router-server-data", "failed"),
      check("dev-hot-reload-protocol", "failed"),
      check("dx-style-drift-fixture", "failed", { lane: 5 }),
      check("default-www-template", "failed", { lane: 13 }),
      check("build-readiness-gate", "failed", { lane: 14 }),
    ],
  };
}

function check(id, status, { lane = 14 } = {}) {
  return {
    id,
    lane,
    boundary: "fixture",
    blocking: true,
    command: `node --test benchmarks/${id}.test.ts`,
    status,
    exitCode: status === "passed" ? 0 : 1,
    failureSummary: status === "passed" ? null : `${id} failed`,
    proves: ["fixture proof"],
    sideEffects: {
      workspaceWrites: false,
      tempFixtureWrites: false,
      writesReceipts: false,
      receiptPaths: [],
      note: "fixture check is read-only",
    },
  };
}
