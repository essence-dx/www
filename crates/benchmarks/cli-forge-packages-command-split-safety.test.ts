import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repo = path.resolve(__dirname, "..");
const cliModPath = path.join(repo, "dx-www", "src", "cli", "mod.rs");
const cliForgeCommandPath = path.join(
  repo,
  "dx-www",
  "src",
  "cli",
  "mod_parts",
  "cli_forge_commands_a.rs",
);
const commandPath = path.join(
  repo,
  "dx-www",
  "src",
  "cli",
  "forge_packages_command.rs",
);

test("Forge packages command orchestration and rendering live outside cli mod.rs", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  const cliForgeCommand = fs.readFileSync(cliForgeCommandPath, "utf8");
  assert.match(cliMod, /(^|\n)mod forge_packages_command;/);
  assert.match(
    cliForgeCommand,
    /"packages"\s*\|\s*"package-catalog"\s*=>\s*\{\s*forge_packages_command::run_forge_packages\(&self\.cwd,\s*&args\[1\.\.\]\)\s*\}/,
  );
  for (const forbidden of [
    "fn cmd_forge_packages(",
    "fn forge_packages_terminal(",
    "fn forge_packages_markdown(",
  ]) {
    assert.equal(
      cliMod.includes(forbidden),
      false,
      `${forbidden} should be owned by forge_packages_command.rs`,
    );
  }

  const command = fs.readFileSync(commandPath, "utf8");
  for (const required of [
    "pub(super) fn run_forge_packages(cwd: &Path, args: &[String]) -> DxResult<()>",
    "parse_forge_packages_options(cwd, args)?",
    "launch_discovery_contract()",
    "www_template_catalog_metadata()",
    "public_forge_package_id(canonical_package_id)",
    "forge_package_discovery_public_api(canonical_package_id)",
    "forge_packages_terminal(&report)",
    "forge_packages_markdown(&report)",
    "DX Forge packages",
    "# DX Forge Packages",
  ]) {
    assert.match(command, new RegExp(escapeRegExp(required)));
  }
  assert.equal(
    command.includes('"package_id": canonical_package_id'),
    false,
    "dx forge packages should expose Forge-native public package ids",
  );
});

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
