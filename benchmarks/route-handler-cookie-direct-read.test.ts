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

test("route handler safe interpreter reads direct cookie values", () => {
  const serverContract = read("core/src/delivery/server_contract.rs");
  const deliveryTests = read("core/src/delivery/tests.rs");
  const cookieStart = serverContract.indexOf("fn route_handler_cookie_roots");
  assert.notEqual(cookieStart, -1);
  const cookieHelpers = serverContract.slice(
    cookieStart,
    serverContract.indexOf("fn route_handler_response_number_aliases"),
  );

  assert.match(serverContract, /RouteHandlerCookieProperty/);
  assert.match(serverContract, /route_handler_destructured_cookie_roots/);
  assert.match(serverContract, /route_handler_cookie_read/);
  assert.match(serverContract, /cookie_read_value/);
  assert.match(cookieHelpers, /format!\("\{root\}\.get"\)/);
  assert.match(cookieHelpers, /route_handler_cookie_get/);
  assert.match(cookieHelpers, /request\.cookies/);
  assert.match(cookieHelpers, /cookies\(\)/);
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_reads_direct_cookie_values_without_next_runtime/,
  );
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_reads_cookie_root_aliases_and_metadata_without_next_runtime/,
  );
  assert.match(deliveryTests, /request\.cookies\.get\("theme"\)\?\.value \?\? "light"/);
  assert.match(deliveryTests, /cookies\(\)\.get\("locale"\)\?\.value \?\? "en"/);
  assert.match(deliveryTests, /const \{ cookies: requestCookies \} = request;/);
  assert.match(deliveryTests, /requestCookies\.get\("theme"\)\?\.name \?\? "missing"/);
  assert.match(deliveryTests, /source-owned-safe-interpreter/);
  assert.doesNotMatch(cookieHelpers, /node_modules|NextRuntime|TurbopackRuntime/);
});
