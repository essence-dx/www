import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";
import assert from "node:assert/strict";

const repoRoot = process.cwd();
const cliModPath = join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const registryCommandsPath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "mod_parts",
  "cli_forge_commands_c.rs",
);
const optionsPath = join(repoRoot, "dx-www", "src", "cli", "forge_registry_options.rs");

const cliMod = readFileSync(cliModPath, "utf8");
const registryCommands = readFileSync(registryCommandsPath, "utf8");

function commandBody(name, nextName) {
  const start = registryCommands.indexOf(`fn ${name}(`);
  assert.notEqual(start, -1, `${name} should exist`);
  const end = registryCommands.indexOf(`fn ${nextName}(`, start);
  assert.notEqual(end, -1, `${nextName} should follow ${name}`);
  return registryCommands.slice(start, end);
}

test("dx forge registry option parsing is split out of the giant CLI module", () => {
  assert.ok(existsSync(optionsPath), "forge_registry_options.rs should own registry parsing");

  assert.match(cliMod, /^mod forge_registry_options;$/m);
  assert.match(cliMod, /use self::forge_registry_options::\{/);
  assert.match(cliMod, /parse_forge_registry_smoke_options/);
  assert.match(cliMod, /parse_forge_registry_plan_options/);
  assert.match(cliMod, /parse_forge_registry_list_options/);
  assert.match(cliMod, /parse_forge_registry_apply_options/);
  assert.match(cliMod, /DxForgeRegistryPublishOptions/);

  const validateBody = commandBody("cmd_forge_registry_validate", "cmd_forge_registry_build");
  const buildBody = commandBody("cmd_forge_registry_build", "cmd_forge_registry_plan");
  const planBody = commandBody("cmd_forge_registry_plan", "cmd_forge_registry_list");
  const listBody = commandBody("cmd_forge_registry_list", "cmd_forge_registry_apply");
  const applyBody = commandBody("cmd_forge_registry_apply", "cmd_forge_registry_init");
  const initBody = commandBody("cmd_forge_registry_init", "cmd_forge_registry_smoke");
  const smokeBody = commandBody("cmd_forge_registry_smoke", "cmd_forge_registry_publish");
  const publishBody = commandBody("cmd_forge_registry_publish", "cmd_forge_registry_pull");
  const pullBody = commandBody("cmd_forge_registry_pull", "cmd_forge_registry_status");
  const statusBody = commandBody("cmd_forge_registry_status", "cmd_check");

  for (const body of [validateBody, buildBody, planBody, listBody, applyBody, initBody, smokeBody, publishBody, pullBody, statusBody]) {
    assert.doesNotMatch(body, /while index < args\.len\(\)/);
    assert.doesNotMatch(body, /let mut index = 0usize/);
  }

  assert.doesNotMatch(validateBody, /Unknown forge registry validate option/);
  assert.doesNotMatch(buildBody, /Unknown forge registry build option/);
  assert.doesNotMatch(planBody, /Unknown forge registry plan option/);
  assert.doesNotMatch(listBody, /Unknown forge registry list option/);
  assert.doesNotMatch(applyBody, /Unknown forge registry apply option/);
  assert.doesNotMatch(initBody, /Unknown forge registry init option/);
  assert.doesNotMatch(smokeBody, /Unknown forge registry smoke option/);
  assert.doesNotMatch(publishBody, /Unknown forge registry publish option/);
  assert.doesNotMatch(pullBody, /Unknown forge registry pull option/);
  assert.doesNotMatch(statusBody, /Unknown forge registry status option/);
  assert.match(validateBody, /parse_forge_registry_validate_options\(&self\.cwd, args\)\?/);
  assert.match(buildBody, /parse_forge_registry_build_options\(&self\.cwd, args\)\?/);
  assert.match(planBody, /parse_forge_registry_plan_options\(&self\.cwd, args\)\?/);
  assert.match(listBody, /parse_forge_registry_list_options\(&self\.cwd, args\)\?/);
  assert.match(listBody, /forge_ui_registry_list_rendered/);
  assert.match(applyBody, /parse_forge_registry_apply_options\(&self\.cwd, args\)\?/);
  assert.match(applyBody, /build_forge_ui_registry_apply_receipt/);
  assert.match(initBody, /parse_forge_registry_init_options\(&self\.cwd, args\)\?/);
  assert.match(smokeBody, /parse_forge_registry_smoke_options\(&self\.cwd, args\)\?/);
  assert.match(publishBody, /parse_forge_registry_publish_options\(args\)\?/);
  assert.match(pullBody, /parse_forge_registry_pull_options\(args\)\?/);
  assert.match(pullBody, /pull_registry_package_from_r2\(\s*&package,\s*&version,\s*dry_run\s*\)/);
  assert.match(statusBody, /parse_forge_registry_status_options\(args\)\?/);
});

test("dx forge registry parser module keeps the existing validation contract", () => {
  const optionsSource = readFileSync(optionsPath, "utf8");

  assert.match(optionsSource, /pub\(super\) struct DxForgeRegistryInitOptions/);
  assert.match(optionsSource, /pub\(super\) struct DxForgeRegistryValidateOptions/);
  assert.match(optionsSource, /pub\(super\) struct DxForgeRegistryBuildOptions/);
  assert.match(optionsSource, /receipt_output: Option<PathBuf>/);
  assert.match(optionsSource, /embed_content: bool/);
  assert.match(optionsSource, /source_root: Option<PathBuf>/);
  assert.match(optionsSource, /pub\(super\) struct DxForgeRegistryPlanOptions/);
  assert.match(optionsSource, /pub\(super\) struct DxForgeRegistryListOptions/);
  assert.match(optionsSource, /parse_forge_registry_list_options/);
  assert.match(optionsSource, /query: Option<String>/);
  assert.match(optionsSource, /item_type: Option<String>/);
  assert.match(optionsSource, /pub\(super\) struct DxForgeRegistryApplyOptions/);
  assert.match(optionsSource, /parse_forge_registry_apply_options/);
  assert.match(optionsSource, /receipt_output: Option<PathBuf>/);
  assert.match(optionsSource, /write: bool/);
  assert.match(optionsSource, /dry_run: bool/);
  assert.match(optionsSource, /pub\(super\) struct DxForgeRegistrySmokeOptions/);
  assert.match(optionsSource, /pub\(super\) struct DxForgeRegistryPublishOptions/);
  assert.match(optionsSource, /pub\(super\) struct DxForgeRegistryPullOptions/);
  assert.match(optionsSource, /pub\(super\) struct DxForgeRegistryPullOptions[\s\S]*?dry_run: bool/);
  assert.match(optionsSource, /pub\(super\) struct DxForgeRegistryStatusOptions/);
  assert.match(optionsSource, /dx forge registry init requires --local <path>/);
  assert.match(optionsSource, /dx forge registry build requires --output <path>/);
  assert.match(optionsSource, /"--receipt" \| "--report"/);
  assert.match(optionsSource, /--receipt requires a path/);
  assert.match(optionsSource, /"--embed-content"/);
  assert.match(optionsSource, /--source-root requires a path/);
  assert.match(optionsSource, /--source-root requires --embed-content/);
  assert.match(optionsSource, /dx forge registry plan requires --item <name>/);
  assert.match(optionsSource, /Unknown forge registry plan option/);
  assert.match(optionsSource, /Unknown forge registry list option/);
  assert.match(optionsSource, /Unknown forge registry apply option/);
  assert.match(optionsSource, /Choose either --dry-run or --write, not both/);
  assert.match(optionsSource, /Unknown forge registry smoke option/);
  assert.match(optionsSource, /dx forge registry publish requires --package <id>/);
  assert.match(optionsSource, /dx forge registry pull requires --version <version>/);
  assert.match(optionsSource, /"--dry-run"/);
  assert.match(optionsSource, /Choose either --dry-run or --write, not both/);
  assert.match(optionsSource, /mod tests/);
});
