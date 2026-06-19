import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");

function readRepoFile(relativePath) {
  return readFileSync(resolve(repoRoot, relativePath), "utf8");
}

test("app API route path ownership stays out of the giant CLI module", () => {
  const cliMod = readRepoFile("dx-www/src/cli/mod.rs");
  const appApiRoutes = readRepoFile("dx-www/src/cli/app_api_routes.rs");

  assert.match(cliMod, /\bmod app_api_routes;/);
  assert.doesNotMatch(cliMod, /\bfn app_api_route_handler_path\b/);
  assert.doesNotMatch(cliMod, /\bfn dynamic_route_handler_match\b/);
  assert.doesNotMatch(cliMod, /\bfn route_match_candidate\b/);
  assert.doesNotMatch(cliMod, /\bfn is_app_router_route_group_segment\b/);
  assert.doesNotMatch(cliMod, /\bfn is_app_router_parallel_slot_segment\b/);

  assert.match(appApiRoutes, /\bpub\(super\) struct AppApiRouteMatch\b/);
  assert.match(appApiRoutes, /\bpub\(super\) fn route_handler_match\b/);
  assert.doesNotMatch(appApiRoutes, /\bpub\(super\) fn route_handler_path\b/);
  assert.match(appApiRoutes, /pub\(super\) params: BTreeMap<String, String>/);
  assert.match(appApiRoutes, /pub\(super\) search_params: BTreeMap<String, String>/);
  assert.match(
    appApiRoutes,
    /const ROUTE_HANDLER_FILENAMES: &\[&str\] = &\["route\.ts", "route\.tsx", "route\.js", "route\.jsx"\];/,
  );
  assert.match(appApiRoutes, /const APP_API_ROUTE_ROOTS: &\[&str\] = &\["app\/api", "src\/app\/api"\];/);
  assert.match(appApiRoutes, /use super::route_request_values::\{decode_path_segment, decode_path_segments, parse_search_params\};/);
  assert.match(appApiRoutes, /\bfn dynamic_route_handler_match\b/);
  assert.match(appApiRoutes, /\bfn route_match_candidate\b/);
  assert.match(appApiRoutes, /\bfn is_app_router_route_group_segment\b/);
  assert.match(appApiRoutes, /\bfn is_app_router_parallel_slot_segment\b/);
  assert.match(appApiRoutes, /search_params: parse_search_params\(request_path\)/);
  assert.match(appApiRoutes, /search_params: parse_search_params\(path\)/);
  assert.match(appApiRoutes, /\bfn route_handler_match_accepts_next_route_handler_extensions\b/);
  assert.match(appApiRoutes, /\bfn route_handler_match_ignores_app_router_route_groups\b/);
  assert.match(appApiRoutes, /\bfn route_handler_match_ignores_app_router_parallel_slots\b/);
  assert.match(appApiRoutes, /\bfn route_handler_match_decodes_request_params_and_search_params\b/);
  assert.match(appApiRoutes, /\bfn route_handler_match_prefers_app_api_over_src_app_api_for_equal_dynamic_score\b/);
});
