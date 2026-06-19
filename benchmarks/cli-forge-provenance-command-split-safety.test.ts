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
  "forge_provenance_command.rs",
);

test("Forge provenance command orchestration lives outside cli mod.rs", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.match(cliMod, /(^|\n)mod forge_provenance_command;/);
  assert.match(
    cliMod,
    /"provenance"\s*=>\s*\{\s*forge_provenance_command::run_forge_provenance\(&self\.cwd,\s*&args\[1\.\.\]\)\s*\}/,
  );
  assert.equal(
    cliMod.includes("fn cmd_forge_provenance("),
    false,
    "provenance command body should be owned by forge_provenance_command.rs",
  );

  const command = fs.readFileSync(commandPath, "utf8");
  for (const required of [
    "pub(super) fn run_forge_provenance(cwd: &Path, args: &[String]) -> DxResult<()>",
    "parse_forge_provenance_options(cwd, args)?",
    "build_forge_provenance_report(&project, fail_under)",
    "forge_provenance_terminal(&report)",
    "forge_provenance_markdown(&report)",
    "forge_provenance_failure_summary(&report)",
    "DX Forge provenance score",
    "forge provenance",
  ]) {
    assert.match(command, new RegExp(escapeRegExp(required)));
  }
});

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
