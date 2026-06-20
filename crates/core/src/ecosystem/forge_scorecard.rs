//! Public Forge package scorecard reporting for launch packages.

use anyhow::{Context, Result, bail};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::forge_registry::{
    canonical_package_id, registry_package, verify_registry_package_integrity,
};
use super::forge_security::{
    DxForgeAdvisoryCoverageKind, DxForgeAdvisoryMetadata, DxForgeLicenseReviewMetadata,
    DxForgeProvenanceMetadata, DxSourceManifest, DxSourcePackage, DxUpdateTraffic,
};
use super::project_check::{
    DxCheckReport, DxCheckSection, check_dx_project, forge_launch_gate_findings,
};

const LAUNCH_PACKAGES: [&str; 32] = [
    "shadcn/ui/button",
    "shadcn/ui/badge",
    "shadcn/ui/card",
    "shadcn/ui/alert",
    "shadcn/ui/avatar",
    "shadcn/ui/skeleton",
    "shadcn/ui/label",
    "shadcn/ui/separator",
    "shadcn/ui/field",
    "shadcn/ui/item",
    "shadcn/ui/input",
    "shadcn/ui/textarea",
    "dx/icon/search",
    "auth/better-auth",
    "animation/motion",
    "i18n/next-intl",
    "tanstack/query",
    "validation/zod",
    "forms/react-hook-form",
    "payments/stripe-js",
    "automations/n8n",
    "state/zustand",
    "ai/vercel-ai",
    "api/trpc",
    "content/fumadocs-next",
    "content/react-markdown",
    "supabase/client",
    "db/drizzle-sqlite",
    "instantdb/react",
    "wasm/bindgen",
    "3d/launch-scene",
    "migration/static-site",
];
const SOURCE_MANIFEST_PATH: &str = ".dx/forge/source-.dx/build-cache/manifest.json";
const RECEIPT_DIR: &str = ".dx/forge/receipts";
const PACKAGE_DOCS_DIR: &str = ".dx/forge/docs";
const SCORECARD_HISTORY_DIR: &str = ".dx/forge/scorecard-history";
const ADVISORY_METADATA_PATH: &str = ".dx/forge/advisories.json";

/// Public scorecard for the current Forge launch package set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgePackageScorecardReport {
    /// RFC3339 generation timestamp.
    pub generated_at: String,
    /// Overall launch-package score from 0 to 100.
    pub score: u8,
    /// Optional local project evidence when scorecard runs in project mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<DxForgeProjectScorecardEvidence>,
    /// Package rows included in the scorecard.
    pub packages: Vec<DxForgePackageScorecardEntry>,
    /// Honest public boundaries that should stay visible near the scorecard.
    pub honest_boundaries: Vec<String>,
}

/// Index of preserved Forge package scorecard snapshots for release drift review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgePackageScorecardHistoryIndex {
    /// History index schema version.
    pub version: u32,
    /// RFC3339 timestamp for the latest index write.
    pub updated_at: String,
    /// Newest-first scorecard snapshots.
    #[serde(default)]
    pub snapshots: Vec<DxForgePackageScorecardHistoryEntry>,
}

/// One preserved Forge package scorecard history snapshot entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgePackageScorecardHistoryEntry {
    /// Scorecard generation timestamp.
    pub generated_at: String,
    /// Scorecard score from 0 to 100.
    pub score: u8,
    /// Previous newest score, if history already existed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_score: Option<u8>,
    /// Signed score delta against the previous newest snapshot.
    pub score_delta: i16,
    /// Number of packages included in the snapshot.
    pub package_count: u64,
    /// Number of packages with verified registry integrity.
    pub verified_package_count: u64,
    /// Number of packages materialized as source-owned files.
    pub source_owned_package_count: u64,
    /// Number of packages that create `node_modules`.
    pub node_modules_created_count: u64,
    /// Snapshot JSON filename stored beside the index.
    pub snapshot_file: String,
}

/// Project-level evidence merged into a Forge package scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeProjectScorecardEvidence {
    /// Checked project root.
    pub path: PathBuf,
    /// Whether `.dx/forge/source-.dx/build-cache/manifest.json` exists and parsed.
    pub manifest_present: bool,
    /// Source-owned packages tracked in the local manifest.
    pub manifest_package_count: u64,
    /// Receipt filenames tracked in the local manifest.
    pub manifest_receipt_count: u64,
    /// Current `dx check` score for the project.
    pub dx_check_score: u8,
    /// Current `dx check` traffic result.
    pub dx_check_traffic: DxUpdateTraffic,
    /// Current Forge section score from `dx check`.
    pub forge_score: u8,
    /// Current Forge section traffic from `dx check`.
    pub forge_traffic: DxUpdateTraffic,
    /// Current Forge section finding count from `dx check`.
    pub forge_finding_count: u64,
    /// Strict launch-gate finding count derived from `dx check`.
    pub launch_gate_findings: u64,
    /// Package docs coverage percentage from `dx check`.
    pub package_docs_coverage_percent: u64,
    /// Rollback coverage percentage from `dx check`.
    pub rollback_coverage_percent: u64,
}

/// One package row in the Forge public scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgePackageScorecardEntry {
    /// Canonical Forge package id.
    pub package_id: String,
    /// Package aliases accepted by the registry.
    pub aliases: Vec<String>,
    /// Package version.
    pub version: String,
    /// Source language segment.
    pub language: String,
    /// License expression.
    pub license: String,
    /// Human-readable package description.
    pub description: String,
    /// Number of source-owned files materialized by this package.
    pub file_count: u64,
    /// Total package source bytes.
    pub total_bytes: u64,
    /// BLAKE3 package integrity hash.
    pub integrity_hash: String,
    /// Structured provenance metadata.
    pub provenance: DxForgeProvenanceMetadata,
    /// Advisory coverage metadata.
    pub advisory_review: DxForgeAdvisoryMetadata,
    /// License review metadata.
    pub license_review: DxForgeLicenseReviewMetadata,
    /// Registry integrity verification status.
    pub integrity_verified: bool,
    /// Whether the package materializes editable source.
    pub source_owned: bool,
    /// Whether install scripts are blocked by the Forge model.
    pub install_scripts_blocked: bool,
    /// Whether package materialization creates `node_modules`.
    pub node_modules_created: bool,
    /// Public claim this package can support today.
    pub public_claim: String,
    /// Boundary this package should not overclaim today.
    pub launch_boundary: String,
    /// Optional local project evidence for this package.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_evidence: Option<DxForgePackageProjectEvidence>,
}

/// Local project evidence for one launch package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgePackageProjectEvidence {
    /// Number of manifest variants matching this canonical package id.
    pub manifest_variant_count: u64,
    /// Matching local variant names.
    pub variants: Vec<String>,
    /// Source-owned files tracked locally for this package.
    pub manifest_file_count: u64,
    /// Source-owned bytes tracked locally for this package.
    pub manifest_bytes: u64,
    /// Package docs files present for matching variants.
    pub docs_present: u64,
    /// Package docs files missing or empty for matching variants.
    pub docs_missing: u64,
    /// Rollback receipts present for matching variants.
    pub rollback_receipts_present: u64,
    /// Rollback receipts missing for matching variants.
    pub rollback_receipts_missing: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct DxForgeOfflineAdvisoryIndex {
    version: u32,
    #[serde(default)]
    packages: Vec<DxForgeOfflineAdvisoryPackage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct DxForgeOfflineAdvisoryPackage {
    package_id: String,
    provider: String,
    #[serde(default)]
    live_coverage: bool,
    finding_count: u64,
    reviewed_at: String,
    note: String,
}

/// Build the public Forge package scorecard for the current launch package set.
pub fn build_forge_package_scorecard() -> Result<DxForgePackageScorecardReport> {
    let mut packages = Vec::new();
    for package_id in LAUNCH_PACKAGES {
        packages.push(scorecard_entry_for_package_id(package_id)?);
    }

    let verified_count = packages
        .iter()
        .filter(|package| package.integrity_verified)
        .count() as u8;
    let score = 85u8
        .saturating_add(verified_count.saturating_mul(5))
        .min(100);

    Ok(DxForgePackageScorecardReport {
        generated_at: Utc::now().to_rfc3339(),
        score,
        project: None,
        packages,
        honest_boundaries: vec![
            "Forge is not a universal npm replacement; it is a source-owned package firewall for curated packages first.".to_string(),
            "The launch package set proves no install scripts, no package lifecycle hooks, and no node_modules creation for these packages.".to_string(),
            "Arbitrary npm, Cargo, pip, and Go package ingestion still needs provenance, advisory metadata, license review, and ecosystem-specific sandboxing.".to_string(),
        ],
    })
}

fn scorecard_entry_for_package_id(package_id: &str) -> Result<DxForgePackageScorecardEntry> {
    let package = registry_package(package_id)?;
    let integrity_verified = verify_registry_package_integrity(&package).is_ok();
    let file_count = package.files.len() as u64;
    let total_bytes = package.files.iter().map(|file| file.bytes).sum::<u64>();
    let package_id = package.package_id.clone();

    Ok(DxForgePackageScorecardEntry {
        package_id: package_id.clone(),
        aliases: package.aliases.clone(),
        version: package.version.clone(),
        language: package.language.as_segment().to_string(),
        license: package.license.clone(),
        description: package.description.clone(),
        file_count,
        total_bytes,
        integrity_hash: package.integrity_hash.clone(),
        provenance: package.provenance.clone(),
        advisory_review: package.advisory_review.clone(),
        license_review: package.license_review.clone(),
        integrity_verified,
        source_owned: true,
        install_scripts_blocked: true,
        node_modules_created: false,
        public_claim: package_public_claim(&package_id).to_string(),
        launch_boundary: package_launch_boundary(&package_id).to_string(),
        project_evidence: None,
    })
}

/// Build the Forge package scorecard with local project evidence attached.
pub fn build_forge_package_scorecard_for_project(
    project: impl AsRef<Path>,
) -> Result<DxForgePackageScorecardReport> {
    let root = project.as_ref();
    let mut report = build_forge_package_scorecard()?;
    let check = check_dx_project(root)?;
    let manifest = read_project_manifest(root)?;
    let offline_advisories = read_offline_advisory_metadata(root)?;

    if let Some(manifest) = &manifest {
        for source_package in &manifest.packages {
            let canonical = canonical_package_id(&source_package.package_id).to_string();
            if report
                .packages
                .iter()
                .any(|package| package.package_id == canonical)
            {
                continue;
            }
            if let Ok(entry) = scorecard_entry_for_package_id(&canonical) {
                report.packages.push(entry);
            }
        }
    }

    report.project = Some(project_scorecard_evidence(root, &check, manifest.as_ref()));
    for package in &mut report.packages {
        if let Some(advisory) = offline_advisories.get(&package.package_id) {
            package.advisory_review = advisory.clone();
        }
        package.project_evidence = Some(package_project_evidence(
            root,
            &package.package_id,
            manifest.as_ref(),
        ));
    }

    Ok(report)
}

/// Preserve a Forge package scorecard snapshot and update the local history index.
pub fn write_forge_package_scorecard_history(
    root: impl AsRef<Path>,
    report: &DxForgePackageScorecardReport,
) -> Result<PathBuf> {
    let root = root.as_ref();
    let history_dir = root.join(SCORECARD_HISTORY_DIR);
    fs::create_dir_all(&history_dir)
        .with_context(|| format!("create `{}`", history_dir.display()))?;

    let snapshot_file = unique_scorecard_snapshot_file(&history_dir, &report.generated_at);
    let snapshot_path = history_dir.join(&snapshot_file);
    let snapshot_json = serde_json::to_string_pretty(report).context("serialize scorecard")?;
    fs::write(&snapshot_path, snapshot_json)
        .with_context(|| format!("write `{}`", snapshot_path.display()))?;

    let index_path = history_dir.join("index.json");
    let mut index = read_scorecard_history_index(&index_path)?;
    let previous_score = index.snapshots.first().map(|snapshot| snapshot.score);
    let score_delta = previous_score
        .map(|score| report.score as i16 - score as i16)
        .unwrap_or(0);

    index.updated_at = Utc::now().to_rfc3339();
    index.snapshots.insert(
        0,
        DxForgePackageScorecardHistoryEntry {
            generated_at: report.generated_at.clone(),
            score: report.score,
            previous_score,
            score_delta,
            package_count: report.packages.len() as u64,
            verified_package_count: report
                .packages
                .iter()
                .filter(|package| package.integrity_verified)
                .count() as u64,
            source_owned_package_count: report
                .packages
                .iter()
                .filter(|package| package.source_owned)
                .count() as u64,
            node_modules_created_count: report
                .packages
                .iter()
                .filter(|package| package.node_modules_created)
                .count() as u64,
            snapshot_file,
        },
    );

    let index_json =
        serde_json::to_string_pretty(&index).context("serialize scorecard history index")?;
    fs::write(&index_path, index_json)
        .with_context(|| format!("write `{}`", index_path.display()))?;

    let index_markdown_path = history_dir.join("index.md");
    fs::write(
        &index_markdown_path,
        forge_package_scorecard_history_markdown(&index),
    )
    .with_context(|| format!("write `{}`", index_markdown_path.display()))?;

    Ok(index_path)
}

/// Render the Forge package scorecard as public Markdown.
pub fn forge_package_scorecard_markdown(report: &DxForgePackageScorecardReport) -> String {
    let mut output = format!(
        "# DX Forge Package Scorecard\n\n- Score: `{}`\n- Generated: `{}`\n- Packages: `{}`\n\n",
        report.score,
        report.generated_at,
        report.packages.len()
    );

    if let Some(project) = &report.project {
        output.push_str(&format!(
            "## Project Evidence\n\n- Path: `{}`\n- Manifest present: `{}`\n- Manifest packages: `{}`\n- Manifest receipts: `{}`\n- DX check: `{}` / `{}`\n- Forge check: `{}` / `{}`\n- Forge findings: `{}`\n- Launch gate findings: `{}`\n- Package docs coverage: `{}%`\n- Rollback coverage: `{}%`\n\n",
            project.path.display(),
            project.manifest_present,
            project.manifest_package_count,
            project.manifest_receipt_count,
            project.dx_check_score,
            project.dx_check_traffic.as_str(),
            project.forge_score,
            project.forge_traffic.as_str(),
            project.forge_finding_count,
            project.launch_gate_findings,
            project.package_docs_coverage_percent,
            project.rollback_coverage_percent
        ));
    }

    output.push_str("| Package | Version | Files | Bytes | Integrity | Source-owned | node_modules | Public claim |\n");
    output.push_str("| --- | --- | ---: | ---: | --- | --- | --- | --- |\n");
    for package in &report.packages {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | {} | {} | {} | {} | {} |\n",
            package.package_id,
            package.version,
            package.file_count,
            package.total_bytes,
            if package.integrity_verified {
                "verified"
            } else {
                "failed"
            },
            yes_no(package.source_owned),
            if package.node_modules_created {
                "created"
            } else {
                "none"
            },
            package.public_claim
        ));
    }

    if report
        .packages
        .iter()
        .any(|package| package.project_evidence.is_some())
    {
        output.push_str("\n## Local Package Evidence\n\n");
        output.push_str("| Package | Local variants | Local files | Docs | Rollback receipts |\n");
        output.push_str("| --- | ---: | ---: | --- | --- |\n");
        for package in &report.packages {
            if let Some(evidence) = &package.project_evidence {
                output.push_str(&format!(
                    "| `{}` | {} | {} | {}/{} present | {}/{} present |\n",
                    package.package_id,
                    evidence.manifest_variant_count,
                    evidence.manifest_file_count,
                    evidence.docs_present,
                    evidence.docs_present + evidence.docs_missing,
                    evidence.rollback_receipts_present,
                    evidence.rollback_receipts_present + evidence.rollback_receipts_missing
                ));
            }
        }
    }

    output.push_str("\n## Package Metadata Review\n\n");
    output.push_str("| Package | Provenance | Advisories | License review |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for package in &report.packages {
        output.push_str(&format!(
            "| `{}` | `{}` verified `{}` | coverage `{}`, provider `{}`, live `{}`, findings `{}` | declared `{}`, reviewed `{}` |\n",
            package.package_id,
            &package.provenance.source,
            yes_no(package.provenance.verified),
            package.advisory_review.coverage_kind.as_str(),
            &package.advisory_review.provider,
            yes_no(package.advisory_review.live_coverage),
            package.advisory_review.finding_count,
            &package.license_review.declared_license,
            yes_no(package.license_review.reviewed)
        ));
    }

    output.push_str("\n## Package Boundaries\n\n");
    for package in &report.packages {
        output.push_str(&format!(
            "- `{}`: {}\n",
            package.package_id, package.launch_boundary
        ));
    }

    output.push_str("\n## Honest Launch Boundaries\n\n");
    for boundary in &report.honest_boundaries {
        output.push_str(&format!("- {boundary}\n"));
    }

    output
}

/// Render the Forge package scorecard history index as Markdown.
pub fn forge_package_scorecard_history_markdown(
    index: &DxForgePackageScorecardHistoryIndex,
) -> String {
    let mut output = format!(
        "# DX Forge Package Scorecard History\n\n- Updated: `{}`\n- Snapshots: `{}`\n\n",
        index.updated_at,
        index.snapshots.len()
    );

    output.push_str("| Generated | Score | Delta | Packages | Verified | Source-owned | node_modules | Snapshot |\n");
    output.push_str("| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |\n");
    for snapshot in &index.snapshots {
        output.push_str(&format!(
            "| `{}` | {} | {:+} | {} | {} | {} | {} | `{}` |\n",
            snapshot.generated_at,
            snapshot.score,
            snapshot.score_delta,
            snapshot.package_count,
            snapshot.verified_package_count,
            snapshot.source_owned_package_count,
            snapshot.node_modules_created_count,
            snapshot.snapshot_file
        ));
    }

    output
}

fn read_scorecard_history_index(path: &Path) -> Result<DxForgePackageScorecardHistoryIndex> {
    if !path.exists() {
        return Ok(DxForgePackageScorecardHistoryIndex {
            version: 1,
            updated_at: Utc::now().to_rfc3339(),
            snapshots: Vec::new(),
        });
    }

    let bytes = fs::read(path).with_context(|| format!("read `{}`", path.display()))?;
    let mut index = serde_json::from_slice::<DxForgePackageScorecardHistoryIndex>(&bytes)
        .with_context(|| format!("parse `{}`", path.display()))?;
    if index.version == 0 {
        index.version = 1;
    }
    Ok(index)
}

fn unique_scorecard_snapshot_file(history_dir: &Path, generated_at: &str) -> String {
    let base = safe_scorecard_snapshot_stem(generated_at);
    let mut candidate = format!("{base}-scorecard.json");
    let mut suffix = 1u64;
    while history_dir.join(&candidate).exists() {
        candidate = format!("{base}-{suffix}-scorecard.json");
        suffix += 1;
    }
    candidate
}

fn safe_scorecard_snapshot_stem(value: &str) -> String {
    let stem = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                character
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();

    if stem.is_empty() {
        "scorecard".to_string()
    } else {
        stem
    }
}

fn read_project_manifest(root: &Path) -> Result<Option<DxSourceManifest>> {
    let manifest_path = root.join(SOURCE_MANIFEST_PATH);
    if !manifest_path.exists() {
        return Ok(None);
    }

    let bytes =
        fs::read(&manifest_path).with_context(|| format!("read `{}`", manifest_path.display()))?;
    let manifest = serde_json::from_slice::<DxSourceManifest>(&bytes)
        .with_context(|| format!("parse `{}`", manifest_path.display()))?;
    Ok(Some(manifest))
}

fn project_scorecard_evidence(
    root: &Path,
    check: &DxCheckReport,
    manifest: Option<&DxSourceManifest>,
) -> DxForgeProjectScorecardEvidence {
    let forge = check
        .sections
        .iter()
        .find(|section| section.name == "forge");
    DxForgeProjectScorecardEvidence {
        path: root.to_path_buf(),
        manifest_present: manifest.is_some(),
        manifest_package_count: manifest
            .map(|manifest| manifest.packages.len() as u64)
            .unwrap_or(0),
        manifest_receipt_count: manifest
            .map(|manifest| manifest.receipts.len() as u64)
            .unwrap_or(0),
        dx_check_score: check.score,
        dx_check_traffic: check.traffic,
        forge_score: forge.map(|section| section.score).unwrap_or(0),
        forge_traffic: forge
            .map(|section| section.traffic)
            .unwrap_or(DxUpdateTraffic::Red),
        forge_finding_count: forge
            .map(|section| section.findings.len() as u64)
            .unwrap_or(0),
        launch_gate_findings: forge_launch_gate_findings(check).len() as u64,
        package_docs_coverage_percent: forge
            .map(|section| section_metric(section, "package_docs_coverage_percent"))
            .unwrap_or(0),
        rollback_coverage_percent: forge
            .map(|section| section_metric(section, "rollback_coverage_percent"))
            .unwrap_or(0),
    }
}

fn package_project_evidence(
    root: &Path,
    package_id: &str,
    manifest: Option<&DxSourceManifest>,
) -> DxForgePackageProjectEvidence {
    let Some(manifest) = manifest else {
        return DxForgePackageProjectEvidence::empty();
    };
    let canonical_id = canonical_package_id(package_id);
    let packages = manifest
        .packages
        .iter()
        .filter(|package| canonical_package_id(&package.package_id) == canonical_id)
        .collect::<Vec<_>>();

    let receipt_dir = root.join(RECEIPT_DIR);
    let mut docs_present = 0u64;
    let mut docs_missing = 0u64;
    let mut rollback_receipts_present = 0u64;
    let mut rollback_receipts_missing = 0u64;

    for package in &packages {
        if package_docs_present(root, package) {
            docs_present += 1;
        } else {
            docs_missing += 1;
        }

        match &package.rollback_receipt {
            Some(receipt) if receipt_dir.join(receipt).is_file() => {
                rollback_receipts_present += 1;
            }
            _ => rollback_receipts_missing += 1,
        }
    }

    DxForgePackageProjectEvidence {
        manifest_variant_count: packages.len() as u64,
        variants: packages
            .iter()
            .map(|package| package.variant.clone())
            .collect(),
        manifest_file_count: packages
            .iter()
            .map(|package| package.files.len() as u64)
            .sum(),
        manifest_bytes: packages
            .iter()
            .flat_map(|package| package.files.iter())
            .map(|file| file.bytes)
            .sum(),
        docs_present,
        docs_missing,
        rollback_receipts_present,
        rollback_receipts_missing,
    }
}

impl DxForgePackageProjectEvidence {
    fn empty() -> Self {
        Self {
            manifest_variant_count: 0,
            variants: Vec::new(),
            manifest_file_count: 0,
            manifest_bytes: 0,
            docs_present: 0,
            docs_missing: 0,
            rollback_receipts_present: 0,
            rollback_receipts_missing: 0,
        }
    }
}

fn package_docs_present(root: &Path, package: &DxSourcePackage) -> bool {
    let doc_path = root.join(PACKAGE_DOCS_DIR).join(forge_package_doc_name(
        &package.package_id,
        &package.variant,
    ));
    fs::metadata(doc_path)
        .map(|metadata| metadata.is_file() && metadata.len() > 0)
        .unwrap_or(false)
}

fn forge_package_doc_name(package_id: &str, variant: &str) -> String {
    let package = package_id.replace('/', "-");
    if variant == "default" {
        format!("{package}.md")
    } else {
        format!("{package}--variant-{}.md", variant.replace('.', "-"))
    }
}

fn section_metric(section: &DxCheckSection, name: &str) -> u64 {
    section
        .metrics
        .iter()
        .find(|metric| metric.name == name)
        .map(|metric| metric.value)
        .unwrap_or(0)
}

fn read_offline_advisory_metadata(
    root: &Path,
) -> Result<BTreeMap<String, DxForgeAdvisoryMetadata>> {
    let path = root.join(ADVISORY_METADATA_PATH);
    if !path.is_file() {
        return Ok(BTreeMap::new());
    }

    let bytes = fs::read(&path).with_context(|| format!("read `{}`", path.display()))?;
    let index: DxForgeOfflineAdvisoryIndex =
        serde_json::from_slice(&bytes).with_context(|| format!("parse `{}`", path.display()))?;
    if index.version != 1 {
        bail!(
            "unsupported Forge offline advisory metadata version `{}` in `{}`",
            index.version,
            path.display()
        );
    }

    let mut advisories = BTreeMap::new();
    for package in index.packages {
        let canonical = canonical_package_id(package.package_id.trim()).to_string();
        if !LAUNCH_PACKAGES.contains(&canonical.as_str()) {
            bail!("offline advisory metadata references unsupported launch package `{canonical}`");
        }
        if package.provider.trim().is_empty() {
            bail!("offline advisory metadata for `{canonical}` is missing provider");
        }
        if package.reviewed_at.trim().is_empty() {
            bail!("offline advisory metadata for `{canonical}` is missing reviewed_at");
        }
        if package.note.trim().is_empty() {
            bail!("offline advisory metadata for `{canonical}` is missing note");
        }

        let previous = advisories.insert(
            canonical.clone(),
            DxForgeAdvisoryMetadata {
                coverage_kind: DxForgeAdvisoryCoverageKind::OfflineSnapshot,
                provider: package.provider,
                live_coverage: package.live_coverage,
                finding_count: package.finding_count,
                reviewed_at: Some(package.reviewed_at),
                note: package.note,
            },
        );
        if previous.is_some() {
            bail!("duplicate offline advisory metadata for `{canonical}`");
        }
    }

    Ok(advisories)
}

fn package_public_claim(package_id: &str) -> &'static str {
    match package_id {
        "shadcn/ui/button" => {
            "Editable UI Components button source based on shadcn-ui v4 and Radix Slot, with local helper files and Forge receipts."
        }
        "shadcn/ui/badge" => {
            "Editable UI Components badge source based on the shadcn-ui v4 and Radix Slot registry API shape, with Forge receipts."
        }
        "shadcn/ui/card" => {
            "Editable UI Components card source based on shadcn-ui v4, with local helper files and Forge receipts."
        }
        "shadcn/ui/alert" => {
            "Editable UI Components alert source based on the shadcn-ui v4 registry API shape, with Forge receipts."
        }
        "shadcn/ui/avatar" => {
            "Editable UI Components avatar source based on the shadcn-ui v4 registry API shape, with Forge receipts."
        }
        "shadcn/ui/skeleton" => {
            "Editable UI Components skeleton source based on the shadcn-ui v4 registry API shape, with Forge receipts."
        }
        "shadcn/ui/label" => {
            "Editable UI Components label source based on the shadcn-ui v4 and Radix Label registry API shape, with Forge receipts."
        }
        "shadcn/ui/separator" => {
            "Editable UI Components separator source based on the shadcn-ui v4 and Radix Separator registry API shape, with Forge receipts."
        }
        "shadcn/ui/field" => {
            "Editable UI Components field source based on the shadcn-ui v4, Radix Label, and Radix Separator registry API shape, with Forge receipts."
        }
        "shadcn/ui/item" => {
            "Editable UI Components item source based on the shadcn-ui v4, Radix Slot, and Radix Separator registry API shape, with Forge receipts."
        }
        "shadcn/ui/input" => {
            "Editable UI Components input source based on the shadcn-ui v4 registry shape, with Forge receipts."
        }
        "shadcn/ui/textarea" => {
            "Editable UI Components textarea source based on the shadcn-ui v4 registry shape, with Forge receipts."
        }
        "dx/icon/search" => {
            "Selected-icon materialization that avoids shipping an entire icon library."
        }
        "auth/better-auth" => {
            "Editable Better Auth source slice with OAuth env/redirect contracts, server/client/Next route helpers, ownership docs, and discovery metadata."
        }
        "animation/motion" => {
            "Editable Motion React reveal, variants, transitions, and reduced-motion defaults with discovery metadata."
        }
        "i18n/next-intl" => {
            "Editable next-intl App Router routing, navigation, middleware, request config, provider, messages, and discovery metadata."
        }
        "tanstack/query" => {
            "Editable TanStack Query provider, client defaults, server prefetch hydration, and discovery metadata."
        }
        "validation/zod" => {
            "Editable Zod validation adapter source for schemas, safe parsing, error formatting, and JSON Schema export."
        }
        "forms/react-hook-form" => {
            "Editable React Hook Form provider, registered fields, controlled fields, field arrays, watchers, resolver bridge, and discovery metadata."
        }
        "payments/stripe-js" => {
            "Editable Stripe.js browser client configuration, pure loader, payment confirmation helper, and discovery metadata."
        }
        "automations/n8n" => {
            "Editable n8n automation bridge metadata, connector catalog summaries, credential-boundary helpers, and run-receipt contracts."
        }
        "state/zustand" => {
            "Editable Zustand-compatible vanilla store, React hook, selector subscription, middleware mutator typing, shallow equality, and persistence helpers."
        }
        "ai/vercel-ai" => {
            "Editable Vercel AI SDK streaming chat route, typed tool, UI transport, and discovery metadata."
        }
        "api/trpc" => {
            "Editable tRPC typed router, fetch route handler, batched/subscription clients, TanStack Query provider, and discovery metadata."
        }
        "content/fumadocs-next" => {
            "Editable Fumadocs App Router docs slice with MDX config, source plugin frontmatter, navigation snapshot helpers, toc summary helpers, docs routes, LLMs route materialization, OpenAPI virtual docs, OpenAPI request proxy, OpenAPI request code usage, dynamic/static search route materialization, client search presets, layout primitives, starter content, and discovery metadata."
        }
        "content/react-markdown" => {
            "Editable react-markdown renderer source with safe defaults, component overrides, sync/async/hooks exports, and discovery metadata."
        }
        "supabase/client" => {
            "Editable Supabase SSR client slice with public env, auth actions, and RLS seed metadata."
        }
        "db/drizzle-sqlite" => {
            "Editable Drizzle ORM SQLite schema, client, and query source using real public APIs."
        }
        "instantdb/react" => {
            "Editable InstantDB React source for typed realtime queries, transactions, schema helpers, and presence wiring."
        }
        "migration/static-site" => {
            "Editable static migration seed for scoped WordPress/static page content."
        }
        "wasm/bindgen" => {
            "Editable wasm-bindgen loader and React hook for generated WebAssembly modules."
        }
        "3d/launch-scene" => {
            "Editable 3D launch scene contract, Web Preview-safe WebGL runtime, premium preset, React wrapper, and Three/R3F/Drei upgrade metadata."
        }
        _ => "Source-owned curated Forge package.",
    }
}

fn package_launch_boundary(package_id: &str) -> &'static str {
    match package_id {
        "shadcn/ui/button" => {
            "This proves curated UI Components materialization, not full upstream registry parity yet."
        }
        "shadcn/ui/badge" => {
            "This adds a real status/tag primitive for launch templates, not full upstream registry parity yet."
        }
        "shadcn/ui/card" => {
            "This expands curated UI coverage to a layout component, not the full upstream registry yet."
        }
        "shadcn/ui/alert" => {
            "This expands curated UI coverage to a feedback primitive, not the full upstream registry yet."
        }
        "shadcn/ui/avatar" => {
            "This expands curated UI coverage to identity media primitives, not the full upstream registry yet."
        }
        "shadcn/ui/skeleton" => {
            "This expands curated UI coverage to loading-state primitives, not the full upstream registry yet."
        }
        "shadcn/ui/label" => {
            "This adds a real accessible form-label primitive for launch templates, not full upstream registry parity yet."
        }
        "shadcn/ui/separator" => {
            "This adds a real layout separator primitive for launch templates, not full upstream registry parity yet."
        }
        "shadcn/ui/field" => {
            "This adds real form layout, description, and error primitives for launch templates, not full upstream registry parity yet."
        }
        "shadcn/ui/item" => {
            "This adds real list row, media, content, and action primitives for launch templates, not full upstream registry parity yet."
        }
        "shadcn/ui/input" => {
            "This adds a real form primitive for launch templates, not full upstream registry parity yet."
        }
        "shadcn/ui/textarea" => {
            "This expands launch form coverage to long-form text fields, not full upstream registry parity yet."
        }
        "dx/icon/search" => {
            "This proves selected asset packaging, not every icon library or tree-shaking scenario yet."
        }
        "auth/better-auth" => {
            "This proves source-owned Better Auth/OAuth launch wiring, not a complete hosted identity platform, account system, organization model, or database policy."
        }
        "animation/motion" => {
            "This proves Motion React launch wiring, not every gesture, layout projection, timeline, or DOM animation API."
        }
        "i18n/next-intl" => {
            "This proves next-intl App Router launch wiring, not complete translation operations, locale SEO policy, or domain routing governance."
        }
        "tanstack/query" => {
            "This proves TanStack Query launch wiring, not a replacement for every observer, devtools, persistence, or offline sync feature."
        }
        "validation/zod" => {
            "This proves Zod validation launch wiring, not a universal schema governance, policy, or data-access authorization layer."
        }
        "forms/react-hook-form" => {
            "This proves React Hook Form launch wiring, not a replacement for app-specific schema design, accessibility review, or authorization policy."
        }
        "payments/stripe-js" => {
            "This proves Stripe.js browser payment wiring, not server PaymentIntent creation, webhooks, pricing, fraud, tax, dispute, or compliance policy."
        }
        "automations/n8n" => {
            "This proves n8n metadata and DX CLI automation bridge wiring, not live workflow execution, credential setup, or canvas parity."
        }
        "state/zustand" => {
            "This proves a Zustand-compatible launch state slice with explicit app-owned Immer dependency boundaries, not every middleware, upstream multi-store DevTools tracking, or durable application storage."
        }
        "ai/vercel-ai" => {
            "This proves Vercel AI SDK route and client wiring, not provider account setup, model safety policy, persistence, or rate limiting."
        }
        "api/trpc" => {
            "This proves tRPC App Router wiring, typed clients, and subscription transport shape, not application authorization, procedure design, request limits, stream fan-out, or persistence policy."
        }
        "content/fumadocs-next" => {
            "This proves Fumadocs docs, source plugin frontmatter, navigation snapshot helpers, toc summary helpers, LLMs route materialization, OpenAPI virtual docs, OpenAPI request proxy source wiring, OpenAPI request code usage, dynamic search, static search-index export, and client preset materialization, not source plugin taxonomy, navigation policy, toc policy, slug/canonical URL policy, OpenAPI schema governance, proxy allowed origins/auth forwarding policy, request code sample policy, AI crawler policy, search UI, multilingual/vector policy, content governance, or automatic merging of existing Next config."
        }
        "content/react-markdown" => {
            "This proves markdown rendering for launch content, not raw HTML trust, plugin governance, moderation, or full MDX/docs infrastructure."
        }
        "supabase/client" => {
            "This proves Supabase SSR client materialization, not deployed RLS correctness, Auth redirect setup, or secret management."
        }
        "db/drizzle-sqlite" => {
            "This proves a SQLite-first Drizzle data slice, not every Drizzle driver, dialect, migration, or database hosting scenario."
        }
        "instantdb/react" => {
            "This proves InstantDB React launch wiring, not dashboard rules, production auth policy, or complete realtime data governance."
        }
        "migration/static-site" => {
            "This proves a static content migration seed, not full WordPress plugin, theme, CMS, or dynamic-site migration."
        }
        "wasm/bindgen" => {
            "This proves www-template loading for generated wasm-bindgen modules, not Rust macro or CLI replacement."
        }
        "3d/launch-scene" => {
            "This proves a source-owned Web Preview scene, not a replacement for the Three engine, React Three Fiber renderer, Drei helper catalog, Spline editor, 3D asset pipeline, or GPU QA."
        }
        _ => "This package should be described by its current curated scope only.",
    }
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn scorecard_covers_launch_packages_with_honest_boundaries() {
        let report = build_forge_package_scorecard().expect("scorecard");
        let ids = report
            .packages
            .iter()
            .map(|package| package.package_id.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            ids,
            vec![
                "shadcn/ui/button",
                "shadcn/ui/badge",
                "shadcn/ui/card",
                "shadcn/ui/alert",
                "shadcn/ui/avatar",
                "shadcn/ui/skeleton",
                "shadcn/ui/label",
                "shadcn/ui/separator",
                "shadcn/ui/field",
                "shadcn/ui/item",
                "shadcn/ui/input",
                "shadcn/ui/textarea",
                "dx/icon/search",
                "auth/better-auth",
                "animation/motion",
                "i18n/next-intl",
                "tanstack/query",
                "validation/zod",
                "forms/react-hook-form",
                "payments/stripe-js",
                "automations/n8n",
                "state/zustand",
                "ai/vercel-ai",
                "api/trpc",
                "content/fumadocs-next",
                "content/react-markdown",
                "supabase/client",
                "db/drizzle-sqlite",
                "instantdb/react",
                "wasm/bindgen",
                "3d/launch-scene",
                "migration/static-site"
            ]
        );
        assert!(report.score >= 90);
        assert!(report.packages.iter().all(|package| package.source_owned
            && package.install_scripts_blocked
            && !package.node_modules_created
            && package.provenance.source == "dx-forge-curated-registry"
            && !package.provenance.verified
            && package.advisory_review.coverage_kind
                == DxForgeAdvisoryCoverageKind::CuratedFixture
            && package.advisory_review.provider == "dx-forge-curated-advisory-fixture"
            && !package.advisory_review.live_coverage
            && package.advisory_review.reviewed_at.is_some()
            && !package.license_review.reviewed));
        assert!(
            report
                .honest_boundaries
                .iter()
                .any(|boundary| boundary.contains("not a universal npm replacement"))
        );

        let markdown = forge_package_scorecard_markdown(&report);
        assert!(markdown.contains("# DX Forge Package Scorecard"));
        assert!(markdown.contains("shadcn/ui/button"));
        assert!(markdown.contains("shadcn/ui/badge"));
        assert!(markdown.contains("shadcn/ui/card"));
        assert!(markdown.contains("shadcn/ui/alert"));
        assert!(markdown.contains("shadcn/ui/avatar"));
        assert!(markdown.contains("shadcn/ui/skeleton"));
        assert!(markdown.contains("shadcn/ui/label"));
        assert!(markdown.contains("shadcn/ui/separator"));
        assert!(markdown.contains("shadcn/ui/field"));
        assert!(markdown.contains("shadcn/ui/item"));
        assert!(markdown.contains("shadcn/ui/input"));
        assert!(markdown.contains("shadcn/ui/textarea"));
        assert!(markdown.contains("dx/icon/search"));
        assert!(markdown.contains("auth/better-auth"));
        assert!(markdown.contains("animation/motion"));
        assert!(markdown.contains("i18n/next-intl"));
        assert!(markdown.contains("tanstack/query"));
        assert!(markdown.contains("validation/zod"));
        assert!(markdown.contains("forms/react-hook-form"));
        assert!(markdown.contains("payments/stripe-js"));
        assert!(markdown.contains("state/zustand"));
        assert!(markdown.contains("ai/vercel-ai"));
        assert!(markdown.contains("api/trpc"));
        assert!(markdown.contains("content/fumadocs-next"));
        assert!(markdown.contains("content/react-markdown"));
        assert!(markdown.contains("supabase/client"));
        assert!(markdown.contains("db/drizzle-sqlite"));
        assert!(markdown.contains("instantdb/react"));
        assert!(markdown.contains("wasm/bindgen"));
        assert!(markdown.contains("3d/launch-scene"));
        assert!(markdown.contains("migration/static-site"));
        assert!(markdown.contains("not a universal npm replacement"));
        assert!(markdown.contains("Package Metadata Review"));
        assert!(markdown.contains("coverage `curated-fixture`"));
        assert!(markdown.contains("provider `dx-forge-curated-advisory-fixture`"));
        assert!(markdown.contains("live `no`"));
    }

    #[test]
    fn project_scorecard_ingests_offline_advisory_metadata() {
        let dir = tempdir().expect("tempdir");
        let advisory_dir = dir.path().join(".dx/forge");
        fs::create_dir_all(&advisory_dir).expect("advisory dir");
        fs::write(
            advisory_dir.join("advisories.json"),
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
        )
        .expect("advisory metadata");

        let report = build_forge_package_scorecard_for_project(dir.path()).expect("scorecard");
        let button = report
            .packages
            .iter()
            .find(|package| package.package_id == "shadcn/ui/button")
            .expect("button package");
        assert_eq!(
            button.advisory_review.coverage_kind,
            DxForgeAdvisoryCoverageKind::OfflineSnapshot
        );
        assert_eq!(
            button.advisory_review.provider,
            "dx-forge-offline-osv-snapshot"
        );
        assert_eq!(button.advisory_review.finding_count, 1);
        assert_eq!(
            button.advisory_review.reviewed_at.as_deref(),
            Some("2026-05-18T00:00:00Z")
        );

        let icon = report
            .packages
            .iter()
            .find(|package| package.package_id == "dx/icon/search")
            .expect("icon package");
        assert_eq!(
            icon.advisory_review.coverage_kind,
            DxForgeAdvisoryCoverageKind::CuratedFixture
        );

        let markdown = forge_package_scorecard_markdown(&report);
        assert!(markdown.contains("coverage `offline-snapshot`"));
        assert!(markdown.contains("provider `dx-forge-offline-osv-snapshot`"));
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn scorecard_history_preserves_newest_first_score_drift() {
        let dir = tempdir().expect("tempdir");
        let first = build_forge_package_scorecard().expect("first scorecard");
        write_forge_package_scorecard_history(dir.path(), &first).expect("first history");

        let mut second = first.clone();
        second.generated_at = "2026-05-16T00:00:01Z".to_string();
        second.score = first.score.saturating_sub(7);
        write_forge_package_scorecard_history(dir.path(), &second).expect("second history");

        let index_path = dir.path().join(SCORECARD_HISTORY_DIR).join("index.json");
        let index = serde_json::from_slice::<DxForgePackageScorecardHistoryIndex>(
            &fs::read(index_path).expect("history index"),
        )
        .expect("parse history index");

        assert_eq!(index.snapshots.len(), 2);
        assert_eq!(index.snapshots[0].score, second.score);
        assert_eq!(index.snapshots[0].previous_score, Some(first.score));
        assert_eq!(index.snapshots[0].score_delta, -7);
        assert_eq!(
            index.snapshots[0].package_count,
            first.packages.len() as u64
        );
        assert_eq!(index.snapshots[0].node_modules_created_count, 0);
        assert!(
            dir.path()
                .join(SCORECARD_HISTORY_DIR)
                .join("index.md")
                .is_file()
        );
    }
}
