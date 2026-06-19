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

test("App Router build-time request props use the shared filesystem segment classifier", () => {
  const source = read("dx-www/src/cli/app_router_build_output.rs");

  assert.match(
    source,
    /use crate::app_router_segments::\{(?:AppRouteSegmentKind, classify_app_route_segment|classify_app_route_segment, AppRouteSegmentKind)\};/,
  );
  assert.match(source, /fn route_param_name\(segment: &str\) -> Option<&str>/);
  assert.match(source, /match classify_app_route_segment\(segment\)/);
  assert.match(source, /AppRouteSegmentKind::OptionalCatchAll\(name\)/);
  assert.match(source, /AppRouteSegmentKind::RequiredCatchAll\(name\)/);
  assert.match(source, /AppRouteSegmentKind::Dynamic\(name\)/);
  assert.match(source, /static_request_props_use_shared_app_router_segments/);
  assert.doesNotMatch(source, /strip_prefix\("\[\[\.\.\."\)/);
  assert.doesNotMatch(source, /strip_prefix\("\[\.\.\."\)/);
});
