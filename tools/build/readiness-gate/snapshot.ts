const path = require("node:path");

const {
  READINESS_GATE_SNAPSHOT,
  SNAPSHOT_SCHEMA,
} = require("./constants.ts");
const { snapshotConsumers } = require("./consumers.ts");

function createConsumerSnapshot(projectRoot, report) {
  return {
    schema: SNAPSHOT_SCHEMA,
    format: 1,
    sourceSchema: report.schema,
    projectRoot,
    receiptPath: path.join(projectRoot, READINESS_GATE_SNAPSHOT),
    status: {
      state: report.status,
      ready: report.productReady,
      sourceReady: report.sourceReady,
      productReady: report.productReady,
      blockerCount: report.blockers.length,
    },
    score: report.score,
    receipts: report.receipts,
    proofs: report.proofs,
    proofBundle: report.proofBundle,
    blockers: report.blockers,
    requiredActions: report.requiredActions,
    quality: {
      entrypointUsesSplitModules: report.quality.entrypointUsesSplitModules,
      maxLineCount: report.quality.maxLineCount,
      monolithFallbackUsed: report.quality.monolithFallbackUsed,
      smallModuleBoundary: report.quality.smallModuleBoundary,
    },
    consumers: snapshotConsumers(),
    nextAction: report.nextAction,
  };
}

module.exports = {
  createConsumerSnapshot,
};
