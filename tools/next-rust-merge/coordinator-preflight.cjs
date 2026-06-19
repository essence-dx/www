const { coordinatorSideEffectSummary } = require("./coordinator-side-effects.cjs");
const {
  coordinatorArchitecture,
  coordinatorProofContract,
  coordinatorReceiptWritePolicy,
} = require("./coordinator-report-contract.cjs");

const PREFLIGHT_SCHEMA = "dx.nextRustMerge.coordinatorPreflight";

function buildCoordinatorPreflightReport({
  checks,
  generatedAt = new Date().toISOString(),
} = {}) {
  const selectedChecks = Array.isArray(checks) ? checks : [];
  const receiptWritingChecks = selectedChecks.filter(
    (entry) => entry.sideEffects.writesReceipts,
  );
  const proofContract = coordinatorProofContract("preflight");

  return {
    schema: PREFLIGHT_SCHEMA,
    lane: 14,
    laneName: "Final Coordinator",
    featureImplementation: false,
    status: "not-run",
    ...proofContract,
    generatedAt,
    totals: {
      total: selectedChecks.length,
      blocking: selectedChecks.filter((entry) => entry.blocking).length,
      nonBlocking: selectedChecks.filter((entry) => !entry.blocking).length,
      receiptWriting: receiptWritingChecks.length,
      readOnly: selectedChecks.length - receiptWritingChecks.length,
    },
    architecture: coordinatorArchitecture(),
    receiptWritePolicy: coordinatorReceiptWritePolicy({
      checks: selectedChecks,
      executionMode: "preflight",
      writesDefaultScorecard: false,
    }),
    sideEffectSummary: coordinatorSideEffectSummary(selectedChecks),
    checks: selectedChecks.map((entry) => ({
      id: entry.id,
      lane: entry.lane,
      boundary: entry.boundary,
      blocking: entry.blocking,
      command: entry.command.join(" "),
      status: "not-run",
      willRun: false,
      proves: entry.proves,
      healthContract: entry.healthContract || null,
      sideEffects: entry.sideEffects,
    })),
  };
}

module.exports = {
  PREFLIGHT_SCHEMA,
  buildCoordinatorPreflightReport,
};
