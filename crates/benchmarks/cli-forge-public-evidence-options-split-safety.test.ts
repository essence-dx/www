import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgePublicEvidencePath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_public_evidence.rs",
);
const forgePublicEvidenceOptionsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_public_evidence_options.rs",
);

test("forge public-evidence command and option parsing live outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(forgePublicEvidencePath),
    "expected dx-www/src/cli/forge_public_evidence.rs",
  );
  assert.ok(
    fs.existsSync(forgePublicEvidenceOptionsPath),
    "expected dx-www/src/cli/forge_public_evidence_options.rs",
  );

  const forgePublicEvidence = fs.readFileSync(forgePublicEvidencePath, "utf8");
  const forgePublicEvidenceOptions = fs.readFileSync(
    forgePublicEvidenceOptionsPath,
    "utf8",
  );

  assert.match(cliMod, /^mod forge_public_evidence_options;$/m);
  assert.match(
    cliMod,
    /use forge_public_evidence::\{\s*DxForgePublicEvidenceReport,\s*build_forge_public_evidence_report,\s*run_forge_public_evidence,\s*verify_forge_public_evidence_export,?\s*\};/s,
  );
  assert.doesNotMatch(
    cliMod,
    /fn cmd_forge_public_evidence\(/,
    "cli module should not own forge public-evidence command execution",
  );
  assert.doesNotMatch(
    cliMod,
    /parse_forge_public_evidence_options/,
    "cli module should not parse forge public-evidence options inline",
  );
  assert.match(
    cliMod,
    /"public-evidence"\s*=>\s*run_forge_public_evidence\(&self\.cwd,\s*&args\[1\.\.\]\)/,
  );

  assert.match(
    forgePublicEvidence,
    /pub\(super\) fn run_forge_public_evidence\(/,
  );
  assert.match(
    forgePublicEvidence,
    /parse_forge_public_evidence_options\(cwd, args\)/,
  );
  assert.match(
    forgePublicEvidence,
    /--project cannot be used with --verify/,
  );
  assert.match(
    forgePublicEvidence,
    /Forge public evidence score \{\} is below required threshold \{minimum\}/,
  );
  assert.match(
    forgePublicEvidence,
    /forge_public_evidence_verification_failure_summary\(&report\)/,
  );

  assert.match(
    forgePublicEvidenceOptions,
    /pub\(super\) struct DxForgePublicEvidenceCommandOptions/,
  );
  assert.match(
    forgePublicEvidenceOptions,
    /pub\(super\) fn parse_forge_public_evidence_options\(/,
  );
  assert.match(
    forgePublicEvidenceOptions,
    /Unknown forge public-evidence option/,
  );
  assert.match(
    forgePublicEvidenceOptions,
    /Unexpected forge public-evidence path/,
  );
  assert.match(forgePublicEvidenceOptions, /Invalid fail-under score/);
  assert.match(forgePublicEvidenceOptions, /mod tests/);
});
