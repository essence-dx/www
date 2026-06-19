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
  "forge_release_candidate_command.rs",
);

test("Forge release candidate command orchestration lives outside cli mod.rs", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.match(cliMod, /(^|\n)mod forge_release_candidate_command;/);
  assert.match(
    cliMod,
    /"release-candidate"\s*=>\s*\{\s*forge_release_candidate_command::run_forge_release_candidate\(&self\.cwd,\s*&args\[1\.\.\]\)\s*\}/,
  );
  assert.equal(
    cliMod.includes("fn cmd_forge_release_candidate("),
    false,
    "release candidate command body should be owned by forge_release_candidate_command.rs",
  );

  const command = fs.readFileSync(commandPath, "utf8");
  for (const required of [
    "pub(super) fn run_forge_release_candidate(cwd: &Path, args: &[String]) -> DxResult<()>",
    "build_forge_release_candidate_report(",
    "forge_release_candidate_terminal(&report)",
    "forge_release_candidate_markdown(&report)",
    "forge_release_candidate_failure_summary(&report)",
    "Unknown forge release-candidate option:",
    "Unexpected forge release-candidate path:",
    "--source-review requires a JSON report",
    "DX Forge release-candidate score",
  ]) {
    assert.match(command, new RegExp(escapeRegExp(required)));
  }
});

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
