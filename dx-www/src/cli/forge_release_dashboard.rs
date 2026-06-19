use super::formatting::markdown_table_cell;
use super::{DxForgeReleaseDashboardCheck, DxForgeReleaseDashboardReport};

pub(super) fn forge_release_dashboard_markdown(report: &DxForgeReleaseDashboardReport) -> String {
    let mut output = format!(
        "# DX Forge Release Dashboard\n\n- Project: `{}`\n- Generated: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Passed: `{}`\n\n",
        report.project.display(),
        report.generated_at,
        report.score,
        report.fail_under,
        report.passed
    );

    output.push_str("## Checks\n\n");
    output.push_str("| Check | Passed | Score | Summary |\n");
    output.push_str("| --- | --- | ---: | --- |\n");
    output.push_str(&forge_release_dashboard_check_row(
        "CI artifacts",
        &report.checks.ci_artifacts,
    ));
    output.push_str(&forge_release_dashboard_check_row(
        "Pages bundle",
        &report.checks.pages_bundle,
    ));
    output.push_str(&forge_release_dashboard_check_row(
        "Release notes",
        &report.checks.release_notes,
    ));
    output.push_str(&forge_release_dashboard_check_row(
        "Launch changelog",
        &report.checks.launch_changelog,
    ));
    output.push_str(&forge_release_dashboard_check_row(
        "Public evidence",
        &report.checks.public_evidence,
    ));
    output.push_str(&forge_release_dashboard_check_row(
        "Route comparison",
        &report.checks.route_comparison,
    ));

    output.push_str("\n## Artifact Inputs\n\n");
    output.push_str(&format!(
        "- CI artifacts: `{}` (`{}` files, `{}` route checks)\n- Pages bundle: `{}` (`{}` files, `{}` checks)\n- Route comparison: `{}` (`{}` routes, `{}` Brotli bytes)\n",
        report.ci_artifacts.artifact_dir.display(),
        report.ci_artifacts.artifact_count,
        report.ci_artifacts.route_count,
        report.pages_bundle.bundle_dir.display(),
        report.pages_bundle.artifact_count,
        report.pages_bundle.check_count,
        report.route_comparison.path.display(),
        report.route_comparison.route_count,
        report.route_comparison.total_brotli_bytes
    ));

    output.push_str("\n## Release Proof\n\n");
    output.push_str(&format!(
        "- Release notes: `{}` (`{}` / `100`)\n- No `node_modules`: `{}`\n- Public evidence links: `{}`\n- Public evidence packages: `{}`\n\n",
        report.release_notes.status,
        report.release_notes.score,
        report.release_notes.no_node_modules,
        report.public_evidence.links,
        report.public_evidence.package_count
    ));
    output.push_str(&format!(
        "- Launch changelog: `{}` (`{}` / `100`, `{}` record, `{}` honest-scope guardrails, `{}` routes)\n\n",
        report.launch_changelog.status,
        report.launch_changelog.score,
        report.launch_changelog.record_count,
        report.launch_changelog.honest_scope_count,
        report.launch_changelog.latest_route_count
    ));

    output.push_str("## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- No release-dashboard findings for the configured threshold.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output
}

fn forge_release_dashboard_check_row(label: &str, check: &DxForgeReleaseDashboardCheck) -> String {
    format!(
        "| {} | `{}` | `{}` | {} |\n",
        markdown_table_cell(label),
        check.passed,
        check.score,
        markdown_table_cell(&check.message)
    )
}

pub(super) fn forge_release_dashboard_failure_summary(
    report: &DxForgeReleaseDashboardReport,
) -> String {
    if report.findings.is_empty() {
        return format!(
            "DX Forge release-dashboard did not pass: score {} / 100, required {} / 100",
            report.score, report.fail_under
        );
    }

    format!(
        "DX Forge release-dashboard did not pass: {}",
        report.findings.join("; ")
    )
}
