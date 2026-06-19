import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = process.cwd();
const applyReceiptPath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_ui_registry_apply_receipt.rs",
);
const cliModPath = join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const registryCommandsPath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "mod_parts",
  "cli_forge_commands_c.rs",
);

test("forge registry apply receipts mirror json sr and machine artifacts", () => {
  assert.equal(existsSync(applyReceiptPath), true);

  const applyReceipt = readFileSync(applyReceiptPath, "utf8");
  const cliMod = readFileSync(cliModPath, "utf8");
  const registryCommands = readFileSync(registryCommandsPath, "utf8");

  assert.match(cliMod, /mod forge_ui_registry_apply_receipt;/);
  assert.match(applyReceipt, /dx\.forge\.registry_apply_receipt/);
  assert.match(applyReceipt, /pub\(super\) struct DxForgeRegistryApplyReceipt/);
  assert.match(applyReceipt, /pub\(super\) struct DxForgeRegistryApplyReceiptArtifacts/);
  assert.match(applyReceipt, /receipt_json_path: Option<PathBuf>/);
  assert.match(applyReceipt, /receipt_sr_path: Option<PathBuf>/);
  assert.match(applyReceipt, /receipt_machine_path: Option<PathBuf>/);
  assert.match(applyReceipt, /receipt_json_machine_path: Option<PathBuf>/);
  assert.match(applyReceipt, /write_forge_ui_registry_apply_receipt_artifacts/);
  assert.match(applyReceipt, /write_sr_artifact/);
  assert.match(applyReceipt, /write_json_receipt_machine_alias/);
  assert.match(applyReceipt, /package_installs_run: false/);
  assert.match(applyReceipt, /lifecycle_scripts_executed: false/);
  assert.match(applyReceipt, /runtime_execution: false/);

  assert.match(registryCommands, /build_forge_ui_registry_apply_receipt/);
  assert.match(registryCommands, /write_forge_ui_registry_apply_receipt_artifacts/);
  assert.match(registryCommands, /forge_ui_registry_apply_rendered/);
});
