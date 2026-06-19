use std::path::Path;

use chrono::Utc;
use serde::Serialize;
use serde_json::Value;

const READINESS_RECEIPT_PATH: &str = ".dx/forge/template-readiness/launch-route.json";
const READINESS_BUNDLE_PATH: &str = ".dx/forge/template-readiness/launch-readiness-bundle.json";
const TEMPLATE_MANIFEST_PATH: &str = ".dx/forge/template-manifest.json";
const PACKAGE_CATALOG_PATH: &str = "components/template-app/package-catalog.ts";
const SOURCE_MANIFEST_PATH: &str = ".dx/forge/source-manifest.json";
const OPTIONAL_RUNTIME_APPROVAL_ARTIFACTS: [&str; 3] = [
    ".dx/forge/template-readiness/launch-runtime-approval-request.json",
    ".dx/forge/template-readiness/launch-runtime-evidence.json",
    ".dx/forge/template-readiness/launch-verification-lane.json",
];

#[derive(Debug, Serialize)]
pub(crate) struct TemplateReadinessCount {
    pub(crate) present: usize,
    pub(crate) total: usize,
    pub(crate) missing: Vec<String>,
}

#[derive(Debug, Serialize)]
pub(crate) struct TemplateReadinessCheck {
    pub(crate) name: String,
    pub(crate) passed: bool,
    pub(crate) message: String,
    pub(crate) requires_explicit_permission: bool,
}

#[derive(Debug, Serialize)]
pub(crate) struct TemplateReadinessVerificationReport {
    pub(crate) schema: String,
    pub(crate) generated_at: String,
    pub(crate) project: String,
    pub(crate) route: String,
    pub(crate) passed: bool,
    pub(crate) score: u8,
    pub(crate) receipt_path: String,
    pub(crate) runtime_verification: String,
    pub(crate) runtime_verification_requires_explicit_permission: bool,
    pub(crate) materialized_files: TemplateReadinessCount,
    pub(crate) required_packages: TemplateReadinessCount,
    pub(crate) checks: Vec<TemplateReadinessCheck>,
    pub(crate) findings: Vec<String>,
    pub(crate) next_commands: Vec<String>,
}

pub(crate) fn verify_template_readiness(
    project: &Path,
) -> anyhow::Result<TemplateReadinessVerificationReport> {
    let receipt_path = project.join(READINESS_RECEIPT_PATH);
    let receipt = read_json_file(&receipt_path)?;
    let route = receipt
        .get("route")
        .and_then(Value::as_str)
        .unwrap_or("/")
        .to_string();
    let runtime_verification = receipt
        .get("runtime_verification")
        .and_then(Value::as_str)
        .unwrap_or("pending-governed-runtime-pass")
        .to_string();
    let runtime_permission_required = receipt
        .get("runtime_verification_requires_explicit_permission")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    let materialized_files = receipt_string_array(&receipt, "materialized_files");
    let source_readiness_files = source_readiness_materialized_files(&materialized_files);
    let materialized_count = count_existing_files(project, &source_readiness_files);
    let required_packages = receipt_string_array(&receipt, "required_packages");
    let package_catalog =
        std::fs::read_to_string(project.join(PACKAGE_CATALOG_PATH)).unwrap_or_default();
    let missing_packages = required_packages
        .iter()
        .filter(|package_id| !package_catalog.contains(&format!("packageId: \"{package_id}\"")))
        .cloned()
        .collect::<Vec<_>>();
    let required_count = TemplateReadinessCount {
        present: required_packages
            .len()
            .saturating_sub(missing_packages.len()),
        total: required_packages.len(),
        missing: missing_packages,
    };

    let manifest = read_json_file(&project.join(TEMPLATE_MANIFEST_PATH)).unwrap_or(Value::Null);
    let readiness_bundle =
        read_json_file(&project.join(READINESS_BUNDLE_PATH)).unwrap_or(Value::Null);
    let source_manifest =
        read_json_file(&project.join(SOURCE_MANIFEST_PATH)).unwrap_or(Value::Null);
    let docs_file = receipt
        .get("docs_file")
        .and_then(Value::as_str)
        .unwrap_or(".dx/forge/docs/dx-www-template-shell--variant-next-familiar.md");
    let package_id = receipt
        .get("package")
        .and_then(|package| package.get("id"))
        .and_then(Value::as_str)
        .unwrap_or("dx-www/template-shell");

    let checks = vec![
        check(
            "readiness-receipt-schema",
            receipt.get("schema").and_then(Value::as_str) == Some("dx.www.template_readiness"),
            "readiness receipt uses the dx.www.template_readiness schema",
            false,
        ),
        check(
            "template-manifest-entrypoint",
            manifest
                .pointer("/www_template_entrypoint/materialized_file")
                .and_then(Value::as_str)
                == Some("app/page.tsx"),
            "template manifest points at the materialized App Router route",
            false,
        ),
        check(
            "materialized-files-present",
            materialized_count.present == materialized_count.total && materialized_count.total > 0,
            "all readiness receipt materialized files exist",
            false,
        ),
        check(
            "required-packages-listed",
            required_count.present == required_count.total && required_count.total > 0,
            "package catalog contains every required launch package",
            false,
        ),
        check(
            "source-owned-forge-receipt",
            source_manifest["packages"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|package| package["package_id"] == package_id),
            "source manifest tracks the generated launch template package",
            false,
        ),
        check(
            "forge-doc-present",
            project.join(docs_file).is_file(),
            "Forge docs for the generated launch template package exist",
            false,
        ),
        check(
            "launch-readiness-bundle",
            readiness_bundle.get("schema").and_then(Value::as_str)
                == Some("dx.launch.readiness_bundle")
                && readiness_bundle
                    .pointer("/runtime_gate/requires_explicit_permission")
                    .and_then(Value::as_bool)
                    == Some(true),
            "launch readiness bundle aggregates source checks, package receipts, Zed handoff, and runtime gate",
            false,
        ),
        check(
            "node-modules-absent",
            !project.join("node_modules").exists(),
            "generated starter does not create node_modules",
            false,
        ),
        check(
            "runtime-verification-gated",
            runtime_verification == "pending-governed-runtime-pass" && runtime_permission_required,
            "runtime verification remains explicitly permission-gated",
            true,
        ),
    ];

    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .chain(
            materialized_count
                .missing
                .iter()
                .map(|path| format!("missing materialized file `{path}`")),
        )
        .chain(
            required_count
                .missing
                .iter()
                .map(|package| format!("missing launch package `{package}`")),
        )
        .collect::<Vec<_>>();
    let passed_checks = checks.iter().filter(|check| check.passed).count();
    let score = ((passed_checks * 100) / checks.len().max(1)) as u8;
    let passed = findings.is_empty() && checks.iter().all(|check| check.passed);

    Ok(TemplateReadinessVerificationReport {
        schema: "dx.www.template_readiness_verification".to_string(),
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        route,
        passed,
        score,
        receipt_path: READINESS_RECEIPT_PATH.to_string(),
        runtime_verification,
        runtime_verification_requires_explicit_permission: runtime_permission_required,
        materialized_files: materialized_count,
        required_packages: required_count,
        checks,
        findings,
        next_commands: vec![
            "dx templates --json".to_string(),
            "dx check . --project-contract".to_string(),
            "governed runtime verification".to_string(),
        ],
    })
}

pub(crate) fn template_readiness_terminal(report: &TemplateReadinessVerificationReport) -> String {
    let mut output = format!(
        "DX template readiness\nProject: {}\nRoute: {}\nPassed: {}\nScore: {}\nFiles: {}/{}\nPackages: {}/{}\nRuntime: {}\n",
        report.project,
        report.route,
        report.passed,
        report.score,
        report.materialized_files.present,
        report.materialized_files.total,
        report.required_packages.present,
        report.required_packages.total,
        report.runtime_verification
    );
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

pub(crate) fn template_readiness_markdown(report: &TemplateReadinessVerificationReport) -> String {
    let mut output = format!(
        "# DX Template Readiness\n\n- Project: `{}`\n- Route: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Files: `{}/{}`\n- Packages: `{}/{}`\n- Runtime: `{}`\n\n",
        report.project,
        report.route,
        report.passed,
        report.score,
        report.materialized_files.present,
        report.materialized_files.total,
        report.required_packages.present,
        report.required_packages.total,
        report.runtime_verification
    );
    output.push_str("## Checks\n\n| Check | Status | Message |\n| --- | --- | --- |\n");
    for check in &report.checks {
        output.push_str(&format!(
            "| `{}` | `{}` | {} |\n",
            check.name, check.passed, check.message
        ));
    }
    if !report.findings.is_empty() {
        output.push_str("\n## Findings\n\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

fn read_json_file(path: &Path) -> anyhow::Result<Value> {
    let bytes = std::fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
}

fn receipt_string_array(receipt: &Value, key: &str) -> Vec<String> {
    receipt[key]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

fn count_existing_files(project: &Path, files: &[String]) -> TemplateReadinessCount {
    let missing = files
        .iter()
        .filter(|path| !project.join(path).is_file())
        .cloned()
        .collect::<Vec<_>>();
    TemplateReadinessCount {
        present: files.len().saturating_sub(missing.len()),
        total: files.len(),
        missing,
    }
}

fn source_readiness_materialized_files(files: &[String]) -> Vec<String> {
    files
        .iter()
        .filter(|path| !is_optional_runtime_approval_artifact(path))
        .cloned()
        .collect()
}

fn is_optional_runtime_approval_artifact(path: &str) -> bool {
    OPTIONAL_RUNTIME_APPROVAL_ARTIFACTS.contains(&path)
}

fn check(
    name: &str,
    passed: bool,
    message: &str,
    requires_explicit_permission: bool,
) -> TemplateReadinessCheck {
    TemplateReadinessCheck {
        name: name.to_string(),
        passed,
        message: message.to_string(),
        requires_explicit_permission,
    }
}
