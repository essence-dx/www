import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";
import assert from "node:assert/strict";

const repoRoot = process.cwd();
const cliModPath = join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const commandPath = join(repoRoot, "dx-www", "src", "cli", "migrate_command.rs");
const commandOutputPath = join(repoRoot, "dx-www", "src", "cli", "command_output.rs");
const optionsPath = join(repoRoot, "dx-www", "src", "cli", "migrate_options.rs");

const cliMod = readFileSync(cliModPath, "utf8");

function commandBody(name, nextName) {
  const start = cliMod.indexOf(`pub fn ${name}(`);
  assert.notEqual(start, -1, `${name} should exist`);
  const end = cliMod.indexOf(`pub fn ${nextName}(`, start);
  assert.notEqual(end, -1, `${nextName} should follow ${name}`);
  return cliMod.slice(start, end);
}

test("dx migrate command execution and option parsing are split out of the giant CLI module", () => {
  assert.ok(existsSync(commandPath), "migrate_command.rs should own dx migrate execution");
  assert.ok(existsSync(commandOutputPath), "command_output.rs should own shared command output writes");
  assert.ok(existsSync(optionsPath), "migrate_options.rs should own dx migrate parsing");

  assert.match(cliMod, /^mod command_output;$/m);
  assert.match(cliMod, /^mod migrate_command;$/m);
  assert.match(cliMod, /^mod migrate_options;$/m);
  assert.match(cliMod, /use self::migrate_command::cmd_migrate;/);
  assert.doesNotMatch(cliMod, /use self::migrate_options::/);

  const body = commandBody("cmd_migrate", "cmd_new");
  assert.match(body, /cmd_migrate\(&self\.cwd, args\)/);
  assert.doesNotMatch(body, /let source = args\.first/);
  assert.doesNotMatch(body, /let mut project: Option<PathBuf> = None/);
  assert.doesNotMatch(body, /while index < args\.len\(\)/);
  assert.doesNotMatch(body, /Unsupported migration source/);
  assert.doesNotMatch(body, /Unknown dx migrate/);
  assert.doesNotMatch(body, /requires --plan/);
  assert.doesNotMatch(body, /parse_migrate_options\(&self\.cwd, args\)\?/);
  assert.doesNotMatch(body, /build_next_migration_plan_report/);
  assert.doesNotMatch(body, /build_react_migration_plan_report/);
  assert.doesNotMatch(body, /std::fs::write/);
});

test("dx migrate command module keeps execution and rendering behavior", () => {
  const commandSource = readFileSync(commandPath, "utf8");

  assert.match(commandSource, /pub\(super\) fn cmd_migrate\(cwd: &Path, args: &\[String\]\) -> DxResult<\(\)>/);
  assert.match(commandSource, /use super::command_output::write_rendered_report;/);
  const migrateOptionsImport = commandSource.match(/use super::migrate_options::\{([\s\S]*?)\};/);
  assert.ok(migrateOptionsImport, "migrate command should import migrate options");
  assert.match(migrateOptionsImport[1], /\bparse_migrate_options\b/);
  assert.match(migrateOptionsImport[1], /\bDxMigrateCommandOptions\b/);
  assert.match(migrateOptionsImport[1], /\bDxMigrateSource\b/);
  assert.match(commandSource, /parse_migrate_options\(cwd, args\)\?/);
  assert.match(commandSource, /build_next_migration_plan_report/);
  assert.match(commandSource, /build_recursive_react_migration_plan_report/);
  assert.match(commandSource, /build_react_migration_plan_report/);
  assert.match(commandSource, /write_rendered_report\(output, &rendered, quiet, "migrate"\)\?/);
  assert.doesNotMatch(commandSource, /std::fs::write\(&output, &rendered\)/);
  assert.match(commandSource, /score < fail_under/);
});

test("shared command output helper keeps lifecycle writes contextual", () => {
  const commandOutput = readFileSync(commandOutputPath, "utf8");

  assert.match(commandOutput, /pub\(super\) fn write_rendered_report\(/);
  assert.match(commandOutput, /std::fs::create_dir_all\(parent\)/);
  assert.match(commandOutput, /std::fs::write\(&output, rendered\)/);
  assert.match(commandOutput, /dx \{command\} failed to \{action\}/);
  assert.match(commandOutput, /field: Some\(format!\("\{command\}\.output"\)\)/);
});

test("dx migrate parser module keeps the existing validation contract", () => {
  const optionsSource = readFileSync(optionsPath, "utf8");

  assert.match(optionsSource, /pub\(super\) enum DxMigrateSource/);
  assert.match(optionsSource, /pub\(super\) struct DxMigrateCommandOptions/);
  assert.match(optionsSource, /pub\(super\) fn parse_migrate_options\(/);
  assert.match(optionsSource, /Unsupported migration source/);
  assert.match(optionsSource, /dx migrate next does not support --recursive yet/);
  assert.match(optionsSource, /dx migrate react --web-only requires --recursive/);
  assert.match(optionsSource, /dx migrate .* requires --plan/);
  assert.match(optionsSource, /mod tests/);
});
