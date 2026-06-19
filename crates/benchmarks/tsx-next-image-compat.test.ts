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

test("generic TSX renderer lowers static next/image usage to source-owned img tags", () => {
  const sourceRender = read(sourceRenderPath);

  assert.match(sourceRender, /next_image_component_names/);
  assert.match(sourceRender, /is_next_image_element/);
  assert.match(sourceRender, /next\/image/);
  assert.match(sourceRender, /next-image-static-img/);
  assert.match(sourceRender, /data-dx-framework-component/);
  assert.match(sourceRender, /width/);
  assert.match(sourceRender, /height/);
  assert.match(sourceRender, /loading/);
  assert.match(sourceRender, /decoding/);
  assert.match(sourceRender, /static_html_tag_name\(document, element\)/);
  assert.match(sourceRender, /is_static_renderable_element\(document, element\)/);
});
