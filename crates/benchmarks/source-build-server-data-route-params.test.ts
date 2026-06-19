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

test("source-build server-data route params use shared App Router segment semantics", () => {
  const serverData = read("dx-www/src/build/source_engine/server_data.rs");

  assert.match(
    serverData,
    /use crate::app_router_segments::\{(?:AppRouteSegmentKind, classify_app_route_segment|classify_app_route_segment, AppRouteSegmentKind)\};/,
  );
  assert.match(serverData, /enum StaticRouteParamShape/);
  assert.match(serverData, /fn static_route_param_samples\(route_source_path: &str\) -> BTreeMap<String, Value>/);
  assert.match(serverData, /fn static_route_param_shape\(segment: &str\) -> Option<\(&str, StaticRouteParamShape\)>/);
  assert.match(serverData, /fn static_route_param_sample_value\(name: &str, shape: StaticRouteParamShape\) -> Value/);
  assert.match(serverData, /AppRouteSegmentKind::Dynamic\(param\)/);
  assert.match(serverData, /AppRouteSegmentKind::RequiredCatchAll\(param\)/);
  assert.match(serverData, /AppRouteSegmentKind::OptionalCatchAll\(param\)/);
  assert.match(serverData, /StaticRouteParamShape::RequiredCatchAll => json!\(\[static_request_sample_value\(name\)\]\)/);
  assert.match(serverData, /StaticRouteParamShape::OptionalCatchAll => json!\(\[\]\)/);
  assert.match(serverData, /AppRouteSegmentKind::RouteGroup/);
  assert.match(serverData, /AppRouteSegmentKind::ParallelSlot/);
  assert.match(serverData, /AppRouteSegmentKind::Malformed/);
  assert.match(serverData, /static_route_param_names_use_shared_app_router_segments/);
  assert.match(serverData, /catch_all_route_params_use_array_samples/);
  assert.match(serverData, /"app\/docs\/\[\.\.\.slug\]\/page\.tsx"/);
  assert.match(serverData, /\["sample-slug"\]/);
  assert.match(serverData, /"app\/files\/\[\[\.\.\.path\]\]\/page\.tsx"/);
  assert.doesNotMatch(serverData, /fn is_route_group_segment/);
  assert.doesNotMatch(serverData, /fn dynamic_segment_name/);
});
