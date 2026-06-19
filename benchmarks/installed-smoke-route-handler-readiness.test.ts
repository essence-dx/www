import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const path = require("node:path");
const test = require("node:test");

const { INSTALLED_BINARY_SMOKE_RECEIPT } = require("../tools/build/installed-smoke/constants.ts");
const { summarizeReadiness } = require("../tools/build/installed-smoke/readiness.ts");

test("installed smoke readiness normalizes route-handler receipt output paths", () => {
  const projectRoot = "G:/Dx/www";
  const routeHandlerReceiptsPath = path.join(
    projectRoot,
    ".dx",
    "build",
    "route-handler-receipts.json",
  );

  const summary = summarizeReadiness({
    projectRoot,
    routeHandlerReceiptsPath,
    readiness: {
      ok: true,
      value: {
        schema: "dx.build.readiness",
        source_ready: true,
        source_score: 100,
        product_ready: false,
        product_score: 85,
        installed_binary_smoke: {
          receipt: INSTALLED_BINARY_SMOKE_RECEIPT,
        },
        receipts: {
          installed_binary_smoke: INSTALLED_BINARY_SMOKE_RECEIPT,
        },
        graph: {
          route_handler_receipt_output: ".\\.dx\\build\\route-handler-receipts.json",
          route_handler_receipts_executed: 1,
          route_handler_receipts_skipped: 1,
          route_handler_receipts_node_modules_required: false,
          route_handler_receipts_lifecycle_scripts_executed: false,
        },
      },
    },
  });

  assert.equal(summary.routeHandlerReceiptOutput, ".\\.dx\\build\\route-handler-receipts.json");
  assert.equal(summary.routeHandlerReceiptOutputMatchesActual, true);
  assert.equal(summary.routeHandlerReceiptsExecuted, 1);
  assert.equal(summary.routeHandlerReceiptsSkipped, 1);
  assert.equal(summary.routeHandlerReceiptsDeclareNoNodeModules, true);
  assert.equal(summary.routeHandlerReceiptsDeclareNoLifecycleScripts, true);
});
