const COORDINATOR_ARCHITECTURE = Object.freeze({
  dxRuntimeAuthoritative: true,
  publicTurbopackDependency: false,
  reactRequiredCore: false,
  nodeModulesRequired: false,
  nodeNapiFoundation: false,
});

function coordinatorArchitecture() {
  return { ...COORDINATOR_ARCHITECTURE };
}

function coordinatorProofContract(executionMode) {
  if (executionMode === "preflight") {
    return {
      executionMode,
      proofLevel: "declared-source-plan-not-executed",
      score: null,
      scoreReason: "not scored because checks were not executed",
    };
  }

  return {
    executionMode,
    proofLevel: "executed-lightweight-source-checks",
  };
}

function coordinatorReceiptWritePolicy({
  checks = [],
  executionMode,
  writesDefaultScorecard = false,
} = {}) {
  const receiptWritingCheckIds = checks
    .filter((entry) => entry.sideEffects && entry.sideEffects.writesReceipts)
    .map((entry) => entry.id);
  const executesChecks = executionMode === "run";

  return {
    writesDefaultScorecard,
    executesChecks,
    mayWriteTempReceipts: executesChecks && receiptWritingCheckIds.length > 0,
    receiptWritingCheckIds,
    note: executesChecks
      ? "running checks may write declared temp fixture receipts"
      : "preflight does not run checks or write temp fixture receipts",
  };
}

module.exports = {
  coordinatorArchitecture,
  coordinatorProofContract,
  coordinatorReceiptWritePolicy,
};
