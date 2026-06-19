import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = join(import.meta.dirname, "..");

function read(path: string): string {
  return readFileSync(join(repoRoot, path), "utf8");
}

test("media-icon public claims match local receipt evidence boundaries", () => {
  const cargoToml = read("related-crates/media-icon/Cargo.toml");
  const readme = read("related-crates/media-icon/README.md");
  const integrationPlan = read("related-crates/media-icon/INTEGRATION_PLAN.md");
  const lib = read("related-crates/media-icon/src/lib.rs");
  const performance = read("related-crates/media-icon/PERFORMANCE.md");
  const engine = read("related-crates/media-icon/src/engine.rs");
  const gpu = read("related-crates/media-icon/src/gpu.rs");
  const multipattern = read("related-crates/media-icon/src/multipattern.rs");
  const search = read("related-crates/media-icon/src/search.rs");
  const demandingBenchmark = read("related-crates/media-icon/src/bin/demanding_benchmark.rs");
  const queryReceipt = read("benchmarks/media-icon-query-latency-receipt-contract.test.ts");

  const publicSurface = [
    cargoToml,
    readme,
    integrationPlan,
    lib,
    performance,
    engine,
    gpu,
    multipattern,
    search,
    demandingBenchmark,
  ].join("\n");
  const unsupportedClaimPatterns = [
    /\bworld'?s\s+(?:fastest|quickest|best)\b/i,
    /\b(?:fastest|quickest|best)\s+(?:media[- ]icon|icon search|icon cache|svg icon)\b/i,
    /\bfastest\b/i,
    /\b\d+(?:\.\d+)?x\s+faster\b/i,
    /\b\d+(?:\.\d+)?\s*-\s*\d+(?:\.\d+)?x\s+faster\b/i,
    /\b\d+(?:\.\d+)?\s*-\s*\d+(?:\.\d+)?x\s+slower\b/i,
    /\b(?:beats?|outperforms?|surpasses?|is faster than)\b.{0,80}\b(?:upstream|baseline|competitors?|iconify|svgl|lucide)\b/i,
    /\b(?:upstream|baseline|competitors?|iconify|svgl|lucide)\b.{0,80}\b(?:slower|beaten|outperformed|worse)\b/i,
    /world records/i,
    /speed advantage/i,
    /competitor comparison/i,
    /we achieve\s+\d/i,
    /unbeatable search experience/i,
    /sub[- ]?2ms search/i,
    /\b98k searches\/sec\b/i,
    /\b98,783 searches\/sec\b/i,
    /SIMD-accelerated/i,
    /game-changing/i,
    /ultra-performance/i,
    /instant WASM startup/i,
  ];

  for (const unsupportedClaim of unsupportedClaimPatterns) {
    assert.doesNotMatch(
      publicSurface,
      unsupportedClaim,
      `unsupported public media-icon claim should not appear without same-machine upstream receipts: ${unsupportedClaim}`,
    );
  }

  assert.match(cargoToml, /description = "Receipt-backed media-icon cache and search components"/);
  assert.match(lib, /DX media-icon cache and search components/);
  assert.match(lib, /same-machine upstream baseline has not been measured/);
  assert.match(performance, /# Media-Icon Evidence Boundary/);
  assert.match(performance, /same-machine upstream baseline has not been measured/i);
  assert.match(performance, /faster-than-upstream is not claimed/i);
  assert.match(performance, /full startup\/search\/render proof is still open/i);
  assert.match(performance, /Do not reuse DX-WWW runtime scores for media-icon/i);

  assert.match(engine, /"faster_than_upstream_claimed": false/);
  assert.match(engine, /"upstream_baseline_measured": false/);
  assert.match(engine, /"same_machine_benchmark_required": true/);
  assert.match(queryReceipt, /upstream_baseline_measured/);
  assert.match(queryReceipt, /faster_than_upstream_claimed/);
});
