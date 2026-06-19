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

test("App Router filesystem routing rejects malformed bracket parameter segments", () => {
  const source = read("dx-www/src/cli/app_page_routes.rs");
  const classifier = read("dx-www/src/app_router_segments.rs");

  assert.match(classifier, /fn valid_app_route_param_name/);
  assert.match(classifier, /AppRouteSegmentKind::Malformed/);
  assert.match(source, /fn is_malformed_app_route_parameter_segment/);
  assert.match(source, /app_router_segments::is_malformed_app_route_parameter_segment/);
  assert.match(source, /discover_page_routes_skips_malformed_parameter_segments/);
  assert.match(source, /route_match_rejects_malformed_parameter_segments/);
  assert.match(source, /route_path_from_page_source_path\("app\/\[\]\/page\.tsx"\),\s*None/);
  assert.match(source, /route_path_from_page_source_path\("app\/\[\[id\]\]\/page\.tsx"\),\s*None/);
  assert.match(source, /route_path_from_page_source_path\("app\/docs\/\[\.\.\.\]\/page\.tsx"\),\s*None/);
  assert.match(source, /route_path_from_page_source_path\("app\/docs\/\[\[\.\.\.\]\]\/page\.tsx"\),\s*None/);
  assert.doesNotMatch(source, /Next DevTools|Turbopack powers|full Next\.js parity/);
});
