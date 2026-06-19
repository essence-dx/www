const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("App Router dynamic shape collision evidence is attached to every peer route", () => {
  const source = read("dx-www/src/cli/app_page_routes.rs");

  assert.match(source, /fn shape_collision_peer_paths/);
  assert.match(source, /discover_page_routes_reports_all_dynamic_shape_collision_peers/);
  assert.match(source, /users_slug\.shape_collision_paths/);
  assert.match(source, /docs_slug\.shape_collision_paths/);
  assert.match(source, /files_segments\.shape_collision_paths/);
  assert.doesNotMatch(
    source,
    /route_for\("app\/users\/\[slug\]\/page\.tsx"\)[\s\S]{0,140}shape_collision_paths\s*\.is_empty\(\)/,
  );
  assert.doesNotMatch(source, /Next DevTools|Turbopack powers|full Next\.js parity/);
});
