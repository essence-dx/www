use std::collections::BTreeMap;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use chrono::Utc;
use dx_compiler::ecosystem::{
    DxForgeAction, DxForgeLicenseReviewMetadata, DxForgeProvenanceMetadata, DxForgeReceipt,
    DxSourceKind, DxSourceManifest, DxSourcePackage, canonical_package_id, registry_package,
    verify_registry_package_integrity,
};
use serde::Serialize;

const SOURCE_MANIFEST_PATH: &str = ".dx/forge/source-.dx/build-cache/manifest.json";
const RECEIPT_DIR: &str = ".dx/forge/receipts";

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeProvenanceReport {
    pub(super) version: u32,
    pub(super) project: PathBuf,
    pub(super) generated_at: String,
    pub(super) passed: bool,
    pub(super) score: u8,
    pub(super) fail_under: u8,
    pub(super) source_manifest_path: PathBuf,
    pub(super) receipt_dir: PathBuf,
    pub(super) registry_manifest: DxForgeProvenanceManifestEvidence,
    pub(super) no_node_modules: bool,
    pub(super) package_count: usize,
    pub(super) receipt_hash_count: usize,
    pub(super) accepted_update_package_count: usize,
    pub(super) rollback_required_package_count: usize,
    pub(super) rollback_covered_package_count: usize,
    pub(super) checks: DxForgeProvenanceChecks,
    pub(super) packages: Vec<DxForgeProvenancePackage>,
    pub(super) findings: Vec<String>,
    pub(super) next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeProvenanceManifestEvidence {
    present: bool,
    version: Option<u32>,
    package_count: usize,
    receipt_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeProvenanceChecks {
    registry_manifest: DxForgeProvenanceCheck,
    receipt_hashes: DxForgeProvenanceCheck,
    accepted_update_receipts: DxForgeProvenanceCheck,
    provenance_metadata: DxForgeProvenanceCheck,
    license_metadata: DxForgeProvenanceCheck,
    rollback_coverage: DxForgeProvenanceCheck,
    no_node_modules: DxForgeProvenanceCheck,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeProvenanceCheck {
    name: String,
    passed: bool,
    score: u8,
    message: String,
    evidence: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeProvenancePackage {
    package_id: String,
    variant: String,
    version: String,
    source_kind: String,
    registry_manifest_present: bool,
    registry_integrity_hash: Option<String>,
    registry_license: DxForgeProvenanceLicense,
    registry_provenance: Option<DxForgeProvenanceMetadata>,
    license: DxForgeProvenanceLicense,
    provenance: DxForgeProvenanceMetadata,
    integrity_hash: String,
    file_count: usize,
    materialized_file_count: usize,
    materialized_files_present: bool,
    receipt_count: usize,
    receipt_hashes: Vec<DxForgeProvenanceReceiptHash>,
    latest_receipt: Option<String>,
    accepted_update_receipt_present: bool,
    accepted_update_receipt: Option<String>,
    last_accepted_update: Option<String>,
    rollback_required: bool,
    rollback_covered: bool,
    rollback_receipt: Option<String>,
    rollback_receipt_hash: Option<String>,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeProvenanceLicense {
    declared: String,
    reviewed: bool,
    reviewed_at: Option<String>,
    note: String,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeProvenanceReceiptHash {
    receipt: String,
    action: String,
    hash: String,
    timestamp: String,
    update_decisions: usize,
}

#[derive(Debug, Clone)]
struct ReceiptEvidence {
    name: String,
    hash: String,
    action: DxForgeAction,
    action_label: String,
    package_id: String,
    variant: String,
    timestamp: String,
    update_decisions: usize,
}

pub(super) fn build_forge_provenance_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeProvenanceReport> {
    let generated_at = Utc::now().to_rfc3339();
    let source_manifest_path = project.join(SOURCE_MANIFEST_PATH);
    let receipt_dir = project.join(RECEIPT_DIR);
    let no_node_modules = !project.join("node_modules").exists();
    let mut findings = Vec::new();

    let manifest = match read_optional_source_manifest(&source_manifest_path)? {
        Some(manifest) => manifest,
        None => {
            findings.push(format!(
                "Forge source manifest is missing at `{}`.",
                source_manifest_path.display()
            ));
            DxSourceManifest::default()
        }
    };

    let receipt_read = read_receipt_evidence(&manifest, &receipt_dir)?;
    findings.extend(receipt_read.findings);
    let receipt_by_name: BTreeMap<String, ReceiptEvidence> = receipt_read
        .receipts
        .iter()
        .map(|receipt| (receipt.name.clone(), receipt.clone()))
        .collect();

    let mut packages = Vec::new();
    for package in &manifest.packages {
        packages.push(build_package_provenance(
            project,
            package,
            &receipt_read.receipts,
            &receipt_by_name,
        )?);
    }

    for package in &packages {
        findings.extend(package.findings.iter().cloned());
    }
    if !no_node_modules {
        findings.push("node_modules exists in the provenance project.".to_string());
    }

    let registry_manifest = DxForgeProvenanceManifestEvidence {
        present: source_manifest_path.exists(),
        version: source_manifest_path.exists().then_some(manifest.version),
        package_count: manifest.packages.len(),
        receipt_count: manifest.receipts.len(),
    };

    let accepted_update_package_count = packages
        .iter()
        .filter(|package| package.accepted_update_receipt_present)
        .count();
    let rollback_required_package_count = packages
        .iter()
        .filter(|package| package.rollback_required)
        .count();
    let rollback_covered_package_count = packages
        .iter()
        .filter(|package| package.rollback_required && package.rollback_covered)
        .count();
    let receipt_hash_count = packages
        .iter()
        .map(|package| package.receipt_hashes.len())
        .sum::<usize>();

    let checks = build_checks(
        no_node_modules,
        &registry_manifest,
        &packages,
        receipt_read.receipt_errors,
    );
    let score = checks.score();
    let passed = findings.is_empty() && score >= fail_under && checks.passed();

    Ok(DxForgeProvenanceReport {
        version: 1,
        project: project.to_path_buf(),
        generated_at,
        passed,
        score,
        fail_under,
        source_manifest_path,
        receipt_dir,
        registry_manifest,
        no_node_modules,
        package_count: packages.len(),
        receipt_hash_count,
        accepted_update_package_count,
        rollback_required_package_count,
        rollback_covered_package_count,
        checks,
        packages,
        findings,
        next_commands: vec![
            "dx forge provenance --project . --format markdown --output .dx/forge/provenance.md"
                .to_string(),
            "dx forge verify-package --all --project . --format markdown".to_string(),
            "dx check --strict-forge --format markdown".to_string(),
        ],
    })
}

pub(super) fn forge_provenance_terminal(report: &DxForgeProvenanceReport) -> String {
    let mut output = format!(
        "DX Forge source-owned package provenance\nProject: {}\nGenerated: {}\nPassed: {}\nScore: {} / 100\nPackages: {}\nReceipt hashes: {}\nAccepted updates: {}\nRollback coverage: {}/{}\nNo node_modules: {}\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.package_count,
        report.receipt_hash_count,
        report.accepted_update_package_count,
        report.rollback_covered_package_count,
        report.rollback_required_package_count,
        report.no_node_modules
    );

    output.push_str("\nChecks:\n");
    for check in report.checks.as_list() {
        output.push_str(&format!(
            "- {}: {} ({} / 100) {}\n",
            check.name, check.passed, check.score, check.message
        ));
    }

    output.push_str("\nPackages:\n");
    for package in &report.packages {
        output.push_str(&format!(
            "- {}@{}: receipts {}, accepted update {}, rollback {}, license {}\n",
            package.package_id,
            package.version,
            package.receipt_count,
            package.accepted_update_receipt_present,
            package.rollback_covered,
            package.license.declared
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

pub(super) fn forge_provenance_markdown(report: &DxForgeProvenanceReport) -> String {
    let mut output = format!(
        "# DX Forge Source-Owned Package Provenance\n\n- Project: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Source manifest: `{}`\n- Receipt directory: `{}`\n- Packages: `{}`\n- Receipt hashes: `{}`\n- Accepted update packages: `{}`\n- Rollback coverage: `{}` / `{}`\n- no `node_modules`: `{}`\n\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.fail_under,
        report.source_manifest_path.display(),
        report.receipt_dir.display(),
        report.package_count,
        report.receipt_hash_count,
        report.accepted_update_package_count,
        report.rollback_covered_package_count,
        report.rollback_required_package_count,
        report.no_node_modules
    );

    output.push_str("## Registry Manifest\n\n");
    output.push_str("| Present | Version | Packages | Receipts |\n");
    output.push_str("| --- | ---: | ---: | ---: |\n");
    output.push_str(&format!(
        "| `{}` | `{}` | {} | {} |\n\n",
        report.registry_manifest.present,
        report
            .registry_manifest
            .version
            .map(|version| version.to_string())
            .unwrap_or_else(|| "-".to_string()),
        report.registry_manifest.package_count,
        report.registry_manifest.receipt_count
    ));

    output.push_str("## Checks\n\n");
    output.push_str("| Check | Passed | Score | Evidence | Message |\n");
    output.push_str("| --- | --- | ---: | --- | --- |\n");
    for check in report.checks.as_list() {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | `{}` | {} |\n",
            check.name,
            check.passed,
            check.score,
            super::markdown_table_cell(check.evidence.as_deref().unwrap_or("-")),
            super::markdown_table_cell(&check.message)
        ));
    }

    output.push_str("\n## Packages\n\n");
    output.push_str("| Package | Version | Source | Registry Manifest | Receipts | Accepted Updates | Rollback Coverage | License Metadata |\n");
    output.push_str("| --- | --- | --- | --- | ---: | --- | --- | --- |\n");
    for package in &report.packages {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | {} | `{}` | `{}` | `{}` |\n",
            package.package_id,
            package.version,
            package.source_kind,
            package.registry_manifest_present,
            package.receipt_count,
            package.accepted_update_receipt_present,
            package.rollback_covered,
            super::markdown_table_cell(&package.license.declared)
        ));
    }

    output.push_str("\n## Receipt Hashes\n\n");
    output.push_str("| Package | Receipt | Action | Hash | Update Decisions |\n");
    output.push_str("| --- | --- | --- | --- | ---: |\n");
    for package in &report.packages {
        for receipt in &package.receipt_hashes {
            output.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | {} |\n",
                package.package_id,
                super::markdown_table_cell(&receipt.receipt),
                receipt.action,
                receipt.hash,
                receipt.update_decisions
            ));
        }
    }

    output.push_str("\n## Accepted Updates\n\n");
    output.push_str("| Package | Last Accepted Update | Receipt | Present |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for package in &report.packages {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` |\n",
            package.package_id,
            package.last_accepted_update.as_deref().unwrap_or("-"),
            package.accepted_update_receipt.as_deref().unwrap_or("-"),
            package.accepted_update_receipt_present
        ));
    }

    output.push_str("\n## License Metadata\n\n");
    output.push_str("| Package | Declared | Registry Declared | Reviewed | Note |\n");
    output.push_str("| --- | --- | --- | --- | --- |\n");
    for package in &report.packages {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | {} |\n",
            package.package_id,
            super::markdown_table_cell(&package.license.declared),
            super::markdown_table_cell(&package.registry_license.declared),
            package.license.reviewed,
            super::markdown_table_cell(&package.license.note)
        ));
    }

    output.push_str("\n## Rollback Coverage\n\n");
    output.push_str("| Package | Required | Covered | Receipt | Receipt Hash |\n");
    output.push_str("| --- | --- | --- | --- | --- |\n");
    for package in &report.packages {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            package.package_id,
            package.rollback_required,
            package.rollback_covered,
            package.rollback_receipt.as_deref().unwrap_or("-"),
            package.rollback_receipt_hash.as_deref().unwrap_or("-")
        ));
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- `pass`: provenance artifact has no findings.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {}\n", super::markdown_table_cell(finding)));
        }
    }

    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{command}`\n"));
    }

    output
}

pub(super) fn forge_provenance_failure_summary(report: &DxForgeProvenanceReport) -> String {
    if report.findings.is_empty() {
        return format!(
            "DX Forge provenance failed with score {} / 100",
            report.score
        );
    }
    format!("DX Forge provenance failed: {}", report.findings.join("; "))
}

fn build_package_provenance(
    project: &Path,
    package: &DxSourcePackage,
    receipts: &[ReceiptEvidence],
    receipt_by_name: &BTreeMap<String, ReceiptEvidence>,
) -> anyhow::Result<DxForgeProvenancePackage> {
    let canonical = canonical_package_id(&package.package_id).to_string();
    let source_kind = source_kind_label(&package.source_kind).to_string();
    let requires_registry_manifest = package.source_kind != DxSourceKind::Local;
    let mut findings = Vec::new();

    let registry = registry_package(&canonical).ok();
    let registry_integrity = registry
        .as_ref()
        .and_then(|package| verify_registry_package_integrity(package).ok());
    let registry_manifest_present = if requires_registry_manifest {
        registry.is_some() && registry_integrity.is_some()
    } else {
        registry.is_some()
    };
    if requires_registry_manifest && registry.is_none() {
        findings.push(format!(
            "`{canonical}` is missing from the curated Forge registry manifest."
        ));
    }
    if requires_registry_manifest && registry.as_ref().is_some() && registry_integrity.is_none() {
        findings.push(format!(
            "`{canonical}` registry manifest exists but integrity verification failed."
        ));
    }

    let registry_license = registry
        .as_ref()
        .map(|registry| license_from_review(&registry.license_review))
        .unwrap_or_else(|| DxForgeProvenanceLicense {
            declared: "UNKNOWN".to_string(),
            reviewed: false,
            reviewed_at: None,
            note: "No registry package metadata was found for this local package.".to_string(),
        });
    let license = license_from_review(&package.license_review);
    if package.license.trim().is_empty() {
        findings.push(format!("`{canonical}` has no declared license metadata."));
    }
    if requires_registry_manifest && registry_license.declared != package.license {
        findings.push(format!(
            "`{canonical}` manifest license `{}` does not match registry license `{}`.",
            package.license, registry_license.declared
        ));
    }
    let registry_provenance = registry
        .as_ref()
        .map(|registry| registry.provenance.clone());
    if requires_registry_manifest
        && registry_provenance
            .as_ref()
            .is_some_and(|provenance| provenance != &package.provenance)
    {
        findings.push(format!(
            "`{canonical}` manifest provenance metadata does not match the curated registry manifest."
        ));
    }

    let matching_receipts = receipts
        .iter()
        .filter(|receipt| {
            canonical_package_id(&receipt.package_id) == canonical
                && receipt.variant == package.variant
        })
        .collect::<Vec<_>>();
    if matching_receipts.is_empty() {
        findings.push(format!(
            "`{canonical}` has no receipt hash evidence in `{RECEIPT_DIR}`."
        ));
    }

    let accepted_update_receipt = package.last_accepted_update.as_ref().and_then(|timestamp| {
        matching_receipts
            .iter()
            .find(|receipt| {
                receipt.action == DxForgeAction::UpdateWrite && receipt.timestamp == *timestamp
            })
            .map(|receipt| receipt.name.clone())
    });
    let accepted_update_receipt_present = if package.last_accepted_update.is_some() {
        accepted_update_receipt.is_some()
    } else {
        false
    };
    if package.last_accepted_update.is_some() && !accepted_update_receipt_present {
        findings.push(format!(
            "`{canonical}` records last_accepted_update but no matching update-write receipt was found."
        ));
    }

    let rollback_required = package.last_accepted_update.is_some();
    let rollback_receipt_hash = package
        .rollback_receipt
        .as_ref()
        .and_then(|receipt| receipt_by_name.get(receipt))
        .map(|receipt| receipt.hash.clone());
    let rollback_covered = if rollback_required {
        rollback_receipt_hash.is_some()
    } else {
        true
    };
    if rollback_required && !rollback_covered {
        findings.push(format!(
            "`{canonical}` accepted an update but its rollback receipt is missing."
        ));
    }

    let receipt_hashes = matching_receipts
        .iter()
        .map(|receipt| DxForgeProvenanceReceiptHash {
            receipt: receipt.name.clone(),
            action: receipt.action_label.clone(),
            hash: receipt.hash.clone(),
            timestamp: receipt.timestamp.clone(),
            update_decisions: receipt.update_decisions,
        })
        .collect::<Vec<_>>();
    let latest_receipt = matching_receipts.last().map(|receipt| receipt.name.clone());
    let materialized_file_count = package
        .files
        .iter()
        .filter(|file| project.join(&file.path).is_file())
        .count();
    let materialized_files_present = materialized_file_count == package.files.len();
    if !materialized_files_present {
        findings.push(format!(
            "`{canonical}` manifest lists {} files but only {} exist locally.",
            package.files.len(),
            materialized_file_count
        ));
    }

    Ok(DxForgeProvenancePackage {
        package_id: canonical,
        variant: package.variant.clone(),
        version: package.version.clone(),
        source_kind,
        registry_manifest_present,
        registry_integrity_hash: registry_integrity.map(|report| report.integrity_hash),
        registry_license,
        registry_provenance,
        license,
        provenance: package.provenance.clone(),
        integrity_hash: package.integrity_hash.clone(),
        file_count: package.files.len(),
        materialized_file_count,
        materialized_files_present,
        receipt_count: receipt_hashes.len(),
        receipt_hashes,
        latest_receipt,
        accepted_update_receipt_present,
        accepted_update_receipt,
        last_accepted_update: package.last_accepted_update.clone(),
        rollback_required,
        rollback_covered,
        rollback_receipt: package.rollback_receipt.clone(),
        rollback_receipt_hash,
        findings,
    })
}

fn build_checks(
    no_node_modules: bool,
    registry_manifest: &DxForgeProvenanceManifestEvidence,
    packages: &[DxForgeProvenancePackage],
    receipt_errors: usize,
) -> DxForgeProvenanceChecks {
    let registry_required = packages
        .iter()
        .filter(|package| package.source_kind != "local")
        .count();
    let registry_present = packages
        .iter()
        .filter(|package| package.source_kind == "local" || package.registry_manifest_present)
        .count();
    let registry_passed = registry_manifest.present
        && registry_manifest.package_count > 0
        && registry_present == packages.len();
    let receipt_packages = packages
        .iter()
        .filter(|package| package.receipt_count > 0)
        .count();
    let receipt_passed =
        receipt_errors == 0 && !packages.is_empty() && receipt_packages == packages.len();
    let accepted_required = packages
        .iter()
        .filter(|package| package.last_accepted_update.is_some())
        .count();
    let accepted_with_receipts = packages
        .iter()
        .filter(|package| package.accepted_update_receipt_present)
        .count();
    let accepted_present = packages
        .iter()
        .filter(|package| {
            package.last_accepted_update.is_none() || package.accepted_update_receipt_present
        })
        .count();
    let accepted_passed = accepted_present == packages.len();
    let provenance_packages = packages
        .iter()
        .filter(|package| {
            package.source_kind == "local"
                || package
                    .registry_provenance
                    .as_ref()
                    .is_some_and(|provenance| provenance == &package.provenance)
        })
        .count();
    let provenance_passed = !packages.is_empty() && provenance_packages == packages.len();
    let license_packages = packages
        .iter()
        .filter(|package| {
            !package.license.declared.trim().is_empty()
                && (package.source_kind == "local"
                    || package.license.declared == package.registry_license.declared)
        })
        .count();
    let license_passed = !packages.is_empty() && license_packages == packages.len();
    let rollback_required = packages
        .iter()
        .filter(|package| package.rollback_required)
        .count();
    let rollback_covered = packages
        .iter()
        .filter(|package| package.rollback_required && package.rollback_covered)
        .count();
    let rollback_present = packages
        .iter()
        .filter(|package| !package.rollback_required || package.rollback_covered)
        .count();
    let rollback_passed = rollback_present == packages.len();

    DxForgeProvenanceChecks {
        registry_manifest: DxForgeProvenanceCheck {
            name: "registry_manifest".to_string(),
            passed: registry_passed,
            score: ratio_score(registry_present, packages.len()),
            message: if registry_required == 0 {
                "Local source packages are present; no curated registry manifest is required."
                    .to_string()
            } else {
                format!(
                    "{registry_present} of {} packages have manifest-backed registry provenance.",
                    packages.len()
                )
            },
            evidence: Some(SOURCE_MANIFEST_PATH.to_string()),
        },
        receipt_hashes: DxForgeProvenanceCheck {
            name: "receipt_hashes".to_string(),
            passed: receipt_passed,
            score: ratio_score(receipt_packages, packages.len()),
            message: format!(
                "{receipt_packages} of {} packages have BLAKE3 receipt hash evidence; receipt read errors: {receipt_errors}.",
                packages.len()
            ),
            evidence: Some(RECEIPT_DIR.to_string()),
        },
        accepted_update_receipts: DxForgeProvenanceCheck {
            name: "accepted_update_receipts".to_string(),
            passed: accepted_passed,
            score: ratio_score(accepted_present, packages.len()),
            message: if accepted_required == 0 {
                "No accepted updates are recorded yet; fresh packages are explicitly reported."
                    .to_string()
            } else {
                format!(
                    "{accepted_with_receipts} of {accepted_required} package(s) with accepted updates have matching update-write receipts."
                )
            },
            evidence: Some("packages[].last_accepted_update".to_string()),
        },
        provenance_metadata: DxForgeProvenanceCheck {
            name: "provenance_metadata".to_string(),
            passed: provenance_passed,
            score: ratio_score(provenance_packages, packages.len()),
            message: format!(
                "{provenance_packages} of {} packages have source manifest provenance matching the curated registry manifest where required.",
                packages.len()
            ),
            evidence: Some("packages[].provenance".to_string()),
        },
        license_metadata: DxForgeProvenanceCheck {
            name: "license_metadata".to_string(),
            passed: license_passed,
            score: ratio_score(license_packages, packages.len()),
            message: format!(
                "{license_packages} of {} packages have declared license metadata matching the registry manifest where required.",
                packages.len()
            ),
            evidence: Some("packages[].license_review".to_string()),
        },
        rollback_coverage: DxForgeProvenanceCheck {
            name: "rollback_coverage".to_string(),
            passed: rollback_passed,
            score: ratio_score(rollback_present, packages.len()),
            message: if rollback_required == 0 {
                "No accepted updates require rollback receipts yet; fresh packages are explicitly exempt."
                    .to_string()
            } else {
                format!(
                    "{rollback_covered} of {rollback_required} package(s) that require rollback coverage have referenced rollback receipts."
                )
            },
            evidence: Some("packages[].rollback_receipt".to_string()),
        },
        no_node_modules: DxForgeProvenanceCheck {
            name: "no_node_modules".to_string(),
            passed: no_node_modules,
            score: if no_node_modules { 100 } else { 0 },
            message: if no_node_modules {
                "Forge provenance did not create node_modules.".to_string()
            } else {
                "node_modules exists in the checked project.".to_string()
            },
            evidence: Some("node_modules".to_string()),
        },
    }
}

impl DxForgeProvenanceChecks {
    fn as_list(&self) -> [&DxForgeProvenanceCheck; 7] {
        [
            &self.registry_manifest,
            &self.receipt_hashes,
            &self.accepted_update_receipts,
            &self.provenance_metadata,
            &self.license_metadata,
            &self.rollback_coverage,
            &self.no_node_modules,
        ]
    }

    fn passed(&self) -> bool {
        self.as_list().iter().all(|check| check.passed)
    }

    fn score(&self) -> u8 {
        self.as_list()
            .into_iter()
            .map(|check| check.score)
            .min()
            .unwrap_or(0)
    }
}

#[derive(Debug)]
struct ReceiptRead {
    receipts: Vec<ReceiptEvidence>,
    findings: Vec<String>,
    receipt_errors: usize,
}

fn read_receipt_evidence(
    manifest: &DxSourceManifest,
    receipt_dir: &Path,
) -> anyhow::Result<ReceiptRead> {
    let mut receipts = Vec::new();
    let mut findings = Vec::new();
    let mut receipt_errors = 0usize;

    for receipt_name in &manifest.receipts {
        let path = receipt_dir.join(receipt_name);
        let bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == ErrorKind::NotFound => {
                receipt_errors += 1;
                findings.push(format!(
                    "Manifest receipt `{receipt_name}` is missing from `{}`.",
                    receipt_dir.display()
                ));
                continue;
            }
            Err(error) => {
                return Err(anyhow::anyhow!("read `{}`: {error}", path.display()));
            }
        };
        let receipt: DxForgeReceipt = match serde_json::from_slice(&bytes) {
            Ok(receipt) => receipt,
            Err(error) => {
                receipt_errors += 1;
                findings.push(format!(
                    "Manifest receipt `{receipt_name}` could not be parsed: {error}."
                ));
                continue;
            }
        };
        receipts.push(ReceiptEvidence {
            name: receipt_name.clone(),
            hash: hash_bytes(&bytes),
            action_label: action_label(&receipt.action).to_string(),
            action: receipt.action,
            package_id: receipt.package.package_id,
            variant: receipt.package.variant,
            timestamp: receipt.timestamp,
            update_decisions: receipt.update_decisions.len(),
        });
    }

    Ok(ReceiptRead {
        receipts,
        findings,
        receipt_errors,
    })
}

fn read_optional_source_manifest(path: &Path) -> anyhow::Result<Option<DxSourceManifest>> {
    match fs::read(path) {
        Ok(bytes) => serde_json::from_slice(&bytes)
            .map(Some)
            .map_err(|error| anyhow::anyhow!("parse `{}`: {error}", path.display())),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(anyhow::anyhow!("read `{}`: {error}", path.display())),
    }
}

fn license_from_review(review: &DxForgeLicenseReviewMetadata) -> DxForgeProvenanceLicense {
    DxForgeProvenanceLicense {
        declared: review.declared_license.clone(),
        reviewed: review.reviewed,
        reviewed_at: review.reviewed_at.clone(),
        note: review.note.clone(),
    }
}

fn source_kind_label(kind: &DxSourceKind) -> &'static str {
    match kind {
        DxSourceKind::CuratedRegistry => "curated-registry",
        DxSourceKind::NpmSnapshot => "npm-snapshot",
        DxSourceKind::ExternalSnapshot => "external-snapshot",
        DxSourceKind::Local => "local",
    }
}

fn action_label(action: &DxForgeAction) -> &'static str {
    match action {
        DxForgeAction::Audit => "audit",
        DxForgeAction::AddDryRun => "add-dry-run",
        DxForgeAction::AddWrite => "add-write",
        DxForgeAction::TrackDryRun => "track-dry-run",
        DxForgeAction::TrackWrite => "track-write",
        DxForgeAction::UpdateDryRun => "update-dry-run",
        DxForgeAction::UpdateWrite => "update-write",
        DxForgeAction::RemoveDryRun => "remove-dry-run",
        DxForgeAction::RemoveWrite => "remove-write",
        DxForgeAction::RollbackDryRun => "rollback-dry-run",
        DxForgeAction::RollbackWrite => "rollback-write",
        DxForgeAction::DocsDryRun => "docs-dry-run",
        DxForgeAction::DocsWrite => "docs-write",
    }
}

fn ratio_score(numerator: usize, denominator: usize) -> u8 {
    if denominator == 0 {
        return 0;
    }
    ((numerator * 100) / denominator) as u8
}

fn hash_bytes(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}
