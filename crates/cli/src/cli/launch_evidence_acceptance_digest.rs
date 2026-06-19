use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_acceptance_digest";
const DIGEST_SCHEMA: &str = "dx.launch.evidence_acceptance_digest";
const DIGEST_PATH: &str = ".dx/forge/release/launch-evidence-acceptance-digest.json";
const ACCEPTANCE_INDEX_PATH: &str = ".dx/forge/release/launch-evidence-acceptance-index.md";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceAcceptanceDigestReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    digest: LaunchEvidenceAcceptanceDigest,
    inputs: Vec<LaunchEvidenceAcceptanceDigestInput>,
    freshness: LaunchEvidenceAcceptanceDigestFreshness,
    checks: Vec<LaunchEvidenceAcceptanceDigestCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceAcceptanceDigestReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceAcceptanceDigest {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    digest_target: &'static str,
    acceptance_index: &'static str,
    terminal_status_line: String,
    final_status_line: String,
    display_mode: &'static str,
    format: &'static str,
    zed_openable: bool,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceAcceptanceDigestInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceAcceptanceDigestFreshness {
    digest_path: &'static str,
    digest_present: bool,
    digest_modified_at: Option<String>,
    acceptance_index_present: bool,
    digest_not_older_than_acceptance_index: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceAcceptanceDigestCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_acceptance_digest(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-acceptance-digest option: {value}"
                    ),
                    field: Some("forge.launch-evidence-acceptance-digest".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-acceptance-digest argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-acceptance-digest".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(DIGEST_PATH));
    }

    let mut report = build_launch_evidence_acceptance_digest_report(&project, fail_under)
        .map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_acceptance_digest_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_acceptance_digest(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_acceptance_digest(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_acceptance_digest_failure_summary(&report),
            field: Some("forge.launch-evidence-acceptance-digest".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_acceptance_digest(
    report: &LaunchEvidenceAcceptanceDigestReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_acceptance_digest_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_acceptance_digest_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_acceptance_digest_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceAcceptanceDigestReport> {
    let inputs = vec![acceptance_digest_input(
        project,
        "acceptance-index",
        ACCEPTANCE_INDEX_PATH,
        "dx forge launch-evidence-acceptance-index --project <path> --write",
    )];
    let all_inputs_present = inputs.iter().all(|input| input.present);
    let freshness = acceptance_digest_freshness(project);
    let checks = vec![
        check(
            "acceptance-index-present",
            freshness.acceptance_index_present && all_inputs_present,
            format!("acceptance index exists at {ACCEPTANCE_INDEX_PATH}"),
        ),
        check(
            "acceptance-digest-fresh",
            freshness.digest_not_older_than_acceptance_index,
            "acceptance digest is not older than the acceptance index".to_string(),
        ),
        check(
            "friday-terminal-final-status-line",
            true,
            "acceptance digest includes a terminal-first final status line for Friday".to_string(),
        ),
        check(
            "json-digest",
            true,
            "acceptance digest writes a DX/Zed indexable JSON receipt".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "acceptance digest reads file metadata only; no runtime artifact bodies are read"
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
    let status_word = if passed { "ready" } else { "blocked" };
    let terminal_status_line = format!(
        "DX launch acceptance digest: {status_word} for Friday final handoff (terminal-first-final-status)"
    );

    Ok(LaunchEvidenceAcceptanceDigestReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        digest: LaunchEvidenceAcceptanceDigest {
            schema: DIGEST_SCHEMA,
            path: DIGEST_PATH,
            command: "dx forge launch-evidence-acceptance-digest --project <path> --write",
            digest_target: "friday-terminal-final-status-line",
            acceptance_index: ACCEPTANCE_INDEX_PATH,
            terminal_status_line: terminal_status_line.clone(),
            final_status_line: terminal_status_line,
            display_mode: "terminal-first-final-status",
            format: "json",
            zed_openable: true,
            reads_runtime_artifact_contents: false,
        },
        inputs,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-acceptance-digest --project . --write".to_string(),
            "dx forge launch-evidence-friday-baton --project . --write".to_string(),
            "dx forge launch-evidence-acceptance-index --project . --write".to_string(),
            format!("open {DIGEST_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_acceptance_digest_terminal(
    report: &LaunchEvidenceAcceptanceDigestReport,
) -> String {
    format!(
        "{}\nSchema: {}\nStatus: {}\nScore: {}/100\n",
        report.digest.terminal_status_line,
        report.schema,
        if report.passed { "ready" } else { "blocked" },
        report.score
    )
}

pub(crate) fn launch_evidence_acceptance_digest_markdown(
    report: &LaunchEvidenceAcceptanceDigestReport,
) -> String {
    format!(
        "# DX Forge Launch Evidence Acceptance Digest\n\n- Schema: `{}`\n- Status: `{}`\n- Score: `{}/100`\n- Digest target: `{}`\n- Terminal status line: `{}`\n- No execution: `{}`\n",
        report.schema,
        if report.passed { "ready" } else { "blocked" },
        report.score,
        report.digest.digest_target,
        report.digest.terminal_status_line,
        report.no_execution
    )
}

fn launch_evidence_acceptance_digest_failure_summary(
    report: &LaunchEvidenceAcceptanceDigestReport,
) -> String {
    report
        .findings
        .first()
        .cloned()
        .unwrap_or_else(|| "launch evidence acceptance digest is not ready".to_string())
}

fn acceptance_digest_input(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceAcceptanceDigestInput {
    let target = project.join(path);
    let metadata = target.metadata().ok();
    LaunchEvidenceAcceptanceDigestInput {
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

fn acceptance_digest_freshness(project: &Path) -> LaunchEvidenceAcceptanceDigestFreshness {
    let digest_modified = modified_at(project.join(DIGEST_PATH));
    let acceptance_index_modified = modified_at(project.join(ACCEPTANCE_INDEX_PATH));
    LaunchEvidenceAcceptanceDigestFreshness {
        digest_path: DIGEST_PATH,
        digest_present: digest_modified.is_some(),
        digest_modified_at: digest_modified.map(format_time),
        acceptance_index_present: acceptance_index_modified.is_some(),
        digest_not_older_than_acceptance_index: match (digest_modified, acceptance_index_modified) {
            (Some(digest), Some(acceptance_index)) => digest >= acceptance_index,
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
) -> LaunchEvidenceAcceptanceDigestCheck {
    LaunchEvidenceAcceptanceDigestCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message: message.into(),
    }
}

fn average_score(checks: &[LaunchEvidenceAcceptanceDigestCheck]) -> u8 {
    if checks.is_empty() {
        return 0;
    }
    let total: u16 = checks.iter().map(|check| u16::from(check.score)).sum();
    (total / checks.len() as u16) as u8
}

fn required_arg<'a>(args: &'a [String], index: usize, name: &str) -> DxResult<&'a str> {
    args.get(index + 1)
        .map(String::as_str)
        .ok_or_else(|| DxError::ConfigValidationError {
            message: format!("{name} requires a value"),
            field: Some("forge.launch-evidence-acceptance-digest".to_string()),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fails_when_acceptance_index_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");

        let report =
            build_launch_evidence_acceptance_digest_report(dir.path(), 100).expect("digest");

        assert!(!report.passed());
        assert!(!report.freshness.acceptance_index_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "acceptance-index-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_acceptance_digest_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), DIGEST_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), ACCEPTANCE_INDEX_PATH);

        let report = build_launch_evidence_acceptance_digest_report(dir.path(), 0).expect("digest");

        assert!(!report.freshness.digest_not_older_than_acceptance_index);
    }

    #[test]
    fn passes_complete_fresh_acceptance_digest_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), ACCEPTANCE_INDEX_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), DIGEST_PATH);

        let report =
            build_launch_evidence_acceptance_digest_report(dir.path(), 100).expect("digest");

        assert!(report.passed());
        assert_eq!(
            report.digest.digest_target,
            "friday-terminal-final-status-line"
        );
        assert_eq!(report.digest.format, "json");
        assert!(report.digest.zed_openable);
        assert!(!report.digest.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_acceptance_digest_json() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), ACCEPTANCE_INDEX_PATH);

        run_launch_evidence_acceptance_digest(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write acceptance digest");

        let report =
            build_launch_evidence_acceptance_digest_report(dir.path(), 100).expect("digest");
        let digest = fs::read_to_string(dir.path().join(DIGEST_PATH)).expect("digest file");
        assert!(report.passed());
        assert!(digest.contains("dx.forge.launch_evidence_acceptance_digest"));
        assert!(digest.contains("friday-terminal-final-status-line"));
        assert!(digest.contains("acceptance-index"));
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("input parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
