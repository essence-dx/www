import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  const fullPath = path.join(repoRoot, relativePath);
  assert.ok(fs.existsSync(fullPath), `expected ${relativePath} to exist`);
  return fs.readFileSync(fullPath, "utf8");
}

test("source-build route outputs pass real App Router segment sources into the compiler", () => {
  const routeOutput = read("dx-www/src/build/source_engine/route_output.rs");

  assert.match(routeOutput, /DxReactAppSegmentKind/);
  assert.match(routeOutput, /DxReactAppSegmentSource/);
  assert.match(routeOutput, /const APP_ROUTER_SOURCE_EXTENSIONS: &\[&str\]/);
  assert.match(routeOutput, /const APP_ROUTER_SOURCE_ROOTS: &\[&str\]/);
  assert.match(routeOutput, /fn app_route_segments\(/);
  assert.match(routeOutput, /fn app_segment_dirs\(/);
  assert.match(routeOutput, /fn push_app_segment_source\(/);
  assert.match(routeOutput, /fn first_existing_app_special_file\(/);
  assert.match(routeOutput, /segments: app_route_segments\(project_root, &route\.path\)\?/);
  assert.match(routeOutput, /DxReactAppSegmentKind::Layout/);
  assert.match(routeOutput, /DxReactAppSegmentKind::Template/);
  assert.match(routeOutput, /DxReactAppSegmentKind::Loading/);
  assert.match(routeOutput, /DxReactAppSegmentKind::Error/);
  assert.match(routeOutput, /DxReactAppSegmentKind::NotFound/);
  assert.doesNotMatch(routeOutput, /segments: Vec::new\(\)/);
  assert.doesNotMatch(routeOutput, /project_root\.join\("node_modules"\)|turbopack_hmr|full Next\.js parity/);
});
