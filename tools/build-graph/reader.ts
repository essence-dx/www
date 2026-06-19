const fs = require("node:fs");
const path = require("node:path");

const { CONTRACT_NAMES } = require("./types.ts");
const {
  summarizeTurbopackCoreConceptMap,
} = require("./turbopack-core-map.ts");
const {
  createTurbopackCoreConceptMapStatus,
} = require("./turbopack-core-status.ts");
const {
  createTurboTasksAdapterStatusFromReceipt,
} = require("./turbo-tasks-status.ts");
const {
  findNextRustVendorWorkspaceRoot,
} = require("./vendor-root.ts");

const CONSUMER_SNAPSHOT_SCHEMA = "dx.build.graph.consumerSnapshot";
const DEFAULT_GRAPH_RECEIPT_PATH = ".dx/receipts/graph/latest.json";
const TURBOPACK_CORE_VENDOR_ROOT = findNextRustVendorWorkspaceRoot(__dirname);

function readDxBuildGraphReceipt(receiptPath) {
  const absoluteReceiptPath = path.resolve(receiptPath);
  const receipt = JSON.parse(fs.readFileSync(absoluteReceiptPath, "utf8"));
  validateBuildGraphReceipt(receipt, absoluteReceiptPath);
  return {
    ...receipt,
    receiptPath: receipt.receiptPath || absoluteReceiptPath,
  };
}

function createDxBuildGraphConsumerSnapshot(receiptPath) {
  const receipt = readDxBuildGraphReceipt(receiptPath);
  return createDxBuildGraphConsumerSnapshotFromReceipt(receipt, receipt.receiptPath);
}

function createDxBuildGraphConsumerSnapshotFromReceipt(receipt, receiptPath = null) {
  validateBuildGraphReceipt(receipt, receiptPath || receipt.receiptPath || "<memory>");
  const coreConceptMap = receipt.coreConceptMap || [];
  const coreConceptMapStatus = createTurbopackCoreConceptMapStatus(
    TURBOPACK_CORE_VENDOR_ROOT,
    coreConceptMap,
  );
  const turboTasksAdapterStatus = createTurboTasksAdapterStatusFromReceipt(
    receipt,
    receiptPath || receipt.receiptPath || null,
  );
  return {
    schema: CONSUMER_SNAPSHOT_SCHEMA,
    format: 1,
    sourceSchema: receipt.schema,
    receiptPath: receiptPath || receipt.receiptPath || null,
    names: receipt.names,
    positioning: receipt.positioning,
    consumers: {
      dxCli: {
        command: "dx graph --consumer-snapshot --json",
        writeCommand:
          "dx graph --changed <path> --write .dx/receipts/graph/latest.json",
      },
      dxWww: {
        receiptPath: DEFAULT_GRAPH_RECEIPT_PATH,
        primaryField: "graph.nodes",
      },
      zedPreview: {
        receiptPath: DEFAULT_GRAPH_RECEIPT_PATH,
        primaryField: "invalidation.affectedNodeIds",
      },
    },
    graph: {
      nodeCount: receipt.graph.nodes.length,
      edgeCount: receipt.graph.edges.length,
      nodeKindCounts: countBy(receipt.graph.nodes, (node) => node.kind),
      styleOptimization: summarizeStyleOptimization(receipt.graph.nodes),
    },
    coreConceptMap: summarizeTurbopackCoreConceptMap(coreConceptMap),
    coreConceptMapValidation: coreConceptMapStatus.validation,
    coreConceptMapStatus,
    turboTasksAdapter: summarizeTurboTasksAdapter(receipt.turboTasksAdapter),
    turboTasksAdapterStatus,
    invalidation: {
      changedNodeIds: receipt.invalidation.changedNodeIds,
      affectedNodeIds: receipt.invalidation.affectedNodeIds,
      rebuildNodeIds: receipt.invalidation.rebuildNodeIds,
      changedCount: receipt.invalidation.changedNodeIds.length,
      affectedCount: receipt.invalidation.affectedNodeIds.length,
      rebuildCount: receipt.invalidation.rebuildNodeIds.length,
    },
    provenance: receipt.provenance,
  };
}

function summarizeTurboTasksAdapter(adapter) {
  if (!adapter) return null;
  return {
    schema: adapter.schema,
    format: adapter.format,
    taskCount: Array.isArray(adapter.tasks) ? adapter.tasks.length : 0,
    fingerprint: adapter.fingerprint || null,
    upstreamCrates: adapter.upstream && Array.isArray(adapter.upstream.crates)
      ? adapter.upstream.crates
      : [],
    boundary: {
      adapterOnly: adapter.positioning && adapter.positioning.adapterOnly === true,
      publicArchitecture: adapter.positioning
        ? adapter.positioning.publicArchitecture === true
        : false,
      dxRuntimeAuthoritative:
        adapter.positioning && adapter.positioning.dxRuntimeAuthoritative === true,
    },
    parallelism: {
      scheduler: adapter.parallelism ? adapter.parallelism.scheduler : null,
      levelCount:
        adapter.parallelism && Array.isArray(adapter.parallelism.executionLevels)
          ? adapter.parallelism.executionLevels.length
          : 0,
      maxParallelWidth: adapter.parallelism ? adapter.parallelism.maxParallelWidth : 0,
    },
    persistence: {
      mode: adapter.persistence ? adapter.persistence.mode : null,
      cacheNamespace: adapter.persistence ? adapter.persistence.cacheNamespace : null,
      hasTaskFingerprints: Array.isArray(adapter.tasks)
        ? adapter.tasks.every((task) => typeof task.inputFingerprint === "string")
        : false,
      turboPersistencePublicDependency: adapter.persistence
        ? adapter.persistence.turboPersistencePublicDependency === true
        : false,
    },
  };
}

function validateBuildGraphReceipt(receipt, receiptPath) {
  if (receipt.schema !== CONTRACT_NAMES.buildGraph) {
    throw new Error(
      `Expected ${CONTRACT_NAMES.buildGraph} receipt at ${receiptPath}`,
    );
  }
  if (!receipt.names || receipt.names.buildGraph !== CONTRACT_NAMES.buildGraph) {
    throw new Error(`DX build graph receipt has invalid contract names: ${receiptPath}`);
  }
  if (!receipt.positioning || receipt.positioning.turbopackPublicDependency !== false) {
    throw new Error(`DX build graph receipt must keep Turbopack non-public: ${receiptPath}`);
  }
  if (!receipt.graph || !Array.isArray(receipt.graph.nodes) || !Array.isArray(receipt.graph.edges)) {
    throw new Error(`DX build graph receipt is missing graph nodes/edges: ${receiptPath}`);
  }
  if (
    !receipt.invalidation ||
    !Array.isArray(receipt.invalidation.changedNodeIds) ||
    !Array.isArray(receipt.invalidation.affectedNodeIds) ||
    !Array.isArray(receipt.invalidation.rebuildNodeIds)
  ) {
    throw new Error(`DX build graph receipt is missing invalidation arrays: ${receiptPath}`);
  }
}

function countBy(items, selectKey) {
  return items.reduce((counts, item) => {
    const key = selectKey(item);
    counts[key] = (counts[key] || 0) + 1;
    return counts;
  }, {});
}

function summarizeStyleOptimization(nodes) {
  const styles = nodes.filter((node) => node.kind === "dx-style-css");
  return styles.reduce(
    (summary, node) => {
      summary.styleNodeCount += 1;
      summary.originalRuleCount += numberField(node, "original_rule_count");
      summary.retainedRuleCount += numberField(node, "retained_rule_count");
      summary.prunedRuleCount += numberField(node, "pruned_rule_count");
      if (node.minified === true) {
        summary.minifiedStyleCount += 1;
      }
      return summary;
    },
    {
      styleNodeCount: 0,
      originalRuleCount: 0,
      retainedRuleCount: 0,
      prunedRuleCount: 0,
      minifiedStyleCount: 0,
    },
  );
}

function numberField(value, fieldName) {
  return Number.isFinite(value[fieldName]) ? value[fieldName] : 0;
}

module.exports = {
  CONSUMER_SNAPSHOT_SCHEMA,
  DEFAULT_GRAPH_RECEIPT_PATH,
  createDxBuildGraphConsumerSnapshot,
  createDxBuildGraphConsumerSnapshotFromReceipt,
  readDxBuildGraphReceipt,
};
