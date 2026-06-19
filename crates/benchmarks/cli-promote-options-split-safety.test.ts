import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const commandOutputPath = path.join(repoRoot, "dx-www", "src", "cli", "command_output.rs");
const promoteCommandPath = path.join(repoRoot, "dx-www", "src", "cli", "promote_command.rs");
const promoteOptionsPath = path.join(repoRoot, "dx-www", "src", "cli", "promote_options.rs");

test("dx promote command execution and option parsing live outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(promoteCommandPath),
    "expected dx-www/src/cli/promote_command.rs",
  );
  assert.ok(
    fs.existsSync(commandOutputPath),
    "expected dx-www/src/cli/command_output.rs",
  );
  assert.ok(
    fs.existsSync(promoteOptionsPath),
    "expected dx-www/src/cli/promote_options.rs",
  );

  const promoteStart = cliMod.indexOf("pub fn cmd_promote");
  const rollbackStart = cliMod.indexOf("/// Verify that a current build can roll back", promoteStart);
  assert.notEqual(promoteStart, -1, "expected cmd_promote in cli module");
  assert.notEqual(rollbackStart, -1, "expected rollback marker after cmd_promote");
  const promoteCommand = cliMod.slice(promoteStart, rollbackStart);

  assert.match(cliMod, /^mod command_output;$/m);
  assert.match(cliMod, /^mod promote_command;$/m);
  assert.match(cliMod, /^mod promote_options;$/m);
  assert.match(cliMod, /use self::promote_command::cmd_promote;/);
  assert.doesNotMatch(cliMod, /use self::promote_options::/);

  assert.match(promoteCommand, /cmd_promote\(&self\.cwd, args\)/);
  assert.doesNotMatch(promoteCommand, /parse_promote_options\(&self\.cwd, args\)\?/);
  assert.doesNotMatch(promoteCommand, /promote_build_manifest_with_local_key/);
  assert.doesNotMatch(promoteCommand, /build_manifest_promotion_terminal/);
  assert.doesNotMatch(promoteCommand, /std::fs::write/);
  assert.doesNotMatch(promoteCommand, /let mut key: Option<PathBuf> = None/);
  assert.doesNotMatch(promoteCommand, /Unknown promote option/);
  assert.doesNotMatch(promoteCommand, /dx promote requires --key <private-key\.json>/);
});

test("dx promote command module keeps execution and reporting behavior", () => {
  const promoteCommand = fs.readFileSync(promoteCommandPath, "utf8");

  assert.match(promoteCommand, /pub\(super\) fn cmd_promote\(cwd: &Path, args: &\[String\]\) -> DxResult<\(\)>/);
  assert.match(promoteCommand, /use super::command_output::write_rendered_report;/);
  const promoteOptionsImport = promoteCommand.match(/use super::promote_options::\{([\s\S]*?)\};/);
  assert.ok(promoteOptionsImport, "promote command should import promote options");
  assert.match(promoteOptionsImport[1], /\bparse_promote_options\b/);
  assert.match(promoteOptionsImport[1], /\bDxPromoteCommandOptions\b/);
  assert.match(promoteCommand, /parse_promote_options\(cwd, args\)\?/);
  assert.match(promoteCommand, /promote_build_manifest_with_local_key/);
  assert.match(promoteCommand, /build_manifest_promotion_terminal/);
  assert.match(promoteCommand, /build_manifest_promotion_markdown/);
  assert.match(promoteCommand, /write_rendered_report\(output, &rendered, quiet, "promote"\)\?/);
  assert.doesNotMatch(promoteCommand, /std::fs::write\(&output, &rendered\)/);
  assert.match(promoteCommand, /build_manifest_promotion_failure_summary/);
});

test("dx promote parser module keeps the existing validation contract", () => {
  const promoteOptions = fs.readFileSync(promoteOptionsPath, "utf8");

  assert.match(promoteOptions, /pub\(super\) struct DxPromoteCommandOptions/);
  assert.match(promoteOptions, /pub\(super\) fn parse_promote_options\(/);
  assert.match(promoteOptions, /Unknown promote option/);
  assert.match(promoteOptions, /dx promote requires --key <private-key\.json>/);
  assert.match(promoteOptions, /mod tests/);
});
