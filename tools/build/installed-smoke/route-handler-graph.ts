const { FIXTURE_PATHS } = require("./fixture-paths.ts");
const {
  REQUIRED_ROUTE_HANDLERS,
  normalizeRouteHandlerMethod,
  normalizeRouteHandlerRoute,
  normalizeRouteHandlerSourcePath,
} = require("./route-handler-requirements.ts");
const { summarizeUnexpectedRouteHandlerEvidence } = require("./route-handler-unexpected-evidence.ts");

function summarizeGraphReceipt(artifact) {
  const receipt = artifact.value || {};
  const graphRouteHandler = summarizeGraphRouteHandler(receipt, FIXTURE_PATHS.routeHandler);
  const graphCheckoutRouteHandler = summarizeGraphRouteHandler(
    receipt,
    FIXTURE_PATHS.checkoutRouteHandler,
    "/api/checkout",
  );
  const unexpectedRouteHandlerNodes = summarizeUnexpectedGraphRouteHandlers(receipt);
  const graphServerLoaderSourceModule = summarizeGraphSourceModule(receipt, FIXTURE_PATHS.serverLoader);
  const graphServerModuleSourceModule = summarizeGraphSourceModule(receipt, FIXTURE_PATHS.serverModule);

  return {
    present: artifact.ok,
    schema: artifact.ok ? receipt.schema || null : null,
    hasRouteHandlerNode: graphRouteHandler.present,
    routeHandlerMethods: graphRouteHandler.methods,
    routeHandlerDeclaresNoNodeModules: graphRouteHandler.declaresNoNodeModules,
    routeHandlerNodeModulesRequired: graphRouteHandler.nodeModulesRequired,
    routeHandlerLifecycleScriptsExecuted: graphRouteHandler.lifecycleScriptsExecuted,
    routeHandlerExecutionModel: graphRouteHandler.executionModel,
    routeHandlerDuplicateCount: graphRouteHandler.duplicateCount,
    hasCheckoutRouteHandlerNode: graphCheckoutRouteHandler.present,
    checkoutRouteHandlerMethods: graphCheckoutRouteHandler.methods,
    checkoutRouteHandlerDeclaresNoNodeModules: graphCheckoutRouteHandler.declaresNoNodeModules,
    checkoutRouteHandlerNodeModulesRequired: graphCheckoutRouteHandler.nodeModulesRequired,
    checkoutRouteHandlerLifecycleScriptsExecuted: graphCheckoutRouteHandler.lifecycleScriptsExecuted,
    checkoutRouteHandlerExecutionModel: graphCheckoutRouteHandler.executionModel,
    checkoutRouteHandlerDuplicateCount: graphCheckoutRouteHandler.duplicateCount,
    unexpectedRouteHandlerNodes,
    hasServerLoaderSourceModule: graphServerLoaderSourceModule.present,
    serverLoaderSourceModuleDeclaresNoNodeModules: graphServerLoaderSourceModule.declaresNoNodeModules,
    serverLoaderSourceModuleCompiledFromSource: graphServerLoaderSourceModule.compiledFromSource,
    hasServerModuleSourceModule: graphServerModuleSourceModule.present,
    serverModuleSourceModuleDeclaresNoNodeModules: graphServerModuleSourceModule.declaresNoNodeModules,
    serverModuleSourceModuleCompiledFromSource: graphServerModuleSourceModule.compiledFromSource,
  };
}

function summarizeGraphRouteHandler(receipt, sourcePath, routePath = "/api/health") {
  const nodes = Array.isArray(receipt.graph?.nodes) ? receipt.graph.nodes : [];
  const expectedSourcePath = normalizeRouteHandlerSourcePath(sourcePath);
  const expectedRoutePath = normalizeRouteHandlerRoute(routePath);
  const handlers = nodes.filter(
    (node) =>
      node.kind === "app-route-handler" &&
      normalizeRouteHandlerSourcePath(node.path) === expectedSourcePath &&
      normalizeRouteHandlerRoute(node.route) === expectedRoutePath,
  );
  const handler = handlers[0] || null;
  return {
    present: Boolean(handler),
    duplicateCount: Math.max(0, handlers.length - 1),
    methods: Array.isArray(handler?.methods)
      ? handler.methods
          .map((method) => normalizeRouteHandlerMethod(method))
          .filter(Boolean)
      : [],
    declaresNoNodeModules: handler?.node_modules_required === false,
    nodeModulesRequired: handler?.node_modules_required === true,
    lifecycleScriptsExecuted: handler?.lifecycle_scripts_executed === true,
    executionModel: handler?.execution_model || null,
  };
}

function summarizeUnexpectedGraphRouteHandlers(receipt) {
  const nodes = Array.isArray(receipt.graph?.nodes) ? receipt.graph.nodes : [];
  return summarizeUnexpectedRouteHandlerEvidence(nodes, REQUIRED_ROUTE_HANDLERS, {
    entryFilter: (node) => node?.kind === "app-route-handler",
    sourcePathField: "path",
    routeField: "route",
    methodsField: "methods",
    includeMethods: true,
    matchMethod: false,
    includeExecutionContract: true,
    includeNoNodeModules: true,
    includeLifecycleScripts: true,
  });
}

function summarizeGraphSourceModule(receipt, sourcePath) {
  const nodes = Array.isArray(receipt.graph?.nodes) ? receipt.graph.nodes : [];
  const edges = Array.isArray(receipt.graph?.edges) ? receipt.graph.edges : [];
  const node = nodes.find(
    (item) => item.kind === "source-module" && item.path === sourcePath,
  );
  const nodeId = node?.id || `source-module:${sourcePath}`;
  const compiledFromSource =
    Boolean(node) &&
    edges.some(
      (edge) =>
        edge.kind === "compiled-from-source" &&
        edge.to === nodeId &&
        typeof edge.from === "string" &&
        edge.from.startsWith("source-module-chunk:"),
    );
  return {
    present: Boolean(node),
    declaresNoNodeModules: node?.node_modules_required === false,
    nodeModulesRequired: node?.node_modules_required === true,
    compiledFromSource,
  };
}

module.exports = {
  summarizeGraphReceipt,
  summarizeGraphRouteHandler,
  summarizeGraphSourceModule,
  summarizeUnexpectedGraphRouteHandlers,
};
