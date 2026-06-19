const {
  PRIMARY_ROUTE_HANDLER,
  REQUIRED_ROUTE_HANDLERS,
  normalizeRouteHandlerMethod,
  normalizeRouteHandlerRequirement,
  normalizeRouteHandlerRoute,
  normalizeRouteHandlerSourcePath,
} = require("./route-handler-requirements.ts");
const { summarizeUnexpectedRouteHandlerEvidence } = require("./route-handler-unexpected-evidence.ts");

function summarizeRouteHandlerManifest(manifest, requirement = PRIMARY_ROUTE_HANDLER, route = null) {
  let expected = normalizeRouteHandlerRequirement(requirement);
  if (route && !expected.route) {
    expected = { ...expected, route };
  }
  const handlers = Array.isArray(manifest.route_handlers)
    ? manifest.route_handlers.filter(
        (item) =>
          normalizeRouteHandlerSourcePath(item.path) === expected.sourcePath &&
          (!expected.route || normalizeRouteHandlerRoute(item.route) === expected.route),
      )
    : [];
  const handler = handlers[0] || null;
  const methods = Array.isArray(handler?.methods)
    ? handler.methods
        .map((method) => normalizeRouteHandlerMethod(method))
        .filter(Boolean)
    : [];
  return {
    key: expected.key,
    sourcePath: expected.sourcePath,
    requiredMethod: expected.method,
    present: Boolean(handler),
    duplicateCount: Math.max(0, handlers.length - 1),
    path: handler ? normalizeRouteHandlerSourcePath(handler.path) : null,
    route: handler ? normalizeRouteHandlerRoute(handler.route) : null,
    methods,
    declaresNoNodeModules: handler?.node_modules_required === false,
    nodeModulesRequired: handler?.node_modules_required === true,
    lifecycleScriptsExecuted: handler?.lifecycle_scripts_executed === true,
    executionModel: handler?.execution_model || null,
  };
}

function summarizeRequiredRouteHandlerManifests(manifest) {
  return REQUIRED_ROUTE_HANDLERS.map((handler) =>
    summarizeRouteHandlerManifest(manifest, handler),
  );
}

function summarizeUnexpectedRouteHandlerManifests(manifest) {
  const handlers = Array.isArray(manifest.route_handlers) ? manifest.route_handlers : [];
  return summarizeUnexpectedRouteHandlerEvidence(handlers, REQUIRED_ROUTE_HANDLERS, {
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

module.exports = {
  summarizeRouteHandlerManifest,
  summarizeRequiredRouteHandlerManifests,
  summarizeUnexpectedRouteHandlerManifests,
};
