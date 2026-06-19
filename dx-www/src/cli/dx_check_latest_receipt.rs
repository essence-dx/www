use std::path::Path;

use chrono::Utc;
use dx_compiler::ecosystem::{
    DX_CHECK_LATEST_RECEIPT_PATH, DX_CHECK_WEIGHT_PROFILE, DX_CHECK_ZED_PANEL_SCHEMA_VERSION,
    DxCheckFinding, DxCheckReport, DxCheckSection, DxSupplyChainSeverity, DxUpdateTraffic,
};
use serde_json::{Value, json};

use crate::error::{DxError, DxResult};

use super::readiness;

pub(super) fn write_dx_check_latest_receipt(
    project: &Path,
    report: &DxCheckReport,
) -> DxResult<()> {
    let receipt_path = project.join(DX_CHECK_LATEST_RECEIPT_PATH);
    if let Some(parent) = receipt_path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| DxError::ConfigValidationError {
            message: format!("Failed to create dx-check receipt directory: {error}"),
            field: Some("check".to_string()),
        })?;
    }

    let receipt = dx_check_latest_receipt(project, report);
    std::fs::write(
        &receipt_path,
        serde_json::to_string_pretty(&receipt).map_err(|error| DxError::ConfigValidationError {
            message: format!("Failed to render dx-check latest receipt: {error}"),
            field: Some("check".to_string()),
        })?,
    )
    .map_err(|error| DxError::ConfigValidationError {
        message: format!("Failed to write dx-check latest receipt: {error}"),
        field: Some("check".to_string()),
    })?;

    Ok(())
}

fn dx_check_latest_receipt(project: &Path, report: &DxCheckReport) -> Value {
    let generated_at_unix_ms = Utc::now().timestamp_millis().max(0) as u128;
    let score_value = u16::from(report.score).saturating_mul(5);
    let readiness_gate_status = readiness::readiness_gate_status_for_project(Some(project));
    let readiness_score = readiness_gate_status
        .get("current_honest_score")
        .and_then(Value::as_i64)
        .unwrap_or_default();
    let readiness_score_kind = readiness_gate_status
        .get("score_kind")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let readiness_replay_commands = readiness::readiness_replay_commands();
    let missing_proof_gates = readiness_gate_status
        .get("missing_proof_gates")
        .cloned()
        .unwrap_or_else(|| json!([]));
    let remaining_proof_gates = readiness_gate_status
        .get("remaining_proof_gates")
        .cloned()
        .unwrap_or_else(|| json!([]));
    let missing_proof_gate_count = missing_proof_gates
        .as_array()
        .map(|items| items.len())
        .unwrap_or_default();
    let readiness_evidence_freshness =
        readiness_evidence_freshness(&readiness_gate_status, missing_proof_gate_count);
    let mut blocker_findings = report
        .sections
        .iter()
        .flat_map(|section| section.findings.iter())
        .filter(|finding| {
            matches!(
                finding.severity,
                DxSupplyChainSeverity::High | DxSupplyChainSeverity::Critical
            )
        })
        .map(zed_finding)
        .collect::<Vec<_>>();
    if !readiness_release_gate_ready(&readiness_gate_status) {
        blocker_findings.push(readiness_zed_blocker());
    }
    let warning_findings = report
        .sections
        .iter()
        .flat_map(|section| section.findings.iter())
        .filter(|finding| {
            !matches!(
                finding.severity,
                DxSupplyChainSeverity::High | DxSupplyChainSeverity::Critical
            )
        })
        .map(zed_finding)
        .collect::<Vec<_>>();
    let quick_fixes = quick_fixes(report);

    json!({
        "schema_version": "dx.check.latest.v1",
        "generated_at_unix_ms": generated_at_unix_ms,
        "command": "dx check --json",
        "weight_profile": DX_CHECK_WEIGHT_PROFILE,
        "project_health_score": score_value,
        "project_health_score_max": 500,
        "project_health_score_percent": report.score,
        "project_health_score_estimated": true,
        "dx_check_score": score_value,
        "dx_check_score_max": 500,
        "dx_check_score_percent": report.score,
        "dx_check_score_estimated": true,
        "readiness_score": readiness_score,
        "readiness_score_max": 100,
        "readiness_score_kind": readiness_score_kind,
        "readiness_score_estimated": false,
        "readiness_evidence_freshness": readiness_evidence_freshness,
        "score": score_value,
        "max_score": 500,
        "score_percent": report.score,
        "score_estimated": true,
        "traffic": report.traffic.as_str(),
        "release_ready": readiness_release_gate_ready(&readiness_gate_status),
        "relative_release_ready": readiness_release_gate_ready(&readiness_gate_status),
        "release_ready_scope": readiness_gate_status.get("release_ready_scope").cloned().unwrap_or(Value::Null),
        "fastest_world_claim": false,
        "release_claim_allowed": readiness_gate_status
            .get("release_claim_allowed")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        "global_speed_claim_allowed": false,
        "missing_proof_gates": missing_proof_gates,
        "remaining_proof_gates": remaining_proof_gates,
        "readiness_gate_status": readiness_gate_status,
        "readiness_replay_commands": readiness_replay_commands.clone(),
        "replay_commands": readiness_replay_commands,
        "sections": report.sections.iter().map(receipt_section).collect::<Vec<_>>(),
        "zed": {
            "panel_kind": "project-health",
            "schema_version": DX_CHECK_ZED_PANEL_SCHEMA_VERSION,
            "source": "dx-www",
            "weight_profile": DX_CHECK_WEIGHT_PROFILE,
            "score_value": score_value,
            "score_max": 500,
            "score_percent": report.score,
            "score_estimated": true,
            "status": zed_status(report.traffic, blocker_findings.len()),
            "generated_at_unix_ms": generated_at_unix_ms,
            "bucket_count": report.sections.len(),
            "blocker_count": blocker_findings.len(),
            "warning_count": warning_findings.len(),
            "quick_fix_count": quick_fixes.len(),
            "receipt_path": DX_CHECK_LATEST_RECEIPT_PATH,
            "refresh_command": "dx check --json",
            "detail_command": "dx check --latest-receipt --json",
            "blockers": blocker_findings,
            "warnings": warning_findings,
            "quick_fixes": quick_fixes,
            "sections": report.sections.iter().map(zed_section).collect::<Vec<_>>()
        }
    })
}

fn readiness_evidence_freshness(gate_status: &Value, missing_proof_gate_count: usize) -> Value {
    let visual_edit_stale_reason = gate_status
        .pointer("/local_replay_receipts/visual_edit_stale_reason")
        .cloned()
        .unwrap_or_else(|| {
            json!({
                "code": "visual-edit-stale-reason-not-reported",
                "message": "The readiness gate status did not report visual-edit stale evidence."
            })
        });

    json!({
        "kind": "release-readiness-evidence-not-dx-check-health",
        "score_source": "readiness_gate_status",
        "receipt_freshness": gate_status
            .get("receipt_freshness")
            .cloned()
            .unwrap_or_else(|| json!("unknown")),
        "verified_from_replay_receipts": gate_status
            .get("verified_from_replay_receipts")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        "missing_proof_gate_count": missing_proof_gate_count,
        "visual_edit_stale_reason": visual_edit_stale_reason,
        "release_ready": readiness_release_gate_ready(gate_status),
        "fastest_world_claim": false,
    })
}

fn readiness_release_gate_ready(gate_status: &Value) -> bool {
    gate_status.get("release_ready").and_then(Value::as_bool) == Some(true)
        && gate_status
            .get("verified_from_replay_receipts")
            .and_then(Value::as_bool)
            == Some(true)
        && gate_status.get("receipt_freshness").and_then(Value::as_str) == Some("current")
}

fn readiness_zed_blocker() -> Value {
    json!({
        "severity": "blocked",
        "code": "www-release-readiness-gate-blocked",
        "message": "WWW release-readiness gates are not replay-verified yet; release-ready and global speed-leadership claims stay disabled.",
        "evidence_path": "readiness_gate_status",
        "next_action": "Run dx www readiness --json --full and complete blocking proof gates before release claims."
    })
}

fn receipt_section(section: &DxCheckSection) -> Value {
    json!({
        "name": section.name,
        "score": section.score,
        "traffic": section.traffic.as_str(),
        "metrics": section.metrics.iter().map(|metric| {
            json!({
                "name": metric.name,
                "value": metric.value
            })
        }).collect::<Vec<_>>()
    })
}

fn zed_section(section: &DxCheckSection) -> Value {
    json!({
        "id": section.name,
        "title": section_title(&section.name),
        "weight": 100,
        "score": section.score,
        "max_score": 100,
        "estimated": true,
        "status": zed_status(section.traffic, 0),
        "summary": format!(
            "{} score is {}/100 with {} finding(s).",
            section_title(&section.name),
            section.score,
            section.findings.len()
        )
    })
}

fn zed_finding(finding: &DxCheckFinding) -> Value {
    json!({
        "severity": finding_severity(finding.severity),
        "code": finding.code,
        "message": finding.message,
        "evidence_path": finding.evidence_path,
        "next_action": finding.remediation
    })
}

fn quick_fixes(report: &DxCheckReport) -> Vec<Value> {
    let mut fixes = vec![json!({
        "id": "refresh-dx-check",
        "label": "Refresh dx-check",
        "next_action": "Run the current fast project check and rewrite the latest receipt.",
        "risk_level": "safe",
        "writes_receipts": true,
        "command": "dx check --json"
    })];

    if report.traffic != DxUpdateTraffic::Green {
        fixes.push(json!({
            "id": "inspect-dx-check-details",
            "label": "Inspect findings",
            "next_action": "Open the detailed dx-check receipt and fix the listed package or source warnings.",
            "risk_level": "safe",
            "writes_receipts": false,
            "command": "dx check --latest-receipt --json"
        }));
    }

    fixes
}

fn section_title(name: &str) -> String {
    name.split(['-', '_'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn zed_status(traffic: DxUpdateTraffic, blocker_count: usize) -> &'static str {
    if blocker_count > 0 {
        return "blocked";
    }
    match traffic {
        DxUpdateTraffic::Green => "ready",
        DxUpdateTraffic::Yellow => "warning",
        DxUpdateTraffic::Red => "blocked",
    }
}

fn finding_severity(severity: DxSupplyChainSeverity) -> &'static str {
    match severity {
        DxSupplyChainSeverity::Info => "info",
        DxSupplyChainSeverity::Low => "warning",
        DxSupplyChainSeverity::Medium => "warning",
        DxSupplyChainSeverity::High => "blocked",
        DxSupplyChainSeverity::Critical => "blocked",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dx_compiler::ecosystem::{DxCheckMetric, DxUpdateTraffic};

    #[test]
    fn latest_receipt_scales_dx_check_score_to_500_point_panel() {
        let report = DxCheckReport {
            path: "fixture".into(),
            score: 98,
            traffic: DxUpdateTraffic::Yellow,
            sections: vec![DxCheckSection {
                name: "forge".to_string(),
                score: 90,
                traffic: DxUpdateTraffic::Yellow,
                metrics: vec![DxCheckMetric {
                    name: "packages".to_string(),
                    value: 21,
                }],
                findings: vec![],
            }],
        };

        let receipt = dx_check_latest_receipt(Path::new("."), &report);

        assert_eq!(receipt["score"], 490);
        assert_eq!(receipt["max_score"], 500);
        assert_eq!(receipt["project_health_score"], 490);
        assert_eq!(receipt["project_health_score_max"], 500);
        assert_eq!(receipt["dx_check_score"], 490);
        assert_eq!(receipt["dx_check_score_max"], 500);
        assert_eq!(receipt["readiness_score"], 99);
        assert_eq!(receipt["readiness_score_max"], 100);
        assert_eq!(
            receipt["readiness_score_kind"],
            "relative-local-proof-backed-release-ready"
        );
        assert_eq!(receipt["readiness_score_estimated"], false);
        assert_eq!(
            receipt["readiness_evidence_freshness"]["kind"],
            "release-readiness-evidence-not-dx-check-health"
        );
        assert_eq!(
            receipt["readiness_evidence_freshness"]["score_source"],
            "readiness_gate_status"
        );
        assert!(
            receipt["readiness_evidence_freshness"]["missing_proof_gate_count"]
                .as_u64()
                .unwrap_or_default()
                == 0
        );
        assert_eq!(receipt["zed"]["score_value"], 490);
        assert_eq!(receipt["zed"]["score_max"], 500);
        assert_eq!(receipt["zed"]["score_percent"], 98);
        assert_eq!(receipt["zed"]["status"], "green");
        assert_eq!(receipt["zed"]["blocker_count"], 0);
        assert_eq!(receipt["zed"]["sections"][0]["id"], "forge");
        assert_eq!(receipt["release_ready"], true);
        assert_eq!(receipt["relative_release_ready"], true);
        assert_eq!(receipt["release_claim_allowed"], true);
        assert_eq!(receipt["global_speed_claim_allowed"], false);
        assert_eq!(receipt["readiness_gate_status"]["release_ready"], true);
        assert_eq!(
            receipt["readiness_gate_status"]["fastest_world_claim"],
            false
        );
        assert!(
            receipt["readiness_replay_commands"]
                .as_array()
                .expect("readiness replay commands")
                .iter()
                .any(|command| command == "dx www readiness --json --full")
        );
    }
}
