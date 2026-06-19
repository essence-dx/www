function contentPipeline(summary) {
  return {
    document_count: summary.contentDocuments,
    mdx_document_count: summary.mdxDocuments,
    node_modules_required: false,
    runtime_proof: false,
    full_mdx_pipeline_parity: false,
    status: "source-metadata-only",
  };
}

function imagePipeline(summary) {
  return {
    image_asset_count: summary.imageAssets,
    metadata_asset_count: summary.imageMetadataAssets,
    optimized_variant_count: summary.optimizedImageVariants,
    placeholder_count: summary.imagePlaceholders,
    route_reference_count: summary.imageRouteReferences,
    full_pipeline_parity: false,
  };
}

function readinessGraph(summary, ready, nodeModulesRequired) {
  return {
    ready,
    routes: summary.routes,
    route_handlers: summary.routeHandlers,
    route_handler_receipt_output: summary.routeHandlerReceiptOutput,
    route_handler_receipts_executed: summary.routeHandlerReceiptsExecuted,
    route_handler_receipts_skipped: summary.routeHandlerReceiptsSkipped,
    route_handler_receipts_node_modules_required:
      summary.routeHandlerReceiptsNodeModulesRequired,
    route_handler_receipts_lifecycle_scripts_executed:
      summary.routeHandlerReceiptsLifecycleScriptsExecuted,
    route_outputs: summary.routeOutputs,
    source_module_chunks: summary.sourceModuleChunks,
    styles: summary.styles,
    content_documents: summary.contentDocuments,
    mdx_documents: summary.mdxDocuments,
    css_original_rules: summary.cssOriginalRules,
    css_retained_rules: summary.cssRetainedRules,
    css_pruned_rules: summary.cssPrunedRules,
    css_minified_styles: summary.cssMinifiedStyles,
    css_source_maps: summary.cssSourceMaps,
    css_source_map_sources: summary.cssSourceMapSources,
    css_flattened_imports: summary.cssFlattenedImports,
    css_retained_imports: summary.cssRetainedImports,
    css_asset_references: summary.cssAssetReferences,
    assets: summary.assets,
    image_assets: summary.imageAssets,
    image_metadata_assets: summary.imageMetadataAssets,
    optimized_image_variants: summary.optimizedImageVariants,
    image_placeholders: summary.imagePlaceholders,
    image_route_references: summary.imageRouteReferences,
    node_modules_required: nodeModulesRequired,
    node_modules_path_count: summary.nodeModulesPathCount,
    node_modules_paths: summary.nodeModulesPaths,
  };
}

function nextRustSummary(receipt) {
  const snapshot = receipt.value?.snapshot || {};
  const boundary = snapshot.boundary || {};
  const claimPolicy = snapshot.claimPolicy || {};

  return {
    status: receipt.value?.status || snapshot.status || "not-present",
    consumer_receipt: receipt.path,
    upstream_repository: snapshot.upstream?.repository || null,
    upstream_commit: snapshot.upstream?.commit || null,
    workspace_quarantined: boundary.workspaceQuarantined === true,
    runtime_takeover_blocked: boundary.runtimeTakeoverBlocked === true,
    full_nextjs_parity_claimed: claimPolicy.fullNextParityClaimed === true,
    next_runtime_takeover_claimed: claimPolicy.nextRuntimeTakeoverClaimed === true,
    node_modules_default_claimed:
      boundary.nodeModulesDefault === true || claimPolicy.nodeModulesDefaultClaimed === true,
    turbopack_public_architecture: boundary.turbopackPublicArchitecture === true,
  };
}

function styleOptimization(summary) {
  return {
    original_rule_count: summary.cssOriginalRules,
    retained_rule_count: summary.cssRetainedRules,
    pruned_rule_count: summary.cssPrunedRules,
    minified_style_count: summary.cssMinifiedStyles,
    source_map_count: summary.cssSourceMaps,
    source_map_source_count: summary.cssSourceMapSources,
    flattened_import_count: summary.cssFlattenedImports,
    retained_import_count: summary.cssRetainedImports,
    asset_reference_count: summary.cssAssetReferences,
  };
}

module.exports = {
  contentPipeline,
  imagePipeline,
  nextRustSummary,
  readinessGraph,
  styleOptimization,
};
