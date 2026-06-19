const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("route handler safe interpreter reads typed request aliases", () => {
  const serverContract = read("core/src/delivery/server_contract.rs");
  const deliveryTests = read("core/src/delivery/tests.rs");
  const urlHelpers = serverContract.slice(
    serverContract.indexOf("fn route_handler_url_roots"),
    serverContract.indexOf("fn route_handler_header_roots"),
  );
  const requestHelpers = serverContract.slice(
    serverContract.indexOf("fn route_handler_header_roots"),
    serverContract.indexOf("fn route_handler_context_param_aliases"),
  );

  assert.match(urlHelpers, /const\|let\)\\s\+\(\[A-Za-z_\$]\[A-Za-z0-9_\$]\*\)\\s\*\(\?:\:\[\^=;\\n]\+\)\?\\s\*=\\s\*/);
  assert.match(requestHelpers, /request\\\.headers\\b/);
  assert.match(requestHelpers, /request\\\.method\\b/);
  assert.match(requestHelpers, /request\\\.\(path\|url\)\\b/);
  assert.match(
    requestHelpers,
    /const\|let\)\\s\+\(\[A-Za-z_\$]\[A-Za-z0-9_\$]\*\)\\s\*\(\?:\:\[\^=;\\n]\+\)\?\\s\*=\\s\*\(\[\^;\\n]\+\)/,
  );
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_reads_typed_request_aliases_without_next_runtime/,
  );
  assert.match(deliveryTests, /const methodName: string = request\.method;/);
  assert.match(deliveryTests, /const requestUrl: string = request\.url;/);
  assert.match(deliveryTests, /const url: URL = new URL\(request\.url\);/);
  assert.match(deliveryTests, /const nextUrl: URL = request\.nextUrl;/);
  assert.match(deliveryTests, /const headers: Headers = request\.headers;/);
  assert.match(deliveryTests, /const agent: string = headers\.get\("user-agent"\) \?\? "none";/);
  assert.match(deliveryTests, /const mode: string = url\.searchParams\.get\("mode"\) \?\? "all";/);
  assert.match(deliveryTests, /const query: string = nextUrl\.searchParams\.get\("q"\) \?\? "none";/);
  assert.doesNotMatch(`${urlHelpers}\n${requestHelpers}`, /node_modules|NextRuntime|TurbopackRuntime/);
});
