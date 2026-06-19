use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_recovery_brief";
const BRIEF_SCHEMA: &str = "dx.launch.evidence_recovery_brief";
const BRIEF_PATH: &str = ".dx/forge/release/launch-evidence-recovery-brief.md";
const RESUMPTION_INDEX_PATH: &str = ".dx/forge/release/launch-evidence-resumption-index.json";
const HANDOFF_CAPSULE_PATH: &str = ".dx/forge/release/launch-evidence-handoff-capsule.json";
const OPERATOR_RUNBOOK_PATH: &str = ".dx/forge/release/launch-evidence-operator-runbook.json";
const FINAL_BRIEF_PATH: &str = ".dx/forge/release/launch-evidence-final-brief.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceRecoveryBriefReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    brief: LaunchEvidenceRecoveryBrief,
    restart_lanes: Vec<LaunchEvidenceRecoveryLane>,
    inputs: Vec<LaunchEvidenceRecoveryBriefInput>,
    freshness: LaunchEvidenceRecoveryBriefFreshness,
    checks: Vec<LaunchEvidenceRecoveryBriefCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceRecoveryBriefReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRecoveryBrief {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    recovery_target: &'static str,
    zed_openable: bool,
    format: &'static str,
    input_count: usize,
    present_inputs: usize,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRecoveryLane {
    id: &'static str,
    label: &'static str,
    command: &'static str,
    requires_runtime_permission: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRecoveryBriefInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRecoveryBriefFreshness {
    brief_path: &'static str,
    brief_present: bool,
    brief_modified_at: Option<String>,
    resumption_index_present: bool,
    brief_not_older_than_resumption_index: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRecoveryBriefCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_recovery_brief(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-recovery-brief option: {value}"
                    ),
                    field: Some("forge.launch-evidence-recovery-brief".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-recovery-brief argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-recovery-brief".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(BRIEF_PATH));
    }

    let mut report =
        build_launch_evidence_recovery_brief_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "").map_err(forge_error)?;
            report = build_launch_evidence_recovery_brief_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_recovery_brief(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_recovery_brief(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_recovery_brief_failure_summary(&report),
            field: Some("forge.launch-evidence-recovery-brief".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_recovery_brief(
    report: &LaunchEvidenceRecoveryBriefReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_recovery_brief_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_recovery_brief_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_recovery_brief_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceRecoveryBriefReport> {
    let inputs = recovery_brief_inputs(project);
    let present_inputs = inputs.iter().filter(|input| input.present).count();
    let all_inputs_present = present_inputs == inputs.len();
    let freshness = recovery_brief_freshness(project, &inputs);
    let checks = vec![
        check(
            "resumption-index-present",
            freshness.resumption_index_present,
            format!("resumption index exists at {RESUMPTION_INDEX_PATH}"),
        ),
        check(
            "recovery-brief-inputs-present",
            all_inputs_present,
            format!(
                "{present_inputs}/{} recovery brief input(s) are present",
                inputs.len()
            ),
        ),
        check(
            "recovery-brief-freshness",
            freshness.brief_not_older_than_resumption_index,
            "recovery brief is not older than the resumption index".to_string(),
        ),
        check(
            "zed-openable-markdown",
            true,
            "recovery brief writes a Zed-openable Markdown restart brief".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "recovery brief uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceRecoveryBriefReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        brief: LaunchEvidenceRecoveryBrief {
            schema: BRIEF_SCHEMA,
            path: BRIEF_PATH,
            command: "dx forge launch-evidence-recovery-brief --project <path> --write",
            recovery_target: "human-readable-dx-worker-restart-brief",
            zed_openable: true,
            format: "markdown",
            input_count: inputs.len(),
            present_inputs,
            reads_runtime_artifact_contents: false,
        },
        restart_lanes: recovery_lanes(),
        inputs,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-resumption-index --project . --write".to_string(),
            "dx forge launch-evidence-recovery-brief --project . --write".to_string(),
            "dx forge launch-evidence-continuation-packet --project . --write".to_string(),
            format!("open {BRIEF_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_recovery_brief_terminal(
    report: &LaunchEvidenceRecoveryBriefReport,
) -> String {
    format!(
        "DX Forge launch evidence recovery brief\nProject: {}\nPassed: {}\nScore: {}\nRecovery target: {}\nInputs present: {}/{}\nBrief fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.brief.recovery_target,
        report.brief.present_inputs,
        report.brief.input_count,
        report.freshness.brief_not_older_than_resumption_index,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_recovery_brief_markdown(
    report: &LaunchEvidenceRecoveryBriefReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Recovery Brief\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Recovery target: `{}`\n- Resumption index fresh: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.brief.recovery_target,
        report.freshness.brief_not_older_than_resumption_index,
        report.no_execution
    );
    output.push_str("## Restart Lanes\n\n");
    for lane in &report.restart_lanes {
        output.push_str(&format!(
            "- `{}`: {} (`{}`) runtime permission `{}`\n",
            lane.id, lane.label, lane.command, lane.requires_runtime_permission
        ));
    }
    output.push_str(
        "\n## Recovery Inputs\n\n| Input | Present | Modified | Bytes | Path |\n| --- | --- | --- | --- | --- |\n",
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

pub(crate) fn launch_evidence_recovery_brief_failure_summary(
    report: &LaunchEvidenceRecoveryBriefReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence recovery brief score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn recovery_brief_inputs(project: &Path) -> Vec<LaunchEvidenceRecoveryBriefInput> {
    [
        (
            "resumption-index",
            RESUMPTION_INDEX_PATH,
            "dx forge launch-evidence-resumption-index --project . --write",
        ),
        (
            "handoff-capsule",
            HANDOFF_CAPSULE_PATH,
            "dx forge launch-evidence-handoff-capsule --project . --write",
        ),
        (
            "operator-runbook",
            OPERATOR_RUNBOOK_PATH,
            "dx forge launch-evidence-operator-runbook --project . --write",
        ),
        (
            "final-brief",
            FINAL_BRIEF_PATH,
            "dx forge launch-evidence-final-brief --project . --write",
        ),
    ]
    .into_iter()
    .map(|(id, path, command)| recovery_brief_input(project, id, path, command))
    .collect()
}

fn recovery_brief_input(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceRecoveryBriefInput {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceRecoveryBriefInput {
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

fn recovery_lanes() -> Vec<LaunchEvidenceRecoveryLane> {
    vec![
        LaunchEvidenceRecoveryLane {
            id: "source-only",
            label: "Source-only lane: continue launch work with lightweight guards",
            command: "dx run --test .\\benchmarks\\template-shell.test.ts",
            requires_runtime_permission: false,
        },
        LaunchEvidenceRecoveryLane {
            id: "runtime-approved",
            label: "Runtime-approved lane: resume runtime proof only after explicit approval",
            command: "dx forge launch-verification-lane --project . --json",
            requires_runtime_permission: true,
        },
        LaunchEvidenceRecoveryLane {
            id: "release-closeout",
            label: "Release-closeout lane: refresh evidence after source or runtime changes",
            command: "dx forge launch-evidence-resumption-index --project . --write",
            requires_runtime_permission: false,
        },
    ]
}

fn recovery_brief_freshness(
    project: &Path,
    inputs: &[LaunchEvidenceRecoveryBriefInput],
) -> LaunchEvidenceRecoveryBriefFreshness {
    let brief_modified =
        file_metadata(&project.join(BRIEF_PATH)).map(|metadata| metadata.modified_at);
    let resumption_index_modified =
        file_metadata(&project.join(RESUMPTION_INDEX_PATH)).map(|metadata| metadata.modified_at);
    let brief_not_older_than_resumption_index = match (brief_modified, resumption_index_modified) {
        (Some(brief), Some(index)) => brief >= index,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceRecoveryBriefFreshness {
        brief_path: BRIEF_PATH,
        brief_present: brief_modified.is_some(),
        brief_modified_at: brief_modified.map(format_system_time),
        resumption_index_present: input_present(inputs, "resumption-index"),
        brief_not_older_than_resumption_index,
        timestamp_source: "filesystem-metadata",
    }
}

fn input_present(inputs: &[LaunchEvidenceRecoveryBriefInput], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-recovery-brief".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceRecoveryBriefCheck {
    LaunchEvidenceRecoveryBriefCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceRecoveryBriefCheck]) -> u8 {
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
    fn fails_when_resumption_index_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            HANDOFF_CAPSULE_PATH,
            OPERATOR_RUNBOOK_PATH,
            FINAL_BRIEF_PATH,
        ] {
            write_input(dir.path(), path);
        }

        let report = build_launch_evidence_recovery_brief_report(dir.path(), 100).expect("brief");

        assert!(!report.passed());
        assert!(!report.freshness.resumption_index_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "resumption-index-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_recovery_brief_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), BRIEF_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), RESUMPTION_INDEX_PATH);

        let report = build_launch_evidence_recovery_brief_report(dir.path(), 0).expect("brief");

        assert!(!report.freshness.brief_not_older_than_resumption_index);
    }

    #[test]
    fn passes_complete_fresh_recovery_brief_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_recovery_brief_inputs(dir.path());
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), BRIEF_PATH);

        let report = build_launch_evidence_recovery_brief_report(dir.path(), 100).expect("brief");

        assert!(report.passed());
        assert_eq!(
            report.brief.recovery_target,
            "human-readable-dx-worker-restart-brief"
        );
        assert!(report.brief.zed_openable);
        assert_eq!(report.brief.format, "markdown");
        assert!(!report.brief.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_zed_openable_recovery_brief() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_recovery_brief_inputs(dir.path());

        run_launch_evidence_recovery_brief(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write brief");

        let brief = fs::read_to_string(dir.path().join(BRIEF_PATH)).expect("brief markdown");
        assert!(brief.contains("# DX Forge Launch Evidence Recovery Brief"));
        let report = build_launch_evidence_recovery_brief_report(dir.path(), 100).expect("brief");
        assert!(report.passed());
    }

    fn write_recovery_brief_inputs(project: &Path) {
        for path in [
            RESUMPTION_INDEX_PATH,
            HANDOFF_CAPSULE_PATH,
            OPERATOR_RUNBOOK_PATH,
            FINAL_BRIEF_PATH,
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
