import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgeSmokeOptionsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_smoke_options.rs",
);

test("forge smoke option parsing lives outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(forgeSmokeOptionsPath),
    "expected dx-www/src/cli/forge_smoke_options.rs",
  );

  const forgeSmokeOptions = fs.readFileSync(forgeSmokeOptionsPath, "utf8");
  const smokeStart = cliMod.indexOf("fn cmd_forge_smoke");
  const badgeStart = cliMod.indexOf("fn cmd_forge_badge", smokeStart);
  const ciStart = cliMod.indexOf("fn cmd_forge_ci", badgeStart);
  const ciSnippetsStart = cliMod.indexOf("fn cmd_forge_ci_snippets", ciStart);

  assert.notEqual(smokeStart, -1, "expected cmd_forge_smoke in cli module");
  assert.notEqual(badgeStart, -1, "expected cmd_forge_badge after smoke");
  assert.notEqual(ciStart, -1, "expected cmd_forge_ci after badge");
  assert.notEqual(
    ciSnippetsStart,
    -1,
    "expected cmd_forge_ci_snippets after forge ci",
  );

  const commandSlices = [
    ["smoke", cliMod.slice(smokeStart, badgeStart)],
    ["badge", cliMod.slice(badgeStart, ciStart)],
    ["ci", cliMod.slice(ciStart, ciSnippetsStart)],
  ];

  assert.match(cliMod, /^mod forge_smoke_options;$/m);
  assert.match(
    cliMod,
    /use self::forge_smoke_options::\{\s*parse_forge_badge_options,\s*parse_forge_ci_options,\s*parse_forge_smoke_options,\s*DxForgeBadgeCommandOptions,\s*DxForgeCiCommandOptions,\s*DxForgeSmokeCommandOptions,?\s*\};/s,
  );

  for (const [name, command] of commandSlices) {
    assert.doesNotMatch(command, /let mut project: Option<PathBuf> = None/, name);
    assert.doesNotMatch(command, /while index < args\.len\(\)/, name);
    assert.doesNotMatch(command, new RegExp(`Unknown forge ${name} option`));
    assert.doesNotMatch(command, new RegExp(`Unexpected forge ${name} path`));
  }

  assert.match(
    forgeSmokeOptions,
    /pub\(super\) struct DxForgeSmokeCommandOptions/,
  );
  assert.match(
    forgeSmokeOptions,
    /pub\(super\) struct DxForgeBadgeCommandOptions/,
  );
  assert.match(
    forgeSmokeOptions,
    /pub\(super\) struct DxForgeCiCommandOptions/,
  );
  assert.match(forgeSmokeOptions, /pub\(super\) fn parse_forge_smoke_options\(/);
  assert.match(forgeSmokeOptions, /pub\(super\) fn parse_forge_badge_options\(/);
  assert.match(forgeSmokeOptions, /pub\(super\) fn parse_forge_ci_options\(/);
  assert.match(forgeSmokeOptions, /Unknown forge smoke option/);
  assert.match(forgeSmokeOptions, /Unknown forge badge option/);
  assert.match(forgeSmokeOptions, /Unknown forge ci option/);
  assert.match(
    forgeSmokeOptions,
    /--verify-artifacts and --verify-pages cannot be used together/,
  );
  assert.match(forgeSmokeOptions, /mod tests/);
});
