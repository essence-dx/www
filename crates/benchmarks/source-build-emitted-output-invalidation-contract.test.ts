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

test("source-build invalidation reports stale emitted output artifacts", () => {
  const invalidation = read("dx-www/src/build/source_engine/ecosystem_invalidation.rs");
  const rustProof = read("dx-www/tests/source_build_engine.rs");

  assert.match(
    invalidation,
    /let emitted_output_node_ids = emitted_output_node_ids\(edges,\s*&affected_node_ids\);/
  );
  assert.match(invalidation, /"emittedOutputNodeIds": emitted_output_node_ids/);
  assert.match(invalidation, /fn is_emitted_output_edge_kind\(kind: &str\) -> bool/);
  assert.match(invalidation, /"emits-css-source-map"/);
  assert.doesNotMatch(invalidation, /Next\s+DevTools|next-devtools|Turbopack\s+powers/i);

  assert.match(
    rustProof,
    /fn source_build_engine_reports_emitted_output_artifacts_for_css_changes/
  );
  assert.match(rustProof, /"emittedOutputNodeIds"/);
  assert.match(rustProof, /"dx-style-source-map:\.dx\/build\/styles\/app\.css\.map"/);
});
