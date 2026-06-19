use std::path::Path;

use chrono::Utc;
use dx_compiler::ecosystem::build_forge_package_scorecard_for_project;

use super::forge_doctor::build_forge_doctor_report;
use super::{
    DxForgeBenchmarkSnapshot, DxForgeReleaseEvidenceReport, check_section_metric,
    forge_benchmark_snapshot_is_release_ready, forge_package_scorecard_release_ready,
    load_latest_benchmark_snapshot, load_latest_forge_route_benchmark_snapshot,
};

pub(super) fn build_forge_release_evidence_report(
    project: &Path,
    benchmark_history_path: &Path,
) -> anyhow::Result<DxForgeReleaseEvidenceReport> {
    let doctor = build_forge_doctor_report(project)?;
    let forge_section = doctor
        .check
        .sections
        .iter()
        .find(|section| section.name == "forge");
    let latest_benchmark = load_latest_benchmark_snapshot(benchmark_history_path)?;
    let latest_forge_route_benchmark =
        load_latest_forge_route_benchmark_snapshot(benchmark_history_path)?;
    let release_gate_score =
        super::forge_release_gate_score(&doctor.check, &doctor.launch_gate_findings);
    let benchmark_ok = latest_forge_route_benchmark
        .as_ref()
        .or(latest_benchmark.as_ref())
        .is_some_and(forge_benchmark_snapshot_is_release_ready);
    let package_scorecard = build_forge_package_scorecard_for_project(project)?;
    let scorecard_ok = forge_package_scorecard_release_ready(&package_scorecard);
    let passed = doctor.passed && benchmark_ok && scorecard_ok;

    Ok(DxForgeReleaseEvidenceReport {
        project: project.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        check_score: doctor.check.score,
        check_traffic: doctor.check.traffic.as_str().to_string(),
        release_gate_score,
        launch_gate_findings: doctor.launch_gate_findings,
        registry_integrity: doctor.registry_integrity,
        rollback_coverage_percent: forge_section
            .map(|section| check_section_metric(section, "rollback_coverage_percent"))
            .unwrap_or(0),
        rollback_missing_packages: forge_section
            .map(|section| check_section_metric(section, "rollback_missing_packages"))
            .unwrap_or(0),
        package_docs_coverage_percent: forge_section
            .map(|section| check_section_metric(section, "package_docs_coverage_percent"))
            .unwrap_or(0),
        package_docs_missing: forge_section
            .map(|section| check_section_metric(section, "package_docs_missing"))
            .unwrap_or(0),
        package_scorecard,
        benchmark_history_path: benchmark_history_path.to_path_buf(),
        latest_benchmark,
        latest_forge_route_benchmark,
    })
}

pub(super) fn forge_release_evidence_markdown(report: &DxForgeReleaseEvidenceReport) -> String {
    let scorecard_verified = report
        .package_scorecard
        .packages
        .iter()
        .filter(|package| package.integrity_verified)
        .count();
    let scorecard_source_owned = report
        .package_scorecard
        .packages
        .iter()
        .filter(|package| package.source_owned)
        .count();
    let scorecard_node_modules = report
        .package_scorecard
        .packages
        .iter()
        .filter(|package| package.node_modules_created)
        .count();
    let mut output = format!(
        "# DX Forge Release Proof\n\n- Project: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Release check: `{}` / `100`\n- DX check: `{}` / `{}`\n- Forge release-check findings: `{}`\n- Registry packages verified: `{}` / `{}`\n- Scorecard score: `{}`\n- Scorecard packages verified: `{}` / `{}`\n- Scorecard source-owned packages: `{}` / `{}`\n- Scorecard node_modules packages: `{}`\n- Rollback coverage: `{}%`\n- Packages missing rollback receipts: `{}`\n- Package docs coverage: `{}%`\n- Package docs missing: `{}`\n- Benchmark history: `{}`\n\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.release_gate_score,
        report.check_score,
        report.check_traffic,
        report.launch_gate_findings.len(),
        report
            .registry_integrity
            .iter()
            .filter(|check| check.verified)
            .count(),
        report.registry_integrity.len(),
        report.package_scorecard.score,
        scorecard_verified,
        report.package_scorecard.packages.len(),
        scorecard_source_owned,
        report.package_scorecard.packages.len(),
        scorecard_node_modules,
        report.rollback_coverage_percent,
        report.rollback_missing_packages,
        report.package_docs_coverage_percent,
        report.package_docs_missing,
        report.benchmark_history_path.display()
    );

    output.push_str("## Registry Integrity\n\n");
    for check in &report.registry_integrity {
        output.push_str(&format!(
            "- `{}`: `{}` ({}/{})",
            check.package_id, check.verified, check.verified_files, check.file_count
        ));
        if let Some(error) = &check.error {
            output.push_str(&format!(" - {error}"));
        }
        output.push('\n');
    }

    output.push_str("\n## Package Scorecard\n\n");
    output.push_str(&format!(
        "- Score: `{}`\n- Packages: `{}`\n- Verified: `{}`\n- Source-owned: `{}`\n- Creates node_modules: `{}`\n",
        report.package_scorecard.score,
        report.package_scorecard.packages.len(),
        scorecard_verified,
        scorecard_source_owned,
        scorecard_node_modules
    ));
    if let Some(project) = &report.package_scorecard.project {
        output.push_str(&format!(
            "- Project manifest: `{}` packages, `{}` receipts\n- Project Forge score: `{}` / `{}`\n- Project docs coverage: `{}%`\n- Project rollback coverage: `{}%`\n",
            project.manifest_package_count,
            project.manifest_receipt_count,
            project.forge_score,
            project.forge_traffic.as_str(),
            project.package_docs_coverage_percent,
            project.rollback_coverage_percent
        ));
    }
    output.push('\n');
    for package in &report.package_scorecard.packages {
        let local = package
            .project_evidence
            .as_ref()
            .map(|evidence| {
                format!(
                    "variants={}, files={}, docs={}/{}, rollback={}/{}",
                    evidence.manifest_variant_count,
                    evidence.manifest_file_count,
                    evidence.docs_present,
                    evidence.docs_present + evidence.docs_missing,
                    evidence.rollback_receipts_present,
                    evidence.rollback_receipts_present + evidence.rollback_receipts_missing
                )
            })
            .unwrap_or_else(|| "no local evidence".to_string());
        output.push_str(&format!(
            "- `{}`: verified=`{}`, source_owned=`{}`, scripts_blocked=`{}`, node_modules=`{}`, local=`{}`\n",
            package.package_id,
            package.integrity_verified,
            package.source_owned,
            package.install_scripts_blocked,
            package.node_modules_created,
            local
        ));
    }

    output.push_str("\n## Latest Vertical Benchmark\n\n");
    if let Some(snapshot) = &report.latest_benchmark {
        output.push_str(&forge_benchmark_snapshot_markdown(snapshot));
    } else {
        output.push_str("- No benchmark history index was found.\n");
    }

    output.push_str("\n## Latest /forge Payload And Browser Timing\n\n");
    if let Some(snapshot) = &report.latest_forge_route_benchmark {
        output.push_str(&forge_benchmark_snapshot_markdown(snapshot));
    } else {
        output.push_str("- No `/forge` benchmark snapshot was found.\n");
    }

    if !report.launch_gate_findings.is_empty() {
        output.push_str("\n## Forge Release-Check Findings\n\n");
        for finding in &report.launch_gate_findings {
            output.push_str(&format!(
                "- `{:?}` `{}`: {}\n",
                finding.severity, finding.code, finding.message
            ));
        }
    }

    output
}

pub(super) fn forge_benchmark_snapshot_markdown(snapshot: &DxForgeBenchmarkSnapshot) -> String {
    let mut output = format!(
        "- Generated: `{}`\n- Mode: `{}`\n- Delivery: `{}`\n- Packages: `{}`\n- Files: `{}`\n- Decoded bytes: `{}`\n- Brotli bytes: `{}`\n- HTTP median: `{}` ms\n- Chrome load: `{}` ms\n- DXPK applied: `{}`\n- Interaction works: `{}`\n",
        snapshot.generated_at.as_deref().unwrap_or("-"),
        snapshot.fixture_mode.as_deref().unwrap_or("-"),
        snapshot.route_delivery.as_deref().unwrap_or("-"),
        snapshot.forge_packages.unwrap_or_default(),
        snapshot.forge_files_tracked.unwrap_or_default(),
        snapshot.decoded_bytes.unwrap_or_default(),
        snapshot.brotli_bytes.unwrap_or_default(),
        snapshot.http_route_median_ms.unwrap_or_default(),
        snapshot.chrome_load_event_ms.unwrap_or_default(),
        snapshot.dx_packet_applied.unwrap_or(false),
        snapshot.interaction_works.unwrap_or(false)
    );
    if let Some(markdown) = &snapshot.markdown {
        output.push_str(&format!("- Snapshot: `{markdown}`\n"));
    }
    output
}
