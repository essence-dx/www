use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_release_seal";
const SEAL_SCHEMA: &str = "dx.launch.evidence_release_seal";
const SEAL_PATH: &str = ".dx/forge/release/launch-evidence-release-seal.json";
const REVIEW_PATH: &str = ".dx/forge/release/launch-evidence-retention-review.json";
const POLICY_PATH: &str = ".dx/forge/release/launch-evidence-retention-policy.json";
const LEDGER_PATH: &str = ".dx/forge/release/launch-evidence-archive-ledger.json";
const PACKET_PATH: &str = ".dx/forge/release/launch-evidence-packet.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceReleaseSealReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    seal: LaunchEvidenceReleaseSealSummary,
    sealed_artifacts: Vec<LaunchEvidenceReleaseSealArtifact>,
    freshness: LaunchEvidenceReleaseSealFreshness,
    checks: Vec<LaunchEvidenceReleaseSealCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceReleaseSealReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceReleaseSealSummary {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    seal_target: &'static str,
    artifact_count: usize,
    present_artifacts: usize,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceReleaseSealArtifact {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceReleaseSealFreshness {
    seal_path: &'static str,
    seal_present: bool,
    seal_modified_at: Option<String>,
    retention_review_present: bool,
    seal_not_older_than_retention_review: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceReleaseSealCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_release_seal(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                    message: format!("Unknown forge launch-evidence-release-seal option: {value}"),
                    field: Some("forge.launch-evidence-release-seal".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-release-seal argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-release-seal".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(SEAL_PATH));
    }

    let mut report =
        build_launch_evidence_release_seal_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_release_seal_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_release_seal(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_release_seal(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_release_seal_failure_summary(&report),
            field: Some("forge.launch-evidence-release-seal".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_release_seal(
    report: &LaunchEvidenceReleaseSealReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_release_seal_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_release_seal_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_release_seal_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceReleaseSealReport> {
    let sealed_artifacts = sealed_artifacts(project);
    let present_artifacts = sealed_artifacts
        .iter()
        .filter(|artifact| artifact.present)
        .count();
    let all_artifacts_present = present_artifacts == sealed_artifacts.len();
    let freshness = release_seal_freshness(project, &sealed_artifacts);
    let checks = vec![
        check(
            "retention-review-present",
            freshness.retention_review_present,
            format!("retention review exists at {REVIEW_PATH}"),
        ),
        check(
            "release-seal-artifacts-present",
            all_artifacts_present,
            format!(
                "{present_artifacts}/{} release seal artifact(s) are present",
                sealed_artifacts.len()
            ),
        ),
        check(
            "release-seal-freshness",
            freshness.seal_not_older_than_retention_review,
            "release seal is not older than the retention review".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "release seal uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceReleaseSealReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        seal: LaunchEvidenceReleaseSealSummary {
            schema: SEAL_SCHEMA,
            path: SEAL_PATH,
            command: "dx forge launch-evidence-release-seal --project <path> --write",
            seal_target: "final-launch-handoff-seal",
            artifact_count: sealed_artifacts.len(),
            present_artifacts,
            reads_runtime_artifact_contents: false,
        },
        sealed_artifacts,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-retention-review --project . --write".to_string(),
            "dx forge launch-evidence-release-seal --project . --write".to_string(),
            "dx forge launch-evidence-operator-summary --project . --write".to_string(),
            format!("open {SEAL_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_release_seal_terminal(
    report: &LaunchEvidenceReleaseSealReport,
) -> String {
    format!(
        "DX Forge launch evidence release seal\nProject: {}\nPassed: {}\nScore: {}\nSeal target: {}\nSeal fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.seal.seal_target,
        report.freshness.seal_not_older_than_retention_review,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_release_seal_markdown(
    report: &LaunchEvidenceReleaseSealReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Release Seal\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Seal target: `{}`\n- No execution: `{}`\n\n",
        report.project, report.passed, report.score, report.seal.seal_target, report.no_execution
    );
    output.push_str("| Artifact | Present | Path |\n| --- | --- | --- |\n");
    for artifact in &report.sealed_artifacts {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` |\n",
            artifact.id, artifact.present, artifact.path
        ));
    }
    output
}

pub(crate) fn launch_evidence_release_seal_failure_summary(
    report: &LaunchEvidenceReleaseSealReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence release seal score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn sealed_artifacts(project: &Path) -> Vec<LaunchEvidenceReleaseSealArtifact> {
    [
        (
            "retention-review",
            REVIEW_PATH,
            "dx forge launch-evidence-retention-review --project . --write",
        ),
        (
            "retention-policy",
            POLICY_PATH,
            "dx forge launch-evidence-retention-policy --project . --write",
        ),
        (
            "archive-ledger",
            LEDGER_PATH,
            "dx forge launch-evidence-archive-ledger --project . --write",
        ),
        (
            "release-packet",
            PACKET_PATH,
            "dx forge launch-evidence-packet --project . --write",
        ),
    ]
    .into_iter()
    .map(|(id, path, command)| sealed_artifact(project, id, path, command))
    .collect()
}

fn sealed_artifact(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceReleaseSealArtifact {
    let modified = file_modified_at(&project.join(path));
    LaunchEvidenceReleaseSealArtifact {
        id,
        path,
        present: modified.is_some(),
        modified_at: modified.map(format_system_time),
        command,
    }
}

fn release_seal_freshness(
    project: &Path,
    sealed_artifacts: &[LaunchEvidenceReleaseSealArtifact],
) -> LaunchEvidenceReleaseSealFreshness {
    let seal_modified = file_modified_at(&project.join(SEAL_PATH));
    let review_modified = file_modified_at(&project.join(REVIEW_PATH));
    let seal_not_older_than_retention_review = match (seal_modified, review_modified) {
        (Some(seal), Some(review)) => seal >= review,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceReleaseSealFreshness {
        seal_path: SEAL_PATH,
        seal_present: seal_modified.is_some(),
        seal_modified_at: seal_modified.map(format_system_time),
        retention_review_present: artifact_present(sealed_artifacts, "retention-review"),
        seal_not_older_than_retention_review,
        timestamp_source: "filesystem-metadata",
    }
}

fn artifact_present(sealed_artifacts: &[LaunchEvidenceReleaseSealArtifact], id: &str) -> bool {
    sealed_artifacts
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
            field: Some("forge.launch-evidence-release-seal".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceReleaseSealCheck {
    LaunchEvidenceReleaseSealCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceReleaseSealCheck]) -> u8 {
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
    fn fails_when_retention_review_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [POLICY_PATH, LEDGER_PATH, PACKET_PATH] {
            write_input(dir.path(), path);
        }

        let report = build_launch_evidence_release_seal_report(dir.path(), 100).expect("seal");

        assert!(!report.passed());
        assert!(!report.freshness.retention_review_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "retention-review-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_release_seal_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), SEAL_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), REVIEW_PATH);

        let report = build_launch_evidence_release_seal_report(dir.path(), 0).expect("seal");

        assert!(!report.freshness.seal_not_older_than_retention_review);
    }

    #[test]
    fn passes_complete_fresh_release_seal_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [REVIEW_PATH, POLICY_PATH, LEDGER_PATH, PACKET_PATH] {
            write_input(dir.path(), path);
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), SEAL_PATH);

        let report = build_launch_evidence_release_seal_report(dir.path(), 100).expect("seal");

        assert!(report.passed());
        assert_eq!(report.seal.seal_target, "final-launch-handoff-seal");
        assert!(!report.seal.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_release_seal() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [REVIEW_PATH, POLICY_PATH, LEDGER_PATH, PACKET_PATH] {
            write_input(dir.path(), path);
        }

        run_launch_evidence_release_seal(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write release seal");

        let report = build_launch_evidence_release_seal_report(dir.path(), 100).expect("seal");
        assert!(report.passed());
        assert!(dir.path().join(SEAL_PATH).is_file());
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("input parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
