const { CSS_SUMMARY_FIELDS } = require("./constants.ts");
const { pick } = require("./io.ts");
const { summarizeManifestServerDataRoutes } = require("./manifest-server-data-routes.ts");

function summarizeManifest(input) {
  const manifest = input.manifest;
  const routeHandlerReceiptsCompiled =
    manifest.ok && Number.isInteger(manifest.value.route_handler_receipts_compiled)
      ? manifest.value.route_handler_receipts_compiled
      : null;
  const routeHandlerReceiptCollection = input.routeHandlerReceipts.value || {};
  const routeHandlerReceiptCollectionCount = Number.isInteger(routeHandlerReceiptCollection.receipt_count)
    ? routeHandlerReceiptCollection.receipt_count
    : null;

  return {
    present: manifest.ok,
    hasCssSummary: manifest.ok && CSS_SUMMARY_FIELDS.every((field) => field in manifest.value),
    hasServerDataSummary:
      manifest.ok &&
      manifest.value.server_data_entries_compiled === 1 &&
      manifest.value.server_contracts_compiled >= 1 &&
      manifest.value.deploy_adapter_emitted === true &&
      manifest.value.node_modules_required === false,
    nextFamiliarCompatibilityEvidenceEmitted:
      manifest.ok && manifest.value.next_familiar_compatibility_evidence_emitted === true,
    oldNextRuntimeParityEvidenceFlagPresent:
      manifest.ok && Object.prototype.hasOwnProperty.call(manifest.value, "next_runtime_parity_evidence_emitted"),
    routeHandlerReceiptsCompiled,
    routeHandlerReceiptCollectionCount,
    routeHandlerReceiptsCompiledMatchesCollection:
      routeHandlerReceiptsCompiled !== null &&
      routeHandlerReceiptCollectionCount !== null &&
      routeHandlerReceiptsCompiled === routeHandlerReceiptCollectionCount,
    cssSummary: manifest.ok ? pick(manifest.value, CSS_SUMMARY_FIELDS) : {},
    serverDataRoutes: summarizeManifestServerDataRoutes(input),
  };
}

module.exports = {
  summarizeManifest,
};
