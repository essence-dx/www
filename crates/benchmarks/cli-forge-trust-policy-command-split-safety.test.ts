import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repo = path.resolve(__dirname, "..");
const cliModPath = path.join(repo, "dx-www", "src", "cli", "mod.rs");
const commandPath = path.join(
  repo,
  "dx-www",
  "src",
  "cli",
  "forge_trust_policy_command.rs",
);

test("Forge trust policy command orchestration lives outside cli mod.rs", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.match(cliMod, /(^|\n)mod forge_trust_policy_command;/);
  assert.match(
    cliMod,
    /"trust-policy"\s*=>\s*\{\s*forge_trust_policy_command::run_forge_trust_policy\(&self\.cwd,\s*&args\[1\.\.\]\)\s*\}/,
  );
  assert.equal(
    cliMod.includes("fn cmd_forge_trust_policy("),
    false,
    "trust-policy command body should be owned by forge_trust_policy_command.rs",
  );

  const command = fs.readFileSync(commandPath, "utf8");
  for (const required of [
    "pub(super) fn run_forge_trust_policy(cwd: &Path, args: &[String]) -> DxResult<()>",
    "parse_forge_trust_policy_options(cwd, args)?",
    "write_forge_trust_policy_file(&project)",
    "build_forge_trust_policy_report(&project)",
    "forge_trust_policy_markdown(&report)",
    "DX Forge trust-policy score",
    "Wrote Forge trust-policy report to",
  ]) {
    assert.match(command, new RegExp(escapeRegExp(required)));
  }
});

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
