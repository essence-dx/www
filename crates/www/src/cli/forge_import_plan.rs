use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;
use chrono::Utc;
use serde::Serialize;

use dx_compiler::ecosystem::{
    DxForgeExternalSourcePackage, DxForgeFileTransaction, DxForgeImportAcquisitionPlan,
    DxForgeImportAppliedCap, DxForgeImportCapabilityTier, DxForgeImportDecision,
    DxForgeImportEcosystem, DxForgeImportEcosystemCapability, DxForgeImportPolicyDecision,
    DxForgeImportRiskFlag, DxForgeImportScoreDimension, DxForgeImportScoreInput,
    DxForgeImportScoreReport, DxForgeImportSliceCandidate, DxForgeImportSliceDecision,
    DxForgeImportSliceKind, DxForgeLocalSourceFile, DxForgePackageBridgeKind,
    DxForgePackageDispositionInput, DxForgePackageDispositionKind, DxForgePackageDispositionReport,
    acquisition_plan_for_package, classify_forge_package_disposition, classify_import_slice,
    import_capability_for_ecosystem, score_forge_import, validate_import_package_name,
    validate_import_relative_path, validate_import_target_path, write_forge_external_source,
};

use super::markdown_table_cell;
use super::serializer_artifacts::{
    SrArtifact, serializer_machine_path_for_sr, sr_bool, sr_null, sr_number, sr_string,
    sr_string_array, write_json_receipt_machine_alias, write_sr_artifact,
};

const FORGE_IMPORT_PLAN_SCHEMA: &str = "dx.forge.package_import_plan";
const FORGE_IMPORT_WRITE_TRANSACTION_MODE: &str =
    "rollback-protected-source-manifest-receipt-docs-import-plan";
const FORGE_IMPORT_WRITE_TRANSACTION_SCOPE: &str = "source files, source manifest, package receipt, package docs, import-plan json, import-plan sr, serializer machine cache, and json machine alias";
const FORGE_IMPORT_WRITE_TRANSACTION_ROLLBACK: &str = "Forge restores captured manifest/docs/import-plan artifacts, removes the new package receipt, and removes unchanged source files created before the failed step";
const FORGE_IMPORT_WRITE_TRANSACTION_LIMITATION: &str = "future rollback after deleting source still requires local registry or project content matching receipt hashes";

#[derive(Debug, Clone, Serialize)]
struct DxForgeImportOrigin {
    ecosystem: String,
    registry: String,
    package_name: String,
    package_id: String,
    upstream_name: String,
    upstream_version: String,
    upstream_reference: Option<String>,
    tarball_integrity: Option<String>,
    source_kind: String,
    generator: String,
    provenance_verified: bool,
    provenance_note: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeImportLicense {
    declared_license: String,
    license_source: String,
    license_file_hash: Option<String>,
    reviewed: bool,
    reviewed_at: Option<String>,
    note: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeImportExport {
    name: String,
    kind: String,
    source_path: String,
    upstream_export: String,
    materialized: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeImportReviewedAdapter {
    specifier: String,
    package_id: String,
    package_name: String,
    materialized_path: String,
    source_path: String,
    root: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeImportFileDisposition {
    path: String,
    logical_path: String,
    hash_algorithm: String,
    hash: String,
    bytes: u64,
    before_hash: Option<String>,
    after_hash: Option<String>,
    tracked_hash: String,
    status: String,
    traffic: String,
    decision: String,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeImportOverwritePolicy {
    missing_file: String,
    matching_existing_file: String,
    different_existing_file: String,
    security_sensitive_or_invalid_path: String,
    partial_write: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeImportRefusalReason {
    phase: String,
    gate: String,
    code: String,
    detail: String,
    remediation: String,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeImportPlanReport {
    schema: &'static str,
    version: u32,
    generated_at: String,
    pub(super) passed: bool,
    pub(super) score: u8,
    score_model_version: String,
    uncapped_score: u8,
    score_ceiling: u8,
    traffic: String,
    score_dimensions: Vec<DxForgeImportScoreDimension>,
    applied_caps: Vec<DxForgeImportAppliedCap>,
    disposition: DxForgePackageDispositionReport,
    ecosystem_capability: DxForgeImportEcosystemCapability,
    fail_under: u8,
    project: PathBuf,
    ecosystem: String,
    package_id: String,
    package_name: String,
    mode: String,
    import_alias: String,
    source_kind: String,
    forge_import_gate: bool,
    source_dir_ready: bool,
    no_node_modules: bool,
    package_installs_run: bool,
    lifecycle_scripts_executed: bool,
    lifecycle_script_status: String,
    acquisition_metadata_inputs: Vec<String>,
    acquisition_artifact_inputs: Vec<String>,
    acquisition_plan: DxForgeImportAcquisitionPlan,
    forbidden_commands: Vec<String>,
    live_fetching_enabled: bool,
    package_manager_execution_allowed: bool,
    accepted_import_receipt_required: bool,
    unsupported_dx_add_form: String,
    risk_flags: Vec<DxForgeImportRiskFlag>,
    source_provenance_verified: bool,
    source_integrity_evidence_declared: bool,
    source_license_reviewed: bool,
    source_advisory_evidence_declared: bool,
    source_advisory_reviewed: bool,
    source_popularity_evidence_declared: bool,
    source_sbom_present: bool,
    selected_files: Vec<String>,
    source_files_inspected_count: usize,
    source_dependency_count: usize,
    requested_symbols: Vec<String>,
    source_slice_decision: String,
    source_slice_kind: String,
    source_slice_policy_decisions: Vec<DxForgeImportPolicyDecision>,
    materialization_blocker_ids: Vec<String>,
    refusal_reasons: Vec<DxForgeImportRefusalReason>,
    bridge_reason_codes: Vec<String>,
    bridge_reason_details: Vec<String>,
    restore_capability: String,
    restore_content_source: String,
    receipt_contains_file_content: bool,
    rollback_after_delete_supported: bool,
    failed_write_atomicity: String,
    failed_write_recovery: String,
    write_transaction_mode: String,
    write_transaction_scope: Vec<String>,
    write_transaction_rollback: String,
    write_transaction_limitations: Vec<String>,
    origin: DxForgeImportOrigin,
    license: DxForgeImportLicense,
    exports: Vec<DxForgeImportExport>,
    reviewed_adapters: Vec<DxForgeImportReviewedAdapter>,
    files_kept: Vec<String>,
    files_written: Vec<String>,
    files_rejected: Vec<String>,
    files_considered: Vec<DxForgeImportFileDisposition>,
    overwrite_policy: DxForgeImportOverwritePolicy,
    materialization_ready: bool,
    materialization_status: String,
    materialized: bool,
    accepted_materialization_receipt_present: bool,
    materialized_package_id: Option<String>,
    materialized_files: Vec<String>,
    accepted_plan_path: Option<PathBuf>,
    accepted_plan_status: String,
    accepted_plan_findings: Vec<String>,
    manifest_path: Option<PathBuf>,
    receipt_path: Option<PathBuf>,
    docs_path: Option<PathBuf>,
    import_plan_path: Option<PathBuf>,
    import_plan_sr_path: Option<PathBuf>,
    import_plan_machine_path: Option<PathBuf>,
    import_plan_json_machine_path: Option<PathBuf>,
    inspected_inputs: Vec<String>,
    review_required: Vec<String>,
    can_materialize: Vec<String>,
    cannot_materialize: Vec<String>,
    findings: Vec<String>,
    review_findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Default)]
struct DxForgeImportSourceEvidence {
    metadata_present: bool,
    metadata_package_name: Option<String>,
    declared_license: Option<String>,
    license_file_hash: Option<String>,
    provenance_verified: bool,
    integrity_evidence_present: bool,
    license_reviewed: bool,
    advisory_evidence_present: bool,
    advisory_reviewed: bool,
    popularity_evidence_present: bool,
    sbom_present: bool,
    source_file_count: usize,
    dependency_count: usize,
    risk_flags: Vec<DxForgeImportRiskFlag>,
}

pub(super) fn build_forge_import_plan_report(
    project: &Path,
    ecosystem: &str,
    package_name: &str,
    source_dir: Option<&Path>,
    fail_under: u8,
) -> anyhow::Result<DxForgeImportPlanReport> {
    build_forge_import_plan_report_with_selection(
        project,
        ecosystem,
        package_name,
        source_dir,
        &[],
        fail_under,
    )
}

pub(super) fn build_forge_import_plan_report_with_selection(
    project: &Path,
    ecosystem: &str,
    package_name: &str,
    source_dir: Option<&Path>,
    selected_files: &[String],
    fail_under: u8,
) -> anyhow::Result<DxForgeImportPlanReport> {
    build_forge_import_plan_report_with_selection_and_artifact_mode(
        project,
        ecosystem,
        package_name,
        source_dir,
        selected_files,
        fail_under,
        true,
    )
}

fn build_forge_import_plan_report_with_selection_and_artifact_mode(
    project: &Path,
    ecosystem: &str,
    package_name: &str,
    source_dir: Option<&Path>,
    selected_files: &[String],
    fail_under: u8,
    write_artifacts: bool,
) -> anyhow::Result<DxForgeImportPlanReport> {
    let ecosystem = parse_import_ecosystem(ecosystem)?;
    let ecosystem_segment = ecosystem.as_segment();
    let package_name = package_name.trim();
    if package_name.is_empty() {
        anyhow::bail!("Forge import package name is required");
    }

    let no_node_modules = !project.join("node_modules").exists();
    let package_installs_run = false;
    let lifecycle_scripts_executed = false;
    let mut findings = Vec::new();
    if !no_node_modules {
        findings
            .push("node_modules exists in the target project before import planning.".to_string());
    }
    let source_dir_ready = source_dir.is_some_and(Path::is_dir);
    if let Some(source_dir) = source_dir {
        if !source_dir.is_dir() {
            findings.push(format!(
                "source directory `{}` does not exist or is not a directory.",
                source_dir.display()
            ));
        }
    }

    let materialization_ready = source_dir_ready && no_node_modules;
    let package_id = import_package_id(ecosystem, package_name)?;
    let import_alias = import_alias(ecosystem, package_name, &package_id);
    let source_kind = if materialization_ready {
        "external-source-snapshot-ready".to_string()
    } else {
        "registry-plan-review-required".to_string()
    };
    let selected_files = normalize_selected_source_files(selected_files)?;
    let exports = source_dir
        .filter(|path| path.is_dir())
        .map(|path| discover_source_exports(path, &selected_files))
        .transpose()?
        .unwrap_or_default();
    let mut source_evidence = source_dir
        .filter(|path| path.is_dir())
        .map(|path| inspect_import_source_evidence(path, &selected_files))
        .transpose()?
        .unwrap_or_default();
    record_package_identity_risk(package_name, &mut source_evidence);
    let risk_flags = source_evidence.risk_flags.clone();
    let score_report = build_import_score_report(ImportScoreReportInput {
        no_node_modules,
        source_dir_ready,
        materialized: false,
        accepted_materialization_receipt_present: false,
        evidence: &source_evidence,
        exports: &exports,
        files_considered: source_evidence.source_file_count,
        files_rejected: 0,
        package_installs_run,
        lifecycle_scripts_executed,
        risk_flags: risk_flags.clone(),
    });
    let disposition = build_import_disposition_report(DxForgePackageDispositionInput {
        requested_kind: if source_dir_ready {
            DxForgeImportSliceKind::SourceSlice
        } else {
            DxForgeImportSliceKind::MetadataOnly
        },
        source_dir_ready,
        materialized: false,
        accepted_materialization_receipt_present: false,
        no_node_modules,
        package_installs_run,
        lifecycle_scripts_executed,
        files_considered: source_evidence.source_file_count,
        files_rejected: 0,
        score: score_report.score,
        score_ceiling: score_report.score_ceiling,
        risk_flags: risk_flags.clone(),
    });
    let passed = findings.is_empty()
        && no_node_modules
        && !package_installs_run
        && !lifecycle_scripts_executed
        && score_report.score >= fail_under;
    let requested_symbols = exports
        .iter()
        .map(|export| export.name.clone())
        .collect::<Vec<_>>();
    let surface = import_plan_surface(ecosystem);
    let ecosystem_capability = import_capability_for_ecosystem(ecosystem);
    let acquisition_plan = acquisition_plan_for_package(ecosystem, package_name, &package_id);

    let mut report = DxForgeImportPlanReport {
        schema: FORGE_IMPORT_PLAN_SCHEMA,
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score: score_report.score,
        score_model_version: score_report.score_model_version,
        uncapped_score: score_report.uncapped_score,
        score_ceiling: score_report.score_ceiling,
        traffic: score_report.traffic,
        score_dimensions: score_report.dimensions,
        applied_caps: score_report.applied_caps,
        disposition,
        ecosystem_capability,
        fail_under,
        project: project.to_path_buf(),
        ecosystem: ecosystem_segment.to_string(),
        package_id: package_id.clone(),
        package_name: package_name.to_string(),
        mode: "plan-only".to_string(),
        import_alias,
        source_kind: source_kind.clone(),
        forge_import_gate: true,
        source_dir_ready,
        no_node_modules,
        package_installs_run,
        lifecycle_scripts_executed,
        lifecycle_script_status: "not-executed".to_string(),
        acquisition_metadata_inputs: surface.metadata_inputs.clone(),
        acquisition_artifact_inputs: surface.artifact_inputs.clone(),
        acquisition_plan,
        forbidden_commands: surface.forbidden_commands.clone(),
        live_fetching_enabled: surface.live_fetching_enabled,
        package_manager_execution_allowed: surface.package_manager_execution,
        accepted_import_receipt_required: surface.accepted_import_receipt_required,
        unsupported_dx_add_form: surface.unsupported_dx_add_form.clone(),
        risk_flags,
        source_provenance_verified: source_evidence.provenance_verified,
        source_integrity_evidence_declared: source_evidence.integrity_evidence_present,
        source_license_reviewed: source_evidence.license_reviewed,
        source_advisory_evidence_declared: source_evidence.advisory_evidence_present,
        source_advisory_reviewed: source_evidence.advisory_reviewed,
        source_popularity_evidence_declared: source_evidence.popularity_evidence_present,
        source_sbom_present: source_evidence.sbom_present,
        selected_files,
        source_files_inspected_count: source_evidence.source_file_count,
        source_dependency_count: source_evidence.dependency_count,
        requested_symbols,
        source_slice_decision: "not-evaluated".to_string(),
        source_slice_kind: "not-evaluated".to_string(),
        source_slice_policy_decisions: Vec::new(),
        materialization_blocker_ids: Vec::new(),
        refusal_reasons: Vec::new(),
        bridge_reason_codes: Vec::new(),
        bridge_reason_details: Vec::new(),
        restore_capability: "hash-only-not-restorable-after-delete".to_string(),
        restore_content_source: "local-registry-required-for-byte-restore".to_string(),
        receipt_contains_file_content: false,
        rollback_after_delete_supported: false,
        failed_write_atomicity: FORGE_IMPORT_WRITE_TRANSACTION_MODE.to_string(),
        failed_write_recovery: FORGE_IMPORT_WRITE_TRANSACTION_ROLLBACK.to_string(),
        write_transaction_mode: FORGE_IMPORT_WRITE_TRANSACTION_MODE.to_string(),
        write_transaction_scope: vec![FORGE_IMPORT_WRITE_TRANSACTION_SCOPE.to_string()],
        write_transaction_rollback: FORGE_IMPORT_WRITE_TRANSACTION_ROLLBACK.to_string(),
        write_transaction_limitations: vec![FORGE_IMPORT_WRITE_TRANSACTION_LIMITATION.to_string()],
        origin: import_origin(
            ecosystem,
            package_name,
            &package_id,
            &source_kind,
            source_evidence.provenance_verified,
        ),
        license: import_license_from_source(&source_evidence),
        exports,
        reviewed_adapters: Vec::new(),
        files_kept: Vec::new(),
        files_written: Vec::new(),
        files_rejected: Vec::new(),
        files_considered: Vec::new(),
        overwrite_policy: forge_import_overwrite_policy(),
        materialization_ready,
        materialization_status: "plan-only-review-required".to_string(),
        materialized: false,
        accepted_materialization_receipt_present: false,
        materialized_package_id: None,
        materialized_files: Vec::new(),
        accepted_plan_path: None,
        accepted_plan_status: "not-provided".to_string(),
        accepted_plan_findings: Vec::new(),
        manifest_path: None,
        receipt_path: None,
        docs_path: None,
        import_plan_path: None,
        import_plan_sr_path: None,
        import_plan_machine_path: None,
        import_plan_json_machine_path: None,
        inspected_inputs: vec![
            format!("{ecosystem_segment}:{package_name}"),
            "local project contract".to_string(),
            "Forge import firewall policy".to_string(),
            source_dir
                .map(|path| format!("source-dir:{}", path.display()))
                .unwrap_or_else(|| "source-dir:not-provided".to_string()),
        ],
        review_required: vec![
            "Resolve package metadata, integrity, license, advisory state, and source provenance without running package-manager installs.".to_string(),
            "Review package exports, dependency graph, runtime target, and side-effect boundaries before writing source-owned files.".to_string(),
            "Use --source-dir with an inspected unpacked package source when materializing a Forge snapshot.".to_string(),
        ],
        can_materialize: vec![
            "Source directories that have already been acquired, unpacked, and reviewed outside package-manager install trees.".to_string(),
            "Small source slices that pass path safety, UTF-8 source, file-count, and byte-budget checks.".to_string(),
            "Forge manifest, receipt, docs, import-plan .sr, and serializer .machine metadata.".to_string(),
        ],
        cannot_materialize: surface
            .manual_review_triggers
            .iter()
            .map(|trigger| format!("Manual review required: {trigger}."))
            .collect(),
        review_findings: findings.clone(),
        findings,
        next_commands: vec![
            format!(
                "dx forge import {ecosystem_segment} {package_name} --plan --source-dir <inspected-source-dir> --output <accepted-import-plan.json>"
            ),
            format!(
                "dx forge import {ecosystem_segment} {package_name} --write --source-dir <inspected-source-dir> --from-plan <accepted-import-plan.json>"
            ),
            "Run dx check . --strict-project-contract after materialization.".to_string(),
        ],
    };
    if materialization_ready {
        if let Some(source_dir) = source_dir.filter(|path| path.is_dir()) {
            match collect_source_dir_files(
                project,
                ecosystem,
                &report.package_name,
                source_dir,
                &report.selected_files,
            )
            .and_then(|source_files| source_snapshot_file_dispositions(project, &source_files))
            {
                Ok(dispositions) => {
                    report.files_considered = dispositions;
                }
                Err(error) => {
                    report.passed = false;
                    report.materialization_ready = false;
                    report.findings.push(format!(
                        "source snapshot could not be planned for accepted receipt review: {error}"
                    ));
                }
            }
        }
    }
    refresh_import_plan_status(&mut report);
    if write_artifacts {
        write_import_plan_artifacts(project, &mut report)?;
    }
    Ok(report)
}

pub(super) fn build_forge_import_write_report(
    project: &Path,
    ecosystem: &str,
    package_name: &str,
    source_dir: Option<&Path>,
    selected_files: &[String],
    accepted_plan: Option<&Path>,
    fail_under: u8,
) -> anyhow::Result<DxForgeImportPlanReport> {
    let source_dir = source_dir.with_context(|| {
        format!(
            "dx forge import {ecosystem} {package_name} --write requires --source-dir <inspected-source-dir>"
        )
    })?;
    let ecosystem = parse_import_ecosystem(ecosystem)?;
    let ecosystem_segment = ecosystem.as_segment();
    let selected_files = normalize_selected_source_files(selected_files)?;
    let source_evidence = if source_dir.is_dir() {
        inspect_import_source_evidence(source_dir, &selected_files)?
    } else {
        DxForgeImportSourceEvidence::default()
    };
    let mut report = build_forge_import_plan_report_with_selection_and_artifact_mode(
        project,
        ecosystem_segment,
        package_name,
        Some(source_dir),
        &selected_files,
        fail_under,
        false,
    )?;
    report.mode = "write".to_string();
    report.accepted_plan_status = if accepted_plan.is_some() {
        "pending-validation".to_string()
    } else {
        "not-provided".to_string()
    };

    if !report.no_node_modules {
        let score_report = build_import_score_report(ImportScoreReportInput {
            no_node_modules: report.no_node_modules,
            source_dir_ready: source_dir.is_dir(),
            materialized: false,
            accepted_materialization_receipt_present: false,
            evidence: &source_evidence,
            exports: &report.exports,
            files_considered: 0,
            files_rejected: 0,
            package_installs_run: report.package_installs_run,
            lifecycle_scripts_executed: report.lifecycle_scripts_executed,
            risk_flags: report.risk_flags.clone(),
        });
        apply_import_score(&mut report, score_report);
        report.passed = false;
        report.materialization_ready = false;
        report
            .findings
            .push("write mode refused to materialize while node_modules exists.".to_string());
        refresh_import_plan_status(&mut report);
        write_import_plan_artifacts(project, &mut report)?;
        return Ok(report);
    }

    if !matches!(
        report.disposition.kind,
        DxForgePackageDispositionKind::Slice
    ) {
        report.passed = false;
        report.materialization_ready = false;
        report.findings.push(format!(
            "write mode refused to materialize because package disposition is `{}` on route `{}`.",
            disposition_kind(&report.disposition),
            report.disposition.route
        ));
        refresh_import_plan_status(&mut report);
        write_import_plan_artifacts(project, &mut report)?;
        return Ok(report);
    }

    if !source_snapshot_preflight_cleared(&report) {
        report.passed = false;
        report.materialization_ready = false;
        report.findings.push(format!(
            "write mode refused to materialize because preflight caps remain: {}.",
            materialization_blocking_cap_ids(&report).join(", ")
        ));
        refresh_import_plan_status(&mut report);
        write_import_plan_artifacts(project, &mut report)?;
        return Ok(report);
    }

    if accepted_plan.is_none() {
        report.accepted_plan_status = "missing-reviewed-plan".to_string();
        report.passed = false;
        report.materialization_ready = false;
        report.findings.push(
            "write mode refused to materialize reviewed source without --from-plan <accepted-import-plan.json>; run --plan first, review the receipt, then pass that plan to --write."
                .to_string(),
        );
        refresh_import_plan_status(&mut report);
        write_import_plan_artifacts(project, &mut report)?;
        return Ok(report);
    }

    let source_files = collect_source_dir_files(
        project,
        ecosystem,
        &report.package_name,
        source_dir,
        &report.selected_files,
    )?;
    if let Some(accepted_plan) = accepted_plan {
        report.accepted_plan_path = Some(accepted_plan.to_path_buf());
        let accepted_plan_findings = validate_forge_import_accepted_plan(
            &report,
            accepted_plan,
            source_dir,
            &source_files,
            fail_under,
        )?;
        if accepted_plan_findings.is_empty() {
            report.accepted_plan_status = "validated".to_string();
            report.accepted_plan_findings.push(
                "accepted import plan matched ecosystem, package, source snapshot, selected files, and fail-under threshold."
                    .to_string(),
            );
        } else {
            report.accepted_plan_status = "mismatch".to_string();
            report.accepted_plan_findings = accepted_plan_findings;
            report.passed = false;
            report.materialization_ready = false;
            report.findings.push(
                "write mode refused to materialize because the accepted import plan did not match the current write request."
                    .to_string(),
            );
            report
                .findings
                .extend(report.accepted_plan_findings.clone());
            refresh_import_plan_status(&mut report);
            write_import_plan_artifacts(project, &mut report)?;
            return Ok(report);
        }
    }
    let source_slice = classify_import_slice(DxForgeImportSliceCandidate {
        requested_kind: DxForgeImportSliceKind::SourceSlice,
        risk_flags: report.risk_flags.clone(),
        writes_importable_source: true,
    });
    record_source_slice_policy(&mut report, &source_slice);
    if source_slice.decision != DxForgeImportDecision::Accept {
        let score_report = build_import_score_report(ImportScoreReportInput {
            no_node_modules: report.no_node_modules,
            source_dir_ready: source_dir.is_dir(),
            materialized: false,
            accepted_materialization_receipt_present: false,
            evidence: &source_evidence,
            exports: &report.exports,
            files_considered: 0,
            files_rejected: 1,
            package_installs_run: report.package_installs_run,
            lifecycle_scripts_executed: report.lifecycle_scripts_executed,
            risk_flags: report.risk_flags.clone(),
        });
        apply_import_score(&mut report, score_report);
        report.passed = false;
        report.materialization_ready = false;
        if report.risk_flags.is_empty() {
            report.risk_flags = vec![DxForgeImportRiskFlag::SideEffectImport];
        }
        report.findings.push(
            "Forge import slice policy did not accept app-importable source materialization."
                .to_string(),
        );
        refresh_import_plan_status(&mut report);
        write_import_plan_artifacts(project, &mut report)?;
        return Ok(report);
    }

    let dispositions = source_snapshot_file_dispositions(project, &source_files)?;
    report.files_considered = dispositions.clone();
    let rejected = dispositions
        .iter()
        .filter(|disposition| disposition.status == "rejected")
        .collect::<Vec<_>>();
    if !rejected.is_empty() {
        let score_report = build_import_score_report(ImportScoreReportInput {
            no_node_modules: report.no_node_modules,
            source_dir_ready: source_dir.is_dir(),
            materialized: false,
            accepted_materialization_receipt_present: false,
            evidence: &source_evidence,
            exports: &report.exports,
            files_considered: dispositions.len(),
            files_rejected: rejected.len(),
            package_installs_run: report.package_installs_run,
            lifecycle_scripts_executed: report.lifecycle_scripts_executed,
            risk_flags: report.risk_flags.clone(),
        });
        apply_import_score(&mut report, score_report);
        report.passed = false;
        report.materialization_ready = false;
        report.files_rejected = rejected
            .iter()
            .map(|disposition| disposition.path.clone())
            .collect();
        report.findings.extend(
            rejected
                .into_iter()
                .map(|disposition| disposition.message.clone()),
        );
        refresh_import_plan_status(&mut report);
        write_import_plan_artifacts(project, &mut report)?;
        return Ok(report);
    }

    let pre_write_score_report = build_import_score_report(ImportScoreReportInput {
        no_node_modules: report.no_node_modules,
        source_dir_ready: source_dir.is_dir(),
        materialized: true,
        accepted_materialization_receipt_present: true,
        evidence: &source_evidence,
        exports: &report.exports,
        files_considered: dispositions.len(),
        files_rejected: 0,
        package_installs_run: report.package_installs_run,
        lifecycle_scripts_executed: report.lifecycle_scripts_executed,
        risk_flags: report.risk_flags.clone(),
    });
    apply_import_score(&mut report, pre_write_score_report);
    if report.score < fail_under {
        report.passed = false;
        report.materialization_ready = false;
        report.findings.push(format!(
            "write mode refused to materialize because projected score {} is below fail-under threshold {}.",
            report.score, fail_under
        ));
        refresh_import_plan_status(&mut report);
        write_import_plan_artifacts(project, &mut report)?;
        return Ok(report);
    }

    let source_transaction = write_materialized_source_files(project, &source_files)?;
    let external_state_snapshot =
        DxForgeExternalStateSnapshot::capture(project, &report.package_id)?;
    let outcome = match write_forge_external_source(
        DxForgeExternalSourcePackage {
            package_id: report.package_id.clone(),
            variant: "default".to_string(),
            ecosystem: ecosystem_segment.to_string(),
            package_name: report.package_name.clone(),
            version: "source-dir-snapshot".to_string(),
            license: report.license.declared_license.clone(),
            files: source_files
                .iter()
                .map(|file| DxForgeLocalSourceFile {
                    path: file.materialized_path.clone(),
                    content: file.content.clone(),
                })
                .collect(),
        },
        project,
    ) {
        Ok(outcome) => outcome,
        Err(error) => {
            let rollback_findings = source_transaction.rollback_created_source_files(project);
            if rollback_findings.is_empty() {
                return Err(error);
            }
            anyhow::bail!(
                "{}; rollback findings: {}",
                error,
                rollback_findings.join("; ")
            );
        }
    };

    report.manifest_path = outcome.manifest_path;
    report.receipt_path = outcome.receipt_path;
    report.accepted_materialization_receipt_present = report.receipt_path.is_some();

    let score_report = build_import_score_report(ImportScoreReportInput {
        no_node_modules: report.no_node_modules,
        source_dir_ready: source_dir.is_dir(),
        materialized: true,
        accepted_materialization_receipt_present: report.accepted_materialization_receipt_present,
        evidence: &source_evidence,
        exports: &report.exports,
        files_considered: dispositions.len(),
        files_rejected: 0,
        package_installs_run: report.package_installs_run,
        lifecycle_scripts_executed: report.lifecycle_scripts_executed,
        risk_flags: report.risk_flags.clone(),
    });
    apply_import_score(&mut report, score_report);
    report.passed = report.score >= fail_under;
    report.materialized = true;
    report.materialized_package_id = Some(report.package_id.clone());
    report.materialized_files = source_files
        .iter()
        .map(|file| file.materialized_path.clone())
        .collect();
    for export in &mut report.exports {
        export.materialized = true;
    }
    report.reviewed_adapters = reviewed_adapters_for_materialized_files(
        ecosystem,
        &report.package_name,
        &report.package_id,
        &source_files,
    )?;
    report.files_kept = dispositions
        .iter()
        .filter(|disposition| disposition.status == "kept")
        .map(|disposition| disposition.path.clone())
        .collect();
    report.files_written = dispositions
        .iter()
        .filter(|disposition| disposition.status == "written")
        .map(|disposition| disposition.path.clone())
        .collect();
    report.docs_path = Some(project.join(import_docs_path(&report.package_id)));
    report.findings.clear();
    refresh_import_plan_status(&mut report);
    if let Err(error) = write_import_plan_artifacts(project, &mut report) {
        let mut rollback_findings = Vec::new();
        rollback_findings
            .extend(external_state_snapshot.rollback(project, report.receipt_path.as_deref()));
        rollback_findings.extend(source_transaction.rollback_created_source_files(project));
        if rollback_findings.is_empty() {
            return Err(error);
        }
        anyhow::bail!(
            "{}; rollback findings: {}",
            error,
            rollback_findings.join("; ")
        );
    }

    Ok(report)
}

pub(super) fn forge_import_plan_terminal(report: &DxForgeImportPlanReport) -> String {
    let mut output = format!(
        "DX Forge import plan\nEcosystem: {}\nPackage: {}\nMode: {}\nPassed: {}\nScore: {} / 100\nUncapped score: {} / 100\nScore ceiling: {} / 100\nTraffic: {}\nPackage disposition: {}\nDisposition route: {}\nMaterialization boundary: {}\nAccepted materialization receipt present: {}\nCapability tier: {}\nCapability score: {} / 100\nDirect WWW bare import: {}\nUniversal package compatibility claim: {}\nClean package can score 100: {}\nOwnership claim: {}\nImport alias: {}\nSource kind: {}\nSource dir ready: {}\nSource provenance verified: {}\nSource license reviewed: {}\nSource advisory reviewed: {}\nSource SBOM present: {}\nNo node_modules: {}\nPackage installs run: {}\nLifecycle status: {}\nLive fetching enabled: {}\nPackage manager execution allowed: {}\nAccepted import receipt required: {}\nUnsupported dx add form: {}\nMaterialization status: {}\nMaterialization ready: {}\nMaterialized: {}\nAccepted plan status: {}\n",
        report.ecosystem,
        report.package_name,
        report.mode,
        report.passed,
        report.score,
        report.uncapped_score,
        report.score_ceiling,
        report.traffic,
        disposition_kind(&report.disposition),
        report.disposition.route,
        materialization_boundary(report),
        report.accepted_materialization_receipt_present,
        capability_tier(&report.ecosystem_capability),
        report.ecosystem_capability.capability_score,
        report.ecosystem_capability.direct_www_bare_import,
        report
            .ecosystem_capability
            .universal_package_compatibility_claim,
        report.ecosystem_capability.package_score_can_reach_100,
        report.disposition.ownership_claim,
        report.import_alias,
        report.source_kind,
        report.source_dir_ready,
        report.source_provenance_verified,
        report.source_license_reviewed,
        report.source_advisory_reviewed,
        report.source_sbom_present,
        report.no_node_modules,
        report.package_installs_run,
        report.lifecycle_script_status,
        report.live_fetching_enabled,
        report.package_manager_execution_allowed,
        report.accepted_import_receipt_required,
        report.unsupported_dx_add_form,
        report.materialization_status,
        report.materialization_ready,
        report.materialized,
        report.accepted_plan_status
    );
    if let Some(path) = &report.accepted_plan_path {
        output.push_str(&format!("Accepted plan path: {}\n", path.display()));
    }
    if !report.accepted_plan_findings.is_empty() {
        output.push_str("Accepted plan findings:\n");
        for finding in &report.accepted_plan_findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output.push_str(&format!(
        "Write transaction mode: {}\nWrite transaction rollback: {}\n",
        report.write_transaction_mode, report.write_transaction_rollback
    ));
    if !report.write_transaction_scope.is_empty() {
        output.push_str("Write transaction scope:\n");
        for scope in &report.write_transaction_scope {
            output.push_str(&format!("- {scope}\n"));
        }
    }
    if !report.write_transaction_limitations.is_empty() {
        output.push_str("Write transaction limitations:\n");
        for limitation in &report.write_transaction_limitations {
            output.push_str(&format!("- {limitation}\n"));
        }
    }
    if !report.acquisition_metadata_inputs.is_empty() {
        output.push_str("Acquisition metadata inputs:\n");
        for input in &report.acquisition_metadata_inputs {
            output.push_str(&format!("- {input}\n"));
        }
    }
    if !report.acquisition_artifact_inputs.is_empty() {
        output.push_str("Acquisition artifact inputs:\n");
        for input in &report.acquisition_artifact_inputs {
            output.push_str(&format!("- {input}\n"));
        }
    }
    if !report.forbidden_commands.is_empty() {
        output.push_str("Forbidden commands:\n");
        for command in &report.forbidden_commands {
            output.push_str(&format!("- {command}\n"));
        }
    }
    if !report.ecosystem_capability.honest_limitations.is_empty() {
        output.push_str("Capability limitations:\n");
        for limitation in &report.ecosystem_capability.honest_limitations {
            output.push_str(&format!("- {limitation}\n"));
        }
    }
    if !report
        .ecosystem_capability
        .clean_package_requirements
        .is_empty()
    {
        output.push_str("Clean package requirements:\n");
        for requirement in &report.ecosystem_capability.clean_package_requirements {
            output.push_str(&format!("- {requirement}\n"));
        }
    }
    if !report
        .ecosystem_capability
        .score_100_requirements
        .is_empty()
    {
        output.push_str("Score 100 requirements:\n");
        for requirement in &report.ecosystem_capability.score_100_requirements {
            output.push_str(&format!("- {requirement}\n"));
        }
    }
    output.push_str(&format!(
        "Bridge kind: {}\nImportable source: {}\nMaterializes source: {}\nRequires accepted receipt: {}\nDisposition reason: {}\nDisposition remediation: {}\nFiles considered: {}\nFiles written: {}\nFiles kept: {}\nFiles rejected: {}\n",
        disposition_bridge_kind(&report.disposition),
        report.disposition.importable_source,
        report.disposition.materializes_source,
        report.disposition.requires_accepted_receipt,
        report.disposition.reason,
        report.disposition.remediation,
        report.files_considered.len(),
        report.files_written.len(),
        report.files_kept.len(),
        report.files_rejected.len(),
    ));
    if let Some(path) = &report.import_plan_path {
        output.push_str(&format!("Import plan json: {}\n", path.display()));
    }
    if !report.materialized_files.is_empty() {
        output.push_str("Materialized files:\n");
        for file in &report.materialized_files {
            output.push_str(&format!("- {file}\n"));
        }
    }
    if let Some(path) = &report.import_plan_sr_path {
        output.push_str(&format!("Import plan sr: {}\n", path.display()));
    }
    if let Some(path) = &report.import_plan_machine_path {
        output.push_str(&format!("Import plan machine: {}\n", path.display()));
    }
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    if !report.files_rejected.is_empty() {
        output.push_str("Rejected files:\n");
        for file in report
            .files_considered
            .iter()
            .filter(|file| file.status == "rejected")
        {
            output.push_str(&format!(
                "- {} [{} / {}]: {}\n",
                file.path, file.traffic, file.decision, file.message
            ));
        }
    }
    output
}

pub(super) fn forge_import_plan_markdown(report: &DxForgeImportPlanReport) -> String {
    let mut output = format!(
        "# DX Forge Import Plan\n\n- Ecosystem: `{}`\n- Package: `{}`\n- Mode: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Uncapped score: `{}` / `100`\n- Score ceiling: `{}` / `100`\n- Traffic: `{}`\n- Required score: `{}` / `100`\n- Package disposition: `{}`\n- Disposition route: `{}`\n- Materialization boundary: `{}`\n- Accepted materialization receipt present: `{}`\n- Capability tier: `{}`\n- Capability score: `{}` / `100`\n- Direct WWW bare import: `{}`\n- Universal package compatibility claim: `{}`\n- Clean package can score 100: `{}`\n- Ownership claim: `{}`\n- Import alias: `{}`\n- Source kind: `{}`\n- Forge import gate: `{}`\n- Source dir ready: `{}`\n- Source provenance verified: `{}`\n- Source license reviewed: `{}`\n- Source advisory reviewed: `{}`\n- Source SBOM present: `{}`\n- No `node_modules`: `{}`\n- Package installs run: `{}`\n- Lifecycle status: `{}`\n- Materialization status: `{}`\n- Materialization ready: `{}`\n- Materialized: `{}`\n- Accepted plan status: `{}`\n- Write transaction mode: `{}`\n- Write transaction rollback: `{}`\n\n",
        markdown_table_cell(&report.ecosystem),
        markdown_table_cell(&report.package_name),
        report.mode,
        report.passed,
        report.score,
        report.uncapped_score,
        report.score_ceiling,
        markdown_table_cell(&report.traffic),
        report.fail_under,
        markdown_table_cell(disposition_kind(&report.disposition)),
        markdown_table_cell(&report.disposition.route),
        markdown_table_cell(materialization_boundary(report)),
        report.accepted_materialization_receipt_present,
        markdown_table_cell(capability_tier(&report.ecosystem_capability)),
        report.ecosystem_capability.capability_score,
        report.ecosystem_capability.direct_www_bare_import,
        report
            .ecosystem_capability
            .universal_package_compatibility_claim,
        report.ecosystem_capability.package_score_can_reach_100,
        markdown_table_cell(&report.disposition.ownership_claim),
        markdown_table_cell(&report.import_alias),
        markdown_table_cell(&report.source_kind),
        report.forge_import_gate,
        report.source_dir_ready,
        report.source_provenance_verified,
        report.source_license_reviewed,
        report.source_advisory_reviewed,
        report.source_sbom_present,
        report.no_node_modules,
        report.package_installs_run,
        markdown_table_cell(&report.lifecycle_script_status),
        markdown_table_cell(&report.materialization_status),
        report.materialization_ready,
        report.materialized,
        markdown_table_cell(&report.accepted_plan_status),
        markdown_table_cell(&report.write_transaction_mode),
        markdown_table_cell(&report.write_transaction_rollback)
    );

    if let Some(path) = &report.accepted_plan_path {
        output.push_str(&format!(
            "Accepted plan path: `{}`\n\n",
            markdown_table_cell(&path.display().to_string())
        ));
    }

    if !report.accepted_plan_findings.is_empty() {
        output.push_str("## Accepted Plan Findings\n\n");
        for finding in &report.accepted_plan_findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
        output.push('\n');
    }

    if !report.write_transaction_limitations.is_empty() {
        output.push_str("## Write Transaction Limitations\n\n");
        for limitation in &report.write_transaction_limitations {
            output.push_str(&format!("- {}\n", markdown_table_cell(limitation)));
        }
        output.push('\n');
    }

    if !report.materialized_files.is_empty() {
        output.push_str("## Materialized Files\n\n");
        for file in &report.materialized_files {
            output.push_str(&format!("- `{}`\n", markdown_table_cell(file)));
        }
        output.push('\n');
    }

    if !report.score_dimensions.is_empty() {
        output.push_str("## Score Dimensions\n\n");
        output.push_str("| Dimension | Score | Evidence |\n|---|---:|---|\n");
        for dimension in &report.score_dimensions {
            output.push_str(&format!(
                "| {} | {}/{} | {} |\n",
                markdown_table_cell(&dimension.label),
                dimension.score,
                dimension.max,
                markdown_table_cell(&dimension.evidence)
            ));
        }
        output.push('\n');
    }

    if !report.applied_caps.is_empty() {
        output.push_str("## Applied Caps\n\n");
        output.push_str("| Cap | Ceiling | Reason |\n|---|---:|---|\n");
        for cap in &report.applied_caps {
            output.push_str(&format!(
                "| {} | {} | {} |\n",
                markdown_table_cell(&cap.id),
                cap.ceiling,
                markdown_table_cell(&cap.reason)
            ));
        }
        output.push('\n');
    }

    output.push_str("## Ecosystem Capability\n\n");
    output.push_str("| Capability | Value |\n|---|---|\n");
    for (label, value) in [
        (
            "Model",
            report
                .ecosystem_capability
                .model_version
                .as_str()
                .to_string(),
        ),
        (
            "Tier",
            capability_tier(&report.ecosystem_capability).to_string(),
        ),
        (
            "Capability score",
            format!("{}/100", report.ecosystem_capability.capability_score),
        ),
        (
            "Direct WWW bare import",
            report
                .ecosystem_capability
                .direct_www_bare_import
                .to_string(),
        ),
        (
            "Universal package compatibility claim",
            report
                .ecosystem_capability
                .universal_package_compatibility_claim
                .to_string(),
        ),
        (
            "Clean package can score 100",
            report
                .ecosystem_capability
                .package_score_can_reach_100
                .to_string(),
        ),
    ] {
        output.push_str(&format!(
            "| {} | {} |\n",
            markdown_table_cell(label),
            markdown_table_cell(&value)
        ));
    }
    output.push('\n');

    if !report.ecosystem_capability.honest_limitations.is_empty() {
        output.push_str("### Capability Limitations\n\n");
        for limitation in &report.ecosystem_capability.honest_limitations {
            output.push_str(&format!("- {}\n", markdown_table_cell(limitation)));
        }
        output.push('\n');
    }

    if !report
        .ecosystem_capability
        .score_100_requirements
        .is_empty()
    {
        output.push_str("### Score 100 Requirements\n\n");
        for requirement in &report.ecosystem_capability.score_100_requirements {
            output.push_str(&format!("- {}\n", markdown_table_cell(requirement)));
        }
        output.push('\n');
    }

    if let Some(path) = &report.import_plan_sr_path {
        output.push_str("## Import Plan Artifacts\n\n");
        if let Some(json) = &report.import_plan_path {
            output.push_str(&format!(
                "- `.json`: `{}`\n",
                markdown_table_cell(&json.display().to_string())
            ));
        }
        output.push_str(&format!(
            "- `.sr`: `{}`\n",
            markdown_table_cell(&path.display().to_string())
        ));
        if let Some(machine) = &report.import_plan_machine_path {
            output.push_str(&format!(
                "- `.machine`: `{}`\n",
                markdown_table_cell(&machine.display().to_string())
            ));
        }
        output.push('\n');
    }

    output.push_str("## Review Required\n\n");
    for item in &report.review_required {
        output.push_str(&format!("- {}\n", markdown_table_cell(item)));
    }

    output.push_str("\n## Can Materialize\n\n");
    for item in &report.can_materialize {
        output.push_str(&format!("- {}\n", markdown_table_cell(item)));
    }

    output.push_str("\n## Cannot Materialize\n\n");
    for item in &report.cannot_materialize {
        output.push_str(&format!("- {}\n", markdown_table_cell(item)));
    }

    output
}

pub(super) fn forge_import_plan_failure_summary(report: &DxForgeImportPlanReport) -> String {
    if report.findings.is_empty() {
        format!(
            "Forge import plan score {} is below threshold",
            report.score
        )
    } else {
        report.findings.join("; ")
    }
}

#[derive(Debug, Clone)]
struct DxForgeMaterializedSourceFile {
    materialized_path: String,
    logical_path: String,
    content: String,
}

fn reviewed_adapters_for_materialized_files(
    ecosystem: DxForgeImportEcosystem,
    package_name: &str,
    package_id: &str,
    files: &[DxForgeMaterializedSourceFile],
) -> anyhow::Result<Vec<DxForgeImportReviewedAdapter>> {
    let Some(ecosystem_segment) = reviewed_javascript_adapter_ecosystem(ecosystem) else {
        return Ok(Vec::new());
    };
    let package_slug = forge_import_plan_slug(package_name)?;

    let mut seen = BTreeSet::new();
    let mut adapters = Vec::new();
    for file in files {
        let Some((specifier, source_path, root)) = reviewed_javascript_adapter_specifier(
            ecosystem_segment,
            &package_slug,
            package_name,
            &file.materialized_path,
        ) else {
            continue;
        };
        if !seen.insert((specifier.clone(), file.materialized_path.clone())) {
            continue;
        }
        adapters.push(DxForgeImportReviewedAdapter {
            specifier,
            package_id: package_id.to_string(),
            package_name: package_name.to_string(),
            materialized_path: file.materialized_path.clone(),
            source_path,
            root,
        });
    }
    Ok(adapters)
}

fn reviewed_javascript_adapter_ecosystem(
    ecosystem: DxForgeImportEcosystem,
) -> Option<&'static str> {
    match ecosystem {
        DxForgeImportEcosystem::Npm => Some("npm"),
        DxForgeImportEcosystem::Jsr => Some("jsr"),
        _ => None,
    }
}

fn reviewed_javascript_adapter_specifier(
    ecosystem_segment: &str,
    package_slug: &str,
    package_name: &str,
    materialized_path: &str,
) -> Option<(String, String, bool)> {
    let path = materialized_path.replace('\\', "/");
    let prefix = format!("lib/forge/{ecosystem_segment}/{package_slug}/");
    let source_relative = path.strip_prefix(&prefix)?;
    let source_without_extension = strip_js_ts_extension(source_relative)?;
    if source_without_extension == "index" {
        return Some((package_name.to_string(), source_relative.to_string(), true));
    }
    let subpath = source_without_extension
        .strip_suffix("/index")
        .unwrap_or(source_without_extension);
    Some((
        format!("{package_name}/{subpath}"),
        source_relative.to_string(),
        false,
    ))
}

fn strip_js_ts_extension(path: &str) -> Option<&str> {
    [".tsx", ".ts", ".jsx", ".js", ".mjs", ".cjs"]
        .iter()
        .find_map(|extension| path.strip_suffix(extension))
}

#[derive(Debug, Clone)]
struct DxForgeMaterializedSourceWrite {
    relative_path: String,
    expected_hash: String,
}

#[derive(Debug, Default)]
struct DxForgeImportWriteTransaction {
    created_source_files: Vec<DxForgeMaterializedSourceWrite>,
}

#[derive(Debug, Clone)]
struct DxForgeImportFileSnapshot {
    path: PathBuf,
    bytes: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
struct DxForgeExternalStateSnapshot {
    manifest: DxForgeImportFileSnapshot,
    docs: DxForgeImportFileSnapshot,
}

fn parse_import_ecosystem(ecosystem: &str) -> anyhow::Result<DxForgeImportEcosystem> {
    DxForgeImportEcosystem::from_segment(ecosystem).ok_or_else(|| {
        anyhow::anyhow!(
            "unsupported Forge import ecosystem `{}`; expected {}; aliases: {}",
            ecosystem.trim(),
            DxForgeImportEcosystem::supported_segments_help(),
            DxForgeImportEcosystem::supported_aliases_help()
        )
    })
}

fn import_plan_surface(
    ecosystem: DxForgeImportEcosystem,
) -> dx_compiler::ecosystem::DxForgeImportPlanSurface {
    match ecosystem {
        DxForgeImportEcosystem::Npm => dx_compiler::ecosystem::npm_import_plan_surface(),
        DxForgeImportEcosystem::Pip => dx_compiler::ecosystem::pip_import_plan_surface(),
        DxForgeImportEcosystem::Cargo => dx_compiler::ecosystem::cargo_import_plan_surface(),
        DxForgeImportEcosystem::Go => dx_compiler::ecosystem::go_import_plan_surface(),
        DxForgeImportEcosystem::Pub => dx_compiler::ecosystem::pub_import_plan_surface(),
        DxForgeImportEcosystem::Maven => dx_compiler::ecosystem::maven_import_plan_surface(),
        DxForgeImportEcosystem::Nuget => dx_compiler::ecosystem::nuget_import_plan_surface(),
        DxForgeImportEcosystem::Composer => dx_compiler::ecosystem::composer_import_plan_surface(),
        DxForgeImportEcosystem::Gem => dx_compiler::ecosystem::gem_import_plan_surface(),
        DxForgeImportEcosystem::Swift => dx_compiler::ecosystem::swift_import_plan_surface(),
        DxForgeImportEcosystem::Jsr => dx_compiler::ecosystem::jsr_import_plan_surface(),
        DxForgeImportEcosystem::Hex => dx_compiler::ecosystem::hex_import_plan_surface(),
        DxForgeImportEcosystem::Cran => dx_compiler::ecosystem::cran_import_plan_surface(),
    }
}

fn import_package_id(
    ecosystem: DxForgeImportEcosystem,
    package_name: &str,
) -> anyhow::Result<String> {
    validate_import_package_name(ecosystem, package_name)?;
    Ok(format!(
        "{}/{}",
        ecosystem.as_segment(),
        package_name.trim()
    ))
}

fn import_alias(ecosystem: DxForgeImportEcosystem, package_name: &str, package_id: &str) -> String {
    if matches!(
        ecosystem,
        DxForgeImportEcosystem::Npm | DxForgeImportEcosystem::Jsr
    ) {
        package_name.to_string()
    } else {
        package_id.to_string()
    }
}

fn normalize_selected_source_files(selected_files: &[String]) -> anyhow::Result<Vec<String>> {
    let mut normalized = BTreeSet::new();
    for selected_file in selected_files {
        let raw = selected_file.trim().replace('\\', "/");
        if raw.is_empty() {
            continue;
        }
        let safe = validate_import_relative_path(&raw)
            .with_context(|| format!("validate selected Forge source file `{raw}`"))?;
        if !is_materializable_source_file(&safe.path) {
            anyhow::bail!(
                "selected Forge source file `{}` is not a materializable source file",
                safe.path
            );
        }
        normalized.insert(safe.path);
    }
    Ok(normalized.into_iter().collect())
}

fn source_file_selected(relative: &str, selected_files: &[String]) -> bool {
    selected_files.is_empty() || selected_files.iter().any(|selected| selected == relative)
}

fn validate_forge_import_accepted_plan(
    report: &DxForgeImportPlanReport,
    accepted_plan: &Path,
    source_dir: &Path,
    source_files: &[DxForgeMaterializedSourceFile],
    fail_under: u8,
) -> anyhow::Result<Vec<String>> {
    let raw_plan = fs::read_to_string(accepted_plan).with_context(|| {
        format!(
            "read accepted Forge import plan `{}`",
            accepted_plan.display()
        )
    })?;
    let plan: serde_json::Value = serde_json::from_str(&raw_plan).with_context(|| {
        format!(
            "parse accepted Forge import plan `{}`",
            accepted_plan.display()
        )
    })?;
    let mut findings = Vec::new();

    validate_plan_string(&plan, "schema", FORGE_IMPORT_PLAN_SCHEMA, &mut findings);
    validate_plan_string(&plan, "mode", "plan-only", &mut findings);
    validate_plan_string(&plan, "ecosystem", &report.ecosystem, &mut findings);
    validate_plan_string(&plan, "package_id", &report.package_id, &mut findings);
    validate_plan_string(&plan, "package_name", &report.package_name, &mut findings);
    validate_plan_bool(&plan, "passed", true, &mut findings);
    validate_plan_bool(&plan, "source_dir_ready", true, &mut findings);
    validate_plan_bool(&plan, "no_node_modules", true, &mut findings);
    validate_plan_bool(&plan, "package_installs_run", false, &mut findings);
    validate_plan_bool(&plan, "lifecycle_scripts_executed", false, &mut findings);
    validate_plan_string_array(
        &plan,
        "selected_files",
        &report.selected_files,
        &mut findings,
    );
    validate_plan_u64_at_least(
        &plan,
        "fail_under",
        fail_under as u64,
        "accepted plan threshold is weaker than the current write threshold",
        &mut findings,
    );
    validate_plan_u64_at_least(
        &plan,
        "score",
        fail_under as u64,
        "accepted plan score is below the current write threshold",
        &mut findings,
    );
    validate_plan_u64_at_least(
        &plan,
        "source_files_inspected_count",
        source_files.len() as u64,
        "accepted plan inspected fewer source files than the current write snapshot",
        &mut findings,
    );
    validate_plan_u64_at_least(
        &plan,
        "source_dependency_count",
        report.source_dependency_count as u64,
        "accepted plan inspected a smaller dependency graph than the current write snapshot",
        &mut findings,
    );
    validate_plan_source_snapshot(&plan, source_files, &mut findings);

    let expected_source_dir =
        normalize_forge_accepted_plan_value(&format!("source-dir:{}", source_dir.display()));
    let inspected_inputs = plan
        .get("inspected_inputs")
        .and_then(serde_json::Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(normalize_forge_accepted_plan_value)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if !inspected_inputs
        .iter()
        .any(|input| input == &expected_source_dir)
    {
        findings.push(format!(
            "accepted plan inspected_inputs did not include `{}`",
            expected_source_dir
        ));
    }

    Ok(findings)
}

fn validate_plan_string(
    plan: &serde_json::Value,
    field: &str,
    expected: &str,
    findings: &mut Vec<String>,
) {
    match plan.get(field).and_then(serde_json::Value::as_str) {
        Some(actual) if actual == expected => {}
        Some(actual) => findings.push(format!(
            "accepted plan `{field}` expected `{expected}` but found `{actual}`"
        )),
        None => findings.push(format!("accepted plan is missing string field `{field}`")),
    }
}

fn validate_plan_bool(
    plan: &serde_json::Value,
    field: &str,
    expected: bool,
    findings: &mut Vec<String>,
) {
    match plan.get(field).and_then(serde_json::Value::as_bool) {
        Some(actual) if actual == expected => {}
        Some(actual) => findings.push(format!(
            "accepted plan `{field}` expected `{expected}` but found `{actual}`"
        )),
        None => findings.push(format!("accepted plan is missing boolean field `{field}`")),
    }
}

fn validate_plan_u64_at_least(
    plan: &serde_json::Value,
    field: &str,
    minimum: u64,
    detail: &str,
    findings: &mut Vec<String>,
) {
    match plan.get(field).and_then(serde_json::Value::as_u64) {
        Some(actual) if actual >= minimum => {}
        Some(actual) => findings.push(format!(
            "{detail}: `{field}` expected at least `{minimum}` but found `{actual}`"
        )),
        None => findings.push(format!("accepted plan is missing numeric field `{field}`")),
    }
}

fn validate_plan_string_array(
    plan: &serde_json::Value,
    field: &str,
    expected: &[String],
    findings: &mut Vec<String>,
) {
    let actual = plan
        .get(field)
        .and_then(serde_json::Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::to_string)
                .collect::<Vec<_>>()
        });
    match actual {
        Some(actual) if actual == expected => {}
        Some(actual) => findings.push(format!(
            "accepted plan `{field}` expected `{}` but found `{}`",
            expected.join(", "),
            actual.join(", ")
        )),
        None => findings.push(format!(
            "accepted plan is missing string array field `{field}`"
        )),
    }
}

fn validate_plan_source_snapshot(
    plan: &serde_json::Value,
    source_files: &[DxForgeMaterializedSourceFile],
    findings: &mut Vec<String>,
) {
    let expected = source_file_snapshot_keys(source_files);
    if expected.is_empty() {
        findings.push("accepted plan source snapshot has no materializable files".to_string());
        return;
    }
    let Some(files) = plan
        .get("files_considered")
        .and_then(serde_json::Value::as_array)
    else {
        findings.push("accepted plan is missing source snapshot files_considered".to_string());
        return;
    };
    if files.is_empty() {
        findings.push("accepted plan source snapshot hash list is empty".to_string());
        return;
    }

    let mut actual = Vec::new();
    for file in files {
        let path = file.get("path").and_then(serde_json::Value::as_str);
        let logical_path = file.get("logical_path").and_then(serde_json::Value::as_str);
        let hash = file.get("hash").and_then(serde_json::Value::as_str);
        let bytes = file.get("bytes").and_then(serde_json::Value::as_u64);
        match (path, logical_path, hash, bytes) {
            (Some(path), Some(logical_path), Some(hash), Some(bytes)) => {
                actual.push(source_snapshot_key(path, logical_path, hash, bytes));
            }
            _ => findings.push(
                "accepted plan source snapshot file is missing path, logical_path, hash, or bytes"
                    .to_string(),
            ),
        }
    }
    actual.sort();
    if actual != expected {
        findings.push(
            "accepted plan source snapshot hash list does not match the current source files"
                .to_string(),
        );
    }
}

fn source_file_snapshot_keys(source_files: &[DxForgeMaterializedSourceFile]) -> Vec<String> {
    let mut keys = source_files
        .iter()
        .map(|file| {
            source_snapshot_key(
                &file.materialized_path,
                &file.logical_path,
                &blake3_hex(file.content.as_bytes()),
                file.content.len() as u64,
            )
        })
        .collect::<Vec<_>>();
    keys.sort();
    keys
}

fn source_snapshot_key(path: &str, logical_path: &str, hash: &str, bytes: u64) -> String {
    format!("{path}\u{1f}{logical_path}\u{1f}{hash}\u{1f}{bytes}")
}

fn normalize_forge_accepted_plan_value(value: &str) -> String {
    value.replace('\\', "/")
}

fn import_disposition_file_count(report: &DxForgeImportPlanReport) -> usize {
    if report.files_considered.is_empty() && !report.materialized {
        report.source_files_inspected_count
    } else {
        report.files_considered.len()
    }
}

fn refresh_import_plan_status(report: &mut DxForgeImportPlanReport) {
    report.lifecycle_script_status = if report.lifecycle_scripts_executed {
        "executed".to_string()
    } else {
        "not-executed".to_string()
    };
    report.disposition = build_import_disposition_report(DxForgePackageDispositionInput {
        requested_kind: if report.materialized || report.source_dir_ready {
            DxForgeImportSliceKind::SourceSlice
        } else {
            DxForgeImportSliceKind::MetadataOnly
        },
        source_dir_ready: report.source_dir_ready,
        materialized: report.materialized,
        accepted_materialization_receipt_present: report.accepted_materialization_receipt_present,
        no_node_modules: report.no_node_modules,
        package_installs_run: report.package_installs_run,
        lifecycle_scripts_executed: report.lifecycle_scripts_executed,
        files_considered: import_disposition_file_count(report),
        files_rejected: report.files_rejected.len(),
        score: report.score,
        score_ceiling: report.score_ceiling,
        risk_flags: report.risk_flags.clone(),
    });
    if report.source_dir_ready && report.source_slice_policy_decisions.is_empty() {
        let source_slice = classify_import_slice(DxForgeImportSliceCandidate {
            requested_kind: DxForgeImportSliceKind::SourceSlice,
            risk_flags: report.risk_flags.clone(),
            writes_importable_source: true,
        });
        record_source_slice_policy(report, &source_slice);
    }
    apply_disposition_score_cap(report);
    match report.disposition.kind {
        DxForgePackageDispositionKind::Materialize => {
            report.materialization_ready = true;
            report.materialization_status = "materialized-source-owned-adapter".to_string();
        }
        DxForgePackageDispositionKind::Slice => {
            report.materialization_ready = report.source_dir_ready
                && report.no_node_modules
                && materialization_blocking_cap_ids(report).is_empty();
            report.materialization_status = if report.materialization_ready {
                "ready-reviewed-source-slice".to_string()
            } else {
                "preflight-review-required".to_string()
            };
        }
        DxForgePackageDispositionKind::Bridge => {
            report.materialization_ready = false;
            report.materialization_status = "bridge-required-before-materialization".to_string();
        }
        DxForgePackageDispositionKind::Reject => {
            report.materialization_ready = false;
            report.materialization_status = "rejected-by-forge-disposition".to_string();
        }
    }
    if !report.no_node_modules {
        report.materialization_ready = false;
        report.materialization_status = "blocked-node-modules-present".to_string();
    }
    let base_passed = report.findings.is_empty()
        && report.no_node_modules
        && !report.package_installs_run
        && !report.lifecycle_scripts_executed
        && report.score >= report.fail_under;
    report.passed = if report.mode == "write" {
        base_passed
            && report.materialized
            && matches!(
                report.disposition.kind,
                DxForgePackageDispositionKind::Materialize
            )
    } else {
        base_passed
    };
    report.review_findings = report.findings.clone();
    refresh_import_decision_receipts(report);
}

fn import_origin(
    ecosystem: DxForgeImportEcosystem,
    package_name: &str,
    package_id: &str,
    source_kind: &str,
    provenance_verified: bool,
) -> DxForgeImportOrigin {
    let ecosystem_segment = ecosystem.as_segment();
    DxForgeImportOrigin {
        ecosystem: ecosystem_segment.to_string(),
        registry: import_registry_name(ecosystem).to_string(),
        package_name: package_name.to_string(),
        package_id: package_id.to_string(),
        upstream_name: format!("{ecosystem_segment}:{package_name}"),
        upstream_version: "unresolved".to_string(),
        upstream_reference: Some(format!("{ecosystem_segment}:{package_name}")),
        tarball_integrity: None,
        source_kind: source_kind.to_string(),
        generator: format!("dx-forge/{ecosystem_segment}-external-source-snapshot"),
        provenance_verified,
        provenance_note: if provenance_verified {
            "External source snapshot with explicit Forge provenance verification; package-manager installs and lifecycle/setup/build scripts were not run."
                .to_string()
        } else {
            "External source snapshot; Forge did not run package-manager installs or lifecycle/setup/build scripts, and provenance verification is still pending."
                .to_string()
        },
    }
}

fn import_registry_name(ecosystem: DxForgeImportEcosystem) -> &'static str {
    match ecosystem {
        DxForgeImportEcosystem::Npm => "npmjs",
        DxForgeImportEcosystem::Pip => "pypi",
        DxForgeImportEcosystem::Cargo => "crates.io",
        DxForgeImportEcosystem::Go => "go-module-proxy",
        DxForgeImportEcosystem::Pub => "pub.dev",
        DxForgeImportEcosystem::Maven => "maven-central",
        DxForgeImportEcosystem::Nuget => "nuget.org",
        DxForgeImportEcosystem::Composer => "packagist",
        DxForgeImportEcosystem::Gem => "rubygems.org",
        DxForgeImportEcosystem::Swift => "swift-package-index",
        DxForgeImportEcosystem::Jsr => "jsr.io",
        DxForgeImportEcosystem::Hex => "hex.pm",
        DxForgeImportEcosystem::Cran => "cran.r-project.org",
    }
}

fn import_license_from_source(evidence: &DxForgeImportSourceEvidence) -> DxForgeImportLicense {
    let declared_license = evidence
        .declared_license
        .clone()
        .unwrap_or_else(|| "unreviewed".to_string());
    DxForgeImportLicense {
        license_source: if evidence.metadata_present {
            "inspected package metadata".to_string()
        } else {
            "Forge import metadata declaration".to_string()
        },
        license_file_hash: evidence.license_file_hash.clone(),
        reviewed: evidence.license_reviewed,
        reviewed_at: None,
        declared_license,
        note: if evidence.license_reviewed {
            "DX Forge evidence records reviewer acceptance for this license declaration."
                .to_string()
        } else {
            "Declaration recorded; no formal DX legal review is claimed.".to_string()
        },
    }
}

struct ImportScoreReportInput<'a> {
    no_node_modules: bool,
    source_dir_ready: bool,
    materialized: bool,
    accepted_materialization_receipt_present: bool,
    evidence: &'a DxForgeImportSourceEvidence,
    exports: &'a [DxForgeImportExport],
    files_considered: usize,
    files_rejected: usize,
    package_installs_run: bool,
    lifecycle_scripts_executed: bool,
    risk_flags: Vec<DxForgeImportRiskFlag>,
}

fn build_import_score_report(input: ImportScoreReportInput<'_>) -> DxForgeImportScoreReport {
    let evidence = input.evidence;
    score_forge_import(DxForgeImportScoreInput {
        no_node_modules: input.no_node_modules,
        source_dir_ready: input.source_dir_ready,
        materialized: input.materialized,
        accepted_materialization_receipt_present: input.accepted_materialization_receipt_present,
        metadata_present: evidence.metadata_present,
        provenance_verified: evidence.provenance_verified,
        artifact_integrity_present: evidence.integrity_evidence_present,
        license_declared: evidence.declared_license.is_some(),
        license_file_present: evidence.license_file_hash.is_some(),
        license_reviewed: evidence.license_reviewed,
        advisory_evidence_present: evidence.advisory_evidence_present,
        advisory_reviewed: evidence.advisory_reviewed,
        popularity_evidence_present: evidence.popularity_evidence_present,
        sbom_present: evidence.sbom_present,
        exports_present: !input.exports.is_empty(),
        dependency_count: evidence.dependency_count,
        files_considered: input.files_considered,
        files_rejected: input.files_rejected,
        package_installs_run: input.package_installs_run,
        lifecycle_scripts_executed: input.lifecycle_scripts_executed,
        risk_flags: input.risk_flags,
    })
}

fn apply_import_score(
    report: &mut DxForgeImportPlanReport,
    score_report: DxForgeImportScoreReport,
) {
    report.score = score_report.score;
    report.score_model_version = score_report.score_model_version;
    report.uncapped_score = score_report.uncapped_score;
    report.score_ceiling = score_report.score_ceiling;
    report.traffic = score_report.traffic;
    report.score_dimensions = score_report.dimensions;
    report.applied_caps = score_report.applied_caps;
}

fn build_import_disposition_report(
    input: DxForgePackageDispositionInput,
) -> DxForgePackageDispositionReport {
    classify_forge_package_disposition(input)
}

fn apply_disposition_score_cap(report: &mut DxForgeImportPlanReport) {
    report
        .applied_caps
        .retain(|cap| !cap.id.starts_with("disposition-"));
    let cap = match report.disposition.kind {
        DxForgePackageDispositionKind::Materialize => None,
        DxForgePackageDispositionKind::Slice => Some(DxForgeImportAppliedCap {
            id: "disposition-slice".to_string(),
            ceiling: 94,
            traffic: "yellow".to_string(),
            reason: "Package is an accepted source-slice candidate, not a fully materialized source-owned package.".to_string(),
        }),
        DxForgePackageDispositionKind::Bridge => Some(DxForgeImportAppliedCap {
            id: "disposition-bridge".to_string(),
            ceiling: 79,
            traffic: "yellow".to_string(),
            reason: "Package must stay behind an explicit bridge or boundary until source ownership is proven.".to_string(),
        }),
        DxForgePackageDispositionKind::Reject => Some(DxForgeImportAppliedCap {
            id: "disposition-reject".to_string(),
            ceiling: 0,
            traffic: "red".to_string(),
            reason: "Package was rejected by the Forge disposition gate.".to_string(),
        }),
    };
    if let Some(cap) = cap {
        report.applied_caps.push(cap);
    }

    report.score_ceiling = report
        .applied_caps
        .iter()
        .map(|cap| cap.ceiling)
        .min()
        .unwrap_or(100);
    report.score = report.uncapped_score.min(report.score_ceiling);
    report.traffic = traffic_for_score(report.score).to_string();
}

fn source_snapshot_preflight_cleared(report: &DxForgeImportPlanReport) -> bool {
    report.no_node_modules
        && report.source_dir_ready
        && report.uncapped_score >= 95
        && materialization_blocking_cap_ids(report).is_empty()
}

fn materialization_blocking_cap_ids(report: &DxForgeImportPlanReport) -> Vec<String> {
    let mut blockers = report
        .applied_caps
        .iter()
        .filter(|cap| cap_blocks_materialization(&cap.id))
        .map(|cap| cap.id.clone())
        .collect::<Vec<_>>();
    if report.uncapped_score < 95 {
        blockers.push("source-score-below-materialization-threshold".to_string());
    }
    blockers
}

fn cap_blocks_materialization(cap_id: &str) -> bool {
    !matches!(
        cap_id,
        "disposition-slice"
            | "provenance-verification-pending"
            | "license-review-pending"
            | "advisory-review-pending"
            | "source-sbom-missing"
    )
}

fn record_source_slice_policy(
    report: &mut DxForgeImportPlanReport,
    source_slice: &DxForgeImportSliceDecision,
) {
    report.source_slice_decision = import_decision_label(source_slice.decision).to_string();
    report.source_slice_kind = slice_kind_label(source_slice.slice_kind).to_string();
    report.source_slice_policy_decisions = source_slice.policy_decisions.clone();
}

fn import_decision_label(decision: DxForgeImportDecision) -> &'static str {
    match decision {
        DxForgeImportDecision::Accept => "accept",
        DxForgeImportDecision::ManualReview => "manual-review",
        DxForgeImportDecision::Block => "block",
    }
}

fn refresh_import_decision_receipts(report: &mut DxForgeImportPlanReport) {
    report.materialization_blocker_ids = materialization_blocking_cap_ids(report);
    if !report.no_node_modules {
        push_unique_string(
            &mut report.materialization_blocker_ids,
            "node-modules-present",
        );
    }
    for code in blocking_risk_flag_codes(&report.risk_flags) {
        push_unique_string(&mut report.materialization_blocker_ids, code);
    }

    report.bridge_reason_codes = bridge_reason_codes(report);
    report.bridge_reason_details = bridge_reason_details(report);
    report.refusal_reasons = build_import_refusal_reasons(report);
}

fn build_import_refusal_reasons(
    report: &DxForgeImportPlanReport,
) -> Vec<DxForgeImportRefusalReason> {
    let mut reasons = Vec::new();
    let mut seen = BTreeSet::new();

    if !report.no_node_modules {
        push_import_refusal_reason(
            &mut reasons,
            &mut seen,
            "plan",
            "no-package-install",
            "node-modules-present",
            "Target project contains node_modules, so Forge cannot claim a source-owned import boundary.",
            "Remove node_modules or run the import in a clean project before materialization.",
        );
    }
    if report.package_installs_run {
        push_import_refusal_reason(
            &mut reasons,
            &mut seen,
            "plan",
            "no-package-install",
            "package-installs-run",
            "Package-manager install commands ran during the Forge import boundary.",
            "Recreate the import receipt from a no-install source snapshot.",
        );
    }
    if report.lifecycle_scripts_executed {
        push_import_refusal_reason(
            &mut reasons,
            &mut seen,
            "plan",
            "no-lifecycle-execution",
            "lifecycle-scripts-executed",
            "Lifecycle or setup scripts executed before Forge accepted the package.",
            "Quarantine the package and rerun inspection without lifecycle execution.",
        );
    }

    for blocker_id in &report.materialization_blocker_ids {
        let detail = applied_cap_reason(report, blocker_id)
            .unwrap_or_else(|| format!("Materialization is blocked by `{blocker_id}`."));
        push_import_refusal_reason(
            &mut reasons,
            &mut seen,
            "materialize",
            "receipt-required",
            blocker_id.clone(),
            detail,
            "Keep the package as a reviewed slice or bridge until the blocker is cleared and receipts are refreshed.",
        );
    }

    if matches!(
        report.disposition.kind,
        DxForgePackageDispositionKind::Bridge | DxForgePackageDispositionKind::Reject
    ) {
        push_import_refusal_reason(
            &mut reasons,
            &mut seen,
            "analyze",
            "slice-scope",
            report.disposition.route.clone(),
            report.disposition.reason.clone(),
            report.disposition.remediation.clone(),
        );
    }

    for policy in &report.source_slice_policy_decisions {
        if policy.decision != DxForgeImportDecision::Accept {
            push_import_refusal_reason(
                &mut reasons,
                &mut seen,
                "slice",
                "slice-scope",
                policy.code.clone(),
                policy.detail.clone(),
                policy.remediation.clone(),
            );
        }
    }

    if report.score < report.fail_under {
        push_import_refusal_reason(
            &mut reasons,
            &mut seen,
            "materialize",
            "receipt-required",
            "score-below-fail-under",
            format!(
                "Forge import score {} is below the fail-under threshold {}.",
                report.score, report.fail_under
            ),
            "Improve source evidence, slice scope, or policy receipts before materialization.",
        );
    }

    reasons
}

fn push_import_refusal_reason(
    reasons: &mut Vec<DxForgeImportRefusalReason>,
    seen: &mut BTreeSet<String>,
    phase: impl Into<String>,
    gate: impl Into<String>,
    code: impl Into<String>,
    detail: impl Into<String>,
    remediation: impl Into<String>,
) {
    let phase = phase.into();
    let gate = gate.into();
    let code = code.into();
    let key = format!("{phase}:{gate}:{code}");
    if !seen.insert(key) {
        return;
    }
    reasons.push(DxForgeImportRefusalReason {
        phase,
        gate,
        code,
        detail: detail.into(),
        remediation: remediation.into(),
    });
}

fn applied_cap_reason(report: &DxForgeImportPlanReport, cap_id: &str) -> Option<String> {
    report
        .applied_caps
        .iter()
        .find(|cap| cap.id == cap_id)
        .map(|cap| cap.reason.clone())
}

fn push_unique_string(values: &mut Vec<String>, value: impl Into<String>) {
    let value = value.into();
    if !values.iter().any(|existing| existing == &value) {
        values.push(value);
    }
}

fn traffic_for_score(score: u8) -> &'static str {
    if score >= 95 {
        "green"
    } else if score >= 80 {
        "yellow"
    } else {
        "red"
    }
}

fn disposition_kind(report: &DxForgePackageDispositionReport) -> &'static str {
    match report.kind {
        DxForgePackageDispositionKind::Materialize => "materialize",
        DxForgePackageDispositionKind::Slice => "slice",
        DxForgePackageDispositionKind::Bridge => "bridge",
        DxForgePackageDispositionKind::Reject => "reject",
    }
}

fn capability_tier(report: &DxForgeImportEcosystemCapability) -> &'static str {
    match report.tier {
        DxForgeImportCapabilityTier::ReviewedJavascriptAdapter => "reviewed-javascript-adapter",
        DxForgeImportCapabilityTier::SourceSnapshot => "source-snapshot",
        DxForgeImportCapabilityTier::RuntimeBridge => "runtime-bridge",
    }
}

fn materialization_boundary(report: &DxForgeImportPlanReport) -> &'static str {
    if report.materialized && report.accepted_materialization_receipt_present {
        return "source-owned";
    }
    if report.materialized {
        return "materialized-without-accepted-receipt";
    }
    match report.disposition.kind {
        DxForgePackageDispositionKind::Materialize => "source-owned-pending-refresh",
        DxForgePackageDispositionKind::Slice => "reviewed-source-candidate",
        DxForgePackageDispositionKind::Bridge => "bridge-only",
        DxForgePackageDispositionKind::Reject => "rejected",
    }
}

fn disposition_slice_kind(report: &DxForgePackageDispositionReport) -> &'static str {
    slice_kind_label(report.slice_kind)
}

fn disposition_bridge_kind(report: &DxForgePackageDispositionReport) -> &'static str {
    match report.bridge_kind {
        Some(kind) => bridge_kind_label(kind),
        None => "none",
    }
}

fn bridge_kind_label(kind: DxForgePackageBridgeKind) -> &'static str {
    match kind {
        DxForgePackageBridgeKind::Tool => "tool",
        DxForgePackageBridgeKind::BinarySnapshot => "binary-snapshot",
        DxForgePackageBridgeKind::NativeRuntime => "native-runtime",
        DxForgePackageBridgeKind::HostedService => "hosted-service",
        DxForgePackageBridgeKind::PackageManagerBoundary => "package-manager-boundary",
    }
}

fn slice_kind_label(kind: DxForgeImportSliceKind) -> &'static str {
    match kind {
        DxForgeImportSliceKind::FullPackage => "full-package",
        DxForgeImportSliceKind::SourceSlice => "source-slice",
        DxForgeImportSliceKind::SymbolSlice => "symbol-slice",
        DxForgeImportSliceKind::Adapter => "adapter",
        DxForgeImportSliceKind::SourceCopy => "source-copy",
        DxForgeImportSliceKind::AssetSlice => "asset-slice",
        DxForgeImportSliceKind::WasmSlice => "wasm-slice",
        DxForgeImportSliceKind::ToolBridge => "tool-bridge",
        DxForgeImportSliceKind::BinaryBridge => "binary-bridge",
        DxForgeImportSliceKind::RuntimeBridge => "runtime-bridge",
        DxForgeImportSliceKind::MetadataOnly => "metadata-only",
        DxForgeImportSliceKind::Blocked => "blocked",
    }
}

fn risk_flag_codes(flags: &[DxForgeImportRiskFlag]) -> Vec<String> {
    flags
        .iter()
        .map(|flag| flag.as_reason_code().to_string())
        .collect()
}

fn blocking_risk_flag_codes(flags: &[DxForgeImportRiskFlag]) -> Vec<String> {
    flags
        .iter()
        .filter(|flag| flag.blocks_materialization())
        .map(|flag| flag.as_reason_code().to_string())
        .collect()
}

fn capability_supported_slice_kinds(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .ecosystem_capability
        .supported_slice_kinds
        .iter()
        .copied()
        .map(slice_kind_label)
        .map(str::to_string)
        .collect()
}

fn capability_bridge_kinds(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .ecosystem_capability
        .bridge_kinds
        .iter()
        .copied()
        .map(bridge_kind_label)
        .map(str::to_string)
        .collect()
}

fn source_slice_policy_codes(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .source_slice_policy_decisions
        .iter()
        .map(|decision| decision.code.clone())
        .collect()
}

fn source_slice_policy_details(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .source_slice_policy_decisions
        .iter()
        .map(|decision| decision.detail.clone())
        .collect()
}

fn source_slice_policy_remediations(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .source_slice_policy_decisions
        .iter()
        .map(|decision| decision.remediation.clone())
        .collect()
}

fn refusal_reason_phases(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .refusal_reasons
        .iter()
        .map(|reason| reason.phase.clone())
        .collect()
}

fn refusal_reason_gates(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .refusal_reasons
        .iter()
        .map(|reason| reason.gate.clone())
        .collect()
}

fn refusal_reason_codes(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .refusal_reasons
        .iter()
        .map(|reason| reason.code.clone())
        .collect()
}

fn refusal_reason_details(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .refusal_reasons
        .iter()
        .map(|reason| reason.detail.clone())
        .collect()
}

fn refusal_reason_remediations(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .refusal_reasons
        .iter()
        .map(|reason| reason.remediation.clone())
        .collect()
}

fn bridge_reason_codes(report: &DxForgeImportPlanReport) -> Vec<String> {
    if !matches!(
        report.disposition.kind,
        DxForgePackageDispositionKind::Bridge
    ) {
        return Vec::new();
    }
    let mut codes = blocking_risk_flag_codes(&report.risk_flags);
    if codes.is_empty() {
        codes.push(report.disposition.route.clone());
    }
    codes
}

fn bridge_reason_details(report: &DxForgeImportPlanReport) -> Vec<String> {
    if matches!(
        report.disposition.kind,
        DxForgePackageDispositionKind::Bridge
    ) {
        vec![report.disposition.reason.clone()]
    } else {
        Vec::new()
    }
}

fn applied_cap_ceilings(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .applied_caps
        .iter()
        .map(|cap| cap.ceiling.to_string())
        .collect()
}

fn applied_cap_traffic(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .applied_caps
        .iter()
        .map(|cap| cap.traffic.clone())
        .collect()
}

fn applied_cap_reasons(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .applied_caps
        .iter()
        .map(|cap| cap.reason.clone())
        .collect()
}

fn source_slice_files(report: &DxForgeImportPlanReport) -> Vec<String> {
    if matches!(
        report.disposition.kind,
        DxForgePackageDispositionKind::Slice
    ) {
        report
            .files_considered
            .iter()
            .map(|file| file.path.clone())
            .collect()
    } else {
        Vec::new()
    }
}

fn bridge_files(report: &DxForgeImportPlanReport) -> Vec<String> {
    if matches!(
        report.disposition.kind,
        DxForgePackageDispositionKind::Bridge
    ) {
        report
            .files_considered
            .iter()
            .map(|file| file.path.clone())
            .collect()
    } else {
        Vec::new()
    }
}

fn file_disposition_kinds(report: &DxForgeImportPlanReport) -> Vec<String> {
    let mut kinds = Vec::new();
    if !report.materialized_files.is_empty() {
        kinds.push("materialized".to_string());
    }
    if !source_slice_files(report).is_empty() {
        kinds.push("sliced".to_string());
    }
    if !bridge_files(report).is_empty() {
        kinds.push("bridged".to_string());
    }
    if !report.files_rejected.is_empty() {
        kinds.push("rejected".to_string());
    }
    if kinds.is_empty() {
        kinds.push(disposition_kind(&report.disposition).to_string());
    }
    kinds
}

fn file_disposition_paths(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .files_considered
        .iter()
        .map(|file| file.path.clone())
        .collect()
}

fn file_disposition_logical_paths(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .files_considered
        .iter()
        .map(|file| file.logical_path.clone())
        .collect()
}

fn file_disposition_hashes(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .files_considered
        .iter()
        .map(|file| file.hash.clone())
        .collect()
}

fn file_disposition_tracked_hashes(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .files_considered
        .iter()
        .map(|file| file.tracked_hash.clone())
        .collect()
}

fn file_disposition_bytes(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .files_considered
        .iter()
        .map(|file| file.bytes.to_string())
        .collect()
}

fn file_disposition_statuses(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .files_considered
        .iter()
        .map(|file| file.status.clone())
        .collect()
}

fn file_disposition_traffic(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .files_considered
        .iter()
        .map(|file| file.traffic.clone())
        .collect()
}

fn file_disposition_decisions(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .files_considered
        .iter()
        .map(|file| file.decision.clone())
        .collect()
}

fn file_disposition_messages(report: &DxForgeImportPlanReport) -> Vec<String> {
    report
        .files_considered
        .iter()
        .map(|file| file.message.clone())
        .collect()
}

fn forge_import_overwrite_policy() -> DxForgeImportOverwritePolicy {
    DxForgeImportOverwritePolicy {
        missing_file: "write".to_string(),
        matching_existing_file: "keep".to_string(),
        different_existing_file: "reject-yellow-no-overwrite".to_string(),
        security_sensitive_or_invalid_path: "reject-red-never-write".to_string(),
        partial_write: "forbidden".to_string(),
    }
}

fn source_snapshot_file_dispositions(
    project: &Path,
    source_files: &[DxForgeMaterializedSourceFile],
) -> anyhow::Result<Vec<DxForgeImportFileDisposition>> {
    source_files
        .iter()
        .map(|file| {
            materialized_source_file_disposition(
                project,
                &file.materialized_path,
                &file.logical_path,
                &file.content,
            )
        })
        .collect()
}

fn materialized_source_file_disposition(
    project: &Path,
    relative_path: &str,
    logical_path: &str,
    content: &str,
) -> anyhow::Result<DxForgeImportFileDisposition> {
    let target = project.join(relative_path);
    let desired_hash = blake3_hex(content.as_bytes());
    let before_hash = if target.exists() {
        let existing = fs::read(&target)
            .with_context(|| format!("read existing source snapshot `{}`", target.display()))?;
        Some(blake3_hex(&existing))
    } else {
        None
    };

    let (status, traffic, decision, message) = match &before_hash {
        None => (
            "written",
            "green",
            "accepted",
            format!("Forge can write `{relative_path}` as an external source snapshot file."),
        ),
        Some(existing_hash) if existing_hash == &desired_hash => (
            "kept",
            "green",
            "accepted",
            format!("Forge kept existing `{relative_path}` because its BLAKE3 hash matches."),
        ),
        Some(_) => (
            "rejected",
            "yellow",
            "rejected",
            format!(
                "Forge refused to overwrite existing `{relative_path}` because local content differs."
            ),
        ),
    };

    Ok(DxForgeImportFileDisposition {
        path: relative_path.to_string(),
        logical_path: logical_path.to_string(),
        hash_algorithm: "BLAKE3".to_string(),
        hash: desired_hash.clone(),
        bytes: content.len() as u64,
        before_hash,
        after_hash: Some(desired_hash.clone()),
        tracked_hash: desired_hash,
        status: status.to_string(),
        traffic: traffic.to_string(),
        decision: decision.to_string(),
        message,
    })
}

fn blake3_hex(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

fn write_materialized_source_files(
    project: &Path,
    source_files: &[DxForgeMaterializedSourceFile],
) -> anyhow::Result<DxForgeImportWriteTransaction> {
    let mut file_transaction = DxForgeFileTransaction::new(project);
    let mut source_transaction = DxForgeImportWriteTransaction::default();
    for file in source_files {
        match write_materialized_source_file(
            &mut file_transaction,
            project,
            &file.materialized_path,
            &file.content,
        ) {
            Ok(Some(created)) => source_transaction.created_source_files.push(created),
            Ok(None) => {}
            Err(error) => {
                let mut rollback_findings = file_transaction.rollback();
                rollback_findings.extend(source_transaction.rollback_created_source_files(project));
                if rollback_findings.is_empty() {
                    return Err(error);
                }
                anyhow::bail!(
                    "{}; rollback findings: {}",
                    error,
                    rollback_findings.join("; ")
                );
            }
        }
    }
    file_transaction.commit();
    Ok(source_transaction)
}

fn write_materialized_source_file(
    transaction: &mut DxForgeFileTransaction,
    project: &Path,
    relative_path: &str,
    content: &str,
) -> anyhow::Result<Option<DxForgeMaterializedSourceWrite>> {
    let target = project.join(relative_path);
    if target.exists() {
        let existing = fs::read_to_string(&target)
            .with_context(|| format!("read existing source snapshot `{}`", target.display()))?;
        if existing != content {
            anyhow::bail!(
                "refusing to overwrite existing `{relative_path}`; review or move the local file before materializing the Forge source snapshot"
            );
        }
        return Ok(None);
    }

    transaction.write_bytes_atomic(&target, content.as_bytes())?;
    Ok(Some(DxForgeMaterializedSourceWrite {
        relative_path: relative_path.to_string(),
        expected_hash: blake3_hex(content.as_bytes()),
    }))
}

impl DxForgeImportWriteTransaction {
    fn rollback_created_source_files(&self, project: &Path) -> Vec<String> {
        self.created_source_files
            .iter()
            .filter_map(|file| rollback_created_source_file(project, file))
            .collect()
    }
}

fn rollback_created_source_file(
    project: &Path,
    file: &DxForgeMaterializedSourceWrite,
) -> Option<String> {
    let path = project.join(&file.relative_path);
    if !path.exists() {
        return None;
    }
    let bytes = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) => {
            return Some(format!(
                "could not read `{}` for rollback: {error}",
                file.relative_path
            ));
        }
    };
    let current_hash = blake3_hex(&bytes);
    if current_hash != file.expected_hash {
        return Some(format!(
            "kept `{}` because its hash changed before rollback",
            file.relative_path
        ));
    }
    match fs::remove_file(&path) {
        Ok(()) => {
            remove_empty_parent_dirs(project, path.parent());
            None
        }
        Err(error) => Some(format!(
            "could not remove `{}` during rollback: {error}",
            file.relative_path
        )),
    }
}

fn remove_empty_parent_dirs(project: &Path, mut parent: Option<&Path>) {
    while let Some(path) = parent {
        if path == project {
            break;
        }
        if fs::remove_dir(path).is_err() {
            break;
        }
        parent = path.parent();
    }
}

impl DxForgeImportFileSnapshot {
    fn capture(path: PathBuf) -> anyhow::Result<Self> {
        let bytes = if path.exists() {
            Some(fs::read(&path).with_context(|| {
                format!(
                    "snapshot Forge import transaction file `{}`",
                    path.display()
                )
            })?)
        } else {
            None
        };
        Ok(Self { path, bytes })
    }

    fn restore_best_effort(&self, project: &Path) -> Option<String> {
        match &self.bytes {
            Some(bytes) => {
                if let Some(parent) = self.path.parent() {
                    if let Err(error) = fs::create_dir_all(parent) {
                        return Some(format!(
                            "could not recreate `{}` during rollback: {error}",
                            parent.display()
                        ));
                    }
                }
                if let Err(error) = fs::write(&self.path, bytes) {
                    return Some(format!(
                        "could not restore `{}` during rollback: {error}",
                        self.path.display()
                    ));
                }
                None
            }
            None => match fs::remove_file(&self.path) {
                Ok(()) => {
                    remove_empty_parent_dirs(project, self.path.parent());
                    None
                }
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
                Err(error) => Some(format!(
                    "could not remove `{}` during rollback: {error}",
                    self.path.display()
                )),
            },
        }
    }
}

impl DxForgeExternalStateSnapshot {
    fn capture(project: &Path, package_id: &str) -> anyhow::Result<Self> {
        Ok(Self {
            manifest: DxForgeImportFileSnapshot::capture(
                project.join(".dx/forge/source-.dx/build-cache/manifest.json"),
            )?,
            docs: DxForgeImportFileSnapshot::capture(project.join(import_docs_path(package_id)))?,
        })
    }

    fn rollback(&self, project: &Path, receipt_path: Option<&Path>) -> Vec<String> {
        let mut findings = Vec::new();
        if let Some(receipt_path) = receipt_path {
            match fs::remove_file(receipt_path) {
                Ok(()) => remove_empty_parent_dirs(project, receipt_path.parent()),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => findings.push(format!(
                    "could not remove `{}` during rollback: {error}",
                    receipt_path.display()
                )),
            }
        }
        if let Some(finding) = self.docs.restore_best_effort(project) {
            findings.push(finding);
        }
        if let Some(finding) = self.manifest.restore_best_effort(project) {
            findings.push(finding);
        }
        findings
    }
}

fn write_import_plan_artifacts(
    project: &Path,
    report: &mut DxForgeImportPlanReport,
) -> anyhow::Result<()> {
    let json_relative_path =
        import_plan_relative_path(&report.ecosystem, &report.package_name, "json")?;
    let sr_relative_path =
        import_plan_relative_path(&report.ecosystem, &report.package_name, "sr")?;
    let json_machine_cache_name = import_plan_json_machine_cache_name(report)?;

    let mut transaction = DxForgeFileTransaction::new(project);
    if let Err(error) = write_import_plan_artifacts_with_transaction(
        project,
        report,
        &json_relative_path,
        &sr_relative_path,
        &json_machine_cache_name,
        &mut transaction,
    ) {
        let rollback_findings = transaction.rollback();
        if rollback_findings.is_empty() {
            return Err(error);
        }
        anyhow::bail!(
            "{}; transaction rollback findings: {}",
            error,
            rollback_findings.join("; ")
        );
    }
    transaction.commit();
    Ok(())
}

fn write_import_plan_artifacts_with_transaction(
    project: &Path,
    report: &mut DxForgeImportPlanReport,
    json_relative_path: &str,
    sr_relative_path: &str,
    json_machine_cache_name: &str,
    transaction: &mut DxForgeFileTransaction,
) -> anyhow::Result<()> {
    snapshot_import_plan_artifact_paths(
        transaction,
        project,
        json_relative_path,
        sr_relative_path,
        json_machine_cache_name,
    )?;
    write_import_plan_artifacts_inner(
        project,
        report,
        json_relative_path,
        sr_relative_path,
        json_machine_cache_name,
        transaction,
    )
}

fn snapshot_import_plan_artifact_paths(
    transaction: &mut DxForgeFileTransaction,
    project: &Path,
    json_relative_path: &str,
    sr_relative_path: &str,
    json_machine_cache_name: &str,
) -> anyhow::Result<()> {
    let sr_source_path = project.join(sr_relative_path);
    let sr_machine_path = serializer_machine_path_for_sr(project, &sr_source_path);
    let json_machine_path = project
        .join(".dx/www")
        .join(format!("{json_machine_cache_name}.machine"));
    let json_machine_metadata_path = project
        .join(".dx/www")
        .join(format!("{json_machine_cache_name}.machine.meta.json"));

    transaction.snapshot_path(project.join(json_relative_path))?;
    transaction.snapshot_path(sr_source_path)?;
    transaction.snapshot_path(sr_machine_path)?;
    transaction.snapshot_path(json_machine_path)?;
    transaction.snapshot_path(json_machine_metadata_path)?;
    Ok(())
}

fn write_import_plan_artifacts_inner(
    project: &Path,
    report: &mut DxForgeImportPlanReport,
    json_relative_path: &str,
    sr_relative_path: &str,
    json_machine_cache_name: &str,
    transaction: &mut DxForgeFileTransaction,
) -> anyhow::Result<()> {
    report.import_plan_path = Some(project.join(json_relative_path));
    report.import_plan_sr_path = Some(project.join(sr_relative_path));
    report.import_plan_json_machine_path = Some(
        project
            .join(".dx/www")
            .join(format!("{json_machine_cache_name}.machine")),
    );

    let first_sr_artifact = write_import_plan_sr(project, report, sr_relative_path)?;
    report.import_plan_sr_path = Some(first_sr_artifact.source);
    report.import_plan_machine_path = Some(first_sr_artifact.machine);
    let final_sr_artifact = write_import_plan_sr(project, report, sr_relative_path)?;
    report.import_plan_sr_path = Some(final_sr_artifact.source);
    report.import_plan_machine_path = Some(final_sr_artifact.machine);
    write_import_plan_json(project, report, transaction)?;
    let report_value = serde_json::to_value(&mut *report)?;
    let json_machine_path = write_json_receipt_machine_alias(
        project,
        json_machine_cache_name,
        json_relative_path,
        &report_value,
    )?;
    report.import_plan_json_machine_path = Some(json_machine_path);
    Ok(())
}

fn import_plan_json_machine_cache_name(report: &DxForgeImportPlanReport) -> anyhow::Result<String> {
    let slug = forge_import_plan_slug(&report.package_name)?;
    Ok(format!("forge-import-plan-{}-{slug}", report.ecosystem))
}

fn write_import_plan_json(
    project: &Path,
    report: &DxForgeImportPlanReport,
    transaction: &mut DxForgeFileTransaction,
) -> anyhow::Result<PathBuf> {
    let path = report
        .import_plan_path
        .clone()
        .unwrap_or(project.join(import_plan_relative_path(
            &report.ecosystem,
            &report.package_name,
            "json",
        )?));
    transaction
        .write_bytes_atomic(&path, &serde_json::to_vec_pretty(report)?)
        .with_context(|| format!("write `{}`", path.display()))?;
    Ok(path)
}

fn write_import_plan_sr(
    project: &Path,
    report: &DxForgeImportPlanReport,
    relative_path: &str,
) -> anyhow::Result<SrArtifact> {
    write_sr_artifact(
        project,
        relative_path,
        &[
            ("schema", sr_string(report.schema)),
            ("version", sr_number(report.version)),
            ("generated_at", sr_string(&report.generated_at)),
            ("passed", sr_bool(report.passed)),
            ("fail_under", sr_number(report.fail_under)),
            ("ecosystem", sr_string(&report.ecosystem)),
            ("package_id", sr_string(&report.package_id)),
            ("package_name", sr_string(&report.package_name)),
            ("mode", sr_string(&report.mode)),
            (
                "score_model_version",
                sr_string(&report.score_model_version),
            ),
            ("score", sr_number(report.score)),
            ("uncapped_score", sr_number(report.uncapped_score)),
            ("score_ceiling", sr_number(report.score_ceiling)),
            ("traffic", sr_string(&report.traffic)),
            (
                "disposition_model_version",
                sr_string(&report.disposition.model_version),
            ),
            (
                "disposition_kind",
                sr_string(disposition_kind(&report.disposition)),
            ),
            (
                "disposition_is_materialize",
                sr_bool(matches!(
                    report.disposition.kind,
                    DxForgePackageDispositionKind::Materialize
                )),
            ),
            (
                "disposition_is_slice",
                sr_bool(matches!(
                    report.disposition.kind,
                    DxForgePackageDispositionKind::Slice
                )),
            ),
            (
                "disposition_is_bridge",
                sr_bool(matches!(
                    report.disposition.kind,
                    DxForgePackageDispositionKind::Bridge
                )),
            ),
            (
                "disposition_is_reject",
                sr_bool(matches!(
                    report.disposition.kind,
                    DxForgePackageDispositionKind::Reject
                )),
            ),
            ("disposition_route", sr_string(&report.disposition.route)),
            (
                "materialization_boundary",
                sr_string(materialization_boundary(report)),
            ),
            (
                "accepted_materialization_receipt_present",
                sr_bool(report.accepted_materialization_receipt_present),
            ),
            (
                "disposition_slice_kind",
                sr_string(disposition_slice_kind(&report.disposition)),
            ),
            (
                "disposition_bridge_kind",
                sr_string(disposition_bridge_kind(&report.disposition)),
            ),
            (
                "disposition_ownership_claim",
                sr_string(&report.disposition.ownership_claim),
            ),
            (
                "disposition_importable_source",
                sr_bool(report.disposition.importable_source),
            ),
            (
                "disposition_materializes_source",
                sr_bool(report.disposition.materializes_source),
            ),
            (
                "disposition_requires_receipt",
                sr_bool(report.disposition.requires_accepted_receipt),
            ),
            (
                "disposition_reason",
                sr_receipt_text(&report.disposition.reason),
            ),
            (
                "disposition_remediation",
                sr_receipt_text(&report.disposition.remediation),
            ),
            (
                "import_capability_model_version",
                sr_string(&report.ecosystem_capability.model_version),
            ),
            (
                "import_capability_tier",
                sr_string(capability_tier(&report.ecosystem_capability)),
            ),
            (
                "import_capability_score",
                sr_number(report.ecosystem_capability.capability_score),
            ),
            (
                "import_capability_plan_surface",
                sr_bool(report.ecosystem_capability.plan_surface),
            ),
            (
                "import_capability_non_executing_acquisition",
                sr_bool(report.ecosystem_capability.non_executing_acquisition),
            ),
            (
                "import_capability_local_source_inspection",
                sr_bool(report.ecosystem_capability.local_source_inspection),
            ),
            (
                "import_capability_reviewed_source_materialization",
                sr_bool(report.ecosystem_capability.reviewed_source_materialization),
            ),
            (
                "import_capability_direct_www_bare_import",
                sr_bool(report.ecosystem_capability.direct_www_bare_import),
            ),
            (
                "import_capability_package_score_can_reach_100",
                sr_bool(report.ecosystem_capability.package_score_can_reach_100),
            ),
            (
                "import_capability_live_registry_fetching",
                sr_bool(report.ecosystem_capability.live_registry_fetching),
            ),
            (
                "import_capability_package_manager_execution",
                sr_bool(report.ecosystem_capability.package_manager_execution),
            ),
            (
                "import_capability_universal_package_compatibility_claim",
                sr_bool(
                    report
                        .ecosystem_capability
                        .universal_package_compatibility_claim,
                ),
            ),
            (
                "import_capability_source_dir_required_for_materialization",
                sr_bool(
                    report
                        .ecosystem_capability
                        .source_dir_required_for_materialization,
                ),
            ),
            (
                "import_capability_supported_slice_kinds",
                sr_string_array(&capability_supported_slice_kinds(report)),
            ),
            (
                "import_capability_bridge_kinds",
                sr_string_array(&capability_bridge_kinds(report)),
            ),
            (
                "import_capability_honest_limitations",
                sr_receipt_text_array(&report.ecosystem_capability.honest_limitations),
            ),
            (
                "import_capability_clean_package_requirements",
                sr_receipt_text_array(&report.ecosystem_capability.clean_package_requirements),
            ),
            (
                "import_capability_score_100_requirements",
                sr_receipt_text_array(&report.ecosystem_capability.score_100_requirements),
            ),
            (
                "package_disposition",
                sr_string(disposition_kind(&report.disposition)),
            ),
            (
                "package_disposition_route",
                sr_string(&report.disposition.route),
            ),
            (
                "package_disposition_slice_kind",
                sr_string(disposition_slice_kind(&report.disposition)),
            ),
            (
                "package_disposition_bridge_kind",
                sr_string(disposition_bridge_kind(&report.disposition)),
            ),
            (
                "package_ownership_claim",
                sr_string(&report.disposition.ownership_claim),
            ),
            (
                "package_disposition_importable_source",
                sr_bool(report.disposition.importable_source),
            ),
            (
                "package_disposition_materializes_source",
                sr_bool(report.disposition.materializes_source),
            ),
            (
                "package_disposition_requires_receipt",
                sr_bool(report.disposition.requires_accepted_receipt),
            ),
            (
                "package_disposition_reason",
                sr_receipt_text(&report.disposition.reason),
            ),
            (
                "package_disposition_remediation",
                sr_receipt_text(&report.disposition.remediation),
            ),
            (
                "source_slice_decision",
                sr_string(&report.source_slice_decision),
            ),
            ("source_slice_kind", sr_string(&report.source_slice_kind)),
            (
                "source_slice_policy_codes",
                sr_string_array(&source_slice_policy_codes(report)),
            ),
            (
                "source_slice_policy_details",
                sr_receipt_text_array(&source_slice_policy_details(report)),
            ),
            (
                "source_slice_policy_remediations",
                sr_receipt_text_array(&source_slice_policy_remediations(report)),
            ),
            (
                "materialization_blocker_ids",
                sr_string_array(&report.materialization_blocker_ids),
            ),
            (
                "refusal_reason_phases",
                sr_string_array(&refusal_reason_phases(report)),
            ),
            (
                "refusal_reason_gates",
                sr_string_array(&refusal_reason_gates(report)),
            ),
            (
                "refusal_reason_codes",
                sr_string_array(&refusal_reason_codes(report)),
            ),
            (
                "refusal_reason_details",
                sr_receipt_text_array(&refusal_reason_details(report)),
            ),
            (
                "refusal_reason_remediations",
                sr_receipt_text_array(&refusal_reason_remediations(report)),
            ),
            (
                "bridge_reason_codes",
                sr_string_array(&report.bridge_reason_codes),
            ),
            (
                "bridge_reason_details",
                sr_receipt_text_array(&report.bridge_reason_details),
            ),
            ("restore_capability", sr_string(&report.restore_capability)),
            (
                "restore_content_source",
                sr_string(&report.restore_content_source),
            ),
            (
                "receipt_contains_file_content",
                sr_bool(report.receipt_contains_file_content),
            ),
            (
                "rollback_after_delete_supported",
                sr_bool(report.rollback_after_delete_supported),
            ),
            (
                "failed_write_atomicity",
                sr_receipt_text(&report.failed_write_atomicity),
            ),
            (
                "failed_write_recovery",
                sr_receipt_text(&report.failed_write_recovery),
            ),
            (
                "write_transaction_mode",
                sr_receipt_text(&report.write_transaction_mode),
            ),
            (
                "write_transaction_scope",
                sr_receipt_text_array(&report.write_transaction_scope),
            ),
            (
                "write_transaction_rollback",
                sr_receipt_text(&report.write_transaction_rollback),
            ),
            (
                "write_transaction_limitations",
                sr_receipt_text_array(&report.write_transaction_limitations),
            ),
            ("import_alias", sr_string(&report.import_alias)),
            ("source_kind", sr_string(&report.source_kind)),
            ("origin_registry", sr_string(&report.origin.registry)),
            ("origin_source_kind", sr_string(&report.origin.source_kind)),
            ("origin_generator", sr_string(&report.origin.generator)),
            (
                "origin_provenance_verified",
                sr_bool(report.origin.provenance_verified),
            ),
            (
                "license_declared",
                sr_string(&report.license.declared_license),
            ),
            ("license_source", sr_string(&report.license.license_source)),
            (
                "license_file_hash",
                sr_optional_string(report.license.license_file_hash.as_deref()),
            ),
            ("license_reviewed", sr_bool(report.license.reviewed)),
            (
                "origin_provenance_note",
                sr_receipt_text(&report.origin.provenance_note),
            ),
            ("forge_import_gate", sr_bool(report.forge_import_gate)),
            ("source_dir_ready", sr_bool(report.source_dir_ready)),
            (
                "source_provenance_verified",
                sr_bool(report.source_provenance_verified),
            ),
            (
                "source_integrity_evidence_declared",
                sr_bool(report.source_integrity_evidence_declared),
            ),
            (
                "source_license_reviewed",
                sr_bool(report.source_license_reviewed),
            ),
            (
                "source_advisory_evidence_declared",
                sr_bool(report.source_advisory_evidence_declared),
            ),
            (
                "source_advisory_reviewed",
                sr_bool(report.source_advisory_reviewed),
            ),
            (
                "source_popularity_evidence_declared",
                sr_bool(report.source_popularity_evidence_declared),
            ),
            ("source_sbom_present", sr_bool(report.source_sbom_present)),
            ("no_node_modules", sr_bool(report.no_node_modules)),
            ("package_installs_run", sr_bool(report.package_installs_run)),
            (
                "lifecycle_scripts_executed",
                sr_bool(report.lifecycle_scripts_executed),
            ),
            (
                "lifecycle_script_status",
                sr_string(&report.lifecycle_script_status),
            ),
            (
                "acquisition_metadata_inputs",
                sr_string_array(&report.acquisition_metadata_inputs),
            ),
            (
                "acquisition_artifact_inputs",
                sr_string_array(&report.acquisition_artifact_inputs),
            ),
            (
                "acquisition_metadata_references",
                sr_string_array(&report.acquisition_plan.metadata_references),
            ),
            (
                "acquisition_artifact_references",
                sr_string_array(&report.acquisition_plan.artifact_references),
            ),
            (
                "acquisition_expected_source_dir",
                sr_string(&report.acquisition_plan.expected_source_dir),
            ),
            (
                "acquisition_quarantine_dir",
                sr_string(&report.acquisition_plan.quarantine_dir),
            ),
            (
                "acquisition_evidence_receipt_path",
                sr_string(&report.acquisition_plan.evidence_receipt_path),
            ),
            (
                "acquisition_source_dir_required_for_materialization",
                sr_bool(
                    report
                        .acquisition_plan
                        .source_dir_required_for_materialization,
                ),
            ),
            (
                "forbidden_commands",
                sr_string_array(&report.forbidden_commands),
            ),
            (
                "live_fetching_enabled",
                sr_bool(report.live_fetching_enabled),
            ),
            (
                "package_manager_execution_allowed",
                sr_bool(report.package_manager_execution_allowed),
            ),
            (
                "accepted_import_receipt_required",
                sr_bool(report.accepted_import_receipt_required),
            ),
            (
                "unsupported_dx_add_form",
                sr_string(&report.unsupported_dx_add_form),
            ),
            (
                "materialization_status",
                sr_string(&report.materialization_status),
            ),
            (
                "materialization_ready",
                sr_bool(report.materialization_ready),
            ),
            ("materialized", sr_bool(report.materialized)),
            (
                "accepted_plan_status",
                sr_string(&report.accepted_plan_status),
            ),
            (
                "accepted_plan_path",
                sr_optional_path(&report.accepted_plan_path),
            ),
            (
                "accepted_plan_findings",
                sr_receipt_text_array(&report.accepted_plan_findings),
            ),
            (
                "requested_symbols",
                sr_string_array(&report.requested_symbols),
            ),
            ("selected_files", sr_string_array(&report.selected_files)),
            (
                "selected_files_count",
                sr_number(report.selected_files.len()),
            ),
            (
                "source_files_inspected_count",
                sr_number(report.source_files_inspected_count),
            ),
            (
                "source_dependency_count",
                sr_number(report.source_dependency_count),
            ),
            (
                "score_dimension_ids",
                sr_string_array(
                    &report
                        .score_dimensions
                        .iter()
                        .map(|dimension| dimension.id.clone())
                        .collect::<Vec<_>>(),
                ),
            ),
            (
                "score_dimension_scores",
                sr_string_array(
                    &report
                        .score_dimensions
                        .iter()
                        .map(|dimension| dimension.score.to_string())
                        .collect::<Vec<_>>(),
                ),
            ),
            (
                "applied_cap_ids",
                sr_string_array(
                    &report
                        .applied_caps
                        .iter()
                        .map(|cap| cap.id.clone())
                        .collect::<Vec<_>>(),
                ),
            ),
            (
                "applied_cap_ceilings",
                sr_string_array(&applied_cap_ceilings(report)),
            ),
            (
                "applied_cap_traffic",
                sr_string_array(&applied_cap_traffic(report)),
            ),
            (
                "applied_cap_reasons",
                sr_receipt_text_array(&applied_cap_reasons(report)),
            ),
            (
                "risk_flag_codes",
                sr_string_array(&risk_flag_codes(&report.risk_flags)),
            ),
            (
                "blocking_risk_flag_codes",
                sr_string_array(&blocking_risk_flag_codes(&report.risk_flags)),
            ),
            (
                "files_considered_count",
                sr_number(report.files_considered.len()),
            ),
            (
                "files_materialized_count",
                sr_number(report.materialized_files.len()),
            ),
            (
                "files_sliced_count",
                sr_number(source_slice_files(report).len()),
            ),
            ("files_bridged_count", sr_number(bridge_files(report).len())),
            ("files_written_count", sr_number(report.files_written.len())),
            ("files_kept_count", sr_number(report.files_kept.len())),
            (
                "files_rejected_count",
                sr_number(report.files_rejected.len()),
            ),
            (
                "file_disposition_paths",
                sr_string_array(&file_disposition_paths(report)),
            ),
            (
                "file_disposition_logical_paths",
                sr_string_array(&file_disposition_logical_paths(report)),
            ),
            (
                "file_disposition_hashes",
                sr_string_array(&file_disposition_hashes(report)),
            ),
            (
                "file_disposition_tracked_hashes",
                sr_string_array(&file_disposition_tracked_hashes(report)),
            ),
            (
                "file_disposition_bytes",
                sr_string_array(&file_disposition_bytes(report)),
            ),
            (
                "file_disposition_statuses",
                sr_string_array(&file_disposition_statuses(report)),
            ),
            (
                "file_disposition_traffic",
                sr_string_array(&file_disposition_traffic(report)),
            ),
            (
                "file_disposition_decisions",
                sr_string_array(&file_disposition_decisions(report)),
            ),
            (
                "file_disposition_messages",
                sr_receipt_text_array(&file_disposition_messages(report)),
            ),
            (
                "review_findings",
                sr_receipt_text_array(&report.review_findings),
            ),
            (
                "next_commands",
                sr_receipt_text_array(&report.next_commands),
            ),
            (
                "materialized_files",
                sr_string_array(&report.materialized_files),
            ),
            (
                "files_materialized",
                sr_string_array(&report.materialized_files),
            ),
            ("files_sliced", sr_string_array(&source_slice_files(report))),
            ("files_bridged", sr_string_array(&bridge_files(report))),
            (
                "file_disposition_kinds",
                sr_string_array(&file_disposition_kinds(report)),
            ),
            ("files_kept", sr_string_array(&report.files_kept)),
            ("files_written", sr_string_array(&report.files_written)),
            ("files_rejected", sr_string_array(&report.files_rejected)),
            (
                "export_names",
                sr_string_array(
                    &report
                        .exports
                        .iter()
                        .map(|export| export.name.clone())
                        .collect::<Vec<_>>(),
                ),
            ),
            (
                "reviewed_adapter_specifiers",
                sr_string_array(
                    &report
                        .reviewed_adapters
                        .iter()
                        .map(|adapter| adapter.specifier.clone())
                        .collect::<Vec<_>>(),
                ),
            ),
            (
                "reviewed_adapter_materialized_paths",
                sr_string_array(
                    &report
                        .reviewed_adapters
                        .iter()
                        .map(|adapter| adapter.materialized_path.clone())
                        .collect::<Vec<_>>(),
                ),
            ),
            (
                "reviewed_adapter_package_ids",
                sr_string_array(
                    &report
                        .reviewed_adapters
                        .iter()
                        .map(|adapter| adapter.package_id.clone())
                        .collect::<Vec<_>>(),
                ),
            ),
            (
                "overwrite_policy_different_existing_file",
                sr_string(&report.overwrite_policy.different_existing_file),
            ),
            (
                "overwrite_policy_missing_file",
                sr_string(&report.overwrite_policy.missing_file),
            ),
            (
                "overwrite_policy_matching_existing_file",
                sr_string(&report.overwrite_policy.matching_existing_file),
            ),
            (
                "overwrite_policy_security_sensitive_or_invalid_path",
                sr_string(&report.overwrite_policy.security_sensitive_or_invalid_path),
            ),
            (
                "overwrite_policy_partial_write",
                sr_string(&report.overwrite_policy.partial_write),
            ),
            (
                "materialized_package_id",
                sr_optional_string(report.materialized_package_id.as_deref()),
            ),
            ("manifest_path", sr_optional_path(&report.manifest_path)),
            ("receipt_path", sr_optional_path(&report.receipt_path)),
            ("docs_path", sr_optional_path(&report.docs_path)),
            (
                "import_plan_path",
                sr_optional_path(&report.import_plan_path),
            ),
            (
                "import_plan_sr_path",
                sr_optional_path(&report.import_plan_sr_path),
            ),
            (
                "import_plan_machine_path",
                sr_optional_path(&report.import_plan_machine_path),
            ),
            (
                "import_plan_json_machine_path",
                sr_optional_path(&report.import_plan_json_machine_path),
            ),
        ],
    )
}

fn import_plan_relative_path(
    ecosystem: &str,
    package_name: &str,
    extension: &str,
) -> anyhow::Result<String> {
    let slug = forge_import_plan_slug(package_name)?;
    match extension {
        "json" | "sr" => Ok(format!(
            ".dx/forge/import-plans/{}-{slug}.{extension}",
            ecosystem.trim()
        )),
        _ => anyhow::bail!("unsupported Forge import-plan extension `{extension}`"),
    }
}

fn forge_import_plan_slug(package_name: &str) -> anyhow::Result<String> {
    let mut slug = String::new();
    let mut last_was_separator = true;
    for character in package_name.trim().chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            last_was_separator = false;
        } else if matches!(character, '@' | '/' | '-' | '_' | '.') && !last_was_separator {
            slug.push('-');
            last_was_separator = true;
        } else if !matches!(character, '@' | '/' | '-' | '_' | '.') {
            anyhow::bail!("unsupported Forge import package name character `{character}`");
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    if slug.is_empty() {
        anyhow::bail!("Forge import package name does not produce an import-plan slug");
    }
    Ok(slug)
}

fn sr_optional_path(path: &Option<PathBuf>) -> String {
    path.as_ref()
        .map(|path| sr_string(path.display().to_string()))
        .unwrap_or_else(sr_null)
}

fn sr_optional_string(value: Option<&str>) -> String {
    value.map(sr_string).unwrap_or_else(sr_null)
}

fn sr_receipt_text(value: impl AsRef<str>) -> String {
    sr_string(value.as_ref().replace(',', ";"))
}

fn sr_receipt_text_array<T: AsRef<str>>(values: &[T]) -> String {
    let values = values
        .iter()
        .map(sr_receipt_text)
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{values}]")
}

fn collect_source_dir_files(
    project: &Path,
    ecosystem: DxForgeImportEcosystem,
    package_name: &str,
    source_dir: &Path,
    selected_files: &[String],
) -> anyhow::Result<Vec<DxForgeMaterializedSourceFile>> {
    const MAX_SOURCE_FILES: usize = 256;
    const MAX_FILE_BYTES: u64 = 2 * 1024 * 1024;
    const MAX_TOTAL_BYTES: u64 = 8 * 1024 * 1024;

    let package_slug = forge_import_plan_slug(package_name)?;
    let mut candidates = Vec::new();
    for entry in walkdir::WalkDir::new(source_dir)
        .into_iter()
        .filter_entry(|entry| should_visit_source_entry(entry.path()))
    {
        let entry = entry.with_context(|| format!("walk `{}`", source_dir.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }
        let relative = entry
            .path()
            .strip_prefix(source_dir)
            .with_context(|| format!("strip source root `{}`", entry.path().display()))?;
        let raw_relative = relative.to_string_lossy().replace('\\', "/");
        if !is_materializable_source_file(&raw_relative) {
            continue;
        }
        let safe_relative = validate_import_relative_path(&raw_relative)
            .with_context(|| format!("validate package source path `{raw_relative}`"))?;
        if !source_file_selected(&safe_relative.path, selected_files) {
            continue;
        }
        candidates.push((
            source_entry_rank(safe_relative.as_str()),
            safe_relative.path,
            entry.path().to_path_buf(),
        ));
    }
    candidates.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    if candidates.is_empty() {
        anyhow::bail!(
            "source directory `{}` does not contain any safe source files",
            source_dir.display()
        );
    }
    if !selected_files.is_empty() {
        let candidate_paths = candidates
            .iter()
            .map(|(_, path, _)| path.clone())
            .collect::<BTreeSet<_>>();
        let missing = selected_files
            .iter()
            .filter(|selected| !candidate_paths.contains(*selected))
            .cloned()
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            anyhow::bail!(
                "selected Forge source files were not found or not materializable: {}",
                missing.join(", ")
            );
        }
    }
    if candidates.len() > MAX_SOURCE_FILES {
        anyhow::bail!(
            "source directory `{}` has {} candidate files; Forge source-dir materialization limit is {} files",
            source_dir.display(),
            candidates.len(),
            MAX_SOURCE_FILES
        );
    }

    let mut total_bytes = 0u64;
    let mut files = Vec::new();
    for (_, source_relative, source_path) in candidates {
        let metadata = fs::metadata(&source_path)
            .with_context(|| format!("metadata `{}`", source_path.display()))?;
        if metadata.len() > MAX_FILE_BYTES {
            anyhow::bail!(
                "source file `{}` is {} bytes; Forge source-dir materialization limit is {} bytes per file",
                source_path.display(),
                metadata.len(),
                MAX_FILE_BYTES
            );
        }
        total_bytes = total_bytes.saturating_add(metadata.len());
        if total_bytes > MAX_TOTAL_BYTES {
            anyhow::bail!(
                "source directory `{}` exceeds Forge source-dir materialization byte budget of {} bytes",
                source_dir.display(),
                MAX_TOTAL_BYTES
            );
        }
        let content = fs::read_to_string(&source_path)
            .with_context(|| format!("read UTF-8 source `{}`", source_path.display()))?;
        let materialized_path = format!(
            "lib/forge/{}/{}/{}",
            ecosystem.as_segment(),
            package_slug,
            source_relative
        );
        let target_validation = validate_import_target_path(project, &materialized_path);
        if target_validation.decision != DxForgeImportDecision::Accept {
            anyhow::bail!("materialized Forge path `{materialized_path}` is not project-safe");
        }
        files.push(DxForgeMaterializedSourceFile {
            logical_path: format!(
                "{}/{}/{}",
                ecosystem.as_segment(),
                package_slug,
                source_relative
            ),
            materialized_path,
            content,
        });
    }

    for file in &files {
        let target = project.join(&file.materialized_path);
        if target.is_absolute() && !target.starts_with(project) {
            anyhow::bail!(
                "materialized file `{}` would escape project `{}`",
                target.display(),
                project.display()
            );
        }
    }
    Ok(files)
}

fn inspect_import_source_evidence(
    source_dir: &Path,
    selected_files: &[String],
) -> anyhow::Result<DxForgeImportSourceEvidence> {
    let mut evidence = DxForgeImportSourceEvidence {
        metadata_package_name: package_name_from_metadata(source_dir)?,
        declared_license: declared_license_from_metadata(source_dir)?,
        metadata_present: [
            "package.json",
            "pyproject.toml",
            "Cargo.toml",
            "go.mod",
            "setup.cfg",
            "pubspec.yaml",
            "pom.xml",
            "build.gradle",
            "build.gradle.kts",
            "composer.json",
            "Gemfile",
            "Package.swift",
            "mix.exs",
            "rebar.config",
            "DESCRIPTION",
            "NAMESPACE",
        ]
        .iter()
        .any(|file| source_dir.join(file).is_file())
            || source_dir_contains_extension(source_dir, "nuspec")?
            || source_dir_contains_extension(source_dir, "csproj")?
            || source_dir_contains_extension(source_dir, "gemspec")?,
        license_file_hash: license_file_hash(source_dir)?,
        provenance_verified: forge_evidence_marker_present(source_dir, "provenance_verified")?,
        integrity_evidence_present: forge_evidence_marker_present(source_dir, "integrity")?,
        license_reviewed: forge_evidence_marker_present(source_dir, "license_reviewed")?,
        advisory_evidence_present: forge_evidence_marker_present(source_dir, "advisory")?,
        advisory_reviewed: forge_evidence_marker_present(source_dir, "advisory_reviewed")?,
        popularity_evidence_present: forge_evidence_marker_present(source_dir, "popularity")?,
        sbom_present: forge_evidence_marker_present(source_dir, "sbom")?
            || source_dir.join("sbom.spdx.json").is_file()
            || source_dir.join("SBOM.spdx.json").is_file()
            || source_dir.join("sbom.cyclonedx.json").is_file()
            || source_dir.join("bom.json").is_file(),
        ..Default::default()
    };
    inspect_metadata_risks(source_dir, &mut evidence)?;

    for entry in walkdir::WalkDir::new(source_dir)
        .into_iter()
        .filter_entry(|entry| should_visit_source_entry(entry.path()))
    {
        let entry = entry.with_context(|| format!("walk evidence `{}`", source_dir.display()))?;
        if entry.file_type().is_symlink() {
            push_source_risk(&mut evidence, DxForgeImportRiskFlag::Symlink);
            continue;
        }
        if !entry.file_type().is_file() {
            continue;
        }
        let relative = entry
            .path()
            .strip_prefix(source_dir)
            .unwrap_or(entry.path())
            .to_string_lossy()
            .replace('\\', "/");
        if is_forge_evidence_marker(&relative) {
            continue;
        }
        if validate_import_relative_path(&relative).is_err() {
            push_source_risk(&mut evidence, DxForgeImportRiskFlag::UnsafePath);
            continue;
        }
        if !source_file_selected(&relative, selected_files) {
            continue;
        }
        if native_source_artifact(&relative) {
            push_source_risk(&mut evidence, DxForgeImportRiskFlag::NativeBinary);
        }
        if counts_as_source_evidence(&relative) {
            evidence.source_file_count = evidence.source_file_count.saturating_add(1);
        }
        inspect_source_text_risks(entry.path(), &relative, &mut evidence)?;
    }

    Ok(evidence)
}

fn push_source_risk(evidence: &mut DxForgeImportSourceEvidence, risk: DxForgeImportRiskFlag) {
    if !evidence.risk_flags.contains(&risk) {
        evidence.risk_flags.push(risk);
    }
}

fn record_package_identity_risk(
    requested_package: &str,
    evidence: &mut DxForgeImportSourceEvidence,
) {
    let Some(metadata_name) = evidence.metadata_package_name.as_deref() else {
        return;
    };
    if metadata_name.trim() != requested_package.trim() {
        push_source_risk(evidence, DxForgeImportRiskFlag::RuntimeMismatch);
    }
}

fn package_name_from_metadata(source_dir: &Path) -> anyhow::Result<Option<String>> {
    let package_json = source_dir.join("package.json");
    if package_json.is_file() {
        let source = fs::read_to_string(&package_json)
            .with_context(|| format!("read `{}`", package_json.display()))?;
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&source) {
            if let Some(name) = value.get("name").and_then(serde_json::Value::as_str) {
                let name = name.trim();
                if !name.is_empty() {
                    return Ok(Some(name.to_string()));
                }
            }
        }
    }

    for metadata_file in ["Cargo.toml", "pyproject.toml"] {
        let path = source_dir.join(metadata_file);
        if !path.is_file() {
            continue;
        }
        let source =
            fs::read_to_string(&path).with_context(|| format!("read `{}`", path.display()))?;
        if let Some(name) = first_quoted_metadata_value(&source, "name") {
            return Ok(Some(name));
        }
    }

    for metadata_file in ["pubspec.yaml", "build.gradle", "build.gradle.kts"] {
        let path = source_dir.join(metadata_file);
        if !path.is_file() {
            continue;
        }
        let source =
            fs::read_to_string(&path).with_context(|| format!("read `{}`", path.display()))?;
        if let Some(name) = first_colon_metadata_value(&source, "name") {
            return Ok(Some(name));
        }
    }

    if let Some(name) = maven_coordinate_from_pom(source_dir)? {
        return Ok(Some(name));
    }
    if let Some(name) = first_xml_tag_value_with_extension(source_dir, "nuspec", "id")? {
        return Ok(Some(name));
    }
    if let Some(name) = first_json_metadata_value(source_dir, "composer.json", "name")? {
        return Ok(Some(name));
    }
    if let Some(name) = first_gemspec_value(source_dir, "name")? {
        return Ok(Some(name));
    }
    if let Some(name) = first_swift_package_name(source_dir)? {
        return Ok(Some(name));
    }
    if let Some(name) = first_hex_package_name(source_dir)? {
        return Ok(Some(name));
    }
    if let Some(name) = first_cran_description_value(source_dir, "Package")? {
        return Ok(Some(name));
    }

    Ok(None)
}

fn inspect_metadata_risks(
    source_dir: &Path,
    evidence: &mut DxForgeImportSourceEvidence,
) -> anyhow::Result<()> {
    let package_json = source_dir.join("package.json");
    if package_json.is_file() {
        let source = fs::read_to_string(&package_json)
            .with_context(|| format!("read `{}`", package_json.display()))?;
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&source) {
            if package_json_has_install_hook(&value) {
                push_source_risk(evidence, DxForgeImportRiskFlag::InstallHook);
            }
            evidence.dependency_count = evidence
                .dependency_count
                .saturating_add(package_json_dependency_count(&value));
            if evidence.dependency_count > 48 {
                push_source_risk(evidence, DxForgeImportRiskFlag::HugeDependencyGraph);
            }
            if package_json_declares_side_effects(&value) {
                push_source_risk(evidence, DxForgeImportRiskFlag::SideEffectImport);
            }
        }
    }

    if source_dir.join("build.rs").is_file() {
        push_source_risk(evidence, DxForgeImportRiskFlag::InstallHook);
    }
    if source_dir.join("setup.py").is_file() {
        push_source_risk(evidence, DxForgeImportRiskFlag::DynamicExecution);
    }
    for metadata_file in [
        "build.gradle",
        "build.gradle.kts",
        "settings.gradle",
        "settings.gradle.kts",
        "Rakefile",
        "Package.swift",
        "configure",
        "cleanup",
    ] {
        if source_dir.join(metadata_file).is_file() && metadata_file != "Package.swift" {
            push_source_risk(evidence, DxForgeImportRiskFlag::InstallHook);
        }
    }
    if source_dir_contains_extension(source_dir, "targets")?
        || source_dir_contains_extension(source_dir, "props")?
        || source_dir_contains_extension(source_dir, "csx")?
        || source_dir_contains_extension(source_dir, "rake")?
        || source_dir_contains_extension(source_dir, "nif")?
    {
        push_source_risk(evidence, DxForgeImportRiskFlag::InstallHook);
    }
    Ok(())
}

fn package_json_has_install_hook(value: &serde_json::Value) -> bool {
    let Some(scripts) = value.get("scripts").and_then(serde_json::Value::as_object) else {
        return false;
    };
    scripts.keys().any(|script| {
        matches!(
            script.as_str(),
            "preinstall"
                | "install"
                | "postinstall"
                | "prepare"
                | "prepublish"
                | "prepublishOnly"
                | "postpublish"
        )
    })
}

fn package_json_dependency_count(value: &serde_json::Value) -> usize {
    [
        "dependencies",
        "devDependencies",
        "optionalDependencies",
        "peerDependencies",
    ]
    .iter()
    .filter_map(|key| value.get(*key).and_then(serde_json::Value::as_object))
    .map(serde_json::Map::len)
    .sum()
}

fn package_json_declares_side_effects(value: &serde_json::Value) -> bool {
    match value.get("sideEffects") {
        Some(serde_json::Value::Bool(true)) => true,
        Some(serde_json::Value::Array(entries)) => !entries.is_empty(),
        Some(serde_json::Value::String(entry)) => !entry.trim().is_empty(),
        Some(serde_json::Value::Object(entries)) => !entries.is_empty(),
        _ => false,
    }
}

fn inspect_source_text_risks(
    path: &Path,
    relative: &str,
    evidence: &mut DxForgeImportSourceEvidence,
) -> anyhow::Result<()> {
    if !is_reviewable_text_source(relative) {
        return Ok(());
    }
    let Ok(metadata) = fs::metadata(path) else {
        return Ok(());
    };
    if metadata.len() > 512 * 1024 {
        push_source_risk(evidence, DxForgeImportRiskFlag::ObfuscatedBlob);
        return Ok(());
    }
    let source = fs::read_to_string(path).with_context(|| format!("read `{}`", path.display()))?;
    if source_has_dynamic_execution(&source) {
        push_source_risk(evidence, DxForgeImportRiskFlag::DynamicExecution);
    }
    if source_has_dynamic_import(&source) {
        push_source_risk(evidence, DxForgeImportRiskFlag::DynamicImport);
    }
    if source_has_side_effect_import(&source) {
        push_source_risk(evidence, DxForgeImportRiskFlag::SideEffectImport);
    }
    if source_looks_obfuscated(&source) {
        push_source_risk(evidence, DxForgeImportRiskFlag::ObfuscatedBlob);
    }
    if source_contains_plaintext_secret(&source) {
        push_source_risk(evidence, DxForgeImportRiskFlag::PlaintextSecret);
    }
    Ok(())
}

fn is_reviewable_text_source(relative: &str) -> bool {
    relative.ends_with(".ts")
        || relative.ends_with(".tsx")
        || relative.ends_with(".js")
        || relative.ends_with(".mjs")
        || relative.ends_with(".cjs")
        || relative.ends_with(".rs")
        || relative.ends_with(".py")
        || relative.ends_with(".go")
        || relative.ends_with(".dart")
        || relative.ends_with(".java")
        || relative.ends_with(".kt")
        || relative.ends_with(".cs")
        || relative.ends_with(".php")
        || relative.ends_with(".rb")
        || relative.ends_with(".swift")
        || relative.ends_with(".ex")
        || relative.ends_with(".exs")
        || relative.ends_with(".erl")
        || relative.ends_with(".hrl")
        || relative.ends_with(".R")
        || relative.ends_with(".r")
}

fn is_materializable_source_file(relative: &str) -> bool {
    (is_reviewable_text_source(relative) || relative.ends_with(".css"))
        && !is_package_metadata_file(relative)
}

fn native_source_artifact(relative: &str) -> bool {
    let lower = relative.to_ascii_lowercase();
    [
        ".node",
        ".dll",
        ".dylib",
        ".so",
        ".exe",
        ".pyd",
        ".a",
        ".lib",
        ".jar",
        ".class",
        ".aar",
        ".nupkg",
        ".gem",
        ".wasm",
        ".nif",
        ".xcframework",
        ".framework",
    ]
    .iter()
    .any(|extension| lower.ends_with(extension))
}

fn source_has_dynamic_execution(source: &str) -> bool {
    source.contains("eval(")
        || source.contains("new Function(")
        || source.contains("Function(")
        || source.contains("exec(")
}

fn source_has_dynamic_import(source: &str) -> bool {
    source.contains("import(") || source.lines().any(line_has_dynamic_require)
}

fn line_has_dynamic_require(line: &str) -> bool {
    let Some((_, value)) = line.split_once("require(") else {
        return false;
    };
    let value = value.trim_start();
    !(value.starts_with('"') || value.starts_with('\''))
}

fn source_has_side_effect_import(source: &str) -> bool {
    source.lines().any(|line| {
        let line = line.trim_start();
        (line.starts_with("import \"") || line.starts_with("import '")) && !line.contains(" from ")
    })
}

fn source_looks_obfuscated(source: &str) -> bool {
    source
        .lines()
        .any(|line| line.len() > 2_000 && line.matches(';').count() > 25)
}

fn source_contains_plaintext_secret(source: &str) -> bool {
    let lower = source.to_ascii_lowercase();
    [
        "api_key=",
        "apikey=",
        "secret=",
        "password=",
        "access_token=",
        "refresh_token=",
        "xkeysib-",
    ]
    .iter()
    .any(|marker| lower.contains(marker))
}

fn declared_license_from_metadata(source_dir: &Path) -> anyhow::Result<Option<String>> {
    let package_json = source_dir.join("package.json");
    if package_json.is_file() {
        let source = fs::read_to_string(&package_json)
            .with_context(|| format!("read `{}`", package_json.display()))?;
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&source) {
            if let Some(license) = value.get("license").and_then(serde_json::Value::as_str) {
                let license = license.trim();
                if !license.is_empty() {
                    return Ok(Some(license.to_string()));
                }
            }
        }
    }

    for metadata_file in ["Cargo.toml", "pyproject.toml", "setup.cfg"] {
        let path = source_dir.join(metadata_file);
        if !path.is_file() {
            continue;
        }
        let source =
            fs::read_to_string(&path).with_context(|| format!("read `{}`", path.display()))?;
        if let Some(license) = first_quoted_metadata_value(&source, "license") {
            return Ok(Some(license));
        }
    }

    for metadata_file in ["pubspec.yaml", "build.gradle", "build.gradle.kts"] {
        let path = source_dir.join(metadata_file);
        if !path.is_file() {
            continue;
        }
        let source =
            fs::read_to_string(&path).with_context(|| format!("read `{}`", path.display()))?;
        if let Some(license) = first_colon_metadata_value(&source, "license") {
            return Ok(Some(license));
        }
    }
    if let Some(license) = first_xml_tag_value(source_dir, "pom.xml", "license")? {
        return Ok(Some(license));
    }
    if let Some(license) = first_xml_tag_value_with_extension(source_dir, "nuspec", "license")? {
        return Ok(Some(license));
    }
    if let Some(license) = first_json_metadata_value(source_dir, "composer.json", "license")? {
        return Ok(Some(license));
    }
    if let Some(license) = first_gemspec_value(source_dir, "license")? {
        return Ok(Some(license));
    }
    if let Some(license) = first_cran_description_value(source_dir, "License")? {
        return Ok(Some(license));
    }
    if let Some(license) = forge_evidence_marker_value(source_dir, "license")? {
        return Ok(Some(license));
    }

    Ok(None)
}

fn first_quoted_metadata_value(source: &str, key: &str) -> Option<String> {
    source.lines().find_map(|line| {
        let line = line.trim();
        if !line.starts_with(key) {
            return None;
        }
        let (_, value) = line.split_once('=')?;
        let value = value.trim().trim_matches('"').trim_matches('\'').trim();
        (!value.is_empty()).then(|| value.to_string())
    })
}

fn first_colon_metadata_value(source: &str, key: &str) -> Option<String> {
    source.lines().find_map(|line| {
        let line = line.trim();
        if !line.starts_with(key) {
            return None;
        }
        let (_, value) = line.split_once(':')?;
        let value = value.trim().trim_matches('"').trim_matches('\'').trim();
        (!value.is_empty()).then(|| value.to_string())
    })
}

fn first_json_metadata_value(
    source_dir: &Path,
    file_name: &str,
    key: &str,
) -> anyhow::Result<Option<String>> {
    let path = source_dir.join(file_name);
    if !path.is_file() {
        return Ok(None);
    }
    let source = fs::read_to_string(&path).with_context(|| format!("read `{}`", path.display()))?;
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&source) else {
        return Ok(None);
    };
    Ok(value
        .get(key)
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string))
}

fn first_xml_tag_value(
    source_dir: &Path,
    file_name: &str,
    tag: &str,
) -> anyhow::Result<Option<String>> {
    let path = source_dir.join(file_name);
    if !path.is_file() {
        return Ok(None);
    }
    let source = fs::read_to_string(&path).with_context(|| format!("read `{}`", path.display()))?;
    Ok(first_xml_tag_value_from_source(&source, tag))
}

fn maven_coordinate_from_pom(source_dir: &Path) -> anyhow::Result<Option<String>> {
    let path = source_dir.join("pom.xml");
    if !path.is_file() {
        return Ok(None);
    }
    let source = fs::read_to_string(&path).with_context(|| format!("read `{}`", path.display()))?;
    let Some(artifact_id) = first_xml_tag_value_from_source(&source, "artifactId") else {
        return Ok(None);
    };
    if let Some(group_id) = first_xml_tag_value_from_source(&source, "groupId") {
        return Ok(Some(format!("{group_id}.{artifact_id}")));
    }
    Ok(Some(artifact_id))
}

fn first_xml_tag_value_with_extension(
    source_dir: &Path,
    extension: &str,
    tag: &str,
) -> anyhow::Result<Option<String>> {
    for entry in walkdir::WalkDir::new(source_dir)
        .max_depth(2)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|value| value.to_str()) != Some(extension) {
            continue;
        }
        let source = fs::read_to_string(entry.path())
            .with_context(|| format!("read `{}`", entry.path().display()))?;
        if let Some(value) = first_xml_tag_value_from_source(&source, tag) {
            return Ok(Some(value));
        }
    }
    Ok(None)
}

fn first_xml_tag_value_from_source(source: &str, tag: &str) -> Option<String> {
    let start_tag = format!("<{tag}>");
    let end_tag = format!("</{tag}>");
    let (_, after_start) = source.split_once(&start_tag)?;
    let (value, _) = after_start.split_once(&end_tag)?;
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_string())
}

fn first_gemspec_value(source_dir: &Path, key: &str) -> anyhow::Result<Option<String>> {
    for entry in walkdir::WalkDir::new(source_dir)
        .max_depth(2)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|value| value.to_str()) != Some("gemspec") {
            continue;
        }
        let source = fs::read_to_string(entry.path())
            .with_context(|| format!("read `{}`", entry.path().display()))?;
        for line in source.lines().map(str::trim) {
            if !(line.contains(&format!(".{key}")) || line.starts_with(key)) {
                continue;
            }
            if let Some((_, value)) = line.split_once('=') {
                let value = value.trim().trim_matches('"').trim_matches('\'').trim();
                if !value.is_empty() {
                    return Ok(Some(value.to_string()));
                }
            }
        }
    }
    Ok(None)
}

fn first_swift_package_name(source_dir: &Path) -> anyhow::Result<Option<String>> {
    let path = source_dir.join("Package.swift");
    if !path.is_file() {
        return Ok(None);
    }
    let source = fs::read_to_string(&path).with_context(|| format!("read `{}`", path.display()))?;
    let Some((_, after_name)) = source.split_once("Package(name:") else {
        return Ok(None);
    };
    let value = after_name
        .trim_start()
        .trim_start_matches('"')
        .trim_start_matches('\'');
    let name = value
        .chars()
        .take_while(|character| *character != '"' && *character != '\'')
        .collect::<String>();
    Ok((!name.trim().is_empty()).then(|| name.trim().to_string()))
}

fn first_hex_package_name(source_dir: &Path) -> anyhow::Result<Option<String>> {
    let path = source_dir.join("mix.exs");
    if !path.is_file() {
        return Ok(None);
    }
    let source = fs::read_to_string(&path).with_context(|| format!("read `{}`", path.display()))?;
    for marker in ["app: :", "app: \"", "app: '"] {
        let Some((_, after_marker)) = source.split_once(marker) else {
            continue;
        };
        let name = after_marker
            .chars()
            .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
            .collect::<String>();
        if !name.trim().is_empty() {
            return Ok(Some(name));
        }
    }
    Ok(None)
}

fn first_cran_description_value(source_dir: &Path, key: &str) -> anyhow::Result<Option<String>> {
    let path = source_dir.join("DESCRIPTION");
    if !path.is_file() {
        return Ok(None);
    }
    let source = fs::read_to_string(&path).with_context(|| format!("read `{}`", path.display()))?;
    Ok(first_colon_metadata_value(&source, key))
}

fn source_dir_contains_extension(source_dir: &Path, extension: &str) -> anyhow::Result<bool> {
    for entry in walkdir::WalkDir::new(source_dir)
        .max_depth(3)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|value| value.to_str()) == Some(extension) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn license_file_hash(source_dir: &Path) -> anyhow::Result<Option<String>> {
    for name in [
        "LICENSE",
        "LICENSE.md",
        "LICENSE.txt",
        "COPYING",
        "COPYING.md",
        "COPYING.txt",
    ] {
        let path = source_dir.join(name);
        if path.is_file() {
            let bytes = fs::read(&path).with_context(|| format!("read `{}`", path.display()))?;
            return Ok(Some(blake3_hex(&bytes)));
        }
    }
    Ok(None)
}

fn forge_evidence_marker_present(source_dir: &Path, marker: &str) -> anyhow::Result<bool> {
    for name in FORGE_EVIDENCE_MARKERS {
        let path = source_dir.join(name);
        if path.is_file() {
            let source =
                fs::read_to_string(&path).with_context(|| format!("read `{}`", path.display()))?;
            if source
                .lines()
                .any(|line| evidence_marker_is_true(line, marker))
            {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn forge_evidence_marker_value(source_dir: &Path, marker: &str) -> anyhow::Result<Option<String>> {
    for name in FORGE_EVIDENCE_MARKERS {
        let path = source_dir.join(name);
        if !path.is_file() {
            continue;
        }
        let source =
            fs::read_to_string(&path).with_context(|| format!("read `{}`", path.display()))?;
        for line in source.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with(marker) {
                continue;
            }
            let Some((_, value)) = trimmed.split_once('=') else {
                continue;
            };
            let value = value.trim().trim_matches('"').trim_matches('\'').trim();
            if !value.is_empty() {
                return Ok(Some(value.to_string()));
            }
        }
    }
    Ok(None)
}

fn evidence_marker_is_true(line: &str, marker: &str) -> bool {
    let Some((key, value)) = line.split_once('=') else {
        return false;
    };
    key.trim() == marker && value.trim().trim_matches('"').eq_ignore_ascii_case("true")
}

const FORGE_EVIDENCE_MARKERS: &[&str] = &[
    ".dx-forge-evidence.sr",
    "dx-forge-evidence.sr",
    ".dx-forge-evidence.machine",
];

fn is_forge_evidence_marker(relative: &str) -> bool {
    FORGE_EVIDENCE_MARKERS.contains(&relative)
}

fn counts_as_source_evidence(relative: &str) -> bool {
    is_package_metadata_file(relative)
        || matches!(
            relative,
            "LICENSE" | "LICENSE.md" | "LICENSE.txt" | "COPYING" | "COPYING.md" | "COPYING.txt"
        )
        || relative.ends_with(".ts")
        || relative.ends_with(".tsx")
        || relative.ends_with(".js")
        || relative.ends_with(".mjs")
        || relative.ends_with(".cjs")
        || relative.ends_with(".rs")
        || relative.ends_with(".py")
        || relative.ends_with(".go")
        || relative.ends_with(".dart")
        || relative.ends_with(".java")
        || relative.ends_with(".kt")
        || relative.ends_with(".cs")
        || relative.ends_with(".php")
        || relative.ends_with(".rb")
        || relative.ends_with(".swift")
        || relative.ends_with(".ex")
        || relative.ends_with(".exs")
        || relative.ends_with(".erl")
        || relative.ends_with(".hrl")
        || relative.ends_with(".R")
        || relative.ends_with(".r")
}

fn is_package_metadata_file(relative: &str) -> bool {
    let normalized = relative.replace('\\', "/");
    let file_name = normalized.rsplit('/').next().unwrap_or(normalized.as_str());
    matches!(
        normalized.as_str(),
        "package.json"
            | "Cargo.toml"
            | "pyproject.toml"
            | "go.mod"
            | "setup.cfg"
            | "pubspec.yaml"
            | "pom.xml"
            | "build.gradle"
            | "build.gradle.kts"
            | "settings.gradle"
            | "settings.gradle.kts"
            | "composer.json"
            | "composer.lock"
            | "Gemfile"
            | "Gemfile.lock"
            | "Package.swift"
            | "Package.resolved"
            | "mix.exs"
            | "mix.lock"
            | "rebar.config"
            | "DESCRIPTION"
            | "NAMESPACE"
    ) || file_name.ends_with(".nuspec")
        || file_name.ends_with(".csproj")
        || file_name.ends_with(".props")
        || file_name.ends_with(".targets")
        || file_name.ends_with(".gemspec")
}

fn should_visit_source_entry(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return true;
    };
    !matches!(
        name,
        ".git" | ".hg" | ".svn" | "node_modules" | "target" | "__pycache__" | ".venv"
    )
}

fn source_entry_rank(relative: &str) -> u8 {
    match relative {
        "index.ts" | "index.tsx" | "index.mjs" | "index.js" | "mod.ts" | "lib.rs" | "main.go"
        | "__init__.py" | "lib/main.dart" | "lib/rails.rb" | "R/dplyr.R" => 0,
        path if path.ends_with("/index.ts")
            || path.ends_with("/index.tsx")
            || path.ends_with("/index.mjs")
            || path.ends_with("/index.js")
            || path.ends_with("/main.dart")
            || path.ends_with("/lib.ex")
            || path.ends_with("/Application.php")
            || path.ends_with("/Chunked.swift") =>
        {
            1
        }
        "package.json" | "Cargo.toml" | "pyproject.toml" | "go.mod" => 3,
        _ => 2,
    }
}

fn discover_source_exports(
    source_dir: &Path,
    selected_files: &[String],
) -> anyhow::Result<Vec<DxForgeImportExport>> {
    let mut exports = Vec::new();
    for entry in walkdir::WalkDir::new(source_dir)
        .into_iter()
        .filter_entry(|entry| should_visit_source_entry(entry.path()))
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let Some(extension) = path.extension().and_then(|extension| extension.to_str()) else {
            continue;
        };
        if !matches!(
            extension,
            "js" | "mjs"
                | "cjs"
                | "ts"
                | "tsx"
                | "rs"
                | "py"
                | "go"
                | "dart"
                | "java"
                | "kt"
                | "cs"
                | "php"
                | "rb"
                | "swift"
                | "ex"
                | "exs"
                | "erl"
                | "hrl"
                | "R"
                | "r"
        ) {
            continue;
        }
        let relative = path
            .strip_prefix(source_dir)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        if !source_file_selected(&relative, selected_files) {
            continue;
        }
        let source = fs::read_to_string(path)
            .with_context(|| format!("read export source `{}`", path.display()))?;
        for export_name in simple_source_public_names(&source, extension) {
            if !exports
                .iter()
                .any(|export: &DxForgeImportExport| export.name == export_name)
            {
                exports.push(DxForgeImportExport {
                    name: export_name.clone(),
                    kind: if export_name == "default" {
                        "default".to_string()
                    } else {
                        "named".to_string()
                    },
                    source_path: relative.clone(),
                    upstream_export: export_name,
                    materialized: false,
                });
            }
        }
    }
    Ok(exports)
}

fn simple_export_names(source: &str) -> Vec<String> {
    let mut names = Vec::new();
    if source.contains("export default") {
        names.push("default".to_string());
    }
    for marker in [
        "export function ",
        "export const ",
        "export let ",
        "export var ",
        "export class ",
        "export type ",
        "export interface ",
    ] {
        let mut cursor = 0usize;
        while let Some(offset) = source[cursor..].find(marker) {
            let start = cursor + offset + marker.len();
            let name = source[start..]
                .chars()
                .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
                .collect::<String>();
            let name_len = name.len();
            if !name.is_empty() && !names.contains(&name) {
                names.push(name);
            }
            cursor = start.saturating_add(name_len);
        }
    }
    names
}

fn simple_source_public_names(source: &str, extension: &str) -> Vec<String> {
    if matches!(extension, "js" | "mjs" | "cjs" | "ts" | "tsx") {
        return simple_export_names(source);
    }
    if extension == "py" {
        return simple_python_public_names(source);
    }
    if extension == "go" {
        return simple_go_public_names(source);
    }
    if matches!(extension, "ex" | "exs" | "erl" | "hrl") {
        return simple_elixir_erlang_public_names(source);
    }
    if matches!(extension, "R" | "r") {
        return simple_r_public_names(source);
    }
    let markers = match extension {
        "dart" => &["class ", "enum ", "mixin ", "typedef ", "extension "][..],
        "rs" => &[
            "pub struct ",
            "pub enum ",
            "pub trait ",
            "pub fn ",
            "pub type ",
            "pub const ",
            "pub static ",
        ][..],
        "java" | "kt" | "cs" | "php" => &[
            "public class ",
            "public static class ",
            "public struct ",
            "public interface ",
            "public enum ",
            "final class ",
            "class ",
            "interface ",
            "enum ",
            "struct ",
        ][..],
        "rb" => &["module ", "class "][..],
        "swift" => &[
            "public struct ",
            "public class ",
            "public enum ",
            "public protocol ",
            "public func ",
            "struct ",
            "class ",
            "enum ",
            "protocol ",
            "func ",
        ][..],
        _ => &[][..],
    };
    let mut names = Vec::new();
    for marker in markers {
        let mut cursor = 0usize;
        while let Some(offset) = source[cursor..].find(marker) {
            let start = cursor + offset + marker.len();
            let name = source[start..]
                .chars()
                .skip_while(|character| !is_symbol_start(*character))
                .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
                .collect::<String>();
            if public_source_name_is_exported(extension, &name) && !names.contains(&name) {
                names.push(name.clone());
            }
            cursor = start.saturating_add(name.len().max(1));
        }
    }
    names
}

fn simple_python_public_names(source: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in source.lines() {
        if line.chars().next().is_some_and(char::is_whitespace) {
            continue;
        }
        let trimmed = line.trim_start();
        for marker in ["class ", "def "] {
            if let Some(name) = public_name_after_marker(trimmed, marker) {
                push_unique_public_name(&mut names, "py", name);
            }
        }
    }
    names
}

fn simple_go_public_names(source: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in source.lines().map(str::trim_start) {
        if let Some(name) = public_name_after_marker(line, "type ") {
            push_unique_public_name(&mut names, "go", name);
        }
        let Some(rest) = line.strip_prefix("func ") else {
            continue;
        };
        if rest.trim_start().starts_with('(') {
            continue;
        }
        let name = rest
            .trim_start()
            .chars()
            .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
            .collect::<String>();
        push_unique_public_name(&mut names, "go", name);
    }
    names
}

fn simple_elixir_erlang_public_names(source: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in source.lines().map(str::trim_start) {
        if let Some(name) = line
            .strip_prefix("defmodule ")
            .and_then(|rest| rest.split_whitespace().next())
        {
            push_unique_public_name(&mut names, "ex", name.trim_end_matches("do").to_string());
        }
        if let Some(name) = public_name_after_marker(line, "def ") {
            push_unique_public_name(&mut names, "ex", name);
        }
        if let Some(module) = line.strip_prefix("-module(") {
            let name = module
                .chars()
                .take_while(|character| *character != ')')
                .collect::<String>();
            push_unique_public_name(&mut names, "erl", name);
        }
    }
    names
}

fn simple_r_public_names(source: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in source.lines().map(str::trim_start) {
        let Some((name, value)) = line.split_once("<-").or_else(|| line.split_once('=')) else {
            continue;
        };
        if !value.trim_start().starts_with("function") {
            continue;
        }
        push_unique_public_name(&mut names, "r", name.trim().to_string());
    }
    names
}

fn public_name_after_marker(line: &str, marker: &str) -> Option<String> {
    let rest = line.strip_prefix(marker)?.trim_start();
    let name = rest
        .chars()
        .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
        .collect::<String>();
    (!name.is_empty()).then_some(name)
}

fn push_unique_public_name(names: &mut Vec<String>, extension: &str, name: String) {
    if public_source_name_is_exported(extension, &name) && !names.contains(&name) {
        names.push(name);
    }
}

fn public_source_name_is_exported(extension: &str, name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    match extension {
        "py" => !name.starts_with('_'),
        "go" => name
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_uppercase()),
        _ => true,
    }
}

fn is_symbol_start(character: char) -> bool {
    character.is_ascii_alphabetic() || character == '_'
}

fn import_docs_path(package_id: &str) -> String {
    format!(".dx/forge/docs/{}.md", package_id.replace('/', "-"))
}
