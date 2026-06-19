import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const adapterPath = path.join(root, "dx-www", "src", "next_rust_source_map_adapter.rs");
const nextRustPath = path.join(root, "dx-www", "src", "next_rust.rs");
const libPath = path.join(root, "dx-www", "src", "lib.rs");

function read(file) {
  return fs.readFileSync(file, "utf8");
}

test("next-rust source-map reference-only adapter is a DX-owned generated-to-source API", () => {
  assert.ok(fs.existsSync(adapterPath), "missing dx-www/src/next_rust_source_map_adapter.rs");

  const adapter = read(adapterPath);
  assert.match(adapter, /DX_NEXT_RUST_SOURCE_MAP_ADAPTER_SCHEMA/);
  assert.match(adapter, /DX_NEXT_RUST_SOURCE_MAP_ADAPTER_FORMAT/);
  assert.match(adapter, /DxNextRustSourceMapSegment/);
  assert.match(adapter, /DxNextRustSourceMapAdapter/);
  assert.match(adapter, /DxNextRustSourceMapLookup/);
  assert.match(adapter, /DxNextRustSourceMapDiagnosticLocation/);
  assert.match(adapter, /dx_next_rust_source_map_adapter/);
  assert.match(adapter, /dx_next_rust_source_map_lookup/);
  assert.match(adapter, /dx_next_rust_source_map_diagnostic_location/);
  assert.match(adapter, /turbopack\/crates\/turbopack-core/);
  assert.match(adapter, /SourceMap/);
  assert.match(adapter, /reference material only/);
  assert.match(adapter, /adapter_boundary: true/);
  assert.match(adapter, /public_architecture: false/);
  assert.match(adapter, /turbopack_runtime_executed: false/);
  assert.match(adapter, /node_modules_required: false/);
  assert.doesNotMatch(adapter, /turbopack_runtime_executed: true/);
  assert.doesNotMatch(adapter, /node_modules_required: true/);
});

test("next-rust source-map adapter is exported through the stable dx-www API", () => {
  const nextRust = read(nextRustPath);
  const lib = read(libPath);

  assert.match(nextRust, /pub use crate::next_rust_source_map_adapter::\{/);
  assert.match(nextRust, /DxNextRustSourceMapAdapter/);
  assert.match(nextRust, /DxNextRustSourceMapDiagnosticLocation/);
  assert.match(nextRust, /DxNextRustSourceMapLookup/);
  assert.match(nextRust, /dx_next_rust_source_map_diagnostic_location/);
  assert.match(nextRust, /dx_next_rust_source_map_lookup/);

  assert.match(lib, /mod next_rust_source_map_adapter;/);
  assert.match(lib, /DX_NEXT_RUST_SOURCE_MAP_ADAPTER_SCHEMA/);
  assert.match(lib, /DxNextRustSourceMapDiagnosticLocation/);
  assert.match(lib, /DxNextRustSourceMapSegment/);
  assert.match(lib, /dx_next_rust_source_map_adapter/);
  assert.match(lib, /dx_next_rust_source_map_diagnostic_location/);
});

test("source-map adapter has Rust behavior coverage for deterministic lookup", () => {
  const adapter = read(adapterPath);

  assert.match(adapter, /reference material only/);
  assert.match(adapter, /fn source_map_adapter_sorts_and_deduplicates_segments/);
  assert.match(adapter, /fn source_map_lookup_uses_nearest_previous_segment/);
  assert.match(adapter, /fn source_map_lookup_rejects_positions_before_first_segment/);
  assert.match(adapter, /fn source_map_diagnostic_location_carries_generated_and_source_positions/);
});
