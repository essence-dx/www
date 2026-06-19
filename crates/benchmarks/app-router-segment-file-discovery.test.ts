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

test("App Router page discovery carries layout and template segment files", () => {
  const source = read("dx-www/src/cli/app_page_routes.rs");

  assert.match(source, /pub\(super\) enum AppDiscoveredSegmentKind/);
  assert.match(source, /pub\(super\) struct AppDiscoveredSegmentFile/);
  assert.match(source, /segment_files: Vec<AppDiscoveredSegmentFile>/);
  assert.match(source, /fn discover_segment_files_for_page_source/);
  assert.match(source, /fn app_special_file_path/);
  assert.match(source, /discover_page_routes_carries_layout_template_and_boundary_files/);
  assert.match(source, /AppDiscoveredSegmentKind::Layout/);
  assert.match(source, /AppDiscoveredSegmentKind::Template/);
  assert.match(source, /AppDiscoveredSegmentKind::Loading/);
  assert.match(source, /AppDiscoveredSegmentKind::Error/);
  assert.match(source, /AppDiscoveredSegmentKind::NotFound/);
  assert.doesNotMatch(source, /Next DevTools|Turbopack powers|full Next\.js parity/);
});
