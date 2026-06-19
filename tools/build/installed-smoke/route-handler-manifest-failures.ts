function routeHandlerManifestFailures(routeHandlers) {
  const failures = [];
  const handlers = routeHandlerManifestFailureEntries(routeHandlers);

  for (const routeHandler of handlers) {
    pushIf(
      failures,
      !routeHandler.present,
      `source-build manifest is missing ${routeHandler.sourcePath} route-handler evidence`,
    );
    if (!routeHandler.present) continue;
    pushIf(
      failures,
      routeHandler.duplicateCount > 0,
      `source-build manifest has duplicate ${routeHandlerManifestLabel(routeHandler)} route-handler evidence`,
    );
    pushIf(
      failures,
      !routeHandler.methods.includes(routeHandler.requiredMethod),
      `source-build manifest ${routeHandler.sourcePath} route handler is missing ${routeHandler.requiredMethod}`,
    );
    pushIf(
      failures,
      routeHandler.executionModel !== "source-owned-route-handler-contract",
      `source-build manifest ${routeHandler.sourcePath} route handler does not use the source-owned execution contract`,
    );
    pushIf(
      failures,
      !routeHandler.declaresNoNodeModules,
      `source-build manifest ${routeHandler.sourcePath} route handler does not declare node_modules_required=false`,
    );
    pushIf(failures, routeHandler.nodeModulesRequired, `source-build manifest ${routeHandler.sourcePath} route handler requires node_modules`);
    pushIf(failures, routeHandler.lifecycleScriptsExecuted, `source-build manifest ${routeHandler.sourcePath} route handler executed lifecycle scripts`);
  }

  const unexpectedRouteHandlers = Array.isArray(routeHandlers?.unexpectedRouteHandlers)
    ? routeHandlers.unexpectedRouteHandlers
    : [];
  for (const routeHandler of unexpectedRouteHandlers) {
    failures.push(
      `source-build manifest has unexpected stale ${routeHandlerManifestEvidenceLabel(routeHandler)} route-handler evidence`,
    );
  }
  return failures;
}

function routeHandlerManifestFailureEntries(routeHandlers) {
  if (Array.isArray(routeHandlers)) {
    return routeHandlers;
  }
  if (routeHandlers?.routeHandler || routeHandlers?.checkoutRouteHandler) {
    return [routeHandlers.routeHandler, routeHandlers.checkoutRouteHandler].filter(Boolean);
  }
  if (routeHandlers?.unexpectedRouteHandlers) {
    return [];
  }
  return [routeHandlers];
}

function routeHandlerManifestLabel(routeHandler) {
  return [routeHandler.sourcePath, routeHandler.route].filter(Boolean).join(" ") || "<unknown-route-handler>";
}

function routeHandlerManifestEvidenceLabel(routeHandler) {
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
  routeHandlerManifestFailures,
};
