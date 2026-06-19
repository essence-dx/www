use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::Serialize;
use serde_json::Value;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_verification_lane";
const LANE_PATH: &str = ".dx/forge/template-readiness/launch-verification-lane.json";
const CHECKLIST_PATH: &str = ".dx/forge/template-readiness/launch-runtime-checklist.json";
const APPROVAL_REQUEST_PATH: &str =
    ".dx/forge/template-readiness/launch-runtime-approval-request.json";
const RUNTIME_EVIDENCE_PATH: &str = ".dx/forge/template-readiness/launch-runtime-evidence.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchVerificationLaneReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    lane: LaunchVerificationLaneSummary,
    open_files: Vec<LaunchVerificationOpenFile>,
    checks: Vec<LaunchVerificationLaneCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchVerificationLaneReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchVerificationLaneSummary {
    lane_id: String,
    route: String,
    file: &'static str,
    command: String,
    step_count: usize,
    present_files: usize,
    requires_explicit_permission: bool,
    runtime_approved: bool,
    blocked_without_permission: Vec<String>,
}

#[derive(Debug, Serialize)]
struct LaunchVerificationOpenFile {
    kind: String,
    path: String,
}

#[derive(Debug, Serialize)]
struct LaunchVerificationLaneCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_verification_lane(cwd: &Path, args: &[String]) -> DxResult<()> {
    let mut project = cwd.to_path_buf();
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 100u8;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--project requires a path".to_string(),
                        field: Some("forge.launch-verification-lane".to_string()),
                    })?;
                project = resolve_cli_path(cwd, value);
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--format" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--format requires a value".to_string(),
                        field: Some("forge.launch-verification-lane".to_string()),
                    })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--output" | "--out" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--output requires a path".to_string(),
                        field: Some("forge.launch-verification-lane".to_string()),
                    })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--fail-under" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--fail-under requires a score".to_string(),
                        field: Some("forge.launch-verification-lane".to_string()),
                    })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(DxError::ConfigValidationError {
                    message: format!("Unknown forge launch-verification-lane option: {value}"),
                    field: Some("forge.launch-verification-lane".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!("Unexpected forge launch-verification-lane argument: {value}"),
                    field: Some("forge.launch-verification-lane".to_string()),
                });
            }
        }
    }

    let report =
        build_launch_verification_lane_report(&project, fail_under).map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => launch_verification_lane_markdown(&report),
        DxOutputFormat::Terminal => launch_verification_lane_terminal(&report),
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
            message: launch_verification_lane_failure_summary(&report),
            field: Some("forge.launch-verification-lane".to_string()),
        });
    }

    Ok(())
}

pub(crate) fn build_launch_verification_lane_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchVerificationLaneReport> {
    let lane = read_json_file(&project.join(LANE_PATH))?;
    let open_files = open_files(&lane["open_files"]);
    let operator_steps = lane["operator_steps"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let runtime_paths = runtime_artifact_paths(&lane["runtime_artifacts"]);
    let present_files = runtime_paths
        .iter()
        .filter(|path| project.join(path).is_file())
        .count();
    let no_execution = lane["no_execution"].as_bool().unwrap_or(false);
    let requires_explicit_permission = lane["requires_explicit_permission"]
        .as_bool()
        .unwrap_or(false);
    let runtime_approved = lane["runtime_approved"].as_bool().unwrap_or(true);

    let summary = LaunchVerificationLaneSummary {
        lane_id: string_field(&lane, "lane_id"),
        route: string_field(&lane, "route"),
        file: LANE_PATH,
        command: string_field(&lane, "command"),
        step_count: operator_steps.len(),
        present_files,
        requires_explicit_permission,
        runtime_approved,
        blocked_without_permission: string_array(&lane["blocked_without_permission"]),
    };

    let checks = vec![
        check(
            "lane-schema",
            lane["schema"].as_str() == Some("dx.launch.verification_lane"),
            "launch verification lane schema is present".to_string(),
        ),
        check(
            "operator-sequence",
            has_step(
                &operator_steps,
                "review-runtime-checklist",
                CHECKLIST_PATH,
                "requires-explicit-permission",
            ) && has_step(
                &operator_steps,
                "record-runtime-approval",
                APPROVAL_REQUEST_PATH,
                "pending-explicit-approval",
            ) && has_step(
                &operator_steps,
                "collect-runtime-evidence",
                RUNTIME_EVIDENCE_PATH,
                "awaiting-approved-runtime-run",
            ),
            "operator sequence links checklist, approval request, and evidence".to_string(),
        ),
        check(
            "runtime-artifacts-present",
            runtime_paths.len() == 3 && present_files == runtime_paths.len(),
            format!(
                "{present_files}/{} runtime lane artifact(s) are present",
                runtime_paths.len()
            ),
        ),
        check(
            "zed-open-files",
            has_open_file(&open_files, "runtime-checklist", CHECKLIST_PATH)
                && has_open_file(
                    &open_files,
                    "runtime-approval-request",
                    APPROVAL_REQUEST_PATH,
                )
                && has_open_file(&open_files, "runtime-evidence", RUNTIME_EVIDENCE_PATH),
            "DX CLI/Zed open-file map includes every runtime lane artifact".to_string(),
        ),
        check(
            "permission-gate",
            requires_explicit_permission && !runtime_approved && no_execution,
            "lane stays no-execution until explicit runtime approval is recorded".to_string(),
        ),
    ];
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .collect::<Vec<_>>();
    let score = average_score(&checks);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(LaunchVerificationLaneReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        lane: summary,
        open_files,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-runtime-checklist --project . --json".to_string(),
            "dx forge launch-runtime-approval-request --project . --json".to_string(),
            "dx forge launch-runtime-evidence --project . --json".to_string(),
        ],
    })
}

pub(crate) fn launch_verification_lane_terminal(report: &LaunchVerificationLaneReport) -> String {
    format!(
        "DX Forge launch verification lane\nProject: {}\nPassed: {}\nScore: {}\nLane: {}\nRuntime files: {}\n",
        report.project, report.passed, report.score, report.lane.lane_id, report.lane.present_files
    )
}

pub(crate) fn launch_verification_lane_markdown(report: &LaunchVerificationLaneReport) -> String {
    let mut output = format!(
        "# DX Forge Launch Verification Lane\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Lane: `{}`\n- Runtime files: `{}`\n\n",
        report.project, report.passed, report.score, report.lane.lane_id, report.lane.present_files
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

pub(crate) fn launch_verification_lane_failure_summary(
    report: &LaunchVerificationLaneReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch verification lane score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn runtime_artifact_paths(value: &Value) -> Vec<String> {
    ["checklist", "approval_request", "runtime_evidence"]
        .iter()
        .filter_map(|key| value[*key].as_str())
        .map(str::to_string)
        .collect()
}

fn open_files(value: &Value) -> Vec<LaunchVerificationOpenFile> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .map(|item| LaunchVerificationOpenFile {
            kind: string_field(item, "kind"),
            path: string_field(item, "path"),
        })
        .collect()
}

fn has_open_file(open_files: &[LaunchVerificationOpenFile], kind: &str, path: &str) -> bool {
    open_files
        .iter()
        .any(|file| file.kind == kind && file.path == path)
}

fn has_step(steps: &[Value], id: &str, file: &str, status: &str) -> bool {
    steps.iter().any(|step| {
        step["id"].as_str() == Some(id)
            && step["file"].as_str() == Some(file)
            && step["status"].as_str() == Some(status)
    })
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

fn string_field(value: &Value, field: &str) -> String {
    value[field].as_str().unwrap_or("").to_string()
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchVerificationLaneCheck {
    LaunchVerificationLaneCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchVerificationLaneCheck]) -> u8 {
    if checks.is_empty() {
        return 0;
    }
    let total = checks.iter().map(|check| check.score as u16).sum::<u16>();
    (total / checks.len() as u16) as u8
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}
