const { FIXTURE_PATHS } = require("./fixture-paths.ts");
const { summarizeGraphConsumerSnapshot } = require("./graph-consumer-snapshot.ts");
const { summarizeAssetOutput, summarizeStyleOutput } = require("./manifest-output.ts");
const { relativePath } = require("./io.ts");
const { summarizeGraphReceipt } = require("./route-handler-graph.ts");
const {
  summarizeRouteHandlerManifest,
  summarizeUnexpectedRouteHandlerManifests,
} = require("./route-handler-manifest.ts");
const { summarizeRouteHandlerReceipts } = require("./route-handler-receipts.ts");
const { summarizeRouteHandlerStaleEvidence } = require("./route-handler-stale-evidence.ts");
const { summarizeRootRouteOutput } = require("./route-output.ts");
const { summarizeAppRouterOutputs } = require("./source-build-app-router.ts");
const { sourceBuildFailures } = require("./source-build-failures.ts");
const { summarizeFixture } = require("./source-build-fixture.ts");
const { summarizeSourceModuleResolver } = require("./source-module-resolver.ts");

function summarizeSourceBuild(input) {
  const manifest = input.sourceBuildManifest.value || {};
  const receipt = input.sourceBuildReceipt.value || {};
  const chunks = sourceModuleChunks(manifest);
  const rootRouteOutput = summarizeRootRouteOutput(input.projectRoot, manifest, "/", FIXTURE_PATHS.appPage);
  const routeHandler = summarizeRouteHandlerManifest(manifest, FIXTURE_PATHS.routeHandler, "/api/health");
  const checkoutRouteHandler = summarizeRouteHandlerManifest(
    manifest,
    FIXTURE_PATHS.checkoutRouteHandler,
    "/api/checkout",
  );
  const unexpectedRouteHandlers = summarizeUnexpectedRouteHandlerManifests(manifest);
  const routeHandlerReceipt = summarizeRouteHandlerReceipts(input, FIXTURE_PATHS.routeHandler);
  const graphReceipt = summarizeGraphReceipt(input.graphReceipt);
  const styleOutput = summarizeStyleOutput(input.projectRoot, manifest.styles, FIXTURE_PATHS.styleSource);
  const publicAssetOutput = summarizeAssetOutput(input.projectRoot, manifest.assets, FIXTURE_PATHS.publicAsset);

  return {
    manifestPath: relativePath(input.projectRoot, input.sourceBuildManifestPath),
    receiptPath: relativePath(input.projectRoot, input.sourceBuildReceiptPath),
    canonicalReceiptPath: relativePath(input.projectRoot, input.canonicalReceiptPath),
    routeHandlerReceiptsPath: relativePath(input.projectRoot, input.routeHandlerReceiptsPath),
    graphReceiptPath: relativePath(input.projectRoot, input.graphReceiptPath),
    graphConsumerSnapshotPath: relativePath(input.projectRoot, input.graphConsumerSnapshotPath),
    manifest: {
      present: input.sourceBuildManifest.ok,
      schema: manifest.schema || null,
      hasRootRoute: hasArrayItem(manifest.routes, (route) => route.route === "/" && route.path === FIXTURE_PATHS.appPage),
      hasRouteHandler: routeHandler.present,
      routeHandler,
      hasCheckoutRouteHandler: checkoutRouteHandler.present,
      checkoutRouteHandler,
      routeHandlerMethods: routeHandler.methods,
      routeHandlerDeclaresNoNodeModules: routeHandler.declaresNoNodeModules,
      checkoutRouteHandlerMethods: checkoutRouteHandler.methods,
      checkoutRouteHandlerDeclaresNoNodeModules: checkoutRouteHandler.declaresNoNodeModules,
      unexpectedRouteHandlers,
      rootRouteOutput,
      hasStyle: hasArrayItem(manifest.styles, (style) => style.path === FIXTURE_PATHS.styleSource),
      hasStyleOutput: styleOutput.present,
      styleOutputPath: styleOutput.path,
      styleOutput,
      hasPublicAsset: hasArrayItem(manifest.assets, (asset) => asset.path === FIXTURE_PATHS.publicAsset),
      hasPublicAssetOutput: publicAssetOutput.present,
      publicAssetOutputPath: publicAssetOutput.outputPath,
      publicAssetSourceOutputPath: publicAssetOutput.sourceOutputPath,
      publicAssetHashedOutputPath: publicAssetOutput.outputPath,
      publicAssetOutput,
      hasLinkedComponent: chunks.some((chunk) => chunk.source_path === FIXTURE_PATHS.component),
      hasLinkedServerLoader: chunks.some((chunk) => chunk.source_path === FIXTURE_PATHS.serverLoader),
      hasLinkedServerModule: chunks.some((chunk) => chunk.source_path === FIXTURE_PATHS.serverModule),
      resolverEvidence: summarizeSourceModuleResolver(manifest),
      declaresNoNodeModules: manifest.node_modules_required === false,
      nodeModulesRequired: manifest.node_modules_required === true,
    },
    receipt: {
      present: input.sourceBuildReceipt.ok,
      schema: receipt.schema || null,
      routeHandlers: receipt.summary?.route_handlers ?? null,
      declaresNoNodeModules: receipt.node_modules_required === false,
      nodeModulesRequired: receipt.node_modules_required === true,
    },
    canonicalReceipt: {
      present: input.canonicalReceipt.ok,
      schema: input.canonicalReceipt.ok ? input.canonicalReceipt.value.schema || null : null,
    },
    routeHandlerReceipt,
    graphReceipt,
    routeHandlerStaleEvidence: summarizeRouteHandlerStaleEvidence({
      manifest: {
        unexpectedRouteHandlers,
      },
      graphReceipt,
      routeHandlerReceipt,
    }),
    graphConsumerSnapshot: summarizeGraphConsumerSnapshot(input.graphConsumerSnapshot),
  };
}

function sourceModuleChunks(manifest) {
  if (!Array.isArray(manifest.route_outputs)) {
    return [];
  }
  return manifest.route_outputs.flatMap((output) => output.source_module_chunks || []);
}

function hasArrayItem(value, predicate) {
  return Array.isArray(value) && value.some(predicate);
}

module.exports = {
  sourceBuildFailures,
  summarizeAppRouterOutputs,
  summarizeFixture,
  summarizeSourceBuild,
};
