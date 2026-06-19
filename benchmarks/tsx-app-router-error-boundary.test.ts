import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceRenderPath = path.join(root, "dx-www", "src", "cli", "app_router_execution", "source_render.rs");

function read(filePath) {
  assert.ok(fs.existsSync(filePath), `missing ${path.relative(root, filePath)}`);
  return fs.readFileSync(filePath, "utf8");
}

test("App Router source render selects error boundary for safe page throws", () => {
  const sourceRender = read(sourceRenderPath);

  assert.match(sourceRender, /detects_static_page_error_throw/);
  assert.match(sourceRender, /selected_app_router_leaf_document/);
  assert.match(sourceRender, /error_boundary_selected/);
  assert.match(sourceRender, /error_boundary_replaces_page_leaf_for_static_throw/);
  assert.match(sourceRender, /"error-boundary-ready"/);
  assert.match(sourceRender, /"error-boundary-static-html"/);
  assert.match(sourceRender, /"selected_leaf"/);
});
