use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::Serialize;
use serde_json::Value;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_runtime_approval_request";
const REQUEST_PATH: &str = ".dx/forge/template-readiness/launch-runtime-approval-request.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchRuntimeApprovalRequestReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    request_path: &'static str,
    approval: RuntimeApprovalRecord,
    requested_commands: RequestedCommandSummary,
    requested_evidence: RequestedEvidenceSummary,
    checks: Vec<RuntimeApprovalCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchRuntimeApprovalRequestReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

pub(crate) fn run_launch_runtime_approval_request(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        field: Some("forge.launch-runtime-approval-request".to_string()),
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
                        field: Some("forge.launch-runtime-approval-request".to_string()),
                    })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--output" | "--out" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--output requires a path".to_string(),
                        field: Some("forge.launch-runtime-approval-request".to_string()),
                    })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--fail-under" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--fail-under requires a score".to_string(),
                        field: Some("forge.launch-runtime-approval-request".to_string()),
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
                    message: format!(
                        "Unknown forge launch-runtime-approval-request option: {value}"
                    ),
                    field: Some("forge.launch-runtime-approval-request".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-runtime-approval-request argument: {value}"
                    ),
                    field: Some("forge.launch-runtime-approval-request".to_string()),
                });
            }
        }
    }

    let report =
        build_launch_runtime_approval_request_report(&project, fail_under).map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => launch_runtime_approval_request_markdown(&report),
        DxOutputFormat::Terminal => launch_runtime_approval_request_terminal(&report),
    };

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent).map_err(forge_error)?;
        }
        std::fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_runtime_approval_request_failure_summary(&report),
            field: Some("forge.launch-runtime-approval-request".to_string()),
        });
    }

    Ok(())
}

#[derive(Debug, Serialize)]
struct RuntimeApprovalRecord {
    status: String,
    approved_by: Option<String>,
    approved_at: Option<String>,
    approval_note: Option<String>,
    scope: String,
}

#[derive(Debug, Serialize)]
struct RequestedCommandSummary {
    total: usize,
    approved: usize,
    requires_explicit_permission: usize,
    items: Vec<RequestedCommand>,
}

#[derive(Debug, Serialize)]
struct RequestedCommand {
    id: String,
    command: String,
    approved: bool,
    requires_explicit_permission: bool,
    expected_evidence: String,
}

#[derive(Debug, Serialize)]
struct RequestedEvidenceSummary {
    total: usize,
    items: Vec<String>,
    receipt: Option<String>,
}

#[derive(Debug, Serialize)]
struct RuntimeApprovalCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn build_launch_runtime_approval_request_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchRuntimeApprovalRequestReport> {
    let request = read_json_file(&project.join(REQUEST_PATH))?;
    let approval = RuntimeApprovalRecord {
        status: string_field(&request["approval_record"], "status"),
        approved_by: optional_string(&request["approval_record"], "approved_by"),
        approved_at: optional_string(&request["approval_record"], "approved_at"),
        approval_note: optional_string(&request["approval_record"], "approval_note"),
        scope: string_field(&request["approval_record"], "scope"),
    };
    let commands = requested_commands(&request["requested_commands"]);
    let command_summary = RequestedCommandSummary {
        total: commands.len(),
        approved: commands.iter().filter(|command| command.approved).count(),
        requires_explicit_permission: commands
            .iter()
            .filter(|command| command.requires_explicit_permission)
            .count(),
        items: commands,
    };
    let evidence_items = string_array(&request["requested_evidence"]["items"]);
    let requested_evidence = RequestedEvidenceSummary {
        total: evidence_items.len(),
        items: evidence_items,
        receipt: optional_string(&request["requested_evidence"], "receipt"),
    };
    let no_execution = request["no_execution"].as_bool().unwrap_or(false);
    let checks = vec![
        check(
            "request-schema",
            request["schema"].as_str() == Some("dx.launch.runtime_approval_request"),
            "runtime approval request schema is present".to_string(),
        ),
        check(
            "approval-pending",
            approval.status == "pending-explicit-approval"
                && approval.approved_by.is_none()
                && approval.approved_at.is_none(),
            "runtime approval request remains pending without an approver".to_string(),
        ),
        check(
            "commands-unapproved",
            command_summary.total > 0
                && command_summary.approved == 0
                && command_summary.requires_explicit_permission == command_summary.total,
            "requested runtime commands are unapproved and permission-gated".to_string(),
        ),
        check(
            "requested-evidence",
            requested_evidence
                .items
                .iter()
                .any(|evidence| evidence == "final-launch-evidence-receipt")
                && requested_evidence.receipt.as_deref()
                    == Some(".dx/forge/template-readiness/launch-runtime-evidence.json"),
            "runtime approval request names the final evidence receipt".to_string(),
        ),
        check(
            "no-execution",
            no_execution,
            "approval request inspection does not run builds, previews, or servers".to_string(),
        ),
    ];
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .collect::<Vec<_>>();
    let score = average_score(&checks);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(LaunchRuntimeApprovalRequestReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        request_path: REQUEST_PATH,
        approval,
        requested_commands: command_summary,
        requested_evidence,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-runtime-checklist --project . --json".to_string(),
            "dx forge launch-readiness-bundle --project . --json".to_string(),
            "request explicit runtime verification approval".to_string(),
        ],
    })
}

pub(crate) fn launch_runtime_approval_request_terminal(
    report: &LaunchRuntimeApprovalRequestReport,
) -> String {
    format!(
        "DX Forge launch runtime approval request\nProject: {}\nPassed: {}\nScore: {}\nStatus: {}\nCommands approved: {}/{}\n",
        report.project,
        report.passed,
        report.score,
        report.approval.status,
        report.requested_commands.approved,
        report.requested_commands.total
    )
}

pub(crate) fn launch_runtime_approval_request_markdown(
    report: &LaunchRuntimeApprovalRequestReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Runtime Approval Request\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Status: `{}`\n- Commands approved: `{}/{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.approval.status,
        report.requested_commands.approved,
        report.requested_commands.total
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

pub(crate) fn launch_runtime_approval_request_failure_summary(
    report: &LaunchRuntimeApprovalRequestReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch runtime approval request score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn requested_commands(value: &Value) -> Vec<RequestedCommand> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .map(|command| RequestedCommand {
            id: string_field(command, "id"),
            command: string_field(command, "command"),
            approved: command["approved"].as_bool().unwrap_or(false),
            requires_explicit_permission: command["requires_explicit_permission"]
                .as_bool()
                .unwrap_or(true),
            expected_evidence: string_field(command, "expected_evidence"),
        })
        .collect()
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

fn optional_string(value: &Value, field: &str) -> Option<String> {
    value[field].as_str().map(str::to_string)
}

fn check(name: &'static str, passed: bool, message: String) -> RuntimeApprovalCheck {
    RuntimeApprovalCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[RuntimeApprovalCheck]) -> u8 {
    if checks.is_empty() {
        return 0;
    }
    let total = checks.iter().map(|check| check.score as u16).sum::<u16>();
    (total / checks.len() as u16) as u8
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}
