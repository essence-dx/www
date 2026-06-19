const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("App API route handler dynamic matches keep root precedence deterministic", () => {
  const appApiRoutes = read("dx-www/src/cli/app_api_routes.rs");

  assert.match(appApiRoutes, /const APP_API_ROUTE_ROOTS: &\[&str\] = &\["app\/api", "src\/app\/api"\]/);
  assert.match(
    appApiRoutes,
    /for \(root_index, api_root\) in app_api_route_roots\(cwd\)\.into_iter\(\)\.enumerate\(\)/,
  );
  assert.match(
    appApiRoutes,
    /let root_score = APP_API_ROUTE_ROOTS\.len\(\)\.saturating_sub\(root_index\);/,
  );
  assert.match(
    appApiRoutes,
    /\.max_by_key\(\|\(score, root_score, _\)\| \(\*score, \*root_score\)\)/,
  );
  assert.match(
    appApiRoutes,
    /route_handler_match_prefers_app_api_over_src_app_api_for_equal_dynamic_score/,
  );
});
