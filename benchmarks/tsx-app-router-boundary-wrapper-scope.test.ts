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

test("App Router boundary leaves are not wrapped by child layouts below the selected boundary", () => {
  const sourceRender = read(sourceRenderPath);

  assert.match(
    sourceRender,
    /not_found_boundary_does_not_use_child_layouts_below_selected_boundary/,
  );
  assert.match(
    sourceRender,
    /error_boundary_does_not_use_child_layouts_below_selected_boundary_and_records_skip/,
  );
  assert.match(sourceRender, /fn app_router_wrapper_scope_matches_leaf\(/);
  assert.match(sourceRender, /fn app_router_scope_skipped_wrapper_record\(/);
  assert.match(sourceRender, /scope_skipped_wrappers: Vec<Value>/);
  assert.match(sourceRender, /"scope_skipped_wrapper_count": self\.scope_skipped_wrappers\.len\(\)/);
  assert.match(sourceRender, /"reason": "below-selected-boundary-scope"/);
  assert.match(
    sourceRender,
    /if !app_router_wrapper_scope_matches_leaf\(document, wrapper_scope_leaf\)/,
  );
  assert.match(
    sourceRender,
    /app_router_boundary_scope_matches_page\(&document\.source_path, &leaf\.source_path\)/,
  );
});
