const {
  TURBOPACK_CORE_DX_GRAPH_CONCEPTS,
  validateTurbopackCoreConceptMap,
} = require("./turbopack-core-map.ts");

const TURBOPACK_CORE_CONCEPT_MAP_STATUS_SCHEMA =
  "dx.build.graph.turbopackCoreConceptMapStatus";

function createTurbopackCoreConceptMapStatus(
  projectRoot,
  concepts = TURBOPACK_CORE_DX_GRAPH_CONCEPTS,
) {
  const validation = validateTurbopackCoreConceptMap(projectRoot, concepts);
  const blockingReasons = collectBlockingReasons(validation);

  return {
    schema: TURBOPACK_CORE_CONCEPT_MAP_STATUS_SCHEMA,
    format: 1,
    lane: 3,
    laneName: "Turbopack Core Module Graph",
    status: blockingReasons.length === 0 ? "passing" : "blocked",
    score: scoreCoreConceptMapStatus(validation),
    blockingReasons,
    architecture: {
      dxRuntimeAuthoritative: true,
      publicTurbopackDependency: false,
      reactRequiredCore: false,
      nodeModulesRequired: validation.nodeModulesRequired,
      nodeNapiFoundation: false,
    },
    boundary: {
      sourceOnly: true,
      adapterBoundary: validation.adapterBoundary,
      publicArchitecture: validation.publicArchitecture,
      fullParityProven: false,
      nextRuntimeAdopted: false,
    },
    evidence: {
      conceptCount: validation.conceptCount,
      upstreamConcepts: validation.upstreamConcepts,
      coveredNodeKinds: validation.coveredNodeKinds,
      coveredEdgeKinds: validation.coveredEdgeKinds,
      missingVendorPathCount: validation.missingVendorPaths.length,
      conceptsWithNodeModulesCount: validation.conceptsWithNodeModules.length,
      conceptsWithoutBoundaryCount: validation.conceptsWithoutBoundary.length,
      conceptsWithPublicOverclaimCount:
        validation.conceptsWithPublicOverclaim.length,
    },
    validation,
    recommendedAction:
      blockingReasons.length === 0
        ? "read dx.build.graph.consumerSnapshot.coreConceptMapStatus"
        : "fix Lane 3 concept map validation before promoting graph evidence",
  };
}

function collectBlockingReasons(validation) {
  const reasons = [];
  if (validation.missingVendorPaths.length > 0) {
    reasons.push("missing-vendor-paths");
  }
  if (validation.conceptsWithNodeModules.length > 0) {
    reasons.push("node-modules-required");
  }
  if (validation.conceptsWithoutBoundary.length > 0) {
    reasons.push("missing-adapter-boundary");
  }
  if (validation.conceptsWithPublicOverclaim.length > 0) {
    reasons.push("public-architecture-overclaim");
  }
  return reasons;
}

function scoreCoreConceptMapStatus(validation) {
  const penalty =
    validation.missingVendorPaths.length * 10 +
    validation.conceptsWithNodeModules.length * 25 +
    validation.conceptsWithoutBoundary.length * 15 +
    validation.conceptsWithPublicOverclaim.length * 25;

  return Math.max(0, Math.min(100, 100 - penalty));
}

module.exports = {
  TURBOPACK_CORE_CONCEPT_MAP_STATUS_SCHEMA,
  createTurbopackCoreConceptMapStatus,
};
