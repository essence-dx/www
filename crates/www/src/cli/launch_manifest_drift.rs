use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use chrono::Utc;
use serde::Serialize;
use serde_json::Value;

const REPORT_SCHEMA: &str = "dx.forge.launch_manifest_drift";
const TEMPLATE_ID: &str = "next-familiar-www-template";
const TEMPLATE_MANIFEST_PATH: &str = ".dx/forge/template-.dx/build-cache/manifest.json";
const READINESS_RECEIPT_PATH: &str = ".dx/forge/template-readiness/launch-route.json";
const READINESS_BUNDLE_PATH: &str = ".dx/forge/template-readiness/launch-readiness-bundle.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchManifestDriftReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    template_id: &'static str,
    route: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    drift_count: usize,
    no_execution: bool,
    source_template: SourceTemplateDrift,
    package_catalog: PackageCatalogDrift,
    generated_manifest: GeneratedManifestDrift,
    companion_coverage: CompanionCoverage,
    readiness_artifacts: ReadinessArtifactSummary,
    runtime_gate: RuntimeGateDrift,
    checks: Vec<ManifestDriftCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchManifestDriftReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct SourceTemplateDrift {
    source_command: &'static str,
    route: String,
    package_count: usize,
    generated_file_count: usize,
    component_file_count: usize,
    package_ids: Vec<String>,
    generated_files: Vec<String>,
    component_materialized_files: Vec<String>,
}

#[derive(Debug, Serialize)]
struct PackageCatalogDrift {
    source_package_count: usize,
    manifest_package_count: usize,
    readiness_required_package_count: usize,
    packages_match_source_template: bool,
    packages_match_readiness_receipt: bool,
    missing_from_manifest: Vec<String>,
    missing_from_readiness: Vec<String>,
    extra_in_manifest: Vec<String>,
    extra_in_readiness: Vec<String>,
}

#[derive(Debug, Serialize)]
struct GeneratedManifestDrift {
    source_file_count: usize,
    manifest_file_count: usize,
    readiness_materialized_file_count: usize,
    files_match_source_template: bool,
    files_match_readiness_receipt: bool,
    missing_from_manifest: Vec<String>,
    missing_from_readiness: Vec<String>,
    extra_in_manifest: Vec<String>,
    extra_in_readiness: Vec<String>,
}

#[derive(Debug, Serialize)]
struct CompanionCoverage {
    present: usize,
    total: usize,
    files: Vec<CompanionCoverageFile>,
}

#[derive(Debug, Serialize)]
struct CompanionCoverageFile {
    kind: String,
    source_file: String,
    materialized_file: String,
    covered_by_generated_files: bool,
    covered_by_readiness_receipt: bool,
    present: bool,
}

#[derive(Debug, Serialize)]
struct ReadinessArtifactSummary {
    template_manifest: ArtifactStatus,
    readiness_receipt: ArtifactStatus,
    readiness_bundle: ArtifactStatus,
}

#[derive(Debug, Serialize)]
struct ArtifactStatus {
    path: &'static str,
    present: bool,
    schema: Option<String>,
}

#[derive(Debug, Serialize)]
struct RuntimeGateDrift {
    status: String,
    requires_explicit_permission: bool,
    no_execution: bool,
}

#[derive(Debug, Serialize)]
struct ManifestDriftCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn build_launch_manifest_drift_report(
    project: &Path,
    fail_under: u8,
    source_template: &Value,
) -> anyhow::Result<LaunchManifestDriftReport> {
    let manifest = read_json_file(&project.join(TEMPLATE_MANIFEST_PATH))?;
    let readiness_receipt = read_json_file(&project.join(READINESS_RECEIPT_PATH))?;
    let readiness_bundle =
        read_json_file(&project.join(READINESS_BUNDLE_PATH)).unwrap_or(Value::Null);
    let entrypoint = &manifest["www_template_entrypoint"];

    let source_template = source_template_drift(source_template);
    let package_catalog = package_catalog_drift(&source_template, &manifest, &readiness_receipt);
    let generated_manifest =
        generated_manifest_drift(&source_template, &manifest, &readiness_receipt);
    let companion_coverage =
        companion_coverage(project, entrypoint, &generated_manifest, &readiness_receipt);
    let readiness_artifacts = readiness_artifacts(&manifest, &readiness_receipt, &readiness_bundle);
    let runtime_gate = runtime_gate_drift(entrypoint, &readiness_receipt);

    let drift_count = package_catalog.missing_from_readiness.len()
        + package_catalog.missing_from_manifest.len()
        + package_catalog.extra_in_manifest.len()
        + package_catalog.extra_in_readiness.len()
        + generated_manifest.missing_from_manifest.len()
        + generated_manifest.missing_from_readiness.len()
        + generated_manifest.extra_in_manifest.len()
        + generated_manifest.extra_in_readiness.len()
        + companion_coverage
            .files
            .iter()
            .filter(|file| {
                !file.present
                    || !file.covered_by_generated_files
                    || !file.covered_by_readiness_receipt
            })
            .count();

    let checks = vec![
        check(
            "templates-source",
            source_template.package_count > 0
                && source_template.generated_file_count > 0
                && source_template.route == "/",
            format!(
                "{} package(s), {} generated file(s) from dx templates --json",
                source_template.package_count, source_template.generated_file_count
            ),
        ),
        check(
            "package-catalog",
            package_catalog.packages_match_source_template
                && package_catalog.packages_match_readiness_receipt
                && package_catalog.manifest_package_count > 0,
            format!(
                "{} source package(s), {} manifest package(s), {} readiness package(s)",
                package_catalog.source_package_count,
                package_catalog.manifest_package_count,
                package_catalog.readiness_required_package_count
            ),
        ),
        check(
            "generated-manifest",
            generated_manifest.files_match_source_template
                && generated_manifest.files_match_readiness_receipt
                && generated_manifest.manifest_file_count > 0,
            format!(
                "{} source file(s), {} manifest file(s), {} readiness file(s)",
                generated_manifest.source_file_count,
                generated_manifest.manifest_file_count,
                generated_manifest.readiness_materialized_file_count
            ),
        ),
        check(
            "companion-coverage",
            companion_coverage.present == companion_coverage.total && companion_coverage.total > 0,
            format!(
                "{}/{} launch companion files are present and cross-referenced",
                companion_coverage.present, companion_coverage.total
            ),
        ),
        check(
            "readiness-artifacts",
            readiness_artifacts.readiness_receipt.schema.as_deref()
                == Some("dx.www.template_readiness")
                && readiness_artifacts.readiness_bundle.schema.as_deref()
                    == Some("dx.launch.readiness_bundle"),
            "readiness receipt and readiness bundle schemas are present".to_string(),
        ),
        check(
            "runtime-gate",
            runtime_gate.status == "pending-governed-runtime-pass"
                && runtime_gate.requires_explicit_permission
                && runtime_gate.no_execution,
            "runtime verification remains explicitly permission-gated".to_string(),
        ),
        check(
            "node-modules-absent",
            !project.join("node_modules").exists(),
            "manifest drift report does not require package installation".to_string(),
        ),
    ];
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .collect::<Vec<_>>();
    let score = average_score(&checks);
    let passed = findings.is_empty() && drift_count == 0 && score >= fail_under;

    Ok(LaunchManifestDriftReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        template_id: TEMPLATE_ID,
        route: entrypoint["route"].as_str().unwrap_or("/").to_string(),
        passed,
        score,
        fail_under,
        drift_count,
        no_execution: true,
        source_template,
        package_catalog,
        generated_manifest,
        companion_coverage,
        readiness_artifacts,
        runtime_gate,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-adoption-report --project . --json".to_string(),
            "dx forge launch-readiness-bundle --project . --json".to_string(),
            "dx templates verify-readiness --project . --json".to_string(),
            "governed runtime verification after explicit permission".to_string(),
        ],
    })
}

pub(crate) fn launch_manifest_drift_terminal(report: &LaunchManifestDriftReport) -> String {
    format!(
        "DX Forge launch manifest drift\nProject: {}\nPassed: {}\nScore: {}\nDrift: {}\nCompanions: {}/{}\n",
        report.project,
        report.passed,
        report.score,
        report.drift_count,
        report.companion_coverage.present,
        report.companion_coverage.total
    )
}

pub(crate) fn launch_manifest_drift_markdown(report: &LaunchManifestDriftReport) -> String {
    let mut output = format!(
        "# DX Forge Launch Manifest Drift\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Drift count: `{}`\n- Companion coverage: `{}/{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.drift_count,
        report.companion_coverage.present,
        report.companion_coverage.total
    );
    output.push_str("| Check | Passed | Score | Message |\n| --- | --- | --- | --- |\n");
    for check in &report.checks {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} |\n",
            check.name,
            check.passed,
            check.score,
            markdown_cell(&check.message)
        ));
    }
    output
}

pub(crate) fn launch_manifest_drift_failure_summary(report: &LaunchManifestDriftReport) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch manifest drift score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn source_template_drift(source_template: &Value) -> SourceTemplateDrift {
    let package_ids = string_array(&source_template["package_ids"]);
    let generated_files = string_array(&source_template["generated_files"]);
    let component_materialized_files =
        string_array(&source_template["component_materialized_files"]);
    SourceTemplateDrift {
        source_command: "dx templates --json",
        route: source_template["route"].as_str().unwrap_or("/").to_string(),
        package_count: package_ids.len(),
        generated_file_count: generated_files.len(),
        component_file_count: component_materialized_files.len(),
        package_ids,
        generated_files,
        component_materialized_files,
    }
}

fn package_catalog_drift(
    source_template: &SourceTemplateDrift,
    manifest: &Value,
    readiness_receipt: &Value,
) -> PackageCatalogDrift {
    let source_packages = source_template
        .package_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let manifest_packages = manifest["www_package_catalog"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|package| package["package_id"].as_str())
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    let readiness_packages = string_set(&readiness_receipt["required_packages"]);

    PackageCatalogDrift {
        source_package_count: source_packages.len(),
        manifest_package_count: manifest_packages.len(),
        readiness_required_package_count: readiness_packages.len(),
        packages_match_source_template: source_packages == manifest_packages,
        packages_match_readiness_receipt: manifest_packages == readiness_packages,
        missing_from_manifest: set_difference(&source_packages, &manifest_packages),
        missing_from_readiness: set_difference(&manifest_packages, &readiness_packages),
        extra_in_manifest: set_difference(&manifest_packages, &source_packages),
        extra_in_readiness: set_difference(&readiness_packages, &manifest_packages),
    }
}

fn generated_manifest_drift(
    source_template: &SourceTemplateDrift,
    manifest: &Value,
    readiness_receipt: &Value,
) -> GeneratedManifestDrift {
    let source_files = source_template
        .generated_files
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let manifest_files = manifest["generated_files"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|file| file["materialized_file"].as_str())
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    let readiness_files = string_set(&readiness_receipt["materialized_files"]);

    GeneratedManifestDrift {
        source_file_count: source_files.len(),
        manifest_file_count: manifest_files.len(),
        readiness_materialized_file_count: readiness_files.len(),
        files_match_source_template: source_files == manifest_files,
        files_match_readiness_receipt: manifest_files == readiness_files,
        missing_from_manifest: set_difference(&source_files, &manifest_files),
        missing_from_readiness: set_difference(&manifest_files, &readiness_files),
        extra_in_manifest: set_difference(&manifest_files, &source_files),
        extra_in_readiness: set_difference(&readiness_files, &manifest_files),
    }
}

fn companion_coverage(
    project: &Path,
    entrypoint: &Value,
    generated_manifest: &GeneratedManifestDrift,
    readiness_receipt: &Value,
) -> CompanionCoverage {
    let generated_files = entrypoint_string_set(entrypoint, "materialized_files");
    let readiness_files = string_set(&readiness_receipt["materialized_files"]);
    let files = entrypoint["materialized_files"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|file| {
            let materialized_file = file["materialized_file"].as_str()?.to_string();
            if !materialized_file.starts_with("components/template-app/")
                || !materialized_file.ends_with(".tsx")
                || materialized_file == "components/template-app/template-shell.tsx"
            {
                return None;
            }
            Some(CompanionCoverageFile {
                kind: file["kind"].as_str().unwrap_or("unknown").to_string(),
                source_file: file["source_file"].as_str().unwrap_or("").to_string(),
                covered_by_generated_files: generated_files.contains(&materialized_file)
                    && generated_manifest.missing_from_readiness.is_empty(),
                covered_by_readiness_receipt: readiness_files.contains(&materialized_file),
                present: project.join(&materialized_file).is_file(),
                materialized_file,
            })
        })
        .collect::<Vec<_>>();
    let present = files
        .iter()
        .filter(|file| {
            file.present && file.covered_by_generated_files && file.covered_by_readiness_receipt
        })
        .count();

    CompanionCoverage {
        present,
        total: files.len(),
        files,
    }
}

fn readiness_artifacts(
    manifest: &Value,
    readiness_receipt: &Value,
    readiness_bundle: &Value,
) -> ReadinessArtifactSummary {
    ReadinessArtifactSummary {
        template_manifest: ArtifactStatus {
            path: TEMPLATE_MANIFEST_PATH,
            present: manifest.is_object(),
            schema: manifest["template"].as_str().map(str::to_string),
        },
        readiness_receipt: ArtifactStatus {
            path: READINESS_RECEIPT_PATH,
            present: readiness_receipt.is_object(),
            schema: readiness_receipt["schema"].as_str().map(str::to_string),
        },
        readiness_bundle: ArtifactStatus {
            path: READINESS_BUNDLE_PATH,
            present: readiness_bundle.is_object(),
            schema: readiness_bundle["schema"].as_str().map(str::to_string),
        },
    }
}

fn runtime_gate_drift(entrypoint: &Value, readiness_receipt: &Value) -> RuntimeGateDrift {
    let request = &entrypoint["runtime_verification_request"];
    RuntimeGateDrift {
        status: readiness_receipt["runtime_verification"]
            .as_str()
            .or_else(|| entrypoint["runtime_verification"].as_str())
            .unwrap_or("pending-governed-runtime-pass")
            .to_string(),
        requires_explicit_permission:
            readiness_receipt["runtime_verification_requires_explicit_permission"]
                .as_bool()
                .or_else(|| {
                    entrypoint["runtime_verification_requires_explicit_permission"].as_bool()
                })
                .unwrap_or(true),
        no_execution: request["no_execution"].as_bool().unwrap_or(true),
    }
}

fn read_json_file(path: &Path) -> anyhow::Result<Value> {
    let bytes = fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
}

fn string_set(value: &Value) -> BTreeSet<String> {
    string_array(value).into_iter().collect()
}

fn string_array(value: &Value) -> Vec<String> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

fn entrypoint_string_set(entrypoint: &Value, key: &str) -> BTreeSet<String> {
    entrypoint[key]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|file| file["materialized_file"].as_str())
        .map(str::to_string)
        .collect()
}

fn set_difference(left: &BTreeSet<String>, right: &BTreeSet<String>) -> Vec<String> {
    left.difference(right).cloned().collect()
}

fn check(name: &'static str, passed: bool, message: String) -> ManifestDriftCheck {
    ManifestDriftCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[ManifestDriftCheck]) -> u8 {
    if checks.is_empty() {
        return 0;
    }
    let total = checks.iter().map(|check| check.score as u16).sum::<u16>();
    (total / checks.len() as u16) as u8
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}
