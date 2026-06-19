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

test("App Router filesystem routing uses one shared segment classifier", () => {
  const classifier = read("dx-www/src/app_router_segments.rs");
  const lib = read("dx-www/src/lib.rs");
  const project = read("dx-www/src/project.rs");
  const appPageRoutes = read("dx-www/src/cli/app_page_routes.rs");

  assert.match(lib, /mod app_router_segments;/);
  assert.match(classifier, /pub\(crate\) enum AppRouteSegmentKind/);
  assert.match(classifier, /pub\(crate\) fn classify_app_route_segment/);
  assert.match(classifier, /RouteGroup/);
  assert.match(classifier, /ParallelSlot/);
  assert.match(classifier, /RequiredCatchAll/);
  assert.match(classifier, /OptionalCatchAll/);
  assert.match(classifier, /Malformed/);
  assert.match(classifier, /classifies_next_familiar_app_route_segments/);

  assert.match(project, /use crate::app_router_segments::\{/);
  assert.match(project, /classify_app_route_segment\(segment\)/);
  assert.doesNotMatch(project, /fn is_valid_app_route_param_name/);

  assert.match(appPageRoutes, /use crate::app_router_segments/);
  assert.match(appPageRoutes, /classify_app_route_segment\(&segment\)/);
  assert.doesNotMatch(appPageRoutes, /fn valid_app_route_param_name/);
});
