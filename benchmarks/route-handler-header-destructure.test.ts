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

test("route handler safe interpreter reads destructured request headers", () => {
  const serverContract = read("core/src/delivery/server_contract.rs");
  const deliveryTests = read("core/src/delivery/tests.rs");
  const headerHelpers = serverContract.slice(
    serverContract.indexOf("fn route_handler_header_roots"),
    serverContract.indexOf("fn route_handler_form_data_read"),
  );

  assert.match(serverContract, /route_handler_destructured_header_roots/);
  assert.match(headerHelpers, /request\.headers/);
  assert.match(headerHelpers, /headers\(\)\.get/);
  assert.match(headerHelpers, /headers\(\)/);
  assert.match(headerHelpers, /format!\("\{root\}\.get"\)/);
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_reads_destructured_request_headers_without_next_runtime/,
  );
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_reads_next_headers_helper_without_next_runtime/,
  );
  assert.match(deliveryTests, /const \{ headers \} = request;/);
  assert.match(deliveryTests, /import \{ headers \} from "next\/headers";/);
  assert.match(deliveryTests, /headers\.get\("x-dx-preview"\) \?\? "0"/);
  assert.match(deliveryTests, /source-owned-safe-interpreter/);
  assert.doesNotMatch(headerHelpers, /node_modules|NextRuntime|TurbopackRuntime/);
});
