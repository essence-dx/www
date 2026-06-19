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

test("App Router discovery summaries expose route specificity metadata", () => {
  const source = read("dx-www/src/cli/app_page_routes.rs");

  assert.match(source, /pub\(super\) enum AppDiscoveredRouteSegmentKind/);
  assert.match(source, /pub\(super\) struct AppDiscoveredRouteSpecificity/);
  assert.match(source, /specificity: AppDiscoveredRouteSpecificity/);
  assert.match(source, /segment_kinds: Vec<AppDiscoveredRouteSegmentKind>/);
  assert.match(source, /static_segment_count: usize/);
  assert.match(source, /dynamic_segment_count: usize/);
  assert.match(source, /catch_all_segment_count: usize/);
  assert.match(source, /optional_catch_all_segment_count: usize/);
  assert.match(source, /precedence_score: usize/);
  assert.match(source, /fn discovered_route_specificity_from_segments/);
  assert.match(source, /fn discovered_route_segment_kind/);
  assert.match(source, /discover_page_route_summaries_carry_specificity_for_manifest_sorting/);
  assert.match(source, /AppDiscoveredRouteSegmentKind::Static/);
  assert.match(source, /AppDiscoveredRouteSegmentKind::Dynamic/);
  assert.match(source, /AppDiscoveredRouteSegmentKind::CatchAll/);
  assert.match(source, /AppDiscoveredRouteSegmentKind::OptionalCatchAll/);
  assert.doesNotMatch(source, /Next DevTools|Turbopack powers|full Next\.js parity/);
});
