import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const newCommandPath = path.join(repoRoot, "dx-www", "src", "cli", "new_command.rs");

test("dx new command materialization lives outside the giant cli module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  const newCommand = fs.readFileSync(newCommandPath, "utf8");

  assert.match(cliMod, /^mod new_command;$/m);
  assert.match(
    cliMod,
    /pub fn cmd_new\(&self, name: &str\) -> DxResult<\(\)> \{\s*new_command::cmd_new\(&self\.cwd, name\)\s*\}/,
  );
  assert.doesNotMatch(cliMod, /fn write_next_familiar_launch_route\(/);
  assert.doesNotMatch(cliMod, /fn write_launch_forge_package_slices\(/);

  assert.match(newCommand, /pub\(super\) fn cmd_new\(cwd: &Path, name: &str\) -> DxResult<\(\)>/);
  assert.match(newCommand, /struct NewCommand<'a>/);
  assert.match(newCommand, /fn write_next_familiar_launch_route\(/);
  assert.match(newCommand, /fn write_launch_forge_package_slices\(/);
  assert.match(newCommand, /write_default_template_source_files\(&project_dir\)\?/);
  assert.ok(
    newCommand.indexOf("Self::write_next_familiar_launch_route(&project_dir)?") <
      newCommand.indexOf("Self::write_launch_forge_package_slices(&project_dir)?"),
    "dx new must materialize TSX launch route files before Forge package slice writes",
  );
});
