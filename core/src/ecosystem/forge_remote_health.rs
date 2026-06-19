//! Health evaluation for Forge remote object HEAD receipts.

use serde::{Deserialize, Serialize};

use super::forge_registry::{
    DxForgeRemoteObjectHeadExecutionReceipt, DxForgeRemoteObjectHeadExecutionStatus,
    DxForgeRemoteProviderKind,
};

/// Health status for a measured remote object HEAD check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxForgeRemoteObjectHeadHealthStatus {
    /// The receipt did not execute this metadata check.
    NotMeasured,
    /// The object exists and matches the available receipt metadata.
    Healthy,
    /// A required object is missing from the remote provider.
    MissingRequiredObject,
    /// An optional object is missing from the remote provider.
    MissingOptionalObject,
    /// The provider-reported byte length does not match the package manifest.
    ByteMismatch,
}

/// Per-object health result derived from a read-only HEAD execution receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRemoteObjectHeadHealthCheck {
    /// Object role, for example `package-manifest` or `content-blob`.
    pub intent: String,
    /// Provider metadata operation recorded in the source receipt.
    pub metadata_operation: String,
    /// Redacted remote object key.
    pub object_key: String,
    /// Whether this object blocks a remote install when unhealthy.
    pub required: bool,
    /// Health status for consumers.
    pub status: DxForgeRemoteObjectHeadHealthStatus,
    /// Whether this single object is safe for a remote install.
    pub safe_for_remote_install: bool,
    /// Manifest-declared byte length when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_bytes: Option<u64>,
    /// Provider-measured byte length when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measured_bytes: Option<u64>,
    /// Human-readable blocker or warning for this object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Aggregate health summary for a Forge remote object HEAD receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeRemoteObjectHeadHealthEvaluation {
    /// Stable schema version for CLI, dx-check, Zed, and future Forge consumers.
    pub schema_version: String,
    /// Source receipt schema used for this evaluation.
    pub source_receipt_schema: String,
    /// Provider kind from the source receipt.
    pub provider_kind: DxForgeRemoteProviderKind,
    /// Canonical package id.
    pub package_id: String,
    /// Manifest package version.
    pub version: String,
    /// Whether all blocking remote object checks are safe.
    pub safe_for_remote_install: bool,
    /// Per-object health checks.
    pub checks: Vec<DxForgeRemoteObjectHeadHealthCheck>,
    /// Missing required object count.
    pub missing_required_count: usize,
    /// Missing optional object count.
    pub missing_optional_count: usize,
    /// Byte mismatch count.
    pub byte_mismatch_count: usize,
    /// Count of checks that block a remote install.
    pub blocking_check_count: usize,
    /// Warnings suitable for CLI/check receipts.
    pub warnings: Vec<String>,
    /// Next actions suitable for CLI/check receipts.
    pub next_actions: Vec<String>,
}

/// Evaluate a read-only R2/S3 object HEAD receipt without touching the network.
pub fn evaluate_r2_remote_object_head_receipt_health(
    receipt: &DxForgeRemoteObjectHeadExecutionReceipt,
) -> DxForgeRemoteObjectHeadHealthEvaluation {
    let checks: Vec<_> = receipt
        .checks
        .iter()
        .map(|check| {
            let status = if check.status != DxForgeRemoteObjectHeadExecutionStatus::Measured
                || !check.executed
            {
                DxForgeRemoteObjectHeadHealthStatus::NotMeasured
            } else if check.measured_exists == Some(false) {
                if check.required {
                    DxForgeRemoteObjectHeadHealthStatus::MissingRequiredObject
                } else {
                    DxForgeRemoteObjectHeadHealthStatus::MissingOptionalObject
                }
            } else if matches!(
                (check.expected_bytes, check.measured_bytes),
                (Some(expected), Some(measured)) if expected != measured
            ) {
                DxForgeRemoteObjectHeadHealthStatus::ByteMismatch
            } else {
                DxForgeRemoteObjectHeadHealthStatus::Healthy
            };

            let safe_for_remote_install = match status {
                DxForgeRemoteObjectHeadHealthStatus::Healthy => true,
                DxForgeRemoteObjectHeadHealthStatus::MissingOptionalObject => true,
                DxForgeRemoteObjectHeadHealthStatus::NotMeasured => !check.required,
                DxForgeRemoteObjectHeadHealthStatus::MissingRequiredObject
                | DxForgeRemoteObjectHeadHealthStatus::ByteMismatch => false,
            };

            DxForgeRemoteObjectHeadHealthCheck {
                intent: check.intent.clone(),
                metadata_operation: check.metadata_operation.clone(),
                object_key: check.object_key.clone(),
                required: check.required,
                status,
                safe_for_remote_install,
                expected_bytes: check.expected_bytes,
                measured_bytes: check.measured_bytes,
                reason: health_reason(
                    status,
                    check.required,
                    check.expected_bytes,
                    check.measured_bytes,
                ),
            }
        })
        .collect();

    let missing_required_count = checks
        .iter()
        .filter(|check| check.status == DxForgeRemoteObjectHeadHealthStatus::MissingRequiredObject)
        .count();
    let missing_optional_count = checks
        .iter()
        .filter(|check| check.status == DxForgeRemoteObjectHeadHealthStatus::MissingOptionalObject)
        .count();
    let byte_mismatch_count = checks
        .iter()
        .filter(|check| check.status == DxForgeRemoteObjectHeadHealthStatus::ByteMismatch)
        .count();
    let blocking_check_count = checks
        .iter()
        .filter(|check| !check.safe_for_remote_install)
        .count();
    let safe_for_remote_install = blocking_check_count == 0;

    let mut warnings = Vec::new();
    if missing_required_count > 0 {
        warnings.push(format!(
            "{missing_required_count} required remote object(s) are missing"
        ));
    }
    if missing_optional_count > 0 {
        warnings.push(format!(
            "{missing_optional_count} optional remote object(s) are missing"
        ));
    }
    if byte_mismatch_count > 0 {
        warnings.push(format!(
            "{byte_mismatch_count} remote object(s) are byte-mismatched against the package manifest"
        ));
    }
    if !receipt.network_allowed {
        warnings.push(
            "health evaluation used receipt metadata only; no live network read was performed here"
                .to_string(),
        );
    }

    let next_actions = if safe_for_remote_install {
        vec![
            "Keep remote install gated behind explicit operator approval.".to_string(),
            "Record this health receipt alongside the Forge lifecycle plan.".to_string(),
        ]
    } else {
        vec![
            "Block remote install until missing or mismatched remote objects are repaired."
                .to_string(),
            "Re-run approved read-only HEAD checks before trusting the remote package.".to_string(),
        ]
    };

    DxForgeRemoteObjectHeadHealthEvaluation {
        schema_version: "dx.forge.remote_object_head_health".to_string(),
        source_receipt_schema: receipt.schema_version.clone(),
        provider_kind: receipt.provider_kind,
        package_id: receipt.package_id.clone(),
        version: receipt.version.clone(),
        safe_for_remote_install,
        checks,
        missing_required_count,
        missing_optional_count,
        byte_mismatch_count,
        blocking_check_count,
        warnings,
        next_actions,
    }
}

fn health_reason(
    status: DxForgeRemoteObjectHeadHealthStatus,
    required: bool,
    expected_bytes: Option<u64>,
    measured_bytes: Option<u64>,
) -> Option<String> {
    match status {
        DxForgeRemoteObjectHeadHealthStatus::NotMeasured if required => {
            Some("required remote object was not measured".to_string())
        }
        DxForgeRemoteObjectHeadHealthStatus::MissingRequiredObject => {
            Some("required remote object is missing".to_string())
        }
        DxForgeRemoteObjectHeadHealthStatus::MissingOptionalObject => {
            Some("optional remote object is missing".to_string())
        }
        DxForgeRemoteObjectHeadHealthStatus::ByteMismatch => Some(format!(
            "remote object byte length mismatch: expected {}, measured {}",
            expected_bytes
                .map(|bytes| bytes.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            measured_bytes
                .map(|bytes| bytes.to_string())
                .unwrap_or_else(|| "unknown".to_string())
        )),
        _ => None,
    }
}
