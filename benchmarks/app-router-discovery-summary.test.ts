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

test("App Router route discovery exposes manifest-ready relative summaries", () => {
  const source = read("dx-www/src/cli/app_page_routes.rs");

  assert.match(source, /pub\(super\) struct AppDiscoveredSegmentSummary/);
  assert.match(source, /pub\(super\) struct AppDiscoveredRouteSummary/);
  assert.match(source, /pub\(super\) fn discover_page_route_summaries\(cwd: &Path\)/);
  assert.match(source, /source_path: String/);
  assert.match(source, /segment_files: Vec<AppDiscoveredSegmentSummary>/);
  assert.match(source, /shadowed_source_paths: Vec<String>/);
  assert.match(source, /fn discovered_segment_summary/);
  assert.match(source, /fn discovered_route_summary/);
  assert.match(source, /discover_page_route_summaries_are_manifest_ready_and_relative/);
  assert.match(source, /assert!\(!summary\.source_path\.contains\(root_string\.as_str\(\)\)\)/);
  assert.doesNotMatch(source, /Next DevTools|Turbopack powers|full Next\.js parity/);
});
