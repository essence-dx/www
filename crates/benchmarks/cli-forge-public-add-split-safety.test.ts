import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const forgePublicAddPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_public_add.rs",
);

test("Forge public add parsing and terminal rendering live outside the giant CLI module", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  assert.ok(
    fs.existsSync(forgePublicAddPath),
    "expected dx-www/src/cli/forge_public_add.rs",
  );

  const forgePublicAdd = fs.readFileSync(forgePublicAddPath, "utf8");

  assert.match(cliMod, /^mod forge_public_add;$/m);
  const importBlock = cliMod.match(/use self::forge_public_add::\{([\s\S]*?)\};/);
  assert.ok(importBlock, "expected CLI module to import forge public add helpers");
  for (const helper of ["dx_add_outcome_terminal", "parse_public_forge_add_request"]) {
    assert.match(importBlock[1], new RegExp(`\\b${helper}\\b`));
  }

  for (const oldDefinition of [
    "fn dx_add_outcome_terminal(",
    "struct PublicForgeAddRequest",
    "fn parse_public_forge_add_request(",
    "fn parse_public_forge_export_list(",
  ]) {
    assert.equal(
      cliMod.includes(oldDefinition),
      false,
      `${oldDefinition} should be owned by forge_public_add.rs`,
    );
  }

  for (const exportedBoundary of [
    "pub(super) fn dx_add_outcome_terminal(outcome: &DxForgeAddOutcome) -> String",
    "pub(super) struct PublicForgeAddRequest",
    "pub(super) fn parse_public_forge_add_request(",
    "fn parse_public_forge_export_list(",
  ]) {
    assert.ok(
      forgePublicAdd.includes(exportedBoundary),
      `${exportedBoundary} should live in forge_public_add.rs`,
    );
  }

  for (const preservedBehavior of [
    "DX add {mode}",
    "No node_modules were created",
    "No files were written and no package scripts ran",
    "shadcn/ui",
    "ui/",
  ]) {
    assert.ok(
      forgePublicAdd.includes(preservedBehavior),
      `forge_public_add.rs should preserve: ${preservedBehavior}`,
    );
  }

  assert.match(forgePublicAdd, /public_forge_package_id/);
  assert.match(forgePublicAdd, /let package_id = public_forge_package_id\(&package\.package_id\)/);
  assert.doesNotMatch(
    forgePublicAdd,
    /dx add \{\}.*package\.package_id/s,
    "terminal commands should use the public Forge package id",
  );
});
