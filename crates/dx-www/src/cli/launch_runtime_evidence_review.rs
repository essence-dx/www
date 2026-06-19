use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::Serialize;
use serde_json::Value;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_runtime_evidence_review";
const FINAL_RECEIPT_SCHEMA: &str = "dx.launch.runtime_evidence_finalization_receipt";
const RUNTIME_EVIDENCE_SCHEMA: &str = "dx.launch.runtime_evidence";
const RUNTIME_EVIDENCE_PATH: &str = ".dx/forge/template-readiness/launch-runtime-evidence.json";
const FINAL_RECEIPT_PATH: &str = ".dx/forge/runtime/final-launch-evidence-receipt.json";
const REVIEW_REPORT_PATH: &str = ".dx/forge/runtime/final-launch-evidence-review.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchRuntimeEvidenceReviewReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    report_path: &'static str,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    runtime_evidence: RuntimeEvidenceReviewContract,
    finalization_receipt: RuntimeEvidenceReviewReceipt,
    artifacts: RuntimeEvidenceReviewArtifacts,
    checks: Vec<RuntimeEvidenceReviewCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchRuntimeEvidenceReviewReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct RuntimeEvidenceReviewContract {
    path: &'static str,
    present: bool,
    schema: Option<String>,
    status: String,
    finalized: bool,
    no_execution: bool,
    fake_proof: bool,
    finalization_receipt: String,
}

#[derive(Debug, Serialize)]
struct RuntimeEvidenceReviewReceipt {
    path: &'static str,
    present: bool,
    schema: Option<String>,
    no_execution: bool,
    import_plan: Option<String>,
    runtime_evidence: String,
    runtime_evidence_hash: Option<String>,
    current_hash: Option<String>,
    runtime_evidence_hash_matches: bool,
}

#[derive(Debug, Serialize)]
struct RuntimeEvidenceReviewArtifacts {
    total: usize,
    present: usize,
    hashes_present: usize,
    hashes_matched: usize,
    runtime_evidence_declared: usize,
    hash_algorithm: &'static str,
    items: Vec<RuntimeEvidenceReviewArtifact>,
}

#[derive(Debug, Serialize)]
struct RuntimeEvidenceReviewArtifact {
    id: String,
    kind: String,
    source_path: Option<String>,
    source_exists: bool,
    source_bytes: Option<u64>,
    hash_algorithm: Option<String>,
    source_hash: Option<String>,
    current_hash: Option<String>,
    source_hash_matches: bool,
    runtime_evidence_declared: bool,
    target_path: String,
}

#[derive(Debug, Serialize)]
struct RuntimeEvidenceReviewCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_runtime_evidence_review(cwd: &Path, args: &[String]) -> DxResult<()> {
    let mut project = cwd.to_path_buf();
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 100u8;
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
                        "Unknown forge launch-runtime-evidence-review option: {value}"
                    ),
                    field: Some("forge.launch-runtime-evidence-review".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-runtime-evidence-review argument: {value}"
                    ),
                    field: Some("forge.launch-runtime-evidence-review".to_string()),
                });
            }
        }
    }

    let report =
        build_launch_runtime_evidence_review_report(&project, fail_under).map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => launch_runtime_evidence_review_markdown(&report),
        DxOutputFormat::Terminal => launch_runtime_evidence_review_terminal(&report),
    };

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_runtime_evidence_review_failure_summary(&report),
            field: Some("forge.launch-runtime-evidence-review".to_string()),
        });
    }

    Ok(())
}

pub(crate) fn build_launch_runtime_evidence_review_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchRuntimeEvidenceReviewReport> {
    let runtime_evidence_file = read_json_file(&project.join(RUNTIME_EVIDENCE_PATH));
    let final_receipt_file = read_json_file(&project.join(FINAL_RECEIPT_PATH));

    let runtime_evidence = RuntimeEvidenceReviewContract {
        path: RUNTIME_EVIDENCE_PATH,
        present: runtime_evidence_file.is_some(),
        schema: runtime_evidence_file
            .as_ref()
            .and_then(|evidence| evidence["schema"].as_str())
            .map(str::to_string),
        status: runtime_evidence_file
            .as_ref()
            .and_then(|evidence| evidence["status"].as_str())
            .unwrap_or("")
            .to_string(),
        finalized: runtime_evidence_file
            .as_ref()
            .and_then(|evidence| evidence["finalized"].as_bool())
            .unwrap_or(false),
        no_execution: runtime_evidence_file
            .as_ref()
            .and_then(|evidence| evidence["no_execution"].as_bool())
            .unwrap_or(false),
        fake_proof: runtime_evidence_file
            .as_ref()
            .and_then(|evidence| evidence["fake_proof"].as_bool())
            .unwrap_or(true),
        finalization_receipt: runtime_evidence_file
            .as_ref()
            .and_then(|evidence| evidence["finalization_receipt"].as_str())
            .unwrap_or("")
            .to_string(),
    };

    let finalization_receipt_hash =
        source_metadata(project, FINAL_RECEIPT_PATH).map(|metadata| metadata.hash);
    let runtime_evidence_receipt_hash = runtime_evidence_collected_hash(
        runtime_evidence_file.as_ref(),
        "final-launch-evidence-receipt",
    );
    let finalization_receipt = RuntimeEvidenceReviewReceipt {
        path: FINAL_RECEIPT_PATH,
        present: final_receipt_file.is_some(),
        schema: final_receipt_file
            .as_ref()
            .and_then(|receipt| receipt["schema"].as_str())
            .map(str::to_string),
        no_execution: final_receipt_file
            .as_ref()
            .and_then(|receipt| receipt["no_execution"].as_bool())
            .unwrap_or(false),
        import_plan: final_receipt_file
            .as_ref()
            .and_then(|receipt| receipt["import_plan"].as_str())
            .map(str::to_string),
        runtime_evidence: final_receipt_file
            .as_ref()
            .and_then(|receipt| receipt["runtime_evidence"].as_str())
            .unwrap_or("")
            .to_string(),
        runtime_evidence_hash: runtime_evidence_receipt_hash.clone(),
        current_hash: finalization_receipt_hash.clone(),
        runtime_evidence_hash_matches: runtime_evidence_receipt_hash.is_some()
            && finalization_receipt_hash == runtime_evidence_receipt_hash,
    };

    let items = review_artifacts(
        project,
        final_receipt_file.as_ref(),
        runtime_evidence_file.as_ref(),
    );
    let present = items.iter().filter(|item| item.source_exists).count();
    let hashes_present = items.iter().filter(|item| has_valid_hash(item)).count();
    let hashes_matched = items.iter().filter(|item| item.source_hash_matches).count();
    let runtime_evidence_declared = items
        .iter()
        .filter(|item| item.runtime_evidence_declared)
        .count();
    let artifacts = RuntimeEvidenceReviewArtifacts {
        total: items.len(),
        present,
        hashes_present,
        hashes_matched,
        runtime_evidence_declared,
        hash_algorithm: "blake3",
        items,
    };

    let checks = vec![
        check(
            "runtime-evidence-contract",
            runtime_evidence.present
                && runtime_evidence.schema.as_deref() == Some(RUNTIME_EVIDENCE_SCHEMA),
            "runtime evidence contract is present and readable".to_string(),
        ),
        check(
            "runtime-evidence-finalized",
            runtime_evidence.status == "complete"
                && runtime_evidence.finalized
                && runtime_evidence.finalization_receipt == FINAL_RECEIPT_PATH,
            "runtime evidence contract is finalized and points at the final receipt".to_string(),
        ),
        check(
            "finalization-receipt",
            finalization_receipt.present,
            "final launch evidence receipt is present".to_string(),
        ),
        check(
            "finalization-receipt-schema",
            finalization_receipt.schema.as_deref() == Some(FINAL_RECEIPT_SCHEMA),
            "final launch evidence receipt schema is readable".to_string(),
        ),
        check(
            "finalization-receipt-hashes-match",
            finalization_receipt.runtime_evidence_hash_matches
                && artifacts.total > 0
                && artifacts.hashes_present == artifacts.total
                && artifacts.hashes_matched == artifacts.total,
            format!(
                "{}/{} finalization receipt source hash(es) still match current files and the receipt hash matches finalized runtime evidence",
                artifacts.hashes_matched, artifacts.total
            ),
        ),
        check(
            "runtime-evidence-declares-receipt-artifacts",
            artifacts.total > 0 && artifacts.runtime_evidence_declared == artifacts.total,
            format!(
                "{}/{} receipt artifact(s) are declared in finalized runtime evidence",
                artifacts.runtime_evidence_declared, artifacts.total
            ),
        ),
        check(
            "no-fake-proof",
            !runtime_evidence.fake_proof,
            "final runtime evidence is marked as real operator-collected proof".to_string(),
        ),
        check(
            "no-execution",
            runtime_evidence.no_execution && finalization_receipt.no_execution,
            "review reads receipts and source files without running builds, previews, servers, copies, or installs".to_string(),
        ),
    ];
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .collect::<Vec<_>>();
    let score = average_score(&checks);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(LaunchRuntimeEvidenceReviewReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        report_path: REVIEW_REPORT_PATH,
        passed,
        score,
        fail_under,
        no_execution: true,
        runtime_evidence,
        finalization_receipt,
        artifacts,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-runtime-evidence-finalize --project . --import-plan <path> --write --json".to_string(),
            "dx forge launch-runtime-evidence-review --project . --json".to_string(),
            format!("open {REVIEW_REPORT_PATH}"),
        ],
    })
}

pub(crate) fn launch_runtime_evidence_review_terminal(
    report: &LaunchRuntimeEvidenceReviewReport,
) -> String {
    format!(
        "DX Forge launch runtime evidence review\nProject: {}\nPassed: {}\nScore: {}\nFinal receipt: {}\nArtifacts: {}/{}\nHashes matched: {}/{}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.finalization_receipt.present,
        report.artifacts.present,
        report.artifacts.total,
        report.artifacts.hashes_matched,
        report.artifacts.total,
        report.no_execution
    )
}

pub(crate) fn launch_runtime_evidence_review_markdown(
    report: &LaunchRuntimeEvidenceReviewReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Runtime Evidence Review\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Runtime evidence: `{}`\n- Final receipt: `{}`\n- Artifacts: `{}/{}`\n- Hashes matched: `{}/{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.runtime_evidence.status,
        report.finalization_receipt.present,
        report.artifacts.present,
        report.artifacts.total,
        report.artifacts.hashes_matched,
        report.artifacts.total,
        report.no_execution
    );
    output.push_str("| Check | Passed | Score | Message |\n| --- | --- | --- | --- |\n");
    for check in &report.checks {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} |\n",
            check.name,
            check.passed,
            check.score,
            markdown_cell(&check.message)
        ));
    }
    output
}

pub(crate) fn launch_runtime_evidence_review_failure_summary(
    report: &LaunchRuntimeEvidenceReviewReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch runtime evidence review score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn review_artifacts(
    project: &Path,
    final_receipt: Option<&Value>,
    runtime_evidence: Option<&Value>,
) -> Vec<RuntimeEvidenceReviewArtifact> {
    final_receipt
        .and_then(|receipt| receipt["artifacts"]["items"].as_array())
        .into_iter()
        .flatten()
        .map(|item| inspect_receipt_artifact(project, item, runtime_evidence))
        .collect()
}

fn inspect_receipt_artifact(
    project: &Path,
    item: &Value,
    runtime_evidence: Option<&Value>,
) -> RuntimeEvidenceReviewArtifact {
    let id = string_field(item, "id");
    let kind = string_field(item, "kind");
    let source_path = item["source_path"]
        .as_str()
        .filter(|path| !path.is_empty())
        .map(str::to_string);
    let metadata = source_path
        .as_deref()
        .and_then(|path| source_metadata(project, path));
    let source_hash = item["source_hash"].as_str().map(str::to_string);
    let current_hash = metadata.as_ref().map(|metadata| metadata.hash.clone());
    let source_hash_matches = source_hash
        .as_ref()
        .zip(current_hash.as_ref())
        .is_some_and(|(expected, current)| expected == current);
    let target_path = string_field(item, "target_path");
    let runtime_evidence_declared = runtime_evidence_declares_artifact(
        runtime_evidence,
        &id,
        source_hash.as_deref(),
        &target_path,
    );

    RuntimeEvidenceReviewArtifact {
        id,
        kind,
        source_path,
        source_exists: metadata.is_some(),
        source_bytes: metadata.as_ref().map(|metadata| metadata.bytes),
        hash_algorithm: item["hash_algorithm"].as_str().map(str::to_string),
        source_hash,
        current_hash,
        source_hash_matches,
        runtime_evidence_declared,
        target_path,
    }
}

fn runtime_evidence_declares_artifact(
    runtime_evidence: Option<&Value>,
    id: &str,
    source_hash: Option<&str>,
    target_path: &str,
) -> bool {
    runtime_evidence
        .and_then(|evidence| evidence["collected_evidence"]["items"].as_array())
        .into_iter()
        .flatten()
        .any(|item| {
            item["id"].as_str() == Some(id)
                && item["source_hash"].as_str() == source_hash
                && item["target_path"].as_str() == Some(target_path)
        })
}

fn runtime_evidence_collected_hash(runtime_evidence: Option<&Value>, id: &str) -> Option<String> {
    runtime_evidence?
        .get("collected_evidence")?
        .get("items")?
        .as_array()?
        .iter()
        .find(|item| item["id"].as_str() == Some(id))?
        .get("source_hash")?
        .as_str()
        .map(str::to_string)
}

fn has_valid_hash(item: &RuntimeEvidenceReviewArtifact) -> bool {
    item.source_exists
        && item.hash_algorithm.as_deref() == Some("blake3")
        && item
            .source_hash
            .as_deref()
            .is_some_and(|hash| hash.starts_with("blake3:"))
}

struct RuntimeSourceMetadata {
    bytes: u64,
    hash: String,
}

fn source_metadata(project: &Path, source_path: &str) -> Option<RuntimeSourceMetadata> {
    let path = PathBuf::from(source_path);
    let path = if path.is_absolute() {
        path
    } else {
        project.join(path)
    };
    let bytes = fs::read(path).ok()?;
    Some(RuntimeSourceMetadata {
        bytes: bytes.len() as u64,
        hash: format!("blake3:{}", blake3::hash(&bytes).to_hex()),
    })
}

fn read_json_file(path: &Path) -> Option<Value> {
    let bytes = fs::read(path).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn string_field(value: &Value, field: &str) -> String {
    value[field].as_str().unwrap_or("").to_string()
}

fn required_arg<'a>(args: &'a [String], index: usize, name: &str) -> DxResult<&'a str> {
    args.get(index + 1)
        .map(String::as_str)
        .ok_or_else(|| DxError::ConfigValidationError {
            message: format!("{name} requires a value"),
            field: Some("forge.launch-runtime-evidence-review".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> RuntimeEvidenceReviewCheck {
    RuntimeEvidenceReviewCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[RuntimeEvidenceReviewCheck]) -> u8 {
    if checks.is_empty() {
        return 0;
    }
    let total = checks.iter().map(|check| check.score as u16).sum::<u16>();
    (total / checks.len() as u16) as u8
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}
