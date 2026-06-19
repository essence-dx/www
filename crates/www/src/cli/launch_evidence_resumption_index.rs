use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_resumption_index";
const INDEX_SCHEMA: &str = "dx.launch.evidence_resumption_index";
const INDEX_PATH: &str = ".dx/forge/release/launch-evidence-resumption-index.json";
const HANDOFF_CAPSULE_PATH: &str = ".dx/forge/release/launch-evidence-handoff-capsule.json";
const OPERATOR_RUNBOOK_PATH: &str = ".dx/forge/release/launch-evidence-operator-runbook.json";
const FINAL_BRIEF_PATH: &str = ".dx/forge/release/launch-evidence-final-brief.json";
const CLOSURE_MEMO_PATH: &str = ".dx/forge/release/launch-evidence-closure-memo.md";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceResumptionIndexReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    index: LaunchEvidenceResumptionIndex,
    lanes: Vec<LaunchEvidenceResumptionLane>,
    restart_artifacts: Vec<LaunchEvidenceResumptionArtifact>,
    freshness: LaunchEvidenceResumptionFreshness,
    checks: Vec<LaunchEvidenceResumptionCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceResumptionIndexReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceResumptionIndex {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    resumption_target: &'static str,
    lanes: Vec<LaunchEvidenceResumptionLane>,
    artifact_count: usize,
    present_artifacts: usize,
    reads_runtime_artifact_contents: bool,
}

#[derive(Clone, Debug, Serialize)]
struct LaunchEvidenceResumptionLane {
    id: &'static str,
    label: &'static str,
    command: &'static str,
    evidence: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceResumptionArtifact {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceResumptionFreshness {
    index_path: &'static str,
    index_present: bool,
    index_modified_at: Option<String>,
    handoff_capsule_present: bool,
    index_not_older_than_handoff_capsule: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceResumptionCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_resumption_index(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-resumption-index option: {value}"
                    ),
                    field: Some("forge.launch-evidence-resumption-index".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-resumption-index argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-resumption-index".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(INDEX_PATH));
    }

    let mut report =
        build_launch_evidence_resumption_index_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_resumption_index_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_resumption_index(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_resumption_index(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_resumption_index_failure_summary(&report),
            field: Some("forge.launch-evidence-resumption-index".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_resumption_index(
    report: &LaunchEvidenceResumptionIndexReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_resumption_index_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_resumption_index_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_resumption_index_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceResumptionIndexReport> {
    let restart_artifacts = resumption_artifacts(project);
    let present_artifacts = restart_artifacts
        .iter()
        .filter(|artifact| artifact.present)
        .count();
    let all_artifacts_present = present_artifacts == restart_artifacts.len();
    let lanes = resumption_lanes();
    let freshness = resumption_freshness(project, &restart_artifacts);
    let checks = vec![
        check(
            "handoff-capsule-present",
            freshness.handoff_capsule_present,
            format!("handoff capsule exists at {HANDOFF_CAPSULE_PATH}"),
        ),
        check(
            "resumption-index-artifacts-present",
            all_artifacts_present,
            format!(
                "{present_artifacts}/{} resumption index artifact(s) are present",
                restart_artifacts.len()
            ),
        ),
        check(
            "resumption-index-freshness",
            freshness.index_not_older_than_handoff_capsule,
            "resumption index is not older than the handoff capsule".to_string(),
        ),
        check(
            "ordered-dx-cli-zed-restart-lanes",
            lanes.len() == 3,
            "resumption index includes ordered source-only, runtime-approved, and release-closeout lanes"
                .to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "resumption index uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceResumptionIndexReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        index: LaunchEvidenceResumptionIndex {
            schema: INDEX_SCHEMA,
            path: INDEX_PATH,
            command: "dx forge launch-evidence-resumption-index --project <path> --write",
            resumption_target: "ordered-dx-cli-zed-restart-lanes",
            lanes: lanes.clone(),
            artifact_count: restart_artifacts.len(),
            present_artifacts,
            reads_runtime_artifact_contents: false,
        },
        lanes,
        restart_artifacts,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-handoff-capsule --project . --write".to_string(),
            "dx forge launch-evidence-resumption-index --project . --write".to_string(),
            "dx forge launch-evidence-recovery-brief --project . --write".to_string(),
            format!("open {INDEX_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_resumption_index_terminal(
    report: &LaunchEvidenceResumptionIndexReport,
) -> String {
    format!(
        "DX Forge launch evidence resumption index\nProject: {}\nPassed: {}\nScore: {}\nResumption target: {}\nLanes: {}\nIndex fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.index.resumption_target,
        report.index.lanes.len(),
        report.freshness.index_not_older_than_handoff_capsule,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_resumption_index_markdown(
    report: &LaunchEvidenceResumptionIndexReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Resumption Index\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Resumption target: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.index.resumption_target,
        report.no_execution
    );
    output.push_str("## Restart Lanes\n\n");
    for lane in &report.index.lanes {
        output.push_str(&format!(
            "- `{}`: {} (`{}` -> `{}`)\n",
            lane.id, lane.label, lane.command, lane.evidence
        ));
    }
    output.push_str(
        "\n## Restart Artifacts\n\n| Artifact | Present | Modified | Bytes | Path |\n| --- | --- | --- | --- | --- |\n",
    );
    for artifact in &report.restart_artifacts {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            artifact.id,
            artifact.present,
            artifact.modified_at.as_deref().unwrap_or("missing"),
            artifact
                .bytes
                .map(|bytes| bytes.to_string())
                .unwrap_or_else(|| "missing".to_string()),
            artifact.path
        ));
    }
    output
}

pub(crate) fn launch_evidence_resumption_index_failure_summary(
    report: &LaunchEvidenceResumptionIndexReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence resumption index score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn resumption_artifacts(project: &Path) -> Vec<LaunchEvidenceResumptionArtifact> {
    [
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
        (
            "closure-memo",
            CLOSURE_MEMO_PATH,
            "dx forge launch-evidence-closure-memo --project . --write",
        ),
    ]
    .into_iter()
    .map(|(id, path, command)| resumption_artifact(project, id, path, command))
    .collect()
}

fn resumption_artifact(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceResumptionArtifact {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceResumptionArtifact {
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

fn resumption_lanes() -> Vec<LaunchEvidenceResumptionLane> {
    vec![
        LaunchEvidenceResumptionLane {
            id: "source-only",
            label: "Continue source-level launch evidence work",
            command: "dx run --test .\\benchmarks\\template-shell.test.ts",
            evidence: INDEX_PATH,
        },
        LaunchEvidenceResumptionLane {
            id: "runtime-approved",
            label: "Resume only after explicit runtime approval",
            command: "dx forge launch-runtime-evidence-review --project <path> --json",
            evidence: ".dx/forge/runtime/final-launch-evidence-review.json",
        },
        LaunchEvidenceResumptionLane {
            id: "release-closeout",
            label: "Refresh release closeout artifacts",
            command: "dx forge launch-evidence-handoff-capsule --project <path> --write",
            evidence: HANDOFF_CAPSULE_PATH,
        },
    ]
}

fn resumption_freshness(
    project: &Path,
    restart_artifacts: &[LaunchEvidenceResumptionArtifact],
) -> LaunchEvidenceResumptionFreshness {
    let index_modified =
        file_metadata(&project.join(INDEX_PATH)).map(|metadata| metadata.modified_at);
    let capsule_modified =
        file_metadata(&project.join(HANDOFF_CAPSULE_PATH)).map(|metadata| metadata.modified_at);
    let index_not_older_than_handoff_capsule = match (index_modified, capsule_modified) {
        (Some(index), Some(capsule)) => index >= capsule,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceResumptionFreshness {
        index_path: INDEX_PATH,
        index_present: index_modified.is_some(),
        index_modified_at: index_modified.map(format_system_time),
        handoff_capsule_present: artifact_present(restart_artifacts, "handoff-capsule"),
        index_not_older_than_handoff_capsule,
        timestamp_source: "filesystem-metadata",
    }
}

fn artifact_present(artifacts: &[LaunchEvidenceResumptionArtifact], id: &str) -> bool {
    artifacts
        .iter()
        .any(|artifact| artifact.id == id && artifact.present)
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
            field: Some("forge.launch-evidence-resumption-index".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceResumptionCheck {
    LaunchEvidenceResumptionCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceResumptionCheck]) -> u8 {
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
    fn fails_when_handoff_capsule_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [OPERATOR_RUNBOOK_PATH, FINAL_BRIEF_PATH, CLOSURE_MEMO_PATH] {
            write_input(dir.path(), path);
        }

        let report = build_launch_evidence_resumption_index_report(dir.path(), 100).expect("index");

        assert!(!report.passed());
        assert!(!report.freshness.handoff_capsule_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "handoff-capsule-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_resumption_index_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), INDEX_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), HANDOFF_CAPSULE_PATH);

        let report = build_launch_evidence_resumption_index_report(dir.path(), 0).expect("index");

        assert!(!report.freshness.index_not_older_than_handoff_capsule);
    }

    #[test]
    fn passes_complete_fresh_index_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_resumption_inputs(dir.path());
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), INDEX_PATH);

        let report = build_launch_evidence_resumption_index_report(dir.path(), 100).expect("index");

        assert!(report.passed());
        assert_eq!(
            report.index.resumption_target,
            "ordered-dx-cli-zed-restart-lanes"
        );
        assert_eq!(report.index.lanes.len(), 3);
        assert!(!report.index.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_resumption_index() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_resumption_inputs(dir.path());

        run_launch_evidence_resumption_index(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write index");

        let report = build_launch_evidence_resumption_index_report(dir.path(), 100).expect("index");
        assert!(report.passed());
        assert!(dir.path().join(INDEX_PATH).is_file());
    }

    fn write_resumption_inputs(project: &Path) {
        for path in [
            HANDOFF_CAPSULE_PATH,
            OPERATOR_RUNBOOK_PATH,
            FINAL_BRIEF_PATH,
            CLOSURE_MEMO_PATH,
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
