import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.join(__dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const launchPagePath = path.join(repoRoot, "dx-www", "src", "cli", "forge_launch_page.rs");

test("Forge launch page command wrapper lives outside cli mod.rs", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  const commandStart = cliMod.indexOf("fn cmd_forge_launch_page");
  const nextCommandStart = cliMod.indexOf("fn cmd_forge_docs", commandStart);
  assert.notEqual(commandStart, -1, "expected launch page command in cli module");
  assert.notEqual(nextCommandStart, -1, "expected forge docs command after launch page");
  const commandBlock = cliMod.slice(commandStart, nextCommandStart);

  assert.ok(fs.existsSync(launchPagePath), "expected dx-www/src/cli/forge_launch_page.rs");
  const launchPage = fs.readFileSync(launchPagePath, "utf8");

  assert.match(cliMod, /^mod forge_launch_page;$/m);
  assert.match(
    cliMod,
    /use forge_launch_page::cmd_forge_launch_page as run_forge_launch_page_command;/,
  );
  assert.match(commandBlock, /run_forge_launch_page_command\(&self\.cwd, args\)/);

  assert.doesNotMatch(commandBlock, /let mut project: Option<PathBuf> = None/);
  assert.doesNotMatch(commandBlock, /Unknown forge launch-page option/);
  assert.doesNotMatch(commandBlock, /Unexpected forge launch-page path/);
  assert.doesNotMatch(commandBlock, /Choose either --dry-run or --write, not both/);
  assert.doesNotMatch(commandBlock, /let mut prove_args = vec!/);
  assert.doesNotMatch(commandBlock, /Cli::with_cwd\(project\)\.cmd_prove/);

  assert.match(launchPage, /pub\(super\) fn cmd_forge_launch_page\(/);
  assert.match(launchPage, /Unknown forge launch-page option/);
  assert.match(launchPage, /Unexpected forge launch-page path/);
  assert.match(launchPage, /Choose either --dry-run or --write, not both/);
  assert.match(launchPage, /"forge-site"\.to_string\(\)/);
  assert.match(launchPage, /Cli::with_cwd\(project\)\.cmd_prove\(&prove_args\)/);
});
