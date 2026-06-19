import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const path = require("node:path");
const test = require("node:test");

const {
  routeHandlerReceiptFailures,
  summarizeRouteHandlerReceipts,
} = require("../tools/build/installed-smoke/route-handler-receipts.ts");
const { sourceBuildFailures } = require("../tools/build/installed-smoke/source-build-failures.ts");

const DUPLICATE_RECEIPT_MESSAGE =
  ".dx/build/route-handler-receipts.json has duplicate app/api/health/route.ts GET /api/health receipts";
const DUPLICATE_SKIP_MESSAGE =
  ".dx/build/route-handler-receipts.json has duplicate app/api/checkout/route.ts POST /api/checkout skipped evidence";

test("installed smoke route-handler receipts reject duplicate receipt and skip evidence", () => {
  const summary = summarizeRouteHandlerReceipts(createReceiptInput());

  assert.equal(summary.receiptCount, 2);
  assert.equal(summary.skippedCount, 2);
  assert.equal(summary.requiredReceipts[0].duplicateCount, 1);
  assert.equal(summary.requiredSkips[0].duplicateCount, 1);
  assert.deepEqual(summary.routeHandlerReadiness, [
    {
      key: "healthGet",
      sourcePath: "app/api/health/route.ts",
      route: "/api/health",
      method: "GET",
      buildStatus: "executed",
      expectedStatus: 200,
      responseStatus: 200,
      skipReason: null,
      duplicateCount: 1,
      sourceOwnedRuntimeBoundary: true,
      externalRuntimeRequired: false,
      externalRuntimeExecuted: false,
      declaresNoNodeModules: true,
      lifecycleScriptsExecuted: false,
    },
    {
      key: "checkoutPost",
      sourcePath: "app/api/checkout/route.ts",
      route: "/api/checkout",
      method: "POST",
      buildStatus: "skipped",
      expectedStatus: 202,
      responseStatus: null,
      skipReason: "unsafe-build-request-body",
      duplicateCount: 1,
      sourceOwnedRuntimeBoundary: null,
      externalRuntimeRequired: null,
      externalRuntimeExecuted: null,
      declaresNoNodeModules: null,
      lifecycleScriptsExecuted: null,
    },
  ]);

  const failures = routeHandlerReceiptFailures(summary);
  assert.ok(failures.includes(DUPLICATE_RECEIPT_MESSAGE), JSON.stringify(failures, null, 2));
  assert.ok(failures.includes(DUPLICATE_SKIP_MESSAGE), JSON.stringify(failures, null, 2));
});

test("installed smoke route-handler readiness normalizes source paths, request paths, and methods", () => {
  const summary = summarizeRouteHandlerReceipts(createNormalizedReceiptInput());

  assert.equal(summary.requiredReceiptCount, 1);
  assert.equal(summary.requiredSkipCount, 1);
  assert.equal(summary.routeHandlerReadiness[0].buildStatus, "executed");
  assert.equal(summary.routeHandlerReadiness[0].responseStatus, 200);
  assert.equal(summary.routeHandlerReadiness[1].buildStatus, "skipped");
  assert.equal(summary.routeHandlerReadiness[1].skipReason, "unsafe-build-request-body");
  assert.deepEqual(routeHandlerReceiptFailures(summary), []);
});

test("source-build failures include duplicate route-handler receipt evidence", () => {
  const summary = summarizeRouteHandlerReceipts(createReceiptInput());
  const failures = sourceBuildFailures(createPassingReport(summary));

  assert.ok(failures.includes(DUPLICATE_RECEIPT_MESSAGE), JSON.stringify(failures, null, 2));
  assert.ok(failures.includes(DUPLICATE_SKIP_MESSAGE), JSON.stringify(failures, null, 2));
});

function createReceiptInput() {
  const healthReceipt = routeHandlerReceipt("app/api/health/route.ts", "/api/health", "GET");
  const checkoutSkip = routeHandlerSkip("app/api/checkout/route.ts", "/api/checkout", "POST");
  return {
    projectRoot: "G:/Dx/www",
    routeHandlerReceiptsPath: path.join("G:/Dx/www", ".dx", "build", "route-handler-receipts.json"),
    routeHandlerReceipts: {
      ok: true,
      value: {
        schema: "dx.next.appRouteHandlerBuildReceipts",
        format: 1,
        node_modules_required: false,
        node_modules_present: false,
        lifecycle_scripts_executed: false,
        receipts: [healthReceipt, { ...healthReceipt }],
        skipped: [checkoutSkip, { ...checkoutSkip }],
      },
    },
  };
}

function createNormalizedReceiptInput() {
  return {
    projectRoot: "G:/Dx/www",
    routeHandlerReceiptsPath: path.join("G:/Dx/www", ".dx", "build", "route-handler-receipts.json"),
    routeHandlerReceipts: {
      ok: true,
      value: {
        schema: "dx.next.appRouteHandlerBuildReceipts",
        format: 1,
        node_modules_required: false,
        node_modules_present: false,
        lifecycle_scripts_executed: false,
        receipts: [
          {
            ...routeHandlerReceipt("app\\api\\health\\route.ts", "http://localhost:3000/api/health?probe=1", "get"),
            response: {
              status: 200,
              content_type: "application/json; charset=utf-8",
              header_count: 1,
            },
          },
        ],
        skipped: [routeHandlerSkip(".\\app\\api\\checkout\\route.ts", "/api/checkout/", "post")],
      },
    },
  };
}

function routeHandlerReceipt(sourcePath, requestPath, method) {
  return {
    schema: "dx.next.appRouteHandlerReceipt",
    format: 1,
    source_path: sourcePath,
    request_path: requestPath,
    method,
    execution_model: "source-owned-route-handler-contract",
    node_modules_required: false,
    node_modules_present: false,
    lifecycle_scripts_executed: false,
    runtime_boundary: {
      source_owned: true,
      external_runtime_required: false,
      external_runtime_executed: false,
    },
    response: {
      status: 200,
      content_type: "application/json",
      header_count: 1,
    },
    adapter_boundary: ["Does not import Next.js Route Handler runtime."],
  };
}

function routeHandlerSkip(sourcePath, requestPath, method) {
  return {
    source_path: sourcePath,
    request_path: requestPath,
    method,
    reason: "unsafe-build-request-body",
  };
}

function createPassingReport(routeHandlerReceipt) {
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
          resolverEvidence: {
            present: true,
            moduleCount: 3,
            diagnosticCount: 0,
            nodeModuleModuleCount: 0,
            nodeModuleDependencyCount: 0,
            diagnosticModules: [],
            nodeModuleModules: [],
            nodeModuleDependencies: [],
          },
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
        routeHandlerReceipt,
        graphReceipt: graphReceiptSummary(),
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
