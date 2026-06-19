const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("route handler unsupported binary body readers fail with DX boundary diagnostics", () => {
  const serverContract = read("core/src/delivery/server_contract.rs");
  const bodyBoundary = read("core/src/delivery/route_handler_body_boundary.rs");
  const deliveryMod = read("core/src/delivery/mod.rs");
  const deliveryTests = read("core/src/delivery/tests.rs");
  const cliMod = read("dx-www/src/cli/mod.rs");

  assert.match(serverContract, /unsupported_body_aliases: BTreeMap<String, RouteHandlerUnsupportedBodyRead>/);
  assert.match(serverContract, /use super::route_handler_body_boundary::\{/);
  assert.match(deliveryMod, /mod route_handler_body_boundary;/);
  assert.match(bodyBoundary, /struct RouteHandlerUnsupportedBodyRead/);
  assert.match(bodyBoundary, /fn route_handler_unsupported_request_body_read/);
  assert.match(bodyBoundary, /fn route_handler_unsupported_body_aliases/);
  assert.match(bodyBoundary, /fn route_handler_unsupported_body_alias_read/);
  assert.match(bodyBoundary, /fn route_handler_unsupported_body_read_message/);
  assert.match(bodyBoundary, /&\["arrayBuffer", "blob", "bytes"\]/);
  assert.match(
    bodyBoundary,
    /source-owned-safe-interpreter supports request\.json\(\), request\.text\(\), request\.formData\(\), and request\.body/,
  );
  assert.match(bodyBoundary, /node_modules_required=false/);
  assert.match(bodyBoundary, /external_runtime_executed=false/);
  assert.match(
    serverContract,
    /route_handler_unsupported_body_alias_read\(\s*expression,\s*&context_bindings\.unsupported_body_aliases,\s*\)/s,
  );
  assert.match(
    serverContract,
    /route_handler_unsupported_request_body_read\(\s*expression,\s*&context_bindings\.body_roots\s*\)/s,
  );
  assert.match(
    cliMod,
    /error\.contains\("unsupported route handler request body reader"\)/,
    "live dx dev must classify unsupported body readers as route-handler boundary responses",
  );

  assert.match(
    deliveryTests,
    /react_route_handler_runtime_reports_unsupported_binary_body_readers_without_next_runtime/,
  );
  assert.match(deliveryTests, /const bytes = await request\.arrayBuffer\(\);/);
  assert.match(deliveryTests, /bytes\.byteLength \?\? 0/);
  assert.match(deliveryTests, /\(await cloned\.blob\(\)\)\.size \?\? 0/);
  assert.match(deliveryTests, /expect_err\("arrayBuffer body reader should stay behind an explicit boundary"\)/);
  assert.match(deliveryTests, /expect_err\("blob body reader should stay behind an explicit boundary"\)/);
});
