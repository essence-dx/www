const fs = require("node:fs");
const path = require("node:path");

const TURBOPACK_CORE_DX_GRAPH_CONCEPTS = Object.freeze([
  {
    upstreamConcept: "ModuleGraph",
    vendorPaths: [
      "vendor/next-rust/turbopack/crates/turbopack-core/src/module_graph/mod.rs",
    ],
    dxContracts: ["dx.build.graph", "dx.www.moduleGraph"],
    dxNodeKinds: [
      "route-shell-chunk",
      "source-module",
      "source-module-chunk",
      "tsx-component",
      "tsx-route",
    ],
    dxEdgeKinds: ["imports", "imports-source-module", "links-entry-module"],
    dxReceiptFields: ["graph.nodes", "graph.edges", "invalidation"],
    boundary:
      "adapter-boundary: graph concepts inform DX receipts; Turbopack is not the public architecture",
    nodeModulesRequired: false,
  },
  {
    upstreamConcept: "Module",
    vendorPaths: [
      "vendor/next-rust/turbopack/crates/turbopack-core/src/module.rs",
    ],
    dxContracts: ["dx.www.moduleGraph"],
    dxNodeKinds: ["source-module", "source-module-chunk", "tsx-component", "tsx-route"],
    dxEdgeKinds: ["imports-source-module"],
    dxReceiptFields: ["route_outputs[].source_module_chunks"],
    boundary:
      "source-owned module chunks stay DX runtime metadata, not React/RSC modules",
    nodeModulesRequired: false,
  },
  {
    upstreamConcept: "ModuleReference",
    vendorPaths: [
      "vendor/next-rust/turbopack/crates/turbopack-core/src/reference/mod.rs",
    ],
    dxContracts: ["dx.build.graph", "dx.www.moduleGraph"],
    dxNodeKinds: [],
    dxEdgeKinds: ["imports", "imports-source-module", "links-entry-module"],
    dxReceiptFields: ["graph.edges", "source_module_chunks[].dependencies"],
    boundary:
      "DX resolver and Forge source rules decide references; Node resolution is compatibility-only",
    nodeModulesRequired: false,
  },
  {
    upstreamConcept: "Asset",
    vendorPaths: [
      "vendor/next-rust/turbopack/crates/turbopack-core/src/asset.rs",
    ],
    dxContracts: ["dx.build.graph"],
    dxNodeKinds: ["dx-style-css", "public-asset"],
    dxEdgeKinds: ["imports"],
    dxReceiptFields: ["styles", "assets"],
    boundary:
      "dx-style and hashed public assets stay source-owned receipt nodes",
    nodeModulesRequired: false,
  },
  {
    upstreamConcept: "OutputAsset",
    vendorPaths: [
      "vendor/next-rust/turbopack/crates/turbopack-core/src/output.rs",
    ],
    dxContracts: ["dx.build.graph", "dx.www.moduleGraph"],
    dxNodeKinds: ["deploy-output", "route-shell-chunk"],
    dxEdgeKinds: ["emits", "emitted-from", "links-entry-module"],
    dxReceiptFields: ["route_outputs", "graph.nodes"],
    boundary:
      "route shells and deploy outputs remain DX-owned outputs, not Next runtime assets",
    nodeModulesRequired: false,
  },
  {
    upstreamConcept: "ChunkingContext",
    vendorPaths: [
      "vendor/next-rust/turbopack/crates/turbopack-core/src/chunk/mod.rs",
    ],
    dxContracts: ["dx.www.moduleGraph"],
    dxNodeKinds: ["route-shell-chunk", "source-module-chunk"],
    dxEdgeKinds: ["links-entry-module", "imports-source-module"],
    dxReceiptFields: [
      "route_outputs[].shell_chunk_output",
      "route_outputs[].source_module_chunks",
    ],
    boundary:
      "DX emits browser-executable metadata chunks without adopting Turbopack chunk runtime",
    nodeModulesRequired: false,
  },
  {
    upstreamConcept: "SourceOwnedInvalidation",
    vendorPaths: [
      "vendor/next-rust/turbopack/crates/turbopack-core/src/module_graph/mod.rs",
    ],
    dxContracts: ["dx.build.graph"],
    dxNodeKinds: ["dx-check-receipt"],
    dxEdgeKinds: ["checks"],
    dxReceiptFields: [
      "invalidation.changedNodeIds",
      "invalidation.affectedNodeIds",
      "invalidation.rebuildNodeIds",
    ],
    boundary:
      "DX invalidation walks receipt edges and does not expose turbo-tasks as architecture",
    nodeModulesRequired: false,
  },
  {
    upstreamConcept: "ForgeSourceSurface",
    vendorPaths: [
      "vendor/next-rust/turbopack/crates/turbopack-core/src/module_graph/mod.rs",
    ],
    dxContracts: ["dx.forge.sourceGraph"],
    dxNodeKinds: ["forge-surface"],
    dxEdgeKinds: ["expects-receipt", "owns-source"],
    dxReceiptFields: [".dx/forge/source-.dx/build-cache/manifest.json"],
    boundary:
      "Forge package ownership is a DX source model; Turbopack has no authority over it",
    nodeModulesRequired: false,
  },
]);

function summarizeTurbopackCoreConceptMap(concepts) {
  const coveredNodeKinds = sortedUnique(flatMap(concepts, "dxNodeKinds"));
  const coveredEdgeKinds = sortedUnique(flatMap(concepts, "dxEdgeKinds"));
  return {
    conceptCount: concepts.length,
    coveredNodeKinds,
    coveredEdgeKinds,
    nodeModulesRequired: concepts.some((concept) => concept.nodeModulesRequired === true),
    adapterBoundary: true,
    publicArchitecture: false,
  };
}

function validateTurbopackCoreConceptMap(projectRoot, concepts = TURBOPACK_CORE_DX_GRAPH_CONCEPTS) {
  const absoluteRoot = path.resolve(projectRoot);
  const missingVendorPaths = [];
  const conceptsWithNodeModules = [];
  const conceptsWithoutBoundary = [];
  const conceptsWithPublicOverclaim = [];

  for (const concept of concepts) {
    for (const vendorPath of concept.vendorPaths || []) {
      const absoluteVendorPath = path.join(absoluteRoot, vendorPath);
      if (!fs.existsSync(absoluteVendorPath)) {
        missingVendorPaths.push(vendorPath);
      }
    }
    if (concept.nodeModulesRequired === true) {
      conceptsWithNodeModules.push(concept.upstreamConcept);
    }
    if (typeof concept.boundary !== "string" || concept.boundary.trim() === "") {
      conceptsWithoutBoundary.push(concept.upstreamConcept);
      continue;
    }
    if (isPublicArchitectureOverclaim(concept.boundary)) {
      conceptsWithPublicOverclaim.push(concept.upstreamConcept);
    }
  }

  return {
    schema: "dx.build.graph.turbopackCoreConceptMapValidation",
    format: 1,
    conceptCount: concepts.length,
    upstreamConcepts: sortedUnique(concepts.map((concept) => concept.upstreamConcept)),
    coveredNodeKinds: sortedUnique(flatMap(concepts, "dxNodeKinds")),
    coveredEdgeKinds: sortedUnique(flatMap(concepts, "dxEdgeKinds")),
    missingVendorPaths: sortedUnique(missingVendorPaths),
    conceptsWithNodeModules: sortedUnique(conceptsWithNodeModules),
    conceptsWithoutBoundary: sortedUnique(conceptsWithoutBoundary),
    conceptsWithPublicOverclaim: sortedUnique(conceptsWithPublicOverclaim),
    nodeModulesRequired: conceptsWithNodeModules.length > 0,
    adapterBoundary:
      conceptsWithoutBoundary.length === 0 && conceptsWithPublicOverclaim.length === 0,
    publicArchitecture: false,
    turbopackPublicDependency: false,
  };
}

function isPublicArchitectureOverclaim(boundary) {
  const normalizedBoundary = boundary
    .replace(/[-_]+/g, " ")
    .replace(/\s+/g, " ")
    .trim()
    .toLowerCase();
  const requiresCoreAppModel =
    /\b(required|requires|default|required as|become|becomes|foundation|core app model|core dependency|default foundation)\b/i;

  return [
    /\bfull\s+Turbopack\s+parity\b/i,
    /\bfull\s+Next(?:\.js)?\s+parity\b/i,
    /\bNext\s+runtime\s+takeover\b/i,
    /\bNext(?:\.js)?\s+runtime\s+replaces\s+DX\s+runtime\b/i,
    /\bReact\/RSC\s+required\b/i,
    /\bNode\/NAPI\s+foundation\b/i,
    /\bnode_modules\s+required\b/i,
    /\bTurbopack\s+is\s+the\s+public\s+graph\s+model\b/i,
    /\bTurbopack\s+becomes\s+the\s+public\s+architecture\b/i,
  ].some((pattern) => pattern.test(boundary)) ||
    (
      normalizedBoundary.includes("react") &&
      normalizedBoundary.includes("rsc") &&
      requiresCoreAppModel.test(boundary)
    ) ||
    (
      normalizedBoundary.includes("node") &&
      normalizedBoundary.includes("napi") &&
      requiresCoreAppModel.test(boundary)
    );
}

function flatMap(items, fieldName) {
  return items.flatMap((item) => (Array.isArray(item[fieldName]) ? item[fieldName] : []));
}

function sortedUnique(values) {
  return [...new Set(values)].sort((left, right) => left.localeCompare(right));
}

module.exports = {
  TURBOPACK_CORE_DX_GRAPH_CONCEPTS,
  summarizeTurbopackCoreConceptMap,
  validateTurbopackCoreConceptMap,
};
