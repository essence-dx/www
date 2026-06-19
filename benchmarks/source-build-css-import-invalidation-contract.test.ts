import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  const fullPath = path.join(repoRoot, relativePath);
  assert.ok(fs.existsSync(fullPath), `expected ${relativePath} to exist`);
  return fs.readFileSync(fullPath, "utf8");
}

test("source-build graph treats flattened CSS imports as invalidation inputs", () => {
  const ecosystemGraph = read("dx-www/src/build/source_engine/ecosystem_graph.rs");
  const ecosystemInvalidation = read(
    "dx-www/src/build/source_engine/ecosystem_invalidation.rs",
  );
  const rustProof = read("dx-www/tests/source_build_engine.rs");

  assert.match(ecosystemGraph, /"kind":\s*"dx-style-import-source"/);
  assert.match(ecosystemGraph, /"kind":\s*"flattens-css-import"/);
  assert.match(ecosystemGraph, /node_id\("dx-style-import-source",\s*&import\.path\)/);
  assert.match(ecosystemGraph, /"source_role":\s*"flattened-import"/);
  assert.doesNotMatch(ecosystemGraph, /project_root\.join\("node_modules"\)/);

  assert.match(ecosystemInvalidation, /"dx-style-css"/);
  assert.match(
    rustProof,
    /fn source_build_engine_invalidates_css_flattened_import_sources/,
  );
  assert.match(rustProof, /"dx-style-import-source:tokens\/theme\.css"/);
  assert.match(rustProof, /"dx-style-css:styles\/app\.css"/);
  assert.match(rustProof, /"tsx-route:app\/page\.tsx"/);
  assert.match(rustProof, /"flattens-css-import"/);
});
