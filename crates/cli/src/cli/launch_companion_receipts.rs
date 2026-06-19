use std::fs;
use std::path::Path;

use chrono::Utc;
use serde::Serialize;
use serde_json::Value;

const REPORT_SCHEMA: &str = "dx.forge.launch_companion_receipts";
const TEMPLATE_ID: &str = "next-familiar-www-template";
const TEMPLATE_MANIFEST_PATH: &str = ".dx/forge/template-manifest.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchCompanionReceiptsReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    template_id: &'static str,
    route: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    companion_receipts: CompanionReceiptSummary,
    checks: Vec<CompanionReceiptCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchCompanionReceiptsReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct CompanionReceiptSummary {
    present: usize,
    total: usize,
    receipts: Vec<CompanionReceipt>,
}

#[derive(Debug, Serialize)]
struct CompanionReceipt {
    package_id: String,
    role: String,
    kind: String,
    source_file: String,
    materialized_file: String,
    materialized_present: bool,
    docs_file: String,
    docs_present: bool,
    package_docs_file: Option<String>,
    proof_export: String,
    public_api: Vec<String>,
    open_files: Vec<Value>,
    no_execution: bool,
}

#[derive(Debug, Serialize)]
struct CompanionReceiptCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn build_launch_companion_receipts_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchCompanionReceiptsReport> {
    let manifest = read_json_file(&project.join(TEMPLATE_MANIFEST_PATH))?;
    let contract = &manifest["launch_companion_receipts"];
    let receipts = contract["receipts"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|receipt| companion_receipt(project, receipt))
        .collect::<Vec<_>>();
    let present = receipts
        .iter()
        .filter(|receipt| receipt.materialized_present && receipt.docs_present)
        .count();
    let total = receipts.len();
    let companion_receipts = CompanionReceiptSummary {
        present,
        total,
        receipts,
    };
    let checks = vec![
        check(
            "companion-receipts-schema",
            contract["schema"].as_str() == Some("dx.launch.companion_receipts"),
            format!(
                "companion receipt contract schema is {}",
                contract["schema"].as_str().unwrap_or("missing")
            ),
        ),
        check(
            "companion-receipts-present",
            companion_receipts.present == companion_receipts.total && companion_receipts.total > 0,
            format!(
                "{}/{} companion receipt docs and materialized proofs are present",
                companion_receipts.present, companion_receipts.total
            ),
        ),
        check(
            "node-modules-absent",
            !project.join("node_modules").exists(),
            "companion receipt report does not require package installation".to_string(),
        ),
        check(
            "runtime-independent",
            contract["no_execution"].as_bool().unwrap_or(false),
            "companion receipt report stays source-only and no-execution".to_string(),
        ),
    ];
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .collect::<Vec<_>>();
    let score = average_score(&checks);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(LaunchCompanionReceiptsReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        template_id: TEMPLATE_ID,
        route: manifest["www_template_entrypoint"]["route"]
            .as_str()
            .unwrap_or("/")
            .to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        companion_receipts,
        checks,
        findings,
        next_commands: vec![
            "dx forge launch-manifest-drift --project . --json".to_string(),
            "dx forge launch-adoption-report --project . --json".to_string(),
            "dx forge launch-readiness-bundle --project . --json".to_string(),
        ],
    })
}

pub(crate) fn launch_companion_receipts_terminal(report: &LaunchCompanionReceiptsReport) -> String {
    format!(
        "DX Forge launch companion receipts\nProject: {}\nPassed: {}\nScore: {}\nReceipts: {}/{}\n",
        report.project,
        report.passed,
        report.score,
        report.companion_receipts.present,
        report.companion_receipts.total
    )
}

pub(crate) fn launch_companion_receipts_markdown(report: &LaunchCompanionReceiptsReport) -> String {
    let mut output = format!(
        "# DX Forge Launch Companion Receipts\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Receipts: `{}/{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report.companion_receipts.present,
        report.companion_receipts.total
    );
    output.push_str("| Package | Proof | Docs | Present |\n| --- | --- | --- | --- |\n");
    for receipt in &report.companion_receipts.receipts {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` |\n",
            receipt.package_id,
            receipt.materialized_file,
            receipt.docs_file,
            receipt.materialized_present && receipt.docs_present
        ));
    }
    output
}

pub(crate) fn launch_companion_receipts_failure_summary(
    report: &LaunchCompanionReceiptsReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch companion receipts score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

fn companion_receipt(project: &Path, receipt: &Value) -> CompanionReceipt {
    let materialized_file = string_field(receipt, "materialized_file");
    let docs_file = string_field(receipt, "docs_file");
    CompanionReceipt {
        package_id: string_field(receipt, "package_id"),
        role: string_field(receipt, "role"),
        kind: string_field(receipt, "kind"),
        source_file: string_field(receipt, "source_file"),
        materialized_present: project.join(&materialized_file).is_file(),
        docs_present: project.join(&docs_file).is_file(),
        materialized_file,
        docs_file,
        package_docs_file: receipt["package_docs_file"].as_str().map(str::to_string),
        proof_export: string_field(receipt, "proof_export"),
        public_api: string_array(&receipt["public_api"]),
        open_files: receipt["open_files"]
            .as_array()
            .cloned()
            .unwrap_or_default(),
        no_execution: receipt["no_execution"].as_bool().unwrap_or(true),
    }
}

fn read_json_file(path: &Path) -> anyhow::Result<Value> {
    let bytes = fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
}

fn string_field(value: &Value, field: &str) -> String {
    value[field].as_str().unwrap_or("").to_string()
}

fn string_array(value: &Value) -> Vec<String> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

fn check(name: &'static str, passed: bool, message: String) -> CompanionReceiptCheck {
    CompanionReceiptCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[CompanionReceiptCheck]) -> u8 {
    if checks.is_empty() {
        return 0;
    }
    let total = checks.iter().map(|check| check.score as u16).sum::<u16>();
    (total / checks.len() as u16) as u8
}
