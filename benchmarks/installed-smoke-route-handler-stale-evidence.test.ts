const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const {
  summarizeRouteHandlerManifest,
  summarizeUnexpectedRouteHandlerManifests,
} = require("../tools/build/installed-smoke/route-handler-manifest.ts");
const {
  routeHandlerManifestFailures,
} = require("../tools/build/installed-smoke/route-handler-manifest-failures.ts");
const {
  summarizeGraphReceipt,
} = require("../tools/build/installed-smoke/route-handler-graph.ts");
const { graphReceiptFailures } = require("../tools/build/installed-smoke/route-handler-graph-failures.ts");
const {
  routeHandlerReceiptFailures,
  summarizeRouteHandlerReceipts,
} = require("../tools/build/installed-smoke/route-handler-receipts.ts");
const { printHumanReport } = require("../tools/build/installed-smoke/human-report.ts");
const { summarizeSourceBuild } = require("../tools/build/installed-smoke/source-build.ts");
const { sourceBuildFailures } = require("../tools/build/installed-smoke/source-build-failures.ts");

const UNEXPECTED_MANIFEST_MESSAGE =
  "source-build manifest has unexpected stale app/api/stale/route.ts GET /api/stale route-handler evidence";
const UNEXPECTED_GRAPH_MESSAGE =
  "dx.build.graph has unexpected stale app/api/stale/route.ts GET /api/stale route-handler node";
const UNEXPECTED_RECEIPT_MESSAGE =
  ".dx/build/route-handler-receipts.json has unexpected stale app/api/stale/route.ts GET /api/stale receipt evidence";
const UNEXPECTED_SKIP_MESSAGE =
  ".dx/build/route-handler-receipts.json has unexpected stale app/api/old-checkout/route.ts POST /api/old-checkout skipped evidence";

test("route-handler unexpected evidence has one installed-smoke ownership module", () => {
  const manifestSource = readInstalledSmokeSource("route-handler-manifest.ts");
  const graphSource = readInstalledSmokeSource("route-handler-graph.ts");
  const receiptsSource = readInstalledSmokeSource("route-handler-receipts.ts");

  for (const source of [manifestSource, graphSource, receiptsSource]) {
    assert.match(source, /route-handler-unexpected-evidence/);
    assert.match(source, /summarizeUnexpectedRouteHandlerEvidence/);
  }
  assert.doesNotMatch(manifestSource, /function routeHandlerManifestEvidence/);
  assert.doesNotMatch(manifestSource, /function routeHandlerManifestEvidenceKey/);
  assert.doesNotMatch(graphSource, /function graphRouteHandlerEvidence/);
  assert.doesNotMatch(graphSource, /function graphRouteHandlerEvidenceKey/);
});

test("installed smoke route-handler manifest rejects unexpected stale handler evidence", () => {
  const manifest = staleRouteHandlerManifest();
  const routeHandler = summarizeRouteHandlerManifest(manifest, {
    sourcePath: "app/api/health/route.ts",
    route: "/api/health",
    method: "GET",
  });
  const unexpectedRouteHandlers = summarizeUnexpectedRouteHandlerManifests(manifest);

  assert.equal(routeHandler.present, true);
  assert.deepEqual(unexpectedRouteHandlers, [
    {
      sourcePath: "app/api/stale/route.ts",
      route: "/api/stale",
      methods: ["GET"],
      duplicateCount: 0,
      sourceOwnedContract: true,
      declaresNoNodeModules: true,
      nodeModulesRequired: false,
      lifecycleScriptsExecuted: false,
    },
  ]);
  assert.ok(
    routeHandlerManifestFailures({ routeHandler, unexpectedRouteHandlers }).includes(
      UNEXPECTED_MANIFEST_MESSAGE,
    ),
  );
});

test("installed smoke graph rejects unexpected stale route-handler nodes", () => {
  const graphReceipt = summarizeGraphReceipt({
    ok: true,
    value: staleRouteHandlerGraphReceipt(),
  });

  assert.equal(graphReceipt.hasRouteHandlerNode, true);
  assert.deepEqual(graphReceipt.unexpectedRouteHandlerNodes, [
    {
      sourcePath: "app/api/stale/route.ts",
      route: "/api/stale",
      methods: ["GET"],
      duplicateCount: 0,
      sourceOwnedContract: true,
      declaresNoNodeModules: true,
      nodeModulesRequired: false,
      lifecycleScriptsExecuted: false,
    },
  ]);
  assert.ok(graphReceiptFailures(graphReceipt).includes(UNEXPECTED_GRAPH_MESSAGE));
});

test("source-build failures include stale manifest and graph route-handler evidence", () => {
  const report = createPassingReport(routeHandlerReceiptSummary());
  report.build.sourceBuild.manifest.unexpectedRouteHandlers = [
    {
      sourcePath: "app/api/stale/route.ts",
      route: "/api/stale",
      methods: ["GET"],
      duplicateCount: 0,
    },
  ];
  report.build.sourceBuild.graphReceipt.unexpectedRouteHandlerNodes = [
    {
      sourcePath: "app/api/stale/route.ts",
      route: "/api/stale",
      methods: ["GET"],
      duplicateCount: 0,
    },
  ];

  const failures = sourceBuildFailures(report);

  assert.ok(failures.includes(UNEXPECTED_MANIFEST_MESSAGE), JSON.stringify(failures, null, 2));
  assert.ok(failures.includes(UNEXPECTED_GRAPH_MESSAGE), JSON.stringify(failures, null, 2));
});

test("installed smoke route-handler receipts reject unexpected stale receipt and skip evidence", () => {
  const summary = summarizeRouteHandlerReceipts(createStaleReceiptInput());

  assert.deepEqual(summary.unexpectedReceipts, [
    {
      sourcePath: "app/api/stale/route.ts",
      route: "/api/stale",
      method: "GET",
      duplicateCount: 0,
      sourceOwnedContract: true,
      declaresNoNodeModules: true,
      nodeModulesRequired: false,
      nodeModulesPresent: false,
      lifecycleScriptsExecuted: false,
      externalRuntimeRequired: false,
      externalRuntimeExecuted: false,
    },
  ]);
  assert.deepEqual(summary.unexpectedSkips, [
    {
      sourcePath: "app/api/old-checkout/route.ts",
      route: "/api/old-checkout",
      method: "POST",
      duplicateCount: 0,
      reason: "unsafe-build-request-body",
    },
  ]);

  const failures = routeHandlerReceiptFailures(summary);
  assert.ok(failures.includes(UNEXPECTED_RECEIPT_MESSAGE), JSON.stringify(failures, null, 2));
  assert.ok(failures.includes(UNEXPECTED_SKIP_MESSAGE), JSON.stringify(failures, null, 2));
});

test("source-build summary carries compact route-handler stale diagnostics", () => {
  const summary = summarizeSourceBuild(createStaleSourceBuildInput());

  assert.deepEqual(summary.routeHandlerStaleEvidence, {
    present: true,
    count: 4,
    omittedCount: 0,
    entries: [
      {
        artifact: "source-build-manifest",
        sourcePath: "app/api/stale/route.ts",
        route: "/api/stale",
        methods: ["GET"],
        duplicateCount: 0,
        sourceOwnedContract: true,
        declaresNoNodeModules: true,
        nodeModulesRequired: false,
        nodeModulesPresent: null,
        lifecycleScriptsExecuted: false,
        externalRuntimeRequired: null,
        externalRuntimeExecuted: null,
        reason: null,
      },
      {
        artifact: "dx-build-graph",
        sourcePath: "app/api/stale/route.ts",
        route: "/api/stale",
        methods: ["GET"],
        duplicateCount: 0,
        sourceOwnedContract: true,
        declaresNoNodeModules: true,
        nodeModulesRequired: false,
        nodeModulesPresent: null,
        lifecycleScriptsExecuted: false,
        externalRuntimeRequired: null,
        externalRuntimeExecuted: null,
        reason: null,
      },
      {
        artifact: "route-handler-receipts",
        sourcePath: "app/api/stale/route.ts",
        route: "/api/stale",
        methods: ["GET"],
        duplicateCount: 0,
        sourceOwnedContract: true,
        declaresNoNodeModules: true,
        nodeModulesRequired: false,
        nodeModulesPresent: false,
        lifecycleScriptsExecuted: false,
        externalRuntimeRequired: false,
        externalRuntimeExecuted: false,
        reason: null,
      },
      {
        artifact: "route-handler-skipped",
        sourcePath: "app/api/old-checkout/route.ts",
        route: "/api/old-checkout",
        methods: ["POST"],
        duplicateCount: 0,
        sourceOwnedContract: null,
        declaresNoNodeModules: null,
        nodeModulesRequired: null,
        nodeModulesPresent: null,
        lifecycleScriptsExecuted: null,
        externalRuntimeRequired: null,
        externalRuntimeExecuted: null,
        reason: "unsafe-build-request-body",
      },
    ],
  });
});

test("human report prints compact stale route-handler diagnostics", () => {
  const report = createPassingReport(routeHandlerReceiptSummary());
  report.passed = false;
  report.productProofRequired = true;
  report.productProofPassed = false;
  report.proof = {
    scope: "installed-default",
    productEligible: false,
  };
  report.failures = [UNEXPECTED_MANIFEST_MESSAGE];
  report.projectRoot = "G:\\Temp\\fixture";
  report.binaryIdentity = {
    path: "G:\\Dx\\www\\target\\debug\\dx-www.exe",
    present: true,
    kind: "file",
    byteLength: 123456,
    sha256: "abc123",
  };
  report.build.command = {
    command: "G:\\Dx\\www\\target\\debug\\dx-www.exe",
    args: ["build"],
    exitCode: 0,
    stdoutTail: "",
    stderrTail: "",
  };
  report.help = {
    command: {
      exitCode: 0,
    },
  };
  report.build.sourceBuild.routeHandlerStaleEvidence = {
    present: true,
    count: 3,
    omittedCount: 1,
    entries: [
      {
        artifact: "source-build-manifest",
        sourcePath: "app/api/stale/route.ts",
        route: "/api/stale",
        methods: ["GET"],
        duplicateCount: 0,
        sourceOwnedContract: true,
        declaresNoNodeModules: true,
        nodeModulesRequired: false,
        nodeModulesPresent: null,
        lifecycleScriptsExecuted: false,
        externalRuntimeRequired: null,
        externalRuntimeExecuted: null,
        reason: null,
      },
      {
        artifact: "route-handler-skipped",
        sourcePath: "app/api/old-checkout/route.ts",
        route: "/api/old-checkout",
        methods: ["POST"],
        duplicateCount: 0,
        sourceOwnedContract: null,
        declaresNoNodeModules: null,
        nodeModulesRequired: null,
        nodeModulesPresent: null,
        lifecycleScriptsExecuted: null,
        externalRuntimeRequired: null,
        externalRuntimeExecuted: null,
        reason: "unsafe-build-request-body",
      },
    ],
  };

  const output = captureStdout(() => printHumanReport(report));

  assert.match(output, /Route-handler stale evidence: 3 found \(showing 2, omitted 1\)/);
  assert.match(
    output,
    /- source-build-manifest: GET \/api\/stale \(app\/api\/stale\/route\.ts\); source-owned: yes; no node_modules: yes/,
  );
  assert.match(
    output,
    /- route-handler-skipped: POST \/api\/old-checkout \(app\/api\/old-checkout\/route\.ts\); skipped: unsafe-build-request-body/,
  );
});

test("source-build failures include stale route-handler receipt evidence", () => {
  const summary = summarizeRouteHandlerReceipts(createStaleReceiptInput());
  const failures = sourceBuildFailures(createPassingReport(summary));

  assert.ok(failures.includes(UNEXPECTED_RECEIPT_MESSAGE), JSON.stringify(failures, null, 2));
  assert.ok(failures.includes(UNEXPECTED_SKIP_MESSAGE), JSON.stringify(failures, null, 2));
});

function createStaleReceiptInput() {
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
          routeHandlerReceipt("app/api/health/route.ts", "/api/health", "GET", 200),
          routeHandlerReceipt("app/api/stale/route.ts", "/api/stale", "GET", 200),
        ],
        skipped: [
          routeHandlerSkip("app/api/checkout/route.ts", "/api/checkout", "POST"),
          routeHandlerSkip("app/api/old-checkout/route.ts", "/api/old-checkout", "POST"),
        ],
      },
    },
  };
}

function readInstalledSmokeSource(fileName) {
  return fs.readFileSync(
    path.join(__dirname, "..", "tools", "build", "installed-smoke", fileName),
    "utf8",
  );
}

function createStaleSourceBuildInput() {
  return {
    projectRoot: "G:/Dx/www",
    sourceBuildManifestPath: "G:/Dx/www/.dx/build/source-build-manifest.json",
    sourceBuildReceiptPath: "G:/Dx/www/.dx/build/source-build-receipt.json",
    canonicalReceiptPath: "G:/Dx/www/.dx/receipts/build/latest.json",
    routeHandlerReceiptsPath: "G:/Dx/www/.dx/build/route-handler-receipts.json",
    graphReceiptPath: "G:/Dx/www/.dx/receipts/graph/latest.json",
    graphConsumerSnapshotPath: "G:/Dx/www/.dx/receipts/graph/consumer-snapshot.json",
    sourceBuildManifest: {
      ok: true,
      value: staleRouteHandlerManifest(),
    },
    sourceBuildReceipt: {
      ok: true,
      value: {
        schema: "dx.www.sourceBuildReceipt",
        node_modules_required: false,
        summary: {
          route_handlers: 2,
        },
      },
    },
    canonicalReceipt: {
      ok: true,
      value: {
        schema: "dx.www.sourceBuildReceipt",
      },
    },
    graphReceipt: {
      ok: true,
      value: staleRouteHandlerGraphReceipt(),
    },
    graphConsumerSnapshot: {
      ok: false,
      value: null,
    },
    routeHandlerReceipts: createStaleReceiptInput().routeHandlerReceipts,
  };
}

function staleRouteHandlerManifest() {
  return {
    route_handlers: [
      manifestRouteHandler("app/api/health/route.ts", "/api/health", ["GET"]),
      manifestRouteHandler("app/api/checkout/route.ts", "/api/checkout", ["POST"]),
      manifestRouteHandler("app/api/stale/route.ts", "/api/stale", ["GET"]),
    ],
  };
}

function staleRouteHandlerGraphReceipt() {
  return {
    schema: "dx.build.graph",
    graph: {
      nodes: [
        graphRouteHandlerNode("app/api/health/route.ts", "/api/health", ["GET"]),
        graphRouteHandlerNode("app/api/checkout/route.ts", "/api/checkout", ["POST"]),
        graphRouteHandlerNode("app/api/stale/route.ts", "/api/stale", ["GET"]),
      ],
      edges: [],
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

function routeHandlerReceipt(sourcePath, requestPath, method, status) {
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
      status,
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
    outputProofSummary: { missingChecks: [] },
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
        graphReceipt: {
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
        },
        graphConsumerSnapshot: {
          present: true,
          schema: "dx.build.graph.consumerSnapshot",
          sourceModuleCount: 2,
          coversSourceModuleKind: true,
          coversCompiledFromSourceEdge: true,
          zedPreviewSourceModuleKind: "source-module",
        },
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

function routeHandlerReceiptSummary() {
  return summarizeRouteHandlerReceipts({
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
        receipts: [routeHandlerReceipt("app/api/health/route.ts", "/api/health", "GET", 200)],
        skipped: [routeHandlerSkip("app/api/checkout/route.ts", "/api/checkout", "POST")],
      },
    },
  });
}

function captureStdout(callback) {
  let output = "";
  const originalWrite = process.stdout.write;
  process.stdout.write = (chunk, encoding, callbackArg) => {
    output += String(chunk);
    if (typeof callbackArg === "function") {
      callbackArg();
    } else if (typeof encoding === "function") {
      encoding();
    }
    return true;
  };

  try {
    callback();
  } finally {
    process.stdout.write = originalWrite;
  }

  return output;
}
