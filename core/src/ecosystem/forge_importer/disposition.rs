use serde::{Deserialize, Serialize};

use super::types::{DxForgeImportRiskFlag, DxForgeImportSliceKind};

/// Stable disposition model used to decide what Forge is allowed to do with a package.
pub const DX_FORGE_PACKAGE_DISPOSITION_MODEL_VERSION: &str = "dx-forge-package-disposition-2026-06";

/// Final action family for an external package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgePackageDispositionKind {
    /// Forge can write reviewed source-owned files.
    Materialize,
    /// Forge can prepare a smaller reviewed source slice, but still needs acceptance.
    Slice,
    /// Forge must keep the package behind an explicit boundary.
    Bridge,
    /// Forge must refuse the package or current source candidate.
    Reject,
}

/// Bridge family when a package cannot honestly become source-owned source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgePackageBridgeKind {
    /// Package is a CLI, formatter, code generator, or build-side tool.
    Tool,
    /// Package needs native, prebuilt, or platform-specific binary material.
    BinarySnapshot,
    /// Package needs a native runtime, FFI, cgo, proc macro, or dynamic host.
    NativeRuntime,
    /// Package semantics depend on a hosted service or credentialed API.
    HostedService,
    /// Package has not been inspected enough to become source-owned.
    PackageManagerBoundary,
}

/// Inputs collected before the disposition decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgePackageDispositionInput {
    pub requested_kind: DxForgeImportSliceKind,
    pub source_dir_ready: bool,
    pub materialized: bool,
    pub accepted_materialization_receipt_present: bool,
    pub no_node_modules: bool,
    pub package_installs_run: bool,
    pub lifecycle_scripts_executed: bool,
    pub files_considered: usize,
    pub files_rejected: usize,
    pub score: u8,
    pub score_ceiling: u8,
    pub risk_flags: Vec<DxForgeImportRiskFlag>,
}

/// Durable package disposition report mirrored into JSON, `.sr`, and machine caches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgePackageDispositionReport {
    pub model_version: String,
    pub kind: DxForgePackageDispositionKind,
    pub route: String,
    pub slice_kind: DxForgeImportSliceKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bridge_kind: Option<DxForgePackageBridgeKind>,
    pub ownership_claim: String,
    pub importable_source: bool,
    pub materializes_source: bool,
    pub requires_accepted_receipt: bool,
    pub reason: String,
    pub remediation: String,
}

/// Classify one package without pretending every external package can be source-owned.
pub fn classify_forge_package_disposition(
    input: DxForgePackageDispositionInput,
) -> DxForgePackageDispositionReport {
    let blocking_security = input.risk_flags.iter().any(|risk| {
        matches!(
            risk,
            DxForgeImportRiskFlag::PlaintextSecret
                | DxForgeImportRiskFlag::ProjectEscape
                | DxForgeImportRiskFlag::UnsafePath
                | DxForgeImportRiskFlag::Symlink
        )
    });
    if input.package_installs_run
        || input.lifecycle_scripts_executed
        || blocking_security
        || input.files_rejected > 0
    {
        return disposition(
            DxForgePackageDispositionKind::Reject,
            "reject-security-or-conflict",
            input.requested_kind,
            None,
            "rejected",
            false,
            false,
            true,
            "Package import failed a hard security, execution, path, or overwrite gate.",
            "Keep the package quarantined, remove the blocking risk, and attach a new accepted receipt before retrying.",
        );
    }

    if let Some(bridge_kind) = bridge_kind_for_risks(&input.risk_flags) {
        return disposition(
            DxForgePackageDispositionKind::Bridge,
            bridge_route(bridge_kind),
            bridge_slice_kind(bridge_kind),
            Some(bridge_kind),
            "bridged-boundary",
            false,
            false,
            true,
            "Forge found behavior that cannot honestly become direct source-owned app code.",
            "Create an explicit bridge, tool package, binary snapshot, or runtime wrapper with its own receipt.",
        );
    }

    if input.materialized
        && input.source_dir_ready
        && input.accepted_materialization_receipt_present
        && input.no_node_modules
        && input.files_considered > 0
        && input.files_rejected == 0
        && input.score >= 95
        && input.score_ceiling >= 95
        && !input
            .risk_flags
            .iter()
            .any(|risk| risk.blocks_materialization())
    {
        return disposition(
            DxForgePackageDispositionKind::Materialize,
            "materialize-source-owned",
            input.requested_kind,
            None,
            "source-owned",
            true,
            true,
            true,
            "Reviewed source files were written with Forge manifest and receipt evidence.",
            "Use the materialized source through the Forge resolver and keep receipts with the project.",
        );
    }

    if input.source_dir_ready
        && input.no_node_modules
        && input.files_considered > 0
        && input.score >= 80
        && input.score_ceiling >= 80
        && !input
            .risk_flags
            .iter()
            .any(|risk| risk.blocks_materialization())
    {
        return disposition(
            DxForgePackageDispositionKind::Slice,
            "slice-reviewed-source",
            input.requested_kind,
            None,
            "reviewed-source-candidate",
            false,
            false,
            true,
            "Inspected source can be reduced into a reviewed Forge slice, but it is not app-importable yet.",
            "Accept the slice receipt, then run write mode to materialize source-owned files.",
        );
    }

    disposition(
        DxForgePackageDispositionKind::Bridge,
        "bridge-uninspected-external-boundary",
        DxForgeImportSliceKind::MetadataOnly,
        Some(DxForgePackageBridgeKind::PackageManagerBoundary),
        "external-boundary",
        false,
        false,
        true,
        "Forge has not inspected enough package source to claim source ownership.",
        "Acquire or provide an inspected source directory, then produce a source slice, bridge, or rejection receipt.",
    )
}

fn bridge_kind_for_risks(risks: &[DxForgeImportRiskFlag]) -> Option<DxForgePackageBridgeKind> {
    if risks.iter().any(|risk| {
        matches!(
            risk,
            DxForgeImportRiskFlag::NativeBinary | DxForgeImportRiskFlag::DynamicExecution
        )
    }) {
        return Some(DxForgePackageBridgeKind::NativeRuntime);
    }
    if risks.iter().any(|risk| {
        matches!(
            risk,
            DxForgeImportRiskFlag::LifecycleScript | DxForgeImportRiskFlag::InstallHook
        )
    }) {
        return Some(DxForgePackageBridgeKind::Tool);
    }
    if risks.iter().any(|risk| {
        matches!(
            risk,
            DxForgeImportRiskFlag::DynamicImport
                | DxForgeImportRiskFlag::HugeDependencyGraph
                | DxForgeImportRiskFlag::RuntimeMismatch
                | DxForgeImportRiskFlag::SideEffectImport
        )
    }) {
        return Some(DxForgePackageBridgeKind::PackageManagerBoundary);
    }
    if risks
        .iter()
        .any(|risk| matches!(risk, DxForgeImportRiskFlag::ObfuscatedBlob))
    {
        return Some(DxForgePackageBridgeKind::BinarySnapshot);
    }
    None
}

fn bridge_route(kind: DxForgePackageBridgeKind) -> &'static str {
    match kind {
        DxForgePackageBridgeKind::Tool => "bridge-tool-package",
        DxForgePackageBridgeKind::BinarySnapshot => "bridge-binary-snapshot",
        DxForgePackageBridgeKind::NativeRuntime => "bridge-native-runtime",
        DxForgePackageBridgeKind::HostedService => "bridge-hosted-service",
        DxForgePackageBridgeKind::PackageManagerBoundary => "bridge-package-boundary",
    }
}

fn bridge_slice_kind(kind: DxForgePackageBridgeKind) -> DxForgeImportSliceKind {
    match kind {
        DxForgePackageBridgeKind::Tool => DxForgeImportSliceKind::ToolBridge,
        DxForgePackageBridgeKind::BinarySnapshot => DxForgeImportSliceKind::BinaryBridge,
        DxForgePackageBridgeKind::NativeRuntime => DxForgeImportSliceKind::RuntimeBridge,
        DxForgePackageBridgeKind::HostedService => DxForgeImportSliceKind::RuntimeBridge,
        DxForgePackageBridgeKind::PackageManagerBoundary => DxForgeImportSliceKind::MetadataOnly,
    }
}

fn disposition(
    kind: DxForgePackageDispositionKind,
    route: &str,
    slice_kind: DxForgeImportSliceKind,
    bridge_kind: Option<DxForgePackageBridgeKind>,
    ownership_claim: &str,
    importable_source: bool,
    materializes_source: bool,
    requires_accepted_receipt: bool,
    reason: &str,
    remediation: &str,
) -> DxForgePackageDispositionReport {
    DxForgePackageDispositionReport {
        model_version: DX_FORGE_PACKAGE_DISPOSITION_MODEL_VERSION.to_string(),
        kind,
        route: route.to_string(),
        slice_kind,
        bridge_kind,
        ownership_claim: ownership_claim.to_string(),
        importable_source,
        materializes_source,
        requires_accepted_receipt,
        reason: reason.to_string(),
        remediation: remediation.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clean_input() -> DxForgePackageDispositionInput {
        DxForgePackageDispositionInput {
            requested_kind: DxForgeImportSliceKind::SourceSlice,
            source_dir_ready: true,
            materialized: false,
            accepted_materialization_receipt_present: false,
            no_node_modules: true,
            package_installs_run: false,
            lifecycle_scripts_executed: false,
            files_considered: 2,
            files_rejected: 0,
            score: 90,
            score_ceiling: 100,
            risk_flags: Vec::new(),
        }
    }

    #[test]
    fn inspected_source_becomes_slice_until_written() {
        let report = classify_forge_package_disposition(clean_input());

        assert_eq!(report.kind, DxForgePackageDispositionKind::Slice);
        assert_eq!(report.route, "slice-reviewed-source");
        assert!(!report.importable_source);
        assert!(report.requires_accepted_receipt);
    }

    #[test]
    fn materialized_source_claims_source_ownership() {
        let mut input = clean_input();
        input.materialized = true;
        input.accepted_materialization_receipt_present = true;
        input.files_considered = 2;
        input.score = 96;
        input.score_ceiling = 100;

        let report = classify_forge_package_disposition(input);

        assert_eq!(report.kind, DxForgePackageDispositionKind::Materialize);
        assert_eq!(report.ownership_claim, "source-owned");
        assert!(report.importable_source);
        assert!(report.materializes_source);
    }

    #[test]
    fn materialized_flag_still_requires_clean_materialization_evidence() {
        let mut low_score = clean_input();
        low_score.materialized = true;
        low_score.accepted_materialization_receipt_present = true;
        low_score.score = 84;
        low_score.score_ceiling = 84;
        let low_score_report = classify_forge_package_disposition(low_score);
        assert_ne!(
            low_score_report.kind,
            DxForgePackageDispositionKind::Materialize
        );

        let mut no_files = clean_input();
        no_files.materialized = true;
        no_files.accepted_materialization_receipt_present = true;
        no_files.files_considered = 0;
        no_files.score = 96;
        no_files.score_ceiling = 100;
        let no_files_report = classify_forge_package_disposition(no_files);
        assert_ne!(
            no_files_report.kind,
            DxForgePackageDispositionKind::Materialize
        );

        let mut runtime_mismatch = clean_input();
        runtime_mismatch.materialized = true;
        runtime_mismatch.accepted_materialization_receipt_present = true;
        runtime_mismatch.risk_flags = vec![DxForgeImportRiskFlag::RuntimeMismatch];
        let mismatch_report = classify_forge_package_disposition(runtime_mismatch);
        assert_eq!(mismatch_report.kind, DxForgePackageDispositionKind::Bridge);
    }

    #[test]
    fn materialized_source_requires_accepted_materialization_receipt() {
        let mut input = clean_input();
        input.materialized = true;
        input.files_considered = 2;
        input.score = 96;
        input.score_ceiling = 100;

        let report = classify_forge_package_disposition(input);

        assert_ne!(report.kind, DxForgePackageDispositionKind::Materialize);
        assert!(report.requires_accepted_receipt);
        assert_eq!(report.ownership_claim, "reviewed-source-candidate");
    }

    #[test]
    fn zero_files_never_becomes_reviewed_slice() {
        let mut input = clean_input();
        input.files_considered = 0;
        input.score = 96;
        input.score_ceiling = 100;

        let report = classify_forge_package_disposition(input);

        assert_eq!(report.kind, DxForgePackageDispositionKind::Bridge);
        assert_eq!(report.route, "bridge-uninspected-external-boundary");
    }

    #[test]
    fn blocking_risks_never_materialize_even_with_green_score() {
        let risks = [
            DxForgeImportRiskFlag::LifecycleScript,
            DxForgeImportRiskFlag::InstallHook,
            DxForgeImportRiskFlag::NativeBinary,
            DxForgeImportRiskFlag::ObfuscatedBlob,
            DxForgeImportRiskFlag::DynamicExecution,
            DxForgeImportRiskFlag::HugeDependencyGraph,
            DxForgeImportRiskFlag::UnsafePath,
            DxForgeImportRiskFlag::Symlink,
            DxForgeImportRiskFlag::ProjectEscape,
            DxForgeImportRiskFlag::MissingIntegrity,
            DxForgeImportRiskFlag::RuntimeMismatch,
            DxForgeImportRiskFlag::DynamicImport,
            DxForgeImportRiskFlag::SideEffectImport,
            DxForgeImportRiskFlag::PlaintextSecret,
        ];

        for risk in risks {
            assert!(risk.blocks_materialization());
            let mut input = clean_input();
            input.materialized = true;
            input.accepted_materialization_receipt_present = true;
            input.files_considered = 2;
            input.score = 100;
            input.score_ceiling = 100;
            input.risk_flags = vec![risk];

            let report = classify_forge_package_disposition(input);

            assert_ne!(report.kind, DxForgePackageDispositionKind::Materialize);
        }
    }

    #[test]
    fn native_binary_is_bridged_not_materialized() {
        let mut input = clean_input();
        input.risk_flags = vec![DxForgeImportRiskFlag::NativeBinary];

        let report = classify_forge_package_disposition(input);

        assert_eq!(report.kind, DxForgePackageDispositionKind::Bridge);
        assert_eq!(
            report.bridge_kind,
            Some(DxForgePackageBridgeKind::NativeRuntime)
        );
        assert!(!report.materializes_source);
    }

    #[test]
    fn unsafe_path_is_rejected() {
        let mut input = clean_input();
        input.risk_flags = vec![DxForgeImportRiskFlag::UnsafePath];

        let report = classify_forge_package_disposition(input);

        assert_eq!(report.kind, DxForgePackageDispositionKind::Reject);
        assert_eq!(report.route, "reject-security-or-conflict");
    }

    #[test]
    fn package_disposition_classifier_covers_materialize_slice_bridge_reject() {
        let slice = classify_forge_package_disposition(clean_input());
        assert_eq!(slice.kind, DxForgePackageDispositionKind::Slice);

        let mut materialize_input = clean_input();
        materialize_input.materialized = true;
        materialize_input.accepted_materialization_receipt_present = true;
        materialize_input.files_considered = 2;
        materialize_input.score = 96;
        materialize_input.score_ceiling = 100;
        let materialize = classify_forge_package_disposition(materialize_input);
        assert_eq!(materialize.kind, DxForgePackageDispositionKind::Materialize);

        let mut bridge_input = clean_input();
        bridge_input.risk_flags = vec![DxForgeImportRiskFlag::NativeBinary];
        let bridge = classify_forge_package_disposition(bridge_input);
        assert_eq!(bridge.kind, DxForgePackageDispositionKind::Bridge);

        let mut reject_input = clean_input();
        reject_input.risk_flags = vec![DxForgeImportRiskFlag::UnsafePath];
        let reject = classify_forge_package_disposition(reject_input);
        assert_eq!(reject.kind, DxForgePackageDispositionKind::Reject);
    }
}
