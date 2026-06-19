import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("source-build graph treats CSS source maps as first-class emitted artifacts", () => {
  const ecosystemGraph = read("dx-www/src/build/source_engine/ecosystem_graph.rs");
  const rustProof = read("dx-www/tests/source_build_engine.rs");

  assert.match(ecosystemGraph, /"kind":\s*"dx-style-source-map"/);
  assert.match(ecosystemGraph, /"kind":\s*"emits-css-source-map"/);
  assert.match(
    ecosystemGraph,
    /node_id\("dx-style-source-map",\s*source_map_output\)/
  );
  assert.match(ecosystemGraph, /"source_map_hash":\s*style\.source_map_hash\.clone\(\)/);
  assert.match(ecosystemGraph, /"node_modules_required":\s*style\.node_modules_required/);
  assert.match(ecosystemGraph, /"source_owned_contract":\s*style\.source_owned_contract/);
  assert.doesNotMatch(ecosystemGraph, /Next\s+DevTools|next-devtools|Turbopack\s+powers/i);

  assert.match(rustProof, /fn source_build_engine_graphs_css_source_map_artifacts/);
  assert.match(rustProof, /"dx-style-source-map:\.dx\/build\/styles\/app\.css\.map"/);
  assert.match(rustProof, /"emits-css-source-map"/);
});
