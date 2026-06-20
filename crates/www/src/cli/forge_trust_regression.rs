use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use dx_compiler::ecosystem::{
    DxForgeLicenseReviewMetadata, DxSourceManifest, canonical_package_id, write_forge_add_variant,
};
use serde::Serialize;

use super::forge_provenance::build_forge_provenance_report;
use super::{
    FORGE_WWW_TEMPLATE_PACKAGE_IDS, build_forge_package_gallery_report,
    build_forge_verify_package_report, markdown_table_cell, write_forge_adoption_project_scaffold,
};

const FIXTURE_ROOT: &str = ".dx/forge/trust-regression-fixtures";
const BUTTON_PACKAGE_ID: &str = "shadcn/ui/button";

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeTrustRegressionReport {
    pub(super) version: u32,
    pub(super) project: PathBuf,
    pub(super) generated_at: String,
    pub(super) passed: bool,
    pub(super) score: u8,
    pub(super) fail_under: u8,
    pub(super) fixture_root: PathBuf,
    pub(super) no_node_modules: bool,
    pub(super) case_count: usize,
    pub(super) cases: Vec<DxForgeTrustRegressionCase>,
    pub(super) findings: Vec<String>,
    pub(super) next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeTrustRegressionCase {
    pub(super) name: String,
    pub(super) mutation: String,
    pub(super) expected_traffic: String,
    pub(super) observed_traffic: String,
    pub(super) passed: bool,
    pub(super) score: u8,
    pub(super) evidence: Vec<String>,
    pub(super) message: String,
}

pub(super) fn build_forge_trust_regression_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeTrustRegressionReport> {
    fs::create_dir_all(project)?;
    let generated_at = Utc::now().to_rfc3339();
    let fixture_root = unique_fixture_root(project)?;
    fs::create_dir_all(&fixture_root)?;

    let cases = vec![
        green_baseline_case(&fixture_root)?,
        yellow_local_edit_case(&fixture_root)?,
        red_provenance_mismatch_case(&fixture_root)?,
        red_license_mismatch_case(&fixture_root)?,
        red_receipt_missing_case(&fixture_root)?,
        yellow_offline_advisory_case(&fixture_root)?,
    ];

    let no_node_modules =
        !project.join("node_modules").exists() && !tree_has_node_modules(&fixture_root)?;
    let mut findings = cases
        .iter()
        .filter(|case| !case.passed)
        .map(|case| {
            format!(
                "{} expected `{}` but observed `{}`.",
                case.name, case.expected_traffic, case.observed_traffic
            )
        })
        .collect::<Vec<_>>();
    if !no_node_modules {
        findings.push(
            "node_modules exists in the trust-regression project or fixture tree.".to_string(),
        );
    }

    let score = cases
        .iter()
        .map(|case| case.score)
        .chain(std::iter::once(if no_node_modules { 100 } else { 0 }))
        .min()
        .unwrap_or(0);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(DxForgeTrustRegressionReport {
        version: 1,
        project: project.to_path_buf(),
        generated_at,
        passed,
        score,
        fail_under,
        fixture_root,
        no_node_modules,
        case_count: cases.len(),
        cases,
        findings,
        next_commands: vec![
            "dx forge trust-regression --project . --format markdown --output .dx/forge/trust-regression.md --fail-under 100".to_string(),
            "dx forge provenance --project . --format markdown --output .dx/forge/provenance.md".to_string(),
            "dx forge package-gallery --project . --format markdown --output .dx/forge/package-gallery.md".to_string(),
        ],
    })
}

pub(super) fn forge_trust_regression_terminal(report: &DxForgeTrustRegressionReport) -> String {
    let mut output = format!(
        "DX Forge package trust regression\nProject: {}\nGenerated: {}\nPassed: {}\nScore: {} / 100\nRequired score: {} / 100\nFixture root: {}\nCases: {}\nNo node_modules: {}\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.fail_under,
        report.fixture_root.display(),
        report.case_count,
        report.no_node_modules
    );

    output.push_str("\nCases:\n");
    for case in &report.cases {
        output.push_str(&format!(
            "- {}: {} -> {} (passed {}, score {})\n",
            case.name, case.expected_traffic, case.observed_traffic, case.passed, case.score
        ));
    }

    if !report.findings.is_empty() {
        output.push_str("\nFindings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output
}

pub(super) fn forge_trust_regression_markdown(report: &DxForgeTrustRegressionReport) -> String {
    let mut output = format!(
        "# DX Forge Package Trust Regression\n\n- Project: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Fixture root: `{}`\n- Cases: `{}`\n- no `node_modules`: `{}`\n\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.fail_under,
        report.fixture_root.display(),
        report.case_count,
        report.no_node_modules
    );

    output.push_str("## Cases\n\n");
    output.push_str("| Case | Mutation | Expected | Observed | Passed | Evidence |\n");
    output.push_str("| --- | --- | --- | --- | --- | --- |\n");
    for case in &report.cases {
        output.push_str(&format!(
            "| `{}` | {} | `{}` | `{}` | `{}` | {} |\n",
            case.name,
            markdown_table_cell(&case.mutation),
            case.expected_traffic,
            case.observed_traffic,
            case.passed,
            markdown_table_cell(&case.evidence.join("; "))
        ));
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str(
            "- `pass`: every mutated trust input produced the expected traffic signal.\n",
        );
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
    }

    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{command}`\n"));
    }

    output
}

pub(super) fn forge_trust_regression_failure_summary(
    report: &DxForgeTrustRegressionReport,
) -> String {
    if report.findings.is_empty() {
        return format!(
            "DX Forge trust-regression score {} is below fail-under threshold {}",
            report.score, report.fail_under
        );
    }
    format!(
        "DX Forge trust-regression failed: {}",
        report.findings.join("; ")
    )
}

fn green_baseline_case(root: &Path) -> anyhow::Result<DxForgeTrustRegressionCase> {
    let project = scaffold_case_project(root, "green-baseline")?;
    let provenance = serde_json::to_value(build_forge_provenance_report(&project, 90)?)?;
    let verify = serde_json::to_value(build_forge_verify_package_report(
        &project,
        BUTTON_PACKAGE_ID,
        "default",
    )?)?;
    let update_traffic = json_string(&verify, &["update", "traffic"]).unwrap_or("unknown");
    let provenance_passed = json_bool(&provenance, &["passed"]);
    let update_passed = update_traffic == "green";
    let passed = provenance_passed && update_passed;

    Ok(case_result(
        "green-baseline",
        "Unmodified init-app package evidence.",
        "green",
        if passed { "green" } else { update_traffic },
        passed,
        vec![
            format!("provenance.passed={provenance_passed}"),
            format!("update.traffic={update_traffic}"),
            format!("fixture={}", project.display()),
        ],
    ))
}

fn yellow_local_edit_case(root: &Path) -> anyhow::Result<DxForgeTrustRegressionCase> {
    let project = scaffold_case_project(root, "yellow-local-edit")?;
    let button = project.join("components/ui/button.tsx");
    let mut content = fs::read_to_string(&button)?;
    content.push_str("\n// Local trust-regression edit that must require update review.\n");
    fs::write(&button, content)?;

    let verify = serde_json::to_value(build_forge_verify_package_report(
        &project,
        BUTTON_PACKAGE_ID,
        "default",
    )?)?;
    let update_traffic = json_string(&verify, &["update", "traffic"]).unwrap_or("unknown");
    let update_message = json_string(&verify, &["update", "message"]).unwrap_or("missing update");
    let passed = update_traffic == "yellow";

    Ok(case_result(
        "yellow-local-edit",
        "Mutate materialized package source after receipt capture.",
        "yellow",
        update_traffic,
        passed,
        vec![
            format!("update.traffic={update_traffic}"),
            update_message.to_string(),
            format!("file={}", button.display()),
        ],
    ))
}

fn red_provenance_mismatch_case(root: &Path) -> anyhow::Result<DxForgeTrustRegressionCase> {
    let project = scaffold_case_project(root, "red-provenance-mismatch")?;
    mutate_manifest_package(&project, BUTTON_PACKAGE_ID, |package| {
        package.provenance.source = "tampered-local-registry".to_string();
        package.provenance.verified = true;
        package.provenance.note =
            "Trust-regression mutation: this provenance must not match registry metadata."
                .to_string();
    })?;

    let provenance = serde_json::to_value(build_forge_provenance_report(&project, 90)?)?;
    let check_passed = json_bool(&provenance, &["checks", "provenance_metadata", "passed"]);
    let check_message = json_string(&provenance, &["checks", "provenance_metadata", "message"])
        .unwrap_or("missing provenance_metadata check");
    let observed = if check_passed { "green" } else { "red" };
    let passed = observed == "red";

    Ok(case_result(
        "red-provenance-mismatch",
        "Mutate source manifest provenance away from the curated registry record.",
        "red",
        observed,
        passed,
        vec![
            format!("provenance_metadata.passed={check_passed}"),
            check_message.to_string(),
            "packages[].provenance".to_string(),
        ],
    ))
}

fn red_license_mismatch_case(root: &Path) -> anyhow::Result<DxForgeTrustRegressionCase> {
    let project = scaffold_case_project(root, "red-license-mismatch")?;
    mutate_manifest_package(&project, BUTTON_PACKAGE_ID, |package| {
        package.license = "UNREVIEWED-PROPRIETARY".to_string();
        package.license_review = DxForgeLicenseReviewMetadata {
            declared_license: "UNREVIEWED-PROPRIETARY".to_string(),
            reviewed: false,
            reviewed_at: None,
            note: "Trust-regression mutation: license must fail registry comparison.".to_string(),
        };
    })?;

    let provenance = serde_json::to_value(build_forge_provenance_report(&project, 90)?)?;
    let check_passed = json_bool(&provenance, &["checks", "license_metadata", "passed"]);
    let check_message = json_string(&provenance, &["checks", "license_metadata", "message"])
        .unwrap_or("missing license_metadata check");
    let observed = if check_passed { "green" } else { "red" };
    let passed = observed == "red";

    Ok(case_result(
        "red-license-mismatch",
        "Mutate source manifest license metadata away from registry metadata.",
        "red",
        observed,
        passed,
        vec![
            format!("license_metadata.passed={check_passed}"),
            check_message.to_string(),
            "packages[].license_review".to_string(),
        ],
    ))
}

fn red_receipt_missing_case(root: &Path) -> anyhow::Result<DxForgeTrustRegressionCase> {
    let project = scaffold_case_project(root, "red-receipt-missing")?;
    let manifest = read_manifest(&project)?;
    let receipt = manifest
        .receipts
        .iter()
        .find(|receipt| receipt.contains("button"))
        .or_else(|| manifest.receipts.first())
        .ok_or_else(|| anyhow::anyhow!("fixture has no receipt to mutate"))?;
    let receipt_path = project.join(".dx/forge/receipts").join(receipt);
    fs::remove_file(&receipt_path)?;

    let provenance = serde_json::to_value(build_forge_provenance_report(&project, 90)?)?;
    let check_passed = json_bool(&provenance, &["checks", "receipt_hashes", "passed"]);
    let check_message = json_string(&provenance, &["checks", "receipt_hashes", "message"])
        .unwrap_or("missing receipt_hashes check");
    let observed = if check_passed { "green" } else { "red" };
    let passed = observed == "red";

    Ok(case_result(
        "red-receipt-missing",
        "Delete a referenced Forge receipt after manifest capture.",
        "red",
        observed,
        passed,
        vec![
            format!("receipt_hashes.passed={check_passed}"),
            check_message.to_string(),
            format!("removed={}", receipt_path.display()),
        ],
    ))
}

fn yellow_offline_advisory_case(root: &Path) -> anyhow::Result<DxForgeTrustRegressionCase> {
    let project = scaffold_case_project(root, "yellow-offline-advisory")?;
    let advisory_path = project.join(".dx/forge/advisories.json");
    fs::write(
        &advisory_path,
        r#"{
  "version": 1,
  "packages": [
    {
      "package_id": "ui/button",
      "provider": "dx-forge-offline-osv-snapshot",
      "finding_count": 1,
      "reviewed_at": "2026-05-18T00:00:00Z",
      "note": "Offline OSV snapshot records one known advisory for regression testing."
    }
  ]
}"#,
    )?;

    let gallery = serde_json::to_value(build_forge_package_gallery_report(&project, 90)?)?;
    let button = gallery["packages"]
        .as_array()
        .and_then(|packages| {
            packages
                .iter()
                .find(|package| package["package_id"] == BUTTON_PACKAGE_ID)
        })
        .ok_or_else(|| anyhow::anyhow!("package gallery did not include {BUTTON_PACKAGE_ID}"))?;
    let coverage = json_string(button, &["advisory", "coverage_kind"]).unwrap_or("unknown");
    let finding_count = button["advisory"]["finding_count"].as_u64().unwrap_or(0);
    let placeholder = button["advisory"]["placeholder_present"]
        .as_bool()
        .unwrap_or(true);
    let observed = if coverage == "offline-snapshot" && finding_count > 0 && !placeholder {
        "yellow"
    } else {
        "green"
    };
    let passed = observed == "yellow";

    Ok(case_result(
        "yellow-offline-advisory",
        "Write offline advisory metadata with a known finding.",
        "yellow",
        observed,
        passed,
        vec![
            format!("advisory.coverage_kind={coverage}"),
            format!("advisory.finding_count={finding_count}"),
            format!("advisory.placeholder_present={placeholder}"),
            format!("file={}", advisory_path.display()),
        ],
    ))
}

fn scaffold_case_project(root: &Path, name: &str) -> anyhow::Result<PathBuf> {
    let project = root.join(name);
    fs::create_dir_all(&project)?;
    write_forge_adoption_project_scaffold(&project)?;
    for package_id in FORGE_WWW_TEMPLATE_PACKAGE_IDS {
        write_forge_add_variant(package_id, "default", &project)?;
    }
    Ok(project)
}

fn mutate_manifest_package<F>(project: &Path, package_id: &str, mutate: F) -> anyhow::Result<()>
where
    F: FnOnce(&mut dx_compiler::ecosystem::DxSourcePackage),
{
    let mut manifest = read_manifest(project)?;
    let canonical = canonical_package_id(package_id).to_string();
    let package = manifest
        .packages
        .iter_mut()
        .find(|package| canonical_package_id(&package.package_id) == canonical)
        .ok_or_else(|| anyhow::anyhow!("fixture package `{canonical}` is missing"))?;
    mutate(package);
    write_manifest(project, &manifest)
}

fn read_manifest(project: &Path) -> anyhow::Result<DxSourceManifest> {
    let manifest_path = project.join(".dx/forge/source-.dx/build-cache/manifest.json");
    Ok(serde_json::from_slice(&fs::read(&manifest_path)?)?)
}

fn write_manifest(project: &Path, manifest: &DxSourceManifest) -> anyhow::Result<()> {
    let manifest_path = project.join(".dx/forge/source-.dx/build-cache/manifest.json");
    fs::write(&manifest_path, serde_json::to_string_pretty(manifest)?)?;
    Ok(())
}

fn case_result(
    name: &str,
    mutation: &str,
    expected_traffic: &str,
    observed_traffic: &str,
    passed: bool,
    evidence: Vec<String>,
) -> DxForgeTrustRegressionCase {
    DxForgeTrustRegressionCase {
        name: name.to_string(),
        mutation: mutation.to_string(),
        expected_traffic: expected_traffic.to_string(),
        observed_traffic: observed_traffic.to_string(),
        passed,
        score: if passed { 100 } else { 0 },
        evidence,
        message: if passed {
            format!("Observed expected `{expected_traffic}` trust signal.")
        } else {
            format!("Expected `{expected_traffic}` but observed `{observed_traffic}`.")
        },
    }
}

fn unique_fixture_root(project: &Path) -> anyhow::Result<PathBuf> {
    let base = format!(
        "{}-{}",
        Utc::now().format("%Y%m%dT%H%M%SZ"),
        std::process::id()
    );
    let parent = project.join(FIXTURE_ROOT);
    for index in 0..100u32 {
        let candidate = if index == 0 {
            parent.join(&base)
        } else {
            parent.join(format!("{base}-{index}"))
        };
        if !candidate.exists() {
            return Ok(candidate);
        }
    }
    Err(anyhow::anyhow!(
        "could not allocate a unique Forge trust-regression fixture directory under `{}`",
        parent.display()
    ))
}

fn tree_has_node_modules(root: &Path) -> anyhow::Result<bool> {
    if !root.exists() {
        return Ok(false);
    }
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        if entry.file_name() == "node_modules" {
            return Ok(true);
        }
        if entry.file_type()?.is_dir() && tree_has_node_modules(&entry.path())? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn json_bool(value: &serde_json::Value, path: &[&str]) -> bool {
    path.iter()
        .try_fold(value, |current, key| current.get(*key))
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
}

fn json_string<'a>(value: &'a serde_json::Value, path: &[&str]) -> Option<&'a str> {
    path.iter()
        .try_fold(value, |current, key| current.get(*key))
        .and_then(serde_json::Value::as_str)
}
