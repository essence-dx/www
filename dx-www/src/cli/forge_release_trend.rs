use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use dx_compiler::ecosystem::{DxUpdateTraffic, build_forge_trust_policy_report};

#[derive(Debug, Clone)]
pub(super) struct DxForgeReleaseReadinessTrendInput {
    pub project: PathBuf,
    pub release_history_path: PathBuf,
    pub medium_route_path: PathBuf,
    pub large_route_path: PathBuf,
    pub trend_history_path: PathBuf,
    pub write_history: bool,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeReleaseReadinessTrendReport {
    pub version: u32,
    pub generated_at: String,
    pub status: String,
    pub passed: bool,
    pub score: u8,
    pub previous_score: Option<u8>,
    pub score_delta: Option<i16>,
    pub history_path: PathBuf,
    pub latest: DxForgeReleaseReadinessTrendRecord,
    pub previous: Option<DxForgeReleaseReadinessTrendRecord>,
    pub findings: Vec<String>,
    pub honest_scope: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DxForgeReleaseReadinessTrendHistory {
    pub updated_at: String,
    #[serde(default)]
    pub records: Vec<DxForgeReleaseReadinessTrendRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DxForgeReleaseReadinessTrendRecord {
    pub generated_at: String,
    pub score: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_score: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score_delta: Option<i16>,
    pub signals: DxForgeReleaseReadinessTrendSignals,
    #[serde(default)]
    pub findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DxForgeReleaseReadinessTrendSignals {
    #[serde(default)]
    pub public_bundle: DxForgeReleaseReadinessTrendSignal,
    #[serde(default)]
    pub medium_route: DxForgeReleaseReadinessTrendSignal,
    #[serde(default)]
    pub large_route: DxForgeReleaseReadinessTrendSignal,
    #[serde(default)]
    pub trust_policy: DxForgeReleaseReadinessTrendSignal,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(super) struct DxForgeReleaseReadinessTrendSignal {
    #[serde(default)]
    pub score: u8,
    #[serde(default)]
    pub passed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decoded_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brotli_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traffic: Option<String>,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub detail: String,
}

pub(super) fn build_forge_release_readiness_trend_report(
    input: DxForgeReleaseReadinessTrendInput,
) -> anyhow::Result<DxForgeReleaseReadinessTrendReport> {
    let previous_history = read_trend_history(&input.trend_history_path)?;
    let previous = previous_history.records.first().cloned();
    let mut latest = build_latest_record(&input, previous.as_ref())?;
    let findings = trend_findings(&latest);
    latest.findings = findings.clone();
    let passed = findings.is_empty() && latest.score >= 80;
    let status = if passed { "passing" } else { "needs-review" }.to_string();

    if input.write_history {
        write_trend_history(&input.trend_history_path, &latest, previous_history)?;
    }

    Ok(DxForgeReleaseReadinessTrendReport {
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        status,
        passed,
        score: latest.score,
        previous_score: latest.previous_score,
        score_delta: latest.score_delta,
        history_path: input.trend_history_path,
        latest,
        previous,
        findings,
        honest_scope: vec![
            "This trend report compares reviewed local Forge evidence artifacts only.".to_string(),
            "Medium and large route rows are deterministic fixture evidence, not full framework benchmarks.".to_string(),
            "Trust-policy advisory rows distinguish curated fixtures from live advisory feeds.".to_string(),
            "A passing trend still expects human review before public launch claims are expanded.".to_string(),
        ],
    })
}

pub(super) fn forge_release_readiness_trend_terminal(
    report: &DxForgeReleaseReadinessTrendReport,
) -> String {
    format!(
        "DX Forge release readiness trend\nStatus: {} ({} / 100)\nHistory: {}\nScore delta: {}\nSignals: public-bundle {}, medium-route {}, large-route {}, trust-policy {}\nFindings: {}\n",
        report.status,
        report.score,
        slash_path(&report.history_path),
        signed_delta(report.score_delta),
        report.latest.signals.public_bundle.score,
        report.latest.signals.medium_route.score,
        report.latest.signals.large_route.score,
        report.latest.signals.trust_policy.score,
        report.findings.len()
    )
}

pub(super) fn forge_release_readiness_trend_markdown(
    report: &DxForgeReleaseReadinessTrendReport,
) -> String {
    let mut output = format!(
        "# DX Forge Release Readiness Trend\n\n- Status: `{}`\n- Score: `{}` / `100`\n- Score delta: `{}`\n- Generated: `{}`\n- History: `{}`\n\n",
        report.status,
        report.score,
        signed_delta(report.score_delta),
        report.generated_at,
        slash_path(&report.history_path)
    );

    output.push_str("## Signals\n\n");
    output.push_str(
        "| Signal | Score | Score delta | Passed | Brotli | Brotli delta | Source | Detail |\n",
    );
    output.push_str("| --- | ---: | ---: | --- | ---: | ---: | --- | --- |\n");
    for (label, latest, previous) in trend_signal_rows(report) {
        output.push_str(&format!(
            "| `{}` | {} | {} | {} | {} | {} | `{}` | {} |\n",
            label,
            latest.score,
            signed_delta(score_delta(previous, latest)),
            latest.passed,
            optional_bytes(latest.brotli_bytes),
            signed_byte_delta(brotli_delta(previous, latest)),
            markdown_cell(&latest.source),
            markdown_cell(&latest.detail)
        ));
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- `pass`: no release-readiness trend findings.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output.push_str("\n## Honest Scope\n\n");
    for boundary in &report.honest_scope {
        output.push_str(&format!("- {boundary}\n"));
    }

    output
}

fn build_latest_record(
    input: &DxForgeReleaseReadinessTrendInput,
    previous: Option<&DxForgeReleaseReadinessTrendRecord>,
) -> anyhow::Result<DxForgeReleaseReadinessTrendRecord> {
    let release_history = read_json(&input.release_history_path, "Forge public release history")?;
    let medium = read_json(&input.medium_route_path, "Forge medium route comparison")?;
    let large = read_json(&input.large_route_path, "Forge large-content comparison")?;
    let trust_policy = build_forge_trust_policy_report(&input.project)?;

    let signals = DxForgeReleaseReadinessTrendSignals {
        public_bundle: public_bundle_signal(&release_history, &input.release_history_path)?,
        medium_route: route_fixture_signal(&medium, &input.medium_route_path, "medium-route")?,
        large_route: large_route_signal(&large, &input.large_route_path)?,
        trust_policy: DxForgeReleaseReadinessTrendSignal {
            score: trust_policy.score,
            passed: trust_policy.score >= 80 && trust_policy.traffic != DxUpdateTraffic::Red,
            traffic: Some(trust_policy.traffic.as_str().to_string()),
            source: "dx forge trust-policy".to_string(),
            detail: trust_policy_detail(&trust_policy),
            ..Default::default()
        },
    };
    let score = average_score([
        signals.public_bundle.score,
        signals.medium_route.score,
        signals.large_route.score,
        signals.trust_policy.score,
    ]);
    let previous_score = previous.map(|record| record.score);
    let score_delta = previous_score.map(|value| score as i16 - value as i16);

    Ok(DxForgeReleaseReadinessTrendRecord {
        generated_at: Utc::now().to_rfc3339(),
        score,
        previous_score,
        score_delta,
        signals,
        findings: Vec::new(),
    })
}

fn public_bundle_signal(
    value: &Value,
    path: &Path,
) -> anyhow::Result<DxForgeReleaseReadinessTrendSignal> {
    let latest = value
        .get("records")
        .and_then(Value::as_array)
        .and_then(|records| records.first())
        .ok_or_else(|| anyhow!("Forge public release history has no records"))?;
    let dashboard = latest
        .get("dashboard")
        .ok_or_else(|| anyhow!("release-history latest record has no dashboard"))?;
    let route_comparison = latest
        .get("route_comparison")
        .ok_or_else(|| anyhow!("release-history latest record has no route_comparison"))?;
    let score = u8_field(dashboard, "score")?;
    let passed = bool_field(dashboard, "passed").unwrap_or(score >= 90);

    Ok(DxForgeReleaseReadinessTrendSignal {
        score,
        passed,
        decoded_bytes: u64_field(route_comparison, "total_decoded_bytes").ok(),
        brotli_bytes: u64_field(route_comparison, "total_brotli_bytes").ok(),
        route_count: u64_field(route_comparison, "route_count").ok(),
        source: slash_path(path),
        detail: format!(
            "release-dashboard score {} with {} public route(s)",
            score,
            u64_field(route_comparison, "route_count").unwrap_or_default()
        ),
        ..Default::default()
    })
}

fn route_fixture_signal(
    value: &Value,
    path: &Path,
    label: &str,
) -> anyhow::Result<DxForgeReleaseReadinessTrendSignal> {
    let dx = dx_framework_row(value)?;
    let brotli = u64_field(dx, "total_brotli_bytes")?;
    let decoded = u64_field(dx, "total_decoded_bytes")?;
    let scoped = value
        .get("scope")
        .and_then(|scope| scope.get("not_full_framework_benchmark"))
        .and_then(Value::as_bool)
        .unwrap_or(false)
        && value
            .get("scope")
            .and_then(|scope| scope.get("no_node_modules_created"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
    let score = if scoped && brotli <= 1_600 {
        100
    } else if scoped {
        90
    } else {
        75
    };

    Ok(DxForgeReleaseReadinessTrendSignal {
        score,
        passed: score >= 80,
        decoded_bytes: Some(decoded),
        brotli_bytes: Some(brotli),
        source: slash_path(path),
        detail: format!("{label} DX-WWW static fixture"),
        ..Default::default()
    })
}

fn large_route_signal(
    value: &Value,
    path: &Path,
) -> anyhow::Result<DxForgeReleaseReadinessTrendSignal> {
    let dx = dx_framework_row(value)?;
    let brotli = u64_field(dx, "total_brotli_bytes")?;
    let decoded = u64_field(dx, "total_decoded_bytes")?;
    let budget_passed = value
        .get("first_route_budget")
        .and_then(|budget| budget.get("passed"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let score = if budget_passed { 100 } else { 70 };

    Ok(DxForgeReleaseReadinessTrendSignal {
        score,
        passed: budget_passed,
        decoded_bytes: Some(decoded),
        brotli_bytes: Some(brotli),
        source: slash_path(path),
        detail: "large-route first-route payload budget".to_string(),
        ..Default::default()
    })
}

fn trend_findings(record: &DxForgeReleaseReadinessTrendRecord) -> Vec<String> {
    let mut findings = Vec::new();
    for (label, signal) in [
        ("public-bundle", &record.signals.public_bundle),
        ("medium-route", &record.signals.medium_route),
        ("large-route", &record.signals.large_route),
        ("trust-policy", &record.signals.trust_policy),
    ] {
        if !signal.passed {
            findings.push(format!("{label} signal is not passing."));
        }
        if signal.score < 80 {
            findings.push(format!("{label} score {} is below 80.", signal.score));
        }
    }
    findings
}

fn write_trend_history(
    path: &Path,
    latest: &DxForgeReleaseReadinessTrendRecord,
    previous: DxForgeReleaseReadinessTrendHistory,
) -> anyhow::Result<()> {
    let mut records = previous.records;
    records.insert(0, latest.clone());
    records.truncate(30);
    let history = DxForgeReleaseReadinessTrendHistory {
        updated_at: latest.generated_at.clone(),
        records,
    };
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(
        path,
        format!("{}\n", serde_json::to_string_pretty(&history)?),
    )?;
    std::fs::write(
        path.with_extension("md"),
        forge_release_readiness_trend_history_markdown(&history),
    )?;
    Ok(())
}

fn forge_release_readiness_trend_history_markdown(
    history: &DxForgeReleaseReadinessTrendHistory,
) -> String {
    let mut output = format!(
        "# DX Forge Release Readiness Trend History\n\n- Updated: `{}`\n- Records: `{}`\n\n",
        history.updated_at,
        history.records.len()
    );
    output.push_str("| Generated | Score | Delta | Public bundle | Medium | Large | Trust |\n");
    output.push_str("| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    for record in &history.records {
        output.push_str(&format!(
            "| `{}` | {} | {} | {} | {} | {} | {} |\n",
            record.generated_at,
            record.score,
            signed_delta(record.score_delta),
            record.signals.public_bundle.score,
            record.signals.medium_route.score,
            record.signals.large_route.score,
            record.signals.trust_policy.score
        ));
    }
    output
}

fn read_trend_history(path: &Path) -> anyhow::Result<DxForgeReleaseReadinessTrendHistory> {
    if !path.exists() {
        return Ok(DxForgeReleaseReadinessTrendHistory {
            updated_at: Utc::now().to_rfc3339(),
            records: Vec::new(),
        });
    }
    let bytes = std::fs::read(path).with_context(|| format!("read `{}`", path.display()))?;
    serde_json::from_slice(&bytes).with_context(|| format!("parse `{}`", path.display()))
}

fn read_json(path: &Path, label: &str) -> anyhow::Result<Value> {
    let bytes =
        std::fs::read(path).with_context(|| format!("read {label} `{}`", path.display()))?;
    serde_json::from_slice(&bytes).with_context(|| format!("parse {label} `{}`", path.display()))
}

fn dx_framework_row(value: &Value) -> anyhow::Result<&Value> {
    value
        .get("frameworks")
        .and_then(Value::as_array)
        .and_then(|frameworks| {
            frameworks.iter().find(|framework| {
                framework
                    .get("framework")
                    .and_then(Value::as_str)
                    .is_some_and(|name| name == "DX-WWW")
            })
        })
        .ok_or_else(|| anyhow!("comparison report does not include a DX-WWW framework row"))
}

fn trust_policy_detail(report: &dx_compiler::ecosystem::DxForgeTrustPolicyReport) -> String {
    let coverage = report
        .packages
        .iter()
        .map(|package| package.advisory_coverage_kind.as_str())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "trust-policy traffic {}, advisory coverage {}",
        report.traffic.as_str(),
        coverage
    )
}

fn average_score(scores: [u8; 4]) -> u8 {
    (scores.iter().map(|score| *score as u16).sum::<u16>() / scores.len() as u16) as u8
}

fn trend_signal_rows(
    report: &DxForgeReleaseReadinessTrendReport,
) -> Vec<(
    &'static str,
    &DxForgeReleaseReadinessTrendSignal,
    Option<&DxForgeReleaseReadinessTrendSignal>,
)> {
    let previous = report.previous.as_ref();
    vec![
        (
            "public-bundle",
            &report.latest.signals.public_bundle,
            previous.map(|record| &record.signals.public_bundle),
        ),
        (
            "medium-route",
            &report.latest.signals.medium_route,
            previous.map(|record| &record.signals.medium_route),
        ),
        (
            "large-route",
            &report.latest.signals.large_route,
            previous.map(|record| &record.signals.large_route),
        ),
        (
            "trust-policy",
            &report.latest.signals.trust_policy,
            previous.map(|record| &record.signals.trust_policy),
        ),
    ]
}

fn score_delta(
    previous: Option<&DxForgeReleaseReadinessTrendSignal>,
    latest: &DxForgeReleaseReadinessTrendSignal,
) -> Option<i16> {
    previous.map(|previous| latest.score as i16 - previous.score as i16)
}

fn brotli_delta(
    previous: Option<&DxForgeReleaseReadinessTrendSignal>,
    latest: &DxForgeReleaseReadinessTrendSignal,
) -> Option<i64> {
    previous.and_then(|previous| Some(latest.brotli_bytes? as i64 - previous.brotli_bytes? as i64))
}

fn u8_field(value: &Value, field: &str) -> anyhow::Result<u8> {
    let raw = u64_field(value, field)?;
    u8::try_from(raw).with_context(|| format!("field `{field}` is outside u8 range"))
}

fn u64_field(value: &Value, field: &str) -> anyhow::Result<u64> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| anyhow!("missing numeric field `{field}`"))
}

fn bool_field(value: &Value, field: &str) -> Option<bool> {
    value.get(field).and_then(Value::as_bool)
}

fn signed_delta(delta: Option<i16>) -> String {
    delta
        .map(|delta| format!("{delta:+}"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn signed_byte_delta(delta: Option<i64>) -> String {
    delta
        .map(|delta| format!("{delta:+} B"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn optional_bytes(value: Option<u64>) -> String {
    value
        .map(|value| format!("{value} B"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}

fn slash_path(path: &Path) -> String {
    let raw = path.to_string_lossy();
    raw.strip_prefix(r"\\?\").unwrap_or(&raw).replace('\\', "/")
}
