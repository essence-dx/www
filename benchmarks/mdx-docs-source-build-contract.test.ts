import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `missing ${relativePath}`);
  return fs.readFileSync(filePath, "utf8");
}

test("DX source build records docs and MDX as a source-owned adapter boundary", () => {
  const sourceEngine = read("dx-www/src/build/source_engine/mod.rs");
  const content = read("dx-www/src/build/source_engine/content.rs");
  const discovery = read("dx-www/src/build/source_engine/discovery.rs");
  const graph = read("dx-www/src/build/source_engine/graph.rs");
  const receipt = read("dx-www/src/build/source_engine/receipt.rs");
  const ecosystemGraph = read("dx-www/src/build/source_engine/ecosystem_graph.rs");
  const ecosystem = read("dx-www/src/build/source_engine/ecosystem.rs");
  const contentFreshness = read("dx-www/src/build/source_engine/content_freshness.rs");
  const contentReceipt = read("dx-www/src/build/source_engine/content_receipt.rs");
  const readiness = read("dx-www/src/build/source_engine/readiness.rs");
  const handoff = read("dx-www/src/build/source_engine/ecosystem_handoff.rs");

  assert.match(sourceEngine, /mod content;/);
  assert.match(sourceEngine, /mod content_freshness;/);
  assert.match(sourceEngine, /mod content_receipt;/);
  assert.match(sourceEngine, /compile_content_documents/);
  assert.match(sourceEngine, /ContentDocsFreshnessStatus/);
  assert.match(sourceEngine, /evaluate_content_docs_hashes/);
  assert.match(sourceEngine, /content_documents: Vec<SourceBuildContentDocument>/);

  assert.match(discovery, /pub content_documents: Vec<PathBuf>/);
  assert.match(discovery, /collect_content_documents/);
  assert.match(discovery, /&\["md", "mdx"\]/);

  assert.match(content, /pub fn compile_content_documents/);
  assert.match(content, /dx\.www\.mdxCompatibilityOptions/);
  assert.match(content, /turbopack-mdx::MdxTransformOptions/);
  assert.match(content, /provider_import_source: "@mdx-js\/react"/);
  assert.match(content, /provider_import_required: false/);
  assert.match(content, /react_runtime_required: false/);
  assert.match(content, /rsc_required: false/);
  assert.match(content, /full_mdx_pipeline_parity: false/);
  assert.match(content, /runtime_proof: false/);

  assert.match(graph, /pub struct SourceBuildContentDocument/);
  assert.match(graph, /pub content_documents: Vec<SourceBuildContentDocument>/);
  assert.match(graph, /pub mdx_options: Option<SourceBuildMdxCompatibilityOptions>/);
  assert.match(graph, /pub provider_import_required: bool/);
  assert.match(graph, /pub react_runtime_required: bool/);
  assert.match(graph, /pub rsc_required: bool/);
  assert.match(graph, /pub full_mdx_pipeline_parity: bool/);
  assert.match(graph, /content_documents: content_documents\.to_vec\(\)/);

  assert.match(receipt, /pub content_documents: usize/);
  assert.match(receipt, /pub mdx_documents: usize/);
  assert.match(receipt, /dx-source-mdx-docs-adapter/);
  assert.match(receipt, /records-source-owned-mdx-options-without-runtime-execution/);
  assert.match(receipt, /MDX compile\/evaluate remains app-owned/);

  assert.match(ecosystemGraph, /"kind": "dx-content-document"/);
  assert.match(ecosystemGraph, /"contentPipeline"/);
  assert.match(ecosystemGraph, /"runtimeProof": false/);

  assert.match(ecosystem, /pub content_receipt_path: PathBuf/);
  assert.match(ecosystem, /build_receipt_dir\.join\("content-docs\.json"\)/);
  assert.match(ecosystem, /content_receipt::content_docs_receipt/);
  assert.doesNotMatch(ecosystem, /fn content_docs_receipt/);

  assert.match(contentFreshness, /use std::collections::BTreeMap;/);
  assert.match(contentFreshness, /pub struct ContentDocsFreshnessStatus/);
  assert.match(contentFreshness, /pub unsafe_paths: Vec<String>/);
  assert.match(contentFreshness, /pub fn evaluate_content_docs_hashes/);
  assert.match(contentFreshness, /pub fn evaluate_content_docs_receipt_freshness/);
  assert.match(contentFreshness, /pub fn read_content_docs_hash_manifest/);
  assert.match(contentFreshness, /pub fn current_content_doc_hashes/);
  assert.match(contentFreshness, /pub fn content_docs_freshness_summary/);
  assert.match(contentFreshness, /fn safe_project_relative_content_path/);
  assert.match(contentFreshness, /pub\(super\) fn content_document_hashes/);
  assert.match(contentFreshness, /hash_bytes/);
  assert.match(contentFreshness, /CONTENT_DOCS_HASH_ALGORITHM: &str = "blake3-16"/);
  assert.match(contentFreshness, /CONTENT_DOCS_SAFE_PATH_ROOTS/);
  assert.match(contentFreshness, /CONTENT_DOCS_UNSAFE_PATH_POLICY/);
  assert.match(contentFreshness, /"safe_path_roots": CONTENT_DOCS_SAFE_PATH_ROOTS/);
  assert.match(contentFreshness, /"unsafe_path_policy": CONTENT_DOCS_UNSAFE_PATH_POLICY/);

  assert.match(contentReceipt, /use super::content_freshness::/);
  assert.match(contentReceipt, /pub fn content_docs_receipt/);
  assert.doesNotMatch(contentReceipt, /pub struct ContentDocsFreshnessStatus/);
  assert.doesNotMatch(contentReceipt, /pub fn evaluate_content_docs_receipt_freshness/);
  assert.match(contentReceipt, /"schema": "dx\.www\.contentDocsReceipt"/);
  assert.match(contentReceipt, /"reference": "turbopack-mdx"/);
  assert.match(contentReceipt, /"hash_manifest"/);
  assert.match(contentReceipt, /"hash_algorithm": CONTENT_DOCS_HASH_ALGORITHM/);
  assert.match(contentReceipt, /"consumer_snapshot"/);
  assert.match(contentReceipt, /"schema": "dx\.www\.contentDocsConsumerSnapshot"/);
  assert.match(contentReceipt, /"freshness_contract"/);
  assert.match(contentReceipt, /"schema": "dx\.www\.contentDocsFreshnessStatus"/);
  assert.match(contentReceipt, /"evaluator": "evaluate_content_docs_hashes"/);
  assert.match(contentReceipt, /"stale_receipt_policy"/);
  assert.match(contentReceipt, /stale_paths/);
  assert.match(contentReceipt, /missing_paths/);
  assert.match(contentReceipt, /unsafe_paths/);
  assert.match(contentReceipt, /"runtime_proof": false/);
  assert.match(contentReceipt, /"full_mdx_pipeline_parity": false/);

  assert.match(readiness, /content_receipt_path: &Path/);
  assert.match(readiness, /super::content_freshness::content_docs_freshness_summary/);
  assert.match(readiness, /content_docs_freshness_summary/);
  assert.match(readiness, /"content_docs": relative_project_path\(project_root, content_receipt_path\)/);
  assert.match(readiness, /"content_freshness": content_docs_freshness_summary\(manifest\)/);
  assert.match(readiness, /"content_documents": manifest\.content_documents\.len\(\)/);
  assert.match(readiness, /"mdx_documents": mdx_documents/);

  assert.match(handoff, /content_receipt_path: &Path/);
  assert.match(handoff, /super::content_freshness::content_docs_freshness_summary/);
  assert.match(handoff, /content_docs_freshness_summary/);
  assert.match(handoff, /"content_docs_receipt": relative_project_path\(project_root, content_receipt_path\)/);
  assert.match(handoff, /"content_pipeline"/);
  assert.match(handoff, /"freshness": content_docs_freshness_summary\(manifest\)/);
  assert.match(handoff, /"runtime_proof": false/);
});
