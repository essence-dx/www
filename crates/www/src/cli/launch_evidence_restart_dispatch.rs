use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_restart_dispatch";
const DISPATCH_SCHEMA: &str = "dx.launch.evidence_restart_dispatch";
const DISPATCH_PATH: &str = ".dx/forge/release/launch-evidence-restart-dispatch.json";
const RESTART_SNAPSHOT_PATH: &str = ".dx/forge/release/launch-evidence-restart-snapshot.json";
const RESTART_SUMMARY_PATH: &str = ".dx/forge/release/launch-evidence-restart-summary.json";
const RESTART_RECEIPT_PATH: &str = ".dx/forge/release/launch-evidence-restart-receipt.json";
const RESTART_MANIFEST_PATH: &str = ".dx/forge/release/launch-evidence-restart-.dx/build-cache/manifest.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceRestartDispatchReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    dispatch: LaunchEvidenceRestartDispatch,
    inputs: Vec<LaunchEvidenceRestartDispatchInput>,
    freshness: LaunchEvidenceRestartDispatchFreshness,
    checks: Vec<LaunchEvidenceRestartDispatchCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceRestartDispatchReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartDispatch {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    dispatch_target: &'static str,
    restart_snapshot: &'static str,
    restart_summary: &'static str,
    restart_receipt: &'static str,
    restart_manifest: &'static str,
    input_count: usize,
    present_inputs: usize,
    display_mode: &'static str,
    zed_handoff_target: &'static str,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartDispatchInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartDispatchFreshness {
    dispatch_path: &'static str,
    dispatch_present: bool,
    dispatch_modified_at: Option<String>,
    restart_snapshot_present: bool,
    dispatch_not_older_than_restart_snapshot: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartDispatchCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_restart_dispatch(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-restart-dispatch option: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-dispatch".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-restart-dispatch argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-dispatch".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(DISPATCH_PATH));
    }

    let mut report =
        build_launch_evidence_restart_dispatch_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_restart_dispatch_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_restart_dispatch(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_restart_dispatch(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_restart_dispatch_failure_summary(&report),
            field: Some("forge.launch-evidence-restart-dispatch".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_restart_dispatch(
    report: &LaunchEvidenceRestartDispatchReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_restart_dispatch_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_restart_dispatch_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_restart_dispatch_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceRestartDispatchReport> {
    let inputs = restart_dispatch_inputs(project);
    let present_inputs = inputs.iter().filter(|input| input.present).count();
    let all_inputs_present = present_inputs == inputs.len();
    let freshness = restart_dispatch_freshness(project, &inputs);
    let checks = vec![
        check(
            "restart-snapshot-present",
            freshness.restart_snapshot_present,
            format!("restart snapshot exists at {RESTART_SNAPSHOT_PATH}"),
        ),
        check(
            "restart-dispatch-inputs-present",
            all_inputs_present,
            format!(
                "{present_inputs}/{} restart dispatch input(s) are present",
                inputs.len()
            ),
        ),
        check(
            "dispatch-not-older-than-restart-snapshot",
            freshness.dispatch_not_older_than_restart_snapshot,
            "restart dispatch is not older than the restart snapshot".to_string(),
        ),
        check(
            "one-command-next-worker-dispatch-card",
            true,
            "restart dispatch packages a one-command next-worker dispatch card".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "restart dispatch uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceRestartDispatchReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        dispatch: LaunchEvidenceRestartDispatch {
            schema: DISPATCH_SCHEMA,
            path: DISPATCH_PATH,
            command: "dx forge launch-evidence-restart-dispatch --project <path> --write",
            dispatch_target: "one-command-next-worker-dispatch-card",
            restart_snapshot: RESTART_SNAPSHOT_PATH,
            restart_summary: RESTART_SUMMARY_PATH,
            restart_receipt: RESTART_RECEIPT_PATH,
            restart_manifest: RESTART_MANIFEST_PATH,
            input_count: inputs.len(),
            present_inputs,
            display_mode: "next-worker-card",
            zed_handoff_target: "one-command-next-worker-dispatch-card",
            reads_runtime_artifact_contents: false,
        },
        inputs,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-restart-snapshot --project . --write".to_string(),
            "dx forge launch-evidence-restart-dispatch --project . --write".to_string(),
            "dx forge launch-evidence-restart-closeout --project . --write".to_string(),
            format!("open {DISPATCH_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_restart_dispatch_terminal(
    report: &LaunchEvidenceRestartDispatchReport,
) -> String {
    format!(
        "DX Forge launch evidence restart dispatch\nProject: {}\nPassed: {}\nScore: {}\nDispatch target: {}\nInputs present: {}/{}\nDispatch fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.dispatch.dispatch_target,
        report.dispatch.present_inputs,
        report.dispatch.input_count,
        report.freshness.dispatch_not_older_than_restart_snapshot,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_restart_dispatch_markdown(
    report: &LaunchEvidenceRestartDispatchReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Restart Dispatch\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Dispatch target: `{}`\n- Dispatch fresh: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.dispatch.dispatch_target,
        report.freshness.dispatch_not_older_than_restart_snapshot,
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

fn launch_evidence_restart_dispatch_failure_summary(
    report: &LaunchEvidenceRestartDispatchReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence restart dispatch score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn restart_dispatch_inputs(project: &Path) -> Vec<LaunchEvidenceRestartDispatchInput> {
    [
        (
            "restart-snapshot",
            RESTART_SNAPSHOT_PATH,
            "dx forge launch-evidence-restart-snapshot --project . --write",
        ),
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
    ]
    .into_iter()
    .map(|(id, path, command)| restart_dispatch_input(project, id, path, command))
    .collect()
}

fn restart_dispatch_input(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceRestartDispatchInput {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceRestartDispatchInput {
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

fn restart_dispatch_freshness(
    project: &Path,
    inputs: &[LaunchEvidenceRestartDispatchInput],
) -> LaunchEvidenceRestartDispatchFreshness {
    let dispatch_modified =
        file_metadata(&project.join(DISPATCH_PATH)).map(|metadata| metadata.modified_at);
    let restart_snapshot_modified =
        file_metadata(&project.join(RESTART_SNAPSHOT_PATH)).map(|metadata| metadata.modified_at);
    let dispatch_not_older_than_restart_snapshot =
        match (dispatch_modified, restart_snapshot_modified) {
            (Some(dispatch), Some(snapshot)) => dispatch >= snapshot,
            (Some(_), None) => true,
            (None, None) => true,
            (None, Some(_)) => false,
        };

    LaunchEvidenceRestartDispatchFreshness {
        dispatch_path: DISPATCH_PATH,
        dispatch_present: dispatch_modified.is_some(),
        dispatch_modified_at: dispatch_modified.map(format_system_time),
        restart_snapshot_present: input_present(inputs, "restart-snapshot"),
        dispatch_not_older_than_restart_snapshot,
        timestamp_source: "filesystem-metadata",
    }
}

fn input_present(inputs: &[LaunchEvidenceRestartDispatchInput], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-restart-dispatch".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceRestartDispatchCheck {
    LaunchEvidenceRestartDispatchCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceRestartDispatchCheck]) -> u8 {
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
    fn fails_when_restart_snapshot_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            RESTART_SUMMARY_PATH,
            RESTART_RECEIPT_PATH,
            RESTART_MANIFEST_PATH,
        ] {
            write_input(dir.path(), path);
        }

        let report =
            build_launch_evidence_restart_dispatch_report(dir.path(), 100).expect("dispatch");

        assert!(!report.passed());
        assert!(!report.freshness.restart_snapshot_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "restart-snapshot-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_restart_dispatch_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), DISPATCH_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), RESTART_SNAPSHOT_PATH);

        let report =
            build_launch_evidence_restart_dispatch_report(dir.path(), 0).expect("dispatch");

        assert!(!report.freshness.dispatch_not_older_than_restart_snapshot);
    }

    #[test]
    fn passes_complete_fresh_dispatch_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_restart_dispatch_inputs(dir.path());
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), DISPATCH_PATH);

        let report =
            build_launch_evidence_restart_dispatch_report(dir.path(), 100).expect("dispatch");

        assert!(report.passed());
        assert_eq!(
            report.dispatch.dispatch_target,
            "one-command-next-worker-dispatch-card"
        );
        assert!(!report.dispatch.reads_runtime_artifact_contents);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "no-runtime-content-read" && check.passed)
        );
    }

    #[test]
    fn write_mode_creates_fresh_restart_dispatch() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_restart_dispatch_inputs(dir.path());

        run_launch_evidence_restart_dispatch(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write dispatch");

        let report =
            build_launch_evidence_restart_dispatch_report(dir.path(), 100).expect("dispatch");
        assert!(report.passed());
        assert!(dir.path().join(DISPATCH_PATH).is_file());
    }

    fn write_restart_dispatch_inputs(project: &Path) {
        for path in [
            RESTART_SNAPSHOT_PATH,
            RESTART_SUMMARY_PATH,
            RESTART_RECEIPT_PATH,
            RESTART_MANIFEST_PATH,
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
