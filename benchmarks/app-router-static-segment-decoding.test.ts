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

test("App Router matcher decodes percent-encoded static request segments", () => {
  const source = read("dx-www/src/cli/app_page_routes.rs");

  assert.match(source, /AppRouteSegmentKind::Static\(segment\)/);
  assert.match(source, /let decoded_value = decode_path_segment\(value\);/);
  assert.match(source, /segment != decoded_value\.as_str\(\)/);
  assert.match(source, /route_match_decodes_static_request_segments/);
  assert.doesNotMatch(source, /Next DevTools|Turbopack powers|full Next\.js parity/);
});
