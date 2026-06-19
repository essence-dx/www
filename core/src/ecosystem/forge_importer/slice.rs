use serde::{Deserialize, Serialize};

use super::types::{
    DxForgeImportDecision, DxForgeImportPhase, DxForgeImportPolicyDecision,
    DxForgeImportPolicyGate, DxForgeImportRiskFlag, DxForgeImportSliceKind,
};

/// Input used to classify a reviewed source slice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeImportSliceCandidate {
    /// Preferred slice kind when policy allows materialization.
    pub requested_kind: DxForgeImportSliceKind,
    /// Risk flags collected during acquisition, quarantine, and analysis.
    pub risk_flags: Vec<DxForgeImportRiskFlag>,
    /// Whether this candidate writes app-importable files.
    pub writes_importable_source: bool,
}

/// Final slice decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeImportSliceDecision {
    /// Accepted slice kind or blocked result.
    pub slice_kind: DxForgeImportSliceKind,
    /// Policy decision for the slice phase.
    pub decision: DxForgeImportDecision,
    /// Reasons recorded in receipts.
    pub policy_decisions: Vec<DxForgeImportPolicyDecision>,
}

/// Classify a source slice with fail-closed materialization rules.
pub fn classify_import_slice(candidate: DxForgeImportSliceCandidate) -> DxForgeImportSliceDecision {
    let blocking_flags: Vec<_> = candidate
        .risk_flags
        .iter()
        .copied()
        .filter(|risk| risk.blocks_materialization())
        .collect();

    if !blocking_flags.is_empty() && candidate.writes_importable_source {
        return DxForgeImportSliceDecision {
            slice_kind: DxForgeImportSliceKind::Blocked,
            decision: DxForgeImportDecision::Block,
            policy_decisions: blocking_flags
                .into_iter()
                .map(|risk| DxForgeImportPolicyDecision {
                    phase: DxForgeImportPhase::Slice,
                    gate: DxForgeImportPolicyGate::SliceScope,
                    decision: DxForgeImportDecision::Block,
                    code: risk.as_reason_code().to_string(),
                    risk_flags: vec![risk],
                    evidence_path: None,
                    detail: "Risk flag blocks app-importable Forge source materialization."
                        .to_string(),
                    remediation:
                        "Keep the package in quarantine or create a smaller reviewed adapter."
                            .to_string(),
                })
                .collect(),
        };
    }

    let manual_flags: Vec<_> = candidate
        .risk_flags
        .iter()
        .copied()
        .filter(|risk| !risk.blocks_materialization())
        .collect();
    if !manual_flags.is_empty() {
        return DxForgeImportSliceDecision {
            slice_kind: candidate.requested_kind,
            decision: DxForgeImportDecision::ManualReview,
            policy_decisions: manual_flags
                .into_iter()
                .map(|risk| DxForgeImportPolicyDecision {
                    phase: DxForgeImportPhase::Slice,
                    gate: DxForgeImportPolicyGate::SliceScope,
                    decision: DxForgeImportDecision::ManualReview,
                    code: risk.as_reason_code().to_string(),
                    risk_flags: vec![risk],
                    evidence_path: None,
                    detail: "Risk flag requires review before Forge accepts the slice.".to_string(),
                    remediation: "Attach reviewer acceptance and receipt evidence before writing."
                        .to_string(),
                })
                .collect(),
        };
    }

    DxForgeImportSliceDecision {
        slice_kind: candidate.requested_kind,
        decision: DxForgeImportDecision::Accept,
        policy_decisions: vec![DxForgeImportPolicyDecision {
            phase: DxForgeImportPhase::Slice,
            gate: DxForgeImportPolicyGate::SliceScope,
            decision: DxForgeImportDecision::Accept,
            code: "source-slice-policy-clean".to_string(),
            risk_flags: Vec::new(),
            evidence_path: None,
            detail: "No blocking or manual-review risk flags were raised.".to_string(),
            remediation: "No remediation required.".to_string(),
        }],
    }
}
