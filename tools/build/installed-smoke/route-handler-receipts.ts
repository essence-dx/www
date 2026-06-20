const { relativePath } = require("./io.ts");
const { routeHandlerReceiptFailures } = require("./route-handler-receipt-failures.ts");
const { summarizeRouteHandlerReceipt, summarizeSkippedRouteHandler } = require("./route-handler-receipt-summary.ts");
const { summarizeUnexpectedRouteHandlerEvidence } = require("./route-handler-unexpected-evidence.ts");
const {
  PRIMARY_ROUTE_HANDLER,
  REQUIRED_ROUTE_HANDLERS,
  normalizeRouteHandlerRequirement,
} = require("./route-handler-requirements.ts");

function summarizeRouteHandlerReceipts(input, sourcePath = PRIMARY_ROUTE_HANDLER.sourcePath) {
  const artifact = input.routeHandlerReceipts || { ok: false, value: null };
  const collection = artifact.value || {};
  const receipts = Array.isArray(collection.receipts) ? collection.receipts : [];
  const skipped = Array.isArray(collection.skipped) ? collection.skipped : [];
  const requiredReceipts = REQUIRED_ROUTE_HANDLERS
    .filter((handler) => handler.buildReceipt !== "skipped")
    .map((handler) => summarizeRouteHandlerReceipt(receipts, handler));
  const requiredSkips = REQUIRED_ROUTE_HANDLERS
    .filter((handler) => handler.buildReceipt === "skipped")
    .map((handler) => summarizeSkippedRouteHandler(skipped, handler));
  const unexpectedReceipts = summarizeUnexpectedRouteHandlerEvidence(
    receipts,
    REQUIRED_ROUTE_HANDLERS.filter((handler) => handler.buildReceipt !== "skipped"),
    { includeRuntimeBoundary: true },
  );
  const unexpectedSkips = summarizeUnexpectedRouteHandlerEvidence(
    skipped,
    REQUIRED_ROUTE_HANDLERS.filter((handler) => handler.buildReceipt === "skipped"),
    { includeSkipReason: true },
  );
  const routeHandlerReadiness = summarizeRouteHandlerReadiness(requiredReceipts, requiredSkips);
  const primary = normalizeRouteHandlerRequirement(sourcePath);
  const receipt =
    requiredReceipts.find((item) => item.sourcePath === primary.sourcePath) ||
    requiredReceipts[0] ||
    summarizeRouteHandlerReceipt(receipts, primary);

  return {
    present: artifact.ok,
    path: input.routeHandlerReceiptsPath
      ? relativePath(input.projectRoot, input.routeHandlerReceiptsPath)
      : ".dx/build/.dx/build-cache/route-handler-receipts.json",
    collectionSchema: collection.schema || null,
    collectionFormat: collection.format ?? null,
    collectionDeclaresNoNodeModules: collection.node_modules_required === false,
    collectionNodeModulesPresent: collection.node_modules_present === true,
    collectionLifecycleScriptsExecuted: collection.lifecycle_scripts_executed === true,
    receiptCount: receipts.length,
    skippedCount: skipped.length,
    requiredReceipts,
    requiredReceiptCount: requiredReceipts.filter((item) => item.present).length,
    requiredSkips,
    requiredSkipCount: requiredSkips.filter((item) => item.present).length,
    unexpectedReceipts,
    unexpectedSkips,
    routeHandlerReadiness,
    hasHealthGetReceipt: requiredReceipts.some((item) => item.key === "healthGet" && item.present),
    hasCheckoutPostReceipt: requiredReceipts.some((item) => item.key === "checkoutPost" && item.present),
    hasCheckoutPostSkipped: requiredSkips.some((item) => item.key === "checkoutPost" && item.present),
    schema: receipt.schema,
    format: receipt.format,
    executionModel: receipt.executionModel,
    declaresNoNodeModules: receipt.declaresNoNodeModules,
    nodeModulesRequired: receipt.nodeModulesRequired,
    nodeModulesPresent: receipt.nodeModulesPresent,
    lifecycleScriptsExecuted: receipt.lifecycleScriptsExecuted,
    sourceOwnedRuntimeBoundary: receipt.sourceOwnedRuntimeBoundary,
    externalRuntimeRequired: receipt.externalRuntimeRequired,
    externalRuntimeExecuted: receipt.externalRuntimeExecuted,
    responseStatus: receipt.responseStatus,
    responseContentType: receipt.responseContentType,
    responseHeaderCount: receipt.responseHeaderCount,
    hasAdapterBoundary: receipt.hasAdapterBoundary,
    adapterBoundaryCount: receipt.adapterBoundaryCount,
  };
}

function summarizeRouteHandlerReadiness(requiredReceipts, requiredSkips) {
  return REQUIRED_ROUTE_HANDLERS.map((handler) => {
    const receipt = requiredReceipts.find((item) => item.key === handler.key);
    if (receipt) {
      return {
        key: handler.key,
        sourcePath: handler.sourcePath,
        route: handler.route,
        method: handler.method,
        buildStatus: receipt.present ? "executed" : "missing",
        expectedStatus: handler.expectedStatus,
        responseStatus: receipt.present ? receipt.responseStatus : null,
        skipReason: null,
        duplicateCount: receipt.duplicateCount,
        sourceOwnedRuntimeBoundary: receipt.present ? receipt.sourceOwnedRuntimeBoundary : null,
        externalRuntimeRequired: receipt.present ? receipt.externalRuntimeRequired : null,
        externalRuntimeExecuted: receipt.present ? receipt.externalRuntimeExecuted : null,
        declaresNoNodeModules: receipt.present ? receipt.declaresNoNodeModules : null,
        lifecycleScriptsExecuted: receipt.present ? receipt.lifecycleScriptsExecuted : null,
      };
    }

    const skipped = requiredSkips.find((item) => item.key === handler.key);
    return {
      key: handler.key,
      sourcePath: handler.sourcePath,
      route: handler.route,
      method: handler.method,
      buildStatus: skipped?.present ? "skipped" : "missing",
      expectedStatus: handler.expectedStatus,
      responseStatus: null,
      skipReason: skipped?.reason || null,
      duplicateCount: skipped?.duplicateCount ?? 0,
      sourceOwnedRuntimeBoundary: null,
      externalRuntimeRequired: null,
      externalRuntimeExecuted: null,
      declaresNoNodeModules: null,
      lifecycleScriptsExecuted: null,
    };
  });
}

module.exports = {
  routeHandlerReceiptFailures,
  summarizeRouteHandlerReceipts,
};
