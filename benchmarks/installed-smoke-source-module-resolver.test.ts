import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const { summarizeSourceBuild } = require("../tools/build/installed-smoke/source-build.ts");
const { sourceBuildFailures } = require("../tools/build/installed-smoke/source-build-failures.ts");

test("source-build summary exposes resolver diagnostics and node_modules dependencies", () => {
  const summary = summarizeSourceBuild(createSourceBuildInput(sourceModuleResolverManifest()));
  const resolverEvidence = summary.manifest.resolverEvidence;

  assert.equal(resolverEvidence?.present, true);
  assert.equal(resolverEvidence.moduleCount, 3);
  assert.equal(resolverEvidence.diagnosticCount, 2);
  assert.equal(resolverEvidence.nodeModuleModuleCount, 1);
  assert.equal(resolverEvidence.nodeModuleDependencyCount, 1);
  assert.deepEqual(resolverEvidence.diagnosticModules, [
    { sourcePath: "components/LaunchCard.tsx", diagnosticCount: 2 },
  ]);
  assert.deepEqual(resolverEvidence.nodeModuleModules, [{ sourcePath: "server/loaders.ts" }]);
  assert.deepEqual(resolverEvidence.nodeModuleDependencies, [
    {
      sourcePath: "app/page.tsx",
      specifier: "@vendor/widget",
      resolvedPath: "node_modules/@vendor/widget/index.js",
    },
  ]);
});

test("source-build failures reject source-module resolver diagnostics and node_modules dependencies", () => {
  const failures = sourceBuildFailures(createPassingReport(sourceModuleResolverEvidence()));

  for (const expectedFailure of [
    "source-build source module components/LaunchCard.tsx has 2 resolver diagnostics",
    "source-build source module server/loaders.ts requires node_modules",
    "source-build source module app/page.tsx dependency @vendor/widget requires node_modules",
  ]) {
    assert.ok(failures.includes(expectedFailure), JSON.stringify(failures, null, 2));
  }
});

function sourceModuleResolverManifest() {
  return {
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
        source_module_chunks: [
          {
            source_path: "app/page.tsx",
            dependencies: [
              {
                specifier: "@vendor/widget",
                resolved_path: "node_modules/@vendor/widget/index.js",
                node_modules_required: false,
              },
            ],
            node_modules_required: false,
          },
          {
            source_path: "components/LaunchCard.tsx",
            dependencies: [],
            diagnostics: 2,
            node_modules_required: false,
          },
        ],
      },
    ],
    source_modules: [
      {
        source_path: "server/loaders.ts",
        dependencies: [],
        diagnostics: 0,
        node_modules_required: true,
      },
    ],
    styles: [],
    assets: [],
    node_modules_required: false,
  };
}

function sourceModuleResolverEvidence() {
  return {
    present: true,
    moduleCount: 3,
    diagnosticCount: 2,
    nodeModuleModuleCount: 1,
    nodeModuleDependencyCount: 1,
    diagnosticModules: [{ sourcePath: "components/LaunchCard.tsx", diagnosticCount: 2 }],
    nodeModuleModules: [{ sourcePath: "server/loaders.ts" }],
    nodeModuleDependencies: [
      {
        sourcePath: "app/page.tsx",
        specifier: "@vendor/widget",
        resolvedPath: "node_modules/@vendor/widget/index.js",
      },
    ],
  };
}

function createSourceBuildInput(sourceBuildManifest) {
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-source-module-resolver-"));
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
    graphReceipt: { ok: true, value: { graph: { nodes: [] } } },
    graphConsumerSnapshot: { ok: true, value: { nodes: [] } },
  };
}

function createPassingReport(resolverEvidence) {
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
          rootRouteOutput: rootRouteOutput(),
          hasStyle: true,
          hasStyleOutput: true,
          hasPublicAsset: true,
          hasPublicAssetOutput: true,
          hasLinkedComponent: true,
          hasLinkedServerLoader: true,
          hasLinkedServerModule: true,
          resolverEvidence,
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

function rootRouteOutput() {
  return {
    present: true,
    route: "/",
    sourcePath: "app/page.tsx",
    serverData: { present: true },
    duplicateCount: 0,
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
    routeHandlerDuplicateCount: 0,
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
