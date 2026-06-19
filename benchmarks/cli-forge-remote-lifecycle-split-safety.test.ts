import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const lifecyclePath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_remote_lifecycle.rs",
);

test("Forge remote lifecycle dry-run and HEAD report helpers live outside the giant CLI module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(lifecyclePath),
    "expected dx-www/src/cli/forge_remote_lifecycle.rs",
  );

  const lifecycle = fs.readFileSync(lifecyclePath, "utf8");

  assert.match(cliMod, /^mod forge_remote_lifecycle;$/m);
  const importBlock = cliMod.match(/use self::forge_remote_lifecycle::\{([\s\S]*?)\};/);
  assert.ok(importBlock, "expected CLI module to import remote lifecycle helpers");
  for (const helper of [
    "DxForgeRemoteHeadCliReport",
    "DxForgeRemoteLifecycleAction",
    "forge_remote_head_receipt_path",
    "forge_remote_lifecycle_dry_run",
    "print_forge_remote_head_report",
    "print_forge_remote_lifecycle_plans",
    "write_forge_remote_head_report",
  ]) {
    assert.match(importBlock[1], new RegExp(`\\b${helper}\\b`));
  }

  for (const oldDefinition of [
    "enum DxForgeRemoteLifecycleAction",
    "struct DxForgeRemoteLifecyclePlan",
    "struct DxForgeRemoteHeadCliReport",
    "fn forge_remote_lifecycle_dry_run(",
    "fn print_forge_remote_lifecycle_plans(",
    "fn forge_remote_lifecycle_plan_markdown(",
    "fn print_forge_remote_head_report(",
    "fn forge_remote_head_report_markdown(",
    "fn write_forge_remote_head_report(",
    "fn forge_remote_head_receipt_path(",
    "fn forge_receipt_segment(",
  ]) {
    assert.equal(
      cliMod.includes(oldDefinition),
      false,
      `${oldDefinition} should be owned by forge_remote_lifecycle.rs`,
    );
  }

  for (const exportedBoundary of [
    "pub(super) enum DxForgeRemoteLifecycleAction",
    "pub(super) struct DxForgeRemoteHeadCliReport",
    "pub(super) fn forge_remote_lifecycle_dry_run(",
    "pub(super) fn print_forge_remote_lifecycle_plans(",
    "pub(super) fn print_forge_remote_head_report(",
    "pub(super) fn write_forge_remote_head_report(",
    "pub(super) fn forge_remote_head_receipt_path(",
  ]) {
    assert.ok(
      lifecycle.includes(exportedBoundary),
      `${exportedBoundary} should live in forge_remote_lifecycle.rs`,
    );
  }

  for (const preservedBehavior of [
    "remote install/update/uninstall dry-run boundary only",
    "s3-compatible-object-storage",
    "DX Forge Remote Lifecycle Dry Run",
    "DX Forge R2 HEAD Health",
    "Remote writes allowed: `false`",
    "r2-head-health.json",
  ]) {
    assert.ok(
      lifecycle.includes(preservedBehavior),
      `forge_remote_lifecycle.rs should preserve: ${preservedBehavior}`,
    );
  }
});
