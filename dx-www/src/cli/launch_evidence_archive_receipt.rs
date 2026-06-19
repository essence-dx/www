use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_archive_receipt";
const RECEIPT_SCHEMA: &str = "dx.launch.evidence_archive_receipt";
const RECEIPT_PATH: &str = ".dx/forge/release/launch-evidence-archive-receipt.json";
const ARCHIVE_INDEX_PATH: &str = ".dx/forge/release/launch-evidence-archive-index.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceArchiveReceiptReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    receipt: LaunchEvidenceArchiveReceiptSummary,
    freshness: LaunchEvidenceArchiveReceiptFreshness,
    checks: Vec<LaunchEvidenceArchiveReceiptCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceArchiveReceiptReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceArchiveReceiptSummary {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    archive_index: &'static str,
    operator_handoff_target: &'static str,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceArchiveReceiptFreshness {
    receipt_path: &'static str,
    receipt_present: bool,
    receipt_modified_at: Option<String>,
    archive_index_present: bool,
    archive_index_modified_at: Option<String>,
    receipt_not_older_than_archive_index: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceArchiveReceiptCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_archive_receipt(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-archive-receipt option: {value}"
                    ),
                    field: Some("forge.launch-evidence-archive-receipt".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-archive-receipt argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-archive-receipt".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(RECEIPT_PATH));
    }

    let mut report =
        build_launch_evidence_archive_receipt_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_archive_receipt_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_archive_receipt(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_archive_receipt(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_archive_receipt_failure_summary(&report),
            field: Some("forge.launch-evidence-archive-receipt".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_archive_receipt(
    report: &LaunchEvidenceArchiveReceiptReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_archive_receipt_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_archive_receipt_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_archive_receipt_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceArchiveReceiptReport> {
    let freshness = archive_receipt_freshness(project);
    let checks = vec![
        check(
            "archive-index-present",
            freshness.archive_index_present,
            format!("archive index exists at {ARCHIVE_INDEX_PATH}"),
        ),
        check(
            "archive-receipt-freshness",
            freshness.receipt_not_older_than_archive_index,
            "archive receipt is not older than the archive index".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "archive receipt uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceArchiveReceiptReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        receipt: LaunchEvidenceArchiveReceiptSummary {
            schema: RECEIPT_SCHEMA,
            path: RECEIPT_PATH,
            command: "dx forge launch-evidence-archive-receipt --project <path> --write",
            archive_index: ARCHIVE_INDEX_PATH,
            operator_handoff_target: "dx-cli-zed-archive",
            reads_runtime_artifact_contents: false,
        },
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-archive-index --project . --write".to_string(),
            "dx forge launch-evidence-archive-receipt --project . --write".to_string(),
            "dx forge launch-evidence-archive-ledger --project . --write".to_string(),
            format!("open {RECEIPT_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_archive_receipt_terminal(
    report: &LaunchEvidenceArchiveReceiptReport,
) -> String {
    format!(
        "DX Forge launch evidence archive receipt\nProject: {}\nPassed: {}\nScore: {}\nArchive index present: {}\nReceipt fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.freshness.archive_index_present,
        report.freshness.receipt_not_older_than_archive_index,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_archive_receipt_markdown(
    report: &LaunchEvidenceArchiveReceiptReport,
) -> String {
    format!(
        "# DX Forge Launch Evidence Archive Receipt\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Archive index: `{}`\n- Operator handoff target: `{}`\n- No execution: `{}`\n",
        report.project,
        report.passed,
        report.score,
        report.receipt.archive_index,
        report.receipt.operator_handoff_target,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_archive_receipt_failure_summary(
    report: &LaunchEvidenceArchiveReceiptReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence archive receipt score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn archive_receipt_freshness(project: &Path) -> LaunchEvidenceArchiveReceiptFreshness {
    let receipt_modified = file_modified_at(&project.join(RECEIPT_PATH));
    let archive_index_modified = file_modified_at(&project.join(ARCHIVE_INDEX_PATH));
    let receipt_not_older_than_archive_index = match (receipt_modified, archive_index_modified) {
        (Some(receipt), Some(index)) => receipt >= index,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceArchiveReceiptFreshness {
        receipt_path: RECEIPT_PATH,
        receipt_present: receipt_modified.is_some(),
        receipt_modified_at: receipt_modified.map(format_system_time),
        archive_index_present: archive_index_modified.is_some(),
        archive_index_modified_at: archive_index_modified.map(format_system_time),
        receipt_not_older_than_archive_index,
        timestamp_source: "filesystem-metadata",
    }
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
            field: Some("forge.launch-evidence-archive-receipt".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceArchiveReceiptCheck {
    LaunchEvidenceArchiveReceiptCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceArchiveReceiptCheck]) -> u8 {
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
    fn fails_when_archive_index_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");

        let report =
            build_launch_evidence_archive_receipt_report(dir.path(), 100).expect("receipt");

        assert!(!report.passed());
        assert!(!report.freshness.archive_index_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "archive-index-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_archive_receipt_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), RECEIPT_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), ARCHIVE_INDEX_PATH);

        let report = build_launch_evidence_archive_receipt_report(dir.path(), 0).expect("receipt");

        assert!(!report.freshness.receipt_not_older_than_archive_index);
    }

    #[test]
    fn passes_complete_fresh_archive_receipt_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), ARCHIVE_INDEX_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), RECEIPT_PATH);

        let report =
            build_launch_evidence_archive_receipt_report(dir.path(), 100).expect("receipt");

        assert!(report.passed());
        assert_eq!(report.receipt.operator_handoff_target, "dx-cli-zed-archive");
        assert!(!report.receipt.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_archive_receipt() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), ARCHIVE_INDEX_PATH);

        run_launch_evidence_archive_receipt(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write archive receipt");

        let report =
            build_launch_evidence_archive_receipt_report(dir.path(), 100).expect("receipt");
        assert!(report.passed());
        assert!(dir.path().join(RECEIPT_PATH).is_file());
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
