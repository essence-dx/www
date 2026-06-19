import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.join(__dirname, "..");
const ecosystemGraphPath = path.join(
  root,
  "dx-www",
  "src",
  "build",
  "source_engine",
  "ecosystem_graph.rs",
);
const ecosystemInvalidationPath = path.join(
  root,
  "dx-www",
  "src",
  "build",
  "source_engine",
  "ecosystem_invalidation.rs",
);
const nextRustPath = path.join(root, "dx-www", "src", "next_rust.rs");
const rustProofPath = path.join(
  root,
  "dx-www",
  "tests",
  "source_build_graph_source_modules.rs",
);

test("source-build graph exposes source support modules as source-module nodes", () => {
  const ecosystemGraph = fs.readFileSync(ecosystemGraphPath, "utf8");
  const ecosystemInvalidation = fs.readFileSync(ecosystemInvalidationPath, "utf8");
  const nextRust = fs.readFileSync(nextRustPath, "utf8");
  const rustProof = fs.readFileSync(rustProofPath, "utf8");

  assert.match(ecosystemGraph, /"sourceModuleKind":\s*"source-module"/);
  assert.match(ecosystemGraph, /node_id\("source-module",\s*&chunk\.source_path\)/);
  assert.match(ecosystemGraph, /"kind":\s*"source-module"/);
  assert.match(ecosystemGraph, /"kind":\s*"compiled-from-source"/);
  assert.match(ecosystemGraph, /fn is_support_source_module/);
  assert.match(ecosystemInvalidation, /"source-module"/);
  assert.match(nextRust, /"source-module"/);
  assert.match(nextRust, /"compiled-from-source"/);
  assert.match(
    rustProof,
    /fn source_build_graph_exposes_lib_server_src_source_modules/,
  );
});
