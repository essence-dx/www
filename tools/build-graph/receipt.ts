const fs = require("node:fs");
const path = require("node:path");

const {
  CONTRACT_NAMES,
  PROVENANCE,
  normalizeRelativePath,
  sortById,
  sortEdges,
} = require("./types.ts");
const { scanProject } = require("./scanner.ts");
const { computeInvalidation, resolveGraph } = require("./resolver.ts");
const { TURBOPACK_CORE_DX_GRAPH_CONCEPTS } = require("./turbopack-core-map.ts");
const { createTurboTasksAdapterPlan } = require("./turbo-tasks-adapter.ts");

function scanDxBuildGraph(projectRoot, options = {}) {
  const absoluteRoot = path.resolve(projectRoot);
  const scanned = scanProject(absoluteRoot);
  const graph = resolveGraph(absoluteRoot, scanned);
  const nodeByPath = new Map(graph.nodes.map((node) => [node.path, node]));
  const changedNodeIds = (options.changedPaths || [])
    .map((changedPath) => normalizeRelativePath(absoluteRoot, changedPath))
    .map((relativePath) => nodeByPath.get(relativePath))
    .filter(Boolean)
    .map((node) => node.id);
  const invalidation = computeInvalidation(graph, changedNodeIds);

  return {
    schema: CONTRACT_NAMES.buildGraph,
    format: 1,
    generatedAt: new Date().toISOString(),
    projectRoot: absoluteRoot,
    names: CONTRACT_NAMES,
    positioning: {
      turbopackPublicDependency: false,
      turbopackCoreAdapterBoundary: true,
      turborepoWorkspaceModel: false,
      dxSourceOwnedReceiptGraph: true,
    },
    consumers: {
      dxCli: "dx graph --json",
      dxWww: ".dx/receipts/graph/latest.json",
      zedPreview: "read graph.nodes, graph.edges, and invalidation.affectedNodeIds",
    },
    graph: {
      nodes: sortById(graph.nodes),
      edges: sortEdges(graph.edges),
    },
    invalidation,
    coreConceptMap: TURBOPACK_CORE_DX_GRAPH_CONCEPTS,
    turboTasksAdapter: createTurboTasksAdapterPlan(absoluteRoot, graph, invalidation),
    provenance: PROVENANCE,
  };
}

function writeDxBuildGraphReceipt(projectRoot, receiptPath, options = {}) {
  const report = scanDxBuildGraph(projectRoot, options);
  const absoluteReceiptPath = path.resolve(receiptPath);
  const withPath = { ...report, receiptPath: absoluteReceiptPath };
  fs.mkdirSync(path.dirname(absoluteReceiptPath), { recursive: true });
  fs.writeFileSync(absoluteReceiptPath, `${JSON.stringify(withPath, null, 2)}\n`);
  return withPath;
}

module.exports = {
  scanDxBuildGraph,
  writeDxBuildGraphReceipt,
};
