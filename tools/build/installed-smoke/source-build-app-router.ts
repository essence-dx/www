const fs = require("node:fs");

const { EXPECTED_ROUTE_HANDLER_COUNT, FIXTURE_PATHS } = require("./fixture-paths.ts");
const { summarizeAppRouterExecution } = require("./app-router-execution.ts");
const { relativePath } = require("./io.ts");

function summarizeAppRouterOutputs(input) {
  const appExecution = input.appExecution.value || {};

  return {
    rootHtmlPath: relativePath(input.projectRoot, input.appHtmlPath),
    rootPacketPath: relativePath(input.projectRoot, input.appPacketPath),
    pageGraphPath: relativePath(input.projectRoot, input.appPageGraphPath),
    executionContractPath: relativePath(input.projectRoot, input.appExecutionPath),
    serverDataPath: relativePath(input.projectRoot, input.serverDataPath),
    rootHtmlPresent: fs.existsSync(input.appHtmlPath),
    rootPacketPresent: fs.existsSync(input.appPacketPath),
    pageGraphPresent: fs.existsSync(input.appPageGraphPath),
    executionContractPresent: input.appExecution.ok,
    execution: summarizeAppRouterExecution(appExecution, FIXTURE_PATHS.appPage),
    serverDataPresent: input.serverData.ok,
    serverData: summarizeServerData(input.serverData.value || {}),
  };
}

function summarizeServerData(serverData) {
  const entries = Array.isArray(serverData.entries) ? serverData.entries : [];
  const loaderEntry = entries.find(
    (entry) =>
      entry.binding === "metrics" &&
      entry.export_name === "loadLaunchMetrics" &&
      entry.source_path === FIXTURE_PATHS.serverLoader &&
      entry.execution_model === "source-owned-safe-interpreter",
  );
  return {
    route: serverData.route || null,
    routeSourcePath: serverData.route_source_path || null,
    hasRootRouteContract:
      serverData.route === "/" &&
      serverData.route_source_path === FIXTURE_PATHS.appPage,
    declaresNoNodeModules: serverData.node_modules_required === false,
    lifecycleScriptsExecuted: serverData.lifecycle_scripts_executed === true,
    hasLoaderEntry: Boolean(loaderEntry),
    hasLoaderValue: loaderEntry?.value?.routeHandlers === EXPECTED_ROUTE_HANDLER_COUNT,
  };
}

module.exports = {
  summarizeAppRouterOutputs,
};
