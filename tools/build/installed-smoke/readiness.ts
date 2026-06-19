const { INSTALLED_BINARY_SMOKE_RECEIPT } = require("./constants.ts");
const { normalizeArtifactPath, relativePath } = require("./io.ts");

function summarizeReadiness(input) {
  const readiness = input.readiness;
  const graph = readiness.ok && readiness.value ? readiness.value.graph || {} : {};
  const routeHandlerReceiptPath = relativePath(input.projectRoot, input.routeHandlerReceiptsPath);
  const routeHandlerReceiptOutput = graph.route_handler_receipt_output || null;

  return {
    present: readiness.ok,
    schema: readiness.ok ? readiness.value.schema : null,
    sourceReady: readiness.ok ? readiness.value.source_ready === true : false,
    sourceScore: readiness.ok ? readiness.value.source_score : null,
    productReady: readiness.ok ? readiness.value.product_ready === true : false,
    productScore: readiness.ok ? readiness.value.product_score : null,
    hasInstalledBinarySmokeReceipt: hasInstalledBinarySmokeReceipt(readiness),
    routeHandlerReceiptOutput,
    routeHandlerReceiptOutputMatchesActual:
      normalizeArtifactPath(routeHandlerReceiptOutput) ===
      normalizeArtifactPath(routeHandlerReceiptPath),
    routeHandlerReceiptsExecuted: graph.route_handler_receipts_executed ?? null,
    routeHandlerReceiptsSkipped: graph.route_handler_receipts_skipped ?? null,
    routeHandlerReceiptsDeclareNoNodeModules:
      graph.route_handler_receipts_node_modules_required === false,
    routeHandlerReceiptsNodeModulesRequired:
      graph.route_handler_receipts_node_modules_required === true,
    routeHandlerReceiptsDeclareNoLifecycleScripts:
      graph.route_handler_receipts_lifecycle_scripts_executed === false,
    routeHandlerReceiptsLifecycleScriptsExecuted:
      graph.route_handler_receipts_lifecycle_scripts_executed === true,
  };
}

function hasInstalledBinarySmokeReceipt(readiness) {
  return (
    readiness.ok &&
    readiness.value &&
    readiness.value.installed_binary_smoke &&
    readiness.value.installed_binary_smoke.receipt === INSTALLED_BINARY_SMOKE_RECEIPT &&
    readiness.value.receipts &&
    readiness.value.receipts.installed_binary_smoke === INSTALLED_BINARY_SMOKE_RECEIPT
  );
}

module.exports = {
  summarizeReadiness,
};
