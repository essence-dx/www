use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_friday_baton";
const BATON_SCHEMA: &str = "dx.launch.evidence_friday_baton";
const BATON_PATH: &str = ".dx/forge/release/launch-evidence-friday-baton.md";
const ACCEPTANCE_DIGEST_PATH: &str = ".dx/forge/release/launch-evidence-acceptance-digest.json";
const ACCEPTANCE_INDEX_PATH: &str = ".dx/forge/release/launch-evidence-acceptance-index.md";
const RESTART_SIGNOFF_PATH: &str = ".dx/forge/release/launch-evidence-restart-signoff.json";
const LAUNCH_VERIFICATION_LANE_PATH: &str =
    ".dx/forge/template-readiness/launch-verification-lane.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceFridayBatonReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    baton: LaunchEvidenceFridayBaton,
    inputs: Vec<LaunchEvidenceFridayBatonInput>,
    freshness: LaunchEvidenceFridayBatonFreshness,
    checks: Vec<LaunchEvidenceFridayBatonCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceFridayBatonReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceFridayBaton {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    baton_target: &'static str,
    acceptance_digest: &'static str,
    acceptance_index: &'static str,
    restart_signoff: &'static str,
    launch_verification_lane: &'static str,
    format: &'static str,
    zed_openable: bool,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceFridayBatonInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceFridayBatonFreshness {
    baton_path: &'static str,
    baton_present: bool,
    baton_modified_at: Option<String>,
    acceptance_digest_present: bool,
    baton_not_older_than_acceptance_digest: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceFridayBatonCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_friday_baton(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                    message: format!("Unknown forge launch-evidence-friday-baton option: {value}"),
                    field: Some("forge.launch-evidence-friday-baton".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-friday-baton argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-friday-baton".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(BATON_PATH));
    }

    let mut report =
        build_launch_evidence_friday_baton_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "# DX Forge Launch Evidence Friday Baton\n").map_err(forge_error)?;
            report = build_launch_evidence_friday_baton_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_friday_baton(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_friday_baton(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_friday_baton_failure_summary(&report),
            field: Some("forge.launch-evidence-friday-baton".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_friday_baton(
    report: &LaunchEvidenceFridayBatonReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_friday_baton_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_friday_baton_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_friday_baton_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceFridayBatonReport> {
    let inputs = friday_baton_inputs(project);
    let all_inputs_present = inputs.iter().all(|input| input.present);
    let freshness = friday_baton_freshness(project);
    let checks = vec![
        check(
            "acceptance-digest-present",
            freshness.acceptance_digest_present,
            format!("acceptance digest exists at {ACCEPTANCE_DIGEST_PATH}"),
        ),
        check(
            "friday-baton-fresh",
            freshness.baton_not_older_than_acceptance_digest,
            "Friday baton is not older than the acceptance digest".to_string(),
        ),
        check(
            "orchestrator-handoff-inputs-present",
            all_inputs_present,
            "acceptance digest, acceptance index, restart signoff, and launch verification lane are present".to_string(),
        ),
        check(
            "markdown-baton",
            true,
            "Friday baton writes a Zed-openable Markdown receipt".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "Friday baton reads file metadata only; no runtime artifact bodies are read".to_string(),
        ),
    ];
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .collect::<Vec<_>>();
    let score = average_score(&checks);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(LaunchEvidenceFridayBatonReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        baton: LaunchEvidenceFridayBaton {
            schema: BATON_SCHEMA,
            path: BATON_PATH,
            command: "dx forge launch-evidence-friday-baton --project <path> --write",
            baton_target: "friday-orchestrator-final-handoff",
            acceptance_digest: ACCEPTANCE_DIGEST_PATH,
            acceptance_index: ACCEPTANCE_INDEX_PATH,
            restart_signoff: RESTART_SIGNOFF_PATH,
            launch_verification_lane: LAUNCH_VERIFICATION_LANE_PATH,
            format: "markdown",
            zed_openable: true,
            reads_runtime_artifact_contents: false,
        },
        inputs,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-friday-baton --project . --write".to_string(),
            "dx forge launch-evidence-acceptance-digest --project . --write".to_string(),
            "dx forge launch-verification-lane --project . --json".to_string(),
            format!("open {BATON_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_friday_baton_terminal(
    report: &LaunchEvidenceFridayBatonReport,
) -> String {
    format!(
        "DX Forge Launch Evidence Friday Baton\nSchema: {}\nStatus: {}\nScore: {}/100\nTarget: {}\n",
        report.schema,
        if report.passed { "ready" } else { "blocked" },
        report.score,
        report.baton.baton_target
    )
}

pub(crate) fn launch_evidence_friday_baton_markdown(
    report: &LaunchEvidenceFridayBatonReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Friday Baton\n\n- Schema: `{}`\n- Status: `{}`\n- Score: `{}/100`\n- Baton target: `{}`\n- No execution: `{}`\n\n## Inputs\n\n",
        report.schema,
        if report.passed { "ready" } else { "blocked" },
        report.score,
        report.baton.baton_target,
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

fn launch_evidence_friday_baton_failure_summary(
    report: &LaunchEvidenceFridayBatonReport,
) -> String {
    report
        .findings
        .first()
        .cloned()
        .unwrap_or_else(|| "launch evidence Friday baton is not ready".to_string())
}

fn friday_baton_inputs(project: &Path) -> Vec<LaunchEvidenceFridayBatonInput> {
    [
        (
            "acceptance-digest",
            ACCEPTANCE_DIGEST_PATH,
            "dx forge launch-evidence-acceptance-digest --project <path> --write",
        ),
        (
            "acceptance-index",
            ACCEPTANCE_INDEX_PATH,
            "dx forge launch-evidence-acceptance-index --project <path> --write",
        ),
        (
            "restart-signoff",
            RESTART_SIGNOFF_PATH,
            "dx forge launch-evidence-restart-signoff --project <path> --write",
        ),
        (
            "launch-verification-lane",
            LAUNCH_VERIFICATION_LANE_PATH,
            "dx forge launch-verification-lane --project <path> --json",
        ),
    ]
    .into_iter()
    .map(|(id, path, command)| friday_baton_input(project, id, path, command))
    .collect()
}

fn friday_baton_input(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceFridayBatonInput {
    let target = project.join(path);
    let metadata = target.metadata().ok();
    LaunchEvidenceFridayBatonInput {
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

fn friday_baton_freshness(project: &Path) -> LaunchEvidenceFridayBatonFreshness {
    let baton_modified = modified_at(project.join(BATON_PATH));
    let acceptance_digest_modified = modified_at(project.join(ACCEPTANCE_DIGEST_PATH));
    LaunchEvidenceFridayBatonFreshness {
        baton_path: BATON_PATH,
        baton_present: baton_modified.is_some(),
        baton_modified_at: baton_modified.map(format_time),
        acceptance_digest_present: acceptance_digest_modified.is_some(),
        baton_not_older_than_acceptance_digest: match (baton_modified, acceptance_digest_modified) {
            (Some(baton), Some(acceptance_digest)) => baton >= acceptance_digest,
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
) -> LaunchEvidenceFridayBatonCheck {
    LaunchEvidenceFridayBatonCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message: message.into(),
    }
}

fn average_score(checks: &[LaunchEvidenceFridayBatonCheck]) -> u8 {
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
            field: Some("forge.launch-evidence-friday-baton".to_string()),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fails_when_acceptance_digest_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");

        let report = build_launch_evidence_friday_baton_report(dir.path(), 100).expect("baton");

        assert!(!report.passed());
        assert!(!report.freshness.acceptance_digest_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "acceptance-digest-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_friday_baton_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), BATON_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), ACCEPTANCE_DIGEST_PATH);

        let report = build_launch_evidence_friday_baton_report(dir.path(), 0).expect("baton");

        assert!(!report.freshness.baton_not_older_than_acceptance_digest);
    }

    #[test]
    fn passes_complete_fresh_friday_baton_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_inputs(dir.path());
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), BATON_PATH);

        let report = build_launch_evidence_friday_baton_report(dir.path(), 100).expect("baton");

        assert!(report.passed());
        assert_eq!(
            report.baton.baton_target,
            "friday-orchestrator-final-handoff"
        );
        assert_eq!(report.baton.format, "markdown");
        assert!(report.baton.zed_openable);
        assert!(!report.baton.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_friday_baton_markdown() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_inputs(dir.path());

        run_launch_evidence_friday_baton(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write Friday baton");

        let report = build_launch_evidence_friday_baton_report(dir.path(), 100).expect("baton");
        let baton = fs::read_to_string(dir.path().join(BATON_PATH)).expect("baton file");
        assert!(report.passed());
        assert!(baton.contains("DX Forge Launch Evidence Friday Baton"));
        assert!(baton.contains("friday-orchestrator-final-handoff"));
        assert!(baton.contains("acceptance-digest"));
    }

    fn write_inputs(project: &Path) {
        write_input(project, ACCEPTANCE_DIGEST_PATH);
        write_input(project, ACCEPTANCE_INDEX_PATH);
        write_input(project, RESTART_SIGNOFF_PATH);
        write_input(project, LAUNCH_VERIFICATION_LANE_PATH);
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("input parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
