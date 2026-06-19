import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";
import assert from "node:assert/strict";

const repoRoot = process.cwd();
const cliModPath = join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const commandPath = join(repoRoot, "dx-www", "src", "cli", "update_command.rs");
const optionsPath = join(repoRoot, "dx-www", "src", "cli", "update_options.rs");

const cliMod = readFileSync(cliModPath, "utf8");

function commandBody(name, nextName) {
  const start = cliMod.indexOf(`pub fn ${name}(`);
  assert.notEqual(start, -1, `${name} should exist`);
  const end = cliMod.indexOf(`pub fn ${nextName}(`, start);
  assert.notEqual(end, -1, `${nextName} should follow ${name}`);
  return cliMod.slice(start, end);
}

test("dx update command execution and option parsing are split out of the giant CLI module", () => {
  assert.ok(existsSync(commandPath), "update_command.rs should own dx update execution");
  assert.ok(existsSync(optionsPath), "update_options.rs should own dx update parsing");

  assert.match(cliMod, /^mod update_command;$/m);
  assert.match(cliMod, /^mod update_options;$/m);
  const commandImport = cliMod.match(/use self::update_command::\{([\s\S]*?)\};/);
  assert.ok(commandImport, "cli module should import update command helpers");
  assert.match(commandImport[1], /\bcmd_update\b/);
  assert.match(commandImport[1], /\bdefault_update_reviewer\b/);
  assert.doesNotMatch(cliMod, /use self::update_options::/);

  const body = commandBody("cmd_update", "cmd_migrate");
  assert.match(body, /cmd_update\(&self\.cwd, args\)/);
  assert.doesNotMatch(body, /let mut package_id: Option<&str> = None/);
  assert.doesNotMatch(body, /while index < args\.len\(\)/);
  assert.doesNotMatch(body, /Unknown dx update option/);
  assert.doesNotMatch(body, /dx update accepts one source-owned package id at a time/);
  assert.doesNotMatch(body, /Source-owned package id required/);
  assert.doesNotMatch(body, /--accept-yellow requires --write/);
  assert.doesNotMatch(body, /parse_update_options\(&self\.cwd, args\)\?/);
  assert.doesNotMatch(body, /write_forge_update_variant/);
  assert.doesNotMatch(body, /plan_forge_update_variant/);
  assert.doesNotMatch(body, /update_outcome_markdown/);
  assert.doesNotMatch(cliMod, /^fn default_update_reviewer\(\)/m);
});

test("dx update command module keeps execution and output behavior", () => {
  const commandSource = readFileSync(commandPath, "utf8");

  assert.match(commandSource, /pub\(super\) fn cmd_update\(cwd: &Path, args: &\[String\]\) -> DxResult<\(\)>/);
  const updateOptionsImport = commandSource.match(/use super::update_options::\{([\s\S]*?)\};/);
  assert.ok(updateOptionsImport, "update command should import update options");
  assert.match(updateOptionsImport[1], /\bparse_update_options\b/);
  assert.match(updateOptionsImport[1], /\bDxUpdateCommandOptions\b/);
  assert.match(commandSource, /parse_update_options\(cwd, args\)\?/);
  assert.match(commandSource, /write_forge_update_reviewed_variant/);
  assert.match(commandSource, /write_forge_update_variant/);
  assert.match(commandSource, /plan_forge_update_variant/);
  assert.match(commandSource, /update_outcome_markdown/);
  assert.match(commandSource, /pub\(super\) fn default_update_reviewer\(\) -> String/);
  assert.match(commandSource, /DX_FORGE_REVIEWER/);
});

test("dx update parser module keeps the existing validation contract", () => {
  const optionsSource = readFileSync(optionsPath, "utf8");

  assert.match(optionsSource, /pub\(super\) struct DxUpdateCommandOptions/);
  assert.match(optionsSource, /pub\(super\) fn parse_update_options\(/);
  assert.match(optionsSource, /Source-owned package id required/);
  assert.match(optionsSource, /Unknown dx update option/);
  assert.match(optionsSource, /dx update accepts one source-owned package id at a time/);
  assert.match(optionsSource, /Choose either --dry-run or --write, not both/);
  assert.match(optionsSource, /--accept-yellow requires --write/);
  assert.match(optionsSource, /--review-note and --reviewer require --accept-yellow/);
  assert.match(optionsSource, /mod tests/);
});
