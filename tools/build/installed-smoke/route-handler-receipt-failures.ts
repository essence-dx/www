const { legacyRouteHandlerReceiptSummary } = require("./route-handler-receipt-summary.ts");

const ROUTE_HANDLER_RECEIPT_COLLECTION_SCHEMA = "dx.next.appRouteHandlerBuildReceipts";
const ROUTE_HANDLER_RECEIPT_SCHEMA = "dx.next.appRouteHandlerReceipt";

function routeHandlerReceiptFailures(routeHandlerReceipt) {
  if (!routeHandlerReceipt.present) {
    return ["dx build did not write .dx/build/route-handler-receipts.json"];
  }

  const failures = collectionFailures(routeHandlerReceipt);
  const requiredReceipts = Array.isArray(routeHandlerReceipt.requiredReceipts)
    ? routeHandlerReceipt.requiredReceipts
    : [legacyRouteHandlerReceiptSummary(routeHandlerReceipt)];
  for (const receipt of requiredReceipts) {
    failures.push(...receiptFailures(receipt));
  }
  const requiredSkips = Array.isArray(routeHandlerReceipt.requiredSkips)
    ? routeHandlerReceipt.requiredSkips
    : [];
  for (const skipped of requiredSkips) {
    failures.push(...skipFailures(skipped));
  }
  const unexpectedReceipts = Array.isArray(routeHandlerReceipt.unexpectedReceipts)
    ? routeHandlerReceipt.unexpectedReceipts
    : [];
  for (const receipt of unexpectedReceipts) {
    failures.push(
      `.dx/build/route-handler-receipts.json has unexpected stale ${routeHandlerReceiptIdentity(receipt)} receipt evidence`,
    );
  }
  const unexpectedSkips = Array.isArray(routeHandlerReceipt.unexpectedSkips)
    ? routeHandlerReceipt.unexpectedSkips
    : [];
  for (const skipped of unexpectedSkips) {
    failures.push(
      `.dx/build/route-handler-receipts.json has unexpected stale ${routeHandlerReceiptIdentity(skipped)} skipped evidence`,
    );
  }
  return failures;
}

function collectionFailures(routeHandlerReceipt) {
  const failures = [];
  pushIf(failures, routeHandlerReceipt.collectionSchema !== ROUTE_HANDLER_RECEIPT_COLLECTION_SCHEMA, "route-handler receipt collection has an unexpected schema");
  pushIf(failures, routeHandlerReceipt.collectionFormat !== 1, "route-handler receipt collection has an unexpected format");
  pushIf(
    failures,
    !routeHandlerReceipt.collectionDeclaresNoNodeModules,
    "route-handler receipt collection does not declare node_modules_required=false",
  );
  pushIf(failures, routeHandlerReceipt.collectionNodeModulesPresent, "route-handler receipt collection reports node_modules_present=true");
  pushIf(failures, routeHandlerReceipt.collectionLifecycleScriptsExecuted, "route-handler receipt collection executed lifecycle scripts");
  return failures;
}

function skipFailures(skipped) {
  if (skipped.duplicateCount > 0) {
    return [`.dx/build/route-handler-receipts.json has duplicate ${routeHandlerReceiptIdentity(skipped)} skipped evidence`];
  }
  if (skipped.present) {
    return [];
  }
  return [
    `.dx/build/route-handler-receipts.json is missing ${skipped.sourcePath} ${skipped.method} skipped evidence`,
  ];
}

function receiptFailures(receipt) {
  const failures = [];
  if (!receipt.present) {
    failures.push(`.dx/build/route-handler-receipts.json is missing ${receipt.sourcePath} ${receipt.method} receipt`);
    return failures;
  }
  pushIf(
    failures,
    receipt.duplicateCount > 0,
    `.dx/build/route-handler-receipts.json has duplicate ${routeHandlerReceiptIdentity(receipt)} receipts`,
  );
  pushIf(failures, receipt.schema !== ROUTE_HANDLER_RECEIPT_SCHEMA, "route-handler receipt has an unexpected schema");
  pushIf(failures, receipt.format !== 1, "route-handler receipt has an unexpected format");
  pushIf(
    failures,
    receipt.executionModel !== "source-owned-route-handler-contract",
    "route-handler receipt does not use the source-owned execution contract",
  );
  pushIf(
    failures,
    receipt.responseStatus !== receipt.expectedStatus || !isJsonContentType(receipt.responseContentType),
    `route-handler receipt does not describe a ${receipt.expectedStatus} JSON response`,
  );
  pushIf(
    failures,
    typeof receipt.responseHeaderCount !== "number" || receipt.responseHeaderCount < 1,
    "route-handler receipt is missing response header evidence",
  );
  pushIf(failures, !receipt.hasAdapterBoundary, "route-handler receipt is missing adapter-boundary evidence");
  pushIf(failures, !receipt.declaresNoNodeModules, "route-handler receipt does not declare node_modules_required=false");
  pushIf(failures, receipt.nodeModulesRequired, "route-handler receipt requires node_modules");
  pushIf(failures, receipt.nodeModulesPresent, "route-handler receipt reports node_modules_present=true");
  pushIf(failures, receipt.lifecycleScriptsExecuted, "route-handler receipt executed lifecycle scripts");
  const label = routeHandlerReceiptLabel(receipt);
  pushIf(failures, !receipt.sourceOwnedRuntimeBoundary, `${label} does not declare source-owned runtime boundary`);
  pushIf(failures, receipt.externalRuntimeRequired, `${label} requires an external runtime`);
  pushIf(failures, receipt.externalRuntimeExecuted, `${label} executed an external runtime`);
  return failures;
}

function routeHandlerReceiptLabel(receipt) {
  return `.dx/build/route-handler-receipts.json ${receipt.sourcePath} ${receipt.method}`;
}

function routeHandlerReceiptIdentity(receipt) {
  return [receipt.sourcePath, receipt.method, receipt.route].filter(Boolean).join(" ");
}

function pushIf(failures, condition, message) {
  if (condition) {
    failures.push(message);
  }
}

function isJsonContentType(contentType) {
  const mediaType = String(contentType || "")
    .toLowerCase()
    .split(";")[0]
    .trim();
  return mediaType === "application/json" || mediaType.endsWith("+json");
}

module.exports = {
  routeHandlerReceiptFailures,
};
