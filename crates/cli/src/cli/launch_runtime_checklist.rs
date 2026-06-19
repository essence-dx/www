use std::fs;
use std::path::Path;

use chrono::Utc;
use serde::Serialize;
use serde_json::Value;

const REPORT_SCHEMA: &str = "dx.forge.launch_runtime_checklist";
const CHECKLIST_PATH: &str = ".dx/forge/template-readiness/launch-runtime-checklist.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchRuntimeChecklistReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    template_id: String,
    route: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    checklist_path: &'static str,
    approval: RuntimeChecklistApproval,
    runtime_commands: RuntimeCommandsSummary,
    expected_evidence: ExpectedEvidenceSummary,
    checks: Vec<RuntimeChecklistCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchRuntimeChecklistReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct RuntimeChecklistApproval {
    status: String,
    requires_explicit_permission: bool,
    default_action: String,
}

#[derive(Debug, Serialize)]
struct RuntimeCommandsSummary {
    total: usize,
    requires_explicit_approval: usize,
    skipped_by_default: usize,
    blocked_by_default: bool,
    commands: Vec<RuntimeCommandSummary>,
}

#[derive(Debug, Serialize)]
struct RuntimeCommandSummary {
    id: String,
    command: String,
    purpose: String,
    requires_explicit_permission: bool,
    default_action: String,
    expected_evidence: Option<String>,
}

#[derive(Debug, Serialize)]
struct ExpectedEvidenceSummary {
    items: Vec<String>,
    receipt: Option<String>,
}

#[derive(Debug, Serialize)]
struct RuntimeChecklistCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn build_launch_runtime_checklist_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchRuntimeChecklistReport> {
    let checklist = read_json_file(&project.join(CHECKLIST_PATH))?;
    let approval = RuntimeChecklistApproval {
        status: string_field(&checklist["approval"], "status"),
        requires_explicit_permission: checklist["approval"]["requires_explicit_permission"]
            .as_bool()
            .unwrap_or(false),
        default_action: string_field(&checklist["approval"], "default_action"),
    };
    let no_execution = checklist["no_execution"].as_bool().unwrap_or(false);
    let commands = runtime_commands(&checklist["commands"]);
    let requires_explicit_approval = commands
        .iter()
        .filter(|command| command.requires_explicit_permission)
        .count();
    let skipped_by_default = commands
        .iter()
        .filter(|command| command.default_action == "skip")
        .count();
    let runtime_commands = RuntimeCommandsSummary {
        total: commands.len(),
        requires_explicit_approval,
        skipped_by_default,
        blocked_by_default: !commands.is_empty()
            && requires_explicit_approval == commands.len()
            && skipped_by_default == commands.len()
            && approval.status == "requires-explicit-permission",
        commands,
    };
    let expected_evidence = expected_evidence_summary(&checklist["expected_evidence"]);
    let checks = vec![
        check(
            "checklist-schema",
            checklist["schema"].as_str() == Some("dx.launch.runtime_checklist"),
            "runtime checklist schema is present".to_string(),
        ),
        check(
            "approval-required",
            approval.status == "requires-explicit-permission"
                && approval.requires_explicit_permission
                && approval.default_action == "skip-runtime-build-preview",
            "runtime verification requires explicit approval before commands run".to_string(),
        ),
        check(
            "runtime-commands-gated",
            runtime_commands.blocked_by_default,
            "build and preview commands are skipped until explicit approval".to_string(),
        ),
        check(
            "expected-evidence",
            expected_evidence
                .items
                .iter()
                .any(|evidence| evidence == "governed-runtime-route-response")
                && expected_evidence
                    .items
                    .iter()
                    .any(|evidence| evidence == "final-launch-evidence-receipt"),
            "runtime pass lists the expected evidence receipts".to_string(),
        ),
        check(
            "no-execution",
            no_execution,
            "checklist inspection does not run builds, previews, or servers".to_string(),
        ),
    ];
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .collect::<Vec<_>>();
    let score = average_score(&checks);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(LaunchRuntimeChecklistReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        template_id: checklist["template_id"]
            .as_str()
            .unwrap_or("next-familiar-www-template")
            .to_string(),
        route: checklist["route"].as_str().unwrap_or("/").to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        checklist_path: CHECKLIST_PATH,
        approval,
        runtime_commands,
        expected_evidence,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-readiness-bundle --project . --json".to_string(),
            "dx forge launch-companion-receipts --project . --json".to_string(),
            "dx forge launch-manifest-drift --project . --json".to_string(),
            "request explicit runtime verification approval".to_string(),
        ],
    })
}

pub(crate) fn launch_runtime_checklist_terminal(report: &LaunchRuntimeChecklistReport) -> String {
    format!(
        "DX Forge launch runtime checklist\nProject: {}\nPassed: {}\nScore: {}\nApproval: {}\nRuntime commands: {}/{}\n",
        report.project,
        report.passed,
        report.score,
        report.approval.status,
        report.runtime_commands.requires_explicit_approval,
        report.runtime_commands.total
    )
}

pub(crate) fn launch_runtime_checklist_markdown(report: &LaunchRuntimeChecklistReport) -> String {
    let mut output = format!(
        "# DX Forge Launch Runtime Checklist\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Approval: `{}`\n- Runtime commands: `{}/{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.approval.status,
        report.runtime_commands.requires_explicit_approval,
        report.runtime_commands.total
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

pub(crate) fn launch_runtime_checklist_failure_summary(
    report: &LaunchRuntimeChecklistReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch runtime checklist score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
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

fn runtime_commands(value: &Value) -> Vec<RuntimeCommandSummary> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .map(|command| RuntimeCommandSummary {
            id: command["id"].as_str().unwrap_or("").to_string(),
            command: string_field(command, "command"),
            purpose: string_field(command, "purpose"),
            requires_explicit_permission: command["requires_explicit_approval"]
                .as_bool()
                .unwrap_or(false),
            default_action: string_field(command, "default_action"),
            expected_evidence: command["expected_evidence"].as_str().map(str::to_string),
        })
        .collect()
}

fn expected_evidence_summary(value: &Value) -> ExpectedEvidenceSummary {
    if value.is_array() {
        return ExpectedEvidenceSummary {
            items: string_array(value),
            receipt: None,
        };
    }

    ExpectedEvidenceSummary {
        items: string_array(&value["items"]),
        receipt: value["receipt"].as_str().map(str::to_string),
    }
}

fn check(name: &'static str, passed: bool, message: String) -> RuntimeChecklistCheck {
    RuntimeChecklistCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[RuntimeChecklistCheck]) -> u8 {
    if checks.is_empty() {
        return 0;
    }
    let total = checks.iter().map(|check| check.score as u16).sum::<u16>();
    (total / checks.len() as u16) as u8
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}
