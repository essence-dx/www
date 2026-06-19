import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = process.cwd();
const transactionPath = join(
  repoRoot,
  "core",
  "src",
  "ecosystem",
  "forge_file_transaction.rs",
);
const ecosystemModPath = join(repoRoot, "core", "src", "ecosystem", "mod.rs");
const forgeSecurityPath = join(repoRoot, "core", "src", "ecosystem", "forge_security.rs");
const forgeImportPlanPath = join(repoRoot, "dx-www", "src", "cli", "forge_import_plan.rs");

const transaction = readFileSync(transactionPath, "utf8");
const ecosystemMod = readFileSync(ecosystemModPath, "utf8");
const forgeSecurity = readFileSync(forgeSecurityPath, "utf8");
const forgeImportPlan = readFileSync(forgeImportPlanPath, "utf8");

test("Forge exposes a shared file transaction primitive", () => {
  assert.match(transaction, /pub struct DxForgeFileTransaction/);
  assert.match(transaction, /pub fn snapshot_path/);
  assert.match(transaction, /pub fn write_bytes_atomic/);
  assert.match(transaction, /pub fn write_json_pretty/);
  assert.match(transaction, /pub fn rollback/);
  assert.match(transaction, /pub fn commit/);
  assert.match(ecosystemMod, /mod forge_file_transaction;/);
  assert.match(ecosystemMod, /pub use forge_file_transaction::\*/);
});

test("Forge source manifest receipts write through one transaction", () => {
  assert.match(forgeSecurity, /let mut transaction = DxForgeFileTransaction::new\(project\)/);
  assert.match(forgeSecurity, /write_source_manifest_receipt_transaction/);
  assert.match(forgeSecurity, /transaction\.rollback\(\)/);
  assert.match(forgeSecurity, /transaction\.commit\(\)/);
  assert.match(forgeSecurity, /transaction\.write_json_pretty\(receipt_path, receipt\)/);
  assert.match(forgeSecurity, /transaction\.write_json_pretty\(manifest_path, manifest\)/);
  assert.match(forgeSecurity, /transaction\.write_bytes_atomic\(docs_path/);
});

test("Forge remove and dry-run receipts write through transactions", () => {
  assert.match(forgeSecurity, /write_source_manifest_remove_receipt_transaction/);
  assert.match(forgeSecurity, /transaction\.write_json_pretty\(manifest_path, manifest\)/);
  assert.match(forgeSecurity, /transaction\.write_json_pretty\(receipt_path, receipt\)/);
  assert.match(forgeSecurity, /let mut transaction = DxForgeFileTransaction::new\(project\)/);
  assert.match(forgeSecurity, /transaction\.write_json_pretty\(&receipt_path, receipt\)/);
  assert.match(forgeSecurity, /let mut transaction = DxForgeFileTransaction::new\(archive_root\)/);
  assert.match(forgeSecurity, /transaction\.write_json_pretty\(&manifest_path, &manifest\)/);
  assert.doesNotMatch(
    forgeSecurity,
    /fs::write\(&manifest_path, serde_json::to_vec_pretty\(&manifest\)\?\)/,
  );
  assert.doesNotMatch(
    forgeSecurity,
    /fs::write\(&receipt_path, serde_json::to_vec_pretty\(&receipt\)\?\)/,
  );
});

test("old source-manifest receipt path no longer owns ad hoc rollback helpers", () => {
  assert.doesNotMatch(forgeSecurity, /struct DxForgePublishFileSnapshot/);
  assert.doesNotMatch(forgeSecurity, /fn rollback_forge_publish_files/);
  assert.doesNotMatch(forgeSecurity, /fn restore_forge_publish_snapshot/);
  assert.doesNotMatch(forgeSecurity, /fn remove_empty_forge_publish_dirs/);
});

test("Forge import source materialization writes through the shared transaction", () => {
  assert.match(forgeImportPlan, /DxForgeFileTransaction/);
  assert.match(
    forgeImportPlan,
    /let mut file_transaction = DxForgeFileTransaction::new\(project\)/,
  );
  assert.match(forgeImportPlan, /file_transaction\.rollback\(\)/);
  assert.match(forgeImportPlan, /file_transaction\.commit\(\)/);
  assert.match(forgeImportPlan, /transaction\.write_bytes_atomic\(&target, content\.as_bytes\(\)\)/);
  assert.doesNotMatch(
    forgeImportPlan,
    /fs::write\(&target, content\)\.with_context\(\|\| format!\("write/,
  );
});

test("Forge import plan artifacts write through the shared transaction", () => {
  assert.match(forgeImportPlan, /write_import_plan_artifacts_with_transaction/);
  assert.match(forgeImportPlan, /snapshot_import_plan_artifact_paths/);
  assert.match(forgeImportPlan, /transaction\.snapshot_path\(project\.join\(json_relative_path\)\)/);
  assert.match(forgeImportPlan, /transaction\.snapshot_path\(sr_source_path\)/);
  assert.match(forgeImportPlan, /transaction\.snapshot_path\(sr_machine_path\)/);
  assert.match(forgeImportPlan, /transaction\.snapshot_path\(json_machine_path\)/);
  assert.match(forgeImportPlan, /transaction\.snapshot_path\(json_machine_metadata_path\)/);
  assert.match(forgeImportPlan, /write_import_plan_json\(project, report, transaction\)/);
  assert.match(forgeImportPlan, /transaction\s*\.\s*write_bytes_atomic\(&path, &serde_json::to_vec_pretty\(report\)\?\)/);
  assert.doesNotMatch(forgeImportPlan, /DxForgeImportPlanArtifactSnapshot/);
});
