import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = process.cwd();
const helpTextPath = join(repoRoot, "dx-www", "src", "cli", "help_text.rs");
const registryCommandsPath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "mod_parts",
  "cli_forge_commands_c.rs",
);
const forgeRegistryPath = join(repoRoot, "core", "src", "ecosystem", "forge_registry.rs");
const registryOperationsPath = join(
  repoRoot,
  "core",
  "src",
  "ecosystem",
  "forge_registry_parts",
  "registry_operations.rs",
);
const registrySchemaPath = join(
  repoRoot,
  "core",
  "src",
  "ecosystem",
  "forge_registry_parts",
  "forge_ui_registry_schema.rs",
);
const hostedRegistrySmokePath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_hosted_registry_smoke.rs",
);
const applyReceiptPath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_ui_registry_apply_receipt.rs",
);

const helpText = readFileSync(helpTextPath, "utf8");
const registryCommands = readFileSync(registryCommandsPath, "utf8");
const forgeRegistry = readFileSync(forgeRegistryPath, "utf8");
const registryOperations = readFileSync(registryOperationsPath, "utf8");
const registrySchema = readFileSync(registrySchemaPath, "utf8");
const hostedRegistrySmoke = readFileSync(hostedRegistrySmokePath, "utf8");
const applyReceipt = readFileSync(applyReceiptPath, "utf8");

test("forge registry apply is a first-class materialization command", () => {
  assert.match(helpText, /dx forge registry apply --item <name>/);
  assert.match(helpText, /\[--dry-run\|--write\]/);
  assert.match(helpText, /\[--receipt <path>\]/);
  assert.match(helpText, /Apply writes only reviewed inline registry content/);
  assert.match(helpText, /source-owned component and hosted package registry tools/);
  assert.match(helpText, /Forge package ids such as ui\/button instead of vendor-branded ids/);
  assert.doesNotMatch(helpText, /shadcn registry apply/);

  assert.match(registryCommands, /"apply" \| "materialize"/);
  assert.doesNotMatch(registryCommands, /"apply" \| "install"/);
  assert.match(registryCommands, /fn cmd_forge_registry_apply/);
  assert.match(registryCommands, /resolve_forge_ui_registry_reference\(&file, &item\)/);
  assert.match(registryCommands, /build_forge_ui_registry_apply_receipt/);
  assert.match(applyReceipt, /dx\.forge\.registry_apply_receipt/);
  assert.match(applyReceipt, /no_package_manager_execution/);
  assert.match(applyReceipt, /refused_external_dependencies/);
  assert.match(applyReceipt, /missing_reviewed_content/);
  assert.doesNotMatch(registryCommands, /npm install/);
  assert.doesNotMatch(registryCommands, /npx shadcn/);
  assert.doesNotMatch(registryCommands, /upstream UI registry reference set/);
});

test("forge registry apply uses Forge file transactions for source writes", () => {
  assert.match(registryCommands, /DxForgeFileTransaction::new\(&plan\.project\)/);
  assert.match(registryCommands, /file_transaction\.rollback\(\)/);
  assert.match(registryCommands, /file_transaction\.commit\(\)/);
  assert.match(registryCommands, /write_forge_ui_registry_source_file_new\(/);
  assert.match(registryCommands, /transaction: &mut DxForgeFileTransaction/);
  assert.match(registryCommands, /write_bytes_atomic\(path, content\.as_bytes\(\)\)/);
  assert.match(registryCommands, /already exists; rerun as a dry-run and review the receipt before overwriting/);
  assert.doesNotMatch(registryCommands, /forge_ui_registry_apply_rollback_created_files/);
});

test("forge registry apply receipts use the same Forge transaction model", () => {
  assert.match(applyReceipt, /DxForgeFileTransaction/);
  assert.match(applyReceipt, /write_forge_ui_registry_apply_receipt_artifacts_with_transaction/);
  assert.match(applyReceipt, /snapshot_forge_ui_registry_apply_receipt_artifact_paths/);
  assert.match(applyReceipt, /transaction\.snapshot_path\(receipt_path\)/);
  assert.match(applyReceipt, /transaction\.snapshot_path\(sr_source_path\)/);
  assert.match(applyReceipt, /transaction\.snapshot_path\(sr_machine_path\)/);
  assert.match(applyReceipt, /transaction\.snapshot_path\(json_machine_path\)/);
  assert.match(applyReceipt, /transaction\.snapshot_path\(json_machine_metadata_path\)/);
  assert.match(applyReceipt, /transaction\.write_bytes_atomic\(receipt_path, &json\)/);
  assert.match(applyReceipt, /transaction\.write_bytes_atomic\(receipt_path, &final_json\)/);
  assert.match(registryCommands, /write_forge_ui_registry_apply_receipt_artifacts_with_transaction/);
  assert.match(
    registryCommands,
    /write_forge_ui_registry_apply_receipt_artifacts_with_transaction\(\s*&project,\s*&receipt_output,\s*&mut report,\s*file_transaction,/,
  );
  assert.doesNotMatch(applyReceipt, /DxForgeRegistryApplyReceiptArtifactSnapshot/);
  assert.doesNotMatch(applyReceipt, /restore_best_effort/);
});

test("forge registry commands present native package ids in public reports", () => {
  assert.match(forgeRegistry, /pub fn public_forge_package_id/);
  assert.match(forgeRegistry, /"shadcn\/ui\/button" => "ui\/button"/);
  assert.match(forgeRegistry, /public_forge_package_id\(package_id\)/);
  assert.match(hostedRegistrySmoke, /public_forge_package_id\(&package\.package_id\)/);
  assert.match(hostedRegistrySmoke, /normalize_registry_operation_package_ids/);
  assert.match(registryOperations, /supported packages are `ui\/button`/);
  assert.doesNotMatch(registryOperations, /supported packages are `shadcn\/ui\/button`/);
});

test("forge ui help has a native command entry point", () => {
  assert.match(helpText, /dx forge ui: source-owned Forge UI capability commands/);
  assert.match(helpText, /dx forge ui parity \[--output <path>\]/);
  assert.match(helpText, /Registry-wide scripts can use dx forge registry parity/);
  assert.doesNotMatch(helpText, /Prefer dx forge registry parity in new scripts/);
});

test("forge registry docs is a native read-only item inspection command", () => {
  assert.match(helpText, /dx forge registry docs --item <name>/);
  assert.match(helpText, /Read reviewed registry item docs without writing files/);
  assert.match(registryCommands, /"docs" \| "view"/);
  assert.match(registryCommands, /fn cmd_forge_registry_docs/);
  assert.match(registryCommands, /describe_forge_ui_registry_item/);
  assert.match(registryCommands, /Package-manager execution: disabled/);
  assert.doesNotMatch(helpText, /shadcn docs/);
});

test("forge registry references support name templates without remote execution", () => {
  assert.match(registrySchema, /configured_forge_ui_registry_reference_url_for_item/);
  assert.match(
    registrySchema,
    /configured_forge_ui_registry_reference_url\(config\)\.replace\("\{name\}", item_name\)/,
  );
  assert.match(registrySchema, /resolve_local_forge_ui_registry_file_reference\(registry_file, &url\)/);
  assert.match(registrySchema, /remote registry resolution is bridge-gated/);
  assert.match(registrySchema, /no network request was made/);
});

test("forge registry pull can plan hosted reads without touching R2", () => {
  const pullStart = registryOperations.indexOf("pub async fn pull_registry_package_from_r2(");
  assert.notEqual(pullStart, -1, "pull_registry_package_from_r2 should exist");
  const pullBody = registryOperations.slice(pullStart);
  const dryRunStart = pullBody.indexOf("if dry_run {");
  const liveConfigStart = pullBody.indexOf("DxForgeR2Config::from_env()");
  const storeReadStart = pullBody.indexOf(".get(&ObjectPath::from");

  assert.notEqual(dryRunStart, -1, "registry pull should have an explicit dry-run branch");
  assert.notEqual(liveConfigStart, -1, "registry pull should keep live R2 config for real pulls");
  assert.notEqual(storeReadStart, -1, "registry pull should keep live object reads for real pulls");
  assert.ok(
    dryRunStart < liveConfigStart,
    "dry-run planning must happen before credential-backed R2 config is loaded",
  );
  assert.ok(
    dryRunStart < storeReadStart,
    "dry-run planning must happen before any R2 object read",
  );
  assert.match(pullBody, /DxForgeR2Config::status_from_env\(\)/);
  assert.match(pullBody, /object_url_from_status\(&status, &manifest_key\)/);
  assert.match(pullBody, /object_url_from_status\(&status, &content_key\)/);
  assert.match(pullBody, /dry_run: true/);
});
