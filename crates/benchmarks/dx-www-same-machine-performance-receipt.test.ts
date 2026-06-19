import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");

function read(relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function expectMarkers(source: string, markers: string[]): void {
  for (const marker of markers) {
    assert.match(source, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
}

test("same-machine performance proof imports target receipts into durable .dx contracts", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");
  const agentContext = read("dx-www/src/cli/agent_context.rs");

  expectMarkers(readiness, [
    "READINESS_SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT",
    "target/framework-comparison-20260531/throughput.json",
    "READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT",
    ".dx/receipts/readiness/same-machine-performance-latest.json",
    "READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_SR",
    ".dx/receipts/readiness/same-machine-performance-latest.sr",
    "READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_MACHINE",
    ".dx/serializer/receipts-readiness-same-machine-performance-latest.machine",
    "READINESS_SAME_MACHINE_PERFORMANCE_IMPORT_COMMAND",
    "dx www readiness --import-same-machine-performance-receipt target/framework-comparison-20260531/throughput.json --json --full",
    "--import-same-machine-performance-receipt",
    "readiness_same_machine_performance_sr_fields",
    "import_readiness_same_machine_performance_receipt",
    "validated-same-machine-performance-current-before-canonical-write",
    "collection_receipt_path",
    "serializer_provenance",
    "release_ready",
    "fastest_world_claim",
    "same-machine throughput raceboard only; not a global speed claim; not Astro payload parity proof; not release-readiness proof",
    "same_machine_first_response_bytes",
    "output_fixtures",
    "first_response_bytes",
    "smallest_first_response_bytes_target",
    "www_first_response_bytes",
  ]);

  expectMarkers(agentContext, [
    "SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT",
    "SAME_MACHINE_PERFORMANCE_RECEIPT_SR",
    "SAME_MACHINE_PERFORMANCE_RECEIPT_MACHINE",
    "SAME_MACHINE_PERFORMANCE_IMPORT_COMMAND",
    "same_machine_performance",
    "readiness-same-machine-performance-machine-contract-missing",
    "Same-machine performance proof is not durable serializer-backed evidence yet",
    "node --test benchmarks/dx-www-same-machine-performance-receipt.test.ts",
    "same_machine_first_response_bytes",
    "first_response_bytes",
  ]);
});

test("same-machine readiness reads the canonical .dx receipt, not the cargo-cleanable target collection path", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");
  const functionBody = readiness.match(
    /fn readiness_same_machine_performance_receipt\(project: &Path\) -> Option<Value> \{([\s\S]*?)\n\}/,
  )?.[1];

  assert.ok(functionBody, "readiness_same_machine_performance_receipt should remain source-visible");
  assert.match(functionBody, /READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT/);
  assert.doesNotMatch(functionBody, /READINESS_SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT/);
  assert.doesNotMatch(functionBody, /\.or_else\(/);
});
