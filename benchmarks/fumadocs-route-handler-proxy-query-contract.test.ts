import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("Fumadocs OpenAPI proxy interpreter matches request.url paths and decodes url query policy", () => {
  const routeHandler = read("core/src/delivery/route_handler_fumadocs.rs");

  assert.match(
    routeHandler,
    /request\.path_for_match\(\)\.trim_end_matches\('\/'\)\s*!=\s*"\/api\/openapi\/proxy"/,
    "proxy endpoint gate must normalize absolute URLs, query strings, and fragments before matching",
  );
  assert.doesNotMatch(
    routeHandler,
    /request\.path\.trim_end_matches\('\/'\)\s*!=\s*"\/api\/openapi\/proxy"/,
    "proxy endpoint gate must not use the raw request path for matching",
  );
  assert.match(
    routeHandler,
    /fn search_param_from_path\(path: &str, key: &str\) -> Option<String>/,
    "proxy policy must recover search params from request.path when no parsed map was provided",
  );
  assert.match(
    routeHandler,
    /percent_decode_query_component\(raw_value\)/,
    "encoded url=https%3A%2F%2F... query values must be decoded before policy evaluation",
  );
  assert.match(
    routeHandler,
    /fn openapi_proxy_accepts_encoded_query_url_from_request_path\(\)/,
    "Rust unit coverage must lock the encoded query-url policy path",
  );
  assert.match(
    routeHandler,
    /proxyRequestForwarded": false/,
    "safe interpreter may accept policy but must not forward network requests",
  );
});
