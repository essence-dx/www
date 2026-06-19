const { TURBO_TASKS_ADAPTER_SCHEMA } = require("./turbo-tasks-adapter.ts");
const {
  TURBO_TASKS_ADAPTER_DIFF_CONSUMER_SUMMARY_SCHEMA,
} = require("./turbo-tasks-diff-reader.ts");
const {
  TURBO_TASKS_LANE_SCORE_FOR_SOURCE_ONLY_STATUS,
} = require("./turbo-tasks-lane-contract.ts");

const TURBO_TASKS_ADAPTER_STATUS_SCHEMA =
  "dx.build.graph.turboTasksAdapterStatus";
const UNPROVEN_RUNTIME_SURFACES = Object.freeze([
  "dx-owned-build-graph-cache-integration",
  "native-editor-build-graph-render",
]);

function createTurboTasksAdapterStatusFromReceipt(receipt, receiptPath = null) {
  const adapter = receipt ? receipt.turboTasksAdapter : null;
  const blockingReasons = collectBlockingReasons(adapter);
  const sourceScore = scoreSourceStatus(blockingReasons);
  const tasks = adapter && Array.isArray(adapter.tasks) ? adapter.tasks : [];

  return {
    schema: TURBO_TASKS_ADAPTER_STATUS_SCHEMA,
    format: 1,
    lane: 2,
    laneName: "Turbo Tasks Build Graph",
    status: blockingReasons.length === 0 ? "source-passing" : "blocked",
    sourceScore,
    score:
      blockingReasons.length === 0
        ? TURBO_TASKS_LANE_SCORE_FOR_SOURCE_ONLY_STATUS
        : Math.max(0, sourceScore - 20),
    receiptPath: receiptPath || (receipt && receipt.receiptPath) || null,
    blockingReasons,
    architecture: {
      dxRuntimeAuthoritative:
        adapter && adapter.positioning
          ? adapter.positioning.dxRuntimeAuthoritative === true
          : false,
      publicTurbopackDependency:
        adapter && adapter.positioning
          ? adapter.positioning.turbopackPublicDependency === true
          : true,
      publicArchitecture:
        adapter && adapter.positioning
          ? adapter.positioning.publicArchitecture === true
          : true,
      reactRequiredCore: false,
      nodeNapiFoundation:
        adapter && adapter.positioning
          ? adapter.positioning.nodeNapiFoundation === true
          : true,
      nodeModulesRequired: false,
      turboPersistenceRuntimeDependency:
        adapter && adapter.persistence
          ? adapter.persistence.turboPersistencePublicDependency === true
          : true,
    },
    evidence: {
      adapterSchema: adapter ? adapter.schema || null : null,
      upstreamCrates:
        adapter && adapter.upstream && Array.isArray(adapter.upstream.crates)
          ? adapter.upstream.crates
          : [],
      copiedUpstreamCode:
        adapter && adapter.upstream ? adapter.upstream.copiedCode === true : true,
      changedNodeCount: count(adapter, ["invalidation", "changedNodeIds"]),
      affectedNodeCount: count(adapter, ["invalidation", "affectedNodeIds"]),
      rebuildNodeCount: count(adapter, ["invalidation", "rebuildNodeIds"]),
      taskCount: tasks.length,
      executionLevelCount: count(adapter, ["parallelism", "executionLevels"]),
      maxParallelWidth:
        adapter && adapter.parallelism
          ? adapter.parallelism.maxParallelWidth || 0
          : 0,
      scheduler: adapter && adapter.parallelism ? adapter.parallelism.scheduler : null,
      persistenceMode: adapter && adapter.persistence ? adapter.persistence.mode : null,
      cacheNamespace:
        adapter && adapter.persistence ? adapter.persistence.cacheNamespace : null,
      hasTaskFingerprints: hasTaskFingerprints(tasks),
      diffSummarySchema: TURBO_TASKS_ADAPTER_DIFF_CONSUMER_SUMMARY_SCHEMA,
      referenceOnly: true,
      runtimeExecutionPath: false,
      dxTaskCacheRuntime: false,
      executorRunReceiptWriter: false,
      executorRunSummaryReader: false,
      combinedGraphAndExecutorReceiptHandoff: false,
      taskCacheExecutorSchema: null,
      taskCacheExecutorSummarySchema: null,
    },
    adapterBoundary: {
      adapterOnly:
        adapter && adapter.positioning
          ? adapter.positioning.adapterOnly === true
          : false,
      sourceOnly:
        Boolean(adapter && adapter.invalidation && adapter.invalidation.sourceOnly) &&
        Boolean(adapter && adapter.parallelism && adapter.parallelism.sourceOnly) &&
        Boolean(adapter && adapter.persistence && adapter.persistence.sourceOnly),
      runtimeSchedulerExecuted: false,
      turboPersistenceOpened: false,
    },
    unproven: [...UNPROVEN_RUNTIME_SURFACES],
    recommendedAction:
      blockingReasons.length === 0
        ? "consume the reference-only adapter/diff/status before wiring DX-owned build graph UI"
        : "fix Lane 2 adapter boundary blockers before using task graph status",
  };
}

function collectBlockingReasons(adapter) {
  const reasons = [];
  if (!adapter) {
    return ["missing-turbo-tasks-adapter"];
  }
  if (adapter.schema !== TURBO_TASKS_ADAPTER_SCHEMA) {
    reasons.push("invalid-adapter-schema");
  }
  if (!adapter.positioning || adapter.positioning.adapterOnly !== true) {
    reasons.push("missing-adapter-boundary");
  }
  if (!adapter.positioning || adapter.positioning.turbopackPublicDependency !== false) {
    reasons.push("public-turbopack-dependency");
  }
  if (!adapter.positioning || adapter.positioning.publicArchitecture !== false) {
    reasons.push("public-architecture-overclaim");
  }
  if (!adapter.positioning || adapter.positioning.nodeNapiFoundation !== false) {
    reasons.push("node-napi-foundation");
  }
  if (!adapter.positioning || adapter.positioning.dxRuntimeAuthoritative !== true) {
    reasons.push("dx-runtime-not-authoritative");
  }
  if (!adapter.parallelism || adapter.parallelism.scheduler !== "dx-owned-topological-levels") {
    reasons.push("scheduler-not-source-owned");
  }
  if (!adapter.persistence || adapter.persistence.mode !== "source-receipt-plan") {
    reasons.push("cache-not-source-receipt-plan");
  }
  if (
    !adapter.persistence ||
    adapter.persistence.turboPersistencePublicDependency !== false
  ) {
    reasons.push("turbo-persistence-public-dependency");
  }
  if (!hasTaskFingerprints(Array.isArray(adapter.tasks) ? adapter.tasks : [])) {
    reasons.push("missing-task-fingerprints");
  }
  return reasons;
}

function scoreSourceStatus(blockingReasons) {
  const penalty = blockingReasons.length * 20;
  return Math.max(0, Math.min(100, 100 - penalty));
}

function count(value, pathParts) {
  let current = value;
  for (const part of pathParts) {
    current = current && current[part];
  }
  return Array.isArray(current) ? current.length : 0;
}

function hasTaskFingerprints(tasks) {
  return (
    tasks.length > 0 &&
    tasks.every(
      (task) =>
        typeof task.inputFingerprint === "string" &&
        /^[a-f0-9]{64}$/.test(task.inputFingerprint),
    )
  );
}

module.exports = {
  TURBO_TASKS_ADAPTER_STATUS_SCHEMA,
  createTurboTasksAdapterStatusFromReceipt,
};
