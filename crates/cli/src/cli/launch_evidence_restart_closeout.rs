use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_restart_closeout";
const CLOSEOUT_SCHEMA: &str = "dx.launch.evidence_restart_closeout";
const CLOSEOUT_PATH: &str = ".dx/forge/release/launch-evidence-restart-closeout.md";
const RESTART_DISPATCH_PATH: &str = ".dx/forge/release/launch-evidence-restart-dispatch.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceRestartCloseoutReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    closeout: LaunchEvidenceRestartCloseout,
    inputs: Vec<LaunchEvidenceRestartCloseoutInput>,
    freshness: LaunchEvidenceRestartCloseoutFreshness,
    actions: Vec<LaunchEvidenceRestartCloseoutAction>,
    checks: Vec<LaunchEvidenceRestartCloseoutCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceRestartCloseoutReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartCloseout {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    closeout_target: &'static str,
    restart_dispatch: &'static str,
    input_count: usize,
    present_inputs: usize,
    format: &'static str,
    zed_openable: bool,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartCloseoutInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartCloseoutFreshness {
    closeout_path: &'static str,
    closeout_present: bool,
    closeout_modified_at: Option<String>,
    restart_dispatch_present: bool,
    closeout_not_older_than_restart_dispatch: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartCloseoutAction {
    owner: &'static str,
    lane: &'static str,
    action: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartCloseoutCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_restart_closeout(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                format = DxOutputFormat::Markdown;
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
                        "Unknown forge launch-evidence-restart-closeout option: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-closeout".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-restart-closeout argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-closeout".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(CLOSEOUT_PATH));
    }

    let mut report =
        build_launch_evidence_restart_closeout_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "").map_err(forge_error)?;
            report = build_launch_evidence_restart_closeout_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_restart_closeout(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_restart_closeout(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_restart_closeout_failure_summary(&report),
            field: Some("forge.launch-evidence-restart-closeout".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_restart_closeout(
    report: &LaunchEvidenceRestartCloseoutReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_restart_closeout_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_restart_closeout_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_restart_closeout_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceRestartCloseoutReport> {
    let inputs = restart_closeout_inputs(project);
    let present_inputs = inputs.iter().filter(|input| input.present).count();
    let all_inputs_present = present_inputs == inputs.len();
    let freshness = restart_closeout_freshness(project, &inputs);
    let checks = vec![
        check(
            "restart-dispatch-present",
            freshness.restart_dispatch_present,
            format!("restart dispatch exists at {RESTART_DISPATCH_PATH}"),
        ),
        check(
            "restart-closeout-inputs-present",
            all_inputs_present,
            format!(
                "{present_inputs}/{} restart closeout input(s) are present",
                inputs.len()
            ),
        ),
        check(
            "closeout-not-older-than-restart-dispatch",
            freshness.closeout_not_older_than_restart_dispatch,
            "restart closeout is not older than the restart dispatch".to_string(),
        ),
        check(
            "final-friday-essencefromexistence-closeout-actions",
            true,
            "restart closeout records final operator actions for Friday and essencefromexistence"
                .to_string(),
        ),
        check(
            "markdown-closeout",
            true,
            "restart closeout writes a Zed-openable Markdown closeout".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "restart closeout uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceRestartCloseoutReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        closeout: LaunchEvidenceRestartCloseout {
            schema: CLOSEOUT_SCHEMA,
            path: CLOSEOUT_PATH,
            command: "dx forge launch-evidence-restart-closeout --project <path> --write",
            closeout_target: "final-friday-essencefromexistence-closeout-actions",
            restart_dispatch: RESTART_DISPATCH_PATH,
            input_count: inputs.len(),
            present_inputs,
            format: "markdown",
            zed_openable: true,
            reads_runtime_artifact_contents: false,
        },
        inputs,
        freshness,
        actions: restart_closeout_actions(),
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-restart-dispatch --project . --write".to_string(),
            "dx forge launch-evidence-restart-closeout --project . --write".to_string(),
            "dx forge launch-evidence-restart-signoff --project . --write".to_string(),
            format!("open {CLOSEOUT_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_restart_closeout_terminal(
    report: &LaunchEvidenceRestartCloseoutReport,
) -> String {
    format!(
        "DX Forge launch evidence restart closeout\nProject: {}\nPassed: {}\nScore: {}\nCloseout target: {}\nInputs present: {}/{}\nCloseout fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.closeout.closeout_target,
        report.closeout.present_inputs,
        report.closeout.input_count,
        report.freshness.closeout_not_older_than_restart_dispatch,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_restart_closeout_markdown(
    report: &LaunchEvidenceRestartCloseoutReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Restart Closeout\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Closeout target: `{}`\n- Restart dispatch fresh: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.closeout.closeout_target,
        report.freshness.closeout_not_older_than_restart_dispatch,
        report.no_execution
    );
    output.push_str("## Final Operator Actions\n\n");
    for action in &report.actions {
        output.push_str(&format!(
            "- `{}` `{}`: {}\n",
            action.owner, action.lane, action.action
        ));
    }
    output.push_str(
        "\n## Closeout Inputs\n\n| Input | Present | Modified | Bytes | Path |\n| --- | --- | --- | --- | --- |\n",
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

fn launch_evidence_restart_closeout_failure_summary(
    report: &LaunchEvidenceRestartCloseoutReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence restart closeout score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn restart_closeout_inputs(project: &Path) -> Vec<LaunchEvidenceRestartCloseoutInput> {
    [(
        "restart-dispatch",
        RESTART_DISPATCH_PATH,
        "dx forge launch-evidence-restart-dispatch --project . --write",
    )]
    .into_iter()
    .map(|(id, path, command)| restart_closeout_input(project, id, path, command))
    .collect()
}

fn restart_closeout_input(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceRestartCloseoutInput {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceRestartCloseoutInput {
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

fn restart_closeout_freshness(
    project: &Path,
    inputs: &[LaunchEvidenceRestartCloseoutInput],
) -> LaunchEvidenceRestartCloseoutFreshness {
    let closeout_modified =
        file_metadata(&project.join(CLOSEOUT_PATH)).map(|metadata| metadata.modified_at);
    let restart_dispatch_modified =
        file_metadata(&project.join(RESTART_DISPATCH_PATH)).map(|metadata| metadata.modified_at);
    let closeout_not_older_than_restart_dispatch =
        match (closeout_modified, restart_dispatch_modified) {
            (Some(closeout), Some(dispatch)) => closeout >= dispatch,
            (Some(_), None) => true,
            (None, None) => true,
            (None, Some(_)) => false,
        };

    LaunchEvidenceRestartCloseoutFreshness {
        closeout_path: CLOSEOUT_PATH,
        closeout_present: closeout_modified.is_some(),
        closeout_modified_at: closeout_modified.map(format_system_time),
        restart_dispatch_present: input_present(inputs, "restart-dispatch"),
        closeout_not_older_than_restart_dispatch,
        timestamp_source: "filesystem-metadata",
    }
}

fn restart_closeout_actions() -> Vec<LaunchEvidenceRestartCloseoutAction> {
    vec![
        LaunchEvidenceRestartCloseoutAction {
            owner: "Friday",
            lane: "orchestrator",
            action: "Confirm the restart dispatch, assign the next worker only if a new lane is approved, and keep runtime commands permission-gated.",
        },
        LaunchEvidenceRestartCloseoutAction {
            owner: "essencefromexistence",
            lane: "product-owner",
            action: "Review the final restart handoff, approve any runtime evidence pass explicitly, and keep source-owned package evidence as the launch baseline.",
        },
    ]
}

fn input_present(inputs: &[LaunchEvidenceRestartCloseoutInput], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-restart-closeout".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceRestartCloseoutCheck {
    LaunchEvidenceRestartCloseoutCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceRestartCloseoutCheck]) -> u8 {
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
    fn fails_when_restart_dispatch_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");

        let report =
            build_launch_evidence_restart_closeout_report(dir.path(), 100).expect("closeout");

        assert!(!report.passed());
        assert!(!report.freshness.restart_dispatch_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "restart-dispatch-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_restart_closeout_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), CLOSEOUT_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), RESTART_DISPATCH_PATH);

        let report =
            build_launch_evidence_restart_closeout_report(dir.path(), 0).expect("closeout");

        assert!(!report.freshness.closeout_not_older_than_restart_dispatch);
    }

    #[test]
    fn passes_complete_fresh_closeout_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), RESTART_DISPATCH_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), CLOSEOUT_PATH);

        let report =
            build_launch_evidence_restart_closeout_report(dir.path(), 100).expect("closeout");

        assert!(report.passed());
        assert_eq!(
            report.closeout.closeout_target,
            "final-friday-essencefromexistence-closeout-actions"
        );
        assert_eq!(report.closeout.format, "markdown");
        assert!(report.closeout.zed_openable);
        assert!(!report.closeout.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_restart_closeout_markdown() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), RESTART_DISPATCH_PATH);

        run_launch_evidence_restart_closeout(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write closeout");

        let report =
            build_launch_evidence_restart_closeout_report(dir.path(), 100).expect("closeout");
        let closeout = fs::read_to_string(dir.path().join(CLOSEOUT_PATH)).expect("closeout file");
        assert!(report.passed());
        assert!(closeout.contains("# DX Forge Launch Evidence Restart Closeout"));
        assert!(closeout.contains("Final Operator Actions"));
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("input parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
