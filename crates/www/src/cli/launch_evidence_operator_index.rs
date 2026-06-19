use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_operator_index";
const INDEX_SCHEMA: &str = "dx.launch.evidence_operator_index";
const INDEX_PATH: &str = ".dx/forge/release/launch-evidence-operator-index.json";
const TEMPLATE_READINESS_PATH: &str = ".dx/forge/template-readiness/launch-route.json";
const READINESS_BUNDLE_PATH: &str = ".dx/forge/template-readiness/launch-readiness-bundle.json";
const RUNTIME_CHECKLIST_PATH: &str = ".dx/forge/template-readiness/launch-runtime-checklist.json";
const RUNTIME_APPROVAL_PATH: &str =
    ".dx/forge/template-readiness/launch-runtime-approval-request.json";
const RUNTIME_EVIDENCE_PATH: &str = ".dx/forge/template-readiness/launch-runtime-evidence.json";
const FINAL_RECEIPT_PATH: &str = ".dx/forge/runtime/final-launch-evidence-receipt.json";
const FINAL_REVIEW_PATH: &str = ".dx/forge/runtime/final-launch-evidence-review.json";
const PACKET_PATH: &str = ".dx/forge/release/launch-evidence-packet.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceOperatorIndexReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    index: LaunchEvidenceOperatorIndexSummary,
    steps: Vec<LaunchEvidenceOperatorStep>,
    stale_step_hints: Vec<LaunchEvidenceStaleStepHint>,
    checks: Vec<LaunchEvidenceOperatorIndexCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceOperatorIndexSummary {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    step_count: usize,
    present_steps: usize,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceOperatorStep {
    id: &'static str,
    label: &'static str,
    path: &'static str,
    command: &'static str,
    status: &'static str,
    present: bool,
    bytes: Option<u64>,
    required_inputs: Vec<&'static str>,
    stale_hint: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceStaleStepHint {
    step: &'static str,
    missing_path: &'static str,
    rerun_command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceOperatorIndexCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_operator_index(cwd: &Path, args: &[String]) -> DxResult<()> {
    let mut project = cwd.to_path_buf();
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 100u8;
    let mut write = false;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = required_arg(args, index, "--project")?;
                project = resolve_cli_path(cwd, value);
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--write" => {
                write = true;
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--dry-run" => {
                write = false;
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
                        "Unknown forge launch-evidence-operator-index option: {value}"
                    ),
                    field: Some("forge.launch-evidence-operator-index".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-operator-index argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-operator-index".to_string()),
                });
            }
        }
    }

    let report =
        build_launch_evidence_operator_index_report(&project, fail_under).map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => launch_evidence_operator_index_markdown(&report),
        DxOutputFormat::Terminal => launch_evidence_operator_index_terminal(&report),
    };

    if write && output.is_none() {
        output = Some(project.join(INDEX_PATH));
    }

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

    Ok(())
}

pub(crate) fn build_launch_evidence_operator_index_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceOperatorIndexReport> {
    let steps = operator_steps(project);
    let present_steps = steps.iter().filter(|step| step.present).count();
    let stale_step_hints = steps
        .iter()
        .filter(|step| !step.present)
        .map(|step| LaunchEvidenceStaleStepHint {
            step: step.id,
            missing_path: step.path,
            rerun_command: step.stale_hint,
        })
        .collect::<Vec<_>>();
    let starter_present = steps.iter().take(5).all(|step| step.present);
    let missing_steps = steps.iter().filter(|step| !step.present).count();
    let packet_present = steps
        .iter()
        .any(|step| step.id == "launch-evidence-packet" && step.present);
    let packet_hinted = stale_step_hints
        .iter()
        .any(|hint| hint.step == "launch-evidence-packet");
    let checks = vec![
        check(
            "starter-evidence-indexed",
            starter_present,
            format!("{present_steps}/{} launch evidence step(s) are present", steps.len()),
        ),
        check(
            "operator-sequence-indexed",
            steps.len() == 8,
            format!("{} launch evidence operator step(s) are indexed", steps.len()),
        ),
        check(
            "stale-step-hints",
            stale_step_hints.len() == missing_steps,
            format!("{} stale step hint(s) are available", stale_step_hints.len()),
        ),
        check(
            "packet-evidence-indexed",
            packet_present || packet_hinted,
            if packet_present {
                "launch evidence release packet is present".to_string()
            } else if packet_hinted {
                "launch evidence release packet is missing; rerun hint is available".to_string()
            } else {
                "launch evidence release packet is missing without a rerun hint".to_string()
            },
        ),
        check(
            "no-runtime-content-read",
            true,
            "operator index uses file metadata and expected commands only; it does not read runtime artifact contents".to_string(),
        ),
    ];
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .collect::<Vec<_>>();
    let score = average_score(&checks);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(LaunchEvidenceOperatorIndexReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        index: LaunchEvidenceOperatorIndexSummary {
            schema: INDEX_SCHEMA,
            path: INDEX_PATH,
            command: "dx forge launch-evidence-operator-index --project <path> --json",
            step_count: steps.len(),
            present_steps,
            reads_runtime_artifact_contents: false,
        },
        steps,
        stale_step_hints,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-runtime-evidence-review --project . --json --output .dx/forge/runtime/final-launch-evidence-review.json".to_string(),
            "dx forge launch-evidence-packet --project . --write --quiet".to_string(),
            "dx forge launch-evidence-operator-index --project . --write".to_string(),
            "dx forge launch-evidence-status-timeline --project . --write".to_string(),
        ],
    })
}

pub(crate) fn launch_evidence_operator_index_terminal(
    report: &LaunchEvidenceOperatorIndexReport,
) -> String {
    format!(
        "DX Forge launch evidence operator index\nProject: {}\nPassed: {}\nScore: {}\nSteps present: {}/{}\nStale hints: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.index.present_steps,
        report.index.step_count,
        report.stale_step_hints.len(),
        report.no_execution
    )
}

pub(crate) fn launch_evidence_operator_index_markdown(
    report: &LaunchEvidenceOperatorIndexReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Operator Index\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Steps present: `{}/{}`\n- Stale hints: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.index.present_steps,
        report.index.step_count,
        report.stale_step_hints.len(),
        report.no_execution
    );
    output.push_str("| Step | Present | Status | Command |\n| --- | --- | --- | --- |\n");
    for step in &report.steps {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` |\n",
            step.id,
            step.present,
            step.status,
            markdown_cell(step.command)
        ));
    }
    output
}

fn operator_steps(project: &Path) -> Vec<LaunchEvidenceOperatorStep> {
    [
        step(
            project,
            "template-readiness",
            "Verify generated /launch route readiness",
            TEMPLATE_READINESS_PATH,
            "dx forge template-readiness --project <path> --json",
            vec![],
            "dx forge template-readiness --project . --json --output .dx/forge/template-readiness/launch-route.json",
        ),
        step(
            project,
            "launch-readiness-bundle",
            "Aggregate launch readiness evidence",
            READINESS_BUNDLE_PATH,
            "dx forge launch-readiness-bundle --project <path> --json",
            vec![TEMPLATE_READINESS_PATH],
            "dx forge launch-readiness-bundle --project . --json --output .dx/forge/template-readiness/launch-readiness-bundle.json",
        ),
        step(
            project,
            "runtime-checklist",
            "Review governed runtime checklist",
            RUNTIME_CHECKLIST_PATH,
            "dx forge launch-runtime-checklist --project <path> --json",
            vec![READINESS_BUNDLE_PATH],
            "dx forge launch-runtime-checklist --project . --json --output .dx/forge/template-readiness/launch-runtime-checklist.json",
        ),
        step(
            project,
            "runtime-approval-request",
            "Record explicit runtime approval request",
            RUNTIME_APPROVAL_PATH,
            "dx forge launch-runtime-approval-request --project <path> --json",
            vec![RUNTIME_CHECKLIST_PATH],
            "dx forge launch-runtime-approval-request --project . --json --output .dx/forge/template-readiness/launch-runtime-approval-request.json",
        ),
        step(
            project,
            "runtime-evidence-contract",
            "Track required runtime evidence contract",
            RUNTIME_EVIDENCE_PATH,
            "dx forge launch-runtime-evidence --project <path> --json",
            vec![RUNTIME_APPROVAL_PATH],
            "dx forge launch-runtime-evidence --project . --json --output .dx/forge/template-readiness/launch-runtime-evidence.json",
        ),
        step(
            project,
            "final-launch-evidence-receipt",
            "Finalize approved runtime evidence",
            FINAL_RECEIPT_PATH,
            "dx forge launch-runtime-evidence-finalize --project <path> --import-plan <path> --write --json",
            vec![RUNTIME_EVIDENCE_PATH],
            "dx forge launch-runtime-evidence-finalize --project . --import-plan <path> --write --json",
        ),
        step(
            project,
            "final-runtime-review",
            "Review finalized runtime evidence",
            FINAL_REVIEW_PATH,
            "dx forge launch-runtime-evidence-review --project <path> --json",
            vec![FINAL_RECEIPT_PATH, RUNTIME_EVIDENCE_PATH],
            "dx forge launch-runtime-evidence-review --project . --json --output .dx/forge/runtime/final-launch-evidence-review.json",
        ),
        step(
            project,
            "launch-evidence-packet",
            "Create launch evidence release packet",
            PACKET_PATH,
            "dx forge launch-evidence-packet --project <path> --json",
            vec![READINESS_BUNDLE_PATH, FINAL_REVIEW_PATH],
            "dx forge launch-evidence-packet --project . --write",
        ),
    ]
    .into()
}

fn step(
    project: &Path,
    id: &'static str,
    label: &'static str,
    path: &'static str,
    command: &'static str,
    required_inputs: Vec<&'static str>,
    stale_hint: &'static str,
) -> LaunchEvidenceOperatorStep {
    let metadata = fs::metadata(project.join(path)).ok();
    let present = metadata.as_ref().is_some_and(|metadata| metadata.is_file());
    LaunchEvidenceOperatorStep {
        id,
        label,
        path,
        command,
        status: if present { "present" } else { "missing" },
        present,
        bytes: metadata.map(|metadata| metadata.len()),
        required_inputs,
        stale_hint,
    }
}

fn required_arg<'a>(args: &'a [String], index: usize, name: &str) -> DxResult<&'a str> {
    args.get(index + 1)
        .map(String::as_str)
        .ok_or_else(|| DxError::ConfigValidationError {
            message: format!("{name} requires a value"),
            field: Some("forge.launch-evidence-operator-index".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceOperatorIndexCheck {
    LaunchEvidenceOperatorIndexCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceOperatorIndexCheck]) -> u8 {
    if checks.is_empty() {
        return 0;
    }
    let total = checks.iter().map(|check| check.score as u16).sum::<u16>();
    (total / checks.len() as u16) as u8
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_stale_hints_without_reading_runtime_contents() {
        let dir = tempfile::tempdir().expect("tempdir");

        let report = build_launch_evidence_operator_index_report(dir.path(), 100).expect("report");

        assert!(!report.passed);
        assert!(report.no_execution);
        assert_eq!(report.index.step_count, 8);
        assert_eq!(report.index.present_steps, 0);
        assert!(!report.index.reads_runtime_artifact_contents);
        assert!(
            report
                .stale_step_hints
                .iter()
                .any(|hint| hint.step == "launch-evidence-packet"
                    && hint.missing_path == PACKET_PATH)
        );
    }

    #[test]
    fn keeps_partial_launch_sequence_actionable_until_packet_exists() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            TEMPLATE_READINESS_PATH,
            READINESS_BUNDLE_PATH,
            RUNTIME_CHECKLIST_PATH,
            RUNTIME_APPROVAL_PATH,
            RUNTIME_EVIDENCE_PATH,
            FINAL_RECEIPT_PATH,
            FINAL_REVIEW_PATH,
        ] {
            write_step(dir.path(), path);
        }

        let report = build_launch_evidence_operator_index_report(dir.path(), 100).expect("report");

        assert!(report.passed);
        assert_eq!(report.index.present_steps, 7);
        assert!(
            report
                .stale_step_hints
                .iter()
                .any(|hint| hint.step == "launch-evidence-packet"
                    && hint.rerun_command == "dx forge launch-evidence-packet --project . --write")
        );
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "packet-evidence-indexed" && check.passed)
        );
    }

    #[test]
    fn passes_when_full_launch_evidence_packet_is_indexed() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            TEMPLATE_READINESS_PATH,
            READINESS_BUNDLE_PATH,
            RUNTIME_CHECKLIST_PATH,
            RUNTIME_APPROVAL_PATH,
            RUNTIME_EVIDENCE_PATH,
            FINAL_RECEIPT_PATH,
            FINAL_REVIEW_PATH,
            PACKET_PATH,
        ] {
            write_step(dir.path(), path);
        }

        let report = build_launch_evidence_operator_index_report(dir.path(), 100).expect("report");

        assert!(report.passed);
        assert_eq!(report.index.present_steps, report.index.step_count);
        assert!(report.stale_step_hints.is_empty());
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "no-runtime-content-read" && check.passed)
        );
    }

    fn write_step(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("parent")).expect("step parent");
        fs::write(path, "{}").expect("step file");
    }
}
