import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = process.cwd();

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("App Router source render replaces safe redirect pages with a redirect marker leaf", () => {
  const sourceRender = read("dx-www/src/cli/app_router_execution/source_render.rs");
  const nextNavigation = read("dx-www/src/cli/app_router_execution/next_navigation.rs");

  assert.match(nextNavigation, /pub\(super\) fn next_navigation_redirect/);
  assert.match(sourceRender, /next_navigation_redirect/);
  assert.match(sourceRender, /redirect_boundary_selected/);
  assert.match(sourceRender, /redirect_leaf_html/);
  assert.match(sourceRender, /data-dx-app-router-redirect/);
  assert.match(sourceRender, /"redirect-boundary-ready"/);
  assert.match(sourceRender, /"redirect_selected"/);
  assert.match(sourceRender, /redirect_control_flow_replaces_page_leaf_with_static_marker/);
});

test("App Router source render honors safe aliased next/navigation control-flow helpers", () => {
  const sourceRender = read("dx-www/src/cli/app_router_execution/source_render.rs");
  const nextNavigation = read("dx-www/src/cli/app_router_execution/next_navigation.rs");

  assert.match(nextNavigation, /next_navigation_imported_names/);
  assert.match(nextNavigation, /next_navigation_namespace_imports/);
  assert.match(nextNavigation, /navigation_control_flow_accepts_aliased_next_navigation_helpers/);
  assert.match(nextNavigation, /navigation_control_flow_accepts_namespace_next_navigation_helpers/);
  assert.match(nextNavigation, /navigation_control_flow_ignores_unimported_local_helper_names/);
  assert.match(sourceRender, /redirect_control_flow_replaces_aliased_redirect_page_leaf/);
  assert.match(sourceRender, /redirect_control_flow_replaces_namespace_redirect_page_leaf/);
  assert.match(sourceRender, /not_found_boundary_selects_aliased_not_found_import/);
  assert.match(sourceRender, /not_found_boundary_selects_namespace_not_found_import/);
  assert.match(sourceRender, /page_named_redirect_function_is_not_navigation_control_flow_without_import/);
});
