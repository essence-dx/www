use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::error::{DxError, DxResult};

use super::forge_error;
use super::options::{DxOutputFormat, resolve_cli_path};

#[derive(Debug, Clone)]
pub(super) struct DxForgePublicReleaseHistoryInput {
    pub root: PathBuf,
    pub dashboard_path: PathBuf,
    pub route_comparison_path: PathBuf,
    pub history_path: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgePublicReleaseHistoryReport {
    pub history_path: PathBuf,
    pub markdown_path: PathBuf,
    pub records: usize,
    pub latest_dashboard_score: Option<u8>,
    pub latest_route_count: Option<u64>,
    pub latest_total_brotli_bytes: Option<u64>,
    pub history: DxForgePublicReleaseHistory,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(default)]
struct DxForgeReleaseHistoryRegressionPolicy {
    expected_route_additions: u64,
    max_total_decoded_growth_bytes: u64,
    max_total_brotli_growth_bytes: u64,
    max_route_decoded_growth_bytes: u64,
    max_route_brotli_growth_bytes: u64,
    max_added_route_decoded_bytes: Option<u64>,
    max_added_route_brotli_bytes: Option<u64>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct DxForgeReleaseHistoryConfigFile {
    forge: DxForgeReleaseHistoryForgeConfig,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct DxForgeReleaseHistoryForgeConfig {
    release_history: DxForgeReleaseHistoryRegressionPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DxForgePublicReleaseHistory {
    pub updated_at: String,
    #[serde(default)]
    pub records: Vec<DxForgePublicReleaseRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DxForgePublicReleaseRecord {
    pub generated_at: String,
    pub source_dashboard: String,
    pub source_route_comparison: String,
    pub dashboard: DxForgePublicReleaseDashboardSnapshot,
    pub route_comparison: DxForgePublicReleaseRouteComparisonSnapshot,
    #[serde(default)]
    pub regression_findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DxForgePublicReleaseDashboardSnapshot {
    pub generated_at: String,
    pub score: u8,
    pub fail_under: u8,
    pub passed: bool,
    pub no_node_modules: Option<bool>,
    pub public_evidence_links: Option<u64>,
    #[serde(default)]
    pub findings: Vec<String>,
    #[serde(default)]
    pub checks: BTreeMap<String, DxForgePublicReleaseDashboardCheckSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DxForgePublicReleaseDashboardCheckSnapshot {
    pub passed: bool,
    pub score: u8,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DxForgePublicReleaseRouteComparisonSnapshot {
    pub generated_at: String,
    pub route_count: u64,
    pub total_decoded_bytes: u64,
    pub total_brotli_bytes: u64,
    pub lowest_brotli_route: String,
    #[serde(default)]
    pub routes: Vec<DxForgePublicReleaseRouteSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DxForgePublicReleaseRouteSnapshot {
    pub route: String,
    pub fixture_mode: String,
    pub delivery: String,
    pub decoded_bytes: u64,
    pub brotli_bytes: u64,
    pub http_route_median_ms: f64,
    pub chrome_load_event_ms: f64,
    pub budget_passed: Option<bool>,
}

pub(super) fn run_forge_release_history(cwd: &Path, args: &[String]) -> DxResult<()> {
    let mut dashboard: Option<PathBuf> = None;
    let mut route_comparison: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--dashboard" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--dashboard requires a JSON file".to_string(),
                        field: Some("forge release-history".to_string()),
                    })?;
                dashboard = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--route-comparison" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--route-comparison requires a JSON file".to_string(),
                        field: Some("forge release-history".to_string()),
                    })?;
                route_comparison = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--output requires a history JSON path".to_string(),
                        field: Some("forge release-history".to_string()),
                    })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--format requires terminal, json, or markdown".to_string(),
                        field: Some("forge release-history".to_string()),
                    })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(DxError::ConfigValidationError {
                    message: format!("Unknown forge release-history option: {value}"),
                    field: Some("forge release-history".to_string()),
                });
            }
            value => {
                if output.is_some() {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unexpected forge release-history path: {value}"),
                        field: Some("forge release-history".to_string()),
                    });
                }
                output = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    let dashboard = dashboard.unwrap_or_else(|| cwd.join(".dx/ci/forge-release-dashboard.json"));
    let route_comparison = route_comparison
        .unwrap_or_else(|| cwd.join("benchmarks/reports/forge-public-route-comparison.json"));
    let output =
        output.unwrap_or_else(|| cwd.join("benchmarks/reports/forge-public-release-history.json"));

    let report = record_forge_public_release_history(DxForgePublicReleaseHistoryInput {
        root: cwd.to_path_buf(),
        dashboard_path: dashboard,
        route_comparison_path: route_comparison,
        history_path: output,
    })
    .map_err(forge_error)?;

    let rendered = match format {
        DxOutputFormat::Terminal => forge_public_release_history_terminal(&report),
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => forge_public_release_history_markdown(&report.history),
    };

    if !quiet {
        println!("{rendered}");
    }

    Ok(())
}

pub(super) fn record_forge_public_release_history(
    input: DxForgePublicReleaseHistoryInput,
) -> anyhow::Result<DxForgePublicReleaseHistoryReport> {
    let dashboard = read_json(&input.dashboard_path, "Forge release-dashboard JSON")?;
    let route_comparison = read_json(
        &input.route_comparison_path,
        "Forge public route comparison JSON",
    )?;
    let markdown_path = input.history_path.with_extension("md");
    let previous = read_existing_history(&input.history_path)?;
    let mut next_record = build_forge_public_release_record(
        &input.root,
        &input.dashboard_path,
        &input.route_comparison_path,
        &dashboard,
        &route_comparison,
    )?;
    let previous_latest = previous
        .records
        .iter()
        .find(|record| !same_evidence(record, &next_record));
    let regression_policy = load_release_history_regression_policy(&input.root)?;
    next_record.regression_findings =
        release_regression_findings(previous_latest, &next_record, regression_policy);
    let mut records = previous
        .records
        .into_iter()
        .filter(|record| !same_evidence(record, &next_record))
        .collect::<Vec<_>>();
    records.push(next_record);
    records.sort_by(|left, right| right.generated_at.cmp(&left.generated_at));
    records.truncate(30);

    let updated_at = records
        .first()
        .map(|record| record.generated_at.clone())
        .unwrap_or_else(|| Utc::now().to_rfc3339());
    let history = DxForgePublicReleaseHistory {
        updated_at,
        records,
    };

    if let Some(parent) = input.history_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(
        &input.history_path,
        format!("{}\n", serde_json::to_string_pretty(&history)?),
    )?;
    std::fs::write(
        &markdown_path,
        forge_public_release_history_markdown(&history),
    )?;

    Ok(DxForgePublicReleaseHistoryReport {
        history_path: input.history_path,
        markdown_path,
        records: history.records.len(),
        latest_dashboard_score: history.records.first().map(|record| record.dashboard.score),
        latest_route_count: history
            .records
            .first()
            .map(|record| record.route_comparison.route_count),
        latest_total_brotli_bytes: history
            .records
            .first()
            .map(|record| record.route_comparison.total_brotli_bytes),
        history,
    })
}

pub(super) fn forge_public_release_history_terminal(
    report: &DxForgePublicReleaseHistoryReport,
) -> String {
    let Some(latest) = report.history.records.first() else {
        return format!(
            "DX Forge public release history\nHistory: {}\nRecords: 0\n",
            report.history_path.display()
        );
    };

    format!(
        "DX Forge public release history\nHistory: {}\nMarkdown: {}\nRecords: {}\nLatest dashboard: {} / 100 (passed: {})\nRoutes: {} static release routes, {} Brotli bytes total\nFindings: {}\nRegressions: {}\n",
        report.history_path.display(),
        report.markdown_path.display(),
        report.records,
        latest.dashboard.score,
        latest.dashboard.passed,
        latest.route_comparison.route_count,
        latest.route_comparison.total_brotli_bytes,
        latest.dashboard.findings.len(),
        latest.regression_findings.len()
    )
}

pub(super) fn forge_public_release_history_markdown(
    history: &DxForgePublicReleaseHistory,
) -> String {
    let mut output = format!(
        "# Forge Public Release History\n\nUpdated: {}\n\n",
        history.updated_at
    );

    if let Some(latest) = history.records.first() {
        output.push_str("## Latest\n\n");
        output.push_str(&format!(
            "- Dashboard score: {} / 100\n- Dashboard passed: {}\n- Required score: {} / 100\n- Public routes: {}\n- Total decoded bytes: {} B\n- Total Brotli estimate: {} B\n- Smallest Brotli route: `{}`\n\n",
            latest.dashboard.score,
            latest.dashboard.passed,
            latest.dashboard.fail_under,
            latest.route_comparison.route_count,
            latest.route_comparison.total_decoded_bytes,
            latest.route_comparison.total_brotli_bytes,
            latest.route_comparison.lowest_brotli_route
        ));

        output.push_str("| Route | Fixture | Delivery | Decoded | Brotli | HTTP median | Chrome load | Budget |\n");
        output.push_str("| --- | --- | --- | ---: | ---: | ---: | ---: | --- |\n");
        for route in &latest.route_comparison.routes {
            output.push_str(&format!(
                "| {} | {} | {} | {} B | {} B | {} ms | {} ms | {} |\n",
                markdown_table_cell(&route.route),
                markdown_table_cell(&route.fixture_mode),
                markdown_table_cell(&route.delivery),
                route.decoded_bytes,
                route.brotli_bytes,
                format_number(route.http_route_median_ms),
                format_number(route.chrome_load_event_ms),
                format_bool(route.budget_passed)
            ));
        }
        output.push('\n');

        output.push_str("## Latest Regression Checks\n\n");
        if latest.regression_findings.is_empty() {
            output.push_str(
                "- No release regressions detected against the previous distinct record.\n\n",
            );
        } else {
            for finding in &latest.regression_findings {
                output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
            }
            output.push('\n');
        }
    }

    output.push_str("## Records\n\n");
    output.push_str(
        "| Recorded | Dashboard | Routes | Decoded | Brotli | Findings | Regressions |\n",
    );
    output.push_str("| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    for record in &history.records {
        output.push_str(&format!(
            "| {} | {} | {} | {} B | {} B | {} | {} |\n",
            markdown_table_cell(&record.generated_at),
            record.dashboard.score,
            record.route_comparison.route_count,
            record.route_comparison.total_decoded_bytes,
            record.route_comparison.total_brotli_bytes,
            record.dashboard.findings.len(),
            record.regression_findings.len()
        ));
    }
    output.push('\n');

    output
}

pub(super) fn build_forge_public_release_record(
    root: &Path,
    dashboard_path: &Path,
    route_comparison_path: &Path,
    dashboard: &serde_json::Value,
    route_comparison: &serde_json::Value,
) -> anyhow::Result<DxForgePublicReleaseRecord> {
    Ok(DxForgePublicReleaseRecord {
        generated_at: Utc::now().to_rfc3339(),
        source_dashboard: relative_path(root, dashboard_path),
        source_route_comparison: relative_path(root, route_comparison_path),
        dashboard: dashboard_snapshot(dashboard)?,
        route_comparison: route_comparison_snapshot(route_comparison)?,
        regression_findings: Vec::new(),
    })
}

fn dashboard_snapshot(
    value: &serde_json::Value,
) -> anyhow::Result<DxForgePublicReleaseDashboardSnapshot> {
    let mut checks = BTreeMap::new();
    if let Some(checks_value) = value.get("checks").and_then(|checks| checks.as_object()) {
        for (name, check) in checks_value {
            checks.insert(
                name.clone(),
                DxForgePublicReleaseDashboardCheckSnapshot {
                    passed: required_bool(check, "passed")?,
                    score: required_u8(check, "score")?,
                    message: required_str(check, "message")?.to_string(),
                },
            );
        }
    }

    Ok(DxForgePublicReleaseDashboardSnapshot {
        generated_at: required_str(value, "generated_at")?.to_string(),
        score: required_u8(value, "score")?,
        fail_under: required_u8(value, "fail_under")?,
        passed: required_bool(value, "passed")?,
        no_node_modules: value
            .get("release_notes")
            .and_then(|notes| notes.get("no_node_modules"))
            .and_then(|no_node_modules| no_node_modules.as_bool()),
        public_evidence_links: value
            .get("public_evidence")
            .and_then(|evidence| evidence.get("links"))
            .and_then(|links| links.as_u64()),
        findings: string_array(value.get("findings")),
        checks,
    })
}

fn route_comparison_snapshot(
    value: &serde_json::Value,
) -> anyhow::Result<DxForgePublicReleaseRouteComparisonSnapshot> {
    let routes = value
        .get("routes")
        .and_then(|routes| routes.as_array())
        .ok_or_else(|| anyhow!("Forge public route comparison JSON must include routes"))?
        .iter()
        .map(route_snapshot)
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(DxForgePublicReleaseRouteComparisonSnapshot {
        generated_at: required_str(value, "generated_at")?.to_string(),
        route_count: value
            .get("route_count")
            .and_then(|count| count.as_u64())
            .unwrap_or(routes.len() as u64),
        total_decoded_bytes: required_u64(value, "total_decoded_bytes")?,
        total_brotli_bytes: required_u64(value, "total_brotli_bytes")?,
        lowest_brotli_route: required_str(value, "lowest_brotli_route")?.to_string(),
        routes,
    })
}

fn route_snapshot(value: &serde_json::Value) -> anyhow::Result<DxForgePublicReleaseRouteSnapshot> {
    Ok(DxForgePublicReleaseRouteSnapshot {
        route: required_str(value, "route")?.to_string(),
        fixture_mode: required_str(value, "fixture_mode")?.to_string(),
        delivery: required_str(value, "route_delivery")?.to_string(),
        decoded_bytes: required_u64(value, "decoded_bytes")?,
        brotli_bytes: required_u64(value, "brotli_bytes")?,
        http_route_median_ms: required_f64(value, "http_route_median_ms")?,
        chrome_load_event_ms: required_f64(value, "chrome_load_event_ms")?,
        budget_passed: value
            .get("budget_passed")
            .and_then(|budget| budget.as_bool()),
    })
}

fn read_existing_history(path: &Path) -> anyhow::Result<DxForgePublicReleaseHistory> {
    if !path.exists() {
        return Ok(DxForgePublicReleaseHistory {
            updated_at: String::new(),
            records: Vec::new(),
        });
    }

    let history = serde_json::from_slice::<DxForgePublicReleaseHistory>(&std::fs::read(path)?)
        .with_context(|| format!("parse Forge public release history {}", path.display()))?;
    Ok(history)
}

fn read_json(path: &Path, label: &str) -> anyhow::Result<serde_json::Value> {
    if !path.exists() {
        return Err(anyhow!("{label} is missing: {}", path.display()));
    }
    serde_json::from_slice(&std::fs::read(path)?)
        .with_context(|| format!("parse {label} {}", path.display()))
}

fn same_evidence(left: &DxForgePublicReleaseRecord, right: &DxForgePublicReleaseRecord) -> bool {
    left.dashboard.generated_at == right.dashboard.generated_at
        && left.route_comparison.generated_at == right.route_comparison.generated_at
}

fn load_release_history_regression_policy(
    root: &Path,
) -> anyhow::Result<DxForgeReleaseHistoryRegressionPolicy> {
    let dx_path = root.join("dx");
    if dx_path.exists() {
        let content = std::fs::read_to_string(&dx_path)
            .with_context(|| format!("read {}", dx_path.display()))?;
        return Ok(parse_release_history_policy_from_dx(&content));
    }

    let config_path = root.join("dx.config.toml");
    if !config_path.exists() {
        return Ok(DxForgeReleaseHistoryRegressionPolicy::default());
    }

    let content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("read {}", config_path.display()))?;
    let config =
        toml::from_str::<DxForgeReleaseHistoryConfigFile>(&content).with_context(|| {
            format!(
                "parse Forge release-history policy {}",
                config_path.display()
            )
        })?;
    Ok(config.forge.release_history)
}

fn parse_release_history_policy_from_dx(content: &str) -> DxForgeReleaseHistoryRegressionPolicy {
    let mut policy = DxForgeReleaseHistoryRegressionPolicy::default();
    for line in content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        let Some(rest) = line.strip_prefix("forge.release_history.") else {
            continue;
        };
        let Some((key, value)) = rest.split_once('=') else {
            continue;
        };
        let value = value.trim().trim_matches('"');
        match key.trim() {
            "expected_route_additions" => {
                policy.expected_route_additions =
                    value.parse().unwrap_or(policy.expected_route_additions)
            }
            "max_total_decoded_growth_bytes" => {
                policy.max_total_decoded_growth_bytes = value
                    .parse()
                    .unwrap_or(policy.max_total_decoded_growth_bytes)
            }
            "max_total_brotli_growth_bytes" => {
                policy.max_total_brotli_growth_bytes = value
                    .parse()
                    .unwrap_or(policy.max_total_brotli_growth_bytes)
            }
            "max_route_decoded_growth_bytes" => {
                policy.max_route_decoded_growth_bytes = value
                    .parse()
                    .unwrap_or(policy.max_route_decoded_growth_bytes)
            }
            "max_route_brotli_growth_bytes" => {
                policy.max_route_brotli_growth_bytes = value
                    .parse()
                    .unwrap_or(policy.max_route_brotli_growth_bytes)
            }
            "max_added_route_decoded_bytes" => {
                policy.max_added_route_decoded_bytes = value.parse().ok()
            }
            "max_added_route_brotli_bytes" => {
                policy.max_added_route_brotli_bytes = value.parse().ok()
            }
            _ => {}
        }
    }
    policy
}

fn release_regression_findings(
    previous: Option<&DxForgePublicReleaseRecord>,
    latest: &DxForgePublicReleaseRecord,
    policy: DxForgeReleaseHistoryRegressionPolicy,
) -> Vec<String> {
    let mut findings = Vec::new();

    if !latest.dashboard.passed {
        findings.push(format!(
            "Release dashboard failed at {} / 100, required {} / 100.",
            latest.dashboard.score, latest.dashboard.fail_under
        ));
    }
    for route in &latest.route_comparison.routes {
        if route.budget_passed == Some(false) {
            findings.push(format!(
                "Route `{}` failed its configured public delivery budget.",
                route.route
            ));
        }
    }

    let Some(previous) = previous else {
        return findings;
    };

    if latest.dashboard.score < previous.dashboard.score {
        findings.push(format!(
            "Dashboard score dropped from {} to {}.",
            previous.dashboard.score, latest.dashboard.score
        ));
    }

    let previous_routes = previous
        .route_comparison
        .routes
        .iter()
        .map(|route| (route.route.as_str(), route))
        .collect::<BTreeMap<_, _>>();
    let latest_routes = latest
        .route_comparison
        .routes
        .iter()
        .map(|route| (route.route.as_str(), route))
        .collect::<BTreeMap<_, _>>();
    let added_routes = latest
        .route_comparison
        .routes
        .iter()
        .filter(|route| !previous_routes.contains_key(route.route.as_str()))
        .collect::<Vec<_>>();
    let added_route_count = added_routes.len() as u64;
    let expected_added_routes = added_route_count <= policy.expected_route_additions;
    if added_route_count > policy.expected_route_additions {
        findings.push(format!(
            "Public route count grew from {} to {}, expected at most {} new routes.",
            previous.route_comparison.route_count,
            latest.route_comparison.route_count,
            policy.expected_route_additions
        ));
    }

    for added_route in &added_routes {
        if let Some(limit) = policy.max_added_route_decoded_bytes {
            if added_route.decoded_bytes > limit {
                findings.push(format!(
                    "Added public route `{}` decoded payload is {} B, above configured {} B limit.",
                    added_route.route, added_route.decoded_bytes, limit
                ));
            }
        }
        if let Some(limit) = policy.max_added_route_brotli_bytes {
            if added_route.brotli_bytes > limit {
                findings.push(format!(
                    "Added public route `{}` Brotli payload is {} B, above configured {} B limit.",
                    added_route.route, added_route.brotli_bytes, limit
                ));
            }
        }
    }

    let expected_added_decoded_bytes = if expected_added_routes {
        added_routes
            .iter()
            .map(|route| route.decoded_bytes)
            .sum::<u64>()
    } else {
        0
    };
    let expected_added_brotli_bytes = if expected_added_routes {
        added_routes
            .iter()
            .map(|route| route.brotli_bytes)
            .sum::<u64>()
    } else {
        0
    };
    let comparable_decoded_growth = latest
        .route_comparison
        .total_decoded_bytes
        .saturating_sub(previous.route_comparison.total_decoded_bytes)
        .saturating_sub(expected_added_decoded_bytes);
    if comparable_decoded_growth > policy.max_total_decoded_growth_bytes {
        let mut message = format!(
            "Total decoded public payload grew from {} B to {} B.",
            previous.route_comparison.total_decoded_bytes,
            latest.route_comparison.total_decoded_bytes
        );
        if expected_added_decoded_bytes > 0 || policy.max_total_decoded_growth_bytes > 0 {
            message.push_str(&format!(
                " Comparable growth after expected route additions was {} B; configured allowance is {} B.",
                comparable_decoded_growth, policy.max_total_decoded_growth_bytes
            ));
        }
        findings.push(message);
    }
    let comparable_brotli_growth = latest
        .route_comparison
        .total_brotli_bytes
        .saturating_sub(previous.route_comparison.total_brotli_bytes)
        .saturating_sub(expected_added_brotli_bytes);
    if comparable_brotli_growth > policy.max_total_brotli_growth_bytes {
        let mut message = format!(
            "Total Brotli public payload grew from {} B to {} B.",
            previous.route_comparison.total_brotli_bytes,
            latest.route_comparison.total_brotli_bytes
        );
        if expected_added_brotli_bytes > 0 || policy.max_total_brotli_growth_bytes > 0 {
            message.push_str(&format!(
                " Comparable growth after expected route additions was {} B; configured allowance is {} B.",
                comparable_brotli_growth, policy.max_total_brotli_growth_bytes
            ));
        }
        findings.push(message);
    }

    for previous_route in &previous.route_comparison.routes {
        let Some(latest_route) = latest_routes.get(previous_route.route.as_str()) else {
            findings.push(format!(
                "Public route `{}` disappeared from release history.",
                previous_route.route
            ));
            continue;
        };
        let decoded_growth = latest_route
            .decoded_bytes
            .saturating_sub(previous_route.decoded_bytes);
        if decoded_growth > policy.max_route_decoded_growth_bytes {
            let mut message = format!(
                "Route `{}` decoded payload grew from {} B to {} B.",
                latest_route.route, previous_route.decoded_bytes, latest_route.decoded_bytes
            );
            if policy.max_route_decoded_growth_bytes > 0 {
                message.push_str(&format!(
                    " Configured route allowance is {} B.",
                    policy.max_route_decoded_growth_bytes
                ));
            }
            findings.push(message);
        }
        let brotli_growth = latest_route
            .brotli_bytes
            .saturating_sub(previous_route.brotli_bytes);
        if brotli_growth > policy.max_route_brotli_growth_bytes {
            let mut message = format!(
                "Route `{}` Brotli payload grew from {} B to {} B.",
                latest_route.route, previous_route.brotli_bytes, latest_route.brotli_bytes
            );
            if policy.max_route_brotli_growth_bytes > 0 {
                message.push_str(&format!(
                    " Configured route allowance is {} B.",
                    policy.max_route_brotli_growth_bytes
                ));
            }
            findings.push(message);
        }
    }

    findings
}

fn required_str<'a>(value: &'a serde_json::Value, field: &str) -> anyhow::Result<&'a str> {
    value
        .get(field)
        .and_then(|field| field.as_str())
        .ok_or_else(|| anyhow!("missing string field `{field}`"))
}

fn required_bool(value: &serde_json::Value, field: &str) -> anyhow::Result<bool> {
    value
        .get(field)
        .and_then(|field| field.as_bool())
        .ok_or_else(|| anyhow!("missing bool field `{field}`"))
}

fn required_u8(value: &serde_json::Value, field: &str) -> anyhow::Result<u8> {
    let value = required_u64(value, field)?;
    u8::try_from(value).map_err(|_| anyhow!("field `{field}` is too large for a score: {value}"))
}

fn required_u64(value: &serde_json::Value, field: &str) -> anyhow::Result<u64> {
    value
        .get(field)
        .and_then(|field| field.as_u64())
        .ok_or_else(|| anyhow!("missing numeric field `{field}`"))
}

fn required_f64(value: &serde_json::Value, field: &str) -> anyhow::Result<f64> {
    value
        .get(field)
        .and_then(|field| field.as_f64())
        .ok_or_else(|| anyhow!("missing numeric field `{field}`"))
}

fn string_array(value: Option<&serde_json::Value>) -> Vec<String> {
    value
        .and_then(|value| value.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
        .replace('\\', "/")
}

fn markdown_table_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}

fn format_bool(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "yes",
        Some(false) => "no",
        None => "n/a",
    }
}

fn format_number(value: f64) -> String {
    let mut formatted = format!("{value:.3}");
    while formatted.contains('.') && formatted.ends_with('0') {
        formatted.pop();
    }
    if formatted.ends_with('.') {
        formatted.pop();
    }
    formatted
}
