use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_restart_brief";
const BRIEF_SCHEMA: &str = "dx.launch.evidence_restart_brief";
const BRIEF_PATH: &str = ".dx/forge/release/launch-evidence-restart-brief.md";
const RESTART_CHECKLIST_PATH: &str = ".dx/forge/release/launch-evidence-restart-checklist.json";
const RESTART_LEDGER_PATH: &str = ".dx/forge/release/launch-evidence-restart-ledger.json";
const OPERATOR_RESUME_CARD_PATH: &str =
    ".dx/forge/release/launch-evidence-operator-resume-card.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceRestartBriefReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    brief: LaunchEvidenceRestartBrief,
    inputs: Vec<LaunchEvidenceRestartBriefInput>,
    freshness: LaunchEvidenceRestartBriefFreshness,
    checks: Vec<LaunchEvidenceRestartBriefCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceRestartBriefReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartBrief {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    brief_target: &'static str,
    format: &'static str,
    restart_checklist: &'static str,
    input_count: usize,
    present_inputs: usize,
    zed_openable: bool,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartBriefInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartBriefFreshness {
    brief_path: &'static str,
    brief_present: bool,
    brief_modified_at: Option<String>,
    restart_checklist_present: bool,
    brief_not_older_than_restart_checklist: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartBriefCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_restart_brief(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                    message: format!("Unknown forge launch-evidence-restart-brief option: {value}"),
                    field: Some("forge.launch-evidence-restart-brief".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-restart-brief argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-brief".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(BRIEF_PATH));
    }

    let mut report =
        build_launch_evidence_restart_brief_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "").map_err(forge_error)?;
            report = build_launch_evidence_restart_brief_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_restart_brief(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_restart_brief(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_restart_brief_failure_summary(&report),
            field: Some("forge.launch-evidence-restart-brief".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_restart_brief(
    report: &LaunchEvidenceRestartBriefReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_restart_brief_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_restart_brief_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_restart_brief_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceRestartBriefReport> {
    let inputs = restart_brief_inputs(project);
    let present_inputs = inputs.iter().filter(|input| input.present).count();
    let all_inputs_present = present_inputs == inputs.len();
    let freshness = restart_brief_freshness(project, &inputs);
    let checks = vec![
        check(
            "restart-checklist-present",
            freshness.restart_checklist_present,
            format!("restart checklist exists at {RESTART_CHECKLIST_PATH}"),
        ),
        check(
            "restart-brief-inputs-present",
            all_inputs_present,
            format!(
                "{present_inputs}/{} restart brief input(s) are present",
                inputs.len()
            ),
        ),
        check(
            "restart-brief-freshness",
            freshness.brief_not_older_than_restart_checklist,
            "restart brief is not older than the restart checklist".to_string(),
        ),
        check(
            "zed-openable-dx-restart-brief",
            true,
            "restart brief writes a compact Zed-openable Markdown handoff".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "restart brief uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceRestartBriefReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        brief: LaunchEvidenceRestartBrief {
            schema: BRIEF_SCHEMA,
            path: BRIEF_PATH,
            command: "dx forge launch-evidence-restart-brief --project <path> --write",
            brief_target: "zed-openable-dx-restart-brief",
            format: "markdown",
            restart_checklist: RESTART_CHECKLIST_PATH,
            input_count: inputs.len(),
            present_inputs,
            zed_openable: true,
            reads_runtime_artifact_contents: false,
        },
        inputs,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-restart-checklist --project . --write".to_string(),
            "dx forge launch-evidence-restart-brief --project . --write".to_string(),
            "dx forge launch-evidence-restart-manifest --project . --write".to_string(),
            format!("open {BRIEF_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_restart_brief_terminal(
    report: &LaunchEvidenceRestartBriefReport,
) -> String {
    format!(
        "DX Forge launch evidence restart brief\nProject: {}\nPassed: {}\nScore: {}\nBrief target: {}\nInputs present: {}/{}\nBrief fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.brief.brief_target,
        report.brief.present_inputs,
        report.brief.input_count,
        report.freshness.brief_not_older_than_restart_checklist,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_restart_brief_markdown(
    report: &LaunchEvidenceRestartBriefReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Restart Brief\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Brief target: `{}`\n- Restart checklist fresh: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.brief.brief_target,
        report.freshness.brief_not_older_than_restart_checklist,
        report.no_execution
    );
    output.push_str("## Next Actions\n\n");
    output.push_str("- `source-only`: run the launch source guard before the next commit.\n");
    output
        .push_str("- `runtime-approved`: collect runtime evidence only after explicit approval.\n");
    output.push_str(
        "- `release-closeout`: refresh restart handoff artifacts after source or runtime changes.\n",
    );
    output.push_str(
        "\n## Brief Inputs\n\n| Input | Present | Modified | Bytes | Path |\n| --- | --- | --- | --- | --- |\n",
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

pub(crate) fn launch_evidence_restart_brief_failure_summary(
    report: &LaunchEvidenceRestartBriefReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence restart brief score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn restart_brief_inputs(project: &Path) -> Vec<LaunchEvidenceRestartBriefInput> {
    [
        (
            "restart-checklist",
            RESTART_CHECKLIST_PATH,
            "dx forge launch-evidence-restart-checklist --project . --write",
        ),
        (
            "restart-ledger",
            RESTART_LEDGER_PATH,
            "dx forge launch-evidence-restart-ledger --project . --write",
        ),
        (
            "operator-resume-card",
            OPERATOR_RESUME_CARD_PATH,
            "dx forge launch-evidence-operator-resume-card --project . --write",
        ),
    ]
    .into_iter()
    .map(|(id, path, command)| restart_brief_input(project, id, path, command))
    .collect()
}

fn restart_brief_input(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceRestartBriefInput {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceRestartBriefInput {
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

fn restart_brief_freshness(
    project: &Path,
    inputs: &[LaunchEvidenceRestartBriefInput],
) -> LaunchEvidenceRestartBriefFreshness {
    let brief_modified =
        file_metadata(&project.join(BRIEF_PATH)).map(|metadata| metadata.modified_at);
    let checklist_modified =
        file_metadata(&project.join(RESTART_CHECKLIST_PATH)).map(|metadata| metadata.modified_at);
    let brief_not_older_than_restart_checklist = match (brief_modified, checklist_modified) {
        (Some(brief), Some(checklist)) => brief >= checklist,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceRestartBriefFreshness {
        brief_path: BRIEF_PATH,
        brief_present: brief_modified.is_some(),
        brief_modified_at: brief_modified.map(format_system_time),
        restart_checklist_present: input_present(inputs, "restart-checklist"),
        brief_not_older_than_restart_checklist,
        timestamp_source: "filesystem-metadata",
    }
}

fn input_present(inputs: &[LaunchEvidenceRestartBriefInput], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-restart-brief".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceRestartBriefCheck {
    LaunchEvidenceRestartBriefCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceRestartBriefCheck]) -> u8 {
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
    fn fails_when_restart_checklist_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), RESTART_LEDGER_PATH);
        write_input(dir.path(), OPERATOR_RESUME_CARD_PATH);

        let report = build_launch_evidence_restart_brief_report(dir.path(), 100).expect("brief");

        assert!(!report.passed());
        assert!(!report.freshness.restart_checklist_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "restart-checklist-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_restart_brief_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), BRIEF_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), RESTART_CHECKLIST_PATH);

        let report = build_launch_evidence_restart_brief_report(dir.path(), 0).expect("brief");

        assert!(!report.freshness.brief_not_older_than_restart_checklist);
    }

    #[test]
    fn passes_complete_fresh_brief_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_restart_brief_inputs(dir.path());
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), BRIEF_PATH);

        let report = build_launch_evidence_restart_brief_report(dir.path(), 100).expect("brief");

        assert!(report.passed());
        assert_eq!(report.brief.brief_target, "zed-openable-dx-restart-brief");
        assert_eq!(report.brief.format, "markdown");
        assert!(report.brief.zed_openable);
        assert!(!report.brief.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_restart_brief() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_restart_brief_inputs(dir.path());

        run_launch_evidence_restart_brief(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write brief");

        let report = build_launch_evidence_restart_brief_report(dir.path(), 100).expect("brief");
        assert!(report.passed());
        assert!(dir.path().join(BRIEF_PATH).is_file());
    }

    fn write_restart_brief_inputs(project: &Path) {
        for path in [
            RESTART_CHECKLIST_PATH,
            RESTART_LEDGER_PATH,
            OPERATOR_RESUME_CARD_PATH,
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
