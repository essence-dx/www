import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceRenderPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_router_execution",
  "source_render.rs",
);

function read(filePath) {
  assert.ok(fs.existsSync(filePath), `missing ${path.relative(root, filePath)}`);
  return fs.readFileSync(filePath, "utf8");
}

test("App Router source render records template remount boundaries", () => {
  const sourceRender = read(sourceRenderPath);

  assert.match(sourceRender, /template_boundaries/);
  assert.match(sourceRender, /template_boundary_record/);
  assert.match(sourceRender, /template_boundary_records_remount_semantics_without_runtime_parity/);
  assert.match(sourceRender, /"source_owned_template_boundary"/);
  assert.match(sourceRender, /"remount_on_navigation": true/);
  assert.match(sourceRender, /"persistent_across_navigation": false/);
  assert.match(sourceRender, /"full_react_template_runtime": false/);
});
