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

test("route handler safe interpreter reads destructured URL searchParams", () => {
  const serverContract = read("core/src/delivery/server_contract.rs");
  const deliveryTests = read("core/src/delivery/tests.rs");
  const searchParamHelpers = serverContract.slice(
    serverContract.indexOf("fn route_handler_url_roots"),
    serverContract.indexOf("fn route_handler_header_roots"),
  );

  assert.match(serverContract, /route_handler_destructured_search_param_roots/);
  assert.match(searchParamHelpers, /searchParams/);
  assert.match(searchParamHelpers, /new URL\(request\.url\)/);
  assert.match(searchParamHelpers, /request\.nextUrl/);
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_reads_destructured_next_request_search_params_without_next_runtime/,
  );
  assert.match(
    deliveryTests,
    /const \{ searchParams: query \} = request\.nextUrl;/,
  );
  assert.match(
    deliveryTests,
    /const \{ searchParams \} = new URL\(request\.url\);/,
  );
  assert.match(deliveryTests, /source-owned-safe-interpreter/);
  assert.doesNotMatch(searchParamHelpers, /node_modules|NextRuntime|TurbopackRuntime/);
});
