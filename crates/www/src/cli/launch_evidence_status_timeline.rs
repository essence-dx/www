use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_status_timeline";
const TIMELINE_SCHEMA: &str = "dx.launch.evidence_status_timeline";
const TIMELINE_PATH: &str = ".dx/forge/release/launch-evidence-status-timeline.json";
const TEMPLATE_READINESS_PATH: &str = ".dx/forge/template-readiness/launch-route.json";
const READINESS_BUNDLE_PATH: &str = ".dx/forge/template-readiness/launch-readiness-bundle.json";
const RUNTIME_CHECKLIST_PATH: &str = ".dx/forge/template-readiness/launch-runtime-checklist.json";
const RUNTIME_APPROVAL_PATH: &str =
    ".dx/forge/template-readiness/launch-runtime-approval-request.json";
const RUNTIME_EVIDENCE_PATH: &str = ".dx/forge/template-readiness/launch-runtime-evidence.json";
const FINAL_RECEIPT_PATH: &str = ".dx/forge/runtime/final-launch-evidence-receipt.json";
const FINAL_REVIEW_PATH: &str = ".dx/forge/runtime/final-launch-evidence-review.json";
const PACKET_PATH: &str = ".dx/forge/release/launch-evidence-packet.json";
const OPERATOR_INDEX_PATH: &str = ".dx/forge/release/launch-evidence-operator-index.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceStatusTimelineReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    timeline: LaunchEvidenceTimelineSummary,
    freshness: LaunchEvidenceTimelineFreshness,
    steps: Vec<LaunchEvidenceTimelineStep>,
    checks: Vec<LaunchEvidenceTimelineCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceStatusTimelineReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceTimelineSummary {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    latest_completed_step: Option<&'static str>,
    next_blocked_step: Option<&'static str>,
    release_ready: bool,
    step_count: usize,
    present_steps: usize,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceTimelineFreshness {
    operator_index_path: &'static str,
    operator_index_present: bool,
    operator_index_modified_at: Option<String>,
    packet_path: &'static str,
    packet_present: bool,
    packet_modified_at: Option<String>,
    operator_index_not_older_than_packet: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceTimelineStep {
    id: &'static str,
    path: &'static str,
    command: &'static str,
    present: bool,
    modified_at: Option<String>,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceTimelineCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_status_timeline(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-status-timeline option: {value}"
                    ),
                    field: Some("forge.launch-evidence-status-timeline".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-status-timeline argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-status-timeline".to_string()),
                });
            }
        }
    }

    let report =
        build_launch_evidence_status_timeline_report(&project, fail_under).map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => launch_evidence_status_timeline_markdown(&report),
        DxOutputFormat::Terminal => launch_evidence_status_timeline_terminal(&report),
    };

    if write && output.is_none() {
        output = Some(project.join(TIMELINE_PATH));
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

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_status_timeline_failure_summary(&report),
            field: Some("forge.launch-evidence-status-timeline".to_string()),
        });
    }

    Ok(())
}

pub(crate) fn build_launch_evidence_status_timeline_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceStatusTimelineReport> {
    let steps = timeline_steps(project);
    let present_steps = steps.iter().filter(|step| step.present).count();
    let next_blocked_index = steps.iter().position(|step| !step.present);
    let latest_completed_step = match next_blocked_index {
        Some(0) => None,
        Some(index) => steps.get(index - 1).map(|step| step.id),
        None => steps.last().map(|step| step.id),
    };
    let next_blocked_step =
        next_blocked_index.and_then(|index| steps.get(index).map(|step| step.id));
    let release_ready = next_blocked_index.is_none();
    let freshness = timeline_freshness(project);
    let operator_index_current =
        !freshness.packet_present || freshness.operator_index_not_older_than_packet;
    let checks = Vec::from([
        check(
            "timeline-sequence-indexed",
            steps.len() == 8,
            format!(
                "{} launch evidence timeline step(s) are indexed",
                steps.len()
            ),
        ),
        check(
            "latest-step-detected",
            latest_completed_step.is_some() || present_steps == 0,
            "latest completed step is derived from file metadata".to_string(),
        ),
        check(
            "next-blocked-step-detected",
            next_blocked_step.is_some() || release_ready,
            "next blocked step is derived from file metadata".to_string(),
        ),
        check(
            "freshness-metadata",
            !release_ready || freshness.operator_index_present,
            "timeline freshness uses operator-index and packet file timestamps".to_string(),
        ),
        check(
            "operator-index-current",
            operator_index_current,
            if operator_index_current {
                "operator index timestamp is current for the release packet".to_string()
            } else {
                "operator index is missing or older than the release packet".to_string()
            },
        ),
        check(
            "release-ready",
            release_ready,
            "all launch evidence timeline steps are present".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "status timeline uses file metadata only; it does not read runtime artifact contents"
                .to_string(),
        ),
    ]);
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .collect::<Vec<_>>();
    let score = average_score(&checks);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(LaunchEvidenceStatusTimelineReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        timeline: LaunchEvidenceTimelineSummary {
            schema: TIMELINE_SCHEMA,
            path: TIMELINE_PATH,
            command: "dx forge launch-evidence-status-timeline --project <path> --json",
            latest_completed_step,
            next_blocked_step,
            release_ready,
            step_count: steps.len(),
            present_steps,
            reads_runtime_artifact_contents: false,
        },
        freshness,
        steps,
        checks,
        findings,
        next_commands: next_commands(next_blocked_step, release_ready),
    })
}

pub(crate) fn launch_evidence_status_timeline_terminal(
    report: &LaunchEvidenceStatusTimelineReport,
) -> String {
    format!(
        "DX Forge launch evidence status timeline\nProject: {}\nPassed: {}\nScore: {}\nLatest step: {}\nNext blocked: {}\nRelease ready: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.timeline.latest_completed_step.unwrap_or("none"),
        report.timeline.next_blocked_step.unwrap_or("none"),
        report.timeline.release_ready,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_status_timeline_markdown(
    report: &LaunchEvidenceStatusTimelineReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Status Timeline\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Latest step: `{}`\n- Next blocked: `{}`\n- Release ready: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.timeline.latest_completed_step.unwrap_or("none"),
        report.timeline.next_blocked_step.unwrap_or("none"),
        report.timeline.release_ready,
        report.no_execution
    );
    output.push_str("| Step | Present | Modified | Command |\n| --- | --- | --- | --- |\n");
    for step in &report.steps {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` |\n",
            step.id,
            step.present,
            step.modified_at.as_deref().unwrap_or("missing"),
            markdown_cell(step.command)
        ));
    }
    output
}

pub(crate) fn launch_evidence_status_timeline_failure_summary(
    report: &LaunchEvidenceStatusTimelineReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence status timeline score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn timeline_steps(project: &Path) -> Vec<LaunchEvidenceTimelineStep> {
    [
        step(
            project,
            "template-readiness",
            TEMPLATE_READINESS_PATH,
            "dx forge template-readiness --project <path> --json",
        ),
        step(
            project,
            "launch-readiness-bundle",
            READINESS_BUNDLE_PATH,
            "dx forge launch-readiness-bundle --project <path> --json",
        ),
        step(
            project,
            "runtime-checklist",
            RUNTIME_CHECKLIST_PATH,
            "dx forge launch-runtime-checklist --project <path> --json",
        ),
        step(
            project,
            "runtime-approval-request",
            RUNTIME_APPROVAL_PATH,
            "dx forge launch-runtime-approval-request --project <path> --json",
        ),
        step(
            project,
            "runtime-evidence-contract",
            RUNTIME_EVIDENCE_PATH,
            "dx forge launch-runtime-evidence --project <path> --json",
        ),
        step(
            project,
            "final-launch-evidence-receipt",
            FINAL_RECEIPT_PATH,
            "dx forge launch-runtime-evidence-finalize --project <path> --import-plan <path> --write --json",
        ),
        step(
            project,
            "final-runtime-review",
            FINAL_REVIEW_PATH,
            "dx forge launch-runtime-evidence-review --project <path> --json",
        ),
        step(
            project,
            "launch-evidence-packet",
            PACKET_PATH,
            "dx forge launch-evidence-packet --project <path> --json",
        ),
    ]
    .into()
}

fn step(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceTimelineStep {
    let modified = modified_at(project.join(path).as_path());
    LaunchEvidenceTimelineStep {
        id,
        path,
        command,
        present: modified.is_some(),
        modified_at: modified.map(format_system_time),
    }
}

fn timeline_freshness(project: &Path) -> LaunchEvidenceTimelineFreshness {
    let operator_index_modified = modified_at(&project.join(OPERATOR_INDEX_PATH));
    let packet_modified = modified_at(&project.join(PACKET_PATH));
    let operator_index_not_older_than_packet = match (operator_index_modified, packet_modified) {
        (Some(operator_index), Some(packet)) => operator_index >= packet,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceTimelineFreshness {
        operator_index_path: OPERATOR_INDEX_PATH,
        operator_index_present: operator_index_modified.is_some(),
        operator_index_modified_at: operator_index_modified.map(format_system_time),
        packet_path: PACKET_PATH,
        packet_present: packet_modified.is_some(),
        packet_modified_at: packet_modified.map(format_system_time),
        operator_index_not_older_than_packet,
        timestamp_source: "filesystem-metadata",
    }
}

fn modified_at(path: &Path) -> Option<SystemTime> {
    let metadata = fs::metadata(path).ok()?;
    if !metadata.is_file() {
        return None;
    }
    metadata.modified().ok()
}

fn format_system_time(time: SystemTime) -> String {
    let datetime: DateTime<Utc> = time.into();
    datetime.to_rfc3339()
}

fn next_commands(next_blocked_step: Option<&str>, release_ready: bool) -> Vec<String> {
    if release_ready {
        return vec![
            "dx forge launch-evidence-status-timeline --project . --write".to_string(),
            "dx forge launch-evidence-operator-index --project . --write".to_string(),
            "open .dx/forge/release/launch-evidence-packet.json".to_string(),
        ];
    }

    match next_blocked_step {
        Some("final-runtime-review") => {
            vec![
                "dx forge launch-runtime-evidence-review --project . --json --output .dx/forge/runtime/final-launch-evidence-review.json".to_string(),
            ]
        }
        Some("launch-evidence-packet") => {
            vec!["dx forge launch-evidence-packet --project . --write".to_string()]
        }
        Some(_) => vec!["dx forge launch-evidence-operator-index --project . --write".to_string()],
        None => Vec::new(),
    }
}

fn required_arg<'a>(args: &'a [String], index: usize, name: &str) -> DxResult<&'a str> {
    args.get(index + 1)
        .map(String::as_str)
        .ok_or_else(|| DxError::ConfigValidationError {
            message: format!("{name} requires a value"),
            field: Some("forge.launch-evidence-status-timeline".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceTimelineCheck {
    LaunchEvidenceTimelineCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceTimelineCheck]) -> u8 {
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
    fn reports_empty_timeline_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");

        let report = build_launch_evidence_status_timeline_report(dir.path(), 0).expect("timeline");

        assert!(!report.timeline.release_ready);
        assert_eq!(report.timeline.latest_completed_step, None);
        assert_eq!(
            report.timeline.next_blocked_step,
            Some("template-readiness")
        );
        assert!(!report.timeline.reads_runtime_artifact_contents);
    }

    #[test]
    fn reports_partial_timeline_next_blocked_step() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            TEMPLATE_READINESS_PATH,
            READINESS_BUNDLE_PATH,
            RUNTIME_CHECKLIST_PATH,
            RUNTIME_APPROVAL_PATH,
            RUNTIME_EVIDENCE_PATH,
            FINAL_RECEIPT_PATH,
        ] {
            write_step(dir.path(), path);
        }

        let report = build_launch_evidence_status_timeline_report(dir.path(), 0).expect("timeline");

        assert_eq!(
            report.timeline.latest_completed_step,
            Some("final-launch-evidence-receipt")
        );
        assert_eq!(
            report.timeline.next_blocked_step,
            Some("final-runtime-review")
        );
        assert!(
            report
                .next_commands
                .iter()
                .any(|command| command.contains("launch-runtime-evidence-review"))
        );
    }

    #[test]
    fn passes_packeted_timeline_with_freshness_metadata() {
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
            OPERATOR_INDEX_PATH,
        ] {
            write_step(dir.path(), path);
        }

        let report =
            build_launch_evidence_status_timeline_report(dir.path(), 100).expect("timeline");

        assert!(report.passed());
        assert!(report.timeline.release_ready);
        assert_eq!(
            report.timeline.latest_completed_step,
            Some("launch-evidence-packet")
        );
        assert_eq!(report.timeline.next_blocked_step, None);
        assert!(report.freshness.operator_index_present);
        assert!(report.freshness.packet_present);
    }

    #[test]
    fn reports_stale_operator_index_timestamp() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_step(dir.path(), OPERATOR_INDEX_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_step(dir.path(), PACKET_PATH);

        let report = build_launch_evidence_status_timeline_report(dir.path(), 0).expect("timeline");

        assert!(!report.freshness.operator_index_not_older_than_packet);
    }

    fn write_step(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("parent")).expect("step parent");
        fs::write(path, "{}").expect("step file");
    }
}
