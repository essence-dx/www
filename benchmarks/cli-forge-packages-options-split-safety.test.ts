import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgePackagesOptionsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_packages_options.rs",
);
const forgePackagesCommandPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_packages_command.rs",
);

test("forge packages option parsing lives outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(forgePackagesOptionsPath),
    "expected dx-www/src/cli/forge_packages_options.rs",
  );

  const forgePackagesOptions = fs.readFileSync(forgePackagesOptionsPath, "utf8");
  const forgePackagesCommand = fs.readFileSync(forgePackagesCommandPath, "utf8");

  assert.match(cliMod, /^mod forge_packages_options;$/m);
  assert.match(
    cliMod,
    /^mod forge_packages_command;$/m,
  );
  assert.match(
    cliMod,
    /"packages"\s*\|\s*"package-catalog"\s*=>\s*\{\s*forge_packages_command::run_forge_packages\(&self\.cwd,\s*&args\[1\.\.\]\)\s*\}/,
  );
  assert.equal(
    cliMod.includes("fn cmd_forge_packages("),
    false,
    "packages command body should be owned by forge_packages_command.rs",
  );
  assert.match(
    forgePackagesCommand,
    /use super::forge_packages_options::\{\s*DxForgePackagesCommandOptions,\s*parse_forge_packages_options,?\s*\};/s,
  );

  assert.doesNotMatch(forgePackagesCommand, /let mut output: Option<PathBuf> = None/);
  assert.doesNotMatch(forgePackagesCommand, /while index < args\.len\(\)/);
  assert.doesNotMatch(forgePackagesCommand, /Unknown forge packages option/);
  assert.doesNotMatch(forgePackagesCommand, /Unexpected forge packages argument/);

  assert.match(
    forgePackagesOptions,
    /pub\(super\) struct DxForgePackagesCommandOptions/,
  );
  assert.match(
    forgePackagesOptions,
    /pub\(super\) fn parse_forge_packages_options\(/,
  );
  assert.match(forgePackagesOptions, /Unknown forge packages option/);
  assert.match(forgePackagesOptions, /Unexpected forge packages argument/);
  assert.match(forgePackagesOptions, /mod tests/);
});
