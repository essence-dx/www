import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repo = path.resolve(__dirname, "..");
const cliModPath = path.join(repo, "dx-www", "src", "cli", "mod.rs");
const releaseCandidatePath = path.join(
  repo,
  "dx-www",
  "src",
  "cli",
  "forge_release_candidate.rs",
);

test("Forge release candidate rendering lives outside cli mod.rs", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.match(cliMod, /(^|\n)mod forge_release_candidate;/);

  for (const forbidden of [
    "fn forge_release_candidate_terminal(",
    "fn forge_release_candidate_markdown(",
    "fn forge_release_candidate_failure_summary(",
  ]) {
    assert.equal(
      cliMod.includes(forbidden),
      false,
      `${forbidden} should be owned by forge_release_candidate.rs`,
    );
  }

  const releaseCandidate = fs.readFileSync(releaseCandidatePath, "utf8");
  for (const required of [
    "pub(super) fn forge_release_candidate_terminal(",
    "pub(super) fn forge_release_candidate_markdown(",
    "pub(super) fn forge_release_candidate_failure_summary(",
    "DX Forge release candidate",
    "# DX Forge Release Candidate Gate",
    "## Checks",
    "## Evidence Inputs",
    "No release-candidate findings for the configured threshold.",
    "DX Forge release-candidate did not pass:",
    "markdown_table_cell",
  ]) {
    assert.match(releaseCandidate, new RegExp(escapeRegExp(required)));
  }
});

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
