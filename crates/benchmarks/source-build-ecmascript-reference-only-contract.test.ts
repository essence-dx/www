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

test("source-build ECMAScript provenance is reference-only and not Turbopack adoption", () => {
  const graph = read("dx-www/src/build/source_engine/graph.rs");
  const analysis = read("dx-www/src/build/source_engine/ecmascript_analysis.rs");
  const receipt = read("dx-www/src/build/source_engine/receipt.rs");
  const publicFrameworkContract = read("benchmarks/public-framework-contract.test.ts");

  assert.match(graph, /pub reference_only: bool/);
  assert.match(graph, /pub runtime_build_adoption: bool/);
  assert.match(graph, /pub public_runtime_dependency: bool/);
  assert.match(analysis, /reference_only: true/);
  assert.match(analysis, /runtime_build_adoption: false/);
  assert.match(analysis, /public_runtime_dependency: false/);

  assert.match(receipt, /Turbopack ECMAScript reference/);
  assert.doesNotMatch(receipt, /name: "Next\.js Turbopack ECMAScript"/);
  assert.doesNotMatch(publicFrameworkContract, /Next\.js Turbopack ECMAScript/);
});
