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

test("App Router filesystem routing rejects non-terminal public catch-all segments", () => {
  const source = read("dx-www/src/cli/app_page_routes.rs");
  const segments = read("dx-www/src/app_router_segments.rs");

  assert.match(segments, /pub\(crate\) fn route_segments_have_nonterminal_catch_all/);
  assert.match(source, /app_router_segments::has_unsupported_app_page_route_segments/);
  assert.match(source, /discover_page_routes_skips_nonterminal_catch_all_segments/);
  assert.match(source, /route_match_rejects_nonterminal_catch_all_segments/);
  assert.match(
    source,
    /route_path_from_page_source_path\("app\/docs\/\[\.\.\.slug\]\/details\/page\.tsx"\),\s*None/
  );
  assert.match(
    source,
    /route_path_from_page_source_path\("app\/files\/\[\[\.\.\.path\]\]\/preview\/page\.tsx"\),\s*None/
  );
  assert.match(
    source,
    /route_path_from_page_source_path\("app\/docs\/\[\.\.\.slug\]\/\(guide\)\/page\.tsx"\),\s*Some\("\/docs\/\[\.\.\.slug\]"\.to_string\(\)\)/
  );
  assert.doesNotMatch(
    [source, segments].join("\n"),
    /Next DevTools|Turbopack powers|full Next\.js parity/
  );
});
