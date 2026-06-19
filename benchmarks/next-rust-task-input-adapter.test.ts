import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("next-rust task input reference adapter is DX-owned and reference-only", () => {
  const adapter = read("dx-www/src/next_rust_task_adapter.rs");
  const nextRust = read("dx-www/src/next_rust.rs");
  const lib = read("dx-www/src/lib.rs");

  assert.match(adapter, /DX_NEXT_RUST_TASK_INPUT_ADAPTER_SCHEMA/);
  assert.match(adapter, /dx\.nextRust\.turboTasks\.taskInputAdapter/);
  assert.match(adapter, /pub struct DxNextRustTaskInputAdapter/);
  assert.match(adapter, /pub fn dx_next_rust_turbo_tasks_task_input_adapter\(/);
  assert.match(adapter, /upstream_concept:\s*"TaskInput"/);
  assert.match(adapter, /upstream_crate:\s*"turbopack\/crates\/turbo-tasks"/);
  assert.match(adapter, /adapter_boundary:\s*true/);
  assert.match(adapter, /public_architecture:\s*false/);
  assert.match(adapter, /turbopack_runtime_executed:\s*false/);
  assert.match(adapter, /node_modules_required:\s*false/);
  assert.match(adapter, /blake3::hash/);
  assert.match(adapter, /fn task_input_adapter_sorts_dependencies_before_fingerprinting\(\)/);
  assert.match(adapter, /fn task_input_adapter_fingerprint_changes_when_content_hash_changes\(\)/);

  assert.match(nextRust, /pub use crate::next_rust_task_adapter::/);
  assert.match(lib, /DxNextRustTaskInputAdapter/);
  assert.match(lib, /dx_next_rust_turbo_tasks_task_input_adapter/);
});

test("next-rust task graph reference adapter exposes deterministic invalidation inputs", () => {
  const adapter = read("dx-www/src/next_rust_task_adapter.rs");
  const nextRust = read("dx-www/src/next_rust.rs");
  const lib = read("dx-www/src/lib.rs");

  assert.match(adapter, /DX_NEXT_RUST_TASK_GRAPH_ADAPTER_SCHEMA/);
  assert.match(adapter, /dx\.nextRust\.turboTasks\.taskGraphAdapter/);
  assert.match(adapter, /pub struct DxNextRustTaskGraphNode/);
  assert.match(adapter, /pub struct DxNextRustTaskGraphAdapter/);
  assert.match(adapter, /pub fn dx_next_rust_turbo_tasks_graph_adapter\(/);
  assert.match(adapter, /changed_node_ids:\s*Vec<String>/);
  assert.match(adapter, /unchanged_node_ids:\s*Vec<String>/);
  assert.match(adapter, /dependency_stale_node_ids:\s*Vec<String>/);
  assert.match(adapter, /affected_node_ids:\s*Vec<String>/);
  assert.match(adapter, /graph_fingerprint:\s*String/);
  assert.match(adapter, /BTreeMap/);
  assert.match(adapter, /fn collect_dependency_stale_node_ids\(/);
  assert.match(adapter, /fn collect_affected_node_ids\(/);
  assert.match(adapter, /fn task_graph_adapter_orders_nodes_before_graph_fingerprint\(\)/);
  assert.match(adapter, /fn task_graph_adapter_marks_changed_and_unchanged_nodes\(\)/);
  assert.match(adapter, /fn task_graph_adapter_marks_transitive_dependents_as_stale\(\)/);

  assert.match(nextRust, /pub use crate::next_rust_task_adapter::/);
  assert.match(lib, /DxNextRustTaskGraphAdapter/);
  assert.match(lib, /DxNextRustTaskGraphNode/);
  assert.match(lib, /dx_next_rust_turbo_tasks_graph_adapter/);
});

test("next-rust task adapters stay reference-only instead of runtime adoption", () => {
  const adapter = read("dx-www/src/next_rust_task_adapter.rs");
  const buildClaim = new RegExp("powers " + "the build");

  assert.match(adapter, /reference material only/);
  assert.match(adapter, /adapter_boundary:\s*true/);
  assert.match(adapter, /public_architecture:\s*false/);
  assert.match(adapter, /turbopack_runtime_executed:\s*false/);
  assert.match(adapter, /node_modules_required:\s*false/);
  assert.doesNotMatch(adapter, buildClaim);
  assert.equal(adapter.includes("runtime adoption " + "goal"), false);
  assert.equal(adapter.includes("Turbopack runtime adoption " + "proof"), false);
});
