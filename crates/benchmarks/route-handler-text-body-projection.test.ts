const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("route handler request.text projects structured bodies as text", () => {
  const serverContract = read("core/src/delivery/server_contract.rs");
  const deliveryTests = read("core/src/delivery/tests.rs");
  const textAliasHelpers = serverContract.slice(
    serverContract.indexOf("fn route_handler_text_body_alias_value"),
    serverContract.indexOf("fn route_handler_body_alias_value"),
  );
  const textReadHelpers = serverContract.slice(
    serverContract.indexOf("fn route_handler_body_read_value"),
    serverContract.indexOf("/// Safe route-handler context roots"),
  );

  assert.match(serverContract, /fn route_handler_text_body_value/);
  assert.match(serverContract, /fn route_handler_body_projection_value/);
  assert.match(serverContract, /fn route_handler_text_body_aliases/);
  assert.match(serverContract, /fn route_handler_text_body_alias_value/);
  assert.match(
    serverContract,
    /route_handler_body_projection_value\(\s*request,\s*method,\s*&path,\s*default\.clone\(\),\s*\)/s,
  );
  assert.match(
    serverContract,
    /route_handler_body_read_value\(\s*request,\s*method,\s*default\.clone\(\),?\s*\)/s,
  );
  assert.match(serverContract, /"text" => route_handler_text_body_value/);
  assert.match(serverContract, /serde_json::to_string\(&request\.body\)/);
  assert.match(
    serverContract,
    /route_handler_text_body_alias_value\(\s*expression,\s*request,\s*&context_bindings\.text_body_aliases,\s*\)/s,
  );
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_projects_structured_request_text_body_as_raw_text/,
  );
  assert.match(deliveryTests, /const raw = await request\.text\(\);/);
  assert.match(deliveryTests, /const cloneRaw = await cloned\.text\(\);/);
  assert.match(deliveryTests, /directCloneRaw: await request\.clone\(\)\.text\(\)/);
  assert.match(deliveryTests, /textProperty: \(await request\.text\(\)\)\.event \?\? "missing"/);
  assert.match(deliveryTests, /assert_eq!\(response\.body\["textProperty"\], "missing"\)/);
  assert.match(deliveryTests, /serde_json::from_str\(raw\)/);
  assert.doesNotMatch(
    `${textAliasHelpers}\n${textReadHelpers}`,
    /node_modules|NextRuntime|TurbopackRuntime/,
  );
});
