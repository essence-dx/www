const crypto = require("node:crypto");

const TURBO_TASKS_ADAPTER_SCHEMA = "dx.build.graph.turboTasksAdapter";
const TURBO_TASKS_ADAPTER_DIFF_SCHEMA = "dx.build.graph.turboTasksAdapterDiff";
const CACHEABLE_TASK_NODE_KINDS = new Set([
  "dx-style-css",
  "source-module",
  "tsx-component",
  "tsx-route",
]);

const TURBO_TASKS_UPSTREAM = Object.freeze({
  crates: [
    "turbo-tasks",
    "turbo-tasks-backend",
    "turbo-tasks-fs",
    "turbo-persistence",
  ],
  copiedCode: false,
  inspectedFiles: [
    "vendor/next-rust/turbopack/crates/turbo-tasks/src/task/task_input.rs",
    "vendor/next-rust/turbopack/crates/turbo-tasks/src/lib.rs",
    "vendor/next-rust/turbopack/crates/turbo-tasks-backend/src/backend/operation/update_cell.rs",
    "vendor/next-rust/turbopack/crates/turbo-tasks-backend/src/backend/operation/invalidate.rs",
    "vendor/next-rust/turbopack/crates/turbo-tasks-fs/src/invalidation.rs",
    "vendor/next-rust/turbopack/crates/turbo-persistence/src/lib.rs",
  ],
});

const TURBO_TASKS_CONCEPTS = Object.freeze([
  {
    upstreamConcept: "TaskInput",
    dxReceiptField: "tasks[].inputKey",
    boundary: "source-owned stable receipt key",
  },
  {
    upstreamConcept: "cell comparison invalidation",
    dxReceiptField: "invalidation.strategy",
    boundary: "reverse graph dirty propagation, no upstream runtime",
  },
  {
    upstreamConcept: "parallel scheduler",
    dxReceiptField: "parallelism.executionLevels",
    boundary: "DX-owned topological levels",
  },
  {
    upstreamConcept: "persistent task cache",
    dxReceiptField: "persistence",
    boundary: "source receipt plan, no turbo database dependency",
  },
]);

function createTurboTasksAdapterPlan(_projectRoot, graph, invalidation) {
  const dependenciesByNode = createDependenciesByNode(graph.edges);
  const dependentsByNode = createDependentsByNode(graph.edges);
  const rebuildNodeIds = [...invalidation.rebuildNodeIds];
  const executionLevels = createExecutionLevels(rebuildNodeIds, dependenciesByNode);
  const nodesById = new Map(graph.nodes.map((node) => [node.id, node]));
  const tasks = rebuildNodeIds.map((nodeId) =>
    createTaskPlan(nodeId, nodesById, dependenciesByNode, dependentsByNode),
  );

  return {
    schema: TURBO_TASKS_ADAPTER_SCHEMA,
    format: 1,
    positioning: {
      adapterOnly: true,
      publicArchitecture: false,
      turbopackPublicDependency: false,
      nodeNapiFoundation: false,
      dxRuntimeAuthoritative: true,
    },
    upstream: TURBO_TASKS_UPSTREAM,
    concepts: TURBO_TASKS_CONCEPTS,
    invalidation: {
      strategy: "reverse-dependency-dirty-propagation",
      changedNodeIds: [...invalidation.changedNodeIds],
      affectedNodeIds: [...invalidation.affectedNodeIds],
      rebuildNodeIds,
      sourceOnly: true,
    },
    parallelism: {
      scheduler: "dx-owned-topological-levels",
      executionLevels,
      maxParallelWidth: executionLevels.reduce(
        (max, level) => Math.max(max, level.nodeIds.length),
        0,
      ),
      sourceOnly: true,
    },
    persistence: {
      mode: "source-receipt-plan",
      cacheNamespace: ".dx/cache/build-graph/tasks",
      keyFields: [
        "schema",
        "node.id",
        "node.kind",
        "node.path",
        "node.contentHash",
        "dependencyNodeIds",
      ],
      turboPersistencePublicDependency: false,
      sourceOnly: true,
    },
    fingerprint: createStableHash(tasks.map((task) => task.inputFingerprint).sort()),
    tasks,
  };
}

function createTaskPlan(nodeId, nodesById, dependenciesByNode, dependentsByNode) {
  const node = nodesById.get(nodeId);
  const dependencyNodeIds = sorted(dependenciesByNode.get(nodeId) || []);
  return {
    id: `dx.build.task:${nodeId}`,
    nodeId,
    nodeKind: node ? node.kind : "unknown",
    path: node ? node.path : null,
    inputKey: createInputKey(node, dependencyNodeIds),
    inputFingerprint: createTaskFingerprint(node, dependencyNodeIds),
    dependencyNodeIds,
    dependentNodeIds: sorted(dependentsByNode.get(nodeId) || []),
    adapterBoundary: true,
    publicArchitecture: false,
    cacheable: Boolean(node && CACHEABLE_TASK_NODE_KINDS.has(node.kind)),
  };
}

function createDependenciesByNode(edges) {
  const dependencies = new Map();
  for (const edge of edges) {
    if (!dependencies.has(edge.from)) dependencies.set(edge.from, new Set());
    dependencies.get(edge.from).add(edge.to);
  }
  return dependencies;
}

function createDependentsByNode(edges) {
  const dependents = new Map();
  for (const edge of edges) {
    if (!dependents.has(edge.to)) dependents.set(edge.to, new Set());
    dependents.get(edge.to).add(edge.from);
  }
  return dependents;
}

function createExecutionLevels(rebuildNodeIds, dependenciesByNode) {
  const rebuildSet = new Set(rebuildNodeIds);
  const dependentsByNode = new Map();
  const remainingDependencies = new Map();

  for (const nodeId of rebuildNodeIds) {
    const dependencies = sorted(dependenciesByNode.get(nodeId) || []).filter((dependency) =>
      rebuildSet.has(dependency),
    );
    remainingDependencies.set(nodeId, new Set(dependencies));
    for (const dependency of dependencies) {
      if (!dependentsByNode.has(dependency)) dependentsByNode.set(dependency, new Set());
      dependentsByNode.get(dependency).add(nodeId);
    }
  }

  const scheduled = new Set();
  const levels = [];
  let ready = sorted(
    rebuildNodeIds.filter((nodeId) => remainingDependencies.get(nodeId).size === 0),
  );

  while (ready.length > 0) {
    const nodeIds = ready.filter((nodeId) => !scheduled.has(nodeId));
    if (nodeIds.length === 0) break;
    levels.push({ level: levels.length, nodeIds });
    ready = [];
    for (const nodeId of nodeIds) {
      scheduled.add(nodeId);
      for (const dependent of dependentsByNode.get(nodeId) || []) {
        const dependencies = remainingDependencies.get(dependent);
        dependencies.delete(nodeId);
        if (dependencies.size === 0 && !scheduled.has(dependent)) {
          ready.push(dependent);
        }
      }
    }
    ready = sorted(ready);
  }

  const unscheduled = sorted(rebuildNodeIds.filter((nodeId) => !scheduled.has(nodeId)));
  if (unscheduled.length > 0) {
    levels.push({
      level: levels.length,
      nodeIds: unscheduled,
      cycleDetected: true,
    });
  }

  return levels;
}

function createInputKey(node, dependencyNodeIds) {
  if (!node) return "dx.build.task:unknown";
  return [
    TURBO_TASKS_ADAPTER_SCHEMA,
    node.id,
    node.kind,
    node.path,
    node.contentHash || "",
    `deps=${dependencyNodeIds.join(",")}`,
  ].join("|");
}

function createTaskFingerprint(node, dependencyNodeIds) {
  if (!node) {
    return createStableHash({
      schema: TURBO_TASKS_ADAPTER_SCHEMA,
      nodeId: "unknown",
      dependencyNodeIds,
    });
  }
  return createStableHash({
    schema: TURBO_TASKS_ADAPTER_SCHEMA,
    nodeId: node.id,
    nodeKind: node.kind,
    path: node.path,
    bytes: node.bytes,
    contentHash: node.contentHash || null,
    dependencyNodeIds,
  });
}

function createStableHash(value) {
  return crypto
    .createHash("sha256")
    .update(JSON.stringify(value))
    .digest("hex");
}

function diffTurboTasksAdapterPlans(previousAdapter, currentAdapter) {
  const previousTasks = taskMap(previousAdapter);
  const currentTasks = taskMap(currentAdapter);
  const currentOrder = new Map(
    tasksOf(currentAdapter).map((task, index) => [task.nodeId, index]),
  );

  const changedTaskNodeIds = tasksOf(currentAdapter)
    .filter((task) => {
      const previous = previousTasks.get(task.nodeId);
      return previous && previous.inputFingerprint !== task.inputFingerprint;
    })
    .map((task) => task.nodeId);
  const addedTaskNodeIds = tasksOf(currentAdapter)
    .filter((task) => !previousTasks.has(task.nodeId))
    .map((task) => task.nodeId);
  const removedTaskNodeIds = tasksOf(previousAdapter)
    .filter((task) => !currentTasks.has(task.nodeId))
    .map((task) => task.nodeId);
  const staleTaskNodeIds = collectStaleTaskNodeIds(
    [...changedTaskNodeIds, ...addedTaskNodeIds],
    currentTasks,
    currentOrder,
  );
  const status =
    changedTaskNodeIds.length === 0 &&
    addedTaskNodeIds.length === 0 &&
    removedTaskNodeIds.length === 0
      ? "current"
      : "stale";

  return {
    schema: TURBO_TASKS_ADAPTER_DIFF_SCHEMA,
    format: 1,
    status,
    previousFingerprint: previousAdapter ? previousAdapter.fingerprint || null : null,
    currentFingerprint: currentAdapter ? currentAdapter.fingerprint || null : null,
    changedTaskNodeIds,
    addedTaskNodeIds,
    removedTaskNodeIds,
    staleTaskNodeIds,
    staleTaskCount: staleTaskNodeIds.length,
    summary: createDiffSummary(
      status,
      changedTaskNodeIds,
      addedTaskNodeIds,
      removedTaskNodeIds,
      staleTaskNodeIds,
    ),
    taskChanges: createTaskChanges(
      changedTaskNodeIds,
      addedTaskNodeIds,
      removedTaskNodeIds,
      previousTasks,
      currentTasks,
    ),
    boundary: {
      adapterOnly: true,
      publicArchitecture: false,
      turboPersistencePublicDependency: false,
      sourceOnly: true,
    },
  };
}

function createDiffSummary(
  status,
  changedTaskNodeIds,
  addedTaskNodeIds,
  removedTaskNodeIds,
  staleTaskNodeIds,
) {
  return {
    changedTaskCount: changedTaskNodeIds.length,
    addedTaskCount: addedTaskNodeIds.length,
    removedTaskCount: removedTaskNodeIds.length,
    staleTaskCount: staleTaskNodeIds.length,
    hasStaleTasks: staleTaskNodeIds.length > 0,
    recommendedAction:
      status === "current"
        ? "reuse-current-task-receipts"
        : "rebuild-stale-task-nodes",
    sourceOnly: true,
  };
}

function collectStaleTaskNodeIds(seedNodeIds, currentTasks, currentOrder) {
  const stale = [];
  const seen = new Set();
  let queue = sortByCurrentOrder(seedNodeIds, currentOrder);

  while (queue.length > 0) {
    const nodeId = queue.shift();
    if (seen.has(nodeId) || !currentTasks.has(nodeId)) continue;
    seen.add(nodeId);
    stale.push(nodeId);
    const dependents = sortByCurrentOrder(
      currentTasks.get(nodeId).dependentNodeIds || [],
      currentOrder,
    ).filter((dependent) => currentTasks.has(dependent) && !seen.has(dependent));
    queue.push(...dependents);
  }

  return stale;
}

function createTaskChanges(
  changedTaskNodeIds,
  addedTaskNodeIds,
  removedTaskNodeIds,
  previousTasks,
  currentTasks,
) {
  const changes = [];
  for (const nodeId of changedTaskNodeIds) {
    changes.push({
      nodeId,
      reason: "input-fingerprint-changed",
      previousInputFingerprint: previousTasks.get(nodeId).inputFingerprint,
      currentInputFingerprint: currentTasks.get(nodeId).inputFingerprint,
    });
  }
  for (const nodeId of addedTaskNodeIds) {
    changes.push({
      nodeId,
      reason: "task-added",
      previousInputFingerprint: null,
      currentInputFingerprint: currentTasks.get(nodeId).inputFingerprint,
    });
  }
  for (const nodeId of removedTaskNodeIds) {
    changes.push({
      nodeId,
      reason: "task-removed",
      previousInputFingerprint: previousTasks.get(nodeId).inputFingerprint,
      currentInputFingerprint: null,
    });
  }
  return changes;
}

function taskMap(adapter) {
  return new Map(tasksOf(adapter).map((task) => [task.nodeId, task]));
}

function tasksOf(adapter) {
  return adapter && Array.isArray(adapter.tasks) ? adapter.tasks : [];
}

function sortByCurrentOrder(nodeIds, currentOrder) {
  return [...nodeIds].sort((left, right) => {
    const leftIndex = currentOrder.has(left) ? currentOrder.get(left) : Number.MAX_SAFE_INTEGER;
    const rightIndex = currentOrder.has(right) ? currentOrder.get(right) : Number.MAX_SAFE_INTEGER;
    if (leftIndex !== rightIndex) return leftIndex - rightIndex;
    return left.localeCompare(right);
  });
}

function sorted(values) {
  return [...values].sort((left, right) => left.localeCompare(right));
}

module.exports = {
  TURBO_TASKS_ADAPTER_SCHEMA,
  TURBO_TASKS_ADAPTER_DIFF_SCHEMA,
  createTurboTasksAdapterPlan,
  diffTurboTasksAdapterPlans,
};
