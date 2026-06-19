use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};
use chrono::Utc;
use dx_compiler::ecosystem::{
    DxForgePackageScorecardReport, build_forge_package_scorecard,
    build_forge_package_scorecard_for_project,
};
use serde::Serialize;

use super::forge_error;
use super::forge_public_evidence_options::{
    DxForgePublicEvidenceCommandOptions, parse_forge_public_evidence_options,
};
use super::options::DxOutputFormat;

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgePublicEvidenceReport {
    pub(super) version: u32,
    pub(super) route: String,
    pub(super) generated_at: String,
    pub(super) score: u8,
    pub(super) package_count: usize,
    pub(super) verified_packages: usize,
    pub(super) source_owned_packages: usize,
    pub(super) node_modules_packages: usize,
    pub(super) links: Vec<DxForgePublicEvidenceLink>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub(super) struct DxForgePublicEvidenceLink {
    pub(super) section: &'static str,
    pub(super) label: &'static str,
    pub(super) href: &'static str,
    pub(super) source_model: &'static str,
    pub(super) description: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgePublicEvidenceVerificationReport {
    pub(super) evidence_dir: PathBuf,
    pub(super) generated_at: String,
    pub(super) passed: bool,
    pub(super) score: u8,
    pub(super) artifacts: Vec<DxForgePublicEvidenceArtifactCheck>,
    pub(super) checks: Vec<DxForgePublicEvidenceCheck>,
    pub(super) findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgePublicEvidenceArtifactCheck {
    pub(super) name: String,
    pub(super) path: PathBuf,
    pub(super) exists: bool,
    pub(super) bytes: u64,
    pub(super) valid_json: Option<bool>,
    pub(super) passed: bool,
    pub(super) message: String,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgePublicEvidenceCheck {
    pub(super) route: String,
    pub(super) artifacts: Vec<String>,
    pub(super) passed: bool,
    pub(super) message: String,
}

pub(super) fn run_forge_public_evidence(cwd: &Path, args: &[String]) -> DxResult<()> {
    let DxForgePublicEvidenceCommandOptions {
        project,
        output,
        verify,
        format,
        fail_under,
        no_fail_under,
        quiet,
    } = parse_forge_public_evidence_options(cwd, args)?;

    if let Some(evidence_dir) = verify {
        if project.is_some() {
            return Err(DxError::ConfigValidationError {
                message: "--project cannot be used with --verify".to_string(),
                field: Some("forge public-evidence".to_string()),
            });
        }

        let report = verify_forge_public_evidence_export(&evidence_dir).map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal => forge_public_evidence_verification_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_public_evidence_verification_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, rendered).map_err(forge_error)?;
        } else if !quiet {
            print!("{rendered}");
        }

        let minimum = if no_fail_under {
            None
        } else {
            Some(fail_under.unwrap_or(90))
        };
        if let Some(minimum) = minimum {
            if report.score < minimum {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Forge public evidence score {} is below required threshold {minimum}",
                        report.score
                    ),
                    field: Some("fail-under".to_string()),
                });
            }
        }
        if !report.passed {
            return Err(DxError::InternalError {
                message: forge_public_evidence_verification_failure_summary(&report),
            });
        }

        return Ok(());
    }

    let scorecard = if let Some(project) = project {
        build_forge_package_scorecard_for_project(&project).map_err(forge_error)?
    } else {
        build_forge_package_scorecard().map_err(forge_error)?
    };
    let report = build_forge_public_evidence_report(&scorecard);
    let rendered = match format {
        DxOutputFormat::Terminal => forge_public_evidence_terminal(&report),
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => forge_public_evidence_markdown(&report),
    };

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent).map_err(forge_error)?;
        }
        std::fs::write(&output, rendered).map_err(forge_error)?;
    } else if !quiet {
        println!("{rendered}");
    }

    Ok(())
}

pub(super) fn build_forge_public_evidence_report(
    scorecard: &DxForgePackageScorecardReport,
) -> DxForgePublicEvidenceReport {
    let verified_packages = scorecard
        .packages
        .iter()
        .filter(|package| package.integrity_verified)
        .count();
    let source_owned_packages = scorecard
        .packages
        .iter()
        .filter(|package| package.source_owned)
        .count();
    let node_modules_packages = scorecard
        .packages
        .iter()
        .filter(|package| package.node_modules_created)
        .count();

    DxForgePublicEvidenceReport {
        version: 1,
        route: "/forge/evidence".to_string(),
        generated_at: scorecard.generated_at.clone(),
        score: scorecard.score,
        package_count: scorecard.packages.len(),
        verified_packages,
        source_owned_packages,
        node_modules_packages,
        links: forge_public_evidence_links(),
    }
}

pub(super) fn forge_public_evidence_links() -> Vec<DxForgePublicEvidenceLink> {
    vec![
        DxForgePublicEvidenceLink {
            section: "Routes",
            label: "Launch page",
            href: "forge.html",
            source_model: "DxForgeReleaseEvidenceReport",
            description: "The compact public /forge route with launch claims, package cards, and budget evidence.",
        },
        DxForgePublicEvidenceLink {
            section: "Routes",
            label: "Package scorecard",
            href: "forge/scorecard.html",
            source_model: "DxForgePackageScorecardReport",
            description: "The public /forge/scorecard route rendered from the package scorecard model.",
        },
        DxForgePublicEvidenceLink {
            section: "Routes",
            label: "CI evidence route",
            href: "forge/ci.html",
            source_model: "DxForgeSmokeReport + DxForgeReadinessBadge",
            description: "The public /forge/ci route rendered from secret-free CI smoke and readiness evidence.",
        },
        DxForgePublicEvidenceLink {
            section: "Badges",
            label: "Readiness badge",
            href: "forge-readiness-badge.json",
            source_model: "DxForgeReadinessBadge",
            description: "The compact release-readiness badge consumed by CI summaries and public status pages.",
        },
        DxForgePublicEvidenceLink {
            section: "Claims",
            label: "Launch claims",
            href: "forge.claims.json",
            source_model: "DxForgeLaunchClaimsManifest",
            description: "The machine-readable claim map for the public /forge route.",
        },
        DxForgePublicEvidenceLink {
            section: "Claims",
            label: "Public evidence claims",
            href: "forge/evidence.claims.json",
            source_model: "DxForgeLaunchClaimsManifest",
            description: "The machine-readable claim map for this /forge/evidence index.",
        },
        DxForgePublicEvidenceLink {
            section: "Evidence",
            label: "Launch evidence model",
            href: "forge.evidence.json",
            source_model: "DxForgeLaunchEvidenceManifest",
            description: "The package, provenance, advisory, license, and benchmark evidence backing /forge.",
        },
        DxForgePublicEvidenceLink {
            section: "Benchmarks",
            label: "Public route comparison",
            href: "forge-public-route-comparison.md",
            source_model: "benchmarks/reports/forge-public-route-comparison.md",
            description: "The current compact-route comparison for /forge, /forge/scorecard, /forge/ci, /forge/evidence, and /forge/releases.",
        },
        DxForgePublicEvidenceLink {
            section: "Benchmarks",
            label: "Launch delivery comparison",
            href: "forge-launch-delivery-comparison.md",
            source_model: "benchmarks/reports/forge-launch-delivery-comparison.md",
            description: "The static /forge delivery comparison against the earlier DXPK runtime delivery.",
        },
    ]
}

pub(super) fn verify_forge_public_evidence_export(
    evidence_dir: &Path,
) -> anyhow::Result<DxForgePublicEvidenceVerificationReport> {
    let links = forge_public_evidence_links();
    let mut required = BTreeSet::<String>::new();
    for artifact in ["forge/evidence.html", "forge/evidence.dxp", "proof.json"] {
        required.insert(artifact.to_string());
    }
    for link in &links {
        required.insert(link.href.to_string());
    }

    let mut findings = Vec::new();
    let mut artifacts = Vec::new();
    let mut json_values = BTreeMap::<String, serde_json::Value>::new();
    let mut penalty = 0u16;

    if !evidence_dir.is_dir() {
        findings.push(format!(
            "Public evidence directory does not exist: {}",
            evidence_dir.display()
        ));
        penalty = 100;
    }

    for link in &links {
        if public_evidence_href_unsafe(link.href) {
            findings.push(format!("unsafe public evidence href `{}`", link.href));
            penalty = penalty.saturating_add(12);
        }
    }

    for name in required {
        let path = public_evidence_artifact_path(evidence_dir, &name);
        let exists = path.is_file();
        let bytes = path.metadata().map(|metadata| metadata.len()).unwrap_or(0);
        let mut messages = Vec::new();
        let mut valid_json = None;

        if !exists {
            messages.push("missing".to_string());
            penalty = penalty.saturating_add(10);
        } else {
            if bytes == 0 {
                messages.push("empty".to_string());
                penalty = penalty.saturating_add(10);
            }

            if name.ends_with(".json") {
                valid_json = Some(false);
                match std::fs::read(&path) {
                    Ok(raw) => match serde_json::from_slice::<serde_json::Value>(&raw) {
                        Ok(value) => {
                            valid_json = Some(true);
                            json_values.insert(name.clone(), value);
                        }
                        Err(error) => {
                            messages.push(format!("invalid json: {error}"));
                            penalty = penalty.saturating_add(10);
                        }
                    },
                    Err(error) => {
                        messages.push(format!("unreadable: {error}"));
                        penalty = penalty.saturating_add(10);
                    }
                }
            }

            if name.ends_with(".dxp") {
                match std::fs::read(&path) {
                    Ok(raw) if raw.starts_with(b"DXPK") => {}
                    Ok(_) => {
                        messages.push("missing DXPK header".to_string());
                        penalty = penalty.saturating_add(10);
                    }
                    Err(error) => {
                        messages.push(format!("unreadable DXPK artifact: {error}"));
                        penalty = penalty.saturating_add(10);
                    }
                }
            }
        }

        let passed = messages.is_empty();
        let message = if passed {
            "ok".to_string()
        } else {
            messages.join("; ")
        };
        if !passed {
            findings.push(format!("{name}: {message}"));
        }
        artifacts.push(DxForgePublicEvidenceArtifactCheck {
            name,
            path,
            exists,
            bytes,
            valid_json,
            passed,
            message,
        });
    }

    let mut checks = forge_public_evidence_export_checks(
        evidence_dir,
        &links,
        &json_values,
        &mut findings,
        &mut penalty,
    );
    checks.extend(forge_public_evidence_secret_marker_checks(
        evidence_dir,
        &mut findings,
        &mut penalty,
    ));

    let score = 100u8.saturating_sub(penalty.min(100) as u8);
    let passed = findings.is_empty();

    Ok(DxForgePublicEvidenceVerificationReport {
        evidence_dir: evidence_dir.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        artifacts,
        checks,
        findings,
    })
}

fn forge_public_evidence_export_checks(
    evidence_dir: &Path,
    links: &[DxForgePublicEvidenceLink],
    json_values: &BTreeMap<String, serde_json::Value>,
    findings: &mut Vec<String>,
    penalty: &mut u16,
) -> Vec<DxForgePublicEvidenceCheck> {
    let mut checks = Vec::new();
    let html_path = public_evidence_artifact_path(evidence_dir, "forge/evidence.html");
    let html_hrefs = std::fs::read_to_string(&html_path)
        .map(|html| html_href_values(&html))
        .unwrap_or_default();
    let missing_hrefs = links
        .iter()
        .filter(|link| !html_hrefs.iter().any(|href| href == link.href))
        .map(|link| link.href)
        .collect::<Vec<_>>();
    checks.push(public_evidence_named_check(
        "/forge/evidence links",
        vec!["forge/evidence.html"],
        missing_hrefs.is_empty(),
        "public evidence HTML links every expected artifact",
        &format!(
            "missing public evidence links: {}",
            missing_hrefs.join(", ")
        ),
        findings,
        penalty,
    ));

    checks.push(public_evidence_named_check(
        "/forge/evidence claims",
        vec!["forge/evidence.claims.json"],
        json_route_value(json_values, "forge/evidence.claims.json") == Some("/forge/evidence")
            && json_claim_statuses_valid(json_values, "forge/evidence.claims.json"),
        "claims manifest targets /forge/evidence and has reviewable statuses",
        "forge/evidence.claims.json must target /forge/evidence and use reviewable claim statuses",
        findings,
        penalty,
    ));

    checks.push(public_evidence_named_check(
        "/forge/evidence proof",
        vec!["proof.json"],
        json_route_value(json_values, "proof.json") == Some("/forge/evidence")
            && json_string_contains(json_values, "proof.json", "forge/evidence.html")
            && json_string_contains(json_values, "proof.json", "forge/evidence.dxp"),
        "proof summary targets /forge/evidence and references HTML plus DXPK",
        "proof.json must target /forge/evidence and reference forge/evidence.html plus forge/evidence.dxp",
        findings,
        penalty,
    ));

    checks.push(public_evidence_named_check(
        "readiness badge",
        vec!["forge-readiness-badge.json"],
        json_bool(json_values, "forge-readiness-badge.json", "passed")
            && json_bool(json_values, "forge-readiness-badge.json", "no_node_modules"),
        "badge reports passed and no node_modules",
        "forge-readiness-badge.json must report passed=true and no_node_modules=true",
        findings,
        penalty,
    ));

    let route_comparison_path =
        public_evidence_artifact_path(evidence_dir, "forge-public-route-comparison.md");
    let route_comparison = std::fs::read_to_string(route_comparison_path).unwrap_or_default();
    let route_comparison_has_routes = [
        "/forge",
        "/forge/scorecard",
        "/forge/ci",
        "/forge/evidence",
        "/forge/releases",
    ]
    .iter()
    .all(|route| route_comparison.contains(route));
    checks.push(public_evidence_named_check(
        "public route comparison",
        vec!["forge-public-route-comparison.md"],
        route_comparison_has_routes,
        "route comparison covers all public Forge routes",
        "forge-public-route-comparison.md must mention /forge, /forge/scorecard, /forge/ci, /forge/evidence, and /forge/releases",
        findings,
        penalty,
    ));

    checks.push(public_evidence_named_check(
        "dependency boundary",
        vec!["node_modules"],
        !public_tree_contains_directory(evidence_dir, "node_modules"),
        "no node_modules directory in public evidence export",
        "public evidence export must not include node_modules",
        findings,
        penalty,
    ));

    checks
}

fn public_evidence_named_check(
    route: &str,
    artifacts: Vec<&str>,
    passed: bool,
    ok_message: &str,
    failed_message: &str,
    findings: &mut Vec<String>,
    penalty: &mut u16,
) -> DxForgePublicEvidenceCheck {
    let message = if passed {
        ok_message.to_string()
    } else {
        findings.push(format!("{route}: {failed_message}"));
        *penalty = penalty.saturating_add(10);
        failed_message.to_string()
    };

    DxForgePublicEvidenceCheck {
        route: route.to_string(),
        artifacts: artifacts.into_iter().map(str::to_string).collect(),
        passed,
        message,
    }
}

fn public_evidence_artifact_path(root: &Path, relative: &str) -> PathBuf {
    relative
        .split('/')
        .fold(root.to_path_buf(), |path, segment| path.join(segment))
}

fn public_evidence_href_unsafe(href: &str) -> bool {
    let href = href.trim();
    href.is_empty()
        || href.contains("..")
        || href.contains('\\')
        || href.contains(':')
        || href.starts_with('/')
}

fn public_tree_contains_directory(root: &Path, directory_name: &str) -> bool {
    let mut pending = vec![root.to_path_buf()];
    while let Some(current) = pending.pop() {
        let Ok(entries) = std::fs::read_dir(&current) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            if path.file_name().and_then(|name| name.to_str()) == Some(directory_name) {
                return true;
            }
            pending.push(path);
        }
    }
    false
}

fn forge_public_evidence_secret_marker_checks(
    evidence_dir: &Path,
    findings: &mut Vec<String>,
    penalty: &mut u16,
) -> Vec<DxForgePublicEvidenceCheck> {
    let mut leaked = Vec::new();
    let mut pending = vec![evidence_dir.to_path_buf()];

    while let Some(current) = pending.pop() {
        let entries = match std::fs::read_dir(&current) {
            Ok(entries) => entries,
            Err(error) => {
                findings.push(format!(
                    "secret scan could not read {}: {error}",
                    current.display()
                ));
                *penalty = penalty.saturating_add(8);
                continue;
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    findings.push(format!("secret scan entry error: {error}"));
                    *penalty = penalty.saturating_add(8);
                    continue;
                }
            };
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
                continue;
            }

            let raw = match std::fs::read(&path) {
                Ok(raw) => raw,
                Err(error) => {
                    findings.push(format!(
                        "secret scan could not read {}: {error}",
                        path.display()
                    ));
                    *penalty = penalty.saturating_add(8);
                    continue;
                }
            };
            let text = String::from_utf8_lossy(&raw);
            for marker in FORGE_PUBLIC_SECRET_MARKERS {
                if text.contains(marker) {
                    leaked.push(format!(
                        "{} contains {marker}",
                        path.strip_prefix(evidence_dir).unwrap_or(&path).display()
                    ));
                }
            }
        }
    }

    let passed = leaked.is_empty();
    let message = if passed {
        "no secret markers found".to_string()
    } else {
        *penalty = penalty.saturating_add(20);
        findings.extend(
            leaked
                .iter()
                .map(|finding| format!("secret marker: {finding}")),
        );
        leaked.join("; ")
    };

    vec![DxForgePublicEvidenceCheck {
        route: "secret-free public evidence".to_string(),
        artifacts: vec!["public evidence export".to_string()],
        passed,
        message,
    }]
}

const FORGE_PUBLIC_SECRET_MARKERS: &[&str] = &[
    "CLOUDFLARE_R2_",
    "DX_FORGE_R2_LIVE",
    "R2_SECRET",
    "SECRET_ACCESS_KEY",
];

fn json_bool(json_values: &BTreeMap<String, serde_json::Value>, artifact: &str, key: &str) -> bool {
    json_values
        .get(artifact)
        .and_then(|value| value.get(key))
        .and_then(|value| value.as_bool())
        == Some(true)
}

fn json_route_value<'a>(
    json_values: &'a BTreeMap<String, serde_json::Value>,
    artifact: &str,
) -> Option<&'a str> {
    json_values
        .get(artifact)
        .and_then(|value| value.get("route"))
        .and_then(|value| value.as_str())
}

fn json_claim_statuses_valid(
    json_values: &BTreeMap<String, serde_json::Value>,
    artifact: &str,
) -> bool {
    json_values
        .get(artifact)
        .and_then(|value| value.get("claims"))
        .and_then(|claims| claims.as_array())
        .is_some_and(|claims| {
            !claims.is_empty()
                && claims.iter().all(|claim| {
                    matches!(
                        claim
                            .get("verification_status")
                            .and_then(|status| status.as_str()),
                        Some("verified" | "declared" | "needs-review" | "pending")
                    )
                })
        })
}

fn json_string_contains(
    json_values: &BTreeMap<String, serde_json::Value>,
    artifact: &str,
    needle: &str,
) -> bool {
    json_values.get(artifact).is_some_and(|value| {
        let normalized = value.to_string().replace("\\\\", "/").replace('\\', "/");
        normalized.contains(needle)
    })
}

fn html_href_values(html: &str) -> Vec<String> {
    let mut hrefs = Vec::new();
    let mut rest = html;
    while let Some(index) = rest.find("href=\"") {
        rest = &rest[index + "href=\"".len()..];
        let Some(end) = rest.find('"') else {
            break;
        };
        hrefs.push(rest[..end].to_string());
        rest = &rest[end + 1..];
    }
    hrefs
}

pub(super) fn forge_public_evidence_terminal(report: &DxForgePublicEvidenceReport) -> String {
    let mut output = format!(
        "DX Forge Public Evidence\nRoute: {}\nGenerated: {}\nScore: {} / 100\nPackages: {}\nVerified: {}\nSource-owned: {}\nnode_modules packages: {}\nArtifacts: {}\n\nLinks:\n",
        report.route,
        report.generated_at,
        report.score,
        report.package_count,
        report.verified_packages,
        report.source_owned_packages,
        report.node_modules_packages,
        report.links.len()
    );
    for link in &report.links {
        output.push_str(&format!(
            "- [{}] {} -> {} ({})\n  {}\n",
            link.section, link.label, link.href, link.source_model, link.description
        ));
    }
    output
}

pub(super) fn forge_public_evidence_markdown(report: &DxForgePublicEvidenceReport) -> String {
    let mut output = format!(
        "# DX Forge Public Evidence\n\n- Route: `{}`\n- Generated: `{}`\n- Score: `{}` / `100`\n- Packages: `{}`\n- Verified packages: `{}`\n- Source-owned packages: `{}`\n- `node_modules` packages: `{}`\n- Public artifacts: `{}`\n\n",
        report.route,
        report.generated_at,
        report.score,
        report.package_count,
        report.verified_packages,
        report.source_owned_packages,
        report.node_modules_packages,
        report.links.len()
    );
    output.push_str("| Section | Artifact | Model | Description |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for link in &report.links {
        output.push_str(&format!(
            "| {} | [`{}`]({}) | `{}` | {} |\n",
            markdown_table_cell(link.section),
            markdown_table_cell(link.href),
            markdown_link_target(link.href),
            markdown_table_cell(link.source_model),
            markdown_table_cell(link.description)
        ));
    }
    output
}

pub(super) fn forge_public_evidence_verification_terminal(
    report: &DxForgePublicEvidenceVerificationReport,
) -> String {
    let mut output = format!(
        "DX Forge public evidence verification\nDirectory: {}\nPassed: {}\nScore: {}\nArtifacts: {}/{}\nChecks: {}/{}\n",
        report.evidence_dir.display(),
        report.passed,
        report.score,
        report
            .artifacts
            .iter()
            .filter(|artifact| artifact.passed)
            .count(),
        report.artifacts.len(),
        report.checks.iter().filter(|check| check.passed).count(),
        report.checks.len()
    );
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

pub(super) fn forge_public_evidence_verification_markdown(
    report: &DxForgePublicEvidenceVerificationReport,
) -> String {
    let mut output = format!(
        "# DX Forge Public Evidence Verification\n\n- Evidence directory: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}`\n\n",
        report.evidence_dir.display(),
        report.generated_at,
        report.passed,
        report.score
    );

    output.push_str("## Artifact Checks\n\n");
    output.push_str("| Artifact | Link | Bytes | Status |\n");
    output.push_str("| --- | --- | ---: | --- |\n");
    for artifact in &report.artifacts {
        output.push_str(&format!(
            "| `{}` | {} | {} | {} |\n",
            artifact.name,
            markdown_artifact_link(&artifact.name),
            artifact.bytes,
            artifact.message
        ));
    }

    output.push_str("\n## Public Evidence Checks\n\n");
    output.push_str("| Check | Artifacts | Status |\n");
    output.push_str("| --- | --- | --- |\n");
    for check in &report.checks {
        let links = check
            .artifacts
            .iter()
            .map(|artifact| markdown_artifact_link(artifact))
            .collect::<Vec<_>>()
            .join(", ");
        output.push_str(&format!(
            "| `{}` | {} | {} |\n",
            check.route, links, check.message
        ));
    }

    if !report.findings.is_empty() {
        output.push_str("\n## Findings\n\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output
}

pub(super) fn forge_public_evidence_verification_failure_summary(
    report: &DxForgePublicEvidenceVerificationReport,
) -> String {
    if report.findings.is_empty() {
        return format!(
            "Forge public evidence verification failed with score {}",
            report.score
        );
    }
    format!(
        "Forge public evidence verification failed: {}",
        report.findings.join("; ")
    )
}

fn markdown_table_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}

fn markdown_link_target(value: &str) -> String {
    value.replace(')', "%29").replace(' ', "%20")
}

fn markdown_artifact_link(relative: &str) -> String {
    let href = relative.replace(' ', "%20");
    format!("[`{relative}`]({href})")
}
