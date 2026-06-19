import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";
import assert from "node:assert/strict";

const repoRoot = process.cwd();
const cliModPath = join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const optionsPath = join(repoRoot, "dx-www", "src", "cli", "forge_audit_options.rs");

const cliMod = readFileSync(cliModPath, "utf8");

function commandBody(name, nextName) {
  const start = cliMod.indexOf(`fn ${name}(`);
  assert.notEqual(start, -1, `${name} should exist`);
  const end = cliMod.indexOf(`fn ${nextName}(`, start);
  assert.notEqual(end, -1, `${nextName} should follow ${name}`);
  return cliMod.slice(start, end);
}

test("dx forge audit option parsing is split out of the giant CLI module", () => {
  assert.ok(existsSync(optionsPath), "forge_audit_options.rs should own dx forge audit parsing");

  assert.match(cliMod, /^mod forge_audit_options;$/m);
  assert.match(cliMod, /use self::forge_audit_options::\{\s*parse_forge_audit_options,\s*DxForgeAuditCommandOptions,\s*\};/s);

  const body = commandBody("cmd_forge_audit", "cmd_forge_add");
  assert.doesNotMatch(body, /let mut path: Option<PathBuf> = None/);
  assert.doesNotMatch(body, /while index < args\.len\(\)/);
  assert.doesNotMatch(body, /Unknown forge audit option/);
  assert.doesNotMatch(body, /Unexpected extra path/);
  assert.doesNotMatch(body, /Invalid fail-under score/);
  assert.match(body, /parse_forge_audit_options\(&self\.cwd, args\)\?/);
});

test("dx forge audit parser module keeps the existing validation contract", () => {
  const optionsSource = readFileSync(optionsPath, "utf8");

  assert.match(optionsSource, /pub\(super\) struct DxForgeAuditCommandOptions/);
  assert.match(optionsSource, /pub\(super\) fn parse_forge_audit_options\(/);
  assert.match(optionsSource, /--format requires a value/);
  assert.match(optionsSource, /--fail-under requires a score/);
  assert.match(optionsSource, /Unknown forge audit option/);
  assert.match(optionsSource, /Unexpected extra path/);
  assert.match(optionsSource, /mod tests/);
});
