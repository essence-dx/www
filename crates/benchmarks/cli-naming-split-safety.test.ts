import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const namingPath = path.join(repoRoot, "dx-www", "src", "cli", "naming.rs");
const generateCommandPath = path.join(repoRoot, "dx-www", "src", "cli", "generate_command.rs");

test("dx new naming helpers live in a focused cli naming module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(fs.existsSync(namingPath), "expected dx-www/src/cli/naming.rs");
  assert.ok(fs.existsSync(generateCommandPath), "expected dx-www/src/cli/generate_command.rs");

  const naming = fs.readFileSync(namingPath, "utf8");
  const generateCommand = fs.readFileSync(generateCommandPath, "utf8");

  assert.match(cliMod, /^mod naming;$/m);
  assert.match(cliMod, /^mod generate_command;$/m);
  assert.match(
    cliMod,
    /use self::naming::\{\s*dx_new_project_name,\s*toml_basic_string_escape\s*\};/s,
  );
  assert.match(generateCommand, /use super::naming::to_pascal_case;/);
  assert.match(generateCommand, /let pascal_name = to_pascal_case\(name\);/);
  assert.doesNotMatch(cliMod, /^fn to_pascal_case\(/m);
  assert.doesNotMatch(cliMod, /^fn dx_new_project_name\(/m);
  assert.doesNotMatch(cliMod, /^fn toml_basic_string_escape\(/m);

  assert.match(naming, /pub\(super\) fn to_pascal_case\(/);
  assert.match(naming, /pub\(super\) fn dx_new_project_name\(/);
  assert.match(naming, /pub\(super\) fn toml_basic_string_escape\(/);
  assert.match(naming, /mod tests/);
});
