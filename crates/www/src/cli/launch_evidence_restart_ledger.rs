use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_restart_ledger";
const LEDGER_SCHEMA: &str = "dx.launch.evidence_restart_ledger";
const LEDGER_PATH: &str = ".dx/forge/release/launch-evidence-restart-ledger.json";
const OPERATOR_RESUME_CARD_PATH: &str =
    ".dx/forge/release/launch-evidence-operator-resume-card.json";
const CONTINUATION_PACKET_PATH: &str = ".dx/forge/release/launch-evidence-continuation-packet.json";
const RECOVERY_BRIEF_PATH: &str = ".dx/forge/release/launch-evidence-recovery-brief.md";
const RESUMPTION_INDEX_PATH: &str = ".dx/forge/release/launch-evidence-resumption-index.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceRestartLedgerReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    ledger: LaunchEvidenceRestartLedger,
    inputs: Vec<LaunchEvidenceRestartLedgerInput>,
    freshness: LaunchEvidenceRestartLedgerFreshness,
    checks: Vec<LaunchEvidenceRestartLedgerCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceRestartLedgerReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartLedger {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    ledger_target: &'static str,
    input_count: usize,
    present_inputs: usize,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartLedgerInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartLedgerFreshness {
    ledger_path: &'static str,
    ledger_present: bool,
    ledger_modified_at: Option<String>,
    operator_resume_card_present: bool,
    ledger_not_older_than_operator_resume_card: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartLedgerCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_restart_ledger(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-restart-ledger option: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-ledger".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-restart-ledger argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-ledger".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(LEDGER_PATH));
    }

    let mut report =
        build_launch_evidence_restart_ledger_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_restart_ledger_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_restart_ledger(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_restart_ledger(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_restart_ledger_failure_summary(&report),
            field: Some("forge.launch-evidence-restart-ledger".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_restart_ledger(
    report: &LaunchEvidenceRestartLedgerReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_restart_ledger_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_restart_ledger_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_restart_ledger_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceRestartLedgerReport> {
    let inputs = restart_ledger_inputs(project);
    let present_inputs = inputs.iter().filter(|input| input.present).count();
    let all_inputs_present = present_inputs == inputs.len();
    let freshness = restart_ledger_freshness(project, &inputs);
    let checks = vec![
        check(
            "operator-resume-card-present",
            freshness.operator_resume_card_present,
            format!("operator resume card exists at {OPERATOR_RESUME_CARD_PATH}"),
        ),
        check(
            "restart-ledger-inputs-present",
            all_inputs_present,
            format!(
                "{present_inputs}/{} restart ledger input(s) are present",
                inputs.len()
            ),
        ),
        check(
            "restart-ledger-freshness",
            freshness.ledger_not_older_than_operator_resume_card,
            "restart ledger is not older than the operator resume card".to_string(),
        ),
        check(
            "durable-dx-restart-ledger",
            true,
            "restart ledger records the durable DX CLI/Zed restart target".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "restart ledger uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceRestartLedgerReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        ledger: LaunchEvidenceRestartLedger {
            schema: LEDGER_SCHEMA,
            path: LEDGER_PATH,
            command: "dx forge launch-evidence-restart-ledger --project <path> --write",
            ledger_target: "durable-dx-restart-ledger",
            input_count: inputs.len(),
            present_inputs,
            reads_runtime_artifact_contents: false,
        },
        inputs,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-operator-resume-card --project . --write".to_string(),
            "dx forge launch-evidence-restart-ledger --project . --write".to_string(),
            "dx forge launch-evidence-restart-checklist --project . --write".to_string(),
            format!("open {LEDGER_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_restart_ledger_terminal(
    report: &LaunchEvidenceRestartLedgerReport,
) -> String {
    format!(
        "DX Forge launch evidence restart ledger\nProject: {}\nPassed: {}\nScore: {}\nLedger target: {}\nInputs present: {}/{}\nLedger fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.ledger.ledger_target,
        report.ledger.present_inputs,
        report.ledger.input_count,
        report.freshness.ledger_not_older_than_operator_resume_card,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_restart_ledger_markdown(
    report: &LaunchEvidenceRestartLedgerReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Restart Ledger\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Ledger target: `{}`\n- Operator resume card fresh: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.ledger.ledger_target,
        report.freshness.ledger_not_older_than_operator_resume_card,
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

pub(crate) fn launch_evidence_restart_ledger_failure_summary(
    report: &LaunchEvidenceRestartLedgerReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence restart ledger score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn restart_ledger_inputs(project: &Path) -> Vec<LaunchEvidenceRestartLedgerInput> {
    [
        (
            "operator-resume-card",
            OPERATOR_RESUME_CARD_PATH,
            "dx forge launch-evidence-operator-resume-card --project . --write",
        ),
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
        (
            "resumption-index",
            RESUMPTION_INDEX_PATH,
            "dx forge launch-evidence-resumption-index --project . --write",
        ),
    ]
    .into_iter()
    .map(|(id, path, command)| restart_ledger_input(project, id, path, command))
    .collect()
}

fn restart_ledger_input(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceRestartLedgerInput {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceRestartLedgerInput {
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

fn restart_ledger_freshness(
    project: &Path,
    inputs: &[LaunchEvidenceRestartLedgerInput],
) -> LaunchEvidenceRestartLedgerFreshness {
    let ledger_modified =
        file_metadata(&project.join(LEDGER_PATH)).map(|metadata| metadata.modified_at);
    let card_modified = file_metadata(&project.join(OPERATOR_RESUME_CARD_PATH))
        .map(|metadata| metadata.modified_at);
    let ledger_not_older_than_operator_resume_card = match (ledger_modified, card_modified) {
        (Some(ledger), Some(card)) => ledger >= card,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceRestartLedgerFreshness {
        ledger_path: LEDGER_PATH,
        ledger_present: ledger_modified.is_some(),
        ledger_modified_at: ledger_modified.map(format_system_time),
        operator_resume_card_present: input_present(inputs, "operator-resume-card"),
        ledger_not_older_than_operator_resume_card,
        timestamp_source: "filesystem-metadata",
    }
}

fn input_present(inputs: &[LaunchEvidenceRestartLedgerInput], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-restart-ledger".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceRestartLedgerCheck {
    LaunchEvidenceRestartLedgerCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceRestartLedgerCheck]) -> u8 {
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
    fn fails_when_operator_resume_card_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            CONTINUATION_PACKET_PATH,
            RECOVERY_BRIEF_PATH,
            RESUMPTION_INDEX_PATH,
        ] {
            write_input(dir.path(), path);
        }

        let report = build_launch_evidence_restart_ledger_report(dir.path(), 100).expect("ledger");

        assert!(!report.passed());
        assert!(!report.freshness.operator_resume_card_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "operator-resume-card-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_restart_ledger_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), LEDGER_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), OPERATOR_RESUME_CARD_PATH);

        let report = build_launch_evidence_restart_ledger_report(dir.path(), 0).expect("ledger");

        assert!(!report.freshness.ledger_not_older_than_operator_resume_card);
    }

    #[test]
    fn passes_complete_fresh_restart_ledger_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_restart_ledger_inputs(dir.path());
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), LEDGER_PATH);

        let report = build_launch_evidence_restart_ledger_report(dir.path(), 100).expect("ledger");

        assert!(report.passed());
        assert_eq!(report.ledger.ledger_target, "durable-dx-restart-ledger");
        assert!(!report.ledger.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_restart_ledger() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_restart_ledger_inputs(dir.path());

        run_launch_evidence_restart_ledger(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write ledger");

        let report = build_launch_evidence_restart_ledger_report(dir.path(), 100).expect("ledger");
        assert!(report.passed());
        assert!(dir.path().join(LEDGER_PATH).is_file());
    }

    fn write_restart_ledger_inputs(project: &Path) {
        for path in [
            OPERATOR_RESUME_CARD_PATH,
            CONTINUATION_PACKET_PATH,
            RECOVERY_BRIEF_PATH,
            RESUMPTION_INDEX_PATH,
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
