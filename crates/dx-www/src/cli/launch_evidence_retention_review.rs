use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_retention_review";
const REVIEW_SCHEMA: &str = "dx.launch.evidence_retention_review";
const REVIEW_PATH: &str = ".dx/forge/release/launch-evidence-retention-review.json";
const POLICY_PATH: &str = ".dx/forge/release/launch-evidence-retention-policy.json";
const LEDGER_PATH: &str = ".dx/forge/release/launch-evidence-archive-ledger.json";
const RECEIPT_PATH: &str = ".dx/forge/release/launch-evidence-archive-receipt.json";
const ARCHIVE_INDEX_PATH: &str = ".dx/forge/release/launch-evidence-archive-index.json";
const SHARE_MANIFEST_PATH: &str = ".dx/forge/release/launch-evidence-share-manifest.json";
const CHECKLIST_PATH: &str = ".dx/forge/release/launch-evidence-release-checklist.json";
const DIGEST_PATH: &str = ".dx/forge/release/launch-evidence-handoff-digest.md";
const PACKET_PATH: &str = ".dx/forge/release/launch-evidence-packet.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceRetentionReviewReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    review: LaunchEvidenceRetentionReviewSummary,
    evidence_chain: Vec<LaunchEvidenceRetentionReviewArtifact>,
    freshness: LaunchEvidenceRetentionReviewFreshness,
    checks: Vec<LaunchEvidenceRetentionReviewCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceRetentionReviewReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRetentionReviewSummary {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    retention_policy: &'static str,
    archive_ledger: &'static str,
    review_target: &'static str,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRetentionReviewArtifact {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRetentionReviewFreshness {
    review_path: &'static str,
    review_present: bool,
    review_modified_at: Option<String>,
    retention_policy_present: bool,
    archive_ledger_present: bool,
    review_not_older_than_retention_policy: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRetentionReviewCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_retention_review(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-retention-review option: {value}"
                    ),
                    field: Some("forge.launch-evidence-retention-review".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-retention-review argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-retention-review".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(REVIEW_PATH));
    }

    let mut report =
        build_launch_evidence_retention_review_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_retention_review_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_retention_review(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_retention_review(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_retention_review_failure_summary(&report),
            field: Some("forge.launch-evidence-retention-review".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_retention_review(
    report: &LaunchEvidenceRetentionReviewReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_retention_review_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_retention_review_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_retention_review_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceRetentionReviewReport> {
    let evidence_chain = evidence_chain(project);
    let freshness = retention_review_freshness(project, &evidence_chain);
    let checks = vec![
        check(
            "retention-policy-present",
            freshness.retention_policy_present,
            format!("retention policy exists at {POLICY_PATH}"),
        ),
        check(
            "archive-ledger-present",
            freshness.archive_ledger_present,
            format!("archive ledger exists at {LEDGER_PATH}"),
        ),
        check(
            "release-proof-chain-present",
            evidence_chain.iter().all(|artifact| artifact.present),
            format!(
                "{}/{} release proof chain artifact(s) are present",
                evidence_chain
                    .iter()
                    .filter(|artifact| artifact.present)
                    .count(),
                evidence_chain.len()
            ),
        ),
        check(
            "retention-review-freshness",
            freshness.review_not_older_than_retention_policy,
            "retention review is not older than the retention policy".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "retention review uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceRetentionReviewReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        review: LaunchEvidenceRetentionReviewSummary {
            schema: REVIEW_SCHEMA,
            path: REVIEW_PATH,
            command: "dx forge launch-evidence-retention-review --project <path> --write",
            retention_policy: POLICY_PATH,
            archive_ledger: LEDGER_PATH,
            review_target: "post-retention-release-proof",
            reads_runtime_artifact_contents: false,
        },
        evidence_chain,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-retention-policy --project . --write".to_string(),
            "dx forge launch-evidence-retention-review --project . --write".to_string(),
            "dx forge launch-evidence-release-seal --project . --write".to_string(),
            format!("open {REVIEW_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_retention_review_terminal(
    report: &LaunchEvidenceRetentionReviewReport,
) -> String {
    format!(
        "DX Forge launch evidence retention review\nProject: {}\nPassed: {}\nScore: {}\nReview target: {}\nReview fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.review.review_target,
        report.freshness.review_not_older_than_retention_policy,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_retention_review_markdown(
    report: &LaunchEvidenceRetentionReviewReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Retention Review\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Review target: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.review.review_target,
        report.no_execution
    );
    output.push_str("| Artifact | Present | Path |\n| --- | --- | --- |\n");
    for artifact in &report.evidence_chain {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` |\n",
            artifact.id, artifact.present, artifact.path
        ));
    }
    output
}

pub(crate) fn launch_evidence_retention_review_failure_summary(
    report: &LaunchEvidenceRetentionReviewReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence retention review score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn evidence_chain(project: &Path) -> Vec<LaunchEvidenceRetentionReviewArtifact> {
    [
        (
            "retention-policy",
            POLICY_PATH,
            "dx forge launch-evidence-retention-policy --project . --write",
        ),
        (
            "archive-ledger",
            LEDGER_PATH,
            "dx forge launch-evidence-archive-ledger --project . --write",
        ),
        (
            "archive-receipt",
            RECEIPT_PATH,
            "dx forge launch-evidence-archive-receipt --project . --write",
        ),
        (
            "archive-index",
            ARCHIVE_INDEX_PATH,
            "dx forge launch-evidence-archive-index --project . --write",
        ),
        (
            "share-manifest",
            SHARE_MANIFEST_PATH,
            "dx forge launch-evidence-share-manifest --project . --write",
        ),
        (
            "release-checklist",
            CHECKLIST_PATH,
            "dx forge launch-evidence-release-checklist --project . --write",
        ),
        (
            "handoff-digest",
            DIGEST_PATH,
            "dx forge launch-evidence-handoff-digest --project . --write",
        ),
        (
            "release-packet",
            PACKET_PATH,
            "dx forge launch-evidence-packet --project . --write",
        ),
    ]
    .into_iter()
    .map(|(id, path, command)| evidence_artifact(project, id, path, command))
    .collect()
}

fn evidence_artifact(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceRetentionReviewArtifact {
    let modified = file_modified_at(&project.join(path));
    LaunchEvidenceRetentionReviewArtifact {
        id,
        path,
        present: modified.is_some(),
        modified_at: modified.map(format_system_time),
        command,
    }
}

fn retention_review_freshness(
    project: &Path,
    evidence_chain: &[LaunchEvidenceRetentionReviewArtifact],
) -> LaunchEvidenceRetentionReviewFreshness {
    let review_modified = file_modified_at(&project.join(REVIEW_PATH));
    let policy_modified = file_modified_at(&project.join(POLICY_PATH));
    let review_not_older_than_retention_policy = match (review_modified, policy_modified) {
        (Some(review), Some(policy)) => review >= policy,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceRetentionReviewFreshness {
        review_path: REVIEW_PATH,
        review_present: review_modified.is_some(),
        review_modified_at: review_modified.map(format_system_time),
        retention_policy_present: artifact_present(evidence_chain, "retention-policy"),
        archive_ledger_present: artifact_present(evidence_chain, "archive-ledger"),
        review_not_older_than_retention_policy,
        timestamp_source: "filesystem-metadata",
    }
}

fn artifact_present(evidence_chain: &[LaunchEvidenceRetentionReviewArtifact], id: &str) -> bool {
    evidence_chain
        .iter()
        .any(|artifact| artifact.id == id && artifact.present)
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
            field: Some("forge.launch-evidence-retention-review".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceRetentionReviewCheck {
    LaunchEvidenceRetentionReviewCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceRetentionReviewCheck]) -> u8 {
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
    fn fails_when_retention_policy_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            LEDGER_PATH,
            RECEIPT_PATH,
            ARCHIVE_INDEX_PATH,
            SHARE_MANIFEST_PATH,
            CHECKLIST_PATH,
            DIGEST_PATH,
            PACKET_PATH,
        ] {
            write_input(dir.path(), path);
        }

        let report =
            build_launch_evidence_retention_review_report(dir.path(), 100).expect("review");

        assert!(!report.passed());
        assert!(!report.freshness.retention_policy_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "retention-policy-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_retention_review_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), REVIEW_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), POLICY_PATH);

        let report = build_launch_evidence_retention_review_report(dir.path(), 0).expect("review");

        assert!(!report.freshness.review_not_older_than_retention_policy);
    }

    #[test]
    fn passes_complete_fresh_retention_review_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            POLICY_PATH,
            LEDGER_PATH,
            RECEIPT_PATH,
            ARCHIVE_INDEX_PATH,
            SHARE_MANIFEST_PATH,
            CHECKLIST_PATH,
            DIGEST_PATH,
            PACKET_PATH,
        ] {
            write_input(dir.path(), path);
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), REVIEW_PATH);

        let report =
            build_launch_evidence_retention_review_report(dir.path(), 100).expect("review");

        assert!(report.passed());
        assert_eq!(report.review.review_target, "post-retention-release-proof");
        assert!(!report.review.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_retention_review() {
        let dir = tempfile::tempdir().expect("tempdir");
        for path in [
            POLICY_PATH,
            LEDGER_PATH,
            RECEIPT_PATH,
            ARCHIVE_INDEX_PATH,
            SHARE_MANIFEST_PATH,
            CHECKLIST_PATH,
            DIGEST_PATH,
            PACKET_PATH,
        ] {
            write_input(dir.path(), path);
        }

        run_launch_evidence_retention_review(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write retention review");

        let report =
            build_launch_evidence_retention_review_report(dir.path(), 100).expect("review");
        assert!(report.passed());
        assert!(dir.path().join(REVIEW_PATH).is_file());
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("input parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
