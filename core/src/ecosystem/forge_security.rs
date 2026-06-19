//! Source-owned package governance for DX Forge.

use anyhow::{Context, Result, bail};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Component;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

use super::DxForgeFileTransaction;
use super::DxForgeImportEcosystem;
use super::forge_registry::{
    canonical_package_id, default_source_package, default_source_package_variant,
    load_local_registry_package, source_package_for_project_variant,
    source_package_from_local_registry_selected_exports, validate_source_variant,
};
#[cfg(test)]
use super::forge_registry::{
    publish_root_dx_package_to_local_registry, source_package_for_project,
};
use super::forge_root_manifest::source_package_from_root_dx_selected_exports;

const SOURCE_MANIFEST_PATH: &str = ".dx/forge/source-manifest.json";
const RECEIPT_DIR: &str = ".dx/forge/receipts";
const PACKAGE_DOCS_DIR: &str = ".dx/forge/docs";
const FORBIDDEN_LIFECYCLE_SCRIPTS: [&str; 5] = [
    "preinstall",
    "install",
    "postinstall",
    "prepare",
    "prepublish",
];

/// Source category for a Forge-owned package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxSourceKind {
    /// A curated DX registry package.
    CuratedRegistry,
    /// A package snapshotted from npm metadata.
    NpmSnapshot,
    /// A package snapshotted from an external ecosystem artifact.
    ExternalSnapshot,
    /// A package sourced from local project files.
    Local,
}

/// Governance traffic-light classification for package changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxUpdateTraffic {
    /// Safe to apply automatically.
    Green,
    /// Requires human review before overwrite.
    Yellow,
    /// Blocked by policy until explicitly resolved.
    Red,
}

impl DxUpdateTraffic {
    /// Lowercase label for terminal and report output.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }
}

/// Severity for package supply-chain findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxSupplyChainSeverity {
    /// Informational finding.
    Info,
    /// Low risk finding.
    Low,
    /// Medium risk finding.
    Medium,
    /// High risk finding.
    High,
    /// Critical risk finding.
    Critical,
}

/// Advisory coverage source for a Forge package metadata record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[derive(Default)]
pub enum DxForgeAdvisoryCoverageKind {
    /// No advisory review metadata is attached.
    #[default]
    Missing,
    /// Curated package fixture records that no live advisory feed is attached yet.
    CuratedFixture,
    /// Offline advisory metadata was ingested from a local snapshot file.
    OfflineSnapshot,
    /// A live advisory provider feed is attached.
    LiveFeed,
}

impl DxForgeAdvisoryCoverageKind {
    /// Lowercase report label.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::CuratedFixture => "curated-fixture",
            Self::OfflineSnapshot => "offline-snapshot",
            Self::LiveFeed => "live-feed",
        }
    }
}

/// One source-owned file belonging to a DX package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxSourceFile {
    /// Project-relative destination path.
    pub path: String,
    /// Original logical Forge path before project path mapping.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_path: Option<String>,
    /// BLAKE3 hash of the file content.
    pub hash: String,
    /// Byte length of the file content.
    pub bytes: u64,
    /// File content used during materialization.
    #[serde(skip, default)]
    pub content: Option<String>,
}

/// Structured provenance metadata for a Forge package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeProvenanceMetadata {
    /// Provenance source or registry that produced this package record.
    pub source: String,
    /// Optional upstream reference, URL, digest, or commit pointer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_reference: Option<String>,
    /// Whether external provenance has been verified.
    pub verified: bool,
    /// Honest note describing what this provenance record does and does not prove.
    pub note: String,
}

impl Default for DxForgeProvenanceMetadata {
    fn default() -> Self {
        Self {
            source: "legacy-or-unknown".to_string(),
            upstream_reference: None,
            verified: false,
            note: "Legacy package metadata did not include structured provenance; DX does not claim SLSA or upstream provenance for this record.".to_string(),
        }
    }
}

/// Advisory coverage metadata for a Forge package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeAdvisoryMetadata {
    /// Advisory coverage source kind.
    #[serde(default)]
    pub coverage_kind: DxForgeAdvisoryCoverageKind,
    /// Advisory provider name, or `none` when no live feed is attached.
    pub provider: String,
    /// Whether this package has live vulnerability advisory coverage.
    pub live_coverage: bool,
    /// Number of advisory findings attached to this package record.
    pub finding_count: u64,
    /// Optional timestamp for the last advisory review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,
    /// Honest note describing advisory coverage boundaries.
    pub note: String,
}

impl Default for DxForgeAdvisoryMetadata {
    fn default() -> Self {
        Self {
            coverage_kind: DxForgeAdvisoryCoverageKind::Missing,
            provider: "none".to_string(),
            live_coverage: false,
            finding_count: 0,
            reviewed_at: None,
            note: "No live advisory coverage is claimed for this package metadata record."
                .to_string(),
        }
    }
}

/// License review metadata for a Forge package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeLicenseReviewMetadata {
    /// Declared package license expression.
    pub declared_license: String,
    /// Whether the declared license has a formal DX license review.
    pub reviewed: bool,
    /// Optional timestamp for the last license review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,
    /// Honest note describing license review boundaries.
    pub note: String,
}

impl Default for DxForgeLicenseReviewMetadata {
    fn default() -> Self {
        Self {
            declared_license: "UNKNOWN".to_string(),
            reviewed: false,
            reviewed_at: None,
            note: "License is a declaration only; no formal DX license review is recorded."
                .to_string(),
        }
    }
}

/// Source-owned package metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxSourcePackage {
    /// Stable DX package id, for example `ui/button`.
    pub package_id: String,
    /// Upstream package or registry name.
    pub upstream_name: String,
    /// Upstream or registry version.
    pub version: String,
    /// Generator that produced or tracked this source-owned package.
    #[serde(default = "default_source_generator")]
    pub generator: String,
    /// Local package variant name.
    #[serde(default = "default_source_variant")]
    pub variant: String,
    /// RFC3339 timestamp for the last accepted update, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_accepted_update: Option<String>,
    /// Receipt filename that can be used as the rollback reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_receipt: Option<String>,
    /// Source category.
    pub source_kind: DxSourceKind,
    /// BLAKE3 hash over path and content hashes.
    pub integrity_hash: String,
    /// License expression.
    pub license: String,
    /// Structured provenance metadata.
    #[serde(default)]
    pub provenance: DxForgeProvenanceMetadata,
    /// Advisory coverage metadata.
    #[serde(default)]
    pub advisory_review: DxForgeAdvisoryMetadata,
    /// License review metadata.
    #[serde(default)]
    pub license_review: DxForgeLicenseReviewMetadata,
    /// Source-owned files.
    pub files: Vec<DxSourceFile>,
}

fn default_source_generator() -> String {
    "dx-forge".to_string()
}

fn default_source_variant() -> String {
    "default".to_string()
}

/// One supply-chain policy finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxSupplyChainFinding {
    /// Finding severity.
    pub severity: DxSupplyChainSeverity,
    /// Stable finding code.
    pub code: String,
    /// Human-readable finding message.
    pub message: String,
    /// Path or manifest field that triggered the finding.
    pub evidence_path: Option<String>,
    /// Recommended remediation.
    pub remediation: String,
}

/// A policy decision recorded into a Forge receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxPolicyDecision {
    /// Stable policy name.
    pub policy: String,
    /// Traffic-light result.
    pub traffic: DxUpdateTraffic,
    /// Human-readable decision.
    pub message: String,
}

/// Per-file accept/reject decision recorded into update receipts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeReceiptFileDecisionKind {
    /// The receipt accepted the current or latest file state.
    Accepted,
    /// The receipt rejected applying or accepting the file state.
    Rejected,
}

/// Reviewable per-file update decision recorded into a Forge receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeReceiptFileDecision {
    /// Project-relative materialized path.
    pub path: String,
    /// Logical registry path when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_path: Option<String>,
    /// Hash that existed before the update action, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before_hash: Option<String>,
    /// Hash that would exist after an accepted update, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_hash: Option<String>,
    /// Manifest hash that Forge used as the tracked base, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracked_hash: Option<String>,
    /// File change kind.
    pub change: DxForgeUpdateChangeKind,
    /// Traffic-light result for this file.
    pub traffic: DxUpdateTraffic,
    /// Whether the update accepted or rejected this file state.
    pub decision: DxForgeReceiptFileDecisionKind,
    /// Human-readable reason for the decision.
    pub message: String,
}

/// Materialized path mapping recorded into a Forge receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeFileMap {
    /// Original logical Forge path.
    pub logical_path: String,
    /// Project-facing materialized path.
    pub materialized_path: String,
    /// BLAKE3 hash of the materialized file content.
    pub hash: String,
    /// Byte length of the materialized file content.
    pub bytes: u64,
}

/// Package action recorded by a Forge receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeAction {
    /// Audit-only action.
    Audit,
    /// Dry-run source package add.
    AddDryRun,
    /// Written source package add.
    AddWrite,
    /// Dry-run tracking of already-owned local source inputs.
    TrackDryRun,
    /// Written tracking receipt for already-owned local source inputs.
    TrackWrite,
    /// Dry-run package update change-set preview.
    UpdateDryRun,
    /// Written package update.
    UpdateWrite,
    /// Dry-run package removal with archive planning.
    RemoveDryRun,
    /// Written package removal after archive backup.
    RemoveWrite,
    /// Dry-run rollback from a prior trusted Forge receipt.
    RollbackDryRun,
    /// Written rollback from a prior trusted Forge receipt.
    RollbackWrite,
    /// Dry-run package docs regeneration.
    DocsDryRun,
    /// Written package docs regeneration.
    DocsWrite,
}

/// One file considered by a Forge rollback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRollbackFile {
    /// Project-relative materialized path.
    pub path: String,
    /// Logical registry path when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_path: Option<String>,
    /// Hash recorded in the rollback receipt.
    pub receipt_hash: String,
    /// Current local file hash when the file exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_hash: Option<String>,
    /// Traffic-light result for this rollback file.
    pub traffic: DxUpdateTraffic,
    /// Whether a write would restore this file.
    pub will_write: bool,
    /// Human-readable rollback note.
    pub message: String,
}

/// Result of a Forge rollback preview or write.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRollbackOutcome {
    /// Project root targeted by the rollback.
    pub project: PathBuf,
    /// Receipt path used as the rollback source.
    pub source_receipt_path: PathBuf,
    /// Canonical package id restored by the rollback.
    pub package_id: String,
    /// Source-owned package variant restored by the rollback.
    pub variant: String,
    /// Strongest traffic-light result across rollback files.
    pub traffic: DxUpdateTraffic,
    /// Risk score for the rollback.
    pub risk_score: u8,
    /// File-level rollback plan.
    pub files: Vec<DxForgeRollbackFile>,
    /// Findings that block or explain rollback risk.
    pub findings: Vec<DxSupplyChainFinding>,
    /// Rollback receipt for review or persistence.
    pub receipt: DxForgeReceipt,
    /// Manifest path when a rollback write recorded project state.
    pub manifest_path: Option<PathBuf>,
    /// Receipt path when a rollback write recorded project state.
    pub receipt_path: Option<PathBuf>,
    /// Whether project files were written.
    pub wrote_files: bool,
}

/// One file considered by a Forge package removal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRemoveFile {
    /// Project-relative materialized path.
    pub path: String,
    /// Logical registry path when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_path: Option<String>,
    /// Hash recorded in the source manifest.
    pub expected_hash: String,
    /// Current local file hash when the file exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_hash: Option<String>,
    /// Traffic-light result for this removal file.
    pub traffic: DxUpdateTraffic,
    /// Whether a write would remove this file.
    pub will_remove: bool,
    /// Archive destination for write mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_path: Option<String>,
    /// Human-readable removal note.
    pub message: String,
}

/// Result of a Forge remove preview or write.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRemoveOutcome {
    /// Project root targeted by the removal.
    pub project: PathBuf,
    /// Canonical package id removed from the project manifest.
    pub package_id: String,
    /// Source-owned package variant removed from the project manifest.
    pub variant: String,
    /// Strongest traffic-light result across removal files.
    pub traffic: DxUpdateTraffic,
    /// Risk score for the removal.
    pub risk_score: u8,
    /// File-level removal plan.
    pub files: Vec<DxForgeRemoveFile>,
    /// Findings that block or explain removal risk.
    pub findings: Vec<DxSupplyChainFinding>,
    /// Remove receipt for review or persistence.
    pub receipt: DxForgeReceipt,
    /// Archive root when a remove write backed up deleted files.
    pub archive_root: Option<PathBuf>,
    /// Manifest path when a remove write recorded project state.
    pub manifest_path: Option<PathBuf>,
    /// Receipt path when a remove write recorded project state.
    pub receipt_path: Option<PathBuf>,
    /// Whether project files were removed.
    pub removed_files: bool,
}

/// Reviewable receipt for a Forge package operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeReceipt {
    /// Action that produced the receipt.
    pub action: DxForgeAction,
    /// Source-owned package involved in the action.
    pub package: DxSourcePackage,
    /// Selected export names requested for root `dx` package installs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selected_exports: Vec<String>,
    /// Files planned or written by this receipt.
    pub files_written: Vec<DxSourceFile>,
    /// Logical-to-materialized generated file map.
    pub file_map: Vec<DxForgeFileMap>,
    /// Policy decisions applied during the action.
    pub policy_decisions: Vec<DxPolicyDecision>,
    /// Per-file update decisions for reviewable update receipts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub update_decisions: Vec<DxForgeReceiptFileDecision>,
    /// Supply-chain risk score after policy evaluation.
    pub risk_score: u8,
    /// RFC3339 timestamp.
    pub timestamp: String,
    /// Optional future signature.
    pub signature: Option<String>,
}

/// Source package manifest stored under `.dx/forge`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxSourceManifest {
    /// Manifest schema version.
    pub version: u32,
    /// Packages currently tracked by Forge.
    pub packages: Vec<DxSourcePackage>,
    /// Receipt filenames recorded for this project.
    pub receipts: Vec<String>,
}

impl Default for DxSourceManifest {
    fn default() -> Self {
        Self {
            version: 1,
            packages: Vec::new(),
            receipts: Vec::new(),
        }
    }
}

/// Full audit report for a project or package directory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeAuditReport {
    /// Audited root path.
    pub path: PathBuf,
    /// Calculated supply-chain score.
    pub risk_score: u8,
    /// Overall traffic-light result.
    pub traffic: DxUpdateTraffic,
    /// Package-level summaries discovered during the audit.
    pub packages: Vec<DxPackageAuditSummary>,
    /// Findings discovered during the audit.
    pub findings: Vec<DxSupplyChainFinding>,
}

/// Package-level audit summary for one discovered `package.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxPackageAuditSummary {
    /// Package name.
    pub package_name: String,
    /// Package version when present.
    pub version: Option<String>,
    /// Path to the package root relative to the audit root.
    pub path: String,
    /// Package-level risk score.
    pub risk_score: u8,
    /// Package traffic-light result.
    pub traffic: DxUpdateTraffic,
    /// Number of findings associated with this package.
    pub finding_count: usize,
}

/// File-level state for one Forge-owned source file in a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeFileState {
    /// Project-relative materialized path.
    pub path: String,
    /// Expected BLAKE3 hash from the source manifest.
    pub expected_hash: String,
    /// Actual BLAKE3 hash when the file exists.
    pub actual_hash: Option<String>,
    /// Traffic-light state for this file.
    pub traffic: DxUpdateTraffic,
}

/// Package-level state for a Forge-owned package in a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgePackageState {
    /// Package id from the Forge source manifest.
    pub package_id: String,
    /// Source-owned package variant.
    #[serde(default = "default_source_variant")]
    pub variant: String,
    /// Strongest traffic-light state across package files.
    pub traffic: DxUpdateTraffic,
    /// Source-owned file states.
    pub files: Vec<DxForgeFileState>,
}

/// Current source-owned package state for a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeSourceStateReport {
    /// Checked project root.
    pub path: PathBuf,
    /// Strongest traffic-light state across all Forge packages.
    pub traffic: DxUpdateTraffic,
    /// Package states.
    pub packages: Vec<DxForgePackageState>,
    /// Findings that explain yellow/red states.
    pub findings: Vec<DxSupplyChainFinding>,
}

/// Per-file change kind for a Forge update preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeUpdateChangeKind {
    /// Current file matches both manifest and latest package.
    Unchanged,
    /// Latest package would add a new tracked file.
    Add,
    /// Latest package would replace a clean tracked file.
    Update,
    /// Local file differs from the current manifest and must be reviewed.
    LocalEdit,
    /// Local file is missing from disk.
    Missing,
    /// Local edit contains a security-sensitive shape and is blocked.
    SecuritySensitiveEdit,
    /// A currently tracked file is no longer in the latest package.
    StaleTrackedFile,
}

/// One file entry in a Forge update change set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeUpdateFileChange {
    /// Project-relative materialized path.
    pub path: String,
    /// Logical registry path when known.
    pub logical_path: Option<String>,
    /// Existing manifest hash when the file was already tracked.
    pub current_manifest_hash: Option<String>,
    /// Actual local file hash when present.
    pub actual_hash: Option<String>,
    /// Latest curated package hash when present.
    pub latest_hash: Option<String>,
    /// Change kind.
    pub change: DxForgeUpdateChangeKind,
    /// Traffic-light classification.
    pub traffic: DxUpdateTraffic,
    /// Human-readable review note.
    pub message: String,
}

/// One human review step for a yellow/red Forge update.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeUpdateReviewStep {
    /// Traffic-light severity for this step.
    pub traffic: DxUpdateTraffic,
    /// Project-relative path when the step applies to one file.
    pub path: Option<String>,
    /// Stable review action name.
    pub action: String,
    /// Human-readable review instruction.
    pub message: String,
}

/// One blocked file in a red Forge update quarantine report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeQuarantineFile {
    /// Project-relative path that stayed untouched.
    pub path: String,
    /// Logical registry path when known.
    pub logical_path: Option<String>,
    /// Red change kind that caused quarantine.
    pub change: DxForgeUpdateChangeKind,
    /// Traffic-light result for this file.
    pub traffic: DxUpdateTraffic,
    /// Human-readable reason the file was quarantined.
    pub reason: String,
    /// Recommended manual resolution.
    pub remediation: String,
    /// Quarantine reports never write or delete files.
    pub would_write: bool,
}

/// Explicit human approval for accepting a yellow Forge update state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxForgeUpdateApproval {
    /// Person or local identity that reviewed the yellow update.
    pub reviewer: String,
    /// Human-readable approval note explaining why the yellow state is accepted.
    pub note: String,
}

/// Dry-run Forge update preview for a source-owned package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeUpdateOutcome {
    /// Project root checked by the update preview.
    pub project: PathBuf,
    /// Canonical package id.
    pub package_id: String,
    /// Source-owned package variant targeted by this update.
    #[serde(default = "default_source_variant")]
    pub variant: String,
    /// Current tracked version from the source manifest.
    pub current_version: String,
    /// Latest curated package version.
    pub latest_version: String,
    /// Strongest traffic-light result across the change set.
    pub traffic: DxUpdateTraffic,
    /// Risk score for the update preview.
    pub risk_score: u8,
    /// Reviewable file changes.
    pub files: Vec<DxForgeUpdateFileChange>,
    /// Findings that explain yellow/red changes.
    pub findings: Vec<DxSupplyChainFinding>,
    /// Explicit review plan for yellow/red updates.
    pub review_plan: Vec<DxForgeUpdateReviewStep>,
    /// Red update files that were quarantined without writes or deletes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub quarantine_report: Vec<DxForgeQuarantineFile>,
    /// Dry-run receipt for review.
    pub receipt: DxForgeReceipt,
    /// Manifest path when an update write recorded project state.
    pub manifest_path: Option<PathBuf>,
    /// Receipt path when an update write recorded project state.
    pub receipt_path: Option<PathBuf>,
    /// Whether project files were written.
    pub wrote_files: bool,
}

/// Result of `dx forge add`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeAddOutcome {
    /// Receipt for the add operation.
    pub receipt: DxForgeReceipt,
    /// Manifest path when a write occurred.
    pub manifest_path: Option<PathBuf>,
    /// Receipt path when a write occurred.
    pub receipt_path: Option<PathBuf>,
    /// Whether project files were written.
    pub wrote_files: bool,
}

/// Result of regenerating package-facing Forge docs from an existing manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeDocsOutcome {
    /// Project root targeted by docs regeneration.
    pub project: PathBuf,
    /// Source manifest used to rebuild docs.
    pub manifest_path: PathBuf,
    /// Per-package docs plans or writes.
    pub packages: Vec<DxForgeDocsPackage>,
    /// Whether docs files were written.
    pub wrote_files: bool,
}

/// One package docs file considered during Forge docs regeneration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeDocsPackage {
    /// Canonical package id.
    pub package_id: String,
    /// Source-owned package variant.
    pub variant: String,
    /// Docs file path.
    pub docs_path: PathBuf,
    /// Whether the docs file existed before the operation.
    pub existed_before: bool,
    /// Whether this docs file was written.
    pub wrote_file: bool,
    /// Number of materialized source files described by the docs.
    pub source_file_count: u64,
}

/// One local source file to track in Forge without materializing dependencies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeLocalSourceFile {
    /// Project-relative path.
    pub path: String,
    /// Current file contents.
    pub content: String,
}

/// Local source-owned package inputs that should receive Forge provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeLocalSourcePackage {
    /// Stable package id, for example `dx-www/vertical/index`.
    pub package_id: String,
    /// Local package variant name.
    #[serde(default = "default_source_variant")]
    pub variant: String,
    /// Local or upstream source label.
    pub upstream_name: String,
    /// Version label for the tracked local source set.
    pub version: String,
    /// License expression.
    pub license: String,
    /// Source-owned local files.
    pub files: Vec<DxForgeLocalSourceFile>,
}

/// Reviewed npm source slice inputs that should receive Forge provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeReviewedNpmSourcePackage {
    /// Stable package id, for example `npm/clsx`.
    pub package_id: String,
    /// Source-owned package variant.
    #[serde(default = "default_source_variant")]
    pub variant: String,
    /// npm package name.
    pub package_name: String,
    /// Reviewed upstream version label.
    pub version: String,
    /// License expression recorded from reviewed metadata.
    pub license: String,
    /// Source-owned files accepted for this reviewed npm slice.
    pub files: Vec<DxForgeLocalSourceFile>,
}

/// External ecosystem source snapshot inputs that should receive Forge provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeExternalSourcePackage {
    /// Stable package id, for example `pip/requests` or `cargo/serde`.
    pub package_id: String,
    /// Source-owned package variant.
    #[serde(default = "default_source_variant")]
    pub variant: String,
    /// Ecosystem label, for example `npm`, `pip`, `cargo`, or `go`.
    pub ecosystem: String,
    /// Upstream package name in that ecosystem.
    pub package_name: String,
    /// Reviewed upstream version label.
    pub version: String,
    /// License expression recorded from inspected metadata.
    pub license: String,
    /// Source-owned files accepted for this external source snapshot.
    pub files: Vec<DxForgeLocalSourceFile>,
}

/// Run a v0 supply-chain audit over a project or package directory.
pub fn audit_supply_chain(path: impl AsRef<Path>) -> Result<DxForgeAuditReport> {
    let root = path.as_ref();
    let mut findings = Vec::new();
    let mut packages = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(should_visit_entry)
        .filter_map(|entry| entry.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let relative = relative_path(root, path);

        if path.file_name().and_then(|value| value.to_str()) == Some("package.json") {
            packages.push(scan_package_json(root, path, &relative, &mut findings)?);
        }

        if path.extension().and_then(|value| value.to_str()) == Some("js") {
            scan_javascript_file(path, &relative, &mut findings)?;
        }
    }

    summarize_package_findings(&mut packages, &findings);
    let traffic = classify_findings(&findings);
    let risk_score = risk_score_from_findings(&findings);

    Ok(DxForgeAuditReport {
        path: root.to_path_buf(),
        risk_score,
        traffic,
        packages,
        findings,
    })
}

/// Return a curated source-owned package.
pub fn curated_source_package(package_id: &str) -> Result<DxSourcePackage> {
    default_source_package(package_id)
}

/// Return a curated source-owned package for a named variant.
pub fn curated_source_package_variant(package_id: &str, variant: &str) -> Result<DxSourcePackage> {
    default_source_package_variant(package_id, variant)
}

/// Create a Forge add receipt without writing project files.
pub fn plan_forge_add(package_id: &str, project: impl AsRef<Path>) -> Result<DxForgeAddOutcome> {
    plan_forge_add_variant(package_id, "default", project)
}

/// Create a Forge add receipt for a named variant without writing project files.
pub fn plan_forge_add_variant(
    package_id: &str,
    variant: &str,
    project: impl AsRef<Path>,
) -> Result<DxForgeAddOutcome> {
    add_forge_package(package_id, variant, project, false)
}

/// Create a Forge add receipt for selected root `dx` package exports.
pub fn plan_forge_add_selected_exports(
    package_id: &str,
    selected_exports: &[String],
    project: impl AsRef<Path>,
) -> Result<DxForgeAddOutcome> {
    add_forge_root_dx_selected_package(package_id, selected_exports, project, false)
}

/// Create a Forge add receipt from a local registry package without writing project files.
pub fn plan_forge_add_from_local_registry(
    package_id: &str,
    version: &str,
    selected_exports: &[String],
    registry_root: impl AsRef<Path>,
    project: impl AsRef<Path>,
) -> Result<DxForgeAddOutcome> {
    add_forge_local_registry_package(
        package_id,
        version,
        selected_exports,
        registry_root,
        project,
        false,
    )
}

/// Materialize a curated Forge package into a project.
pub fn write_forge_add(package_id: &str, project: impl AsRef<Path>) -> Result<DxForgeAddOutcome> {
    write_forge_add_variant(package_id, "default", project)
}

/// Materialize a named Forge package variant into a project.
pub fn write_forge_add_variant(
    package_id: &str,
    variant: &str,
    project: impl AsRef<Path>,
) -> Result<DxForgeAddOutcome> {
    add_forge_package(package_id, variant, project, true)
}

/// Materialize selected root `dx` package exports into a project.
pub fn write_forge_add_selected_exports(
    package_id: &str,
    selected_exports: &[String],
    project: impl AsRef<Path>,
) -> Result<DxForgeAddOutcome> {
    add_forge_root_dx_selected_package(package_id, selected_exports, project, true)
}

/// Materialize selected local registry package exports into a project.
pub fn write_forge_add_from_local_registry(
    package_id: &str,
    version: &str,
    selected_exports: &[String],
    registry_root: impl AsRef<Path>,
    project: impl AsRef<Path>,
) -> Result<DxForgeAddOutcome> {
    add_forge_local_registry_package(
        package_id,
        version,
        selected_exports,
        registry_root,
        project,
        true,
    )
}

/// Preview removing a tracked Forge package without deleting project files.
pub fn plan_forge_remove(
    package_id: &str,
    project: impl AsRef<Path>,
) -> Result<DxForgeRemoveOutcome> {
    plan_forge_remove_variant(package_id, "default", project)
}

/// Preview removing a tracked Forge package variant without deleting project files.
pub fn plan_forge_remove_variant(
    package_id: &str,
    variant: &str,
    project: impl AsRef<Path>,
) -> Result<DxForgeRemoveOutcome> {
    remove_forge_package_variant(package_id, variant, project, false)
}

/// Persist a reviewable remove dry-run receipt without deleting project files.
pub fn write_forge_remove_dry_run_variant(
    package_id: &str,
    variant: &str,
    project: impl AsRef<Path>,
) -> Result<DxForgeRemoveOutcome> {
    let project = project.as_ref();
    let mut outcome = plan_forge_remove_variant(package_id, variant, project)?;
    outcome.receipt_path = Some(write_forge_dry_run_receipt(project, &outcome.receipt)?);
    Ok(outcome)
}

/// Archive and remove a tracked Forge package from a project.
pub fn write_forge_remove(
    package_id: &str,
    project: impl AsRef<Path>,
) -> Result<DxForgeRemoveOutcome> {
    write_forge_remove_variant(package_id, "default", project)
}

/// Archive and remove a tracked Forge package variant from a project.
pub fn write_forge_remove_variant(
    package_id: &str,
    variant: &str,
    project: impl AsRef<Path>,
) -> Result<DxForgeRemoveOutcome> {
    remove_forge_package_variant(package_id, variant, project, true)
}

/// Preview package-facing docs regeneration without rewriting source files.
pub fn plan_forge_docs(project: impl AsRef<Path>) -> Result<DxForgeDocsOutcome> {
    regenerate_forge_docs(project.as_ref(), false)
}

/// Regenerate package-facing docs from the existing Forge source manifest.
pub fn write_forge_docs(project: impl AsRef<Path>) -> Result<DxForgeDocsOutcome> {
    regenerate_forge_docs(project.as_ref(), true)
}

/// Materialize a green-only Forge update into a project.
pub fn write_forge_update(
    package_id: &str,
    project: impl AsRef<Path>,
) -> Result<DxForgeUpdateOutcome> {
    write_forge_update_variant(package_id, "default", project)
}

/// Materialize a green-only Forge update for a named variant into a project.
pub fn write_forge_update_variant(
    package_id: &str,
    variant: &str,
    project: impl AsRef<Path>,
) -> Result<DxForgeUpdateOutcome> {
    write_forge_update_variant_inner(package_id, variant, project, None)
}

/// Materialize a reviewed yellow Forge update for a named variant into a project.
pub fn write_forge_update_reviewed_variant(
    package_id: &str,
    variant: &str,
    project: impl AsRef<Path>,
    approval: DxForgeUpdateApproval,
) -> Result<DxForgeUpdateOutcome> {
    write_forge_update_variant_inner(package_id, variant, project, Some(approval))
}

/// Materialize a reviewed yellow Forge update into a project.
pub fn write_forge_update_reviewed(
    package_id: &str,
    project: impl AsRef<Path>,
    approval: DxForgeUpdateApproval,
) -> Result<DxForgeUpdateOutcome> {
    write_forge_update_reviewed_variant(package_id, "default", project, approval)
}

fn write_forge_update_variant_inner(
    package_id: &str,
    variant: &str,
    project: impl AsRef<Path>,
    approval: Option<DxForgeUpdateApproval>,
) -> Result<DxForgeUpdateOutcome> {
    let project = project.as_ref();
    let mut outcome = plan_forge_update_variant(package_id, variant, project)?;
    let accept_yellow = match outcome.traffic {
        DxUpdateTraffic::Green => false,
        DxUpdateTraffic::Yellow => {
            let approval = approval.as_ref().with_context(|| {
                format!(
                    "Forge update for `{}` variant `{}` is yellow; pass explicit review approval to accept local edits",
                    outcome.package_id, outcome.variant
                )
            })?;
            validate_update_approval(approval)?;
            ensure_yellow_update_can_be_accepted(&outcome)?;
            true
        }
        DxUpdateTraffic::Red if approval.is_some() => {
            bail!(
                "Forge update for `{}` variant `{}` is red and cannot be accepted by yellow review; run a dry-run to inspect the quarantine report",
                outcome.package_id,
                outcome.variant
            );
        }
        DxUpdateTraffic::Red => false,
    };

    if outcome.traffic != DxUpdateTraffic::Green && !accept_yellow {
        bail!(
            "Forge update for `{}` variant `{}` is {}; only green updates can be written without explicit review",
            outcome.package_id,
            outcome.variant,
            outcome.traffic.as_str()
        );
    }

    let latest_package =
        source_package_for_project_variant(&outcome.package_id, project, &outcome.variant)?;
    let planned_paths = outcome
        .files
        .iter()
        .filter(|file| {
            matches!(
                file.change,
                DxForgeUpdateChangeKind::Add | DxForgeUpdateChangeKind::Update
            )
        })
        .map(|file| file.path.as_str())
        .collect::<BTreeSet<_>>();
    let planned_files = latest_package
        .files
        .iter()
        .filter(|file| planned_paths.contains(file.path.as_str()))
        .cloned()
        .collect::<Vec<_>>();

    for file in &planned_files {
        let target = project.join(&file.path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).with_context(|| format!("create `{}`", parent.display()))?;
        }
        let content = file
            .content
            .as_deref()
            .context("curated package file is missing update content")?;
        fs::write(&target, content).with_context(|| format!("write `{}`", target.display()))?;
    }

    let receipt_package = if accept_yellow {
        reviewed_update_package(project, latest_package.without_content(), &outcome.files)?
    } else {
        latest_package.without_content()
    };
    let mut policy_decisions = update_policy_decisions(&outcome.files, true);
    if let Some(approval) = approval.as_ref() {
        policy_decisions.push(DxPolicyDecision {
            policy: "explicit-yellow-review".to_string(),
            traffic: outcome.traffic,
            message: format!(
                "Reviewer `{}` accepted yellow local edits for `{}` variant `{}`: {}",
                approval.reviewer.trim(),
                outcome.package_id,
                outcome.variant,
                approval.note.trim()
            ),
        });
    }
    let receipt = DxForgeReceipt {
        action: DxForgeAction::UpdateWrite,
        file_map: receipt_file_map(&receipt_package),
        package: receipt_package,
        selected_exports: Vec::new(),
        files_written: planned_files
            .iter()
            .map(DxSourceFile::without_content)
            .collect(),
        policy_decisions,
        update_decisions: update_receipt_file_decisions(
            &outcome.files,
            true,
            approval.as_ref().filter(|_| accept_yellow),
        ),
        risk_score: outcome.risk_score,
        timestamp: Utc::now().to_rfc3339(),
        signature: None,
    };
    let persisted =
        write_source_manifest_receipt(project, receipt.clone(), !planned_files.is_empty())?;

    outcome.receipt = receipt;
    outcome.manifest_path = persisted.manifest_path;
    outcome.receipt_path = persisted.receipt_path;
    outcome.wrote_files = !planned_files.is_empty();
    Ok(outcome)
}

fn validate_update_approval(approval: &DxForgeUpdateApproval) -> Result<()> {
    if approval.reviewer.trim().is_empty() {
        bail!("Forge yellow update approval requires a reviewer");
    }
    if approval.note.trim().is_empty() {
        bail!("Forge yellow update approval requires a review note");
    }
    Ok(())
}

fn ensure_yellow_update_can_be_accepted(outcome: &DxForgeUpdateOutcome) -> Result<()> {
    for change in outcome
        .files
        .iter()
        .filter(|change| change.traffic != DxUpdateTraffic::Green)
    {
        let reviewable_yellow = matches!(
            change.change,
            DxForgeUpdateChangeKind::LocalEdit | DxForgeUpdateChangeKind::StaleTrackedFile
        );
        if change.traffic != DxUpdateTraffic::Yellow
            || !reviewable_yellow
            || change.actual_hash.is_none()
        {
            bail!(
                "Forge yellow review can only accept existing local edits or reviewed stale tracked files; `{}` is {:?} traffic {}",
                change.path,
                change.change,
                change.traffic.as_str()
            );
        }
    }
    Ok(())
}

fn reviewed_update_package(
    project: &Path,
    mut package: DxSourcePackage,
    changes: &[DxForgeUpdateFileChange],
) -> Result<DxSourcePackage> {
    for change in changes.iter().filter(|change| {
        matches!(
            change.change,
            DxForgeUpdateChangeKind::LocalEdit | DxForgeUpdateChangeKind::StaleTrackedFile
        ) && change.traffic == DxUpdateTraffic::Yellow
    }) {
        let target = project.join(&change.path);
        let bytes = fs::metadata(&target)
            .with_context(|| format!("metadata `{}`", target.display()))?
            .len();
        let actual_hash = change
            .actual_hash
            .clone()
            .context("reviewed yellow file missing actual hash")?;
        if let Some(file) = package
            .files
            .iter_mut()
            .find(|file| file.path == change.path)
        {
            file.hash = actual_hash;
            file.bytes = bytes;
        } else {
            package.files.push(DxSourceFile {
                path: change.path.clone(),
                logical_path: change.logical_path.clone(),
                hash: actual_hash,
                bytes,
                content: None,
            });
        }
    }
    package.integrity_hash = package_integrity_hash(&package.files);
    Ok(package)
}

/// Preview restoring source-owned files from a prior Forge receipt.
pub fn plan_forge_rollback(
    receipt_path: impl AsRef<Path>,
    project: impl AsRef<Path>,
) -> Result<DxForgeRollbackOutcome> {
    rollback_forge_receipt(receipt_path.as_ref(), project.as_ref(), false)
}

/// Restore source-owned files from a prior Forge receipt.
pub fn write_forge_rollback(
    receipt_path: impl AsRef<Path>,
    project: impl AsRef<Path>,
) -> Result<DxForgeRollbackOutcome> {
    rollback_forge_receipt(receipt_path.as_ref(), project.as_ref(), true)
}

/// Create a Forge receipt for local source inputs without writing project state.
pub fn plan_forge_local_source(input: DxForgeLocalSourcePackage) -> Result<DxForgeAddOutcome> {
    track_forge_local_source(input, None, false)
}

/// Write Forge manifest and receipt state for local source inputs.
pub fn write_forge_local_source(
    input: DxForgeLocalSourcePackage,
    project: impl AsRef<Path>,
) -> Result<DxForgeAddOutcome> {
    track_forge_local_source(input, Some(project.as_ref()), true)
}

/// Write Forge manifest and receipt state for a reviewed npm source slice.
pub fn write_forge_reviewed_npm_source(
    input: DxForgeReviewedNpmSourcePackage,
    project: impl AsRef<Path>,
) -> Result<DxForgeAddOutcome> {
    track_forge_reviewed_npm_source(input, Some(project.as_ref()), true)
}

/// Write Forge manifest and receipt state for an external package source snapshot.
pub fn write_forge_external_source(
    input: DxForgeExternalSourcePackage,
    project: impl AsRef<Path>,
) -> Result<DxForgeAddOutcome> {
    track_forge_external_source(input, Some(project.as_ref()), true)
}

/// Classify findings into the strongest traffic-light result.
pub fn classify_findings(findings: &[DxSupplyChainFinding]) -> DxUpdateTraffic {
    if findings.iter().any(|finding| {
        matches!(
            finding.severity,
            DxSupplyChainSeverity::Critical | DxSupplyChainSeverity::High
        )
    }) {
        return DxUpdateTraffic::Red;
    }

    if findings
        .iter()
        .any(|finding| matches!(finding.severity, DxSupplyChainSeverity::Medium))
    {
        return DxUpdateTraffic::Yellow;
    }

    DxUpdateTraffic::Green
}

/// Calculate a deterministic v0 package risk score.
pub fn risk_score_from_findings(findings: &[DxSupplyChainFinding]) -> u8 {
    let mut score = 100i32;

    for finding in findings {
        score -= match finding.severity {
            DxSupplyChainSeverity::Critical => 60,
            DxSupplyChainSeverity::High => 40,
            DxSupplyChainSeverity::Medium => 15,
            DxSupplyChainSeverity::Low => 5,
            DxSupplyChainSeverity::Info => 0,
        };
    }

    score.clamp(0, 100) as u8
}

/// Classify current Forge-owned source files against `.dx/forge/source-manifest.json`.
pub fn classify_forge_source_state(project: impl AsRef<Path>) -> Result<DxForgeSourceStateReport> {
    let project = project.as_ref();
    let manifest_path = project.join(SOURCE_MANIFEST_PATH);
    let manifest = load_source_manifest(&manifest_path)?;
    let mut findings = Vec::new();
    let mut packages = Vec::new();
    let mut report_traffic = DxUpdateTraffic::Green;

    for package in manifest.packages {
        let mut package_traffic = DxUpdateTraffic::Green;
        let mut files = Vec::new();

        for file in package.files {
            let target = resolve_forge_project_file(project, &file.path)
                .unwrap_or_else(|| project.join(&file.path));
            let (actual_hash, traffic) =
                classify_forge_file(project, &package.package_id, &file, &target, &mut findings)?;
            package_traffic = strongest_traffic(package_traffic, traffic);
            files.push(DxForgeFileState {
                path: file.path,
                expected_hash: file.hash,
                actual_hash,
                traffic,
            });
        }

        report_traffic = strongest_traffic(report_traffic, package_traffic);
        packages.push(DxForgePackageState {
            package_id: package.package_id,
            variant: package.variant,
            traffic: package_traffic,
            files,
        });
    }

    Ok(DxForgeSourceStateReport {
        path: project.to_path_buf(),
        traffic: report_traffic,
        packages,
        findings,
    })
}

fn resolve_forge_project_file(project: &Path, relative: &str) -> Option<PathBuf> {
    let path = Path::new(relative);
    if path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return None;
    }

    let normalized = relative.replace('\\', "/");
    let mut candidates = vec![project.join(path)];

    if let Some(stripped) = normalized.strip_prefix("examples/template/") {
        candidates.push(project.join(Path::new(stripped)));
    }

    if let Some(repo_root) = project.parent().and_then(Path::parent) {
        if normalized.starts_with("examples/template/")
            || normalized.starts_with("tools/")
            || normalized.starts_with("docs/")
            || normalized.starts_with("core/")
            || normalized.starts_with("dx-www/")
            || normalized.starts_with("benchmarks/")
        {
            candidates.push(repo_root.join(Path::new(&normalized)));
        }
    }

    candidates
        .iter()
        .find(|candidate| candidate.is_file())
        .cloned()
        .or_else(|| candidates.into_iter().next())
}

/// Plan a Forge package update without writing project files.
pub fn plan_forge_update(
    package_id: &str,
    project: impl AsRef<Path>,
) -> Result<DxForgeUpdateOutcome> {
    plan_forge_update_variant(package_id, "default", project)
}

/// Plan a Forge package update for a named variant without writing project files.
pub fn plan_forge_update_variant(
    package_id: &str,
    variant: &str,
    project: impl AsRef<Path>,
) -> Result<DxForgeUpdateOutcome> {
    let project = project.as_ref();
    let canonical = canonical_package_id(package_id).to_string();
    let variant = validate_source_variant(variant)?;
    let latest_package = source_package_for_project_variant(&canonical, project, &variant)?;
    let manifest_path = project.join(SOURCE_MANIFEST_PATH);
    let manifest = load_source_manifest(&manifest_path)?;
    let current_package = manifest
        .packages
        .iter()
        .find(|package| {
            canonical_package_id(&package.package_id) == canonical && package.variant == variant
        })
        .cloned()
        .with_context(|| {
            format!(
                "Forge package `{canonical}` variant `{variant}` is not tracked in `{}`; run `dx add {package_id} --variant {variant}` first",
                manifest_path.display()
            )
        })?;

    let mut current_files: BTreeMap<String, DxSourceFile> = current_package
        .files
        .iter()
        .map(|file| (file.path.clone(), file.clone()))
        .collect();
    let mut changes = Vec::new();
    let mut findings = Vec::new();
    let mut planned_files = Vec::new();

    for latest_file in &latest_package.files {
        let current_file = current_files.remove(&latest_file.path);
        match current_file {
            Some(current_file) => {
                let target = project.join(&latest_file.path);
                let (actual_hash, state_traffic) = classify_forge_file(
                    project,
                    &current_package.package_id,
                    &current_file,
                    &target,
                    &mut findings,
                )?;

                if state_traffic != DxUpdateTraffic::Green {
                    let change = match (actual_hash.is_some(), state_traffic) {
                        (false, _) => DxForgeUpdateChangeKind::Missing,
                        (true, DxUpdateTraffic::Red) => {
                            DxForgeUpdateChangeKind::SecuritySensitiveEdit
                        }
                        _ => DxForgeUpdateChangeKind::LocalEdit,
                    };
                    changes.push(DxForgeUpdateFileChange {
                        path: latest_file.path.clone(),
                        logical_path: latest_file.logical_path.clone(),
                        current_manifest_hash: Some(current_file.hash),
                        actual_hash,
                        latest_hash: Some(latest_file.hash.clone()),
                        change,
                        traffic: state_traffic,
                        message: format!(
                            "{} has local state that must be resolved before update.",
                            latest_file.path
                        ),
                    });
                    continue;
                }

                if current_file.hash == latest_file.hash {
                    changes.push(DxForgeUpdateFileChange {
                        path: latest_file.path.clone(),
                        logical_path: latest_file.logical_path.clone(),
                        current_manifest_hash: Some(current_file.hash),
                        actual_hash,
                        latest_hash: Some(latest_file.hash.clone()),
                        change: DxForgeUpdateChangeKind::Unchanged,
                        traffic: DxUpdateTraffic::Green,
                        message: format!(
                            "{} already matches the latest package.",
                            latest_file.path
                        ),
                    });
                } else {
                    planned_files.push(latest_file.clone());
                    changes.push(DxForgeUpdateFileChange {
                        path: latest_file.path.clone(),
                        logical_path: latest_file.logical_path.clone(),
                        current_manifest_hash: Some(current_file.hash),
                        actual_hash,
                        latest_hash: Some(latest_file.hash.clone()),
                        change: DxForgeUpdateChangeKind::Update,
                        traffic: DxUpdateTraffic::Green,
                        message: format!(
                            "{} would be updated from the latest package.",
                            latest_file.path
                        ),
                    });
                }
            }
            None => {
                let target = project.join(&latest_file.path);
                let actual_hash = if target.exists() {
                    Some(hash_bytes(
                        &fs::read(&target)
                            .with_context(|| format!("read `{}`", target.display()))?,
                    ))
                } else {
                    None
                };

                if actual_hash
                    .as_deref()
                    .is_some_and(|actual| actual != latest_file.hash)
                {
                    findings.push(DxSupplyChainFinding {
                        severity: DxSupplyChainSeverity::Medium,
                        code: "forge-update-untracked-path-conflict".to_string(),
                        message: format!(
                            "Latest Forge package `{canonical}` wants to own `{}` but a different local file already exists",
                            latest_file.path
                        ),
                        evidence_path: Some(latest_file.path.clone()),
                        remediation: "Move or review the local file before accepting the update."
                            .to_string(),
                    });
                    changes.push(DxForgeUpdateFileChange {
                        path: latest_file.path.clone(),
                        logical_path: latest_file.logical_path.clone(),
                        current_manifest_hash: None,
                        actual_hash,
                        latest_hash: Some(latest_file.hash.clone()),
                        change: DxForgeUpdateChangeKind::LocalEdit,
                        traffic: DxUpdateTraffic::Yellow,
                        message: format!(
                            "{} exists locally but is not tracked by this Forge package.",
                            latest_file.path
                        ),
                    });
                    continue;
                }

                planned_files.push(latest_file.clone());
                changes.push(DxForgeUpdateFileChange {
                    path: latest_file.path.clone(),
                    logical_path: latest_file.logical_path.clone(),
                    current_manifest_hash: None,
                    actual_hash,
                    latest_hash: Some(latest_file.hash.clone()),
                    change: DxForgeUpdateChangeKind::Add,
                    traffic: DxUpdateTraffic::Green,
                    message: format!("{} would be added to the Forge package.", latest_file.path),
                });
            }
        }
    }

    for current_file in current_files.into_values() {
        findings.push(DxSupplyChainFinding {
            severity: DxSupplyChainSeverity::Medium,
            code: "forge-update-stale-tracked-file".to_string(),
            message: format!(
                "Forge package `{canonical}` still tracks `{}` but the latest package does not include it",
                current_file.path
            ),
            evidence_path: Some(current_file.path.clone()),
            remediation:
                "Review the stale file and remove it only through an explicit Forge update receipt."
                    .to_string(),
        });
        let target = project.join(&current_file.path);
        let actual_hash = if target.exists() {
            Some(hash_bytes(
                &fs::read(&target).with_context(|| format!("read `{}`", target.display()))?,
            ))
        } else {
            None
        };
        changes.push(DxForgeUpdateFileChange {
            path: current_file.path.clone(),
            logical_path: current_file.logical_path.clone(),
            current_manifest_hash: Some(current_file.hash),
            actual_hash,
            latest_hash: None,
            change: DxForgeUpdateChangeKind::StaleTrackedFile,
            traffic: DxUpdateTraffic::Yellow,
            message: format!(
                "{} is tracked locally but absent from the latest package.",
                current_file.path
            ),
        });
    }

    let traffic = changes.iter().fold(DxUpdateTraffic::Green, |acc, change| {
        strongest_traffic(acc, change.traffic)
    });
    let risk_score = risk_score_from_findings(&findings);
    let policy_decisions = update_policy_decisions(&changes, false);
    let quarantine_report = update_quarantine_report(&changes, &findings);
    let receipt_package = latest_package.without_content();
    let receipt = DxForgeReceipt {
        action: DxForgeAction::UpdateDryRun,
        file_map: receipt_file_map(&receipt_package),
        package: receipt_package,
        selected_exports: Vec::new(),
        files_written: planned_files
            .iter()
            .map(DxSourceFile::without_content)
            .collect(),
        policy_decisions,
        update_decisions: update_receipt_file_decisions(&changes, false, None),
        risk_score,
        timestamp: Utc::now().to_rfc3339(),
        signature: None,
    };
    let review_plan = update_review_plan(&canonical, traffic, &changes, &findings);

    Ok(DxForgeUpdateOutcome {
        project: project.to_path_buf(),
        package_id: canonical,
        variant,
        current_version: current_package.version,
        latest_version: latest_package.version,
        traffic,
        risk_score,
        files: changes,
        findings,
        review_plan,
        quarantine_report,
        receipt,
        manifest_path: None,
        receipt_path: None,
        wrote_files: false,
    })
}

/// Persist a reviewable update dry-run receipt without writing project files.
pub fn write_forge_update_dry_run_variant(
    package_id: &str,
    variant: &str,
    project: impl AsRef<Path>,
) -> Result<DxForgeUpdateOutcome> {
    let project = project.as_ref();
    let mut outcome = plan_forge_update_variant(package_id, variant, project)?;
    outcome.receipt_path = Some(write_forge_dry_run_receipt(project, &outcome.receipt)?);
    Ok(outcome)
}

/// Plan a Forge package update from a local filesystem registry without writing project files.
pub fn plan_forge_update_from_local_registry(
    package_id: &str,
    version: &str,
    selected_exports: &[String],
    registry_root: impl AsRef<Path>,
    project: impl AsRef<Path>,
) -> Result<DxForgeUpdateOutcome> {
    let project = project.as_ref();
    let latest_package = source_package_from_local_registry_selected_exports(
        registry_root,
        package_id,
        version,
        selected_exports,
        project,
    )?;
    let canonical = canonical_package_id(package_id).to_string();
    let variant = latest_package.variant.clone();
    plan_forge_update_against_latest(
        &canonical,
        &variant,
        latest_package,
        selected_exports,
        project,
    )
}

/// Persist a local-registry update dry-run receipt without writing project files.
pub fn write_forge_update_dry_run_from_local_registry(
    package_id: &str,
    version: &str,
    selected_exports: &[String],
    registry_root: impl AsRef<Path>,
    project: impl AsRef<Path>,
) -> Result<DxForgeUpdateOutcome> {
    let project = project.as_ref();
    let mut outcome = plan_forge_update_from_local_registry(
        package_id,
        version,
        selected_exports,
        registry_root,
        project,
    )?;
    outcome.receipt_path = Some(write_forge_dry_run_receipt(project, &outcome.receipt)?);
    Ok(outcome)
}

/// Materialize a green-only Forge package update from a local filesystem registry.
pub fn write_forge_update_from_local_registry(
    package_id: &str,
    version: &str,
    selected_exports: &[String],
    registry_root: impl AsRef<Path>,
    project: impl AsRef<Path>,
) -> Result<DxForgeUpdateOutcome> {
    let project = project.as_ref();
    let latest_package = source_package_from_local_registry_selected_exports(
        registry_root,
        package_id,
        version,
        selected_exports,
        project,
    )?;
    write_forge_update_against_latest(latest_package, selected_exports, project, None)
}

fn plan_forge_update_against_latest(
    canonical: &str,
    variant: &str,
    latest_package: DxSourcePackage,
    selected_exports: &[String],
    project: &Path,
) -> Result<DxForgeUpdateOutcome> {
    let manifest_path = project.join(SOURCE_MANIFEST_PATH);
    let manifest = load_source_manifest(&manifest_path)?;
    let current_package = manifest
        .packages
        .iter()
        .find(|package| {
            canonical_package_id(&package.package_id) == canonical && package.variant == variant
        })
        .cloned()
        .with_context(|| {
            format!(
                "Forge package `{canonical}` variant `{variant}` is not tracked in `{}`; run `dx forge add {canonical}` first",
                manifest_path.display()
            )
        })?;

    let mut current_files: BTreeMap<String, DxSourceFile> = current_package
        .files
        .iter()
        .map(|file| (file.path.clone(), file.clone()))
        .collect();
    let mut changes = Vec::new();
    let mut findings = Vec::new();
    let mut planned_files = Vec::new();

    for latest_file in &latest_package.files {
        let current_file = current_files.remove(&latest_file.path);
        match current_file {
            Some(current_file) => {
                let target = project.join(&latest_file.path);
                let (actual_hash, state_traffic) = classify_forge_file(
                    project,
                    &current_package.package_id,
                    &current_file,
                    &target,
                    &mut findings,
                )?;

                if state_traffic != DxUpdateTraffic::Green {
                    let change = match (actual_hash.is_some(), state_traffic) {
                        (false, _) => DxForgeUpdateChangeKind::Missing,
                        (true, DxUpdateTraffic::Red) => {
                            DxForgeUpdateChangeKind::SecuritySensitiveEdit
                        }
                        _ => DxForgeUpdateChangeKind::LocalEdit,
                    };
                    changes.push(DxForgeUpdateFileChange {
                        path: latest_file.path.clone(),
                        logical_path: latest_file.logical_path.clone(),
                        current_manifest_hash: Some(current_file.hash),
                        actual_hash,
                        latest_hash: Some(latest_file.hash.clone()),
                        change,
                        traffic: state_traffic,
                        message: format!(
                            "{} has local state that must be resolved before update.",
                            latest_file.path
                        ),
                    });
                    continue;
                }

                if current_file.hash == latest_file.hash {
                    changes.push(DxForgeUpdateFileChange {
                        path: latest_file.path.clone(),
                        logical_path: latest_file.logical_path.clone(),
                        current_manifest_hash: Some(current_file.hash),
                        actual_hash,
                        latest_hash: Some(latest_file.hash.clone()),
                        change: DxForgeUpdateChangeKind::Unchanged,
                        traffic: DxUpdateTraffic::Green,
                        message: format!(
                            "{} already matches the latest package.",
                            latest_file.path
                        ),
                    });
                } else {
                    planned_files.push(latest_file.clone());
                    changes.push(DxForgeUpdateFileChange {
                        path: latest_file.path.clone(),
                        logical_path: latest_file.logical_path.clone(),
                        current_manifest_hash: Some(current_file.hash),
                        actual_hash,
                        latest_hash: Some(latest_file.hash.clone()),
                        change: DxForgeUpdateChangeKind::Update,
                        traffic: DxUpdateTraffic::Green,
                        message: format!(
                            "{} would be updated from the latest package.",
                            latest_file.path
                        ),
                    });
                }
            }
            None => {
                let target = project.join(&latest_file.path);
                let actual_hash = if target.exists() {
                    Some(hash_bytes(
                        &fs::read(&target)
                            .with_context(|| format!("read `{}`", target.display()))?,
                    ))
                } else {
                    None
                };

                if actual_hash
                    .as_deref()
                    .is_some_and(|actual| actual != latest_file.hash)
                {
                    findings.push(DxSupplyChainFinding {
                        severity: DxSupplyChainSeverity::Medium,
                        code: "forge-update-untracked-path-conflict".to_string(),
                        message: format!(
                            "Latest Forge package `{canonical}` wants to own `{}` but a different local file already exists",
                            latest_file.path
                        ),
                        evidence_path: Some(latest_file.path.clone()),
                        remediation: "Move or review the local file before accepting the update."
                            .to_string(),
                    });
                    changes.push(DxForgeUpdateFileChange {
                        path: latest_file.path.clone(),
                        logical_path: latest_file.logical_path.clone(),
                        current_manifest_hash: None,
                        actual_hash,
                        latest_hash: Some(latest_file.hash.clone()),
                        change: DxForgeUpdateChangeKind::LocalEdit,
                        traffic: DxUpdateTraffic::Yellow,
                        message: format!(
                            "{} exists locally but is not tracked by this Forge package.",
                            latest_file.path
                        ),
                    });
                    continue;
                }

                planned_files.push(latest_file.clone());
                changes.push(DxForgeUpdateFileChange {
                    path: latest_file.path.clone(),
                    logical_path: latest_file.logical_path.clone(),
                    current_manifest_hash: None,
                    actual_hash,
                    latest_hash: Some(latest_file.hash.clone()),
                    change: DxForgeUpdateChangeKind::Add,
                    traffic: DxUpdateTraffic::Green,
                    message: format!("{} would be added to the Forge package.", latest_file.path),
                });
            }
        }
    }

    for current_file in current_files.into_values() {
        findings.push(DxSupplyChainFinding {
            severity: DxSupplyChainSeverity::Medium,
            code: "forge-update-stale-tracked-file".to_string(),
            message: format!(
                "Forge package `{canonical}` still tracks `{}` but the latest package does not include it",
                current_file.path
            ),
            evidence_path: Some(current_file.path.clone()),
            remediation:
                "Review the stale file and remove it only through an explicit Forge update receipt."
                    .to_string(),
        });
        let target = project.join(&current_file.path);
        let actual_hash = if target.exists() {
            Some(hash_bytes(
                &fs::read(&target).with_context(|| format!("read `{}`", target.display()))?,
            ))
        } else {
            None
        };
        changes.push(DxForgeUpdateFileChange {
            path: current_file.path.clone(),
            logical_path: current_file.logical_path.clone(),
            current_manifest_hash: Some(current_file.hash),
            actual_hash,
            latest_hash: None,
            change: DxForgeUpdateChangeKind::StaleTrackedFile,
            traffic: DxUpdateTraffic::Yellow,
            message: format!(
                "{} is tracked locally but absent from the latest package.",
                current_file.path
            ),
        });
    }

    let traffic = changes.iter().fold(DxUpdateTraffic::Green, |acc, change| {
        strongest_traffic(acc, change.traffic)
    });
    let risk_score = risk_score_from_findings(&findings);
    let policy_decisions = update_policy_decisions(&changes, false);
    let quarantine_report = update_quarantine_report(&changes, &findings);
    let receipt_package = latest_package.without_content();
    let receipt = DxForgeReceipt {
        action: DxForgeAction::UpdateDryRun,
        file_map: receipt_file_map(&receipt_package),
        package: receipt_package,
        selected_exports: receipt_selected_exports(selected_exports),
        files_written: planned_files
            .iter()
            .map(DxSourceFile::without_content)
            .collect(),
        policy_decisions,
        update_decisions: update_receipt_file_decisions(&changes, false, None),
        risk_score,
        timestamp: Utc::now().to_rfc3339(),
        signature: None,
    };
    let review_plan = update_review_plan(canonical, traffic, &changes, &findings);

    Ok(DxForgeUpdateOutcome {
        project: project.to_path_buf(),
        package_id: canonical.to_string(),
        variant: variant.to_string(),
        current_version: current_package.version,
        latest_version: latest_package.version,
        traffic,
        risk_score,
        files: changes,
        findings,
        review_plan,
        quarantine_report,
        receipt,
        manifest_path: None,
        receipt_path: None,
        wrote_files: false,
    })
}

fn write_forge_update_against_latest(
    latest_package: DxSourcePackage,
    selected_exports: &[String],
    project: &Path,
    approval: Option<DxForgeUpdateApproval>,
) -> Result<DxForgeUpdateOutcome> {
    let canonical = canonical_package_id(&latest_package.package_id).to_string();
    let variant = latest_package.variant.clone();
    let mut outcome = plan_forge_update_against_latest(
        &canonical,
        &variant,
        latest_package.clone(),
        selected_exports,
        project,
    )?;
    let accept_yellow = match outcome.traffic {
        DxUpdateTraffic::Green => false,
        DxUpdateTraffic::Yellow => {
            let approval = approval.as_ref().with_context(|| {
                format!(
                    "Forge update for `{}` variant `{}` is yellow; pass explicit review approval to accept local edits",
                    outcome.package_id, outcome.variant
                )
            })?;
            validate_update_approval(approval)?;
            ensure_yellow_update_can_be_accepted(&outcome)?;
            true
        }
        DxUpdateTraffic::Red if approval.is_some() => {
            bail!(
                "Forge update for `{}` variant `{}` is red and cannot be accepted by yellow review; run a dry-run to inspect the quarantine report",
                outcome.package_id,
                outcome.variant
            );
        }
        DxUpdateTraffic::Red => false,
    };

    if outcome.traffic != DxUpdateTraffic::Green && !accept_yellow {
        bail!(
            "Forge update for `{}` variant `{}` is {}; only green updates can be written without explicit review",
            outcome.package_id,
            outcome.variant,
            outcome.traffic.as_str()
        );
    }

    let planned_paths = outcome
        .files
        .iter()
        .filter(|file| {
            matches!(
                file.change,
                DxForgeUpdateChangeKind::Add | DxForgeUpdateChangeKind::Update
            )
        })
        .map(|file| file.path.as_str())
        .collect::<BTreeSet<_>>();
    let planned_files = latest_package
        .files
        .iter()
        .filter(|file| planned_paths.contains(file.path.as_str()))
        .cloned()
        .collect::<Vec<_>>();

    for file in &planned_files {
        let target = project.join(&file.path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).with_context(|| format!("create `{}`", parent.display()))?;
        }
        let content = file
            .content
            .as_deref()
            .context("local registry package file is missing update content")?;
        fs::write(&target, content).with_context(|| format!("write `{}`", target.display()))?;
    }

    let receipt_package = if accept_yellow {
        reviewed_update_package(project, latest_package.without_content(), &outcome.files)?
    } else {
        latest_package.without_content()
    };
    let mut policy_decisions = update_policy_decisions(&outcome.files, true);
    if !selected_exports.is_empty() {
        policy_decisions.push(DxPolicyDecision {
            policy: "local-registry-selected-exports".to_string(),
            traffic: DxUpdateTraffic::Green,
            message: format!(
                "Forge update selected exports `{}` from the local registry package.",
                selected_exports.join(",")
            ),
        });
    }
    if let Some(approval) = approval.as_ref() {
        policy_decisions.push(DxPolicyDecision {
            policy: "explicit-yellow-review".to_string(),
            traffic: outcome.traffic,
            message: format!(
                "Reviewer `{}` accepted yellow local edits for `{}` variant `{}`: {}",
                approval.reviewer.trim(),
                outcome.package_id,
                outcome.variant,
                approval.note.trim()
            ),
        });
    }
    let receipt = DxForgeReceipt {
        action: DxForgeAction::UpdateWrite,
        file_map: receipt_file_map(&receipt_package),
        package: receipt_package,
        selected_exports: receipt_selected_exports(selected_exports),
        files_written: planned_files
            .iter()
            .map(DxSourceFile::without_content)
            .collect(),
        policy_decisions,
        update_decisions: update_receipt_file_decisions(
            &outcome.files,
            true,
            approval.as_ref().filter(|_| accept_yellow),
        ),
        risk_score: outcome.risk_score,
        timestamp: Utc::now().to_rfc3339(),
        signature: None,
    };
    let persisted =
        write_source_manifest_receipt(project, receipt.clone(), !planned_files.is_empty())?;

    outcome.receipt = receipt;
    outcome.manifest_path = persisted.manifest_path;
    outcome.receipt_path = persisted.receipt_path;
    outcome.wrote_files = !planned_files.is_empty();
    Ok(outcome)
}

fn add_forge_package(
    package_id: &str,
    variant: &str,
    project: impl AsRef<Path>,
    write: bool,
) -> Result<DxForgeAddOutcome> {
    let project = project.as_ref();
    let package = source_package_for_project_variant(package_id, project, variant)?;
    add_source_package(package, project, write, &[])
}

fn add_forge_root_dx_selected_package(
    package_id: &str,
    selected_exports: &[String],
    project: impl AsRef<Path>,
    write: bool,
) -> Result<DxForgeAddOutcome> {
    let project = project.as_ref();
    let package =
        source_package_from_root_dx_selected_exports(project, package_id, selected_exports)?;
    add_source_package(package, project, write, selected_exports)
}

fn add_forge_local_registry_package(
    package_id: &str,
    version: &str,
    selected_exports: &[String],
    registry_root: impl AsRef<Path>,
    project: impl AsRef<Path>,
    write: bool,
) -> Result<DxForgeAddOutcome> {
    let project = project.as_ref();
    let package = source_package_from_local_registry_selected_exports(
        registry_root,
        package_id,
        version,
        selected_exports,
        project,
    )?;
    add_source_package(package, project, write, selected_exports)
}

fn add_source_package(
    package: DxSourcePackage,
    project: &Path,
    write: bool,
    selected_exports: &[String],
) -> Result<DxForgeAddOutcome> {
    let mut policy_decisions = vec![DxPolicyDecision {
        policy: "no-lifecycle-execution".to_string(),
        traffic: DxUpdateTraffic::Green,
        message: "Forge add does not run npm install, lifecycle hooks, or upstream scripts."
            .to_string(),
    }];
    if !selected_exports.is_empty() {
        policy_decisions.push(DxPolicyDecision {
            policy: "root-dx-selected-exports".to_string(),
            traffic: DxUpdateTraffic::Green,
            message: format!(
                "Forge selected exports `{}` from the root dx package manifest and planned only their front-facing files.",
                selected_exports.join(",")
            ),
        });
    }
    policy_decisions.extend(package_contract_policy_decisions(&package));
    let mut planned_files = Vec::new();

    for file in &package.files {
        let target = project.join(&file.path);
        if target.exists() {
            let existing = fs::read(&target)
                .with_context(|| format!("read existing file `{}`", target.display()))?;
            let existing_hash = hash_bytes(&existing);
            if existing_hash == file.hash {
                policy_decisions.push(DxPolicyDecision {
                    policy: "existing-file".to_string(),
                    traffic: DxUpdateTraffic::Green,
                    message: format!("{} already matches the curated source.", file.path),
                });
            } else {
                policy_decisions.push(DxPolicyDecision {
                    policy: "preserve-local-edit".to_string(),
                    traffic: DxUpdateTraffic::Yellow,
                    message: format!("{} differs locally and was not overwritten.", file.path),
                });
            }
            continue;
        }

        planned_files.push(file.clone());
    }

    if write {
        for file in &planned_files {
            let target = project.join(&file.path);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("create `{}`", parent.display()))?;
            }
            let content = file
                .content
                .as_deref()
                .context("curated package file is missing materialization content")?;
            fs::write(&target, content).with_context(|| format!("write `{}`", target.display()))?;
        }
    }

    let traffic = policy_decisions
        .iter()
        .fold(DxUpdateTraffic::Green, |acc, decision| {
            strongest_traffic(acc, decision.traffic)
        });
    let risk_score = match traffic {
        DxUpdateTraffic::Green => 100,
        DxUpdateTraffic::Yellow => 85,
        DxUpdateTraffic::Red => 40,
    };
    let receipt_package = package.without_content();
    let receipt = DxForgeReceipt {
        action: if write {
            DxForgeAction::AddWrite
        } else {
            DxForgeAction::AddDryRun
        },
        file_map: receipt_file_map(&receipt_package),
        package: receipt_package,
        selected_exports: receipt_selected_exports(selected_exports),
        files_written: planned_files
            .iter()
            .map(DxSourceFile::without_content)
            .collect(),
        policy_decisions,
        update_decisions: Vec::new(),
        risk_score,
        timestamp: Utc::now().to_rfc3339(),
        signature: None,
    };

    if !write {
        return Ok(DxForgeAddOutcome {
            receipt,
            manifest_path: None,
            receipt_path: None,
            wrote_files: false,
        });
    }

    write_source_manifest_receipt(project, receipt, true)
}

fn receipt_selected_exports(selected_exports: &[String]) -> Vec<String> {
    let mut exports = Vec::new();
    for export in selected_exports {
        let export = export.trim();
        if !export.is_empty() && !exports.iter().any(|existing| existing == export) {
            exports.push(export.to_string());
        }
    }
    exports
}

fn remove_forge_package_variant(
    package_id: &str,
    variant: &str,
    project: impl AsRef<Path>,
    write: bool,
) -> Result<DxForgeRemoveOutcome> {
    let project = project.as_ref();
    let canonical = canonical_package_id(package_id).to_string();
    let variant = validate_source_variant(variant)?;
    let manifest_path = project.join(SOURCE_MANIFEST_PATH);
    let manifest = load_source_manifest(&manifest_path)?;
    let package = manifest
        .packages
        .iter()
        .find(|package| {
            canonical_package_id(&package.package_id) == canonical && package.variant == variant
        })
        .cloned()
        .with_context(|| {
            format!(
                "Forge package `{canonical}` variant `{variant}` is not tracked in `{}`; run `dx forge add {package_id}` first",
                manifest_path.display()
            )
        })?;

    let archive_root = if write {
        Some(remove_archive_root(
            project,
            &package.package_id,
            &package.variant,
        ))
    } else {
        None
    };
    let mut findings = Vec::new();
    let mut files = Vec::new();

    for source_file in &package.files {
        validate_project_relative_path(&source_file.path)?;
        let target = project.join(&source_file.path);
        let archive_path = archive_root
            .as_ref()
            .map(|root| root.join(&source_file.path).display().to_string());

        let (actual_hash, traffic, will_remove, message) = if !target.exists() {
            (
                None,
                DxUpdateTraffic::Green,
                false,
                "Already missing; Forge will only remove manifest tracking.".to_string(),
            )
        } else if target.is_dir() {
            findings.push(DxSupplyChainFinding {
                severity: DxSupplyChainSeverity::High,
                code: "forge-remove-non-file-target".to_string(),
                message: format!(
                    "Forge-owned path `{}` for `{}` is a directory, not a file",
                    source_file.path, package.package_id
                ),
                evidence_path: Some(source_file.path.clone()),
                remediation:
                    "Resolve the directory conflict manually before removing the Forge package."
                        .to_string(),
            });
            (
                None,
                DxUpdateTraffic::Red,
                false,
                "Blocked because the target path is a directory.".to_string(),
            )
        } else {
            let bytes =
                fs::read(&target).with_context(|| format!("read `{}`", target.display()))?;
            let actual_hash = hash_bytes(&bytes);
            if actual_hash == source_file.hash {
                (
                    Some(actual_hash),
                    DxUpdateTraffic::Green,
                    true,
                    if write {
                        "Will archive then remove clean Forge-owned file.".to_string()
                    } else {
                        "Would archive then remove clean Forge-owned file.".to_string()
                    },
                )
            } else {
                findings.push(DxSupplyChainFinding {
                    severity: DxSupplyChainSeverity::Medium,
                    code: "forge-remove-local-edit".to_string(),
                    message: format!(
                        "Forge-owned file `{}` from `{}` has local edits and was not removed",
                        source_file.path, package.package_id
                    ),
                    evidence_path: Some(source_file.path.clone()),
                    remediation:
                        "Review, commit, or rollback the local edit before running `dx forge remove --write`."
                            .to_string(),
                });
                (
                    Some(actual_hash),
                    DxUpdateTraffic::Yellow,
                    false,
                    "Blocked because the file has local edits.".to_string(),
                )
            }
        };

        files.push(DxForgeRemoveFile {
            path: source_file.path.clone(),
            logical_path: source_file.logical_path.clone(),
            expected_hash: source_file.hash.clone(),
            actual_hash,
            traffic,
            will_remove,
            archive_path: archive_path.filter(|_| will_remove),
            message,
        });
    }

    let traffic = files
        .iter()
        .fold(classify_findings(&findings), |acc, file| {
            strongest_traffic(acc, file.traffic)
        });
    let risk_score = risk_score_from_findings(&findings);
    let removed_file_paths = files
        .iter()
        .filter(|file| file.will_remove)
        .map(|file| file.path.as_str())
        .collect::<BTreeSet<_>>();
    let receipt_package = package.without_content();
    let receipt = DxForgeReceipt {
        action: if write {
            DxForgeAction::RemoveWrite
        } else {
            DxForgeAction::RemoveDryRun
        },
        file_map: receipt_file_map(&receipt_package),
        package: receipt_package.clone(),
        selected_exports: Vec::new(),
        files_written: package
            .files
            .iter()
            .filter(|file| removed_file_paths.contains(file.path.as_str()))
            .map(DxSourceFile::without_content)
            .collect(),
        policy_decisions: remove_policy_decisions(
            &receipt_package,
            traffic,
            write,
            archive_root.as_deref(),
        ),
        update_decisions: Vec::new(),
        risk_score,
        timestamp: Utc::now().to_rfc3339(),
        signature: None,
    };

    if write && traffic != DxUpdateTraffic::Green {
        bail!(
            "Forge remove for `{}` variant `{}` is {}; local edits are preserved until reviewed",
            package.package_id,
            package.variant,
            traffic.as_str()
        );
    }

    let mut outcome = DxForgeRemoveOutcome {
        project: project.to_path_buf(),
        package_id: package.package_id.clone(),
        variant: package.variant.clone(),
        traffic,
        risk_score,
        files,
        findings,
        receipt,
        archive_root: archive_root.clone(),
        manifest_path: None,
        receipt_path: None,
        removed_files: false,
    };

    if !write {
        return Ok(outcome);
    }

    let archive_root = archive_root.context("remove write missing archive root")?;
    fs::create_dir_all(&archive_root)
        .with_context(|| format!("create `{}`", archive_root.display()))?;

    let removable_paths = outcome
        .files
        .iter()
        .filter(|file| file.will_remove)
        .map(|file| file.path.clone())
        .collect::<Vec<_>>();

    for path in &removable_paths {
        let target = project.join(path);
        if !target.exists() {
            continue;
        }
        let archive_target = archive_root.join(path);
        if let Some(parent) = archive_target.parent() {
            fs::create_dir_all(parent).with_context(|| format!("create `{}`", parent.display()))?;
        }
        fs::copy(&target, &archive_target).with_context(|| {
            format!(
                "archive `{}` to `{}`",
                target.display(),
                archive_target.display()
            )
        })?;
        fs::remove_file(&target).with_context(|| format!("remove_file `{}`", target.display()))?;
    }

    write_remove_archive_manifest(&archive_root, &outcome.receipt, &outcome.files)?;
    let persisted = write_source_manifest_remove_receipt(project, outcome.receipt.clone())?;
    outcome.manifest_path = Some(persisted.manifest_path);
    outcome.receipt_path = Some(persisted.receipt_path);
    outcome.removed_files = !removable_paths.is_empty();
    Ok(outcome)
}

fn track_forge_local_source(
    input: DxForgeLocalSourcePackage,
    project: Option<&Path>,
    write: bool,
) -> Result<DxForgeAddOutcome> {
    let package = local_source_package(input)?;
    let files_written = package
        .files
        .iter()
        .map(DxSourceFile::without_content)
        .collect::<Vec<_>>();
    let receipt_package = package.without_content();
    let receipt = DxForgeReceipt {
        action: if write {
            DxForgeAction::TrackWrite
        } else {
            DxForgeAction::TrackDryRun
        },
        file_map: receipt_file_map(&receipt_package),
        package: receipt_package,
        selected_exports: Vec::new(),
        files_written,
        policy_decisions: vec![
            DxPolicyDecision {
                policy: "source-owned-inputs".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "Forge records existing local source files as editable, reviewable project-owned inputs.".to_string(),
            },
            DxPolicyDecision {
                policy: "no-lifecycle-execution".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "Forge tracking does not run npm install, lifecycle hooks, or upstream scripts.".to_string(),
            },
        ],
        update_decisions: Vec::new(),
        risk_score: 100,
        timestamp: Utc::now().to_rfc3339(),
        signature: None,
    };

    if !write {
        return Ok(DxForgeAddOutcome {
            receipt,
            manifest_path: None,
            receipt_path: None,
            wrote_files: false,
        });
    }

    let project =
        project.context("project path is required when writing Forge local source state")?;
    write_source_manifest_receipt(project, receipt, true)
}

fn track_forge_reviewed_npm_source(
    input: DxForgeReviewedNpmSourcePackage,
    project: Option<&Path>,
    write: bool,
) -> Result<DxForgeAddOutcome> {
    let package = reviewed_npm_source_package(input)?;
    let files_written = package
        .files
        .iter()
        .map(DxSourceFile::without_content)
        .collect::<Vec<_>>();
    let receipt_package = package.without_content();
    let receipt = DxForgeReceipt {
        action: if write {
            DxForgeAction::TrackWrite
        } else {
            DxForgeAction::TrackDryRun
        },
        file_map: receipt_file_map(&receipt_package),
        package: receipt_package,
        selected_exports: Vec::new(),
        files_written,
        policy_decisions: vec![
            DxPolicyDecision {
                policy: "reviewed-npm-source-slice".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "Forge records this npm package as a reviewed source-owned adapter slice, not an installed node_modules dependency.".to_string(),
            },
            DxPolicyDecision {
                policy: "no-package-install".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "Forge did not run npm, pnpm, yarn, bun, or any package-manager install command.".to_string(),
            },
            DxPolicyDecision {
                policy: "no-lifecycle-execution".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "Forge did not run npm lifecycle hooks, postinstall scripts, or upstream generated install artifacts.".to_string(),
            },
        ],
        update_decisions: Vec::new(),
        risk_score: 95,
        timestamp: Utc::now().to_rfc3339(),
        signature: None,
    };

    if !write {
        return Ok(DxForgeAddOutcome {
            receipt,
            manifest_path: None,
            receipt_path: None,
            wrote_files: false,
        });
    }

    let project =
        project.context("project path is required when writing Forge reviewed npm source state")?;
    write_source_manifest_receipt(project, receipt, true)
}

fn track_forge_external_source(
    input: DxForgeExternalSourcePackage,
    project: Option<&Path>,
    write: bool,
) -> Result<DxForgeAddOutcome> {
    let package = external_source_package(input)?;
    let files_written = package
        .files
        .iter()
        .map(DxSourceFile::without_content)
        .collect::<Vec<_>>();
    let receipt_package = package.without_content();
    let receipt = DxForgeReceipt {
        action: if write {
            DxForgeAction::TrackWrite
        } else {
            DxForgeAction::TrackDryRun
        },
        file_map: receipt_file_map(&receipt_package),
        package: receipt_package,
        selected_exports: Vec::new(),
        files_written,
        policy_decisions: vec![
            DxPolicyDecision {
                policy: "external-source-snapshot".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "Forge records this package as a source-owned external ecosystem snapshot, not as a package-manager install.".to_string(),
            },
            DxPolicyDecision {
                policy: "no-package-install".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "Forge did not run npm, pip, cargo, go, or any package-manager install command.".to_string(),
            },
            DxPolicyDecision {
                policy: "no-lifecycle-execution".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "Forge did not run lifecycle hooks, build scripts, setup hooks, or upstream generated install artifacts.".to_string(),
            },
        ],
        update_decisions: Vec::new(),
        risk_score: 90,
        timestamp: Utc::now().to_rfc3339(),
        signature: None,
    };

    if !write {
        return Ok(DxForgeAddOutcome {
            receipt,
            manifest_path: None,
            receipt_path: None,
            wrote_files: false,
        });
    }

    let project =
        project.context("project path is required when writing Forge external source state")?;
    write_source_manifest_receipt(project, receipt, true)
}

fn local_source_package(input: DxForgeLocalSourcePackage) -> Result<DxSourcePackage> {
    validate_package_id(&input.package_id)?;
    let variant = validate_source_variant(&input.variant)?;
    if input.files.is_empty() {
        bail!("local source package must include at least one file");
    }

    let files = input
        .files
        .into_iter()
        .map(|file| {
            validate_project_relative_path(&file.path)?;
            Ok(DxSourceFile {
                path: file.path,
                logical_path: None,
                hash: hash_bytes(file.content.as_bytes()),
                bytes: file.content.len() as u64,
                content: Some(file.content),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let integrity_hash = package_integrity_hash(&files);

    Ok(DxSourcePackage {
        package_id: input.package_id,
        upstream_name: input.upstream_name.clone(),
        version: input.version,
        generator: "dx-forge/local-source".to_string(),
        variant,
        last_accepted_update: None,
        rollback_receipt: None,
        source_kind: DxSourceKind::Local,
        integrity_hash,
        license: input.license.clone(),
        provenance: DxForgeProvenanceMetadata {
            source: "dx-forge-local-source".to_string(),
            upstream_reference: Some(input.upstream_name),
            verified: false,
            note: "Local source was tracked by Forge, but no external upstream provenance is claimed."
                .to_string(),
        },
        advisory_review: DxForgeAdvisoryMetadata {
            coverage_kind: DxForgeAdvisoryCoverageKind::Missing,
            provider: "none".to_string(),
            live_coverage: false,
            finding_count: 0,
            reviewed_at: None,
            note: "Local source packages do not have live advisory coverage attached by Forge yet."
                .to_string(),
        },
        license_review: DxForgeLicenseReviewMetadata {
            declared_license: input.license,
            reviewed: false,
            reviewed_at: None,
            note: "License is recorded from the local package declaration only; no formal DX legal review is claimed."
                .to_string(),
        },
        files,
    })
}

fn external_source_package(input: DxForgeExternalSourcePackage) -> Result<DxSourcePackage> {
    validate_package_id(&input.package_id)?;
    let variant = validate_source_variant(&input.variant)?;
    if input.files.is_empty() {
        bail!("external source package must include at least one file");
    }

    let ecosystem = validate_external_source_ecosystem(&input.ecosystem)?;
    let files = input
        .files
        .into_iter()
        .map(|file| {
            validate_project_relative_path(&file.path)?;
            Ok(DxSourceFile {
                path: file.path,
                logical_path: None,
                hash: hash_bytes(file.content.as_bytes()),
                bytes: file.content.len() as u64,
                content: Some(file.content),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let integrity_hash = package_integrity_hash(&files);
    let upstream_reference = format!("{ecosystem}:{}", input.package_name);

    Ok(DxSourcePackage {
        package_id: input.package_id,
        upstream_name: upstream_reference.clone(),
        version: input.version,
        generator: format!("dx-forge/{ecosystem}-external-source-snapshot"),
        variant,
        last_accepted_update: None,
        rollback_receipt: None,
        source_kind: if ecosystem == "npm" {
            DxSourceKind::NpmSnapshot
        } else {
            DxSourceKind::ExternalSnapshot
        },
        integrity_hash,
        license: input.license.clone(),
        provenance: DxForgeProvenanceMetadata {
            source: format!("dx-forge-{ecosystem}-external-source-snapshot"),
            upstream_reference: Some(upstream_reference),
            verified: false,
            note: "External ecosystem source was materialized into Forge-owned files without running package-manager install or lifecycle commands.".to_string(),
        },
        advisory_review: DxForgeAdvisoryMetadata {
            coverage_kind: DxForgeAdvisoryCoverageKind::Missing,
            provider: "none".to_string(),
            live_coverage: false,
            finding_count: 0,
            reviewed_at: None,
            note: "External source snapshots do not have live advisory coverage attached by Forge yet."
                .to_string(),
        },
        license_review: DxForgeLicenseReviewMetadata {
            declared_license: input.license,
            reviewed: false,
            reviewed_at: None,
            note: "License is recorded from inspected package metadata only; no formal DX legal review is claimed.".to_string(),
        },
        files,
    })
}

fn reviewed_npm_source_package(input: DxForgeReviewedNpmSourcePackage) -> Result<DxSourcePackage> {
    validate_package_id(&input.package_id)?;
    let variant = validate_source_variant(&input.variant)?;
    if input.files.is_empty() {
        bail!("reviewed npm source package must include at least one file");
    }

    let files = input
        .files
        .into_iter()
        .map(|file| {
            validate_project_relative_path(&file.path)?;
            Ok(DxSourceFile {
                path: file.path,
                logical_path: None,
                hash: hash_bytes(file.content.as_bytes()),
                bytes: file.content.len() as u64,
                content: Some(file.content),
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let integrity_hash = package_integrity_hash(&files);
    let upstream_reference = format!("npm:{}", input.package_name);

    Ok(DxSourcePackage {
        package_id: input.package_id,
        upstream_name: upstream_reference.clone(),
        version: input.version,
        generator: "dx-forge/npm-reviewed-source-slice".to_string(),
        variant,
        last_accepted_update: None,
        rollback_receipt: None,
        source_kind: DxSourceKind::NpmSnapshot,
        integrity_hash,
        license: input.license.clone(),
        provenance: DxForgeProvenanceMetadata {
            source: "dx-forge-npm-reviewed-source-slice".to_string(),
            upstream_reference: Some(upstream_reference),
            verified: false,
            note: "Reviewed npm adapter/source slice; Forge did not run npm install or lifecycle scripts.".to_string(),
        },
        advisory_review: DxForgeAdvisoryMetadata {
            coverage_kind: DxForgeAdvisoryCoverageKind::Missing,
            provider: "none".to_string(),
            live_coverage: false,
            finding_count: 0,
            reviewed_at: None,
            note: "Reviewed npm source slices do not have live advisory coverage attached by Forge yet.".to_string(),
        },
        license_review: DxForgeLicenseReviewMetadata {
            declared_license: input.license,
            reviewed: false,
            reviewed_at: None,
            note: "License is recorded from reviewed npm metadata only; no formal DX legal review is claimed.".to_string(),
        },
        files,
    })
}

fn rollback_forge_receipt(
    receipt_path: &Path,
    project: &Path,
    write: bool,
) -> Result<DxForgeRollbackOutcome> {
    let source_receipt: DxForgeReceipt =
        serde_json::from_slice(&fs::read(receipt_path).with_context(|| {
            format!("read Forge rollback receipt `{}`", receipt_path.display())
        })?)
        .with_context(|| format!("parse Forge rollback receipt `{}`", receipt_path.display()))?;
    let package = source_receipt.package.clone();
    let mut findings = Vec::new();
    let mut files = Vec::new();
    let mut restored_files = Vec::new();

    let content_package = match rollback_content_package(project, &package) {
        Ok(package) => Some(package),
        Err(error) => {
            findings.push(DxSupplyChainFinding {
                severity: DxSupplyChainSeverity::High,
                code: "forge-rollback-content-unavailable".to_string(),
                message: format!(
                    "Forge cannot resolve rollback content for `{}` variant `{}`: {error}",
                    package.package_id, package.variant
                ),
                evidence_path: Some(receipt_path.display().to_string()),
                remediation:
                    "Use a receipt whose package content is still available in the DX Forge registry."
                        .to_string(),
            });
            None
        }
    };
    let content_by_path_hash = content_package
        .as_ref()
        .map(|package| {
            package
                .files
                .iter()
                .map(|file| ((file.path.clone(), file.hash.clone()), file.clone()))
                .collect::<BTreeMap<_, _>>()
        })
        .unwrap_or_default();

    for receipt_file in &package.files {
        let target = project.join(&receipt_file.path);
        let current_hash = if target.exists() {
            Some(hash_bytes(
                &fs::read(&target).with_context(|| format!("read `{}`", target.display()))?,
            ))
        } else {
            None
        };
        let content = content_by_path_hash
            .get(&(receipt_file.path.clone(), receipt_file.hash.clone()))
            .cloned();

        let (traffic, will_write, message) = if content.is_none() {
            findings.push(DxSupplyChainFinding {
                severity: DxSupplyChainSeverity::High,
                code: "forge-rollback-file-content-missing".to_string(),
                message: format!(
                    "Rollback receipt expects `{}` hash `{}`, but matching content is unavailable",
                    receipt_file.path, receipt_file.hash
                ),
                evidence_path: Some(receipt_file.path.clone()),
                remediation:
                    "Do not write rollback files unless the registry can provide content matching the receipt hash."
                        .to_string(),
            });
            (
                DxUpdateTraffic::Red,
                false,
                "Blocked because matching rollback content is unavailable.".to_string(),
            )
        } else if current_hash.as_deref() == Some(receipt_file.hash.as_str()) {
            (
                DxUpdateTraffic::Green,
                false,
                "Already matches the rollback receipt.".to_string(),
            )
        } else {
            (
                DxUpdateTraffic::Green,
                true,
                "Will restore content matching the rollback receipt.".to_string(),
            )
        };

        if will_write {
            let mut restored = receipt_file.clone();
            restored.content = content.and_then(|file| file.content);
            restored_files.push(restored);
        }

        files.push(DxForgeRollbackFile {
            path: receipt_file.path.clone(),
            logical_path: receipt_file.logical_path.clone(),
            receipt_hash: receipt_file.hash.clone(),
            current_hash,
            traffic,
            will_write,
            message,
        });
    }

    let traffic = files
        .iter()
        .fold(classify_findings(&findings), |acc, file| {
            strongest_traffic(acc, file.traffic)
        });
    let risk_score = match traffic {
        DxUpdateTraffic::Green => 100,
        DxUpdateTraffic::Yellow => 85,
        DxUpdateTraffic::Red => 40,
    };
    let wrote_files = write && traffic == DxUpdateTraffic::Green && !restored_files.is_empty();
    let receipt = DxForgeReceipt {
        action: if write {
            DxForgeAction::RollbackWrite
        } else {
            DxForgeAction::RollbackDryRun
        },
        file_map: receipt_file_map(&package),
        package: package.clone(),
        selected_exports: Vec::new(),
        files_written: restored_files
            .iter()
            .map(DxSourceFile::without_content)
            .collect(),
        policy_decisions: rollback_policy_decisions(&package, receipt_path, traffic, write),
        update_decisions: Vec::new(),
        risk_score,
        timestamp: Utc::now().to_rfc3339(),
        signature: None,
    };

    if write && traffic != DxUpdateTraffic::Green {
        bail!(
            "Forge rollback for `{}` variant `{}` is {}; only green rollback receipts can be written",
            package.package_id,
            package.variant,
            traffic.as_str()
        );
    }

    if wrote_files {
        for file in &restored_files {
            let target = project.join(&file.path);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("create `{}`", parent.display()))?;
            }
            let content = file
                .content
                .as_deref()
                .context("rollback file is missing restored content")?;
            fs::write(&target, content).with_context(|| format!("write `{}`", target.display()))?;
        }
    }

    if write {
        let persisted = write_source_manifest_receipt(project, receipt.clone(), wrote_files)?;
        return Ok(DxForgeRollbackOutcome {
            project: project.to_path_buf(),
            source_receipt_path: receipt_path.to_path_buf(),
            package_id: package.package_id,
            variant: package.variant,
            traffic,
            risk_score,
            files,
            findings,
            receipt,
            manifest_path: persisted.manifest_path,
            receipt_path: persisted.receipt_path,
            wrote_files,
        });
    }

    Ok(DxForgeRollbackOutcome {
        project: project.to_path_buf(),
        source_receipt_path: receipt_path.to_path_buf(),
        package_id: package.package_id,
        variant: package.variant,
        traffic,
        risk_score,
        files,
        findings,
        receipt,
        manifest_path: None,
        receipt_path: None,
        wrote_files: false,
    })
}

fn rollback_content_package(
    project: &Path,
    receipt_package: &DxSourcePackage,
) -> Result<DxSourcePackage> {
    let project_package = || {
        source_package_for_project_variant(
            &receipt_package.package_id,
            project,
            &receipt_package.variant,
        )
    };
    let local_registry_package = || local_registry_rollback_package(project, receipt_package);

    if receipt_package
        .generator
        .starts_with("dx-forge/local-registry")
    {
        local_registry_package().or_else(|registry_error| {
            project_package().with_context(|| {
                format!(
                    "local registry error: {registry_error}; project package fallback also failed"
                )
            })
        })
    } else {
        project_package().or_else(|source_error| {
            local_registry_package().with_context(|| {
                format!(
                    "project package error: {source_error}; local registry fallback also failed"
                )
            })
        })
    }
}

fn local_registry_rollback_package(
    project: &Path,
    receipt_package: &DxSourcePackage,
) -> Result<DxSourcePackage> {
    let registry_root = project.join(".dx/forge/registry/local");
    let registry_package = load_local_registry_package(
        &registry_root,
        &receipt_package.package_id,
        &receipt_package.version,
    )?;
    let mut files = Vec::new();

    for receipt_file in &receipt_package.files {
        let registry_file = registry_package
            .files
            .iter()
            .find(|file| {
                file.path == receipt_file.path
                    || file.logical_path.as_deref() == Some(receipt_file.path.as_str())
                    || receipt_file.logical_path.as_deref() == Some(file.path.as_str())
                    || file.logical_path.as_deref() == receipt_file.logical_path.as_deref()
            })
            .with_context(|| {
                format!(
                    "local registry package `{}` version `{}` does not contain `{}`",
                    receipt_package.package_id, receipt_package.version, receipt_file.path
                )
            })?;
        let content = registry_file
            .content
            .clone()
            .context("local registry rollback file is missing hydrated content")?;
        let hash = hash_bytes(content.as_bytes());
        if hash != receipt_file.hash {
            bail!(
                "local registry rollback content for `{}` has hash `{}`, receipt expects `{}`",
                receipt_file.path,
                hash,
                receipt_file.hash
            );
        }
        files.push(DxSourceFile {
            path: receipt_file.path.clone(),
            logical_path: receipt_file.logical_path.clone(),
            hash,
            bytes: content.len() as u64,
            content: Some(content),
        });
    }

    let integrity_hash = package_integrity_hash(&files);
    Ok(DxSourcePackage {
        package_id: registry_package.package_id,
        upstream_name: registry_package.description,
        version: registry_package.version,
        generator: "dx-forge/local-registry-rollback".to_string(),
        variant: receipt_package.variant.clone(),
        last_accepted_update: receipt_package.last_accepted_update.clone(),
        rollback_receipt: receipt_package.rollback_receipt.clone(),
        source_kind: registry_package.source_kind,
        integrity_hash,
        license: registry_package.license,
        provenance: registry_package.provenance,
        advisory_review: registry_package.advisory_review,
        license_review: registry_package.license_review,
        files,
    })
}

fn rollback_policy_decisions(
    package: &DxSourcePackage,
    receipt_path: &Path,
    traffic: DxUpdateTraffic,
    write: bool,
) -> Vec<DxPolicyDecision> {
    vec![
        DxPolicyDecision {
            policy: "receipt-content-match".to_string(),
            traffic,
            message: format!(
                "Rollback for `{}` variant `{}` uses only registry content matching `{}`.",
                package.package_id,
                package.variant,
                receipt_path.display()
            ),
        },
        DxPolicyDecision {
            policy: "rollback-write-mode".to_string(),
            traffic: DxUpdateTraffic::Green,
            message: if write {
                "Forge rollback writes restored source files and records a new receipt.".to_string()
            } else {
                "Forge rollback dry-run reports restore actions without writing files.".to_string()
            },
        },
    ]
}

fn remove_policy_decisions(
    package: &DxSourcePackage,
    traffic: DxUpdateTraffic,
    write: bool,
    archive_root: Option<&Path>,
) -> Vec<DxPolicyDecision> {
    vec![
        DxPolicyDecision {
            policy: "archive-before-remove".to_string(),
            traffic,
            message: match archive_root {
                Some(path) => format!(
                    "Forge remove for `{}` variant `{}` archives clean files under `{}` before deletion.",
                    package.package_id,
                    package.variant,
                    path.display()
                ),
                None => format!(
                    "Forge remove for `{}` variant `{}` dry-runs archive and delete actions without touching files.",
                    package.package_id, package.variant
                ),
            },
        },
        DxPolicyDecision {
            policy: "remove-write-mode".to_string(),
            traffic: DxUpdateTraffic::Green,
            message: if write {
                "Forge remove writes only after all tracked files are green; local edits are blocked."
                    .to_string()
            } else {
                "Forge remove dry-run reports archive/delete actions without writing files."
                    .to_string()
            },
        },
        DxPolicyDecision {
            policy: "no-lifecycle-execution".to_string(),
            traffic: DxUpdateTraffic::Green,
            message: "Forge remove does not run package lifecycle scripts or remote hooks."
                .to_string(),
        },
    ]
}

struct DxForgeRemovePersisted {
    manifest_path: PathBuf,
    receipt_path: PathBuf,
}

fn write_source_manifest_remove_receipt(
    project: &Path,
    receipt: DxForgeReceipt,
) -> Result<DxForgeRemovePersisted> {
    let manifest_path = project.join(SOURCE_MANIFEST_PATH);
    let receipt_dir = project.join(RECEIPT_DIR);
    let mut manifest = load_source_manifest(&manifest_path)?;
    let receipt_name = format!(
        "{}-{}",
        Utc::now().format("%Y%m%dT%H%M%S%fZ"),
        receipt_package_variant_suffix(&receipt.package.package_id, &receipt.package.variant)
    );

    manifest.remove_package(&receipt.package.package_id, &receipt.package.variant);
    manifest.receipts.push(receipt_name.clone());

    let receipt_path = receipt_dir.join(receipt_name);
    let mut transaction = DxForgeFileTransaction::new(project);
    if let Err(error) = write_source_manifest_remove_receipt_transaction(
        &mut transaction,
        &manifest_path,
        &receipt_path,
        &manifest,
        &receipt,
    ) {
        let rollback_findings = transaction.rollback();
        if rollback_findings.is_empty() {
            return Err(error);
        }
        bail!(
            "{}; transaction rollback findings: {}",
            error,
            rollback_findings.join("; ")
        );
    }
    transaction.commit();

    Ok(DxForgeRemovePersisted {
        manifest_path,
        receipt_path,
    })
}

fn write_source_manifest_remove_receipt_transaction(
    transaction: &mut DxForgeFileTransaction,
    manifest_path: &Path,
    receipt_path: &Path,
    manifest: &DxSourceManifest,
    receipt: &DxForgeReceipt,
) -> Result<()> {
    transaction.write_json_pretty(manifest_path, manifest)?;
    transaction.write_json_pretty(receipt_path, receipt)?;
    Ok(())
}

fn write_forge_dry_run_receipt(project: &Path, receipt: &DxForgeReceipt) -> Result<PathBuf> {
    let action = match receipt.action {
        DxForgeAction::AddDryRun => "add-dry-run",
        DxForgeAction::TrackDryRun => "track-dry-run",
        DxForgeAction::UpdateDryRun => "update-dry-run",
        DxForgeAction::RemoveDryRun => "remove-dry-run",
        DxForgeAction::RollbackDryRun => "rollback-dry-run",
        DxForgeAction::DocsDryRun => "docs-dry-run",
        _ => bail!(
            "refusing to persist non-dry-run Forge receipt action `{:?}` without a manifest update",
            receipt.action
        ),
    };
    let receipt_dir = project.join(RECEIPT_DIR);
    fs::create_dir_all(&receipt_dir)
        .with_context(|| format!("create `{}`", receipt_dir.display()))?;
    let receipt_name = format!(
        "{}-{}-{}",
        Utc::now().format("%Y%m%dT%H%M%S%fZ"),
        action,
        receipt_package_variant_suffix(&receipt.package.package_id, &receipt.package.variant)
    );
    let receipt_path = receipt_dir.join(receipt_name);
    let mut transaction = DxForgeFileTransaction::new(project);
    if let Err(error) = transaction.write_json_pretty(&receipt_path, receipt) {
        let rollback_findings = transaction.rollback();
        if rollback_findings.is_empty() {
            return Err(error);
        }
        bail!(
            "{}; transaction rollback findings: {}",
            error,
            rollback_findings.join("; ")
        );
    }
    transaction.commit();
    Ok(receipt_path)
}

fn write_source_manifest_receipt(
    project: &Path,
    receipt: DxForgeReceipt,
    wrote_files: bool,
) -> Result<DxForgeAddOutcome> {
    let mut receipt = receipt;
    let manifest_path = project.join(SOURCE_MANIFEST_PATH);
    let receipt_dir = project.join(RECEIPT_DIR);
    let docs_path = forge_package_docs_path(project, &receipt);
    let mut manifest = load_source_manifest(&manifest_path)?;
    let receipt_name = format!(
        "{}-{}",
        Utc::now().format("%Y%m%dT%H%M%S%fZ"),
        receipt_package_variant_suffix(&receipt.package.package_id, &receipt.package.variant)
    );

    if matches!(receipt.action, DxForgeAction::UpdateWrite) {
        receipt.package.last_accepted_update = Some(receipt.timestamp.clone());
        receipt.package.rollback_receipt = latest_receipt_for_package_variant(
            &manifest,
            &receipt.package.package_id,
            &receipt.package.variant,
        );
    }

    manifest.upsert_package(receipt.package.clone());
    manifest.receipts.push(receipt_name.clone());

    let receipt_path = receipt_dir.join(receipt_name);
    let mut transaction = DxForgeFileTransaction::new(project);
    if let Err(error) = write_source_manifest_receipt_transaction(
        &mut transaction,
        &receipt_path,
        &docs_path,
        &manifest_path,
        &receipt,
        &manifest,
    ) {
        let rollback_findings = transaction.rollback();
        if rollback_findings.is_empty() {
            return Err(error);
        }
        bail!(
            "{}; transaction rollback findings: {}",
            error,
            rollback_findings.join("; ")
        );
    }
    transaction.commit();

    Ok(DxForgeAddOutcome {
        receipt,
        manifest_path: Some(manifest_path),
        receipt_path: Some(receipt_path),
        wrote_files,
    })
}

fn forge_package_docs_path(project: &Path, receipt: &DxForgeReceipt) -> PathBuf {
    let doc_name =
        receipt_package_variant_suffix(&receipt.package.package_id, &receipt.package.variant)
            .trim_end_matches(".json")
            .to_string()
            + ".md";
    project.join(PACKAGE_DOCS_DIR).join(doc_name)
}

fn write_source_manifest_receipt_transaction(
    transaction: &mut DxForgeFileTransaction,
    receipt_path: &Path,
    docs_path: &Path,
    manifest_path: &Path,
    receipt: &DxForgeReceipt,
    manifest: &DxSourceManifest,
) -> Result<()> {
    transaction.write_json_pretty(receipt_path, receipt)?;
    write_forge_package_docs_at(transaction, docs_path, receipt)?;
    transaction.write_json_pretty(manifest_path, manifest)?;
    Ok(())
}

fn write_forge_package_docs_at(
    transaction: &mut DxForgeFileTransaction,
    docs_path: &Path,
    receipt: &DxForgeReceipt,
) -> Result<()> {
    transaction.write_bytes_atomic(docs_path, forge_package_docs_markdown(receipt).as_bytes())
}

fn package_contract_policy_decisions(package: &DxSourcePackage) -> Vec<DxPolicyDecision> {
    let mut decisions = vec![
        DxPolicyDecision {
            policy: "package-provenance-recorded".to_string(),
            traffic: package_provenance_policy_traffic(package),
            message: format!(
                "Forge records package provenance source `{}` with external verification `{}`.",
                package.provenance.source,
                bool_word(package.provenance.verified)
            ),
        },
        DxPolicyDecision {
            policy: "package-advisory-boundary".to_string(),
            traffic: package_advisory_policy_traffic(&package.advisory_review),
            message: format!(
                "Forge records advisory coverage `{}` from provider `{}` with live coverage `{}` and finding count `{}`.",
                package.advisory_review.coverage_kind.as_str(),
                package.advisory_review.provider,
                bool_word(package.advisory_review.live_coverage),
                package.advisory_review.finding_count
            ),
        },
        DxPolicyDecision {
            policy: "package-license-review-boundary".to_string(),
            traffic: package_license_policy_traffic(&package.license_review),
            message: format!(
                "Forge records declared license `{}` with formal review `{}`.",
                package.license_review.declared_license,
                bool_word(package.license_review.reviewed)
            ),
        },
    ];

    if package.package_id.as_str() == "auth/better-auth" {
        decisions.extend([
            DxPolicyDecision {
                policy: "auth-google-env-contract".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "auth/better-auth materializes `.env.example` for GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET, GOOGLE_REDIRECT_URI, GOOGLE_OAUTH_SCOPES, DX_GOOGLE_STATE_COOKIE, and DX_GOOGLE_ALLOWED_REDIRECT_ORIGIN; secrets remain outside Forge receipts.".to_string(),
            },
            DxPolicyDecision {
                policy: "auth-google-redirect-contract".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "auth/better-auth documents the local callback URL `http://localhost:3000/auth/better-auth/callback` and keeps redirect/origin values editable in project source.".to_string(),
            },
            DxPolicyDecision {
                policy: "auth-google-source-ownership".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "Forge owns the starter config, start route, callback handler, env example, and README files; the application owns session storage, secret rotation, and production redirect policy.".to_string(),
            },
            DxPolicyDecision {
                policy: "better-auth-database-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "auth/better-auth materializes a Better Auth server factory that requires the application to pass a real database adapter; Forge does not hide session storage behind a dummy in-memory default.".to_string(),
            },
            DxPolicyDecision {
                policy: "better-auth-next-handler-contract".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "auth/better-auth uses Better Auth's public `toNextJsHandler` and `nextCookies` APIs for route-handler integration and keeps the app-owned auth instance explicit.".to_string(),
            },
            DxPolicyDecision {
                policy: "better-auth-discovery-metadata".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "auth/better-auth includes `metadata.ts` so DX CLI, Zed, and launch templates can discover package id, upstream package, required dependency, and primary helper names without scanning generated source text.".to_string(),
            },
        ]);
    }

    if package.package_id.as_str() == "animation/motion" {
        decisions.extend([
            DxPolicyDecision {
                policy: "motion-react-public-api-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "animation/motion materializes a small adapter around Motion's public `motion/react` exports instead of copying private animation internals or faking runtime behavior.".to_string(),
            },
            DxPolicyDecision {
                policy: "motion-reduced-motion-contract".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "animation/motion includes a reveal component backed by `useInView` and `useReducedMotion` so launch templates get a real reduced-motion-safe animation baseline.".to_string(),
            },
            DxPolicyDecision {
                policy: "motion-discovery-metadata".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "animation/motion includes metadata.ts so DX CLI, Zed, and launch templates can discover package id, upstream package, import path, required dependencies, and primary helper names without scanning generated source text.".to_string(),
            },
        ]);
    }

    if package.package_id.as_str() == "tanstack/query" {
        decisions.extend([
            DxPolicyDecision {
                policy: "tanstack-query-public-api-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "tanstack/query materializes adapters around @tanstack/react-query public APIs instead of copying private observers, caches, or package internals.".to_string(),
            },
            DxPolicyDecision {
                policy: "tanstack-query-hydration-contract".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "tanstack/query includes a server prefetch helper around prefetchQuery(), dehydrate(), and HydrationBoundary so launch templates can wire real cached data paths.".to_string(),
            },
            DxPolicyDecision {
                policy: "tanstack-query-discovery-metadata".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "tanstack/query includes metadata.ts so DX CLI, Zed, and launch templates can discover package id, upstream package, required dependencies, and supported helper names without scanning source text.".to_string(),
            },
        ]);
    }

    if package.package_id.as_str() == "supabase/client" {
        decisions.extend([
            DxPolicyDecision {
                policy: "supabase-client-env-contract".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "supabase/client materializes `.env.example` for NEXT_PUBLIC_SUPABASE_URL and NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY; service-role secrets are intentionally excluded from the package.".to_string(),
            },
            DxPolicyDecision {
                policy: "supabase-client-ssr-contract".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "supabase/client uses @supabase/ssr createBrowserClient and createServerClient adapters with Next cookies so browser and server clients stay explicit application source.".to_string(),
            },
            DxPolicyDecision {
                policy: "supabase-client-source-ownership".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "Forge owns the adapter files, password auth actions, discovery metadata, env example, README, and profiles RLS seed; the application owns Supabase project configuration, Auth redirects, dependency versions, and database policy review.".to_string(),
            },
        ]);
    }

    if package.package_id.as_str() == "api/trpc" {
        decisions.extend([
            DxPolicyDecision {
                policy: "trpc-router-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "api/trpc materializes the app-owned initTRPC root, typed router, public/protected base procedures, and createCallerFactory export without hiding procedure design in a generated black box.".to_string(),
            },
            DxPolicyDecision {
                policy: "trpc-next-route-contract".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "api/trpc uses tRPC's public fetchRequestHandler adapter and mounts a Next App Router GET/POST route shim at app/api/trpc/[trpc]/route.ts.".to_string(),
            },
            DxPolicyDecision {
                policy: "trpc-discovery-metadata".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "api/trpc includes metadata.ts so DX CLI, Zed, and launch templates can discover package id, upstream packages, route path, provider, router type, and primary hook without scanning source text.".to_string(),
            },
        ]);
    }

    if package.package_id.as_str() == "forms/react-hook-form" {
        decisions.extend([
            DxPolicyDecision {
                policy: "react-hook-form-public-api-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "forms/react-hook-form materializes adapters around React Hook Form public APIs including useForm(), FormProvider, useFormContext(), Controller, useController(), useFieldArray(), useWatch(), and Resolver.".to_string(),
            },
            DxPolicyDecision {
                policy: "react-hook-form-shadcn-field-contract".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "forms/react-hook-form includes a registered input helper that composes the source-owned shadcn/ui input slice while leaving field labels, validation rules, and submit behavior application-owned.".to_string(),
            },
            DxPolicyDecision {
                policy: "react-hook-form-application-submit-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "forms/react-hook-form owns form shell and field helpers, while validation schemas, submit actions, persistence, spam protection, and authorization remain application-owned.".to_string(),
            },
            DxPolicyDecision {
                policy: "react-hook-form-discovery-metadata".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "forms/react-hook-form includes metadata.ts so DX CLI, Zed, and launch templates can discover package id, upstream package, required dependencies, and helper names without scanning source text.".to_string(),
            },
        ]);
    }

    if package.package_id.as_str() == "payments/stripe-js" {
        decisions.extend([
            DxPolicyDecision {
                policy: "stripe-js-pure-loader-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "payments/stripe-js imports loadStripe from @stripe/stripe-js/pure so Stripe.js is loaded only when application code requests the payment client.".to_string(),
            },
            DxPolicyDecision {
                policy: "stripe-js-browser-key-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "payments/stripe-js reads only NEXT_PUBLIC_STRIPE_* browser configuration and rejects missing or non-publishable keys; secret keys and webhook secrets remain server-owned.".to_string(),
            },
            DxPolicyDecision {
                policy: "stripe-js-payment-policy-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "payments/stripe-js provides a confirmPayment helper for Payment Element flows while PaymentIntent creation, pricing, tax, fraud, refunds, and webhook handling stay application-owned.".to_string(),
            },
            DxPolicyDecision {
                policy: "stripe-js-discovery-metadata".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "payments/stripe-js includes metadata.ts so DX CLI, Zed, and launch templates can discover package id, upstream package, env requirements, and helper names without scanning source text.".to_string(),
            },
        ]);
    }

    if package.package_id.as_str() == "db/drizzle-sqlite" {
        decisions.extend([
            DxPolicyDecision {
                policy: "drizzle-sqlite-public-api-contract".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "db/drizzle-sqlite uses Drizzle ORM public SQLite APIs for schema, views, relations, typed models, SQL helpers, typed joins, set operations, CTE/subquery helpers, conflict writes, update/delete mutations, analytics aggregates, and the better-sqlite3 driver; it does not vendor or rewrite upstream internals.".to_string(),
            },
            DxPolicyDecision {
                policy: "drizzle-sqlite-application-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "db/drizzle-sqlite materializes editable source for a local SQLite launch slice while database path and migrations stay application-owned. View SQL definitions, migration lifecycle, and compatibility with existing database views stay application-owned; join shape, null-handling, and cross-table authorization stay application-owned; set operation operand order, duplicate policy, result ordering, and pagination stay application-owned; CTE names, SQL aliases, aggregation semantics, and subquery pagination stay application-owned; conflict targets and merge policy, mutation authorization, audit trail, soft-delete policy, analytics definitions, and business KPI definitions stay application-owned; backups, permissions, and deployed data policy also stay application-owned.".to_string(),
            },
            DxPolicyDecision {
                policy: "drizzle-sqlite-discovery-metadata".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "db/drizzle-sqlite includes metadata.ts so DX CLI, Zed, and launch templates can discover package id, upstream package, dependencies, driver, and primary helper names without scanning README text.".to_string(),
            },
        ]);
    }

    if package.package_id.as_str() == "ai/vercel-ai" {
        decisions.extend([
            DxPolicyDecision {
                policy: "vercel-ai-model-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "ai/vercel-ai materializes an explicit model factory so the application owns provider package choice, API keys, model selection, and deployment policy.".to_string(),
            },
            DxPolicyDecision {
                policy: "vercel-ai-streaming-route-contract".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "ai/vercel-ai uses Vercel AI SDK public streamText(), convertToModelMessages(), tool(), and toUIMessageStreamResponse() APIs for a real streaming chat route.".to_string(),
            },
            DxPolicyDecision {
                policy: "vercel-ai-model-policy-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "ai/vercel-ai exposes a small model policy helper around wrapLanguageModel() and defaultSettingsMiddleware(); the app still owns model safety review, provider-specific options, and production evaluation.".to_string(),
            },
            DxPolicyDecision {
                policy: "vercel-ai-discovery-metadata".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "ai/vercel-ai includes metadata.ts so DX CLI, Zed, and launch templates can discover package id, upstream package, required dependencies, and primary helper names without scanning generated source text.".to_string(),
            },
            DxPolicyDecision {
                policy: "vercel-ai-telemetry-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "ai/vercel-ai exposes privacy-first TelemetryOptions helpers with input/output recording disabled by default; the application owns the telemetry sink, exporter, retention, and runtime-context inclusion policy.".to_string(),
            },
            DxPolicyDecision {
                policy: "vercel-ai-reranking-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "ai/vercel-ai exposes a reranking helper around rerank() and RerankingModel for launch evidence ordering; the app owns reranking model choice, provider options, relevance policy, and query/document governance.".to_string(),
            },
            DxPolicyDecision {
                policy: "vercel-ai-agent-route-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "ai/vercel-ai exposes a ToolLoopAgent factory and createAgentUIStreamResponse() agent route helper; the app owns agent instructions, tool approval policy, runtime context, auth, and rate limiting.".to_string(),
            },
            DxPolicyDecision {
                policy: "vercel-ai-tool-approval-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "ai/vercel-ai exposes typed tool approval helpers around ToolApprovalConfiguration, ToolApprovalRequest, and ToolApprovalResponse; the app owns approval policy, reviewer UX, audit logging, and unsafe tool governance.".to_string(),
            },
            DxPolicyDecision {
                policy: "vercel-ai-image-generation-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "ai/vercel-ai exposes image generation helpers around generateImage(), ImageModel, and GenerateImageResult; the app owns image model choice, prompt policy, moderation, storage, CDN, and asset licensing.".to_string(),
            },
            DxPolicyDecision {
                policy: "vercel-ai-speech-generation-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "ai/vercel-ai exposes speech generation helpers around experimental_generateSpeech(), SpeechModel, Experimental_SpeechResult, and GeneratedAudioFile; the app owns speech model and voice choice, copy policy, moderation, consent, storage, CDN, and audio licensing.".to_string(),
            },
            DxPolicyDecision {
                policy: "vercel-ai-transcription-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "ai/vercel-ai exposes transcription helpers around experimental_transcribe(), TranscriptionModel, Experimental_TranscriptionResult, and createDownload(); the app owns transcription model choice, audio upload limits, consent, retention, PII handling, and transcript storage policy.".to_string(),
            },
            DxPolicyDecision {
                policy: "vercel-ai-video-generation-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "ai/vercel-ai exposes video generation helpers around experimental_generateVideo(), GenerateVideoPrompt, GenerateVideoResult, VideoModel, and createDownload(); the app owns video model choice, prompt policy, download limits, moderation, storage, CDN, rights review, and asset licensing.".to_string(),
            },
            DxPolicyDecision {
                policy: "vercel-ai-object-generation-compatibility-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "ai/vercel-ai exposes object generation compatibility helpers around deprecated-but-exported generateObject(), streamObject(), GenerateObjectResult, and StreamObjectResult; the app owns schema design, compatibility policy, and migration toward generateText() with Output.object().".to_string(),
            },
            DxPolicyDecision {
                policy: "vercel-ai-ui-message-stream-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "ai/vercel-ai exposes UI message stream helpers around createUIMessageStream(), createUIMessageStreamResponse(), pipeUIMessageStreamToResponse(), readUIMessageStream(), and UI_MESSAGE_STREAM_HEADERS; the app owns persistence, resume, error, and client transport policy.".to_string(),
            },
            DxPolicyDecision {
                policy: "vercel-ai-text-stream-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "ai/vercel-ai exposes plain text stream helpers around createTextStreamResponse() and pipeTextStreamToResponse(); the app owns stream content, pacing, Node response integration, and public text exposure policy.".to_string(),
            },
            DxPolicyDecision {
                policy: "vercel-ai-file-upload-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "ai/vercel-ai exposes provider file upload helpers around uploadFile() and UploadFileResult; the app owns provider file API support, upload limits, scanning, retention, and storage policy.".to_string(),
            },
            DxPolicyDecision {
                policy: "vercel-ai-message-pruning-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "ai/vercel-ai exposes message pruning helpers around pruneMessages() and ModelMessage so launch routes can trim reasoning, tool, and approval history before model calls; the app owns retention, audit, and context-budget policy.".to_string(),
            },
        ]);
    }

    if package.package_id.as_str() == "content/fumadocs-next" {
        decisions.extend([
            DxPolicyDecision {
                policy: "fumadocs-public-api-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "content/fumadocs-next materializes adapters around Fumadocs public exports including createMDX(), defineDocs(), loader(), lucideIconsPlugin(), statusBadgesPlugin(), slugsFromData(), getBreadcrumbItems(), flattenTree(), findNeighbour(), getPageTreePeers(), getTableOfContents(), TOCItemType, page.data.toc, llms(), processed Markdown getText(), createOpenAPI(), staticSource(), loaderPlugin(), createProxy(), proxyUrl, createAPIPage(), createCodeUsageGeneratorRegistry(), registerDefault(), defineClientConfig(), createFromSource(), staticGET(), useDocsSearch client presets, DocsLayout, DocsPage, and createRelativeLink; it does not copy Fumadocs internals.".to_string(),
            },
            DxPolicyDecision {
                policy: "fumadocs-app-router-contract".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "content/fumadocs-next writes editable Next App Router docs files for /docs, source plugin frontmatter/status badge/icon wiring, navigation snapshot helpers, TOC summary helpers, OpenAPI virtual docs under /docs/api, an allowed-origin OpenAPI proxy at /api/openapi/proxy, OpenAPI request code usage generators, Fumadocs llms() routes at /llms.txt, /llms-full.txt, and /llms.mdx/docs, a createFromSource-backed /api/search route, a staticGET-backed /api/search-static route, generated static params, metadata, MDX component merging, and starter content that the application owns after materialization.".to_string(),
            },
            DxPolicyDecision {
                policy: "fumadocs-discovery-metadata".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "content/fumadocs-next includes metadata.ts so DX CLI, Zed, and launch templates can discover package id, upstream Fumadocs packages, dependencies, docs/LLMs/OpenAPI/proxy/search/static-search routes, source plugin frontmatter fields, navigation snapshot helpers, TOC summary helpers, OpenAPI code usage registry, client search presets, materialized files, and package boundaries without scanning README text.".to_string(),
            },
        ]);
    }

    if package.package_id.as_str() == "content/react-markdown" {
        decisions.extend([
            DxPolicyDecision {
                policy: "react-markdown-public-api-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "content/react-markdown materializes adapters around react-markdown public APIs including Markdown, MarkdownAsync, MarkdownHooks, defaultUrlTransform, Components, Options, and UrlTransform.".to_string(),
            },
            DxPolicyDecision {
                policy: "react-markdown-safe-defaults".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "content/react-markdown keeps skipHtml enabled and constrains default allowedElements so launch content rendering does not silently opt into raw HTML trust.".to_string(),
            },
            DxPolicyDecision {
                policy: "react-markdown-application-content-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "content/react-markdown owns editable renderer helpers while content moderation, remark/rehype plugin selection, link policy, sanitization review, and final typography stay application-owned.".to_string(),
            },
        ]);
    }

    if package.package_id.as_str() == "migration/static-site" {
        decisions.extend([
            DxPolicyDecision {
                policy: "static-migration-scope-boundary".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "migration/static-site materializes only a scoped static page/content example; plugin, theme, CMS, ecommerce, form, comment, search, account, and shortcode behavior remains application-owned work.".to_string(),
            },
            DxPolicyDecision {
                policy: "static-migration-no-install".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "migration/static-site writes editable local source and fixture content without running package installs, lifecycle hooks, or remote imports.".to_string(),
            },
        ]);
    }

    decisions
}

fn package_provenance_policy_traffic(package: &DxSourcePackage) -> DxUpdateTraffic {
    if package.provenance.verified {
        DxUpdateTraffic::Green
    } else {
        DxUpdateTraffic::Yellow
    }
}

fn package_advisory_policy_traffic(review: &DxForgeAdvisoryMetadata) -> DxUpdateTraffic {
    let reviewed_offline = review.coverage_kind == DxForgeAdvisoryCoverageKind::OfflineSnapshot
        && review.reviewed_at.is_some();
    let live_feed =
        review.coverage_kind == DxForgeAdvisoryCoverageKind::LiveFeed && review.live_coverage;

    if reviewed_offline || live_feed {
        DxUpdateTraffic::Green
    } else {
        DxUpdateTraffic::Yellow
    }
}

fn package_license_policy_traffic(review: &DxForgeLicenseReviewMetadata) -> DxUpdateTraffic {
    if review.reviewed {
        DxUpdateTraffic::Green
    } else {
        DxUpdateTraffic::Yellow
    }
}

fn regenerate_forge_docs(project: &Path, write: bool) -> Result<DxForgeDocsOutcome> {
    let manifest_path = project.join(SOURCE_MANIFEST_PATH);
    let manifest = load_source_manifest(&manifest_path)?;
    let docs_dir = project.join(PACKAGE_DOCS_DIR);
    if write {
        fs::create_dir_all(&docs_dir)
            .with_context(|| format!("create `{}`", docs_dir.display()))?;
    }

    let action = if write {
        DxForgeAction::DocsWrite
    } else {
        DxForgeAction::DocsDryRun
    };
    let mut packages = Vec::new();

    for package in &manifest.packages {
        let docs_name = receipt_package_variant_suffix(&package.package_id, &package.variant)
            .trim_end_matches(".json")
            .to_string()
            + ".md";
        let docs_path = docs_dir.join(docs_name);
        let existed_before = docs_path.exists();

        if write {
            let receipt = docs_regeneration_receipt(package, action.clone());
            fs::write(&docs_path, forge_package_docs_markdown(&receipt))
                .with_context(|| format!("write `{}`", docs_path.display()))?;
        }

        packages.push(DxForgeDocsPackage {
            package_id: package.package_id.clone(),
            variant: package.variant.clone(),
            docs_path,
            existed_before,
            wrote_file: write,
            source_file_count: package.files.len() as u64,
        });
    }

    Ok(DxForgeDocsOutcome {
        project: project.to_path_buf(),
        manifest_path,
        packages,
        wrote_files: write,
    })
}

fn docs_regeneration_receipt(package: &DxSourcePackage, action: DxForgeAction) -> DxForgeReceipt {
    DxForgeReceipt {
        action,
        package: package.without_content(),
        selected_exports: Vec::new(),
        files_written: Vec::new(),
        file_map: receipt_file_map(package),
        policy_decisions: vec![DxPolicyDecision {
            policy: "docs-regeneration".to_string(),
            traffic: DxUpdateTraffic::Green,
            message: "Forge docs were regenerated from the existing source manifest without rewriting package source files."
                .to_string(),
        }],
        update_decisions: Vec::new(),
        risk_score: 100,
        timestamp: Utc::now().to_rfc3339(),
        signature: None,
    }
}

fn forge_package_docs_markdown(receipt: &DxForgeReceipt) -> String {
    let package = &receipt.package;
    let update_command = if package.variant == "default" {
        format!("dx update {}", package.package_id)
    } else {
        format!(
            "dx update {} --variant {}",
            package.package_id, package.variant
        )
    };
    let mut output = format!(
        "# DX Forge Package: `{}`\n\n\
- Variant: `{}`\n\
- Version: `{}`\n\
- Upstream: `{}`\n\
- Generator: `{}`\n\
- License: `{}`\n\
- Provenance: `{}` (verified: `{}`)\n\
- Advisory coverage: `{}` via `{}` (live: `{}`, findings: `{}`)\n\
- License review: declared `{}` (reviewed: `{}`)\n\
- Last action: `{:?}`\n\
- Risk score: `{}`\n\n\
This package is source-owned. The files below are editable project files, not opaque `node_modules` content. Forge tracks their hashes, treats local edits as reviewable yellow traffic, blocks red/security-sensitive traffic, and updates them through `{}`.\n\n",
        package.package_id,
        package.variant,
        package.version,
        package.upstream_name,
        package.generator,
        package.license,
        &package.provenance.source,
        bool_word(package.provenance.verified),
        package.advisory_review.coverage_kind.as_str(),
        &package.advisory_review.provider,
        bool_word(package.advisory_review.live_coverage),
        package.advisory_review.finding_count,
        &package.license_review.declared_license,
        bool_word(package.license_review.reviewed),
        receipt.action,
        receipt.risk_score,
        update_command
    );

    output.push_str("## Package Metadata Review\n\n");
    output.push_str(&format!(
        "- Provenance note: {}\n- Advisory note: {}\n- License review note: {}\n\n",
        &package.provenance.note, &package.advisory_review.note, &package.license_review.note
    ));

    if let Some(package_docs) = package_specific_docs_markdown(package) {
        output.push_str(&package_docs);
    }

    output.push_str("## Materialized Files\n\n");
    output.push_str("| File | Logical Source | Bytes | Hash |\n");
    output.push_str("| --- | --- | ---: | --- |\n");
    for file in &receipt.file_map {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` |\n",
            md_cell(&file.materialized_path),
            md_cell(&file.logical_path),
            file.bytes,
            file.hash
        ));
    }

    if !receipt.policy_decisions.is_empty() {
        output.push_str("\n## Forge Policy\n\n");
        output.push_str("| Traffic | Policy | Decision |\n");
        output.push_str("| --- | --- | --- |\n");
        for decision in &receipt.policy_decisions {
            output.push_str(&format!(
                "| `{}` | `{}` | {} |\n",
                decision.traffic.as_str(),
                md_cell(&decision.policy),
                md_cell(&decision.message)
            ));
        }
    }

    if !receipt.update_decisions.is_empty() {
        output.push_str("\n## Update Decisions\n\n");
        output.push_str("| Traffic | Change | Path | Decision |\n");
        output.push_str("| --- | --- | --- | --- |\n");
        for decision in &receipt.update_decisions {
            output.push_str(&format!(
                "| `{}` | `{:?}` | `{}` | `{:?}` |\n",
                decision.traffic.as_str(),
                decision.change,
                md_cell(&decision.path),
                decision.decision
            ));
        }
    }

    output
}

fn package_specific_docs_markdown(package: &DxSourcePackage) -> Option<String> {
    match package.package_id.as_str() {
        "auth/better-auth" => Some(
            "## Google OAuth Contract\n\n\
Forge owns the starter files for the OAuth handoff, but it does not own your production identity policy. Review and edit these values before deployment.\n\n\
- Required env vars: `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`, `GOOGLE_REDIRECT_URI`.\n\
- Optional env vars: `GOOGLE_OAUTH_SCOPES`, `DX_GOOGLE_STATE_COOKIE`, `DX_GOOGLE_ALLOWED_REDIRECT_ORIGIN`.\n\
- Local callback example: `http://localhost:3000/auth/better-auth/callback`.\n\
- Owned source files: `auth/better-auth/config.ts`, `auth/better-auth/route.ts`, `auth/better-auth/callback.ts`, `auth/better-auth/.env.example`, and `auth/better-auth/README.md`.\n\
- Application-owned work: connect the callback token response to your session store, rotate secrets outside the repo, and set production redirect origins explicitly.\n\n\
## Better Auth Contract\n\n\
Forge owns the launch slice around Better Auth's public APIs, but it does not own your database adapter, production identity policy, or secret rotation.\n\n\
- Required env vars: `BETTER_AUTH_SECRET` and `BETTER_AUTH_URL`.\n\
- Optional env vars: `BETTER_AUTH_TRUSTED_ORIGINS`, `BETTER_AUTH_APP_NAME`, `BETTER_AUTH_EMAIL_PASSWORD_ENABLED`, `NEXT_PUBLIC_BETTER_AUTH_URL`, `GITHUB_CLIENT_ID`, `GITHUB_CLIENT_SECRET`, `GOOGLE_CLIENT_ID`, and `GOOGLE_CLIENT_SECRET`.\n\
- Owned source files: `auth/better-auth/options.ts`, `auth/better-auth/server.ts`, `auth/better-auth/client.ts`, `auth/better-auth/route.ts`, `auth/better-auth/metadata.ts`, `auth/better-auth/.env.example`, and `auth/better-auth/README.md`.\n\
- Application-owned work: provide the Better Auth database adapter, keep secrets outside the repo, review trusted origins, and mount the route helper from your framework route file.\n\n"
                .to_string(),
        ),
        "animation/motion" => Some(
            "## Motion React Contract\n\n\
Forge owns a small source-owned launch slice around Motion's public React API, but it does not own your application choreography, gesture model, layout projection strategy, or performance budget.\n\n\
- Required app dependencies: `motion` and React.\n\
- Owned source files: `motion/presets.ts`, `motion/reveal.tsx`, `motion/metadata.ts`, and `motion/README.md`.\n\
- Real API surface: `motion`, `useInView`, `useReducedMotion`, `Transition`, `Variants`, and `HTMLMotionProps` from `motion/react`.\n\
- Application-owned work: install the runtime dependency, choose route-specific animation timing, preserve reduced-motion UX, and profile high-density animated screens.\n\n"
                .to_string(),
        ),
        "tanstack/query" => Some(
            "## TanStack Query Contract\n\n\
Forge owns a small source-owned launch adapter around TanStack Query's public APIs, but it does not vendor TanStack Query internals or own your endpoint behavior.\n\n\
- Required dependencies: `@tanstack/react-query` and React.\n\
- Owned source files: `lib/query/client.ts`, `lib/query/provider.tsx`, `lib/query/prefetch.tsx`, `lib/query/metadata.ts`, and `lib/query/README.md`.\n\
- Public APIs used: `QueryClient`, `QueryClientProvider`, `queryOptions`, `prefetchQuery`, `dehydrate`, and `HydrationBoundary`.\n\
- Application-owned work: install the runtime dependency, define real query functions, review cache lifetimes per route, and keep sensitive payloads out of persisted caches.\n\n"
                .to_string(),
        ),
        "forms/react-hook-form" => Some(
            "## React Hook Form Contract\n\n\
Forge owns a small source-owned launch adapter around React Hook Form's public APIs, but it does not own your submitted data, validation schema quality, accessibility review, spam protection, or authorization policy.\n\n\
- Required dependencies: `react-hook-form`, React, and the source-owned `shadcn/ui/input` slice for the default field helper.\n\
- Recommended integration: `validation/zod` for schema-backed resolver usage.\n\
- Owned source files: `lib/forms/react-hook-form/form.tsx`, `lib/forms/react-hook-form/fields.tsx`, `lib/forms/react-hook-form/resolver.ts`, `lib/forms/react-hook-form/example.tsx`, `lib/forms/react-hook-form/metadata.ts`, and `lib/forms/react-hook-form/README.md`.\n\
- Public APIs used: `useForm`, `FormProvider`, `useFormContext`, `Controller`, `useFieldArray`, `Resolver`, `register`, and `handleSubmit`.\n\
- Application-owned work: install the runtime dependency, wire real submit handlers/server actions, review validation and accessibility, and keep sensitive form data protected.\n\n"
                .to_string(),
        ),
        "content/react-markdown" => Some(
            "## React Markdown Contract\n\n\
Forge owns a small source-owned launch content renderer around react-markdown's public APIs, but it does not own content moderation, plugin policy, raw HTML sanitization, link governance, or final typography.\n\n\
- Required dependencies: `react-markdown` and React.\n\
- Owned source files: `components/content/markdown.tsx`, `components/content/markdown-components.tsx`, `components/content/markdown-metadata.ts`, `components/content/README.md`, `components/markdown.tsx`, `components/markdown-client.tsx`, `lib/react-markdown/metadata.ts`, and `lib/react-markdown/README.md`.\n\
- Public APIs used: default `Markdown`, `MarkdownAsync`, `MarkdownHooks`, `defaultUrlTransform`, `Components`, `Options`, and `UrlTransform`.\n\
- Application-owned work: install the runtime dependency, keep raw HTML disabled unless reviewed, choose remark/rehype plugins carefully, moderate user-generated content, and review link policy before release.\n\n"
                .to_string(),
        ),
        "payments/stripe-js" => Some(
            "## Stripe.js Contract\n\n\
Forge owns a small source-owned launch adapter around Stripe.js browser APIs, but it does not own backend payment creation, secret-key handling, webhooks, pricing, tax, fraud, disputes, refunds, or compliance policy.\n\n\
- Required dependencies: `@stripe/stripe-js` in browser code and `stripe` in server-only route code.\n\
- Required env vars: `NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY`, `STRIPE_SECRET_KEY`, and an app-owned `STRIPE_PRICE_ID` or product catalog lookup before live Checkout.\n\
- Optional env vars: `NEXT_PUBLIC_STRIPE_ACCOUNT`, `NEXT_PUBLIC_STRIPE_API_VERSION`, `NEXT_PUBLIC_STRIPE_LOCALE`, `NEXT_PUBLIC_STRIPE_ADVANCED_FRAUD_SIGNALS`, `STRIPE_WEBHOOK_SECRET`, `APP_URL`, and `NEXT_PUBLIC_APP_URL`.\n\
- Owned source files: `lib/payments/stripe-js/config.ts`, `lib/payments/stripe-js/client.ts`, `lib/payments/stripe-js/payment.ts`, `lib/payments/stripe-js/checkout.ts`, `lib/payments/stripe-js/dashboard-checkout.ts`, `lib/payments/stripe-js/server.ts`, `app/api/checkout/route.ts`, `app/api/stripe/webhook/route.ts`, `lib/payments/stripe-js/metadata.ts`, and `lib/payments/stripe-js/README.md`.\n\
- Public APIs used: `loadStripe`, `loadStripe.setLoadParameters`, `Stripe`, `StripeElements`, `StripeConstructorOptions`, `stripe.confirmPayment`, `stripe.retrievePaymentIntent`, `stripe.createEmbeddedCheckoutPage`, `StripeEmbeddedCheckoutOptions.fetchClientSecret`, server Checkout Session creation, Billing Portal Session creation, and webhook signature verification.\n\
- Dashboard starter usage: `examples/dashboard/src/components/StripePlanCheckout.tsx` exposes `data-dx-component=\"dashboard-stripe-plan-checkout\"`, DX Icons metadata, hosted/embedded checkout mode selection, and a safe missing-config receipt instead of fake payment success.\n\
- Launch billing workflow usage: `examples/template/payments-status.tsx` exposes `data-dx-component=\"launch-billing-checkout-workflow\"`, `data-dx-dashboard-flow=\"billing-checkout\"`, plan selection, hosted/embedded checkout mode selection, and a safe missing-config receipt instead of fake payment success.\n\
- Application-owned work: create PaymentIntents or Checkout Sessions on the server, keep secret keys out of browser code, mount Stripe Elements or Checkout UI, handle failures, map plans to prices, verify webhooks, persist fulfillment, and review production payment policy.\n\n"
                .to_string(),
        ),
        "i18n/next-intl" => Some(
            "## next-intl Contract\n\n\
Forge owns the App Router i18n slice around next-intl's public APIs, but it does not own your translation process, locale strategy, or production routing policy.\n\n\
- Required app dependencies: `next-intl`, Next.js, and React.\n\
- Owned source files: `i18n/routing.ts`, `i18n/navigation.ts`, `i18n/request.ts`, `i18n/middleware.ts`, `i18n/provider.tsx`, `i18n/messages/en.json`, `i18n/messages/bn.json`, `i18n/metadata.ts`, and `i18n/README.md`.\n\
- Public APIs used: `defineRouting`, `createNavigation`, `getRequestConfig`, `createMiddleware`, `NextIntlClientProvider`, `useTranslations`, and `getTranslations`.\n\
- Application-owned work: mount middleware/request config in the expected Next.js locations, replace starter messages, review locale prefixes/domains, and keep translated content current before release.\n\n"
                .to_string(),
        ),
        "ai/vercel-ai" => Some(
            "## Vercel AI SDK Contract\n\n\
Forge owns a small source-owned launch slice around the Vercel AI SDK public APIs, but it does not own provider credentials, model policy, persistence, moderation, or rate limits.\n\n\
- Required dependencies: `ai`, a provider package such as `@ai-sdk/openai`, and `zod`.\n\
- Owned source files: `lib/ai/model.ts`, `lib/ai/model-policy.ts`, `lib/ai/tools.ts`, `lib/ai/chat-route.ts`, `lib/ai/message-pruning.ts`, `lib/ai/tool-approval.ts`, `lib/ai/image-generation.ts`, `lib/ai/speech-generation.ts`, `lib/ai/transcription.ts`, `lib/ai/video-generation.ts`, `lib/ai/object-generation.ts`, `lib/ai/file-upload.ts`, `lib/ai/text-stream.ts`, `lib/ai/ui-message-stream.ts`, `lib/ai/structured-output.ts`, `lib/ai/telemetry.ts`, `lib/ai/embeddings.ts`, `lib/ai/reranking.ts`, `lib/ai/agent.ts`, `lib/ai/provider-freedom.ts`, `lib/ai/client-chat.tsx`, `lib/ai/metadata.ts`, `app/api/ai/chat/route.ts`, `app/api/ai/agent/route.ts`, `app/api/ai/image/route.ts`, `app/api/ai/speech/route.ts`, `app/api/ai/transcribe/route.ts`, `app/api/ai/video/route.ts`, `app/api/ai/object/route.ts`, `app/api/ai/upload-file/route.ts`, `app/api/ai/text-stream/route.ts`, `app/api/ai/ui-stream/route.ts`, and `lib/ai/README.md`.\n\
- Real API surface: `streamText`, `pruneMessages`, `ModelMessage`, `generateImage`, `ImageModel`, `GenerateImageResult`, `GeneratedFile`, `experimental_generateSpeech`, `SpeechModel`, `Experimental_SpeechResult`, `GeneratedAudioFile`, `experimental_transcribe`, `TranscriptionModel`, `Experimental_TranscriptionResult`, `experimental_generateVideo`, `GenerateVideoPrompt`, `GenerateVideoResult`, `VideoModel`, `createDownload`, `generateObject`, `streamObject`, `GenerateObjectResult`, `StreamObjectResult`, `uploadFile`, `UploadFileResult`, `createTextStreamResponse`, `pipeTextStreamToResponse`, `createUIMessageStream`, `createUIMessageStreamResponse`, `pipeUIMessageStreamToResponse`, `readUIMessageStream`, `UI_MESSAGE_STREAM_HEADERS`, `generateText`, `Output.object`, `embed`, `embedMany`, `EmbeddingModel`, `cosineSimilarity`, `rerank`, `RerankingModel`, `ToolLoopAgent`, `InferAgentUIMessage`, `createAgentUIStreamResponse`, `ToolApprovalConfiguration`, `ToolApprovalStatus`, `ToolApprovalRequest`, `ToolApprovalResponse`, `LanguageModelMiddleware`, `wrapLanguageModel`, `defaultSettingsMiddleware`, `Telemetry`, `TelemetryOptions`, `registerTelemetry`, `gateway`, `createGateway`, `customProvider`, `createProviderRegistry`, `convertToModelMessages`, `tool`, `DefaultChatTransport`, and `UIMessage`.\n\
- Application-owned work: choose the provider/model, image model, speech model/voice, transcription model, video model, embedding model, and reranking model, review provider registry policy and model policy, keep API keys outside source control, connect live launch receipts, add persistence/vector storage/generated asset storage/CDN if needed, and enforce production safety, schema design, object generation compatibility policy, migration toward `generateText` with `Output.object`, provider file API support, upload limits, scanning, retention, storage policy, text stream content/pacing/Node response policy, UI message stream persistence/resume/error policy, audio/video upload and download limits, consent, PII handling, prompt and speech copy moderation, voice consent, video rights review, audio/image/video asset licensing, telemetry sink/exporter policy, agent route auth/rate limiting, agent instructions/message pruning/tool approval/reviewer UX/runtime context, relevance policy, and structured output policy.\n\n"
                .to_string(),
        ),
        "api/trpc" => Some(
            "## tRPC Contract\n\n\
Forge owns a small source-owned launch slice around tRPC's public APIs, but it does not own application authorization, procedure design, persistence, or request policy.\n\n\
- Required dependencies: `@trpc/server`, `@trpc/client`, `@trpc/tanstack-react-query`, `@tanstack/react-query`, and `zod`.\n\
- Owned source files: `lib/trpc/context.ts`, `lib/trpc/server.ts`, `lib/trpc/router.ts`, `lib/trpc/route-handler.ts`, `app/api/trpc/[trpc]/route.ts`, `lib/trpc/client.ts`, `lib/trpc/provider.tsx`, `lib/trpc/metadata.ts`, and `lib/trpc/README.md`.\n\
- Real API surface: `initTRPC`, `router`, `procedure`, `createCallerFactory`, `fetchRequestHandler`, `createTRPCClient`, `httpBatchLink`, and `createTRPCContext`.\n\
- Application-owned work: add domain routers, enforce authorization, connect sessions, set request limits, and install the runtime dependencies in the host app.\n\n"
                .to_string(),
        ),
        "supabase/client" => Some(
            "## Supabase SSR Client Contract\n\n\
Forge owns the adapter files around Supabase's public SSR APIs, but it does not own your deployed Supabase project, dependency installation, Auth redirect policy, or database RLS review.\n\n\
- Required env vars: `NEXT_PUBLIC_SUPABASE_URL` and `NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY`.\n\
- Required app dependencies: `@supabase/ssr` and `@supabase/supabase-js`.\n\
- Owned source files: `lib/supabase/env.ts`, `lib/supabase/browser.ts`, `lib/supabase/server.ts`, `lib/supabase/auth-actions.ts`, `lib/supabase/metadata.ts`, `lib/supabase/schema.sql`, `lib/supabase/.env.example`, and `lib/supabase/README.md`.\n\
- Application-owned work: install dependencies, run the SQL in the intended Supabase project, configure Auth redirects, and keep service-role secrets out of public env files.\n\n"
                .to_string(),
        ),
        "db/drizzle-sqlite" => Some(
            "## Drizzle SQLite Contract\n\n\
Forge owns a SQLite-first Drizzle ORM launch slice based on Drizzle's public APIs, but it does not own your deployed database policy, migrations, backups, or hosting choice.\n\n\
- Required app dependencies: `drizzle-orm` and `better-sqlite3`.\n\
- Recommended dev dependencies: `drizzle-kit` and `@types/better-sqlite3`.\n\
- Owned source files: `db/drizzle/client.ts`, `db/drizzle/schema.ts`, `db/drizzle/views.ts`, `db/drizzle/queries.ts`, `db/drizzle/migrations.ts`, `db/drizzle/relational-queries.ts`, `db/drizzle/joins.ts`, `db/drizzle/set-operations.ts`, `db/drizzle/cte-queries.ts`, `db/drizzle/transactions.ts`, `db/drizzle/prepared-queries.ts`, `db/drizzle/upserts.ts`, `db/drizzle/mutations.ts`, `db/drizzle/analytics.ts`, `db/drizzle/dashboard-workflow.ts`, `db/drizzle/metadata.ts`, and `db/drizzle/README.md`.\n\
- Application-owned work: choose the database file path, generate and review migration SQL, run migrations at the right deployment boundary, protect backups, review query access. View SQL definitions, migration lifecycle, and compatibility with existing database views stay application-owned; join shape, null-handling, cross-table authorization, set operation operand order, duplicate policy, result ordering, pagination, CTE names, SQL aliases, aggregation semantics, subquery pagination, transaction boundaries, prepared statement lifetime, conflict targets and merge policy, mutation authorization, audit trail, and soft-delete policy, analytics definitions, and business KPI ownership, and driver changes beyond local SQLite stay application-owned.\n\n"
                .to_string(),
        ),
        "migration/static-site" => Some(
            "## Static Migration Contract\n\n\
Forge owns the scoped migration seed files, but it does not migrate a whole WordPress site or dynamic application behavior.\n\n\
- Owned source files: `migrations/static-site/content.ts`, `migrations/static-site/page.tsx`, `migrations/static-site/sample-wordpress-export.json`, and `migrations/static-site/README.md`.\n\
- No package install is required; the package writes editable local files only.\n\
- In scope: a reviewed static page/content fixture, asset mapping notes, route helper, and visible manual-review warnings.\n\
- Out of scope: WordPress plugins, themes, block-editor semantics, shortcodes, forms, comments, search, ecommerce, memberships, accounts, CMS editing, redirects, analytics, and unsanitized remote HTML trust.\n\n"
                .to_string(),
        ),
        _ => None,
    }
}

fn md_cell(value: &str) -> String {
    value.replace('|', "\\|")
}

fn bool_word(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

impl DxSourcePackage {
    fn without_content(&self) -> Self {
        let mut package = self.clone();
        package.files = package
            .files
            .iter()
            .map(DxSourceFile::without_content)
            .collect();
        package
    }
}

impl DxSourceFile {
    fn without_content(&self) -> Self {
        Self {
            path: self.path.clone(),
            logical_path: self.logical_path.clone(),
            hash: self.hash.clone(),
            bytes: self.bytes,
            content: None,
        }
    }
}

fn receipt_file_map(package: &DxSourcePackage) -> Vec<DxForgeFileMap> {
    package
        .files
        .iter()
        .map(|file| DxForgeFileMap {
            logical_path: file
                .logical_path
                .clone()
                .unwrap_or_else(|| file.path.clone()),
            materialized_path: file.path.clone(),
            hash: file.hash.clone(),
            bytes: file.bytes,
        })
        .collect()
}

fn update_policy_decisions(
    changes: &[DxForgeUpdateFileChange],
    write: bool,
) -> Vec<DxPolicyDecision> {
    let mut decisions = vec![if write {
        DxPolicyDecision {
            policy: "green-only-update".to_string(),
            traffic: DxUpdateTraffic::Green,
            message:
                "Forge update write only applies green changes and never runs package scripts."
                    .to_string(),
        }
    } else {
        DxPolicyDecision {
            policy: "dry-run-only".to_string(),
            traffic: DxUpdateTraffic::Green,
            message: "Forge update preview does not write files or run package scripts."
                .to_string(),
        }
    }];

    for change in changes {
        decisions.push(DxPolicyDecision {
            policy: format!("update-{}", change.change.as_policy_suffix()),
            traffic: change.traffic,
            message: change.message.clone(),
        });
    }

    decisions
}

fn update_review_plan(
    package_id: &str,
    traffic: DxUpdateTraffic,
    changes: &[DxForgeUpdateFileChange],
    findings: &[DxSupplyChainFinding],
) -> Vec<DxForgeUpdateReviewStep> {
    if traffic == DxUpdateTraffic::Green {
        return Vec::new();
    }

    let mut steps = vec![DxForgeUpdateReviewStep {
        traffic,
        path: None,
        action: "human-review-required".to_string(),
        message: format!(
            "`{package_id}` update is {}. Forge will not write until every file is green or a future explicit review command records accepted decisions.",
            traffic.as_str()
        ),
    }];

    for change in changes
        .iter()
        .filter(|change| change.traffic != DxUpdateTraffic::Green)
    {
        let remediation = findings
            .iter()
            .find(|finding| finding.evidence_path.as_deref() == Some(change.path.as_str()))
            .map(|finding| finding.remediation.as_str())
            .unwrap_or(change.message.as_str());
        steps.push(DxForgeUpdateReviewStep {
            traffic: change.traffic,
            path: Some(change.path.clone()),
            action: change.change.review_action().to_string(),
            message: remediation.to_string(),
        });
    }

    steps
}

fn update_quarantine_report(
    changes: &[DxForgeUpdateFileChange],
    findings: &[DxSupplyChainFinding],
) -> Vec<DxForgeQuarantineFile> {
    changes
        .iter()
        .filter(|change| change.traffic == DxUpdateTraffic::Red)
        .map(|change| {
            let finding = findings
                .iter()
                .find(|finding| finding.evidence_path.as_deref() == Some(change.path.as_str()));
            DxForgeQuarantineFile {
                path: change.path.clone(),
                logical_path: change.logical_path.clone(),
                change: change.change,
                traffic: change.traffic,
                reason: finding
                    .map(|finding| finding.message.clone())
                    .unwrap_or_else(|| change.message.clone()),
                remediation: finding
                    .map(|finding| finding.remediation.clone())
                    .unwrap_or_else(|| {
                        "Review the file manually, restore from a trusted receipt, or create a reviewed replacement before running a write."
                            .to_string()
                    }),
                would_write: false,
            }
        })
        .collect()
}

fn update_receipt_file_decisions(
    changes: &[DxForgeUpdateFileChange],
    write: bool,
    approval: Option<&DxForgeUpdateApproval>,
) -> Vec<DxForgeReceiptFileDecision> {
    changes
        .iter()
        .map(|change| {
            let approved_yellow_local = write
                && approval.is_some()
                && change.change == DxForgeUpdateChangeKind::LocalEdit
                && change.traffic == DxUpdateTraffic::Yellow;
            let accepted = if write {
                matches!(
                    change.change,
                    DxForgeUpdateChangeKind::Unchanged
                        | DxForgeUpdateChangeKind::Add
                        | DxForgeUpdateChangeKind::Update
                ) && change.traffic == DxUpdateTraffic::Green
                    || approved_yellow_local
            } else {
                change.change == DxForgeUpdateChangeKind::Unchanged
                    && change.traffic == DxUpdateTraffic::Green
            };
            let decision = if accepted {
                DxForgeReceiptFileDecisionKind::Accepted
            } else {
                DxForgeReceiptFileDecisionKind::Rejected
            };
            let message = match (write, decision, change.change) {
                (
                    _,
                    DxForgeReceiptFileDecisionKind::Accepted,
                    DxForgeUpdateChangeKind::Unchanged,
                ) => {
                    format!(
                        "{} accepted as already matching the latest package.",
                        change.path
                    )
                }
                (true, DxForgeReceiptFileDecisionKind::Accepted, DxForgeUpdateChangeKind::Add) => {
                    format!(
                        "{} accepted and written as a new Forge-owned file.",
                        change.path
                    )
                }
                (
                    true,
                    DxForgeReceiptFileDecisionKind::Accepted,
                    DxForgeUpdateChangeKind::LocalEdit,
                ) => {
                    let reviewer = approval
                        .map(|approval| approval.reviewer.trim())
                        .unwrap_or("unknown reviewer");
                    format!(
                        "{} accepted by `{reviewer}` as a reviewed local edit; Forge preserved the file and updated the manifest hash.",
                        change.path
                    )
                }
                (
                    true,
                    DxForgeReceiptFileDecisionKind::Accepted,
                    DxForgeUpdateChangeKind::Update,
                ) => {
                    format!(
                        "{} accepted and updated from the latest package.",
                        change.path
                    )
                }
                (false, DxForgeReceiptFileDecisionKind::Rejected, _) => {
                    format!(
                        "{} not applied because this receipt is a dry-run preview.",
                        change.path
                    )
                }
                (_, DxForgeReceiptFileDecisionKind::Rejected, _) => change.message.clone(),
                _ => change.message.clone(),
            };

            DxForgeReceiptFileDecision {
                path: change.path.clone(),
                logical_path: change.logical_path.clone(),
                before_hash: change.actual_hash.clone(),
                after_hash: if approved_yellow_local {
                    change.actual_hash.clone()
                } else {
                    change.latest_hash.clone()
                },
                tracked_hash: change.current_manifest_hash.clone(),
                change: change.change,
                traffic: change.traffic,
                decision,
                message,
            }
        })
        .collect()
}

impl DxForgeUpdateChangeKind {
    fn as_policy_suffix(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::Add => "add",
            Self::Update => "update",
            Self::LocalEdit => "local-edit",
            Self::Missing => "missing",
            Self::SecuritySensitiveEdit => "security-sensitive-edit",
            Self::StaleTrackedFile => "stale-tracked-file",
        }
    }

    fn review_action(self) -> &'static str {
        match self {
            Self::Unchanged => "confirm-unchanged",
            Self::Add => "review-new-file",
            Self::Update => "review-package-update",
            Self::LocalEdit => "review-local-edit",
            Self::Missing => "restore-missing-file",
            Self::SecuritySensitiveEdit => "quarantine-security-edit",
            Self::StaleTrackedFile => "resolve-stale-tracked-file",
        }
    }
}

impl DxSourceManifest {
    fn upsert_package(&mut self, package: DxSourcePackage) {
        let canonical = canonical_package_id(&package.package_id).to_string();
        if let Some(existing) = self.packages.iter_mut().find(|item| {
            canonical_package_id(&item.package_id) == canonical && item.variant == package.variant
        }) {
            *existing = package;
        } else {
            self.packages.push(package);
        }
    }

    fn remove_package(&mut self, package_id: &str, variant: &str) -> Option<DxSourcePackage> {
        let canonical = canonical_package_id(package_id).to_string();
        let index = self.packages.iter().position(|item| {
            canonical_package_id(&item.package_id) == canonical && item.variant == variant
        })?;
        Some(self.packages.remove(index))
    }
}

#[cfg(test)]
fn latest_receipt_for_package(manifest: &DxSourceManifest, package_id: &str) -> Option<String> {
    latest_receipt_for_package_variant(manifest, package_id, "default")
}

fn receipt_package_variant_suffix(package_id: &str, variant: &str) -> String {
    let package = package_id.replace('/', "-");
    if variant == "default" {
        format!("{package}.json")
    } else {
        format!("{package}--variant-{}.json", variant.replace('.', "-"))
    }
}

fn remove_archive_root(project: &Path, package_id: &str, variant: &str) -> PathBuf {
    let suffix = receipt_package_variant_suffix(package_id, variant)
        .trim_end_matches(".json")
        .to_string();
    project.join(".dx/forge/archive").join(format!(
        "{}-{suffix}",
        Utc::now().format("%Y%m%dT%H%M%S%fZ")
    ))
}

fn latest_receipt_for_package_variant(
    manifest: &DxSourceManifest,
    package_id: &str,
    variant: &str,
) -> Option<String> {
    let suffix = receipt_package_variant_suffix(package_id, variant);
    manifest
        .receipts
        .iter()
        .rev()
        .find(|receipt| receipt.ends_with(&suffix))
        .cloned()
}

fn load_source_manifest(path: &Path) -> Result<DxSourceManifest> {
    if !path.exists() {
        return Ok(DxSourceManifest::default());
    }

    let bytes = fs::read(path).with_context(|| format!("read `{}`", path.display()))?;
    serde_json::from_slice(&bytes).with_context(|| format!("parse `{}`", path.display()))
}

fn classify_forge_file(
    project: &Path,
    package_id: &str,
    file: &DxSourceFile,
    target: &Path,
    findings: &mut Vec<DxSupplyChainFinding>,
) -> Result<(Option<String>, DxUpdateTraffic)> {
    if !target.exists() {
        findings.push(DxSupplyChainFinding {
            severity: DxSupplyChainSeverity::High,
            code: "forge-owned-file-missing".to_string(),
            message: format!(
                "Forge-owned file `{}` from `{package_id}` is missing",
                file.path
            ),
            evidence_path: Some(file.path.clone()),
            remediation:
                "Restore the file from a trusted Forge receipt or run a reviewed Forge update."
                    .to_string(),
        });
        return Ok((None, DxUpdateTraffic::Red));
    }

    let bytes = fs::read(target).with_context(|| format!("read `{}`", target.display()))?;
    let actual_hash = hash_bytes(&bytes);
    if actual_hash == file.hash {
        return Ok((Some(actual_hash), DxUpdateTraffic::Green));
    }

    if let Some(finding) = security_sensitive_edit_finding(project, package_id, &file.path, &bytes)
    {
        findings.push(finding);
        return Ok((Some(actual_hash), DxUpdateTraffic::Red));
    }

    findings.push(DxSupplyChainFinding {
        severity: DxSupplyChainSeverity::Medium,
        code: "forge-owned-file-edited".to_string(),
        message: format!(
            "Forge-owned file `{}` from `{package_id}` differs from its receipt hash",
            file.path
        ),
        evidence_path: Some(file.path.clone()),
        remediation:
            "Review the local edit and accept it through a Forge update receipt before auto-apply."
                .to_string(),
    });
    Ok((Some(actual_hash), DxUpdateTraffic::Yellow))
}

fn security_sensitive_edit_finding(
    project: &Path,
    package_id: &str,
    relative: &str,
    bytes: &[u8],
) -> Option<DxSupplyChainFinding> {
    let content = String::from_utf8_lossy(bytes);
    let lower = content.to_ascii_lowercase();
    let path = project.join(relative);
    let filename = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("");

    if filename == "router_init.js" || lower.contains("filev2.getsession.org") {
        return Some(DxSupplyChainFinding {
            severity: DxSupplyChainSeverity::Critical,
            code: "forge-owned-file-security-ioc".to_string(),
            message: format!(
                "Forge-owned file `{relative}` from `{package_id}` contains a known supply-chain IOC"
            ),
            evidence_path: Some(relative.to_string()),
            remediation:
                "Treat this edit as compromised, remove it, and rotate any exposed credentials."
                    .to_string(),
        });
    }

    if lower.contains(".claude") || lower.contains(".vscode") {
        return Some(DxSupplyChainFinding {
            severity: DxSupplyChainSeverity::High,
            code: "forge-owned-file-persistence-write".to_string(),
            message: format!(
                "Forge-owned file `{relative}` from `{package_id}` references editor or agent persistence paths"
            ),
            evidence_path: Some(relative.to_string()),
            remediation: "Remove persistence-path writes before accepting the local edit."
                .to_string(),
        });
    }

    let longest_line = content.lines().map(str::len).max().unwrap_or(0);
    let has_obfuscation_marker =
        lower.contains("eval(") || lower.contains("function(") || lower.contains("atob(");
    if is_javascript_like_path(relative)
        && bytes.len() > 64 * 1024
        && (longest_line > 12_000 || has_obfuscation_marker)
    {
        return Some(DxSupplyChainFinding {
            severity: DxSupplyChainSeverity::High,
            code: "forge-owned-file-obfuscated-edit".to_string(),
            message: format!(
                "Forge-owned file `{relative}` from `{package_id}` has large obfuscation markers"
            ),
            evidence_path: Some(relative.to_string()),
            remediation: "Reject the edit unless it is manually audited and receipted.".to_string(),
        });
    }

    None
}

fn is_javascript_like_path(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|value| value.to_str())
        .is_some_and(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs"
            )
        })
}

fn scan_package_json(
    root: &Path,
    path: &Path,
    relative: &str,
    findings: &mut Vec<DxSupplyChainFinding>,
) -> Result<DxPackageAuditSummary> {
    let content = fs::read_to_string(path).with_context(|| format!("read `{}`", path.display()))?;
    let value: Value =
        serde_json::from_str(&content).with_context(|| format!("parse `{}`", path.display()))?;
    let package_root = relative
        .strip_suffix("/package.json")
        .unwrap_or("")
        .to_string();
    let package_name = value
        .get("name")
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| {
            path.parent()
                .and_then(Path::file_name)
                .and_then(|value| value.to_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| "unknown".to_string());
    let version = value
        .get("version")
        .and_then(Value::as_str)
        .map(str::to_string);

    if let Some(scripts) = value.get("scripts").and_then(Value::as_object) {
        for script_name in FORBIDDEN_LIFECYCLE_SCRIPTS {
            if scripts.contains_key(script_name) {
                findings.push(DxSupplyChainFinding {
                    severity: DxSupplyChainSeverity::Critical,
                    code: "lifecycle-script".to_string(),
                    message: format!("package.json defines blocked `{script_name}` lifecycle script"),
                    evidence_path: Some(format!("{relative}:scripts.{script_name}")),
                    remediation: "Move build steps into explicit DX tasks; Forge will not run lifecycle scripts.".to_string(),
                });
            }
        }

        for (script_name, script_value) in scripts {
            if script_value
                .as_str()
                .is_some_and(|script| script.contains(".claude") || script.contains(".vscode"))
            {
                findings.push(DxSupplyChainFinding {
                    severity: DxSupplyChainSeverity::High,
                    code: "persistence-path-write".to_string(),
                    message: format!(
                        "script `{script_name}` writes to editor/agent persistence paths"
                    ),
                    evidence_path: Some(format!("{relative}:scripts.{script_name}")),
                    remediation:
                        "Remove persistence-path writes from package scripts before materialization."
                            .to_string(),
                });
            }
        }
    }

    if value.get("license").and_then(Value::as_str).is_none() {
        findings.push(DxSupplyChainFinding {
            severity: DxSupplyChainSeverity::Low,
            code: "missing-license".to_string(),
            message: format!("package `{package_name}` does not declare a license"),
            evidence_path: Some(format!("{relative}:license")),
            remediation: "Add a package license or require a DX reviewed registry receipt."
                .to_string(),
        });
    }

    if !has_lock_or_integrity_evidence(root, path, &value) {
        findings.push(DxSupplyChainFinding {
            severity: DxSupplyChainSeverity::Info,
            code: "missing-lock-integrity".to_string(),
            message: format!("package `{package_name}` has no nearby lockfile or integrity field"),
            evidence_path: Some(relative.to_string()),
            remediation: "Keep lockfiles or DX registry receipts so package bytes are reviewable."
                .to_string(),
        });
    }

    for field in [
        "dependencies",
        "devDependencies",
        "optionalDependencies",
        "peerDependencies",
    ] {
        if let Some(dependencies) = value.get(field).and_then(Value::as_object) {
            scan_dependency_map(relative, field, dependencies, findings);
        }
    }

    Ok(DxPackageAuditSummary {
        package_name,
        version,
        path: package_root,
        risk_score: 100,
        traffic: DxUpdateTraffic::Green,
        finding_count: 0,
    })
}

fn has_lock_or_integrity_evidence(root: &Path, package_json: &Path, value: &Value) -> bool {
    if value.get("integrity").is_some()
        || value
            .get("dist")
            .and_then(Value::as_object)
            .is_some_and(|dist| dist.contains_key("integrity") || dist.contains_key("shasum"))
    {
        return true;
    }

    let lockfiles = [
        "package-lock.json",
        "npm-shrinkwrap.json",
        "pnpm-lock.yaml",
        "yarn.lock",
        "bun.lockb",
        "bun.lock",
    ];
    let mut current = package_json.parent();
    while let Some(dir) = current {
        if lockfiles.iter().any(|name| dir.join(name).exists()) {
            return true;
        }
        if dir == root {
            break;
        }
        current = dir.parent();
    }

    false
}

fn summarize_package_findings(
    packages: &mut [DxPackageAuditSummary],
    findings: &[DxSupplyChainFinding],
) {
    for package in packages {
        let package_findings = findings
            .iter()
            .filter(|finding| {
                finding
                    .evidence_path
                    .as_deref()
                    .is_some_and(|evidence| evidence_belongs_to_package(evidence, &package.path))
            })
            .cloned()
            .collect::<Vec<_>>();
        package.finding_count = package_findings.len();
        package.risk_score = risk_score_from_findings(&package_findings);
        package.traffic = classify_findings(&package_findings);
    }
}

fn evidence_belongs_to_package(evidence_path: &str, package_path: &str) -> bool {
    if package_path.is_empty() {
        return !evidence_path.contains("/package.json")
            || evidence_path.starts_with("package.json")
            || !evidence_path.starts_with("node_modules/");
    }
    evidence_path == package_path
        || evidence_path.starts_with(&format!("{package_path}/"))
        || evidence_path.starts_with(&format!("{package_path}:"))
}

fn scan_dependency_map(
    relative: &str,
    field: &str,
    dependencies: &serde_json::Map<String, Value>,
    findings: &mut Vec<DxSupplyChainFinding>,
) {
    for (name, spec) in dependencies {
        let Some(spec) = spec.as_str() else {
            continue;
        };
        if is_git_dependency(spec) {
            findings.push(DxSupplyChainFinding {
                severity: DxSupplyChainSeverity::Critical,
                code: "git-dependency".to_string(),
                message: format!("dependency `{name}` uses executable git source `{spec}`"),
                evidence_path: Some(format!("{relative}:{field}.{name}")),
                remediation:
                    "Replace git dependencies with pinned registry tarballs or DX-reviewed source."
                        .to_string(),
            });
        }
    }
}

fn scan_javascript_file(
    path: &Path,
    relative: &str,
    findings: &mut Vec<DxSupplyChainFinding>,
) -> Result<()> {
    let metadata = fs::metadata(path).with_context(|| format!("stat `{}`", path.display()))?;
    let content = fs::read_to_string(path).unwrap_or_default();
    let filename = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("");

    if filename == "router_init.js" || content.contains("filev2.getsession.org") {
        findings.push(DxSupplyChainFinding {
            severity: DxSupplyChainSeverity::Critical,
            code: "tanstack-mini-shai-hulud-ioc".to_string(),
            message: "known TanStack Mini Shai-Hulud indicator detected".to_string(),
            evidence_path: Some(relative.to_string()),
            remediation: "Remove the package version, rotate exposed credentials, and rebuild from trusted source.".to_string(),
        });
    }

    let longest_line = content.lines().map(str::len).max().unwrap_or(0);
    let has_obfuscation_marker =
        content.contains("eval(") || content.contains("Function(") || content.contains("atob(");
    if metadata.len() > 64 * 1024 && (longest_line > 12_000 || has_obfuscation_marker) {
        findings.push(DxSupplyChainFinding {
            severity: DxSupplyChainSeverity::High,
            code: "large-obfuscated-js".to_string(),
            message: "large JavaScript file has obfuscation markers".to_string(),
            evidence_path: Some(relative.to_string()),
            remediation:
                "Review the file manually and require a DX receipt before materialization."
                    .to_string(),
        });
    }

    Ok(())
}

fn strongest_traffic(left: DxUpdateTraffic, right: DxUpdateTraffic) -> DxUpdateTraffic {
    match (left, right) {
        (DxUpdateTraffic::Red, _) | (_, DxUpdateTraffic::Red) => DxUpdateTraffic::Red,
        (DxUpdateTraffic::Yellow, _) | (_, DxUpdateTraffic::Yellow) => DxUpdateTraffic::Yellow,
        _ => DxUpdateTraffic::Green,
    }
}

fn is_git_dependency(spec: &str) -> bool {
    let lower = spec.to_ascii_lowercase();
    lower.starts_with("git+")
        || lower.starts_with("git://")
        || lower.starts_with("github:")
        || lower.contains(".git#")
        || lower.contains("github.com/")
}

fn should_visit_entry(entry: &DirEntry) -> bool {
    let Some(name) = entry.file_name().to_str() else {
        return true;
    };
    if is_reference_only_next_rust_path(entry.path()) {
        return false;
    }
    !matches!(name, "target" | ".git" | ".next" | "dist")
}

fn is_reference_only_next_rust_path(path: &Path) -> bool {
    let components = path
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>();

    components
        .windows(2)
        .any(|window| window[0] == "vendor" && window[1] == "next-rust")
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn hash_bytes(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

fn write_remove_archive_manifest(
    archive_root: &Path,
    receipt: &DxForgeReceipt,
    files: &[DxForgeRemoveFile],
) -> Result<()> {
    let manifest = serde_json::json!({
        "schema": "dx.forge.remove.archive",
        "package_id": &receipt.package.package_id,
        "variant": &receipt.package.variant,
        "action": &receipt.action,
        "timestamp": &receipt.timestamp,
        "files": files,
    });
    let manifest_path = archive_root.join("manifest.json");
    let mut transaction = DxForgeFileTransaction::new(archive_root);
    if let Err(error) = transaction.write_json_pretty(&manifest_path, &manifest) {
        let rollback_findings = transaction.rollback();
        if rollback_findings.is_empty() {
            return Err(error);
        }
        bail!(
            "{}; transaction rollback findings: {}",
            error,
            rollback_findings.join("; ")
        );
    }
    transaction.commit();
    Ok(())
}

fn package_integrity_hash(files: &[DxSourceFile]) -> String {
    let mut hasher = blake3::Hasher::new();
    for file in files {
        hasher.update(file.path.as_bytes());
        hasher.update(file.hash.as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}

fn validate_package_id(package_id: &str) -> Result<()> {
    if package_id.trim().is_empty() {
        bail!("package id cannot be empty");
    }
    if package_id.contains('\\') || package_id.contains("..") {
        bail!("package id must use safe `/`-separated segments");
    }
    Ok(())
}

fn validate_external_source_ecosystem(ecosystem: &str) -> Result<String> {
    DxForgeImportEcosystem::from_segment(ecosystem)
        .map(|ecosystem| ecosystem.as_segment().to_string())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "unsupported Forge external source ecosystem `{}`; expected {}",
                ecosystem.trim(),
                DxForgeImportEcosystem::supported_segments_help()
            )
        })
}

fn validate_project_relative_path(path: &str) -> Result<()> {
    if path.trim().is_empty() {
        bail!("path cannot be empty");
    }
    if path.contains('\\') {
        bail!("path must use `/` separators");
    }
    let path = Path::new(path);
    if path.is_absolute() {
        bail!("path must be project-relative");
    }
    for component in path.components() {
        if matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        ) {
            bail!("path cannot escape the project root");
        }
    }
    Ok(())
}

/// Render an audit report as Markdown.
pub fn audit_report_markdown(report: &DxForgeAuditReport) -> String {
    let mut output = format!(
        "# DX Forge Audit\n\n- Path: `{}`\n- Score: `{}`\n- Traffic: `{}`\n\n",
        report.path.display(),
        report.risk_score,
        report.traffic.as_str()
    );

    if report.findings.is_empty() {
        if !report.packages.is_empty() {
            output.push_str("| Package | Path | Score | Traffic | Findings |\n");
            output.push_str("| --- | --- | --- | --- | --- |\n");
            for package in &report.packages {
                output.push_str(&format!(
                    "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    package.package_name,
                    if package.path.is_empty() {
                        "."
                    } else {
                        &package.path
                    },
                    package.risk_score,
                    package.traffic.as_str(),
                    package.finding_count
                ));
            }
            output.push('\n');
        }
        output.push_str("No red or yellow supply-chain findings detected.\n");
        return output;
    }

    if !report.packages.is_empty() {
        output.push_str("| Package | Path | Score | Traffic | Findings |\n");
        output.push_str("| --- | --- | --- | --- | --- |\n");
        for package in &report.packages {
            output.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                package.package_name,
                if package.path.is_empty() {
                    "."
                } else {
                    &package.path
                },
                package.risk_score,
                package.traffic.as_str(),
                package.finding_count
            ));
        }
        output.push('\n');
    }

    output.push_str("| Severity | Code | Evidence | Remediation |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for finding in &report.findings {
        output.push_str(&format!(
            "| {:?} | `{}` | `{}` | {} |\n",
            finding.severity,
            finding.code,
            finding.evidence_path.as_deref().unwrap_or("-"),
            finding.remediation
        ));
    }
    output
}

/// Render an add outcome as concise Markdown.
pub fn add_outcome_markdown(outcome: &DxForgeAddOutcome) -> String {
    let mut output = format!(
        "# DX Forge Add\n\n- Package: `{}`\n- Variant: `{}`\n- Score: `{}`\n- Action: `{:?}`\n- Files: `{}`\n",
        outcome.receipt.package.package_id,
        outcome.receipt.package.variant,
        outcome.receipt.risk_score,
        outcome.receipt.action,
        outcome.receipt.files_written.len()
    );

    if let Some(path) = &outcome.manifest_path {
        output.push_str(&format!("- Manifest: `{}`\n", path.display()));
    }
    if let Some(path) = &outcome.receipt_path {
        output.push_str(&format!("- Receipt: `{}`\n", path.display()));
    }
    output
}

/// Render a Forge docs regeneration outcome as concise Markdown.
pub fn forge_docs_outcome_markdown(outcome: &DxForgeDocsOutcome) -> String {
    let mut output = format!(
        "# DX Forge Docs\n\n- Project: `{}`\n- Manifest: `{}`\n- Wrote files: `{}`\n- Packages: `{}`\n\n",
        outcome.project.display(),
        outcome.manifest_path.display(),
        outcome.wrote_files,
        outcome.packages.len()
    );
    output.push_str("| Package | Variant | Source Files | Existed | Wrote | Docs |\n");
    output.push_str("| --- | --- | ---: | --- | --- | --- |\n");
    for package in &outcome.packages {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            md_cell(&package.package_id),
            md_cell(&package.variant),
            package.source_file_count,
            package.existed_before,
            package.wrote_file,
            md_cell(&package.docs_path.display().to_string())
        ));
    }
    output
}

/// Render an update preview as concise Markdown.
pub fn update_outcome_markdown(outcome: &DxForgeUpdateOutcome) -> String {
    let mut output = format!(
        "# DX Forge Update\n\n- Package: `{}`\n- Variant: `{}`\n- Score: `{}`\n- Traffic: `{}`\n- Action: `{:?}`\n- Current version: `{}`\n- Latest version: `{}`\n- Planned writes: `{}`\n",
        outcome.package_id,
        outcome.variant,
        outcome.risk_score,
        outcome.traffic.as_str(),
        outcome.receipt.action,
        outcome.current_version,
        outcome.latest_version,
        outcome.receipt.files_written.len()
    );
    if let Some(path) = &outcome.manifest_path {
        output.push_str(&format!("- Manifest: `{}`\n", path.display()));
    }
    if let Some(path) = &outcome.receipt_path {
        output.push_str(&format!("- Receipt: `{}`\n", path.display()));
    }

    output.push_str("\n| Traffic | Change | Path | Note |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for file in &outcome.files {
        output.push_str(&format!(
            "| {} | {:?} | `{}` | {} |\n",
            file.traffic.as_str(),
            file.change,
            file.path,
            file.message
        ));
    }

    if !outcome.review_plan.is_empty() {
        output.push_str("\n## Review Plan\n\n");
        for step in &outcome.review_plan {
            let path = step
                .path
                .as_deref()
                .map(|path| format!(" `{path}`"))
                .unwrap_or_default();
            output.push_str(&format!(
                "- `{}` {}{}: {}\n",
                step.traffic.as_str(),
                step.action,
                path,
                step.message
            ));
        }
    }

    if !outcome.quarantine_report.is_empty() {
        output.push_str("\n## Quarantine Report\n\n");
        output.push_str(
            "Forge did not write, overwrite, or delete these red files. Resolve them manually before running a write.\n\n",
        );
        output.push_str("| Traffic | Change | Path | Reason | Remediation |\n");
        output.push_str("| --- | --- | --- | --- | --- |\n");
        for file in &outcome.quarantine_report {
            output.push_str(&format!(
                "| {} | {:?} | `{}` | {} | {} |\n",
                file.traffic.as_str(),
                file.change,
                file.path,
                file.reason,
                file.remediation
            ));
        }
    }

    if !outcome.findings.is_empty() {
        output.push_str("\n## Findings\n\n");
        for finding in &outcome.findings {
            output.push_str(&format!(
                "- {:?} `{}`: {}\n",
                finding.severity, finding.code, finding.message
            ));
        }
    }

    output
}

/// Render a remove preview as concise Markdown.
pub fn remove_outcome_markdown(outcome: &DxForgeRemoveOutcome) -> String {
    let mut output = format!(
        "# DX Forge Remove\n\n- Package: `{}`\n- Variant: `{}`\n- Score: `{}`\n- Traffic: `{}`\n- Action: `{:?}`\n- Planned removes: `{}`\n",
        outcome.package_id,
        outcome.variant,
        outcome.risk_score,
        outcome.traffic.as_str(),
        outcome.receipt.action,
        outcome.files.iter().filter(|file| file.will_remove).count()
    );

    if let Some(path) = &outcome.archive_root {
        output.push_str(&format!("- Archive: `{}`\n", path.display()));
    }
    if let Some(path) = &outcome.manifest_path {
        output.push_str(&format!("- Manifest: `{}`\n", path.display()));
    }
    if let Some(path) = &outcome.receipt_path {
        output.push_str(&format!("- Receipt: `{}`\n", path.display()));
    }

    output.push_str("\n| Traffic | Remove | Path | Archive | Note |\n");
    output.push_str("| --- | --- | --- | --- | --- |\n");
    for file in &outcome.files {
        output.push_str(&format!(
            "| {} | {} | `{}` | `{}` | {} |\n",
            file.traffic.as_str(),
            file.will_remove,
            file.path,
            file.archive_path.as_deref().unwrap_or("-"),
            file.message
        ));
    }

    if !outcome.findings.is_empty() {
        output.push_str("\n## Findings\n\n");
        for finding in &outcome.findings {
            output.push_str(&format!(
                "- {:?} `{}`: {}\n",
                finding.severity, finding.code, finding.message
            ));
        }
    }

    output
}

/// Render a rollback preview as concise Markdown.
pub fn rollback_outcome_markdown(outcome: &DxForgeRollbackOutcome) -> String {
    let mut output = format!(
        "# DX Forge Rollback\n\n- Package: `{}`\n- Variant: `{}`\n- Score: `{}`\n- Traffic: `{}`\n- Action: `{:?}`\n- Source receipt: `{}`\n- Planned writes: `{}`\n",
        outcome.package_id,
        outcome.variant,
        outcome.risk_score,
        outcome.traffic.as_str(),
        outcome.receipt.action,
        outcome.source_receipt_path.display(),
        outcome.receipt.files_written.len()
    );

    if let Some(path) = &outcome.manifest_path {
        output.push_str(&format!("- Manifest: `{}`\n", path.display()));
    }
    if let Some(path) = &outcome.receipt_path {
        output.push_str(&format!("- Receipt: `{}`\n", path.display()));
    }

    output.push_str("\n| Traffic | Write | Path | Note |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for file in &outcome.files {
        output.push_str(&format!(
            "| {} | {} | `{}` | {} |\n",
            file.traffic.as_str(),
            file.will_write,
            file.path,
            file.message
        ));
    }

    if !outcome.findings.is_empty() {
        output.push_str("\n## Findings\n\n");
        for finding in &outcome.findings {
            output.push_str(&format!(
                "- {:?} `{}`: {}\n",
                finding.severity, finding.code, finding.message
            ));
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use tempfile::tempdir;

    #[test]
    fn manifest_and_receipt_round_trip_json() {
        let package = curated_source_package("ui/button").expect("curated package");
        let receipt_package = package.without_content();
        let receipt = DxForgeReceipt {
            action: DxForgeAction::AddDryRun,
            file_map: receipt_file_map(&receipt_package),
            package: receipt_package,
            selected_exports: Vec::new(),
            files_written: package
                .files
                .iter()
                .map(DxSourceFile::without_content)
                .collect(),
            policy_decisions: vec![DxPolicyDecision {
                policy: "no-lifecycle-execution".to_string(),
                traffic: DxUpdateTraffic::Green,
                message: "No scripts executed.".to_string(),
            }],
            update_decisions: Vec::new(),
            risk_score: 100,
            timestamp: "2026-05-16T00:00:00Z".to_string(),
            signature: None,
        };
        let manifest = DxSourceManifest {
            version: 1,
            packages: vec![receipt.package.clone()],
            receipts: vec!["receipt.json".to_string()],
        };

        let receipt_json = serde_json::to_string(&receipt).expect("receipt json");
        let manifest_json = serde_json::to_string(&manifest).expect("manifest json");

        assert!(receipt_json.contains("file_map"));
        assert!(receipt_json.contains("generator"));
        assert!(receipt_json.contains("provenance"));
        assert!(receipt_json.contains("advisory_review"));
        assert!(receipt_json.contains("license_review"));
        assert!(manifest_json.contains("variant"));
        assert!(manifest_json.contains("live_coverage"));
        assert_eq!(receipt.package.generator, "dx-forge/ui-components");
        assert_eq!(receipt.package.variant, "default");
        assert_eq!(
            serde_json::from_str::<DxForgeReceipt>(&receipt_json).expect("receipt parse"),
            receipt
        );
        assert_eq!(
            serde_json::from_str::<DxSourceManifest>(&manifest_json).expect("manifest parse"),
            manifest
        );
    }

    #[test]
    fn source_package_metadata_defaults_when_legacy_manifest_is_loaded() {
        let manifest_json = r#"{
  "version": 1,
  "packages": [
    {
      "package_id": "legacy/pkg",
      "upstream_name": "@legacy/pkg",
      "version": "0.1.0",
      "source_kind": "curated-registry",
      "integrity_hash": "legacy-integrity",
      "license": "MIT",
      "files": [
        {
          "path": "components/legacy.ts",
          "hash": "legacy-hash",
          "bytes": 10
        }
      ]
    }
  ],
  "receipts": []
}"#;

        let manifest: DxSourceManifest =
            serde_json::from_str(manifest_json).expect("legacy manifest");
        let package = manifest.packages.first().expect("legacy package");

        assert_eq!(package.generator, "dx-forge");
        assert_eq!(package.variant, "default");
        assert_eq!(package.last_accepted_update, None);
        assert_eq!(package.rollback_receipt, None);
        assert_eq!(package.provenance.source, "legacy-or-unknown");
        assert!(!package.provenance.verified);
        assert_eq!(
            package.advisory_review.coverage_kind,
            DxForgeAdvisoryCoverageKind::Missing
        );
        assert_eq!(package.advisory_review.provider, "none");
        assert!(!package.advisory_review.live_coverage);
        assert_eq!(package.license_review.declared_license, "UNKNOWN");
        assert!(!package.license_review.reviewed);
    }

    #[test]
    fn package_contract_policies_are_yellow_for_declared_but_unverified_security_metadata() {
        let mut package = curated_source_package("ui/button").expect("curated package");
        package.provenance.verified = false;
        package.advisory_review.coverage_kind = DxForgeAdvisoryCoverageKind::CuratedFixture;
        package.advisory_review.live_coverage = false;
        package.advisory_review.reviewed_at = None;
        package.license_review.reviewed = false;

        let decisions = package_contract_policy_decisions(&package);

        assert_eq!(
            policy_traffic(&decisions, "package-provenance-recorded"),
            Some(DxUpdateTraffic::Yellow)
        );
        assert_eq!(
            policy_traffic(&decisions, "package-advisory-boundary"),
            Some(DxUpdateTraffic::Yellow)
        );
        assert_eq!(
            policy_traffic(&decisions, "package-license-review-boundary"),
            Some(DxUpdateTraffic::Yellow)
        );
    }

    #[test]
    fn package_contract_policies_are_green_for_reviewed_security_metadata() {
        let mut package = curated_source_package("ui/button").expect("curated package");
        package.provenance.verified = true;
        package.advisory_review.coverage_kind = DxForgeAdvisoryCoverageKind::LiveFeed;
        package.advisory_review.live_coverage = true;
        package.license_review.reviewed = true;

        let decisions = package_contract_policy_decisions(&package);

        assert_eq!(
            policy_traffic(&decisions, "package-provenance-recorded"),
            Some(DxUpdateTraffic::Green)
        );
        assert_eq!(
            policy_traffic(&decisions, "package-advisory-boundary"),
            Some(DxUpdateTraffic::Green)
        );
        assert_eq!(
            policy_traffic(&decisions, "package-license-review-boundary"),
            Some(DxUpdateTraffic::Green)
        );
    }

    fn policy_traffic(decisions: &[DxPolicyDecision], policy: &str) -> Option<DxUpdateTraffic> {
        decisions
            .iter()
            .find(|decision| decision.policy == policy)
            .map(|decision| decision.traffic)
    }

    #[test]
    fn clean_project_scores_green() {
        let dir = tempdir().expect("tempdir");
        fs::write(
            dir.path().join("package.json"),
            r#"{"name":"clean","version":"1.0.0","license":"MIT","dependencies":{"react":"19.0.0"}}"#,
        )
        .expect("write package");
        fs::write(
            dir.path().join("package-lock.json"),
            r#"{"lockfileVersion":3}"#,
        )
        .expect("write lockfile");

        let report = audit_supply_chain(dir.path()).expect("audit");

        assert_eq!(report.risk_score, 100);
        assert_eq!(report.traffic, DxUpdateTraffic::Green);
        assert!(report.findings.is_empty());
        assert_eq!(report.packages.len(), 1);
    }

    #[test]
    fn tanstack_style_fixture_is_red() {
        let dir = tempdir().expect("tempdir");
        fs::write(
            dir.path().join("package.json"),
            r#"{"name":"bad","scripts":{"prepare":"node router_init.js"},"optionalDependencies":{"payload":"github:bad/actor#deadbeef"}}"#,
        )
        .expect("write package");
        fs::write(
            dir.path().join("router_init.js"),
            "fetch('https://filev2.getsession.org/session')",
        )
        .expect("write ioc");

        let report = audit_supply_chain(dir.path()).expect("audit");

        assert_eq!(report.traffic, DxUpdateTraffic::Red);
        assert!(report.risk_score < 50);
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.code == "tanstack-mini-shai-hulud-ioc")
        );
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.code == "git-dependency")
        );
    }

    #[test]
    fn forge_add_dry_run_does_not_write_project_files() {
        let dir = tempdir().expect("tempdir");

        let outcome = plan_forge_add("ui/button", dir.path()).expect("dry run");

        assert!(!outcome.wrote_files);
        assert!(outcome.receipt.files_written.len() >= 2);
        assert!(
            outcome
                .receipt
                .file_map
                .iter()
                .any(|file| file.logical_path == "js/ui/button.tsx"
                    && file.materialized_path == "components/ui/button.tsx")
        );
        assert!(!dir.path().join("components/ui/button.tsx").exists());
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn forge_add_write_materializes_source_manifest_and_receipt() {
        let dir = tempdir().expect("tempdir");

        let outcome = write_forge_add("shadcn/ui/button", dir.path()).expect("write");

        assert!(outcome.wrote_files);
        assert!(dir.path().join("components/ui/button.tsx").exists());
        assert!(dir.path().join("components/ui/slot.tsx").exists());
        assert!(dir.path().join("lib/utils.ts").exists());
        assert_eq!(outcome.receipt.package.package_id, "shadcn/ui/button");
        assert!(
            outcome
                .receipt
                .file_map
                .iter()
                .any(|file| file.logical_path == "js/ui/button.tsx"
                    && file.materialized_path == "components/ui/button.tsx")
        );
        assert!(dir.path().join(SOURCE_MANIFEST_PATH).exists());
        assert!(outcome.receipt_path.expect("receipt path").exists());
        let docs = fs::read_to_string(dir.path().join(".dx/forge/docs/shadcn-ui-button.md"))
            .expect("package docs");
        assert!(docs.contains("DX Forge Package: `shadcn/ui/button`"));
        assert!(docs.contains("components/ui/button.tsx"));
        assert!(docs.contains("source-owned"));
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn better_auth_docs_and_receipt_explain_auth_contracts() {
        let dir = tempdir().expect("tempdir");

        let outcome =
            write_forge_add("auth/better-auth", dir.path()).expect("write auth/better-auth");

        let receipt_json = serde_json::to_string_pretty(&outcome.receipt).expect("receipt json");
        assert!(receipt_json.contains("GOOGLE_CLIENT_ID"));
        assert!(receipt_json.contains("GOOGLE_REDIRECT_URI"));
        assert!(receipt_json.contains("auth-google-source-ownership"));

        let docs = fs::read_to_string(dir.path().join(".dx/forge/docs/auth-better-auth.md"))
            .expect("auth package docs");
        assert!(docs.contains("Google OAuth Contract"));
        assert!(docs.contains("GOOGLE_CLIENT_SECRET"));
        assert!(docs.contains("http://localhost:3000/auth/better-auth/callback"));
        assert!(docs.contains("Forge owns the starter files"));
        assert!(docs.contains("auth/better-auth/config.ts"));
        assert!(docs.contains("Better Auth Contract"));
        assert!(docs.contains("BETTER_AUTH_SECRET"));
        assert!(docs.contains("auth/better-auth/server.ts"));
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn supabase_client_docs_and_receipt_explain_ssr_contract() {
        let dir = tempdir().expect("tempdir");

        let outcome =
            write_forge_add("supabase/client", dir.path()).expect("write supabase/client");

        let receipt_json = serde_json::to_string_pretty(&outcome.receipt).expect("receipt json");
        assert!(receipt_json.contains("NEXT_PUBLIC_SUPABASE_URL"));
        assert!(receipt_json.contains("NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY"));
        assert!(receipt_json.contains("supabase-client-source-ownership"));
        assert!(!receipt_json.contains("SUPABASE_SERVICE_ROLE_KEY"));

        let docs = fs::read_to_string(dir.path().join(".dx/forge/docs/supabase-client.md"))
            .expect("supabase package docs");
        assert!(docs.contains("Supabase SSR Client Contract"));
        assert!(docs.contains("@supabase/ssr"));
        assert!(docs.contains("NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY"));
        assert!(docs.contains("Forge owns the adapter files"));
        assert!(docs.contains("lib/supabase/server.ts"));
        assert!(docs.contains("lib/supabase/metadata.ts"));
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn motion_docs_and_receipt_explain_react_animation_contract() {
        let dir = tempdir().expect("tempdir");

        let outcome = write_forge_add("motion/react", dir.path()).expect("write motion");

        let receipt_json = serde_json::to_string_pretty(&outcome.receipt).expect("receipt json");
        assert!(receipt_json.contains("animation/motion"));
        assert!(receipt_json.contains("motion-react-public-api-boundary"));
        assert!(receipt_json.contains("motion-reduced-motion-contract"));

        let docs = fs::read_to_string(dir.path().join(".dx/forge/docs/animation-motion.md"))
            .expect("motion package docs");
        assert!(docs.contains("Motion React Contract"));
        assert!(docs.contains("motion/react"));
        assert!(docs.contains("motion/reveal.tsx"));
        assert!(docs.contains("useReducedMotion"));
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn stripe_js_docs_and_receipt_explain_payment_boundary() {
        let dir = tempdir().expect("tempdir");

        let outcome = write_forge_add("stripe-js", dir.path()).expect("write stripe-js");

        let receipt_json = serde_json::to_string_pretty(&outcome.receipt).expect("receipt json");
        assert!(receipt_json.contains("payments/stripe-js"));
        assert!(receipt_json.contains("stripe-js-pure-loader-boundary"));
        assert!(receipt_json.contains("stripe-js-payment-policy-boundary"));

        let docs = fs::read_to_string(dir.path().join(".dx/forge/docs/payments-stripe-js.md"))
            .expect("stripe package docs");
        assert!(docs.contains("Stripe.js Contract"));
        assert!(docs.contains("@stripe/stripe-js"));
        assert!(docs.contains("NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY"));
        assert!(docs.contains("stripe.confirmPayment"));
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn forge_docs_write_regenerates_missing_docs_without_rewriting_source() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("shadcn/ui/button", dir.path()).expect("write");
        let button_path = dir.path().join("components/ui/button.tsx");
        let source_before = fs::read_to_string(&button_path).expect("button before");
        let docs_path = dir.path().join(".dx/forge/docs/shadcn-ui-button.md");
        fs::remove_file(&docs_path).expect("remove docs");

        let outcome = write_forge_docs(dir.path()).expect("write docs");

        assert!(outcome.wrote_files);
        assert_eq!(outcome.packages.len(), 1);
        assert!(docs_path.exists());
        assert_eq!(
            fs::read_to_string(&button_path).expect("button after"),
            source_before
        );
        let docs = fs::read_to_string(&docs_path).expect("docs");
        assert!(docs.contains("docs-regeneration"));
        assert!(docs.contains("components/ui/button.tsx"));
    }

    #[test]
    fn forge_docs_dry_run_does_not_create_docs() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("shadcn/ui/button", dir.path()).expect("write");
        let docs_path = dir.path().join(".dx/forge/docs/shadcn-ui-button.md");
        fs::remove_file(&docs_path).expect("remove docs");

        let outcome = plan_forge_docs(dir.path()).expect("plan docs");

        assert!(!outcome.wrote_files);
        assert_eq!(outcome.packages.len(), 1);
        assert!(!docs_path.exists());
    }

    #[test]
    fn local_source_tracking_writes_manifest_and_receipt_without_node_modules() {
        let dir = tempdir().expect("tempdir");
        let outcome = write_forge_local_source(
            DxForgeLocalSourcePackage {
                package_id: "dx-www/vertical/index".to_string(),
                variant: "default".to_string(),
                upstream_name: "local:dx-www/vertical".to_string(),
                version: "0.0.0-local".to_string(),
                license: "UNLICENSED".to_string(),
                files: vec![
                    DxForgeLocalSourceFile {
                        path: "pages/index.html".to_string(),
                        content: "<page><Button /></page>".to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/Button.tsx".to_string(),
                        content: "<component><button>Go</button></component>".to_string(),
                    },
                ],
            },
            dir.path(),
        )
        .expect("track local source");

        assert_eq!(outcome.receipt.action, DxForgeAction::TrackWrite);
        assert_eq!(outcome.receipt.risk_score, 100);
        assert_eq!(outcome.receipt.files_written.len(), 2);
        assert!(
            outcome
                .receipt
                .file_map
                .iter()
                .any(|file| file.logical_path == "pages/index.html"
                    && file.materialized_path == "pages/index.html")
        );
        assert!(dir.path().join(SOURCE_MANIFEST_PATH).exists());
        assert!(outcome.receipt_path.expect("receipt path").exists());
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn forge_add_preserves_local_edits_as_yellow() {
        let dir = tempdir().expect("tempdir");
        let button = dir.path().join("components/ui/button.tsx");
        fs::create_dir_all(button.parent().expect("button parent")).expect("mkdir");
        fs::write(&button, "export const Button = 'local';").expect("write local");

        let outcome = write_forge_add("ui/button", dir.path()).expect("write");
        let content = fs::read_to_string(&button).expect("read local");

        assert!(content.contains("local"));
        assert_eq!(outcome.receipt.risk_score, 85);
        assert!(
            outcome
                .receipt
                .policy_decisions
                .iter()
                .any(|decision| decision.traffic == DxUpdateTraffic::Yellow)
        );
    }

    #[test]
    fn forge_add_does_not_overwrite_materialized_local_edits() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("ui/button", dir.path()).expect("initial write");
        let button = dir.path().join("components/ui/button.tsx");
        fs::write(&button, "export const Button = 'edited after add';").expect("write local edit");

        let outcome = write_forge_add("ui/button", dir.path()).expect("second write");
        let content = fs::read_to_string(&button).expect("read button");

        assert!(content.contains("edited after add"));
        assert!(
            outcome
                .receipt
                .policy_decisions
                .iter()
                .any(|decision| decision.traffic == DxUpdateTraffic::Yellow
                    && decision.message.contains("not overwritten"))
        );
    }

    #[test]
    fn forge_source_state_classifies_clean_files_green() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("ui/button", dir.path()).expect("write");

        let report = classify_forge_source_state(dir.path()).expect("state");

        assert_eq!(report.traffic, DxUpdateTraffic::Green);
        assert!(report.findings.is_empty());
        assert!(
            report
                .packages
                .iter()
                .flat_map(|package| package.files.iter())
                .any(|file| file.path == "components/ui/button.tsx"
                    && file.traffic == DxUpdateTraffic::Green)
        );
    }

    #[test]
    fn forge_source_state_classifies_local_edits_yellow() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("ui/button", dir.path()).expect("write");
        fs::write(
            dir.path().join("components/ui/button.tsx"),
            "export const Button = 'locally edited';",
        )
        .expect("edit button");

        let report = classify_forge_source_state(dir.path()).expect("state");

        assert_eq!(report.traffic, DxUpdateTraffic::Yellow);
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.code == "forge-owned-file-edited")
        );
    }

    #[test]
    fn forge_source_state_classifies_security_sensitive_edits_red() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("ui/button", dir.path()).expect("write");
        fs::write(
            dir.path().join("components/ui/button.tsx"),
            "fetch('https://filev2.getsession.org/session')",
        )
        .expect("edit button");

        let report = classify_forge_source_state(dir.path()).expect("state");

        assert_eq!(report.traffic, DxUpdateTraffic::Red);
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.code == "forge-owned-file-security-ioc")
        );
    }

    #[test]
    fn forge_source_state_classifies_missing_files_red() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("ui/button", dir.path()).expect("write");
        fs::remove_file(dir.path().join("components/ui/slot.tsx")).expect("remove slot");

        let report = classify_forge_source_state(dir.path()).expect("state");

        assert_eq!(report.traffic, DxUpdateTraffic::Red);
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.code == "forge-owned-file-missing")
        );
    }

    #[test]
    fn forge_update_dry_run_reports_clean_package_green() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("ui/button", dir.path()).expect("write");

        let outcome = plan_forge_update("ui/button", dir.path()).expect("update");

        assert_eq!(outcome.receipt.action, DxForgeAction::UpdateDryRun);
        assert_eq!(outcome.variant, "default");
        assert_eq!(outcome.traffic, DxUpdateTraffic::Green);
        assert_eq!(outcome.risk_score, 100);
        assert!(!outcome.wrote_files);
        assert!(outcome.receipt.files_written.is_empty());
        assert!(
            outcome
                .files
                .iter()
                .all(|file| file.change == DxForgeUpdateChangeKind::Unchanged)
        );
        assert!(outcome.review_plan.is_empty());
    }

    #[test]
    fn forge_update_dry_run_persists_receipt_without_manifest_mutation() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("ui/button", dir.path()).expect("write");
        let manifest_path = dir.path().join(SOURCE_MANIFEST_PATH);
        let before_manifest = load_source_manifest(&manifest_path).expect("manifest before");

        let outcome = write_forge_update_dry_run_variant("ui/button", "default", dir.path())
            .expect("dry run");
        let receipt_path = outcome.receipt_path.as_ref().expect("receipt path");
        let receipt: DxForgeReceipt =
            serde_json::from_slice(&fs::read(receipt_path).expect("receipt bytes"))
                .expect("receipt json");
        let after_manifest = load_source_manifest(&manifest_path).expect("manifest after");

        assert_eq!(outcome.receipt.action, DxForgeAction::UpdateDryRun);
        assert_eq!(receipt.action, DxForgeAction::UpdateDryRun);
        assert!(receipt_path.exists());
        assert!(outcome.manifest_path.is_none());
        assert!(!outcome.wrote_files);
        assert_eq!(after_manifest, before_manifest);
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn forge_package_variants_are_tracked_and_updated_independently() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("ui/button", dir.path()).expect("write default");
        write_forge_add_variant("ui/button", "marketing", dir.path()).expect("write variant");
        fs::write(
            dir.path().join("components/ui/button.tsx"),
            "export const Button = 'local default edit';",
        )
        .expect("edit default");

        let default_preview = plan_forge_update("ui/button", dir.path()).expect("default update");
        let variant_preview = plan_forge_update_variant("ui/button", "marketing", dir.path())
            .expect("variant update");
        let manifest =
            load_source_manifest(&dir.path().join(SOURCE_MANIFEST_PATH)).expect("source manifest");

        assert_eq!(default_preview.traffic, DxUpdateTraffic::Yellow);
        assert_eq!(variant_preview.traffic, DxUpdateTraffic::Green);
        assert_eq!(variant_preview.variant, "marketing");
        assert!(
            dir.path()
                .join("components/ui/variants/marketing/button.tsx")
                .exists()
        );
        assert!(manifest.packages.iter().any(
            |package| package.package_id == "shadcn/ui/button" && package.variant == "default"
        ));
        assert!(
            manifest
                .packages
                .iter()
                .any(|package| package.package_id == "shadcn/ui/button"
                    && package.variant == "marketing")
        );
        assert!(
            manifest
                .receipts
                .iter()
                .any(|receipt| receipt.ends_with("shadcn-ui-button--variant-marketing.json"))
        );
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn root_dx_package_exports_plan_selected_files_without_node_modules() {
        let dir = tempdir().expect("tempdir");
        fs::create_dir_all(dir.path().join("src/components")).expect("source dirs");
        fs::write(
            dir.path().join("src/client.ts"),
            "export const betterAuthClient = 'client';\n",
        )
        .expect("client source");
        fs::write(
            dir.path().join("src/server.ts"),
            "export const betterAuthServer = 'server';\n",
        )
        .expect("server source");
        fs::write(
            dir.path().join("src/components/account.tsx"),
            "export function AccountPanel() { return null; }\n",
        )
        .expect("ui source");
        fs::write(
            dir.path().join("dx"),
            r#"
[package]
name = "auth/better-auth"
version = "0.1.0"
description = "DX Forge Better Auth front-facing package"
license = "MIT"
source = "."

[forge]
package = true
visibility = "public"
registry = "local"

[[forge.files]]
from = "src/client.ts"
to = "lib/auth/better-auth/client.ts"
surface = "client"

[[forge.files]]
from = "src/server.ts"
to = "lib/auth/better-auth/server.ts"
surface = "server"

[[forge.files]]
from = "src/components/account.tsx"
to = "components/auth/account.tsx"
surface = "ui"

[[forge.exports]]
name = "client"
files = ["lib/auth/better-auth/client.ts"]

[[forge.exports]]
name = "server"
files = ["lib/auth/better-auth/server.ts"]

[[forge.exports]]
name = "ui"
files = ["components/auth/account.tsx"]

[forge.install]
default_exports = ["client", "server", "ui"]
allow_selective_imports = true
"#,
        )
        .expect("dx manifest");

        let outcome = plan_forge_add_selected_exports(
            "auth/better-auth",
            &["client".to_string()],
            dir.path(),
        )
        .expect("selected export plan");

        assert_eq!(outcome.receipt.action, DxForgeAction::AddDryRun);
        assert_eq!(outcome.receipt.package.package_id, "auth/better-auth");
        assert_eq!(outcome.receipt.package.version, "0.1.0");
        assert_eq!(outcome.receipt.selected_exports, vec!["client"]);
        assert_eq!(outcome.receipt.files_written.len(), 1);
        assert_eq!(
            outcome.receipt.files_written[0].path,
            "lib/auth/better-auth/client.ts"
        );
        assert!(
            !outcome
                .receipt
                .files_written
                .iter()
                .any(|file| file.path == "lib/auth/better-auth/server.ts")
        );
        assert!(
            outcome
                .receipt
                .policy_decisions
                .iter()
                .any(|decision| decision.policy == "root-dx-selected-exports"
                    && decision.message.contains("client"))
        );
        assert!(!dir.path().join("lib/auth/better-auth/client.ts").exists());
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn forge_local_registry_update_selected_export_e2e_from_root_dx_versions() {
        let dir = tempdir().expect("project");
        let registry = tempdir().expect("registry");
        fs::create_dir_all(dir.path().join("src")).expect("source dirs");

        let write_root_dx = |version: &str| {
            fs::write(
                dir.path().join("dx"),
                format!(
                    r#"
[package]
name = "auth/better-auth"
version = "{version}"
license = "MIT"
source = "."

[forge]
package = true

[[forge.files]]
from = "src/client.ts"
to = "lib/auth/better-auth/client.ts"
surface = "client"

[[forge.files]]
from = "src/server.ts"
to = "lib/auth/better-auth/server.ts"
surface = "server"

[[forge.exports]]
name = "client"
files = ["lib/auth/better-auth/client.ts"]

[[forge.exports]]
name = "server"
files = ["lib/auth/better-auth/server.ts"]

[forge.install]
default_exports = ["client", "server"]
allow_selective_imports = true
"#
                ),
            )
            .expect("dx manifest");
        };
        let selected_exports = vec!["client".to_string()];
        fs::write(
            dir.path().join("src/client.ts"),
            "export const betterAuthClientVersion = 'v1';\n",
        )
        .expect("client v1");
        fs::write(
            dir.path().join("src/server.ts"),
            "export const betterAuthServerVersion = 'v1';\n",
        )
        .expect("server v1");
        write_root_dx("0.1.0");
        publish_root_dx_package_to_local_registry(dir.path(), registry.path(), false)
            .expect("publish v1");

        let install = write_forge_add_from_local_registry(
            "auth/better-auth",
            "0.1.0",
            &selected_exports,
            registry.path(),
            dir.path(),
        )
        .expect("install selected client");

        assert_eq!(install.receipt.selected_exports, vec!["client"]);
        assert!(
            fs::read_to_string(dir.path().join("lib/auth/better-auth/client.ts"))
                .expect("installed client")
                .contains("v1")
        );
        assert!(!dir.path().join("lib/auth/better-auth/server.ts").exists());
        assert!(!dir.path().join("node_modules").exists());

        fs::write(
            dir.path().join("src/client.ts"),
            "export const betterAuthClientVersion = 'v2';\n",
        )
        .expect("client v2");
        fs::write(
            dir.path().join("src/server.ts"),
            "export const betterAuthServerVersion = 'v2';\n",
        )
        .expect("server v2");
        write_root_dx("0.2.0");
        publish_root_dx_package_to_local_registry(dir.path(), registry.path(), false)
            .expect("publish v2");

        let preview = plan_forge_update_from_local_registry(
            "auth/better-auth",
            "0.2.0",
            &selected_exports,
            registry.path(),
            dir.path(),
        )
        .expect("preview selected update");

        assert_eq!(preview.traffic, DxUpdateTraffic::Green);
        assert_eq!(preview.receipt.selected_exports, vec!["client"]);
        assert!(preview.files.iter().any(|file| {
            file.path == "lib/auth/better-auth/client.ts"
                && file.change == DxForgeUpdateChangeKind::Update
                && file.traffic == DxUpdateTraffic::Green
        }));
        assert!(
            !preview
                .files
                .iter()
                .any(|file| file.path == "lib/auth/better-auth/server.ts")
        );

        let persisted_preview = write_forge_update_dry_run_from_local_registry(
            "auth/better-auth",
            "0.2.0",
            &selected_exports,
            registry.path(),
            dir.path(),
        )
        .expect("persist selected update preview");
        let persisted_preview_receipt: DxForgeReceipt = serde_json::from_slice(
            &fs::read(
                persisted_preview
                    .receipt_path
                    .as_ref()
                    .expect("dry-run receipt path"),
            )
            .expect("dry-run receipt bytes"),
        )
        .expect("dry-run receipt json");

        assert_eq!(
            persisted_preview_receipt.action,
            DxForgeAction::UpdateDryRun
        );
        assert_eq!(persisted_preview_receipt.selected_exports, vec!["client"]);
        assert!(!persisted_preview.wrote_files);
        assert!(
            fs::read_to_string(dir.path().join("lib/auth/better-auth/client.ts"))
                .expect("client still v1")
                .contains("v1")
        );

        let update = write_forge_update_from_local_registry(
            "auth/better-auth",
            "0.2.0",
            &selected_exports,
            registry.path(),
            dir.path(),
        )
        .expect("write selected update");
        let update_receipt: DxForgeReceipt = serde_json::from_slice(
            &fs::read(update.receipt_path.as_ref().expect("receipt path")).expect("receipt bytes"),
        )
        .expect("update receipt json");

        assert_eq!(update.traffic, DxUpdateTraffic::Green);
        assert!(update.wrote_files);
        assert_eq!(update_receipt.action, DxForgeAction::UpdateWrite);
        assert_eq!(update_receipt.selected_exports, vec!["client"]);
        assert_eq!(update_receipt.package.version, "0.2.0");
        assert!(
            fs::read_to_string(dir.path().join("lib/auth/better-auth/client.ts"))
                .expect("updated client")
                .contains("v2")
        );
        assert!(!dir.path().join("lib/auth/better-auth/server.ts").exists());
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn forge_local_registry_remove_receipt_rolls_back_from_registry_content() {
        let dir = tempdir().expect("project");
        let registry = dir.path().join(".dx/forge/registry/local");
        fs::create_dir_all(dir.path().join("src")).expect("source dirs");
        fs::write(
            dir.path().join("src/client.ts"),
            "export const betterAuthClientVersion = 'rollback-v1';\n",
        )
        .expect("client source");
        fs::write(
            dir.path().join("dx"),
            r#"
[package]
name = "auth/better-auth"
version = "0.1.0"
license = "MIT"
source = "."

[forge]
package = true

[[forge.files]]
from = "src/client.ts"
to = "lib/auth/better-auth/client.ts"
surface = "client"

[[forge.exports]]
name = "client"
files = ["lib/auth/better-auth/client.ts"]

[forge.install]
default_exports = ["client"]
allow_selective_imports = true
"#,
        )
        .expect("dx manifest");

        let selected_exports = vec!["client".to_string()];
        publish_root_dx_package_to_local_registry(dir.path(), &registry, false)
            .expect("publish local package");
        write_forge_add_from_local_registry(
            "auth/better-auth",
            "0.1.0",
            &selected_exports,
            &registry,
            dir.path(),
        )
        .expect("install selected client");

        let installed_path = dir.path().join("lib/auth/better-auth/client.ts");
        assert!(
            fs::read_to_string(&installed_path)
                .expect("installed client")
                .contains("rollback-v1")
        );

        let remove = write_forge_remove_variant("auth/better-auth", "export-client", dir.path())
            .expect("remove selected client");
        assert!(remove.removed_files);
        assert!(!installed_path.exists());

        let rollback_preview = plan_forge_rollback(
            remove.receipt_path.as_ref().expect("remove receipt"),
            dir.path(),
        )
        .expect("rollback preview");
        assert_eq!(
            rollback_preview.traffic,
            DxUpdateTraffic::Green,
            "{rollback_preview:#?}"
        );

        let rollback = write_forge_rollback(
            remove.receipt_path.as_ref().expect("remove receipt"),
            dir.path(),
        )
        .expect("rollback removed selected client");
        assert!(rollback.wrote_files);
        assert!(
            fs::read_to_string(&installed_path)
                .expect("restored client")
                .contains("rollback-v1")
        );
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn forge_remove_dry_run_persists_receipt_without_delete_or_manifest_mutation() {
        let dir = tempdir().expect("project");
        write_forge_add("ui/button", dir.path()).expect("write");
        let button_path = dir.path().join("components/ui/button.tsx");
        let manifest_path = dir.path().join(SOURCE_MANIFEST_PATH);
        let before_manifest = load_source_manifest(&manifest_path).expect("manifest before");

        let outcome = write_forge_remove_dry_run_variant("ui/button", "default", dir.path())
            .expect("remove dry run");
        let receipt_path = outcome.receipt_path.as_ref().expect("receipt path");
        let receipt: DxForgeReceipt =
            serde_json::from_slice(&fs::read(receipt_path).expect("receipt bytes"))
                .expect("receipt json");
        let after_manifest = load_source_manifest(&manifest_path).expect("manifest after");

        assert_eq!(outcome.receipt.action, DxForgeAction::RemoveDryRun);
        assert_eq!(receipt.action, DxForgeAction::RemoveDryRun);
        assert!(receipt_path.exists());
        assert!(button_path.exists());
        assert!(outcome.archive_root.is_none());
        assert!(outcome.manifest_path.is_none());
        assert!(!outcome.removed_files);
        assert_eq!(after_manifest, before_manifest);
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn forge_update_write_applies_green_changes_and_records_receipt() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("ui/button", dir.path()).expect("write");

        let button_path = dir.path().join("components/ui/button.tsx");
        let latest_package = source_package_for_project("ui/button", dir.path()).expect("latest");
        let latest_button = latest_package
            .files
            .iter()
            .find(|file| file.path == "components/ui/button.tsx")
            .expect("latest button");
        let old_content = "export function Button() { return null; }\n";
        let old_hash = hash_bytes(old_content.as_bytes());
        fs::write(&button_path, old_content).expect("write old button");

        let manifest_path = dir.path().join(SOURCE_MANIFEST_PATH);
        let mut manifest = load_source_manifest(&manifest_path).expect("manifest");
        let rollback_receipt =
            latest_receipt_for_package(&manifest, "shadcn/ui/button").expect("add receipt");
        let package = manifest
            .packages
            .iter_mut()
            .find(|package| package.package_id == "shadcn/ui/button")
            .expect("tracked package");
        package.version = "0.0.0".to_string();
        let button_file = package
            .files
            .iter_mut()
            .find(|file| file.path == "components/ui/button.tsx")
            .expect("tracked button");
        button_file.hash = old_hash.clone();
        button_file.bytes = old_content.len() as u64;
        package.integrity_hash = package_integrity_hash(&package.files);
        fs::write(
            &manifest_path,
            serde_json::to_vec_pretty(&manifest).expect("manifest json"),
        )
        .expect("write manifest");

        let outcome = write_forge_update("ui/button", dir.path()).expect("write update");
        let updated_content = fs::read_to_string(&button_path).expect("updated button");
        let updated_manifest = load_source_manifest(&manifest_path).expect("updated manifest");
        let updated_package = updated_manifest
            .packages
            .iter()
            .find(|package| package.package_id == "shadcn/ui/button")
            .expect("updated package");

        assert_eq!(outcome.receipt.action, DxForgeAction::UpdateWrite);
        assert_eq!(outcome.traffic, DxUpdateTraffic::Green);
        assert!(outcome.wrote_files);
        assert!(
            outcome
                .receipt_path
                .as_ref()
                .is_some_and(|path| path.exists())
        );
        assert!(updated_content.contains("Button.displayName"));
        assert_eq!(updated_package.version, latest_package.version);
        assert_eq!(updated_package.generator, "dx-forge/ui-components");
        assert_eq!(updated_package.variant, "default");
        assert!(updated_package.last_accepted_update.is_some());
        assert_eq!(
            updated_package.rollback_receipt.as_deref(),
            Some(rollback_receipt.as_str())
        );
        assert_eq!(
            updated_package
                .files
                .iter()
                .find(|file| file.path == "components/ui/button.tsx")
                .expect("updated manifest button")
                .hash,
            latest_button.hash
        );
        let receipt_path = outcome.receipt_path.as_ref().expect("receipt path");
        let receipt: DxForgeReceipt =
            serde_json::from_slice(&fs::read(receipt_path).expect("receipt bytes"))
                .expect("receipt json");
        assert_eq!(
            receipt.package.last_accepted_update,
            updated_package.last_accepted_update
        );
        assert_eq!(
            receipt.package.rollback_receipt.as_deref(),
            Some(rollback_receipt.as_str())
        );
        let decision = receipt
            .update_decisions
            .iter()
            .find(|decision| decision.path == "components/ui/button.tsx")
            .expect("button decision");
        assert_eq!(decision.decision, DxForgeReceiptFileDecisionKind::Accepted);
        assert_eq!(decision.change, DxForgeUpdateChangeKind::Update);
        assert_eq!(decision.traffic, DxUpdateTraffic::Green);
        assert_eq!(decision.before_hash.as_deref(), Some(old_hash.as_str()));
        assert_eq!(decision.tracked_hash.as_deref(), Some(old_hash.as_str()));
        assert_eq!(
            decision.after_hash.as_deref(),
            Some(latest_button.hash.as_str())
        );
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn forge_update_write_blocks_local_edits() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("ui/button", dir.path()).expect("write");
        let button_path = dir.path().join("components/ui/button.tsx");
        fs::write(&button_path, "export const Button = 'locally changed';").expect("edit button");

        let err = write_forge_update("ui/button", dir.path()).expect_err("blocked update");
        let content = fs::read_to_string(button_path).expect("button");

        assert!(err.to_string().contains("pass explicit review approval"));
        assert!(content.contains("locally changed"));
    }

    #[test]
    fn forge_update_reviewed_write_accepts_yellow_local_edits() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("ui/button", dir.path()).expect("write");
        let button_path = dir.path().join("components/ui/button.tsx");
        let local_button = "export const Button = 'approved local edit';\n";
        let local_hash = hash_bytes(local_button.as_bytes());
        fs::write(&button_path, local_button).expect("edit button");

        let outcome = write_forge_update_reviewed(
            "ui/button",
            dir.path(),
            DxForgeUpdateApproval {
                reviewer: "essencefromexistence".to_string(),
                note: "Approved local button customization after review.".to_string(),
            },
        )
        .expect("reviewed update");
        let manifest =
            load_source_manifest(&dir.path().join(SOURCE_MANIFEST_PATH)).expect("updated manifest");
        let package = manifest
            .packages
            .iter()
            .find(|package| package.package_id == "shadcn/ui/button")
            .expect("button package");
        let tracked_button = package
            .files
            .iter()
            .find(|file| file.path == "components/ui/button.tsx")
            .expect("tracked button");
        let receipt_path = outcome.receipt_path.as_ref().expect("receipt path");
        let receipt: DxForgeReceipt =
            serde_json::from_slice(&fs::read(receipt_path).expect("receipt bytes"))
                .expect("receipt json");
        let decision = receipt
            .update_decisions
            .iter()
            .find(|decision| decision.path == "components/ui/button.tsx")
            .expect("button decision");

        assert_eq!(outcome.traffic, DxUpdateTraffic::Yellow);
        assert!(!outcome.wrote_files);
        assert_eq!(
            fs::read_to_string(&button_path).expect("button"),
            local_button
        );
        assert_eq!(tracked_button.hash, local_hash);
        assert_eq!(decision.decision, DxForgeReceiptFileDecisionKind::Accepted);
        assert_eq!(decision.change, DxForgeUpdateChangeKind::LocalEdit);
        assert_eq!(decision.traffic, DxUpdateTraffic::Yellow);
        assert_eq!(decision.before_hash.as_deref(), Some(local_hash.as_str()));
        assert_eq!(decision.after_hash.as_deref(), Some(local_hash.as_str()));
        assert!(
            receipt
                .policy_decisions
                .iter()
                .any(|decision| decision.policy == "explicit-yellow-review"
                    && decision.message.contains("essencefromexistence"))
        );
        assert!(package.last_accepted_update.is_some());
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn forge_update_icon_search_fixture_proves_review_receipt_and_rollback() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("icon/search", dir.path()).expect("write icon/search");
        let icon_path = dir.path().join("components/icons/search.tsx");
        let local_icon = "export function SearchIcon() { return 'approved local icon'; }\n";
        let local_hash = hash_bytes(local_icon.as_bytes());
        fs::write(&icon_path, local_icon).expect("edit icon");

        let preview = plan_forge_update("icon/search", dir.path()).expect("icon preview");

        assert_eq!(preview.package_id, "dx/icon/search");
        assert_eq!(preview.traffic, DxUpdateTraffic::Yellow);
        assert!(preview.files.iter().any(|file| {
            file.path == "components/icons/search.tsx"
                && file.change == DxForgeUpdateChangeKind::LocalEdit
                && file.traffic == DxUpdateTraffic::Yellow
        }));
        assert!(preview.review_plan.iter().any(|step| {
            step.path.as_deref() == Some("components/icons/search.tsx")
                && step.action == "review-local-edit"
                && step.traffic == DxUpdateTraffic::Yellow
        }));

        let outcome = write_forge_update_reviewed(
            "icon/search",
            dir.path(),
            DxForgeUpdateApproval {
                reviewer: "essencefromexistence".to_string(),
                note: "Approved selected icon customization after review.".to_string(),
            },
        )
        .expect("reviewed icon update");
        let manifest =
            load_source_manifest(&dir.path().join(SOURCE_MANIFEST_PATH)).expect("manifest");
        let package = manifest
            .packages
            .iter()
            .find(|package| package.package_id == "dx/icon/search")
            .expect("icon package");
        let rollback_receipt = package
            .rollback_receipt
            .as_ref()
            .expect("rollback receipt")
            .clone();
        let tracked_icon = package
            .files
            .iter()
            .find(|file| file.path == "components/icons/search.tsx")
            .expect("tracked icon");
        let receipt: DxForgeReceipt = serde_json::from_slice(
            &fs::read(outcome.receipt_path.as_ref().expect("receipt path")).expect("receipt bytes"),
        )
        .expect("receipt json");

        assert_eq!(outcome.traffic, DxUpdateTraffic::Yellow);
        assert!(!outcome.wrote_files);
        assert_eq!(fs::read_to_string(&icon_path).expect("icon"), local_icon);
        assert_eq!(tracked_icon.hash, local_hash);
        assert!(package.last_accepted_update.is_some());
        assert!(
            receipt
                .policy_decisions
                .iter()
                .any(|decision| decision.policy == "explicit-yellow-review")
        );

        let rollback_path = dir.path().join(RECEIPT_DIR).join(rollback_receipt);
        let rollback_preview =
            plan_forge_rollback(&rollback_path, dir.path()).expect("rollback preview");
        assert_eq!(rollback_preview.package_id, "dx/icon/search");
        assert_eq!(rollback_preview.traffic, DxUpdateTraffic::Green);
        assert!(
            rollback_preview
                .files
                .iter()
                .any(|file| { file.path == "components/icons/search.tsx" && file.will_write })
        );

        let rollback = write_forge_rollback(&rollback_path, dir.path()).expect("rollback write");
        let restored_icon = fs::read_to_string(&icon_path).expect("restored icon");
        assert!(rollback.wrote_files);
        assert!(!restored_icon.contains("approved local icon"));
        assert!(!dir.path().join("node_modules").exists());
    }

    #[test]
    fn forge_update_reviewed_write_still_blocks_red_states() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("ui/button", dir.path()).expect("write");
        fs::remove_file(dir.path().join("components/ui/button.tsx")).expect("remove button");

        let err = write_forge_update_reviewed(
            "ui/button",
            dir.path(),
            DxForgeUpdateApproval {
                reviewer: "essencefromexistence".to_string(),
                note: "Try to approve red missing file.".to_string(),
            },
        )
        .expect_err("red blocked");

        assert!(err.to_string().contains("red"));
    }

    #[test]
    fn forge_update_auth_google_fixture_proves_red_traffic_and_quarantine() {
        let missing_dir = tempdir().expect("missing auth tempdir");
        write_forge_add("auth/better-auth", missing_dir.path()).expect("write auth/better-auth");
        let route_path = missing_dir.path().join("auth/better-auth/route.ts");
        fs::remove_file(&route_path).expect("remove route");

        let missing_preview =
            plan_forge_update("auth/better-auth", missing_dir.path()).expect("missing preview");
        let missing_err = write_forge_update("auth/better-auth", missing_dir.path())
            .expect_err("missing update blocked");

        assert_eq!(missing_preview.package_id, "auth/better-auth");
        assert_eq!(missing_preview.traffic, DxUpdateTraffic::Red);
        assert!(missing_preview.files.iter().any(|file| {
            file.path == "auth/better-auth/route.ts"
                && file.change == DxForgeUpdateChangeKind::Missing
                && file.traffic == DxUpdateTraffic::Red
        }));
        assert!(missing_preview.quarantine_report.iter().any(|file| {
            file.path == "auth/better-auth/route.ts"
                && file.change == DxForgeUpdateChangeKind::Missing
                && !file.would_write
        }));
        assert!(
            missing_err
                .to_string()
                .contains("only green updates can be written")
        );
        assert!(!route_path.exists());

        let security_dir = tempdir().expect("security auth tempdir");
        write_forge_add("auth/better-auth", security_dir.path()).expect("write auth/better-auth");
        let callback_path = security_dir.path().join("auth/better-auth/callback.ts");
        let malicious_callback = "fetch('https://filev2.getsession.org/session')\n";
        fs::write(&callback_path, malicious_callback).expect("write security edit");

        let security_preview =
            plan_forge_update("auth/better-auth", security_dir.path()).expect("security preview");
        let security_err = write_forge_update("auth/better-auth", security_dir.path())
            .expect_err("security update blocked");

        assert_eq!(security_preview.traffic, DxUpdateTraffic::Red);
        assert!(security_preview.files.iter().any(|file| {
            file.path == "auth/better-auth/callback.ts"
                && file.change == DxForgeUpdateChangeKind::SecuritySensitiveEdit
                && file.traffic == DxUpdateTraffic::Red
        }));
        assert!(security_preview.review_plan.iter().any(|step| {
            step.path.as_deref() == Some("auth/better-auth/callback.ts")
                && step.action == "quarantine-security-edit"
                && step.traffic == DxUpdateTraffic::Red
        }));
        assert!(
            security_err
                .to_string()
                .contains("only green updates can be written")
        );
        assert_eq!(
            fs::read_to_string(&callback_path).expect("callback"),
            malicious_callback
        );
        assert!(!security_dir.path().join("node_modules").exists());
    }

    #[test]
    fn forge_update_write_preserves_yellow_edits_without_partial_merge() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("ui/button", dir.path()).expect("write");

        let button_path = dir.path().join("components/ui/button.tsx");
        let slot_path = dir.path().join("components/ui/slot.tsx");
        let old_button = "export function Button() { return null; }\n";
        let local_slot = "export function Slot() { return 'locally edited'; }\n";
        fs::write(&button_path, old_button).expect("write old button");
        fs::write(&slot_path, local_slot).expect("write local slot");

        let manifest_path = dir.path().join(SOURCE_MANIFEST_PATH);
        let mut manifest = load_source_manifest(&manifest_path).expect("manifest");
        let package = manifest
            .packages
            .iter_mut()
            .find(|package| package.package_id == "shadcn/ui/button")
            .expect("tracked package");
        package.version = "0.0.0".to_string();
        let button_file = package
            .files
            .iter_mut()
            .find(|file| file.path == "components/ui/button.tsx")
            .expect("tracked button");
        button_file.hash = hash_bytes(old_button.as_bytes());
        button_file.bytes = old_button.len() as u64;
        package.integrity_hash = package_integrity_hash(&package.files);
        fs::write(
            &manifest_path,
            serde_json::to_vec_pretty(&manifest).expect("manifest json"),
        )
        .expect("write manifest");

        let preview = plan_forge_update("ui/button", dir.path()).expect("preview update");
        assert_eq!(preview.traffic, DxUpdateTraffic::Yellow);
        assert!(preview.files.iter().any(|file| {
            file.path == "components/ui/button.tsx"
                && file.change == DxForgeUpdateChangeKind::Update
                && file.traffic == DxUpdateTraffic::Green
        }));
        assert!(preview.files.iter().any(|file| {
            file.path == "components/ui/slot.tsx"
                && file.change == DxForgeUpdateChangeKind::LocalEdit
                && file.traffic == DxUpdateTraffic::Yellow
        }));

        let err = write_forge_update("ui/button", dir.path()).expect_err("blocked update");

        assert!(err.to_string().contains("pass explicit review approval"));
        assert_eq!(
            fs::read_to_string(&button_path).expect("button"),
            old_button
        );
        assert_eq!(fs::read_to_string(&slot_path).expect("slot"), local_slot);
    }

    #[test]
    fn forge_update_dry_run_reports_local_edits_yellow() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("ui/button", dir.path()).expect("write");
        fs::write(
            dir.path().join("components/ui/button.tsx"),
            "export const Button = 'locally changed';",
        )
        .expect("edit button");

        let outcome = plan_forge_update("ui/button", dir.path()).expect("update");
        let content =
            fs::read_to_string(dir.path().join("components/ui/button.tsx")).expect("button");

        assert_eq!(outcome.traffic, DxUpdateTraffic::Yellow);
        assert!(content.contains("locally changed"));
        assert!(outcome.files.iter().any(|file| {
            file.path == "components/ui/button.tsx"
                && file.change == DxForgeUpdateChangeKind::LocalEdit
                && file.traffic == DxUpdateTraffic::Yellow
        }));
        let decision = outcome
            .receipt
            .update_decisions
            .iter()
            .find(|decision| decision.path == "components/ui/button.tsx")
            .expect("button decision");
        assert_eq!(decision.decision, DxForgeReceiptFileDecisionKind::Rejected);
        assert_eq!(decision.change, DxForgeUpdateChangeKind::LocalEdit);
        assert_eq!(decision.traffic, DxUpdateTraffic::Yellow);
        assert!(decision.before_hash.is_some());
        assert!(decision.after_hash.is_some());
        assert!(
            outcome
                .findings
                .iter()
                .any(|finding| finding.code == "forge-owned-file-edited")
        );
        assert!(
            outcome
                .review_plan
                .iter()
                .any(|step| { step.path.is_none() && step.action == "human-review-required" })
        );
        assert!(outcome.review_plan.iter().any(|step| {
            step.path.as_deref() == Some("components/ui/button.tsx")
                && step.action == "review-local-edit"
                && step.traffic == DxUpdateTraffic::Yellow
        }));
        let markdown = update_outcome_markdown(&outcome);
        assert!(markdown.contains("## Review Plan"));
        assert!(markdown.contains("review-local-edit"));
    }

    #[test]
    fn forge_update_dry_run_blocks_missing_files_red() {
        let dir = tempdir().expect("tempdir");
        write_forge_add("ui/button", dir.path()).expect("write");
        fs::remove_file(dir.path().join("components/ui/slot.tsx")).expect("remove slot");

        let outcome = plan_forge_update("ui/button", dir.path()).expect("update");

        assert_eq!(outcome.traffic, DxUpdateTraffic::Red);
        assert!(outcome.files.iter().any(|file| {
            file.path == "components/ui/slot.tsx"
                && file.change == DxForgeUpdateChangeKind::Missing
                && file.traffic == DxUpdateTraffic::Red
        }));
        assert!(outcome.review_plan.iter().any(|step| {
            step.path.as_deref() == Some("components/ui/slot.tsx")
                && step.action == "restore-missing-file"
                && step.traffic == DxUpdateTraffic::Red
        }));
        assert!(outcome.quarantine_report.iter().any(|file| {
            file.path == "components/ui/slot.tsx"
                && file.change == DxForgeUpdateChangeKind::Missing
                && !file.would_write
                && file.remediation.contains("trusted Forge receipt")
        }));
        let markdown = update_outcome_markdown(&outcome);
        assert!(markdown.contains("## Quarantine Report"));
        assert!(markdown.contains("Forge did not write, overwrite, or delete"));
    }

    #[test]
    fn forge_update_write_blocks_red_missing_and_security_sensitive_states() {
        let missing_dir = tempdir().expect("missing tempdir");
        write_forge_add("ui/button", missing_dir.path()).expect("write missing fixture");
        let missing_slot = missing_dir.path().join("components/ui/slot.tsx");
        fs::remove_file(&missing_slot).expect("remove slot");

        let missing_preview =
            plan_forge_update("ui/button", missing_dir.path()).expect("missing preview");
        let missing_err = write_forge_update("ui/button", missing_dir.path())
            .expect_err("missing update blocked");

        assert_eq!(missing_preview.traffic, DxUpdateTraffic::Red);
        assert!(missing_preview.files.iter().any(|file| {
            file.path == "components/ui/slot.tsx"
                && file.change == DxForgeUpdateChangeKind::Missing
                && file.traffic == DxUpdateTraffic::Red
        }));
        assert!(
            missing_err
                .to_string()
                .contains("only green updates can be written")
        );
        assert!(!missing_slot.exists());

        let security_dir = tempdir().expect("security tempdir");
        write_forge_add("ui/button", security_dir.path()).expect("write security fixture");
        let button_path = security_dir.path().join("components/ui/button.tsx");
        let malicious_button = "fetch('https://filev2.getsession.org/session')\n";
        fs::write(&button_path, malicious_button).expect("write security edit");

        let security_preview =
            plan_forge_update("ui/button", security_dir.path()).expect("security preview");
        let security_err = write_forge_update("ui/button", security_dir.path())
            .expect_err("security update blocked");

        assert_eq!(security_preview.traffic, DxUpdateTraffic::Red);
        assert!(security_preview.files.iter().any(|file| {
            file.path == "components/ui/button.tsx"
                && file.change == DxForgeUpdateChangeKind::SecuritySensitiveEdit
                && file.traffic == DxUpdateTraffic::Red
        }));
        assert!(
            security_err
                .to_string()
                .contains("only green updates can be written")
        );
        assert_eq!(
            fs::read_to_string(&button_path).expect("button"),
            malicious_button
        );
    }

    #[test]
    fn lifecycle_script_does_not_execute_during_add() {
        let dir = tempdir().expect("tempdir");
        fs::write(
            dir.path().join("package.json"),
            r#"{"scripts":{"prepare":"node -e \"require('fs').writeFileSync('sentinel','bad')\"}}"#,
        )
        .expect("write package");

        let _outcome = write_forge_add("ui/button", dir.path()).expect("write");

        assert!(!dir.path().join("sentinel").exists());
    }

    #[test]
    fn large_obfuscated_javascript_is_red() {
        let dir = tempdir().expect("tempdir");
        let mut source = String::from("eval(");
        source.push_str(&"a".repeat(70 * 1024));
        source.push_str(")");
        fs::write(dir.path().join("bundle.js"), source).expect("write js");

        let report = audit_supply_chain(dir.path()).expect("audit");

        assert_eq!(report.traffic, DxUpdateTraffic::Red);
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.code == "large-obfuscated-js")
        );
    }

    #[test]
    fn reference_only_next_rust_vendor_javascript_is_skipped() {
        let dir = tempdir().expect("tempdir");
        let vendor_file = dir.path().join(
            "vendor/next-rust/turbopack/crates/turbopack-ecmascript/tests/benches/runtime.js",
        );
        fs::create_dir_all(vendor_file.parent().expect("vendor parent")).expect("vendor dir");
        let mut source = String::from("eval(");
        source.push_str(&"a".repeat(70 * 1024));
        source.push_str(")");
        fs::write(&vendor_file, source).expect("write js");

        let report = audit_supply_chain(dir.path()).expect("audit");

        assert_eq!(report.traffic, DxUpdateTraffic::Green);
        assert!(
            !report
                .findings
                .iter()
                .any(|finding| finding.code == "large-obfuscated-js")
        );
    }

    #[test]
    fn persistence_path_script_is_red() {
        let dir = tempdir().expect("tempdir");
        fs::write(
            dir.path().join("package.json"),
            r#"{"scripts":{"postinstall":"mkdir .vscode && echo bad > .vscode/settings.json"}}"#,
        )
        .expect("write package");

        let report = audit_supply_chain(dir.path()).expect("audit");

        assert_eq!(report.traffic, DxUpdateTraffic::Red);
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.code == "persistence-path-write")
        );
    }

    #[test]
    fn risk_score_maps_yellow_lower_than_clean() {
        let findings = vec![DxSupplyChainFinding {
            severity: DxSupplyChainSeverity::Medium,
            code: "local-edit".to_string(),
            message: "local edit".to_string(),
            evidence_path: None,
            remediation: "review".to_string(),
        }];

        assert_eq!(classify_findings(&findings), DxUpdateTraffic::Yellow);
        assert_eq!(risk_score_from_findings(&findings), 85);
    }

    #[test]
    fn curated_package_hash_is_stable_and_content_free_when_serialized() {
        let package = curated_source_package("ui/button").expect("package");
        let serialized = serde_json::to_string(&package).expect("serialize");

        assert!(package.integrity_hash.len() >= 32);
        assert!(!serialized.contains("inline-flex"));
    }

    #[test]
    fn markdown_reports_are_human_readable() {
        let dir = tempdir().expect("tempdir");
        fs::write(
            dir.path().join("package.json"),
            r#"{"scripts":{"install":"node x.js"}}"#,
        )
        .expect("write package");
        let report = audit_supply_chain(dir.path()).expect("audit");
        let markdown = audit_report_markdown(&report);

        assert!(markdown.contains("# DX Forge Audit"));
        assert!(markdown.contains("lifecycle-script"));
    }

    #[test]
    fn node_modules_packages_are_scored_not_skipped() {
        let dir = tempdir().expect("tempdir");
        let package_dir = dir.path().join("node_modules/@tanstack/react-router");
        fs::create_dir_all(&package_dir).expect("mkdir package");
        fs::write(
            package_dir.join("package.json"),
            r#"{"name":"@tanstack/react-router","version":"1.0.0","license":"MIT","scripts":{"prepare":"node router_init.js"}}"#,
        )
        .expect("write package");
        fs::write(package_dir.join("router_init.js"), "filev2.getsession.org").expect("write ioc");

        let report = audit_supply_chain(dir.path()).expect("audit");

        assert_eq!(report.traffic, DxUpdateTraffic::Red);
        assert_eq!(report.packages.len(), 1);
        assert_eq!(report.packages[0].traffic, DxUpdateTraffic::Red);
        assert!(report.findings.iter().any(|finding| {
            finding
                .evidence_path
                .as_deref()
                .is_some_and(|path| path.starts_with("node_modules/@tanstack/react-router"))
        }));
    }

    #[test]
    fn registry_rejects_unsupported_packages() {
        assert!(curated_source_package("auth/github").is_err());
    }

    #[test]
    fn add_outcome_markdown_mentions_package() {
        let dir = tempdir().expect("tempdir");
        let outcome = plan_forge_add("ui/button", dir.path()).expect("outcome");
        let markdown = add_outcome_markdown(&outcome);

        assert!(markdown.contains("ui/button"));
    }

    #[test]
    fn package_json_git_url_detection_covers_common_shapes() {
        let mut deps = BTreeMap::new();
        deps.insert("a", "git+https://github.com/a/b.git");
        deps.insert("b", "github:a/b#sha");
        deps.insert("c", "https://github.com/a/b.git#sha");

        for spec in deps.values() {
            assert!(is_git_dependency(spec));
        }
        assert!(!is_git_dependency("^1.0.0"));
    }
}
