import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgeProvenanceOptionsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_provenance_options.rs",
);
const forgeProvenanceCommandPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_provenance_command.rs",
);

test("forge provenance option parsing lives outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(forgeProvenanceOptionsPath),
    "expected dx-www/src/cli/forge_provenance_options.rs",
  );
  assert.ok(
    fs.existsSync(forgeProvenanceCommandPath),
    "expected dx-www/src/cli/forge_provenance_command.rs",
  );

  const forgeProvenanceOptions = fs.readFileSync(
    forgeProvenanceOptionsPath,
    "utf8",
  );
  const forgeProvenanceCommand = fs.readFileSync(
    forgeProvenanceCommandPath,
    "utf8",
  );

  assert.match(cliMod, /^mod forge_provenance_options;$/m);
  assert.match(cliMod, /^mod forge_provenance_command;$/m);
  assert.match(
    forgeProvenanceCommand,
    /use super::forge_provenance_options::\{\s*DxForgeProvenanceCommandOptions,\s*parse_forge_provenance_options,?\s*\};/s,
  );
  assert.match(
    cliMod,
    /forge_provenance_command::run_forge_provenance\(&self\.cwd,\s*&args\[1\.\.\]\)/,
  );

  assert.doesNotMatch(cliMod, /fn cmd_forge_provenance/);
  assert.doesNotMatch(forgeProvenanceCommand, /let mut project: Option<PathBuf> = None/);
  assert.doesNotMatch(forgeProvenanceCommand, /while index < args\.len\(\)/);
  assert.doesNotMatch(forgeProvenanceCommand, /Unknown forge provenance option/);
  assert.doesNotMatch(forgeProvenanceCommand, /Unexpected forge provenance path/);

  assert.match(
    forgeProvenanceOptions,
    /pub\(super\) struct DxForgeProvenanceCommandOptions/,
  );
  assert.match(
    forgeProvenanceOptions,
    /pub\(super\) fn parse_forge_provenance_options\(/,
  );
  assert.match(forgeProvenanceOptions, /Unknown forge provenance option/);
  assert.match(forgeProvenanceOptions, /Unexpected forge provenance path/);
  assert.match(forgeProvenanceOptions, /mod tests/);
});
