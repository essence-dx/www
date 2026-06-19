const path = require("node:path");

const { normalizeRelativePath, sortEdges, toPosixPath } = require("./types.ts");

const RESOLUTION_EXTENSIONS = ["", ".tsx", ".ts", ".jsx", ".js", ".css", ".json"];
const INDEX_EXTENSIONS = [".tsx", ".ts", ".jsx", ".js"];
const REBUILDABLE_NODE_KINDS = new Set([
  "dx-style-css",
  "source-module",
  "tsx-component",
  "tsx-route",
]);

function resolveGraph(projectRoot, scanned) {
  const edges = [...scanned.edges];

  for (const rawImport of scanned.rawImports) {
    const resolved = resolveImport(projectRoot, rawImport.from.path, rawImport.specifier);
    if (!resolved) continue;
    const target = scanned.nodesByPath.get(resolved);
    if (target) {
      edges.push({
        from: rawImport.from.id,
        to: target.id,
        kind: "imports",
        specifier: rawImport.specifier,
      });
    }
  }

  return {
    nodes: scanned.nodes,
    edges: dedupeEdges(edges),
  };
}

function resolveImport(projectRoot, importerPath, specifier) {
  const cleanSpecifier = stripUrlSuffix(specifier);
  if (cleanSpecifier.startsWith("/")) {
    const publicPath = toPosixPath(path.join("public", cleanSpecifier.slice(1)));
    return pathExists(projectRoot, publicPath) ? publicPath : null;
  }

  const base = importBasePath(projectRoot, importerPath, cleanSpecifier);
  const candidates = [];

  for (const extension of RESOLUTION_EXTENSIONS) {
    candidates.push(`${base}${extension}`);
  }
  for (const extension of INDEX_EXTENSIONS) {
    candidates.push(toPosixPath(path.join(base, `index${extension}`)));
  }

  return candidates.find((candidate) => pathExists(projectRoot, candidate)) || null;
}

function importBasePath(projectRoot, importerPath, specifier) {
  if (specifier.startsWith("@/")) {
    return toPosixPath(specifier.slice(2));
  }

  const importerDir = path.dirname(importerPath);
  return normalizeRelativePath(projectRoot, path.join(importerDir, specifier));
}

function stripUrlSuffix(specifier) {
  return specifier.split(/[?#]/, 1)[0];
}

function pathExists(projectRoot, relativePath) {
  return require("node:fs").existsSync(path.join(projectRoot, relativePath));
}

function computeInvalidation(graph, changedNodeIds) {
  const nodesById = new Map(graph.nodes.map((node) => [node.id, node]));
  const incoming = new Map();
  for (const edge of graph.edges) {
    if (!incoming.has(edge.to)) incoming.set(edge.to, []);
    incoming.get(edge.to).push(edge.from);
  }
  for (const parents of incoming.values()) {
    parents.sort();
  }

  const affected = [];
  const seen = new Set();
  const queue = [...changedNodeIds];
  while (queue.length > 0) {
    const current = queue.shift();
    if (seen.has(current)) continue;
    seen.add(current);
    affected.push(current);
    for (const parent of incoming.get(current) || []) {
      if (!seen.has(parent)) queue.push(parent);
    }
  }

  return {
    changedNodeIds,
    affectedNodeIds: affected,
    rebuildNodeIds: affected.filter((nodeId) => {
      const node = nodesById.get(nodeId);
      return node && REBUILDABLE_NODE_KINDS.has(node.kind);
    }),
  };
}

function dedupeEdges(edges) {
  const seen = new Set();
  const unique = [];
  for (const edge of edges) {
    const key = `${edge.from}\0${edge.to}\0${edge.kind}\0${edge.specifier || ""}`;
    if (seen.has(key)) continue;
    seen.add(key);
    unique.push(edge);
  }
  return sortEdges(unique);
}

module.exports = {
  computeInvalidation,
  importBasePath,
  resolveGraph,
  resolveImport,
};
