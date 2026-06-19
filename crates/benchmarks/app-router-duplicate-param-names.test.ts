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

test("App Router filesystem routing rejects duplicate route parameter names", () => {
  const source = read("dx-www/src/cli/app_page_routes.rs");
  const segments = read("dx-www/src/app_router_segments.rs");

  assert.match(segments, /pub\(crate\) fn route_segments_have_duplicate_param_names/);
  assert.match(segments, /pub\(crate\) fn route_segment_param_name/);
  assert.match(source, /app_router_segments::has_unsupported_app_page_route_segments/);
  assert.match(source, /discover_page_routes_skips_duplicate_parameter_names/);
  assert.match(source, /route_match_rejects_duplicate_parameter_names/);
  assert.match(
    source,
    /route_path_from_page_source_path\("app\/\[team\]\/\[team\]\/page\.tsx"\),\s*None/
  );
  assert.match(
    source,
    /route_path_from_page_source_path\("app\/docs\/\[slug\]\/\[\.\.\.slug\]\/page\.tsx"\),\s*None/
  );
  assert.match(
    source,
    /route_path_from_page_source_path\("app\/files\/\[path\]\/\[\[\.\.\.path\]\]\/page\.tsx"\),\s*None/
  );
  assert.doesNotMatch(
    [source, segments].join("\n"),
    /Next DevTools|Turbopack powers|full Next\.js parity/
  );
});
