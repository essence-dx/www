import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const repoRoot = process.cwd();
const importPlanPath = join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "forge_import_plan.rs",
);
const scoringPath = join(
  repoRoot,
  "core",
  "src",
  "ecosystem",
  "forge_importer",
  "scoring.rs",
);
const dispositionPath = join(
  repoRoot,
  "core",
  "src",
  "ecosystem",
  "forge_importer",
  "disposition.rs",
);

const source = readFileSync(importPlanPath, "utf8");
const scoring = readFileSync(scoringPath, "utf8");
const disposition = readFileSync(dispositionPath, "utf8");

test("forge import plan exposes schema and serializer artifact paths", () => {
  assert.match(source, /FORGE_IMPORT_PLAN_SCHEMA/);
  assert.match(source, /dx\.forge\.package_import_plan/);
  assert.match(source, /import_plan_relative_path\(/);
  assert.match(source, /\.dx\/forge\/import-plans\/\{\}-\{slug\}\.\{extension\}/);
  assert.match(source, /import_plan_sr_path:\s*Option<PathBuf>/);
  assert.match(source, /import_plan_machine_path:\s*Option<PathBuf>/);
  assert.match(source, /import_plan_json_machine_path:\s*Option<PathBuf>/);
  assert.match(source, /write_json_receipt_machine_alias/);
});

test("forge import plan and write paths generate sr artifacts and machine caches", () => {
  assert.match(source, /write_import_plan_artifacts\(/);
  assert.match(source, /write_import_plan_artifacts_with_transaction\(/);
  assert.match(source, /snapshot_import_plan_artifact_paths\(/);
  assert.match(source, /transaction\.snapshot_path\(project\.join\(json_relative_path\)\)/);
  assert.match(source, /transaction\.snapshot_path\(sr_source_path\)/);
  assert.match(source, /transaction\.snapshot_path\(sr_machine_path\)/);
  assert.match(source, /transaction\.snapshot_path\(json_machine_path\)/);
  assert.match(source, /transaction\.snapshot_path\(json_machine_metadata_path\)/);
  assert.match(source, /write_import_plan_sr\(/);
  assert.match(source, /write_sr_artifact\(/);
  assert.match(source, /write_import_plan_json\(project, report, transaction\)/);
  assert.match(source, /transaction\s*\.\s*write_bytes_atomic\(&path, &serde_json::to_vec_pretty\(report\)\?\)/);
  assert.match(source, /report\.import_plan_sr_path = Some\(final_sr_artifact\.source\)/);
  assert.match(
    source,
    /report\.import_plan_machine_path = Some\(final_sr_artifact\.machine\)/,
  );
  assert.doesNotMatch(source, /DxForgeImportPlanArtifactSnapshot/);
});

test("serializer import plan records the no install source-owned policy", () => {
  for (const field of [
    "schema",
    "passed",
    "fail_under",
    "ecosystem",
    "package_id",
    "package_name",
    "score_model_version",
    "score",
    "uncapped_score",
    "score_ceiling",
    "traffic",
    "forge_import_gate",
    "source_provenance_verified",
    "source_integrity_evidence_declared",
    "source_license_reviewed",
    "source_advisory_evidence_declared",
    "source_advisory_reviewed",
    "source_popularity_evidence_declared",
    "source_sbom_present",
    "no_node_modules",
    "package_installs_run",
    "lifecycle_scripts_executed",
    "lifecycle_script_status",
    "import_alias",
    "source_kind",
    "materialization_status",
    "materialization_boundary",
    "accepted_materialization_receipt_present",
    "review_findings",
    "next_commands",
    "materialized_files",
    "selected_files",
    "selected_files_count",
    "source_files_inspected_count",
    "score_dimension_ids",
    "score_dimension_scores",
    "applied_cap_ids",
    "disposition_kind",
    "disposition_is_materialize",
    "disposition_is_slice",
    "disposition_is_bridge",
    "disposition_is_reject",
    "disposition_route",
    "disposition_slice_kind",
    "disposition_bridge_kind",
    "disposition_ownership_claim",
    "disposition_importable_source",
    "disposition_materializes_source",
    "disposition_requires_receipt",
    "package_disposition",
    "package_disposition_route",
    "package_disposition_slice_kind",
    "package_disposition_bridge_kind",
    "package_ownership_claim",
    "package_disposition_importable_source",
    "package_disposition_materializes_source",
    "package_disposition_requires_receipt",
    "source_slice_decision",
    "source_slice_kind",
    "source_slice_policy_codes",
    "source_slice_policy_details",
    "source_slice_policy_remediations",
    "materialization_blocker_ids",
    "refusal_reason_phases",
    "refusal_reason_gates",
    "refusal_reason_codes",
    "refusal_reason_details",
    "refusal_reason_remediations",
    "bridge_reason_codes",
    "bridge_reason_details",
    "import_capability_score_100_requirements",
    "restore_capability",
    "restore_content_source",
    "receipt_contains_file_content",
    "rollback_after_delete_supported",
    "failed_write_atomicity",
    "failed_write_recovery",
    "write_transaction_mode",
    "write_transaction_scope",
    "write_transaction_rollback",
    "write_transaction_limitations",
    "risk_flag_codes",
    "blocking_risk_flag_codes",
    "applied_cap_ceilings",
    "applied_cap_traffic",
    "applied_cap_reasons",
    "file_disposition_kinds",
    "files_materialized_count",
    "files_sliced_count",
    "files_bridged_count",
    "files_written_count",
    "files_kept_count",
    "files_materialized",
    "files_sliced",
    "files_bridged",
  ]) {
    assert.match(source, new RegExp(`"${field}"`));
  }
});

test("forge import exposes stable materialize slice bridge reject disposition vocabulary", () => {
  assert.match(disposition, /DxForgePackageDispositionKind/);
  for (const marker of ["Materialize", "Slice", "Bridge", "Reject"]) {
    assert.match(disposition, new RegExp(marker));
  }
  assert.match(source, /DxForgePackageDispositionReport/);
  assert.match(source, /package_disposition/);
});

test("forge import scoring caps are keyed by package disposition risks", () => {
  for (const cap of [
    "package-code-executed",
    "plan-only-no-inspected-source",
    "artifact-integrity-incomplete",
    "materialization-receipt-missing",
  ]) {
    assert.match(scoring, new RegExp(cap));
  }
  for (const risk of [
    "LifecycleScript",
    "NativeBinary",
    "SideEffectImport",
  ]) {
    assert.match(scoring, new RegExp(risk));
  }
  for (const cap of [
    "disposition-slice",
    "disposition-bridge",
    "disposition-reject",
  ]) {
    assert.match(source, new RegExp(cap));
  }
  assert.match(disposition, /reject-security-or-conflict/);
  assert.match(disposition, /bridge-native-runtime/);
  assert.match(disposition, /slice-reviewed-source/);
  assert.match(disposition, /materialize-source-owned/);
  assert.match(disposition, /accepted_materialization_receipt_present/);
  assert.match(source, /accepted_materialization_receipt_present/);
  assert.match(source, /materialization_boundary/);
});

test("forge import production scoring requires explicit artifact integrity evidence", () => {
  assert.match(source, /artifact_integrity_present:\s*evidence\.integrity_evidence_present/);
  assert.doesNotMatch(source, /artifact_integrity_present:\s*evidence\.integrity_evidence_present\s*\|\|\s*files_considered\s*>\s*0/);
});

test("forge import separates declared evidence from reviewed supply-chain proof", () => {
  for (const marker of [
    "provenance_verified",
    "license_reviewed",
    "advisory_reviewed",
    "source_provenance_verified",
    "source_license_reviewed",
    "source_advisory_reviewed",
    "source_sbom_present",
    "sbom.spdx.json",
    "SBOM.spdx.json",
    "sbom.cyclonedx.json",
  ]) {
    assert.match(source, new RegExp(marker.replaceAll("/", "\\/")));
  }
  assert.match(scoring, /declared_evidence_without_review_cannot_claim_green_score/);
});

test("forge import uses evidence dimensions and hard caps instead of fixed package scores", () => {
  for (const marker of [
    "score_forge_import",
    "DxForgeImportScoreInput",
    "score_dimensions",
    "applied_caps",
    "uncapped_score",
    "score_ceiling",
  ]) {
    assert.match(source, new RegExp(marker.replaceAll("/", "\\/")));
  }
  for (const cap of [
    "source-dir-only-no-registry-metadata",
    "license-review-incomplete",
    "advisory-evidence-incomplete",
    "provenance-verification-pending",
    "license-review-pending",
    "advisory-review-pending",
    "source-sbom-missing",
  ]) {
    assert.match(scoring, new RegExp(cap));
  }

  assert.doesNotMatch(source, /report\.score = 96/);
});

test("forge import preserves source-slice and refusal policy payloads", () => {
  for (const marker of [
    "DxForgeImportRefusalReason",
    "source_slice_policy_decisions",
    "materialization_blocker_ids",
    "refusal_reasons",
    "bridge_reason_codes",
    "record_source_slice_policy(",
    "source_slice.policy_decisions.clone()",
    "refresh_import_decision_receipts(report)",
    "build_import_refusal_reasons(report)",
    "source_slice_policy_codes(report)",
    "refusal_reason_codes(report)",
    "applied_cap_reasons(report)",
  ]) {
    assert.ok(source.includes(marker), `missing marker ${marker}`);
  }
});

test("forge import receipts are honest about restore and write atomicity", () => {
  for (const marker of [
    "hash-only-not-restorable-after-delete",
    "local-registry-required-for-byte-restore",
    "receipt_contains_file_content: false",
    "rollback_after_delete_supported: false",
    "rollback-protected-source-manifest-receipt-docs-import-plan",
    "future rollback after deleting source still requires local registry or project content matching receipt hashes",
    '"restore_capability"',
    '"restore_content_source"',
    '"receipt_contains_file_content"',
    '"rollback_after_delete_supported"',
    '"failed_write_atomicity"',
    '"failed_write_recovery"',
  ]) {
    assert.ok(source.includes(marker), `missing marker ${marker}`);
  }
});

test("forge import receipts expose rollback-protected write transactions", () => {
  for (const field of [
    "write_transaction_mode",
    "write_transaction_scope",
    "write_transaction_rollback",
    "write_transaction_limitations",
  ]) {
    assert.match(source, new RegExp(field));
  }

  for (const marker of [
    "rollback-protected-source-manifest-receipt-docs-import-plan",
    "source files, source manifest, package receipt, package docs, import-plan json, import-plan sr, serializer machine cache, and json machine alias",
    "Forge restores captured manifest/docs/import-plan artifacts, removes the new package receipt, and removes unchanged source files created before the failed step",
  ]) {
    assert.ok(source.includes(marker), `missing marker ${marker}`);
  }

  assert.doesNotMatch(
    source,
    /source-files-and-receipts-are-not-yet-one-atomic-transaction/,
  );
  assert.doesNotMatch(
    source,
    /manual-cleanup-required-if-a-post-write-artifact-fails/,
  );
});

test("forge import write path gates risky source before materialization", () => {
  for (const marker of [
    "let risk_flags = source_evidence.risk_flags.clone()",
    "risk_flags: risk_flags.clone()",
    "inspect_metadata_risks",
    "package_json_has_install_hook",
    "native_source_artifact",
    "source_has_dynamic_execution",
    "source_has_dynamic_import",
    "source_has_side_effect_import",
    "source_contains_plaintext_secret",
    "evidence_marker_is_true",
    "source_snapshot_preflight_cleared",
    "write mode refused to materialize because package disposition",
    "write mode refused to materialize because preflight caps remain",
  ]) {
    assert.match(source, new RegExp(marker.replaceAll("/", "\\/")));
  }
});

test("external source import records source snapshot origin and file disposition", () => {
  for (const marker of [
    "DxForgeExternalSourcePackage",
    "write_forge_external_source",
    "external-source-snapshot",
    "source_dir",
    "collect_source_dir_files",
    "files_kept",
    "files_written",
    "files_rejected",
    "files_considered",
    "overwrite_policy",
    "reject-yellow-no-overwrite",
  ]) {
    assert.match(source, new RegExp(marker.replaceAll("/", "\\/")));
  }
});

test("external source sr receipt mirrors origin license export and overwrite fields", () => {
  for (const field of [
    "origin_registry",
    "origin_source_kind",
    "origin_generator",
    "origin_provenance_verified",
    "origin_provenance_note",
    "license_declared",
    "license_source",
    "license_file_hash",
    "license_reviewed",
    "export_names",
    "import_plan_path",
    "import_plan_sr_path",
    "import_plan_machine_path",
    "import_plan_json_machine_path",
    "overwrite_policy_different_existing_file",
    "overwrite_policy_missing_file",
    "overwrite_policy_matching_existing_file",
    "overwrite_policy_security_sensitive_or_invalid_path",
    "overwrite_policy_partial_write",
    "materialized_package_id",
    "manifest_path",
    "receipt_path",
    "docs_path",
  ]) {
    assert.match(source, new RegExp(`"${field}"`));
  }
});
