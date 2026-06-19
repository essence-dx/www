import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgeBetaOptionsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_beta_options.rs",
);

test("forge beta option parsing lives outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(forgeBetaOptionsPath),
    "expected dx-www/src/cli/forge_beta_options.rs",
  );

  const forgeBetaOptions = fs.readFileSync(forgeBetaOptionsPath, "utf8");
  const betaInstallStart = cliMod.indexOf("fn cmd_forge_beta_install");
  const smokeStart = cliMod.indexOf("fn cmd_forge_smoke", betaInstallStart);
  const betaUpgradeStart = cliMod.indexOf("fn cmd_forge_beta_upgrade_smoke");
  const registryStart = cliMod.indexOf("fn cmd_forge_registry", betaUpgradeStart);

  assert.notEqual(
    betaInstallStart,
    -1,
    "expected cmd_forge_beta_install in cli module",
  );
  assert.notEqual(
    smokeStart,
    -1,
    "expected forge smoke command after beta-install",
  );
  assert.notEqual(
    betaUpgradeStart,
    -1,
    "expected cmd_forge_beta_upgrade_smoke in cli module",
  );
  assert.notEqual(
    registryStart,
    -1,
    "expected forge registry command after beta-upgrade-smoke",
  );

  const betaInstallCommand = cliMod.slice(betaInstallStart, smokeStart);
  const betaUpgradeCommand = cliMod.slice(betaUpgradeStart, registryStart);

  assert.match(cliMod, /^mod forge_beta_options;$/m);
  assert.match(
    cliMod,
    /use self::forge_beta_options::\{\s*parse_forge_beta_install_options,\s*parse_forge_beta_upgrade_smoke_options,\s*DxForgeBetaInstallCommandOptions,\s*DxForgeBetaUpgradeSmokeCommandOptions,?\s*\};/s,
  );

  for (const [name, command] of [
    ["beta-install", betaInstallCommand],
    ["beta-upgrade-smoke", betaUpgradeCommand],
  ]) {
    assert.doesNotMatch(command, /let mut project: Option<PathBuf> = None/, name);
    assert.doesNotMatch(command, /while index < args\.len\(\)/, name);
    assert.doesNotMatch(command, new RegExp(`Unknown forge ${name} option`));
    assert.doesNotMatch(command, new RegExp(`Unexpected forge ${name} path`));
    assert.doesNotMatch(
      command,
      /Choose either --dry-run or --write, not both/,
      name,
    );
  }

  assert.match(
    forgeBetaOptions,
    /pub\(super\) struct DxForgeBetaInstallCommandOptions/,
  );
  assert.match(
    forgeBetaOptions,
    /pub\(super\) struct DxForgeBetaUpgradeSmokeCommandOptions/,
  );
  assert.match(
    forgeBetaOptions,
    /pub\(super\) fn parse_forge_beta_install_options\(/,
  );
  assert.match(
    forgeBetaOptions,
    /pub\(super\) fn parse_forge_beta_upgrade_smoke_options\(/,
  );
  assert.match(forgeBetaOptions, /Unknown forge beta-install option/);
  assert.match(forgeBetaOptions, /Unknown forge beta-upgrade-smoke option/);
  assert.match(forgeBetaOptions, /Choose either --dry-run or --write, not both/);
  assert.match(forgeBetaOptions, /mod tests/);
});
