const path = require("node:path");

const {
  INSTALLED_BINARY_SMOKE_RECEIPT,
  PRODUCT_PREVIEW_SCORE,
  READINESS_RECEIPT,
  ZED_HANDOFF_RECEIPT,
} = require("./constants.ts");
const { normalizePath, writeJson } = require("./io.ts");
const {
  contentPipeline,
  imagePipeline,
  nextRustSummary,
  readinessGraph,
  styleOptimization,
} = require("./projection-sections.ts");
const {
  sourceBuildHasUnsafeRouteHandlerReceipts,
  sourceBuildSummary,
  sourceBuildUsesNodeModules,
} = require("./source-build.ts");

function writeSourceProjection(projectRoot, receipts) {
  assertSourceBuildReceipt(receipts.sourceBuild);
  const readiness = buildReadinessProjection(projectRoot, receipts);
  const handoff = buildZedHandoffProjection(projectRoot, receipts);

  writeJson(path.join(projectRoot, READINESS_RECEIPT), readiness);
  writeJson(path.join(projectRoot, ZED_HANDOFF_RECEIPT), handoff);

  return {
    handoff,
    readiness,
  };
}

function buildReadinessProjection(projectRoot, receipts) {
  const sourceBuild = receipts.sourceBuild.value;
  const summary = sourceBuildSummary(sourceBuild);
  const nextRust = nextRustSummary(receipts.nextRustBoundary);
  const sourceReady =
    sourceBuild.schema === "dx.www.sourceBuildReceipt" &&
    !sourceBuildUsesNodeModules(sourceBuild) &&
    !sourceBuildHasUnsafeRouteHandlerReceipts(sourceBuild);

  return {
    schema: "dx.build.readiness",
    schema_revision: 1,
    status: "source-ready-runtime-governed",
    generated_at: new Date().toISOString(),
    project_root: normalizePath(projectRoot),
    source_ready: sourceReady,
    source_score: sourceReady ? 100 : 0,
    product_ready: false,
    product_score: sourceReady ? PRODUCT_PREVIEW_SCORE : 0,
    product_score_ceiling: sourceReady ? PRODUCT_PREVIEW_SCORE : 0,
    product_score_basis: sourceReady
      ? [
          "source-build-graph-ready",
          "installed-binary-smoke-pending",
          "runtime-proof-pending",
          "live-browser-proof-pending",
        ]
      : ["source-build-receipt-missing-or-node-modules"],
    receipts: projectionReceipts(receipts.sourceBuild.path),
    graph: readinessGraph(summary, sourceReady, sourceBuildUsesNodeModules(sourceBuild)),
    next_rust_merge: nextRust,
    installed_binary_smoke: {
      required: true,
      receipt: INSTALLED_BINARY_SMOKE_RECEIPT,
      status: "pending-governed-refresh",
    },
    runtime_validation: {
      required: true,
      status: "pending-governed-runtime-proof",
      live_hydration_proof: false,
    },
    next_action:
      "Run governed installed-binary smoke and runtime validation before claiming product readiness.",
  };
}

function buildZedHandoffProjection(projectRoot, receipts) {
  const sourceBuild = receipts.sourceBuild.value;
  const summary = sourceBuildSummary(sourceBuild);
  const nextRust = nextRustSummary(receipts.nextRustBoundary);

  return {
    schema: "dx.build.zedHandoff",
    schema_revision: 1,
    status: "source-ready-runtime-governed",
    generated_at: new Date().toISOString(),
    project_root: normalizePath(projectRoot),
    source_build_receipt: receipts.sourceBuild.path,
    canonical_build_receipt: receipts.sourceBuild.path,
    build_readiness: READINESS_RECEIPT,
    installed_binary_smoke_receipt: INSTALLED_BINARY_SMOKE_RECEIPT,
    route_handlers: summary.routeHandlers,
    route_shell_chunks: summary.routeOutputs,
    source_module_chunks: summary.sourceModuleChunks,
    content_pipeline: contentPipeline(summary),
    style_optimization: styleOptimization(summary),
    image_pipeline: imagePipeline(summary),
    node_modules_required: sourceBuildUsesNodeModules(sourceBuild),
    node_modules_path_count: summary.nodeModulesPathCount,
    node_modules_paths: summary.nodeModulesPaths,
    next_rust_merge: {
      consumer_receipt: receipts.nextRustBoundary.path,
      status: nextRust.status,
      runtime_takeover_blocked: nextRust.runtime_takeover_blocked,
      full_nextjs_parity_claimed: nextRust.full_nextjs_parity_claimed,
    },
    next_action:
      "Keep the source projection current, then run governed runtime validation before release.",
  };
}

function projectionReceipts(sourceBuildPath) {
  return {
    source_build_receipt: sourceBuildPath,
    canonical_build_receipt: sourceBuildPath,
    zed_handoff: ZED_HANDOFF_RECEIPT,
    installed_binary_smoke: INSTALLED_BINARY_SMOKE_RECEIPT,
  };
}

function assertSourceBuildReceipt(receipt) {
  if (!receipt.present || receipt.malformed) {
    throw new Error("Cannot write source projection without a valid source-build receipt");
  }
  if (receipt.value?.schema !== "dx.www.sourceBuildReceipt") {
    throw new Error("Cannot write source projection from an unexpected source-build schema");
  }
}

module.exports = {
  writeSourceProjection,
};
