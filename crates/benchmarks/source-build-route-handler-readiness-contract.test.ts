import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("source-build readiness graph exposes route-handler receipt execution counts", () => {
  const readiness = read("dx-www/src/build/source_engine/readiness.rs");
  const receipt = read("dx-www/src/build/source_engine/receipt.rs");

  assert.match(
    readiness,
    /"route_handler_receipt_output": manifest\.route_handler_receipts\.output/,
  );
  assert.match(
    readiness,
    /"route_handler_receipts_executed": manifest\.route_handler_receipts\.receipt_count/,
  );
  assert.match(
    readiness,
    /"route_handler_receipts_skipped": manifest\.route_handler_receipts\.skipped_count/,
  );
  assert.match(
    readiness,
    /"route_handler_receipts_node_modules_required": manifest\.route_handler_receipts\.node_modules_required/,
  );
  assert.match(
    readiness,
    /"route_handler_receipts_lifecycle_scripts_executed": manifest\.route_handler_receipts\.lifecycle_scripts_executed/,
  );
  assert.match(receipt, /pub route_handler_receipt_output: String/);
  assert.match(receipt, /pub route_handler_receipts_node_modules_required: bool/);
  assert.match(receipt, /pub route_handler_receipts_lifecycle_scripts_executed: bool/);
  assert.match(receipt, /route_handler_receipt_output: manifest\.route_handler_receipts\.output\.clone\(\)/);
  assert.match(
    receipt,
    /route_handler_receipts_node_modules_required:\s*manifest\s*\.\s*route_handler_receipts\s*\.\s*node_modules_required/,
  );
  assert.match(
    receipt,
    /route_handler_receipts_lifecycle_scripts_executed:\s*manifest\s*\.\s*route_handler_receipts\s*\.\s*lifecycle_scripts_executed/,
  );
});
