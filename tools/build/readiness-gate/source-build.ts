function sourceBuildSummary(sourceBuild) {
  const summary = sourceBuild.summary || {};
  const nodeModulesPaths = sourceBuildNodeModulesPaths(sourceBuild);

  return {
    assets: summary.assets || 0,
    contentDocuments: summary.content_documents || 0,
    cssAssetReferences: summary.css_asset_references || 0,
    cssFlattenedImports: summary.css_flattened_imports || 0,
    cssMinifiedStyles: summary.css_minified_styles || 0,
    cssOriginalRules: summary.css_original_rules || 0,
    cssPrunedRules: summary.css_pruned_rules || 0,
    cssRetainedImports: summary.css_retained_imports || 0,
    cssRetainedRules: summary.css_retained_rules || 0,
    cssSourceMaps: summary.css_source_maps || 0,
    cssSourceMapSources: summary.css_source_map_sources || 0,
    imageAssets: summary.image_assets || 0,
    imageMetadataAssets: summary.image_metadata_assets || 0,
    imagePlaceholders: summary.image_placeholders || 0,
    imageRouteReferences: summary.image_route_references || 0,
    manifestSchema: summary.manifest_schema || null,
    mdxDocuments: summary.mdx_documents || 0,
    nodeModulesPathCount: nodeModulesPaths.length,
    nodeModulesPaths,
    nodeModulesRequired: sourceBuild.node_modules_required ?? null,
    optimizedImageVariants: summary.optimized_image_variants || 0,
    routeHandlerReceiptOutput: summary.route_handler_receipt_output || null,
    routeHandlerReceiptsExecuted: summary.route_handler_receipts_executed || 0,
    routeHandlerReceiptsSkipped: summary.route_handler_receipts_skipped || 0,
    routeHandlerReceiptsNodeModulesRequired:
      summary.route_handler_receipts_node_modules_required ?? null,
    routeHandlerReceiptsLifecycleScriptsExecuted:
      summary.route_handler_receipts_lifecycle_scripts_executed ?? null,
    routeHandlers: summary.route_handlers || 0,
    routeOutputs: summary.route_outputs || 0,
    routes: summary.routes || 0,
    sourceModuleChunks: summary.source_module_chunks || 0,
    styles: summary.styles || 0,
  };
}

function sourceBuildUsesNodeModules(sourceBuild) {
  return (
    sourceBuild.node_modules_required === true ||
    sourceBuild.summary?.route_handler_receipts_node_modules_required === true ||
    sourceBuildNodeModulesPaths(sourceBuild).length > 0
  );
}

function sourceBuildHasUnsafeRouteHandlerReceipts(sourceBuild) {
  return sourceBuild.summary?.route_handler_receipts_lifecycle_scripts_executed === true;
}

function sourceBuildBlockers(sourceBuild) {
  const summary = sourceBuildSummary(sourceBuild);
  const blockers = [];
  if (summary.nodeModulesRequired === true) {
    blockers.push("source build receipt requires node_modules");
  }
  if (summary.routeHandlerReceiptsNodeModulesRequired === true) {
    blockers.push("route-handler receipt collection requires node_modules");
  }
  if (summary.routeHandlerReceiptsLifecycleScriptsExecuted === true) {
    blockers.push("route-handler receipt collection executed lifecycle scripts");
  }
  if (summary.nodeModulesPathCount > 0) {
    blockers.push("source build receipt contains node_modules paths");
  }
  return blockers;
}

function sourceBuildNodeModulesPaths(sourceBuild) {
  const paths = new Set();
  collectNodeModulesPaths(sourceBuild, paths);
  return [...paths].sort();
}

function collectNodeModulesPaths(value, paths) {
  if (typeof value === "string") {
    const normalized = normalizePath(value);
    if (/(^|\/)node_modules(\/|$)/.test(normalized)) {
      paths.add(normalized);
    }
    return;
  }

  if (Array.isArray(value)) {
    for (const item of value) {
      collectNodeModulesPaths(item, paths);
    }
    return;
  }

  if (!value || typeof value !== "object") {
    return;
  }

  for (const item of Object.values(value)) {
    collectNodeModulesPaths(item, paths);
  }
}

function normalizePath(value) {
  return value.replace(/\\/g, "/");
}

module.exports = {
  sourceBuildBlockers,
  sourceBuildHasUnsafeRouteHandlerReceipts,
  sourceBuildNodeModulesPaths,
  sourceBuildSummary,
  sourceBuildUsesNodeModules,
};
