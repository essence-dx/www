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

test("React starter benchmark reports a comparison baseline instead of Next.js parity", () => {
  const benchmark = read("dx-www/src/cli/forge_react_starter_benchmark.rs");
  const cliTests = read("dx-www/src/cli/mod.rs");

  assert.doesNotMatch(benchmark, /nextjs_parity/);
  assert.doesNotMatch(benchmark, /DxReactStarterNextJsParity/);
  assert.doesNotMatch(benchmark, /Next\.js Parity Fixture|Next\.js parity fixture/);
  assert.doesNotMatch(benchmark, /Content parity|content_parity|route-shape parity/);
  assert.doesNotMatch(benchmark, /Next\.js DevTools clone|nextjs_devtools_clone_target/);
  assert.doesNotMatch(benchmark, /nextjs_runtime_adoption/);
  assert.doesNotMatch(benchmark, /Turbopack build\/dev adoption|turbopack_runtime_build_adoption/);

  assert.match(benchmark, /nextjs_baseline/);
  assert.match(benchmark, /DxReactStarterNextJsBaseline/);
  assert.match(benchmark, /Next\.js Comparison Baseline/);
  assert.match(benchmark, /content_match/);
  assert.match(benchmark, /route-shape comparison evidence/);
  assert.match(benchmark, /architecture_boundaries/);
  assert.match(benchmark, /DxReactStarterArchitectureBoundaries/);
  assert.match(benchmark, /dx_owned_www_framework: true/);
  assert.match(benchmark, /next_familiar_authoring: true/);
  assert.match(benchmark, /runtime_core: "dx-owned-rust-wasm"\.to_string\(\)/);
  assert.match(benchmark, /build_engine: "dx-source-build"\.to_string\(\)/);
  assert.match(benchmark, /forge_first_no_node_modules_default: true/);

  assert.match(cliTests, /report\["nextjs_baseline"\]\["baseline_kind"\]/);
  assert.match(
    cliTests,
    /report\["architecture_boundaries"\]\["next_familiar_authoring"\]/,
  );
  assert.doesNotMatch(cliTests, /nextjs_devtools_clone_target/);
  assert.doesNotMatch(cliTests, /nextjs_runtime_adoption/);
  assert.doesNotMatch(cliTests, /turbopack_runtime_build_adoption/);
  assert.doesNotMatch(cliTests, /report\["nextjs_parity"\]/);
});
