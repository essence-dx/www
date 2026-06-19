const {
  duplicateServerDataRoutes,
  serverDataRouteOutputKey,
} = require("./manifest-server-data-route-keys.ts");
const {
  manifestServerDataRouteFailures,
} = require("./manifest-server-data-route-failures.ts");
const {
  sourceBuildServerDataOutput,
  sourceBuildServerDataRoute,
  sourceBuildServerDataRoutes,
} = require("./manifest-server-data-source-routes.ts");
const { summarizeRouteManifest } = require("./manifest-server-data-route-manifest.ts");
const { summarizeServerDataRoute } = require("./manifest-server-data-route-summary.ts");

function summarizeManifestServerDataRoutes(input) {
  const manifest = input.manifest.value || {};
  const sourceBuildManifest = input.sourceBuildManifest.value || {};
  const routes = Array.isArray(manifest.server_data_routes) ? manifest.server_data_routes : [];
  const sourceBuildRoutes = sourceBuildServerDataRoutes(sourceBuildManifest);
  const summarizedRoutes = routes.map((route) =>
    summarizeServerDataRoute(
      input.projectRoot,
      route,
      sourceBuildServerDataOutput(sourceBuildManifest, route.route, route.route_source_path),
      sourceBuildServerDataRoute(sourceBuildManifest, route.route, route.route_source_path),
    ),
  );
  const duplicateManifestRoutes = duplicateServerDataRoutes(summarizedRoutes);
  const manifestRouteKeys = new Set(summarizedRoutes.map(serverDataRouteOutputKey));
  const missingSourceBuildRoutes = sourceBuildRoutes.filter(
    (route) => !manifestRouteKeys.has(serverDataRouteOutputKey(route)),
  );
  const rootRoute = summarizedRoutes.find(
    (route) => route.route === "/" && route.routeSourcePath === "app/page.tsx",
  );
  const routeManifest = summarizeRouteManifest(
    manifest.server_data_route_manifest,
    summarizedRoutes,
    sourceBuildRoutes,
    missingSourceBuildRoutes,
  );

  return {
    present: Array.isArray(manifest.server_data_routes),
    compiledCount: Number.isInteger(manifest.server_data_routes_compiled)
      ? manifest.server_data_routes_compiled
      : null,
    routeCount: routes.length,
    emittedCount: summarizedRoutes.filter((route) => route.output.present).length,
    sourceBuildMatchCount: summarizedRoutes.filter((route) => route.matchesSourceBuildRouteOutput).length,
    sourceBuildServerDataRouteMatchCount: summarizedRoutes.filter(
      (route) => route.matchesSourceBuildServerDataRoute,
    ).length,
    allOutputsPresent:
      summarizedRoutes.length > 0 && summarizedRoutes.every((route) => route.output.present),
    allRoutesMatchSourceBuildOutputs:
      summarizedRoutes.length > 0 &&
      summarizedRoutes.every((route) => route.matchesSourceBuildRouteOutput),
    allRoutesMatchSourceBuildServerDataRoutes:
      summarizedRoutes.length > 0 &&
      summarizedRoutes.every((route) => route.matchesSourceBuildServerDataRoute),
    sourceBuildRouteCount: sourceBuildRoutes.length,
    duplicateRouteCount: duplicateManifestRoutes.length,
    duplicateManifestRoutes,
    missingSourceBuildRoutes,
    routeManifest,
    routes: summarizedRoutes,
    hasRootRoute: Boolean(rootRoute),
    rootRoute: rootRoute || summarizeServerDataRoute(input.projectRoot, null, null),
  };
}

module.exports = {
  manifestServerDataRouteFailures,
  summarizeManifestServerDataRoutes,
};
