import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = process.cwd();
const cliModPath = join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgeCommandsPath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "mod_parts",
  "cli_forge_commands_a.rs",
);
const optionsPath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_import_options.rs",
);

const cliMod = readFileSync(cliModPath, "utf8");
const forgeCommands = readFileSync(forgeCommandsPath, "utf8");

function commandBody(name: string, nextName: string) {
  const start = forgeCommands.indexOf(`fn ${name}(`);
  assert.notEqual(start, -1, `${name} should exist`);
  const end = forgeCommands.indexOf(`fn ${nextName}(`, start);
  assert.notEqual(end, -1, `${nextName} should follow ${name}`);
  return forgeCommands.slice(start, end);
}

test("dx forge import option parsing is split out of the giant CLI module", () => {
  assert.ok(
    existsSync(optionsPath),
    "forge_import_options.rs should own dx forge import parsing",
  );

  assert.match(cliMod, /^mod forge_import_options;$/m);
  assert.match(
    cliMod,
    /use forge_import_options::parse_forge_import_options;/,
  );

  const body = commandBody("cmd_forge_import", "cmd_forge_launch_copy_review");
  assert.doesNotMatch(body, /let mut package_name: Option<String>/);
  assert.doesNotMatch(body, /while index < args\.len\(\)/);
  assert.doesNotMatch(body, /Unknown forge import option/);
  assert.doesNotMatch(body, /accepts one package at a time/);
  assert.doesNotMatch(body, /requires --plan or --write/);
  assert.match(body, /parse_forge_import_options\(&self\.cwd, args\)\?/);
});

test("dx forge import parser module keeps the review-gate validation contract", () => {
  const optionsSource = readFileSync(optionsPath, "utf8");

  assert.match(optionsSource, /pub\(super\) struct DxForgeImportCommandOptions/);
  assert.match(optionsSource, /pub\(super\) fn parse_forge_import_options\(/);
  assert.match(optionsSource, /Unsupported Forge import source/);
  assert.match(optionsSource, /dx forge import accepts one package at a time/);
  assert.match(optionsSource, /dx forge import requires --plan or --write/);
  assert.match(optionsSource, /Unknown forge import option/);
  assert.match(optionsSource, /"--source-dir"/);
  assert.match(optionsSource, /"--file"/);
  assert.match(optionsSource, /"--from-plan"/);
  assert.match(optionsSource, /accepted_plan/);
  assert.match(optionsSource, /selected_files/);
  assert.match(optionsSource, /"--json"/);
  assert.match(optionsSource, /fail_under_explicit/);
  assert.match(optionsSource, /mod tests/);
});
