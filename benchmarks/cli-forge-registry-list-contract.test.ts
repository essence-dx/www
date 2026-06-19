import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = process.cwd();
const helpTextPath = join(repoRoot, "dx-www", "src", "cli", "help_text.rs");
const registryCommandsPath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "mod_parts",
  "cli_forge_commands_c.rs",
);

const helpText = readFileSync(helpTextPath, "utf8");
const registryCommands = readFileSync(registryCommandsPath, "utf8");

test("forge registry list is a first-class Forge-native discovery command", () => {
  assert.match(helpText, /dx forge registry list \[--file <path>\]/);
  assert.match(helpText, /\[--type <registry:type>\]/);
  assert.match(helpText, /\[--query <text>\]/);
  assert.match(helpText, /List source-owned Forge registry items/);
  assert.match(helpText, /Registry list is discovery; registry plan, parity, and apply receipts are the capability and scoring truth/);
  assert.doesNotMatch(helpText, /shadcn registry list/);

  assert.match(registryCommands, /"list" \| "items" \| "search"/);
  assert.match(registryCommands, /fn cmd_forge_registry_list/);
  assert.match(registryCommands, /forge_ui_registry_list_terminal/);
  assert.match(registryCommands, /forge_ui_registry_list_markdown/);
  assert.match(registryCommands, /no_package_manager_execution/);
  assert.match(registryCommands, /Registry list is discovery; plan, parity, and apply receipts are the capability and scoring truth/);
});
