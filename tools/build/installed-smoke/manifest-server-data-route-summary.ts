const { emitted, readServerDataArtifact, requestPropKeys, requestPropObject, requestPropsEqual, requestPropsMatchArtifact, serverDataArtifactMatchesManifest } = require("./manifest-server-data-artifact.ts");

function summarizeServerDataRoute(root, route, sourceBuildOutput, sourceBuildServerDataRoute) {
  const outputPath = typeof route?.output === "string" ? route.output : null;
  const sourceBuildServerDataRouteOutput =
    typeof sourceBuildServerDataRoute?.output === "string" ? sourceBuildServerDataRoute.output : null;
  const output = emitted(root, outputPath);
  const artifact = readServerDataArtifact(output.fullPath);
  const routeParams = requestPropObject(route, "route_params");
  const searchParams = requestPropObject(route, "search_params");
  const routeParamKeys = requestPropKeys(route, "route_params");
  const searchParamKeys = requestPropKeys(route, "search_params");
  const sourceBuildRouteParams = requestPropObject(sourceBuildServerDataRoute, "route_params");
  const sourceBuildSearchParams = requestPropObject(sourceBuildServerDataRoute, "search_params");
  const sourceBuildRouteParamKeys = requestPropKeys(sourceBuildServerDataRoute, "route_params");
  const sourceBuildSearchParamKeys = requestPropKeys(sourceBuildServerDataRoute, "search_params");
  return {
    route: route?.route || null,
    routeSourcePath: route?.route_source_path || null,
    status: route?.status || null,
    entryCount: Number.isInteger(route?.entry_count) ? route.entry_count : null,
    executionModel: route?.execution_model || null,
    requestMode: route?.request?.mode || null,
    routeParams,
    searchParams,
    routeParamKeys,
    searchParamKeys,
    sourceBuildRouteParams,
    sourceBuildSearchParams,
    sourceBuildRouteParamKeys,
    sourceBuildSearchParamKeys,
    declaresNoNodeModules: route?.node_modules_required === false,
    lifecycleScriptsExecuted: route?.lifecycle_scripts_executed === true,
    sourceOwnedContract: route?.source_owned_contract === true,
    externalRuntimeRequired: route?.external_runtime_required === true,
    externalRuntimeExecuted: route?.external_runtime_executed === true,
    output,
    artifact,
    artifactMatchesManifest: serverDataArtifactMatchesManifest(route, artifact),
    requestPropsMatchArtifact: requestPropsMatchArtifact(route, artifact),
    requestPropsMatchSourceBuildRoute: sourceBuildServerDataRoute
      ? requestPropsEqual(routeParams, sourceBuildRouteParams) &&
        requestPropsEqual(searchParams, sourceBuildSearchParams)
      : null,
    matchesSourceBuildRouteOutput:
      typeof outputPath === "string" &&
      typeof sourceBuildOutput === "string" &&
      outputPath === sourceBuildOutput,
    matchesSourceBuildServerDataRoute:
      typeof outputPath === "string" &&
      typeof sourceBuildServerDataRouteOutput === "string" &&
      outputPath === sourceBuildServerDataRouteOutput,
  };
}

module.exports = {
  summarizeServerDataRoute,
};
