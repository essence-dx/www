import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";
import assert from "node:assert/strict";

const repoRoot = process.cwd();
const cliModPath = join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const optionsPath = join(repoRoot, "dx-www", "src", "cli", "forge_publish_options.rs");

const cliMod = readFileSync(cliModPath, "utf8");

function commandBody(name, nextName) {
  const start = cliMod.indexOf(`fn ${name}(`);
  assert.notEqual(start, -1, `${name} should exist`);
  const end = cliMod.indexOf(`fn ${nextName}(`, start);
  assert.notEqual(end, -1, `${nextName} should follow ${name}`);
  return cliMod.slice(start, end);
}

test("dx forge publish option parsing is split out of the giant CLI module", () => {
  assert.ok(existsSync(optionsPath), "forge_publish_options.rs should own dx forge publish parsing");

  assert.match(cliMod, /^mod forge_publish_options;$/m);
  assert.match(cliMod, /use self::forge_publish_options::\{\s*parse_forge_publish_options,\s*DxForgePublishCommandOptions,\s*\};/s);

  const body = commandBody("cmd_forge_public_publish", "cmd_forge_remote_head");
  assert.doesNotMatch(body, /let mut registry = "local"\.to_string\(\)/);
  assert.doesNotMatch(body, /while index < args\.len\(\)/);
  assert.doesNotMatch(body, /Unknown forge publish option/);
  assert.doesNotMatch(body, /Unexpected extra package id/);
  assert.doesNotMatch(body, /Choose either --dry-run or --write, not both/);
  assert.match(body, /parse_forge_publish_options\(&self\.cwd, args\)\?/);
});

test("dx forge publish parser module keeps the existing validation contract", () => {
  const optionsSource = readFileSync(optionsPath, "utf8");

  assert.match(optionsSource, /pub\(super\) struct DxForgePublishCommandOptions/);
  assert.match(optionsSource, /pub\(super\) fn parse_forge_publish_options\(/);
  assert.match(optionsSource, /--registry requires local or r2/);
  assert.match(optionsSource, /--package requires a package id/);
  assert.match(optionsSource, /--local requires a path/);
  assert.match(optionsSource, /--format requires terminal, json, or markdown/);
  assert.match(optionsSource, /Unknown forge publish option/);
  assert.match(optionsSource, /Unexpected extra package id/);
  assert.match(optionsSource, /Choose either --dry-run or --write, not both/);
  assert.match(optionsSource, /mod tests/);
});
