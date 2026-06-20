use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::Serialize;
use serde_json::Value;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, launch_runtime_evidence_review,
    parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_evidence_packet";
const PACKET_SCHEMA: &str = "dx.launch.evidence_packet";
const PACKET_PATH: &str = ".dx/forge/release/launch-evidence-packet.json";
const TEMPLATE_MANIFEST_PATH: &str = ".dx/forge/template-.dx/build-cache/manifest.json";
const SOURCE_MANIFEST_PATH: &str = ".dx/forge/source-.dx/build-cache/manifest.json";
const READINESS_BUNDLE_PATH: &str = ".dx/forge/template-readiness/launch-readiness-bundle.json";
const RUNTIME_EVIDENCE_PATH: &str = ".dx/forge/template-readiness/launch-runtime-evidence.json";
const FINAL_RECEIPT_PATH: &str = ".dx/forge/runtime/final-launch-evidence-receipt.json";
const FINAL_REVIEW_PATH: &str = ".dx/forge/runtime/final-launch-evidence-review.json";
const RECEIPT_DIR: &str = ".dx/forge/receipts";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchEvidencePacketReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    packet: LaunchEvidencePacketSummary,
    packet_integrity: LaunchEvidencePacketIntegrity,
    final_evidence_review: LaunchEvidencePacketReview,
    contracts: LaunchEvidencePacketContracts,
    source_receipts: LaunchEvidencePacketSourceReceipts,
    checks: Vec<LaunchEvidencePacketCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchEvidencePacketReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct LaunchEvidencePacketSummary {
    schema: &'static str,
    path: &'static str,
    command: &'static str,
    hash_algorithm: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchEvidencePacketIntegrity {
    passed: bool,
    hash_algorithm: &'static str,
    included_contracts: usize,
    hashed_contracts: usize,
    runtime_artifacts_copied: bool,
}

#[derive(Debug, Serialize)]
struct LaunchEvidencePacketReview {
    review_report: PacketReviewSummary,
    current: PacketReviewSummary,
    fresh: bool,
}

#[derive(Debug, Serialize)]
struct PacketReviewSummary {
    path: &'static str,
    present: bool,
    schema: Option<String>,
    passed: bool,
    score: u8,
    source_hash: Option<String>,
}

#[derive(Debug, Serialize)]
struct LaunchEvidencePacketContracts {
    total: usize,
    present: usize,
    hashed: usize,
    hash_algorithm: &'static str,
    items: Vec<LaunchEvidencePacketContract>,
}

#[derive(Debug, Serialize)]
struct LaunchEvidencePacketContract {
    id: &'static str,
    path: &'static str,
    present: bool,
    bytes: Option<u64>,
    hash_algorithm: Option<&'static str>,
    source_hash: Option<String>,
    schema: Option<String>,
    passed: Option<bool>,
}

#[derive(Debug, Serialize)]
struct LaunchEvidencePacketSourceReceipts {
    source_manifest_path: &'static str,
    source_manifest_present: bool,
    package_count: usize,
    receipt_dir: &'static str,
    receipt_count: usize,
}

#[derive(Debug, Serialize)]
struct LaunchEvidencePacketCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_evidence_packet(cwd: &Path, args: &[String]) -> DxResult<()> {
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
                    message: format!("Unknown forge launch-evidence-packet option: {value}"),
                    field: Some("forge.launch-evidence-packet".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!("Unexpected forge launch-evidence-packet argument: {value}"),
                    field: Some("forge.launch-evidence-packet".to_string()),
                });
            }
        }
    }

    let report = build_launch_evidence_packet_report(&project, fail_under).map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => launch_evidence_packet_markdown(&report),
        DxOutputFormat::Terminal => launch_evidence_packet_terminal(&report),
    };

    if write && output.is_none() {
        output = Some(project.join(PACKET_PATH));
    }

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
            message: launch_evidence_packet_failure_summary(&report),
            field: Some("forge.launch-evidence-packet".to_string()),
        });
    }

    Ok(())
}

pub(crate) fn build_launch_evidence_packet_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchEvidencePacketReport> {
    let review_report = packet_review_report(project);
    let current_review_report =
        launch_runtime_evidence_review::build_launch_runtime_evidence_review_report(project, 100)?;
    let current_review = current_review_summary(&current_review_report);
    let final_evidence_review = LaunchEvidencePacketReview {
        fresh: review_report.passed && current_review.passed,
        review_report,
        current: current_review,
    };
    let contracts = packet_integrity(project);
    let packet_integrity = LaunchEvidencePacketIntegrity {
        passed: contracts.total > 0
            && contracts.present == contracts.total
            && contracts.hashed == contracts.total,
        hash_algorithm: "blake3",
        included_contracts: contracts.total,
        hashed_contracts: contracts.hashed,
        runtime_artifacts_copied: false,
    };
    let source_receipts = source_receipts(project);

    let checks = vec![
        check(
            "final-evidence-review-report",
            final_evidence_review.review_report.present
                && final_evidence_review.review_report.schema.as_deref()
                    == Some("dx.forge.launch_runtime_evidence_review")
                && final_evidence_review.review_report.passed,
            "written final runtime evidence review report is present and passing".to_string(),
        ),
        check(
            "fresh-final-evidence-review",
            final_evidence_review.fresh,
            "current final-runtime-review still matches the written final evidence review"
                .to_string(),
        ),
        check(
            "contracts-present",
            contracts.total > 0 && contracts.present == contracts.total,
            format!(
                "{}/{} launch evidence contract input(s) are present",
                contracts.present, contracts.total
            ),
        ),
        check(
            "packet-integrity",
            packet_integrity.passed,
            format!(
                "{}/{} launch evidence contract input(s) have BLAKE3 hashes",
                packet_integrity.hashed_contracts, packet_integrity.included_contracts
            ),
        ),
        check(
            "source-receipts",
            source_receipts.source_manifest_present && source_receipts.receipt_count > 0,
            format!(
                "{} source package(s), {} Forge receipt file(s)",
                source_receipts.package_count, source_receipts.receipt_count
            ),
        ),
        check(
            "no-execution",
            true,
            "launch evidence packet reads existing contracts and hashes only; it does not run builds, previews, servers, copies, or installs".to_string(),
        ),
    ];
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .collect::<Vec<_>>();
    let score = average_score(&checks);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(LaunchEvidencePacketReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        packet: LaunchEvidencePacketSummary {
            schema: PACKET_SCHEMA,
            path: PACKET_PATH,
            command: "dx forge launch-evidence-packet --project <path> --json",
            hash_algorithm: "blake3",
        },
        packet_integrity,
        final_evidence_review,
        contracts,
        source_receipts,
        checks,
        findings,
        next_commands: vec![
            format!("dx forge launch-evidence-packet --project . --json --output {PACKET_PATH}"),
            "dx forge launch-evidence-operator-index --project . --write".to_string(),
            "dx forge launch-evidence-status-timeline --project . --write".to_string(),
            format!("open {PACKET_PATH}"),
        ],
    })
}

pub(crate) fn launch_evidence_packet_terminal(report: &LaunchEvidencePacketReport) -> String {
    format!(
        "DX Forge launch evidence packet\nProject: {}\nPassed: {}\nScore: {}\nContracts hashed: {}/{}\nFinal review fresh: {}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report.contracts.hashed,
        report.contracts.total,
        report.final_evidence_review.fresh,
        report.no_execution
    )
}

pub(crate) fn launch_evidence_packet_markdown(report: &LaunchEvidencePacketReport) -> String {
    let mut output = format!(
        "# DX Forge Launch Evidence Packet\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Packet: `{}`\n- Contracts hashed: `{}/{}`\n- Final review fresh: `{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.packet.path,
        report.contracts.hashed,
        report.contracts.total,
        report.final_evidence_review.fresh,
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

pub(crate) fn launch_evidence_packet_failure_summary(
    report: &LaunchEvidencePacketReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch evidence packet score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn packet_review_report(project: &Path) -> PacketReviewSummary {
    let path = project.join(FINAL_REVIEW_PATH);
    let value = read_json_file(&path);
    let metadata = source_metadata(&path);
    PacketReviewSummary {
        path: FINAL_REVIEW_PATH,
        present: value.is_some(),
        schema: value
            .as_ref()
            .and_then(|value| value["schema"].as_str())
            .map(str::to_string),
        passed: value
            .as_ref()
            .and_then(|value| value["passed"].as_bool())
            .unwrap_or(false),
        score: value
            .as_ref()
            .and_then(|value| value["score"].as_u64())
            .map(|score| score as u8)
            .unwrap_or_default(),
        source_hash: metadata.map(|metadata| metadata.hash),
    }
}

fn current_review_summary(
    report: &launch_runtime_evidence_review::LaunchRuntimeEvidenceReviewReport,
) -> PacketReviewSummary {
    let value = serde_json::to_value(report).unwrap_or(Value::Null);
    PacketReviewSummary {
        path: FINAL_REVIEW_PATH,
        present: true,
        schema: value["schema"].as_str().map(str::to_string),
        passed: report.passed(),
        score: value["score"]
            .as_u64()
            .map(|score| score as u8)
            .unwrap_or(0),
        source_hash: None,
    }
}

fn packet_integrity(project: &Path) -> LaunchEvidencePacketContracts {
    let items = [
        ("template-manifest", TEMPLATE_MANIFEST_PATH),
        ("source-manifest", SOURCE_MANIFEST_PATH),
        ("launch-readiness-bundle", READINESS_BUNDLE_PATH),
        ("runtime-evidence", RUNTIME_EVIDENCE_PATH),
        ("final-evidence-receipt", FINAL_RECEIPT_PATH),
        ("final-evidence-review", FINAL_REVIEW_PATH),
    ]
    .into_iter()
    .map(|(id, path)| packet_contract(project, id, path))
    .collect::<Vec<_>>();
    let present = items.iter().filter(|item| item.present).count();
    let hashed = items
        .iter()
        .filter(|item| item.source_hash.is_some())
        .count();
    LaunchEvidencePacketContracts {
        total: items.len(),
        present,
        hashed,
        hash_algorithm: "blake3",
        items,
    }
}

fn packet_contract(
    project: &Path,
    id: &'static str,
    path: &'static str,
) -> LaunchEvidencePacketContract {
    let path_buf = project.join(path);
    let metadata = source_metadata(&path_buf);
    let json = read_json_file(&path_buf);
    LaunchEvidencePacketContract {
        id,
        path,
        present: metadata.is_some(),
        bytes: metadata.as_ref().map(|metadata| metadata.bytes),
        hash_algorithm: metadata.as_ref().map(|_| "blake3"),
        source_hash: metadata.map(|metadata| metadata.hash),
        schema: json
            .as_ref()
            .and_then(|value| value["schema"].as_str())
            .map(str::to_string),
        passed: json.as_ref().and_then(|value| value["passed"].as_bool()),
    }
}

fn source_receipts(project: &Path) -> LaunchEvidencePacketSourceReceipts {
    let source_manifest = read_json_file(&project.join(SOURCE_MANIFEST_PATH));
    LaunchEvidencePacketSourceReceipts {
        source_manifest_path: SOURCE_MANIFEST_PATH,
        source_manifest_present: source_manifest.is_some(),
        package_count: source_manifest
            .as_ref()
            .and_then(|value| value["packages"].as_array())
            .map(Vec::len)
            .unwrap_or_default(),
        receipt_dir: RECEIPT_DIR,
        receipt_count: count_regular_files(&project.join(RECEIPT_DIR)),
    }
}

struct SourceMetadata {
    bytes: u64,
    hash: String,
}

fn source_metadata(path: &Path) -> Option<SourceMetadata> {
    let bytes = fs::read(path).ok()?;
    Some(SourceMetadata {
        bytes: bytes.len() as u64,
        hash: format!("blake3:{}", blake3::hash(&bytes).to_hex()),
    })
}

fn count_regular_files(path: &Path) -> usize {
    fs::read_dir(path)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .count()
}

fn read_json_file(path: &Path) -> Option<Value> {
    let bytes = fs::read(path).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn required_arg<'a>(args: &'a [String], index: usize, name: &str) -> DxResult<&'a str> {
    args.get(index + 1)
        .map(String::as_str)
        .ok_or_else(|| DxError::ConfigValidationError {
            message: format!("{name} requires a value"),
            field: Some("forge.launch-evidence-packet".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> LaunchEvidencePacketCheck {
    LaunchEvidencePacketCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[LaunchEvidencePacketCheck]) -> u8 {
    if checks.is_empty() {
        return 0;
    }
    let total = checks.iter().map(|check| check.score as u16).sum::<u16>();
    (total / checks.len() as u16) as u8
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}
