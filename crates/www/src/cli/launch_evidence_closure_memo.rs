use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_closure_memo";
const MEMO_SCHEMA: &str = "dx.launch.evidence_closure_memo";
const MEMO_PATH: &str = ".dx/forge/release/launch-evidence-closure-memo.md";
const COMPLETION_LEDGER_PATH: &str = ".dx/forge/release/launch-evidence-completion-ledger.json";
const OPERATOR_SUMMARY_PATH: &str = ".dx/forge/release/launch-evidence-operator-summary.json";
const RELEASE_SEAL_PATH: &str = ".dx/forge/release/launch-evidence-release-seal.json";
const FINAL_REVIEW_PATH: &str = ".dx/forge/runtime/final-launch-evidence-review.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidenceClosureMemoReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    memo: LaunchEvidenceClosureMemo,
    inputs: Vec<LaunchEvidenceClosureMemoInput>,
    freshness: LaunchEvidenceClosureMemoFreshness,
    checks: Vec<LaunchEvidenceClosureMemoCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidenceClosureMemoReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceClosureMemo {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    memo_target: &'static str,
    zed_openable: bool,
    format: &'static str,
    input_count: usize,
    present_inputs: usize,
    reads_runtime_artifact_contents: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceClosureMemoInput {
    id: &'static str,
    path: &'static str,
    present: bool,
    modified_at: Option<String>,
    bytes: Option<u64>,
    command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceClosureMemoFreshness {
    memo_path: &'static str,
    memo_present: bool,
    memo_modified_at: Option<String>,
    completion_ledger_present: bool,
    memo_not_older_than_completion_ledger: bool,
    timestamp_source: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidenceClosureMemoCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_closure_memo(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                    message: format!("Unknown forge launch-evidence-closure-memo option: {value}"),
                    field: Some("forge.launch-evidence-closure-memo".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-evidence-closure-memo argument: {value}"
                    ),
                    field: Some("forge.launch-evidence-closure-memo".to_string()),
                });
            }
        }
    }

    if write && output.is_none() {
        output = Some(project.join(MEMO_PATH));
    }

    let mut report =
        build_launch_evidence_closure_memo_report(&project, fail_under).map_err(forge_error)?;

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        if write {
            fs::write(&output, "").map_err(forge_error)?;
            report = build_launch_evidence_closure_memo_report(&project, fail_under)
                .map_err(forge_error)?;
        }
        let rendered = render_launch_evidence_closure_memo(&report, format)?;
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        let rendered = render_launch_evidence_closure_memo(&report, format)?;
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_evidence_closure_memo_failure_summary(&report),
            field: Some("forge.launch-evidence-closure-memo".to_string()),
        });
    }

    Ok(())
}

fn render_launch_evidence_closure_memo(
    report: &LaunchEvidenceClosureMemoReport,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(report).map_err(forge_error),
        DxOutputFormat::Markdown => Ok(launch_evidence_closure_memo_markdown(report)),
        DxOutputFormat::Terminal => Ok(launch_evidence_closure_memo_terminal(report)),
    }
}

pub(crate) fn build_launch_evidence_closure_memo_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidenceClosureMemoReport> {
    let inputs = closure_memo_inputs(project);
    let present_inputs = inputs.iter().filter(|input| input.present).count();
    let all_inputs_present = present_inputs == inputs.len();
    let freshness = closure_memo_freshness(project, &inputs);
    let checks = vec![
        check(
            "completion-ledger-present",
            freshness.completion_ledger_present,
            format!("completion ledger exists at {COMPLETION_LEDGER_PATH}"),
        ),
        check(
            "closure-memo-inputs-present",
            all_inputs_present,
            format!(
                "{present_inputs}/{} closure memo input(s) are present",
                inputs.len()
            ),
        ),
        check(
            "closure-memo-freshness",
            freshness.memo_not_older_than_completion_ledger,
            "closure memo is not older than the completion ledger".to_string(),
        ),
        check(
            "zed-openable-markdown",
            true,
            "closure memo writes a Zed-openable Markdown closeout".to_string(),
        ),
        check(
            "no-runtime-content-read",
            true,
            "closure memo uses file metadata only; it does not read runtime artifact contents"
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

    Ok(LaunchEvidenceClosureMemoReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        memo: LaunchEvidenceClosureMemo {
            schema: MEMO_SCHEMA,
            path: MEMO_PATH,
            command: "dx forge launch-evidence-closure-memo --project <path> --write",
            memo_target: "human-readable-launch-release-closeout",
            zed_openable: true,
            format: "markdown",
            input_count: inputs.len(),
            present_inputs,
            reads_runtime_artifact_contents: false,
        },
        inputs,
        freshness,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-evidence-completion-ledger --project . --write".to_string(),
            "dx forge launch-evidence-closure-memo --project . --write".to_string(),
            "dx forge launch-evidence-final-brief --project . --write".to_string(),
            format!("open {MEMO_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_closure_memo_terminal(
    report: &LaunchEvidenceClosureMemoReport,
) -> String {
    format!(
        "DX Forge launch evidence closure memo\nProject: {}\nPassed: {}\nScore: {}\nMemo target: {}\nInputs present: {}/{}\nMemo fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.memo.memo_target,
        report.memo.present_inputs,
        report.memo.input_count,
        report.freshness.memo_not_older_than_completion_ledger,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_closure_memo_markdown(
    report: &LaunchEvidenceClosureMemoReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Closure Memo\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Memo target: `{}`\n- Completion ledger fresh: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.memo.memo_target,
        report.freshness.memo_not_older_than_completion_ledger,
        report.no_execution
    );
    output.push_str("## Closeout Artifacts\n\n");
    output.push_str(
        "This memo closes the no-execution launch evidence chain from filesystem metadata only.\n\n",
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

pub(crate) fn launch_evidence_closure_memo_failure_summary(
    report: &LaunchEvidenceClosureMemoReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence closure memo score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn closure_memo_inputs(project: &Path) -> Vec<LaunchEvidenceClosureMemoInput> {
    [
        (
            "completion-ledger",
            COMPLETION_LEDGER_PATH,
            "dx forge launch-evidence-completion-ledger --project . --write",
        ),
        (
            "operator-summary",
            OPERATOR_SUMMARY_PATH,
            "dx forge launch-evidence-operator-summary --project . --write",
        ),
        (
            "release-seal",
            RELEASE_SEAL_PATH,
            "dx forge launch-evidence-release-seal --project . --write",
        ),
        (
            "final-runtime-review",
            FINAL_REVIEW_PATH,
            "dx forge launch-runtime-evidence-review --project . --json",
        ),
    ]
    .into_iter()
    .map(|(id, path, command)| closure_memo_input(project, id, path, command))
    .collect()
}

fn closure_memo_input(
    project: &Path,
    id: &'static str,
    path: &'static str,
    command: &'static str,
) -> LaunchEvidenceClosureMemoInput {
    let metadata = file_metadata(&project.join(path));
    LaunchEvidenceClosureMemoInput {
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

fn closure_memo_freshness(
    project: &Path,
    inputs: &[LaunchEvidenceClosureMemoInput],
) -> LaunchEvidenceClosureMemoFreshness {
    let memo_modified =
        file_metadata(&project.join(MEMO_PATH)).map(|metadata| metadata.modified_at);
    let ledger_modified =
        file_metadata(&project.join(COMPLETION_LEDGER_PATH)).map(|metadata| metadata.modified_at);
    let memo_not_older_than_completion_ledger = match (memo_modified, ledger_modified) {
        (Some(memo), Some(ledger)) => memo >= ledger,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    };

    LaunchEvidenceClosureMemoFreshness {
        memo_path: MEMO_PATH,
        memo_present: memo_modified.is_some(),
        memo_modified_at: memo_modified.map(format_system_time),
        completion_ledger_present: input_present(inputs, "completion-ledger"),
        memo_not_older_than_completion_ledger,
        timestamp_source: "filesystem-metadata",
    }
}

fn input_present(inputs: &[LaunchEvidenceClosureMemoInput], id: &str) -> bool {
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
            field: Some("forge.launch-evidence-closure-memo".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidenceClosureMemoCheck {
    LaunchEvidenceClosureMemoCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidenceClosureMemoCheck]) -> u8 {
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
    fn fails_when_completion_ledger_is_missing() {
        let dir = tempfile::tempdir().expect("tempdir");

        let report =
            build_launch_evidence_closure_memo_report(dir.path(), 100).expect("closure memo");

        assert!(!report.passed());
        assert!(!report.freshness.completion_ledger_present);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.name == "completion-ledger-present" && !check.passed)
        );
    }

    #[test]
    fn reports_stale_closure_memo_from_file_timestamps() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_input(dir.path(), MEMO_PATH);
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), COMPLETION_LEDGER_PATH);

        let report =
            build_launch_evidence_closure_memo_report(dir.path(), 0).expect("closure memo");

        assert!(!report.freshness.memo_not_older_than_completion_ledger);
    }

    #[test]
    fn passes_complete_fresh_closure_memo_without_runtime_content_reads() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_closure_memo_inputs(dir.path());
        std::thread::sleep(std::time::Duration::from_millis(5));
        write_input(dir.path(), MEMO_PATH);

        let report =
            build_launch_evidence_closure_memo_report(dir.path(), 100).expect("closure memo");

        assert!(report.passed());
        assert_eq!(
            report.memo.memo_target,
            "human-readable-launch-release-closeout"
        );
        assert!(report.memo.zed_openable);
        assert!(!report.memo.reads_runtime_artifact_contents);
    }

    #[test]
    fn write_mode_creates_fresh_zed_openable_closure_memo() {
        let dir = tempfile::tempdir().expect("tempdir");
        write_closure_memo_inputs(dir.path());

        run_launch_evidence_closure_memo(
            dir.path(),
            &[
                "--project".to_string(),
                dir.path().to_string_lossy().into_owned(),
                "--write".to_string(),
                "--quiet".to_string(),
            ],
        )
        .expect("write closure memo");

        let memo = fs::read_to_string(dir.path().join(MEMO_PATH)).expect("closure memo markdown");
        assert!(memo.contains("# DX Forge Launch Evidence Closure Memo"));
        let report =
            build_launch_evidence_closure_memo_report(dir.path(), 100).expect("closure memo");
        assert!(report.passed());
    }

    fn write_closure_memo_inputs(project: &Path) {
        for path in [
            COMPLETION_LEDGER_PATH,
            OPERATOR_SUMMARY_PATH,
            RELEASE_SEAL_PATH,
            FINAL_REVIEW_PATH,
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
