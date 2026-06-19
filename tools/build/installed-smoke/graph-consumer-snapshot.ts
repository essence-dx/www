function summarizeGraphConsumerSnapshot(snapshot) {
  const value = snapshot.value || {};
  const graph = value.graph || {};
  const nodeKindCounts = graph.nodeKindCounts || {};
  const coreConceptMap = value.coreConceptMap || {};
  const coveredNodeKinds = Array.isArray(coreConceptMap.coveredNodeKinds)
    ? coreConceptMap.coveredNodeKinds
    : [];
  const coveredEdgeKinds = Array.isArray(coreConceptMap.coveredEdgeKinds)
    ? coreConceptMap.coveredEdgeKinds
    : [];
  const zedPreview = value.consumers?.zedPreview || {};

  return {
    present: snapshot.ok,
    schema: snapshot.ok ? value.schema || null : null,
    sourceModuleCount: countKind(nodeKindCounts, "source-module"),
    sourceModuleChunkCount: countKind(nodeKindCounts, "source-module-chunk"),
    coversSourceModuleKind: coveredNodeKinds.includes("source-module"),
    coversCompiledFromSourceEdge: coveredEdgeKinds.includes("compiled-from-source"),
    zedPreviewSourceModuleKind: zedPreview.sourceModuleKind || null,
    zedPreviewSourceModuleChunkKind: zedPreview.sourceModuleChunkKind || null,
  };
}

function graphConsumerSnapshotFailures(snapshot) {
  const failures = [];
  pushIf(failures, !snapshot.present, "dx build did not write .dx/receipts/graph/consumer-snapshot.json");
  if (!snapshot.present) {
    return failures;
  }
  pushIf(
    failures,
    snapshot.schema !== "dx.build.graph.consumerSnapshot",
    "dx.build.graph consumer snapshot has an unexpected schema",
  );
  pushIf(
    failures,
    snapshot.sourceModuleCount < 2,
    "dx.build.graph consumer snapshot is missing source-module counts",
  );
  pushIf(
    failures,
    !snapshot.coversSourceModuleKind,
    "dx.build.graph consumer snapshot is missing source-module coverage",
  );
  pushIf(
    failures,
    !snapshot.coversCompiledFromSourceEdge,
    "dx.build.graph consumer snapshot is missing compiled-from-source coverage",
  );
  pushIf(
    failures,
    snapshot.zedPreviewSourceModuleKind !== "source-module",
    "dx.build.graph consumer snapshot is missing Zed source-module kind metadata",
  );
  return failures;
}

function countKind(nodeKindCounts, kind) {
  const count = nodeKindCounts[kind];
  return Number.isFinite(count) ? count : 0;
}

function pushIf(failures, condition, message) {
  if (condition) failures.push(message);
}

module.exports = {
  graphConsumerSnapshotFailures,
  summarizeGraphConsumerSnapshot,
};
