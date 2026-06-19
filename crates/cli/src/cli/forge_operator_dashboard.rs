use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::Serialize;

use super::{
    forge_publish_plan_json_bool, forge_publish_plan_json_string, forge_publish_plan_json_value,
    json_u8, json_u64, markdown_table_cell, read_forge_publish_plan_json,
};

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeOperatorDashboardReport {
    version: u32,
    generated_at: String,
    pub(super) passed: bool,
    status: String,
    pub(super) score: u8,
    fail_under: u8,
    inputs: DxForgeOperatorDashboardInputs,
    checks: DxForgeOperatorDashboardChecks,
    summary: DxForgeOperatorDashboardSummary,
    review_cards: Vec<DxForgeOperatorDashboardCard>,
    first_actions: Vec<String>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeOperatorDashboardInputs {
    release_triage: PathBuf,
    beta_artifact_verify: PathBuf,
    ci_snippets: PathBuf,
    installability: PathBuf,
    installability_history: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeOperatorDashboardChecks {
    release_triage: DxForgeOperatorDashboardCheck,
    beta_artifacts: DxForgeOperatorDashboardCheck,
    ci_snippet_provenance: DxForgeOperatorDashboardCheck,
    installability: DxForgeOperatorDashboardCheck,
    installability_history: DxForgeOperatorDashboardCheck,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeOperatorDashboardCheck {
    passed: bool,
    score: u8,
    message: String,
    evidence: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeOperatorDashboardSummary {
    release_status: Option<String>,
    beta_status: Option<String>,
    ci_status: Option<String>,
    installability_trend: String,
    pages_targets: u64,
    r2_targets: u64,
    ci_snippets: u64,
    install_time_ms: Option<u64>,
    upgrade_time_ms: Option<u64>,
    install_artifact_bytes: Option<u64>,
    upgrade_artifact_bytes: Option<u64>,
    signed_ci_provenance: bool,
    requires_rebuild: bool,
    requires_secrets: bool,
    no_package_installs: bool,
    no_node_modules: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeOperatorDashboardCard {
    key: String,
    title: String,
    passed: bool,
    score: u8,
    message: String,
    evidence: Option<String>,
}

pub(super) fn build_forge_operator_dashboard_report(
    release_triage_path: &Path,
    beta_artifact_verify_path: &Path,
    ci_snippets_path: &Path,
    installability_path: &Path,
    installability_history_path: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeOperatorDashboardReport> {
    let release_triage = read_forge_publish_plan_json(release_triage_path, "release triage")?;
    let beta_artifact_verify =
        read_forge_publish_plan_json(beta_artifact_verify_path, "beta artifact verification")?;
    let ci_snippets = read_forge_publish_plan_json(ci_snippets_path, "CI snippets")?;
    let installability = read_forge_publish_plan_json(installability_path, "installability")?;
    let installability_history =
        read_forge_publish_plan_json(installability_history_path, "installability history")?;

    let pages_targets = count_targets(&beta_artifact_verify, "pages");
    let r2_targets = count_targets(&beta_artifact_verify, "r2");
    let ci_snippet_count = json_u64(ci_snippets.get("snippet_count")).unwrap_or(0);
    let install_row = row_by_id(&installability, "dx-forge-beta-install");
    let upgrade_row = row_by_id(&installability, "dx-forge-beta-upgrade");
    let install_time_ms = install_row.and_then(|row| json_u64(row.get("time_ms")));
    let upgrade_time_ms = upgrade_row.and_then(|row| json_u64(row.get("time_ms")));
    let install_artifact_bytes = install_row.and_then(|row| json_u64(row.get("artifact_bytes")));
    let upgrade_artifact_bytes = upgrade_row.and_then(|row| json_u64(row.get("artifact_bytes")));
    let signed_ci_provenance =
        forge_publish_plan_json_bool(&ci_snippets, &["provenance", "signed"])
            && forge_publish_plan_json_bool(&ci_snippets, &["provenance", "signature_verified"]);
    let requires_rebuild =
        forge_publish_plan_json_bool(&beta_artifact_verify, &["requires_rebuild"]);
    let requires_secrets = forge_publish_plan_json_bool(
        &beta_artifact_verify,
        &["secret_requirements", "requires_secrets"],
    );
    let no_package_installs =
        forge_publish_plan_json_bool(&installability, &["scope", "no_package_installs_run"]);
    let installability_no_node_modules =
        forge_publish_plan_json_bool(&installability, &["scope", "no_node_modules_created"]);
    let beta_no_node_modules =
        forge_publish_plan_json_bool(&beta_artifact_verify, &["no_node_modules", "passed"]);
    let history_no_node_modules = forge_publish_plan_json_bool(
        &installability_history,
        &["checks", "no_package_installs", "passed"],
    );
    let installability_trend =
        forge_publish_plan_json_string(&installability_history, &["latest", "trend"])
            .unwrap_or("unknown")
            .to_string();

    let checks = DxForgeOperatorDashboardChecks {
        release_triage: operator_check(
            forge_publish_plan_json_bool(&release_triage, &["passed"])
                && forge_publish_plan_json_bool(&release_triage, &["shipping_ready"]),
            json_u8(release_triage.get("score")).unwrap_or(0),
            format!(
                "release triage status `{}` with {} first action(s).",
                forge_publish_plan_json_string(&release_triage, &["status"]).unwrap_or("unknown"),
                release_triage
                    .get("first_actions")
                    .and_then(|value| value.as_array())
                    .map(Vec::len)
                    .unwrap_or(0)
            ),
            Some(release_triage_path.display().to_string()),
        ),
        beta_artifacts: operator_check(
            forge_publish_plan_json_bool(&beta_artifact_verify, &["passed"])
                && !requires_rebuild
                && !requires_secrets
                && beta_no_node_modules
                && pages_targets > 0
                && r2_targets > 0,
            json_u8(beta_artifact_verify.get("score")).unwrap_or(0),
            format!(
                "beta artifacts status `{}` with {} Pages target(s), {} R2 target(s), requires_rebuild={}, requires_secrets={}.",
                forge_publish_plan_json_string(&beta_artifact_verify, &["status"])
                    .unwrap_or("unknown"),
                pages_targets,
                r2_targets,
                requires_rebuild,
                requires_secrets
            ),
            Some(beta_artifact_verify_path.display().to_string()),
        ),
        ci_snippet_provenance: operator_check(
            forge_publish_plan_json_bool(&ci_snippets, &["passed"])
                && ci_snippet_count >= 3
                && signed_ci_provenance,
            json_u8(ci_snippets.get("score")).unwrap_or(0),
            format!(
                "{} CI snippet(s), signed provenance={}.",
                ci_snippet_count, signed_ci_provenance
            ),
            Some(ci_snippets_path.display().to_string()),
        ),
        installability: operator_check(
            forge_publish_plan_json_bool(&installability, &["passed"])
                && no_package_installs
                && installability_no_node_modules
                && install_time_ms.is_some()
                && upgrade_time_ms.is_some(),
            json_u8(installability.get("score")).unwrap_or(0),
            format!(
                "install={} ms, upgrade={} ms, package_installs_run={}.",
                format_optional_u64(install_time_ms),
                format_optional_u64(upgrade_time_ms),
                !no_package_installs
            ),
            Some(installability_path.display().to_string()),
        ),
        installability_history: operator_check(
            json_u64(installability_history.get("snapshot_count")).unwrap_or(0) > 0
                && forge_publish_plan_json_bool(
                    &installability_history,
                    &["checks", "latest_passed", "passed"],
                )
                && history_no_node_modules,
            json_u8(forge_publish_plan_json_value(
                &installability_history,
                &["checks", "latest_passed", "score"],
            ))
            .unwrap_or(100),
            format!(
                "{} installability snapshot(s), latest trend `{}`.",
                json_u64(installability_history.get("snapshot_count")).unwrap_or(0),
                installability_trend
            ),
            Some(installability_history_path.display().to_string()),
        ),
    };

    let summary = DxForgeOperatorDashboardSummary {
        release_status: forge_publish_plan_json_string(&release_triage, &["status"])
            .map(str::to_string),
        beta_status: forge_publish_plan_json_string(&beta_artifact_verify, &["status"])
            .map(str::to_string),
        ci_status: forge_publish_plan_json_string(&ci_snippets, &["status"]).map(str::to_string),
        installability_trend,
        pages_targets,
        r2_targets,
        ci_snippets: ci_snippet_count,
        install_time_ms,
        upgrade_time_ms,
        install_artifact_bytes,
        upgrade_artifact_bytes,
        signed_ci_provenance,
        requires_rebuild,
        requires_secrets,
        no_package_installs,
        no_node_modules: installability_no_node_modules
            && beta_no_node_modules
            && history_no_node_modules,
    };

    let review_cards = vec![
        dashboard_card("release_triage", "Release Triage", &checks.release_triage),
        dashboard_card("beta_artifacts", "Beta Artifacts", &checks.beta_artifacts),
        dashboard_card(
            "ci_snippet_provenance",
            "CI Snippet Provenance",
            &checks.ci_snippet_provenance,
        ),
        dashboard_card("installability", "Installability", &checks.installability),
        dashboard_card(
            "installability_history",
            "Installability History",
            &checks.installability_history,
        ),
    ];
    let score = review_cards
        .iter()
        .map(|card| card.score)
        .min()
        .unwrap_or(0);
    let mut findings = dashboard_findings(&release_triage, "release-triage");
    findings.extend(dashboard_findings(
        &beta_artifact_verify,
        "beta-artifact-verify",
    ));
    findings.extend(dashboard_findings(&ci_snippets, "ci-snippets"));
    findings.extend(dashboard_findings(&installability, "installability"));
    for card in &review_cards {
        if !card.passed {
            findings.push(format!("{}: {}", card.key, card.message));
        }
    }
    findings.sort();
    findings.dedup();

    let passed =
        score >= fail_under && review_cards.iter().all(|card| card.passed) && findings.is_empty();
    let status = if passed {
        "ready-for-beta-operator-review"
    } else {
        "operator-action-required"
    }
    .to_string();
    let first_actions = operator_first_actions(&release_triage, &review_cards);

    Ok(DxForgeOperatorDashboardReport {
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        passed,
        status,
        score,
        fail_under,
        inputs: DxForgeOperatorDashboardInputs {
            release_triage: release_triage_path.to_path_buf(),
            beta_artifact_verify: beta_artifact_verify_path.to_path_buf(),
            ci_snippets: ci_snippets_path.to_path_buf(),
            installability: installability_path.to_path_buf(),
            installability_history: installability_history_path.to_path_buf(),
        },
        checks,
        summary,
        review_cards,
        first_actions,
        findings,
        next_commands: vec![
            format!(
                "dx forge operator-dashboard --release-triage {} --beta-artifact-verify {} --ci-snippets {} --installability {} --installability-history {} --format markdown --fail-under {}",
                release_triage_path.display(),
                beta_artifact_verify_path.display(),
                ci_snippets_path.display(),
                installability_path.display(),
                installability_history_path.display(),
                fail_under
            ),
            "dx forge release-triage --release-operations <json> --publish-plan <json> --format markdown".to_string(),
            "dx forge beta-artifact-verify --release-bundle <bundle> --pages <pages> --registry-smoke <json> --format markdown".to_string(),
        ],
    })
}

pub(super) fn forge_operator_dashboard_terminal(report: &DxForgeOperatorDashboardReport) -> String {
    let mut output = format!(
        "DX Forge operator dashboard\nStatus: {} ({} / 100)\nPassed: {}\nPages targets: {}\nR2 targets: {}\nCI snippets: {}\nInstall: {} ms\nUpgrade: {} ms\nInstallability trend: {}\n",
        report.status,
        report.score,
        report.passed,
        report.summary.pages_targets,
        report.summary.r2_targets,
        report.summary.ci_snippets,
        format_optional_u64(report.summary.install_time_ms),
        format_optional_u64(report.summary.upgrade_time_ms),
        report.summary.installability_trend
    );
    if !report.first_actions.is_empty() {
        output.push_str("First actions:\n");
        for action in &report.first_actions {
            output.push_str(&format!("- {action}\n"));
        }
    }
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

pub(super) fn forge_operator_dashboard_markdown(report: &DxForgeOperatorDashboardReport) -> String {
    let mut output = format!(
        "# DX Forge Operator Dashboard\n\n- Generated: `{}`\n- Status: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Release triage: `{}`\n- Beta artifacts: `{}`\n- CI snippets: `{}`\n- Installability trend: `{}`\n\n",
        report.generated_at,
        report.status,
        report.passed,
        report.score,
        report.fail_under,
        report
            .summary
            .release_status
            .as_deref()
            .unwrap_or("unknown"),
        report.summary.beta_status.as_deref().unwrap_or("unknown"),
        report.summary.ci_status.as_deref().unwrap_or("unknown"),
        report.summary.installability_trend
    );

    output.push_str("## Summary\n\n");
    output.push_str("| Signal | Value |\n");
    output.push_str("| --- | ---: |\n");
    for (label, value) in [
        ("Pages targets", report.summary.pages_targets.to_string()),
        ("R2 targets", report.summary.r2_targets.to_string()),
        ("CI snippets", report.summary.ci_snippets.to_string()),
        (
            "Install time",
            format!("{} ms", format_optional_u64(report.summary.install_time_ms)),
        ),
        (
            "Upgrade time",
            format!("{} ms", format_optional_u64(report.summary.upgrade_time_ms)),
        ),
        (
            "Signed CI provenance",
            report.summary.signed_ci_provenance.to_string(),
        ),
        (
            "Requires rebuild",
            report.summary.requires_rebuild.to_string(),
        ),
        (
            "Requires secrets",
            report.summary.requires_secrets.to_string(),
        ),
        (
            "No package installs",
            report.summary.no_package_installs.to_string(),
        ),
        (
            "No node_modules",
            report.summary.no_node_modules.to_string(),
        ),
    ] {
        output.push_str(&format!(
            "| {} | `{}` |\n",
            label,
            markdown_table_cell(&value)
        ));
    }

    output.push_str("\n## Checks\n\n");
    output.push_str("| Check | Passed | Score | Message | Evidence |\n");
    output.push_str("| --- | --- | ---: | --- | --- |\n");
    for card in &report.review_cards {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | {} | `{}` |\n",
            card.title,
            card.passed,
            card.score,
            markdown_table_cell(&card.message),
            markdown_table_cell(card.evidence.as_deref().unwrap_or("-"))
        ));
    }

    output.push_str("\n## First Actions\n\n");
    if report.first_actions.is_empty() {
        output.push_str("- No operator actions are required for the configured threshold.\n");
    } else {
        for action in &report.first_actions {
            output.push_str(&format!("- {}\n", markdown_table_cell(action)));
        }
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- No operator dashboard findings.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
    }

    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{command}`\n"));
    }

    output
}

pub(super) fn forge_operator_dashboard_failure_summary(
    report: &DxForgeOperatorDashboardReport,
) -> String {
    if report.findings.is_empty() {
        return format!(
            "DX Forge operator dashboard did not pass: score {} / 100, required {} / 100",
            report.score, report.fail_under
        );
    }
    format!(
        "DX Forge operator dashboard did not pass: {}",
        report.findings.join("; ")
    )
}

fn operator_check(
    passed: bool,
    score: u8,
    message: String,
    evidence: Option<String>,
) -> DxForgeOperatorDashboardCheck {
    DxForgeOperatorDashboardCheck {
        passed,
        score: if passed { score } else { score.min(89) },
        message,
        evidence,
    }
}

fn dashboard_card(
    key: &str,
    title: &str,
    check: &DxForgeOperatorDashboardCheck,
) -> DxForgeOperatorDashboardCard {
    DxForgeOperatorDashboardCard {
        key: key.to_string(),
        title: title.to_string(),
        passed: check.passed,
        score: check.score,
        message: check.message.clone(),
        evidence: check.evidence.clone(),
    }
}

fn row_by_id<'a>(report: &'a serde_json::Value, id: &str) -> Option<&'a serde_json::Value> {
    report
        .get("rows")
        .and_then(|value| value.as_array())?
        .iter()
        .find(|row| row.get("id").and_then(|value| value.as_str()) == Some(id))
}

fn count_targets(report: &serde_json::Value, channel: &str) -> u64 {
    report
        .get("artifact_targets")
        .and_then(|value| value.as_array())
        .map(|targets| {
            targets
                .iter()
                .filter(|target| {
                    target.get("channel").and_then(|value| value.as_str()) == Some(channel)
                        && target.get("passed").and_then(|value| value.as_bool()) != Some(false)
                })
                .count() as u64
        })
        .unwrap_or(0)
}

fn dashboard_findings(report: &serde_json::Value, label: &str) -> Vec<String> {
    report
        .get("findings")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|finding| finding.as_str())
        .filter(|finding| !finding.trim().is_empty())
        .map(|finding| format!("{label}: {finding}"))
        .collect()
}

fn operator_first_actions(
    release_triage: &serde_json::Value,
    review_cards: &[DxForgeOperatorDashboardCard],
) -> Vec<String> {
    let mut actions = release_triage
        .get("first_actions")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|action| action.as_str())
        .filter(|action| !action.trim().is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();

    for card in review_cards.iter().filter(|card| !card.passed) {
        actions.push(format!("Repair {} before promoting the beta.", card.title));
    }

    actions.sort();
    actions.dedup();
    actions
}

fn format_optional_u64(value: Option<u64>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "n/a".to_string())
}
