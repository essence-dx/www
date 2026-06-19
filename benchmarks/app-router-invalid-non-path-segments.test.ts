const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("App Router filesystem routing rejects malformed non-path segment shapes", () => {
  const classifier = read("dx-www/src/app_router_segments.rs");
  const source = read("dx-www/src/cli/app_page_routes.rs");
  const discovery = read("dx-www/src/build/source_engine/discovery.rs");

  assert.match(classifier, /fn valid_route_group_name/);
  assert.match(classifier, /fn has_mismatched_route_group_delimiters/);
  assert.match(classifier, /segment == "@"/);
  assert.match(classifier, /classify_app_route_segment\("\(\)"\),\s*AppRouteSegmentKind::Malformed/);
  assert.match(
    source,
    /discover_page_routes_skips_malformed_non_path_segments/,
  );
  assert.match(source, /route_match_rejects_malformed_non_path_segments/);
  assert.match(source, /route_path_from_page_source_path\("app\/\(\)\/page\.tsx"\),\s*None/);
  assert.match(source, /route_path_from_page_source_path\("app\/\(marketing\/page\.tsx"\),\s*None/);
  assert.match(source, /route_path_from_page_source_path\("app\/marketing\)\/page\.tsx"\),\s*None/);
  assert.match(source, /route_path_from_page_source_path\("app\/@\/page\.tsx"\),\s*None/);
  assert.match(discovery, /app\/\(\)\/page\.tsx/);
  assert.match(discovery, /app\/@\/page\.tsx/);
  assert.doesNotMatch(
    [classifier, source, discovery].join("\n"),
    /Next DevTools|Turbopack powers|full Next\.js parity/,
  );
});
