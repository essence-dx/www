function summarizeAppRouterExecution(execution, expectedSourcePath) {
  const runtimeBoundary = execution.runtime_boundary || {};
  return {
    route: execution.route || null,
    routeSourcePath: execution.source_path || null,
    hasRootRouteContract: execution.route === "/" && execution.source_path === expectedSourcePath,
    declaresNoNodeModules: execution.node_modules_present === false,
    nodeModulesPresent: execution.node_modules_present === true,
    sourceOwnedRuntimeBoundary: runtimeBoundary.source_owned === true,
    externalRuntimeRequired: runtimeBoundary.external_runtime_required === true,
    externalRuntimeExecuted: runtimeBoundary.external_runtime_executed === true,
  };
}

function appRouterExecutionFailures(appRouter) {
  if (!appRouter.executionContractPresent) {
    return [];
  }

  const execution = appRouter.execution;
  const failures = [];
  pushIf(failures, !execution.hasRootRouteContract, "app-router-execution.json does not describe the root app route");
  pushIf(failures, !execution.declaresNoNodeModules, "app-router-execution.json does not declare node_modules_present=false");
  pushIf(failures, execution.nodeModulesPresent, "app-router-execution.json says node_modules are present");
  pushIf(failures, !execution.sourceOwnedRuntimeBoundary, "app-router-execution.json does not declare source-owned runtime boundary");
  pushIf(failures, execution.externalRuntimeRequired, "app-router-execution.json requires an external runtime");
  pushIf(failures, execution.externalRuntimeExecuted, "app-router-execution.json executed an external runtime");
  return failures;
}

function pushIf(failures, condition, message) {
  if (condition) failures.push(message);
}

module.exports = {
  appRouterExecutionFailures,
  summarizeAppRouterExecution,
};
