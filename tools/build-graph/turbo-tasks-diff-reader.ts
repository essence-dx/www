const fs = require("node:fs");
const path = require("node:path");

const { TURBO_TASKS_ADAPTER_DIFF_SCHEMA } = require("./turbo-tasks-adapter.ts");

const TURBO_TASKS_ADAPTER_DIFF_CONSUMER_SUMMARY_SCHEMA =
  "dx.build.graph.turboTasksAdapterDiff.consumerSummary";

function readTurboTasksAdapterDiffReceipt(receiptPath) {
  const absoluteReceiptPath = path.resolve(receiptPath);
  const receiptText = fs.readFileSync(absoluteReceiptPath, "utf8");
  let receipt;
  try {
    receipt = JSON.parse(receiptText);
  } catch (error) {
    throw new Error(
      `Invalid Turbo Tasks adapter diff receipt JSON at ${absoluteReceiptPath}: ${error.message}`,
    );
  }
  validateTurboTasksAdapterDiffReceipt(receipt, absoluteReceiptPath);
  return {
    ...receipt,
    receiptPath: receipt.receiptPath || absoluteReceiptPath,
  };
}

function createTurboTasksAdapterDiffConsumerSummary(receiptPath) {
  const receipt = readTurboTasksAdapterDiffReceipt(receiptPath);
  return createTurboTasksAdapterDiffConsumerSummaryFromReceipt(
    receipt,
    receipt.receiptPath,
  );
}

function createTurboTasksAdapterDiffConsumerSummaryFromReceipt(
  receipt,
  receiptPath = null,
) {
  validateTurboTasksAdapterDiffReceipt(
    receipt,
    receiptPath || receipt.receiptPath || "<memory>",
  );
  const summary = receipt.summary || {};
  return {
    schema: TURBO_TASKS_ADAPTER_DIFF_CONSUMER_SUMMARY_SCHEMA,
    format: 1,
    sourceSchema: receipt.schema,
    receiptPath: receiptPath || receipt.receiptPath || null,
    status: receipt.status,
    previousFingerprint: receipt.previousFingerprint || null,
    currentFingerprint: receipt.currentFingerprint || null,
    taskCounts: {
      changed: countFrom(summary, "changedTaskCount", receipt.changedTaskNodeIds),
      added: countFrom(summary, "addedTaskCount", receipt.addedTaskNodeIds),
      removed: countFrom(summary, "removedTaskCount", receipt.removedTaskNodeIds),
      stale: countFrom(summary, "staleTaskCount", receipt.staleTaskNodeIds),
    },
    staleTaskNodeIds: Array.isArray(receipt.staleTaskNodeIds)
      ? receipt.staleTaskNodeIds
      : [],
    recommendedAction:
      summary.recommendedAction ||
      (receipt.status === "current"
        ? "reuse-current-task-receipts"
        : "rebuild-stale-task-nodes"),
    boundary: {
      adapterOnly: receipt.boundary && receipt.boundary.adapterOnly === true,
      publicArchitecture: receipt.boundary
        ? receipt.boundary.publicArchitecture === true
        : false,
      turboPersistencePublicDependency: receipt.boundary
        ? receipt.boundary.turboPersistencePublicDependency === true
        : false,
      sourceOnly:
        (receipt.boundary && receipt.boundary.sourceOnly === true) ||
        summary.sourceOnly === true,
    },
    consumers: {
      dxCli: {
        diffCommand:
          "dx graph --changed <path> --diff-against .dx/receipts/graph/previous.json --json",
        writeCommand:
          "dx graph --changed <path> --diff-against .dx/receipts/graph/previous.json --write-diff .dx/receipts/graph/turbo-tasks-diff.json --json",
      },
      dxWww: {
        receiptPath: ".dx/receipts/graph/turbo-tasks-diff.json",
        primaryField: "summary",
      },
      zedPreview: {
        receiptPath: ".dx/receipts/graph/turbo-tasks-diff.json",
        primaryField: "summary",
      },
    },
  };
}

function validateTurboTasksAdapterDiffReceipt(receipt, receiptPath) {
  if (receipt.schema !== TURBO_TASKS_ADAPTER_DIFF_SCHEMA) {
    throw new Error(
      `Expected ${TURBO_TASKS_ADAPTER_DIFF_SCHEMA} receipt at ${receiptPath}`,
    );
  }
  if (!receipt.boundary || receipt.boundary.adapterOnly !== true) {
    throw new Error(`Turbo Tasks adapter diff must stay adapter-only: ${receiptPath}`);
  }
  if (receipt.boundary.publicArchitecture !== false) {
    throw new Error(
      `Turbo Tasks adapter diff must not be public architecture: ${receiptPath}`,
    );
  }
  if (receipt.boundary.turboPersistencePublicDependency !== false) {
    throw new Error(
      `Turbo Tasks adapter diff must not require Turbo Persistence: ${receiptPath}`,
    );
  }
}

function countFrom(summary, fieldName, fallbackItems) {
  if (Number.isFinite(summary[fieldName])) return summary[fieldName];
  return Array.isArray(fallbackItems) ? fallbackItems.length : 0;
}

module.exports = {
  TURBO_TASKS_ADAPTER_DIFF_CONSUMER_SUMMARY_SCHEMA,
  createTurboTasksAdapterDiffConsumerSummary,
  createTurboTasksAdapterDiffConsumerSummaryFromReceipt,
  readTurboTasksAdapterDiffReceipt,
};
