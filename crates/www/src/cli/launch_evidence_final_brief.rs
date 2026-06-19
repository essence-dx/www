use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_final_brief";
const BRIEF_SCHEMA: &str = "dx.launch.evidence_final_brief";
const BRIEF_PATH: &str = ".dx/forge/release/launch-evidence-final-brief.json";
const CLOSURE_MEMO_PATH: &str = ".dx/forge/release/launch-evidence-closure-memo.md";
const COMPLETION_LEDGER_PATH: &str = ".dx/forge/release/launch-evidence-completion-ledger.json";
const OPERATOR_SUMMARY_PATH: &str = ".dx/forge/release/launch-evidence-operator-summary.json";
const RELEASE_SEAL_PATH: &str = ".dx/forge/release/launch-evidence-release-seal.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceFinalBriefReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    brief: LaunchEvidenceFinalBrief,
    closeout_artifacts: Vec<LaunchEvidenceFinalBriefArtifact>,
    freshness: LaunchEvidenceFinalBriefFreshness,
    checks: Vec<LaunchEvidenceFinalBriefCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceFinalBriefReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceFinalBrief {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    brief_target: &'static str,
    artifact_count: usize,
    present_artifacts: usize,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceFinalBriefArtifact {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceFinalBriefFreshness {
    brief_path: &'static str,
    brief_present: bool,
    brief_modified_at: Option<String>,
    closure_memo_present: bool,
    brief_not_older_than_closure_memo: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceFinalBriefCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_final_brief(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                    message: format!("Unknown forge launch-evidence-final-brief option: {value}"),
                    field: Some("forge.launch-evidence-final-brief".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-final-brief argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-final-brief".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(BRIEF_PATH));
    }

    let mut report =
        build_launch_evidence_final_brief_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_final_brief_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_final_brief(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_final_brief(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_final_brief_failure_summary(&report),
            field: Some("forge.launch-evidence-final-brief".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_final_brief(
    report: &LaunchEvidenceFinalBriefReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_final_brief_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_final_brief_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_final_brief_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceFinalBriefReport> {
    let closeout_artifacts = final_brief_artifacts(project);
    let present_artifacts = closeout_artifacts
        .iter()
        .filter(|artifact| artifact.present)
        .count();
    let all_artifacts_present = present_artifacts == closeout_artifacts.len();
    let freshness = final_brief_freshness(project, &closeout_artifacts);
    let checks = vec![
        check(
            "closure-memo-present",
            freshness.closure_memo_present,
            format!("closure memo exists at {CLOSURE_MEMO_PATH}"),
        ),
        check(
            "final-brief-artifacts-present",
            all_artifacts_present,
            format!(
                "{present_artifacts}/{} final brief artifact(s) are present",
                closeout_artifacts.len()
            ),
        ),
        check(
            "final-brief-freshness",
            freshness.brief_not_older_than_closure_memo,
            "final brief is not older than the closure memo".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "final brief uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceFinalBriefReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        brief: LaunchEvidenceFinalBrief {
            schema: BRIEF_SCHEMA,
            path: BRIEF_PATH,
            command: "dx forge launch-evidence-final-brief --project <path> --write",
            brief_target: "dx-cli-zed-launch-closeout-pointer",
            artifact_count: closeout_artifacts.len(),
            present_artifacts,
            reads_runtime_artifact_contents: false,
        },
        closeout_artifacts,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-closure-memo --project . --write".to_string(),
            "dx forge launch-evidence-final-brief --project . --write".to_string(),
            "dx forge launch-evidence-operator-runbook --project . --write".to_string(),
            format!("open {BRIEF_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_final_brief_terminal(
    report: &LaunchEvidenceFinalBriefReport,
) -> String {
    format!(
        "DX Forge launch evidence final brief\nProject: {}\nPassed: {}\nScore: {}\nBrief target: {}\nArtifacts: {}/{}\nBrief fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.brief.brief_target,
        report.brief.present_artifacts,
        report.brief.artifact_count,
        report.freshness.brief_not_older_than_closure_memo,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_final_brief_markdown(
    report: &LaunchEvidenceFinalBriefReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Final Brief\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Brief target: `{}`\n- No execution: `{}`\n\n",
        report.project, report.passed, report.score, report.brief.brief_target, report.no_execution
    );
    output.push_str(
        "| Artifact | Present | Modified | Bytes | Path |\n| --- | --- | --- | --- | --- |\n",
    );
    for artifact in &report.closeout_artifacts {
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

pub(crate) fn launch_evidence_final_brief_failure_summary(
    report: &LaunchEvidenceFinalBriefReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence final brief score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn final_brief_artifacts(project: &Path) -> Vec<LaunchEvidenceFinalBriefArtifact> {
    [
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
    .map(|(id, path, command)| final_brief_artifact(project, id, path, command))
    .collect()
}

fn final_brief_artifact(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceFinalBriefArtifact {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceFinalBriefArtifact {
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

fn final_brief_freshness(
    project: &Path,
    closeout_artifacts: &[LaunchEvidenceFinalBriefArtifact],
) -> LaunchEvidenceFinalBriefFreshness {
    let brief_modified =
        file_metadata(&project.join(BRIEF_PATH)).map(|metadata| metadata.modified_at);
    let closure_memo_modified =
        file_metadata(&project.join(CLOSURE_MEMO_PATH)).map(|metadata| metadata.modified_at);
    let brief_not_older_than_closure_memo = match (brief_modified, closure_memo_modified) {
        (Some(brief), Some(memo)) => brief >= memo,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceFinalBriefFreshness {
        brief_path: BRIEF_PATH,
        brief_present: brief_modified.is_some(),
        brief_modified_at: brief_modified.map(format_system_time),
        closure_memo_present: artifact_present(closeout_artifacts, "closure-memo"),
        brief_not_older_than_closure_memo,
        timestamp_source: "filesystem-metadata",
    }
}

fn artifact_present(artifacts: &[LaunchEvidenceFinalBriefArtifact], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-final-brief".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceFinalBriefCheck {
    LaunchEvidenceFinalBriefCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceFinalBriefCheck]) -> u8 {
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
    fn fails_when_closure_memo_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            COMPLETION_LEDGER_PATH,
            OPERATOR_SUMMARY_PATH,
            RELEASE_SEAL_PATH,
        ] {
            write_input(dir.path(), path);
        }

        let report =
            build_launch_evidence_final_brief_report(dir.path(), 100).expect("final brief");

        assert!(!report.passed());
        assert!(!report.freshness.closure_memo_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "closure-memo-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_final_brief_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), BRIEF_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), CLOSURE_MEMO_PATH);

        let report = build_launch_evidence_final_brief_report(dir.path(), 0).expect("final brief");

        assert!(!report.freshness.brief_not_older_than_closure_memo);
    }

    #[test]
    fn passes_complete_fresh_final_brief_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_final_brief_inputs(dir.path());
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), BRIEF_PATH);

        let report =
            build_launch_evidence_final_brief_report(dir.path(), 100).expect("final brief");

        assert!(report.passed());
        assert_eq!(
            report.brief.brief_target,
            "dx-cli-zed-launch-closeout-pointer"
        );
        assert!(!report.brief.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_final_brief() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_final_brief_inputs(dir.path());

        run_launch_evidence_final_brief(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write final brief");

        let report =
            build_launch_evidence_final_brief_report(dir.path(), 100).expect("final brief");
        assert!(report.passed());
        assert!(dir.path().join(BRIEF_PATH).is_file());
    }

    fn write_final_brief_inputs(project: &Path) {
        for path in [
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
