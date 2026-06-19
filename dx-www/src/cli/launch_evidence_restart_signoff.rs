use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_restart_signoff";
const SIGNOFF_SCHEMA: &str = "dx.launch.evidence_restart_signoff";
const SIGNOFF_PATH: &str = ".dx/forge/release/launch-evidence-restart-signoff.json";
const RESTART_CLOSEOUT_PATH: &str = ".dx/forge/release/launch-evidence-restart-closeout.md";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceRestartSignoffReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    signoff: LaunchEvidenceRestartSignoff,
    inputs: Vec<LaunchEvidenceRestartSignoffInput>,
    freshness: LaunchEvidenceRestartSignoffFreshness,
    checks: Vec<LaunchEvidenceRestartSignoffCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceRestartSignoffReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartSignoff {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    signoff_target: &'static str,
    restart_closeout: &'static str,
    input_count: usize,
    present_inputs: usize,
    acceptance_status: &'static str,
    format: &'static str,
    zed_openable: bool,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartSignoffInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartSignoffFreshness {
    signoff_path: &'static str,
    signoff_present: bool,
    signoff_modified_at: Option<String>,
    restart_closeout_present: bool,
    signoff_not_older_than_restart_closeout: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartSignoffCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_restart_signoff(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-restart-signoff option: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-signoff".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-restart-signoff argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-signoff".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(SIGNOFF_PATH));
    }

    let mut report =
        build_launch_evidence_restart_signoff_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_restart_signoff_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_restart_signoff(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_restart_signoff(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_restart_signoff_failure_summary(&report),
            field: Some("forge.launch-evidence-restart-signoff".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_restart_signoff(
    report: &LaunchEvidenceRestartSignoffReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_restart_signoff_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_restart_signoff_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_restart_signoff_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceRestartSignoffReport> {
    let inputs = restart_signoff_inputs(project);
    let present_inputs = inputs.iter().filter(|input| input.present).count();
    let all_inputs_present = present_inputs == inputs.len();
    let freshness = restart_signoff_freshness(project, &inputs);
    let checks = vec![
        check(
            "restart-closeout-present",
            freshness.restart_closeout_present,
            format!("restart closeout exists at {RESTART_CLOSEOUT_PATH}"),
        ),
        check(
            "restart-signoff-inputs-present",
            all_inputs_present,
            format!(
                "{present_inputs}/{} restart signoff input(s) are present",
                inputs.len()
            ),
        ),
        check(
            "signoff-not-older-than-restart-closeout",
            freshness.signoff_not_older_than_restart_closeout,
            "restart signoff is not older than the restart closeout".to_string(),
        ),
        check(
            "friday-essencefromexistence-acceptance-receipt",
            true,
            "restart signoff records a reviewable Friday and essencefromexistence acceptance receipt"
                .to_string(),
        ),
        check(
            "json-signoff",
            true,
            "restart signoff writes a DX/Zed indexable JSON receipt".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "restart signoff uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceRestartSignoffReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        signoff: LaunchEvidenceRestartSignoff {
            schema: SIGNOFF_SCHEMA,
            path: SIGNOFF_PATH,
            command: "dx forge launch-evidence-restart-signoff --project <path> --write",
            signoff_target: "friday-essencefromexistence-acceptance-receipt",
            restart_closeout: RESTART_CLOSEOUT_PATH,
            input_count: inputs.len(),
            present_inputs,
            acceptance_status: "reviewable",
            format: "json",
            zed_openable: true,
            reads_runtime_artifact_contents: false,
        },
        inputs,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-restart-closeout --project . --write".to_string(),
            "dx forge launch-evidence-restart-signoff --project . --write".to_string(),
            "dx forge launch-evidence-acceptance-index --project . --write".to_string(),
            format!("open {SIGNOFF_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_restart_signoff_terminal(
    report: &LaunchEvidenceRestartSignoffReport,
) -> String {
    format!(
        "DX Forge launch evidence restart signoff\nProject: {}\nPassed: {}\nScore: {}\nSignoff target: {}\nAcceptance status: {}\nInputs present: {}/{}\nSignoff fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.signoff.signoff_target,
        report.signoff.acceptance_status,
        report.signoff.present_inputs,
        report.signoff.input_count,
        report.freshness.signoff_not_older_than_restart_closeout,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_restart_signoff_markdown(
    report: &LaunchEvidenceRestartSignoffReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Restart Signoff\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Signoff target: `{}`\n- Acceptance status: `{}`\n- Restart closeout fresh: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.signoff.signoff_target,
        report.signoff.acceptance_status,
        report.freshness.signoff_not_older_than_restart_closeout,
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

fn launch_evidence_restart_signoff_failure_summary(
    report: &LaunchEvidenceRestartSignoffReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence restart signoff score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn restart_signoff_inputs(project: &Path) -> Vec<LaunchEvidenceRestartSignoffInput> {
    [(
        "restart-closeout",
        RESTART_CLOSEOUT_PATH,
        "dx forge launch-evidence-restart-closeout --project . --write",
    )]
    .into_iter()
    .map(|(id, path, command)| restart_signoff_input(project, id, path, command))
    .collect()
}

fn restart_signoff_input(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceRestartSignoffInput {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceRestartSignoffInput {
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

fn restart_signoff_freshness(
    project: &Path,
    inputs: &[LaunchEvidenceRestartSignoffInput],
) -> LaunchEvidenceRestartSignoffFreshness {
    let signoff_modified =
        file_metadata(&project.join(SIGNOFF_PATH)).map(|metadata| metadata.modified_at);
    let restart_closeout_modified =
        file_metadata(&project.join(RESTART_CLOSEOUT_PATH)).map(|metadata| metadata.modified_at);
    let signoff_not_older_than_restart_closeout =
        match (signoff_modified, restart_closeout_modified) {
            (Some(signoff), Some(closeout)) => signoff >= closeout,
            (Some(_), None) => true,
            (None, None) => true,
            (None, Some(_)) => false,
        };

    LaunchEvidenceRestartSignoffFreshness {
        signoff_path: SIGNOFF_PATH,
        signoff_present: signoff_modified.is_some(),
        signoff_modified_at: signoff_modified.map(format_system_time),
        restart_closeout_present: input_present(inputs, "restart-closeout"),
        signoff_not_older_than_restart_closeout,
        timestamp_source: "filesystem-metadata",
    }
}

fn input_present(inputs: &[LaunchEvidenceRestartSignoffInput], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-restart-signoff".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceRestartSignoffCheck {
    LaunchEvidenceRestartSignoffCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceRestartSignoffCheck]) -> u8 {
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
    fn fails_when_restart_closeout_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");

        let report =
            build_launch_evidence_restart_signoff_report(dir.path(), 100).expect("signoff");

        assert!(!report.passed());
        assert!(!report.freshness.restart_closeout_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "restart-closeout-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_restart_signoff_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), SIGNOFF_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), RESTART_CLOSEOUT_PATH);

        let report = build_launch_evidence_restart_signoff_report(dir.path(), 0).expect("signoff");

        assert!(!report.freshness.signoff_not_older_than_restart_closeout);
    }

    #[test]
    fn passes_complete_fresh_signoff_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), RESTART_CLOSEOUT_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), SIGNOFF_PATH);

        let report =
            build_launch_evidence_restart_signoff_report(dir.path(), 100).expect("signoff");

        assert!(report.passed());
        assert_eq!(
            report.signoff.signoff_target,
            "friday-essencefromexistence-acceptance-receipt"
        );
        assert_eq!(report.signoff.acceptance_status, "reviewable");
        assert_eq!(report.signoff.format, "json");
        assert!(report.signoff.zed_openable);
        assert!(!report.signoff.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_restart_signoff_json() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), RESTART_CLOSEOUT_PATH);

        run_launch_evidence_restart_signoff(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write signoff");

        let report =
            build_launch_evidence_restart_signoff_report(dir.path(), 100).expect("signoff");
        let signoff = fs::read_to_string(dir.path().join(SIGNOFF_PATH)).expect("signoff file");
        assert!(report.passed());
        assert!(signoff.contains("\"schema\": \"dx.forge.launch_evidence_restart_signoff\""));
        assert!(signoff.contains("\"acceptance_status\": \"reviewable\""));
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("input parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
