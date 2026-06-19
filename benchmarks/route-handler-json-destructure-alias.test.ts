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

test("route handler safe interpreter reads destructured request.json fields", () => {
  const serverContract = read("core/src/delivery/server_contract.rs");
  const deliveryTests = read("core/src/delivery/tests.rs");
  const aliasFunction = serverContract.slice(
    serverContract.indexOf("fn route_handler_json_body_field_aliases"),
    serverContract.indexOf("fn route_handler_destructured_param_alias"),
  );

  assert.match(serverContract, /route_handler_json_body_field_aliases/);
  assert.match(serverContract, /body_field_aliases/);
  assert.match(serverContract, /RouteHandlerBodyFieldAlias/);
  assert.match(serverContract, /field_name: String/);
  assert.match(serverContract, /default: Value/);
  assert.match(serverContract, /json_path_value_or_missing\(&request\.body, &alias\.field_name\)/);
  assert.match(serverContract, /unwrap_or_else\(\|\| alias\.default\.clone\(\)\)/);
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_serializes_destructured_request_json_fields/,
  );
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_serializes_destructured_request_json_field_defaults/,
  );
  assert.match(deliveryTests, /const \{ email, count: total \} = await request\.json\(\);/);
  assert.match(
    deliveryTests,
    /const \{ email = "unknown", count: total = 0 \} = await request\.json\(\);/,
  );
  assert.match(deliveryTests, /source-owned-safe-interpreter/);
  assert.doesNotMatch(aliasFunction, /node_modules|NextRuntime|TurbopackRuntime/);
});
