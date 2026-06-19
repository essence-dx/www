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

test("App Router discovery exposes dynamic public-shape collisions", () => {
  const source = read("dx-www/src/cli/app_page_routes.rs");

  assert.match(source, /route_shape: String/);
  assert.match(source, /shape_collision_paths: Vec<PathBuf>/);
  assert.match(source, /shape_collision_source_paths: Vec<String>/);
  assert.match(source, /fn route_shape_from_page_source_path/);
  assert.match(source, /fn shape_segment_from_route_segment/);
  assert.match(source, /fn mark_shape_collision_discovered_page_routes/);
  assert.match(source, /discover_page_routes_reports_dynamic_shape_collisions/);
  assert.match(source, /"\/users\/\[\]"/);
  assert.match(source, /"\/docs\/\[\.\.\.\]"/);
  assert.match(source, /"\/files\/\[\[\.\.\.\]\]"/);
  assert.doesNotMatch(source, /Next DevTools|Turbopack powers|full Next\.js parity/);
});
