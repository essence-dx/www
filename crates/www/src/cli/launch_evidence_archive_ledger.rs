use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_archive_ledger";
const LEDGER_SCHEMA: &str = "dx.launch.evidence_archive_ledger";
const LEDGER_PATH: &str = ".dx/forge/release/launch-evidence-archive-ledger.json";
const RECEIPT_PATH: &str = ".dx/forge/release/launch-evidence-archive-receipt.json";
const ARCHIVE_INDEX_PATH: &str = ".dx/forge/release/launch-evidence-archive-index.json";
const SHARE_MANIFEST_PATH: &str = ".dx/forge/release/launch-evidence-share-.dx/build-cache/manifest.json";
const CHECKLIST_PATH: &str = ".dx/forge/release/launch-evidence-release-checklist.json";
const DIGEST_PATH: &str = ".dx/forge/release/launch-evidence-handoff-digest.md";
const PACKET_PATH: &str = ".dx/forge/release/launch-evidence-packet.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceArchiveLedgerReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    ledger: LaunchEvidenceArchiveLedgerSummary,
    artifacts: Vec<LaunchEvidenceArchiveLedgerArtifact>,
    freshness: LaunchEvidenceArchiveLedgerFreshness,
    checks: Vec<LaunchEvidenceArchiveLedgerCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceArchiveLedgerReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceArchiveLedgerSummary {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    artifact_count: usize,
    present_artifacts: usize,
    ledger_target: &'static str,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceArchiveLedgerArtifact {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceArchiveLedgerFreshness {
    ledger_path: &'static str,
    ledger_present: bool,
    ledger_modified_at: Option<String>,
    archive_receipt_present: bool,
    ledger_not_older_than_archive_receipt: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceArchiveLedgerCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_archive_ledger(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-archive-ledger option: {value}"
                    ),
                    field: Some("forge.launch-evidence-archive-ledger".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-archive-ledger argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-archive-ledger".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(LEDGER_PATH));
    }

    let mut report =
        build_launch_evidence_archive_ledger_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_archive_ledger_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_archive_ledger(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_archive_ledger(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_archive_ledger_failure_summary(&report),
            field: Some("forge.launch-evidence-archive-ledger".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_archive_ledger(
    report: &LaunchEvidenceArchiveLedgerReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_archive_ledger_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_archive_ledger_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_archive_ledger_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceArchiveLedgerReport> {
    let artifacts = ledger_artifacts(project);
    let present_artifacts = artifacts.iter().filter(|artifact| artifact.present).count();
    let all_artifacts_present = present_artifacts == artifacts.len();
    let freshness = archive_ledger_freshness(project, &artifacts);
    let checks = vec![
        check(
            "archive-ledger-artifacts-present",
            all_artifacts_present,
            format!(
                "{present_artifacts}/{} archive ledger artifact(s) are present",
                artifacts.len()
            ),
        ),
        check(
            "archive-ledger-freshness",
            freshness.ledger_not_older_than_archive_receipt,
            "archive ledger is not older than the archive receipt".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "archive ledger uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceArchiveLedgerReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        ledger: LaunchEvidenceArchiveLedgerSummary {
            schema: LEDGER_SCHEMA,
            path: LEDGER_PATH,
            command: "dx forge launch-evidence-archive-ledger --project <path> --write",
            artifact_count: artifacts.len(),
            present_artifacts,
            ledger_target: "durable-release-ledger",
            reads_runtime_artifact_contents: false,
        },
        artifacts,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-archive-receipt --project . --write".to_string(),
            "dx forge launch-evidence-archive-ledger --project . --write".to_string(),
            "dx forge launch-evidence-retention-policy --project . --write".to_string(),
            format!("open {LEDGER_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_archive_ledger_terminal(
    report: &LaunchEvidenceArchiveLedgerReport,
) -> String {
    format!(
        "DX Forge launch evidence archive ledger\nProject: {}\nPassed: {}\nScore: {}\nArtifacts: {}/{}\nLedger fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.ledger.present_artifacts,
        report.ledger.artifact_count,
        report.freshness.ledger_not_older_than_archive_receipt,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_archive_ledger_markdown(
    report: &LaunchEvidenceArchiveLedgerReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Archive Ledger\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Ledger target: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.ledger.ledger_target,
        report.no_execution
    );
    output.push_str("| Artifact | Present | Path |\n| --- | --- | --- |\n");
    for artifact in &report.artifacts {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` |\n",
            artifact.id, artifact.present, artifact.path
        ));
    }
    output
}

pub(crate) fn launch_evidence_archive_ledger_failure_summary(
    report: &LaunchEvidenceArchiveLedgerReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence archive ledger score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn ledger_artifacts(project: &Path) -> Vec<LaunchEvidenceArchiveLedgerArtifact> {
    [
        (
            "archive-receipt",
            RECEIPT_PATH,
            "dx forge launch-evidence-archive-receipt --project . --write",
        ),
        (
            "archive-index",
            ARCHIVE_INDEX_PATH,
            "dx forge launch-evidence-archive-index --project . --write",
        ),
        (
            "share-manifest",
            SHARE_MANIFEST_PATH,
            "dx forge launch-evidence-share-manifest --project . --write",
        ),
        (
            "release-checklist",
            CHECKLIST_PATH,
            "dx forge launch-evidence-release-checklist --project . --write",
        ),
        (
            "handoff-digest",
            DIGEST_PATH,
            "dx forge launch-evidence-handoff-digest --project . --write",
        ),
        (
            "release-packet",
            PACKET_PATH,
            "dx forge launch-evidence-packet --project . --write",
        ),
    ]
    .into_iter()
    .map(|(id, path, command)| ledger_artifact(project, id, path, command))
    .collect()
}

fn ledger_artifact(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceArchiveLedgerArtifact {
    let modified = file_modified_at(&project.join(path));
    LaunchEvidenceArchiveLedgerArtifact {
        id,
        path,
        present: modified.is_some(),
        modified_at: modified.map(format_system_time),
        command,
    }
}

fn archive_ledger_freshness(
    project: &Path,
    artifacts: &[LaunchEvidenceArchiveLedgerArtifact],
) -> LaunchEvidenceArchiveLedgerFreshness {
    let ledger_modified = file_modified_at(&project.join(LEDGER_PATH));
    let receipt_modified = file_modified_at(&project.join(RECEIPT_PATH));
    let ledger_not_older_than_archive_receipt = match (ledger_modified, receipt_modified) {
        (Some(ledger), Some(receipt)) => ledger >= receipt,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceArchiveLedgerFreshness {
        ledger_path: LEDGER_PATH,
        ledger_present: ledger_modified.is_some(),
        ledger_modified_at: ledger_modified.map(format_system_time),
        archive_receipt_present: artifact_present(artifacts, "archive-receipt"),
        ledger_not_older_than_archive_receipt,
        timestamp_source: "filesystem-metadata",
    }
}

fn artifact_present(artifacts: &[LaunchEvidenceArchiveLedgerArtifact], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-archive-ledger".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceArchiveLedgerCheck {
    LaunchEvidenceArchiveLedgerCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceArchiveLedgerCheck]) -> u8 {
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
    fn fails_when_archive_receipt_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            ARCHIVE_INDEX_PATH,
            SHARE_MANIFEST_PATH,
            CHECKLIST_PATH,
            DIGEST_PATH,
            PACKET_PATH,
        ] {
            write_input(dir.path(), path);
        }

        let report = build_launch_evidence_archive_ledger_report(dir.path(), 100).expect("ledger");

        assert!(!report.passed());
        assert!(!report.freshness.archive_receipt_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "archive-ledger-artifacts-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_archive_ledger_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), LEDGER_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), RECEIPT_PATH);

        let report = build_launch_evidence_archive_ledger_report(dir.path(), 0).expect("ledger");

        assert!(!report.freshness.ledger_not_older_than_archive_receipt);
    }

    #[test]
    fn passes_complete_fresh_archive_ledger_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            RECEIPT_PATH,
            ARCHIVE_INDEX_PATH,
            SHARE_MANIFEST_PATH,
            CHECKLIST_PATH,
            DIGEST_PATH,
            PACKET_PATH,
        ] {
            write_input(dir.path(), path);
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), LEDGER_PATH);

        let report = build_launch_evidence_archive_ledger_report(dir.path(), 100).expect("ledger");

        assert!(report.passed());
        assert_eq!(report.ledger.ledger_target, "durable-release-ledger");
        assert!(!report.ledger.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_archive_ledger() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            RECEIPT_PATH,
            ARCHIVE_INDEX_PATH,
            SHARE_MANIFEST_PATH,
            CHECKLIST_PATH,
            DIGEST_PATH,
            PACKET_PATH,
        ] {
            write_input(dir.path(), path);
        }

        run_launch_evidence_archive_ledger(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write archive ledger");

        let report = build_launch_evidence_archive_ledger_report(dir.path(), 100).expect("ledger");
        assert!(report.passed());
        assert!(dir.path().join(LEDGER_PATH).is_file());
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
