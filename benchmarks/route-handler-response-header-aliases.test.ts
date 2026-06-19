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

test("route handler safe interpreter normalizes response header aliases", () => {
  const serverContract = read("core/src/delivery/server_contract.rs");
  const deliveryTests = read("core/src/delivery/tests.rs");
  const optionsStart = serverContract.indexOf("fn parse_route_handler_response_options");
  assert.notEqual(optionsStart, -1);
  const responseHelpers = serverContract.slice(
    optionsStart,
    serverContract.indexOf("fn parse_redirect_target"),
  );

  assert.match(serverContract, /route_handler_response_header_aliases/);
  assert.match(serverContract, /route_handler_response_header_mutations/);
  assert.match(responseHelpers, /parse_safe_response_header_entries\(\s*value\.trim\(\),\s*header_aliases,\s*\)/s);
  assert.match(responseHelpers, /safe_response_header_name/);
  assert.match(responseHelpers, /"set-cookie"/);
  assert.match(deliveryTests, /react_route_handler_runtime_serializes_response_header_aliases_without_next_runtime/);
  assert.match(deliveryTests, /const responseHeaders = new Headers\(\);/);
  assert.match(deliveryTests, /responseHeaders\.set\("x-dx-response", "alias"\);/);
  assert.match(deliveryTests, /responseHeaders\.append\("set-cookie", "theme=dark; Path=\/; HttpOnly"\);/);
  assert.match(deliveryTests, /headers: responseHeaders/);
  assert.match(deliveryTests, /return new Response\("accepted", \{/);
  assert.match(deliveryTests, /response\.headers\.get\("set-cookie"\)\.map\(String::as_str\)/);
  assert.doesNotMatch(responseHelpers, /node_modules|NextRuntime|TurbopackRuntime/);
});
