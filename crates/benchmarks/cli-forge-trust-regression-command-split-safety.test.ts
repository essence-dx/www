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
  "forge_trust_regression_command.rs",
);

test("Forge trust regression command orchestration lives outside cli mod.rs", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.match(cliMod, /(^|\n)mod forge_trust_regression_command;/);
  assert.match(
    cliMod,
    /"trust-regression"\s*=>\s*\{\s*forge_trust_regression_command::run_forge_trust_regression\(&self\.cwd,\s*&args\[1\.\.\]\)\s*\}/,
  );
  assert.equal(
    cliMod.includes("fn cmd_forge_trust_regression("),
    false,
    "trust-regression command body should be owned by forge_trust_regression_command.rs",
  );

  const command = fs.readFileSync(commandPath, "utf8");
  for (const required of [
    "pub(super) fn run_forge_trust_regression(cwd: &Path, args: &[String]) -> DxResult<()>",
    "parse_forge_trust_regression_options(cwd, args)?",
    "build_forge_trust_regression_report(&project, fail_under)",
    "forge_trust_regression_terminal(&report)",
    "forge_trust_regression_markdown(&report)",
    "forge_trust_regression_failure_summary(&report)",
    "DX Forge trust-regression score",
    "forge trust-regression",
  ]) {
    assert.match(command, new RegExp(escapeRegExp(required)));
  }
});

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
