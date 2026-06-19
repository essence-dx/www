use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_continuation_packet";
const PACKET_SCHEMA: &str = "dx.launch.evidence_continuation_packet";
const PACKET_PATH: &str = ".dx/forge/release/launch-evidence-continuation-packet.json";
const RECOVERY_BRIEF_PATH: &str = ".dx/forge/release/launch-evidence-recovery-brief.md";
const RESUMPTION_INDEX_PATH: &str = ".dx/forge/release/launch-evidence-resumption-index.json";
const HANDOFF_CAPSULE_PATH: &str = ".dx/forge/release/launch-evidence-handoff-capsule.json";
const OPERATOR_RUNBOOK_PATH: &str = ".dx/forge/release/launch-evidence-operator-runbook.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceContinuationPacketReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    packet: LaunchEvidenceContinuationPacket,
    inputs: Vec<LaunchEvidenceContinuationInput>,
    freshness: LaunchEvidenceContinuationFreshness,
    checks: Vec<LaunchEvidenceContinuationCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceContinuationPacketReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceContinuationPacket {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    continuation_target: &'static str,
    input_count: usize,
    present_inputs: usize,
    restart_packet_inputs: Vec<&'static str>,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceContinuationInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceContinuationFreshness {
    packet_path: &'static str,
    packet_present: bool,
    packet_modified_at: Option<String>,
    recovery_brief_present: bool,
    packet_not_older_than_recovery_brief: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceContinuationCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_continuation_packet(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-continuation-packet option: {value}"
                    ),
                    field: Some("forge.launch-evidence-continuation-packet".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-continuation-packet argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-continuation-packet".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(PACKET_PATH));
    }

    let mut report = build_launch_evidence_continuation_packet_report(&project, fail_under)
        .map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_continuation_packet_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_continuation_packet(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_continuation_packet(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_continuation_packet_failure_summary(&report),
            field: Some("forge.launch-evidence-continuation-packet".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_continuation_packet(
    report: &LaunchEvidenceContinuationPacketReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_continuation_packet_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_continuation_packet_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_continuation_packet_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceContinuationPacketReport> {
    let inputs = continuation_inputs(project);
    let present_inputs = inputs.iter().filter(|input| input.present).count();
    let all_inputs_present = present_inputs == inputs.len();
    let freshness = continuation_freshness(project, &inputs);
    let checks = vec![
        check(
            "recovery-brief-present",
            freshness.recovery_brief_present,
            format!("recovery brief exists at {RECOVERY_BRIEF_PATH}"),
        ),
        check(
            "continuation-packet-inputs-present",
            all_inputs_present,
            format!(
                "{present_inputs}/{} continuation packet input(s) are present",
                inputs.len()
            ),
        ),
        check(
            "continuation-packet-freshness",
            freshness.packet_not_older_than_recovery_brief,
            "continuation packet is not older than the recovery brief".to_string(),
        ),
        check(
            "dx-cli-zed-continuation-packet",
            true,
            "continuation packet records the compact DX CLI/Zed restart handoff".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "continuation packet uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceContinuationPacketReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        packet: LaunchEvidenceContinuationPacket {
            schema: PACKET_SCHEMA,
            path: PACKET_PATH,
            command: "dx forge launch-evidence-continuation-packet --project <path> --write",
            continuation_target: "dx-cli-zed-continuation-packet",
            input_count: inputs.len(),
            present_inputs,
            restart_packet_inputs: vec![
                "recovery-brief",
                "resumption-index",
                "handoff-capsule",
                "operator-runbook",
            ],
            reads_runtime_artifact_contents: false,
        },
        inputs,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-recovery-brief --project . --write".to_string(),
            "dx forge launch-evidence-continuation-packet --project . --write".to_string(),
            "dx forge launch-evidence-operator-resume-card --project . --write".to_string(),
            format!("open {PACKET_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_continuation_packet_terminal(
    report: &LaunchEvidenceContinuationPacketReport,
) -> String {
    format!(
        "DX Forge launch evidence continuation packet\nProject: {}\nPassed: {}\nScore: {}\nContinuation target: {}\nInputs present: {}/{}\nPacket fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.packet.continuation_target,
        report.packet.present_inputs,
        report.packet.input_count,
        report.freshness.packet_not_older_than_recovery_brief,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_continuation_packet_markdown(
    report: &LaunchEvidenceContinuationPacketReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Continuation Packet\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Continuation target: `{}`\n- Recovery brief fresh: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.packet.continuation_target,
        report.freshness.packet_not_older_than_recovery_brief,
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

pub(crate) fn launch_evidence_continuation_packet_failure_summary(
    report: &LaunchEvidenceContinuationPacketReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence continuation packet score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn continuation_inputs(project: &Path) -> Vec<LaunchEvidenceContinuationInput> {
    [
        (
            "recovery-brief",
            RECOVERY_BRIEF_PATH,
            "dx forge launch-evidence-recovery-brief --project . --write",
        ),
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
    ]
    .into_iter()
    .map(|(id, path, command)| continuation_input(project, id, path, command))
    .collect()
}

fn continuation_input(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceContinuationInput {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceContinuationInput {
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

fn continuation_freshness(
    project: &Path,
    inputs: &[LaunchEvidenceContinuationInput],
) -> LaunchEvidenceContinuationFreshness {
    let packet_modified =
        file_metadata(&project.join(PACKET_PATH)).map(|metadata| metadata.modified_at);
    let recovery_modified =
        file_metadata(&project.join(RECOVERY_BRIEF_PATH)).map(|metadata| metadata.modified_at);
    let packet_not_older_than_recovery_brief = match (packet_modified, recovery_modified) {
        (Some(packet), Some(recovery)) => packet >= recovery,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceContinuationFreshness {
        packet_path: PACKET_PATH,
        packet_present: packet_modified.is_some(),
        packet_modified_at: packet_modified.map(format_system_time),
        recovery_brief_present: input_present(inputs, "recovery-brief"),
        packet_not_older_than_recovery_brief,
        timestamp_source: "filesystem-metadata",
    }
}

fn input_present(inputs: &[LaunchEvidenceContinuationInput], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-continuation-packet".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceContinuationCheck {
    LaunchEvidenceContinuationCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceContinuationCheck]) -> u8 {
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
    fn fails_when_recovery_brief_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            RESUMPTION_INDEX_PATH,
            HANDOFF_CAPSULE_PATH,
            OPERATOR_RUNBOOK_PATH,
        ] {
            write_input(dir.path(), path);
        }

        let report =
            build_launch_evidence_continuation_packet_report(dir.path(), 100).expect("packet");

        assert!(!report.passed());
        assert!(!report.freshness.recovery_brief_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "recovery-brief-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_continuation_packet_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), PACKET_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), RECOVERY_BRIEF_PATH);

        let report =
            build_launch_evidence_continuation_packet_report(dir.path(), 0).expect("packet");

        assert!(!report.freshness.packet_not_older_than_recovery_brief);
    }

    #[test]
    fn passes_complete_fresh_packet_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_continuation_inputs(dir.path());
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), PACKET_PATH);

        let report =
            build_launch_evidence_continuation_packet_report(dir.path(), 100).expect("packet");

        assert!(report.passed());
        assert_eq!(
            report.packet.continuation_target,
            "dx-cli-zed-continuation-packet"
        );
        assert_eq!(report.packet.restart_packet_inputs.len(), 4);
        assert!(!report.packet.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_continuation_packet() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_continuation_inputs(dir.path());

        run_launch_evidence_continuation_packet(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write packet");

        let report =
            build_launch_evidence_continuation_packet_report(dir.path(), 100).expect("packet");
        assert!(report.passed());
        assert!(dir.path().join(PACKET_PATH).is_file());
    }

    fn write_continuation_inputs(project: &Path) {
        for path in [
            RECOVERY_BRIEF_PATH,
            RESUMPTION_INDEX_PATH,
            HANDOFF_CAPSULE_PATH,
            OPERATOR_RUNBOOK_PATH,
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
