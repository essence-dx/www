import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.join(__dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const staticAssetsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_static_asset_materialization.rs",
);

test("Forge materialize static assets command wrapper lives outside cli mod.rs", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  const staticAssets = fs.readFileSync(staticAssetsPath, "utf8");
  const commandStart = cliMod.indexOf("fn cmd_forge_materialize_static_assets");
  const nextCommandStart = cliMod.indexOf("fn cmd_forge_migrated_route_benchmark", commandStart);
  assert.notEqual(commandStart, -1, "expected materialize static assets command in cli module");
  assert.notEqual(nextCommandStart, -1, "expected migrated route benchmark command after static assets");
  const commandBlock = cliMod.slice(commandStart, nextCommandStart);

  assert.match(cliMod, /^mod forge_static_asset_materialization;$/m);
  assert.match(
    cliMod,
    /use forge_static_asset_materialization::cmd_forge_materialize_static_assets as run_forge_materialize_static_assets_command;/,
  );
  assert.match(commandBlock, /run_forge_materialize_static_assets_command\(&self\.cwd, args\)/);

  assert.doesNotMatch(commandBlock, /let mut manifest: Option<PathBuf> = None/);
  assert.doesNotMatch(commandBlock, /Unknown forge materialize-static-assets option/);
  assert.doesNotMatch(commandBlock, /Unexpected forge materialize-static-assets argument/);
  assert.doesNotMatch(commandBlock, /requires --manifest <asset-manifest\.json>/);
  assert.doesNotMatch(commandBlock, /std::fs::write\(&output, &rendered\)/);
  assert.doesNotMatch(commandBlock, /forge_static_asset_materialization_failure_summary\(&report\)/);

  assert.match(staticAssets, /pub\(super\) fn cmd_forge_materialize_static_assets\(/);
  assert.match(staticAssets, /parse_score_threshold\(value\)\?/);
  assert.match(staticAssets, /Unknown forge materialize-static-assets option/);
  assert.match(staticAssets, /Unexpected forge materialize-static-assets argument/);
  assert.match(staticAssets, /dx forge materialize-static-assets requires --manifest <asset-manifest\.json>/);
  assert.match(staticAssets, /forge_static_asset_materialization_failure_summary\(&report\)/);
});
