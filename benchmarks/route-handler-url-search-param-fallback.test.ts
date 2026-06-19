const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("route handler safe interpreter derives URL search params from request.url", () => {
  const serverContract = read("core/src/delivery/server_contract.rs");
  const deliveryTests = read("core/src/delivery/tests.rs");
  const searchParamHelpers = serverContract.slice(
    serverContract.indexOf("fn route_request_search_params"),
    serverContract.indexOf("fn route_request_absolute_url_path"),
  );

  assert.match(serverContract, /fn route_request_search_params/);
  assert.match(serverContract, /fn route_request_query_params/);
  assert.match(serverContract, /route_request_decode_query_component/);
  assert.match(searchParamHelpers, /route_request_absolute_url_path\(actual\)/);
  assert.match(searchParamHelpers, /split_once\('#'\)/);
  assert.match(searchParamHelpers, /params\.extend\(request\.search_params\.clone\(\)\)/);
  assert.match(
    serverContract,
    /"request\.searchParams" => \{\s*return Ok\(serde_json::json!\(route_request_search_params\(request\)\)\);/s,
  );
  assert.match(
    serverContract,
    /route_handler_context_map_value\(\s*expression,\s*&context_bindings\.search_param_roots,\s*&effective_search_params,/s,
  );
  assert.match(
    serverContract,
    /route_request_search_params\(request\)\s*\.get\(param_name\)/,
  );
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_reads_url_search_params_from_request_path_without_matcher_params/,
  );
  assert.match(deliveryTests, /search_params: BTreeMap::new\(\)/);
  assert.match(deliveryTests, /encoded=a%2Fb&plus=hello\+world#top/);
  assert.match(deliveryTests, /assert_eq!\(response\.body\["encoded"\], "a\/b"\)/);
  assert.match(deliveryTests, /assert_eq!\(response\.body\["plus"\], "hello world"\)/);
  assert.doesNotMatch(searchParamHelpers, /node_modules|NextRuntime|TurbopackRuntime/);
});
