const { PRODUCT_PREVIEW_SCORE } = require("./constants.ts");
const { manifestServerDataRouteFailures } = require("./manifest-server-data-routes.ts");
const { nextFamiliarCompatibilityFailures } = require("./next-familiar-compatibility.ts");

function buildReceiptFailures(report, input) {
  const failures = [];
  pushIf(failures, input.build.status !== 0, "dx build fixture exited non-zero");
  pushIf(failures, !report.build.manifest.present, "dx build did not write .dx/build/.dx/build-cache/manifest.json");
  pushIf(failures, !report.build.manifest.hasCssSummary, "manifest is missing source_build_css_* summary fields");
  pushIf(failures, !report.build.manifest.hasServerDataSummary, "manifest is missing server-data build summary fields");
  pushIf(
    failures,
    report.build.manifest.routeHandlerReceiptsCompiled === null,
    "manifest is missing route_handler_receipts_compiled",
  );
  pushIf(
    failures,
    report.build.manifest.routeHandlerReceiptsCompiled !== null &&
      report.build.manifest.routeHandlerReceiptCollectionCount !== null &&
      !report.build.manifest.routeHandlerReceiptsCompiledMatchesCollection,
    "manifest route_handler_receipts_compiled does not match route-handler receipts",
  );
  failures.push(...manifestServerDataRouteFailures(report));
  failures.push(...nextFamiliarCompatibilityFailures(report));
  pushIf(failures, !report.build.zedHandoff.present, "dx build did not write .dx/receipts/build/zed-handoff.json");
  pushIf(failures, report.build.zedHandoff.schema !== "dx.build.zedHandoff", "Zed handoff receipt has an unexpected schema");
  pushIf(failures, !report.build.zedHandoff.hasStyleOptimization, "Zed handoff is missing style_optimization");
  pushIf(failures, !report.build.zedHandoff.hasBuildReadinessPointer, "Zed handoff is missing build_readiness");
  pushIf(
    failures,
    !report.build.zedHandoff.hasInstalledBinarySmokeReceiptPointer,
    "Zed handoff is missing installed_binary_smoke_receipt",
  );
  pushIf(failures, !report.build.readiness.present, "dx build did not write .dx/receipts/build/readiness.json");
  pushIf(failures, report.build.readiness.schema !== "dx.build.readiness", "build readiness receipt has an unexpected schema");
  pushIf(
    failures,
    !report.build.readiness.sourceReady || report.build.readiness.sourceScore !== 100,
    "build readiness receipt does not report source readiness",
  );
  pushIf(
    failures,
    report.build.readiness.productReady ||
      report.build.readiness.productScore !== PRODUCT_PREVIEW_SCORE,
    "build readiness receipt does not keep product readiness governed",
  );
  pushIf(
    failures,
    !report.build.readiness.hasInstalledBinarySmokeReceipt,
    "build readiness receipt is missing installed-binary smoke receipt pointers",
  );
  pushIf(
    failures,
    !report.build.readiness.routeHandlerReceiptOutputMatchesActual,
    "build readiness route_handler_receipt_output does not match route-handler receipts path",
  );
  pushIf(
    failures,
    report.build.readiness.routeHandlerReceiptsExecuted !==
      report.build.sourceBuild.routeHandlerReceipt.receiptCount,
    "build readiness route-handler receipt executed count does not match receipt collection",
  );
  pushIf(
    failures,
    report.build.readiness.routeHandlerReceiptsSkipped !==
      report.build.sourceBuild.routeHandlerReceipt.skippedCount,
    "build readiness route-handler receipt skipped count does not match receipt collection",
  );
  pushIf(
    failures,
    !report.build.readiness.routeHandlerReceiptsDeclareNoNodeModules,
    "build readiness route-handler receipts do not declare node_modules_required=false",
  );
  pushIf(
    failures,
    report.build.readiness.routeHandlerReceiptsNodeModulesRequired,
    "build readiness route-handler receipts require node_modules",
  );
  pushIf(
    failures,
    !report.build.readiness.routeHandlerReceiptsDeclareNoLifecycleScripts,
    "build readiness route-handler receipts do not declare lifecycle_scripts_executed=false",
  );
  pushIf(
    failures,
    report.build.readiness.routeHandlerReceiptsLifecycleScriptsExecuted,
    "build readiness route-handler receipts executed lifecycle scripts",
  );
  const nodeModulesBeforePaths = Array.isArray(input.nodeModulesBeforePaths) ? input.nodeModulesBeforePaths : [];
  const nodeModulesCreatedPaths = Array.isArray(input.nodeModulesCreatedPaths)
    ? input.nodeModulesCreatedPaths
    : [];
  const nodeModulesPaths = Array.isArray(input.nodeModulesPaths) ? input.nodeModulesPaths : [];
  pushIf(
    failures,
    nodeModulesBeforePaths.length > 0,
    `fixture contains pre-existing node_modules before dx build: ${nodeModulesBeforePaths.join(", ")}`,
  );
  pushIf(
    failures,
    nodeModulesCreatedPaths.length > 0,
    `dx build created node_modules in the fixture: ${nodeModulesCreatedPaths.join(", ")}`,
  );
  pushIf(
    failures,
    nodeModulesPaths.length > 0 &&
      nodeModulesBeforePaths.length === 0 &&
      nodeModulesCreatedPaths.length === 0,
    `fixture contains node_modules after dx build: ${nodeModulesPaths.join(", ")}`,
  );
  return failures;
}

function pushIf(failures, condition, message) {
  if (condition) {
    failures.push(message);
  }
}

module.exports = {
  buildReceiptFailures,
};
