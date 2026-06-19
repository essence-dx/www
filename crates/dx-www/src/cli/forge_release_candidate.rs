use std::collections::HashSet;
use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::Serialize;

use super::formatting::markdown_table_cell;
use super::{
    DxForgeReleaseDashboardRouteComparison, FORGE_PUBLIC_SECRET_MARKERS,
    FORGE_REQUIRED_PUBLIC_ROUTES, FORGE_WWW_TEMPLATE_PACKAGE_IDS,
    append_release_dashboard_findings, json_u8, verify_forge_ci_artifacts,
    verify_forge_pages_bundle, verify_release_dashboard_route_comparison,
};

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeReleaseCandidateReport {
    version: u32,
    project: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    checks: DxForgeReleaseCandidateChecks,
    ci_artifacts: DxForgeReleaseCandidateCiArtifacts,
    pages_bundle: DxForgeReleaseCandidatePagesBundle,
    route_comparison: DxForgeReleaseDashboardRouteComparison,
    source_owned_review: DxForgeReleaseCandidateSourceOwnedReview,
    static_competitor_evidence: DxForgeReleaseCandidateStaticEvidence,
    secret_markers: DxForgeReleaseCandidateSecretMarkers,
    no_node_modules: DxForgeReleaseCandidateNoNodeModules,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl DxForgeReleaseCandidateReport {
    pub(super) fn score(&self) -> u8 {
        self.score
    }

    pub(super) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseCandidateChecks {
    ci_artifacts: DxForgeReleaseCandidateCheck,
    pages_bundle: DxForgeReleaseCandidateCheck,
    route_comparison: DxForgeReleaseCandidateCheck,
    source_owned_review: DxForgeReleaseCandidateCheck,
    static_competitor_evidence: DxForgeReleaseCandidateCheck,
    secret_markers: DxForgeReleaseCandidateCheck,
    no_node_modules: DxForgeReleaseCandidateCheck,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseCandidateCheck {
    passed: bool,
    score: u8,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseCandidateCiArtifacts {
    artifact_dir: PathBuf,
    passed: bool,
    score: u8,
    artifact_count: usize,
    route_count: usize,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseCandidatePagesBundle {
    bundle_dir: PathBuf,
    passed: bool,
    score: u8,
    artifact_count: usize,
    check_count: usize,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseCandidateSourceOwnedReview {
    path: PathBuf,
    passed: bool,
    score: u8,
    no_node_modules: bool,
    package_count: u64,
    required_signals: Vec<String>,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseCandidateStaticEvidence {
    path: PathBuf,
    passed: bool,
    score: u8,
    route_count: usize,
    framework_count: usize,
    static_floor_count: usize,
    scope: serde_json::Value,
    required_signals: Vec<String>,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeReleaseCandidateSecretMarkers {
    pub(super) passed: bool,
    pub(super) score: u8,
    pub(super) scanned_paths: Vec<PathBuf>,
    pub(super) findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeReleaseCandidateNoNodeModules {
    pub(super) passed: bool,
    pub(super) score: u8,
    pub(super) checked_paths: Vec<PathBuf>,
    pub(super) findings: Vec<String>,
}

pub(super) fn build_forge_release_candidate_report(
    project: &Path,
    ci_artifacts: &Path,
    pages: &Path,
    route_comparison: &Path,
    source_review: &Path,
    static_evidence: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeReleaseCandidateReport> {
    let ci_report = verify_forge_ci_artifacts(ci_artifacts)?;
    let pages_report = verify_forge_pages_bundle(pages)?;
    let route_comparison = verify_release_dashboard_route_comparison(route_comparison)?;
    let source_owned_review =
        verify_release_candidate_source_owned_review(source_review, fail_under)?;
    let static_competitor_evidence =
        verify_release_candidate_static_evidence(static_evidence, fail_under)?;
    let secret_markers = verify_release_candidate_secret_markers(&[
        ci_artifacts,
        pages,
        route_comparison.path.as_path(),
        source_owned_review.path.as_path(),
        static_competitor_evidence.path.as_path(),
    ]);
    let no_node_modules = verify_release_candidate_no_node_modules(&[
        project,
        ci_artifacts,
        pages,
        source_owned_review.path.parent().unwrap_or(source_review),
        static_competitor_evidence
            .path
            .parent()
            .unwrap_or(static_evidence),
    ]);

    let ci_artifacts = DxForgeReleaseCandidateCiArtifacts {
        artifact_dir: ci_report.artifact_dir.clone(),
        passed: ci_report.passed,
        score: ci_report.score,
        artifact_count: ci_report.artifacts.len(),
        route_count: ci_report.routes.len(),
        findings: ci_report.findings.clone(),
    };
    let pages_bundle = DxForgeReleaseCandidatePagesBundle {
        bundle_dir: pages_report.bundle_dir.clone(),
        passed: pages_report.passed,
        score: pages_report.score,
        artifact_count: pages_report.artifacts.len(),
        check_count: pages_report.checks.len(),
        findings: pages_report.findings.clone(),
    };
    let checks = DxForgeReleaseCandidateChecks {
        ci_artifacts: release_candidate_check(
            ci_artifacts.passed,
            ci_artifacts.score,
            format!(
                "{} CI artifacts and {} route checks verified.",
                ci_artifacts.artifact_count, ci_artifacts.route_count
            ),
        ),
        pages_bundle: release_candidate_check(
            pages_bundle.passed,
            pages_bundle.score,
            format!(
                "{} Pages artifacts and {} publish checks verified.",
                pages_bundle.artifact_count, pages_bundle.check_count
            ),
        ),
        route_comparison: release_candidate_check(
            route_comparison.passed,
            route_comparison.score,
            format!(
                "{} public routes measured, {} Brotli bytes total.",
                route_comparison.route_count, route_comparison.total_brotli_bytes
            ),
        ),
        source_owned_review: release_candidate_check(
            source_owned_review.passed,
            source_owned_review.score,
            format!(
                "{} source-owned package(s), no node_modules: {}.",
                source_owned_review.package_count, source_owned_review.no_node_modules
            ),
        ),
        static_competitor_evidence: release_candidate_check(
            static_competitor_evidence.passed,
            static_competitor_evidence.score,
            format!(
                "{} framework row(s), {} static-floor row(s), {} required route(s).",
                static_competitor_evidence.framework_count,
                static_competitor_evidence.static_floor_count,
                static_competitor_evidence.route_count
            ),
        ),
        secret_markers: release_candidate_check(
            secret_markers.passed,
            secret_markers.score,
            format!(
                "{} release-candidate input(s) scanned for public secret markers.",
                secret_markers.scanned_paths.len()
            ),
        ),
        no_node_modules: release_candidate_check(
            no_node_modules.passed,
            no_node_modules.score,
            format!(
                "{} dependency boundary path(s) checked.",
                no_node_modules.checked_paths.len()
            ),
        ),
    };

    let mut findings = Vec::new();
    append_release_dashboard_findings("ci-artifacts", &ci_artifacts.findings, &mut findings);
    append_release_dashboard_findings("pages-bundle", &pages_bundle.findings, &mut findings);
    append_release_dashboard_findings(
        "route-comparison",
        &route_comparison.findings,
        &mut findings,
    );
    append_release_dashboard_findings(
        "source-owned-review",
        &source_owned_review.findings,
        &mut findings,
    );
    append_release_dashboard_findings(
        "static-competitor-evidence",
        &static_competitor_evidence.findings,
        &mut findings,
    );
    append_release_dashboard_findings("secret-markers", &secret_markers.findings, &mut findings);
    append_release_dashboard_findings("no-node-modules", &no_node_modules.findings, &mut findings);

    let score = [
        checks.ci_artifacts.score,
        checks.pages_bundle.score,
        checks.route_comparison.score,
        checks.source_owned_review.score,
        checks.static_competitor_evidence.score,
        checks.secret_markers.score,
        checks.no_node_modules.score,
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let passed = checks.ci_artifacts.passed
        && checks.pages_bundle.passed
        && checks.route_comparison.passed
        && checks.source_owned_review.passed
        && checks.static_competitor_evidence.passed
        && checks.secret_markers.passed
        && checks.no_node_modules.passed
        && score >= fail_under;

    Ok(DxForgeReleaseCandidateReport {
        version: 1,
        project: project.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        fail_under,
        checks,
        ci_artifacts,
        pages_bundle,
        route_comparison,
        source_owned_review,
        static_competitor_evidence,
        secret_markers,
        no_node_modules,
        findings,
        next_commands: vec![
            "dx forge release-candidate --project . --format markdown".to_string(),
            "dx forge release-review --project . --format markdown".to_string(),
            "node .\\benchmarks\\compare-forge-static-competitors.ts".to_string(),
        ],
    })
}

fn verify_release_candidate_source_owned_review(
    path: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeReleaseCandidateSourceOwnedReview> {
    let raw = std::fs::read(path)?;
    let value = serde_json::from_slice::<serde_json::Value>(&raw)?;
    let text = String::from_utf8_lossy(&raw).to_string();
    let normalized = text.to_ascii_lowercase();
    let mut findings = Vec::new();
    let mut penalty = 0u16;
    let reported_score = json_u8(value.get("score")).unwrap_or(100);
    let passed_signal = value
        .get("passed")
        .and_then(|value| value.as_bool())
        .unwrap_or(reported_score >= fail_under);
    let no_node_modules = value
        .get("no_node_modules")
        .and_then(|value| value.as_bool())
        == Some(true);
    let package_count = value
        .get("package_count")
        .and_then(|value| value.as_u64())
        .or_else(|| {
            value
                .get("packages")
                .and_then(|packages| packages.as_array())
                .map(|packages| packages.len() as u64)
        })
        .unwrap_or_default();
    let mut required_signals = Vec::new();

    if !passed_signal {
        findings.push("source-owned package review report does not pass".to_string());
        penalty = penalty.saturating_add(30);
    }
    if reported_score < fail_under {
        findings.push(format!(
            "source-owned package review score {reported_score} is below {fail_under}"
        ));
        penalty = penalty.saturating_add(25);
    }
    if !no_node_modules {
        findings
            .push("source-owned package review must explicitly prove no node_modules".to_string());
        penalty = penalty.saturating_add(25);
    }
    if package_count < FORGE_WWW_TEMPLATE_PACKAGE_IDS.len() as u64 {
        findings.push(format!(
            "source-owned package review covers {package_count} package(s); expected at least {}",
            FORGE_WWW_TEMPLATE_PACKAGE_IDS.len()
        ));
        penalty = penalty.saturating_add(20);
    }

    for package_id in FORGE_WWW_TEMPLATE_PACKAGE_IDS {
        if text.contains(package_id) {
            required_signals.push(package_id.to_string());
        } else {
            findings.push(format!(
                "source-owned package review is missing launch package `{package_id}`"
            ));
            penalty = penalty.saturating_add(12);
        }
    }

    for signal in ["docs", "receipt", "rollback", "advisory", "yellow"] {
        if normalized.contains(signal) {
            required_signals.push(signal.to_string());
        } else {
            findings.push(format!(
                "source-owned package review is missing `{signal}` evidence"
            ));
            penalty = penalty.saturating_add(10);
        }
    }

    let score = reported_score.min(100u8.saturating_sub(penalty.min(100) as u8));
    let passed = passed_signal && findings.is_empty() && score >= fail_under;

    Ok(DxForgeReleaseCandidateSourceOwnedReview {
        path: path.to_path_buf(),
        passed,
        score,
        no_node_modules,
        package_count,
        required_signals,
        findings,
    })
}

fn verify_release_candidate_static_evidence(
    path: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeReleaseCandidateStaticEvidence> {
    let raw = std::fs::read(path)?;
    let value = serde_json::from_slice::<serde_json::Value>(&raw)?;
    let text = String::from_utf8_lossy(&raw).to_string();
    let normalized = text.to_ascii_lowercase();
    let mut findings = Vec::new();
    let mut penalty = 0u16;
    let reported_score = json_u8(value.get("score")).unwrap_or(100);
    let passed_signal = value
        .get("passed")
        .and_then(|value| value.as_bool())
        .unwrap_or(true);
    let scope = value
        .get("scope")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let required_routes = value
        .get("required_routes")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();
    let frameworks = value
        .get("frameworks")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();
    let static_floor_count = frameworks
        .iter()
        .filter(|framework| {
            framework
                .get("baseline_kind")
                .and_then(|value| value.as_str())
                .is_some_and(|kind| kind.contains("static-floor"))
        })
        .count();
    let mut required_signals = Vec::new();

    if !passed_signal {
        findings.push("static competitor evidence report does not pass".to_string());
        penalty = penalty.saturating_add(30);
    }
    if reported_score < fail_under {
        findings.push(format!(
            "static competitor evidence score {reported_score} is below {fail_under}"
        ));
        penalty = penalty.saturating_add(25);
    }

    for scope_flag in [
        "not_full_framework_benchmark",
        "competitor_builds_not_run",
        "no_package_install",
        "no_node_modules_created",
    ] {
        if scope.get(scope_flag).and_then(|value| value.as_bool()) == Some(true) {
            required_signals.push(scope_flag.to_string());
        } else {
            findings.push(format!(
                "static competitor evidence must set scope.{scope_flag}=true"
            ));
            penalty = penalty.saturating_add(12);
        }
    }

    for framework in ["dx-www", "astro", "svelte", "next"] {
        if normalized.contains(framework) {
            required_signals.push(framework.to_string());
        } else {
            findings.push(format!(
                "static competitor evidence is missing `{framework}` coverage"
            ));
            penalty = penalty.saturating_add(10);
        }
    }

    if !normalized.contains("static-floor") {
        findings.push("static competitor evidence must name static-floor baselines".to_string());
        penalty = penalty.saturating_add(12);
    } else {
        required_signals.push("static-floor".to_string());
    }
    if !normalized.contains("does not prove broad framework replacement")
        && !normalized.contains("not full framework benchmark")
        && !normalized.contains("not_full_framework_benchmark")
    {
        findings.push(
            "static competitor evidence must keep the broad framework replacement caveat"
                .to_string(),
        );
        penalty = penalty.saturating_add(12);
    } else {
        required_signals.push("honest-scope-caveat".to_string());
    }
    if required_routes.len() < FORGE_REQUIRED_PUBLIC_ROUTES.len() {
        findings.push(format!(
            "static competitor evidence covers {} route(s); expected at least {}",
            required_routes.len(),
            FORGE_REQUIRED_PUBLIC_ROUTES.len()
        ));
        penalty = penalty.saturating_add(12);
    }
    if static_floor_count < 2 {
        findings.push(format!(
            "static competitor evidence has {static_floor_count} static-floor row(s); expected at least 2"
        ));
        penalty = penalty.saturating_add(12);
    }

    let score = reported_score.min(100u8.saturating_sub(penalty.min(100) as u8));
    let passed = passed_signal && findings.is_empty() && score >= fail_under;

    Ok(DxForgeReleaseCandidateStaticEvidence {
        path: path.to_path_buf(),
        passed,
        score,
        route_count: required_routes.len(),
        framework_count: frameworks.len(),
        static_floor_count,
        scope,
        required_signals,
        findings,
    })
}

pub(super) fn verify_release_candidate_secret_markers(
    paths: &[&Path],
) -> DxForgeReleaseCandidateSecretMarkers {
    let mut findings = Vec::new();
    let mut scanned_paths = Vec::new();
    let mut visited = HashSet::<PathBuf>::new();

    for path in paths {
        let path = (*path).to_path_buf();
        if visited.insert(path.clone()) {
            scanned_paths.push(path.clone());
            scan_release_candidate_secret_path(&path, &path, &mut findings);
        }
    }

    let penalty = (findings.len() as u16).saturating_mul(20).min(100) as u8;
    let score = 100u8.saturating_sub(penalty);
    DxForgeReleaseCandidateSecretMarkers {
        passed: findings.is_empty(),
        score,
        scanned_paths,
        findings,
    }
}

fn scan_release_candidate_secret_path(root: &Path, path: &Path, findings: &mut Vec<String>) {
    if path.is_dir() {
        let entries = match std::fs::read_dir(path) {
            Ok(entries) => entries,
            Err(error) => {
                findings.push(format!(
                    "secret marker scan could not read {}: {error}",
                    path.display()
                ));
                return;
            }
        };
        for entry in entries {
            match entry {
                Ok(entry) => scan_release_candidate_secret_path(root, &entry.path(), findings),
                Err(error) => findings.push(format!("secret marker scan entry error: {error}")),
            }
        }
        return;
    }

    if !path.is_file() {
        findings.push(format!(
            "secret marker scan input does not exist: {}",
            path.display()
        ));
        return;
    }

    let raw = match std::fs::read(path) {
        Ok(raw) => raw,
        Err(error) => {
            findings.push(format!(
                "secret marker scan could not read {}: {error}",
                path.display()
            ));
            return;
        }
    };
    let text = String::from_utf8_lossy(&raw);
    for marker in FORGE_PUBLIC_SECRET_MARKERS {
        if text.contains(marker) {
            findings.push(format!(
                "{} contains {marker}",
                path.strip_prefix(root).unwrap_or(path).display()
            ));
        }
    }
}

pub(super) fn verify_release_candidate_no_node_modules(
    paths: &[&Path],
) -> DxForgeReleaseCandidateNoNodeModules {
    let mut checked_paths = Vec::new();
    let mut findings = Vec::new();
    let mut seen = HashSet::<PathBuf>::new();

    for path in paths {
        let node_modules = path.join("node_modules");
        if seen.insert(node_modules.clone()) {
            if node_modules.exists() {
                findings.push(format!("node_modules exists at {}", node_modules.display()));
            }
            checked_paths.push(node_modules);
        }
    }

    let penalty = (findings.len() as u16).saturating_mul(25).min(100) as u8;
    let score = 100u8.saturating_sub(penalty);
    DxForgeReleaseCandidateNoNodeModules {
        passed: findings.is_empty(),
        score,
        checked_paths,
        findings,
    }
}

fn release_candidate_check(
    passed: bool,
    score: u8,
    message: impl Into<String>,
) -> DxForgeReleaseCandidateCheck {
    DxForgeReleaseCandidateCheck {
        passed,
        score,
        message: message.into(),
    }
}

pub(super) fn forge_release_candidate_terminal(report: &DxForgeReleaseCandidateReport) -> String {
    let mut output = format!(
        "DX Forge release candidate\nProject: {}\nPassed: {}\nScore: {} / 100\nRequired score: {} / 100\nCI artifacts: {} / 100\nPages bundle: {} / 100\nRoute comparison: {} / 100\nSource-owned review: {} / 100\nStatic competitor evidence: {} / 100\nSecret markers: {} / 100\nNo node_modules: {} / 100\n",
        report.project.display(),
        report.passed,
        report.score,
        report.fail_under,
        report.ci_artifacts.score,
        report.pages_bundle.score,
        report.route_comparison.score,
        report.source_owned_review.score,
        report.static_competitor_evidence.score,
        report.secret_markers.score,
        report.no_node_modules.score
    );
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

pub(super) fn forge_release_candidate_markdown(report: &DxForgeReleaseCandidateReport) -> String {
    let mut output = format!(
        "# DX Forge Release Candidate Gate\n\n- Project: `{}`\n- Generated: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Passed: `{}`\n\n",
        report.project.display(),
        report.generated_at,
        report.score,
        report.fail_under,
        report.passed
    );

    output.push_str("## Checks\n\n");
    output.push_str("| Lane | Passed | Score | Summary |\n");
    output.push_str("| --- | --- | ---: | --- |\n");
    for (label, check) in [
        ("CI artifacts", &report.checks.ci_artifacts),
        ("Pages bundle", &report.checks.pages_bundle),
        ("Route comparison", &report.checks.route_comparison),
        ("Source-owned review", &report.checks.source_owned_review),
        (
            "Static competitor evidence",
            &report.checks.static_competitor_evidence,
        ),
        ("Secret markers", &report.checks.secret_markers),
        ("No node_modules", &report.checks.no_node_modules),
    ] {
        output.push_str(&format!(
            "| {} | `{}` | `{}` | {} |\n",
            markdown_table_cell(label),
            check.passed,
            check.score,
            markdown_table_cell(&check.message)
        ));
    }

    output.push_str("\n## Evidence Inputs\n\n");
    output.push_str(&format!(
        "- CI artifacts: `{}` (`{}` files, `{}` route checks)\n- Pages bundle: `{}` (`{}` files, `{}` publish checks)\n- Route comparison: `{}` (`{}` routes, `{}` Brotli bytes)\n- Source-owned review: `{}` (`{}` package rows, no `node_modules`: `{}`)\n- Static competitor evidence: `{}` (`{}` framework rows, `{}` static-floor rows)\n- Secret-marker inputs scanned: `{}`\n- Dependency boundary paths checked: `{}`\n\n",
        report.ci_artifacts.artifact_dir.display(),
        report.ci_artifacts.artifact_count,
        report.ci_artifacts.route_count,
        report.pages_bundle.bundle_dir.display(),
        report.pages_bundle.artifact_count,
        report.pages_bundle.check_count,
        report.route_comparison.path.display(),
        report.route_comparison.route_count,
        report.route_comparison.total_brotli_bytes,
        report.source_owned_review.path.display(),
        report.source_owned_review.package_count,
        report.source_owned_review.no_node_modules,
        report.static_competitor_evidence.path.display(),
        report.static_competitor_evidence.framework_count,
        report.static_competitor_evidence.static_floor_count,
        report.secret_markers.scanned_paths.len(),
        report.no_node_modules.checked_paths.len()
    ));

    output.push_str("## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- No release-candidate findings for the configured threshold.\n");
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

pub(super) fn forge_release_candidate_failure_summary(
    report: &DxForgeReleaseCandidateReport,
) -> String {
    if report.findings.is_empty() {
        return format!(
            "DX Forge release-candidate did not pass: score {} / 100, required {} / 100",
            report.score, report.fail_under
        );
    }

    format!(
        "DX Forge release-candidate did not pass: {}",
        report.findings.join("; ")
    )
}
