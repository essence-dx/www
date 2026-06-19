const { appRouterExecutionFailures } = require("./app-router-execution.ts");
const { EXPECTED_ROUTE_HANDLER_COUNT } = require("./fixture-paths.ts");
const { graphConsumerSnapshotFailures } = require("./graph-consumer-snapshot.ts");
const { graphReceiptFailures } = require("./route-handler-graph-failures.ts");
const { routeHandlerManifestFailures } = require("./route-handler-manifest-failures.ts");
const { routeHandlerReceiptFailures } = require("./route-handler-receipts.ts");
const { routeOutputFailures } = require("./route-output.ts");
const { outputProofFailures } = require("./output-proof-failures.ts");
const { sourceModuleResolverFailures } = require("./source-module-resolver.ts");

function sourceBuildFailures(report) {
  const failures = [];
  const fixture = report.fixture;
  const appRouter = report.build.appRouter;
  const sourceBuild = report.build.sourceBuild;

  pushIf(failures, !fixture.hasAppPage, "fixture is missing app/page.tsx");
  pushIf(failures, !fixture.hasAppLayout, "fixture is missing app/layout.tsx");
  pushIf(failures, !fixture.hasRouteHandler, "fixture is missing app/api/health/route.ts");
  pushIf(failures, !fixture.hasCheckoutRouteHandler, "fixture is missing app/api/checkout/route.ts");
  pushIf(failures, !fixture.hasComponent, "fixture is missing components/LaunchCard.tsx");
  pushIf(failures, !fixture.hasServerLoader, "fixture is missing server/loaders.ts");
  pushIf(failures, !fixture.hasServerModule, "fixture is missing server/launch-copy.ts");
  pushIf(failures, !fixture.hasStyleSource, "fixture is missing styles/app.css");
  pushIf(failures, !fixture.hasPublicAsset, "fixture is missing public/icons/mark.svg");
  pushIf(failures, !appRouter.rootHtmlPresent, "dx build did not write .dx/build/app/index.html");
  pushIf(failures, !appRouter.rootPacketPresent, "dx build did not write .dx/build/app/index.dxpk");
  pushIf(failures, !appRouter.pageGraphPresent, "dx build did not write .dx/build/app/page-graph.json");
  pushIf(failures, !appRouter.executionContractPresent, "dx build did not write app-router-execution.json");
  failures.push(...appRouterExecutionFailures(appRouter));
  pushIf(failures, !appRouter.serverDataPresent, "dx build did not write .dx/build/app/server-data.json");
  pushIf(failures, !appRouter.serverData.hasRootRouteContract, "server-data.json does not describe the root app route");
  pushIf(failures, !appRouter.serverData.hasLoaderEntry, "server-data.json is missing loadLaunchMetrics");
  pushIf(failures, !appRouter.serverData.hasLoaderValue, "server-data.json did not materialize launch metrics");
  pushIf(failures, !appRouter.serverData.declaresNoNodeModules, "server-data.json does not declare node_modules_required=false");
  pushIf(failures, appRouter.serverData.lifecycleScriptsExecuted, "server-data.json executed lifecycle scripts");
  pushIf(failures, !sourceBuild.manifest.present, "dx build did not write source-build-manifest.json");
  if (sourceBuild.manifest.present) {
    pushIf(
      failures,
      sourceBuild.manifest.schema !== "dx.www.sourceBuildManifest",
      "source-build manifest has an unexpected schema",
    );
  }
  pushIf(failures, !sourceBuild.manifest.hasRootRoute, "source-build manifest is missing the root app route");
  failures.push(
    ...routeHandlerManifestFailures(sourceBuild.manifest.routeHandler, {
      sourcePath: "app/api/health/route.ts",
      method: "GET",
    }),
  );
  failures.push(
    ...routeHandlerManifestFailures(sourceBuild.manifest.checkoutRouteHandler, {
      sourcePath: "app/api/checkout/route.ts",
      method: "POST",
    }),
  );
  failures.push(
    ...routeHandlerManifestFailures({
      unexpectedRouteHandlers: sourceBuild.manifest.unexpectedRouteHandlers,
    }),
  );
  pushIf(failures, !sourceBuild.manifest.rootRouteOutput.present, "source-build did not emit manifest-declared root route outputs");
  pushIf(
    failures,
    !sourceBuild.manifest.rootRouteOutput.serverData.present,
    "source-build root route output is missing manifest-declared server-data output",
  );
  failures.push(...routeOutputFailures(sourceBuild.manifest.rootRouteOutput));
  pushIf(failures, !sourceBuild.manifest.hasStyle, "source-build manifest is missing styles/app.css");
  pushIf(failures, !sourceBuild.manifest.hasPublicAsset, "source-build manifest is missing public/icons/mark.svg");
  failures.push(...outputProofFailures(report));
  pushIf(failures, !sourceBuild.manifest.hasLinkedComponent, "source-build manifest is missing the linked component module");
  pushIf(failures, !sourceBuild.manifest.hasLinkedServerLoader, "source-build manifest is missing the linked server loader");
  pushIf(failures, !sourceBuild.manifest.hasLinkedServerModule, "source-build manifest is missing the linked server module");
  failures.push(...sourceModuleResolverFailures(sourceBuild.manifest.resolverEvidence));
  pushIf(failures, !sourceBuild.manifest.declaresNoNodeModules, "source-build manifest does not declare node_modules_required=false");
  pushIf(failures, sourceBuild.manifest.nodeModulesRequired, "source-build manifest requires node_modules");
  pushIf(failures, !sourceBuild.receipt.present, "dx build did not write source-build-receipt.json");
  if (sourceBuild.receipt.present) {
    pushIf(
      failures,
      sourceBuild.receipt.schema !== "dx.www.sourceBuildReceipt",
      "source-build receipt has an unexpected schema",
    );
  }
  pushIf(
    failures,
    sourceBuild.receipt.routeHandlers !== EXPECTED_ROUTE_HANDLER_COUNT,
    "source-build receipt did not count the route handlers",
  );
  pushIf(failures, !sourceBuild.receipt.declaresNoNodeModules, "source-build receipt does not declare node_modules_required=false");
  pushIf(failures, sourceBuild.receipt.nodeModulesRequired, "source-build receipt requires node_modules");
  failures.push(...routeHandlerReceiptFailures(sourceBuild.routeHandlerReceipt));
  pushIf(failures, !sourceBuild.canonicalReceipt.present, "dx build did not write .dx/receipts/build/latest.json");
  if (sourceBuild.canonicalReceipt.present) {
    pushIf(
      failures,
      sourceBuild.canonicalReceipt.schema !== "dx.www.sourceBuildReceipt",
      "canonical source-build receipt has an unexpected schema",
    );
  }
  failures.push(...graphReceiptFailures(sourceBuild.graphReceipt));
  failures.push(...graphConsumerSnapshotFailures(sourceBuild.graphConsumerSnapshot));
  return failures;
}

function pushIf(failures, condition, message) {
  if (condition) failures.push(message);
}

module.exports = {
  sourceBuildFailures,
};
