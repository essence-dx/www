import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const {
  serverArtifactFailures,
  summarizeDeployAdapter,
  summarizeServerContracts,
} = require("../tools/build/installed-smoke/server-artifacts.ts");
const { routeHandlerReceiptFailures } = require("../tools/build/installed-smoke/route-handler-receipts.ts");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("route-handler receipts use DX-owned runtime-boundary vocabulary", () => {
  const activeFiles = [
    "dx-www/src/cli/app_route_handler_receipt.rs",
    "dx-www/src/cli/app_route_handler_build_output.rs",
    "dx-www/src/build/source_engine/route_handler_receipts.rs",
    "dx-www/src/cli/mod.rs",
    "dx-www/src/cli/mod_parts/cli_core_impl.rs",
    "dx-www/tests/dx_build_tiny_app.rs",
    "dx-www/tests/source_build_engine.rs",
    "tools/build/installed-smoke/server-artifacts.ts",
    "benchmarks/dx-build-installed-smoke.test.ts",
    "benchmarks/nextjs-compatibility-map.test.ts",
    "docs/NEXTJS_COMPATIBILITY_MAP.md",
    "DX.md",
    "CHANGELOG.md",
  ];

  for (const relativePath of activeFiles) {
    const source = read(relativePath);
    assert.doesNotMatch(
      source,
      /full_nextjs_route_handler_parity|x-dx-full-nextjs-route-handler-parity|claimsFullNextParity|full Route Handler runtime parity/,
      `${relativePath} must not keep removed route-handler parity vocabulary`,
    );
  }

  const cliReceipt = read("dx-www/src/cli/app_route_handler_receipt.rs");
  assert.match(cliReceipt, /"runtime_boundary":\s*\{/);
  assert.match(cliReceipt, /"source_owned":\s*true/);
  assert.match(cliReceipt, /"external_runtime_required":\s*false/);
  assert.match(cliReceipt, /"external_runtime_executed":\s*false/);
  assert.match(cliReceipt, /x-dx-route-handler-source-owned/);
  assert.match(cliReceipt, /x-dx-external-runtime-required/);
  assert.match(cliReceipt, /x-dx-external-runtime-executed/);

  const cliBuildOutput = read("dx-www/src/cli/app_route_handler_build_output.rs");
  assert.match(cliBuildOutput, /"runtime_boundary":\s*\{/);
  assert.match(cliBuildOutput, /"source_owned":\s*true/);

  const sourceBuildReceipt = read("dx-www/src/build/source_engine/route_handler_receipts.rs");
  assert.match(sourceBuildReceipt, /"runtime_boundary":\s*\{/);
  assert.match(sourceBuildReceipt, /"source_owned":\s*true/);

  const boundaryResponse = read("dx-www/src/cli/mod_parts/cli_core_impl.rs");
  assert.match(boundaryResponse, /x-dx-route-handler-source-owned/);
  assert.match(boundaryResponse, /x-dx-external-runtime-required/);
  assert.match(boundaryResponse, /x-dx-external-runtime-executed/);
});

test("installed smoke reports route-handler deploy runtime-boundary failures directly", () => {
  const goodFailures = serverArtifactFailures(deployReport(checkoutRouteHandler()));
  assert.deepEqual(goodFailures, []);

  const missingBoundaryFailures = serverArtifactFailures(
    deployReport(checkoutRouteHandler({ runtime_boundary: undefined })),
  );
  assert.ok(
    missingBoundaryFailures.includes(
      "deploy adapter /api/checkout route handler does not declare source-owned runtime boundary",
    ),
    JSON.stringify(missingBoundaryFailures, null, 2),
  );
  assert.equal(
    missingBoundaryFailures.includes("deploy adapter is missing /api/checkout route handler"),
    false,
  );

  const externalRuntimeFailures = serverArtifactFailures(
    deployReport(
      checkoutRouteHandler({
        runtime_boundary: {
          source_owned: true,
          external_runtime_required: true,
          external_runtime_executed: true,
        },
      }),
    ),
  );
  assert.ok(
    externalRuntimeFailures.includes("deploy adapter /api/checkout route handler requires an external runtime"),
    JSON.stringify(externalRuntimeFailures, null, 2),
  );
  assert.ok(
    externalRuntimeFailures.includes("deploy adapter /api/checkout route handler executed an external runtime"),
    JSON.stringify(externalRuntimeFailures, null, 2),
  );
});

test("installed smoke route-handler receipt runtime-boundary failures name the route source", () => {
  const failures = routeHandlerReceiptFailures({
    present: true,
    collectionSchema: "dx.next.appRouteHandlerBuildReceipts",
    collectionFormat: 1,
    collectionDeclaresNoNodeModules: true,
    collectionNodeModulesPresent: false,
    collectionLifecycleScriptsExecuted: false,
    requiredReceipts: [
      {
        key: "checkoutPost",
        sourcePath: "app/api/checkout/route.ts",
        route: "/api/checkout",
        method: "POST",
        expectedStatus: 202,
        present: true,
        schema: "dx.next.appRouteHandlerReceipt",
        format: 1,
        executionModel: "source-owned-route-handler-contract",
        declaresNoNodeModules: true,
        nodeModulesRequired: false,
        nodeModulesPresent: false,
        lifecycleScriptsExecuted: false,
        sourceOwnedRuntimeBoundary: false,
        externalRuntimeRequired: true,
        externalRuntimeExecuted: true,
        responseStatus: 202,
        responseContentType: "application/json; charset=utf-8",
        responseHeaderCount: 1,
        hasAdapterBoundary: true,
      },
    ],
    requiredSkips: [],
  });

  assert.ok(
    failures.includes(
      ".dx/build/route-handler-receipts.json app/api/checkout/route.ts POST does not declare source-owned runtime boundary",
    ),
    JSON.stringify(failures, null, 2),
  );
  assert.ok(
    failures.includes(
      ".dx/build/route-handler-receipts.json app/api/checkout/route.ts POST requires an external runtime",
    ),
    JSON.stringify(failures, null, 2),
  );
  assert.ok(
    failures.includes(
      ".dx/build/route-handler-receipts.json app/api/checkout/route.ts POST executed an external runtime",
    ),
    JSON.stringify(failures, null, 2),
  );
  assert.equal(
    failures.includes("route-handler receipt does not declare source-owned runtime boundary"),
    false,
  );
});

function deployReport(checkoutHandler) {
  const input = {
    serverContracts: {
      ok: true,
      value: [
        {
          source_path: "app/api/health/route.ts",
          endpoint: "/api/health",
          lifecycle_scripts_executed: false,
        },
        {
          source_path: "app/api/checkout/route.ts",
          endpoint: "/api/checkout",
          lifecycle_scripts_executed: false,
        },
        {
          source_path: "server/loaders.ts",
          lifecycle_scripts_executed: false,
        },
      ],
    },
    deployAdapter: {
      ok: true,
      value: {
        no_node_modules_required: true,
        routes: [{ path: "/", server_data: "app/server-data.json" }],
        health_checks: [
          {
            path: "/api/health",
            source_path: "app/api/health/route.ts",
            method: "GET",
          },
        ],
        route_handlers: [checkoutHandler],
      },
    },
  };

  return {
    build: {
      serverContracts: summarizeServerContracts(input),
      deployAdapter: summarizeDeployAdapter(input),
    },
  };
}

function checkoutRouteHandler(overrides = {}) {
  return {
    path: "/api/checkout",
    source_path: "app/api/checkout/route.ts",
    methods: ["POST"],
    skipped_build_methods: ["POST"],
    build_execution: "skipped-build-execution",
    node_modules_required: false,
    runtime_boundary: {
      source_owned: true,
      external_runtime_required: false,
      external_runtime_executed: false,
    },
    ...overrides,
  };
}
