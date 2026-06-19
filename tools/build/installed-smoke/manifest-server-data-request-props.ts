function sumEntryCounts(routes) {
  return routes.reduce(
    (total, route) => total + (Number.isInteger(route.entryCount) ? route.entryCount : 0),
    0,
  );
}

function requestPropRouteCount(routes, key) {
  return routes.filter((route) => route[key].length > 0).length;
}

function requestPropKeysForRoutes(routes, key) {
  return [...new Set(routes.flatMap((route) => route[key]))].sort();
}

function integerOrNull(value) {
  return Number.isInteger(value) ? value : null;
}

module.exports = {
  integerOrNull,
  requestPropKeysForRoutes,
  requestPropRouteCount,
  sumEntryCounts,
};
