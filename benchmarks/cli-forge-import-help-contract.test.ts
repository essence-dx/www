import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = process.cwd();
const helpTextPath = join(repoRoot, "dx-www", "src", "cli", "help_text.rs");
const commandPath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "mod_parts",
  "cli_forge_commands_a.rs",
);
const ecosystemTypesPath = join(
  repoRoot,
  "core",
  "src",
  "ecosystem",
  "forge_importer",
  "types.rs",
);

const helpText = readFileSync(helpTextPath, "utf8");
const commandSource = readFileSync(commandPath, "utf8");
const ecosystemTypesSource = readFileSync(ecosystemTypesPath, "utf8");
const forgeHelpBody = helpText.match(
  /pub\(super\) fn print_forge_help\(\) \{([\s\S]*?)\n\}\s+pub\(super\) fn print_forge_ui_help/,
)?.[1] ?? "";

test("forge top-level help is a professional curated command surface", () => {
  assert.match(forgeHelpBody, /source-owned package firewall, materializer, and registry/);
  assert.match(forgeHelpBody, /USAGE:/);
  assert.match(forgeHelpBody, /REGISTRY:/);
  assert.match(forgeHelpBody, /PROJECT:/);
  assert.match(forgeHelpBody, /IMPORT POLICY:/);
  assert.match(forgeHelpBody, /ADVANCED:/);
  assert.doesNotMatch(forgeHelpBody, /beta-/);
  assert.doesNotMatch(forgeHelpBody, /launch-/);
  assert.doesNotMatch(forgeHelpBody, /smoke/);
  assert.doesNotMatch(forgeHelpBody, /dx forge ui-parity/);
});

test("forge import help advertises plan and write review-gate modes", () => {
  assert.match(
    ecosystemTypesSource,
    /DX_FORGE_IMPORT_ECOSYSTEMS_HELP: &str =\s+"npm\|pip\|cargo\|go\|jsr\|pub\|maven\|nuget\|composer\|gem\|swift\|hex\|cran"/,
  );
  for (const source of [helpText, commandSource]) {
    assert.match(source, /DX_FORGE_IMPORT_ECOSYSTEMS_HELP/);
    assert.match(source, /dx forge review <\{\}> <package> --plan\|--write/);
    assert.match(source, /dx forge import <\{\}> <package> --plan\|--write/);
    assert.match(source, /--source-dir <path>/);
    assert.match(source, /--file <package-path>/);
    assert.match(source, /--from-plan <path>/);
    assert.match(source, /--project <path>/);
    assert.match(source, /--output <path>/);
    assert.match(source, /--format terminal\|json\|markdown/);
    assert.match(source, /--json/);
    assert.match(source, /--fail-under <score>/);
    assert.match(source, /--quiet/);
  }
});

test("forge import help names the no-install ownership boundary", () => {
  assert.match(helpText, /without package installs/);
  assert.match(helpText, /forge import\s+Compatibility alias for forge review/);
  assert.match(commandSource, /"review" \| "source-review" \| "package-review" \| "import"/);
  assert.match(helpText, /Import is a Forge review gate, not a package-manager install/);
  assert.match(helpText, /Ecosystem support means modeled review surfaces, not universal or live package-manager compatibility/);
  assert.match(helpText, /Registry apply receipts mirror JSON, \.sr, and machine artifacts/);
  assert.match(commandSource, /does not run package-manager installs or lifecycle\/setup\/build scripts/);
  assert.match(commandSource, /Ecosystem support means modeled review surfaces, not universal or live package-manager compatibility/);
  assert.match(helpText, /--plan writes evidence receipts only; no app source, node_modules, or package code is created/);
  assert.match(commandSource, /--plan writes evidence receipts only; no app source, node_modules, or package code is created/);
  assert.match(helpText, /--write materializes inspected source directories into source-owned Forge files/);
  assert.match(commandSource, /--write materializes inspected source directories into source-owned Forge files/);
  assert.match(helpText, /clean package-name imports require a compatible reviewed adapter or bridge/);
  assert.match(commandSource, /clean package-name imports require a compatible reviewed adapter or bridge/);
  assert.match(helpText, /materialize a reviewed package-relative source slice/);
  assert.match(commandSource, /materialize a reviewed package-relative source slice/);
  assert.match(helpText, /accepted import plan/);
  assert.match(commandSource, /accepted import plan/);
});

test("forge import help explains disposition outcomes", () => {
  for (const source of [helpText, commandSource]) {
    assert.match(source, /materialize, slice, bridge, or reject/);
    assert.match(source, /reject mode never overwrites different local source/);
    assert.match(source, /Bridge requires adapter\/manual wrapper evidence|bridge requires adapter\/manual wrapper evidence/);
  }
});
