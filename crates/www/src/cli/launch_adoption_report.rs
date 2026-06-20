use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use chrono::Utc;
use serde::Serialize;
use serde_json::Value;

const REPORT_SCHEMA: &str = "dx.forge.launch_adoption_report";
const TEMPLATE_ID: &str = "next-familiar-www-template";
const TEMPLATE_MANIFEST_PATH: &str = ".dx/forge/template-.dx/build-cache/manifest.json";
const READINESS_RECEIPT_PATH: &str = ".dx/forge/template-readiness/launch-route.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchAdoptionReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    template_id: &'static str,
    route: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    readiness_receipt: ReadinessReceiptSummary,
    companion_files: CompanionFileSummary,
    app_owned_dependencies: AppOwnedDependencySummary,
    runtime_proofs: RuntimeProofSummary,
    checks: Vec<AdoptionReportCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchAdoptionReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct CompanionFileSummary {
    present: usize,
    total: usize,
    files: Vec<CompanionFile>,
}

#[derive(Debug, Serialize)]
struct CompanionFile {
    kind: String,
    source_file: String,
    materialized_file: String,
    present: bool,
}

#[derive(Debug, Serialize)]
struct ReadinessReceiptSummary {
    path: &'static str,
    present: bool,
    schema: Option<String>,
    status: Option<String>,
    score: Option<u64>,
}

#[derive(Debug, Serialize)]
struct AppOwnedDependencySummary {
    package_count: usize,
    required_env: Vec<String>,
    packages: Vec<AppOwnedPackage>,
}

#[derive(Debug, Serialize)]
struct AppOwnedPackage {
    package_id: String,
    role: String,
    command: String,
    required_env: Vec<String>,
    app_owned_boundaries: Vec<String>,
}

#[derive(Debug, Serialize)]
struct RuntimeProofSummary {
    status: String,
    requires_explicit_permission: bool,
    expected_evidence: Vec<String>,
    automation_default: String,
    no_execution: bool,
}

#[derive(Debug, Serialize)]
struct AdoptionReportCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn build_launch_adoption_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchAdoptionReport> {
    let manifest = read_json_file(&project.join(TEMPLATE_MANIFEST_PATH))?;
    let entrypoint = &manifest["www_template_entrypoint"];
    let readiness_receipt = readiness_receipt_summary(project)?;
    let companion_files = companion_file_summary(project, entrypoint);
    let app_owned_dependencies = app_owned_dependency_summary(&manifest);
    let runtime_proofs = runtime_proof_summary(entrypoint);
    let checks = vec![
        check(
            "readiness-receipt",
            readiness_receipt.present
                && readiness_receipt.schema.as_deref() == Some("dx.www.template_readiness"),
            if readiness_receipt.present
                && readiness_receipt.schema.as_deref() == Some("dx.www.template_readiness")
            {
                100
            } else {
                0
            },
            format!(
                "template readiness receipt {}",
                if readiness_receipt.present {
                    "is present"
                } else {
                    "is missing"
                }
            ),
        ),
        check(
            "companion-files",
            companion_files.present == companion_files.total && companion_files.total > 0,
            if companion_files.present == companion_files.total && companion_files.total > 0 {
                100
            } else {
                0
            },
            format!(
                "{}/{} launch companion files are present",
                companion_files.present, companion_files.total
            ),
        ),
        check(
            "app-owned-dependencies",
            app_owned_dependencies.package_count > 0,
            if app_owned_dependencies.package_count > 0 {
                100
            } else {
                0
            },
            format!(
                "{} package boundary record(s), {} required env var(s)",
                app_owned_dependencies.package_count,
                app_owned_dependencies.required_env.len()
            ),
        ),
        check(
            "runtime-permission-gate",
            runtime_proofs.status == "pending-governed-runtime-pass"
                && runtime_proofs.requires_explicit_permission
                && !runtime_proofs.expected_evidence.is_empty(),
            if runtime_proofs.status == "pending-governed-runtime-pass"
                && runtime_proofs.requires_explicit_permission
                && !runtime_proofs.expected_evidence.is_empty()
            {
                100
            } else {
                0
            },
            "runtime proofs remain permission-gated".to_string(),
        ),
        check(
            "node-modules-absent",
            !project.join("node_modules").exists(),
            if project.join("node_modules").exists() {
                0
            } else {
                100
            },
            "launch adoption report does not require package installation".to_string(),
        ),
    ];
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .collect::<Vec<_>>();
    let score = average_score(&checks);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(LaunchAdoptionReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        template_id: TEMPLATE_ID,
        route: entrypoint["route"].as_str().unwrap_or("/").to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        readiness_receipt,
        companion_files,
        app_owned_dependencies,
        runtime_proofs,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-readiness-bundle --project . --json".to_string(),
            "dx templates verify-readiness --project . --json".to_string(),
            "governed runtime verification after explicit permission".to_string(),
        ],
    })
}

pub(crate) fn launch_adoption_report_terminal(report: &LaunchAdoptionReport) -> String {
    format!(
        "DX Forge launch adoption report\nProject: {}\nPassed: {}\nScore: {}\nCompanion files: {}/{}\nRuntime: {}\n",
        report.project,
        report.passed,
        report.score,
        report.companion_files.present,
        report.companion_files.total,
        report.runtime_proofs.status
    )
}

pub(crate) fn launch_adoption_report_markdown(report: &LaunchAdoptionReport) -> String {
    format!(
        "# DX Forge Launch Adoption Report\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Companion files: `{}/{}`\n- Runtime: `{}`\n",
        report.project,
        report.passed,
        report.score,
        report.companion_files.present,
        report.companion_files.total,
        report.runtime_proofs.status
    )
}

pub(crate) fn launch_adoption_report_failure_summary(report: &LaunchAdoptionReport) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch adoption report score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn readiness_receipt_summary(project: &Path) -> anyhow::Result<ReadinessReceiptSummary> {
    let path = project.join(READINESS_RECEIPT_PATH);
    if !path.is_file() {
        return Ok(ReadinessReceiptSummary {
            path: READINESS_RECEIPT_PATH,
            present: false,
            schema: None,
            status: None,
            score: None,
        });
    }

    let receipt = read_json_file(&path)?;
    Ok(ReadinessReceiptSummary {
        path: READINESS_RECEIPT_PATH,
        present: true,
        schema: receipt["schema"].as_str().map(str::to_string),
        status: receipt["status"].as_str().map(str::to_string),
        score: receipt["summary"]["score"].as_u64(),
    })
}

fn companion_file_summary(project: &Path, entrypoint: &Value) -> CompanionFileSummary {
    let files = entrypoint["materialized_files"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|file| {
            let materialized_file = file["materialized_file"].as_str()?.to_string();
            let source_file = file["source_file"].as_str()?.to_string();
            if materialized_file.starts_with("components/template-app/")
                && materialized_file.ends_with(".tsx")
                && materialized_file != "components/template-app/template-shell.tsx"
            {
                Some(CompanionFile {
                    kind: file["kind"].as_str().unwrap_or("unknown").to_string(),
                    present: project.join(&materialized_file).is_file(),
                    source_file,
                    materialized_file,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let present = files.iter().filter(|file| file.present).count();

    CompanionFileSummary {
        present,
        total: files.len(),
        files,
    }
}

fn app_owned_dependency_summary(manifest: &Value) -> AppOwnedDependencySummary {
    let packages = manifest["www_package_catalog"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|package| AppOwnedPackage {
            package_id: package["package_id"]
                .as_str()
                .unwrap_or("unknown")
                .to_string(),
            role: package["role"].as_str().unwrap_or("supporting").to_string(),
            command: package["command"]
                .as_str()
                .unwrap_or("dx forge packages --json")
                .to_string(),
            required_env: string_array(&package["env"]),
            app_owned_boundaries: string_array(&package["app_owned_boundaries"]),
        })
        .collect::<Vec<_>>();
    let required_env = packages
        .iter()
        .flat_map(|package| package.required_env.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    AppOwnedDependencySummary {
        package_count: packages.len(),
        required_env,
        packages,
    }
}

fn runtime_proof_summary(entrypoint: &Value) -> RuntimeProofSummary {
    let request = &entrypoint["runtime_verification_request"];

    RuntimeProofSummary {
        status: entrypoint["runtime_verification"]
            .as_str()
            .unwrap_or("pending-governed-runtime-pass")
            .to_string(),
        requires_explicit_permission:
            entrypoint["runtime_verification_requires_explicit_permission"]
                .as_bool()
                .unwrap_or(true),
        expected_evidence: string_array(&request["expected_evidence"]),
        automation_default: request["automation_default"]
            .as_str()
            .unwrap_or("skip-runtime-build-preview")
            .to_string(),
        no_execution: request["no_execution"].as_bool().unwrap_or(true),
    }
}

fn read_json_file(path: &Path) -> anyhow::Result<Value> {
    let bytes = fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
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

fn check(name: &'static str, passed: bool, score: u8, message: String) -> AdoptionReportCheck {
    AdoptionReportCheck {
        name,
        passed,
        score,
        message,
    }
}

fn average_score(checks: &[AdoptionReportCheck]) -> u8 {
    if checks.is_empty() {
        return 0;
    }
    let total = checks.iter().map(|check| check.score as u16).sum::<u16>();
    (total / checks.len() as u16) as u8
}
