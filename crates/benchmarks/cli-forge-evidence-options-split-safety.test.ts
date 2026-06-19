import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgeEvidenceOptionsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_evidence_options.rs",
);

function nextCommandStart(source, start) {
  const nextCommand = /\n    fn cmd_/g;
  nextCommand.lastIndex = start + 1;
  const match = nextCommand.exec(source);
  return match ? match.index : source.length;
}

test("forge evidence option parsing lives outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(forgeEvidenceOptionsPath),
    "expected dx-www/src/cli/forge_evidence_options.rs",
  );

  const forgeEvidenceOptions = fs.readFileSync(forgeEvidenceOptionsPath, "utf8");
  const evidenceStart = cliMod.indexOf("fn cmd_forge_evidence");
  assert.notEqual(evidenceStart, -1, "expected cmd_forge_evidence in cli module");
  const evidenceCommand = cliMod.slice(evidenceStart, nextCommandStart(cliMod, evidenceStart));

  assert.match(cliMod, /^mod forge_evidence_options;$/m);
  assert.match(
    cliMod,
    /use self::forge_evidence_options::\{\s*parse_forge_evidence_options,\s*DxForgeEvidenceCommandOptions,?\s*\};/s,
  );

  assert.doesNotMatch(evidenceCommand, /let mut project: Option<PathBuf> = None/);
  assert.doesNotMatch(evidenceCommand, /while index < args\.len\(\)/);
  assert.doesNotMatch(evidenceCommand, /Unknown forge evidence option/);
  assert.doesNotMatch(evidenceCommand, /Unexpected forge evidence path/);

  assert.match(
    forgeEvidenceOptions,
    /pub\(super\) struct DxForgeEvidenceCommandOptions/,
  );
  assert.match(
    forgeEvidenceOptions,
    /pub\(super\) fn parse_forge_evidence_options\(/,
  );
  assert.match(forgeEvidenceOptions, /Unknown forge evidence option/);
  assert.match(forgeEvidenceOptions, /Unexpected forge evidence path/);
  assert.match(forgeEvidenceOptions, /mod tests/);
});
