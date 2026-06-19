const fs = require("node:fs");

const { FIXTURE_PATHS } = require("./fixture-paths.ts");

function summarizeFixture(projectRoot) {
  return {
    hasAppPage: has(projectRoot, FIXTURE_PATHS.appPage),
    hasAppLayout: has(projectRoot, FIXTURE_PATHS.appLayout),
    hasRouteHandler: has(projectRoot, FIXTURE_PATHS.routeHandler),
    hasCheckoutRouteHandler: has(projectRoot, FIXTURE_PATHS.checkoutRouteHandler),
    hasComponent: has(projectRoot, FIXTURE_PATHS.component),
    hasServerLoader: has(projectRoot, FIXTURE_PATHS.serverLoader),
    hasServerModule: has(projectRoot, FIXTURE_PATHS.serverModule),
    hasStyleSource: has(projectRoot, FIXTURE_PATHS.styleSource),
    hasPublicAsset: has(projectRoot, FIXTURE_PATHS.publicAsset),
  };
}

function has(root, relative) {
  return fs.existsSync(`${root}/${relative}`);
}

module.exports = {
  summarizeFixture,
};
