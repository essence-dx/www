use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow};
use chrono::Utc;
use serde::Serialize;

use super::forge_release_history::{
    DxForgePublicReleaseHistory, DxForgePublicReleaseRecord, DxForgePublicReleaseRouteSnapshot,
};

#[derive(Debug, Clone)]
pub(super) struct DxForgeLaunchChangelogInput {
    pub history_path: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeLaunchChangelogReport {
    pub version: u32,
    pub history_path: PathBuf,
    pub generated_at: String,
    pub passed: bool,
    pub score: u8,
    pub status: String,
    pub record_count: usize,
    pub latest: Option<DxForgeLaunchChangelogEntry>,
    pub entries: Vec<DxForgeLaunchChangelogEntry>,
    pub honest_scope: Vec<String>,
    pub findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeLaunchChangelogEntry {
    pub generated_at: String,
    pub dashboard_score: u8,
    pub dashboard_passed: bool,
    pub route_count: u64,
    pub total_decoded_bytes: u64,
    pub total_brotli_bytes: u64,
    pub total_decoded_delta_bytes: Option<i64>,
    pub total_brotli_delta_bytes: Option<i64>,
    pub added_routes: Vec<String>,
    pub removed_routes: Vec<String>,
    pub changed_routes: Vec<DxForgeLaunchChangelogRouteChange>,
    pub regression_findings: Vec<String>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeLaunchChangelogRouteChange {
    pub route: String,
    pub decoded_delta_bytes: i64,
    pub brotli_delta_bytes: i64,
}

pub(super) fn build_forge_launch_changelog_report(
    input: DxForgeLaunchChangelogInput,
) -> anyhow::Result<DxForgeLaunchChangelogReport> {
    let history = read_release_history(&input.history_path)?;
    Ok(build_forge_launch_changelog_report_from_history(
        input.history_path,
        history,
    ))
}

pub(super) fn build_forge_launch_changelog_report_from_history(
    history_path: PathBuf,
    history: DxForgePublicReleaseHistory,
) -> DxForgeLaunchChangelogReport {
    let entries = history
        .records
        .iter()
        .enumerate()
        .map(|(index, record)| changelog_entry(record, history.records.get(index + 1)))
        .collect::<Vec<_>>();
    let latest = entries.first().cloned();
    let findings = changelog_findings(latest.as_ref());
    let score = changelog_score(latest.as_ref(), &findings);
    let passed = latest
        .as_ref()
        .is_some_and(|entry| entry.dashboard_passed && entry.regression_findings.is_empty())
        && score >= 90;
    let status = if passed { "passing" } else { "needs-review" }.to_string();

    DxForgeLaunchChangelogReport {
        version: 1,
        history_path,
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        status,
        record_count: history.records.len(),
        latest,
        entries,
        honest_scope: vec![
            "This changelog is generated only from reviewed Forge release-history records.".to_string(),
            "It does not claim live production traffic, customer adoption, or universal npm replacement coverage.".to_string(),
            "Route byte and timing numbers reflect the benchmark artifacts recorded in release history.".to_string(),
            "A passing changelog still expects human review of release-dashboard, route comparison, and CI artifacts.".to_string(),
        ],
        findings,
    }
}

pub(super) fn forge_launch_changelog_terminal(report: &DxForgeLaunchChangelogReport) -> String {
    let latest = report
        .latest
        .as_ref()
        .map(|entry| {
            format!(
                "Latest dashboard: {} / 100\nRoutes: {}\nBrotli total: {} B\nAdded routes: {}\nRegressions: {}",
                entry.dashboard_score,
                entry.route_count,
                entry.total_brotli_bytes,
                route_list_summary(&entry.added_routes),
                entry.regression_findings.len()
            )
        })
        .unwrap_or_else(|| "Latest dashboard: missing\nRoutes: 0\nBrotli total: 0 B\nAdded routes: none\nRegressions: 0".to_string());

    format!(
        "DX Forge public launch changelog\nHistory: {}\nStatus: {} ({} / 100)\nRecords: {}\n{}\n",
        report.history_path.display(),
        report.status,
        report.score,
        report.record_count,
        latest
    )
}

pub(super) fn forge_launch_changelog_markdown(report: &DxForgeLaunchChangelogReport) -> String {
    let mut output = format!(
        "# DX Forge Public Launch Changelog\n\n- History: `{}`\n- Generated: `{}`\n- Status: `{}`\n- Score: `{}` / `100`\n- Records: `{}`\n\n",
        report.history_path.display(),
        report.generated_at,
        report.status,
        report.score,
        report.record_count
    );

    output.push_str("## Latest Summary\n\n");
    if let Some(latest) = &report.latest {
        output.push_str(&format!(
            "- Dashboard: `{}` / `100` (passed: `{}`)\n- Public routes: `{}`\n- Total decoded bytes: `{}` B{}\n- Total Brotli estimate: `{}` B{}\n- Added public routes: {}\n- Removed public routes: {}\n\n",
            latest.dashboard_score,
            latest.dashboard_passed,
            latest.route_count,
            latest.total_decoded_bytes,
            format_optional_delta(latest.total_decoded_delta_bytes),
            latest.total_brotli_bytes,
            format_optional_delta(latest.total_brotli_delta_bytes),
            route_list_summary(&latest.added_routes),
            route_list_summary(&latest.removed_routes)
        ));
        output.push_str(&format!("{}\n\n", latest.summary));
    } else {
        output.push_str("- No release-history records were found.\n\n");
    }

    output.push_str("## Route Changes\n\n");
    if let Some(latest) = &report.latest {
        if latest.changed_routes.is_empty() {
            output.push_str("- No existing public route payload changes were recorded.\n\n");
        } else {
            output.push_str("| Route | Decoded delta | Brotli delta |\n");
            output.push_str("| --- | ---: | ---: |\n");
            for route in &latest.changed_routes {
                output.push_str(&format!(
                    "| `{}` | `{}` B | `{}` B |\n",
                    route.route,
                    format_signed(route.decoded_delta_bytes),
                    format_signed(route.brotli_delta_bytes)
                ));
            }
            output.push('\n');
        }
    } else {
        output.push_str("- No route changes are available.\n\n");
    }

    output.push_str("## Release Regression Findings\n\n");
    if let Some(latest) = &report.latest {
        if latest.regression_findings.is_empty() {
            output.push_str(
                "- No release-regression findings were recorded for the latest history entry.\n\n",
            );
        } else {
            for finding in &latest.regression_findings {
                output.push_str(&format!("- {}\n", markdown_cell(finding)));
            }
            output.push('\n');
        }
    } else {
        output.push_str("- No latest entry exists to evaluate.\n\n");
    }

    output.push_str("## Recorded Entries\n\n");
    output.push_str(
        "| Recorded | Dashboard | Routes | Decoded | Brotli | Added | Removed | Regressions |\n",
    );
    output.push_str("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    for entry in &report.entries {
        output.push_str(&format!(
            "| {} | {} | {} | {} B | {} B | {} | {} | {} |\n",
            markdown_cell(&entry.generated_at),
            entry.dashboard_score,
            entry.route_count,
            entry.total_decoded_bytes,
            entry.total_brotli_bytes,
            entry.added_routes.len(),
            entry.removed_routes.len(),
            entry.regression_findings.len()
        ));
    }
    output.push('\n');

    output.push_str("## Honest Scope\n\n");
    for scope in &report.honest_scope {
        output.push_str(&format!("- {scope}\n"));
    }

    output.push_str("\n## Changelog Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- No changelog generation findings.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output
}

fn read_release_history(path: &Path) -> anyhow::Result<DxForgePublicReleaseHistory> {
    if !path.exists() {
        return Err(anyhow!(
            "Forge public release history is missing: {}",
            path.display()
        ));
    }
    serde_json::from_slice(&std::fs::read(path)?)
        .with_context(|| format!("parse Forge public release history {}", path.display()))
}

fn changelog_entry(
    record: &DxForgePublicReleaseRecord,
    previous: Option<&DxForgePublicReleaseRecord>,
) -> DxForgeLaunchChangelogEntry {
    let latest_routes = route_map(record);
    let previous_routes = previous.map(route_map).unwrap_or_default();
    let added_routes = latest_routes
        .keys()
        .filter(|route| !previous_routes.contains_key(*route))
        .map(|route| (*route).to_string())
        .collect::<Vec<_>>();
    let removed_routes = previous_routes
        .keys()
        .filter(|route| !latest_routes.contains_key(*route))
        .map(|route| (*route).to_string())
        .collect::<Vec<_>>();
    let mut changed_routes = Vec::new();
    for (route, latest_route) in &latest_routes {
        if let Some(previous_route) = previous_routes.get(route) {
            let decoded_delta =
                byte_delta(latest_route.decoded_bytes, previous_route.decoded_bytes);
            let brotli_delta = byte_delta(latest_route.brotli_bytes, previous_route.brotli_bytes);
            if decoded_delta != 0 || brotli_delta != 0 {
                changed_routes.push(DxForgeLaunchChangelogRouteChange {
                    route: (*route).to_string(),
                    decoded_delta_bytes: decoded_delta,
                    brotli_delta_bytes: brotli_delta,
                });
            }
        }
    }

    let total_decoded_delta_bytes = previous.map(|previous| {
        byte_delta(
            record.route_comparison.total_decoded_bytes,
            previous.route_comparison.total_decoded_bytes,
        )
    });
    let total_brotli_delta_bytes = previous.map(|previous| {
        byte_delta(
            record.route_comparison.total_brotli_bytes,
            previous.route_comparison.total_brotli_bytes,
        )
    });
    let summary = entry_summary(
        previous,
        &added_routes,
        &removed_routes,
        &changed_routes,
        &record.regression_findings,
    );

    DxForgeLaunchChangelogEntry {
        generated_at: record.generated_at.clone(),
        dashboard_score: record.dashboard.score,
        dashboard_passed: record.dashboard.passed,
        route_count: record.route_comparison.route_count,
        total_decoded_bytes: record.route_comparison.total_decoded_bytes,
        total_brotli_bytes: record.route_comparison.total_brotli_bytes,
        total_decoded_delta_bytes,
        total_brotli_delta_bytes,
        added_routes,
        removed_routes,
        changed_routes,
        regression_findings: record.regression_findings.clone(),
        summary,
    }
}

fn route_map(
    record: &DxForgePublicReleaseRecord,
) -> BTreeMap<&str, &DxForgePublicReleaseRouteSnapshot> {
    record
        .route_comparison
        .routes
        .iter()
        .map(|route| (route.route.as_str(), route))
        .collect()
}

fn entry_summary(
    previous: Option<&DxForgePublicReleaseRecord>,
    added_routes: &[String],
    removed_routes: &[String],
    changed_routes: &[DxForgeLaunchChangelogRouteChange],
    regression_findings: &[String],
) -> String {
    if previous.is_none() {
        return "Initial recorded Forge public launch evidence snapshot.".to_string();
    }
    if !regression_findings.is_empty() {
        return format!(
            "Needs review: {} release-regression finding(s) are attached to this history entry.",
            regression_findings.len()
        );
    }
    if added_routes.is_empty() && removed_routes.is_empty() && changed_routes.is_empty() {
        return "No public route payload changes were recorded against the previous release-history entry.".to_string();
    }
    let mut parts = Vec::new();
    if !added_routes.is_empty() {
        parts.push(format!("added {}", route_list_summary(added_routes)));
    }
    if !removed_routes.is_empty() {
        parts.push(format!("removed {}", route_list_summary(removed_routes)));
    }
    if !changed_routes.is_empty() {
        parts.push(format!(
            "changed {} existing route payload(s)",
            changed_routes.len()
        ));
    }
    format!("Reviewed launch evidence {}.", parts.join(", "))
}

fn changelog_findings(latest: Option<&DxForgeLaunchChangelogEntry>) -> Vec<String> {
    let Some(latest) = latest else {
        return vec!["Release history does not contain any records.".to_string()];
    };
    let mut findings = Vec::new();
    if !latest.dashboard_passed {
        findings.push(format!(
            "Latest release-dashboard did not pass at {} / 100.",
            latest.dashboard_score
        ));
    }
    findings.extend(latest.regression_findings.iter().cloned());
    findings
}

fn changelog_score(latest: Option<&DxForgeLaunchChangelogEntry>, findings: &[String]) -> u8 {
    let Some(latest) = latest else {
        return 0;
    };
    let mut score = 100i16;
    if !latest.dashboard_passed {
        score -= 30;
    }
    score -= (findings.len() as i16).saturating_mul(10).min(40);
    if latest.route_count == 0 {
        score -= 20;
    }
    score.clamp(0, 100) as u8
}

fn byte_delta(latest: u64, previous: u64) -> i64 {
    if latest >= previous {
        latest.saturating_sub(previous).min(i64::MAX as u64) as i64
    } else {
        -(previous.saturating_sub(latest).min(i64::MAX as u64) as i64)
    }
}

fn route_list_summary(routes: &[String]) -> String {
    if routes.is_empty() {
        "none".to_string()
    } else {
        routes
            .iter()
            .map(|route| format!("`{}`", route))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn format_optional_delta(value: Option<i64>) -> String {
    value
        .map(|delta| format!(" (`{}` B)", format_signed(delta)))
        .unwrap_or_default()
}

fn format_signed(value: i64) -> String {
    if value > 0 {
        format!("+{value}")
    } else {
        value.to_string()
    }
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}
