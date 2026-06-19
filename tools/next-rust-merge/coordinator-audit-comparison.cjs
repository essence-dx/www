const { normalizeReadinessAuditInput } = require("./coordinator-readiness-audit.cjs");
const {
  coordinatorArchitecture,
  coordinatorReceiptWritePolicy,
} = require("./coordinator-report-contract.cjs");
const fs = require("node:fs");
const path = require("node:path");

const AUDIT_COMPARISON_SCHEMA = "dx.nextRustMerge.auditComparison";
const AUDIT_COMPARISON_OPEN_GAPS_SCHEMA =
  "dx.nextRustMerge.auditComparisonOpenGaps";
const DEFAULT_AUDIT_GAP_CHECK_MAP_PATH =
  "tools/next-rust-merge/audit-gap-check-map.json";

const DEFAULT_AUDIT_GAP_CHECK_MAP = Object.freeze({
  source: "inline default audit gap check map",
  gapMappings: [
    {
      id: "dx-build-graph-integration",
      reason:
        "Tracks DX-owned build graph integration checks while keeping vendored Turbopack ideas reference-only.",
      checkIds: [
        "turbo-tasks-adapter",
        "turbopack-core-map",
        "turbopack-core-graph",
        "next-custom-transforms",
      ],
    },
    {
      id: "giant-cli-mod",
      reason:
        "Tracks the read-only CLI split-risk reporter until dx-www/src/cli/mod.rs is split.",
      checkIds: ["giant-cli-mod"],
      confirmsOpenWhenPassed: true,
    },
    {
      id: "dx-style-drift-fixture",
      reason: "Tracks the Lane 5 dx-style drift fixture source guard.",
      checkIds: ["dx-style-drift-fixture"],
    },
    {
      id: "missing-app-server-data",
      reason: "Tracks the Lane 12 App Router server-data build contract check.",
      checkIds: ["app-router-server-data"],
    },
    {
      id: "hmr-polling-only",
      reason: "Tracks the Lane 8 hot reload protocol source guard.",
      checkIds: ["dev-hot-reload-protocol"],
    },
    {
      id: "default-www-template",
      reason: "Tracks the Lane 13 default WWW template source contract.",
      checkIds: ["default-www-template"],
    },
    {
      id: "build-readiness-gate",
      reason: "Tracks the Lane 14 build readiness source/product proof gate.",
      checkIds: ["build-readiness-gate"],
    },
    {
      id: "docs-status-overclaims",
      reason: "Tracks honest Next familiarity limits through the compatibility map.",
      checkIds: ["next-compatibility-map"],
    },
    {
      id: "schema-status-noise",
      reason: "Tracks public schema suffixes and generated/status overclaims.",
      checkIds: ["schema-status-noise"],
    },
  ],
});

function compareAuditBaselineToCoordinatorRun({
  auditInput,
  coordinatorReport,
  gapCheckMap = readDefaultAuditGapCheckMap(),
  generatedAt = new Date().toISOString(),
} = {}) {
  if (!coordinatorReport || typeof coordinatorReport !== "object") {
    throw comparisonInputError("coordinatorReport is required for audit comparison");
  }

  const normalizedAudit = normalizeReadinessAuditInput(auditInput);
  const checks = Array.isArray(coordinatorReport.checks)
    ? coordinatorReport.checks
    : [];
  const checksById = new Map(checks.map((entry) => [entry.id, entry]));
  const normalizedMap = normalizeAuditGapCheckMap(gapCheckMap);
  const gaps = normalizedAudit.criticalGaps.map((gap) =>
    compareGapToChecks(gap, checksById, normalizedMap),
  );
  const unmappedBlockingFailures = findUnmappedBlockingFailures(checks, normalizedMap);
  const counts = countGapStatuses(gaps, { unmappedBlockingFailures });

  return {
    schema: AUDIT_COMPARISON_SCHEMA,
    lane: 14,
    laneName: "Final Coordinator",
    featureImplementation: false,
    status: hasReviewWork(counts) ? "needs-review" : "aligned",
    executionMode: "audit-comparison",
    proofLevel: "read-only-baseline-run-comparison",
    generatedAt,
    baselineSource: normalizedAudit.source,
    baselineScore: normalizedAudit.overallScore,
    baselineDxBuildGraphIntegrationScore: normalizedAudit.dxBuildGraphIntegrationScore,
    coordinatorEvidence: summarizeCoordinatorEvidence(coordinatorReport),
    architecture: coordinatorReport.architecture || coordinatorArchitecture(),
    receiptWritePolicy:
      coordinatorReport.receiptWritePolicy ||
      coordinatorReceiptWritePolicy({
        checks,
        executionMode: "run",
        writesDefaultScorecard: false,
      }),
    counts,
    gaps,
    unmappedBlockingFailures,
    nextHighestImpactAction: normalizedAudit.nextHighestImpactAction,
  };
}

function compactAuditComparisonOpenGaps(report) {
  const gaps = Array.isArray(report?.gaps) ? report.gaps : [];
  const counts = report?.counts || countGapStatuses(gaps);
  const openGaps = gaps
    .filter((entry) => entry.comparisonStatus === "confirmed-open")
    .map(compactConfirmedOpenGap);

  return {
    schema: AUDIT_COMPARISON_OPEN_GAPS_SCHEMA,
    generatedFromSchema: report?.schema || AUDIT_COMPARISON_SCHEMA,
    lane: report?.lane ?? 14,
    laneName: report?.laneName || "Final Coordinator",
    featureImplementation: false,
    status: report?.status || "needs-review",
    executionMode: "audit-comparison-open-gaps",
    proofLevel: report?.proofLevel || "read-only-baseline-run-comparison",
    generatedAt: report?.generatedAt || null,
    baselineSource: report?.baselineSource || null,
    baselineScore: report?.baselineScore ?? null,
    baselineDxBuildGraphIntegrationScore:
      report?.baselineDxBuildGraphIntegrationScore ?? null,
    coordinatorEvidence: report?.coordinatorEvidence || null,
    architecture: report?.architecture || coordinatorArchitecture(),
    receiptWritePolicy:
      report?.receiptWritePolicy ||
      coordinatorReceiptWritePolicy({
        checks: [],
        executionMode: "run",
        writesDefaultScorecard: false,
      }),
    counts,
    confirmedOpenCount: openGaps.length,
    possiblyStaleCount: counts.possiblyStale || 0,
    partialEvidenceCount: counts.partialEvidence || 0,
    unmappedBlockingFailureCount: counts.unmappedBlockingFailures || 0,
    openGaps,
    unmappedBlockingFailures: Array.isArray(report?.unmappedBlockingFailures)
      ? report.unmappedBlockingFailures
      : [],
    nextHighestImpactAction: report?.nextHighestImpactAction || null,
  };
}

function compactConfirmedOpenGap(entry) {
  return {
    id: entry.id,
    lane: entry.lane,
    severity: entry.severity,
    baselineStatus: entry.baselineStatus,
    baselineScore: entry.baselineScore ?? null,
    comparisonStatus: entry.comparisonStatus,
    failedCheckIds: [...(entry.failedCheckIds || [])],
    passedCheckIds: [...(entry.passedCheckIds || [])],
    confirmsOpenWhenPassed: Boolean(entry.confirmsOpenWhenPassed),
    mappingReason: entry.mappingReason || null,
    evidence: entry.evidence,
    nextAction: entry.nextAction,
    owner: compactGapOwner(entry),
  };
}

function compactGapOwner(entry) {
  const lane14Owned = entry.lane === 14 || entry.lane === "14";

  return {
    lane: entry.lane,
    action: entry.nextAction || null,
    requiresFeatureImplementation: !lane14Owned,
    coordinatorRole: lane14Owned ? "own-and-score" : "track-and-score",
  };
}

function compareGapToChecks(gap, checksById, gapCheckMap) {
  const mapping = gapCheckMap.byId.get(gap.id);
  const mappedCheckIds = mapping?.checkIds ? [...mapping.checkIds] : [];
  const untrackedReason = mapping?.untrackedReason || null;
  const confirmsOpenWhenPassed = mapping?.confirmsOpenWhenPassed || false;
  const passedCheckIds = [];
  const failedCheckIds = [];
  const missingCheckIds = [];

  for (const checkId of mappedCheckIds) {
    const check = checksById.get(checkId);
    if (!check) {
      missingCheckIds.push(checkId);
    } else if (check.status === "passed") {
      passedCheckIds.push(checkId);
    } else {
      failedCheckIds.push(checkId);
    }
  }

  return {
    id: gap.id,
    lane: gap.lane,
    severity: gap.severity,
    baselineStatus: gap.status,
    baselineScore: gap.score ?? null,
    comparisonStatus: gapComparisonStatus({
      mappedCheckIds,
      failedCheckIds,
      missingCheckIds,
      untrackedReason,
      confirmsOpenWhenPassed,
    }),
    mappedCheckIds,
    passedCheckIds,
    failedCheckIds,
    missingCheckIds,
    untrackedReason,
    confirmsOpenWhenPassed,
    mappingReason: mapping?.reason || null,
    evidence: gap.evidence,
    nextAction: gap.nextAction,
  };
}

function gapComparisonStatus({
  mappedCheckIds,
  failedCheckIds,
  missingCheckIds,
  untrackedReason,
  confirmsOpenWhenPassed,
}) {
  if (untrackedReason) return "intentionally-untracked";
  if (mappedCheckIds.length === 0) return "untracked-by-coordinator";
  if (failedCheckIds.length > 0) return "confirmed-open";
  if (missingCheckIds.length > 0) return "partial-evidence";
  if (confirmsOpenWhenPassed) return "confirmed-open";
  return "possibly-stale";
}

function countGapStatuses(gaps, { unmappedBlockingFailures = [] } = {}) {
  return {
    totalGaps: gaps.length,
    confirmedOpen: gaps.filter((entry) => entry.comparisonStatus === "confirmed-open")
      .length,
    possiblyStale: gaps.filter((entry) => entry.comparisonStatus === "possibly-stale")
      .length,
    partialEvidence: gaps.filter((entry) => entry.comparisonStatus === "partial-evidence")
      .length,
    untracked: gaps.filter((entry) => entry.comparisonStatus === "untracked-by-coordinator")
      .length,
    intentionallyUntracked: gaps.filter(
      (entry) => entry.comparisonStatus === "intentionally-untracked",
    ).length,
    missingCoordinatorChecks: gaps.reduce(
      (total, entry) => total + entry.missingCheckIds.length,
      0,
    ),
    unmappedBlockingFailures: unmappedBlockingFailures.length,
  };
}

function findUnmappedBlockingFailures(checks, gapCheckMap) {
  const mappedCheckIds = new Set(
    gapCheckMap.gapMappings.flatMap((entry) => entry.checkIds || []),
  );

  return checks
    .filter((entry) => {
      return (
        entry &&
        entry.blocking === true &&
        entry.status &&
        entry.status !== "passed" &&
        !mappedCheckIds.has(entry.id)
      );
    })
    .map(summarizeUnmappedBlockingFailure);
}

function summarizeUnmappedBlockingFailure(entry) {
  return {
    id: entry.id,
    lane: entry.lane,
    boundary: entry.boundary,
    command: entry.command,
    status: entry.status,
    exitCode: entry.exitCode ?? null,
    failureSummary: entry.failureSummary || null,
    proves: Array.isArray(entry.proves) ? [...entry.proves] : [],
  };
}

function readDefaultAuditGapCheckMap({ cwd = process.cwd() } = {}) {
  return readAuditGapCheckMap(DEFAULT_AUDIT_GAP_CHECK_MAP_PATH, { cwd });
}

function readAuditGapCheckMap(filePath, { cwd = process.cwd() } = {}) {
  if (!filePath || typeof filePath !== "string") {
    throw comparisonInputError("audit gap check map requires a JSON file path");
  }

  const resolvedPath = path.resolve(cwd, filePath);
  let parsed;
  try {
    parsed = JSON.parse(fs.readFileSync(resolvedPath, "utf8"));
  } catch (error) {
    throw comparisonInputError(
      `Unable to read audit gap check map "${resolvedPath}": ${error.message}`,
    );
  }

  return normalizeAuditGapCheckMap(parsed, { source: resolvedPath });
}

function normalizeAuditGapCheckMap(input, { source = null } = {}) {
  if (input && input.byId instanceof Map) return input;

  if (input && !Array.isArray(input) && typeof input === "object") {
    if (Array.isArray(input.gapMappings)) {
      const gapMappings = input.gapMappings.map(normalizeGapMapping);
      return {
        source: source || requiredString(input.source, "source"),
        gapMappings,
        byId: new Map(gapMappings.map((entry) => [entry.id, entry])),
      };
    }

    return normalizeAuditGapCheckMap(legacyGapCheckMap(Object.entries(input)), {
      source: source || "inline legacy gap check map",
    });
  }

  throw comparisonInputError("audit gap check map must be a JSON object");
}

function normalizeGapMapping(entry, index) {
  if (!entry || typeof entry !== "object" || Array.isArray(entry)) {
    throw comparisonInputError(`gapMappings[${index}] must be an object`);
  }

  const checkIds = Array.isArray(entry.checkIds)
    ? entry.checkIds.map((value, checkIndex) =>
        requiredString(value, `gapMappings[${index}].checkIds[${checkIndex}]`),
      )
    : [];
  const untrackedReason =
    entry.untrackedReason === undefined
      ? null
      : requiredString(entry.untrackedReason, `gapMappings[${index}].untrackedReason`);

  if ((checkIds.length > 0) === Boolean(untrackedReason)) {
    throw comparisonInputError(
      `gapMappings[${index}] must define checkIds or untrackedReason, not both`,
    );
  }

  return {
    id: requiredString(entry.id, `gapMappings[${index}].id`),
    reason: requiredString(entry.reason, `gapMappings[${index}].reason`),
    checkIds,
    untrackedReason,
    confirmsOpenWhenPassed: Boolean(entry.confirmsOpenWhenPassed),
  };
}

function legacyGapCheckMap(entries) {
  return {
    source: "inline legacy gap check map",
    gapMappings: entries.map(([id, checkIds]) => ({
      id,
      reason: "legacy inline check mapping",
      checkIds: [...checkIds],
    })),
  };
}

function requiredString(value, fieldName) {
  if (typeof value !== "string" || value.trim().length === 0) {
    throw comparisonInputError(`${fieldName} must be a non-empty string`);
  }
  return value;
}

function hasReviewWork(counts) {
  return (
    counts.confirmedOpen > 0 ||
    counts.possiblyStale > 0 ||
    counts.partialEvidence > 0 ||
    counts.untracked > 0 ||
    counts.unmappedBlockingFailures > 0
  );
}

function summarizeCoordinatorEvidence(report) {
  return {
    schema: report.schema || null,
    executionMode: report.executionMode || "unknown",
    proofLevel: report.proofLevel || "unknown",
    status: report.status || "unknown",
    score: Number.isFinite(report.score) ? report.score : null,
    totals: report.totals || null,
  };
}

function comparisonInputError(message) {
  const error = new Error(message);
  error.code = "DX_NEXT_RUST_AUDIT_COMPARISON_INPUT";
  return error;
}

module.exports = {
  AUDIT_COMPARISON_SCHEMA,
  AUDIT_COMPARISON_OPEN_GAPS_SCHEMA,
  DEFAULT_AUDIT_GAP_CHECK_MAP_PATH,
  DEFAULT_AUDIT_GAP_CHECK_MAP,
  compactAuditComparisonOpenGaps,
  compareAuditBaselineToCoordinatorRun,
  normalizeAuditGapCheckMap,
  readAuditGapCheckMap,
  readDefaultAuditGapCheckMap,
};
