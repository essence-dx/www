use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::Serialize;
use serde_json::Value;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_runtime_evidence";
const EVIDENCE_PATH: &str = ".dx/forge/template-readiness/launch-runtime-evidence.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchRuntimeEvidenceReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    fake_proof: bool,
    status: String,
    runtime_approved: bool,
    evidence_path: &'static str,
    approval_request: String,
    required_evidence: RequiredEvidenceSummary,
    collected_evidence: CollectedEvidenceSummary,
    checks: Vec<RuntimeEvidenceCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchRuntimeEvidenceReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

pub(crate) fn run_launch_runtime_evidence(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        field: Some("forge.launch-runtime-evidence".to_string()),
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
                        field: Some("forge.launch-runtime-evidence".to_string()),
                    })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--output" | "--out" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--output requires a path".to_string(),
                        field: Some("forge.launch-runtime-evidence".to_string()),
                    })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--fail-under" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--fail-under requires a score".to_string(),
                        field: Some("forge.launch-runtime-evidence".to_string()),
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
                    message: format!("Unknown forge launch-runtime-evidence option: {value}"),
                    field: Some("forge.launch-runtime-evidence".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!("Unexpected forge launch-runtime-evidence argument: {value}"),
                    field: Some("forge.launch-runtime-evidence".to_string()),
                });
            }
        }
    }

    let report = build_launch_runtime_evidence_report(&project, fail_under).map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => launch_runtime_evidence_markdown(&report),
        DxOutputFormat::Terminal => launch_runtime_evidence_terminal(&report),
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
            message: launch_runtime_evidence_failure_summary(&report),
            field: Some("forge.launch-runtime-evidence".to_string()),
        });
    }

    Ok(())
}

#[derive(Debug, Serialize)]
struct RequiredEvidenceSummary {
    total: usize,
    not_collected: usize,
    items: Vec<RequiredEvidenceItem>,
}

#[derive(Debug, Serialize)]
struct RequiredEvidenceItem {
    id: String,
    kind: String,
    status: String,
    required: bool,
    artifact_path: String,
    source_command: String,
}

#[derive(Debug, Serialize)]
struct CollectedEvidenceSummary {
    present: usize,
    existing_artifacts: usize,
    artifacts: Vec<String>,
}

#[derive(Debug, Serialize)]
struct RuntimeEvidenceCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn build_launch_runtime_evidence_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchRuntimeEvidenceReport> {
    let evidence = read_json_file(&project.join(EVIDENCE_PATH))?;
    let required_items = required_evidence(&evidence["required_evidence"]);
    let not_collected = required_items
        .iter()
        .filter(|item| item.status == "not-collected")
        .count();
    let required_evidence = RequiredEvidenceSummary {
        total: required_items.len(),
        not_collected,
        items: required_items,
    };
    let collected_artifacts = string_array(&evidence["collected_evidence"]["artifacts"]);
    let existing_artifacts = count_existing_artifacts(project, &collected_artifacts);
    let collected_evidence = CollectedEvidenceSummary {
        present: evidence["collected_evidence"]["present"]
            .as_u64()
            .map(|value| value as usize)
            .unwrap_or(collected_artifacts.len()),
        existing_artifacts,
        artifacts: collected_artifacts,
    };
    let fake_proof = evidence["fake_proof"].as_bool().unwrap_or(true);
    let no_execution = evidence["no_execution"].as_bool().unwrap_or(false);
    let status = string_field(&evidence, "status");
    let approval_request = string_field(&evidence, "approval_request");
    let approval_required = evidence["approval_required"].as_bool().unwrap_or(false)
        || evidence["blocked_until_approved"]
            .as_bool()
            .unwrap_or(false);
    let runtime_approved = approval_gate_recorded(&evidence["approval_gate"])
        || runtime_approval_recorded(project, &approval_request);
    let collected_runtime_evidence = collected_evidence.present >= required_evidence.total
        && required_evidence.total > 0
        && required_evidence.not_collected == 0
        && collected_evidence.artifacts.len() >= required_evidence.total
        && collected_evidence.existing_artifacts >= required_evidence.total;

    let checks = vec![
        check(
            "evidence-schema",
            evidence["schema"].as_str() == Some("dx.launch.runtime_evidence"),
            "runtime evidence schema stub is present".to_string(),
        ),
        check(
            "awaiting-approved-runtime-run",
            (!runtime_approved && status == "awaiting-approved-runtime-run")
                || (runtime_approved && collected_runtime_evidence),
            "runtime evidence remains pending until approval, then requires collected proof"
                .to_string(),
        ),
        check(
            "approval-gate",
            approval_required
                && approval_request
                    == ".dx/forge/template-readiness/launch-runtime-approval-request.json",
            "runtime evidence links the explicit approval request".to_string(),
        ),
        check(
            "required-evidence",
            required_evidence.total == 3
                && has_required_evidence(
                    &required_evidence.items,
                    "governed-runtime-route-response",
                )
                && has_required_evidence(
                    &required_evidence.items,
                    "production-contract-route-proof",
                )
                && has_required_evidence(&required_evidence.items, "final-launch-evidence-receipt")
                && if runtime_approved {
                    collected_runtime_evidence
                } else {
                    required_evidence.not_collected == required_evidence.total
                },
            "runtime evidence stub names all required artifacts without collected proof"
                .to_string(),
        ),
        check(
            "no-fake-proof",
            !fake_proof
                && (runtime_approved
                    || (collected_evidence.present == 0
                        && collected_evidence.artifacts.is_empty())),
            "runtime evidence stub does not include fake collected proof".to_string(),
        ),
        check(
            "approved-runtime-evidence",
            !runtime_approved || collected_runtime_evidence,
            "approved runtime verification must include real collected evidence artifacts"
                .to_string(),
        ),
        check(
            "no-execution",
            no_execution,
            "runtime evidence inspection does not run builds, previews, or servers".to_string(),
        ),
    ];
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .collect::<Vec<_>>();
    let score = average_score(&checks);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(LaunchRuntimeEvidenceReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        fake_proof,
        status,
        runtime_approved,
        evidence_path: EVIDENCE_PATH,
        approval_request,
        required_evidence,
        collected_evidence,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-runtime-approval-request --project . --json".to_string(),
            "request explicit runtime verification approval".to_string(),
            "collect approved runtime route and production-preview evidence".to_string(),
        ],
    })
}

pub(crate) fn launch_runtime_evidence_terminal(report: &LaunchRuntimeEvidenceReport) -> String {
    format!(
        "DX Forge launch runtime evidence\nProject: {}\nPassed: {}\nScore: {}\nStatus: {}\nRequired evidence: {}/{}\nCollected evidence: {}\n",
        report.project,
        report.passed,
        report.score,
        report.status,
        report.required_evidence.not_collected,
        report.required_evidence.total,
        report.collected_evidence.present
    )
}

pub(crate) fn launch_runtime_evidence_markdown(report: &LaunchRuntimeEvidenceReport) -> String {
    let mut output = format!(
        "# DX Forge Launch Runtime Evidence\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Status: `{}`\n- Required evidence: `{}`\n- Collected evidence: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.status,
        report.required_evidence.total,
        report.collected_evidence.present
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

pub(crate) fn launch_runtime_evidence_failure_summary(
    report: &LaunchRuntimeEvidenceReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch runtime evidence score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn required_evidence(value: &Value) -> Vec<RequiredEvidenceItem> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .map(|item| RequiredEvidenceItem {
            id: string_field(item, "id"),
            kind: string_field(item, "kind"),
            status: string_field(item, "status"),
            required: item["required"].as_bool().unwrap_or(false),
            artifact_path: {
                let path = string_field(item, "artifact_path");
                if path.is_empty() {
                    string_field(item, "artifact")
                } else {
                    path
                }
            },
            source_command: string_field(item, "source_command"),
        })
        .collect()
}

fn has_required_evidence(items: &[RequiredEvidenceItem], id: &str) -> bool {
    items.iter().any(|item| {
        item.id == id
            && item.required
            && !item.artifact_path.is_empty()
            && !item.source_command.is_empty()
    })
}

fn approval_gate_recorded(gate: &Value) -> bool {
    gate["approved"].as_bool().unwrap_or(false) || gate["status"].as_str() == Some("approved")
}

fn runtime_approval_recorded(project: &Path, approval_request: &str) -> bool {
    if approval_request.is_empty() {
        return false;
    }
    let Ok(request) = read_json_file(&project.join(approval_request)) else {
        return false;
    };
    let approval = &request["approval_record"];
    let approval_recorded = approval["approved"].as_bool().unwrap_or(false)
        || approval["status"].as_str() == Some("approved");
    let commands = request["requested_commands"]
        .as_array()
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    let commands_approved = !commands.is_empty()
        && commands
            .iter()
            .all(|command| command["approved"].as_bool().unwrap_or(false));

    approval_recorded && commands_approved
}

fn count_existing_artifacts(project: &Path, artifacts: &[String]) -> usize {
    artifacts
        .iter()
        .filter(|artifact| !artifact.is_empty() && project.join(artifact).is_file())
        .count()
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

fn check(name: &'static str, passed: bool, message: String) -> RuntimeEvidenceCheck {
    RuntimeEvidenceCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[RuntimeEvidenceCheck]) -> u8 {
    if checks.is_empty() {
        return 0;
    }
    let total = checks.iter().map(|check| check.score as u16).sum::<u16>();
    (total / checks.len() as u16) as u8
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}
