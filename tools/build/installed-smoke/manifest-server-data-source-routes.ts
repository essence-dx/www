const { requestPropKeys } = require("./manifest-server-data-artifact.ts");
const { serverDataRouteKey } = require("./manifest-server-data-route-keys.ts");

function sourceBuildServerDataOutput(manifest, routePath, sourcePath) {
  return Array.isArray(manifest.route_outputs)
    ? manifest.route_outputs.find((route) => route.route === routePath && route.source_path === sourcePath)
        ?.server_data_output || null
    : null;
}

function sourceBuildServerDataRoute(manifest, routePath, sourcePath) {
  return Array.isArray(manifest.server_data_routes)
    ? manifest.server_data_routes.find(
        (route) => route.route === routePath && route.route_source_path === sourcePath,
      ) || null
    : null;
}

function sourceBuildServerDataRoutes(manifest) {
  const routeMap = new Map();
  addRouteOutputs(routeMap, manifest.route_outputs);
  addServerDataRoutes(routeMap, manifest.server_data_routes);
  return [...routeMap.values()];
}

function addRouteOutputs(routeMap, routes) {
  if (!Array.isArray(routes)) {
    return;
  }
  for (const route of routes) {
    if (typeof route.server_data_output !== "string") {
      continue;
    }
    const summary = {
      route: route.route || null,
      routeSourcePath: route.source_path || null,
      output: route.server_data_output || null,
      entryCount: null,
      routeParamKeys: [],
      searchParamKeys: [],
    };
    routeMap.set(serverDataRouteKey(summary), summary);
  }
}

function addServerDataRoutes(routeMap, routes) {
  if (!Array.isArray(routes)) {
    return;
  }
  for (const route of routes) {
    if (typeof route.output !== "string") {
      continue;
    }
    const summary = {
      route: route.route || null,
      routeSourcePath: route.route_source_path || null,
      output: route.output || null,
      entryCount: Number.isInteger(route.entry_count) ? route.entry_count : 0,
      routeParamKeys: requestPropKeys(route, "route_params"),
      searchParamKeys: requestPropKeys(route, "search_params"),
    };
    routeMap.set(serverDataRouteKey(summary), summary);
  }
}

module.exports = {
  sourceBuildServerDataOutput,
  sourceBuildServerDataRoute,
  sourceBuildServerDataRoutes,
};
