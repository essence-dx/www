const path = require("node:path");

const CONTRACT_NAMES = Object.freeze({
  buildGraph: "dx.build.graph",
  wwwModuleGraph: "dx.www.moduleGraph",
  forgeSourceGraph: "dx.forge.sourceGraph",
});

const PROVENANCE = Object.freeze([
  {
    name: "Turbopack source study",
    upstream: "vercel/next.js",
    license: "MIT",
    copiedCode: false,
    sourceUrls: [
      "https://github.com/vercel/next.js/tree/canary/turbopack",
      "https://github.com/vercel/next.js/blob/canary/turbopack/crates/turbopack-core/src/module_graph/mod.rs",
      "https://github.com/vercel/next.js/blob/canary/turbopack/crates/turbopack-core/src/module.rs",
      "https://github.com/vercel/next.js/blob/canary/turbopack/crates/turbopack-core/src/reference/mod.rs",
      "https://github.com/vercel/next.js/blob/canary/license.md",
    ],
  },
]);

function toPosixPath(value) {
  return value.split(path.sep).join("/").replaceAll("\\", "/");
}

function normalizeRelativePath(projectRoot, candidate) {
  const absolute = path.isAbsolute(candidate)
    ? candidate
    : path.join(projectRoot, candidate);
  return toPosixPath(path.relative(projectRoot, absolute));
}

function nodeId(kind, relativePath) {
  return `${kind}:${toPosixPath(relativePath)}`;
}

function sortById(items) {
  return [...items].sort((left, right) => left.id.localeCompare(right.id));
}

function sortEdges(edges) {
  return [...edges].sort((left, right) => {
    const byFrom = left.from.localeCompare(right.from);
    if (byFrom !== 0) return byFrom;
    const byTo = left.to.localeCompare(right.to);
    if (byTo !== 0) return byTo;
    return left.kind.localeCompare(right.kind);
  });
}

module.exports = {
  CONTRACT_NAMES,
  PROVENANCE,
  nodeId,
  normalizeRelativePath,
  sortById,
  sortEdges,
  toPosixPath,
};
