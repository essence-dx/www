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

test("route handler safe interpreter accepts typed request.json body aliases", () => {
  const serverContract = read("core/src/delivery/server_contract.rs");
  const deliveryTests = read("core/src/delivery/tests.rs");
  const aliasFunction = serverContract.slice(
    serverContract.indexOf("fn route_handler_json_body_aliases"),
    serverContract.indexOf("#[derive(Debug, Clone, PartialEq)]\nstruct RouteHandlerCookieAlias"),
  );

  assert.match(aliasFunction, /fn route_handler_json_body_aliases/);
  assert.match(aliasFunction, /\(\?::\[\^=;\\n\]\+\)\?/);
  assert.ok(aliasFunction.includes(".json\\(\\)"));
  assert.ok(aliasFunction.includes("\\s+as\\s+[^;\\n]+"));
  assert.match(serverContract, /fn route_handler_path_typescript_suffix/);
  assert.match(serverContract, /fn route_handler_body_alias_matches/);
  assert.match(serverContract, /fn route_handler_body_root_expression_pattern/);
  assert.match(serverContract, /fn route_handler_body_alias_type_postfix_pattern/);
  assert.match(serverContract, /route_handler_path_typescript_suffix\(rest\)/);
  assert.match(serverContract, /route_handler_body_alias_matches\(read, alias\)/);
  assert.match(aliasFunction, /route_handler_body_root_expression_pattern\(body_roots\)/);
  assert.match(aliasFunction, /route_handler_body_alias_type_postfix_pattern\(\)/);
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_serializes_typed_request_json_body_aliases/,
  );
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_serializes_typed_request_json_body_alias_postfixes/,
  );
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_serializes_typed_request_json_body_alias_root_postfixes/,
  );
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_serializes_typed_request_json_cast_root_aliases/,
  );
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_serializes_typed_request_json_satisfies_aliases/,
  );
  assert.match(deliveryTests, /const body: Payload = await request\.json\(\);/);
  assert.match(deliveryTests, /\(await request\.json\(\)\) as Payload/);
  assert.match(deliveryTests, /const body = await request\.json\(\) as Payload;/);
  assert.match(deliveryTests, /email: body\.email!,/);
  assert.match(deliveryTests, /count: body\.count as number,/);
  assert.match(deliveryTests, /label: body\.profile!\.label satisfies string,/);
  assert.match(deliveryTests, /return Response\.json\(body as Payload\);/);
  assert.match(deliveryTests, /return Response\.json\(body!\);/);
  assert.match(deliveryTests, /const body = await \(request as Request\)\.json\(\) as Payload;/);
  assert.match(deliveryTests, /const body = await request\.json\(\) satisfies Payload;/);
  assert.match(deliveryTests, /const \{ email \} = await request\.json\(\) satisfies Payload;/);
  assert.match(deliveryTests, /source-owned-safe-interpreter/);
  assert.doesNotMatch(aliasFunction, /node_modules|NextRuntime|TurbopackRuntime/);
});
