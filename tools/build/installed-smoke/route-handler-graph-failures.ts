function graphReceiptFailures(graphReceipt) {
  const failures = [];
  if (!graphReceipt.present) {
    return ["dx build did not write .dx/receipts/graph/latest.json"];
  }

  pushIf(failures, graphReceipt.schema !== "dx.build.graph", "dx.build.graph receipt has an unexpected schema");
  pushIf(
    failures,
    !graphReceipt.hasRouteHandlerNode,
    "dx.build.graph is missing the app/api/health/route.ts route-handler node",
  );
  pushIf(
    failures,
    graphReceipt.routeHandlerDuplicateCount > 0,
    "dx.build.graph has duplicate app/api/health/route.ts /api/health route-handler nodes",
  );
  pushIf(failures, !graphReceipt.routeHandlerMethods.includes("GET"), "dx.build.graph route handler is missing GET");
  pushIf(
    failures,
    graphReceipt.routeHandlerExecutionModel !== "source-owned-route-handler-contract",
    "dx.build.graph route handler does not use the source-owned execution contract",
  );
  pushIf(
    failures,
    !graphReceipt.routeHandlerDeclaresNoNodeModules,
    "dx.build.graph route handler does not declare node_modules_required=false",
  );
  pushIf(failures, graphReceipt.routeHandlerNodeModulesRequired, "dx.build.graph route handler requires node_modules");
  pushIf(failures, graphReceipt.routeHandlerLifecycleScriptsExecuted, "dx.build.graph route handler executed lifecycle scripts");
  pushIf(
    failures,
    !graphReceipt.hasCheckoutRouteHandlerNode,
    "dx.build.graph is missing the app/api/checkout/route.ts route-handler node",
  );
  pushIf(
    failures,
    graphReceipt.checkoutRouteHandlerDuplicateCount > 0,
    "dx.build.graph has duplicate app/api/checkout/route.ts /api/checkout route-handler nodes",
  );
  pushIf(
    failures,
    !graphReceipt.checkoutRouteHandlerMethods.includes("POST"),
    "dx.build.graph checkout route handler is missing POST",
  );
  pushIf(
    failures,
    graphReceipt.checkoutRouteHandlerExecutionModel !== "source-owned-route-handler-contract",
    "dx.build.graph checkout route handler does not use the source-owned execution contract",
  );
  pushIf(
    failures,
    !graphReceipt.checkoutRouteHandlerDeclaresNoNodeModules,
    "dx.build.graph checkout route handler does not declare node_modules_required=false",
  );
  pushIf(failures, graphReceipt.checkoutRouteHandlerNodeModulesRequired, "dx.build.graph checkout route handler requires node_modules");
  pushIf(failures, graphReceipt.checkoutRouteHandlerLifecycleScriptsExecuted, "dx.build.graph checkout route handler executed lifecycle scripts");
  failures.push(...unexpectedRouteHandlerFailures(graphReceipt));
  failures.push(...sourceModuleFailures(graphReceipt));
  return failures;
}

function unexpectedRouteHandlerFailures(graphReceipt) {
  const routeHandlers = Array.isArray(graphReceipt.unexpectedRouteHandlerNodes)
    ? graphReceipt.unexpectedRouteHandlerNodes
    : [];
  return routeHandlers.map(
    (routeHandler) =>
      `dx.build.graph has unexpected stale ${graphRouteHandlerEvidenceLabel(routeHandler)} route-handler node`,
  );
}

function sourceModuleFailures(graphReceipt) {
  const failures = [];
  for (const sourceModule of [
    ["server/loaders.ts", graphReceipt.hasServerLoaderSourceModule, graphReceipt.serverLoaderSourceModuleDeclaresNoNodeModules, graphReceipt.serverLoaderSourceModuleCompiledFromSource],
    ["server/launch-copy.ts", graphReceipt.hasServerModuleSourceModule, graphReceipt.serverModuleSourceModuleDeclaresNoNodeModules, graphReceipt.serverModuleSourceModuleCompiledFromSource],
  ]) {
    const [path, present, declaresNoNodeModules, compiledFromSource] = sourceModule;
    pushIf(failures, !present, `dx.build.graph is missing the ${path} source-module node`);
    pushIf(failures, !declaresNoNodeModules, `dx.build.graph ${path} source-module does not declare node_modules_required=false`);
    pushIf(failures, !compiledFromSource, `dx.build.graph is missing compiled-from-source edge for ${path}`);
  }
  return failures;
}

function graphRouteHandlerEvidenceLabel(routeHandler) {
  const method = Array.isArray(routeHandler.methods) && routeHandler.methods.length > 0
    ? routeHandler.methods.join(",")
    : "<unknown-method>";
  return [routeHandler.sourcePath, method, routeHandler.route].filter(Boolean).join(" ");
}

function pushIf(failures, condition, message) {
  if (condition) {
    failures.push(message);
  }
}

module.exports = {
  graphReceiptFailures,
};
