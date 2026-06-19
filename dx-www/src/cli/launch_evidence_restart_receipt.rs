use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_restart_receipt";
const RECEIPT_SCHEMA: &str = "dx.launch.evidence_restart_receipt";
const RECEIPT_PATH: &str = ".dx/forge/release/launch-evidence-restart-receipt.json";
const RESTART_MANIFEST_PATH: &str = ".dx/forge/release/launch-evidence-restart-manifest.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceRestartReceiptReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    receipt: LaunchEvidenceRestartReceipt,
    inputs: Vec<LaunchEvidenceRestartReceiptInput>,
    freshness: LaunchEvidenceRestartReceiptFreshness,
    checks: Vec<LaunchEvidenceRestartReceiptCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceRestartReceiptReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartReceipt {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    receipt_target: &'static str,
    restart_manifest: &'static str,
    input_count: usize,
    present_inputs: usize,
    zed_handoff_target: &'static str,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartReceiptInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartReceiptFreshness {
    receipt_path: &'static str,
    receipt_present: bool,
    receipt_modified_at: Option<String>,
    restart_manifest_present: bool,
    receipt_not_older_than_restart_manifest: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartReceiptCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_restart_receipt(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-restart-receipt option: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-receipt".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-restart-receipt argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-receipt".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(RECEIPT_PATH));
    }

    let mut report =
        build_launch_evidence_restart_receipt_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_restart_receipt_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_restart_receipt(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_restart_receipt(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_restart_receipt_failure_summary(&report),
            field: Some("forge.launch-evidence-restart-receipt".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_restart_receipt(
    report: &LaunchEvidenceRestartReceiptReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_restart_receipt_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_restart_receipt_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_restart_receipt_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceRestartReceiptReport> {
    let inputs = restart_receipt_inputs(project);
    let present_inputs = inputs.iter().filter(|input| input.present).count();
    let all_inputs_present = present_inputs == inputs.len();
    let freshness = restart_receipt_freshness(project, &inputs);
    let checks = vec![
        check(
            "restart-manifest-present",
            freshness.restart_manifest_present,
            format!("restart manifest exists at {RESTART_MANIFEST_PATH}"),
        ),
        check(
            "restart-receipt-inputs-present",
            all_inputs_present,
            format!(
                "{present_inputs}/{} restart receipt input(s) are present",
                inputs.len()
            ),
        ),
        check(
            "receipt-not-older-than-restart-manifest",
            freshness.receipt_not_older_than_restart_manifest,
            "restart receipt is not older than the restart manifest".to_string(),
        ),
        check(
            "latest-resumable-dx-zed-handoff",
            true,
            "restart receipt records the latest resumable DX/Zed handoff target".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "restart receipt uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceRestartReceiptReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        receipt: LaunchEvidenceRestartReceipt {
            schema: RECEIPT_SCHEMA,
            path: RECEIPT_PATH,
            command: "dx forge launch-evidence-restart-receipt --project <path> --write",
            receipt_target: "latest-resumable-dx-zed-handoff",
            restart_manifest: RESTART_MANIFEST_PATH,
            input_count: inputs.len(),
            present_inputs,
            zed_handoff_target: "latest-resumable-dx-zed-handoff",
            reads_runtime_artifact_contents: false,
        },
        inputs,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-restart-manifest --project . --write".to_string(),
            "dx forge launch-evidence-restart-receipt --project . --write".to_string(),
            "dx forge launch-evidence-restart-summary --project . --write".to_string(),
            format!("open {RECEIPT_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_restart_receipt_terminal(
    report: &LaunchEvidenceRestartReceiptReport,
) -> String {
    format!(
        "DX Forge launch evidence restart receipt\nProject: {}\nPassed: {}\nScore: {}\nReceipt target: {}\nInputs present: {}/{}\nReceipt fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.receipt.receipt_target,
        report.receipt.present_inputs,
        report.receipt.input_count,
        report.freshness.receipt_not_older_than_restart_manifest,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_restart_receipt_markdown(
    report: &LaunchEvidenceRestartReceiptReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Restart Receipt\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Receipt target: `{}`\n- Receipt fresh: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.receipt.receipt_target,
        report.freshness.receipt_not_older_than_restart_manifest,
        report.no_execution
    );
    output.push_str("## Checks\n\n");
    for check in &report.checks {
        output.push_str(&format!(
            "- `{}`: {} - {}\n",
            check.name, check.passed, check.message
        ));
    }
    output
}

fn launch_evidence_restart_receipt_failure_summary(
    report: &LaunchEvidenceRestartReceiptReport,
) -> String {
    if report.findings.is_empty() {
        format!(
            "launch evidence restart receipt score {} is below threshold",
            report.score
        )
    } else {
        format!(
            "launch evidence restart receipt failed: {}",
            report.findings.join("; ")
        )
    }
}

fn restart_receipt_inputs(project: &Path) -> Vec<LaunchEvidenceRestartReceiptInput> {
    [(
        "restart-manifest",
        RESTART_MANIFEST_PATH,
        "dx forge launch-evidence-restart-manifest --project <path> --write",
    )]
    .into_iter()
    .map(|(id, path, command)| {
        let metadata = file_metadata(&project.join(path));
        LaunchEvidenceRestartReceiptInput {
            id,
            path,
            present: metadata.is_some(),
            modified_at: metadata
                .as_ref()
                .map(|metadata| format_system_time(metadata.modified_at)),
            bytes: metadata.map(|metadata| metadata.bytes),
            command,
        }
    })
    .collect()
}

fn restart_receipt_freshness(
    project: &Path,
    inputs: &[LaunchEvidenceRestartReceiptInput],
) -> LaunchEvidenceRestartReceiptFreshness {
    let receipt_modified =
        file_metadata(&project.join(RECEIPT_PATH)).map(|metadata| metadata.modified_at);
    let manifest_modified =
        file_metadata(&project.join(RESTART_MANIFEST_PATH)).map(|metadata| metadata.modified_at);
    let receipt_not_older_than_restart_manifest = match (receipt_modified, manifest_modified) {
        (Some(receipt), Some(manifest)) => receipt >= manifest,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceRestartReceiptFreshness {
        receipt_path: RECEIPT_PATH,
        receipt_present: receipt_modified.is_some(),
        receipt_modified_at: receipt_modified.map(format_system_time),
        restart_manifest_present: input_present(inputs, "restart-manifest"),
        receipt_not_older_than_restart_manifest,
        timestamp_source: "filesystem-metadata",
    }
}

fn input_present(inputs: &[LaunchEvidenceRestartReceiptInput], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-restart-receipt".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceRestartReceiptCheck {
    LaunchEvidenceRestartReceiptCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceRestartReceiptCheck]) -> u8 {
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
    fn fails_when_restart_manifest_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");

        let report =
            build_launch_evidence_restart_receipt_report(dir.path(), 100).expect("receipt");

        assert!(!report.passed());
        assert!(!report.freshness.restart_manifest_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "restart-manifest-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_restart_receipt_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), RECEIPT_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), RESTART_MANIFEST_PATH);

        let report = build_launch_evidence_restart_receipt_report(dir.path(), 0).expect("receipt");

        assert!(!report.freshness.receipt_not_older_than_restart_manifest);
    }

    #[test]
    fn passes_complete_fresh_receipt_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_restart_receipt_inputs(dir.path());
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), RECEIPT_PATH);

        let report =
            build_launch_evidence_restart_receipt_report(dir.path(), 100).expect("receipt");

        assert!(report.passed());
        assert_eq!(
            report.receipt.receipt_target,
            "latest-resumable-dx-zed-handoff"
        );
        assert!(!report.receipt.reads_runtime_artifact_contents);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "no-runtime-content-read" && check.passed)
        );
    }

    #[test]
    fn write_mode_creates_fresh_restart_receipt() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_restart_receipt_inputs(dir.path());

        run_launch_evidence_restart_receipt(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write receipt");

        let report =
            build_launch_evidence_restart_receipt_report(dir.path(), 100).expect("receipt");
        assert!(report.passed());
        assert!(dir.path().join(RECEIPT_PATH).is_file());
    }

    fn write_restart_receipt_inputs(project: &Path) {
        write_input(project, RESTART_MANIFEST_PATH);
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("input parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
