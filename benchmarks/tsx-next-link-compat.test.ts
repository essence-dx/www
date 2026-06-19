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

test("generic TSX renderer lowers static next/link usage to source-owned anchors", () => {
  const sourceRender = read(sourceRenderPath);

  assert.match(sourceRender, /next_link_component_names/);
  assert.match(sourceRender, /is_next_link_element/);
  assert.match(sourceRender, /is_static_renderable_element/);
  assert.match(sourceRender, /static_html_tag_name/);
  assert.match(sourceRender, /next\/link/);
  assert.match(sourceRender, /next-link-static-anchor/);
  assert.match(sourceRender, /data-dx-framework-component/);
  assert.match(sourceRender, /framework_component/);
  assert.match(sourceRender, /resolved_in_graph": component_names\.contains\(element\.name\.as_str\(\)\) \|\| framework_component/);
  assert.match(sourceRender, /static_html_tag_name\(document, element\)/);
});
