use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_restart_manifest";
const MANIFEST_SCHEMA: &str = "dx.launch.evidence_restart_manifest";
const MANIFEST_PATH: &str = ".dx/forge/release/launch-evidence-restart-manifest.json";
const RESTART_BRIEF_PATH: &str = ".dx/forge/release/launch-evidence-restart-brief.md";
const RESTART_CHECKLIST_PATH: &str = ".dx/forge/release/launch-evidence-restart-checklist.json";
const RESTART_LEDGER_PATH: &str = ".dx/forge/release/launch-evidence-restart-ledger.json";
const OPERATOR_RESUME_CARD_PATH: &str =
    ".dx/forge/release/launch-evidence-operator-resume-card.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceRestartManifestReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    manifest: LaunchEvidenceRestartManifest,
    inputs: Vec<LaunchEvidenceRestartManifestInput>,
    freshness: LaunchEvidenceRestartManifestFreshness,
    checks: Vec<LaunchEvidenceRestartManifestCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceRestartManifestReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartManifest {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    manifest_target: &'static str,
    restart_brief: &'static str,
    indexable_paths: Vec<&'static str>,
    input_count: usize,
    present_inputs: usize,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartManifestInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartManifestFreshness {
    manifest_path: &'static str,
    manifest_present: bool,
    manifest_modified_at: Option<String>,
    restart_brief_present: bool,
    manifest_not_older_than_restart_brief: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRestartManifestCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_restart_manifest(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-restart-manifest option: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-manifest".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-restart-manifest argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-restart-manifest".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(MANIFEST_PATH));
    }

    let mut report =
        build_launch_evidence_restart_manifest_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_restart_manifest_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_restart_manifest(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_restart_manifest(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_restart_manifest_failure_summary(&report),
            field: Some("forge.launch-evidence-restart-manifest".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_restart_manifest(
    report: &LaunchEvidenceRestartManifestReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_restart_manifest_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_restart_manifest_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_restart_manifest_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceRestartManifestReport> {
    let inputs = restart_manifest_inputs(project);
    let present_inputs = inputs.iter().filter(|input| input.present).count();
    let all_inputs_present = present_inputs == inputs.len();
    let freshness = restart_manifest_freshness(project, &inputs);
    let checks = vec![
        check(
            "restart-brief-present",
            freshness.restart_brief_present,
            format!("restart brief exists at {RESTART_BRIEF_PATH}"),
        ),
        check(
            "restart-manifest-inputs-present",
            all_inputs_present,
            format!(
                "{present_inputs}/{} restart manifest input(s) are present",
                inputs.len()
            ),
        ),
        check(
            "restart-manifest-freshness",
            freshness.manifest_not_older_than_restart_brief,
            "restart manifest is not older than the restart brief".to_string(),
        ),
        check(
            "dx-cli-zed-indexable-restart-manifest",
            true,
            "restart manifest records indexable JSON paths for DX CLI/Zed".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "restart manifest uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceRestartManifestReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        manifest: LaunchEvidenceRestartManifest {
            schema: MANIFEST_SCHEMA,
            path: MANIFEST_PATH,
            command: "dx forge launch-evidence-restart-manifest --project <path> --write",
            manifest_target: "dx-cli-zed-indexable-restart-manifest",
            restart_brief: RESTART_BRIEF_PATH,
            indexable_paths: vec![
                RESTART_BRIEF_PATH,
                RESTART_CHECKLIST_PATH,
                RESTART_LEDGER_PATH,
                OPERATOR_RESUME_CARD_PATH,
            ],
            input_count: inputs.len(),
            present_inputs,
            reads_runtime_artifact_contents: false,
        },
        inputs,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-restart-brief --project . --write".to_string(),
            "dx forge launch-evidence-restart-manifest --project . --write".to_string(),
            "dx forge launch-evidence-restart-receipt --project . --write".to_string(),
            format!("open {MANIFEST_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_restart_manifest_terminal(
    report: &LaunchEvidenceRestartManifestReport,
) -> String {
    format!(
        "DX Forge launch evidence restart manifest\nProject: {}\nPassed: {}\nScore: {}\nManifest target: {}\nInputs present: {}/{}\nManifest fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.manifest.manifest_target,
        report.manifest.present_inputs,
        report.manifest.input_count,
        report.freshness.manifest_not_older_than_restart_brief,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_restart_manifest_markdown(
    report: &LaunchEvidenceRestartManifestReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Restart Manifest\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Manifest target: `{}`\n- Restart brief fresh: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.manifest.manifest_target,
        report.freshness.manifest_not_older_than_restart_brief,
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

pub(crate) fn launch_evidence_restart_manifest_failure_summary(
    report: &LaunchEvidenceRestartManifestReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence restart manifest score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn restart_manifest_inputs(project: &Path) -> Vec<LaunchEvidenceRestartManifestInput> {
    [
        (
            "restart-brief",
            RESTART_BRIEF_PATH,
            "dx forge launch-evidence-restart-brief --project . --write",
        ),
        (
            "restart-checklist",
            RESTART_CHECKLIST_PATH,
            "dx forge launch-evidence-restart-checklist --project . --write",
        ),
        (
            "restart-ledger",
            RESTART_LEDGER_PATH,
            "dx forge launch-evidence-restart-ledger --project . --write",
        ),
        (
            "operator-resume-card",
            OPERATOR_RESUME_CARD_PATH,
            "dx forge launch-evidence-operator-resume-card --project . --write",
        ),
    ]
    .into_iter()
    .map(|(id, path, command)| restart_manifest_input(project, id, path, command))
    .collect()
}

fn restart_manifest_input(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceRestartManifestInput {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceRestartManifestInput {
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

fn restart_manifest_freshness(
    project: &Path,
    inputs: &[LaunchEvidenceRestartManifestInput],
) -> LaunchEvidenceRestartManifestFreshness {
    let manifest_modified =
        file_metadata(&project.join(MANIFEST_PATH)).map(|metadata| metadata.modified_at);
    let restart_brief_modified =
        file_metadata(&project.join(RESTART_BRIEF_PATH)).map(|metadata| metadata.modified_at);
    let manifest_not_older_than_restart_brief = match (manifest_modified, restart_brief_modified) {
        (Some(manifest), Some(brief)) => manifest >= brief,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceRestartManifestFreshness {
        manifest_path: MANIFEST_PATH,
        manifest_present: manifest_modified.is_some(),
        manifest_modified_at: manifest_modified.map(format_system_time),
        restart_brief_present: input_present(inputs, "restart-brief"),
        manifest_not_older_than_restart_brief,
        timestamp_source: "filesystem-metadata",
    }
}

fn input_present(inputs: &[LaunchEvidenceRestartManifestInput], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-restart-manifest".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceRestartManifestCheck {
    LaunchEvidenceRestartManifestCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceRestartManifestCheck]) -> u8 {
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
    fn fails_when_restart_brief_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            RESTART_CHECKLIST_PATH,
            RESTART_LEDGER_PATH,
            OPERATOR_RESUME_CARD_PATH,
        ] {
            write_input(dir.path(), path);
        }

        let report =
            build_launch_evidence_restart_manifest_report(dir.path(), 100).expect("manifest");

        assert!(!report.passed());
        assert!(!report.freshness.restart_brief_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "restart-brief-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_restart_manifest_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), MANIFEST_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), RESTART_BRIEF_PATH);

        let report =
            build_launch_evidence_restart_manifest_report(dir.path(), 0).expect("manifest");

        assert!(!report.freshness.manifest_not_older_than_restart_brief);
    }

    #[test]
    fn passes_complete_fresh_manifest_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_restart_manifest_inputs(dir.path());
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), MANIFEST_PATH);

        let report =
            build_launch_evidence_restart_manifest_report(dir.path(), 100).expect("manifest");

        assert!(report.passed());
        assert_eq!(
            report.manifest.manifest_target,
            "dx-cli-zed-indexable-restart-manifest"
        );
        assert_eq!(report.manifest.indexable_paths.len(), 4);
        assert!(!report.manifest.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_restart_manifest() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_restart_manifest_inputs(dir.path());

        run_launch_evidence_restart_manifest(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write manifest");

        let report =
            build_launch_evidence_restart_manifest_report(dir.path(), 100).expect("manifest");
        assert!(report.passed());
        assert!(dir.path().join(MANIFEST_PATH).is_file());
    }

    fn write_restart_manifest_inputs(project: &Path) {
        for path in [
            RESTART_BRIEF_PATH,
            RESTART_CHECKLIST_PATH,
            RESTART_LEDGER_PATH,
            OPERATOR_RESUME_CARD_PATH,
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
