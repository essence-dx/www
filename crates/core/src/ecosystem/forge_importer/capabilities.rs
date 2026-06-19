use serde::{Deserialize, Serialize};

use super::disposition::DxForgePackageBridgeKind;
use super::types::{DX_FORGE_IMPORT_ECOSYSTEMS, DxForgeImportEcosystem, DxForgeImportSliceKind};

/// Stable model for ecosystem-level Forge import capability reports.
pub const DX_FORGE_IMPORT_CAPABILITY_MODEL_VERSION: &str = "dx-forge-import-capability-2026-06";

/// High-level capability tier for an external package ecosystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeImportCapabilityTier {
    /// Forge can plan, acquire metadata, inspect source, and create reviewed JS adapters.
    ReviewedJavascriptAdapter,
    /// Forge can plan, acquire metadata, inspect source snapshots, and materialize reviewed files.
    SourceSnapshot,
    /// Forge can plan and quarantine, but runtime use needs an explicit bridge.
    RuntimeBridge,
}

/// Honest capability report for one external ecosystem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeImportEcosystemCapability {
    pub model_version: String,
    pub ecosystem: DxForgeImportEcosystem,
    pub ecosystem_segment: String,
    pub tier: DxForgeImportCapabilityTier,
    pub capability_score: u8,
    pub plan_surface: bool,
    pub non_executing_acquisition: bool,
    pub local_source_inspection: bool,
    pub reviewed_source_materialization: bool,
    pub direct_www_bare_import: bool,
    pub package_score_can_reach_100: bool,
    pub live_registry_fetching: bool,
    pub package_manager_execution: bool,
    pub universal_package_compatibility_claim: bool,
    pub source_dir_required_for_materialization: bool,
    pub score_100_requirements: Vec<String>,
    pub supported_slice_kinds: Vec<DxForgeImportSliceKind>,
    pub bridge_kinds: Vec<DxForgePackageBridgeKind>,
    pub honest_limitations: Vec<String>,
    pub clean_package_requirements: Vec<String>,
}

/// Return the current capability report for one ecosystem.
pub fn import_capability_for_ecosystem(
    ecosystem: DxForgeImportEcosystem,
) -> DxForgeImportEcosystemCapability {
    let direct_www_bare_import = matches!(
        ecosystem,
        DxForgeImportEcosystem::Npm | DxForgeImportEcosystem::Jsr
    );
    let tier = if direct_www_bare_import {
        DxForgeImportCapabilityTier::ReviewedJavascriptAdapter
    } else {
        DxForgeImportCapabilityTier::SourceSnapshot
    };
    let mut supported_slice_kinds = vec![
        DxForgeImportSliceKind::SourceSlice,
        DxForgeImportSliceKind::SymbolSlice,
        DxForgeImportSliceKind::Adapter,
        DxForgeImportSliceKind::SourceCopy,
        DxForgeImportSliceKind::AssetSlice,
        DxForgeImportSliceKind::MetadataOnly,
    ];
    if matches!(
        ecosystem,
        DxForgeImportEcosystem::Cargo
            | DxForgeImportEcosystem::Go
            | DxForgeImportEcosystem::Pub
            | DxForgeImportEcosystem::Maven
            | DxForgeImportEcosystem::Nuget
            | DxForgeImportEcosystem::Swift
            | DxForgeImportEcosystem::Hex
            | DxForgeImportEcosystem::Cran
    ) {
        supported_slice_kinds.push(DxForgeImportSliceKind::ToolBridge);
        supported_slice_kinds.push(DxForgeImportSliceKind::RuntimeBridge);
    }
    if matches!(
        ecosystem,
        DxForgeImportEcosystem::Npm
            | DxForgeImportEcosystem::Pip
            | DxForgeImportEcosystem::Pub
            | DxForgeImportEcosystem::Maven
            | DxForgeImportEcosystem::Nuget
            | DxForgeImportEcosystem::Gem
            | DxForgeImportEcosystem::Swift
            | DxForgeImportEcosystem::Hex
            | DxForgeImportEcosystem::Cran
    ) {
        supported_slice_kinds.push(DxForgeImportSliceKind::BinaryBridge);
    }

    DxForgeImportEcosystemCapability {
        model_version: DX_FORGE_IMPORT_CAPABILITY_MODEL_VERSION.to_string(),
        ecosystem,
        ecosystem_segment: ecosystem.as_segment().to_string(),
        tier,
        capability_score: if direct_www_bare_import { 92 } else { 84 },
        plan_surface: true,
        non_executing_acquisition: true,
        local_source_inspection: true,
        reviewed_source_materialization: true,
        direct_www_bare_import,
        package_score_can_reach_100: true,
        live_registry_fetching: matches!(ecosystem, DxForgeImportEcosystem::Npm),
        package_manager_execution: false,
        universal_package_compatibility_claim: false,
        source_dir_required_for_materialization: true,
        score_100_requirements: vec![
            "verified package provenance and registry identity".to_string(),
            "source integrity hashes plus source bill of materials receipt".to_string(),
            "reviewer-accepted license evidence".to_string(),
            "live or reviewed advisory coverage".to_string(),
            "accepted materialization receipt for written source".to_string(),
            "no package-manager installs, lifecycle scripts, or node_modules authority".to_string(),
        ],
        supported_slice_kinds,
        bridge_kinds: ecosystem_bridge_kinds(ecosystem),
        honest_limitations: ecosystem_limitations(ecosystem, direct_www_bare_import),
        clean_package_requirements: vec![
            "inspected source directory outside package-manager install trees".to_string(),
            "metadata, integrity, license, and advisory evidence".to_string(),
            "no lifecycle/setup/build script execution".to_string(),
            "safe project-relative target paths".to_string(),
            "accepted Forge import receipt before write or rewrite".to_string(),
        ],
    }
}

/// Return capability reports for every modeled import ecosystem in stable order.
pub fn default_import_capabilities() -> Vec<DxForgeImportEcosystemCapability> {
    DX_FORGE_IMPORT_ECOSYSTEMS
        .iter()
        .copied()
        .map(import_capability_for_ecosystem)
        .collect()
}

fn ecosystem_bridge_kinds(ecosystem: DxForgeImportEcosystem) -> Vec<DxForgePackageBridgeKind> {
    let mut kinds = vec![DxForgePackageBridgeKind::PackageManagerBoundary];
    if matches!(
        ecosystem,
        DxForgeImportEcosystem::Npm
            | DxForgeImportEcosystem::Pip
            | DxForgeImportEcosystem::Pub
            | DxForgeImportEcosystem::Maven
            | DxForgeImportEcosystem::Nuget
            | DxForgeImportEcosystem::Gem
            | DxForgeImportEcosystem::Swift
            | DxForgeImportEcosystem::Hex
            | DxForgeImportEcosystem::Cran
    ) {
        kinds.push(DxForgePackageBridgeKind::BinarySnapshot);
    }
    if matches!(
        ecosystem,
        DxForgeImportEcosystem::Pip
            | DxForgeImportEcosystem::Cargo
            | DxForgeImportEcosystem::Go
            | DxForgeImportEcosystem::Pub
            | DxForgeImportEcosystem::Maven
            | DxForgeImportEcosystem::Nuget
            | DxForgeImportEcosystem::Composer
            | DxForgeImportEcosystem::Gem
            | DxForgeImportEcosystem::Swift
            | DxForgeImportEcosystem::Hex
            | DxForgeImportEcosystem::Cran
    ) {
        kinds.push(DxForgePackageBridgeKind::NativeRuntime);
    }
    if matches!(
        ecosystem,
        DxForgeImportEcosystem::Cargo
            | DxForgeImportEcosystem::Go
            | DxForgeImportEcosystem::Maven
            | DxForgeImportEcosystem::Nuget
            | DxForgeImportEcosystem::Swift
            | DxForgeImportEcosystem::Hex
            | DxForgeImportEcosystem::Cran
    ) {
        kinds.push(DxForgePackageBridgeKind::Tool);
    }
    kinds
}

fn ecosystem_limitations(
    ecosystem: DxForgeImportEcosystem,
    direct_www_bare_import: bool,
) -> Vec<String> {
    let mut limitations = vec![
        "no package-manager install or package lifecycle execution".to_string(),
        "not universal package compatibility".to_string(),
        "source ownership requires accepted receipts and local source evidence".to_string(),
    ];
    if !matches!(ecosystem, DxForgeImportEcosystem::Npm) {
        limitations.push("no live registry fetching by default".to_string());
    }
    if !direct_www_bare_import {
        limitations.push(
            "direct WWW bare imports require a reviewed adapter or explicit bridge".to_string(),
        );
    }
    if matches!(
        ecosystem,
        DxForgeImportEcosystem::Pip
            | DxForgeImportEcosystem::Cargo
            | DxForgeImportEcosystem::Go
            | DxForgeImportEcosystem::Pub
            | DxForgeImportEcosystem::Maven
            | DxForgeImportEcosystem::Nuget
            | DxForgeImportEcosystem::Composer
            | DxForgeImportEcosystem::Gem
            | DxForgeImportEcosystem::Swift
            | DxForgeImportEcosystem::Hex
            | DxForgeImportEcosystem::Cran
    ) {
        limitations.push(
            "native, toolchain, server, or language runtime behavior stays behind a bridge"
                .to_string(),
        );
    }
    limitations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn javascript_registries_are_the_only_direct_www_bare_import_surfaces() {
        for capability in default_import_capabilities() {
            let expected = matches!(
                capability.ecosystem,
                DxForgeImportEcosystem::Npm | DxForgeImportEcosystem::Jsr
            );
            assert_eq!(capability.direct_www_bare_import, expected);
            assert_eq!(
                capability.live_registry_fetching,
                matches!(capability.ecosystem, DxForgeImportEcosystem::Npm)
            );
            assert!(!capability.package_manager_execution);
            assert!(!capability.universal_package_compatibility_claim);
            assert!(capability.source_dir_required_for_materialization);
            assert!(capability.package_score_can_reach_100);
            assert!(
                capability
                    .score_100_requirements
                    .iter()
                    .any(|requirement| requirement.contains("verified package provenance"))
            );
            assert!(
                capability
                    .score_100_requirements
                    .iter()
                    .any(|requirement| requirement.contains("no package-manager installs"))
            );
        }
    }

    #[test]
    fn every_capability_has_honest_limitations_and_bridge_boundaries() {
        for capability in default_import_capabilities() {
            assert!(capability.plan_surface);
            assert!(capability.non_executing_acquisition);
            assert!(capability.local_source_inspection);
            assert!(capability.reviewed_source_materialization);
            assert!(capability.capability_score < 100);
            assert!(
                capability
                    .honest_limitations
                    .iter()
                    .any(|limitation| limitation.contains("not universal package compatibility"))
            );
            assert!(
                capability
                    .bridge_kinds
                    .contains(&DxForgePackageBridgeKind::PackageManagerBoundary)
            );
            assert!(capability.score_100_requirements.len() >= 5);
        }
    }
}
