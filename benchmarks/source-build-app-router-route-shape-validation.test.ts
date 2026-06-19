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

test("source-build App Router discovery uses shared page-route shape validation", () => {
  const segments = read("dx-www/src/app_router_segments.rs");
  const discovery = read("dx-www/src/build/source_engine/discovery.rs");
  const cliRoutes = read("dx-www/src/cli/app_page_routes.rs");

  assert.match(segments, /pub\(crate\) fn has_unsupported_app_page_route_segments/);
  assert.match(segments, /pub\(crate\) fn route_segments_have_duplicate_param_names/);
  assert.match(segments, /pub\(crate\) fn route_segments_have_nonterminal_catch_all/);
  assert.match(segments, /rejects_unsupported_app_page_route_segment_shapes/);

  assert.match(discovery, /has_unsupported_app_page_route_segments/);
  assert.match(discovery, /source_discovery_skips_unsupported_app_page_route_shapes/);
  assert.match(
    discovery,
    /app\/docs\/\[\.\.\.slug\]\/details\/page\.tsx/
  );
  assert.match(discovery, /app\/\[team\]\/\[team\]\/page\.tsx/);

  assert.match(cliRoutes, /app_router_segments::has_unsupported_app_page_route_segments/);
  assert.doesNotMatch(
    [segments, discovery, cliRoutes].join("\n"),
    /Next DevTools|Turbopack powers|full Next\.js parity/
  );
});
