const path = require("node:path");

const {
  GATE_SCHEMA,
  READINESS_GATE_RECEIPT,
} = require("./constants.ts");
const { requiredActions } = require("./actions.ts");
const { reportConsumers } = require("./consumers.ts");
const { collectBlockers, summarizeReceipts } = require("./receipt-checks.ts");
const {
  installedSmokeReadyForProduct,
  productEvidenceNeedsReadinessConfirmation,
  runtimeValidationApproved,
  summarizeProductEvidence,
} = require("./proofs.ts");

function createReport(projectRoot, receipts, quality, proofBundle = null) {
  const blockers = [
    ...collectBlockers(receipts),
    ...proofBundleBlockers(proofBundle),
  ];
  const readiness = receipts.readiness.value || {};
  const installedSmoke = receipts.installedBinarySmoke.value || {};
  const sourceReady = readiness.source_ready === true && hasRequiredReceipts(receipts);
  const productReady =
    sourceReady &&
    readiness.product_ready === true &&
    installedSmokeReadyForProduct(installedSmoke) &&
    runtimeValidationApproved(receipts.checkLaunch) &&
    blockers.length === 0;
  const actions = requiredActions(receipts, productReady);

  return {
    schema: GATE_SCHEMA,
    schemaRevision: 1,
    generatedAt: new Date().toISOString(),
    projectRoot,
    status: productReady ? "ready" : "blocked",
    sourceReady,
    productReady,
    score: {
      product: productReady ? 100 : readiness.product_score || 0,
      source: sourceReady ? readiness.source_score || 100 : 0,
    },
    receipts: summarizeReceipts(receipts),
    proofs: summarizeProductEvidence({
      sourceReady,
      productReady,
      receipts,
      requiredActions: actions,
    }),
    proofBundle,
    blockers,
    requiredActions: actions,
    quality,
    consumers: reportConsumers(),
    nextAction: nextAction({ productReady, sourceReady, receipts }),
    receiptPath: path.join(projectRoot, READINESS_GATE_RECEIPT),
  };
}

function hasRequiredReceipts(receipts) {
  return (
    receipts.readiness.present &&
    !receipts.readiness.malformed &&
    receipts.zedHandoff.present &&
    !receipts.zedHandoff.malformed
  );
}

function nextAction({ productReady, sourceReady, receipts }) {
  if (productReady) {
    return "Keep the latest build readiness, Zed handoff, and installed-binary smoke receipts current.";
  }
  const readiness = receipts.readiness.value || {};
  const installedSmoke = receipts.installedBinarySmoke.value || {};
  if (
    sourceReady &&
    productEvidenceNeedsReadinessConfirmation(readiness, installedSmoke, receipts.checkLaunch)
  ) {
    return "Refresh DX Build readiness after governed smoke and runtime validation so product readiness can be confirmed.";
  }
  if (sourceReady) {
    return "Refresh installed-binary smoke and run governed runtime validation before claiming product readiness.";
  }
  return "Refresh build readiness, Zed handoff, and installed-binary smoke receipts before claiming product readiness.";
}

function proofBundleBlockers(proofBundle) {
  if (!proofBundle || proofBundle.mode !== "run") {
    return [];
  }
  return (proofBundle.steps || [])
    .filter((step) => step.status === "failed")
    .map((step) => `validation bundle step failed: ${step.id}`);
}

module.exports = {
  createReport,
};
