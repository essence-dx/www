const {
  arrayEquals,
  manifestServerDataRouteLabel,
  sortedRouteOutputKeys,
} = require("./manifest-server-data-route-keys.ts");
const {
  integerOrNull,
  requestPropKeysForRoutes,
  requestPropRouteCount,
  sumEntryCounts,
} = require("./manifest-server-data-request-props.ts");

function summarizeRouteManifest(routeManifest, routes, sourceBuildRoutes, missingSourceBuildRoutes) {
  const present = routeManifest && typeof routeManifest === "object" && !Array.isArray(routeManifest);
  const expectedSourceBuildRoutes = sourceBuildRoutes.length;
  const expectedManifestRoutes = routes.length;
  const expectedSourceBuildEntries = sumEntryCounts(sourceBuildRoutes);
  const expectedManifestEntries = sumEntryCounts(routes);
  const expectedManifestIncludesSourceBuildRoutes = missingSourceBuildRoutes.length === 0;
  const missingSourceBuildRouteKeys = sortedRouteOutputKeys(missingSourceBuildRoutes);
  const expectedRouteParamKeys = requestPropKeysForRoutes(routes, "routeParamKeys");
  const expectedSearchParamKeys = requestPropKeysForRoutes(routes, "searchParamKeys");
  const sourceBuildRouteParamKeys = requestPropKeysForRoutes(sourceBuildRoutes, "routeParamKeys");
  const sourceBuildSearchParamKeys = requestPropKeysForRoutes(sourceBuildRoutes, "searchParamKeys");
  const expectedRoutesWithRouteParams = requestPropRouteCount(routes, "routeParamKeys");
  const expectedRoutesWithSearchParams = requestPropRouteCount(routes, "searchParamKeys");
  const sourceBuildRequestPropMismatchRoutes = routes
    .filter((route) => route.requestPropsMatchSourceBuildRoute === false)
    .map(manifestServerDataRouteLabel)
    .sort();
  const declaredMissingSourceBuildRouteKeys = sortedRouteOutputKeys(
    present && Array.isArray(routeManifest.missing_source_build_routes)
      ? routeManifest.missing_source_build_routes
      : [],
  );
  const routeParamKeys =
    present && Array.isArray(routeManifest.route_param_keys) ? routeManifest.route_param_keys : [];
  const searchParamKeys =
    present && Array.isArray(routeManifest.search_param_keys) ? routeManifest.search_param_keys : [];

  const summary = {
    present,
    sourceBuildRoutes: integerOrNull(present && routeManifest.source_build_routes),
    manifestRoutes: integerOrNull(present && routeManifest.manifest_routes),
    sourceBuildEntries: integerOrNull(present && routeManifest.source_build_entries),
    manifestEntries: integerOrNull(present && routeManifest.manifest_entries),
    manifestIncludesSourceBuildRoutes:
      present && typeof routeManifest.manifest_includes_source_build_routes === "boolean"
        ? routeManifest.manifest_includes_source_build_routes
        : null,
    missingSourceBuildRoutes:
      present && Array.isArray(routeManifest.missing_source_build_routes)
        ? routeManifest.missing_source_build_routes
        : [],
    expectedSourceBuildRoutes,
    expectedManifestRoutes,
    expectedSourceBuildEntries,
    expectedManifestEntries,
    expectedManifestIncludesSourceBuildRoutes,
    routesWithRouteParams: integerOrNull(present && routeManifest.routes_with_route_params),
    routesWithSearchParams: integerOrNull(present && routeManifest.routes_with_search_params),
    routeParamKeys,
    searchParamKeys,
    expectedRoutesWithRouteParams,
    expectedRoutesWithSearchParams,
    expectedRouteParamKeys,
    expectedSearchParamKeys,
    sourceBuildRouteParamKeys,
    sourceBuildSearchParamKeys,
    sourceBuildRequestPropsMatchManifest: sourceBuildRequestPropMismatchRoutes.length === 0,
    sourceBuildRequestPropMismatchCount: sourceBuildRequestPropMismatchRoutes.length,
    sourceBuildRequestPropMismatchRoutes,
    missingSourceBuildRoutesMatchComputed: arrayEquals(
      declaredMissingSourceBuildRouteKeys,
      missingSourceBuildRouteKeys,
    ),
  };
  summary.requestPropsMatchComputed =
    summary.routesWithRouteParams === summary.expectedRoutesWithRouteParams &&
    summary.routesWithSearchParams === summary.expectedRoutesWithSearchParams &&
    arrayEquals(summary.routeParamKeys, summary.expectedRouteParamKeys) &&
    arrayEquals(summary.searchParamKeys, summary.expectedSearchParamKeys);
  summary.consistentWithComputedRoutes =
    summary.present &&
    summary.sourceBuildRoutes === summary.expectedSourceBuildRoutes &&
    summary.manifestRoutes === summary.expectedManifestRoutes &&
    summary.sourceBuildEntries === summary.expectedSourceBuildEntries &&
    summary.manifestEntries === summary.expectedManifestEntries &&
    summary.manifestIncludesSourceBuildRoutes === summary.expectedManifestIncludesSourceBuildRoutes &&
    summary.missingSourceBuildRoutesMatchComputed &&
    summary.requestPropsMatchComputed &&
    summary.sourceBuildRequestPropsMatchManifest;
  return summary;
}

module.exports = {
  summarizeRouteManifest,
};
