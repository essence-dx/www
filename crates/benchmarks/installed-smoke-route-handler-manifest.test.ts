import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const test = require("node:test");

const {
  summarizeRouteHandlerManifest,
} = require("../tools/build/installed-smoke/route-handler-manifest.ts");
const {
  routeHandlerManifestFailures,
} = require("../tools/build/installed-smoke/route-handler-manifest-failures.ts");
const { sourceBuildFailures } = require("../tools/build/installed-smoke/source-build-failures.ts");

const DUPLICATE_HEALTH_MESSAGE =
  "source-build manifest has duplicate app/api/health/route.ts /api/health route-handler evidence";

test("installed smoke route-handler manifest rejects duplicate handler evidence", () => {
  const routeHandler = summarizeRouteHandlerManifest(duplicateHealthManifest(), {
    sourcePath: "app/api/health/route.ts",
    route: "/api/health",
    method: "GET",
  });

  assert.equal(routeHandler.present, true);
  assert.equal(routeHandler.duplicateCount, 1);
  assert.deepEqual(routeHandlerManifestFailures(routeHandler), [DUPLICATE_HEALTH_MESSAGE]);
});

test("installed smoke route-handler manifest normalizes source paths, routes, and methods", () => {
  const routeHandlers = summarizeRouteHandlerManifest(
    {
      route_handlers: [
        manifestRouteHandler("app\\api\\health\\route.ts", "/api/health/", ["get"]),
      ],
    },
    {
      sourcePath: "app/api/health/route.ts",
      route: "/api/health",
      method: "GET",
    },
  );

  assert.equal(routeHandlers.present, true);
  assert.deepEqual(routeHandlers.methods, ["GET"]);
  assert.deepEqual(routeHandlerManifestFailures(routeHandlers), []);
});

test("source-build failures include duplicate route-handler manifest evidence", () => {
  const routeHandler = summarizeRouteHandlerManifest(duplicateHealthManifest(), {
    sourcePath: "app/api/health/route.ts",
    route: "/api/health",
    method: "GET",
  });
  const failures = sourceBuildFailures(createPassingReport(routeHandler));

  assert.ok(failures.includes(DUPLICATE_HEALTH_MESSAGE), JSON.stringify(failures, null, 2));
});

function duplicateHealthManifest() {
  const health = manifestRouteHandler("app/api/health/route.ts", "/api/health", ["GET"]);
  return {
    route_handlers: [health, { ...health }],
  };
}

function createPassingReport(routeHandler) {
  return {
    fixture: {
      hasAppPage: true,
      hasAppLayout: true,
      hasRouteHandler: true,
      hasCheckoutRouteHandler: true,
      hasComponent: true,
      hasServerLoader: true,
      hasServerModule: true,
      hasStyleSource: true,
      hasPublicAsset: true,
    },
    build: {
      appRouter: {
        rootHtmlPresent: true,
        rootPacketPresent: true,
        pageGraphPresent: true,
        executionContractPresent: true,
        execution: {
          hasRootRouteContract: true,
          declaresNoNodeModules: true,
          nodeModulesPresent: false,
          sourceOwnedRuntimeBoundary: true,
          externalRuntimeRequired: false,
          externalRuntimeExecuted: false,
        },
        serverDataPresent: true,
        serverData: {
          hasRootRouteContract: true,
          hasLoaderEntry: true,
          hasLoaderValue: true,
          declaresNoNodeModules: true,
          lifecycleScriptsExecuted: false,
        },
      },
      sourceBuild: {
        manifest: {
          present: true,
          schema: "dx.www.sourceBuildManifest",
          hasRootRoute: true,
          routeHandler,
          checkoutRouteHandler: passingRouteHandler("app/api/checkout/route.ts", "/api/checkout", "POST"),
          rootRouteOutput: rootRouteOutput(),
          hasStyle: true,
          hasStyleOutput: true,
          hasPublicAsset: true,
          hasPublicAssetOutput: true,
          hasLinkedComponent: true,
          hasLinkedServerLoader: true,
          hasLinkedServerModule: true,
          resolverEvidence: sourceModuleResolverEvidence(),
          declaresNoNodeModules: true,
          nodeModulesRequired: false,
        },
        receipt: {
          present: true,
          schema: "dx.www.sourceBuildReceipt",
          routeHandlers: 2,
          declaresNoNodeModules: true,
          nodeModulesRequired: false,
        },
        canonicalReceipt: { present: true, schema: "dx.www.sourceBuildReceipt" },
        routeHandlerReceipt: routeHandlerReceiptSummary(),
        graphReceipt: graphReceiptSummary(),
        graphConsumerSnapshot: graphConsumerSnapshotSummary(),
      },
    },
  };
}

function passingRouteHandler(sourcePath, route, method) {
  return summarizeRouteHandlerManifest(
    { route_handlers: [manifestRouteHandler(sourcePath, route, [method])] },
    { sourcePath, route, method },
  );
}

function manifestRouteHandler(sourcePath, route, methods) {
  return {
    path: sourcePath,
    route,
    methods,
    node_modules_required: false,
    lifecycle_scripts_executed: false,
    execution_model: "source-owned-route-handler-contract",
  };
}

function rootRouteOutput() {
  return {
    present: true,
    route: "/",
    sourcePath: "app/page.tsx",
    serverData: { present: true },
    duplicateCount: 0,
  };
}

function sourceModuleResolverEvidence() {
  return {
    present: true,
    moduleCount: 3,
    diagnosticCount: 0,
    nodeModuleModuleCount: 0,
    nodeModuleDependencyCount: 0,
    diagnosticModules: [],
    nodeModuleModules: [],
    nodeModuleDependencies: [],
  };
}

function routeHandlerReceiptSummary() {
  return {
    present: true,
    collectionSchema: "dx.next.appRouteHandlerBuildReceipts",
    collectionFormat: 1,
    collectionDeclaresNoNodeModules: true,
    collectionNodeModulesPresent: false,
    collectionLifecycleScriptsExecuted: false,
    requiredReceipts: [
      {
        present: true,
        sourcePath: "app/api/health/route.ts",
        method: "GET",
        schema: "dx.next.appRouteHandlerReceipt",
        format: 1,
        executionModel: "source-owned-route-handler-contract",
        expectedStatus: 200,
        responseStatus: 200,
        responseContentType: "application/json",
        responseHeaderCount: 1,
        hasAdapterBoundary: true,
        declaresNoNodeModules: true,
        nodeModulesRequired: false,
        nodeModulesPresent: false,
        lifecycleScriptsExecuted: false,
        sourceOwnedRuntimeBoundary: true,
        externalRuntimeRequired: false,
        externalRuntimeExecuted: false,
      },
    ],
    requiredSkips: [{ present: true, sourcePath: "app/api/checkout/route.ts", method: "POST" }],
  };
}

function graphReceiptSummary() {
  return {
    present: true,
    schema: "dx.build.graph",
    hasRouteHandlerNode: true,
    routeHandlerMethods: ["GET"],
    routeHandlerExecutionModel: "source-owned-route-handler-contract",
    routeHandlerDeclaresNoNodeModules: true,
    routeHandlerNodeModulesRequired: false,
    routeHandlerLifecycleScriptsExecuted: false,
    hasCheckoutRouteHandlerNode: true,
    checkoutRouteHandlerMethods: ["POST"],
    checkoutRouteHandlerExecutionModel: "source-owned-route-handler-contract",
    checkoutRouteHandlerDeclaresNoNodeModules: true,
    checkoutRouteHandlerNodeModulesRequired: false,
    checkoutRouteHandlerLifecycleScriptsExecuted: false,
    hasServerLoaderSourceModule: true,
    serverLoaderSourceModuleDeclaresNoNodeModules: true,
    serverLoaderSourceModuleCompiledFromSource: true,
    hasServerModuleSourceModule: true,
    serverModuleSourceModuleDeclaresNoNodeModules: true,
    serverModuleSourceModuleCompiledFromSource: true,
  };
}

function graphConsumerSnapshotSummary() {
  return {
    present: true,
    schema: "dx.build.graph.consumerSnapshot",
    sourceModuleCount: 2,
    coversSourceModuleKind: true,
    coversCompiledFromSourceEdge: true,
    zedPreviewSourceModuleKind: "source-module",
  };
}
