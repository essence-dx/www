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

test("source-build App Router route paths use the shared segment classifier", () => {
  const graph = read("dx-www/src/build/source_engine/graph.rs");
  const discovery = read("dx-www/src/build/source_engine/discovery.rs");

  assert.match(graph, /use crate::app_router_segments::\{[^}]*AppRouteSegmentKind[^}]*\};/);
  assert.match(graph, /use crate::app_router_segments::\{[^}]*classify_app_route_segment[^}]*\};/);
  assert.match(graph, /fn route_segment\(segment: &str\) -> Option<String>/);
  assert.match(graph, /AppRouteSegmentKind::RouteGroup \| AppRouteSegmentKind::ParallelSlot => None/);
  assert.match(graph, /AppRouteSegmentKind::RequiredCatchAll\(param\) => Some\(format!\("\+{param}"\)\)/);
  assert.match(graph, /AppRouteSegmentKind::OptionalCatchAll\(param\) => Some\(format!\("\*{param}"\)\)/);
  assert.match(graph, /AppRouteSegmentKind::Dynamic\(param\) => Some\(format!\(":{param}"\)\)/);
  assert.match(graph, /source_build_route_from_app_page_uses_next_familiar_segments/);
  assert.doesNotMatch(graph, /segment\.starts_with\("\[\.\.\."\) && segment\.ends_with\('\]'\)/);

  assert.match(discovery, /has_unsupported_app_page_route_segments/);
  assert.match(discovery, /unsupported_app_page_route_segment/);
  assert.match(discovery, /has_unsupported_app_page_route_segments\(&segments\)/);
  assert.match(discovery, /source_discovery_skips_unsupported_app_page_route_shapes/);
  assert.doesNotMatch(discovery, /fn is_private_app_folder_segment/);
  assert.doesNotMatch(discovery, /fn is_intercepting_app_route_segment/);
  assert.doesNotMatch(discovery, /classify_app_route_segment\(&segment\)/);
});
