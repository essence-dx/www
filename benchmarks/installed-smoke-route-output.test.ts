import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const { routeOutputFailures, summarizeRootRouteOutput } = require("../tools/build/installed-smoke/route-output.ts");
const { sourceBuildFailures } = require("../tools/build/installed-smoke/source-build-failures.ts");

function createManifest() {
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-route-output-"));
  for (const filePath of [
    ".dx/build/source-routes/root/index.html",
    ".dx/build/source-routes/root/index.dxpk",
    ".dx/build/source-routes/root/page-graph.json",
    ".dx/build/source-routes/root/server-data.json",
    ".dx/build/source-routes/root/modules/app-page.mjs",
  ]) {
    fs.mkdirSync(path.dirname(path.join(projectRoot, filePath)), { recursive: true });
    fs.writeFileSync(path.join(projectRoot, filePath), "{}");
  }

  const rootOutput = {
    route: "/",
    source_path: "app/page.tsx",
    html_output: ".dx/build/source-routes/root/index.html",
    packet_output: ".dx/build/source-routes/root/index.dxpk",
    page_graph_output: ".dx/build/source-routes/root/page-graph.json",
    server_data_output: ".dx/build/source-routes/root/server-data.json",
    entry_module_chunk_output: ".dx/build/source-routes/root/modules/app-page.mjs",
  };
  return { projectRoot, manifest: { route_outputs: [rootOutput, { ...rootOutput }] } };
}

test("installed smoke route output rejects duplicate root route outputs", () => {
  const { projectRoot, manifest } = createManifest();
  const rootRouteOutput = summarizeRootRouteOutput(projectRoot, manifest, "/", "app/page.tsx");

  assert.equal(rootRouteOutput.present, true);
  assert.equal(rootRouteOutput.duplicateCount, 1);
  assert.deepEqual(routeOutputFailures(rootRouteOutput), [
    "source-build manifest has duplicate root route outputs for / app/page.tsx",
  ]);
});

test("source-build failures include duplicate root route output evidence", () => {
  const { projectRoot, manifest } = createManifest();
  const rootRouteOutput = summarizeRootRouteOutput(projectRoot, manifest, "/", "app/page.tsx");
  const failures = sourceBuildFailures(createPassingReport(rootRouteOutput));

  assert.ok(
    failures.includes("source-build manifest has duplicate root route outputs for / app/page.tsx"),
    JSON.stringify(failures, null, 2),
  );
});

function createPassingReport(rootRouteOutput) {
  const routeHandler = manifestRouteHandler("app/api/health/route.ts", "GET");
  const checkoutRouteHandler = manifestRouteHandler("app/api/checkout/route.ts", "POST");
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
          checkoutRouteHandler,
          rootRouteOutput,
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

function manifestRouteHandler(sourcePath, method) {
  return {
    present: true,
    sourcePath,
    requiredMethod: method,
    methods: [method],
    declaresNoNodeModules: true,
    nodeModulesRequired: false,
    lifecycleScriptsExecuted: false,
    executionModel: "source-owned-route-handler-contract",
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
