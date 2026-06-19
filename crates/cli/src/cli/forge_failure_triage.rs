use super::{
    DxForgeReadinessBadge, DxForgeSmokeReport, forge_launch_page_quality_checks,
    markdown_table_cell,
};

pub(super) fn forge_failure_triage_markdown(
    report: &DxForgeSmokeReport,
    badge: &DxForgeReadinessBadge,
) -> String {
    let mut output = format!(
        "# DX Forge Failure Triage\n\n- Project: `{}`\n- Generated: `{}`\n- Smoke passed: `{}`\n- Readiness status: `{}`\n- Readiness score: `{}` / `100`\n- Fail-under: `{}`\n- No `node_modules`: `{}`\n\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        badge.status,
        badge.score,
        badge.fail_under,
        report.no_node_modules
    );

    output.push_str("## Summary\n\n");
    if report.passed && badge.passed {
        output.push_str(
            "- `pass`: Forge CI artifacts are release-ready for the configured threshold.\n",
        );
    } else {
        output.push_str("- `review`: one or more Forge CI gates need attention before release.\n");
    }
    if !badge.findings.is_empty() {
        for finding in &badge.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output.push_str("\n## Smoke Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- `pass`: no smoke findings.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- `fail`: {finding}\n"));
        }
    }

    output.push_str("\n## Launch Page Quality\n\n");
    output.push_str("| Check | Passed | Evidence |\n");
    output.push_str("| --- | --- | --- |\n");
    for (label, check) in forge_launch_page_quality_checks(&report.launch_page_quality) {
        output.push_str(&format!(
            "| `{label}` | `{}` | `{}` |\n",
            check.passed,
            markdown_table_cell(&check.evidence)
        ));
    }
    if !report.launch_page_quality.findings.is_empty() {
        output.push_str("\nLaunch-page findings:\n");
        for finding in &report.launch_page_quality.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output.push_str("\n## Launch Gate Findings\n\n");
    if report.launch_gate_findings.is_empty() {
        output.push_str("- `pass`: no strict Forge launch-gate findings.\n");
    } else {
        for finding in &report.launch_gate_findings {
            output.push_str(&format!(
                "- `{:?}` `{}`: {} Evidence: `{}` Remediation: {}\n",
                finding.severity,
                finding.code,
                finding.message,
                finding.evidence_path.as_deref().unwrap_or("-"),
                finding.remediation
            ));
        }
    }

    output.push_str("\n## Package And Evidence Gates\n\n");
    output.push_str("| Gate | Passed | Score | Summary |\n");
    output.push_str("| --- | --- | ---: | --- |\n");
    for (name, check) in [
        ("smoke", &badge.smoke),
        ("release proof", &badge.evidence),
        ("package scorecard", &badge.scorecard),
        ("launch page quality", &badge.launch_page_quality),
    ] {
        output.push_str(&format!(
            "| `{name}` | `{}` | {} | `{}` |\n",
            check.passed,
            check.score,
            markdown_table_cell(&check.summary)
        ));
    }

    output.push_str("\n## Budget Gate\n\n");
    if let Some(benchmark) = &badge.latest_forge_route_benchmark {
        output.push_str("| Metric | Value |\n");
        output.push_str("| --- | ---: |\n");
        output.push_str(&format!("| Passed | `{}` |\n", benchmark.passed));
        output.push_str(&format!(
            "| Fixture | `{}` |\n",
            benchmark.fixture_mode.as_deref().unwrap_or("-")
        ));
        output.push_str(&format!(
            "| Delivery | `{}` |\n",
            benchmark.route_delivery.as_deref().unwrap_or("-")
        ));
        output.push_str(&format!(
            "| Decoded bytes | `{}` |\n",
            benchmark.decoded_bytes.unwrap_or_default()
        ));
        output.push_str(&format!(
            "| Brotli bytes | `{}` |\n",
            benchmark.brotli_bytes.unwrap_or_default()
        ));
        output.push_str(&format!(
            "| HTTP median | `{}` ms |\n",
            benchmark.http_route_median_ms.unwrap_or_default()
        ));
        output.push_str(&format!(
            "| Chrome load | `{}` ms |\n",
            benchmark.chrome_load_event_ms.unwrap_or_default()
        ));
    } else {
        output.push_str("- `review`: no latest `/forge` benchmark snapshot was found.\n");
    }
    output.push_str(
        "\nBudget-specific failures from `DX_VERTICAL_BUDGET_GATE=1` are written to `benchmarks/reports/vertical-proof-triage.md` by the launch-budget benchmark lane.\n",
    );

    output.push_str("\n## First Actions\n\n");
    let actions = forge_failure_triage_actions(report, badge);
    if actions.is_empty() {
        output.push_str(
            "- No immediate triage actions. Keep this artifact as the clean CI baseline.\n",
        );
    } else {
        for action in actions {
            output.push_str(&format!("- {action}\n"));
        }
    }

    output
}

fn forge_failure_triage_actions(
    report: &DxForgeSmokeReport,
    badge: &DxForgeReadinessBadge,
) -> Vec<&'static str> {
    let mut actions = Vec::new();
    if !report.no_node_modules {
        actions.push("Remove unexpected `node_modules` from the Forge smoke path before release.");
    }
    if !report.findings.is_empty() {
        actions.push(
            "Resolve smoke findings first because they describe the highest-level release blocker.",
        );
    }
    if !report.launch_page_quality.passed {
        actions.push("Fix launch-page quality findings in headings, SEO/static shape, links, or claims-manifest evidence.");
    }
    if !report.launch_gate_findings.is_empty() {
        actions.push("Resolve strict Forge launch-gate findings before changing score thresholds.");
    }
    if !report.doctor_passed {
        actions.push("Run `dx forge doctor` locally and fix strict check, registry, rollback, or docs coverage failures.");
    }
    if !report.verify_passed {
        actions.push("Run `dx forge verify-package --all` and repair package integrity, docs, update, or rollback evidence.");
    }
    if badge.score < badge.fail_under {
        actions.push("Raise the readiness score or lower the fail-under threshold only after a reviewed policy decision.");
    }
    if badge
        .latest_forge_route_benchmark
        .as_ref()
        .is_none_or(|benchmark| !benchmark.passed)
    {
        actions.push("Run the launch-budget lane and inspect `benchmarks/reports/vertical-proof-triage.md` for byte or timing failures.");
    }
    actions
}
