import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");

function read(relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("generic TSX renderer lowers static next/script usage to source-owned script tags", () => {
  const sourceRender = read("dx-www/src/cli/app_router_execution/source_render.rs");
  const staticMarkup = read(
    "dx-www/src/cli/app_router_execution/source_render_parts/static_markup.rs",
  );
  const staticExpression = read(
    "dx-www/src/cli/app_router_execution/source_render_parts/static_expression.rs",
  );

  assert.match(sourceRender, /next_script_component_names/);
  assert.match(sourceRender, /is_next_script_element/);
  assert.match(sourceRender, /next\/script/);
  assert.match(sourceRender, /next-script-static-script/);
  assert.match(sourceRender, /"script"/);
  assert.match(sourceRender, /crossOrigin.*crossorigin/s);
  assert.match(sourceRender, /referrerPolicy.*referrerpolicy/s);
  assert.match(sourceRender, /noModule.*nomodule/s);

  assert.match(staticMarkup, /apply_next_script_static_attributes/);
  assert.match(staticExpression, /data-dx-next-script-strategy/);
  assert.match(staticExpression, /next-script-static-script-metadata/);
  assert.match(staticExpression, /attributes\.retain\(\|attribute\| !attribute\.starts_with\("strategy="\)\)/);
  assert.match(staticExpression, /strategy != "beforeInteractive"/);
});
