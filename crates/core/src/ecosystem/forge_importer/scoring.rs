use serde::{Deserialize, Serialize};

use super::types::DxForgeImportRiskFlag;

/// Stable Forge scoring model for external ecosystem imports.
pub const DX_FORGE_IMPORT_SCORE_MODEL_VERSION: &str = "dx-forge-import-score-2026-06";

/// Inputs collected by the import firewall before a package score is computed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeImportScoreInput {
    pub no_node_modules: bool,
    pub source_dir_ready: bool,
    pub materialized: bool,
    pub accepted_materialization_receipt_present: bool,
    pub metadata_present: bool,
    pub provenance_verified: bool,
    pub artifact_integrity_present: bool,
    pub license_declared: bool,
    pub license_file_present: bool,
    pub license_reviewed: bool,
    pub advisory_evidence_present: bool,
    pub advisory_reviewed: bool,
    pub popularity_evidence_present: bool,
    pub sbom_present: bool,
    pub exports_present: bool,
    pub dependency_count: usize,
    pub files_considered: usize,
    pub files_rejected: usize,
    pub package_installs_run: bool,
    pub lifecycle_scripts_executed: bool,
    pub risk_flags: Vec<DxForgeImportRiskFlag>,
}

/// One scored dimension in the import firewall.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeImportScoreDimension {
    pub id: String,
    pub label: String,
    pub score: u8,
    pub max: u8,
    pub evidence: String,
}

/// A cap that limits the final package score regardless of weighted points.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeImportAppliedCap {
    pub id: String,
    pub ceiling: u8,
    pub traffic: String,
    pub reason: String,
}

/// Final score report used by JSON, .sr, and generated machine receipts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeImportScoreReport {
    pub score_model_version: String,
    pub uncapped_score: u8,
    pub score_ceiling: u8,
    pub score: u8,
    pub traffic: String,
    pub dimensions: Vec<DxForgeImportScoreDimension>,
    pub applied_caps: Vec<DxForgeImportAppliedCap>,
}

pub fn score_forge_import(input: DxForgeImportScoreInput) -> DxForgeImportScoreReport {
    let dimensions = vec![
        dimension(
            "provenance",
            "Provenance and registry identity",
            if input.metadata_present {
                15
            } else if input.source_dir_ready {
                8
            } else {
                2
            },
            15,
            if input.metadata_present {
                if input.provenance_verified {
                    "package metadata and provenance verification were recorded"
                } else {
                    "package metadata was found; provenance verification is still pending"
                }
            } else if input.source_dir_ready {
                "inspected source exists but registry metadata is not proven"
            } else {
                "plan-only mode has no inspected source metadata"
            },
        ),
        dimension(
            "integrity",
            "Artifact integrity and reproducible source",
            if input.artifact_integrity_present {
                15
            } else if input.files_considered > 0 {
                8
            } else {
                0
            },
            15,
            if input.artifact_integrity_present {
                "Forge recorded source file hashes"
            } else {
                "artifact integrity evidence is incomplete"
            },
        ),
        dimension(
            "source-slice",
            "Source slice and public API review",
            if input.files_rejected > 0 {
                4
            } else if input.exports_present && input.source_dir_ready && input.files_considered > 0
            {
                20
            } else if input.source_dir_ready && input.files_considered > 0 {
                12
            } else {
                0
            },
            20,
            if input.exports_present {
                "public exports were discovered from inspected source"
            } else if input.source_dir_ready {
                "source is present but public exports need reviewer confirmation"
            } else {
                "slice review is pending inspected source"
            },
        ),
        dimension(
            "execution-safety",
            "Execution, lifecycle, and native safety",
            if input.no_node_modules
                && !input.package_installs_run
                && !input.lifecycle_scripts_executed
                && !input
                    .risk_flags
                    .iter()
                    .any(|risk| risk.blocks_materialization())
            {
                15
            } else if input.no_node_modules && !input.package_installs_run {
                8
            } else {
                0
            },
            15,
            if input.package_installs_run || input.lifecycle_scripts_executed {
                "package-manager installs or lifecycle scripts were observed; Forge caps this import"
            } else if input
                .risk_flags
                .iter()
                .any(|risk| risk.blocks_materialization())
            {
                "blocking source risk requires bridge, rejection, or manual quarantine"
            } else {
                "Forge import did not run package-manager installs or lifecycle scripts"
            },
        ),
        dimension(
            "license-advisory",
            "License and advisory review",
            match (
                input.license_declared,
                input.license_file_present,
                input.advisory_evidence_present,
            ) {
                (true, true, true) => 15,
                (true, true, false) => 8,
                (true, false, true) => 7,
                (true, false, false) => 5,
                _ => 0,
            },
            15,
            if input.license_file_present {
                if input.license_reviewed && input.advisory_reviewed {
                    "license and advisory evidence are reviewer accepted"
                } else {
                    "license/advisory declarations are present; reviewer proof is still pending"
                }
            } else {
                "license/advisory evidence is incomplete"
            },
        ),
        dimension(
            "health",
            "Maintainer health and popularity",
            if input.popularity_evidence_present {
                10
            } else if input.metadata_present {
                5
            } else {
                0
            },
            10,
            if input.popularity_evidence_present {
                "package health evidence was attached"
            } else {
                "popularity cannot override supply-chain gates"
            },
        ),
        dimension(
            "receipt-coherence",
            "DX materialization and receipt coherence",
            if input.materialized
                && input.files_considered > 0
                && input.accepted_materialization_receipt_present
            {
                10
            } else if input.materialized && input.files_considered > 0 {
                4
            } else if input.source_dir_ready {
                6
            } else {
                2
            },
            10,
            if input.materialized && input.accepted_materialization_receipt_present {
                "Forge manifest, import plan, .sr, and machine receipts were written"
            } else if input.materialized {
                "materialized files are present but accepted materialization receipt evidence is missing"
            } else {
                "receipt is plan-only"
            },
        ),
    ];

    let uncapped_score = dimensions
        .iter()
        .map(|dimension| dimension.score)
        .sum::<u8>()
        .min(100);
    let applied_caps = applied_caps(&input);
    let score_ceiling = applied_caps
        .iter()
        .map(|cap| cap.ceiling)
        .min()
        .unwrap_or(100);
    let score = uncapped_score.min(score_ceiling);
    let traffic = if score >= 95 {
        "green"
    } else if score >= 80 {
        "yellow"
    } else {
        "red"
    }
    .to_string();

    DxForgeImportScoreReport {
        score_model_version: DX_FORGE_IMPORT_SCORE_MODEL_VERSION.to_string(),
        uncapped_score,
        score_ceiling,
        score,
        traffic,
        dimensions,
        applied_caps,
    }
}

fn dimension(
    id: &str,
    label: &str,
    score: u8,
    max: u8,
    evidence: &str,
) -> DxForgeImportScoreDimension {
    DxForgeImportScoreDimension {
        id: id.to_string(),
        label: label.to_string(),
        score,
        max,
        evidence: evidence.to_string(),
    }
}

fn applied_caps(input: &DxForgeImportScoreInput) -> Vec<DxForgeImportAppliedCap> {
    let mut caps = Vec::new();

    if input.package_installs_run || input.lifecycle_scripts_executed {
        caps.push(cap(
            "package-code-executed",
            0,
            "red",
            "Package-manager install or lifecycle code was executed during import.",
        ));
    }
    if !input.no_node_modules {
        caps.push(cap(
            "node-modules-present",
            45,
            "red",
            "Target project already has node_modules, so Forge cannot claim source ownership.",
        ));
    }
    if !input.source_dir_ready {
        caps.push(cap(
            "plan-only-no-inspected-source",
            79,
            "yellow",
            "No inspected source directory was provided.",
        ));
    }
    if input.source_dir_ready && !input.metadata_present {
        caps.push(cap(
            "source-dir-only-no-registry-metadata",
            84,
            "yellow",
            "The source snapshot has no package metadata evidence.",
        ));
    }
    if input.metadata_present && !input.provenance_verified {
        caps.push(cap(
            "provenance-verification-pending",
            94,
            "yellow",
            "Package metadata was declared, but provenance verification is still pending.",
        ));
    }
    if !input.license_declared || !input.license_file_present {
        caps.push(cap(
            "license-review-incomplete",
            89,
            "yellow",
            "License evidence is incomplete or unreviewed.",
        ));
    }
    if input.license_declared && input.license_file_present && !input.license_reviewed {
        caps.push(cap(
            "license-review-pending",
            94,
            "yellow",
            "License evidence is declared, but reviewer acceptance is still pending.",
        ));
    }
    if !input.artifact_integrity_present {
        caps.push(cap(
            "artifact-integrity-incomplete",
            84,
            "yellow",
            "Artifact integrity evidence is incomplete.",
        ));
    }
    if !input.advisory_evidence_present {
        caps.push(cap(
            "advisory-evidence-incomplete",
            89,
            "yellow",
            "Advisory coverage is not proven.",
        ));
    }
    if input.advisory_evidence_present && !input.advisory_reviewed {
        caps.push(cap(
            "advisory-review-pending",
            94,
            "yellow",
            "Advisory evidence is declared, but live or reviewed coverage is still pending.",
        ));
    }
    if input.materialized && !input.sbom_present {
        caps.push(cap(
            "source-sbom-missing",
            94,
            "yellow",
            "Materialized source does not yet have a source bill of materials receipt.",
        ));
    }
    if input.files_rejected > 0 {
        caps.push(cap(
            "rejected-source-files",
            70,
            "red",
            "One or more source files were rejected by the import firewall.",
        ));
    }
    if input.materialized && input.files_considered == 0 {
        caps.push(cap(
            "materialized-zero-source-files",
            70,
            "red",
            "Forge cannot claim materialization when no source files were considered.",
        ));
    }
    if input.materialized && !input.accepted_materialization_receipt_present {
        caps.push(cap(
            "materialization-receipt-missing",
            79,
            "yellow",
            "Forge cannot claim source-owned materialization without an accepted materialization receipt.",
        ));
    }
    if input.dependency_count > 48 {
        caps.push(cap(
            "large-dependency-graph",
            72,
            "yellow",
            "Large dependency graphs require a bridge or narrower reviewed slice before Forge can claim full source ownership.",
        ));
    }

    for risk in &input.risk_flags {
        match risk {
            DxForgeImportRiskFlag::PlaintextSecret
            | DxForgeImportRiskFlag::ProjectEscape
            | DxForgeImportRiskFlag::UnsafePath
            | DxForgeImportRiskFlag::Symlink => caps.push(cap(
                risk.as_reason_code(),
                0,
                "red",
                "A hard security risk blocks Forge source materialization.",
            )),
            DxForgeImportRiskFlag::LifecycleScript | DxForgeImportRiskFlag::InstallHook => caps
                .push(cap(
                    risk.as_reason_code(),
                    60,
                    "red",
                    "Lifecycle or install hooks require a tool bridge or manual receipt.",
                )),
            DxForgeImportRiskFlag::NativeBinary => caps.push(cap(
                risk.as_reason_code(),
                70,
                "red",
                "Native binaries require a bridge, binary snapshot, or manual wrapper.",
            )),
            DxForgeImportRiskFlag::DynamicExecution => caps.push(cap(
                risk.as_reason_code(),
                55,
                "red",
                "Dynamic execution prevents automatic source-owned materialization.",
            )),
            DxForgeImportRiskFlag::DynamicImport => caps.push(cap(
                risk.as_reason_code(),
                72,
                "yellow",
                "Dynamic import prevents a precise source slice.",
            )),
            DxForgeImportRiskFlag::ObfuscatedBlob => caps.push(cap(
                risk.as_reason_code(),
                60,
                "red",
                "Minified or obfuscated runtime blobs need source-map or original-source proof.",
            )),
            DxForgeImportRiskFlag::HugeDependencyGraph => caps.push(cap(
                risk.as_reason_code(),
                72,
                "yellow",
                "Large dependency graphs require a bridge or narrower reviewed slice.",
            )),
            DxForgeImportRiskFlag::MissingIntegrity => caps.push(cap(
                risk.as_reason_code(),
                84,
                "yellow",
                "Missing integrity evidence prevents a full source-owned score.",
            )),
            DxForgeImportRiskFlag::SideEffectImport => caps.push(cap(
                risk.as_reason_code(),
                72,
                "yellow",
                "Side-effect imports require reviewer acceptance before source materialization.",
            )),
            DxForgeImportRiskFlag::RuntimeMismatch => caps.push(cap(
                risk.as_reason_code(),
                60,
                "red",
                "Package identity or runtime target mismatch requires a bridge boundary.",
            )),
            DxForgeImportRiskFlag::AdvisoryCoverageMissing => caps.push(cap(
                risk.as_reason_code(),
                89,
                "yellow",
                "Advisory coverage is not proven.",
            )),
            _ => {}
        }
    }

    caps
}

fn cap(id: &str, ceiling: u8, traffic: &str, reason: &str) -> DxForgeImportAppliedCap {
    DxForgeImportAppliedCap {
        id: id.to_string(),
        ceiling,
        traffic: traffic.to_string(),
        reason: reason.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_popular_source_package_can_score_one_hundred() {
        let report = score_forge_import(DxForgeImportScoreInput {
            no_node_modules: true,
            source_dir_ready: true,
            materialized: true,
            accepted_materialization_receipt_present: true,
            metadata_present: true,
            provenance_verified: true,
            artifact_integrity_present: true,
            license_declared: true,
            license_file_present: true,
            license_reviewed: true,
            advisory_evidence_present: true,
            advisory_reviewed: true,
            popularity_evidence_present: true,
            sbom_present: true,
            exports_present: true,
            dependency_count: 0,
            files_considered: 3,
            files_rejected: 0,
            package_installs_run: false,
            lifecycle_scripts_executed: false,
            risk_flags: Vec::new(),
        });

        assert_eq!(report.uncapped_score, 100);
        assert_eq!(report.score_ceiling, 100);
        assert_eq!(report.score, 100);
        assert!(report.applied_caps.is_empty());
    }

    #[test]
    fn source_dir_without_metadata_is_capped_honestly() {
        let report = score_forge_import(DxForgeImportScoreInput {
            no_node_modules: true,
            source_dir_ready: true,
            materialized: true,
            accepted_materialization_receipt_present: true,
            metadata_present: false,
            provenance_verified: false,
            artifact_integrity_present: true,
            license_declared: true,
            license_file_present: true,
            license_reviewed: true,
            advisory_evidence_present: true,
            advisory_reviewed: true,
            popularity_evidence_present: false,
            sbom_present: true,
            exports_present: true,
            dependency_count: 0,
            files_considered: 2,
            files_rejected: 0,
            package_installs_run: false,
            lifecycle_scripts_executed: false,
            risk_flags: Vec::new(),
        });

        assert_eq!(report.score_ceiling, 84);
        assert!(report.score <= 84);
    }

    #[test]
    fn plan_only_without_source_stays_below_green() {
        let report = score_forge_import(DxForgeImportScoreInput {
            no_node_modules: true,
            source_dir_ready: false,
            materialized: false,
            accepted_materialization_receipt_present: false,
            metadata_present: false,
            provenance_verified: false,
            artifact_integrity_present: false,
            license_declared: false,
            license_file_present: false,
            license_reviewed: false,
            advisory_evidence_present: false,
            advisory_reviewed: false,
            popularity_evidence_present: false,
            sbom_present: false,
            exports_present: false,
            dependency_count: 0,
            files_considered: 0,
            files_rejected: 0,
            package_installs_run: false,
            lifecycle_scripts_executed: false,
            risk_flags: Vec::new(),
        });

        assert_eq!(report.score_ceiling, 79);
        assert!(report.score < 80);
        assert_eq!(report.traffic, "red");
    }

    #[test]
    fn popularity_cannot_bypass_missing_integrity_license_or_advisory_caps() {
        let report = score_forge_import(DxForgeImportScoreInput {
            no_node_modules: true,
            source_dir_ready: true,
            materialized: true,
            accepted_materialization_receipt_present: true,
            metadata_present: true,
            provenance_verified: true,
            artifact_integrity_present: false,
            license_declared: false,
            license_file_present: false,
            license_reviewed: false,
            advisory_evidence_present: false,
            advisory_reviewed: false,
            popularity_evidence_present: true,
            sbom_present: true,
            exports_present: true,
            dependency_count: 0,
            files_considered: 3,
            files_rejected: 0,
            package_installs_run: false,
            lifecycle_scripts_executed: false,
            risk_flags: Vec::new(),
        });

        assert!(report.score <= 84);
        assert!(
            report
                .applied_caps
                .iter()
                .any(|cap| cap.id == "artifact-integrity-incomplete")
        );
    }

    #[test]
    fn materialized_zero_files_cannot_score_green() {
        let report = score_forge_import(DxForgeImportScoreInput {
            no_node_modules: true,
            source_dir_ready: true,
            materialized: true,
            accepted_materialization_receipt_present: true,
            metadata_present: true,
            provenance_verified: true,
            artifact_integrity_present: true,
            license_declared: true,
            license_file_present: true,
            license_reviewed: true,
            advisory_evidence_present: true,
            advisory_reviewed: true,
            popularity_evidence_present: true,
            sbom_present: true,
            exports_present: true,
            dependency_count: 0,
            files_considered: 0,
            files_rejected: 0,
            package_installs_run: false,
            lifecycle_scripts_executed: false,
            risk_flags: Vec::new(),
        });

        assert_eq!(report.score_ceiling, 70);
        assert!(report.score < 80);
        assert!(
            report
                .applied_caps
                .iter()
                .any(|cap| cap.id == "materialized-zero-source-files")
        );
    }

    #[test]
    fn materialized_without_receipt_is_capped_below_green() {
        let report = score_forge_import(DxForgeImportScoreInput {
            no_node_modules: true,
            source_dir_ready: true,
            materialized: true,
            accepted_materialization_receipt_present: false,
            metadata_present: true,
            provenance_verified: true,
            artifact_integrity_present: true,
            license_declared: true,
            license_file_present: true,
            license_reviewed: true,
            advisory_evidence_present: true,
            advisory_reviewed: true,
            popularity_evidence_present: true,
            sbom_present: true,
            exports_present: true,
            dependency_count: 0,
            files_considered: 3,
            files_rejected: 0,
            package_installs_run: false,
            lifecycle_scripts_executed: false,
            risk_flags: Vec::new(),
        });

        assert_eq!(report.score_ceiling, 79);
        assert!(report.score < 80);
        assert!(
            report
                .applied_caps
                .iter()
                .any(|cap| cap.id == "materialization-receipt-missing")
        );
    }

    #[test]
    fn large_dependency_graph_is_capped_until_sliced_or_bridged() {
        let report = score_forge_import(DxForgeImportScoreInput {
            no_node_modules: true,
            source_dir_ready: true,
            materialized: true,
            accepted_materialization_receipt_present: true,
            metadata_present: true,
            provenance_verified: true,
            artifact_integrity_present: true,
            license_declared: true,
            license_file_present: true,
            license_reviewed: true,
            advisory_evidence_present: true,
            advisory_reviewed: true,
            popularity_evidence_present: true,
            sbom_present: true,
            exports_present: true,
            dependency_count: 121,
            files_considered: 6,
            files_rejected: 0,
            package_installs_run: false,
            lifecycle_scripts_executed: false,
            risk_flags: Vec::new(),
        });

        assert_eq!(report.score_ceiling, 72);
        assert!(report.score < 80);
        assert!(
            report
                .applied_caps
                .iter()
                .any(|cap| cap.id == "large-dependency-graph")
        );
    }

    #[test]
    fn declared_evidence_without_review_cannot_claim_green_score() {
        let report = score_forge_import(DxForgeImportScoreInput {
            no_node_modules: true,
            source_dir_ready: true,
            materialized: true,
            accepted_materialization_receipt_present: true,
            metadata_present: true,
            provenance_verified: false,
            artifact_integrity_present: true,
            license_declared: true,
            license_file_present: true,
            license_reviewed: false,
            advisory_evidence_present: true,
            advisory_reviewed: false,
            popularity_evidence_present: true,
            sbom_present: false,
            exports_present: true,
            dependency_count: 0,
            files_considered: 3,
            files_rejected: 0,
            package_installs_run: false,
            lifecycle_scripts_executed: false,
            risk_flags: Vec::new(),
        });

        assert_eq!(report.score_ceiling, 94);
        assert_eq!(report.traffic, "yellow");
        for cap_id in [
            "provenance-verification-pending",
            "license-review-pending",
            "advisory-review-pending",
            "source-sbom-missing",
        ] {
            assert!(
                report.applied_caps.iter().any(|cap| cap.id == cap_id),
                "missing cap {cap_id}"
            );
        }
    }
}
