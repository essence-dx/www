import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const { summarizeGraphRouteHandler } = require("../tools/build/installed-smoke/route-handler-graph.ts");
const { summarizeSourceBuild } = require("../tools/build/installed-smoke/source-build.ts");
const { sourceBuildFailures } = require("../tools/build/installed-smoke/source-build-failures.ts");

const DUPLICATE_GRAPH_MESSAGE =
  "dx.build.graph has duplicate app/api/health/route.ts /api/health route-handler nodes";

test("installed smoke graph summary reports duplicate route-handler nodes", () => {
  const routeHandler = summarizeGraphRouteHandler(
    duplicateHealthGraphReceipt(),
    "app/api/health/route.ts",
    "/api/health",
  );

  assert.equal(routeHandler.present, true);
  assert.equal(routeHandler.duplicateCount, 1);
});

test("installed smoke graph summary normalizes route-handler source paths, routes, and methods", () => {
  const routeHandler = summarizeGraphRouteHandler(
    {
      graph: {
        nodes: [
          graphRouteHandlerNode("app\\api\\health\\route.ts", "/api/health/", ["get"]),
        ],
      },
    },
    "app/api/health/route.ts",
    "/api/health",
  );

  assert.equal(routeHandler.present, true);
  assert.deepEqual(routeHandler.methods, ["GET"]);
});

test("source-build failures include duplicate graph route-handler evidence", () => {
  const routeHandler = summarizeGraphRouteHandler(
    duplicateHealthGraphReceipt(),
    "app/api/health/route.ts",
    "/api/health",
  );
  const failures = sourceBuildFailures(createPassingReport(routeHandler));

  assert.ok(failures.includes(DUPLICATE_GRAPH_MESSAGE), JSON.stringify(failures, null, 2));
});

test("source-build summary carries duplicate graph route-handler evidence", () => {
  const summary = summarizeSourceBuild(createSourceBuildInput(duplicateHealthGraphReceipt()));

  assert.equal(summary.graphReceipt.routeHandlerDuplicateCount, 1);
});

function duplicateHealthGraphReceipt() {
  const healthNode = graphRouteHandlerNode("app/api/health/route.ts", "/api/health", ["GET"]);
  return {
    graph: {
      nodes: [healthNode, { ...healthNode, id: "app-route-handler:health-duplicate" }],
    },
  };
}

function createSourceBuildInput(graphReceipt) {
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-route-handler-graph-"));
  const sourceBuildManifest = {
    schema: "dx.www.sourceBuildManifest",
    routes: [{ route: "/", path: "app/page.tsx" }],
    route_handlers: [
      manifestRouteHandler("app/api/health/route.ts", "/api/health", ["GET"]),
      manifestRouteHandler("app/api/checkout/route.ts", "/api/checkout", ["POST"]),
    ],
    route_outputs: [
      {
        route: "/",
        source_path: "app/page.tsx",
        server_data_output: ".dx/build/source-routes/root/server-data.json",
      },
    ],
    styles: [],
    assets: [],
    node_modules_required: false,
  };

  return {
    projectRoot,
    sourceBuildManifestPath: path.join(projectRoot, ".dx", "build", "source-build-manifest.json"),
    sourceBuildReceiptPath: path.join(projectRoot, ".dx", "build", "source-build-receipt.json"),
    canonicalReceiptPath: path.join(projectRoot, ".dx", "receipts", "build", "latest.json"),
    routeHandlerReceiptsPath: path.join(projectRoot, ".dx", "build", "route-handler-receipts.json"),
    graphReceiptPath: path.join(projectRoot, ".dx", "receipts", "graph", "latest.json"),
    graphConsumerSnapshotPath: path.join(projectRoot, ".dx", "receipts", "graph", "consumer-snapshot.json"),
    sourceBuildManifest: { ok: true, value: sourceBuildManifest },
    sourceBuildReceipt: {
      ok: true,
      value: {
        schema: "dx.www.sourceBuildReceipt",
        summary: { route_handlers: 2 },
        node_modules_required: false,
      },
    },
    canonicalReceipt: { ok: true, value: { schema: "dx.www.sourceBuildReceipt" } },
    routeHandlerReceipts: { ok: true, value: { receipts: [], skipped: [] } },
    graphReceipt: { ok: true, value: graphReceipt },
    graphConsumerSnapshot: { ok: true, value: graphConsumerSnapshotSummary() },
  };
}

function createPassingReport(graphRouteHandler) {
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
          routeHandler: routeHandlerManifestSummary("app/api/health/route.ts", "/api/health", "GET"),
          checkoutRouteHandler: routeHandlerManifestSummary("app/api/checkout/route.ts", "/api/checkout", "POST"),
          rootRouteOutput: {
            present: true,
            route: "/",
            sourcePath: "app/page.tsx",
            serverData: { present: true },
            duplicateCount: 0,
          },
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
        graphReceipt: graphReceiptSummary(graphRouteHandler),
        graphConsumerSnapshot: graphConsumerSnapshotSummary(),
      },
    },
  };
}

function routeHandlerManifestSummary(sourcePath, route, method) {
  return {
    present: true,
    sourcePath,
    route,
    requiredMethod: method,
    methods: [method],
    duplicateCount: 0,
    declaresNoNodeModules: true,
    nodeModulesRequired: false,
    lifecycleScriptsExecuted: false,
    executionModel: "source-owned-route-handler-contract",
  };
}

function graphRouteHandlerNode(sourcePath, route, methods) {
  return {
    id: `app-route-handler:${route}`,
    kind: "app-route-handler",
    path: sourcePath,
    route,
    methods,
    node_modules_required: false,
    lifecycle_scripts_executed: false,
    execution_model: "source-owned-route-handler-contract",
  };
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

function graphReceiptSummary(graphRouteHandler) {
  return {
    present: true,
    schema: "dx.build.graph",
    hasRouteHandlerNode: graphRouteHandler.present,
    routeHandlerMethods: graphRouteHandler.methods,
    routeHandlerExecutionModel: graphRouteHandler.executionModel,
    routeHandlerDeclaresNoNodeModules: graphRouteHandler.declaresNoNodeModules,
    routeHandlerNodeModulesRequired: graphRouteHandler.nodeModulesRequired,
    routeHandlerLifecycleScriptsExecuted: graphRouteHandler.lifecycleScriptsExecuted,
    routeHandlerDuplicateCount: graphRouteHandler.duplicateCount,
    hasCheckoutRouteHandlerNode: true,
    checkoutRouteHandlerMethods: ["POST"],
    checkoutRouteHandlerExecutionModel: "source-owned-route-handler-contract",
    checkoutRouteHandlerDeclaresNoNodeModules: true,
    checkoutRouteHandlerNodeModulesRequired: false,
    checkoutRouteHandlerLifecycleScriptsExecuted: false,
    checkoutRouteHandlerDuplicateCount: 0,
    hasServerLoaderSourceModule: true,
    serverLoaderSourceModuleDeclaresNoNodeModules: true,
    serverLoaderSourceModuleCompiledFromSource: true,
    hasServerModuleSourceModule: true,
    serverModuleSourceModuleDeclaresNoNodeModules: true,
    serverModuleSourceModuleCompiledFromSource: true,
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
