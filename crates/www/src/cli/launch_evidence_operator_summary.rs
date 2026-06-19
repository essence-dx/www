use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_operator_summary";
const SUMMARY_SCHEMA: &str = "dx.launch.evidence_operator_summary";
const SUMMARY_PATH: &str = ".dx/forge/release/launch-evidence-operator-summary.json";
const SEAL_PATH: &str = ".dx/forge/release/launch-evidence-release-seal.json";
const RETENTION_REVIEW_PATH: &str = ".dx/forge/release/launch-evidence-retention-review.json";
const PACKET_PATH: &str = ".dx/forge/release/launch-evidence-packet.json";
const FINAL_REVIEW_PATH: &str = ".dx/forge/runtime/final-launch-evidence-review.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceOperatorSummaryReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    summary: LaunchEvidenceOperatorSummary,
    source_artifacts: Vec<LaunchEvidenceOperatorSummaryArtifact>,
    freshness: LaunchEvidenceOperatorSummaryFreshness,
    checks: Vec<LaunchEvidenceOperatorSummaryCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceOperatorSummaryReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceOperatorSummary {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    summary_target: &'static str,
    artifact_count: usize,
    present_artifacts: usize,
    terminal_friendly: bool,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceOperatorSummaryArtifact {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceOperatorSummaryFreshness {
    summary_path: &'static str,
    summary_present: bool,
    summary_modified_at: Option<String>,
    release_seal_present: bool,
    summary_not_older_than_release_seal: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceOperatorSummaryCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_operator_summary(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-operator-summary option: {value}"
                    ),
                    field: Some("forge.launch-evidence-operator-summary".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-operator-summary argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-operator-summary".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(SUMMARY_PATH));
    }

    let mut report =
        build_launch_evidence_operator_summary_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_operator_summary_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_operator_summary(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_operator_summary(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_operator_summary_failure_summary(&report),
            field: Some("forge.launch-evidence-operator-summary".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_operator_summary(
    report: &LaunchEvidenceOperatorSummaryReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_operator_summary_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_operator_summary_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_operator_summary_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceOperatorSummaryReport> {
    let source_artifacts = summary_artifacts(project);
    let present_artifacts = source_artifacts
        .iter()
        .filter(|artifact| artifact.present)
        .count();
    let all_artifacts_present = present_artifacts == source_artifacts.len();
    let freshness = operator_summary_freshness(project, &source_artifacts);
    let checks = vec![
        check(
            "release-seal-present",
            freshness.release_seal_present,
            format!("release seal exists at {SEAL_PATH}"),
        ),
        check(
            "operator-summary-artifacts-present",
            all_artifacts_present,
            format!(
                "{present_artifacts}/{} operator summary artifact(s) are present",
                source_artifacts.len()
            ),
        ),
        check(
            "operator-summary-freshness",
            freshness.summary_not_older_than_release_seal,
            "operator summary is not older than the release seal".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "operator summary uses file metadata only; it does not read runtime artifact contents"
                .to_string(),
        ),
    ];
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .collect::<Vec<_>>();
    let score = average_score(&checks);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(LaunchEvidenceOperatorSummaryReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        summary: LaunchEvidenceOperatorSummary {
            schema: SUMMARY_SCHEMA,
            path: SUMMARY_PATH,
            command: "dx forge launch-evidence-operator-summary --project <path> --write",
            summary_target: "terminal-friendly-launch-handoff",
            artifact_count: source_artifacts.len(),
            present_artifacts,
            terminal_friendly: true,
            reads_runtime_artifact_contents: false,
        },
        source_artifacts,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-release-seal --project . --write".to_string(),
            "dx forge launch-evidence-operator-summary --project . --write".to_string(),
            "dx forge launch-evidence-completion-ledger --project . --write".to_string(),
            format!("open {SUMMARY_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_operator_summary_terminal(
    report: &LaunchEvidenceOperatorSummaryReport,
) -> String {
    format!(
        "DX Forge launch evidence operator summary\nProject: {}\nPassed: {}\nScore: {}\nSummary target: {}\nArtifacts: {}/{}\nSummary fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.summary.summary_target,
        report.summary.present_artifacts,
        report.summary.artifact_count,
        report.freshness.summary_not_older_than_release_seal,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_operator_summary_markdown(
    report: &LaunchEvidenceOperatorSummaryReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Operator Summary\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Summary target: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.summary.summary_target,
        report.no_execution
    );
    output.push_str("| Artifact | Present | Path |\n| --- | --- | --- |\n");
    for artifact in &report.source_artifacts {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` |\n",
            artifact.id, artifact.present, artifact.path
        ));
    }
    output
}

pub(crate) fn launch_evidence_operator_summary_failure_summary(
    report: &LaunchEvidenceOperatorSummaryReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence operator summary score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn summary_artifacts(project: &Path) -> Vec<LaunchEvidenceOperatorSummaryArtifact> {
    [
        (
            "release-seal",
            SEAL_PATH,
            "dx forge launch-evidence-release-seal --project . --write",
        ),
        (
            "retention-review",
            RETENTION_REVIEW_PATH,
            "dx forge launch-evidence-retention-review --project . --write",
        ),
        (
            "release-packet",
            PACKET_PATH,
            "dx forge launch-evidence-packet --project . --write",
        ),
        (
            "final-runtime-review",
            FINAL_REVIEW_PATH,
            "dx forge launch-runtime-evidence-review --project . --json",
        ),
    ]
    .into_iter()
    .map(|(id, path, command)| summary_artifact(project, id, path, command))
    .collect()
}

fn summary_artifact(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceOperatorSummaryArtifact {
    let modified = file_modified_at(&project.join(path));
    LaunchEvidenceOperatorSummaryArtifact {
        id,
        path,
        present: modified.is_some(),
        modified_at: modified.map(format_system_time),
        command,
    }
}

fn operator_summary_freshness(
    project: &Path,
    source_artifacts: &[LaunchEvidenceOperatorSummaryArtifact],
) -> LaunchEvidenceOperatorSummaryFreshness {
    let summary_modified = file_modified_at(&project.join(SUMMARY_PATH));
    let seal_modified = file_modified_at(&project.join(SEAL_PATH));
    let summary_not_older_than_release_seal = match (summary_modified, seal_modified) {
        (Some(summary), Some(seal)) => summary >= seal,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceOperatorSummaryFreshness {
        summary_path: SUMMARY_PATH,
        summary_present: summary_modified.is_some(),
        summary_modified_at: summary_modified.map(format_system_time),
        release_seal_present: artifact_present(source_artifacts, "release-seal"),
        summary_not_older_than_release_seal,
        timestamp_source: "filesystem-metadata",
    }
}

fn artifact_present(artifacts: &[LaunchEvidenceOperatorSummaryArtifact], id: &str) -> bool {
    artifacts
        .iter()
        .any(|artifact| artifact.id == id && artifact.present)
}

fn file_modified_at(path: &Path) -> Option<SystemTime> {
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

fn required_arg<'a>(args: &'a [String], index: usize, name: &str) -> DxResult<&'a str> {
    args.get(index + 1)
        .map(String::as_str)
        .ok_or_else(|| DxError::ConfigValidationError {
            message: format!("{name} requires a value"),
            field: Some("forge.launch-evidence-operator-summary".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceOperatorSummaryCheck {
    LaunchEvidenceOperatorSummaryCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceOperatorSummaryCheck]) -> u8 {
    if checks.is_empty() {
        return 0;
    }
    let total = checks.iter().map(|check| check.score as u16).sum::<u16>();
    (total / checks.len() as u16) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fails_when_release_seal_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [RETENTION_REVIEW_PATH, PACKET_PATH, FINAL_REVIEW_PATH] {
            write_input(dir.path(), path);
        }

        let report =
            build_launch_evidence_operator_summary_report(dir.path(), 100).expect("summary");

        assert!(!report.passed());
        assert!(!report.freshness.release_seal_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "release-seal-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_operator_summary_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), SUMMARY_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), SEAL_PATH);

        let report = build_launch_evidence_operator_summary_report(dir.path(), 0).expect("summary");

        assert!(!report.freshness.summary_not_older_than_release_seal);
    }

    #[test]
    fn passes_complete_fresh_operator_summary_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            SEAL_PATH,
            RETENTION_REVIEW_PATH,
            PACKET_PATH,
            FINAL_REVIEW_PATH,
        ] {
            write_input(dir.path(), path);
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), SUMMARY_PATH);

        let report =
            build_launch_evidence_operator_summary_report(dir.path(), 100).expect("summary");

        assert!(report.passed());
        assert_eq!(
            report.summary.summary_target,
            "terminal-friendly-launch-handoff"
        );
        assert!(report.summary.terminal_friendly);
        assert!(!report.summary.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_operator_summary() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            SEAL_PATH,
            RETENTION_REVIEW_PATH,
            PACKET_PATH,
            FINAL_REVIEW_PATH,
        ] {
            write_input(dir.path(), path);
        }

        run_launch_evidence_operator_summary(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write operator summary");

        let report =
            build_launch_evidence_operator_summary_report(dir.path(), 100).expect("summary");
        assert!(report.passed());
        assert!(dir.path().join(SUMMARY_PATH).is_file());
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("input parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
