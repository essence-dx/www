//! Forge trust-policy reporting for source-owned package governance.

use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use super::forge_registry::{registry_package, verify_registry_package_integrity};
use super::forge_security::DxUpdateTraffic;

const TRUST_POLICY_PATH: &str = ".dx/forge/trust-policy.json";
const LAUNCH_PACKAGES: [&str; 32] = [
    "shadcn/ui/button",
    "shadcn/ui/badge",
    "shadcn/ui/card",
    "shadcn/ui/alert",
    "shadcn/ui/avatar",
    "shadcn/ui/skeleton",
    "shadcn/ui/label",
    "shadcn/ui/separator",
    "shadcn/ui/field",
    "shadcn/ui/item",
    "shadcn/ui/input",
    "shadcn/ui/textarea",
    "dx/icon/search",
    "auth/better-auth",
    "animation/motion",
    "i18n/next-intl",
    "tanstack/query",
    "validation/zod",
    "forms/react-hook-form",
    "payments/stripe-js",
    "automations/n8n",
    "state/zustand",
    "ai/vercel-ai",
    "api/trpc",
    "content/fumadocs-next",
    "content/react-markdown",
    "supabase/client",
    "db/drizzle-sqlite",
    "instantdb/react",
    "wasm/bindgen",
    "3d/launch-scene",
    "migration/static-site",
];

/// Reviewable Forge trust-policy report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeTrustPolicyReport {
    /// RFC3339 generation timestamp.
    pub generated_at: String,
    /// Checked project root.
    pub project: PathBuf,
    /// Overall trust-policy score from 0 to 100.
    pub score: u8,
    /// Overall traffic-light result.
    pub traffic: DxUpdateTraffic,
    /// Project policy file path.
    pub policy_file: PathBuf,
    /// Whether the policy file exists.
    pub policy_file_present: bool,
    /// Whether the policy file matches the current generated policy.
    pub policy_file_matches_current: bool,
    /// BLAKE3 hash of the policy file when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_file_hash: Option<String>,
    /// Current generated policy.
    pub policy: DxForgeTrustPolicy,
    /// Package rows included in the policy report.
    pub packages: Vec<DxForgeTrustPolicyPackage>,
    /// Policy findings that should stay visible to release reviewers.
    pub findings: Vec<DxForgeTrustPolicyFinding>,
    /// Honest boundaries for public launch claims.
    pub honest_boundaries: Vec<String>,
}

/// Stable policy file written under `.dx/forge/trust-policy.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeTrustPolicy {
    /// Policy schema version.
    pub version: u32,
    /// Allowed source-owned packages for the current launch wedge.
    pub allowed_packages: Vec<DxForgeAllowedPackage>,
    /// Blocked install-time and persistence shapes.
    pub blocked_shapes: Vec<DxForgeBlockedShape>,
    /// Advisory handling policy for v1.
    pub advisory_policy: String,
    /// License handling policy for v1.
    pub license_review_policy: String,
    /// Responsibilities that stay with the application/package owner.
    pub package_owner_responsibilities: Vec<String>,
}

/// One allowed package in the trust policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeAllowedPackage {
    /// Canonical Forge package id.
    pub package_id: String,
    /// Accepted aliases.
    pub aliases: Vec<String>,
    /// Curated registry version.
    pub version: String,
    /// Source language segment.
    pub language: String,
    /// Registry source kind.
    pub source: String,
    /// License declaration.
    pub license: String,
}

/// A blocked supply-chain shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeBlockedShape {
    /// Stable policy code.
    pub code: String,
    /// Shape pattern or manifest key.
    pub pattern: String,
    /// Why Forge blocks this shape by default.
    pub reason: String,
    /// Recommended remediation.
    pub remediation: String,
}

/// Package evidence row for a trust-policy report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeTrustPolicyPackage {
    /// Canonical Forge package id.
    pub package_id: String,
    /// Curated registry version.
    pub version: String,
    /// License declaration.
    pub license: String,
    /// Whether registry integrity verified.
    pub integrity_verified: bool,
    /// Whether live advisory coverage is attached.
    pub advisory_live_coverage: bool,
    /// Advisory coverage kind.
    pub advisory_coverage_kind: String,
    /// Advisory provider name.
    pub advisory_provider: String,
    /// Advisory finding count currently attached.
    pub advisory_findings: u64,
    /// Whether formal license review is recorded.
    pub license_reviewed: bool,
    /// Whether external provenance is verified.
    pub provenance_verified: bool,
    /// Source-owned files represented by this package.
    pub file_count: u64,
    /// Bytes represented by this package.
    pub total_bytes: u64,
    /// Owner responsibility for this package.
    pub owner_responsibility: String,
}

/// One trust-policy finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxForgeTrustPolicyFinding {
    /// Traffic-light severity.
    pub traffic: DxUpdateTraffic,
    /// Stable finding code.
    pub code: String,
    /// Human-readable finding message.
    pub message: String,
    /// Evidence pointer.
    pub evidence: String,
    /// Recommended remediation.
    pub remediation: String,
}

/// Build a Forge trust-policy report for a project.
pub fn build_forge_trust_policy_report(
    project: impl AsRef<Path>,
) -> Result<DxForgeTrustPolicyReport> {
    let project = project.as_ref();
    let policy = default_forge_trust_policy()?;
    let packages = trust_policy_packages()?;
    let policy_file = project.join(TRUST_POLICY_PATH);
    let file_status = read_policy_file_status(&policy_file, &policy);
    let mut findings = trust_policy_findings(&packages, &file_status);
    let score = trust_policy_score(&findings);
    let traffic = aggregate_trust_policy_traffic(&findings);

    findings.sort_by(|left, right| {
        trust_policy_traffic_rank(right.traffic)
            .cmp(&trust_policy_traffic_rank(left.traffic))
            .then_with(|| left.code.cmp(&right.code))
    });

    Ok(DxForgeTrustPolicyReport {
        generated_at: Utc::now().to_rfc3339(),
        project: project.to_path_buf(),
        score,
        traffic,
        policy_file,
        policy_file_present: file_status.present,
        policy_file_matches_current: file_status.matches_current,
        policy_file_hash: file_status.hash,
        policy,
        packages,
        findings,
        honest_boundaries: vec![
            "Forge trust policy currently covers the curated JS launch packages only."
                .to_string(),
            "The policy blocks install-time execution shapes by default, but it is not a full sandbox for arbitrary package ecosystems yet."
                .to_string(),
            "Advisory fields use curated fixtures today, not a live vulnerability feed; license fields remain declared-only until formal legal review is connected."
                .to_string(),
        ],
    })
}

/// Write the stable trust-policy file for a project.
pub fn write_forge_trust_policy_file(project: impl AsRef<Path>) -> Result<PathBuf> {
    let project = project.as_ref();
    let policy = default_forge_trust_policy()?;
    let policy_path = project.join(TRUST_POLICY_PATH);
    if let Some(parent) = policy_path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create `{}`", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(&policy).context("serialize Forge trust policy")?;
    fs::write(&policy_path, format!("{json}\n"))
        .with_context(|| format!("write `{}`", policy_path.display()))?;
    Ok(policy_path)
}

/// Render a Forge trust-policy report as Markdown.
pub fn forge_trust_policy_markdown(report: &DxForgeTrustPolicyReport) -> String {
    let mut output = format!(
        "# DX Forge Trust Policy\n\n- Score: `{}` / `100`\n- Traffic: `{}`\n- Generated: `{}`\n- Project: `{}`\n- Policy file: `{}`\n- Policy file present: `{}`\n- Policy file current: `{}`\n\n",
        report.score,
        report.traffic.as_str(),
        report.generated_at,
        report.project.display(),
        report.policy_file.display(),
        report.policy_file_present,
        report.policy_file_matches_current
    );

    output.push_str("## Allowed Packages\n\n");
    output.push_str("| Package | Version | License | Integrity | Advisory | License review | Provenance | Owner responsibility |\n");
    output.push_str("| --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for package in &report.packages {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} | {} `{}` provider `{}` | {} | {} | {} |\n",
            package.package_id,
            package.version,
            package.license,
            yes_no(package.integrity_verified),
            if package.advisory_live_coverage {
                "live"
            } else {
                "placeholder"
            },
            package.advisory_coverage_kind,
            package.advisory_provider,
            if package.license_reviewed {
                "reviewed"
            } else {
                "declared-only"
            },
            if package.provenance_verified {
                "verified"
            } else {
                "curated-record"
            },
            package.owner_responsibility
        ));
    }

    output.push_str("\n## Blocked Shapes\n\n");
    output.push_str("| Code | Pattern | Reason | Remediation |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for shape in &report.policy.blocked_shapes {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | {} |\n",
            shape.code, shape.pattern, shape.reason, shape.remediation
        ));
    }

    output.push_str("\n## Package Owner Responsibilities\n\n");
    for responsibility in &report.policy.package_owner_responsibilities {
        output.push_str(&format!("- {responsibility}\n"));
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- `green`: trust-policy report has no findings.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!(
                "- `{}` `{}`: {} Evidence: `{}`. Remediation: {}\n",
                finding.traffic.as_str(),
                finding.code,
                finding.message,
                finding.evidence,
                finding.remediation
            ));
        }
    }

    output.push_str("\n## Honest Boundaries\n\n");
    for boundary in &report.honest_boundaries {
        output.push_str(&format!("- {boundary}\n"));
    }

    output
}

fn default_forge_trust_policy() -> Result<DxForgeTrustPolicy> {
    let mut allowed_packages = Vec::new();
    for package_id in LAUNCH_PACKAGES {
        let package = registry_package(package_id)?;
        allowed_packages.push(DxForgeAllowedPackage {
            package_id: package.package_id,
            aliases: package.aliases,
            version: package.version,
            language: package.language.as_segment().to_string(),
            source: "curated-registry".to_string(),
            license: package.license,
        });
    }

    Ok(DxForgeTrustPolicy {
        version: 1,
        allowed_packages,
        blocked_shapes: blocked_shapes(),
        advisory_policy: "Do not claim live advisory coverage until a provider-backed feed is attached; keep curated advisory fixtures visible in reports.".to_string(),
        license_review_policy: "Treat declared licenses as package metadata only until formal DX license review is recorded.".to_string(),
        package_owner_responsibilities: vec![
            "Review yellow and red Forge update plans before accepting source-owned package changes.".to_string(),
            "Own application secrets, OAuth redirect policy, session storage, and deployment environment configuration.".to_string(),
            "Keep local edits reviewable through Forge receipts instead of replacing files blindly.".to_string(),
            "Run project security checks before publishing generated or curated source packages.".to_string(),
        ],
    })
}

fn trust_policy_packages() -> Result<Vec<DxForgeTrustPolicyPackage>> {
    let mut packages = Vec::new();
    for package_id in LAUNCH_PACKAGES {
        let package = registry_package(package_id)?;
        let integrity_verified = verify_registry_package_integrity(&package).is_ok();
        packages.push(DxForgeTrustPolicyPackage {
            package_id: package.package_id.clone(),
            version: package.version.clone(),
            license: package.license.clone(),
            integrity_verified,
            advisory_live_coverage: package.advisory_review.live_coverage,
            advisory_coverage_kind: package.advisory_review.coverage_kind.as_str().to_string(),
            advisory_provider: package.advisory_review.provider.clone(),
            advisory_findings: package.advisory_review.finding_count,
            license_reviewed: package.license_review.reviewed,
            provenance_verified: package.provenance.verified,
            file_count: package.files.len() as u64,
            total_bytes: package.files.iter().map(|file| file.bytes).sum(),
            owner_responsibility: package_owner_responsibility(&package.package_id).to_string(),
        });
    }
    Ok(packages)
}

fn blocked_shapes() -> Vec<DxForgeBlockedShape> {
    vec![
        blocked_shape(
            "lifecycle-preinstall",
            "scripts.preinstall",
            "Install-time package code can run before users inspect source.",
            "Materialize editable source files without running package lifecycle hooks.",
        ),
        blocked_shape(
            "lifecycle-install",
            "scripts.install",
            "Install scripts can execute arbitrary code on developer and CI machines.",
            "Replace install-time behavior with reviewed source files or explicit build steps.",
        ),
        blocked_shape(
            "lifecycle-postinstall",
            "scripts.postinstall",
            "Postinstall hooks are a common supply-chain execution path.",
            "Block package installation and require reviewable source materialization.",
        ),
        blocked_shape(
            "lifecycle-prepare",
            "scripts.prepare",
            "Prepare hooks can run from git dependencies and publish workflows.",
            "Require a reviewed package snapshot before Forge materialization.",
        ),
        blocked_shape(
            "dependency-git",
            "dependencies.* = git+ssh/git+https/github tarball",
            "Git dependencies can hide unreviewed commits and install-time behavior.",
            "Pin reviewed snapshots with file hashes and registry receipts.",
        ),
        blocked_shape(
            "obfuscated-large-js",
            "large minified or obfuscated JavaScript payload",
            "Large unreadable payloads are hard to review as source-owned files.",
            "Quarantine until the source, build recipe, and hashes are reviewable.",
        ),
        blocked_shape(
            "persistence-claude",
            ".claude/** writes",
            "Tool-persistence writes can silently alter developer agent behavior.",
            "Require explicit project-owner approval and a red Forge review plan.",
        ),
        blocked_shape(
            "persistence-vscode",
            ".vscode/** writes",
            "Editor persistence writes can alter tasks, settings, and developer execution.",
            "Require explicit project-owner approval and a red Forge review plan.",
        ),
        blocked_shape(
            "tanstack-router-init-ioc",
            "router_init.js plus git dependency or lifecycle hook",
            "This matches the known shape of credential-stealing supply-chain incidents.",
            "Block by default, rotate credentials if executed, and replace with a reviewed snapshot.",
        ),
    ]
}

fn blocked_shape(
    code: &str,
    pattern: &str,
    reason: &str,
    remediation: &str,
) -> DxForgeBlockedShape {
    DxForgeBlockedShape {
        code: code.to_string(),
        pattern: pattern.to_string(),
        reason: reason.to_string(),
        remediation: remediation.to_string(),
    }
}

fn package_owner_responsibility(package_id: &str) -> &'static str {
    match package_id {
        "auth/better-auth" => {
            "Application owner must configure secrets, redirect allowlists, trusted origins, sessions, database adapter, and production identity policy."
        }
        "animation/motion" => {
            "Application owner must install Motion/React, review animation performance, reduced-motion UX, and route-specific motion choices."
        }
        "i18n/next-intl" => {
            "Application owner must mount next-intl middleware/request config, own translated messages, review locale routing, and keep locale SEO policy current."
        }
        "supabase/client" => {
            "Application owner must install Supabase dependencies, configure Auth redirects, review RLS, and keep service-role secrets out of public client files."
        }
        "tanstack/query" => {
            "Application owner must install TanStack Query, choose cache policy per feature, and review server-prefetch data boundaries."
        }
        "validation/zod" => {
            "Application owner must install Zod, review accepted schemas, and keep validation separate from authorization policy."
        }
        "forms/react-hook-form" => {
            "Application owner must install React Hook Form, review form accessibility, own validation rules, and connect submit handlers to real application flows."
        }
        "payments/stripe-js" => {
            "Application owner must install Stripe.js, keep secret keys server-only, create payment intents or checkout sessions on the server, and review payment compliance policy."
        }
        "automations/n8n" => {
            "Application owner must approve credentials, choose connector scopes, review workflow execution policy, and keep run receipts redacted."
        }
        "state/zustand" => {
            "Application owner must keep sensitive data out of browser-local state, review persistence keys, install the optional Immer peer dependency when using draft updates, and decide when state belongs in durable storage."
        }
        "ai/vercel-ai" => {
            "Application owner must configure model providers, API keys, tool safety, rate limits, and streaming route policy."
        }
        "api/trpc" => {
            "Application owner must design procedures, enforce authorization, connect sessions, set request limits, review API route deployment policy, and own subscription fan-out, stream pacing, and EventSource/runtime policy."
        }
        "content/fumadocs-next" => {
            "Application owner must install Fumadocs dependencies, review Next config merges, own docs content, source plugin taxonomy, icon naming, status lifecycle, navigation policy, toc policy, slug/canonical URL policy, OpenAPI schema governance, OpenAPI proxy allowed origins, auth/cookie forwarding policy, request code sample policy, AI crawler exposure, private content exclusion, search UI, static-index payload budget, multilingual/vector policy, deployment policy, and route-level styling."
        }
        "content/react-markdown" => {
            "Application owner must install react-markdown, keep raw HTML disabled unless explicitly reviewed, choose plugins carefully, and review user-generated content/link policy."
        }
        "db/drizzle-sqlite" => {
            "Application owner must configure database file location, migrations, backups, permissions, and query access policy."
        }
        "instantdb/react" => {
            "Application owner must configure the Instant dashboard app, rules, auth policy, production schema, and public app id."
        }
        "3d/launch-scene" => {
            "Application owner must review GPU budget, reduced-motion behavior, shader changes, external 3D assets, and optional Three/R3F/Drei dependencies before release."
        }
        "migration/static-site" => {
            "Application owner must review imported HTML, media, redirects, metadata, and any dynamic WordPress behavior before release."
        }
        "shadcn/ui/button" => {
            "Application owner may edit component variants but must review yellow Forge update plans."
        }
        "shadcn/ui/badge" => {
            "Application owner may edit badge variants but must own status taxonomy, labels, tone, and accessibility."
        }
        "shadcn/ui/card" => {
            "Application owner may edit card composition styles but must review yellow Forge update plans."
        }
        "shadcn/ui/label" => {
            "Application owner may edit label styling but must preserve accessible names, descriptions, and validation relationships."
        }
        "shadcn/ui/separator" => {
            "Application owner may edit separator styling but must preserve information hierarchy and decorative-versus-semantic divider policy."
        }
        "shadcn/ui/field" => {
            "Application owner may edit field styling but must preserve accessible names, descriptions, validation relationships, and error announcement policy."
        }
        "shadcn/ui/item" => {
            "Application owner may edit item styling but must preserve list semantics, row actions, keyboard reachability, and row-level authorization."
        }
        "shadcn/ui/input" => {
            "Application owner may edit input styling but must preserve accessible form semantics and review yellow Forge update plans."
        }
        "shadcn/ui/textarea" => {
            "Application owner may edit textarea styling but must preserve accessible form semantics and review yellow Forge update plans."
        }
        "dx/icon/search" => {
            "Application owner may edit the local icon helper but must keep selected-icon provenance reviewable."
        }
        _ => "Application owner must review local edits and package updates before release.",
    }
}

fn trust_policy_findings(
    packages: &[DxForgeTrustPolicyPackage],
    file_status: &PolicyFileStatus,
) -> Vec<DxForgeTrustPolicyFinding> {
    let mut findings = Vec::new();
    if !file_status.present {
        findings.push(finding(
            DxUpdateTraffic::Yellow,
            "missing-trust-policy-file",
            "Project has no `.dx/forge/trust-policy.json` file yet.",
            TRUST_POLICY_PATH,
            "Run `dx forge trust-policy --write-policy` and review the generated file.",
        ));
    } else if !file_status.matches_current {
        findings.push(finding(
            DxUpdateTraffic::Yellow,
            "stale-trust-policy-file",
            "Project trust-policy file does not match the current generated policy.",
            TRUST_POLICY_PATH,
            "Regenerate with `dx forge trust-policy --write-policy` or review local policy changes.",
        ));
    }

    if packages.iter().any(|package| !package.integrity_verified) {
        findings.push(finding(
            DxUpdateTraffic::Red,
            "registry-integrity-failed",
            "At least one allowed package failed curated registry integrity verification.",
            "policy.allowed_packages",
            "Fix the package manifest or block the package before release.",
        ));
    }
    if packages
        .iter()
        .any(|package| package.advisory_coverage_kind == "missing")
    {
        findings.push(finding(
            DxUpdateTraffic::Red,
            "advisory-metadata-missing",
            "At least one allowed package has no advisory review metadata.",
            "packages[].advisory_review.coverage_kind",
            "Attach curated advisory fixture metadata or a live advisory provider before release.",
        ));
    } else if packages
        .iter()
        .any(|package| !package.advisory_live_coverage)
    {
        findings.push(finding(
            DxUpdateTraffic::Yellow,
            "advisory-fixture-no-live-feed",
            "Allowed packages have curated advisory fixtures, but no live vulnerability feed is connected yet.",
            "packages[].advisory_review",
            "Keep the fixture visible until live advisory metadata is connected.",
        ));
    }
    if packages.iter().any(|package| !package.license_reviewed) {
        findings.push(finding(
            DxUpdateTraffic::Yellow,
            "license-review-placeholder",
            "Allowed packages record declared licenses but no formal DX license review yet.",
            "packages[].license_review",
            "Attach reviewed license metadata before claiming formal legal coverage.",
        ));
    }
    if packages.iter().any(|package| !package.provenance_verified) {
        findings.push(finding(
            DxUpdateTraffic::Yellow,
            "provenance-placeholder",
            "Allowed packages have curated records but no external SLSA-style provenance yet.",
            "packages[].provenance",
            "Attach verifiable upstream provenance before expanding beyond curated packages.",
        ));
    }
    findings
}

fn finding(
    traffic: DxUpdateTraffic,
    code: &str,
    message: &str,
    evidence: &str,
    remediation: &str,
) -> DxForgeTrustPolicyFinding {
    DxForgeTrustPolicyFinding {
        traffic,
        code: code.to_string(),
        message: message.to_string(),
        evidence: evidence.to_string(),
        remediation: remediation.to_string(),
    }
}

#[derive(Debug)]
struct PolicyFileStatus {
    present: bool,
    matches_current: bool,
    hash: Option<String>,
}

fn read_policy_file_status(path: &Path, expected: &DxForgeTrustPolicy) -> PolicyFileStatus {
    let Ok(text) = fs::read_to_string(path) else {
        return PolicyFileStatus {
            present: false,
            matches_current: false,
            hash: None,
        };
    };
    let hash = Some(blake3::hash(text.as_bytes()).to_hex().to_string());
    let matches_current = serde_json::from_str::<DxForgeTrustPolicy>(&text)
        .map(|policy| policy == *expected)
        .unwrap_or(false);
    PolicyFileStatus {
        present: true,
        matches_current,
        hash,
    }
}

fn trust_policy_score(findings: &[DxForgeTrustPolicyFinding]) -> u8 {
    let mut score = 100u8;
    for finding in findings {
        score = score.saturating_sub(match finding.traffic {
            DxUpdateTraffic::Green => 0,
            DxUpdateTraffic::Yellow => match finding.code.as_str() {
                "missing-trust-policy-file" | "stale-trust-policy-file" => 5,
                "advisory-fixture-no-live-feed" => 4,
                "license-review-placeholder" => 3,
                "provenance-placeholder" => 3,
                _ => 5,
            },
            DxUpdateTraffic::Red => 35,
        });
    }
    score
}

fn aggregate_trust_policy_traffic(findings: &[DxForgeTrustPolicyFinding]) -> DxUpdateTraffic {
    if findings
        .iter()
        .any(|finding| finding.traffic == DxUpdateTraffic::Red)
    {
        DxUpdateTraffic::Red
    } else if findings
        .iter()
        .any(|finding| finding.traffic == DxUpdateTraffic::Yellow)
    {
        DxUpdateTraffic::Yellow
    } else {
        DxUpdateTraffic::Green
    }
}

fn trust_policy_traffic_rank(traffic: DxUpdateTraffic) -> u8 {
    match traffic {
        DxUpdateTraffic::Green => 0,
        DxUpdateTraffic::Yellow => 1,
        DxUpdateTraffic::Red => 2,
    }
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn trust_policy_covers_launch_packages_and_blocked_shapes() {
        let dir = tempdir().expect("tempdir");
        let report = build_forge_trust_policy_report(dir.path()).expect("report");

        assert_eq!(report.policy.allowed_packages.len(), LAUNCH_PACKAGES.len());
        assert!(report.score >= 80);
        assert_eq!(report.traffic, DxUpdateTraffic::Yellow);
        assert!(report.policy_file.ends_with(TRUST_POLICY_PATH));
        assert!(!report.policy_file_present);
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "shadcn/ui/button")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "shadcn/ui/badge")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "shadcn/ui/card")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "shadcn/ui/alert")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "shadcn/ui/avatar")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "shadcn/ui/skeleton")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "shadcn/ui/label")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "shadcn/ui/separator")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "shadcn/ui/field")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "shadcn/ui/item")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "auth/better-auth")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "animation/motion")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "tanstack/query")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "validation/zod")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "forms/react-hook-form")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "payments/stripe-js")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "automations/n8n")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "content/react-markdown")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "state/zustand")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "ai/vercel-ai")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "supabase/client")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "db/drizzle-sqlite")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "instantdb/react")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "3d/launch-scene")
        );
        assert!(
            report
                .policy
                .allowed_packages
                .iter()
                .any(|package| package.package_id == "migration/static-site")
        );
        assert!(
            report
                .policy
                .blocked_shapes
                .iter()
                .any(|shape| shape.code == "lifecycle-postinstall")
        );
        assert!(
            report
                .policy
                .blocked_shapes
                .iter()
                .any(|shape| shape.code == "dependency-git")
        );
        assert!(
            report
                .policy
                .blocked_shapes
                .iter()
                .any(|shape| shape.code == "tanstack-router-init-ioc")
        );
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.code == "missing-trust-policy-file")
        );
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.code == "advisory-fixture-no-live-feed")
        );
        assert!(report.packages.iter().all(|package| {
            package.advisory_coverage_kind == "curated-fixture"
                && package.advisory_provider == "dx-forge-curated-advisory-fixture"
                && !package.advisory_live_coverage
        }));
    }

    #[test]
    fn trust_policy_file_round_trips_as_current_policy() {
        let dir = tempdir().expect("tempdir");
        let policy_path = write_forge_trust_policy_file(dir.path()).expect("write policy");
        let report = build_forge_trust_policy_report(dir.path()).expect("report");

        assert!(policy_path.exists());
        assert!(report.policy_file_present);
        assert!(report.policy_file_matches_current);
        assert!(report.policy_file_hash.is_some());
        assert!(
            !report
                .findings
                .iter()
                .any(|finding| finding.code == "missing-trust-policy-file")
        );
    }

    #[test]
    fn trust_policy_markdown_explains_governance_boundaries() {
        let dir = tempdir().expect("tempdir");
        let report = build_forge_trust_policy_report(dir.path()).expect("report");
        let markdown = forge_trust_policy_markdown(&report);

        assert!(markdown.contains("Allowed Packages"));
        assert!(markdown.contains("Blocked Shapes"));
        assert!(markdown.contains("Package Owner Responsibilities"));
        assert!(markdown.contains("curated-fixture"));
        assert!(markdown.contains("dx-forge-curated-advisory-fixture"));
        assert!(markdown.contains("curated advisory fixtures"));
        assert!(markdown.contains("install-time execution"));
        assert!(markdown.contains("auth/better-auth"));
        assert!(markdown.contains("validation/zod"));
        assert!(markdown.contains("forms/react-hook-form"));
        assert!(markdown.contains("payments/stripe-js"));
        assert!(markdown.contains("content/react-markdown"));
        assert!(markdown.contains("state/zustand"));
        assert!(markdown.contains("instantdb/react"));
    }
}
