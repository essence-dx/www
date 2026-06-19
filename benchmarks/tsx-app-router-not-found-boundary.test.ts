import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceRenderPath = path.join(root, "dx-www", "src", "cli", "app_router_execution", "source_render.rs");
const nextNavigationPath = path.join(root, "dx-www", "src", "cli", "app_router_execution", "next_navigation.rs");

function read(filePath) {
  assert.ok(fs.existsSync(filePath), `missing ${path.relative(root, filePath)}`);
  return fs.readFileSync(filePath, "utf8");
}

test("App Router source render selects not-found boundary for safe notFound calls", () => {
  const sourceRender = read(sourceRenderPath);
  const nextNavigation = read(nextNavigationPath);

  assert.match(nextNavigation, /pub\(super\) fn detects_next_navigation_not_found/);
  assert.match(sourceRender, /detects_next_navigation_not_found/);
  assert.match(sourceRender, /selected_app_router_leaf_document/);
  assert.match(sourceRender, /nearest_app_router_boundary_document/);
  assert.match(sourceRender, /app_router_boundary_scope_matches_page/);
  assert.match(sourceRender, /not_found_boundary_selected/);
  assert.match(sourceRender, /not_found_boundary_replaces_page_leaf_in_app_router_shell/);
  assert.match(sourceRender, /not_found_boundary_uses_nearest_route_scoped_boundary/);
  assert.match(sourceRender, /"not-found-boundary-ready"/);
  assert.match(sourceRender, /"selected_leaf"/);
});
