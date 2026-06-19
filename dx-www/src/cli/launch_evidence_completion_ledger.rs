use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_completion_ledger";
const LEDGER_SCHEMA: &str = "dx.launch.evidence_completion_ledger";
const LEDGER_PATH: &str = ".dx/forge/release/launch-evidence-completion-ledger.json";
const SUMMARY_PATH: &str = ".dx/forge/release/launch-evidence-operator-summary.json";
const SEAL_PATH: &str = ".dx/forge/release/launch-evidence-release-seal.json";
const RETENTION_REVIEW_PATH: &str = ".dx/forge/release/launch-evidence-retention-review.json";
const FINAL_REVIEW_PATH: &str = ".dx/forge/runtime/final-launch-evidence-review.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceCompletionLedgerReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    ledger: LaunchEvidenceCompletionLedger,
    completion_artifacts: Vec<LaunchEvidenceCompletionLedgerArtifact>,
    freshness: LaunchEvidenceCompletionLedgerFreshness,
    checks: Vec<LaunchEvidenceCompletionLedgerCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceCompletionLedgerReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceCompletionLedger {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    #[serde(rename = "completion_target")]
    ledger_target: &'static str,
    artifact_count: usize,
    present_artifacts: usize,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceCompletionLedgerArtifact {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceCompletionLedgerFreshness {
    ledger_path: &'static str,
    ledger_present: bool,
    ledger_modified_at: Option<String>,
    operator_summary_present: bool,
    ledger_not_older_than_operator_summary: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceCompletionLedgerCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_completion_ledger(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-completion-ledger option: {value}"
                    ),
                    field: Some("forge.launch-evidence-completion-ledger".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-completion-ledger argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-completion-ledger".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(LEDGER_PATH));
    }

    let mut report = build_launch_evidence_completion_ledger_report(&project, fail_under)
        .map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_completion_ledger_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_completion_ledger(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_completion_ledger(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_completion_ledger_failure_summary(&report),
            field: Some("forge.launch-evidence-completion-ledger".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_completion_ledger(
    report: &LaunchEvidenceCompletionLedgerReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_completion_ledger_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_completion_ledger_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_completion_ledger_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceCompletionLedgerReport> {
    let completion_artifacts = completion_artifacts(project);
    let present_artifacts = completion_artifacts
        .iter()
        .filter(|artifact| artifact.present)
        .count();
    let all_artifacts_present = present_artifacts == completion_artifacts.len();
    let freshness = completion_ledger_freshness(project, &completion_artifacts);
    let checks = vec![
        check(
            "operator-summary-present",
            freshness.operator_summary_present,
            format!("operator summary exists at {SUMMARY_PATH}"),
        ),
        check(
            "completion-ledger-artifacts-present",
            all_artifacts_present,
            format!(
                "{present_artifacts}/{} completion ledger artifact(s) are present",
                completion_artifacts.len()
            ),
        ),
        check(
            "completion-ledger-freshness",
            freshness.ledger_not_older_than_operator_summary,
            "completion ledger is not older than the operator summary".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "completion ledger uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceCompletionLedgerReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        ledger: LaunchEvidenceCompletionLedger {
            schema: LEDGER_SCHEMA,
            path: LEDGER_PATH,
            command: "dx forge launch-evidence-completion-ledger --project <path> --write",
            ledger_target: "final-launch-evidence-completion-map",
            artifact_count: completion_artifacts.len(),
            present_artifacts,
            reads_runtime_artifact_contents: false,
        },
        completion_artifacts,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-operator-summary --project . --write".to_string(),
            "dx forge launch-evidence-completion-ledger --project . --write".to_string(),
            "dx forge launch-evidence-closure-memo --project . --write".to_string(),
            format!("open {LEDGER_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_completion_ledger_terminal(
    report: &LaunchEvidenceCompletionLedgerReport,
) -> String {
    format!(
        "DX Forge launch evidence completion ledger\nProject: {}\nPassed: {}\nScore: {}\nCompletion target: {}\nArtifacts: {}/{}\nLedger fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.ledger.ledger_target,
        report.ledger.present_artifacts,
        report.ledger.artifact_count,
        report.freshness.ledger_not_older_than_operator_summary,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_completion_ledger_markdown(
    report: &LaunchEvidenceCompletionLedgerReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Completion Ledger\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Completion target: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.ledger.ledger_target,
        report.no_execution
    );
    output.push_str("| Artifact | Present | Path |\n| --- | --- | --- |\n");
    for artifact in &report.completion_artifacts {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` |\n",
            artifact.id, artifact.present, artifact.path
        ));
    }
    output
}

pub(crate) fn launch_evidence_completion_ledger_failure_summary(
    report: &LaunchEvidenceCompletionLedgerReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence completion ledger score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn completion_artifacts(project: &Path) -> Vec<LaunchEvidenceCompletionLedgerArtifact> {
    [
        (
            "operator-summary",
            SUMMARY_PATH,
            "dx forge launch-evidence-operator-summary --project . --write",
        ),
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
            "final-runtime-review",
            FINAL_REVIEW_PATH,
            "dx forge launch-runtime-evidence-review --project . --json",
        ),
    ]
    .into_iter()
    .map(|(id, path, command)| completion_artifact(project, id, path, command))
    .collect()
}

fn completion_artifact(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceCompletionLedgerArtifact {
    let modified = file_modified_at(&project.join(path));
    LaunchEvidenceCompletionLedgerArtifact {
        id,
        path,
        present: modified.is_some(),
        modified_at: modified.map(format_system_time),
        command,
    }
}

fn completion_ledger_freshness(
    project: &Path,
    completion_artifacts: &[LaunchEvidenceCompletionLedgerArtifact],
) -> LaunchEvidenceCompletionLedgerFreshness {
    let ledger_modified = file_modified_at(&project.join(LEDGER_PATH));
    let summary_modified = file_modified_at(&project.join(SUMMARY_PATH));
    let ledger_not_older_than_operator_summary = match (ledger_modified, summary_modified) {
        (Some(ledger), Some(summary)) => ledger >= summary,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceCompletionLedgerFreshness {
        ledger_path: LEDGER_PATH,
        ledger_present: ledger_modified.is_some(),
        ledger_modified_at: ledger_modified.map(format_system_time),
        operator_summary_present: artifact_present(completion_artifacts, "operator-summary"),
        ledger_not_older_than_operator_summary,
        timestamp_source: "filesystem-metadata",
    }
}

fn artifact_present(artifacts: &[LaunchEvidenceCompletionLedgerArtifact], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-completion-ledger".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceCompletionLedgerCheck {
    LaunchEvidenceCompletionLedgerCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceCompletionLedgerCheck]) -> u8 {
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
    fn fails_when_operator_summary_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [SEAL_PATH, RETENTION_REVIEW_PATH, FINAL_REVIEW_PATH] {
            write_input(dir.path(), path);
        }

        let report =
            build_launch_evidence_completion_ledger_report(dir.path(), 100).expect("ledger");

        assert!(!report.passed());
        assert!(!report.freshness.operator_summary_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "operator-summary-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_completion_ledger_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), LEDGER_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), SUMMARY_PATH);

        let report = build_launch_evidence_completion_ledger_report(dir.path(), 0).expect("ledger");

        assert!(!report.freshness.ledger_not_older_than_operator_summary);
    }

    #[test]
    fn passes_complete_fresh_completion_ledger_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            SUMMARY_PATH,
            SEAL_PATH,
            RETENTION_REVIEW_PATH,
            FINAL_REVIEW_PATH,
        ] {
            write_input(dir.path(), path);
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), LEDGER_PATH);

        let report =
            build_launch_evidence_completion_ledger_report(dir.path(), 100).expect("ledger");

        assert!(report.passed());
        assert_eq!(
            report.ledger.ledger_target,
            "final-launch-evidence-completion-map"
        );
        assert!(!report.ledger.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_completion_ledger() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            SUMMARY_PATH,
            SEAL_PATH,
            RETENTION_REVIEW_PATH,
            FINAL_REVIEW_PATH,
        ] {
            write_input(dir.path(), path);
        }

        run_launch_evidence_completion_ledger(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write completion ledger");

        let report =
            build_launch_evidence_completion_ledger_report(dir.path(), 100).expect("ledger");
        assert!(report.passed());
        assert!(dir.path().join(LEDGER_PATH).is_file());
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("input parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
