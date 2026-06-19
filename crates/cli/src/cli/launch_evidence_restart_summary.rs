use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_restart_summary";
const SUMMARY_SCHEMA: &str = "dx.launch.evidence_restart_summary";
const SUMMARY_PATH: &str = ".dx/forge/release/launch-evidence-restart-summary.json";
const RESTART_RECEIPT_PATH: &str = ".dx/forge/release/launch-evidence-restart-receipt.json";
const RESTART_MANIFEST_PATH: &str = ".dx/forge/release/launch-evidence-restart-manifest.json";
const RESTART_BRIEF_PATH: &str = ".dx/forge/release/launch-evidence-restart-brief.md";
const RESTART_CHECKLIST_PATH: &str = ".dx/forge/release/launch-evidence-restart-checklist.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceRestartSummaryReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    summary: LaunchEvidenceRestartSummary,
    inputs: Vec<LaunchEvidenceRestartSummaryInput>,
    freshness: LaunchEvidenceRestartSummaryFreshness,
    checks: Vec<LaunchEvidenceRestartSummaryCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceRestartSummaryReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartSummary {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    summary_target: &'static str,
    restart_receipt: &'static str,
    restart_manifest: &'static str,
    restart_brief: &'static str,
    restart_checklist: &'static str,
    input_count: usize,
    present_inputs: usize,
    display_mode: &'static str,
    zed_handoff_target: &'static str,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartSummaryInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartSummaryFreshness {
    summary_path: &'static str,
    summary_present: bool,
    summary_modified_at: Option<String>,
    restart_receipt_present: bool,
    summary_not_older_than_restart_receipt: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartSummaryCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_restart_summary(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-restart-summary option: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-summary".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-restart-summary argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-summary".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(SUMMARY_PATH));
    }

    let mut report =
        build_launch_evidence_restart_summary_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_restart_summary_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_restart_summary(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_restart_summary(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_restart_summary_failure_summary(&report),
            field: Some("forge.launch-evidence-restart-summary".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_restart_summary(
    report: &LaunchEvidenceRestartSummaryReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_restart_summary_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_restart_summary_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_restart_summary_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceRestartSummaryReport> {
    let inputs = restart_summary_inputs(project);
    let present_inputs = inputs.iter().filter(|input| input.present).count();
    let all_inputs_present = present_inputs == inputs.len();
    let freshness = restart_summary_freshness(project, &inputs);
    let checks = vec![
        check(
            "restart-receipt-present",
            freshness.restart_receipt_present,
            format!("restart receipt exists at {RESTART_RECEIPT_PATH}"),
        ),
        check(
            "restart-summary-inputs-present",
            all_inputs_present,
            format!(
                "{present_inputs}/{} restart summary input(s) are present",
                inputs.len()
            ),
        ),
        check(
            "summary-not-older-than-restart-receipt",
            freshness.summary_not_older_than_restart_receipt,
            "restart summary is not older than the restart receipt".to_string(),
        ),
        check(
            "terminal-friendly-dx-zed-restart-handoff",
            true,
            "restart summary condenses the restart evidence into a terminal-friendly DX/Zed handoff"
                .to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "restart summary uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceRestartSummaryReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        summary: LaunchEvidenceRestartSummary {
            schema: SUMMARY_SCHEMA,
            path: SUMMARY_PATH,
            command: "dx forge launch-evidence-restart-summary --project <path> --write",
            summary_target: "terminal-friendly-dx-zed-restart-handoff",
            restart_receipt: RESTART_RECEIPT_PATH,
            restart_manifest: RESTART_MANIFEST_PATH,
            restart_brief: RESTART_BRIEF_PATH,
            restart_checklist: RESTART_CHECKLIST_PATH,
            input_count: inputs.len(),
            present_inputs,
            display_mode: "terminal-first",
            zed_handoff_target: "terminal-friendly-dx-zed-restart-handoff",
            reads_runtime_artifact_contents: false,
        },
        inputs,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-restart-receipt --project . --write".to_string(),
            "dx forge launch-evidence-restart-summary --project . --write".to_string(),
            "dx forge launch-evidence-restart-snapshot --project . --write".to_string(),
            format!("open {SUMMARY_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_restart_summary_terminal(
    report: &LaunchEvidenceRestartSummaryReport,
) -> String {
    format!(
        "DX Forge launch evidence restart summary\nProject: {}\nPassed: {}\nScore: {}\nSummary target: {}\nInputs present: {}/{}\nSummary fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.summary.summary_target,
        report.summary.present_inputs,
        report.summary.input_count,
        report.freshness.summary_not_older_than_restart_receipt,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_restart_summary_markdown(
    report: &LaunchEvidenceRestartSummaryReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Restart Summary\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Summary target: `{}`\n- Summary fresh: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.summary.summary_target,
        report.freshness.summary_not_older_than_restart_receipt,
        report.no_execution
    );
    output.push_str(
        "| Input | Present | Modified | Bytes | Path |\n| --- | --- | --- | --- | --- |\n",
    );
    for input in &report.inputs {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            input.id,
            input.present,
            input.modified_at.as_deref().unwrap_or("missing"),
            input
                .bytes
                .map(|bytes| bytes.to_string())
                .unwrap_or_else(|| "missing".to_string()),
            input.path
        ));
    }
    output
}

fn launch_evidence_restart_summary_failure_summary(
    report: &LaunchEvidenceRestartSummaryReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence restart summary score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn restart_summary_inputs(project: &Path) -> Vec<LaunchEvidenceRestartSummaryInput> {
    [
        (
            "restart-receipt",
            RESTART_RECEIPT_PATH,
            "dx forge launch-evidence-restart-receipt --project . --write",
        ),
        (
            "restart-manifest",
            RESTART_MANIFEST_PATH,
            "dx forge launch-evidence-restart-manifest --project . --write",
        ),
        (
            "restart-brief",
            RESTART_BRIEF_PATH,
            "dx forge launch-evidence-restart-brief --project . --write",
        ),
        (
            "restart-checklist",
            RESTART_CHECKLIST_PATH,
            "dx forge launch-evidence-restart-checklist --project . --write",
        ),
    ]
    .into_iter()
    .map(|(id, path, command)| restart_summary_input(project, id, path, command))
    .collect()
}

fn restart_summary_input(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceRestartSummaryInput {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceRestartSummaryInput {
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

fn restart_summary_freshness(
    project: &Path,
    inputs: &[LaunchEvidenceRestartSummaryInput],
) -> LaunchEvidenceRestartSummaryFreshness {
    let summary_modified =
        file_metadata(&project.join(SUMMARY_PATH)).map(|metadata| metadata.modified_at);
    let restart_receipt_modified =
        file_metadata(&project.join(RESTART_RECEIPT_PATH)).map(|metadata| metadata.modified_at);
    let summary_not_older_than_restart_receipt = match (summary_modified, restart_receipt_modified)
    {
        (Some(summary), Some(receipt)) => summary >= receipt,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceRestartSummaryFreshness {
        summary_path: SUMMARY_PATH,
        summary_present: summary_modified.is_some(),
        summary_modified_at: summary_modified.map(format_system_time),
        restart_receipt_present: input_present(inputs, "restart-receipt"),
        summary_not_older_than_restart_receipt,
        timestamp_source: "filesystem-metadata",
    }
}

fn input_present(inputs: &[LaunchEvidenceRestartSummaryInput], id: &str) -> bool {
    inputs.iter().any(|input| input.id == id && input.present)
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
            field: Some("forge.launch-evidence-restart-summary".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceRestartSummaryCheck {
    LaunchEvidenceRestartSummaryCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceRestartSummaryCheck]) -> u8 {
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
    fn fails_when_restart_receipt_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            RESTART_MANIFEST_PATH,
            RESTART_BRIEF_PATH,
            RESTART_CHECKLIST_PATH,
        ] {
            write_input(dir.path(), path);
        }

        let report =
            build_launch_evidence_restart_summary_report(dir.path(), 100).expect("summary");

        assert!(!report.passed());
        assert!(!report.freshness.restart_receipt_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "restart-receipt-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_restart_summary_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), SUMMARY_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), RESTART_RECEIPT_PATH);

        let report = build_launch_evidence_restart_summary_report(dir.path(), 0).expect("summary");

        assert!(!report.freshness.summary_not_older_than_restart_receipt);
    }

    #[test]
    fn passes_complete_fresh_summary_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_restart_summary_inputs(dir.path());
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), SUMMARY_PATH);

        let report =
            build_launch_evidence_restart_summary_report(dir.path(), 100).expect("summary");

        assert!(report.passed());
        assert_eq!(
            report.summary.summary_target,
            "terminal-friendly-dx-zed-restart-handoff"
        );
        assert!(!report.summary.reads_runtime_artifact_contents);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "no-runtime-content-read" && check.passed)
        );
    }

    #[test]
    fn write_mode_creates_fresh_restart_summary() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_restart_summary_inputs(dir.path());

        run_launch_evidence_restart_summary(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write summary");

        let report =
            build_launch_evidence_restart_summary_report(dir.path(), 100).expect("summary");
        assert!(report.passed());
        assert!(dir.path().join(SUMMARY_PATH).is_file());
    }

    fn write_restart_summary_inputs(project: &Path) {
        for path in [
            RESTART_RECEIPT_PATH,
            RESTART_MANIFEST_PATH,
            RESTART_BRIEF_PATH,
            RESTART_CHECKLIST_PATH,
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
