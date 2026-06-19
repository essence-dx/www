use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_operator_resume_card";
const CARD_SCHEMA: &str = "dx.launch.evidence_operator_resume_card";
const CARD_PATH: &str = ".dx/forge/release/launch-evidence-operator-resume-card.json";
const CONTINUATION_PACKET_PATH: &str = ".dx/forge/release/launch-evidence-continuation-packet.json";
const RECOVERY_BRIEF_PATH: &str = ".dx/forge/release/launch-evidence-recovery-brief.md";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceOperatorResumeCardReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    card: LaunchEvidenceOperatorResumeCard,
    inputs: Vec<LaunchEvidenceOperatorResumeInput>,
    freshness: LaunchEvidenceOperatorResumeFreshness,
    checks: Vec<LaunchEvidenceOperatorResumeCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceOperatorResumeCardReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceOperatorResumeCard {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    resume_target: &'static str,
    terminal_first: bool,
    continuation_packet: &'static str,
    input_count: usize,
    present_inputs: usize,
    card_lines: Vec<&'static str>,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceOperatorResumeInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceOperatorResumeFreshness {
    card_path: &'static str,
    card_present: bool,
    card_modified_at: Option<String>,
    continuation_packet_present: bool,
    card_not_older_than_continuation_packet: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceOperatorResumeCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_operator_resume_card(
    cwd: &Path,
    args: &[String],
) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-operator-resume-card option: {value}"
                    ),
                    field: Some("forge.launch-evidence-operator-resume-card".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-operator-resume-card argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-operator-resume-card".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(CARD_PATH));
    }

    let mut report = build_launch_evidence_operator_resume_card_report(&project, fail_under)
        .map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_operator_resume_card_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_operator_resume_card(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_operator_resume_card(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_operator_resume_card_failure_summary(&report),
            field: Some("forge.launch-evidence-operator-resume-card".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_operator_resume_card(
    report: &LaunchEvidenceOperatorResumeCardReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_operator_resume_card_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_operator_resume_card_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_operator_resume_card_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceOperatorResumeCardReport> {
    let inputs = operator_resume_inputs(project);
    let present_inputs = inputs.iter().filter(|input| input.present).count();
    let all_inputs_present = present_inputs == inputs.len();
    let freshness = operator_resume_freshness(project, &inputs);
    let checks = vec![
        check(
            "continuation-packet-present",
            freshness.continuation_packet_present,
            format!("continuation packet exists at {CONTINUATION_PACKET_PATH}"),
        ),
        check(
            "operator-resume-card-inputs-present",
            all_inputs_present,
            format!(
                "{present_inputs}/{} operator resume card input(s) are present",
                inputs.len()
            ),
        ),
        check(
            "operator-resume-card-freshness",
            freshness.card_not_older_than_continuation_packet,
            "operator resume card is not older than the continuation packet".to_string(),
        ),
        check(
            "terminal-first-dx-resume-card",
            true,
            "operator resume card records the tiny terminal-first DX CLI/Zed handoff".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "operator resume card uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceOperatorResumeCardReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        card: LaunchEvidenceOperatorResumeCard {
            schema: CARD_SCHEMA,
            path: CARD_PATH,
            command: "dx forge launch-evidence-operator-resume-card --project <path> --write",
            resume_target: "terminal-first-dx-resume-card",
            terminal_first: true,
            continuation_packet: CONTINUATION_PACKET_PATH,
            input_count: inputs.len(),
            present_inputs,
            card_lines: vec![
                "open continuation packet",
                "choose source-only or runtime-approved lane",
                "rerun lightweight source guard before committing",
            ],
            reads_runtime_artifact_contents: false,
        },
        inputs,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-continuation-packet --project . --write".to_string(),
            "dx forge launch-evidence-operator-resume-card --project . --write".to_string(),
            "dx forge launch-evidence-restart-ledger --project . --write".to_string(),
            format!("open {CARD_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_operator_resume_card_terminal(
    report: &LaunchEvidenceOperatorResumeCardReport,
) -> String {
    format!(
        "DX Forge launch evidence operator resume card\nProject: {}\nPassed: {}\nScore: {}\nResume target: {}\nInputs present: {}/{}\nCard fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.card.resume_target,
        report.card.present_inputs,
        report.card.input_count,
        report.freshness.card_not_older_than_continuation_packet,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_operator_resume_card_markdown(
    report: &LaunchEvidenceOperatorResumeCardReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Operator Resume Card\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Resume target: `{}`\n- Continuation packet fresh: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.card.resume_target,
        report.freshness.card_not_older_than_continuation_packet,
        report.no_execution
    );
    output.push_str("## Card Lines\n\n");
    for line in &report.card.card_lines {
        output.push_str(&format!("- {line}\n"));
    }
    output.push_str(
        "\n## Resume Inputs\n\n| Input | Present | Modified | Bytes | Path |\n| --- | --- | --- | --- | --- |\n",
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

pub(crate) fn launch_evidence_operator_resume_card_failure_summary(
    report: &LaunchEvidenceOperatorResumeCardReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence operator resume card score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn operator_resume_inputs(project: &Path) -> Vec<LaunchEvidenceOperatorResumeInput> {
    [
        (
            "continuation-packet",
            CONTINUATION_PACKET_PATH,
            "dx forge launch-evidence-continuation-packet --project . --write",
        ),
        (
            "recovery-brief",
            RECOVERY_BRIEF_PATH,
            "dx forge launch-evidence-recovery-brief --project . --write",
        ),
    ]
    .into_iter()
    .map(|(id, path, command)| operator_resume_input(project, id, path, command))
    .collect()
}

fn operator_resume_input(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceOperatorResumeInput {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceOperatorResumeInput {
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

fn operator_resume_freshness(
    project: &Path,
    inputs: &[LaunchEvidenceOperatorResumeInput],
) -> LaunchEvidenceOperatorResumeFreshness {
    let card_modified =
        file_metadata(&project.join(CARD_PATH)).map(|metadata| metadata.modified_at);
    let continuation_modified =
        file_metadata(&project.join(CONTINUATION_PACKET_PATH)).map(|metadata| metadata.modified_at);
    let card_not_older_than_continuation_packet = match (card_modified, continuation_modified) {
        (Some(card), Some(packet)) => card >= packet,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceOperatorResumeFreshness {
        card_path: CARD_PATH,
        card_present: card_modified.is_some(),
        card_modified_at: card_modified.map(format_system_time),
        continuation_packet_present: input_present(inputs, "continuation-packet"),
        card_not_older_than_continuation_packet,
        timestamp_source: "filesystem-metadata",
    }
}

fn input_present(inputs: &[LaunchEvidenceOperatorResumeInput], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-operator-resume-card".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceOperatorResumeCheck {
    LaunchEvidenceOperatorResumeCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceOperatorResumeCheck]) -> u8 {
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
    fn fails_when_continuation_packet_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), RECOVERY_BRIEF_PATH);

        let report =
            build_launch_evidence_operator_resume_card_report(dir.path(), 100).expect("card");

        assert!(!report.passed());
        assert!(!report.freshness.continuation_packet_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "continuation-packet-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_operator_resume_card_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), CARD_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), CONTINUATION_PACKET_PATH);

        let report =
            build_launch_evidence_operator_resume_card_report(dir.path(), 0).expect("card");

        assert!(!report.freshness.card_not_older_than_continuation_packet);
    }

    #[test]
    fn passes_complete_fresh_card_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_operator_resume_inputs(dir.path());
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), CARD_PATH);

        let report =
            build_launch_evidence_operator_resume_card_report(dir.path(), 100).expect("card");

        assert!(report.passed());
        assert_eq!(report.card.resume_target, "terminal-first-dx-resume-card");
        assert!(report.card.terminal_first);
        assert!(!report.card.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_operator_resume_card() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_operator_resume_inputs(dir.path());

        run_launch_evidence_operator_resume_card(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write card");

        let report =
            build_launch_evidence_operator_resume_card_report(dir.path(), 100).expect("card");
        assert!(report.passed());
        assert!(dir.path().join(CARD_PATH).is_file());
    }

    fn write_operator_resume_inputs(project: &Path) {
        for path in [CONTINUATION_PACKET_PATH, RECOVERY_BRIEF_PATH] {
            write_input(project, path);
        }
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("input parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
