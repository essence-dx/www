import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repo = path.resolve(__dirname, "..");
const cliModPath = path.join(repo, "dx-www", "src", "cli", "mod.rs");
const releaseEvidencePath = path.join(
  repo,
  "dx-www",
  "src",
  "cli",
  "forge_release_proof.rs",
);

test("Forge release proof build and rendering logic lives outside cli mod.rs", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.match(cliMod, /(^|\n)mod forge_release_proof;/);
  assert.match(
    cliMod,
    /use self::forge_release_proof::\{\s*build_forge_release_evidence_report,\s*forge_benchmark_snapshot_markdown,\s*forge_release_evidence_markdown,\s*\};/s,
  );

  for (const forbidden of [
    "fn build_forge_release_evidence_report(",
    "fn forge_release_evidence_markdown(",
    "fn forge_benchmark_snapshot_markdown(",
  ]) {
    assert.equal(
      cliMod.includes(forbidden),
      false,
      `${forbidden} should be owned by forge_release_proof.rs`,
    );
  }

  const releaseEvidence = fs.readFileSync(releaseEvidencePath, "utf8");
  for (const required of [
    "pub(super) fn build_forge_release_evidence_report(",
    "pub(super) fn forge_release_evidence_markdown(",
    "pub(super) fn forge_benchmark_snapshot_markdown(",
    "build_forge_doctor_report(project)?",
    "build_forge_package_scorecard_for_project(project)?",
    "forge_benchmark_snapshot_is_release_ready",
    "forge_package_scorecard_release_ready",
    "# DX Forge Release Proof",
    "Latest Vertical Benchmark",
    "Latest /forge Payload And Browser Timing",
    "No benchmark history index was found",
    "No `/forge` benchmark snapshot was found",
  ]) {
    assert.match(releaseEvidence, new RegExp(escapeRegExp(required)));
  }
});

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
