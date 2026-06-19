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

test("route handler safe interpreter reads request clone bodies directly", () => {
  const serverContract = read("core/src/delivery/server_contract.rs");
  const deliveryTests = read("core/src/delivery/tests.rs");
  const bodyStart = serverContract.indexOf("fn route_handler_body_aliases");
  assert.notEqual(bodyStart, -1);
  const bodyHelpers = serverContract.slice(
    bodyStart,
    serverContract.indexOf("#[derive(Debug, Clone, PartialEq)]\nstruct RouteHandlerCookieAlias"),
  );

  assert.match(serverContract, /body_roots: Vec<String>/);
  assert.match(serverContract, /route_handler_request_body_roots/);
  assert.match(serverContract, /route_handler_request_clone_aliases/);
  assert.match(serverContract, /route_handler_request_body_read/);
  assert.match(serverContract, /route_handler_request_body_projection_path/);
  assert.match(serverContract, /route_handler_request_body_projection_suffix_path/);
  assert.match(serverContract, /fn route_handler_json_path_suffix/);
  assert.match(serverContract, /route_handler_request_body_method_read/);
  assert.match(serverContract, /body_aliases: Vec<String>/);
  assert.match(serverContract, /route_handler_effective_body_aliases\(function_body, context_bindings\)/);
  assert.match(serverContract, /context_bindings\.body_aliases\.clone\(\)/);
  assert.match(serverContract, /fn route_handler_body_aliases\(\s*function_body: &str,\s*body_roots: &\[String\]/);
  assert.match(serverContract, /route_handler_json_body_field_aliases\(function_body, &bindings\.body_roots\)/);
  assert.match(serverContract, /regex::escape\(root\)/);
  assert.match(serverContract, /route_handler_body_read_value/);
  assert.match(serverContract, /fn route_handler_body_alias_path/);
  assert.match(serverContract, /"request\.clone\(\)"/);
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_reads_request_clone_body_without_next_runtime/,
  );
  assert.match(deliveryTests, /const cloned = request\.clone\(\);/);
  assert.match(deliveryTests, /const directBody = await request\.clone\(\)\.json\(\);/);
  assert.match(deliveryTests, /const clonedBody = await cloned\.json\(\);/);
  assert.match(
    deliveryTests,
    /const \{ email: destructuredEmail, count: destructuredCount, missing: destructuredMissing = "fallback" \} = await request\.clone\(\)\.json\(\);/,
  );
  assert.match(deliveryTests, /const \{ email: clonedEmail \} = await cloned\.json\(\);/);
  assert.match(deliveryTests, /payload: await cloned\.json\(\)/);
  assert.match(deliveryTests, /directPayload: await request\.clone\(\)\.json\(\)/);
  assert.match(deliveryTests, /directAliasEmail: directBody\.email,/);
  assert.match(deliveryTests, /directAliasMissing: directBody\.missing \?\? "fallback",/);
  assert.match(deliveryTests, /clonedAliasCount: clonedBody\.count,/);
  assert.match(deliveryTests, /directAliasBracketEmail: directBody\["email"\],/);
  assert.match(deliveryTests, /directAliasBracketMissing: directBody\["missing"\] \?\? "fallback",/);
  assert.match(deliveryTests, /clonedAliasBracketCount: clonedBody\["count"\],/);
  assert.match(deliveryTests, /directAliasNestedEmail: directBody\["profile"\]\["email"\],/);
  assert.match(deliveryTests, /directAliasNestedMissing: directBody\["profile"\]\["missing"\] \?\? "fallback",/);
  assert.match(deliveryTests, /clonedAliasNestedEmail: clonedBody\["profile"\]\?\.\["email"\],/);
  assert.match(deliveryTests, /destructuredEmail,/);
  assert.match(deliveryTests, /destructuredCount,/);
  assert.match(deliveryTests, /destructuredMissing,/);
  assert.match(deliveryTests, /clonedEmail,/);
  assert.match(deliveryTests, /email: \(await cloned\.json\(\)\)\.email \?\? "unknown"/);
  assert.match(deliveryTests, /directEmail: \(await request\.clone\(\)\.json\(\)\)\.email/);
  assert.match(deliveryTests, /missing: \(await cloned\.json\(\)\)\.missing \?\? "fallback"/);
  assert.match(deliveryTests, /bracketEmail: \(await cloned\.json\(\)\)\["email"\] \?\? "unknown"/);
  assert.match(deliveryTests, /directBracketEmail: \(await request\.clone\(\)\.json\(\)\)\["email"\]/);
  assert.match(deliveryTests, /bracketMissing: \(await cloned\.json\(\)\)\["missing"\] \?\? "fallback"/);
  assert.match(deliveryTests, /optionalBracketCount: \(await cloned\.json\(\)\)\?\.\["count"\]/);
  assert.match(deliveryTests, /nestedBracketEmail: \(await cloned\.json\(\)\)\["profile"\]\["email"\] \?\? "unknown"/);
  assert.match(deliveryTests, /directNestedBracketEmail: \(await request\.clone\(\)\.json\(\)\)\["profile"\]\?\.\["email"\]/);
  assert.match(deliveryTests, /nestedBracketMissing: \(await cloned\.json\(\)\)\["profile"\]\["missing"\] \?\? "fallback"/);
  assert.match(deliveryTests, /raw: await request\.clone\(\)\.text\(\)/);
  assert.match(deliveryTests, /source-owned-safe-interpreter/);
  assert.doesNotMatch(bodyHelpers, /node_modules[\\/]|require\(|NextRuntime|TurbopackRuntime/);
});
