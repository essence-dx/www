const {
  PRIMARY_ROUTE_HANDLER,
  normalizeRouteHandlerMethod,
  normalizeRouteHandlerRequirement,
  normalizeRouteHandlerRoute,
  normalizeRouteHandlerSourcePath,
} = require("./route-handler-requirements.ts");

function summarizeRouteHandlerReceipt(receipts, requirement) {
  const expected = normalizeRouteHandlerRequirement(requirement);
  const matches = receipts.filter((item) => matchesRouteHandlerRequirement(item, expected));
  const receipt = matches[0];
  const runtimeBoundary = receipt?.runtime_boundary || {};

  return {
    key: expected.key,
    sourcePath: expected.sourcePath,
    route: expected.route,
    method: expected.method,
    expectedStatus: expected.expectedStatus,
    present: Boolean(receipt),
    duplicateCount: Math.max(0, matches.length - 1),
    schema: receipt?.schema || null,
    format: receipt?.format ?? null,
    executionModel: receipt?.execution_model || null,
    declaresNoNodeModules: receipt?.node_modules_required === false,
    nodeModulesRequired: receipt?.node_modules_required === true,
    nodeModulesPresent: receipt?.node_modules_present === true,
    lifecycleScriptsExecuted: receipt?.lifecycle_scripts_executed === true,
    sourceOwnedRuntimeBoundary: runtimeBoundary.source_owned === true,
    externalRuntimeRequired: runtimeBoundary.external_runtime_required === true,
    externalRuntimeExecuted: runtimeBoundary.external_runtime_executed === true,
    responseStatus: receipt?.response?.status ?? null,
    responseContentType: receipt?.response?.content_type || null,
    responseHeaderCount: receipt?.response?.header_count ?? receipt?.response_header_count ?? null,
    hasAdapterBoundary: Array.isArray(receipt?.adapter_boundary) && receipt.adapter_boundary.length > 0,
    adapterBoundaryCount: Array.isArray(receipt?.adapter_boundary)
      ? receipt.adapter_boundary.length
      : 0,
  };
}

function summarizeSkippedRouteHandler(skipped, requirement) {
  const expected = normalizeRouteHandlerRequirement(requirement);
  const matches = skipped.filter((entry) => matchesRouteHandlerRequirement(entry, expected));
  const item = matches[0];
  return {
    key: expected.key,
    sourcePath: expected.sourcePath,
    route: expected.route,
    method: expected.method,
    present: Boolean(item),
    duplicateCount: Math.max(0, matches.length - 1),
    reason: item?.reason || null,
  };
}

function matchesRouteHandlerRequirement(item, expected) {
  return (
    normalizeRouteHandlerSourcePath(item?.source_path) === expected.sourcePath &&
    normalizeRouteHandlerMethod(item?.method) === expected.method &&
    normalizeRouteHandlerRoute(item?.request_path) === expected.route
  );
}

function legacyRouteHandlerReceiptSummary(summary) {
  return {
    sourcePath: PRIMARY_ROUTE_HANDLER.sourcePath,
    route: PRIMARY_ROUTE_HANDLER.route,
    method: PRIMARY_ROUTE_HANDLER.method,
    expectedStatus: PRIMARY_ROUTE_HANDLER.expectedStatus,
    present: summary.hasHealthGetReceipt === true,
    schema: summary.schema,
    format: summary.format,
    executionModel: summary.executionModel,
    declaresNoNodeModules: summary.declaresNoNodeModules,
    nodeModulesRequired: summary.nodeModulesRequired,
    nodeModulesPresent: summary.nodeModulesPresent,
    lifecycleScriptsExecuted: summary.lifecycleScriptsExecuted,
    sourceOwnedRuntimeBoundary: summary.sourceOwnedRuntimeBoundary,
    externalRuntimeRequired: summary.externalRuntimeRequired,
    externalRuntimeExecuted: summary.externalRuntimeExecuted,
    responseStatus: summary.responseStatus,
    responseContentType: summary.responseContentType,
    responseHeaderCount: summary.responseHeaderCount,
    hasAdapterBoundary: summary.hasAdapterBoundary,
  };
}

module.exports = {
  legacyRouteHandlerReceiptSummary,
  summarizeRouteHandlerReceipt,
  summarizeSkippedRouteHandler,
};
