use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::Serialize;

use super::{
    forge_publish_plan_json_bool, forge_publish_plan_json_string, forge_publish_plan_json_value,
    json_u8, markdown_table_cell, read_forge_publish_plan_json,
};
#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeReleaseTriageReport {
    version: u32,
    release_operations: PathBuf,
    publish_plan: PathBuf,
    generated_at: String,
    passed: bool,
    shipping_ready: bool,
    status: String,
    score: u8,
    fail_under: u8,
    source_reports: DxForgeReleaseTriageSourceReports,
    groups: DxForgeReleaseTriageGroups,
    first_actions: Vec<String>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseTriageSourceReports {
    release_operations_passed: bool,
    release_operations_status: Option<String>,
    release_operations_score: u8,
    publish_plan_passed: bool,
    publish_plan_status: Option<String>,
    publish_plan_score: u8,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseTriageGroups {
    missing_artifacts: DxForgeReleaseTriageGroup,
    secret_risk: DxForgeReleaseTriageGroup,
    cache_policy: DxForgeReleaseTriageGroup,
    rollback_readiness: DxForgeReleaseTriageGroup,
    dependency_boundary: DxForgeReleaseTriageGroup,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseTriageGroup {
    key: String,
    title: String,
    passed: bool,
    score: u8,
    finding_count: usize,
    findings: Vec<String>,
    evidence: Vec<String>,
    actions: Vec<String>,
}

#[derive(Debug, Default)]
struct DxForgeReleaseTriageGroupBuilder {
    findings: BTreeSet<String>,
    evidence: BTreeSet<String>,
    actions: BTreeSet<String>,
    scores: Vec<u8>,
}

pub(super) fn build_forge_release_triage_report(
    release_operations_path: &Path,
    publish_plan_path: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeReleaseTriageReport> {
    let release_operations =
        read_forge_publish_plan_json(release_operations_path, "release operations")?;
    let publish_plan = read_forge_publish_plan_json(publish_plan_path, "publish plan")?;

    let source_reports = DxForgeReleaseTriageSourceReports {
        release_operations_passed: forge_publish_plan_json_bool(&release_operations, &["passed"]),
        release_operations_status: forge_publish_plan_json_string(&release_operations, &["status"])
            .map(str::to_string),
        release_operations_score: json_u8(release_operations.get("score")).unwrap_or(0),
        publish_plan_passed: forge_publish_plan_json_bool(&publish_plan, &["passed"]),
        publish_plan_status: forge_publish_plan_json_string(&publish_plan, &["status"])
            .map(str::to_string),
        publish_plan_score: json_u8(publish_plan.get("score")).unwrap_or(0),
    };

    let mut missing_artifacts = DxForgeReleaseTriageGroupBuilder::default();
    let mut secret_risk = DxForgeReleaseTriageGroupBuilder::default();
    let mut cache_policy = DxForgeReleaseTriageGroupBuilder::default();
    let mut rollback_readiness = DxForgeReleaseTriageGroupBuilder::default();
    let mut dependency_boundary = DxForgeReleaseTriageGroupBuilder::default();

    for (path, label) in [
        (
            ["checks", "signed_manifest"],
            "release-operations signed manifest",
        ),
        (
            ["checks", "trust_regression"],
            "release-operations trust regression",
        ),
        (
            ["checks", "release_candidate"],
            "release-operations release candidate",
        ),
        (
            ["checks", "ci_artifacts"],
            "release-operations CI artifacts",
        ),
        (
            ["checks", "public_evidence"],
            "release-operations public evidence",
        ),
        (
            ["checks", "package_gallery"],
            "release-operations package gallery",
        ),
    ] {
        forge_release_triage_collect_check(
            &release_operations,
            &path,
            label,
            &mut missing_artifacts,
            "Restore missing artifacts from the release bundle and CI evidence lane before rerunning publish-plan.",
        );
    }

    for (path, label) in [
        (
            ["checks", "local_artifacts"],
            "publish-plan local artifacts",
        ),
        (
            ["checks", "pages_artifacts"],
            "publish-plan Pages artifacts",
        ),
        (["checks", "r2_artifacts"], "publish-plan R2 artifacts"),
    ] {
        forge_release_triage_collect_check(
            &publish_plan,
            &path,
            label,
            &mut missing_artifacts,
            "Restore missing artifacts from the release bundle and CI evidence lane before rerunning publish-plan.",
        );
    }

    forge_release_triage_collect_check(
        &publish_plan,
        &["checks", "no_secret_requirements"],
        "publish-plan secret requirements",
        &mut secret_risk,
        "Keep registry operations dry-run or move live publishing behind reviewed secret handling.",
    );
    forge_release_triage_collect_check(
        &publish_plan,
        &["checks", "cache_headers"],
        "publish-plan cache headers",
        &mut cache_policy,
        "Repair cache policy rows before promoting Pages or R2 artifacts.",
    );
    forge_release_triage_collect_check(
        &publish_plan,
        &["checks", "rollback_inputs"],
        "publish-plan rollback inputs",
        &mut rollback_readiness,
        "Restore rollback inputs so the beta can be reverted from reviewed artifacts.",
    );
    forge_release_triage_collect_check(
        &release_operations,
        &["checks", "no_node_modules"],
        "release-operations dependency boundary",
        &mut dependency_boundary,
        "Remove node_modules from release, Pages, and registry artifact boundaries.",
    );
    forge_release_triage_collect_check(
        &publish_plan,
        &["checks", "no_node_modules"],
        "publish-plan dependency boundary",
        &mut dependency_boundary,
        "Remove node_modules from release, Pages, and registry artifact boundaries.",
    );

    forge_release_triage_collect_artifact_targets(
        &publish_plan,
        &mut missing_artifacts,
        &mut cache_policy,
    );
    forge_release_triage_collect_cache_headers(&publish_plan, &mut cache_policy);
    forge_release_triage_collect_rollback_inputs(&publish_plan, &mut rollback_readiness);
    forge_release_triage_collect_secret_requirements(&publish_plan, &mut secret_risk);
    forge_release_triage_collect_no_node_modules(
        &release_operations,
        "release-operations",
        &mut dependency_boundary,
    );
    forge_release_triage_collect_no_node_modules(
        &publish_plan,
        "publish-plan",
        &mut dependency_boundary,
    );
    forge_release_triage_collect_report_findings(
        &release_operations,
        "release-operations",
        &mut missing_artifacts,
        &mut secret_risk,
        &mut cache_policy,
        &mut rollback_readiness,
        &mut dependency_boundary,
    );
    forge_release_triage_collect_report_findings(
        &publish_plan,
        "publish-plan",
        &mut missing_artifacts,
        &mut secret_risk,
        &mut cache_policy,
        &mut rollback_readiness,
        &mut dependency_boundary,
    );

    let groups = DxForgeReleaseTriageGroups {
        missing_artifacts: missing_artifacts.finish("missing_artifacts", "Missing Artifacts"),
        secret_risk: secret_risk.finish("secret_risk", "Secret Risk"),
        cache_policy: cache_policy.finish("cache_policy", "Cache Policy"),
        rollback_readiness: rollback_readiness.finish("rollback_readiness", "Rollback Readiness"),
        dependency_boundary: dependency_boundary
            .finish("dependency_boundary", "Dependency Boundary"),
    };
    let group_refs = [
        &groups.missing_artifacts,
        &groups.secret_risk,
        &groups.cache_policy,
        &groups.rollback_readiness,
        &groups.dependency_boundary,
    ];
    let shipping_ready = source_reports.release_operations_passed
        && source_reports.publish_plan_passed
        && group_refs.iter().all(|group| group.passed);
    let score = [
        source_reports.release_operations_score,
        source_reports.publish_plan_score,
        groups.missing_artifacts.score,
        groups.secret_risk.score,
        groups.cache_policy.score,
        groups.rollback_readiness.score,
        groups.dependency_boundary.score,
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let passed = shipping_ready && score >= fail_under;
    let status = if passed {
        "ready-for-operator-review"
    } else {
        "operator-action-required"
    }
    .to_string();
    let first_actions = forge_release_triage_first_actions(&groups);
    let findings = group_refs
        .iter()
        .filter(|group| !group.passed)
        .flat_map(|group| {
            group
                .findings
                .iter()
                .map(|finding| format!("{}: {finding}", group.key))
        })
        .collect::<Vec<_>>();

    Ok(DxForgeReleaseTriageReport {
        version: 1,
        release_operations: release_operations_path.to_path_buf(),
        publish_plan: publish_plan_path.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        shipping_ready,
        status,
        score,
        fail_under,
        source_reports,
        groups,
        first_actions,
        findings,
        next_commands: vec![
            format!(
                "dx forge release-triage --release-operations {} --publish-plan {} --format markdown --fail-under {}",
                release_operations_path.display(),
                publish_plan_path.display(),
                fail_under
            ),
            "dx forge release-operations --project . --release-manifest <manifest> --trust-regression <json> --release-candidate <json> --ci-artifacts <dir> --public-evidence <dir> --format markdown".to_string(),
            "dx forge publish-plan --project . --release-bundle <bundle> --pages <pages> --registry-smoke <json> --release-operations <json> --format markdown".to_string(),
        ],
    })
}

impl DxForgeReleaseTriageGroupBuilder {
    fn add(&mut self, finding: impl Into<String>, score: u8, evidence: Option<&str>, action: &str) {
        let finding = finding.into();
        if !finding.trim().is_empty() {
            self.findings.insert(finding);
        }
        if let Some(evidence) = evidence.filter(|evidence| !evidence.trim().is_empty()) {
            self.evidence.insert(evidence.to_string());
        }
        if !action.trim().is_empty() {
            self.actions.insert(action.to_string());
        }
        self.scores.push(score);
    }

    fn finish(self, key: &str, title: &str) -> DxForgeReleaseTriageGroup {
        let finding_count = self.findings.len();
        let passed = finding_count == 0;
        let score = if passed {
            100
        } else {
            self.scores.into_iter().min().unwrap_or(0)
        };
        DxForgeReleaseTriageGroup {
            key: key.to_string(),
            title: title.to_string(),
            passed,
            score,
            finding_count,
            findings: self.findings.into_iter().collect(),
            evidence: self.evidence.into_iter().collect(),
            actions: if passed {
                Vec::new()
            } else {
                self.actions.into_iter().collect()
            },
        }
    }
}

fn forge_release_triage_collect_check(
    report: &serde_json::Value,
    path: &[&str],
    label: &str,
    group: &mut DxForgeReleaseTriageGroupBuilder,
    action: &str,
) {
    let Some(check) = forge_publish_plan_json_value(report, path) else {
        return;
    };
    if check.get("passed").and_then(|value| value.as_bool()) == Some(true) {
        return;
    }
    let score = json_u8(check.get("score")).unwrap_or(0);
    let message = check
        .get("message")
        .and_then(|value| value.as_str())
        .unwrap_or("check did not pass");
    let evidence = check.get("evidence").and_then(|value| value.as_str());
    group.add(format!("{label}: {message}"), score, evidence, action);
}

fn forge_release_triage_collect_artifact_targets(
    publish_plan: &serde_json::Value,
    missing_artifacts: &mut DxForgeReleaseTriageGroupBuilder,
    cache_policy: &mut DxForgeReleaseTriageGroupBuilder,
) {
    let Some(targets) = publish_plan
        .get("artifact_targets")
        .and_then(|value| value.as_array())
    else {
        return;
    };
    for target in targets {
        let required = target
            .get("required")
            .and_then(|value| value.as_bool())
            .unwrap_or(true);
        let passed = target.get("passed").and_then(|value| value.as_bool()) == Some(true);
        let artifact = target
            .get("artifact")
            .and_then(|value| value.as_str())
            .unwrap_or("artifact");
        let destination = target
            .get("destination")
            .and_then(|value| value.as_str())
            .unwrap_or("destination");
        let message = target
            .get("message")
            .and_then(|value| value.as_str())
            .unwrap_or("target did not pass");
        if required && !passed {
            missing_artifacts.add(
                format!("artifact target `{artifact}` -> `{destination}`: {message}"),
                0,
                target.get("source").and_then(|value| value.as_str()),
                "Restore missing artifacts from the release bundle and CI evidence lane before rerunning publish-plan.",
            );
        }
        if target
            .get("cache_control")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .trim()
            .is_empty()
        {
            cache_policy.add(
                format!(
                    "artifact target `{artifact}` -> `{destination}` has no cache-control policy"
                ),
                0,
                target.get("source").and_then(|value| value.as_str()),
                "Repair cache policy rows before promoting Pages or R2 artifacts.",
            );
        }
    }
}

fn forge_release_triage_collect_cache_headers(
    publish_plan: &serde_json::Value,
    group: &mut DxForgeReleaseTriageGroupBuilder,
) {
    let Some(headers) = publish_plan
        .get("cache_headers")
        .and_then(|value| value.as_array())
    else {
        return;
    };
    for header in headers {
        let required = header
            .get("required")
            .and_then(|value| value.as_bool())
            .unwrap_or(true);
        let passed = header.get("passed").and_then(|value| value.as_bool()) == Some(true);
        if !required || passed {
            continue;
        }
        let channel = header
            .get("channel")
            .and_then(|value| value.as_str())
            .unwrap_or("channel");
        let pattern = header
            .get("pattern")
            .and_then(|value| value.as_str())
            .unwrap_or("pattern");
        let reason = header
            .get("reason")
            .and_then(|value| value.as_str())
            .unwrap_or("cache header did not pass");
        group.add(
            format!("cache header `{channel}` `{pattern}`: {reason}"),
            0,
            None,
            "Repair cache policy rows before promoting Pages or R2 artifacts.",
        );
    }
}

fn forge_release_triage_collect_rollback_inputs(
    publish_plan: &serde_json::Value,
    group: &mut DxForgeReleaseTriageGroupBuilder,
) {
    let Some(inputs) = publish_plan
        .get("rollback_inputs")
        .and_then(|value| value.as_array())
    else {
        return;
    };
    for input in inputs {
        let required = input
            .get("required")
            .and_then(|value| value.as_bool())
            .unwrap_or(true);
        let passed = input.get("passed").and_then(|value| value.as_bool()) == Some(true);
        if !required || passed {
            continue;
        }
        let name = input
            .get("name")
            .and_then(|value| value.as_str())
            .unwrap_or("rollback-input");
        let message = input
            .get("message")
            .and_then(|value| value.as_str())
            .unwrap_or("rollback input did not pass");
        group.add(
            format!("rollback input `{name}`: {message}"),
            0,
            input.get("path").and_then(|value| value.as_str()),
            "Restore rollback inputs so the beta can be reverted from reviewed artifacts.",
        );
    }
}

fn forge_release_triage_collect_secret_requirements(
    publish_plan: &serde_json::Value,
    group: &mut DxForgeReleaseTriageGroupBuilder,
) {
    let Some(secret_requirements) = publish_plan.get("secret_requirements") else {
        return;
    };
    if secret_requirements
        .get("requires_secrets")
        .and_then(|value| value.as_bool())
        == Some(true)
    {
        group.add(
            "publish-plan secret requirements: registry smoke requires secrets",
            json_u8(secret_requirements.get("score")).unwrap_or(0),
            None,
            "Keep registry operations dry-run or move live publishing behind reviewed secret handling.",
        );
    }
    if secret_requirements
        .get("registry_operations_dry_run")
        .and_then(|value| value.as_bool())
        == Some(false)
    {
        group.add(
            "publish-plan secret requirements: registry operations are not all dry-run",
            json_u8(secret_requirements.get("score")).unwrap_or(0),
            None,
            "Keep registry operations dry-run or move live publishing behind reviewed secret handling.",
        );
    }
    forge_release_triage_collect_nested_findings(
        secret_requirements,
        "secret-requirements",
        group,
        "Keep registry operations dry-run or move live publishing behind reviewed secret handling.",
    );
}

fn forge_release_triage_collect_no_node_modules(
    report: &serde_json::Value,
    label: &str,
    group: &mut DxForgeReleaseTriageGroupBuilder,
) {
    let Some(no_node_modules) = report.get("no_node_modules") else {
        return;
    };
    if no_node_modules
        .get("passed")
        .and_then(|value| value.as_bool())
        == Some(false)
    {
        group.add(
            format!("{label} no_node_modules boundary failed"),
            json_u8(no_node_modules.get("score")).unwrap_or(0),
            None,
            "Remove node_modules from release, Pages, and registry artifact boundaries.",
        );
    }
    forge_release_triage_collect_nested_findings(
        no_node_modules,
        label,
        group,
        "Remove node_modules from release, Pages, and registry artifact boundaries.",
    );
}

fn forge_release_triage_collect_nested_findings(
    value: &serde_json::Value,
    label: &str,
    group: &mut DxForgeReleaseTriageGroupBuilder,
    action: &str,
) {
    if let Some(findings) = value.get("findings").and_then(|value| value.as_array()) {
        for finding in findings.iter().filter_map(|value| value.as_str()) {
            group.add(format!("{label}: {finding}"), 0, None, action);
        }
    }
}

fn forge_release_triage_collect_report_findings(
    report: &serde_json::Value,
    label: &str,
    missing_artifacts: &mut DxForgeReleaseTriageGroupBuilder,
    secret_risk: &mut DxForgeReleaseTriageGroupBuilder,
    cache_policy: &mut DxForgeReleaseTriageGroupBuilder,
    rollback_readiness: &mut DxForgeReleaseTriageGroupBuilder,
    dependency_boundary: &mut DxForgeReleaseTriageGroupBuilder,
) {
    let Some(findings) = report.get("findings").and_then(|value| value.as_array()) else {
        return;
    };
    for finding in findings.iter().filter_map(|value| value.as_str()) {
        let lower = finding.to_ascii_lowercase();
        if lower.contains("secret") || lower.contains("cloudflare_r2_") {
            secret_risk.add(
                format!("{label}: {finding}"),
                0,
                None,
                "Keep registry operations dry-run or move live publishing behind reviewed secret handling.",
            );
        }
        if lower.contains("cache") {
            cache_policy.add(
                format!("{label}: {finding}"),
                0,
                None,
                "Repair cache policy rows before promoting Pages or R2 artifacts.",
            );
        }
        if lower.contains("rollback") {
            rollback_readiness.add(
                format!("{label}: {finding}"),
                0,
                None,
                "Restore rollback inputs so the beta can be reverted from reviewed artifacts.",
            );
        }
        if lower.contains("node_modules") {
            dependency_boundary.add(
                format!("{label}: {finding}"),
                0,
                None,
                "Remove node_modules from release, Pages, and registry artifact boundaries.",
            );
        }
        if lower.contains("missing")
            || lower.contains("does not exist")
            || lower.contains("unreadable")
            || lower.contains("artifact")
            || lower.contains("manifest")
            || lower.contains("package-gallery")
        {
            missing_artifacts.add(
                format!("{label}: {finding}"),
                0,
                None,
                "Restore missing artifacts from the release bundle and CI evidence lane before rerunning publish-plan.",
            );
        }
    }
}

fn forge_release_triage_first_actions(groups: &DxForgeReleaseTriageGroups) -> Vec<String> {
    [
        &groups.missing_artifacts,
        &groups.secret_risk,
        &groups.cache_policy,
        &groups.rollback_readiness,
        &groups.dependency_boundary,
    ]
    .into_iter()
    .filter(|group| !group.passed)
    .filter_map(|group| group.actions.first().cloned())
    .collect()
}

pub(super) fn forge_release_triage_terminal(report: &DxForgeReleaseTriageReport) -> String {
    let mut output = format!(
        "DX Forge release triage\nRelease operations: {}\nPublish plan: {}\nStatus: {} ({} / 100)\nShipping ready: {}\nMissing artifacts: {}\nSecret risk: {}\nCache policy: {}\nRollback readiness: {}\nDependency boundary: {}\n",
        report.release_operations.display(),
        report.publish_plan.display(),
        report.status,
        report.score,
        report.shipping_ready,
        report.groups.missing_artifacts.finding_count,
        report.groups.secret_risk.finding_count,
        report.groups.cache_policy.finding_count,
        report.groups.rollback_readiness.finding_count,
        report.groups.dependency_boundary.finding_count
    );
    if !report.first_actions.is_empty() {
        output.push_str("First actions:\n");
        for action in &report.first_actions {
            output.push_str(&format!("- {action}\n"));
        }
    }
    output
}

pub(super) fn forge_release_triage_markdown(report: &DxForgeReleaseTriageReport) -> String {
    let mut output = format!(
        "# DX Forge Release Triage\n\n- Release operations: `{}`\n- Publish plan: `{}`\n- Generated: `{}`\n- Status: `{}`\n- Passed: `{}`\n- Shipping ready: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n\n",
        report.release_operations.display(),
        report.publish_plan.display(),
        report.generated_at,
        report.status,
        report.passed,
        report.shipping_ready,
        report.score,
        report.fail_under
    );

    output.push_str("## Source Reports\n\n");
    output.push_str("| Report | Passed | Status | Score |\n");
    output.push_str("| --- | --- | --- | ---: |\n");
    output.push_str(&format!(
        "| `release-operations` | `{}` | `{}` | {} |\n",
        report.source_reports.release_operations_passed,
        markdown_table_cell(
            report
                .source_reports
                .release_operations_status
                .as_deref()
                .unwrap_or("missing")
        ),
        report.source_reports.release_operations_score
    ));
    output.push_str(&format!(
        "| `publish-plan` | `{}` | `{}` | {} |\n",
        report.source_reports.publish_plan_passed,
        markdown_table_cell(
            report
                .source_reports
                .publish_plan_status
                .as_deref()
                .unwrap_or("missing")
        ),
        report.source_reports.publish_plan_score
    ));

    output.push_str("\n## Triage Groups\n\n");
    for group in [
        &report.groups.missing_artifacts,
        &report.groups.secret_risk,
        &report.groups.cache_policy,
        &report.groups.rollback_readiness,
        &report.groups.dependency_boundary,
    ] {
        output.push_str(&format!(
            "### {}\n\n- Passed: `{}`\n- Score: `{}` / `100`\n- Findings: `{}`\n\n",
            group.title, group.passed, group.score, group.finding_count
        ));
        if group.findings.is_empty() {
            output.push_str("- No findings in this group.\n\n");
        } else {
            output.push_str("Findings:\n");
            for finding in &group.findings {
                output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
            }
            output.push('\n');
        }
        if !group.actions.is_empty() {
            output.push_str("Actions:\n");
            for action in &group.actions {
                output.push_str(&format!("- {}\n", markdown_table_cell(action)));
            }
            output.push('\n');
        }
        if !group.evidence.is_empty() {
            output.push_str("Evidence:\n");
            for evidence in &group.evidence {
                output.push_str(&format!("- `{}`\n", markdown_table_cell(evidence)));
            }
            output.push('\n');
        }
    }

    output.push_str("## First Actions\n\n");
    if report.first_actions.is_empty() {
        output.push_str(
            "- No immediate operator actions; keep this triage artifact with the release packet.\n",
        );
    } else {
        for action in &report.first_actions {
            output.push_str(&format!("- {}\n", markdown_table_cell(action)));
        }
    }

    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{command}`\n"));
    }

    output
}
