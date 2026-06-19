import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgeInitAppOptionsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_init_app_options.rs",
);

test("forge init-app option parsing lives outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(forgeInitAppOptionsPath),
    "expected dx-www/src/cli/forge_init_app_options.rs",
  );

  const forgeInitAppOptions = fs.readFileSync(forgeInitAppOptionsPath, "utf8");
  const initAppStart = cliMod.indexOf("fn cmd_forge_init_app");
  const adoptionSmokeStart = cliMod.indexOf(
    "fn cmd_forge_adoption_smoke",
    initAppStart,
  );

  assert.notEqual(initAppStart, -1, "expected cmd_forge_init_app in cli module");
  assert.notEqual(
    adoptionSmokeStart,
    -1,
    "expected forge adoption-smoke command after init-app",
  );

  const initAppCommand = cliMod.slice(initAppStart, adoptionSmokeStart);

  assert.match(cliMod, /^mod forge_init_app_options;$/m);
  assert.match(
    cliMod,
    /use self::forge_init_app_options::\{\s*parse_forge_init_app_options,\s*DxForgeInitAppCommandOptions,?\s*\};/s,
  );

  assert.doesNotMatch(initAppCommand, /let mut project: Option<PathBuf> = None/);
  assert.doesNotMatch(initAppCommand, /while index < args\.len\(\)/);
  assert.doesNotMatch(initAppCommand, /Unknown forge init-app option/);
  assert.doesNotMatch(initAppCommand, /Unexpected forge init-app path/);

  assert.match(
    forgeInitAppOptions,
    /pub\(super\) struct DxForgeInitAppCommandOptions/,
  );
  assert.match(
    forgeInitAppOptions,
    /pub\(super\) fn parse_forge_init_app_options\(/,
  );
  assert.match(forgeInitAppOptions, /Unknown forge init-app option/);
  assert.match(forgeInitAppOptions, /Unexpected forge init-app path/);
  assert.match(forgeInitAppOptions, /Choose either --dry-run or --write/);
  assert.match(forgeInitAppOptions, /mod tests/);
});
