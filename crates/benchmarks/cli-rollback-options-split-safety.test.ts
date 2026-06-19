import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const commandOutputPath = path.join(repoRoot, "dx-www", "src", "cli", "command_output.rs");
const rollbackCommandPath = path.join(repoRoot, "dx-www", "src", "cli", "rollback_command.rs");
const rollbackOptionsPath = path.join(repoRoot, "dx-www", "src", "cli", "rollback_options.rs");

test("dx rollback command execution and option parsing live outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(rollbackCommandPath),
    "expected dx-www/src/cli/rollback_command.rs",
  );
  assert.ok(
    fs.existsSync(commandOutputPath),
    "expected dx-www/src/cli/command_output.rs",
  );
  assert.ok(
    fs.existsSync(rollbackOptionsPath),
    "expected dx-www/src/cli/rollback_options.rs",
  );

  const rollbackCommand = cliMod.match(
    /pub fn cmd_rollback\(&self, args: &\[String\]\) -> DxResult<\(\)> \{\s*cmd_rollback\(&self\.cwd, args\)\s*\}/,
  )?.[0];
  assert.ok(rollbackCommand, "expected cmd_rollback wrapper in cli module");

  assert.match(cliMod, /^mod command_output;$/m);
  assert.match(cliMod, /^mod rollback_command;$/m);
  assert.match(cliMod, /^mod rollback_options;$/m);
  assert.match(cliMod, /use self::rollback_command::cmd_rollback;/);
  assert.doesNotMatch(cliMod, /use self::rollback_options::/);

  assert.match(rollbackCommand, /cmd_rollback\(&self\.cwd, args\)/);
  assert.doesNotMatch(rollbackCommand, /cmd_rollback_verify/);
  assert.doesNotMatch(rollbackCommand, /parse_rollback_verify_options\(&self\.cwd, args\)\?/);
  assert.doesNotMatch(rollbackCommand, /verify_build_rollback/);
  assert.doesNotMatch(rollbackCommand, /build_rollback_verification_terminal/);
  assert.doesNotMatch(rollbackCommand, /std::fs::write/);
  assert.doesNotMatch(rollbackCommand, /let mut previous_build_dir: Option<PathBuf> = None/);
  assert.doesNotMatch(rollbackCommand, /Unknown rollback verify option/);
  assert.doesNotMatch(
    rollbackCommand,
    /dx rollback verify requires --previous-build-dir <path>/,
  );
});

test("dx rollback command module keeps verify execution and reporting behavior", () => {
  const rollbackCommand = fs.readFileSync(rollbackCommandPath, "utf8");

  assert.match(rollbackCommand, /pub\(super\) fn cmd_rollback\(cwd: &Path, args: &\[String\]\) -> DxResult<\(\)>/);
  assert.match(rollbackCommand, /fn cmd_rollback_verify\(cwd: &Path, args: &\[String\]\) -> DxResult<\(\)>/);
  assert.match(rollbackCommand, /use super::command_output::write_rendered_report;/);
  const rollbackOptionsImport = rollbackCommand.match(/use super::rollback_options::\{([\s\S]*?)\};/);
  assert.ok(rollbackOptionsImport, "rollback command should import rollback options");
  assert.match(rollbackOptionsImport[1], /\bparse_rollback_verify_options\b/);
  assert.match(rollbackOptionsImport[1], /\bDxRollbackVerifyCommandOptions\b/);
  assert.match(rollbackCommand, /parse_rollback_verify_options\(cwd, args\)\?/);
  assert.match(rollbackCommand, /verify_build_rollback/);
  assert.match(rollbackCommand, /build_rollback_verification_terminal/);
  assert.match(rollbackCommand, /build_rollback_verification_markdown/);
  assert.match(rollbackCommand, /write_rendered_report\(output, &rendered, quiet, "rollback verify"\)\?/);
  assert.doesNotMatch(rollbackCommand, /std::fs::write\(&output, &rendered\)/);
  assert.match(rollbackCommand, /build_rollback_verification_failure_summary/);
  assert.match(rollbackCommand, /Unknown rollback command/);
});

test("dx rollback parser module keeps the existing validation contract", () => {
  const rollbackOptions = fs.readFileSync(rollbackOptionsPath, "utf8");

  assert.match(rollbackOptions, /pub\(super\) struct DxRollbackVerifyCommandOptions/);
  assert.match(rollbackOptions, /pub\(super\) fn parse_rollback_verify_options\(/);
  assert.match(rollbackOptions, /Unknown rollback verify option/);
  assert.match(rollbackOptions, /dx rollback verify requires --previous-build-dir <path>/);
  assert.match(rollbackOptions, /mod tests/);
});
