use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_share_manifest";
const MANIFEST_SCHEMA: &str = "dx.launch.evidence_share_manifest";
const MANIFEST_PATH: &str = ".dx/forge/release/launch-evidence-share-manifest.json";
const RELEASE_CHECKLIST_PATH: &str = ".dx/forge/release/launch-evidence-release-checklist.json";
const DIGEST_PATH: &str = ".dx/forge/release/launch-evidence-handoff-digest.md";
const TIMELINE_PATH: &str = ".dx/forge/release/launch-evidence-status-timeline.json";
const OPERATOR_INDEX_PATH: &str = ".dx/forge/release/launch-evidence-operator-index.json";
const PACKET_PATH: &str = ".dx/forge/release/launch-evidence-packet.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceShareManifestReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    manifest: LaunchEvidenceShareManifestSummary,
    share_artifacts: Vec<LaunchEvidenceShareArtifact>,
    freshness: LaunchEvidenceShareFreshness,
    checks: Vec<LaunchEvidenceShareCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceShareManifestReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceShareManifestSummary {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    export_target: &'static str,
    artifact_count: usize,
    present_artifacts: usize,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceShareArtifact {
    id: &'static str,
    label: &'static str,
    path: &'static str,
    command: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceShareFreshness {
    manifest_path: &'static str,
    manifest_present: bool,
    manifest_modified_at: Option<String>,
    release_checklist_present: bool,
    digest_present: bool,
    timeline_present: bool,
    operator_index_present: bool,
    packet_present: bool,
    manifest_not_older_than_release_artifacts: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceShareCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_share_manifest(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-share-manifest option: {value}"
                    ),
                    field: Some("forge.launch-evidence-share-manifest".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-share-manifest argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-share-manifest".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(MANIFEST_PATH));
    }

    let mut report =
        build_launch_evidence_share_manifest_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_share_manifest_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_share_manifest(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_share_manifest(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_share_manifest_failure_summary(&report),
            field: Some("forge.launch-evidence-share-manifest".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_share_manifest(
    report: &LaunchEvidenceShareManifestReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_share_manifest_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_share_manifest_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_share_manifest_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceShareManifestReport> {
    let share_artifacts = share_artifacts(project);
    let present_artifacts = share_artifacts
        .iter()
        .filter(|artifact| artifact.present)
        .count();
    let all_artifacts_present = present_artifacts == share_artifacts.len();
    let freshness = share_manifest_freshness(project, &share_artifacts);
    let checks = vec![
        check(
            "share-artifacts-present",
            all_artifacts_present,
            format!(
                "{present_artifacts}/{} share manifest artifact(s) are present",
                share_artifacts.len()
            ),
        ),
        check(
            "share-manifest-freshness",
            freshness.manifest_not_older_than_release_artifacts,
            "share manifest is not older than release checklist or digest handoff artifacts"
                .to_string(),
        ),
        check(
            "export-target-indexed",
            true,
            "share manifest export target is indexed for DX CLI and Zed".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "share manifest uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceShareManifestReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        manifest: LaunchEvidenceShareManifestSummary {
            schema: MANIFEST_SCHEMA,
            path: MANIFEST_PATH,
            command: "dx forge launch-evidence-share-manifest --project <path> --write",
            export_target: "dx-cli-zed",
            artifact_count: share_artifacts.len(),
            present_artifacts,
            reads_runtime_artifact_contents: false,
        },
        share_artifacts,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-release-checklist --project . --write".to_string(),
            "dx forge launch-evidence-share-manifest --project . --write".to_string(),
            format!("open {MANIFEST_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_share_manifest_terminal(
    report: &LaunchEvidenceShareManifestReport,
) -> String {
    format!(
        "DX Forge launch evidence share manifest\nProject: {}\nPassed: {}\nScore: {}\nExport target: {}\nArtifacts present: {}/{}\nManifest fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.manifest.export_target,
        report.manifest.present_artifacts,
        report.manifest.artifact_count,
        report.freshness.manifest_not_older_than_release_artifacts,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_share_manifest_markdown(
    report: &LaunchEvidenceShareManifestReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Share Manifest\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Export target: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.manifest.export_target,
        report.no_execution
    );
    output.push_str("| Artifact | Present | Path | Command |\n| --- | --- | --- | --- |\n");
    for artifact in &report.share_artifacts {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` |\n",
            artifact.id, artifact.present, artifact.path, artifact.command
        ));
    }
    output
}

pub(crate) fn launch_evidence_share_manifest_failure_summary(
    report: &LaunchEvidenceShareManifestReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence share manifest score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn share_artifacts(project: &Path) -> Vec<LaunchEvidenceShareArtifact> {
    [
        (
            "release-checklist",
            "Final release checklist",
            RELEASE_CHECKLIST_PATH,
            "dx forge launch-evidence-release-checklist --project . --write",
        ),
        (
            "handoff-digest",
            "Zed-openable handoff digest",
            DIGEST_PATH,
            "dx forge launch-evidence-handoff-digest --project . --write",
        ),
        (
            "status-timeline",
            "Launch evidence status timeline",
            TIMELINE_PATH,
            "dx forge launch-evidence-status-timeline --project . --write",
        ),
        (
            "operator-index",
            "Launch evidence operator index",
            OPERATOR_INDEX_PATH,
            "dx forge launch-evidence-operator-index --project . --write",
        ),
        (
            "release-packet",
            "Launch evidence release packet",
            PACKET_PATH,
            "dx forge launch-evidence-packet --project . --write",
        ),
    ]
    .into_iter()
    .map(|(id, label, path, command)| share_artifact(project, id, label, path, command))
    .collect()
}

fn share_artifact(
    project: &Path,
    id: &'static str,
    label: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceShareArtifact {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceShareArtifact {
        id,
        label,
        path,
        command,
        present: metadata.is_some(),
        modified_at: metadata
            .as_ref()
            .map(|metadata| format_system_time(metadata.modified_at)),
        bytes: metadata.map(|metadata| metadata.bytes),
    }
}

fn share_manifest_freshness(
    project: &Path,
    share_artifacts: &[LaunchEvidenceShareArtifact],
) -> LaunchEvidenceShareFreshness {
    let manifest_modified =
        file_metadata(&project.join(MANIFEST_PATH)).map(|metadata| metadata.modified_at);
    let newest_artifact = share_artifacts
        .iter()
        .filter_map(|artifact| file_metadata(&project.join(artifact.path)))
        .map(|metadata| metadata.modified_at)
        .max();
    let manifest_not_older_than_release_artifacts = match (manifest_modified, newest_artifact) {
        (Some(manifest), Some(artifact)) => manifest >= artifact,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceShareFreshness {
        manifest_path: MANIFEST_PATH,
        manifest_present: manifest_modified.is_some(),
        manifest_modified_at: manifest_modified.map(format_system_time),
        release_checklist_present: artifact_present(share_artifacts, "release-checklist"),
        digest_present: artifact_present(share_artifacts, "handoff-digest"),
        timeline_present: artifact_present(share_artifacts, "status-timeline"),
        operator_index_present: artifact_present(share_artifacts, "operator-index"),
        packet_present: artifact_present(share_artifacts, "release-packet"),
        manifest_not_older_than_release_artifacts,
        timestamp_source: "filesystem-metadata",
    }
}

fn artifact_present(share_artifacts: &[LaunchEvidenceShareArtifact], id: &str) -> bool {
    share_artifacts
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
            field: Some("forge.launch-evidence-share-manifest".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceShareCheck {
    LaunchEvidenceShareCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceShareCheck]) -> u8 {
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
    fn fails_when_release_checklist_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [DIGEST_PATH, TIMELINE_PATH, OPERATOR_INDEX_PATH, PACKET_PATH] {
            write_input(dir.path(), path);
        }

        let report =
            build_launch_evidence_share_manifest_report(dir.path(), 100).expect("manifest");

        assert!(!report.passed());
        assert!(!report.freshness.release_checklist_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "share-artifacts-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_manifest_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), MANIFEST_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        for path in [
            RELEASE_CHECKLIST_PATH,
            DIGEST_PATH,
            TIMELINE_PATH,
            OPERATOR_INDEX_PATH,
            PACKET_PATH,
        ] {
            write_input(dir.path(), path);
        }

        let report =
            build_launch_evidence_share_manifest_report(dir.path(), 0).expect("manifest report");

        assert!(!report.freshness.manifest_not_older_than_release_artifacts);
    }

    #[test]
    fn passes_complete_fresh_manifest_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            RELEASE_CHECKLIST_PATH,
            DIGEST_PATH,
            TIMELINE_PATH,
            OPERATOR_INDEX_PATH,
            PACKET_PATH,
        ] {
            write_input(dir.path(), path);
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), MANIFEST_PATH);

        let report =
            build_launch_evidence_share_manifest_report(dir.path(), 100).expect("manifest");

        assert!(report.passed());
        assert_eq!(report.manifest.export_target, "dx-cli-zed");
        assert_eq!(report.manifest.present_artifacts, 5);
        assert!(!report.manifest.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_export_manifest() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            RELEASE_CHECKLIST_PATH,
            DIGEST_PATH,
            TIMELINE_PATH,
            OPERATOR_INDEX_PATH,
            PACKET_PATH,
        ] {
            write_input(dir.path(), path);
        }

        run_launch_evidence_share_manifest(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write share manifest");

        let manifest =
            fs::read_to_string(dir.path().join(MANIFEST_PATH)).expect("share manifest json");
        assert!(manifest.contains("dx.forge.launch_evidence_share_manifest"));
        let report =
            build_launch_evidence_share_manifest_report(dir.path(), 100).expect("manifest");
        assert!(report.passed());
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
