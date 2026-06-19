import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";
import assert from "node:assert/strict";

const repoRoot = process.cwd();
const cliModPath = join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const optionsPath = join(repoRoot, "dx-www", "src", "cli", "forge_update_options.rs");

const cliMod = readFileSync(cliModPath, "utf8");

function commandBody(name, nextName) {
  const start = cliMod.indexOf(`fn ${name}(`);
  assert.notEqual(start, -1, `${name} should exist`);
  const end = cliMod.indexOf(`fn ${nextName}(`, start);
  assert.notEqual(end, -1, `${nextName} should follow ${name}`);
  return cliMod.slice(start, end);
}

test("dx forge update option parsing is split out of the giant CLI module", () => {
  assert.ok(existsSync(optionsPath), "forge_update_options.rs should own dx forge update parsing");

  assert.match(cliMod, /^mod forge_update_options;$/m);
  assert.match(cliMod, /use self::forge_update_options::\{\s*parse_forge_update_options,\s*DxForgeUpdateCommandOptions,\s*\};/s);

  const body = commandBody("cmd_forge_update", "cmd_forge_public_publish");
  assert.doesNotMatch(body, /let mut package_spec: Option<&str> = None/);
  assert.doesNotMatch(body, /while index < args\.len\(\)/);
  assert.doesNotMatch(body, /Unknown forge update option/);
  assert.doesNotMatch(body, /dx forge update accepts one package spec at a time/);
  assert.doesNotMatch(body, /Choose either --dry-run or --write, not both/);
  assert.doesNotMatch(body, /--accept-yellow requires --write/);
  assert.doesNotMatch(body, /--review-note and --reviewer require --accept-yellow/);
  assert.doesNotMatch(body, /--local requires --registry local/);
  assert.match(body, /parse_forge_update_options\(&self\.cwd, args\)\?/);
});

test("dx forge update parser module keeps the existing validation contract", () => {
  const optionsSource = readFileSync(optionsPath, "utf8");

  assert.match(optionsSource, /pub\(super\) struct DxForgeUpdateCommandOptions/);
  assert.match(optionsSource, /pub\(super\) fn parse_forge_update_options\(/);
  assert.match(optionsSource, /Forge package id required/);
  assert.match(optionsSource, /Unknown forge update option/);
  assert.match(optionsSource, /dx forge update accepts one package spec at a time/);
  assert.match(optionsSource, /Choose either --dry-run or --write, not both/);
  assert.match(optionsSource, /--accept-yellow requires --write/);
  assert.match(optionsSource, /--review-note and --reviewer require --accept-yellow/);
  assert.match(optionsSource, /--local requires --registry local/);
  assert.match(optionsSource, /mod tests/);
});
