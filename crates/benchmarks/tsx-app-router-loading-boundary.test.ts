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

test("App Router source render exposes loading boundary preview for deferred pages", () => {
  const sourceRender = read(sourceRenderPath);

  assert.match(sourceRender, /detects_deferred_page_render/);
  assert.match(sourceRender, /loading_boundary_preview/);
  assert.match(sourceRender, /loading_boundary_reuses_static_loading_segment_for_deferred_page/);
  assert.match(sourceRender, /"loading_boundary"/);
  assert.match(sourceRender, /"loading-boundary-ready"/);
  assert.match(sourceRender, /"full_streaming_runtime": false/);
  assert.match(sourceRender, /"source_owned_loading_boundary": true/);
});
