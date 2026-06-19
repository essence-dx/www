import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = process.cwd();
const commandPath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_ui_registry_parity.rs",
);
const forgeCommandsPath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "mod_parts",
  "cli_forge_commands_a.rs",
);
const helpTextPath = join(repoRoot, "dx-www", "src", "cli", "help_text.rs");

const source = readFileSync(commandPath, "utf8");
const forgeCommands = readFileSync(forgeCommandsPath, "utf8");
const helpText = readFileSync(helpTextPath, "utf8");

test("forge ui registry parity command records current reference evidence", () => {
  assert.match(source, /dx\.forge\.ui_registry_parity/);
  assert.match(source, /https:\/\/github\.com\/shadcn-ui\/ui/);
  assert.match(source, /ced2a5beb5069e87df5cdc08b1f034a38e8f37a3/);
  assert.match(source, /shadcn@4\.11\.0/);
  assert.match(source, /UPSTREAM_UI_REGISTRY_REPO/);
  assert.match(source, /upstream_reference_package/);
  assert.match(source, /upstream repository HEAD verified through git ls-remote/);
  assert.doesNotMatch(source, /UPSTREAM_SHADCN/);
  assert.match(forgeCommands, /"ui-parity" \| "registry-parity"/);
  assert.doesNotMatch(forgeCommands, /"shadcn-parity"/);
  assert.doesNotMatch(forgeCommands, /"shadcn"/);
  assert.match(forgeCommands, /print_forge_ui_help/);
});

test("forge ui registry parity covers the real upstream capability groups", () => {
  for (const capability of [
    "cli-surface",
    "component-catalog",
    "init-create",
    "add",
    "presets",
    "registry-build",
    "registry-schema",
    "registry-resolution",
    "file-updaters",
    "theming",
    "dependency-policy",
    "icons",
    "mcp",
    "monorepo",
  ]) {
    assert.match(source, new RegExp(capability));
  }
});

test("forge ui registry parity is honest about current score versus target", () => {
  assert.match(source, /target_score: 100/);
  assert.match(source, /current_score/);
  assert.match(source, /not a claim that every reference command is implemented/);
  assert.match(source, /public commands and package ids stay Forge-native/);
  assert.match(source, /curated launch UI catalog/);
  assert.match(source, /broader upstream component registry is not claimed/);
  assert.match(source, /Complete live hosted registry pull materialization from verified manifests/);
  assert.match(source, /current planning records source kinds and refuses hidden network fetches/);
});

test("forge ui registry parity lists the current curated Forge UI catalog", () => {
  for (const packageId of [
    "shadcn/ui/button",
    "shadcn/ui/badge",
    "shadcn/ui/card",
    "shadcn/ui/alert",
    "shadcn/ui/avatar",
    "shadcn/ui/skeleton",
    "shadcn/ui/label",
    "shadcn/ui/separator",
    "shadcn/ui/field",
    "shadcn/ui/item",
    "shadcn/ui/input",
    "shadcn/ui/textarea",
  ]) {
    assert.match(source, new RegExp(packageId.replace("/", "\\/")));
  }

  assert.match(source, /CURATED_UI_PACKAGE_IDS: \[&str; 12\]/);
  assert.match(source, /add_command: format!\("dx add \{public_package_id\} --write"\)/);
  assert.match(source, /source files, receipts, docs, and rollback evidence/);
  assert.doesNotMatch(source, /full upstream registry is implemented/);
});

test("forge ui registry parity keeps shadcn branding out of user-facing report identity", () => {
  assert.doesNotMatch(source, /DX Forge shadcn parity/);
  assert.doesNotMatch(source, /forge shadcn-parity failed/);
  assert.match(source, /DX Forge UI registry parity/);
  assert.match(source, /Upstream provenance package:/);
  assert.match(source, /Upstream provenance repository:/);
  assert.match(source, /Usage: dx forge registry parity/);
  assert.match(helpText, /dx forge ui: source-owned Forge UI capability commands/);
  assert.match(helpText, /Registry-wide scripts can use dx forge registry parity/);
  assert.doesNotMatch(helpText, /dx forge ui-parity/);
  assert.match(helpText, /dx forge registry publish --remote r2 --package <id> --write --yes/);
});
