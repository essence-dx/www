use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_handoff_digest";
const DIGEST_SCHEMA: &str = "dx.launch.evidence_handoff_digest";
const DIGEST_PATH: &str = ".dx/forge/release/launch-evidence-handoff-digest.md";
const TIMELINE_PATH: &str = ".dx/forge/release/launch-evidence-status-timeline.json";
const OPERATOR_INDEX_PATH: &str = ".dx/forge/release/launch-evidence-operator-index.json";
const PACKET_PATH: &str = ".dx/forge/release/launch-evidence-packet.json";
const FINAL_REVIEW_PATH: &str = ".dx/forge/runtime/final-launch-evidence-review.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceHandoffDigestReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    digest: LaunchEvidenceDigestSummary,
    inputs: Vec<LaunchEvidenceDigestInput>,
    freshness: LaunchEvidenceDigestFreshness,
    checks: Vec<LaunchEvidenceDigestCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceHandoffDigestReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceDigestSummary {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    zed_openable: bool,
    format: &'static str,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceDigestInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceDigestFreshness {
    digest_path: &'static str,
    digest_present: bool,
    digest_modified_at: Option<String>,
    timeline_present: bool,
    operator_index_present: bool,
    packet_present: bool,
    final_review_present: bool,
    digest_not_older_than_inputs: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceDigestCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_handoff_digest(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-handoff-digest option: {value}"
                    ),
                    field: Some("forge.launch-evidence-handoff-digest".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-handoff-digest argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-handoff-digest".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(DIGEST_PATH));
    }

    let mut report =
        build_launch_evidence_handoff_digest_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "").map_err(forge_error)?;
            report = build_launch_evidence_handoff_digest_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_handoff_digest(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_handoff_digest(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_handoff_digest_failure_summary(&report),
            field: Some("forge.launch-evidence-handoff-digest".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_handoff_digest(
    report: &LaunchEvidenceHandoffDigestReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_handoff_digest_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_handoff_digest_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_handoff_digest_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceHandoffDigestReport> {
    let inputs = digest_inputs(project);
    let freshness = digest_freshness(project, &inputs);
    let all_inputs_present = inputs.iter().all(|input| input.present);
    let checks = vec![
        check(
            "handoff-inputs-present",
            all_inputs_present,
            format!(
                "{}/{} handoff digest input(s) are present",
                inputs.iter().filter(|input| input.present).count(),
                inputs.len()
            ),
        ),
        check(
            "digest-freshness",
            freshness.digest_not_older_than_inputs,
            "handoff digest is not older than timeline, operator index, packet, or final review"
                .to_string(),
        ),
        check(
            "zed-openable-markdown",
            true,
            "handoff digest writes a Zed-openable Markdown report".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "handoff digest uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceHandoffDigestReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        digest: LaunchEvidenceDigestSummary {
            schema: DIGEST_SCHEMA,
            path: DIGEST_PATH,
            command: "dx forge launch-evidence-handoff-digest --project <path> --write",
            zed_openable: true,
            format: "markdown",
            reads_runtime_artifact_contents: false,
        },
        inputs,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-status-timeline --project . --write".to_string(),
            "dx forge launch-evidence-handoff-digest --project . --write".to_string(),
            "dx forge launch-evidence-release-checklist --project . --write".to_string(),
            format!("open {DIGEST_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_handoff_digest_terminal(
    report: &LaunchEvidenceHandoffDigestReport,
) -> String {
    format!(
        "DX Forge launch evidence handoff digest\nProject: {}\nPassed: {}\nScore: {}\nInputs present: {}/{}\nDigest fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.inputs.iter().filter(|input| input.present).count(),
        report.inputs.len(),
        report.freshness.digest_not_older_than_inputs,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_handoff_digest_markdown(
    report: &LaunchEvidenceHandoffDigestReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Handoff Digest\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Digest fresh: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.freshness.digest_not_older_than_inputs,
        report.no_execution
    );
    output.push_str("| Input | Present | Modified | Bytes |\n| --- | --- | --- | --- |\n");
    for input in &report.inputs {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` |\n",
            input.id,
            input.present,
            input.modified_at.as_deref().unwrap_or("missing"),
            input
                .bytes
                .map(|bytes| bytes.to_string())
                .unwrap_or_else(|| "missing".to_string())
        ));
    }
    output
}

pub(crate) fn launch_evidence_handoff_digest_failure_summary(
    report: &LaunchEvidenceHandoffDigestReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence handoff digest score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn digest_inputs(project: &Path) -> Vec<LaunchEvidenceDigestInput> {
    [
        ("status-timeline", TIMELINE_PATH),
        ("operator-index", OPERATOR_INDEX_PATH),
        ("release-packet", PACKET_PATH),
        ("final-runtime-review", FINAL_REVIEW_PATH),
    ]
    .into_iter()
    .map(|(id, path)| digest_input(project, id, path))
    .collect()
}

fn digest_input(project: &Path, id: &'static str, path: &'static str) -> LaunchEvidenceDigestInput {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceDigestInput {
        id,
        path,
        present: metadata.is_some(),
        modified_at: metadata
            .as_ref()
            .map(|metadata| format_system_time(metadata.modified_at)),
        bytes: metadata.map(|metadata| metadata.bytes),
    }
}

fn digest_freshness(
    project: &Path,
    inputs: &[LaunchEvidenceDigestInput],
) -> LaunchEvidenceDigestFreshness {
    let digest_modified =
        file_metadata(&project.join(DIGEST_PATH)).map(|metadata| metadata.modified_at);
    let newest_input = inputs
        .iter()
        .filter_map(|input| {
            file_metadata(&project.join(input_path(input.id))).map(|metadata| metadata.modified_at)
        })
        .max();
    let digest_not_older_than_inputs = match (digest_modified, newest_input) {
        (Some(digest), Some(input)) => digest >= input,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceDigestFreshness {
        digest_path: DIGEST_PATH,
        digest_present: digest_modified.is_some(),
        digest_modified_at: digest_modified.map(format_system_time),
        timeline_present: input_present(inputs, "status-timeline"),
        operator_index_present: input_present(inputs, "operator-index"),
        packet_present: input_present(inputs, "release-packet"),
        final_review_present: input_present(inputs, "final-runtime-review"),
        digest_not_older_than_inputs,
        timestamp_source: "filesystem-metadata",
    }
}

fn input_path(id: &str) -> &'static str {
    match id {
        "status-timeline" => TIMELINE_PATH,
        "operator-index" => OPERATOR_INDEX_PATH,
        "release-packet" => PACKET_PATH,
        "final-runtime-review" => FINAL_REVIEW_PATH,
        _ => DIGEST_PATH,
    }
}

fn input_present(inputs: &[LaunchEvidenceDigestInput], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-handoff-digest".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceDigestCheck {
    LaunchEvidenceDigestCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceDigestCheck]) -> u8 {
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
    fn fails_when_timeline_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), OPERATOR_INDEX_PATH);
        write_input(dir.path(), PACKET_PATH);
        write_input(dir.path(), FINAL_REVIEW_PATH);

        let report =
            build_launch_evidence_handoff_digest_report(dir.path(), 100).expect("digest report");

        assert!(!report.passed());
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "handoff-inputs-present" && !check.passed)
        );
        assert!(!report.freshness.timeline_present);
    }

    #[test]
    fn reports_stale_digest_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), DIGEST_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        for path in [
            TIMELINE_PATH,
            OPERATOR_INDEX_PATH,
            PACKET_PATH,
            FINAL_REVIEW_PATH,
        ] {
            write_input(dir.path(), path);
        }

        let report =
            build_launch_evidence_handoff_digest_report(dir.path(), 0).expect("digest report");

        assert!(!report.freshness.digest_not_older_than_inputs);
    }

    #[test]
    fn passes_complete_fresh_digest_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            TIMELINE_PATH,
            OPERATOR_INDEX_PATH,
            PACKET_PATH,
            FINAL_REVIEW_PATH,
        ] {
            write_input(dir.path(), path);
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), DIGEST_PATH);

        let report =
            build_launch_evidence_handoff_digest_report(dir.path(), 100).expect("digest report");

        assert!(report.passed());
        assert!(report.digest.zed_openable);
        assert_eq!(report.digest.format, "markdown");
        assert!(!report.digest.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_zed_openable_digest() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            TIMELINE_PATH,
            OPERATOR_INDEX_PATH,
            PACKET_PATH,
            FINAL_REVIEW_PATH,
        ] {
            write_input(dir.path(), path);
        }

        run_launch_evidence_handoff_digest(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write digest");

        let digest = fs::read_to_string(dir.path().join(DIGEST_PATH)).expect("digest markdown");
        assert!(digest.contains("# DX Forge Launch Evidence Handoff Digest"));
        let report =
            build_launch_evidence_handoff_digest_report(dir.path(), 100).expect("digest report");
        assert!(report.passed());
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
