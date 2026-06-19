const REQUIRED_ROUTE_HANDLERS = Object.freeze([
  Object.freeze({
    key: "healthGet",
    sourcePath: "app/api/health/route.ts",
    route: "/api/health",
    method: "GET",
    expectedStatus: 200,
    buildReceipt: "receipt",
  }),
  Object.freeze({
    key: "checkoutPost",
    sourcePath: "app/api/checkout/route.ts",
    route: "/api/checkout",
    method: "POST",
    expectedStatus: 202,
    buildReceipt: "skipped",
  }),
]);

const PRIMARY_ROUTE_HANDLER = REQUIRED_ROUTE_HANDLERS[0];

function normalizeRouteHandlerRequirement(value = PRIMARY_ROUTE_HANDLER) {
  if (typeof value === "string") {
    const sourcePath = normalizeRouteHandlerSourcePath(value);
    return (
      REQUIRED_ROUTE_HANDLERS.find(
        (handler) => normalizeRouteHandlerSourcePath(handler.sourcePath) === sourcePath,
      ) || {
        key: sourcePath,
        sourcePath,
        route: null,
        method: null,
        expectedStatus: 200,
      }
    );
  }

  return {
    ...value,
    sourcePath: normalizeRouteHandlerSourcePath(value.sourcePath),
    route: normalizeRouteHandlerRoute(value.route),
    method: normalizeRouteHandlerMethod(value.method),
  };
}

function normalizeRouteHandlerSourcePath(value) {
  return String(value || "")
    .replace(/\\/g, "/")
    .replace(/^\.\//, "");
}

function normalizeRouteHandlerRoute(value) {
  if (value == null || value === "") {
    return null;
  }

  let route = String(value).trim();
  try {
    if (/^[a-zA-Z][a-zA-Z\d+.-]*:/.test(route)) {
      route = new URL(route).pathname;
    }
  } catch {
    // Keep the original value and normalize it as a route-like path below.
  }
  route = route.split(/[?#]/, 1)[0];
  if (!route.startsWith("/")) {
    route = `/${route}`;
  }
  route = route.replace(/\/+$/, "");
  return route || "/";
}

function normalizeRouteHandlerMethod(value) {
  if (value == null || value === "") {
    return null;
  }
  return String(value).trim().toUpperCase();
}

module.exports = {
  PRIMARY_ROUTE_HANDLER,
  REQUIRED_ROUTE_HANDLERS,
  normalizeRouteHandlerMethod,
  normalizeRouteHandlerRequirement,
  normalizeRouteHandlerRoute,
  normalizeRouteHandlerSourcePath,
};
