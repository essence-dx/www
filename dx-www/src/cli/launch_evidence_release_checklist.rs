use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_release_checklist";
const CHECKLIST_SCHEMA: &str = "dx.launch.evidence_release_checklist";
const CHECKLIST_PATH: &str = ".dx/forge/release/launch-evidence-release-checklist.json";
const DIGEST_PATH: &str = ".dx/forge/release/launch-evidence-handoff-digest.md";
const TIMELINE_PATH: &str = ".dx/forge/release/launch-evidence-status-timeline.json";
const OPERATOR_INDEX_PATH: &str = ".dx/forge/release/launch-evidence-operator-index.json";
const PACKET_PATH: &str = ".dx/forge/release/launch-evidence-packet.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceReleaseChecklistReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    checklist: LaunchEvidenceReleaseChecklistSummary,
    signoff_items: Vec<LaunchEvidenceReleaseSignoffItem>,
    freshness: LaunchEvidenceReleaseChecklistFreshness,
    checks: Vec<LaunchEvidenceReleaseChecklistCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceReleaseChecklistReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceReleaseChecklistSummary {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    release_ready: bool,
    signoff_item_count: usize,
    completed_signoff_items: usize,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceReleaseSignoffItem {
    id: &'static str,
    label: &'static str,
    path: &'static str,
    present: bool,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceReleaseChecklistFreshness {
    checklist_path: &'static str,
    checklist_present: bool,
    checklist_modified_at: Option<String>,
    digest_present: bool,
    timeline_present: bool,
    operator_index_present: bool,
    packet_present: bool,
    checklist_not_older_than_inputs: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceReleaseChecklistCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_release_checklist(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-release-checklist option: {value}"
                    ),
                    field: Some("forge.launch-evidence-release-checklist".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-release-checklist argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-release-checklist".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(CHECKLIST_PATH));
    }

    let mut report = build_launch_evidence_release_checklist_report(&project, fail_under)
        .map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_release_checklist_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_release_checklist(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_release_checklist(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_release_checklist_failure_summary(&report),
            field: Some("forge.launch-evidence-release-checklist".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_release_checklist(
    report: &LaunchEvidenceReleaseChecklistReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_release_checklist_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_release_checklist_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_release_checklist_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceReleaseChecklistReport> {
    let signoff_items = signoff_items(project);
    let completed_signoff_items = signoff_items.iter().filter(|item| item.present).count();
    let release_ready = completed_signoff_items == signoff_items.len();
    let freshness = release_checklist_freshness(project, &signoff_items);
    let checks = vec![
        check(
            "release-signoff-inputs",
            release_ready,
            format!(
                "{completed_signoff_items}/{} release signoff input(s) are present",
                signoff_items.len()
            ),
        ),
        check(
            "checklist-freshness",
            freshness.checklist_not_older_than_inputs,
            "release checklist is not older than digest, timeline, operator index, or packet"
                .to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "release checklist uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceReleaseChecklistReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        checklist: LaunchEvidenceReleaseChecklistSummary {
            schema: CHECKLIST_SCHEMA,
            path: CHECKLIST_PATH,
            command: "dx forge launch-evidence-release-checklist --project <path> --write",
            release_ready,
            signoff_item_count: signoff_items.len(),
            completed_signoff_items,
            reads_runtime_artifact_contents: false,
        },
        signoff_items,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-handoff-digest --project . --write".to_string(),
            "dx forge launch-evidence-release-checklist --project . --write".to_string(),
            "dx forge launch-evidence-share-manifest --project . --write".to_string(),
            format!("open {CHECKLIST_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_release_checklist_terminal(
    report: &LaunchEvidenceReleaseChecklistReport,
) -> String {
    format!(
        "DX Forge launch evidence release checklist\nProject: {}\nPassed: {}\nScore: {}\nSignoff: {}/{}\nChecklist fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.checklist.completed_signoff_items,
        report.checklist.signoff_item_count,
        report.freshness.checklist_not_older_than_inputs,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_release_checklist_markdown(
    report: &LaunchEvidenceReleaseChecklistReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Release Checklist\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Release ready: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.checklist.release_ready,
        report.no_execution
    );
    output.push_str("| Signoff | Present | Command |\n| --- | --- | --- |\n");
    for item in &report.signoff_items {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` |\n",
            item.id, item.present, item.command
        ));
    }
    output
}

pub(crate) fn launch_evidence_release_checklist_failure_summary(
    report: &LaunchEvidenceReleaseChecklistReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence release checklist score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn signoff_items(project: &Path) -> Vec<LaunchEvidenceReleaseSignoffItem> {
    [
        (
            "handoff-digest",
            "Zed-openable handoff digest exists",
            DIGEST_PATH,
            "dx forge launch-evidence-handoff-digest --project . --write",
        ),
        (
            "status-timeline",
            "Status timeline exists",
            TIMELINE_PATH,
            "dx forge launch-evidence-status-timeline --project . --write",
        ),
        (
            "operator-index",
            "Operator index exists",
            OPERATOR_INDEX_PATH,
            "dx forge launch-evidence-operator-index --project . --write",
        ),
        (
            "release-packet",
            "Release packet exists",
            PACKET_PATH,
            "dx forge launch-evidence-packet --project . --write",
        ),
    ]
    .into_iter()
    .map(|(id, label, path, command)| signoff_item(project, id, label, path, command))
    .collect()
}

fn signoff_item(
    project: &Path,
    id: &'static str,
    label: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceReleaseSignoffItem {
    LaunchEvidenceReleaseSignoffItem {
        id,
        label,
        path,
        command,
        present: file_modified_at(&project.join(path)).is_some(),
    }
}

fn release_checklist_freshness(
    project: &Path,
    signoff_items: &[LaunchEvidenceReleaseSignoffItem],
) -> LaunchEvidenceReleaseChecklistFreshness {
    let checklist_modified = file_modified_at(&project.join(CHECKLIST_PATH));
    let newest_input = signoff_items
        .iter()
        .filter_map(|item| file_modified_at(&project.join(item.path)))
        .max();
    let checklist_not_older_than_inputs = match (checklist_modified, newest_input) {
        (Some(checklist), Some(input)) => checklist >= input,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceReleaseChecklistFreshness {
        checklist_path: CHECKLIST_PATH,
        checklist_present: checklist_modified.is_some(),
        checklist_modified_at: checklist_modified.map(format_system_time),
        digest_present: signoff_present(signoff_items, "handoff-digest"),
        timeline_present: signoff_present(signoff_items, "status-timeline"),
        operator_index_present: signoff_present(signoff_items, "operator-index"),
        packet_present: signoff_present(signoff_items, "release-packet"),
        checklist_not_older_than_inputs,
        timestamp_source: "filesystem-metadata",
    }
}

fn signoff_present(signoff_items: &[LaunchEvidenceReleaseSignoffItem], id: &str) -> bool {
    signoff_items
        .iter()
        .any(|item| item.id == id && item.present)
}

fn file_modified_at(path: &Path) -> Option<SystemTime> {
    let metadata = fs::metadata(path).ok()?;
    if !metadata.is_file() {
        return None;
    }
    metadata.modified().ok()
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
            field: Some("forge.launch-evidence-release-checklist".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceReleaseChecklistCheck {
    LaunchEvidenceReleaseChecklistCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceReleaseChecklistCheck]) -> u8 {
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
    fn fails_when_digest_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [TIMELINE_PATH, OPERATOR_INDEX_PATH, PACKET_PATH] {
            write_input(dir.path(), path);
        }

        let report =
            build_launch_evidence_release_checklist_report(dir.path(), 100).expect("checklist");

        assert!(!report.passed());
        assert!(!report.freshness.digest_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "release-signoff-inputs" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_checklist_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), CHECKLIST_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        for path in [DIGEST_PATH, TIMELINE_PATH, OPERATOR_INDEX_PATH, PACKET_PATH] {
            write_input(dir.path(), path);
        }

        let report =
            build_launch_evidence_release_checklist_report(dir.path(), 0).expect("checklist");

        assert!(!report.freshness.checklist_not_older_than_inputs);
    }

    #[test]
    fn passes_complete_fresh_checklist_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [DIGEST_PATH, TIMELINE_PATH, OPERATOR_INDEX_PATH, PACKET_PATH] {
            write_input(dir.path(), path);
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), CHECKLIST_PATH);

        let report =
            build_launch_evidence_release_checklist_report(dir.path(), 100).expect("checklist");

        assert!(report.passed());
        assert!(report.checklist.release_ready);
        assert!(!report.checklist.reads_runtime_artifact_contents);
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
