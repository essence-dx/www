import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const modPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const statusModulePath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_public_status.rs",
);
const forgeCommandModulePath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "mod_parts",
  "cli_forge_commands_a.rs",
);

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Forge public status/remotes/receipts commands live outside cli mod.rs", () => {
  assert.equal(
    fs.existsSync(statusModulePath),
    true,
    "expected forge_public_status.rs to own status/remotes/receipts reports",
  );

  const modSource = read(modPath);
  const moduleSource = read(statusModulePath);

  assert.match(modSource, /\bmod forge_public_status;/);
  const importBlock = modSource.match(/use self::forge_public_status::\{[\s\S]*?\};/);
  assert.ok(importBlock, "expected forge_public_status import block in cli mod.rs");

  for (const helper of [
    "run_forge_public_receipts",
    "run_forge_public_remote",
    "run_forge_public_remotes",
    "run_forge_public_status",
  ]) {
    assert.equal(importBlock[0].includes(helper), true, `${helper} should be imported`);
  }

  for (const removedFromMod of [
    "fn cmd_forge_public_status(",
    "fn cmd_forge_public_remotes(",
    "fn cmd_forge_public_remote(",
    "fn cmd_forge_public_receipts(",
    "struct DxForgePublicStatusReport",
    "struct DxForgePublicRemotesReport",
    "struct DxForgePublicReceiptsReport",
    "Unknown forge status option",
    "Unknown forge remotes option",
    "Unknown forge receipts option",
    "DX Forge R2 remote is environment-backed",
    "fn forge_public_status_report(",
    "fn forge_public_remotes_report(",
    "fn forge_public_receipts_report(",
    "fn print_forge_public_status(",
    "fn print_forge_public_remotes(",
    "fn print_forge_public_receipts(",
    "fn forge_public_remote_states(",
    "fn forge_public_remote_object_head_health_from_value(",
  ]) {
    assert.equal(
      modSource.includes(removedFromMod),
      false,
      `${removedFromMod} should not live in cli mod.rs`,
    );
  }

  for (const expectedInModule of [
    "pub(super) fn run_forge_public_status(",
    "pub(super) fn run_forge_public_remotes(",
    "pub(super) fn run_forge_public_remote(",
    "pub(super) fn run_forge_public_receipts(",
    "pub(super) struct DxForgePublicStatusReport",
    "pub(super) struct DxForgePublicRemotesReport",
    "pub(super) struct DxForgePublicReceiptsReport",
    "Unknown forge status option",
    "Unknown forge remotes option",
    "Unknown forge receipts option",
    "DX Forge R2 remote is environment-backed",
    "pub(super) fn forge_public_status_report(",
    "pub(super) fn forge_public_remotes_report(",
    "pub(super) fn forge_public_receipts_report(",
    "pub(super) fn print_forge_public_status(",
    "pub(super) fn print_forge_public_remotes(",
    "pub(super) fn print_forge_public_receipts(",
    "fn forge_public_remote_states(",
  ]) {
    assert.equal(
      moduleSource.includes(expectedInModule),
      true,
      `${expectedInModule} should live in forge_public_status.rs`,
    );
  }

  for (const contractText of [
    "dx.forge.status",
    "dx.forge.remotes",
    "dx.forge.receipts",
    "DX Forge status",
    "DX Forge receipts",
    "R2 remote is",
    "dx forge remote login is not supported",
  ]) {
    assert.equal(
      moduleSource.includes(contractText),
      true,
      `status module should preserve ${contractText}`,
    );
  }
});

test("Forge remote command avoids login-style credential wording", () => {
  const modSource = read(modPath);
  const commandSource = read(forgeCommandModulePath);
  const moduleSource = read(statusModulePath);

  assert.doesNotMatch(commandSource, /"remote"\s*\|\s*"login"/);
  assert.match(commandSource, /"remote" => run_forge_public_remote/);
  assert.doesNotMatch(modSource, /"remote"\s*\|\s*"login"/);
  assert.doesNotMatch(moduleSource, /matches!\(action\.as_str\(\), "add" \| "status" \| "login"\)/);
  assert.match(moduleSource, /dx forge remote login is not supported/);
  assert.match(moduleSource, /Use `dx forge remote add r2`/);
});
