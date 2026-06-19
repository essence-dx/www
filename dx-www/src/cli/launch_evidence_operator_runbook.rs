use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_operator_runbook";
const RUNBOOK_SCHEMA: &str = "dx.launch.evidence_operator_runbook";
const RUNBOOK_PATH: &str = ".dx/forge/release/launch-evidence-operator-runbook.json";
const FINAL_BRIEF_PATH: &str = ".dx/forge/release/launch-evidence-final-brief.json";
const CLOSURE_MEMO_PATH: &str = ".dx/forge/release/launch-evidence-closure-memo.md";
const COMPLETION_LEDGER_PATH: &str = ".dx/forge/release/launch-evidence-completion-ledger.json";
const OPERATOR_SUMMARY_PATH: &str = ".dx/forge/release/launch-evidence-operator-summary.json";
const RELEASE_SEAL_PATH: &str = ".dx/forge/release/launch-evidence-release-seal.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceOperatorRunbookReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    runbook: LaunchEvidenceOperatorRunbook,
    restart_artifacts: Vec<LaunchEvidenceOperatorRunbookArtifact>,
    freshness: LaunchEvidenceOperatorRunbookFreshness,
    checks: Vec<LaunchEvidenceOperatorRunbookCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceOperatorRunbookReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceOperatorRunbook {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    runbook_target: &'static str,
    restart_steps: Vec<LaunchEvidenceOperatorRunbookStep>,
    artifact_count: usize,
    present_artifacts: usize,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceOperatorRunbookStep {
    id: &'static str,
    label: &'static str,
    command: &'static str,
    evidence: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceOperatorRunbookArtifact {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceOperatorRunbookFreshness {
    runbook_path: &'static str,
    runbook_present: bool,
    runbook_modified_at: Option<String>,
    final_brief_present: bool,
    runbook_not_older_than_final_brief: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceOperatorRunbookCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_operator_runbook(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-operator-runbook option: {value}"
                    ),
                    field: Some("forge.launch-evidence-operator-runbook".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-operator-runbook argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-operator-runbook".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(RUNBOOK_PATH));
    }

    let mut report =
        build_launch_evidence_operator_runbook_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_operator_runbook_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_operator_runbook(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_operator_runbook(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_operator_runbook_failure_summary(&report),
            field: Some("forge.launch-evidence-operator-runbook".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_operator_runbook(
    report: &LaunchEvidenceOperatorRunbookReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_operator_runbook_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_operator_runbook_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_operator_runbook_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceOperatorRunbookReport> {
    let restart_artifacts = operator_runbook_artifacts(project);
    let present_artifacts = restart_artifacts
        .iter()
        .filter(|artifact| artifact.present)
        .count();
    let all_artifacts_present = present_artifacts == restart_artifacts.len();
    let restart_steps = operator_runbook_steps();
    let freshness = operator_runbook_freshness(project, &restart_artifacts);
    let checks = vec![
        check(
            "final-brief-present",
            freshness.final_brief_present,
            format!("final brief exists at {FINAL_BRIEF_PATH}"),
        ),
        check(
            "operator-runbook-artifacts-present",
            all_artifacts_present,
            format!(
                "{present_artifacts}/{} operator runbook artifact(s) are present",
                restart_artifacts.len()
            ),
        ),
        check(
            "operator-runbook-freshness",
            freshness.runbook_not_older_than_final_brief,
            "operator runbook is not older than the final brief".to_string(),
        ),
        check(
            "restartable-dx-worker-checklist",
            !restart_steps.is_empty(),
            "operator runbook includes restartable DX worker checklist steps".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "operator runbook uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceOperatorRunbookReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        runbook: LaunchEvidenceOperatorRunbook {
            schema: RUNBOOK_SCHEMA,
            path: RUNBOOK_PATH,
            command: "dx forge launch-evidence-operator-runbook --project <path> --write",
            runbook_target: "restartable-dx-worker-checklist",
            restart_steps,
            artifact_count: restart_artifacts.len(),
            present_artifacts,
            reads_runtime_artifact_contents: false,
        },
        restart_artifacts,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-final-brief --project . --write".to_string(),
            "dx forge launch-evidence-operator-runbook --project . --write".to_string(),
            "dx forge launch-evidence-handoff-capsule --project . --write".to_string(),
            format!("open {RUNBOOK_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_operator_runbook_terminal(
    report: &LaunchEvidenceOperatorRunbookReport,
) -> String {
    format!(
        "DX Forge launch evidence operator runbook\nProject: {}\nPassed: {}\nScore: {}\nRunbook target: {}\nArtifacts: {}/{}\nRunbook fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.runbook.runbook_target,
        report.runbook.present_artifacts,
        report.runbook.artifact_count,
        report.freshness.runbook_not_older_than_final_brief,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_operator_runbook_markdown(
    report: &LaunchEvidenceOperatorRunbookReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Operator Runbook\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Runbook target: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.runbook.runbook_target,
        report.no_execution
    );
    output.push_str("## Restart Steps\n\n");
    for step in &report.runbook.restart_steps {
        output.push_str(&format!(
            "- `{}`: {} (`{}` -> `{}`)\n",
            step.id, step.label, step.command, step.evidence
        ));
    }
    output.push_str(
        "\n## Restart Artifacts\n\n| Artifact | Present | Modified | Bytes | Path |\n| --- | --- | --- | --- | --- |\n",
    );
    for artifact in &report.restart_artifacts {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            artifact.id,
            artifact.present,
            artifact.modified_at.as_deref().unwrap_or("missing"),
            artifact
                .bytes
                .map(|bytes| bytes.to_string())
                .unwrap_or_else(|| "missing".to_string()),
            artifact.path
        ));
    }
    output
}

pub(crate) fn launch_evidence_operator_runbook_failure_summary(
    report: &LaunchEvidenceOperatorRunbookReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence operator runbook score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn operator_runbook_artifacts(project: &Path) -> Vec<LaunchEvidenceOperatorRunbookArtifact> {
    [
        (
            "final-brief",
            FINAL_BRIEF_PATH,
            "dx forge launch-evidence-final-brief --project . --write",
        ),
        (
            "closure-memo",
            CLOSURE_MEMO_PATH,
            "dx forge launch-evidence-closure-memo --project . --write",
        ),
        (
            "completion-ledger",
            COMPLETION_LEDGER_PATH,
            "dx forge launch-evidence-completion-ledger --project . --write",
        ),
        (
            "operator-summary",
            OPERATOR_SUMMARY_PATH,
            "dx forge launch-evidence-operator-summary --project . --write",
        ),
        (
            "release-seal",
            RELEASE_SEAL_PATH,
            "dx forge launch-evidence-release-seal --project . --write",
        ),
    ]
    .into_iter()
    .map(|(id, path, command)| operator_runbook_artifact(project, id, path, command))
    .collect()
}

fn operator_runbook_artifact(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceOperatorRunbookArtifact {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceOperatorRunbookArtifact {
        id,
        path,
        present: metadata.is_some(),
        modified_at: metadata
            .as_ref()
            .map(|metadata| format_system_time(metadata.modified_at)),
        bytes: metadata.map(|metadata| metadata.bytes),
        command,
    }
}

fn operator_runbook_steps() -> Vec<LaunchEvidenceOperatorRunbookStep> {
    vec![
        LaunchEvidenceOperatorRunbookStep {
            id: "review-final-brief",
            label: "Review the final closeout pointer",
            command: "dx forge launch-evidence-final-brief --project . --write",
            evidence: FINAL_BRIEF_PATH,
        },
        LaunchEvidenceOperatorRunbookStep {
            id: "open-closure-memo",
            label: "Open the human-readable closure memo",
            command: "open .dx/forge/release/launch-evidence-closure-memo.md",
            evidence: CLOSURE_MEMO_PATH,
        },
        LaunchEvidenceOperatorRunbookStep {
            id: "confirm-completion-ledger",
            label: "Confirm the launch evidence completion ledger",
            command: "dx forge launch-evidence-completion-ledger --project . --write",
            evidence: COMPLETION_LEDGER_PATH,
        },
        LaunchEvidenceOperatorRunbookStep {
            id: "review-operator-summary",
            label: "Review the terminal-friendly operator summary",
            command: "dx forge launch-evidence-operator-summary --project . --write",
            evidence: OPERATOR_SUMMARY_PATH,
        },
        LaunchEvidenceOperatorRunbookStep {
            id: "confirm-release-seal",
            label: "Confirm the release seal before continuing",
            command: "dx forge launch-evidence-release-seal --project . --write",
            evidence: RELEASE_SEAL_PATH,
        },
    ]
}

fn operator_runbook_freshness(
    project: &Path,
    restart_artifacts: &[LaunchEvidenceOperatorRunbookArtifact],
) -> LaunchEvidenceOperatorRunbookFreshness {
    let runbook_modified =
        file_metadata(&project.join(RUNBOOK_PATH)).map(|metadata| metadata.modified_at);
    let final_brief_modified =
        file_metadata(&project.join(FINAL_BRIEF_PATH)).map(|metadata| metadata.modified_at);
    let runbook_not_older_than_final_brief = match (runbook_modified, final_brief_modified) {
        (Some(runbook), Some(final_brief)) => runbook >= final_brief,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceOperatorRunbookFreshness {
        runbook_path: RUNBOOK_PATH,
        runbook_present: runbook_modified.is_some(),
        runbook_modified_at: runbook_modified.map(format_system_time),
        final_brief_present: artifact_present(restart_artifacts, "final-brief"),
        runbook_not_older_than_final_brief,
        timestamp_source: "filesystem-metadata",
    }
}

fn artifact_present(artifacts: &[LaunchEvidenceOperatorRunbookArtifact], id: &str) -> bool {
    artifacts
        .iter()
        .any(|artifact| artifact.id == id && artifact.present)
}

struct FileMetadata {
    modified_at: SystemTime,
    bytes: u64,
}

fn file_metadata(path: &Path) -> Option<FileMetadata> {
    let metadata = fs::metadata(path).ok()?;
    if !metadata.is_file() {
        return None;
    }
    Some(FileMetadata {
        modified_at: metadata.modified().ok()?,
        bytes: metadata.len(),
    })
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
            field: Some("forge.launch-evidence-operator-runbook".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceOperatorRunbookCheck {
    LaunchEvidenceOperatorRunbookCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceOperatorRunbookCheck]) -> u8 {
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
    fn fails_when_final_brief_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            CLOSURE_MEMO_PATH,
            COMPLETION_LEDGER_PATH,
            OPERATOR_SUMMARY_PATH,
            RELEASE_SEAL_PATH,
        ] {
            write_input(dir.path(), path);
        }

        let report =
            build_launch_evidence_operator_runbook_report(dir.path(), 100).expect("runbook");

        assert!(!report.passed());
        assert!(!report.freshness.final_brief_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "final-brief-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_operator_runbook_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), RUNBOOK_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), FINAL_BRIEF_PATH);

        let report = build_launch_evidence_operator_runbook_report(dir.path(), 0).expect("runbook");

        assert!(!report.freshness.runbook_not_older_than_final_brief);
    }

    #[test]
    fn passes_complete_fresh_operator_runbook_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_operator_runbook_inputs(dir.path());
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), RUNBOOK_PATH);

        let report =
            build_launch_evidence_operator_runbook_report(dir.path(), 100).expect("runbook");

        assert!(report.passed());
        assert_eq!(
            report.runbook.runbook_target,
            "restartable-dx-worker-checklist"
        );
        assert_eq!(report.runbook.restart_steps.len(), 5);
        assert!(!report.runbook.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_operator_runbook() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_operator_runbook_inputs(dir.path());

        run_launch_evidence_operator_runbook(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write runbook");

        let report =
            build_launch_evidence_operator_runbook_report(dir.path(), 100).expect("runbook");
        assert!(report.passed());
        assert!(dir.path().join(RUNBOOK_PATH).is_file());
    }

    fn write_operator_runbook_inputs(project: &Path) {
        for path in [
            FINAL_BRIEF_PATH,
            CLOSURE_MEMO_PATH,
            COMPLETION_LEDGER_PATH,
            OPERATOR_SUMMARY_PATH,
            RELEASE_SEAL_PATH,
        ] {
            write_input(project, path);
        }
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("input parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
