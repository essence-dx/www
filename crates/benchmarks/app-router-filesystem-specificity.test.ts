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

test("App Router filesystem matching keeps non-path and catch-all segments below exact routes", () => {
  const source = read("dx-www/src/cli/app_page_routes.rs");

  assert.match(source, /catch_all_used/);
  assert.match(source, /non_path_segment_count/);
  assert.doesNotMatch(source, /is_route_group_segment\(&segment\)\s*\{\s*score \+= 1;/);
  assert.match(source, /route_match_prefers_exact_route_over_optional_catchall/);
  assert.match(source, /route_match_prefers_visible_route_over_route_group_duplicate/);
  assert.match(source, /route_match_prefers_visible_route_over_parallel_slot_duplicate/);
});
