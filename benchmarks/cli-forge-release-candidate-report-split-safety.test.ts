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
const commandPath = path.join(
  repo,
  "dx-www",
  "src",
  "cli",
  "forge_release_candidate_command.rs",
);

test("Forge release candidate report model and checks live outside cli mod.rs", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.match(
    cliMod,
    /use self::forge_release_candidate::\{\s*DxForgeReleaseCandidateNoNodeModules,\s*verify_release_candidate_no_node_modules,\s*verify_release_candidate_secret_markers,\s*\};/s,
  );

  for (const forbidden of [
    "struct DxForgeReleaseCandidateReport",
    "struct DxForgeReleaseCandidateChecks",
    "struct DxForgeReleaseCandidateCheck",
    "struct DxForgeReleaseCandidateCiArtifacts",
    "struct DxForgeReleaseCandidatePagesBundle",
    "struct DxForgeReleaseCandidateSourceOwnedReview",
    "struct DxForgeReleaseCandidateStaticEvidence",
    "struct DxForgeReleaseCandidateSecretMarkers",
    "struct DxForgeReleaseCandidateNoNodeModules",
    "fn build_forge_release_candidate_report(",
    "fn verify_release_candidate_source_owned_review(",
    "fn verify_release_candidate_static_evidence(",
    "fn verify_release_candidate_secret_markers(",
    "fn scan_release_candidate_secret_path(",
    "fn verify_release_candidate_no_node_modules(",
    "fn release_candidate_check(",
  ]) {
    assert.equal(
      cliMod.includes(forbidden),
      false,
      `${forbidden} should be owned by forge_release_candidate.rs`,
    );
  }

  const releaseCandidate = fs.readFileSync(releaseCandidatePath, "utf8");
  for (const required of [
    "pub(super) struct DxForgeReleaseCandidateReport",
    "struct DxForgeReleaseCandidateChecks",
    "struct DxForgeReleaseCandidateCheck",
    "struct DxForgeReleaseCandidateCiArtifacts",
    "struct DxForgeReleaseCandidatePagesBundle",
    "struct DxForgeReleaseCandidateSourceOwnedReview",
    "struct DxForgeReleaseCandidateStaticEvidence",
    "pub(super) struct DxForgeReleaseCandidateSecretMarkers",
    "pub(super) struct DxForgeReleaseCandidateNoNodeModules",
    "pub(super) fn build_forge_release_candidate_report(",
    "fn verify_release_candidate_source_owned_review(",
    "fn verify_release_candidate_static_evidence(",
    "pub(super) fn verify_release_candidate_secret_markers(",
    "fn scan_release_candidate_secret_path(",
    "pub(super) fn verify_release_candidate_no_node_modules(",
    "fn release_candidate_check(",
  ]) {
    assert.match(releaseCandidate, new RegExp(escapeRegExp(required)));
  }

  const command = fs.readFileSync(commandPath, "utf8");
  assert.match(
    command,
    /use super::forge_release_candidate::\{\s*build_forge_release_candidate_report,/s,
  );
});

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
