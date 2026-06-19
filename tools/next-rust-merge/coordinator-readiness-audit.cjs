const {
  coordinatorArchitecture,
  coordinatorReceiptWritePolicy,
} = require("./coordinator-report-contract.cjs");
const fs = require("node:fs");
const path = require("node:path");

const READINESS_AUDIT_SCHEMA = "dx.nextRustMerge.readinessAudit";
const AUDITED_OVERALL_SCORE = 46;
const AUDITED_DX_BUILD_GRAPH_INTEGRATION_SCORE = 24;
const DEFAULT_AUDIT_SOURCE = "user-provided readiness audit summary";
const DEFAULT_READINESS_AUDIT_BASELINE_PATH =
  "tools/next-rust-merge/readiness-audit-baseline.json";

const CRITICAL_GAPS = Object.freeze([
  Object.freeze({
    id: "dx-build-graph-integration",
    lane: "2-11",
    severity: "blocking",
    score: AUDITED_DX_BUILD_GRAPH_INTEGRATION_SCORE,
    status: "under-proven",
    evidence:
      "DX-owned build graph integration still relies on reference/provenance and source guards before production dx build output uses it",
    nextAction:
      "wire DX-owned graph and invalidation evidence into source-build receipts and cache decisions without adopting Turbopack runtime/build execution",
  }),
  Object.freeze({
    id: "giant-cli-mod",
    lane: "cross-cutting",
    severity: "risk",
    status: "unresolved",
    evidence: "dx-www/src/cli/mod.rs remains huge and risky for merge work",
    nextAction: "extract any touched CLI behavior into small modules before adding more CLI code",
  }),
  Object.freeze({
    id: "dx-style-drift-fixture",
    lane: 5,
    severity: "blocking",
    status: "unresolved",
    evidence:
      "coordinator run reports dx-style drift fixture consumption as a blocking Lane 5 failure",
    nextAction:
      "fix the dx-style drift read model or demote this only after the focused source guard passes",
  }),
  Object.freeze({
    id: "missing-app-server-data",
    lane: 12,
    severity: "blocking",
    status: "unresolved",
    evidence: "App Router build contract still reports missing .dx/build/app/server-data.json",
    nextAction: "make App Router build output write and verify server-data.json from source-owned route data",
  }),
  Object.freeze({
    id: "hmr-polling-only",
    lane: 8,
    severity: "blocking",
    status: "under-proven",
    evidence: "dev server hot reload remains polling/overlay rather than real HMR",
    nextAction: "add a real source-owned hot-update path or keep the protocol explicitly marked as a gap",
  }),
  Object.freeze({
    id: "default-www-template",
    lane: 13,
    severity: "blocking",
    status: "unresolved",
    evidence:
      "coordinator run reports the default WWW template source contract as a blocking Lane 13 failure",
    nextAction:
      "fix the template source owner/materializer contract or update the source guard after the extraction lands",
  }),
  Object.freeze({
    id: "build-readiness-gate",
    lane: 14,
    severity: "blocking",
    status: "unresolved",
    evidence:
      "coordinator run reports the build readiness gate as a blocking Lane 14 source/product proof failure",
    nextAction:
      "fix the readiness gate command/proof contract before using it as product readiness evidence",
  }),
  Object.freeze({
    id: "docs-status-overclaims",
    lane: 14,
    severity: "risk",
    status: "active-risk",
    evidence: "docs/status surfaces have overstated proof compared with executable checks",
    nextAction: "replace broad proof language with proven, adapter-boundary, or unimplemented statuses",
  }),
  Object.freeze({
    id: "schema-status-noise",
    lane: "cross-cutting",
    severity: "risk",
    status: "active-risk",
    evidence: "public .v1-style schema names and generated/status noise returned in several lanes",
    nextAction: "remove public version suffix noise unless a compatibility boundary requires it",
  }),
]);

const DEFAULT_READINESS_AUDIT_INPUT = Object.freeze({
  source: DEFAULT_AUDIT_SOURCE,
  overallScore: AUDITED_OVERALL_SCORE,
  dxBuildGraphIntegrationScore: AUDITED_DX_BUILD_GRAPH_INTEGRATION_SCORE,
  criticalGaps: CRITICAL_GAPS,
  nextHighestImpactAction:
    "close one executable dx build or App Router output blocker before raising the merge score above the audit floor",
});

function buildReadinessAuditReport({
  coordinatorReport = null,
  auditInput = DEFAULT_READINESS_AUDIT_INPUT,
  generatedAt = new Date().toISOString(),
} = {}) {
  const normalizedAudit = normalizeReadinessAuditInput(auditInput);
  const coordinatorScore =
    coordinatorReport && Number.isFinite(coordinatorReport.score)
      ? coordinatorReport.score
      : null;
  const reconciledScore =
    coordinatorScore === null
      ? normalizedAudit.overallScore
      : Math.min(normalizedAudit.overallScore, coordinatorScore);

  return {
    schema: READINESS_AUDIT_SCHEMA,
    lane: 14,
    laneName: "Final Coordinator",
    featureImplementation: false,
    status: "blocked",
    executionMode: "audit",
    proofLevel: "audit-reconciled-source-scorecard",
    productionMergeReady: false,
    generatedAt,
    auditSource: normalizedAudit.source,
    auditedOverallScore: normalizedAudit.overallScore,
    auditedDxBuildGraphIntegrationScore: normalizedAudit.dxBuildGraphIntegrationScore,
    coordinatorScore,
    reconciledScore,
    scoreReason:
      "reconciled score is capped by the readiness audit until executable dx build, App Router, hot reload, and codebase-structure blockers are closed",
    architecture: coordinatorReport?.architecture || coordinatorArchitecture(),
    receiptWritePolicy:
      coordinatorReport?.receiptWritePolicy ||
      coordinatorReceiptWritePolicy({
        checks: [],
        executionMode: "preflight",
        writesDefaultScorecard: false,
    }),
    coordinatorEvidence: summarizeCoordinatorEvidence(coordinatorReport),
    criticalGaps: normalizedAudit.criticalGaps.map((gap) => ({ ...gap })),
    blockingGapCount: normalizedAudit.criticalGaps.filter(
      (gap) => gap.severity === "blocking",
    ).length,
    nextHighestImpactAction: normalizedAudit.nextHighestImpactAction,
  };
}

function readReadinessAuditInput(filePath, { cwd = process.cwd() } = {}) {
  if (!filePath || typeof filePath !== "string") {
    throw auditInputError("--audit-file requires a JSON file path");
  }

  const resolvedPath = path.resolve(cwd, filePath);
  let parsed;
  try {
    parsed = JSON.parse(fs.readFileSync(resolvedPath, "utf8"));
  } catch (error) {
    throw auditInputError(`Unable to read audit file "${resolvedPath}": ${error.message}`);
  }

  return normalizeReadinessAuditInput(parsed, { source: resolvedPath });
}

function readDefaultReadinessAuditInput({ cwd = process.cwd() } = {}) {
  return readReadinessAuditInput(DEFAULT_READINESS_AUDIT_BASELINE_PATH, { cwd });
}

function normalizeReadinessAuditInput(
  input = DEFAULT_READINESS_AUDIT_INPUT,
  { source = null } = {},
) {
  if (!input || typeof input !== "object" || Array.isArray(input)) {
    throw auditInputError("Audit input must be a JSON object");
  }

  const criticalGaps = Array.isArray(input.criticalGaps)
    ? input.criticalGaps.map(normalizeCriticalGap)
    : null;
  if (!criticalGaps || criticalGaps.length === 0) {
    throw auditInputError("Audit input must include at least one critical gap");
  }

  return {
    source: source || requiredString(input.source, "source"),
    overallScore: boundedScore(input.overallScore, "overallScore"),
    dxBuildGraphIntegrationScore: boundedScore(
      input.dxBuildGraphIntegrationScore,
      "dxBuildGraphIntegrationScore",
    ),
    criticalGaps,
    nextHighestImpactAction: requiredString(
      input.nextHighestImpactAction,
      "nextHighestImpactAction",
    ),
  };
}

function normalizeCriticalGap(gap, index) {
  if (!gap || typeof gap !== "object" || Array.isArray(gap)) {
    throw auditInputError(`criticalGaps[${index}] must be an object`);
  }

  const normalized = {
    id: requiredString(gap.id, `criticalGaps[${index}].id`),
    lane: normalizeLane(gap.lane, `criticalGaps[${index}].lane`),
    severity: requiredString(gap.severity, `criticalGaps[${index}].severity`),
    status: requiredString(gap.status, `criticalGaps[${index}].status`),
    evidence: requiredString(gap.evidence, `criticalGaps[${index}].evidence`),
    nextAction: requiredString(gap.nextAction, `criticalGaps[${index}].nextAction`),
  };

  if (gap.score !== undefined) {
    normalized.score = boundedScore(gap.score, `criticalGaps[${index}].score`);
  }

  return normalized;
}

function normalizeLane(value, fieldName) {
  if (typeof value === "number" && Number.isFinite(value)) return value;
  return requiredString(value, fieldName);
}

function boundedScore(value, fieldName) {
  if (!Number.isFinite(value) || value < 0 || value > 100) {
    throw auditInputError(`${fieldName} must be a number from 0 to 100`);
  }
  return value;
}

function requiredString(value, fieldName) {
  if (typeof value !== "string" || value.trim().length === 0) {
    throw auditInputError(`${fieldName} must be a non-empty string`);
  }
  return value;
}

function auditInputError(message) {
  const error = new Error(message);
  error.code = "DX_NEXT_RUST_AUDIT_INPUT";
  return error;
}

function summarizeCoordinatorEvidence(report) {
  if (!report) {
    return {
      schema: null,
      executionMode: "none",
      proofLevel: "not-run",
      status: "not-run",
      score: null,
      totals: null,
    };
  }

  return {
    schema: report.schema || null,
    executionMode: report.executionMode || "unknown",
    proofLevel: report.proofLevel || "unknown",
    status: report.status || "unknown",
    score: Number.isFinite(report.score) ? report.score : null,
    totals: report.totals || null,
  };
}

module.exports = {
  READINESS_AUDIT_SCHEMA,
  DEFAULT_READINESS_AUDIT_BASELINE_PATH,
  DEFAULT_READINESS_AUDIT_INPUT,
  buildReadinessAuditReport,
  normalizeReadinessAuditInput,
  readDefaultReadinessAuditInput,
  readReadinessAuditInput,
};
