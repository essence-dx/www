use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_retention_policy";
const POLICY_SCHEMA: &str = "dx.launch.evidence_retention_policy";
const POLICY_PATH: &str = ".dx/forge/release/launch-evidence-retention-policy.json";
const LEDGER_PATH: &str = ".dx/forge/release/launch-evidence-archive-ledger.json";
const RECEIPT_PATH: &str = ".dx/forge/release/launch-evidence-archive-receipt.json";
const ARCHIVE_INDEX_PATH: &str = ".dx/forge/release/launch-evidence-archive-index.json";
const SHARE_MANIFEST_PATH: &str = ".dx/forge/release/launch-evidence-share-.dx/build-cache/manifest.json";
const CHECKLIST_PATH: &str = ".dx/forge/release/launch-evidence-release-checklist.json";
const DIGEST_PATH: &str = ".dx/forge/release/launch-evidence-handoff-digest.md";
const PACKET_PATH: &str = ".dx/forge/release/launch-evidence-packet.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceRetentionPolicyReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    policy: LaunchEvidenceRetentionPolicySummary,
    retention_actions: Vec<LaunchEvidenceRetentionAction>,
    freshness: LaunchEvidenceRetentionFreshness,
    checks: Vec<LaunchEvidenceRetentionCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceRetentionPolicyReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRetentionPolicySummary {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    action_count: usize,
    present_artifacts: usize,
    policy_target: &'static str,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRetentionAction {
    id: &'static str,
    source_path: &'static str,
    retention_action: &'static str,
    reason: &'static str,
    present: bool,
    modified_at: Option<String>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRetentionFreshness {
    policy_path: &'static str,
    policy_present: bool,
    policy_modified_at: Option<String>,
    archive_ledger_present: bool,
    archive_ledger_modified_at: Option<String>,
    policy_not_older_than_archive_ledger: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceRetentionCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_retention_policy(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                        "Unknown forge launch-evidence-retention-policy option: {value}"
                    ),
                    field: Some("forge.launch-evidence-retention-policy".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-retention-policy argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-retention-policy".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(POLICY_PATH));
    }

    let mut report =
        build_launch_evidence_retention_policy_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "{}").map_err(forge_error)?;
            report = build_launch_evidence_retention_policy_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_retention_policy(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_retention_policy(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_retention_policy_failure_summary(&report),
            field: Some("forge.launch-evidence-retention-policy".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_retention_policy(
    report: &LaunchEvidenceRetentionPolicyReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_retention_policy_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_retention_policy_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_retention_policy_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceRetentionPolicyReport> {
    let retention_actions = retention_actions(project);
    let present_artifacts = retention_actions
        .iter()
        .filter(|action| action.present)
        .count();
    let freshness = retention_policy_freshness(project);
    let checks = vec![
        check(
            "archive-ledger-present",
            freshness.archive_ledger_present,
            format!("archive ledger exists at {LEDGER_PATH}"),
        ),
        check(
            "retention-policy-freshness",
            freshness.policy_not_older_than_archive_ledger,
            "retention policy is not older than the archive ledger".to_string(),
        ),
        check(
            "retention-actions-indexed",
            !retention_actions.is_empty(),
            "retention policy lists keep, refresh, and regenerate actions".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "retention policy uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceRetentionPolicyReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        policy: LaunchEvidenceRetentionPolicySummary {
            schema: POLICY_SCHEMA,
            path: POLICY_PATH,
            command: "dx forge launch-evidence-retention-policy --project <path> --write",
            action_count: retention_actions.len(),
            present_artifacts,
            policy_target: "release-proof-retention",
            reads_runtime_artifact_contents: false,
        },
        retention_actions,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-archive-ledger --project . --write".to_string(),
            "dx forge launch-evidence-retention-policy --project . --write".to_string(),
            "dx forge launch-evidence-retention-review --project . --write".to_string(),
            format!("open {POLICY_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_retention_policy_terminal(
    report: &LaunchEvidenceRetentionPolicyReport,
) -> String {
    format!(
        "DX Forge launch evidence retention policy\nProject: {}\nPassed: {}\nScore: {}\nActions: {}\nPolicy fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.policy.action_count,
        report.freshness.policy_not_older_than_archive_ledger,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_retention_policy_markdown(
    report: &LaunchEvidenceRetentionPolicyReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Retention Policy\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Policy target: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.policy.policy_target,
        report.no_execution
    );
    output.push_str("| Evidence | Action | Present | Source path |\n| --- | --- | --- | --- |\n");
    for action in &report.retention_actions {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` |\n",
            action.id, action.retention_action, action.present, action.source_path
        ));
    }
    output
}

pub(crate) fn launch_evidence_retention_policy_failure_summary(
    report: &LaunchEvidenceRetentionPolicyReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence retention policy score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn retention_actions(project: &Path) -> Vec<LaunchEvidenceRetentionAction> {
    [
        (
            "archive-ledger",
            LEDGER_PATH,
            "keep",
            "The ledger is the durable release archive inventory.",
            "dx forge launch-evidence-archive-ledger --project . --write",
        ),
        (
            "archive-receipt",
            RECEIPT_PATH,
            "keep",
            "The receipt confirms the archive handoff target and timestamp.",
            "dx forge launch-evidence-archive-receipt --project . --write",
        ),
        (
            "archive-index",
            ARCHIVE_INDEX_PATH,
            "keep",
            "The index records the long-term archive file set.",
            "dx forge launch-evidence-archive-index --project . --write",
        ),
        (
            "share-manifest",
            SHARE_MANIFEST_PATH,
            "refresh",
            "The manifest should be refreshed before a new external export.",
            "dx forge launch-evidence-share-manifest --project . --write",
        ),
        (
            "release-checklist",
            CHECKLIST_PATH,
            "refresh",
            "The checklist should be refreshed before launch signoff is reused.",
            "dx forge launch-evidence-release-checklist --project . --write",
        ),
        (
            "handoff-digest",
            DIGEST_PATH,
            "regenerate",
            "The human-readable digest should be regenerated after evidence changes.",
            "dx forge launch-evidence-handoff-digest --project . --write",
        ),
        (
            "release-packet",
            PACKET_PATH,
            "keep",
            "The packet is the hash-backed release proof bundle.",
            "dx forge launch-evidence-packet --project . --json",
        ),
    ]
    .into_iter()
    .map(|(id, source_path, retention_action, reason, command)| {
        retention_action_entry(project, id, source_path, retention_action, reason, command)
    })
    .collect()
}

fn retention_action_entry(
    project: &Path,
    id: &'static str,
    source_path: &'static str,
    retention_action: &'static str,
    reason: &'static str,
    command: &'static str,
) -> LaunchEvidenceRetentionAction {
    let modified = file_modified_at(&project.join(source_path));
    LaunchEvidenceRetentionAction {
        id,
        source_path,
        retention_action,
        reason,
        present: modified.is_some(),
        modified_at: modified.map(format_system_time),
        command,
    }
}

fn retention_policy_freshness(project: &Path) -> LaunchEvidenceRetentionFreshness {
    let policy_modified = file_modified_at(&project.join(POLICY_PATH));
    let ledger_modified = file_modified_at(&project.join(LEDGER_PATH));
    let policy_not_older_than_archive_ledger = match (policy_modified, ledger_modified) {
        (Some(policy), Some(ledger)) => policy >= ledger,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceRetentionFreshness {
        policy_path: POLICY_PATH,
        policy_present: policy_modified.is_some(),
        policy_modified_at: policy_modified.map(format_system_time),
        archive_ledger_present: ledger_modified.is_some(),
        archive_ledger_modified_at: ledger_modified.map(format_system_time),
        policy_not_older_than_archive_ledger,
        timestamp_source: "filesystem-metadata",
    }
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
            field: Some("forge.launch-evidence-retention-policy".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceRetentionCheck {
    LaunchEvidenceRetentionCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceRetentionCheck]) -> u8 {
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
    fn fails_when_archive_ledger_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");

        let report =
            build_launch_evidence_retention_policy_report(dir.path(), 100).expect("policy");

        assert!(!report.passed());
        assert!(!report.freshness.archive_ledger_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "archive-ledger-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_retention_policy_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), POLICY_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), LEDGER_PATH);

        let report = build_launch_evidence_retention_policy_report(dir.path(), 0).expect("policy");

        assert!(!report.freshness.policy_not_older_than_archive_ledger);
    }

    #[test]
    fn passes_complete_fresh_retention_policy_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), LEDGER_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), POLICY_PATH);

        let report =
            build_launch_evidence_retention_policy_report(dir.path(), 100).expect("policy");

        assert!(report.passed());
        assert_eq!(report.policy.policy_target, "release-proof-retention");
        assert!(
            report
                .retention_actions
                .iter()
                .any(|action| action.id == "handoff-digest"
                    && action.retention_action == "regenerate")
        );
        assert!(!report.policy.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_retention_policy() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), LEDGER_PATH);

        run_launch_evidence_retention_policy(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write retention policy");

        let report =
            build_launch_evidence_retention_policy_report(dir.path(), 100).expect("policy");
        assert!(report.passed());
        assert!(dir.path().join(POLICY_PATH).is_file());
    }

    fn write_input(project: &Path, path: &str) {
        let path = project.join(path);
        fs::create_dir_all(path.parent().expect("parent")).expect("input parent");
        fs::write(path, "{}").expect("input file");
    }
}
