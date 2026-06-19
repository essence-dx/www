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
  "forge_release_dashboard_command.rs",
);

test("Forge release dashboard command orchestration lives outside cli mod.rs", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.match(cliMod, /(^|\n)mod forge_release_dashboard_command;/);
  assert.match(
    cliMod,
    /"release-dashboard"\s*=>\s*\{\s*forge_release_dashboard_command::run_forge_release_dashboard\(&self\.cwd,\s*&args\[1\.\.\]\)\s*\}/,
  );
  assert.equal(
    cliMod.includes("fn cmd_forge_release_dashboard("),
    false,
    "release dashboard command body should be owned by forge_release_dashboard_command.rs",
  );

  const command = fs.readFileSync(commandPath, "utf8");
  for (const required of [
    "pub(super) fn run_forge_release_dashboard(cwd: &Path, args: &[String]) -> DxResult<()>",
    "build_forge_release_dashboard_report(",
    "forge_release_dashboard_markdown(&report)",
    "forge_release_dashboard_failure_summary(&report)",
    "Unknown forge release-dashboard option:",
    "Unexpected forge release-dashboard path:",
    "--route-comparison requires a JSON report",
    "DX Forge release-dashboard score",
  ]) {
    assert.match(command, new RegExp(escapeRegExp(required)));
  }
});

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
