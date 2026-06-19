import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgeAdoptionOptionsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_adoption_options.rs",
);

test("forge adoption option parsing lives outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(forgeAdoptionOptionsPath),
    "expected dx-www/src/cli/forge_adoption_options.rs",
  );

  const forgeAdoptionOptions = fs.readFileSync(forgeAdoptionOptionsPath, "utf8");
  const adoptionSmokeStart = cliMod.indexOf("fn cmd_forge_adoption_smoke");
  const adoptionReportStart = cliMod.indexOf(
    "fn cmd_forge_adoption_report",
    adoptionSmokeStart,
  );
  const betaInstallStart = cliMod.indexOf(
    "fn cmd_forge_beta_install",
    adoptionReportStart,
  );

  assert.notEqual(
    adoptionSmokeStart,
    -1,
    "expected cmd_forge_adoption_smoke in cli module",
  );
  assert.notEqual(
    adoptionReportStart,
    -1,
    "expected forge adoption-report command after adoption-smoke",
  );
  assert.notEqual(
    betaInstallStart,
    -1,
    "expected forge beta-install command after adoption-report",
  );

  const adoptionSmokeCommand = cliMod.slice(
    adoptionSmokeStart,
    adoptionReportStart,
  );
  const adoptionReportCommand = cliMod.slice(
    adoptionReportStart,
    betaInstallStart,
  );

  assert.match(cliMod, /^mod forge_adoption_options;$/m);
  assert.match(
    cliMod,
    /use self::forge_adoption_options::\{\s*parse_forge_adoption_report_options,\s*parse_forge_adoption_smoke_options,\s*DxForgeAdoptionReportCommandOptions,\s*DxForgeAdoptionSmokeCommandOptions,?\s*\};/s,
  );

  for (const [name, command] of [
    ["adoption-smoke", adoptionSmokeCommand],
    ["adoption-report", adoptionReportCommand],
  ]) {
    assert.doesNotMatch(command, /let mut project: Option<PathBuf> = None/, name);
    assert.doesNotMatch(command, /while index < args\.len\(\)/, name);
    assert.doesNotMatch(command, new RegExp(`Unknown forge ${name} option`));
    assert.doesNotMatch(command, new RegExp(`Unexpected forge ${name} path`));
  }

  assert.match(
    forgeAdoptionOptions,
    /pub\(super\) struct DxForgeAdoptionSmokeCommandOptions/,
  );
  assert.match(
    forgeAdoptionOptions,
    /pub\(super\) struct DxForgeAdoptionReportCommandOptions/,
  );
  assert.match(
    forgeAdoptionOptions,
    /pub\(super\) fn parse_forge_adoption_smoke_options\(/,
  );
  assert.match(
    forgeAdoptionOptions,
    /pub\(super\) fn parse_forge_adoption_report_options\(/,
  );
  assert.match(forgeAdoptionOptions, /Unknown forge adoption-smoke option/);
  assert.match(forgeAdoptionOptions, /Unknown forge adoption-report option/);
  assert.match(forgeAdoptionOptions, /mod tests/);
});
