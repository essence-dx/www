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

test("App Router page matcher records shadowed duplicate public routes", () => {
  const source = read("dx-www/src/cli/app_page_routes.rs");

  assert.match(source, /shadowed_paths: Vec<PathBuf>/);
  assert.match(source, /struct ScoredAppRouteMatch/);
  assert.match(source, /route_path: String/);
  assert.match(source, /route_match_reports_shadowed_route_group_duplicates/);
  assert.match(source, /route_match_reports_shadowed_parallel_slot_duplicates/);
  assert.doesNotMatch(source, /map\(\|\(_, _, _, _, route_match\)\| route_match\)/);
});
