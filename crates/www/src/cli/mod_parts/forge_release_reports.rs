fn forge_local_registry_publish_plan(
    local: &Path,
    package: Option<&str>,
) -> DxForgeRegistryOperationReport {
    let mut objects = vec![
        local.display().to_string(),
        local.join("index.json").display().to_string(),
    ];
    if let Some(package) = package {
        objects.push(
            local
                .join("packages/js")
                .join(canonical_package_id(package))
                .display()
                .to_string(),
        );
    } else {
        objects.push(local.join("packages/js").display().to_string());
    }

    DxForgeRegistryOperationReport {
        action: "registry-init-plan".to_string(),
        package_id: package.map(|value| canonical_package_id(value).to_string()),
        version: None,
        remote: local.display().to_string(),
        dry_run: true,
        r2_status: None,
        objects,
    }
}

fn ensure_root_dx_publish_package_matches(project: &Path, requested_package: &str) -> DxResult<()> {
    let root_package = root_dx_registry_package(project).map_err(forge_error)?;
    let canonical = canonical_package_id(requested_package);
    if root_package.package_id != canonical {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "root dx package is `{}`, not `{canonical}`",
                root_package.package_id
            ),
            field: Some("forge publish".to_string()),
        });
    }
    Ok(())
}

fn root_dx_publish_package_matches(project: &Path, requested_package: &str) -> bool {
    let canonical = canonical_package_id(requested_package);
    root_dx_registry_package(project).is_ok_and(|package| package.package_id == canonical)
}

fn resolve_local_registry_version(
    local_registry: &Path,
    package_id: &str,
    version: Option<&str>,
) -> DxResult<String> {
    if let Some(version) = version {
        let version = version.trim();
        if version.is_empty() {
            return Err(DxError::ConfigValidationError {
                message: "--version cannot be empty".to_string(),
                field: Some("forge add".to_string()),
            });
        }
        return Ok(version.to_string());
    }
    latest_local_registry_package_version(local_registry, package_id).map_err(forge_error)
}

fn forge_registry_publish_args(
    remote: &str,
    package: &str,
    write: bool,
    dry_run: bool,
    confirmed: bool,
) -> Vec<String> {
    let mut args = vec![
        "--remote".to_string(),
        remote.to_string(),
        "--package".to_string(),
        package.to_string(),
    ];
    if write {
        args.push("--write".to_string());
    }
    if dry_run {
        args.push("--dry-run".to_string());
    }
    if confirmed {
        args.push("--yes".to_string());
    }
    args
}

fn forge_r2_publish_report(
    package: &str,
    write: bool,
    dry_run: bool,
    confirmed: bool,
) -> DxResult<DxForgeRegistryOperationReport> {
    if write && dry_run {
        return Err(DxError::ConfigValidationError {
            message: "Choose either --dry-run or --write, not both".to_string(),
            field: Some("forge publish".to_string()),
        });
    }
    let dry_run = !write || dry_run;
    if !dry_run && !confirmed {
        return Err(DxError::ConfigValidationError {
            message: "dx forge publish --registry r2 --write requires --yes; run --dry-run first and get operator approval before live R2 upload".to_string(),
            field: Some("forge publish".to_string()),
        });
    }
    block_on_registry(publish_registry_package_to_r2(package, dry_run))
}

fn print_registry_report(
    report: &DxForgeRegistryOperationReport,
    format: DxOutputFormat,
) -> DxResult<()> {
    match format {
        DxOutputFormat::Terminal | DxOutputFormat::Markdown => {
            println!("{}", registry_operation_markdown(report));
        }
        DxOutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(report).map_err(forge_error)?
            );
        }
    }
    Ok(())
}

pub(super) fn forge_error(error: impl std::fmt::Display) -> DxError {
    DxError::InternalError {
        message: error.to_string(),
    }
}

#[derive(Debug, Serialize)]
struct DxForgeReleaseEvidenceReport {
    project: PathBuf,
    generated_at: String,
    passed: bool,
    check_score: u8,
    check_traffic: String,
    release_gate_score: u8,
    launch_gate_findings: Vec<DxCheckFinding>,
    registry_integrity: Vec<DxForgeDoctorRegistryCheck>,
    rollback_coverage_percent: u64,
    rollback_missing_packages: u64,
    package_docs_coverage_percent: u64,
    package_docs_missing: u64,
    package_scorecard: DxForgePackageScorecardReport,
    benchmark_history_path: PathBuf,
    latest_benchmark: Option<DxForgeBenchmarkSnapshot>,
    latest_forge_route_benchmark: Option<DxForgeBenchmarkSnapshot>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseNotesReport {
    version: u32,
    project: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    status: String,
    ci_readiness: DxForgeReleaseNotesReadiness,
    package_scorecard: DxForgeReleaseNotesScorecard,
    route_measurements: DxForgeReleaseNotesRouteMeasurements,
    honest_launch_limitations: Vec<String>,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseNotesReadiness {
    smoke_passed: bool,
    smoke_score: u8,
    release_evidence_passed: bool,
    release_evidence_score: u8,
    launch_page_quality_score: u8,
    doctor_passed: bool,
    verify_passed: bool,
    no_node_modules: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseNotesScorecard {
    score: u8,
    package_count: usize,
    verified_packages: usize,
    source_owned_packages: usize,
    node_modules_packages: usize,
    packages: Vec<DxForgeReleaseNotesPackage>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseNotesPackage {
    package_id: String,
    version: String,
    file_count: u64,
    integrity_verified: bool,
    source_owned: bool,
    install_scripts_blocked: bool,
    node_modules_created: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseNotesRouteMeasurements {
    benchmark_history_path: PathBuf,
    latest_forge_route_benchmark: Option<DxForgeBenchmarkSnapshot>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseDashboardReport {
    version: u32,
    project: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    checks: DxForgeReleaseDashboardChecks,
    ci_artifacts: DxForgeReleaseDashboardCiArtifacts,
    pages_bundle: DxForgeReleaseDashboardPagesBundle,
    release_notes: DxForgeReleaseDashboardReleaseNotes,
    launch_changelog: DxForgeReleaseDashboardLaunchChangelog,
    public_evidence: DxForgeReleaseDashboardPublicEvidence,
    route_comparison: DxForgeReleaseDashboardRouteComparison,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseDashboardChecks {
    ci_artifacts: DxForgeReleaseDashboardCheck,
    pages_bundle: DxForgeReleaseDashboardCheck,
    release_notes: DxForgeReleaseDashboardCheck,
    launch_changelog: DxForgeReleaseDashboardCheck,
    public_evidence: DxForgeReleaseDashboardCheck,
    route_comparison: DxForgeReleaseDashboardCheck,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseDashboardCheck {
    passed: bool,
    score: u8,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseDashboardCiArtifacts {
    artifact_dir: PathBuf,
    passed: bool,
    score: u8,
    artifact_count: usize,
    route_count: usize,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseDashboardPagesBundle {
    bundle_dir: PathBuf,
    passed: bool,
    score: u8,
    artifact_count: usize,
    check_count: usize,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseDashboardReleaseNotes {
    passed: bool,
    score: u8,
    status: String,
    no_node_modules: bool,
    package_count: usize,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseDashboardLaunchChangelog {
    history_path: PathBuf,
    passed: bool,
    score: u8,
    status: String,
    record_count: usize,
    honest_scope_count: usize,
    latest_route_count: u64,
    findings: Vec<String>,
}

struct DxForgeReleaseDashboardLaunchChangelogInput<'a> {
    project: &'a Path,
    ci_artifact_dir: &'a Path,
    route_comparison_path: &'a Path,
    generated_at: &'a str,
    fail_under: u8,
    base_score: u8,
    base_passed: bool,
    base_checks: [(&'static str, &'a DxForgeReleaseDashboardCheck); 5],
    ci_artifacts: &'a DxForgeReleaseDashboardCiArtifacts,
    pages_bundle: &'a DxForgeReleaseDashboardPagesBundle,
    release_notes: &'a DxForgeReleaseDashboardReleaseNotes,
    public_evidence: &'a DxForgeReleaseDashboardPublicEvidence,
    route_comparison: &'a DxForgeReleaseDashboardRouteComparison,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseDashboardPublicEvidence {
    route: String,
    passed: bool,
    score: u8,
    package_count: usize,
    links: usize,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseDashboardRouteComparison {
    path: PathBuf,
    passed: bool,
    score: u8,
    route_count: usize,
    total_decoded_bytes: u64,
    total_brotli_bytes: u64,
    missing_routes: Vec<String>,
    non_static_routes: Vec<String>,
    failing_budget_routes: Vec<String>,
    findings: Vec<String>,
}

#[derive(Debug, Serialize)]
struct DxForgeScorecardCliReport {
    #[serde(flatten)]
    scorecard: DxForgePackageScorecardReport,
    #[serde(skip_serializing_if = "Option::is_none")]
    latest_forge_route_benchmark: Option<DxForgeBenchmarkSnapshot>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeVerifyPackageReport {
    project: PathBuf,
    package_id: String,
    variant: String,
    score: u8,
    passed: bool,
    registry_integrity: DxForgeVerifyCheck,
    docs: DxForgeVerifyCheck,
    update: DxForgeVerifyCheck,
    rollback: DxForgeVerifyCheck,
    scorecard: DxForgeVerifyCheck,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    package_specific_checks: Vec<DxForgeVerifyCheck>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeVerifyAllPackagesReport {
    project: PathBuf,
    generated_at: String,
    score: u8,
    passed: bool,
    packages: Vec<DxForgeVerifyPackageReport>,
    missing_packages: Vec<DxForgeVerifyMissingPackage>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeVerifyMissingPackage {
    package_id: String,
    variant: String,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeMigrationGuideReport {
    version: u32,
    project: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    package_id: String,
    variant: String,
    upstream_command: String,
    forge_commands: DxForgeMigrationGuideCommands,
    checks: DxForgeMigrationGuideChecks,
    expectation_map: Vec<DxForgeMigrationExpectation>,
    file_map: Vec<DxForgeMigrationFile>,
    docs_path: PathBuf,
    receipt_count: usize,
    latest_receipt: Option<PathBuf>,
    update_preview: DxForgeMigrationUpdatePreview,
    no_node_modules: bool,
    ownership_boundaries: Vec<String>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeMigrationGuideCommands {
    upstream: String,
    dry_run: String,
    write: String,
    verify: String,
    update_preview: String,
    package_gallery: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeMigrationGuideChecks {
    materialized: DxForgeMigrationCheck,
    docs: DxForgeMigrationCheck,
    receipts: DxForgeMigrationCheck,
    verify_package: DxForgeMigrationCheck,
    local_ownership: DxForgeMigrationCheck,
    no_node_modules: DxForgeMigrationCheck,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeMigrationCheck {
    passed: bool,
    score: u8,
    message: String,
    evidence: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeMigrationExpectation {
    upstream_expectation: String,
    forge_behavior: String,
    evidence: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeMigrationFile {
    logical_path: String,
    materialized_path: String,
    exists: bool,
    bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeMigrationUpdatePreview {
    traffic: String,
    changed_files: u64,
    current_version: Option<String>,
    latest_version: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePackageGalleryReport {
    version: u32,
    project: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_node_modules: bool,
    package_count: usize,
    packages: Vec<DxForgePackageGalleryPackage>,
    findings: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hosted_index: Option<DxForgePackageGalleryHostedIndex>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePackageGalleryHostedIndex {
    route: String,
    out_dir: PathBuf,
    html_path: PathBuf,
    json_path: PathBuf,
    markdown_path: PathBuf,
    passed: bool,
    artifact_count: usize,
    package_count: usize,
    migration_guides: Vec<DxForgePackageGalleryMigrationGuide>,
    migration_gallery: DxForgeMigrationGalleryHostedArtifact,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeMigrationGalleryHostedArtifact {
    route: String,
    out_dir: PathBuf,
    html_path: PathBuf,
    json_path: PathBuf,
    markdown_path: PathBuf,
    passed: bool,
    artifact_count: usize,
    package_id: String,
    score: u8,
    no_node_modules: bool,
    supported_scope: Vec<String>,
    manual_gaps: Vec<String>,
    package_evidence: Vec<DxForgePackageGalleryCheck>,
    payload_comparison_boundaries: Vec<String>,
    next_commands: Vec<String>,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePackageGalleryMigrationGuide {
    package_id: String,
    title: String,
    command: String,
    href: String,
    supported: bool,
    summary: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePackageGalleryPackage {
    package_id: String,
    variant: String,
    version: String,
    description: String,
    materialized: bool,
    source_owned: bool,
    ownership_boundary: String,
    public_claim: String,
    launch_boundary: String,
    file_map: Vec<DxForgePackageGalleryFile>,
    advisory: DxForgePackageGalleryAdvisory,
    docs_status: DxForgePackageGalleryCheck,
    update_status: DxForgePackageGalleryUpdate,
    rollback_status: DxForgePackageGalleryCheck,
    scorecard_status: DxForgePackageGalleryCheck,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    migration_checks: Vec<DxForgePackageGalleryCheck>,
    score: u8,
    passed: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePackageGalleryFile {
    logical_path: String,
    materialized_path: String,
    hash: String,
    bytes: u64,
    exists: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePackageGalleryAdvisory {
    coverage_kind: String,
    provider: String,
    live_coverage: bool,
    finding_count: u64,
    reviewed_at: Option<String>,
    placeholder_present: bool,
    note: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePackageGalleryCheck {
    name: String,
    passed: bool,
    traffic: String,
    score: u8,
    message: String,
    evidence: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePackageGalleryUpdate {
    #[serde(flatten)]
    check: DxForgePackageGalleryCheck,
    current_version: Option<String>,
    latest_version: Option<String>,
    changed_files: u64,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeSmokeReport {
    project: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    no_node_modules: bool,
    packages: Vec<DxForgeSmokePackage>,
    check_score: u8,
    check_traffic: String,
    release_gate_score: u8,
    doctor_passed: bool,
    doctor_score: u8,
    verify_passed: bool,
    verify_score: u8,
    scorecard_score: u8,
    #[serde(flatten)]
    launch_artifacts: DxForgeSmokeArtifacts,
    launch_page_quality: DxForgeLaunchPageQualityReport,
    launch_gate_findings: Vec<DxCheckFinding>,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeSmokeArtifacts {
    benchmark_history_path: PathBuf,
    evidence_report_path: PathBuf,
    scorecard_report_path: PathBuf,
    launch_source_path: PathBuf,
    launch_html_path: PathBuf,
    launch_packet_path: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_runtime_path: Option<PathBuf>,
    launch_summary_path: PathBuf,
    launch_claims_path: PathBuf,
    launch_evidence_model_path: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeSmokePackage {
    package_id: String,
    variant: String,
    files_written: u64,
    risk_score: u8,
    receipt_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeInitAppReport {
    version: u32,
    project: PathBuf,
    generated_at: String,
    mode: String,
    passed: bool,
    score: u8,
    no_node_modules: bool,
    package_ids: Vec<String>,
    planned_files: Vec<String>,
    scaffolded_files: Vec<PathBuf>,
    packages: Vec<DxForgeInitAppPackage>,
    source_manifest_path: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    scorecard_report_path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dx_check_report_path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    check_score: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    check_traffic: Option<String>,
    strict_forge_passed: bool,
    launch_gate_findings: Vec<DxCheckFinding>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeInitAppPackage {
    package_id: String,
    variant: String,
    risk_score: u8,
    files: u64,
    wrote_files: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    manifest_path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receipt_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeAdoptionSmokeReport {
    version: u32,
    project: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_node_modules: bool,
    package_count: usize,
    route_count: usize,
    scaffolded_files: Vec<PathBuf>,
    routes: Vec<DxForgeAdoptionSmokeRoute>,
    smoke_report_path: PathBuf,
    adoption_artifacts_dir: PathBuf,
    release_bundle_dir: PathBuf,
    release_bundle_manifest_path: PathBuf,
    public_dir: PathBuf,
    source_manifest_path: PathBuf,
    route_comparison_path: PathBuf,
    release_history_path: PathBuf,
    smoke_score: u8,
    release_bundle_score: u8,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeAdoptionSmokeRoute {
    route: String,
    html_path: PathBuf,
    clean_index_path: PathBuf,
    packet_path: PathBuf,
    claims_path: Option<PathBuf>,
    proof_path: PathBuf,
    exists: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeAdoptionReport {
    version: u32,
    project: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_node_modules: bool,
    package_count: usize,
    packages: Vec<DxForgeAdoptionPackageEvidence>,
    receipt_count: usize,
    package_docs_present: usize,
    package_docs_missing: usize,
    project_structure: DxForgeAdoptionProjectStructure,
    source_manifest_path: PathBuf,
    receipt_dir: PathBuf,
    package_docs_dir: PathBuf,
    public_dir: PathBuf,
    public_routes: Vec<DxForgeAdoptionRouteArtifact>,
    release_bundle: DxForgeAdoptionReleaseBundleEvidence,
    dx_check: DxForgeAdoptionCheckEvidence,
    findings: Vec<String>,
    honest_scope: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeAdoptionPackageEvidence {
    package_id: String,
    variant: String,
    version: String,
    file_count: usize,
    docs_path: PathBuf,
    docs_exists: bool,
    rollback_receipt: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeAdoptionProjectStructure {
    dx_config_path: PathBuf,
    dx_config_exists: bool,
    pages_dir: PathBuf,
    pages_dir_exists: bool,
    components_dir: PathBuf,
    components_dir_exists: bool,
    app_route_path: PathBuf,
    app_route_exists: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeAdoptionRouteArtifact {
    route: String,
    html_path: PathBuf,
    clean_index_path: PathBuf,
    packet_path: PathBuf,
    proof_path: PathBuf,
    claims_path: Option<PathBuf>,
    html_exists: bool,
    clean_index_exists: bool,
    packet_exists: bool,
    proof_exists: bool,
    passed: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeAdoptionReleaseBundleEvidence {
    bundle_dir: PathBuf,
    exists: bool,
    passed: bool,
    score: u8,
    artifact_count: usize,
    route_count: usize,
    no_node_modules: bool,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeAdoptionCheckEvidence {
    score: u8,
    release_gate_score: u8,
    traffic: String,
    strict_forge_passed: bool,
    section_count: usize,
    finding_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeBetaInstallReport {
    version: u32,
    project: PathBuf,
    release_bundle_dir: PathBuf,
    artifact_dir: PathBuf,
    generated_at: String,
    mode: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    clean_project: bool,
    no_node_modules: bool,
    wrote_project: bool,
    script_path: Option<PathBuf>,
    release_manifest_status: String,
    release_bundle: DxForgeBetaInstallCheck,
    release_manifest: DxForgeBetaInstallCheck,
    init_app: DxForgeBetaInstallCheck,
    provenance: DxForgeBetaInstallCheck,
    trust_regression: DxForgeBetaInstallCheck,
    adoption_report: DxForgeBetaInstallCheck,
    routes: Vec<DxForgeAdoptionSmokeRoute>,
    artifacts: Vec<PathBuf>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeBetaInstallCheck {
    name: String,
    passed: bool,
    score: u8,
    message: String,
    evidence: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeBetaUpgradeSmokeReport {
    version: u32,
    project: PathBuf,
    from_release_bundle_dir: PathBuf,
    to_release_bundle_dir: PathBuf,
    artifact_dir: PathBuf,
    generated_at: String,
    mode: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_node_modules: bool,
    from_release_manifest: DxForgeBetaInstallCheck,
    to_release_bundle: DxForgeBetaInstallCheck,
    to_release_manifest: DxForgeBetaInstallCheck,
    initial_install: DxForgeBetaInstallCheck,
    reviewed_update: DxForgeBetaInstallCheck,
    provenance: DxForgeBetaInstallCheck,
    trust_regression: DxForgeBetaInstallCheck,
    adoption_report: DxForgeBetaInstallCheck,
    local_edit: DxForgeBetaUpgradeLocalEdit,
    routes: Vec<DxForgeAdoptionSmokeRoute>,
    artifacts: Vec<PathBuf>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeBetaUpgradeLocalEdit {
    package_id: String,
    path: String,
    marker: String,
    preserved: bool,
    reviewed_update_traffic: String,
    receipt_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeCiArtifactVerificationReport {
    artifact_dir: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    artifacts: Vec<DxForgeCiArtifactCheck>,
    routes: Vec<DxForgeCiRouteArtifactCheck>,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePagesBundleVerificationReport {
    bundle_dir: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    artifacts: Vec<DxForgeCiArtifactCheck>,
    checks: Vec<DxForgeCiRouteArtifactCheck>,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseBundleReport {
    version: u32,
    bundle_dir: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    artifact_count: usize,
    route_count: usize,
    no_node_modules: bool,
    artifacts: Vec<DxForgeCiArtifactCheck>,
    routes: Vec<DxForgeCiRouteArtifactCheck>,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DxForgeReleaseBundleManifest {
    version: u32,
    generated_at: String,
    artifact_count: usize,
    hash_algorithm: String,
    artifacts: Vec<DxForgeReleaseBundleManifestArtifact>,
    integrity: DxForgeReleaseBundleManifestIntegrity,
    artifact_integrity: DxForgeReleaseBundleArtifactIntegrity,
    publisher_identity: DxForgeReleaseBundlePublisherIdentity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DxForgeReleaseBundleManifestArtifact {
    path: String,
    artifact_type: String,
    route: Option<String>,
    bytes: u64,
    blake3: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DxForgeReleaseBundleManifestIntegrity {
    scheme: String,
    signed: bool,
    digest: String,
    signature: Option<String>,
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DxForgeReleaseBundleArtifactIntegrity {
    scheme: String,
    hash_algorithm: String,
    digest: String,
    artifact_count: usize,
    verified_locally: bool,
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DxForgeReleaseBundlePublisherIdentity {
    scheme: String,
    status: String,
    signer: Option<String>,
    key_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    algorithm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    public_key: Option<String>,
    signature: Option<String>,
    signed_at: Option<String>,
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DxForgePublisherPrivateKeyFile {
    version: u32,
    scheme: String,
    created_at: String,
    signer: String,
    algorithm: String,
    key_id: String,
    public_key: String,
    private_key: String,
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DxForgePublisherPublicKeyFile {
    version: u32,
    scheme: String,
    created_at: String,
    signer: String,
    algorithm: String,
    key_id: String,
    public_key: String,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePublisherKeyGenerateReport {
    version: u32,
    generated_at: String,
    passed: bool,
    score: u8,
    signer: String,
    algorithm: String,
    key_id: String,
    public_key: String,
    private_key_path: PathBuf,
    public_key_path: PathBuf,
    private_key_written: bool,
    public_key_written: bool,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePublisherKeySignReport {
    version: u32,
    generated_at: String,
    passed: bool,
    score: u8,
    signer: String,
    key_id: String,
    public_key: String,
    key_path: PathBuf,
    manifest_path: PathBuf,
    output_manifest_path: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    markdown_path: Option<PathBuf>,
    signed_at: String,
    wrote_manifest: bool,
    signature: String,
    signature_verified: bool,
    manifest_digest: String,
    artifact_count: usize,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseOperationsReport {
    version: u32,
    project: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    status: String,
    fail_under: u8,
    inputs: DxForgeReleaseOperationsInputs,
    checks: DxForgeReleaseOperationsChecks,
    signed_manifest: DxForgeReleaseOperationsSignedManifest,
    trust_regression: DxForgeReleaseOperationsJsonEvidence,
    release_candidate: DxForgeReleaseOperationsJsonEvidence,
    ci_artifacts: DxForgeReleaseOperationsArtifactEvidence,
    public_evidence: DxForgeReleaseOperationsArtifactEvidence,
    package_gallery: DxForgeReleaseOperationsPackageGallery,
    no_node_modules: DxForgeReleaseCandidateNoNodeModules,
    shipping_gate: Vec<DxForgeReleaseOperationsSignoffItem>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseOperationsInputs {
    release_manifest: PathBuf,
    trust_regression: PathBuf,
    release_candidate: PathBuf,
    ci_artifacts: PathBuf,
    public_evidence: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseOperationsChecks {
    signed_manifest: DxForgeReleaseOperationsCheck,
    trust_regression: DxForgeReleaseOperationsCheck,
    release_candidate: DxForgeReleaseOperationsCheck,
    ci_artifacts: DxForgeReleaseOperationsCheck,
    public_evidence: DxForgeReleaseOperationsCheck,
    package_gallery: DxForgeReleaseOperationsCheck,
    no_node_modules: DxForgeReleaseOperationsCheck,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseOperationsCheck {
    passed: bool,
    score: u8,
    message: String,
    evidence: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseOperationsSignedManifest {
    path: PathBuf,
    artifact_dir: PathBuf,
    exists: bool,
    signed: bool,
    publisher_status: String,
    publisher_signer: Option<String>,
    publisher_key_id: Option<String>,
    signature_verified: bool,
    manifest_digest_verified: bool,
    artifact_integrity_verified: bool,
    artifact_count: usize,
    digest: Option<String>,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseOperationsJsonEvidence {
    path: PathBuf,
    exists: bool,
    passed: bool,
    score: u8,
    label: String,
    case_count: Option<u64>,
    check_count: Option<usize>,
    no_node_modules: Option<bool>,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseOperationsArtifactEvidence {
    path: PathBuf,
    passed: bool,
    score: u8,
    artifact_count: usize,
    check_count: usize,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseOperationsPackageGallery {
    route: String,
    artifact_dir: PathBuf,
    html_path: PathBuf,
    json_path: PathBuf,
    markdown_path: PathBuf,
    passed: bool,
    score: u8,
    artifact_count: usize,
    package_count: usize,
    migration_guide_count: usize,
    no_node_modules: bool,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseOperationsSignoffItem {
    label: String,
    artifact: String,
    status: String,
    required: bool,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePublishPlanReport {
    version: u32,
    project: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    status: String,
    fail_under: u8,
    inputs: DxForgePublishPlanInputs,
    checks: DxForgePublishPlanChecks,
    artifact_targets: Vec<DxForgePublishPlanArtifactTarget>,
    cache_headers: Vec<DxForgePublishPlanCacheHeader>,
    rollback_inputs: Vec<DxForgePublishPlanRollbackInput>,
    secret_requirements: DxForgePublishPlanSecretRequirements,
    no_node_modules: DxForgeReleaseCandidateNoNodeModules,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeBetaArtifactVerifyReport {
    version: u32,
    release_bundle: PathBuf,
    pages: PathBuf,
    registry_smoke: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    status: String,
    fail_under: u8,
    requires_rebuild: bool,
    checks: DxForgeBetaArtifactVerifyChecks,
    release_bundle_report: DxForgeReleaseBundleReport,
    pages_bundle_report: DxForgePagesBundleVerificationReport,
    signed_manifest: DxForgeReleaseOperationsSignedManifest,
    artifact_targets: Vec<DxForgePublishPlanArtifactTarget>,
    cache_headers: Vec<DxForgePublishPlanCacheHeader>,
    rollback_inputs: Vec<DxForgePublishPlanRollbackInput>,
    secret_requirements: DxForgePublishPlanSecretRequirements,
    no_node_modules: DxForgeReleaseCandidateNoNodeModules,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeBetaArtifactVerifyChecks {
    release_bundle: DxForgeReleaseOperationsCheck,
    pages_bundle: DxForgeReleaseOperationsCheck,
    r2_evidence: DxForgeReleaseOperationsCheck,
    cache_policy: DxForgeReleaseOperationsCheck,
    rollback_inputs: DxForgeReleaseOperationsCheck,
    no_secret_requirements: DxForgeReleaseOperationsCheck,
    no_node_modules: DxForgeReleaseOperationsCheck,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePublishPlanInputs {
    release_bundle: PathBuf,
    pages: PathBuf,
    registry_smoke: PathBuf,
    release_operations: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePublishPlanChecks {
    local_artifacts: DxForgePublishPlanCheck,
    pages_artifacts: DxForgePublishPlanCheck,
    r2_artifacts: DxForgePublishPlanCheck,
    cache_headers: DxForgePublishPlanCheck,
    rollback_inputs: DxForgePublishPlanCheck,
    no_secret_requirements: DxForgePublishPlanCheck,
    no_node_modules: DxForgePublishPlanCheck,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePublishPlanCheck {
    passed: bool,
    score: u8,
    message: String,
    evidence: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePublishPlanArtifactTarget {
    channel: String,
    artifact: String,
    source: String,
    destination: String,
    route: Option<String>,
    cache_control: String,
    required: bool,
    passed: bool,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePublishPlanCacheHeader {
    channel: String,
    pattern: String,
    cache_control: String,
    required: bool,
    passed: bool,
    reason: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePublishPlanRollbackInput {
    name: String,
    path: PathBuf,
    required: bool,
    exists: bool,
    passed: bool,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgePublishPlanSecretRequirements {
    requires_secrets: bool,
    registry_smoke_requires_secrets: bool,
    registry_operations_dry_run: bool,
    blocked_markers: Vec<String>,
    scanned_paths: Vec<PathBuf>,
    passed: bool,
    score: u8,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseReviewReport {
    version: u32,
    project: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    status: String,
    fail_under: u8,
    inputs: DxForgeReleaseReviewInputs,
    checks: DxForgeReleaseReviewChecks,
    release_dashboard: DxForgeReleaseReviewDashboard,
    release_bundle: DxForgeReleaseReviewBundle,
    launch_changelog: DxForgeReleaseReviewLaunchChangelog,
    route_comparison: DxForgeReleaseReviewRouteComparison,
    release_history: DxForgeReleaseReviewHistory,
    signoff_items: Vec<DxForgeReleaseReviewSignoffItem>,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseReviewInputs {
    dashboard: PathBuf,
    bundle_dir: PathBuf,
    bundle_manifest: PathBuf,
    launch_changelog: PathBuf,
    route_comparison: PathBuf,
    release_history: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseReviewChecks {
    release_dashboard: DxForgeReleaseReviewCheck,
    release_bundle: DxForgeReleaseReviewCheck,
    bundle_manifest: DxForgeReleaseReviewCheck,
    launch_changelog: DxForgeReleaseReviewCheck,
    route_comparison: DxForgeReleaseReviewCheck,
    release_history: DxForgeReleaseReviewCheck,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseReviewCheck {
    passed: bool,
    score: u8,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseReviewDashboard {
    path: PathBuf,
    passed: bool,
    score: u8,
    no_node_modules: bool,
    check_count: usize,
    finding_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseReviewBundle {
    bundle_dir: PathBuf,
    passed: bool,
    score: u8,
    route_count: usize,
    artifact_count: usize,
    no_node_modules: bool,
    manifest_path: PathBuf,
    manifest_artifacts: usize,
    manifest_digest: String,
    manifest_digest_verified: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseReviewLaunchChangelog {
    path: PathBuf,
    passed: bool,
    score: u8,
    status: String,
    record_count: usize,
    honest_scope_count: usize,
    finding_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseReviewRouteComparison {
    path: PathBuf,
    passed: bool,
    score: u8,
    route_count: u64,
    total_decoded_bytes: u64,
    total_brotli_bytes: u64,
    missing_routes: Vec<String>,
    failing_budget_routes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseReviewHistory {
    path: PathBuf,
    passed: bool,
    score: u8,
    record_count: usize,
    latest_dashboard_score: Option<u8>,
    latest_route_count: Option<u64>,
    latest_total_brotli_bytes: Option<u64>,
    latest_regression_findings: usize,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseReviewSignoffItem {
    label: String,
    artifact: String,
    status: String,
    required: bool,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeCiArtifactCheck {
    name: String,
    path: PathBuf,
    exists: bool,
    bytes: u64,
    valid_json: Option<bool>,
    passed: bool,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeCiRouteArtifactCheck {
    route: String,
    artifacts: Vec<String>,
    passed: bool,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReadinessBadge {
    #[serde(rename = "schemaVersion")]
    schema_version: u8,
    generated_at: String,
    project: PathBuf,
    label: String,
    status: String,
    message: String,
    color: String,
    score: u8,
    passed: bool,
    #[serde(rename = "isError")]
    is_error: bool,
    fail_under: u8,
    no_node_modules: bool,
    smoke: DxForgeReadinessBadgeCheck,
    evidence: DxForgeReadinessBadgeCheck,
    scorecard: DxForgeReadinessBadgeCheck,
    launch_page_quality: DxForgeReadinessBadgeCheck,
    #[serde(skip_serializing_if = "Option::is_none")]
    latest_forge_route_benchmark: Option<DxForgeReadinessBadgeBenchmark>,
    artifacts: DxForgeReadinessBadgeArtifacts,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReadinessBadgeCheck {
    passed: bool,
    score: u8,
    summary: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReadinessBadgeBenchmark {
    passed: bool,
    generated_at: Option<String>,
    fixture_mode: Option<String>,
    route_delivery: Option<String>,
    forge_packages: Option<u64>,
    forge_files_tracked: Option<u64>,
    decoded_bytes: Option<u64>,
    brotli_bytes: Option<u64>,
    http_route_median_ms: Option<f64>,
    chrome_load_event_ms: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReadinessBadgeArtifacts {
    #[serde(skip_serializing_if = "Option::is_none")]
    smoke_report: Option<PathBuf>,
    release_evidence: PathBuf,
    package_scorecard: PathBuf,
    benchmark_history: PathBuf,
    launch_html: PathBuf,
    launch_claims: PathBuf,
    launch_evidence_model: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeLaunchPageQualityReport {
    passed: bool,
    score: u8,
    headings: DxForgeLaunchPageQualityCheck,
    seo: DxForgeLaunchPageQualityCheck,
    links: DxForgeLaunchPageQualityCheck,
    claims_manifest: DxForgeLaunchPageQualityCheck,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeLaunchPageQualityCheck {
    passed: bool,
    message: String,
    evidence: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeVerifyCheck {
    name: String,
    passed: bool,
    traffic: DxUpdateTraffic,
    score: u8,
    message: String,
    evidence: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DxForgeBenchmarkHistoryIndex {
    #[serde(default)]
    snapshots: Vec<DxForgeBenchmarkSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DxForgeBenchmarkSnapshot {
    #[serde(default)]
    generated_at: Option<String>,
    #[serde(default)]
    fixture_mode: Option<String>,
    #[serde(default)]
    route_delivery: Option<String>,
    #[serde(default)]
    forge_packages: Option<u64>,
    #[serde(default)]
    forge_files_tracked: Option<u64>,
    #[serde(default)]
    decoded_bytes: Option<u64>,
    #[serde(default)]
    brotli_bytes: Option<u64>,
    #[serde(default)]
    http_route_median_ms: Option<f64>,
    #[serde(default)]
    chrome_load_event_ms: Option<f64>,
    #[serde(default)]
    dx_packet_applied: Option<bool>,
    #[serde(default)]
    interaction_works: Option<bool>,
    #[serde(default)]
    markdown: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DxForgeReleaseEvidenceHistoryIndex {
    version: u32,
    updated_at: String,
    snapshots: Vec<DxForgeReleaseEvidenceHistoryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DxForgeReleaseEvidenceHistoryEntry {
    generated_at: String,
    passed: bool,
    check_score: u8,
    previous_check_score: Option<u8>,
    check_score_delta: i16,
    check_traffic: String,
    registry_verified_count: u64,
    registry_package_count: u64,
    package_score: u8,
    package_count: u64,
    latest_benchmark_fixture_mode: Option<String>,
    snapshot_file: String,
}

fn build_forge_release_notes_report(
    project: &Path,
    benchmark_history_path: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeReleaseNotesReport> {
    let smoke = build_forge_smoke_report(project)?;
    let evidence = build_forge_release_evidence_report(project, benchmark_history_path)?;
    let badge = build_forge_readiness_badge(&smoke, &evidence, None, fail_under);
    let scorecard = &evidence.package_scorecard;
    let packages = scorecard
        .packages
        .iter()
        .map(|package| DxForgeReleaseNotesPackage {
            package_id: package.package_id.clone(),
            version: package.version.clone(),
            file_count: package.file_count,
            integrity_verified: package.integrity_verified,
            source_owned: package.source_owned,
            install_scripts_blocked: package.install_scripts_blocked,
            node_modules_created: package.node_modules_created,
        })
        .collect::<Vec<_>>();
    let verified_packages = scorecard
        .packages
        .iter()
        .filter(|package| package.integrity_verified)
        .count();
    let source_owned_packages = scorecard
        .packages
        .iter()
        .filter(|package| package.source_owned)
        .count();
    let node_modules_packages = scorecard
        .packages
        .iter()
        .filter(|package| package.node_modules_created)
        .count();
    let latest_forge_route_benchmark = evidence
        .latest_forge_route_benchmark
        .clone()
        .or_else(|| evidence.latest_benchmark.clone());

    Ok(DxForgeReleaseNotesReport {
        version: 1,
        project: project.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed: badge.passed,
        score: badge.score,
        status: badge.status,
        ci_readiness: DxForgeReleaseNotesReadiness {
            smoke_passed: smoke.passed,
            smoke_score: smoke.score,
            release_evidence_passed: evidence.passed,
            release_evidence_score: evidence.release_gate_score,
            launch_page_quality_score: smoke.launch_page_quality.score,
            doctor_passed: smoke.doctor_passed,
            verify_passed: smoke.verify_passed,
            no_node_modules: smoke.no_node_modules,
        },
        package_scorecard: DxForgeReleaseNotesScorecard {
            score: scorecard.score,
            package_count: scorecard.packages.len(),
            verified_packages,
            source_owned_packages,
            node_modules_packages,
            packages,
        },
        route_measurements: DxForgeReleaseNotesRouteMeasurements {
            benchmark_history_path: benchmark_history_path.to_path_buf(),
            latest_forge_route_benchmark,
        },
        honest_launch_limitations: scorecard.honest_boundaries.clone(),
        findings: badge.findings,
    })
}

fn build_forge_release_dashboard_report(
    project: &Path,
    ci_artifacts: &Path,
    pages: &Path,
    history: &Path,
    route_comparison: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeReleaseDashboardReport> {
    let generated_at = Utc::now().to_rfc3339();
    let ci_report = verify_forge_ci_artifacts(ci_artifacts)?;
    let pages_report = verify_forge_pages_bundle(pages)?;
    let release_notes = build_forge_release_notes_report(project, history, fail_under)?;
    let scorecard = build_forge_package_scorecard_for_project(project)?;
    let public_evidence_report = build_forge_public_evidence_report(&scorecard);
    let public_evidence = verify_release_dashboard_public_evidence(&public_evidence_report);
    let route_comparison = verify_release_dashboard_route_comparison(route_comparison)?;
    let ci_artifacts = DxForgeReleaseDashboardCiArtifacts {
        artifact_dir: ci_report.artifact_dir.clone(),
        passed: ci_report.passed,
        score: ci_report.score,
        artifact_count: ci_report.artifacts.len(),
        route_count: ci_report.routes.len(),
        findings: ci_report.findings.clone(),
    };
    let pages_bundle = DxForgeReleaseDashboardPagesBundle {
        bundle_dir: pages_report.bundle_dir.clone(),
        passed: pages_report.passed,
        score: pages_report.score,
        artifact_count: pages_report.artifacts.len(),
        check_count: pages_report.checks.len(),
        findings: pages_report.findings.clone(),
    };
    let release_notes_summary = DxForgeReleaseDashboardReleaseNotes {
        passed: release_notes.passed,
        score: release_notes.score,
        status: release_notes.status.clone(),
        no_node_modules: release_notes.ci_readiness.no_node_modules,
        package_count: release_notes.package_scorecard.package_count,
        findings: release_notes.findings.clone(),
    };
    let ci_artifacts_check = release_dashboard_check(
        ci_artifacts.passed,
        ci_artifacts.score,
        format!(
            "{} CI artifacts and {} route checks verified.",
            ci_artifacts.artifact_count, ci_artifacts.route_count
        ),
    );
    let pages_bundle_check = release_dashboard_check(
        pages_bundle.passed,
        pages_bundle.score,
        format!(
            "{} Pages artifacts and {} publish checks verified.",
            pages_bundle.artifact_count, pages_bundle.check_count
        ),
    );
    let release_notes_check = release_dashboard_check(
        release_notes_summary.passed,
        release_notes_summary.score,
        format!(
            "Release notes status `{}` with {} source-owned packages.",
            release_notes_summary.status, release_notes_summary.package_count
        ),
    );
    let public_evidence_check = release_dashboard_check(
        public_evidence.passed,
        public_evidence.score,
        format!(
            "{} public evidence links checked for route `{}`.",
            public_evidence.links, public_evidence.route
        ),
    );
    let route_comparison_check = release_dashboard_check(
        route_comparison.passed,
        route_comparison.score,
        format!(
            "{} public routes measured, {} Brotli bytes total.",
            route_comparison.route_count, route_comparison.total_brotli_bytes
        ),
    );
    let base_score = [
        ci_artifacts_check.score,
        pages_bundle_check.score,
        release_notes_check.score,
        public_evidence_check.score,
        route_comparison_check.score,
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let base_passed = ci_artifacts_check.passed
        && pages_bundle_check.passed
        && release_notes_check.passed
        && public_evidence_check.passed
        && route_comparison_check.passed
        && base_score >= fail_under;
    let launch_changelog =
        verify_release_dashboard_launch_changelog(DxForgeReleaseDashboardLaunchChangelogInput {
            project,
            ci_artifact_dir: ci_artifacts.artifact_dir.as_path(),
            route_comparison_path: route_comparison.path.as_path(),
            generated_at: &generated_at,
            fail_under,
            base_score,
            base_passed,
            base_checks: [
                ("ci_artifacts", &ci_artifacts_check),
                ("pages_bundle", &pages_bundle_check),
                ("release_notes", &release_notes_check),
                ("public_evidence", &public_evidence_check),
                ("route_comparison", &route_comparison_check),
            ],
            ci_artifacts: &ci_artifacts,
            pages_bundle: &pages_bundle,
            release_notes: &release_notes_summary,
            public_evidence: &public_evidence,
            route_comparison: &route_comparison,
        })?;
    let launch_changelog_check = release_dashboard_check(
        launch_changelog.passed,
        launch_changelog.score,
        format!(
            "Launch changelog readiness reviewed release-history evidence with {} record(s), {} honest-scope guardrail(s), and {} route(s).",
            launch_changelog.record_count,
            launch_changelog.honest_scope_count,
            launch_changelog.latest_route_count
        ),
    );
    let checks = DxForgeReleaseDashboardChecks {
        ci_artifacts: ci_artifacts_check,
        pages_bundle: pages_bundle_check,
        release_notes: release_notes_check,
        launch_changelog: launch_changelog_check,
        public_evidence: public_evidence_check,
        route_comparison: route_comparison_check,
    };
    let mut findings = Vec::new();
    append_release_dashboard_findings("ci-artifacts", &ci_artifacts.findings, &mut findings);
    append_release_dashboard_findings("pages-bundle", &pages_bundle.findings, &mut findings);
    append_release_dashboard_findings(
        "release-notes",
        &release_notes_summary.findings,
        &mut findings,
    );
    append_release_dashboard_findings(
        "launch-changelog",
        &launch_changelog.findings,
        &mut findings,
    );
    append_release_dashboard_findings("public-evidence", &public_evidence.findings, &mut findings);
    append_release_dashboard_findings(
        "route-comparison",
        &route_comparison.findings,
        &mut findings,
    );

    let score = [
        checks.ci_artifacts.score,
        checks.pages_bundle.score,
        checks.release_notes.score,
        checks.launch_changelog.score,
        checks.public_evidence.score,
        checks.route_comparison.score,
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let passed = checks.ci_artifacts.passed
        && checks.pages_bundle.passed
        && checks.release_notes.passed
        && checks.launch_changelog.passed
        && checks.public_evidence.passed
        && checks.route_comparison.passed
        && score >= fail_under;

    Ok(DxForgeReleaseDashboardReport {
        version: 1,
        project: project.to_path_buf(),
        generated_at,
        passed,
        score,
        fail_under,
        checks,
        ci_artifacts,
        pages_bundle,
        release_notes: release_notes_summary,
        launch_changelog,
        public_evidence,
        route_comparison,
        findings,
    })
}

fn release_dashboard_check(
    passed: bool,
    score: u8,
    message: impl Into<String>,
) -> DxForgeReleaseDashboardCheck {
    DxForgeReleaseDashboardCheck {
        passed,
        score,
        message: message.into(),
    }
}

fn append_release_dashboard_findings(label: &str, findings: &[String], output: &mut Vec<String>) {
    for finding in findings {
        output.push(format!("{label}: {finding}"));
    }
}

fn verify_release_dashboard_launch_changelog(
    input: DxForgeReleaseDashboardLaunchChangelogInput<'_>,
) -> anyhow::Result<DxForgeReleaseDashboardLaunchChangelog> {
    let mut checks = serde_json::Map::new();
    for (name, check) in input.base_checks {
        checks.insert(
            name.to_string(),
            serde_json::json!({
                "passed": check.passed,
                "score": check.score,
                "message": check.message,
            }),
        );
    }

    let mut dashboard_findings = Vec::new();
    dashboard_findings.extend(input.ci_artifacts.findings.iter().cloned());
    dashboard_findings.extend(input.pages_bundle.findings.iter().cloned());
    dashboard_findings.extend(input.release_notes.findings.iter().cloned());
    dashboard_findings.extend(input.public_evidence.findings.iter().cloned());
    dashboard_findings.extend(input.route_comparison.findings.iter().cloned());

    let dashboard_value = serde_json::json!({
        "version": 1,
        "project": input.project.display().to_string(),
        "generated_at": input.generated_at,
        "passed": input.base_passed,
        "score": input.base_score,
        "fail_under": input.fail_under,
        "checks": checks,
        "release_notes": {
            "passed": input.release_notes.passed,
            "score": input.release_notes.score,
            "status": input.release_notes.status,
            "no_node_modules": input.release_notes.no_node_modules,
            "package_count": input.release_notes.package_count,
            "findings": input.release_notes.findings,
        },
        "public_evidence": {
            "route": input.public_evidence.route,
            "passed": input.public_evidence.passed,
            "score": input.public_evidence.score,
            "package_count": input.public_evidence.package_count,
            "links": input.public_evidence.links,
            "findings": input.public_evidence.findings,
        },
        "findings": dashboard_findings,
    });
    let route_comparison_value =
        serde_json::from_slice::<serde_json::Value>(&std::fs::read(input.route_comparison_path)?)?;
    let history_path = input
        .ci_artifact_dir
        .join("forge-public-release-history.json");
    let record = build_forge_public_release_record(
        input.project,
        &input.ci_artifact_dir.join("forge-release-dashboard.json"),
        input.route_comparison_path,
        &dashboard_value,
        &route_comparison_value,
    )?;
    let updated_at = record.generated_at.clone();
    let report = build_forge_launch_changelog_report_from_history(
        history_path.clone(),
        DxForgePublicReleaseHistory {
            updated_at,
            records: vec![record],
        },
    );

    let honest_scope_count = report.honest_scope.len();
    let latest_route_count = report
        .latest
        .as_ref()
        .map(|latest| latest.route_count)
        .unwrap_or_default();
    let mut findings = report.findings.clone();
    if honest_scope_count < 4 {
        findings.push(format!(
            "launch changelog honest-scope guardrails are incomplete: {honest_scope_count} present, expected at least 4"
        ));
    }
    let mut score = report.score;
    if honest_scope_count < 4 {
        score = score.saturating_sub(10);
    }
    let passed = report.passed && findings.is_empty() && honest_scope_count >= 4;

    Ok(DxForgeReleaseDashboardLaunchChangelog {
        history_path,
        passed,
        score,
        status: report.status,
        record_count: report.record_count,
        honest_scope_count,
        latest_route_count,
        findings,
    })
}

fn verify_release_dashboard_public_evidence(
    report: &DxForgePublicEvidenceReport,
) -> DxForgeReleaseDashboardPublicEvidence {
    let mut findings = Vec::new();
    let mut penalty = 0u16;

    if report.score < 95 {
        findings.push(format!(
            "public evidence score {} is below the release-ready floor 95",
            report.score
        ));
        penalty = penalty.saturating_add(20);
    }
    if report.links.len() < 9 {
        findings.push(format!(
            "public evidence lists {} links; expected at least 9",
            report.links.len()
        ));
        penalty = penalty.saturating_add(20);
    }

    for link in &report.links {
        let href = link.href.trim();
        if href.is_empty()
            || href.contains("..")
            || href.contains('\\')
            || href.contains(':')
            || href.starts_with('/')
        {
            findings.push(format!("unsafe public evidence href `{}`", link.href));
            penalty = penalty.saturating_add(10);
        }
    }

    for required in [
        "forge-readiness-badge.json",
        "forge-public-route-comparison.md",
        "forge/ci.html",
    ] {
        if !report.links.iter().any(|link| link.href == required) {
            findings.push(format!("missing public evidence link `{required}`"));
            penalty = penalty.saturating_add(15);
        }
    }

    let score = 100u8.saturating_sub(penalty.min(100) as u8);
    let passed = findings.is_empty();

    DxForgeReleaseDashboardPublicEvidence {
        route: report.route.clone(),
        passed,
        score,
        package_count: report.package_count,
        links: report.links.len(),
        findings,
    }
}

fn verify_release_dashboard_route_comparison(
    path: &Path,
) -> anyhow::Result<DxForgeReleaseDashboardRouteComparison> {
    let value = serde_json::from_slice::<serde_json::Value>(&std::fs::read(path)?)?;
    let routes = value
        .get("routes")
        .and_then(|routes| routes.as_array())
        .cloned()
        .unwrap_or_default();
    let mut missing_routes = Vec::new();
    let mut non_static_routes = Vec::new();
    let mut failing_budget_routes = Vec::new();
    let mut findings = Vec::new();
    let mut penalty = 0u16;

    for required in FORGE_REQUIRED_PUBLIC_ROUTES {
        match routes.iter().find(|route| {
            route
                .get("route")
                .and_then(|route| route.as_str())
                .is_some_and(|route| route == *required)
        }) {
            Some(route) => {
                let status = route.get("status").and_then(|status| status.as_str());
                if status != Some("measured") {
                    findings.push(format!(
                        "public route `{required}` has status `{}`",
                        status.unwrap_or("missing")
                    ));
                    penalty = penalty.saturating_add(12);
                }

                let delivery = route
                    .get("route_delivery")
                    .and_then(|delivery| delivery.as_str());
                if delivery != Some("static") {
                    non_static_routes.push(required.to_string());
                    penalty = penalty.saturating_add(18);
                }

                if route
                    .get("budget_passed")
                    .and_then(|budget| budget.as_bool())
                    != Some(true)
                {
                    failing_budget_routes.push(required.to_string());
                    penalty = penalty.saturating_add(18);
                }
            }
            None => {
                missing_routes.push((*required).to_string());
                penalty = penalty.saturating_add(25);
            }
        }
    }

    if !missing_routes.is_empty() {
        findings.push(format!(
            "missing public routes: {}",
            missing_routes.join(", ")
        ));
    }
    if !non_static_routes.is_empty() {
        findings.push(format!(
            "non-static public routes: {}",
            non_static_routes.join(", ")
        ));
    }
    if !failing_budget_routes.is_empty() {
        findings.push(format!(
            "budget-failing public routes: {}",
            failing_budget_routes.join(", ")
        ));
    }

    let route_count = value
        .get("route_count")
        .and_then(|count| count.as_u64())
        .unwrap_or(routes.len() as u64) as usize;
    if route_count < FORGE_REQUIRED_PUBLIC_ROUTES.len() {
        findings.push(format!(
            "route_count is {route_count}; expected at least {}",
            FORGE_REQUIRED_PUBLIC_ROUTES.len()
        ));
        penalty = penalty.saturating_add(20);
    }

    let score = 100u8.saturating_sub(penalty.min(100) as u8);
    let passed = findings.is_empty();

    Ok(DxForgeReleaseDashboardRouteComparison {
        path: path.to_path_buf(),
        passed,
        score,
        route_count,
        total_decoded_bytes: value
            .get("total_decoded_bytes")
            .and_then(|bytes| bytes.as_u64())
            .unwrap_or_default(),
        total_brotli_bytes: value
            .get("total_brotli_bytes")
            .and_then(|bytes| bytes.as_u64())
            .unwrap_or_default(),
        missing_routes,
        non_static_routes,
        failing_budget_routes,
        findings,
    })
}

fn forge_package_scorecard_release_ready(report: &DxForgePackageScorecardReport) -> bool {
    report.score >= 95
        && !report.packages.is_empty()
        && report.packages.iter().all(|package| {
            package.integrity_verified
                && package.source_owned
                && package.install_scripts_blocked
                && !package.node_modules_created
        })
}

fn forge_benchmark_snapshot_is_release_ready(snapshot: &DxForgeBenchmarkSnapshot) -> bool {
    if snapshot.route_delivery.as_deref() == Some("static") {
        return snapshot.decoded_bytes.is_some()
            && snapshot.brotli_bytes.is_some()
            && snapshot.http_route_median_ms.is_some();
    }

    snapshot.dx_packet_applied == Some(true) && snapshot.interaction_works == Some(true)
}

fn forge_release_gate_score(check: &DxCheckReport, launch_gate_findings: &[DxCheckFinding]) -> u8 {
    let launch_gate_score = check_score_from_cli_findings(launch_gate_findings);
    if launch_gate_findings.is_empty() {
        return launch_gate_score;
    }

    let required_section_score = ["project", "packages", "security", "maintainability"]
        .into_iter()
        .filter_map(|name| {
            check
                .sections
                .iter()
                .find(|section| section.name == name)
                .map(|section| section.score)
        })
        .min()
        .unwrap_or(check.score);

    launch_gate_score.min(required_section_score)
}

fn check_score_from_cli_findings(findings: &[DxCheckFinding]) -> u8 {
    let mut score = 100i32;
    for finding in findings {
        score -= match finding.severity {
            DxSupplyChainSeverity::Critical => 60,
            DxSupplyChainSeverity::High => 40,
            DxSupplyChainSeverity::Medium => 15,
            DxSupplyChainSeverity::Low => 5,
            DxSupplyChainSeverity::Info => 0,
        };
    }
    score.clamp(0, 100) as u8
}

fn build_forge_smoke_report(project: &Path) -> anyhow::Result<DxForgeSmokeReport> {
    std::fs::create_dir_all(project)?;
    let mut packages = Vec::new();

    for package_id in FORGE_WWW_TEMPLATE_PACKAGE_IDS {
        let outcome = write_forge_add_variant(package_id, "default", project)?;
        packages.push(DxForgeSmokePackage {
            package_id: outcome.receipt.package.package_id,
            variant: outcome.receipt.package.variant,
            files_written: outcome.receipt.files_written.len() as u64,
            risk_score: outcome.receipt.risk_score,
            receipt_path: outcome.receipt_path,
        });
    }

    let launch_artifacts = write_forge_launch_smoke_artifacts(project)?;
    let check = check_dx_project(project)?;
    let launch_gate_findings = forge_launch_gate_findings(&check);
    let release_gate_score = forge_release_gate_score(&check, &launch_gate_findings);
    let doctor = build_forge_doctor_report(project)?;
    let verify = build_forge_verify_all_packages_report(project)?;
    let scorecard = build_forge_package_scorecard_for_project(project)?;
    let launch_page_quality = check_forge_launch_page_quality(&launch_artifacts)?;
    let scorecard_ready = forge_package_scorecard_release_ready(&scorecard);
    let no_node_modules = !project.join("node_modules").exists();
    let launch_artifacts_present = forge_smoke_artifact_paths(&launch_artifacts)
        .into_iter()
        .all(|(_, path)| path.exists());
    let evidence_passed = forge_smoke_json_bool(&launch_artifacts.evidence_report_path, "passed")?;
    let mut findings = Vec::new();

    if !no_node_modules {
        findings.push("node_modules was created during Forge smoke.".to_string());
    }
    if !launch_artifacts_present {
        findings.push("One or more Forge launch smoke artifacts were not written.".to_string());
    }
    if !evidence_passed {
        findings.push("dx forge evidence did not produce passing launch evidence.".to_string());
    }
    if !launch_gate_findings.is_empty() {
        findings.push(format!(
            "{} strict Forge launch-gate finding(s) remain.",
            launch_gate_findings.len()
        ));
    }
    if !doctor.passed {
        findings.push("dx forge doctor did not pass.".to_string());
    }
    if !verify.passed {
        findings.push("dx forge verify-package --all did not pass.".to_string());
    }
    if !scorecard_ready {
        findings.push("Forge package scorecard is not release-ready.".to_string());
    }
    if !launch_page_quality.passed {
        findings.push(format!(
            "Forge launch page quality failed: {}",
            launch_page_quality.findings.join("; ")
        ));
    }

    let score = [
        release_gate_score,
        verify.score,
        scorecard.score,
        launch_page_quality.score,
        if launch_artifacts_present { 100 } else { 0 },
        if evidence_passed { 100 } else { 0 },
        if no_node_modules { 100 } else { 0 },
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let passed = findings.is_empty() && score >= 90;

    Ok(DxForgeSmokeReport {
        project: project.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        no_node_modules,
        packages,
        check_score: check.score,
        check_traffic: check.traffic.as_str().to_string(),
        release_gate_score,
        doctor_passed: doctor.passed,
        doctor_score: doctor.check.score,
        verify_passed: verify.passed,
        verify_score: verify.score,
        scorecard_score: scorecard.score,
        launch_artifacts,
        launch_page_quality,
        launch_gate_findings,
        findings,
    })
}

fn build_forge_init_app_report(
    project: &Path,
    write: bool,
) -> anyhow::Result<DxForgeInitAppReport> {
    let generated_at = Utc::now().to_rfc3339();
    let mut planned_files = forge_init_app_scaffold_file_list();
    planned_files.extend(forge_init_app_artifact_file_list());
    let mut packages = Vec::new();
    let mut findings = Vec::new();
    let mut scaffolded_files = Vec::new();
    let mut check_score = None;
    let mut check_traffic = None;
    let mut launch_gate_findings = Vec::new();
    let mut scorecard_report_path = None;
    let mut dx_check_report_path = None;
    let mut scorecard_score = None;

    if write {
        std::fs::create_dir_all(project)?;
        scaffolded_files = write_forge_adoption_project_scaffold(project)?;
    }

    for package_id in FORGE_WWW_TEMPLATE_PACKAGE_IDS {
        let outcome = if write {
            write_forge_add_variant(package_id, "default", project)?
        } else {
            plan_forge_add_variant(package_id, "default", project)?
        };

        planned_files.extend(
            outcome
                .receipt
                .files_written
                .iter()
                .map(|file| file.path.clone()),
        );
        packages.push(forge_init_app_package_report(&outcome));
    }

    planned_files.sort();
    planned_files.dedup();

    if write {
        let scorecard = build_forge_package_scorecard_for_project(project)?;
        let check = check_dx_project_with_options(
            project,
            DxCheckOptions {
                project_contract: true,
            },
        )?;
        let gate_findings = forge_launch_gate_findings(&check);
        let artifacts_dir = project.join(".dx/forge/init-app");
        let scorecard_json_path = artifacts_dir.join("forge-scorecard.json");
        let scorecard_markdown_path = artifacts_dir.join("forge-scorecard.md");
        let check_json_path = artifacts_dir.join("dx-check.json");
        let check_markdown_path = artifacts_dir.join("dx-check.md");

        std::fs::create_dir_all(&artifacts_dir)?;
        std::fs::write(
            &scorecard_json_path,
            serde_json::to_string_pretty(&scorecard)?,
        )?;
        std::fs::write(
            &scorecard_markdown_path,
            forge_package_scorecard_markdown(&scorecard),
        )?;
        std::fs::write(&check_json_path, serde_json::to_string_pretty(&check)?)?;
        std::fs::write(&check_markdown_path, dx_check_report_markdown(&check))?;

        scorecard_score = Some(scorecard.score);
        scorecard_report_path = Some(scorecard_json_path);
        dx_check_report_path = Some(check_json_path);
        check_score = Some(check.score);
        check_traffic = Some(check.traffic.as_str().to_string());
        launch_gate_findings = gate_findings;
    }

    let no_node_modules = !project.join("node_modules").exists();
    let package_score = packages
        .iter()
        .map(|package| package.risk_score)
        .min()
        .unwrap_or(0);
    let package_gate_score = if package_score >= 85 {
        100
    } else {
        package_score
    };
    if !no_node_modules {
        findings.push("node_modules exists after Forge init-app".to_string());
    }
    if package_score < 85 {
        findings.push(format!(
            "minimum launch package score is {package_score}, below warning-tolerant beta threshold 85"
        ));
    }
    if let Some(score) = check_score {
        if score < 90 {
            findings.push(format!(
                "dx check score is {score}, below beta threshold 90"
            ));
        }
    }
    if let Some(score) = scorecard_score {
        if score < 90 {
            findings.push(format!(
                "Forge package scorecard is {score}, below beta threshold 90"
            ));
        }
    }
    if !launch_gate_findings.is_empty() {
        findings.push(format!(
            "{} strict Forge launch-gate finding(s) remain",
            launch_gate_findings.len()
        ));
    }

    let mut score_inputs = vec![package_gate_score, if no_node_modules { 100 } else { 0 }];
    if let Some(score) = check_score {
        score_inputs.push(score);
    }
    if let Some(score) = scorecard_score {
        score_inputs.push(score);
    }
    let score = score_inputs.into_iter().min().unwrap_or(0);
    let passed = findings.is_empty() && score >= 90;

    Ok(DxForgeInitAppReport {
        version: 1,
        project: project.to_path_buf(),
        generated_at,
        mode: if write { "write" } else { "dry-run" }.to_string(),
        passed,
        score,
        no_node_modules,
        package_ids: FORGE_WWW_TEMPLATE_PACKAGE_IDS
            .iter()
            .map(|package_id| (*package_id).to_string())
            .collect(),
        planned_files,
        scaffolded_files,
        packages,
        source_manifest_path: project.join(".dx/forge/source-manifest.json"),
        scorecard_report_path,
        dx_check_report_path,
        check_score,
        check_traffic,
        strict_forge_passed: launch_gate_findings.is_empty(),
        launch_gate_findings,
        findings,
        next_commands: vec![
            "dx check . --strict-project-contract --fail-under 90".to_string(),
            "dx check . --strict-forge --fail-under 90".to_string(),
            "dx forge ci --project . --format markdown".to_string(),
            "dx forge adoption-report --project . --format markdown".to_string(),
        ],
    })
}

fn forge_init_app_scaffold_file_list() -> Vec<String> {
    [
        "dx",
        "README.md",
        "app/layout.tsx",
        "app/page.tsx",
        "components/local/WelcomeCard.tsx",
        "server/actions.ts",
        "styles/tokens.css",
        "styles/global.css",
        "pages/index.html",
        "pages/forge-adoption.html",
        ".dx/serializer/dx.machine",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn forge_init_app_artifact_file_list() -> Vec<String> {
    [
        ".dx/forge/source-manifest.json",
        ".dx/forge/init-app/forge-scorecard.json",
        ".dx/forge/init-app/forge-scorecard.md",
        ".dx/forge/init-app/dx-check.json",
        ".dx/forge/init-app/dx-check.md",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn forge_init_app_package_report(outcome: &DxForgeAddOutcome) -> DxForgeInitAppPackage {
    DxForgeInitAppPackage {
        package_id: outcome.receipt.package.package_id.clone(),
        variant: outcome.receipt.package.variant.clone(),
        risk_score: outcome.receipt.risk_score,
        files: outcome.receipt.files_written.len() as u64,
        wrote_files: outcome.wrote_files,
        manifest_path: outcome.manifest_path.clone(),
        receipt_path: outcome.receipt_path.clone(),
    }
}

fn build_forge_adoption_smoke_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeAdoptionSmokeReport> {
    std::fs::create_dir_all(project)?;
    let scaffolded_files = write_forge_adoption_project_scaffold(project)?;
    let adoption_dir = project.join(".dx/forge/adoption-smoke");
    std::fs::create_dir_all(&adoption_dir)?;
    let route_comparison_path = write_forge_adoption_route_comparison(project)?;
    let release_history_path =
        write_forge_adoption_release_history(project, &route_comparison_path)?;
    let release_bundle_dir = adoption_dir.join("release-bundle");
    let release_bundle =
        build_forge_release_bundle(project, &release_bundle_dir, fail_under, false)?;
    let smoke_report_path = adoption_dir.join("forge-smoke.json");
    std::fs::copy(
        release_bundle_dir.join("forge-smoke.json"),
        &smoke_report_path,
    )?;
    let smoke_json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&smoke_report_path)?)?;
    let smoke_score = smoke_json
        .get("score")
        .and_then(|value| value.as_u64())
        .unwrap_or_default()
        .min(100) as u8;
    let smoke_passed = smoke_json
        .get("passed")
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
    let package_count = smoke_json
        .get("packages")
        .and_then(|value| value.as_array())
        .map(Vec::len)
        .unwrap_or_default();
    let public_dir = project.join("public");
    let routes = copy_forge_release_bundle_routes_to_public(&release_bundle_dir, &public_dir)?;
    let no_node_modules =
        !project.join("node_modules").exists() && !release_bundle_dir.join("node_modules").exists();

    let mut findings = Vec::new();
    if !smoke_passed {
        findings.push("base Forge smoke did not pass".to_string());
    }
    if !release_bundle.passed {
        findings.push("release bundle verification did not pass".to_string());
    }
    if !no_node_modules {
        findings.push("node_modules was created during adoption smoke".to_string());
    }
    for route in routes.iter().filter(|route| !route.exists) {
        findings.push(format!("public route artifact is missing: {}", route.route));
    }

    let route_score = if routes.iter().all(|route| route.exists)
        && routes.len() == FORGE_REQUIRED_PUBLIC_ROUTES.len()
    {
        100
    } else {
        70
    };
    let score = [
        smoke_score,
        release_bundle.score,
        route_score,
        if no_node_modules { 100 } else { 0 },
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(DxForgeAdoptionSmokeReport {
        version: 1,
        project: project.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        fail_under,
        no_node_modules,
        package_count,
        route_count: routes.len(),
        scaffolded_files,
        routes,
        smoke_report_path,
        adoption_artifacts_dir: adoption_dir.clone(),
        release_bundle_dir: release_bundle_dir.clone(),
        release_bundle_manifest_path: release_bundle_dir.join(FORGE_RELEASE_BUNDLE_MANIFEST_JSON),
        public_dir,
        source_manifest_path: project.join(".dx/forge/source-manifest.json"),
        route_comparison_path,
        release_history_path,
        smoke_score,
        release_bundle_score: release_bundle.score,
        findings,
    })
}

fn build_forge_adoption_report(
    project: &Path,
    release_bundle: Option<PathBuf>,
    fail_under: u8,
) -> anyhow::Result<DxForgeAdoptionReport> {
    let source_manifest_path = project.join(".dx/forge/source-manifest.json");
    let receipt_dir = project.join(".dx/forge/receipts");
    let package_docs_dir = project.join(".dx/forge/docs");
    let public_dir = project.join("public");
    let mut findings = Vec::new();
    let manifest = read_adoption_source_manifest(&source_manifest_path, &mut findings)?;
    let packages = adoption_package_evidence(&manifest, &package_docs_dir);
    let package_count = manifest.packages.len();
    let receipt_count = count_regular_files(&receipt_dir);
    let package_docs_present = packages
        .iter()
        .filter(|package| package.docs_exists)
        .count();
    let package_docs_missing = package_count.saturating_sub(package_docs_present);
    let project_structure = adoption_project_structure(project);
    let public_routes = adoption_public_route_artifacts(&public_dir);
    let release_bundle_dir =
        release_bundle.unwrap_or_else(|| project.join(".dx/forge/adoption-smoke/release-bundle"));
    let release_bundle = adoption_release_bundle_evidence(&release_bundle_dir);
    let dx_check = check_dx_project(project)?;
    let strict_findings = forge_launch_gate_findings(&dx_check);
    let release_gate_score = forge_release_gate_score(&dx_check, &strict_findings);
    let dx_check =
        adoption_check_evidence(&dx_check, strict_findings.is_empty(), release_gate_score);
    let no_node_modules =
        !project.join("node_modules").exists() && !release_bundle_dir.join("node_modules").exists();

    if package_count == 0 {
        findings.push("Forge source manifest has no packages".to_string());
    }
    if receipt_count == 0 {
        findings.push("No Forge receipts were found".to_string());
    }
    if package_docs_missing > 0 {
        findings.push(format!(
            "{package_docs_missing} source-owned package(s) are missing Forge docs"
        ));
    }
    if !project_structure.app_route_exists {
        findings.push(format!(
            "Adoption app route is missing: {}",
            project_structure.app_route_path.display()
        ));
    }
    for route in public_routes.iter().filter(|route| !route.passed) {
        findings.push(format!(
            "Public route artifact set is incomplete for {}",
            route.route
        ));
    }
    if !release_bundle.exists {
        findings.push(format!(
            "Release bundle is missing: {}",
            release_bundle.bundle_dir.display()
        ));
    } else if !release_bundle.passed {
        findings.push(format!(
            "Release bundle verification did not pass: {}",
            release_bundle.findings.join("; ")
        ));
    }
    if !dx_check.strict_forge_passed {
        findings.push(format!(
            "Strict Forge gate did not pass for dx check score {}",
            dx_check.score
        ));
    }
    for finding in strict_findings {
        findings.push(format!("{}: {}", finding.code, finding.message));
    }
    if !no_node_modules {
        findings
            .push("node_modules was found in the adoption project or release bundle".to_string());
    }

    let docs_score = adoption_percent_score(package_docs_present, package_count);
    let route_score = adoption_percent_score(
        public_routes.iter().filter(|route| route.passed).count(),
        public_routes.len(),
    );
    let receipt_score = if receipt_count >= package_count && package_count > 0 {
        100
    } else if receipt_count > 0 {
        80
    } else {
        0
    };
    let app_route_score = if project_structure.app_route_exists {
        100
    } else {
        70
    };
    let score = [
        dx_check.release_gate_score,
        release_bundle.score,
        docs_score,
        route_score,
        receipt_score,
        app_route_score,
        if no_node_modules { 100 } else { 0 },
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(DxForgeAdoptionReport {
        version: 1,
        project: project.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        fail_under,
        no_node_modules,
        package_count,
        packages,
        receipt_count,
        package_docs_present,
        package_docs_missing,
        project_structure,
        source_manifest_path,
        receipt_dir,
        package_docs_dir,
        public_dir,
        public_routes,
        release_bundle,
        dx_check,
        findings,
        honest_scope: vec![
            "This report summarizes local Forge adoption evidence for one existing project path."
                .to_string(),
            "It proves source-owned package state, receipts, docs, route artifacts, and no direct node_modules creation for the checked path.".to_string(),
            "It is not a universal npm replacement claim and not a full framework benchmark.".to_string(),
        ],
    })
}

fn build_forge_beta_install_report(
    project: &Path,
    release_bundle_dir: &Path,
    artifact_dir: &Path,
    write: bool,
    fail_under: u8,
) -> anyhow::Result<DxForgeBetaInstallReport> {
    let generated_at = Utc::now().to_rfc3339();
    let clean_project = forge_beta_install_target_is_clean(project)?;
    let release_bundle_report = verify_forge_release_bundle_with_options(release_bundle_dir, true)?;
    let release_manifest = forge_beta_install_manifest_check(release_bundle_dir)?;
    let release_manifest_status = forge_beta_install_manifest_status(release_bundle_dir)
        .unwrap_or_else(|| "unknown".to_string());
    let init_app_report = build_forge_init_app_report(project, write)?;
    let include_adoption = forge_release_bundle_includes_adoption(release_bundle_dir);
    let mut routes = Vec::new();
    let mut artifacts = Vec::new();
    let mut script_path = None;
    let mut findings = Vec::new();

    if write {
        std::fs::create_dir_all(artifact_dir)?;
        let script = write_forge_beta_install_script(project, release_bundle_dir, artifact_dir)?;
        script_path = Some(script.clone());
        artifacts.push(script);

        artifacts.push(write_forge_beta_install_json_artifact(
            &artifact_dir.join("forge-release-bundle.json"),
            &release_bundle_report,
        )?);
        artifacts.push(write_forge_beta_install_text_artifact(
            &artifact_dir.join("forge-release-bundle.md"),
            &forge_release_bundle_markdown(&release_bundle_report),
        )?);
        artifacts.push(write_forge_beta_install_json_artifact(
            &artifact_dir.join("forge-init-app.json"),
            &init_app_report,
        )?);
        artifacts.push(write_forge_beta_install_text_artifact(
            &artifact_dir.join("forge-init-app.md"),
            &forge_init_app_markdown(&init_app_report),
        )?);

        routes = copy_forge_release_bundle_routes_to_public_with_options(
            release_bundle_dir,
            &project.join("public"),
            include_adoption,
        )?;
    }

    let release_bundle = forge_beta_install_check(
        "release_bundle",
        release_bundle_report.passed,
        release_bundle_report.score,
        format!(
            "Release bundle verified with {} artifacts and {} route(s).",
            release_bundle_report.artifact_count, release_bundle_report.route_count
        ),
        Some(release_bundle_dir.display().to_string()),
    );
    let init_app = forge_beta_install_check(
        "init_app",
        init_app_report.passed,
        init_app_report.score,
        if write {
            format!(
                "Initialized {} source-owned launch package(s) and wrote review artifacts.",
                init_app_report.package_ids.len()
            )
        } else {
            format!(
                "Planned {} source-owned launch package(s); pass --write to bootstrap the clean project.",
                init_app_report.package_ids.len()
            )
        },
        Some(init_app_report.source_manifest_path.display().to_string()),
    );

    let mut provenance = forge_beta_install_check(
        "provenance",
        !write,
        if write { 0 } else { 90 },
        if write {
            "Provenance gate has not run yet.".to_string()
        } else {
            "Provenance gate is planned; pass --write to materialize and verify it.".to_string()
        },
        Some(
            project
                .join(".dx/forge/source-manifest.json")
                .display()
                .to_string(),
        ),
    );
    let mut trust_regression = forge_beta_install_check(
        "trust_regression",
        !write,
        if write { 0 } else { 90 },
        if write {
            "Trust-regression gate has not run yet.".to_string()
        } else {
            "Trust-regression gate is planned; pass --write to run fixture mutations.".to_string()
        },
        Some(
            project
                .join(".dx/forge/trust-regression-fixtures")
                .display()
                .to_string(),
        ),
    );
    let mut adoption_report = forge_beta_install_check(
        "adoption_report",
        !write,
        if write { 0 } else { 90 },
        if write {
            "Adoption report has not run yet.".to_string()
        } else {
            "Adoption report is planned; pass --write to copy public evidence routes.".to_string()
        },
        Some(
            artifact_dir
                .join("forge-adoption-report.json")
                .display()
                .to_string(),
        ),
    );

    if write {
        let provenance_report = build_forge_provenance_report(project, fail_under)?;
        provenance = forge_beta_install_check(
            "provenance",
            provenance_report.passed,
            provenance_report.score,
            format!(
                "Verified provenance for {} package(s), {} receipt hash(es), and rollback coverage {}/{}.",
                provenance_report.package_count,
                provenance_report.receipt_hash_count,
                provenance_report.rollback_covered_package_count,
                provenance_report.rollback_required_package_count
            ),
            Some(provenance_report.source_manifest_path.display().to_string()),
        );
        artifacts.push(write_forge_beta_install_json_artifact(
            &artifact_dir.join("forge-provenance.json"),
            &provenance_report,
        )?);
        artifacts.push(write_forge_beta_install_text_artifact(
            &artifact_dir.join("forge-provenance.md"),
            &forge_provenance_markdown(&provenance_report),
        )?);

        let trust_report = build_forge_trust_regression_report(project, 100)?;
        trust_regression = forge_beta_install_check(
            "trust_regression",
            trust_report.passed,
            trust_report.score,
            format!(
                "Ran {} trust-regression case(s) with fixture root `{}`.",
                trust_report.case_count,
                trust_report.fixture_root.display()
            ),
            Some(trust_report.fixture_root.display().to_string()),
        );
        artifacts.push(write_forge_beta_install_json_artifact(
            &artifact_dir.join("forge-trust-regression.json"),
            &trust_report,
        )?);
        artifacts.push(write_forge_beta_install_text_artifact(
            &artifact_dir.join("forge-trust-regression.md"),
            &forge_trust_regression_markdown(&trust_report),
        )?);

        let adoption = build_forge_adoption_report(
            project,
            Some(release_bundle_dir.to_path_buf()),
            fail_under,
        )?;
        adoption_report = forge_beta_install_check(
            "adoption_report",
            adoption.passed,
            adoption.score,
            format!(
                "Verified adoption project with {} package(s), {} receipt(s), and {} public route artifact set(s).",
                adoption.package_count,
                adoption.receipt_count,
                adoption.public_routes.len()
            ),
            Some(project.join("public").display().to_string()),
        );
        artifacts.push(write_forge_beta_install_json_artifact(
            &artifact_dir.join("forge-adoption-report.json"),
            &adoption,
        )?);
        artifacts.push(write_forge_beta_install_text_artifact(
            &artifact_dir.join("forge-adoption-report.md"),
            &forge_adoption_report_markdown(&adoption),
        )?);
    }

    let no_node_modules = !project.join("node_modules").exists()
        && !release_bundle_dir.join("node_modules").exists()
        && !artifact_dir.join("node_modules").exists();
    if write && !clean_project {
        findings.push(format!(
            "Beta install target was not clean before bootstrap: {}",
            project.display()
        ));
    }
    if !no_node_modules {
        findings.push(
            "node_modules exists in the beta install project, release bundle, or artifact directory."
                .to_string(),
        );
    }
    for route in routes.iter().filter(|route| !route.exists) {
        findings.push(format!(
            "release-bundle route copy is incomplete: {}",
            route.route
        ));
    }
    for check in [
        &release_bundle,
        &release_manifest,
        &init_app,
        &provenance,
        &trust_regression,
        &adoption_report,
    ] {
        if !check.passed {
            findings.push(format!("{}: {}", check.name, check.message));
        }
    }

    if write {
        artifacts.push(artifact_dir.join("forge-beta-install.json"));
        artifacts.push(artifact_dir.join("forge-beta-install.md"));
    }

    let score = [
        release_bundle.score,
        release_manifest.score,
        init_app.score,
        provenance.score,
        trust_regression.score,
        adoption_report.score,
        if clean_project || !write { 100 } else { 80 },
        if no_node_modules { 100 } else { 0 },
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let passed = findings.is_empty() && score >= fail_under;

    let report = DxForgeBetaInstallReport {
        version: 1,
        project: project.to_path_buf(),
        release_bundle_dir: release_bundle_dir.to_path_buf(),
        artifact_dir: artifact_dir.to_path_buf(),
        generated_at,
        mode: if write { "write" } else { "dry-run" }.to_string(),
        passed,
        score,
        fail_under,
        clean_project,
        no_node_modules,
        wrote_project: write,
        script_path,
        release_manifest_status,
        release_bundle,
        release_manifest,
        init_app,
        provenance,
        trust_regression,
        adoption_report,
        routes,
        artifacts,
        findings,
        next_commands: vec![
            "dx forge release-bundle --verify <release-bundle> --include-adoption --format markdown".to_string(),
            "dx forge provenance --project <beta-project> --format markdown".to_string(),
            "dx forge trust-regression --project <beta-project> --format markdown --fail-under 100".to_string(),
            "dx forge adoption-report --project <beta-project> --release-bundle <release-bundle> --format markdown".to_string(),
        ],
    };

    if write {
        write_forge_beta_install_json_artifact(
            &artifact_dir.join("forge-beta-install.json"),
            &report,
        )?;
        write_forge_beta_install_text_artifact(
            &artifact_dir.join("forge-beta-install.md"),
            &forge_beta_install_markdown(&report),
        )?;
    }

    Ok(report)
}

fn forge_beta_install_target_is_clean(project: &Path) -> anyhow::Result<bool> {
    if !project.exists() {
        return Ok(true);
    }
    Ok(project.read_dir()?.next().is_none())
}

fn forge_beta_install_check(
    name: impl Into<String>,
    passed: bool,
    score: u8,
    message: impl Into<String>,
    evidence: Option<String>,
) -> DxForgeBetaInstallCheck {
    DxForgeBetaInstallCheck {
        name: name.into(),
        passed,
        score,
        message: message.into(),
        evidence,
    }
}

fn forge_beta_install_manifest_status(release_bundle_dir: &Path) -> Option<String> {
    let manifest_path = release_bundle_dir.join(FORGE_RELEASE_BUNDLE_MANIFEST_JSON);
    let manifest: DxForgeReleaseBundleManifest =
        serde_json::from_slice(&std::fs::read(manifest_path).ok()?).ok()?;
    Some(manifest.publisher_identity.status)
}

fn forge_beta_install_manifest_check(
    release_bundle_dir: &Path,
) -> anyhow::Result<DxForgeBetaInstallCheck> {
    let manifest_path = release_bundle_dir.join(FORGE_RELEASE_BUNDLE_MANIFEST_JSON);
    let manifest: DxForgeReleaseBundleManifest =
        serde_json::from_slice(&std::fs::read(&manifest_path)?)?;
    let computed_digest = forge_release_bundle_manifest_digest(&manifest.artifacts)?;
    let mut issues = Vec::new();

    if computed_digest != manifest.integrity.digest {
        issues.push(format!(
            "manifest digest mismatch: expected {}, computed {}",
            manifest.integrity.digest, computed_digest
        ));
    }
    if computed_digest != manifest.artifact_integrity.digest {
        issues.push(format!(
            "artifact integrity digest mismatch: expected {}, computed {}",
            manifest.artifact_integrity.digest, computed_digest
        ));
    }
    if !manifest.artifact_integrity.verified_locally {
        issues.push("artifact integrity was not marked locally verified".to_string());
    }

    let mut signature_verified = false;
    match manifest.publisher_identity.status.as_str() {
        FORGE_RELEASE_BUNDLE_SIGNATURE_STATUS_SIGNED => {
            if let Err(error) = verify_forge_release_bundle_manifest_signature(&manifest) {
                issues.push(error);
            } else {
                signature_verified = true;
            }
        }
        FORGE_RELEASE_BUNDLE_SIGNATURE_STATUS_UNSIGNED => {}
        other => issues.push(format!("unknown publisher identity status `{other}`")),
    }

    let passed = issues.is_empty();
    let score = if !passed {
        0
    } else if signature_verified {
        100
    } else {
        95
    };
    let message = if passed && signature_verified {
        "Signed release manifest publisher identity verified with local artifact integrity."
            .to_string()
    } else if passed {
        "Unsigned release manifest passed local BLAKE3 artifact integrity; signed manifests are verified when publisher identity is attached."
            .to_string()
    } else {
        issues.join("; ")
    };

    Ok(forge_beta_install_check(
        "release_manifest",
        passed,
        score,
        message,
        Some(manifest_path.display().to_string()),
    ))
}

fn write_forge_beta_install_json_artifact<T: Serialize>(
    path: &Path,
    value: &T,
) -> anyhow::Result<PathBuf> {
    write_forge_beta_install_text_artifact(path, &serde_json::to_string_pretty(value)?)
}

fn write_forge_beta_install_text_artifact(path: &Path, content: &str) -> anyhow::Result<PathBuf> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)?;
    Ok(path.to_path_buf())
}

fn write_forge_beta_install_script(
    project: &Path,
    release_bundle_dir: &Path,
    artifact_dir: &Path,
) -> anyhow::Result<PathBuf> {
    let script_path = artifact_dir.join("forge-beta-install.ps1");
    let project = project.display();
    let release_bundle = release_bundle_dir.display();
    let artifacts = artifact_dir.display();
    let script = format!(
        r#"# DX Forge beta install adoption script
param(
    [string]$Project = "{project}",
    [string]$ReleaseBundle = "{release_bundle}",
    [string]$Artifacts = "{artifacts}",
    [int]$FailUnder = 90
)

$ErrorActionPreference = "Stop"
New-Item -ItemType Directory -Force -Path $Artifacts | Out-Null

dx forge release-bundle --verify $ReleaseBundle --include-adoption --format markdown --fail-under $FailUnder | Set-Content -Path (Join-Path $Artifacts "forge-release-bundle.md")
dx forge init-app --project $Project --write --format markdown --output (Join-Path $Artifacts "forge-init-app.md") --quiet
dx forge provenance --project $Project --format markdown --output (Join-Path $Artifacts "forge-provenance.md") --fail-under $FailUnder --quiet
dx forge trust-regression --project $Project --format markdown --output (Join-Path $Artifacts "forge-trust-regression.md") --fail-under 100 --quiet
dx forge adoption-report --project $Project --release-bundle $ReleaseBundle --format markdown --output (Join-Path $Artifacts "forge-adoption-report.md") --fail-under $FailUnder --quiet
"#
    );
    write_forge_beta_install_text_artifact(&script_path, &script)
}

fn forge_beta_install_terminal(report: &DxForgeBetaInstallReport) -> String {
    let mut output = format!(
        "DX Forge beta install\nProject: {}\nRelease bundle: {}\nArtifacts: {}\nGenerated: {}\nMode: {}\nPassed: {}\nScore: {} / 100\nRequired score: {} / 100\nClean project: {}\nNo node_modules: {}\nRelease manifest: {}\n",
        report.project.display(),
        report.release_bundle_dir.display(),
        report.artifact_dir.display(),
        report.generated_at,
        report.mode,
        report.passed,
        report.score,
        report.fail_under,
        report.clean_project,
        report.no_node_modules,
        report.release_manifest_status
    );

    output.push_str("\nChecks:\n");
    for check in [
        &report.release_bundle,
        &report.release_manifest,
        &report.init_app,
        &report.provenance,
        &report.trust_regression,
        &report.adoption_report,
    ] {
        output.push_str(&format!(
            "- {}: {} ({} / 100) {}\n",
            check.name, check.passed, check.score, check.message
        ));
    }

    if !report.findings.is_empty() {
        output.push_str("\nFindings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output
}

fn forge_beta_install_markdown(report: &DxForgeBetaInstallReport) -> String {
    let mut output = format!(
        "# DX Forge Beta Install\n\n- Project: `{}`\n- Release bundle: `{}`\n- Artifacts: `{}`\n- Generated: `{}`\n- Mode: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Clean project: `{}`\n- no `node_modules`: `{}`\n- Release manifest: `{}`\n\n",
        report.project.display(),
        report.release_bundle_dir.display(),
        report.artifact_dir.display(),
        report.generated_at,
        report.mode,
        report.passed,
        report.score,
        report.fail_under,
        report.clean_project,
        report.no_node_modules,
        report.release_manifest_status
    );

    output.push_str("## Checks\n\n");
    output.push_str("| Check | Passed | Score | Evidence | Message |\n");
    output.push_str("| --- | --- | ---: | --- | --- |\n");
    for check in [
        &report.release_bundle,
        &report.release_manifest,
        &report.init_app,
        &report.provenance,
        &report.trust_regression,
        &report.adoption_report,
    ] {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | `{}` | {} |\n",
            check.name,
            check.passed,
            check.score,
            markdown_table_cell(check.evidence.as_deref().unwrap_or("-")),
            markdown_table_cell(&check.message)
        ));
    }

    output.push_str("\n## Public Routes\n\n");
    if report.routes.is_empty() {
        output.push_str("- `dry-run`: route artifacts are copied during `--write`.\n");
    } else {
        for route in &report.routes {
            output.push_str(&format!(
                "- `{}`: exists `{}` via `{}`\n",
                route.route,
                route.exists,
                route.html_path.display()
            ));
        }
    }

    output.push_str("\n## Artifacts\n\n");
    if report.artifacts.is_empty() {
        output.push_str("- `dry-run`: no artifact trail was written.\n");
    } else {
        for artifact in &report.artifacts {
            output.push_str(&format!("- `{}`\n", artifact.display()));
        }
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str(
            "- `pass`: release bundle, manifest, provenance, trust-regression, and adoption evidence are coherent.\n",
        );
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
    }

    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{command}`\n"));
    }

    output
}

fn forge_beta_install_failure_summary(report: &DxForgeBetaInstallReport) -> String {
    if report.findings.is_empty() {
        return format!(
            "DX Forge beta-install score {} is below fail-under threshold {}",
            report.score, report.fail_under
        );
    }
    format!(
        "DX Forge beta-install failed: {}",
        report.findings.join("; ")
    )
}

fn build_forge_beta_upgrade_smoke_report(
    project: &Path,
    from_release_bundle_dir: &Path,
    to_release_bundle_dir: &Path,
    artifact_dir: &Path,
    write: bool,
    fail_under: u8,
) -> anyhow::Result<DxForgeBetaUpgradeSmokeReport> {
    let generated_at = Utc::now().to_rfc3339();
    let from_install_artifact_dir = artifact_dir.join("from-install");
    let initial_install_report = build_forge_beta_install_report(
        project,
        from_release_bundle_dir,
        &from_install_artifact_dir,
        write,
        fail_under,
    )?;
    let from_release_manifest = initial_install_report.release_manifest.clone();
    let initial_install = forge_beta_install_check(
        "initial_install",
        initial_install_report.passed,
        initial_install_report.score,
        if write {
            "Installed the beta app from the first signed release bundle.".to_string()
        } else {
            "Planned the first signed release-bundle install; pass --write to run the upgrade smoke."
                .to_string()
        },
        if write {
            Some(
                from_install_artifact_dir
                    .join("forge-beta-install.json")
                    .display()
                    .to_string(),
            )
        } else {
            Some(from_release_bundle_dir.display().to_string())
        },
    );
    let to_release_bundle_report =
        verify_forge_release_bundle_with_options(to_release_bundle_dir, true)?;
    let to_release_manifest = forge_beta_install_manifest_check(to_release_bundle_dir)?;
    let to_release_bundle = forge_beta_install_check(
        "to_release_bundle",
        to_release_bundle_report.passed,
        to_release_bundle_report.score,
        format!(
            "Next release bundle verified with {} artifacts and {} route(s).",
            to_release_bundle_report.artifact_count, to_release_bundle_report.route_count
        ),
        Some(to_release_bundle_dir.display().to_string()),
    );

    let mut artifacts = Vec::new();
    let mut routes = Vec::new();
    let mut findings = Vec::new();
    let mut local_edit = DxForgeBetaUpgradeLocalEdit {
        package_id: FORGE_BETA_UPGRADE_LOCAL_EDIT_PACKAGE_ID.to_string(),
        path: FORGE_BETA_UPGRADE_LOCAL_EDIT_PATH.to_string(),
        marker: FORGE_BETA_UPGRADE_LOCAL_EDIT_MARKER.to_string(),
        preserved: false,
        reviewed_update_traffic: "planned".to_string(),
        receipt_path: None,
    };
    let mut reviewed_update = forge_beta_install_check(
        "reviewed_update",
        !write,
        if write { 0 } else { 90 },
        if write {
            "Reviewed source-owned update has not run yet.".to_string()
        } else {
            "Synthetic local-edit review is planned; pass --write to verify the installed app upgrade."
                .to_string()
        },
        Some(
            project
                .join(FORGE_BETA_UPGRADE_LOCAL_EDIT_PATH)
                .display()
                .to_string(),
        ),
    );
    let mut provenance = forge_beta_install_check(
        "provenance",
        !write,
        if write { 0 } else { 90 },
        if write {
            "Provenance gate has not run after upgrade yet.".to_string()
        } else {
            "Provenance gate is planned after the reviewed local edit.".to_string()
        },
        Some(
            project
                .join(".dx/forge/source-manifest.json")
                .display()
                .to_string(),
        ),
    );
    let mut trust_regression = forge_beta_install_check(
        "trust_regression",
        !write,
        if write { 0 } else { 90 },
        if write {
            "Trust-regression gate has not run after upgrade yet.".to_string()
        } else {
            "Trust-regression gate is planned after the release-bundle upgrade.".to_string()
        },
        Some(
            project
                .join(".dx/forge/trust-regression-fixtures")
                .display()
                .to_string(),
        ),
    );
    let mut adoption_report = forge_beta_install_check(
        "adoption_report",
        !write,
        if write { 0 } else { 90 },
        if write {
            "Adoption report has not run after upgrade yet.".to_string()
        } else {
            "Adoption report is planned against the next signed release bundle.".to_string()
        },
        Some(project.join("public").display().to_string()),
    );

    if write {
        std::fs::create_dir_all(artifact_dir)?;
        artifacts.push(write_forge_beta_upgrade_smoke_script(
            project,
            from_release_bundle_dir,
            to_release_bundle_dir,
            artifact_dir,
        )?);
        artifacts.push(write_forge_beta_install_json_artifact(
            &artifact_dir.join("forge-release-bundle-next.json"),
            &to_release_bundle_report,
        )?);
        artifacts.push(write_forge_beta_install_text_artifact(
            &artifact_dir.join("forge-release-bundle-next.md"),
            &forge_release_bundle_markdown(&to_release_bundle_report),
        )?);

        write_forge_beta_upgrade_smoke_local_edit(project)?;
        let update = write_forge_update_reviewed_variant(
            FORGE_BETA_UPGRADE_LOCAL_EDIT_PACKAGE_ID,
            "default",
            project,
            DxForgeUpdateApproval {
                reviewer: "dx-forge-beta-upgrade-smoke".to_string(),
                note: "Synthetic upgrade smoke reviewed the controlled local edit before moving to the next signed release bundle.".to_string(),
            },
        )?;
        local_edit.reviewed_update_traffic = update.traffic.as_str().to_string();
        local_edit.receipt_path = update.receipt_path.clone();
        local_edit.preserved = forge_beta_upgrade_local_edit_preserved(project)?;
        reviewed_update = forge_beta_install_check(
            "reviewed_update",
            local_edit.preserved && update.traffic == DxUpdateTraffic::Yellow,
            if local_edit.preserved && update.traffic == DxUpdateTraffic::Yellow {
                100
            } else {
                update.risk_score
            },
            format!(
                "Accepted a reviewed yellow local edit through Forge update without overwriting `{}`.",
                FORGE_BETA_UPGRADE_LOCAL_EDIT_PATH
            ),
            update
                .receipt_path
                .as_ref()
                .map(|path| path.display().to_string()),
        );

        let include_adoption = forge_release_bundle_includes_adoption(to_release_bundle_dir);
        routes = copy_forge_release_bundle_routes_to_public_with_options(
            to_release_bundle_dir,
            &project.join("public"),
            include_adoption,
        )?;

        let provenance_report = build_forge_provenance_report(project, fail_under)?;
        provenance = forge_beta_install_check(
            "provenance",
            provenance_report.passed,
            provenance_report.score,
            format!(
                "Verified upgraded provenance for {} package(s) and {} receipt hash(es).",
                provenance_report.package_count, provenance_report.receipt_hash_count
            ),
            Some(provenance_report.source_manifest_path.display().to_string()),
        );
        artifacts.push(write_forge_beta_install_json_artifact(
            &artifact_dir.join("forge-provenance.json"),
            &provenance_report,
        )?);
        artifacts.push(write_forge_beta_install_text_artifact(
            &artifact_dir.join("forge-provenance.md"),
            &forge_provenance_markdown(&provenance_report),
        )?);

        let trust_report = build_forge_trust_regression_report(project, 100)?;
        trust_regression = forge_beta_install_check(
            "trust_regression",
            trust_report.passed,
            trust_report.score,
            format!(
                "Ran {} trust-regression case(s) after the upgrade smoke.",
                trust_report.case_count
            ),
            Some(trust_report.fixture_root.display().to_string()),
        );
        artifacts.push(write_forge_beta_install_json_artifact(
            &artifact_dir.join("forge-trust-regression.json"),
            &trust_report,
        )?);
        artifacts.push(write_forge_beta_install_text_artifact(
            &artifact_dir.join("forge-trust-regression.md"),
            &forge_trust_regression_markdown(&trust_report),
        )?);

        let adoption = build_forge_adoption_report(
            project,
            Some(to_release_bundle_dir.to_path_buf()),
            fail_under,
        )?;
        let adoption_allows_reviewed_local_edit =
            forge_beta_upgrade_adoption_allows_reviewed_local_edit(
                &adoption,
                &reviewed_update,
                local_edit.preserved,
                fail_under,
            );
        let adoption_passed = adoption.passed || adoption_allows_reviewed_local_edit;
        let adoption_message = if adoption.passed {
            format!(
                "Verified upgraded beta app with {} package(s), {} receipt(s), and {} public route artifact set(s).",
                adoption.package_count,
                adoption.receipt_count,
                adoption.public_routes.len()
            )
        } else if adoption_allows_reviewed_local_edit {
            format!(
                "Verified upgraded beta app with a preserved reviewed local edit; adoption score stayed at {} with review-only strict-gate findings.",
                adoption.score
            )
        } else {
            format!(
                "Verified upgraded beta app with {} package(s), {} receipt(s), and {} public route artifact set(s).",
                adoption.package_count,
                adoption.receipt_count,
                adoption.public_routes.len()
            )
        };
        adoption_report = forge_beta_install_check(
            "adoption_report",
            adoption_passed,
            adoption.score,
            adoption_message,
            Some(project.join("public").display().to_string()),
        );
        artifacts.push(write_forge_beta_install_json_artifact(
            &artifact_dir.join("forge-adoption-report.json"),
            &adoption,
        )?);
        artifacts.push(write_forge_beta_install_text_artifact(
            &artifact_dir.join("forge-adoption-report.md"),
            &forge_adoption_report_markdown(&adoption),
        )?);
    }

    let no_node_modules = !project.join("node_modules").exists()
        && !from_release_bundle_dir.join("node_modules").exists()
        && !to_release_bundle_dir.join("node_modules").exists()
        && !artifact_dir.join("node_modules").exists();
    if write && !local_edit.preserved {
        findings.push(format!(
            "Local source-owned edit marker was not preserved in `{}`.",
            FORGE_BETA_UPGRADE_LOCAL_EDIT_PATH
        ));
    }
    if !no_node_modules {
        findings.push(
            "node_modules exists in the beta app, release bundles, or smoke artifact directory."
                .to_string(),
        );
    }
    for route in routes.iter().filter(|route| !route.exists) {
        findings.push(format!(
            "next release-bundle route copy is incomplete: {}",
            route.route
        ));
    }
    for check in [
        &from_release_manifest,
        &to_release_bundle,
        &to_release_manifest,
        &initial_install,
        &reviewed_update,
        &provenance,
        &trust_regression,
        &adoption_report,
    ] {
        if !check.passed {
            findings.push(format!("{}: {}", check.name, check.message));
        }
    }

    if write {
        artifacts.push(artifact_dir.join("forge-beta-upgrade-smoke.json"));
        artifacts.push(artifact_dir.join("forge-beta-upgrade-smoke.md"));
    }

    let local_edit_score = if !write || local_edit.preserved {
        100
    } else {
        0
    };
    let score = [
        from_release_manifest.score,
        to_release_bundle.score,
        to_release_manifest.score,
        initial_install.score,
        reviewed_update.score,
        provenance.score,
        trust_regression.score,
        adoption_report.score,
        local_edit_score,
        if no_node_modules { 100 } else { 0 },
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let passed = findings.is_empty() && score >= fail_under;

    let report = DxForgeBetaUpgradeSmokeReport {
        version: 1,
        project: project.to_path_buf(),
        from_release_bundle_dir: from_release_bundle_dir.to_path_buf(),
        to_release_bundle_dir: to_release_bundle_dir.to_path_buf(),
        artifact_dir: artifact_dir.to_path_buf(),
        generated_at,
        mode: if write { "write" } else { "dry-run" }.to_string(),
        passed,
        score,
        fail_under,
        no_node_modules,
        from_release_manifest,
        to_release_bundle,
        to_release_manifest,
        initial_install,
        reviewed_update,
        provenance,
        trust_regression,
        adoption_report,
        local_edit,
        routes,
        artifacts,
        findings,
        next_commands: vec![
            "dx forge beta-install --project <beta-project> --release-bundle <from-release-bundle> --write --format markdown".to_string(),
            "dx forge beta-upgrade-smoke --project <beta-project> --from-release-bundle <from-release-bundle> --to-release-bundle <to-release-bundle> --write --format markdown".to_string(),
            "dx forge provenance --project <beta-project> --format markdown".to_string(),
            "dx forge adoption-report --project <beta-project> --release-bundle <to-release-bundle> --format markdown".to_string(),
        ],
    };

    if write {
        write_forge_beta_install_json_artifact(
            &artifact_dir.join("forge-beta-upgrade-smoke.json"),
            &report,
        )?;
        write_forge_beta_install_text_artifact(
            &artifact_dir.join("forge-beta-upgrade-smoke.md"),
            &forge_beta_upgrade_smoke_markdown(&report),
        )?;
    }

    Ok(report)
}

fn write_forge_beta_upgrade_smoke_local_edit(project: &Path) -> anyhow::Result<()> {
    let path = project.join(FORGE_BETA_UPGRADE_LOCAL_EDIT_PATH);
    let mut content = std::fs::read_to_string(&path)?;
    if !content.contains(FORGE_BETA_UPGRADE_LOCAL_EDIT_MARKER) {
        if !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str("// ");
        content.push_str(FORGE_BETA_UPGRADE_LOCAL_EDIT_MARKER);
        content.push('\n');
        std::fs::write(&path, content)?;
    }
    Ok(())
}

fn forge_beta_upgrade_local_edit_preserved(project: &Path) -> anyhow::Result<bool> {
    let path = project.join(FORGE_BETA_UPGRADE_LOCAL_EDIT_PATH);
    Ok(std::fs::read_to_string(path)
        .map(|content| content.contains(FORGE_BETA_UPGRADE_LOCAL_EDIT_MARKER))
        .unwrap_or(false))
}

fn forge_beta_upgrade_adoption_allows_reviewed_local_edit(
    adoption: &DxForgeAdoptionReport,
    reviewed_update: &DxForgeBetaInstallCheck,
    local_edit_preserved: bool,
    fail_under: u8,
) -> bool {
    local_edit_preserved
        && reviewed_update.passed
        && adoption.score >= fail_under
        && !adoption.findings.is_empty()
        && adoption.findings.iter().all(|finding| {
            finding.contains("Strict Forge gate did not pass")
                || finding.contains("forge-launch-gate-stale-receipts")
        })
}

fn write_forge_beta_upgrade_smoke_script(
    project: &Path,
    from_release_bundle_dir: &Path,
    to_release_bundle_dir: &Path,
    artifact_dir: &Path,
) -> anyhow::Result<PathBuf> {
    let script_path = artifact_dir.join("forge-beta-upgrade-smoke.ps1");
    let project = project.display();
    let from_release_bundle = from_release_bundle_dir.display();
    let to_release_bundle = to_release_bundle_dir.display();
    let artifacts = artifact_dir.display();
    let script = format!(
        r#"# DX Forge beta upgrade smoke script
param(
    [string]$Project = "{project}",
    [string]$FromReleaseBundle = "{from_release_bundle}",
    [string]$ToReleaseBundle = "{to_release_bundle}",
    [string]$Artifacts = "{artifacts}",
    [int]$FailUnder = 90
)

$ErrorActionPreference = "Stop"
New-Item -ItemType Directory -Force -Path $Artifacts | Out-Null

dx forge beta-upgrade-smoke --project $Project --from-release-bundle $FromReleaseBundle --to-release-bundle $ToReleaseBundle --artifacts $Artifacts --write --format markdown --output (Join-Path $Artifacts "forge-beta-upgrade-smoke.md") --fail-under $FailUnder --quiet
"#
    );
    write_forge_beta_install_text_artifact(&script_path, &script)
}

fn forge_beta_upgrade_smoke_terminal(report: &DxForgeBetaUpgradeSmokeReport) -> String {
    let mut output = format!(
        "DX Forge beta upgrade smoke\nProject: {}\nFrom release bundle: {}\nTo release bundle: {}\nArtifacts: {}\nGenerated: {}\nMode: {}\nPassed: {}\nScore: {} / 100\nRequired score: {} / 100\nNo node_modules: {}\nLocal edit preserved: {}\n",
        report.project.display(),
        report.from_release_bundle_dir.display(),
        report.to_release_bundle_dir.display(),
        report.artifact_dir.display(),
        report.generated_at,
        report.mode,
        report.passed,
        report.score,
        report.fail_under,
        report.no_node_modules,
        report.local_edit.preserved
    );

    output.push_str("\nChecks:\n");
    for check in forge_beta_upgrade_smoke_checks(report) {
        output.push_str(&format!(
            "- {}: {} ({} / 100) {}\n",
            check.name, check.passed, check.score, check.message
        ));
    }

    if !report.findings.is_empty() {
        output.push_str("\nFindings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output
}

fn forge_beta_upgrade_smoke_markdown(report: &DxForgeBetaUpgradeSmokeReport) -> String {
    let mut output = format!(
        "# DX Forge Beta Upgrade Smoke\n\n- Project: `{}`\n- From release bundle: `{}`\n- To release bundle: `{}`\n- Artifacts: `{}`\n- Generated: `{}`\n- Mode: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- no `node_modules`: `{}`\n\n",
        report.project.display(),
        report.from_release_bundle_dir.display(),
        report.to_release_bundle_dir.display(),
        report.artifact_dir.display(),
        report.generated_at,
        report.mode,
        report.passed,
        report.score,
        report.fail_under,
        report.no_node_modules
    );

    output.push_str("## Local Source-Owned Edit\n\n");
    output.push_str(&format!(
        "- Package: `{}`\n- Path: `{}`\n- Marker: `{}`\n- Preserved: `{}`\n- Reviewed update traffic: `{}`\n- Receipt: `{}`\n\n",
        report.local_edit.package_id,
        report.local_edit.path,
        report.local_edit.marker,
        report.local_edit.preserved,
        report.local_edit.reviewed_update_traffic,
        report
            .local_edit
            .receipt_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "-".to_string())
    ));

    output.push_str("## Checks\n\n");
    output.push_str("| Check | Passed | Score | Evidence | Message |\n");
    output.push_str("| --- | --- | ---: | --- | --- |\n");
    for check in forge_beta_upgrade_smoke_checks(report) {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | `{}` | {} |\n",
            check.name,
            check.passed,
            check.score,
            markdown_table_cell(check.evidence.as_deref().unwrap_or("-")),
            markdown_table_cell(&check.message)
        ));
    }

    output.push_str("\n## Public Routes\n\n");
    if report.routes.is_empty() {
        output.push_str("- `dry-run`: next release routes are copied during `--write`.\n");
    } else {
        for route in &report.routes {
            output.push_str(&format!(
                "- `{}`: exists `{}` via `{}`\n",
                route.route,
                route.exists,
                route.html_path.display()
            ));
        }
    }

    output.push_str("\n## Artifacts\n\n");
    if report.artifacts.is_empty() {
        output.push_str("- `dry-run`: no artifact trail was written.\n");
    } else {
        for artifact in &report.artifacts {
            output.push_str(&format!("- `{}`\n", artifact.display()));
        }
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- `pass`: signed bundles, reviewed local edit, provenance, trust-regression, and adoption evidence are coherent.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
    }

    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{command}`\n"));
    }

    output
}

fn forge_beta_upgrade_smoke_checks(
    report: &DxForgeBetaUpgradeSmokeReport,
) -> [&DxForgeBetaInstallCheck; 8] {
    [
        &report.from_release_manifest,
        &report.to_release_bundle,
        &report.to_release_manifest,
        &report.initial_install,
        &report.reviewed_update,
        &report.provenance,
        &report.trust_regression,
        &report.adoption_report,
    ]
}

fn forge_beta_upgrade_smoke_failure_summary(report: &DxForgeBetaUpgradeSmokeReport) -> String {
    if report.findings.is_empty() {
        return format!(
            "DX Forge beta-upgrade-smoke score {} is below fail-under threshold {}",
            report.score, report.fail_under
        );
    }
    format!(
        "DX Forge beta-upgrade-smoke failed: {}",
        report.findings.join("; ")
    )
}
