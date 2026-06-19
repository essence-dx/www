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

test("DX Rust routers normalize browser request paths without hiding boundaries", () => {
  const router = read("dx-www/src/router/mod.rs");
  const matcher = read("dx-www/src/router/matcher.rs");
  const pattern = read("dx-www/src/router/pattern.rs");
  const api = read("dx-www/src/api/mod.rs");

  assert.match(router, /fn strip_request_suffix\(path: &str\) -> &str/);
  assert.match(router, /let mut normalized = strip_request_suffix\(path\)\.to_string\(\);/);
  assert.match(router, /route_matching_ignores_query_and_fragment_suffixes/);

  assert.match(matcher, /fn normalized_request_path_segments\(path: &str\) -> Vec<String>/);
  assert.match(matcher, /let segments = normalized_request_path_segments\(path\);/);
  assert.match(matcher, /params\.insert\(name\.clone\(\), segment\.clone\(\)\);/);
  assert.match(matcher, /matcher_ignores_query_and_decodes_params/);
  assert.doesNotMatch(matcher, /let segments: Vec<&str> = path\.split\('/);

  assert.match(pattern, /fn normalized_request_path_segments\(path: &str\) -> Vec<String>/);
  assert.match(pattern, /\.map\(decode_path_segment\)/);
  assert.match(pattern, /match_path_decodes_dynamic_and_catch_all_params/);
  assert.match(pattern, /match_path_ignores_query_and_fragment_suffixes/);
  assert.doesNotMatch(pattern, /node_modules|_next|turbopack/);

  assert.match(api, /fn normalize_api_request_path\(path: &str\) -> String/);
  assert.match(api, /fn decode_api_path_segment\(value: &str\) -> String/);
  assert.match(api, /api_match_route_ignores_query_and_decodes_params/);
  assert.match(api, /api_find_route_accepts_query_suffixes/);
  assert.doesNotMatch(api, /node_modules|_next|turbopack/);
});
