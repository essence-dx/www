import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";
import assert from "node:assert/strict";

const repoRoot = process.cwd();
const cliModPath = join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgeCommandsPath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "mod_parts",
  "cli_forge_commands_c.rs",
);
const optionsPath = join(repoRoot, "dx-www", "src", "cli", "forge_add_options.rs");

const cliMod = readFileSync(cliModPath, "utf8");
const forgeCommands = readFileSync(forgeCommandsPath, "utf8");

function commandBody(name, nextName) {
  const start = forgeCommands.indexOf(`fn ${name}(`);
  assert.notEqual(start, -1, `${name} should exist`);
  const end = forgeCommands.indexOf(`fn ${nextName}(`, start);
  assert.notEqual(end, -1, `${nextName} should follow ${name}`);
  return forgeCommands.slice(start, end);
}

test("dx forge add option parsing is split out of the giant CLI module", () => {
  assert.ok(existsSync(optionsPath), "forge_add_options.rs should own dx forge add parsing");

  assert.match(cliMod, /^mod forge_add_options;$/m);
  assert.match(cliMod, /use self::forge_add_options::\{\s*parse_forge_add_options,\s*DxForgeAddCommandOptions,\s*\};/s);

  const body = commandBody("cmd_forge_add", "cmd_forge_update");
  assert.doesNotMatch(body, /let mut project = self\.cwd\.clone\(\)/);
  assert.doesNotMatch(body, /while index < args\.len\(\)/);
  assert.doesNotMatch(body, /Unknown forge add option/);
  assert.doesNotMatch(body, /Choose either --dry-run or --write, not both/);
  assert.doesNotMatch(body, /--remote-manifest is only valid with --registry r2/);
  assert.match(body, /parse_forge_add_options\(&self\.cwd, args\)\?/);
});

test("dx forge add parser module keeps the existing validation contract", () => {
  const optionsSource = readFileSync(optionsPath, "utf8");

  assert.match(optionsSource, /pub\(super\) struct DxForgeAddCommandOptions/);
  assert.match(optionsSource, /pub\(super\) fn parse_forge_add_options\(/);
  assert.match(optionsSource, /Forge package id required/);
  assert.match(optionsSource, /Unknown forge add option/);
  assert.match(optionsSource, /Choose either --dry-run or --write, not both/);
  assert.match(optionsSource, /--remote-manifest is only valid with --registry r2/);
  assert.match(optionsSource, /mod tests/);
});
