use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_restart_snapshot";
const SNAPSHOT_SCHEMA: &str = "dx.launch.evidence_restart_snapshot";
const SNAPSHOT_PATH: &str = ".dx/forge/release/launch-evidence-restart-snapshot.json";
const RESTART_SUMMARY_PATH: &str = ".dx/forge/release/launch-evidence-restart-summary.json";
const RESTART_RECEIPT_PATH: &str = ".dx/forge/release/launch-evidence-restart-receipt.json";
const RESTART_MANIFEST_PATH: &str = ".dx/forge/release/launch-evidence-restart-.dx/build-cache/manifest.json";
const RESTART_BRIEF_PATH: &str = ".dx/forge/release/launch-evidence-restart-brief.md";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceRestartSnapshotReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    snapshot: LaunchEvidenceRestartSnapshot,
    inputs: Vec<LaunchEvidenceRestartSnapshotInput>,
    freshness: LaunchEvidenceRestartSnapshotFreshness,
    checks: Vec<LaunchEvidenceRestartSnapshotCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceRestartSnapshotReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartSnapshot {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    snapshot_target: &'static str,
    restart_summary: &'static str,
    restart_receipt: &'static str,
    restart_manifest: &'static str,
    restart_brief: &'static str,
    input_count: usize,
    present_inputs: usize,
    display_mode: &'static str,
    zed_handoff_target: &'static str,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartSnapshotInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartSnapshotFreshness {
    snapshot_path: &'static str,
    snapshot_present: bool,
    snapshot_modified_at: Option<String>,
    restart_summary_present: bool,
    snapshot_not_older_than_restart_summary: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartSnapshotCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_restart_snapshot(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-restart-snapshot option: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-snapshot".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-restart-snapshot argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-snapshot".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(SNAPSHOT_PATH));
    }

    let mut report =
        build_launch_evidence_restart_snapshot_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_restart_snapshot_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_restart_snapshot(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_restart_snapshot(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_restart_snapshot_failure_summary(&report),
            field: Some("forge.launch-evidence-restart-snapshot".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_restart_snapshot(
    report: &LaunchEvidenceRestartSnapshotReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_restart_snapshot_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_restart_snapshot_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_restart_snapshot_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceRestartSnapshotReport> {
    let inputs = restart_snapshot_inputs(project);
    let present_inputs = inputs.iter().filter(|input| input.present).count();
    let all_inputs_present = present_inputs == inputs.len();
    let freshness = restart_snapshot_freshness(project, &inputs);
    let checks = vec![
        check(
            "restart-summary-present",
            freshness.restart_summary_present,
            format!("restart summary exists at {RESTART_SUMMARY_PATH}"),
        ),
        check(
            "restart-snapshot-inputs-present",
            all_inputs_present,
            format!(
                "{present_inputs}/{} restart snapshot input(s) are present",
                inputs.len()
            ),
        ),
        check(
            "snapshot-not-older-than-restart-summary",
            freshness.snapshot_not_older_than_restart_summary,
            "restart snapshot is not older than the restart summary".to_string(),
        ),
        check(
            "latest-openable-dx-zed-restart-file",
            true,
            "restart snapshot packages the latest openable DX/Zed restart handoff".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "restart snapshot uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceRestartSnapshotReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        snapshot: LaunchEvidenceRestartSnapshot {
            schema: SNAPSHOT_SCHEMA,
            path: SNAPSHOT_PATH,
            command: "dx forge launch-evidence-restart-snapshot --project <path> --write",
            snapshot_target: "latest-openable-dx-zed-restart-file",
            restart_summary: RESTART_SUMMARY_PATH,
            restart_receipt: RESTART_RECEIPT_PATH,
            restart_manifest: RESTART_MANIFEST_PATH,
            restart_brief: RESTART_BRIEF_PATH,
            input_count: inputs.len(),
            present_inputs,
            display_mode: "openable-json",
            zed_handoff_target: "latest-openable-dx-zed-restart-file",
            reads_runtime_artifact_contents: false,
        },
        inputs,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-restart-summary --project . --write".to_string(),
            "dx forge launch-evidence-restart-snapshot --project . --write".to_string(),
            "dx forge launch-evidence-restart-dispatch --project . --write".to_string(),
            format!("open {SNAPSHOT_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_restart_snapshot_terminal(
    report: &LaunchEvidenceRestartSnapshotReport,
) -> String {
    format!(
        "DX Forge launch evidence restart snapshot\nProject: {}\nPassed: {}\nScore: {}\nSnapshot target: {}\nInputs present: {}/{}\nSnapshot fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.snapshot.snapshot_target,
        report.snapshot.present_inputs,
        report.snapshot.input_count,
        report.freshness.snapshot_not_older_than_restart_summary,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_restart_snapshot_markdown(
    report: &LaunchEvidenceRestartSnapshotReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Restart Snapshot\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Snapshot target: `{}`\n- Snapshot fresh: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.snapshot.snapshot_target,
        report.freshness.snapshot_not_older_than_restart_summary,
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

fn launch_evidence_restart_snapshot_failure_summary(
    report: &LaunchEvidenceRestartSnapshotReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence restart snapshot score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn restart_snapshot_inputs(project: &Path) -> Vec<LaunchEvidenceRestartSnapshotInput> {
    [
        (
            "restart-summary",
            RESTART_SUMMARY_PATH,
            "dx forge launch-evidence-restart-summary --project . --write",
        ),
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
    ]
    .into_iter()
    .map(|(id, path, command)| restart_snapshot_input(project, id, path, command))
    .collect()
}

fn restart_snapshot_input(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceRestartSnapshotInput {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceRestartSnapshotInput {
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

fn restart_snapshot_freshness(
    project: &Path,
    inputs: &[LaunchEvidenceRestartSnapshotInput],
) -> LaunchEvidenceRestartSnapshotFreshness {
    let snapshot_modified =
        file_metadata(&project.join(SNAPSHOT_PATH)).map(|metadata| metadata.modified_at);
    let restart_summary_modified =
        file_metadata(&project.join(RESTART_SUMMARY_PATH)).map(|metadata| metadata.modified_at);
    let snapshot_not_older_than_restart_summary =
        match (snapshot_modified, restart_summary_modified) {
            (Some(snapshot), Some(summary)) => snapshot >= summary,
            (Some(_), None) => true,
            (None, None) => true,
            (None, Some(_)) => false,
        };

    LaunchEvidenceRestartSnapshotFreshness {
        snapshot_path: SNAPSHOT_PATH,
        snapshot_present: snapshot_modified.is_some(),
        snapshot_modified_at: snapshot_modified.map(format_system_time),
        restart_summary_present: input_present(inputs, "restart-summary"),
        snapshot_not_older_than_restart_summary,
        timestamp_source: "filesystem-metadata",
    }
}

fn input_present(inputs: &[LaunchEvidenceRestartSnapshotInput], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-restart-snapshot".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceRestartSnapshotCheck {
    LaunchEvidenceRestartSnapshotCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceRestartSnapshotCheck]) -> u8 {
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
    fn fails_when_restart_summary_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            RESTART_RECEIPT_PATH,
            RESTART_MANIFEST_PATH,
            RESTART_BRIEF_PATH,
        ] {
            write_input(dir.path(), path);
        }

        let report =
            build_launch_evidence_restart_snapshot_report(dir.path(), 100).expect("snapshot");

        assert!(!report.passed());
        assert!(!report.freshness.restart_summary_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "restart-summary-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_restart_snapshot_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), SNAPSHOT_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), RESTART_SUMMARY_PATH);

        let report =
            build_launch_evidence_restart_snapshot_report(dir.path(), 0).expect("snapshot");

        assert!(!report.freshness.snapshot_not_older_than_restart_summary);
    }

    #[test]
    fn passes_complete_fresh_snapshot_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_restart_snapshot_inputs(dir.path());
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), SNAPSHOT_PATH);

        let report =
            build_launch_evidence_restart_snapshot_report(dir.path(), 100).expect("snapshot");

        assert!(report.passed());
        assert_eq!(
            report.snapshot.snapshot_target,
            "latest-openable-dx-zed-restart-file"
        );
        assert!(!report.snapshot.reads_runtime_artifact_contents);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "no-runtime-content-read" && check.passed)
        );
    }

    #[test]
    fn write_mode_creates_fresh_restart_snapshot() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_restart_snapshot_inputs(dir.path());

        run_launch_evidence_restart_snapshot(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write snapshot");

        let report =
            build_launch_evidence_restart_snapshot_report(dir.path(), 100).expect("snapshot");
        assert!(report.passed());
        assert!(dir.path().join(SNAPSHOT_PATH).is_file());
    }

    fn write_restart_snapshot_inputs(project: &Path) {
        for path in [
            RESTART_SUMMARY_PATH,
            RESTART_RECEIPT_PATH,
            RESTART_MANIFEST_PATH,
            RESTART_BRIEF_PATH,
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
