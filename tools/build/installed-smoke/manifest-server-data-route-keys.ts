function manifestServerDataRouteLabel(route) {
  return [route.route, route.routeSourcePath].filter(Boolean).join(" ") || "<unknown-route>";
}

function serverDataRouteKey(route) {
  return `${route.route || ""}\0${route.routeSourcePath || ""}`;
}

function serverDataRouteOutputKey(route) {
  return `${serverDataRouteKey(route)}\0${route.output?.path || route.output || ""}`;
}

function sortedRouteOutputKeys(routes) {
  return routes.map(serverDataRouteOutputKey).sort();
}

function arrayEquals(left, right) {
  return left.length === right.length && left.every((value, index) => value === right[index]);
}

function duplicateServerDataRoutes(routes) {
  const seen = new Set();
  return routes.filter((route) => {
    const key = serverDataRouteKey(route);
    if (seen.has(key)) {
      return true;
    }
    seen.add(key);
    return false;
  });
}

module.exports = {
  arrayEquals,
  duplicateServerDataRoutes,
  manifestServerDataRouteLabel,
  serverDataRouteKey,
  serverDataRouteOutputKey,
  sortedRouteOutputKeys,
};
