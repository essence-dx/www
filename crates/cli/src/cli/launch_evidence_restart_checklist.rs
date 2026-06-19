use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_restart_checklist";
const CHECKLIST_SCHEMA: &str = "dx.launch.evidence_restart_checklist";
const CHECKLIST_PATH: &str = ".dx/forge/release/launch-evidence-restart-checklist.json";
const RESTART_LEDGER_PATH: &str = ".dx/forge/release/launch-evidence-restart-ledger.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceRestartChecklistReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    checklist: LaunchEvidenceRestartChecklist,
    inputs: Vec<LaunchEvidenceRestartChecklistInput>,
    freshness: LaunchEvidenceRestartChecklistFreshness,
    checks: Vec<LaunchEvidenceRestartChecklistCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceRestartChecklistReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartChecklist {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    checklist_target: &'static str,
    restart_ledger: &'static str,
    lanes: Vec<LaunchEvidenceRestartLane>,
    input_count: usize,
    present_inputs: usize,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartLane {
    id: &'static str,
    action: &'static str,
    requires_permission: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartChecklistInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartChecklistFreshness {
    checklist_path: &'static str,
    checklist_present: bool,
    checklist_modified_at: Option<String>,
    restart_ledger_present: bool,
    checklist_not_older_than_restart_ledger: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartChecklistCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_restart_checklist(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-restart-checklist option: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-checklist".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-restart-checklist argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-checklist".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(CHECKLIST_PATH));
    }

    let mut report = build_launch_evidence_restart_checklist_report(&project, fail_under)
        .map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_restart_checklist_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_restart_checklist(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_restart_checklist(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_restart_checklist_failure_summary(&report),
            field: Some("forge.launch-evidence-restart-checklist".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_restart_checklist(
    report: &LaunchEvidenceRestartChecklistReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_restart_checklist_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_restart_checklist_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_restart_checklist_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceRestartChecklistReport> {
    let inputs = restart_checklist_inputs(project);
    let present_inputs = inputs.iter().filter(|input| input.present).count();
    let all_inputs_present = present_inputs == inputs.len();
    let freshness = restart_checklist_freshness(project, &inputs);
    let checks = vec![
        check(
            "restart-ledger-present",
            freshness.restart_ledger_present,
            format!("restart ledger exists at {RESTART_LEDGER_PATH}"),
        ),
        check(
            "restart-checklist-inputs-present",
            all_inputs_present,
            format!(
                "{present_inputs}/{} restart checklist input(s) are present",
                inputs.len()
            ),
        ),
        check(
            "restart-checklist-freshness",
            freshness.checklist_not_older_than_restart_ledger,
            "restart checklist is not older than the restart ledger".to_string(),
        ),
        check(
            "dx-cli-zed-restart-next-actions",
            true,
            "restart checklist records source-only, runtime-approved, and release-closeout actions"
                .to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "restart checklist uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceRestartChecklistReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        checklist: LaunchEvidenceRestartChecklist {
            schema: CHECKLIST_SCHEMA,
            path: CHECKLIST_PATH,
            command: "dx forge launch-evidence-restart-checklist --project <path> --write",
            checklist_target: "dx-cli-zed-restart-next-actions",
            restart_ledger: RESTART_LEDGER_PATH,
            lanes: restart_lanes(),
            input_count: inputs.len(),
            present_inputs,
            reads_runtime_artifact_contents: false,
        },
        inputs,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-restart-ledger --project . --write".to_string(),
            "dx forge launch-evidence-restart-checklist --project . --write".to_string(),
            "dx forge launch-evidence-restart-brief --project . --write".to_string(),
            format!("open {CHECKLIST_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_restart_checklist_terminal(
    report: &LaunchEvidenceRestartChecklistReport,
) -> String {
    format!(
        "DX Forge launch evidence restart checklist\nProject: {}\nPassed: {}\nScore: {}\nChecklist target: {}\nInputs present: {}/{}\nChecklist fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.checklist.checklist_target,
        report.checklist.present_inputs,
        report.checklist.input_count,
        report.freshness.checklist_not_older_than_restart_ledger,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_restart_checklist_markdown(
    report: &LaunchEvidenceRestartChecklistReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Restart Checklist\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Checklist target: `{}`\n- Restart ledger fresh: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.checklist.checklist_target,
        report.freshness.checklist_not_older_than_restart_ledger,
        report.no_execution
    );
    output.push_str(
        "## Next Actions\n\n| Lane | Action | Runtime permission |\n| --- | --- | --- |\n",
    );
    for lane in &report.checklist.lanes {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` |\n",
            lane.id, lane.action, lane.requires_permission
        ));
    }
    output.push_str(
        "\n## Checklist Inputs\n\n| Input | Present | Modified | Bytes | Path |\n| --- | --- | --- | --- | --- |\n",
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

pub(crate) fn launch_evidence_restart_checklist_failure_summary(
    report: &LaunchEvidenceRestartChecklistReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence restart checklist score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn restart_checklist_inputs(project: &Path) -> Vec<LaunchEvidenceRestartChecklistInput> {
    [(
        "restart-ledger",
        RESTART_LEDGER_PATH,
        "dx forge launch-evidence-restart-ledger --project . --write",
    )]
    .into_iter()
    .map(|(id, path, command)| restart_checklist_input(project, id, path, command))
    .collect()
}

fn restart_checklist_input(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceRestartChecklistInput {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceRestartChecklistInput {
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

fn restart_checklist_freshness(
    project: &Path,
    inputs: &[LaunchEvidenceRestartChecklistInput],
) -> LaunchEvidenceRestartChecklistFreshness {
    let checklist_modified =
        file_metadata(&project.join(CHECKLIST_PATH)).map(|metadata| metadata.modified_at);
    let ledger_modified =
        file_metadata(&project.join(RESTART_LEDGER_PATH)).map(|metadata| metadata.modified_at);
    let checklist_not_older_than_restart_ledger = match (checklist_modified, ledger_modified) {
        (Some(checklist), Some(ledger)) => checklist >= ledger,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceRestartChecklistFreshness {
        checklist_path: CHECKLIST_PATH,
        checklist_present: checklist_modified.is_some(),
        checklist_modified_at: checklist_modified.map(format_system_time),
        restart_ledger_present: input_present(inputs, "restart-ledger"),
        checklist_not_older_than_restart_ledger,
        timestamp_source: "filesystem-metadata",
    }
}

fn restart_lanes() -> Vec<LaunchEvidenceRestartLane> {
    vec![
        LaunchEvidenceRestartLane {
            id: "source-only",
            action: "continue source checks and commits without runtime execution",
            requires_permission: false,
        },
        LaunchEvidenceRestartLane {
            id: "runtime-approved",
            action: "collect governed runtime evidence only after explicit permission",
            requires_permission: true,
        },
        LaunchEvidenceRestartLane {
            id: "release-closeout",
            action: "refresh release handoff artifacts after evidence changes",
            requires_permission: false,
        },
    ]
}

fn input_present(inputs: &[LaunchEvidenceRestartChecklistInput], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-restart-checklist".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceRestartChecklistCheck {
    LaunchEvidenceRestartChecklistCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceRestartChecklistCheck]) -> u8 {
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
    fn fails_when_restart_ledger_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");

        let report =
            build_launch_evidence_restart_checklist_report(dir.path(), 100).expect("checklist");

        assert!(!report.passed());
        assert!(!report.freshness.restart_ledger_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "restart-ledger-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_restart_checklist_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), CHECKLIST_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), RESTART_LEDGER_PATH);

        let report =
            build_launch_evidence_restart_checklist_report(dir.path(), 0).expect("checklist");

        assert!(!report.freshness.checklist_not_older_than_restart_ledger);
    }

    #[test]
    fn passes_complete_fresh_checklist_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), RESTART_LEDGER_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), CHECKLIST_PATH);

        let report =
            build_launch_evidence_restart_checklist_report(dir.path(), 100).expect("checklist");

        assert!(report.passed());
        assert_eq!(
            report.checklist.checklist_target,
            "dx-cli-zed-restart-next-actions"
        );
        assert_eq!(report.checklist.lanes.len(), 3);
        assert!(
            report
                .checklist
                .lanes
                .iter()
                .any(|lane| lane.id == "runtime-approved" && lane.requires_permission)
        );
        assert!(!report.checklist.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_restart_checklist() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), RESTART_LEDGER_PATH);

        run_launch_evidence_restart_checklist(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write checklist");

        let report =
            build_launch_evidence_restart_checklist_report(dir.path(), 100).expect("checklist");
        assert!(report.passed());
        assert!(dir.path().join(CHECKLIST_PATH).is_file());
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("input parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
