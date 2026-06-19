use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_runtime_evidence_import_plan";
const APPROVAL_REQUEST_PATH: &str =
    ".dx/forge/template-readiness/launch-runtime-approval-request.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchRuntimeEvidenceImportPlanReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    approval: RuntimeImportApproval,
    imports: RuntimeImportSummary,
    checks: Vec<RuntimeImportCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchRuntimeEvidenceImportPlanReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct RuntimeImportApproval {
    path: &'static str,
    approved: bool,
    approved_by: Option<String>,
    approved_at: Option<String>,
    commands_approved: usize,
    commands_total: usize,
}

#[derive(Debug, Serialize)]
struct RuntimeImportSummary {
    total: usize,
    source_present: usize,
    source_hashed: usize,
    items: Vec<RuntimeImportItem>,
}

#[derive(Debug, Serialize)]
struct RuntimeImportItem {
    id: &'static str,
    kind: &'static str,
    source_path: String,
    source_exists: bool,
    source_bytes: Option<u64>,
    hash_algorithm: &'static str,
    source_hash: Option<String>,
    target_path: &'static str,
}

#[derive(Debug, Serialize)]
struct RuntimeImportCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_runtime_evidence_import_plan(cwd: &Path, args: &[String]) -> DxResult<()> {
    let mut project = cwd.to_path_buf();
    let mut build_log: Option<PathBuf> = None;
    let mut route_response: Option<PathBuf> = None;
    let mut preview_proof: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 100u8;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = required_arg(args, index, "--project")?;
                project = resolve_cli_path(cwd, value);
                index += 2;
            }
            "--build-log" => {
                let value = required_arg(args, index, "--build-log")?;
                build_log = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--route-response" => {
                let value = required_arg(args, index, "--route-response")?;
                route_response = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--preview-proof" => {
                let value = required_arg(args, index, "--preview-proof")?;
                preview_proof = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--format" => {
                let value = required_arg(args, index, "--format")?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--output" | "--out" => {
                let value = required_arg(args, index, "--output")?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--fail-under" => {
                let value = required_arg(args, index, "--fail-under")?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unknown forge launch-runtime-evidence-import-plan option: {value}"
                    ),
                    field: Some("forge.launch-runtime-evidence-import-plan".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-runtime-evidence-import-plan argument: {value}"
                    ),
                    field: Some("forge.launch-runtime-evidence-import-plan".to_string()),
                });
            }
        }
    }

    let report = build_launch_runtime_evidence_import_plan_report(
        &project,
        build_log,
        route_response,
        preview_proof,
        fail_under,
    )
    .map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => launch_runtime_evidence_import_plan_markdown(&report),
        DxOutputFormat::Terminal => launch_runtime_evidence_import_plan_terminal(&report),
    };

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_runtime_evidence_import_plan_failure_summary(&report),
            field: Some("forge.launch-runtime-evidence-import-plan".to_string()),
        });
    }

    Ok(())
}

pub(crate) fn build_launch_runtime_evidence_import_plan_report(
    project: &Path,
    build_log: Option<PathBuf>,
    route_response: Option<PathBuf>,
    preview_proof: Option<PathBuf>,
    fail_under: u8,
) -> anyhow::Result<LaunchRuntimeEvidenceImportPlanReport> {
    let approval = runtime_import_approval(project);
    let items = vec![
        import_item(
            "production-contract-build-log",
            "build-log",
            build_log,
            ".dx/forge/runtime/production-contract-build.log",
        ),
        import_item(
            "governed-runtime-route-response",
            "route-response",
            route_response,
            ".dx/forge/runtime/launch-route-response.json",
        ),
        import_item(
            "production-contract-route-proof",
            "production-preview",
            preview_proof,
            ".dx/forge/runtime/production-contract-route-proof.json",
        ),
    ];
    let source_present = items.iter().filter(|item| item.source_exists).count();
    let source_hashed = items
        .iter()
        .filter(|item| item.source_hash.is_some())
        .count();
    let imports = RuntimeImportSummary {
        total: items.len(),
        source_present,
        source_hashed,
        items,
    };

    let checks = vec![
        check(
            "runtime-approval-recorded",
            approval.approved,
            "runtime evidence import requires the approved approval-request receipt".to_string(),
        ),
        check(
            "source-files-present",
            imports.source_present == imports.total,
            format!(
                "{}/{} external runtime evidence source file(s) exist",
                imports.source_present, imports.total
            ),
        ),
        check(
            "source-hashes-present",
            imports.source_hashed == imports.source_present,
            format!(
                "{}/{} present runtime evidence source file(s) include BLAKE3 hashes",
                imports.source_hashed, imports.source_present
            ),
        ),
        check(
            "runtime-targets-declared",
            imports
                .items
                .iter()
                .all(|item| item.target_path.starts_with(".dx/forge/runtime/")),
            "runtime evidence targets stay inside the Forge runtime evidence directory".to_string(),
        ),
        check(
            "no-execution",
            true,
            "import planning does not copy files, run builds, previews, or servers".to_string(),
        ),
    ];
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .collect::<Vec<_>>();
    let score = average_score(&checks);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(LaunchRuntimeEvidenceImportPlanReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        approval,
        imports,
        checks,
        findings,
        next_commands: vec![
            "copy approved evidence files into the reported target paths".to_string(),
            "dx forge launch-runtime-evidence --project . --json".to_string(),
        ],
    })
}

pub(crate) fn launch_runtime_evidence_import_plan_terminal(
    report: &LaunchRuntimeEvidenceImportPlanReport,
) -> String {
    format!(
        "DX Forge launch runtime evidence import plan\nProject: {}\nPassed: {}\nScore: {}\nApproved: {}\nSources: {}/{}\n",
        report.project,
        report.passed,
        report.score,
        report.approval.approved,
        report.imports.source_present,
        report.imports.total
    )
}

pub(crate) fn launch_runtime_evidence_import_plan_markdown(
    report: &LaunchRuntimeEvidenceImportPlanReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Runtime Evidence Import Plan\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Approved: `{}`\n- Sources: `{}/{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.approval.approved,
        report.imports.source_present,
        report.imports.total
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

pub(crate) fn launch_runtime_evidence_import_plan_failure_summary(
    report: &LaunchRuntimeEvidenceImportPlanReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch runtime evidence import plan score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn runtime_import_approval(project: &Path) -> RuntimeImportApproval {
    let request = read_json_file(&project.join(APPROVAL_REQUEST_PATH));
    let approval_record = request
        .as_ref()
        .map(|value| &value["approval_record"])
        .unwrap_or(&serde_json::Value::Null);
    let approved_recorded = approval_record["approved"].as_bool().unwrap_or(false)
        || approval_record["status"].as_str() == Some("approved");
    let commands = request
        .as_ref()
        .and_then(|value| value["requested_commands"].as_array())
        .cloned()
        .unwrap_or_default();
    let commands_approved = commands
        .iter()
        .filter(|command| command["approved"].as_bool().unwrap_or(false))
        .count();
    let approved = approved_recorded && !commands.is_empty() && commands_approved == commands.len();

    RuntimeImportApproval {
        path: APPROVAL_REQUEST_PATH,
        approved,
        approved_by: optional_string(approval_record, "approved_by"),
        approved_at: optional_string(approval_record, "approved_at"),
        commands_approved,
        commands_total: commands.len(),
    }
}

fn import_item(
    id: &'static str,
    kind: &'static str,
    source: Option<PathBuf>,
    target_path: &'static str,
) -> RuntimeImportItem {
    let metadata = source.as_ref().and_then(|path| source_metadata(path));
    let source_exists = metadata.is_some();
    RuntimeImportItem {
        id,
        kind,
        source_path: source
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_default(),
        source_exists,
        source_bytes: metadata.as_ref().map(|metadata| metadata.bytes),
        hash_algorithm: "blake3",
        source_hash: metadata.map(|metadata| metadata.hash),
        target_path,
    }
}

struct RuntimeSourceMetadata {
    bytes: u64,
    hash: String,
}

fn source_metadata(path: &Path) -> Option<RuntimeSourceMetadata> {
    let bytes = fs::read(path).ok()?;
    Some(RuntimeSourceMetadata {
        bytes: bytes.len() as u64,
        hash: format!("blake3:{}", blake3::hash(&bytes).to_hex()),
    })
}

fn required_arg<'a>(args: &'a [String], index: usize, name: &str) -> DxResult<&'a str> {
    args.get(index + 1)
        .map(String::as_str)
        .ok_or_else(|| DxError::ConfigValidationError {
            message: format!("{name} requires a value"),
            field: Some("forge.launch-runtime-evidence-import-plan".to_string()),
        })
}

fn read_json_file(path: &Path) -> Option<serde_json::Value> {
    let bytes = fs::read(path).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn optional_string(value: &serde_json::Value, field: &str) -> Option<String> {
    value[field].as_str().map(str::to_string)
}

fn check(name: &'static str, passed: bool, message: String) -> RuntimeImportCheck {
    RuntimeImportCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[RuntimeImportCheck]) -> u8 {
    if checks.is_empty() {
        return 0;
    }
    let total = checks.iter().map(|check| check.score as u16).sum::<u16>();
    (total / checks.len() as u16) as u8
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}
