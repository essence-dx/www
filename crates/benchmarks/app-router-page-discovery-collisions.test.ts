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

test("App Router page discovery reports duplicate public routes before request matching", () => {
  const source = read("dx-www/src/cli/app_page_routes.rs");

  assert.match(source, /pub\(super\) struct AppDiscoveredPageRoute/);
  assert.match(source, /pub\(super\) fn discover_page_routes\(cwd: &Path\)/);
  assert.match(source, /fn app_page_route_sources\(cwd: &Path\)/);
  assert.match(source, /route_path: String/);
  assert.match(source, /root_index: usize/);
  assert.match(source, /non_path_segment_count: usize/);
  assert.match(source, /shadowed_paths: Vec<PathBuf>/);
  assert.match(
    source,
    /discover_page_routes_reports_duplicate_public_paths_before_matching/,
  );
  assert.doesNotMatch(source, /Next DevTools|Turbopack powers|full Next\.js parity/);
});
