import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const modPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const smokeModulePath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_hosted_registry_smoke.rs",
);

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Forge hosted registry smoke reports and renderers live outside cli mod.rs", () => {
  assert.equal(
    fs.existsSync(smokeModulePath),
    true,
    "expected forge_hosted_registry_smoke.rs to own hosted registry smoke reports",
  );

  const modSource = read(modPath);
  const moduleSource = read(smokeModulePath);

  assert.match(modSource, /\bmod forge_hosted_registry_smoke;/);
  const importBlock = modSource.match(/use self::forge_hosted_registry_smoke::\{[\s\S]*?\};/);
  assert.ok(importBlock, "expected forge_hosted_registry_smoke import block");

  for (const helper of [
    "build_forge_hosted_registry_smoke_report",
    "forge_hosted_registry_smoke_failure_summary",
    "forge_hosted_registry_smoke_markdown",
    "forge_hosted_registry_smoke_terminal",
  ]) {
    assert.equal(importBlock[0].includes(helper), true, `${helper} should be imported`);
  }

  for (const removedFromMod of [
    "struct DxForgeHostedRegistrySmokeReport",
    "struct DxForgeHostedRegistrySmokeCheck",
    "fn build_forge_hosted_registry_smoke_report(",
    "fn build_registry_pull_dry_run_report(",
    "fn registry_smoke_r2_object_keys(",
    "fn registry_smoke_object_url(",
    "fn registry_smoke_url_escape_segment(",
    "fn forge_hosted_registry_smoke_terminal(",
    "fn forge_hosted_registry_smoke_markdown(",
    "fn forge_hosted_registry_smoke_failure_summary(",
  ]) {
    assert.equal(
      modSource.includes(removedFromMod),
      false,
      `${removedFromMod} should not live in cli mod.rs`,
    );
  }

  for (const expectedInModule of [
    "pub(super) struct DxForgeHostedRegistrySmokeReport",
    "struct DxForgeHostedRegistrySmokeCheck",
    "pub(super) fn build_forge_hosted_registry_smoke_report(",
    "fn build_registry_pull_dry_run_report(",
    "fn registry_smoke_r2_object_keys(",
    "fn registry_smoke_object_url(",
    "fn registry_smoke_url_escape_segment(",
    "pub(super) fn forge_hosted_registry_smoke_terminal(",
    "pub(super) fn forge_hosted_registry_smoke_markdown(",
    "pub(super) fn forge_hosted_registry_smoke_failure_summary(",
  ]) {
    assert.equal(
      moduleSource.includes(expectedInModule),
      true,
      `${expectedInModule} should live in forge_hosted_registry_smoke.rs`,
    );
  }

  for (const contractText of [
    "DX Forge hosted registry smoke",
    "# DX Forge Hosted Registry Smoke",
    "publish_dry_run",
    "pull_dry_run",
    "no_secret_requirement",
    "no_node_modules",
    "registry-pull",
    "r2://",
  ]) {
    assert.equal(
      moduleSource.includes(contractText),
      true,
      `hosted registry smoke module should preserve ${contractText}`,
    );
  }
});
