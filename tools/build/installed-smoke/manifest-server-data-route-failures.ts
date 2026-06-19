const { manifestServerDataRouteLabel } = require("./manifest-server-data-route-keys.ts");

function manifestServerDataRouteFailures(report) {
  const summary = report.build.manifest.serverDataRoutes;
  const failures = [];

  pushIf(failures, !summary.present, "manifest is missing server_data_routes");
  pushIf(
    failures,
    summary.present && summary.compiledCount !== summary.routeCount,
    "manifest server_data_routes_compiled does not match server_data_routes",
  );
  pushIf(
    failures,
    summary.present && !summary.hasRootRoute,
    "manifest server_data_routes is missing the root app route",
  );
  pushIf(failures, !summary.routeManifest.present, "manifest is missing server_data_route_manifest");
  pushRouteManifestFailures(failures, summary);
  pushMissingAndDuplicateRouteFailures(failures, summary);

  for (const route of summary.routes || []) {
    pushManifestRouteFailures(failures, route);
  }

  return failures;
}

function pushRouteManifestFailures(failures, summary) {
  const routeManifest = summary.routeManifest;
  if (!routeManifest.present) {
    return;
  }

  pushIf(
    failures,
    routeManifest.sourceBuildRoutes !== routeManifest.expectedSourceBuildRoutes,
    "manifest server_data_route_manifest source_build_routes does not match source-build server-data routes",
  );
  pushIf(
    failures,
    routeManifest.manifestRoutes !== routeManifest.expectedManifestRoutes,
    "manifest server_data_route_manifest manifest_routes does not match server_data_routes",
  );
  pushIf(
    failures,
    routeManifest.sourceBuildEntries !== routeManifest.expectedSourceBuildEntries,
    "manifest server_data_route_manifest source_build_entries does not match source-build server-data entries",
  );
  pushIf(
    failures,
    routeManifest.manifestEntries !== routeManifest.expectedManifestEntries,
    "manifest server_data_route_manifest manifest_entries does not match server_data_routes entries",
  );
  pushIf(
    failures,
    !routeManifest.missingSourceBuildRoutesMatchComputed,
    "manifest server_data_route_manifest missing_source_build_routes disagrees with source-build server-data routes",
  );
  pushIf(
    failures,
    routeManifest.routesWithRouteParams !== routeManifest.expectedRoutesWithRouteParams,
    "manifest server_data_route_manifest routes_with_route_params disagrees with server_data_routes requests",
  );
  pushIf(
    failures,
    routeManifest.routesWithSearchParams !== routeManifest.expectedRoutesWithSearchParams,
    "manifest server_data_route_manifest routes_with_search_params disagrees with server_data_routes requests",
  );
  pushIf(
    failures,
    !sameArray(routeManifest.routeParamKeys, routeManifest.expectedRouteParamKeys),
    "manifest server_data_route_manifest route_param_keys disagree with server_data_routes requests",
  );
  pushIf(
    failures,
    !sameArray(routeManifest.searchParamKeys, routeManifest.expectedSearchParamKeys),
    "manifest server_data_route_manifest search_param_keys disagree with server_data_routes requests",
  );
  pushIf(
    failures,
    routeManifest.sourceBuildRequestPropsMatchManifest === false,
    "manifest server_data_route_manifest source-build request props do not match server_data_routes requests",
  );
  pushIf(
    failures,
    routeManifest.manifestIncludesSourceBuildRoutes !==
      routeManifest.expectedManifestIncludesSourceBuildRoutes,
    "manifest server_data_route_manifest manifest_includes_source_build_routes disagrees with source-build server-data routes",
  );
}

function pushMissingAndDuplicateRouteFailures(failures, summary) {
  for (const route of summary.missingSourceBuildRoutes || []) {
    failures.push(
      `manifest server_data_routes is missing source-build route output ${manifestServerDataRouteLabel(route)}`,
    );
  }

  for (const route of summary.duplicateManifestRoutes || []) {
    failures.push(`manifest server_data_routes has duplicate route ${manifestServerDataRouteLabel(route)}`);
  }
}

function pushManifestRouteFailures(failures, route) {
  const label = manifestServerDataRouteLabel(route);
  pushIf(failures, !route.output.present, `manifest server_data_routes ${label} output was not emitted`);
  pushIf(failures, route.output.path && !route.output.insideProject, `manifest server_data_routes ${label} output escapes the project root`);
  pushIf(failures, route.output.present && !route.artifact.parseOk, `manifest server_data_routes ${label} output is not valid JSON`);
  pushIf(failures, route.output.present && route.artifact.parseOk && !route.artifactMatchesManifest, `manifest server_data_routes ${label} output payload does not match manifest route`);
  pushIf(failures, route.output.present && route.artifact.parseOk && !route.requestPropsMatchArtifact, `manifest server_data_routes ${label} request props do not match emitted artifact`);
  pushIf(failures, route.requestPropsMatchSourceBuildRoute === false, `manifest server_data_routes ${label} request props do not match source-build server-data route`);
  pushIf(failures, !route.matchesSourceBuildRouteOutput, `manifest server_data_routes ${label} output does not match source-build route output`);
  pushIf(failures, !route.matchesSourceBuildServerDataRoute, `manifest server_data_routes ${label} output does not match source-build server-data route`);
  pushIf(failures, !route.declaresNoNodeModules, `manifest server_data_routes ${label} does not declare node_modules_required=false`);
  pushIf(failures, route.lifecycleScriptsExecuted, `manifest server_data_routes ${label} executed lifecycle scripts`);
  pushIf(failures, !route.sourceOwnedContract, `manifest server_data_routes ${label} does not declare source_owned_contract=true`);
  pushIf(failures, route.externalRuntimeRequired, `manifest server_data_routes ${label} requires an external runtime`);
  pushIf(failures, route.externalRuntimeExecuted, `manifest server_data_routes ${label} executed an external runtime`);
}

function sameArray(left, right) {
  return left.length === right.length && left.every((value, index) => value === right[index]);
}

function pushIf(failures, condition, message) {
  if (condition) {
    failures.push(message);
  }
}

module.exports = {
  manifestServerDataRouteFailures,
};
