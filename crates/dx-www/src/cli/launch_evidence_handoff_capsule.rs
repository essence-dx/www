use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_handoff_capsule";
const CAPSULE_SCHEMA: &str = "dx.launch.evidence_handoff_capsule";
const CAPSULE_PATH: &str = ".dx/forge/release/launch-evidence-handoff-capsule.json";
const OPERATOR_RUNBOOK_PATH: &str = ".dx/forge/release/launch-evidence-operator-runbook.json";
const FINAL_BRIEF_PATH: &str = ".dx/forge/release/launch-evidence-final-brief.json";
const CLOSURE_MEMO_PATH: &str = ".dx/forge/release/launch-evidence-closure-memo.md";
const COMPLETION_LEDGER_PATH: &str = ".dx/forge/release/launch-evidence-completion-ledger.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceHandoffCapsuleReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    capsule: LaunchEvidenceHandoffCapsule,
    restart_artifacts: Vec<LaunchEvidenceHandoffCapsuleArtifact>,
    freshness: LaunchEvidenceHandoffCapsuleFreshness,
    checks: Vec<LaunchEvidenceHandoffCapsuleCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceHandoffCapsuleReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceHandoffCapsule {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    capsule_target: &'static str,
    restart_steps: Vec<LaunchEvidenceHandoffCapsuleStep>,
    artifact_count: usize,
    present_artifacts: usize,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceHandoffCapsuleStep {
    id: &'static str,
    label: &'static str,
    command: &'static str,
    evidence: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceHandoffCapsuleArtifact {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceHandoffCapsuleFreshness {
    capsule_path: &'static str,
    capsule_present: bool,
    capsule_modified_at: Option<String>,
    operator_runbook_present: bool,
    capsule_not_older_than_operator_runbook: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceHandoffCapsuleCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_handoff_capsule(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-handoff-capsule option: {value}"
                    ),
                    field: Some("forge.launch-evidence-handoff-capsule".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-handoff-capsule argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-handoff-capsule".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(CAPSULE_PATH));
    }

    let mut report =
        build_launch_evidence_handoff_capsule_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_handoff_capsule_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_handoff_capsule(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_handoff_capsule(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_handoff_capsule_failure_summary(&report),
            field: Some("forge.launch-evidence-handoff-capsule".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_handoff_capsule(
    report: &LaunchEvidenceHandoffCapsuleReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_handoff_capsule_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_handoff_capsule_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_handoff_capsule_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceHandoffCapsuleReport> {
    let restart_artifacts = handoff_capsule_artifacts(project);
    let present_artifacts = restart_artifacts
        .iter()
        .filter(|artifact| artifact.present)
        .count();
    let all_artifacts_present = present_artifacts == restart_artifacts.len();
    let restart_steps = handoff_capsule_steps();
    let freshness = handoff_capsule_freshness(project, &restart_artifacts);
    let checks = vec![
        check(
            "operator-runbook-present",
            freshness.operator_runbook_present,
            format!("operator runbook exists at {OPERATOR_RUNBOOK_PATH}"),
        ),
        check(
            "handoff-capsule-artifacts-present",
            all_artifacts_present,
            format!(
                "{present_artifacts}/{} handoff capsule artifact(s) are present",
                restart_artifacts.len()
            ),
        ),
        check(
            "handoff-capsule-freshness",
            freshness.capsule_not_older_than_operator_runbook,
            "handoff capsule is not older than the operator runbook".to_string(),
        ),
        check(
            "dx-cli-zed-restart-artifact",
            !restart_steps.is_empty(),
            "handoff capsule includes DX CLI/Zed restart artifact steps".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "handoff capsule uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceHandoffCapsuleReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        capsule: LaunchEvidenceHandoffCapsule {
            schema: CAPSULE_SCHEMA,
            path: CAPSULE_PATH,
            command: "dx forge launch-evidence-handoff-capsule --project <path> --write",
            capsule_target: "dx-cli-zed-restart-artifact",
            restart_steps,
            artifact_count: restart_artifacts.len(),
            present_artifacts,
            reads_runtime_artifact_contents: false,
        },
        restart_artifacts,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-operator-runbook --project . --write".to_string(),
            "dx forge launch-evidence-handoff-capsule --project . --write".to_string(),
            "dx forge launch-evidence-resumption-index --project . --write".to_string(),
            format!("open {CAPSULE_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_handoff_capsule_terminal(
    report: &LaunchEvidenceHandoffCapsuleReport,
) -> String {
    format!(
        "DX Forge launch evidence handoff capsule\nProject: {}\nPassed: {}\nScore: {}\nCapsule target: {}\nArtifacts: {}/{}\nCapsule fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.capsule.capsule_target,
        report.capsule.present_artifacts,
        report.capsule.artifact_count,
        report.freshness.capsule_not_older_than_operator_runbook,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_handoff_capsule_markdown(
    report: &LaunchEvidenceHandoffCapsuleReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Handoff Capsule\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Capsule target: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.capsule.capsule_target,
        report.no_execution
    );
    output.push_str("## Restart Steps\n\n");
    for step in &report.capsule.restart_steps {
        output.push_str(&format!(
            "- `{}`: {} (`{}` -> `{}`)\n",
            step.id, step.label, step.command, step.evidence
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

pub(crate) fn launch_evidence_handoff_capsule_failure_summary(
    report: &LaunchEvidenceHandoffCapsuleReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence handoff capsule score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn handoff_capsule_artifacts(project: &Path) -> Vec<LaunchEvidenceHandoffCapsuleArtifact> {
    [
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
        (
            "completion-ledger",
            COMPLETION_LEDGER_PATH,
            "dx forge launch-evidence-completion-ledger --project . --write",
        ),
    ]
    .into_iter()
    .map(|(id, path, command)| handoff_capsule_artifact(project, id, path, command))
    .collect()
}

fn handoff_capsule_artifact(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceHandoffCapsuleArtifact {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceHandoffCapsuleArtifact {
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

fn handoff_capsule_steps() -> Vec<LaunchEvidenceHandoffCapsuleStep> {
    vec![
        LaunchEvidenceHandoffCapsuleStep {
            id: "review-operator-runbook",
            label: "Review the restartable operator runbook",
            command: "dx forge launch-evidence-operator-runbook --project . --write",
            evidence: OPERATOR_RUNBOOK_PATH,
        },
        LaunchEvidenceHandoffCapsuleStep {
            id: "review-final-brief",
            label: "Review the final launch closeout pointer",
            command: "dx forge launch-evidence-final-brief --project . --write",
            evidence: FINAL_BRIEF_PATH,
        },
        LaunchEvidenceHandoffCapsuleStep {
            id: "open-closure-memo",
            label: "Open the human-readable closure memo",
            command: "open .dx/forge/release/launch-evidence-closure-memo.md",
            evidence: CLOSURE_MEMO_PATH,
        },
        LaunchEvidenceHandoffCapsuleStep {
            id: "confirm-completion-ledger",
            label: "Confirm the launch evidence completion ledger",
            command: "dx forge launch-evidence-completion-ledger --project . --write",
            evidence: COMPLETION_LEDGER_PATH,
        },
    ]
}

fn handoff_capsule_freshness(
    project: &Path,
    restart_artifacts: &[LaunchEvidenceHandoffCapsuleArtifact],
) -> LaunchEvidenceHandoffCapsuleFreshness {
    let capsule_modified =
        file_metadata(&project.join(CAPSULE_PATH)).map(|metadata| metadata.modified_at);
    let operator_runbook_modified =
        file_metadata(&project.join(OPERATOR_RUNBOOK_PATH)).map(|metadata| metadata.modified_at);
    let capsule_not_older_than_operator_runbook =
        match (capsule_modified, operator_runbook_modified) {
            (Some(capsule), Some(operator_runbook)) => capsule >= operator_runbook,
            (Some(_), None) => true,
            (None, None) => true,
            (None, Some(_)) => false,
        };

    LaunchEvidenceHandoffCapsuleFreshness {
        capsule_path: CAPSULE_PATH,
        capsule_present: capsule_modified.is_some(),
        capsule_modified_at: capsule_modified.map(format_system_time),
        operator_runbook_present: artifact_present(restart_artifacts, "operator-runbook"),
        capsule_not_older_than_operator_runbook,
        timestamp_source: "filesystem-metadata",
    }
}

fn artifact_present(artifacts: &[LaunchEvidenceHandoffCapsuleArtifact], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-handoff-capsule".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceHandoffCapsuleCheck {
    LaunchEvidenceHandoffCapsuleCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceHandoffCapsuleCheck]) -> u8 {
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
    fn fails_when_operator_runbook_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [FINAL_BRIEF_PATH, CLOSURE_MEMO_PATH, COMPLETION_LEDGER_PATH] {
            write_input(dir.path(), path);
        }

        let report =
            build_launch_evidence_handoff_capsule_report(dir.path(), 100).expect("capsule");

        assert!(!report.passed());
        assert!(!report.freshness.operator_runbook_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "operator-runbook-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_handoff_capsule_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), CAPSULE_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), OPERATOR_RUNBOOK_PATH);

        let report = build_launch_evidence_handoff_capsule_report(dir.path(), 0).expect("capsule");

        assert!(!report.freshness.capsule_not_older_than_operator_runbook);
    }

    #[test]
    fn passes_complete_fresh_handoff_capsule_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_handoff_capsule_inputs(dir.path());
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), CAPSULE_PATH);

        let report =
            build_launch_evidence_handoff_capsule_report(dir.path(), 100).expect("capsule");

        assert!(report.passed());
        assert_eq!(report.capsule.capsule_target, "dx-cli-zed-restart-artifact");
        assert_eq!(report.capsule.restart_steps.len(), 4);
        assert!(!report.capsule.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_handoff_capsule() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_handoff_capsule_inputs(dir.path());

        run_launch_evidence_handoff_capsule(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write capsule");

        let report =
            build_launch_evidence_handoff_capsule_report(dir.path(), 100).expect("capsule");
        assert!(report.passed());
        assert!(dir.path().join(CAPSULE_PATH).is_file());
    }

    fn write_handoff_capsule_inputs(project: &Path) {
        for path in [
            OPERATOR_RUNBOOK_PATH,
            FINAL_BRIEF_PATH,
            CLOSURE_MEMO_PATH,
            COMPLETION_LEDGER_PATH,
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
