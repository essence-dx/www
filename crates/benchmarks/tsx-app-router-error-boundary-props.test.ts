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

test("App Router source render exposes bounded error boundary props", () => {
  const sourceRender = read(sourceRenderPath);

  assert.match(sourceRender, /error_boundary_props/);
  assert.match(sourceRender, /error_boundary_prop_bindings/);
  assert.match(sourceRender, /error_boundary_records_error_and_reset_props_without_client_runtime/);
  assert.match(sourceRender, /"source_owned_error_boundary_props"/);
  assert.match(sourceRender, /"error.message"/);
  assert.match(sourceRender, /"reset"/);
  assert.match(sourceRender, /"reset_invocable": false/);
  assert.match(sourceRender, /"full_client_error_runtime": false/);
});
