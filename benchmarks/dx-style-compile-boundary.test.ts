import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function readRequiredFile(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `expected ${relativePath} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

test("dx-style public compile boundary keeps receipt APIs reachable under deny warnings", () => {
  const lib = readRequiredFile("related-crates/style/src/lib.rs");
  const core = readRequiredFile("related-crates/style/src/core/mod.rs");
  const cache = readRequiredFile("related-crates/style/src/cache/mod.rs");

  assert.match(lib, /#!\[deny\(warnings\)\]/);
  assert.match(cache, /pub mod registry_snapshot_plan;/);
  assert.match(core, /pub mod engine;/);
  assert.doesNotMatch(core, /\nmod engine;/);
  assert.match(core, /#\[allow\(unused_imports\)\]\s+pub use engine::theme_css::\{/);
  assert.match(core, /theme_layer_css_from_source/);
  assert.match(core, /css_first_directive_diagnostics/);
  assert.match(core, /#\[allow\(unused_imports\)\]\s+pub use engine::StyleEngine;/);
});
