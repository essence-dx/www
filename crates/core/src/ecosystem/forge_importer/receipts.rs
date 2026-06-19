use serde::{Deserialize, Serialize};

use super::types::{
    DxForgeImportDecision, DxForgeImportEcosystem, DxForgeImportPhase, DxForgeImportPolicyDecision,
    DxForgeImportRiskFlag, DxForgeImportSliceKind,
};

/// Stable schema for policy-only import firewall reports.
pub const DX_FORGE_IMPORT_FIREWALL_SCHEMA: &str = "dx.forge.import_firewall";

/// Policy report that future CLI commands can serialize as JSON, `.sr`, and machine cache.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeImportFirewallReport {
    /// Receipt schema.
    pub schema: String,
    /// Schema version.
    pub version: u32,
    /// Ecosystem under review.
    pub ecosystem: DxForgeImportEcosystem,
    /// Requested package or module specifier.
    pub package: String,
    /// Current review phase.
    pub phase: DxForgeImportPhase,
    /// Desired source slice kind.
    pub slice_kind: DxForgeImportSliceKind,
    /// Final policy decision for this report.
    pub decision: DxForgeImportDecision,
    /// Whether package-manager installs were executed.
    pub package_installs_run: bool,
    /// Whether lifecycle or setup scripts were executed.
    pub lifecycle_scripts_executed: bool,
    /// Whether quarantine paths are still non-importable by app code.
    pub quarantine_is_not_importable: bool,
    /// Risk flags recorded during review.
    pub risk_flags: Vec<DxForgeImportRiskFlag>,
    /// Detailed policy decisions.
    pub policy_decisions: Vec<DxForgeImportPolicyDecision>,
}

impl DxForgeImportFirewallReport {
    /// Create a fail-closed report for one import phase.
    pub fn new(
        ecosystem: DxForgeImportEcosystem,
        package: impl Into<String>,
        phase: DxForgeImportPhase,
        slice_kind: DxForgeImportSliceKind,
        decision: DxForgeImportDecision,
    ) -> Self {
        Self {
            schema: DX_FORGE_IMPORT_FIREWALL_SCHEMA.to_string(),
            version: 1,
            ecosystem,
            package: package.into(),
            phase,
            slice_kind,
            decision,
            package_installs_run: false,
            lifecycle_scripts_executed: false,
            quarantine_is_not_importable: true,
            risk_flags: Vec::new(),
            policy_decisions: Vec::new(),
        }
    }
}
