const FIXTURE_PATHS = {
  appPage: "app/page.tsx",
  appLayout: "app/layout.tsx",
  component: "components/LaunchCard.tsx",
  routeHandler: "app/api/health/route.ts",
  checkoutRouteHandler: "app/api/checkout/route.ts",
  serverLoader: "server/loaders.ts",
  serverModule: "server/launch-copy.ts",
  styleSource: "styles/app.css",
  publicAsset: "public/icons/mark.svg",
};

module.exports = {
  EXPECTED_ROUTE_HANDLER_COUNT: 2,
  FIXTURE_PATHS,
};
