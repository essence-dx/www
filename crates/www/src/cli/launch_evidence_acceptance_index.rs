use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_acceptance_index";
const INDEX_SCHEMA: &str = "dx.launch.evidence_acceptance_index";
const INDEX_PATH: &str = ".dx/forge/release/launch-evidence-acceptance-index.md";
const RESTART_SIGNOFF_PATH: &str = ".dx/forge/release/launch-evidence-restart-signoff.json";
const RESTART_CLOSEOUT_PATH: &str = ".dx/forge/release/launch-evidence-restart-closeout.md";
const RESTART_DISPATCH_PATH: &str = ".dx/forge/release/launch-evidence-restart-dispatch.json";
const RESTART_SNAPSHOT_PATH: &str = ".dx/forge/release/launch-evidence-restart-snapshot.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceAcceptanceIndexReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    index: LaunchEvidenceAcceptanceIndex,
    inputs: Vec<LaunchEvidenceAcceptanceIndexInput>,
    freshness: LaunchEvidenceAcceptanceIndexFreshness,
    checks: Vec<LaunchEvidenceAcceptanceIndexCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceAcceptanceIndexReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceAcceptanceIndex {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    acceptance_target: &'static str,
    restart_signoff: &'static str,
    restart_closeout: &'static str,
    restart_dispatch: &'static str,
    restart_snapshot: &'static str,
    format: &'static str,
    zed_openable: bool,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceAcceptanceIndexInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceAcceptanceIndexFreshness {
    index_path: &'static str,
    index_present: bool,
    index_modified_at: Option<String>,
    restart_signoff_present: bool,
    index_not_older_than_restart_signoff: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceAcceptanceIndexCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_acceptance_index(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-acceptance-index option: {value}"
                    ),
                    field: Some("forge.launch-evidence-acceptance-index".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-acceptance-index argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-acceptance-index".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(INDEX_PATH));
    }

    let mut report =
        build_launch_evidence_acceptance_index_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "# DX Forge Launch Evidence Acceptance Index\n")
                .map_err(forge_error)?;
            report = build_launch_evidence_acceptance_index_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_acceptance_index(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_acceptance_index(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_acceptance_index_failure_summary(&report),
            field: Some("forge.launch-evidence-acceptance-index".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_acceptance_index(
    report: &LaunchEvidenceAcceptanceIndexReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_acceptance_index_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_acceptance_index_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_acceptance_index_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceAcceptanceIndexReport> {
    let inputs = acceptance_index_inputs(project);
    let all_inputs_present = inputs.iter().all(|input| input.present);
    let freshness = acceptance_index_freshness(project);
    let checks = vec![
        check(
            "restart-signoff-present",
            freshness.restart_signoff_present,
            format!("restart signoff exists at {RESTART_SIGNOFF_PATH}"),
        ),
        check(
            "acceptance-index-fresh",
            freshness.index_not_older_than_restart_signoff,
            "acceptance index is not older than restart-signoff".to_string(),
        ),
        check(
            "restart-handoff-inputs-present",
            all_inputs_present,
            "restart-signoff, restart-closeout, restart-dispatch, and restart-snapshot are present"
                .to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "acceptance index reads file metadata only; no runtime artifact bodies are read"
                .to_string(),
        ),
    ];
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .collect::<Vec<_>>();
    let score = if findings.is_empty() { 100 } else { 75 };
    let passed = findings.is_empty() && score >= fail_under;

    Ok(LaunchEvidenceAcceptanceIndexReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        index: LaunchEvidenceAcceptanceIndex {
            schema: INDEX_SCHEMA,
            path: INDEX_PATH,
            command: "dx forge launch-evidence-acceptance-index --project <path> --write",
            acceptance_target: "friday-final-handoff-index",
            restart_signoff: RESTART_SIGNOFF_PATH,
            restart_closeout: RESTART_CLOSEOUT_PATH,
            restart_dispatch: RESTART_DISPATCH_PATH,
            restart_snapshot: RESTART_SNAPSHOT_PATH,
            format: "markdown",
            zed_openable: true,
            reads_runtime_artifact_contents: false,
        },
        inputs,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-acceptance-index --project . --write".to_string(),
            "dx forge launch-evidence-acceptance-digest --project . --write".to_string(),
            "dx forge launch-evidence-restart-signoff --project . --write".to_string(),
            format!("open {INDEX_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_acceptance_index_terminal(
    report: &LaunchEvidenceAcceptanceIndexReport,
) -> String {
    format!(
        "DX Forge Launch Evidence Acceptance Index\nSchema: {}\nStatus: {}\nScore: {}/100\nTarget: {}\n",
        report.schema,
        if report.passed { "ready" } else { "blocked" },
        report.score,
        report.index.acceptance_target
    )
}

pub(crate) fn launch_evidence_acceptance_index_markdown(
    report: &LaunchEvidenceAcceptanceIndexReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Acceptance Index\n\n- Schema: `{}`\n- Status: `{}`\n- Score: `{}/100`\n- Acceptance target: `{}`\n- No execution: `{}`\n\n## Inputs\n\n",
        report.schema,
        if report.passed { "ready" } else { "blocked" },
        report.score,
        report.index.acceptance_target,
        report.no_execution
    );
    for input in &report.inputs {
        output.push_str(&format!(
            "- `{}`: `{}` at `{}`\n",
            input.id,
            if input.present { "present" } else { "missing" },
            input.path
        ));
    }
    output.push_str("\n## Checks\n\n");
    for check in &report.checks {
        output.push_str(&format!(
            "- `{}`: {} - {}\n",
            check.name,
            if check.passed { "pass" } else { "fail" },
            check.message
        ));
    }
    output
}

fn launch_evidence_acceptance_index_failure_summary(
    report: &LaunchEvidenceAcceptanceIndexReport,
) -> String {
    report
        .findings
        .first()
        .cloned()
        .unwrap_or_else(|| "launch evidence acceptance index is not ready".to_string())
}

fn acceptance_index_inputs(project: &Path) -> Vec<LaunchEvidenceAcceptanceIndexInput> {
    [
        (
            "restart-signoff",
            RESTART_SIGNOFF_PATH,
            "dx forge launch-evidence-restart-signoff --project <path> --write",
        ),
        (
            "restart-closeout",
            RESTART_CLOSEOUT_PATH,
            "dx forge launch-evidence-restart-closeout --project <path> --write",
        ),
        (
            "restart-dispatch",
            RESTART_DISPATCH_PATH,
            "dx forge launch-evidence-restart-dispatch --project <path> --write",
        ),
        (
            "restart-snapshot",
            RESTART_SNAPSHOT_PATH,
            "dx forge launch-evidence-restart-snapshot --project <path> --write",
        ),
    ]
    .into_iter()
    .map(|(id, path, command)| acceptance_index_input(project, id, path, command))
    .collect()
}

fn acceptance_index_input(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceAcceptanceIndexInput {
    let target = project.join(path);
    let metadata = target.metadata().ok();
    LaunchEvidenceAcceptanceIndexInput {
        id,
        path,
        present: metadata.as_ref().is_some_and(|metadata| metadata.is_file()),
        modified_at: metadata
            .as_ref()
            .and_then(|metadata| metadata.modified().ok())
            .map(format_time),
        bytes: metadata.map(|metadata| metadata.len()),
        command,
    }
}

fn acceptance_index_freshness(project: &Path) -> LaunchEvidenceAcceptanceIndexFreshness {
    let index_modified = modified_at(project.join(INDEX_PATH));
    let signoff_modified = modified_at(project.join(RESTART_SIGNOFF_PATH));
    LaunchEvidenceAcceptanceIndexFreshness {
        index_path: INDEX_PATH,
        index_present: index_modified.is_some(),
        index_modified_at: index_modified.map(format_time),
        restart_signoff_present: signoff_modified.is_some(),
        index_not_older_than_restart_signoff: match (index_modified, signoff_modified) {
            (Some(index), Some(signoff)) => index >= signoff,
            _ => false,
        },
        timestamp_source: "filesystem-metadata",
    }
}

fn modified_at(path: PathBuf) -> Option<SystemTime> {
    path.metadata().ok()?.modified().ok()
}

fn format_time(time: SystemTime) -> String {
    DateTime::<Utc>::from(time).to_rfc3339()
}

fn check(
    name: &'static str,
    passed: bool,
    message: impl Into<String>,
) -> LaunchEvidenceAcceptanceIndexCheck {
    LaunchEvidenceAcceptanceIndexCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message: message.into(),
    }
}

fn required_arg<'a>(args: &'a [String], index: usize, name: &str) -> DxResult<&'a str> {
    args.get(index + 1)
        .map(String::as_str)
        .ok_or_else(|| DxError::ConfigValidationError {
            message: format!("{name} requires a value"),
            field: Some("forge.launch-evidence-acceptance-index".to_string()),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fails_when_restart_signoff_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");

        let report = build_launch_evidence_acceptance_index_report(dir.path(), 100).expect("index");

        assert!(!report.passed());
        assert!(!report.freshness.restart_signoff_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "restart-signoff-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_acceptance_index_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), INDEX_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), RESTART_SIGNOFF_PATH);

        let report = build_launch_evidence_acceptance_index_report(dir.path(), 0).expect("index");

        assert!(!report.freshness.index_not_older_than_restart_signoff);
    }

    #[test]
    fn passes_complete_fresh_acceptance_index_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            RESTART_SIGNOFF_PATH,
            RESTART_CLOSEOUT_PATH,
            RESTART_DISPATCH_PATH,
            RESTART_SNAPSHOT_PATH,
        ] {
            write_input(dir.path(), path);
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), INDEX_PATH);

        let report = build_launch_evidence_acceptance_index_report(dir.path(), 100).expect("index");

        assert!(report.passed());
        assert_eq!(report.index.acceptance_target, "friday-final-handoff-index");
        assert_eq!(report.index.format, "markdown");
        assert!(report.index.zed_openable);
        assert!(!report.index.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_acceptance_index_markdown() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            RESTART_SIGNOFF_PATH,
            RESTART_CLOSEOUT_PATH,
            RESTART_DISPATCH_PATH,
            RESTART_SNAPSHOT_PATH,
        ] {
            write_input(dir.path(), path);
        }

        run_launch_evidence_acceptance_index(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write acceptance index");

        let report = build_launch_evidence_acceptance_index_report(dir.path(), 100).expect("index");
        let index = fs::read_to_string(dir.path().join(INDEX_PATH)).expect("index file");
        assert!(report.passed());
        assert!(index.contains("# DX Forge Launch Evidence Acceptance Index"));
        assert!(index.contains("friday-final-handoff-index"));
        assert!(index.contains("restart-signoff"));
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("input parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
