import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = process.cwd();
const receiptPath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_ui_registry_build_receipt.rs",
);
const commandPath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "mod_parts",
  "cli_forge_commands_c.rs",
);
const optionsPath = join(repoRoot, "dx-www", "src", "cli", "forge_registry_options.rs");
const modPath = join(repoRoot, "dx-www", "src", "cli", "mod.rs");

test("forge registry build has a stable source-owned receipt schema", () => {
  const source = readFileSync(receiptPath, "utf8");

  assert.match(source, /dx\.forge\.registry_build_receipt/);
  assert.match(source, /generated_at/);
  assert.match(source, /command/);
  assert.match(source, /mode/);
  assert.match(source, /passed/);
  assert.match(source, /output_hash/);
  assert.match(source, /output_bytes/);
  assert.match(source, /no_package_manager_execution/);
  assert.match(source, /package_installs_run/);
  assert.match(source, /lifecycle_scripts_executed/);
  assert.match(source, /runtime_execution/);
  assert.match(source, /write_sr_artifact/);
  assert.match(source, /write_json_receipt_machine_alias/);
});

test("forge registry build receipts use Forge transactions for artifact writes", () => {
  const source = readFileSync(receiptPath, "utf8");

  assert.match(source, /DxForgeFileTransaction/);
  assert.match(source, /write_forge_ui_registry_build_receipt_artifacts_with_transaction/);
  assert.match(source, /snapshot_forge_ui_registry_build_receipt_artifact_paths/);
  assert.match(source, /transaction\.snapshot_path\(receipt_path\)/);
  assert.match(source, /transaction\.snapshot_path\(sr_source_path\)/);
  assert.match(source, /transaction\.snapshot_path\(sr_machine_path\)/);
  assert.match(source, /transaction\.snapshot_path\(json_machine_path\)/);
  assert.match(source, /transaction\.snapshot_path\(json_machine_metadata_path\)/);
  assert.match(source, /transaction\.write_bytes_atomic\(receipt_path, &json\)/);
  assert.match(source, /transaction\.write_bytes_atomic\(receipt_path, &final_json\)/);
  assert.doesNotMatch(source, /super::write_forge_ui_registry_output\(receipt_path/);
  assert.doesNotMatch(source, /restore_best_effort/);
});

test("forge registry build wires receipt artifacts into the command surface", () => {
  const command = readFileSync(commandPath, "utf8");
  const options = readFileSync(optionsPath, "utf8");
  const modSource = readFileSync(modPath, "utf8");

  assert.match(options, /receipt_output: Option<PathBuf>/);
  assert.match(options, /"--receipt"/);
  assert.match(command, /build_forge_ui_registry_build_receipt/);
  assert.match(command, /write_forge_ui_registry_build_receipt_artifacts/);
  assert.match(command, /build_receipt\.as_ref\(\)/);
  assert.match(modSource, /mod forge_ui_registry_build_receipt/);
});
