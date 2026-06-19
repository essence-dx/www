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

test("source-build invalidation publishes structured emitted output artifacts", () => {
  const invalidation = read("dx-www/src/build/source_engine/ecosystem_invalidation.rs");
  const rustProof = read("dx-www/tests/source_build_emitted_outputs.rs");

  assert.match(
    invalidation,
    /let emitted_output_artifacts = emitted_output_artifacts\(nodes,\s*edges,\s*&affected_node_ids\);/
  );
  assert.match(invalidation, /"emittedOutputArtifacts": emitted_output_artifacts/);
  assert.match(
    invalidation,
    /fn emitted_output_artifacts\(\s*nodes: &\[Value\],\s*edges: &\[Value\],\s*affected_node_ids: &\[String\],\s*\) -> Vec<Value>/
  );
  assert.match(invalidation, /"edgeKind": kind/);
  assert.match(invalidation, /"sourceNodeId": from/);
  assert.match(invalidation, /"node_modules_required": node\["node_modules_required"\]\.clone\(\)/);
  assert.doesNotMatch(invalidation, /Next\s+DevTools|next-devtools|Turbopack\s+powers/i);

  assert.match(
    rustProof,
    /fn source_build_invalidation_reports_structured_emitted_output_artifacts/
  );
  assert.match(rustProof, /"emittedOutputArtifacts"/);
  assert.match(rustProof, /"dx-style-source-map:\.dx\/build\/styles\/app\.css\.map"/);
  assert.match(rustProof, /"route-shell-chunk:\.dx\/build\/source-routes\/root\/route-shell-/);
});
