use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};
use dx_compiler::ecosystem::{
    DxForgePackageProjectEvidence, DxForgePackageScorecardEntry, DxForgePackageScorecardReport,
    build_forge_package_scorecard,
};
use serde::Serialize;

use super::{
    DxForgeAdoptionReport, DxForgeBenchmarkSnapshot, DxForgeReadinessBadge,
    DxForgeReleaseEvidenceReport, DxForgeSmokeReport, build_forge_adoption_report,
    build_forge_readiness_badge, build_forge_release_evidence_report, build_forge_smoke_report,
    forge_launch_changelog::{
        DxForgeLaunchChangelogInput, DxForgeLaunchChangelogReport,
        build_forge_launch_changelog_report,
    },
    forge_public_evidence::{DxForgePublicEvidenceReport, build_forge_public_evidence_report},
    forge_release_history::{DxForgePublicReleaseHistory, DxForgePublicReleaseRouteSnapshot},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DxVerticalProofFixture {
    Site,
    Scorecard,
    Ci,
    Evidence,
    Releases,
    Changelog,
    Quickstart,
    Adoption,
}

impl DxVerticalProofFixture {
    pub(super) fn parse(value: &str) -> DxResult<Self> {
        match value {
            "forge-site" => Ok(Self::Site),
            "forge-scorecard" => Ok(Self::Scorecard),
            "forge-ci" => Ok(Self::Ci),
            "forge-evidence" => Ok(Self::Evidence),
            "forge-releases" => Ok(Self::Releases),
            "forge-changelog" => Ok(Self::Changelog),
            "forge-quickstart" => Ok(Self::Quickstart),
            "forge-adoption" => Ok(Self::Adoption),
            other => Err(DxError::ConfigValidationError {
                message: format!("Unknown prove vertical fixture: {other}"),
                field: Some("prove vertical fixture".to_string()),
            }),
        }
    }

    pub(super) fn source(self) -> DxResult<DxVerticalFixtureSource> {
        match self {
            Self::Site => {
                let report = build_forge_package_scorecard().map_err(|error| {
                    DxError::ConfigValidationError {
                        message: format!("Could not build Forge launch fixture: {error}"),
                        field: Some("prove vertical fixture".to_string()),
                    }
                })?;
                Ok(DxVerticalFixtureSource {
                    route: "/forge".to_string(),
                    page_path: "pages/forge.html".to_string(),
                    page_source: forge_launch_preview_page_source(&report),
                    packages: forge_site_packages(),
                    claims_manifest: Some(forge_launch_preview_claims_manifest(&report)),
                    evidence_manifest: Some(forge_launch_preview_evidence_manifest(&report)),
                })
            }
            Self::Scorecard => {
                let report = build_forge_package_scorecard().map_err(|error| {
                    DxError::ConfigValidationError {
                        message: format!("Could not build Forge scorecard fixture: {error}"),
                        field: Some("prove vertical fixture".to_string()),
                    }
                })?;
                Ok(DxVerticalFixtureSource {
                    route: "/forge/scorecard".to_string(),
                    page_path: "pages/forge/scorecard.html".to_string(),
                    page_source: forge_scorecard_page_source(&report),
                    packages: Vec::new(),
                    claims_manifest: None,
                    evidence_manifest: None,
                })
            }
            Self::Ci => Ok(DxVerticalFixtureSource {
                route: "/forge/ci".to_string(),
                page_path: "pages/forge/ci.html".to_string(),
                page_source: forge_ci_preview_page_source(),
                packages: Vec::new(),
                claims_manifest: None,
                evidence_manifest: None,
            }),
            Self::Evidence => {
                let report = build_forge_package_scorecard().map_err(|error| {
                    DxError::ConfigValidationError {
                        message: format!("Could not build Forge evidence fixture: {error}"),
                        field: Some("prove vertical fixture".to_string()),
                    }
                })?;
                let evidence = build_forge_public_evidence_report(&report);
                Ok(DxVerticalFixtureSource {
                    route: "/forge/evidence".to_string(),
                    page_path: "pages/forge/evidence.html".to_string(),
                    page_source: forge_public_evidence_page_source(&evidence),
                    packages: Vec::new(),
                    claims_manifest: Some(forge_public_evidence_claims_manifest(&evidence)),
                    evidence_manifest: None,
                })
            }
            Self::Releases => {
                let history = empty_release_history();
                Ok(DxVerticalFixtureSource {
                    route: "/forge/releases".to_string(),
                    page_path: "pages/forge/releases.html".to_string(),
                    page_source: forge_releases_page_source(&history),
                    packages: Vec::new(),
                    claims_manifest: Some(forge_releases_claims_manifest(&history)),
                    evidence_manifest: None,
                })
            }
            Self::Changelog => {
                let report = preview_launch_changelog_report();
                Ok(DxVerticalFixtureSource {
                    route: "/forge/changelog".to_string(),
                    page_path: "pages/forge/changelog.html".to_string(),
                    page_source: forge_changelog_page_source(&report),
                    packages: Vec::new(),
                    claims_manifest: Some(forge_changelog_claims_manifest(&report)),
                    evidence_manifest: None,
                })
            }
            Self::Quickstart => Ok(DxVerticalFixtureSource {
                route: "/forge/quickstart".to_string(),
                page_path: "pages/forge/quickstart.html".to_string(),
                page_source: forge_quickstart_page_source(),
                packages: Vec::new(),
                claims_manifest: Some(forge_quickstart_claims_manifest()),
                evidence_manifest: None,
            }),
            Self::Adoption => Ok(DxVerticalFixtureSource {
                route: "/forge/adoption".to_string(),
                page_path: "pages/forge/adoption.html".to_string(),
                page_source: forge_adoption_preview_page_source(),
                packages: Vec::new(),
                claims_manifest: None,
                evidence_manifest: None,
            }),
        }
    }

    pub(super) fn source_for_project(
        self,
        project: &Path,
        benchmark_history_path: &Path,
    ) -> DxResult<DxVerticalFixtureSource> {
        match self {
            Self::Site => {
                let report =
                    match build_forge_release_evidence_report(project, benchmark_history_path) {
                        Ok(report) => report,
                        Err(error) => {
                            let message =
                                format!("Could not build Forge launch fixture evidence: {error}");
                            return Err(DxError::ConfigValidationError {
                                message,
                                field: Some("prove vertical fixture".to_string()),
                            });
                        }
                    };
                Ok(DxVerticalFixtureSource {
                    route: "/forge".to_string(),
                    page_path: "pages/forge.html".to_string(),
                    page_source: forge_launch_page_source(&report),
                    packages: forge_site_packages(),
                    claims_manifest: Some(forge_launch_claims_manifest(&report)),
                    evidence_manifest: Some(forge_launch_evidence_manifest(&report)),
                })
            }
            Self::Scorecard => self.source(),
            Self::Ci => {
                let smoke = build_forge_smoke_report(project).map_err(|error| {
                    DxError::ConfigValidationError {
                        message: format!("Could not build Forge CI fixture smoke report: {error}"),
                        field: Some("prove vertical fixture".to_string()),
                    }
                })?;
                let evidence = build_forge_release_evidence_report(
                    project,
                    &smoke.launch_artifacts.benchmark_history_path,
                )
                .map_err(|error| DxError::ConfigValidationError {
                    message: format!("Could not build Forge CI fixture evidence: {error}"),
                    field: Some("prove vertical fixture".to_string()),
                })?;
                let badge = build_forge_readiness_badge(&smoke, &evidence, None, 90);
                Ok(DxVerticalFixtureSource {
                    route: "/forge/ci".to_string(),
                    page_path: "pages/forge/ci.html".to_string(),
                    page_source: forge_ci_page_source(&smoke, &badge),
                    packages: Vec::new(),
                    claims_manifest: Some(forge_ci_claims_manifest(&smoke, &badge)),
                    evidence_manifest: None,
                })
            }
            Self::Evidence => self.source(),
            Self::Releases => {
                let history = read_release_history_for_project(project)?;
                Ok(DxVerticalFixtureSource {
                    route: "/forge/releases".to_string(),
                    page_path: "pages/forge/releases.html".to_string(),
                    page_source: forge_releases_page_source(&history),
                    packages: Vec::new(),
                    claims_manifest: Some(forge_releases_claims_manifest(&history)),
                    evidence_manifest: None,
                })
            }
            Self::Changelog => {
                let report = read_launch_changelog_for_project(project)?;
                Ok(DxVerticalFixtureSource {
                    route: "/forge/changelog".to_string(),
                    page_path: "pages/forge/changelog.html".to_string(),
                    page_source: forge_changelog_page_source(&report),
                    packages: Vec::new(),
                    claims_manifest: Some(forge_changelog_claims_manifest(&report)),
                    evidence_manifest: None,
                })
            }
            Self::Quickstart => self.source(),
            Self::Adoption => {
                let report = build_forge_adoption_report(project, None, 90).map_err(|error| {
                    DxError::ConfigValidationError {
                        message: format!("Could not build Forge adoption fixture report: {error}"),
                        field: Some("prove vertical fixture".to_string()),
                    }
                })?;
                Ok(DxVerticalFixtureSource {
                    route: "/forge/adoption".to_string(),
                    page_path: "pages/forge/adoption.html".to_string(),
                    page_source: forge_adoption_page_source(&report),
                    packages: Vec::new(),
                    claims_manifest: Some(forge_adoption_claims_manifest(&report)),
                    evidence_manifest: None,
                })
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct DxVerticalFixtureSource {
    pub(super) route: String,
    pub(super) page_path: String,
    pub(super) page_source: String,
    pub(super) packages: Vec<String>,
    pub(super) claims_manifest: Option<DxForgeLaunchClaimsManifest>,
    pub(super) evidence_manifest: Option<DxForgeLaunchEvidenceManifest>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeLaunchClaimsManifest {
    version: u32,
    route: String,
    generated_at: String,
    claims: Vec<DxForgeLaunchClaim>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeLaunchClaim {
    id: String,
    claim: String,
    source_model: String,
    source_field: String,
    verification_status: String,
    evidence: String,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeLaunchEvidenceManifest {
    version: u32,
    route: String,
    generated_at: String,
    summary: DxForgeLaunchEvidenceSummary,
    packages: Vec<DxForgeLaunchPackageEvidence>,
    honest_boundaries: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeLaunchEvidenceSummary {
    package_score: u8,
    package_count: usize,
    verified_packages: usize,
    source_owned_packages: usize,
    node_modules_packages: usize,
    benchmark_summary: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeLaunchPackageEvidence {
    package_id: String,
    description: String,
    version: String,
    language: String,
    license: String,
    file_count: u64,
    total_bytes: u64,
    public_claim: String,
    launch_boundary: String,
    provenance_source: String,
    provenance_verified: bool,
    provenance_note: String,
    advisory_provider: String,
    advisory_live_coverage: bool,
    advisory_findings: u64,
    advisory_note: String,
    declared_license: String,
    license_reviewed: bool,
    license_note: String,
    local_evidence: String,
}

fn forge_site_packages() -> Vec<String> {
    vec!["ui/button".to_string(), "icon/search".to_string()]
}

fn forge_launch_preview_page_source(report: &DxForgePackageScorecardReport) -> String {
    forge_launch_page_source_inner(ForgeLaunchPageFacts {
        generated_at: escape_page_text(&report.generated_at),
        release_gate: "Preview - release proof is generated in write mode".to_string(),
        check_score: "n/a".to_string(),
        check_traffic: "n/a".to_string(),
        launch_findings: "n/a".to_string(),
        rollback_coverage: "n/a".to_string(),
        docs_coverage: "n/a".to_string(),
        benchmark_summary: "Benchmark evidence is attached when release proof is available."
            .to_string(),
        scorecard: report,
    })
}

fn forge_launch_page_source(report: &DxForgeReleaseEvidenceReport) -> String {
    let release_gate = if report.passed {
        "Passed"
    } else {
        "Needs review"
    };
    forge_launch_page_source_inner(ForgeLaunchPageFacts {
        generated_at: escape_page_text(&report.generated_at),
        release_gate: release_gate.to_string(),
        check_score: report.check_score.to_string(),
        check_traffic: escape_page_text(&report.check_traffic),
        launch_findings: report.launch_gate_findings.len().to_string(),
        rollback_coverage: format!("{}%", report.rollback_coverage_percent),
        docs_coverage: format!("{}%", report.package_docs_coverage_percent),
        benchmark_summary: benchmark_summary(report.latest_benchmark.as_ref()),
        scorecard: &report.package_scorecard,
    })
}

fn forge_launch_preview_claims_manifest(
    scorecard: &DxForgePackageScorecardReport,
) -> DxForgeLaunchClaimsManifest {
    let mut claims = scorecard_claims(scorecard);
    claims.push(launch_claim(
        "benchmark-evidence",
        "Benchmark evidence is attached when release proof is available.",
        "DxForgeReleaseEvidenceReport",
        "latest_benchmark",
        "pending",
        "Preview mode has no release-proof benchmark snapshot attached yet.",
    ));

    DxForgeLaunchClaimsManifest {
        version: 1,
        route: "/forge".to_string(),
        generated_at: scorecard.generated_at.clone(),
        claims,
    }
}

fn forge_launch_preview_evidence_manifest(
    scorecard: &DxForgePackageScorecardReport,
) -> DxForgeLaunchEvidenceManifest {
    forge_launch_evidence_manifest_inner(
        scorecard,
        scorecard.generated_at.clone(),
        "Benchmark evidence is attached when release proof is available.".to_string(),
    )
}

fn forge_launch_claims_manifest(
    report: &DxForgeReleaseEvidenceReport,
) -> DxForgeLaunchClaimsManifest {
    let scorecard = &report.package_scorecard;
    let mut claims = Vec::new();
    claims.push(launch_claim(
        "release-gate",
        format!(
            "Release gate is `{}` with DX check {} / 100, traffic `{}`, and {} launch findings.",
            if report.passed {
                "Passed"
            } else {
                "Needs review"
            },
            report.check_score,
            report.check_traffic,
            report.launch_gate_findings.len()
        ),
        "DxForgeReleaseEvidenceReport",
        "passed,check_score,check_traffic,launch_gate_findings",
        if report.passed {
            "verified"
        } else {
            "needs-review"
        },
        format!(
            "rollback_coverage={}%, package_docs_coverage={}%",
            report.rollback_coverage_percent, report.package_docs_coverage_percent
        ),
    ));
    claims.push(launch_claim(
        "benchmark-evidence",
        benchmark_summary(report.latest_benchmark.as_ref()),
        "DxForgeReleaseEvidenceReport",
        "latest_benchmark",
        if report
            .latest_benchmark
            .as_ref()
            .is_some_and(forge_benchmark_snapshot_is_release_ready)
        {
            "verified"
        } else {
            "needs-evidence"
        },
        report
            .latest_benchmark
            .as_ref()
            .and_then(|snapshot| snapshot.fixture_mode.clone())
            .unwrap_or_else(|| "No benchmark history snapshot is attached yet.".to_string()),
    ));
    claims.extend(scorecard_claims(scorecard));

    DxForgeLaunchClaimsManifest {
        version: 1,
        route: "/forge".to_string(),
        generated_at: report.generated_at.clone(),
        claims,
    }
}

fn forge_launch_evidence_manifest(
    report: &DxForgeReleaseEvidenceReport,
) -> DxForgeLaunchEvidenceManifest {
    forge_launch_evidence_manifest_inner(
        &report.package_scorecard,
        report.generated_at.clone(),
        benchmark_summary(report.latest_benchmark.as_ref()),
    )
}

fn forge_launch_evidence_manifest_inner(
    scorecard: &DxForgePackageScorecardReport,
    generated_at: String,
    benchmark_summary: String,
) -> DxForgeLaunchEvidenceManifest {
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

    DxForgeLaunchEvidenceManifest {
        version: 1,
        route: "/forge".to_string(),
        generated_at,
        summary: DxForgeLaunchEvidenceSummary {
            package_score: scorecard.score,
            package_count: scorecard.packages.len(),
            verified_packages,
            source_owned_packages,
            node_modules_packages,
            benchmark_summary,
        },
        packages: scorecard.packages.iter().map(package_evidence).collect(),
        honest_boundaries: scorecard.honest_boundaries.clone(),
    }
}

fn package_evidence(package: &DxForgePackageScorecardEntry) -> DxForgeLaunchPackageEvidence {
    DxForgeLaunchPackageEvidence {
        package_id: package.package_id.clone(),
        description: package.description.clone(),
        version: package.version.clone(),
        language: package.language.clone(),
        license: package.license.clone(),
        file_count: package.file_count,
        total_bytes: package.total_bytes,
        public_claim: package.public_claim.clone(),
        launch_boundary: package.launch_boundary.clone(),
        provenance_source: package.provenance.source.clone(),
        provenance_verified: package.provenance.verified,
        provenance_note: package.provenance.note.clone(),
        advisory_provider: package.advisory_review.provider.clone(),
        advisory_live_coverage: package.advisory_review.live_coverage,
        advisory_findings: package.advisory_review.finding_count,
        advisory_note: package.advisory_review.note.clone(),
        declared_license: package.license_review.declared_license.clone(),
        license_reviewed: package.license_review.reviewed,
        license_note: package.license_review.note.clone(),
        local_evidence: package
            .project_evidence
            .as_ref()
            .map(local_package_evidence)
            .unwrap_or_else(|| "public registry only".to_string()),
    }
}

fn local_package_evidence(evidence: &DxForgePackageProjectEvidence) -> String {
    format!(
        "{} variants, {} tracked files, docs {}/{}, rollback {}/{}",
        evidence.manifest_variant_count,
        evidence.manifest_file_count,
        evidence.docs_present,
        evidence.docs_present + evidence.docs_missing,
        evidence.rollback_receipts_present,
        evidence.rollback_receipts_present + evidence.rollback_receipts_missing
    )
}

fn scorecard_claims(scorecard: &DxForgePackageScorecardReport) -> Vec<DxForgeLaunchClaim> {
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

    let mut claims = vec![
        launch_claim(
            "package-scorecard-score",
            format!(
                "Package scorecard is {} / 100 for {} launch packages.",
                scorecard.score,
                scorecard.packages.len()
            ),
            "DxForgePackageScorecardReport",
            "package_scorecard.score,package_scorecard.packages",
            if scorecard.score >= 95 {
                "verified"
            } else {
                "needs-review"
            },
            format!(
                "verified_packages={}, source_owned_packages={}, node_modules_packages={}",
                verified_packages, source_owned_packages, node_modules_packages
            ),
        ),
        launch_claim(
            "source-owned-no-node-modules",
            "The launch package set is source-owned and creates no node_modules packages.",
            "DxForgePackageScorecardReport",
            "package_scorecard.packages[].source_owned",
            if source_owned_packages == scorecard.packages.len() && node_modules_packages == 0 {
                "verified"
            } else {
                "needs-review"
            },
            format!(
                "source_owned={}/{}, node_modules={}",
                source_owned_packages,
                scorecard.packages.len(),
                node_modules_packages
            ),
        ),
        launch_claim(
            "install-scripts-blocked",
            "The launch package set blocks install scripts and package lifecycle execution.",
            "DxForgePackageScorecardReport",
            "package_scorecard.packages[].install_scripts_blocked",
            if scorecard
                .packages
                .iter()
                .all(|package| package.install_scripts_blocked)
            {
                "verified"
            } else {
                "needs-review"
            },
            "Every package card reads `scripts blocked yes` only from the scorecard model.",
        ),
    ];

    for package in &scorecard.packages {
        let package_status = if package.integrity_verified
            && package.source_owned
            && package.install_scripts_blocked
            && !package.node_modules_created
        {
            "verified"
        } else {
            "needs-review"
        };
        let package_key = safe_claim_id(&package.package_id);
        claims.push(launch_claim(
            format!("package-{package_key}"),
            format!(
                "{} is rendered as source-owned with {} files, {} bytes, and launch boundary `{}`.",
                package.package_id, package.file_count, package.total_bytes, package.launch_boundary
            ),
            "DxForgePackageScorecardReport",
            format!("package_scorecard.packages[{}]", package.package_id),
            package_status,
            format!(
                "integrity_verified={}, source_owned={}, scripts_blocked={}, node_modules_created={}",
                package.integrity_verified,
                package.source_owned,
                package.install_scripts_blocked,
                package.node_modules_created
            ),
        ));
        claims.push(launch_claim(
            format!("package-{package_key}-provenance"),
            format!(
                "{} provenance source is `{}` and verified is `{}`.",
                package.package_id, package.provenance.source, package.provenance.verified
            ),
            "DxForgePackageScorecardReport",
            format!(
                "package_scorecard.packages[{}].provenance",
                package.package_id
            ),
            if package.provenance.verified {
                "verified"
            } else {
                "declared"
            },
            package.provenance.note.clone(),
        ));
        claims.push(launch_claim(
            format!("package-{package_key}-advisory"),
            format!(
                "{} advisory coverage uses `{}` with live coverage `{}` and {} findings.",
                package.package_id,
                package.advisory_review.provider,
                package.advisory_review.live_coverage,
                package.advisory_review.finding_count
            ),
            "DxForgePackageScorecardReport",
            format!(
                "package_scorecard.packages[{}].advisory_review",
                package.package_id
            ),
            if package.advisory_review.live_coverage {
                "verified"
            } else {
                "declared"
            },
            package.advisory_review.note.clone(),
        ));
        claims.push(launch_claim(
            format!("package-{package_key}-license"),
            format!(
                "{} declares license `{}` and license review `{}`.",
                package.package_id,
                package.license_review.declared_license,
                package.license_review.reviewed
            ),
            "DxForgePackageScorecardReport",
            format!(
                "package_scorecard.packages[{}].license_review",
                package.package_id
            ),
            if package.license_review.reviewed {
                "verified"
            } else {
                "declared"
            },
            package.license_review.note.clone(),
        ));
    }

    for (index, boundary) in scorecard.honest_boundaries.iter().enumerate() {
        claims.push(launch_claim(
            format!("honest-boundary-{}", index + 1),
            boundary.clone(),
            "DxForgePackageScorecardReport",
            "package_scorecard.honest_boundaries[]",
            "declared",
            "Displayed directly in the launch-page honest-boundary section.",
        ));
    }

    claims
}

fn launch_claim(
    id: impl Into<String>,
    claim: impl Into<String>,
    source_model: impl Into<String>,
    source_field: impl Into<String>,
    verification_status: impl Into<String>,
    evidence: impl Into<String>,
) -> DxForgeLaunchClaim {
    DxForgeLaunchClaim {
        id: id.into(),
        claim: claim.into(),
        source_model: source_model.into(),
        source_field: source_field.into(),
        verification_status: verification_status.into(),
        evidence: evidence.into(),
    }
}

fn safe_claim_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn forge_public_evidence_page_source(report: &DxForgePublicEvidenceReport) -> String {
    let link_cards = report
        .links
        .iter()
        .map(|link| {
            format!(
                r#"
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium uppercase text-neutral-500">{section}</p>
                    <h2 class="mt-2 text-xl font-semibold">
                        <a class="text-neutral-50 underline decoration-neutral-700 underline-offset-4" href="{href}">{label}</a>
                    </h2>
                    <p class="mt-3 text-sm text-neutral-300">{description}</p>
                    <p class="mt-3 text-xs text-neutral-500">Model: {source_model}</p>
                </article>
"#,
                section = escape_page_text(link.section),
                href = escape_page_text(link.href),
                label = escape_page_text(link.label),
                description = escape_page_text(link.description),
                source_model = escape_page_text(link.source_model)
            )
        })
        .collect::<String>();

    format!(
        r#"
<page>
    <main class="min-h-screen bg-neutral-950 px-6 py-10 text-neutral-50">
        <section class="mx-auto grid max-w-5xl gap-8">
            <div class="grid gap-4 border-b border-neutral-800 pb-8">
                <p class="text-sm font-medium uppercase text-neutral-400">DX Forge public evidence</p>
                <h1 class="max-w-3xl text-5xl font-semibold">DX Forge Public Evidence</h1>
                <p class="max-w-3xl text-lg text-neutral-300">
                    One static, compiler-generated index for the launch page, scorecard, CI evidence, readiness badge, claims manifests, evidence models, and benchmark comparisons.
                </p>
                <p class="text-sm text-neutral-300">
                    Score {score} / 100. Packages {package_count}. Verified {verified_packages}. Source-owned {source_owned_packages}. node_modules packages {node_modules_packages}. Generated {generated_at}.
                </p>
            </div>

            <section class="grid gap-4 md:grid-cols-3">
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Evidence map</p>
                    <h2 class="mt-2 text-2xl font-semibold">{link_count} public artifacts</h2>
                    <p class="mt-3 text-sm text-neutral-300">Every link is reviewable source or generated evidence. This route is static and ships no authored client runtime.</p>
                </article>
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Package scorecard</p>
                    <h2 class="mt-2 text-2xl font-semibold">{score} / 100</h2>
                    <p class="mt-3 text-sm text-neutral-300">Source-owned packages remain editable local files with receipts, not node_modules installs.</p>
                </article>
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Public claim</p>
                    <h2 class="mt-2 text-2xl font-semibold">Evidence first</h2>
                    <p class="mt-3 text-sm text-neutral-300">DX Forge publishes claims beside their source models so marketing copy stays auditable.</p>
                </article>
            </section>

            <section class="grid gap-4 md:grid-cols-2">
                {link_cards}
            </section>
        </section>
    </main>
</page>
"#,
        score = report.score,
        package_count = report.package_count,
        verified_packages = report.verified_packages,
        source_owned_packages = report.source_owned_packages,
        node_modules_packages = report.node_modules_packages,
        generated_at = escape_page_text(&report.generated_at),
        link_count = report.links.len(),
        link_cards = link_cards
    )
}

fn forge_public_evidence_claims_manifest(
    report: &DxForgePublicEvidenceReport,
) -> DxForgeLaunchClaimsManifest {
    let evidence_paths = report
        .links
        .iter()
        .map(|link| link.href)
        .collect::<Vec<_>>()
        .join(", ");

    DxForgeLaunchClaimsManifest {
        version: 1,
        route: "/forge/evidence".to_string(),
        generated_at: report.generated_at.clone(),
        claims: vec![
            launch_claim(
                "public-evidence-index",
                format!(
                    "The /forge/evidence route links {} public launch, package, CI, badge, claims, evidence, and benchmark artifacts.",
                    report.links.len()
                ),
                "DxForgePublicEvidenceIndex",
                "links[]",
                "declared",
                evidence_paths,
            ),
            launch_claim(
                "public-evidence-static-route",
                "The public evidence index is a static route with no authored interaction contract.",
                "DxVerticalProofFixture",
                "forge-evidence",
                "declared",
                "dx prove vertical writes a DXPK proof artifact but no route runtime asset for static evidence fixtures.",
            ),
            launch_claim(
                "public-evidence-scorecard-context",
                format!(
                    "The evidence index was generated with package scorecard {} / 100 across {} launch packages.",
                    report.score, report.package_count
                ),
                "DxForgePackageScorecardReport",
                "score,packages",
                if report.score >= 95 {
                    "verified"
                } else {
                    "needs-review"
                },
                format!(
                    "verified_packages={}, source_owned_packages={}, node_modules_packages={}, generated_at={}",
                    report.verified_packages,
                    report.source_owned_packages,
                    report.node_modules_packages,
                    report.generated_at
                ),
            ),
        ],
    }
}

fn empty_release_history() -> DxForgePublicReleaseHistory {
    DxForgePublicReleaseHistory {
        updated_at: "preview".to_string(),
        records: Vec::new(),
    }
}

fn read_release_history_for_project(project: &Path) -> DxResult<DxForgePublicReleaseHistory> {
    let history_path = project.join("benchmarks/reports/forge-public-release-history.json");
    if !history_path.is_file() {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Forge public release history is missing: {}",
                history_path.display()
            ),
            field: Some("prove vertical fixture".to_string()),
        });
    }

    serde_json::from_slice(&std::fs::read(&history_path).map_err(|error| {
        DxError::ConfigValidationError {
            message: format!("Could not read {}: {error}", history_path.display()),
            field: Some("prove vertical fixture".to_string()),
        }
    })?)
    .map_err(|error| DxError::ConfigValidationError {
        message: format!(
            "Could not parse Forge public release history {}: {error}",
            history_path.display()
        ),
        field: Some("prove vertical fixture".to_string()),
    })
}

fn read_launch_changelog_for_project(project: &Path) -> DxResult<DxForgeLaunchChangelogReport> {
    let history_path = project.join("benchmarks/reports/forge-public-release-history.json");
    build_forge_launch_changelog_report(DxForgeLaunchChangelogInput { history_path }).map_err(
        |error| DxError::ConfigValidationError {
            message: format!("Could not build Forge launch changelog fixture: {error}"),
            field: Some("prove vertical fixture".to_string()),
        },
    )
}

fn preview_launch_changelog_report() -> DxForgeLaunchChangelogReport {
    DxForgeLaunchChangelogReport {
        version: 1,
        history_path: PathBuf::from("benchmarks/reports/forge-public-release-history.json"),
        generated_at: "preview".to_string(),
        passed: false,
        score: 0,
        status: "preview".to_string(),
        record_count: 0,
        latest: None,
        entries: Vec::new(),
        honest_scope: vec![
            "This changelog is generated only from reviewed Forge release-history records."
                .to_string(),
            "It does not claim live production traffic, customer adoption, or universal npm replacement coverage."
                .to_string(),
        ],
        findings: vec!["Preview mode has no release-history records attached yet.".to_string()],
    }
}

fn forge_changelog_page_source(report: &DxForgeLaunchChangelogReport) -> String {
    let latest = report.latest.as_ref();
    let dashboard_score = latest
        .map(|entry| format!("{} / 100", entry.dashboard_score))
        .unwrap_or_else(|| "Needs evidence".to_string());
    let route_count = latest
        .map(|entry| entry.route_count.to_string())
        .unwrap_or_else(|| "0".to_string());
    let total_brotli = latest
        .map(|entry| format!("{} B", entry.total_brotli_bytes))
        .unwrap_or_else(|| "0 B".to_string());
    let added_routes = latest
        .map(|entry| route_summary_cards("Added public routes", &entry.added_routes))
        .unwrap_or_else(|| route_summary_cards("Added public routes", &[]));
    let removed_routes = latest
        .map(|entry| route_summary_cards("Removed public routes", &entry.removed_routes))
        .unwrap_or_else(|| route_summary_cards("Removed public routes", &[]));
    let changed_routes = latest
        .map(|entry| changelog_route_change_cards(&entry.changed_routes))
        .unwrap_or_else(|| {
            r#"
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Changed routes</p>
                    <h2 class="mt-2 text-xl font-semibold">Needs evidence</h2>
                    <p class="mt-3 text-sm text-neutral-300">No release-history entries are attached yet.</p>
                </article>
"#
            .to_string()
        });
    let findings = if report.findings.is_empty() {
        r#"
                    <li class="rounded-md border border-neutral-800 bg-neutral-950 p-3">No changelog generation findings.</li>
"#
        .to_string()
    } else {
        report
            .findings
            .iter()
            .map(|finding| {
                format!(
                    r#"
                    <li class="rounded-md border border-neutral-800 bg-neutral-950 p-3">{}</li>
"#,
                    escape_page_text(finding)
                )
            })
            .collect::<String>()
    };
    let honest_scope = report
        .honest_scope
        .iter()
        .map(|scope| {
            format!(
                r#"
                    <li class="rounded-md border border-neutral-800 bg-neutral-950 p-3">{}</li>
"#,
                escape_page_text(scope)
            )
        })
        .collect::<String>();

    format!(
        r#"
<page>
    <main class="min-h-screen bg-neutral-950 px-6 py-10 text-neutral-50">
        <section class="mx-auto grid max-w-5xl gap-8">
            <div class="grid gap-4 border-b border-neutral-800 pb-8">
                <p class="text-sm font-medium uppercase text-neutral-400">DX Forge launch changelog</p>
                <h1 class="max-w-3xl text-5xl font-semibold">DX Forge Public Launch Changelog</h1>
                <p class="max-w-3xl text-lg text-neutral-300">
                    A crawlable static route generated from forge-public-launch-changelog.json so public launch reviewers can see what changed without executing a client runtime.
                </p>
                <p class="text-sm text-neutral-300">Route /forge/changelog. Status {status}. Generated {generated_at}. Records {records}.</p>
            </div>

            <section class="grid gap-4 md:grid-cols-4">
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Changelog score</p>
                    <h2 class="mt-2 text-2xl font-semibold">{score} / 100</h2>
                    <p class="mt-3 text-sm text-neutral-300">Dashboard {dashboard_score}. Passing {passed}.</p>
                </article>
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Public routes</p>
                    <h2 class="mt-2 text-2xl font-semibold">{route_count}</h2>
                    <p class="mt-3 text-sm text-neutral-300">Route coverage comes from reviewed release-history records.</p>
                </article>
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Brotli total</p>
                    <h2 class="mt-2 text-2xl font-semibold">{total_brotli}</h2>
                    <p class="mt-3 text-sm text-neutral-300">Payload drift is compared against the previous release record.</p>
                </article>
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Machine evidence</p>
                    <h2 class="mt-2 text-2xl font-semibold">Reviewable JSON</h2>
                    <p class="mt-3 text-sm text-neutral-300"><code>forge-public-launch-changelog.json</code> backs this route and its claims manifest.</p>
                </article>
            </section>

            <section class="grid gap-4 md:grid-cols-3">
                {added_routes}
                {removed_routes}
                {changed_routes}
            </section>

            <section class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                <p class="text-sm font-medium text-neutral-400">Honest scope</p>
                <h2 class="mt-2 text-2xl font-semibold">Generated release notes, not adoption proof</h2>
                <ul class="mt-4 grid gap-3 text-sm text-neutral-300">
                    {honest_scope}
                </ul>
            </section>

            <section class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                <p class="text-sm font-medium text-neutral-400">Changelog findings</p>
                <h2 class="mt-2 text-2xl font-semibold">Public review notes</h2>
                <ul class="mt-4 grid gap-3 text-sm text-neutral-300">
                    {findings}
                </ul>
            </section>
        </section>
    </main>
</page>
"#,
        status = escape_page_text(&report.status),
        generated_at = escape_page_text(&report.generated_at),
        records = report.record_count,
        score = report.score,
        dashboard_score = escape_page_text(&dashboard_score),
        passed = report.passed,
        route_count = route_count,
        total_brotli = total_brotli,
        added_routes = added_routes,
        removed_routes = removed_routes,
        changed_routes = changed_routes,
        honest_scope = honest_scope,
        findings = findings
    )
}

fn route_summary_cards(label: &str, routes: &[String]) -> String {
    let summary = if routes.is_empty() {
        "None".to_string()
    } else {
        routes
            .iter()
            .map(|route| format!("`{}`", escape_page_text(route)))
            .collect::<Vec<_>>()
            .join(", ")
    };

    format!(
        r#"
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">{label}</p>
                    <h2 class="mt-2 text-xl font-semibold">{count}</h2>
                    <p class="mt-3 text-sm text-neutral-300">{summary}</p>
                </article>
"#,
        label = escape_page_text(label),
        count = routes.len(),
        summary = summary
    )
}

fn changelog_route_change_cards(
    changes: &[super::forge_launch_changelog::DxForgeLaunchChangelogRouteChange],
) -> String {
    if changes.is_empty() {
        return r#"
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Changed routes</p>
                    <h2 class="mt-2 text-xl font-semibold">0</h2>
                    <p class="mt-3 text-sm text-neutral-300">No existing public route payload changes were recorded.</p>
                </article>
"#
        .to_string();
    }

    changes
        .iter()
        .map(|change| {
            format!(
                r#"
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">{route}</p>
                    <h2 class="mt-2 text-xl font-semibold">Payload changed</h2>
                    <p class="mt-3 text-sm text-neutral-300">Decoded delta {decoded} B. Brotli delta {brotli} B.</p>
                </article>
"#,
                route = escape_page_text(&change.route),
                decoded = change.decoded_delta_bytes,
                brotli = change.brotli_delta_bytes
            )
        })
        .collect::<String>()
}

fn forge_changelog_claims_manifest(
    report: &DxForgeLaunchChangelogReport,
) -> DxForgeLaunchClaimsManifest {
    let latest = report.latest.as_ref();
    let mut claims = vec![
        launch_claim(
            "launch-changelog-static-route",
            "The /forge/changelog route is generated from the public launch changelog and ships no authored client runtime.",
            "DxForgeLaunchChangelogReport",
            "status,score,latest,entries",
            if report.latest.is_some() {
                "declared"
            } else {
                "needs-evidence"
            },
            format!(
                "status={}, score={}, records={}",
                report.status, report.score, report.record_count
            ),
        ),
        launch_claim(
            "launch-changelog-honest-scope",
            "The launch changelog publishes honest scope limits beside the public route.",
            "DxForgeLaunchChangelogReport",
            "honest_scope[]",
            if report
                .honest_scope
                .iter()
                .any(|scope| scope.contains("does not claim live production traffic"))
            {
                "verified"
            } else {
                "needs-review"
            },
            report.honest_scope.join("; "),
        ),
    ];

    if let Some(entry) = latest {
        claims.push(launch_claim(
            "latest-launch-changelog-score",
            format!(
                "Latest launch changelog reports dashboard {} / 100 across {} public routes.",
                entry.dashboard_score, entry.route_count
            ),
            "DxForgeLaunchChangelogEntry",
            "dashboard_score,route_count,total_brotli_bytes",
            if report.passed {
                "verified"
            } else {
                "needs-review"
            },
            format!(
                "total_brotli_bytes={}, added_routes={}, regression_findings={}",
                entry.total_brotli_bytes,
                entry.added_routes.len(),
                entry.regression_findings.len()
            ),
        ));
    }

    DxForgeLaunchClaimsManifest {
        version: 1,
        route: "/forge/changelog".to_string(),
        generated_at: report.generated_at.clone(),
        claims,
    }
}

fn forge_releases_page_source(history: &DxForgePublicReleaseHistory) -> String {
    let latest = history.records.first();
    let dashboard_score = latest
        .map(|record| format!("{} / 100", record.dashboard.score))
        .unwrap_or_else(|| "Needs evidence".to_string());
    let dashboard_status = latest
        .map(|record| {
            if record.dashboard.passed {
                "Passed"
            } else {
                "Needs review"
            }
        })
        .unwrap_or("No release record");
    let route_count = latest
        .map(|record| record.route_comparison.route_count.to_string())
        .unwrap_or_else(|| "0".to_string());
    let total_brotli = latest
        .map(|record| format!("{} B", record.route_comparison.total_brotli_bytes))
        .unwrap_or_else(|| "0 B".to_string());
    let regression_count = latest
        .map(|record| record.regression_findings.len().to_string())
        .unwrap_or_else(|| "0".to_string());
    let latest_routes = latest
        .map(|record| release_route_cards(&record.route_comparison.routes))
        .unwrap_or_else(|| {
            r#"
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">No release routes yet</p>
                    <h2 class="mt-2 text-xl font-semibold">Run dx forge release-history</h2>
                    <p class="mt-3 text-sm text-neutral-300">The public release route needs a checked-in release-history JSON file.</p>
                </article>
"#
            .to_string()
        });
    let regression_list = latest
        .map(|record| release_regression_list(&record.regression_findings))
        .unwrap_or_else(|| {
            r#"
                    <li class="rounded-md border border-neutral-800 bg-neutral-950 p-3">No release history has been recorded yet.</li>
"#
            .to_string()
        });
    let record_rows = release_record_rows(history);

    format!(
        r#"
<page>
    <main class="min-h-screen bg-neutral-950 px-6 py-10 text-neutral-50">
        <section class="mx-auto grid max-w-5xl gap-8">
            <div class="grid gap-4 border-b border-neutral-800 pb-8">
                <p class="text-sm font-medium uppercase text-neutral-400">DX Forge release history</p>
                <h1 class="max-w-3xl text-5xl font-semibold">DX Forge Release History</h1>
                <p class="max-w-3xl text-lg text-neutral-300">
                    A compact static route generated from forge-public-release-history.json so public launch reviews can see score, route budget, and regression drift without a client runtime.
                </p>
                <p class="text-sm text-neutral-300">Route /forge/releases. Updated {updated_at}. Records {records}.</p>
            </div>

            <section class="grid gap-4 md:grid-cols-4">
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Dashboard</p>
                    <h2 class="mt-2 text-2xl font-semibold">{dashboard_score}</h2>
                    <p class="mt-3 text-sm text-neutral-300">{dashboard_status}</p>
                </article>
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Public routes</p>
                    <h2 class="mt-2 text-2xl font-semibold">{route_count}</h2>
                    <p class="mt-3 text-sm text-neutral-300">Static route budget evidence from the latest comparison.</p>
                </article>
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Brotli total</p>
                    <h2 class="mt-2 text-2xl font-semibold">{total_brotli}</h2>
                    <p class="mt-3 text-sm text-neutral-300">First-route payload pressure across public Forge routes.</p>
                </article>
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Regressions</p>
                    <h2 class="mt-2 text-2xl font-semibold">{regression_count}</h2>
                    <p class="mt-3 text-sm text-neutral-300">Score drops, route growth, missing routes, and failed budgets.</p>
                </article>
            </section>

            <section class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                <p class="text-sm font-medium text-neutral-400">Latest regression checks</p>
                <h2 class="mt-2 text-2xl font-semibold">Release drift gate</h2>
                <ul class="mt-4 grid gap-3 text-sm text-neutral-300">
                    {regression_list}
                </ul>
            </section>

            <section class="grid gap-4 md:grid-cols-2">
                {latest_routes}
            </section>

            <section class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                <p class="text-sm font-medium text-neutral-400">Recorded releases</p>
                <h2 class="mt-2 text-2xl font-semibold">Newest-first evidence trail</h2>
                <ul class="mt-4 grid gap-3 text-sm text-neutral-300">
                    {record_rows}
                </ul>
            </section>
        </section>
    </main>
</page>
"#,
        updated_at = escape_page_text(&history.updated_at),
        records = history.records.len(),
        dashboard_score = escape_page_text(&dashboard_score),
        dashboard_status = dashboard_status,
        route_count = route_count,
        total_brotli = total_brotli,
        regression_count = regression_count,
        regression_list = regression_list,
        latest_routes = latest_routes,
        record_rows = record_rows
    )
}

fn release_route_cards(routes: &[DxForgePublicReleaseRouteSnapshot]) -> String {
    routes
        .iter()
        .map(|route| {
            format!(
                r#"
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">{route_path}</p>
                    <h2 class="mt-2 text-xl font-semibold">{fixture}</h2>
                    <p class="mt-3 text-sm text-neutral-300">Delivery {delivery}. Decoded {decoded} B. Brotli {brotli} B.</p>
                    <p class="mt-2 text-sm text-neutral-400">HTTP {http} ms. Chrome load {chrome} ms. Budget {budget}.</p>
                </article>
"#,
                route_path = escape_page_text(&route.route),
                fixture = escape_page_text(&route.fixture_mode),
                delivery = escape_page_text(&route.delivery),
                decoded = route.decoded_bytes,
                brotli = route.brotli_bytes,
                http = format_release_number(route.http_route_median_ms),
                chrome = format_release_number(route.chrome_load_event_ms),
                budget = release_budget_label(route.budget_passed)
            )
        })
        .collect::<String>()
}

fn release_regression_list(findings: &[String]) -> String {
    if findings.is_empty() {
        return r#"
                    <li class="rounded-md border border-neutral-800 bg-neutral-950 p-3">No release regressions detected against the previous distinct record.</li>
"#
        .to_string();
    }

    findings
        .iter()
        .map(|finding| {
            format!(
                r#"
                    <li class="rounded-md border border-red-900 bg-red-950 p-3">{}</li>
"#,
                escape_page_text(finding)
            )
        })
        .collect::<String>()
}

fn release_record_rows(history: &DxForgePublicReleaseHistory) -> String {
    if history.records.is_empty() {
        return r#"
                    <li class="rounded-md border border-neutral-800 bg-neutral-950 p-3">No release records yet.</li>
"#
        .to_string();
    }

    history
        .records
        .iter()
        .map(|record| {
            format!(
                r#"
                    <li class="rounded-md border border-neutral-800 bg-neutral-950 p-3">
                        {generated_at}: dashboard {score} / 100, routes {routes}, Brotli {brotli} B, regressions {regressions}.
                    </li>
"#,
                generated_at = escape_page_text(&record.generated_at),
                score = record.dashboard.score,
                routes = record.route_comparison.route_count,
                brotli = record.route_comparison.total_brotli_bytes,
                regressions = record.regression_findings.len()
            )
        })
        .collect::<String>()
}

fn release_budget_label(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "passed",
        Some(false) => "failed",
        None => "not configured",
    }
}

fn format_release_number(value: f64) -> String {
    let mut text = format!("{value:.3}");
    while text.contains('.') && text.ends_with('0') {
        text.pop();
    }
    if text.ends_with('.') {
        text.pop();
    }
    text
}

fn forge_releases_claims_manifest(
    history: &DxForgePublicReleaseHistory,
) -> DxForgeLaunchClaimsManifest {
    let latest = history.records.first();
    let generated_at = if history.updated_at.is_empty() {
        "preview".to_string()
    } else {
        history.updated_at.clone()
    };
    let mut claims = vec![launch_claim(
        "release-history-static-route",
        "The /forge/releases route is generated from the public release-history model and ships no authored client runtime.",
        "DxForgePublicReleaseHistory",
        "records[]",
        if latest.is_some() {
            "declared"
        } else {
            "needs-evidence"
        },
        format!("records={}", history.records.len()),
    )];

    if let Some(record) = latest {
        claims.push(launch_claim(
            "latest-release-dashboard",
            format!(
                "Latest release dashboard is {} / 100 with passed `{}`.",
                record.dashboard.score, record.dashboard.passed
            ),
            "DxForgePublicReleaseRecord",
            "dashboard.score,dashboard.passed",
            if record.dashboard.passed {
                "verified"
            } else {
                "needs-review"
            },
            format!(
                "fail_under={}, findings={}",
                record.dashboard.fail_under,
                record.dashboard.findings.len()
            ),
        ));
        claims.push(launch_claim(
            "latest-release-route-budget",
            format!(
                "Latest release comparison covers {} routes with {} Brotli bytes total.",
                record.route_comparison.route_count, record.route_comparison.total_brotli_bytes
            ),
            "DxForgePublicReleaseRouteComparisonSnapshot",
            "route_count,total_brotli_bytes,routes[].budget_passed",
            if record
                .route_comparison
                .routes
                .iter()
                .all(|route| route.budget_passed != Some(false))
            {
                "verified"
            } else {
                "needs-review"
            },
            format!(
                "lowest_brotli_route={}",
                record.route_comparison.lowest_brotli_route
            ),
        ));
        let regression_evidence = if record.regression_findings.is_empty() {
            "No release regressions detected.".to_string()
        } else {
            record.regression_findings.join("; ")
        };
        claims.push(launch_claim(
            "latest-release-regressions",
            format!(
                "Latest release history has {} regression findings.",
                record.regression_findings.len()
            ),
            "DxForgePublicReleaseRecord",
            "regression_findings[]",
            if record.regression_findings.is_empty() {
                "verified"
            } else {
                "needs-review"
            },
            regression_evidence,
        ));
    }

    DxForgeLaunchClaimsManifest {
        version: 1,
        route: "/forge/releases".to_string(),
        generated_at,
        claims,
    }
}

struct ForgeLaunchPageFacts<'a> {
    generated_at: String,
    release_gate: String,
    check_score: String,
    check_traffic: String,
    launch_findings: String,
    rollback_coverage: String,
    docs_coverage: String,
    benchmark_summary: String,
    scorecard: &'a DxForgePackageScorecardReport,
}

fn forge_launch_page_source_inner(facts: ForgeLaunchPageFacts<'_>) -> String {
    let scorecard = facts.scorecard;
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
    let package_cards = compact_package_cards(scorecard);
    let boundaries = compact_boundary_list(scorecard);

    format!(
        r#"
<page>
    <main class="min-h-screen bg-neutral-950 px-6 py-10 text-neutral-50">
        <section class="mx-auto grid max-w-5xl gap-6">
            <div class="grid gap-4 border-b border-neutral-800 pb-7">
                <p class="text-sm font-medium uppercase text-neutral-400">DX Forge launch evidence</p>
                <h1 class="max-w-3xl text-5xl font-semibold">Source-owned package firewall for selected web code.</h1>
                <p class="max-w-3xl text-lg text-neutral-300">
                    This compact page is generated from the same release-proof and package-scorecard models used by the DX CLI.
                </p>
                <div class="flex flex-wrap gap-3">
                    <Button>
                        <SearchIcon />
                        Review evidence
                    </Button>
                </div>
            </div>

            <section class="grid gap-4 md:grid-cols-3">
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Release gate</p>
                    <h2 class="mt-2 text-2xl font-semibold">{release_gate}</h2>
                    <p class="mt-3 text-sm text-neutral-300">DX check {check_score} / 100, traffic {check_traffic}, launch findings {launch_findings}.</p>
                    <p class="mt-2 text-sm text-neutral-300">Rollback coverage {rollback_coverage}; package docs coverage {docs_coverage}.</p>
                </article>
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Package scorecard</p>
                    <h2 class="mt-2 text-2xl font-semibold">{score} / 100</h2>
                    <p class="mt-3 text-sm text-neutral-300">{verified_packages} verified, {source_owned_packages} source-owned, {node_modules_packages} node_modules packages.</p>
                    <p class="mt-2 text-sm text-neutral-300">Generated {generated_at}.</p>
                </article>
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Benchmark evidence</p>
                    <h2 class="mt-2 text-2xl font-semibold">DXPK proof path</h2>
                    <p class="mt-3 text-sm text-neutral-300">{benchmark_summary}</p>
                </article>
            </section>

            <section class="grid gap-4 md:grid-cols-3">
                {package_cards}
            </section>

            <section class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                <p class="text-sm font-medium text-neutral-400">Evidence model</p>
                <h2 class="mt-2 text-2xl font-semibold">Detailed package proof loads outside the first route payload</h2>
                <p class="mt-3 text-sm text-neutral-300">
                    Provenance, advisory, license-review, local-doc, and rollback details are written to <code>forge.evidence.json</code>.
                </p>
                <p class="mt-2 text-sm text-neutral-400">
                    The first route keeps only release status, package summaries, benchmark evidence, and launch boundaries.
                </p>
            </section>

            <section class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                <p class="text-sm font-medium text-neutral-400">Honest launch boundary</p>
                <h2 class="mt-2 text-2xl font-semibold">Not universal npm yet</h2>
                <ul class="mt-4 grid gap-3 text-sm text-neutral-300">
                    {boundaries}
                </ul>
            </section>
        </section>
    </main>
</page>
"#,
        release_gate = facts.release_gate,
        check_score = facts.check_score,
        check_traffic = facts.check_traffic,
        launch_findings = facts.launch_findings,
        rollback_coverage = facts.rollback_coverage,
        docs_coverage = facts.docs_coverage,
        score = scorecard.score,
        verified_packages = verified_packages,
        source_owned_packages = source_owned_packages,
        node_modules_packages = node_modules_packages,
        generated_at = facts.generated_at,
        benchmark_summary = facts.benchmark_summary,
        package_cards = package_cards,
        boundaries = boundaries
    )
}

fn compact_package_cards(report: &DxForgePackageScorecardReport) -> String {
    report
        .packages
        .iter()
        .map(|package| {
            format!(
                r#"
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">{package_id}</p>
                    <h2 class="mt-2 text-xl font-semibold">{status}</h2>
                    <p class="mt-3 text-sm text-neutral-300">{files} files, {bytes} source bytes, scripts blocked {scripts_blocked}.</p>
                    <p class="mt-2 text-sm text-neutral-400">{boundary}</p>
                </article>
"#,
                package_id = escape_page_text(&package.package_id),
                status = if package.integrity_verified && package.source_owned {
                    "Source-owned and verified"
                } else {
                    "Needs review"
                },
                files = package.file_count,
                bytes = package.total_bytes,
                scripts_blocked = if package.install_scripts_blocked {
                    "yes"
                } else {
                    "no"
                },
                boundary = escape_page_text(&package.launch_boundary)
            )
        })
        .collect::<String>()
}

fn compact_boundary_list(report: &DxForgePackageScorecardReport) -> String {
    report
        .honest_boundaries
        .iter()
        .map(|boundary| {
            format!(
                r#"
                    <li class="rounded-md border border-neutral-800 bg-neutral-950 p-3">{}</li>
"#,
                escape_page_text(boundary)
            )
        })
        .collect::<String>()
}

fn benchmark_summary(snapshot: Option<&DxForgeBenchmarkSnapshot>) -> String {
    let Some(snapshot) = snapshot else {
        return "No benchmark history snapshot is attached yet.".to_string();
    };
    format!(
        "Mode {}, delivery {}, decoded {}, Brotli {}, HTTP median {}, Chrome load {}, DXPK applied {}, interaction {}.",
        escape_page_text(snapshot.fixture_mode.as_deref().unwrap_or("unknown")),
        escape_page_text(snapshot.route_delivery.as_deref().unwrap_or("unknown")),
        snapshot
            .decoded_bytes
            .map(|value| value.to_string())
            .unwrap_or_else(|| "n/a".to_string()),
        snapshot
            .brotli_bytes
            .map(|value| value.to_string())
            .unwrap_or_else(|| "n/a".to_string()),
        format_optional_ms(snapshot.http_route_median_ms),
        format_optional_ms(snapshot.chrome_load_event_ms),
        yes_no(snapshot.dx_packet_applied),
        yes_no(snapshot.interaction_works)
    )
}

fn forge_benchmark_snapshot_is_release_ready(snapshot: &DxForgeBenchmarkSnapshot) -> bool {
    if snapshot.route_delivery.as_deref() == Some("static") {
        return snapshot.decoded_bytes.is_some()
            && snapshot.brotli_bytes.is_some()
            && snapshot.http_route_median_ms.is_some();
    }

    snapshot.dx_packet_applied == Some(true) && snapshot.interaction_works == Some(true)
}

fn format_optional_ms(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.3} ms"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn yes_no(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "yes",
        Some(false) => "no",
        None => "n/a",
    }
}

fn forge_ci_preview_page_source() -> String {
    forge_ci_page_source_inner(ForgeCiPageFacts {
        generated_at: "preview".to_string(),
        readiness_status: "Preview".to_string(),
        readiness_score: "n/a".to_string(),
        smoke_score: "n/a".to_string(),
        launch_quality: "n/a".to_string(),
        no_node_modules: "n/a".to_string(),
        package_count: "n/a".to_string(),
        artifact_count: "n/a".to_string(),
        budget_summary:
            "Budget evidence is attached after the Forge CI smoke model is generated.".to_string(),
        findings: "<li class=\"rounded-md border border-neutral-800 bg-neutral-950 p-3\">Preview mode: run with --write to render DxForgeSmokeReport and DxForgeReadinessBadge evidence.</li>".to_string(),
    })
}

fn forge_ci_page_source(smoke: &DxForgeSmokeReport, badge: &DxForgeReadinessBadge) -> String {
    forge_ci_page_source_inner(ForgeCiPageFacts {
        generated_at: escape_page_text(&smoke.generated_at),
        readiness_status: escape_page_text(&badge.status),
        readiness_score: badge.score.to_string(),
        smoke_score: smoke.score.to_string(),
        launch_quality: format!(
            "{} / 100, passed {}",
            smoke.launch_page_quality.score, smoke.launch_page_quality.passed
        ),
        no_node_modules: if smoke.no_node_modules {
            "yes".to_string()
        } else {
            "no".to_string()
        },
        package_count: smoke.packages.len().to_string(),
        artifact_count: "13".to_string(),
        budget_summary: ci_budget_summary(badge),
        findings: ci_findings_list(smoke, badge),
    })
}

struct ForgeCiPageFacts {
    generated_at: String,
    readiness_status: String,
    readiness_score: String,
    smoke_score: String,
    launch_quality: String,
    no_node_modules: String,
    package_count: String,
    artifact_count: String,
    budget_summary: String,
    findings: String,
}

fn forge_ci_page_source_inner(facts: ForgeCiPageFacts) -> String {
    format!(
        r#"
<page>
    <main class="min-h-screen bg-neutral-950 px-6 py-10 text-neutral-50">
        <section class="mx-auto grid max-w-5xl gap-8">
            <div class="grid gap-4 border-b border-neutral-800 pb-8">
                <p class="text-sm font-medium uppercase text-neutral-400">DX Forge CI evidence</p>
                <h1 class="max-w-3xl text-5xl font-semibold">Secret-free release checks for source-owned web packages.</h1>
                <p class="max-w-3xl text-lg text-neutral-300">
                    This compiler-generated route is rendered from DxForgeSmokeReport and DxForgeReadinessBadge, the same evidence models used by dx forge ci.
                </p>
                <p class="text-sm text-neutral-300">Generated {generated_at}.</p>
            </div>

            <section class="grid gap-4 md:grid-cols-3">
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Readiness badge</p>
                    <h2 class="mt-2 text-2xl font-semibold">{readiness_status}</h2>
                    <p class="mt-3 text-sm text-neutral-300">Score {readiness_score} / 100. CI keeps forge-readiness-badge.json as the compact release signal.</p>
                </article>
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Smoke model</p>
                    <h2 class="mt-2 text-2xl font-semibold">{smoke_score} / 100</h2>
                    <p class="mt-3 text-sm text-neutral-300">Packages {package_count}. No node_modules {no_node_modules}. Launch quality {launch_quality}.</p>
                </article>
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Budget signal</p>
                    <h2 class="mt-2 text-2xl font-semibold">Static /forge proof</h2>
                    <p class="mt-3 text-sm text-neutral-300">{budget_summary}</p>
                </article>
            </section>

            <section class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                <p class="text-sm font-medium text-neutral-400">CI artifact lane</p>
                <h2 class="mt-2 text-2xl font-semibold">Reviewable artifacts without R2 secrets</h2>
                <p class="mt-3 text-sm text-neutral-300">The artifact lane writes {artifact_count} bounded files and does not require Cloudflare R2 credentials.</p>
                <ul class="mt-4 grid gap-2 text-sm text-neutral-300 md:grid-cols-2">
                    <li><code>forge-smoke.json</code></li>
                    <li><code>forge-smoke.md</code></li>
                    <li><code>forge-triage.md</code></li>
                    <li><code>forge-readiness-badge.json</code></li>
                    <li><code>forge-evidence.json</code></li>
                    <li><code>forge-scorecard.json</code></li>
                    <li><code>forge-benchmark-history.json</code></li>
                    <li><code>forge.html</code></li>
                    <li><code>forge.claims.json</code></li>
                    <li><code>forge.evidence.json</code></li>
                    <li><code>forge.dxp</code></li>
                    <li><code>forge-page.html</code></li>
                    <li><code>forge-proof.json</code></li>
                </ul>
            </section>

            <section class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                <p class="text-sm font-medium text-neutral-400">Failure triage</p>
                <h2 class="mt-2 text-2xl font-semibold">First review actions come from the CI model</h2>
                <ul class="mt-4 grid gap-3 text-sm text-neutral-300">
                    {findings}
                </ul>
            </section>
        </section>
    </main>
</page>
"#,
        generated_at = facts.generated_at,
        readiness_status = facts.readiness_status,
        readiness_score = facts.readiness_score,
        smoke_score = facts.smoke_score,
        package_count = facts.package_count,
        no_node_modules = facts.no_node_modules,
        launch_quality = escape_page_text(&facts.launch_quality),
        budget_summary = escape_page_text(&facts.budget_summary),
        artifact_count = facts.artifact_count,
        findings = facts.findings,
    )
}

fn ci_budget_summary(badge: &DxForgeReadinessBadge) -> String {
    let Some(benchmark) = &badge.latest_forge_route_benchmark else {
        return "No latest /forge benchmark snapshot is attached yet.".to_string();
    };

    format!(
        "Fixture {}, delivery {}, decoded {}, Brotli {}, HTTP median {}, Chrome load {}, release-ready {}.",
        benchmark.fixture_mode.as_deref().unwrap_or("unknown"),
        benchmark.route_delivery.as_deref().unwrap_or("unknown"),
        benchmark
            .decoded_bytes
            .map(|value| value.to_string())
            .unwrap_or_else(|| "n/a".to_string()),
        benchmark
            .brotli_bytes
            .map(|value| value.to_string())
            .unwrap_or_else(|| "n/a".to_string()),
        format_optional_ms(benchmark.http_route_median_ms),
        format_optional_ms(benchmark.chrome_load_event_ms),
        if benchmark.passed { "yes" } else { "no" }
    )
}

fn ci_budget_claim_evidence(badge: &DxForgeReadinessBadge) -> String {
    let Some(benchmark) = &badge.latest_forge_route_benchmark else {
        return "No latest /forge benchmark snapshot is attached yet.".to_string();
    };

    format!(
        "fixture={}, delivery={}, decoded_bytes={}, brotli_bytes={}, http_route_median_ms={}, chrome_load_event_ms={}, passed={}",
        benchmark.fixture_mode.as_deref().unwrap_or("unknown"),
        benchmark.route_delivery.as_deref().unwrap_or("unknown"),
        benchmark
            .decoded_bytes
            .map(|value| value.to_string())
            .unwrap_or_else(|| "n/a".to_string()),
        benchmark
            .brotli_bytes
            .map(|value| value.to_string())
            .unwrap_or_else(|| "n/a".to_string()),
        format_optional_ms(benchmark.http_route_median_ms),
        format_optional_ms(benchmark.chrome_load_event_ms),
        benchmark.passed
    )
}

fn ci_findings_list(smoke: &DxForgeSmokeReport, badge: &DxForgeReadinessBadge) -> String {
    let mut findings = Vec::new();
    findings.extend(smoke.findings.iter().cloned());
    findings.extend(badge.findings.iter().cloned());
    if findings.is_empty() {
        findings.push(
            "No active release blockers. Keep forge-triage.md as the review baseline.".to_string(),
        );
    }

    findings
        .into_iter()
        .map(|finding| {
            format!(
                r#"
                    <li class="rounded-md border border-neutral-800 bg-neutral-950 p-3">{}</li>
"#,
                escape_page_text(&finding)
            )
        })
        .collect::<String>()
}

pub(super) fn forge_ci_claims_manifest(
    smoke: &DxForgeSmokeReport,
    badge: &DxForgeReadinessBadge,
) -> DxForgeLaunchClaimsManifest {
    let mut claims = vec![
        launch_claim(
            "ci-readiness-badge",
            format!(
                "DX Forge CI readiness is `{}` with score {} / 100.",
                badge.status, badge.score
            ),
            "DxForgeReadinessBadge",
            "status,score,passed,fail_under",
            if badge.passed {
                "verified"
            } else {
                "needs-review"
            },
            format!(
                "fail_under={}, findings={}",
                badge.fail_under,
                badge.findings.len()
            ),
        ),
        launch_claim(
            "ci-smoke-no-node-modules",
            format!(
                "Forge CI smoke reports no_node_modules `{}` across {} materialized packages.",
                smoke.no_node_modules,
                smoke.packages.len()
            ),
            "DxForgeSmokeReport",
            "no_node_modules,packages",
            if smoke.no_node_modules {
                "verified"
            } else {
                "needs-review"
            },
            format!(
                "smoke_score={}, findings={}",
                smoke.score,
                smoke.findings.len()
            ),
        ),
        launch_claim(
            "ci-launch-page-quality",
            format!(
                "Launch page quality score is {} / 100 and passed `{}`.",
                smoke.launch_page_quality.score, smoke.launch_page_quality.passed
            ),
            "DxForgeSmokeReport",
            "launch_page_quality",
            if smoke.launch_page_quality.passed {
                "verified"
            } else {
                "needs-review"
            },
            format!(
                "headings={}, seo={}, links={}, claims={}",
                smoke.launch_page_quality.headings.passed,
                smoke.launch_page_quality.seo.passed,
                smoke.launch_page_quality.links.passed,
                smoke.launch_page_quality.claims_manifest.passed
            ),
        ),
        launch_claim(
            "ci-budget-gate",
            ci_budget_summary(badge),
            "DxForgeReadinessBadge",
            "latest_forge_route_benchmark",
            if badge
                .latest_forge_route_benchmark
                .as_ref()
                .is_some_and(|benchmark| benchmark.passed)
            {
                "verified"
            } else {
                "needs-review"
            },
            ci_budget_claim_evidence(badge),
        ),
    ];

    claims.push(launch_claim(
        "ci-artifact-lane",
        "Forge CI writes bounded review artifacts without requiring Cloudflare R2 credentials.",
        "DxForgeReadinessBadge",
        "artifacts",
        "declared",
        "Artifact boundary is covered by the Forge CI CLI fixture test.",
    ));

    DxForgeLaunchClaimsManifest {
        version: 1,
        route: "/forge/ci".to_string(),
        generated_at: smoke.generated_at.clone(),
        claims,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::cli::{
        DxForgeLaunchPageQualityCheck, DxForgeLaunchPageQualityReport,
        DxForgeReadinessBadgeArtifacts, DxForgeReadinessBadgeBenchmark, DxForgeReadinessBadgeCheck,
        DxForgeSmokeArtifacts,
    };

    #[test]
    fn forge_ci_page_renders_failing_readiness_quality_and_budget_honestly() {
        let smoke = failing_forge_ci_smoke();
        let badge = failing_forge_ci_badge(&smoke);

        let source = forge_ci_page_source(&smoke, &badge);
        let claims = forge_ci_claims_manifest(&smoke, &badge);
        let budget_claim = claims
            .claims
            .iter()
            .find(|claim| claim.id == "ci-budget-gate")
            .expect("budget claim");
        let claims_json = serde_json::to_string_pretty(&claims).expect("claims json");

        assert!(source.contains("DX Forge CI evidence"));
        assert!(source.contains("<h2 class=\"mt-2 text-2xl font-semibold\">failing</h2>"));
        assert!(source.contains("Score 61 / 100"));
        assert!(source.contains("Smoke model"));
        assert!(source.contains("55 / 100"));
        assert!(source.contains("No node_modules no"));
        assert!(source.contains("Launch quality 0 / 100, passed false"));
        assert!(source.contains("release-ready no"));
        assert!(source.contains("Latest /forge benchmark snapshot is not release-ready."));
        assert!(source.contains("Forge launch page quality failed: headings"));
        assert!(source.contains("Budget gate exceeded decoded byte threshold."));
        assert!(claims_json.contains("\"ci-readiness-badge\""));
        assert!(claims_json.contains("\"ci-launch-page-quality\""));
        assert!(claims_json.contains("\"ci-budget-gate\""));
        assert_eq!(budget_claim.verification_status, "needs-review");
        assert!(budget_claim.claim.contains("release-ready no"));
        assert!(budget_claim.evidence.contains("decoded_bytes=99999"));
    }

    fn failing_forge_ci_smoke() -> DxForgeSmokeReport {
        let launch_page_quality = DxForgeLaunchPageQualityReport {
            passed: false,
            score: 0,
            headings: failed_quality_check("headings", "h1=0, h2=1"),
            seo: passed_quality_check("seo", "title=true, main=true, runtime_script=false"),
            links: passed_quality_check("links", "links=4, unsafe=0"),
            claims_manifest: failed_quality_check(
                "claims_manifest",
                "claims=4, verified=1, invalid_statuses=0, incomplete_claims=1",
            ),
            findings: vec![
                "headings (h1=0, h2=1)".to_string(),
                "claims_manifest (claims=4, verified=1, invalid_statuses=0, incomplete_claims=1)"
                    .to_string(),
            ],
        };

        DxForgeSmokeReport {
            project: PathBuf::from("G:/Temp/dx-forge-ci-failure-fixture"),
            generated_at: "2026-05-16T00:00:00Z".to_string(),
            passed: false,
            score: 55,
            no_node_modules: false,
            packages: Vec::new(),
            check_score: 72,
            check_traffic: "Yellow".to_string(),
            release_gate_score: 72,
            doctor_passed: false,
            doctor_score: 72,
            verify_passed: false,
            verify_score: 64,
            scorecard_score: 100,
            launch_artifacts: fixture_smoke_artifacts(),
            launch_page_quality,
            launch_gate_findings: Vec::new(),
            findings: vec![
                "node_modules was created during Forge smoke.".to_string(),
                "Forge launch page quality failed: headings; claims_manifest".to_string(),
            ],
        }
    }

    fn failing_forge_ci_badge(smoke: &DxForgeSmokeReport) -> DxForgeReadinessBadge {
        DxForgeReadinessBadge {
            schema_version: 1,
            generated_at: smoke.generated_at.clone(),
            project: smoke.project.clone(),
            label: "DX Forge".to_string(),
            status: "failing".to_string(),
            message: "review 61/100".to_string(),
            color: "red".to_string(),
            score: 61,
            passed: false,
            is_error: true,
            fail_under: 90,
            no_node_modules: false,
            smoke: DxForgeReadinessBadgeCheck {
                passed: false,
                score: smoke.score,
                summary:
                    "Smoke packages=0, check=72, doctor=false, verify=false, no_node_modules=false"
                        .to_string(),
            },
            evidence: DxForgeReadinessBadgeCheck {
                passed: false,
                score: 72,
                summary: "Release evidence traffic=Yellow, rollback=60%, docs=50%".to_string(),
            },
            scorecard: DxForgeReadinessBadgeCheck {
                passed: true,
                score: 100,
                summary: "Scorecard packages=3, verified=3".to_string(),
            },
            launch_page_quality: DxForgeReadinessBadgeCheck {
                passed: false,
                score: smoke.launch_page_quality.score,
                summary: "headings=false, seo=true, links=true, claims=false".to_string(),
            },
            latest_forge_route_benchmark: Some(DxForgeReadinessBadgeBenchmark {
                passed: false,
                generated_at: Some("2026-05-16T00:00:00Z".to_string()),
                fixture_mode: Some("forge-site".to_string()),
                route_delivery: Some("static".to_string()),
                forge_packages: Some(2),
                forge_files_tracked: Some(7),
                decoded_bytes: Some(99_999),
                brotli_bytes: Some(24_000),
                http_route_median_ms: Some(38.2),
                chrome_load_event_ms: Some(220.0),
            }),
            artifacts: DxForgeReadinessBadgeArtifacts {
                smoke_report: Some(PathBuf::from("forge-smoke.json")),
                release_evidence: PathBuf::from("forge-evidence.json"),
                package_scorecard: PathBuf::from("forge-scorecard.json"),
                benchmark_history: PathBuf::from("forge-benchmark-history.json"),
                launch_html: PathBuf::from("forge.html"),
                launch_claims: PathBuf::from("forge.claims.json"),
                launch_evidence_model: PathBuf::from("forge.evidence.json"),
            },
            findings: vec![
                "Latest /forge benchmark snapshot is not release-ready.".to_string(),
                "Budget gate exceeded decoded byte threshold.".to_string(),
                "Release-readiness score 61 is below required threshold 90.".to_string(),
            ],
        }
    }

    fn fixture_smoke_artifacts() -> DxForgeSmokeArtifacts {
        DxForgeSmokeArtifacts {
            benchmark_history_path: PathBuf::from("forge-benchmark-history.json"),
            evidence_report_path: PathBuf::from("forge-evidence.json"),
            scorecard_report_path: PathBuf::from("forge-scorecard.json"),
            launch_source_path: PathBuf::from("forge-page.html"),
            launch_html_path: PathBuf::from("forge.html"),
            launch_packet_path: PathBuf::from("forge.dxp"),
            launch_runtime_path: None,
            launch_summary_path: PathBuf::from("forge-proof.json"),
            launch_claims_path: PathBuf::from("forge.claims.json"),
            launch_evidence_model_path: PathBuf::from("forge.evidence.json"),
        }
    }

    fn passed_quality_check(
        message: impl Into<String>,
        evidence: impl Into<String>,
    ) -> DxForgeLaunchPageQualityCheck {
        DxForgeLaunchPageQualityCheck {
            passed: true,
            message: message.into(),
            evidence: evidence.into(),
        }
    }

    fn failed_quality_check(
        message: impl Into<String>,
        evidence: impl Into<String>,
    ) -> DxForgeLaunchPageQualityCheck {
        DxForgeLaunchPageQualityCheck {
            passed: false,
            message: message.into(),
            evidence: evidence.into(),
        }
    }
}

fn forge_quickstart_page_source() -> String {
    r#"
<page>
    <main class="min-h-screen bg-neutral-950 px-6 py-10 text-neutral-50">
        <section class="mx-auto grid max-w-5xl gap-8">
            <div class="grid gap-4 border-b border-neutral-800 pb-8">
                <p class="text-sm font-medium uppercase text-neutral-400">DX Forge public beta</p>
                <h1 class="max-w-3xl text-5xl font-semibold">DX Forge Public Beta Quickstart</h1>
                <p class="max-w-3xl text-lg text-neutral-300">
                    Start with <code>dx forge init-app --write</code>, review the strict check, then publish only evidence-backed beta routes.
                    This path proves no node_modules and is not a universal npm replacement.
                </p>
                <p class="text-sm text-neutral-400">
                    Full operator guide: <code>docs/forge-public-beta-quickstart.md</code>.
                </p>
            </div>

            <section class="grid gap-4 md:grid-cols-3">
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">1. Create the beta app</p>
                    <h2 class="mt-2 text-2xl font-semibold">Init from source-owned packages</h2>
                    <p class="mt-3 text-sm text-neutral-300"><code>dx forge init-app --write</code></p>
                    <p class="mt-3 text-sm text-neutral-400">Writes the clean app scaffold, launch packages, package docs, receipts, scorecard, and strict check artifacts.</p>
                </article>
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">2. Verify before publishing</p>
                    <h2 class="mt-2 text-2xl font-semibold">Strict Forge check</h2>
                    <p class="mt-3 text-sm text-neutral-300"><code>dx check . --strict-forge --fail-under 90</code></p>
                    <p class="mt-3 text-sm text-neutral-400">No node_modules, source manifest, receipts, rollback coverage, and package docs remain reviewable.</p>
                </article>
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">3. Build beta evidence</p>
                    <h2 class="mt-2 text-2xl font-semibold">CI plus package review</h2>
                    <p class="mt-3 text-sm text-neutral-300"><code>dx forge ci</code></p>
                    <p class="mt-3 text-sm text-neutral-300"><code>node .\benchmarks\measure-forge-source-owned-package-review.ts</code></p>
                </article>
            </section>

            <section class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                <p class="text-sm font-medium text-neutral-400">Public beta route command</p>
                <h2 class="mt-2 text-2xl font-semibold">Generate this route without a client runtime</h2>
                <p class="mt-3 text-sm text-neutral-300">
                    <code>dx prove vertical --fixture forge-quickstart --out public --write</code>
                </p>
                <p class="mt-3 text-sm text-neutral-400">
                    The route is static, writes <code>forge/quickstart.dxp</code>, and does not write <code>forge/quickstart.dxp.js</code>.
                </p>
            </section>

            <section class="grid gap-4 md:grid-cols-2">
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Evidence to keep</p>
                    <ul class="mt-4 grid gap-3 text-sm text-neutral-300">
                        <li><code>.dx/forge/init-app/dx-check.json</code></li>
                        <li><code>.dx/forge/init-app/forge-scorecard.json</code></li>
                        <li><code>.dx/forge/source-.dx/build-cache/manifest.json</code></li>
                        <li><code>.dx/forge/docs</code> and <code>.dx/forge/receipts</code></li>
                    </ul>
                </article>
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Honest scope</p>
                    <ul class="mt-4 grid gap-3 text-sm text-neutral-300">
                        <li>Public beta evidence is local and reproducible.</li>
                        <li>No node_modules is required by the Forge onboarding path.</li>
                        <li>This route does not claim full Next.js, npm, WordPress, or framework replacement.</li>
                        <li>Secret markers such as <code>CLOUDFLARE_R2_</code> stay out of public artifacts.</li>
                    </ul>
                </article>
            </section>
        </section>
    </main>
</page>
"#
    .to_string()
}

fn forge_quickstart_claims_manifest() -> DxForgeLaunchClaimsManifest {
    DxForgeLaunchClaimsManifest {
        version: 1,
        route: "/forge/quickstart".to_string(),
        generated_at: chrono::Utc::now().to_rfc3339(),
        claims: vec![
            launch_claim(
                "quickstart-starts-with-init-app",
                "The public beta quickstart starts with `dx forge init-app --write` before CI or public route publishing.",
                "DxForgePublicBetaQuickstart",
                "commands.init_app",
                "declared",
                "The route and docs both name `dx forge init-app --write` as the first onboarding command.",
            ),
            launch_claim(
                "quickstart-ci-and-review",
                "The public beta quickstart links strict Forge checks, `dx forge ci`, and source-owned package review evidence.",
                "DxForgePublicBetaQuickstart",
                "commands.verify; commands.ci; commands.package_review",
                "declared",
                "The route lists `dx check --strict-forge`, `dx forge ci`, and `measure-forge-source-owned-package-review.ts`.",
            ),
            launch_claim(
                "quickstart-no-node-modules",
                "The public beta quickstart keeps no-node_modules evidence visible as a required beta boundary.",
                "DxForgePublicBetaQuickstart",
                "evidence.no_node_modules",
                "declared",
                "The route lists `.dx/forge/init-app` artifacts and the no-node_modules review boundary.",
            ),
            launch_claim(
                "quickstart-honest-scope",
                "The public beta quickstart does not claim full Next.js, npm, WordPress, or framework replacement.",
                "DxForgePublicBetaQuickstart",
                "honest_scope[]",
                "declared",
                "The route displays the honest scope beside the beta commands.",
            ),
        ],
    }
}

fn forge_adoption_preview_page_source() -> String {
    r#"
<page>
    <main class="min-h-screen bg-neutral-950 px-6 py-10 text-neutral-50">
        <section class="mx-auto grid max-w-5xl gap-8">
            <div class="grid gap-4 border-b border-neutral-800 pb-8">
                <p class="text-sm font-medium uppercase text-neutral-400">DX Forge adoption evidence</p>
                <h1 class="max-w-3xl text-5xl font-semibold">DX Forge Adoption Report</h1>
                <p class="max-w-3xl text-lg text-neutral-300">
                    This route is generated from a real DxForgeAdoptionReport when written inside an adoption project.
                </p>
            </div>
        </section>
    </main>
</page>
"#
    .to_string()
}

fn forge_adoption_page_source(report: &DxForgeAdoptionReport) -> String {
    let package_rows = report
        .packages
        .iter()
        .map(|package| {
            format!(
                r#"
                    <li class="rounded-md border border-neutral-800 bg-neutral-950 p-4">
                        <p class="text-sm font-medium text-neutral-200">{package_id}</p>
                        <p class="mt-2 text-sm text-neutral-400">Variant {variant}, version {version}, files {files}, docs {docs}.</p>
                    </li>
"#,
                package_id = escape_page_text(&package.package_id),
                variant = escape_page_text(&package.variant),
                version = escape_page_text(&package.version),
                files = package.file_count,
                docs = if package.docs_exists {
                    "present"
                } else {
                    "missing"
                }
            )
        })
        .collect::<String>();

    let route_rows = report
        .public_routes
        .iter()
        .map(|route| {
            format!(
                r#"
                    <li class="rounded-md border border-neutral-800 bg-neutral-950 p-4">
                        <p class="text-sm font-medium text-neutral-200">{route}</p>
                        <p class="mt-2 text-sm text-neutral-400">HTML {html}, packet {packet}, proof {proof}, passed {passed}.</p>
                    </li>
"#,
                route = escape_page_text(&route.route),
                html = escape_page_text(&route.html_path.display().to_string()),
                packet = escape_page_text(&route.packet_path.display().to_string()),
                proof = escape_page_text(&route.proof_path.display().to_string()),
                passed = route.passed
            )
        })
        .collect::<String>();

    let scope_rows = report
        .honest_scope
        .iter()
        .map(|scope| {
            format!(
                r#"
                    <li class="rounded-md border border-neutral-800 bg-neutral-950 p-3">{}</li>
"#,
                escape_page_text(scope)
            )
        })
        .collect::<String>();

    let finding_rows = if report.findings.is_empty() {
        r#"<li class="rounded-md border border-emerald-900 bg-emerald-950 p-3 text-emerald-100">No adoption-report findings.</li>"#
            .to_string()
    } else {
        report
            .findings
            .iter()
            .map(|finding| {
                format!(
                    r#"
                    <li class="rounded-md border border-amber-900 bg-amber-950 p-3 text-amber-100">{}</li>
"#,
                    escape_page_text(finding)
                )
            })
            .collect::<String>()
    };

    format!(
        r#"
<page>
    <main class="min-h-screen bg-neutral-950 px-6 py-10 text-neutral-50">
        <section class="mx-auto grid max-w-5xl gap-8">
            <div class="grid gap-4 border-b border-neutral-800 pb-8">
                <p class="text-sm font-medium uppercase text-neutral-400">DX Forge adoption evidence</p>
                <h1 class="max-w-3xl text-5xl font-semibold">DX Forge Adoption Report</h1>
                <p class="max-w-3xl text-lg text-neutral-300">
                    This compiler-generated route is backed by the real DxForgeAdoptionReport for this project.
                    It shows source-owned packages, receipts, public artifacts, strict Forge status, release bundle evidence, and No node_modules proof.
                </p>
                <p class="text-sm text-neutral-300">Score: {score} / 100. Passed: {passed}. Generated: {generated_at}.</p>
            </div>

            <section class="grid gap-4 md:grid-cols-4">
                <div class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm text-neutral-400">Source-owned packages</p>
                    <h2 class="mt-2 text-3xl font-semibold">{package_count}</h2>
                    <p class="mt-2 text-sm text-neutral-300">Docs present {docs_present}, receipts {receipt_count}.</p>
                </div>
                <div class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm text-neutral-400">DX check</p>
                    <h2 class="mt-2 text-3xl font-semibold">{dx_check_score}</h2>
                    <p class="mt-2 text-sm text-neutral-300">Strict Forge gate {strict_forge}.</p>
                </div>
                <div class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm text-neutral-400">Release bundle</p>
                    <h2 class="mt-2 text-3xl font-semibold">{bundle_score}</h2>
                    <p class="mt-2 text-sm text-neutral-300">Routes {bundle_routes}, artifacts {bundle_artifacts}.</p>
                </div>
                <div class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm text-neutral-400">No node_modules</p>
                    <h2 class="mt-2 text-3xl font-semibold">{no_node_modules}</h2>
                    <p class="mt-2 text-sm text-neutral-300">Forge materialized editable source files only.</p>
                </div>
            </section>

            <section class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                <p class="text-sm font-medium text-neutral-400">Reproducible commands</p>
                <h2 class="mt-2 text-2xl font-semibold">Clean-project adoption path</h2>
                <ul class="mt-4 grid gap-3 text-sm text-neutral-300">
                    <li><code>dx forge adoption-smoke --project .dx/adoption-app --fail-under 90</code></li>
                    <li><code>dx forge adoption-report --project .dx/adoption-app --fail-under 90</code></li>
                    <li><code>dx forge release-bundle --project .dx/adoption-app --out .dx/adoption-app/.dx/forge/adoption-smoke/release-bundle --fail-under 90</code></li>
                </ul>
                <p class="mt-4 text-sm text-neutral-400">The app route <code>pages/forge-adoption.html</code> references shadcn/ui/button, dx/icon/search, auth/better-auth, and auth/better-auth together.</p>
            </section>

            <section class="grid gap-4 md:grid-cols-2">
                <div class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Packages</p>
                    <ul class="mt-4 grid gap-3">{package_rows}</ul>
                </div>
                <div class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">Public artifacts</p>
                    <ul class="mt-4 grid gap-3">{route_rows}</ul>
                </div>
            </section>

            <section class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                <p class="text-sm font-medium text-neutral-400">Evidence paths</p>
                <h2 class="mt-2 text-2xl font-semibold">Reviewable local proof</h2>
                <ul class="mt-4 grid gap-3 text-sm text-neutral-300">
                    <li>Source manifest: <code>{source_manifest}</code></li>
                    <li>Package docs: <code>{package_docs}</code></li>
                    <li>Public dir: <code>{public_dir}</code></li>
                    <li>Release bundle: <code>{release_bundle}</code></li>
                </ul>
            </section>

            <section class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                <p class="text-sm font-medium text-neutral-400">Findings</p>
                <ul class="mt-4 grid gap-3 text-sm">{finding_rows}</ul>
            </section>

            <section class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                <p class="text-sm font-medium text-neutral-400">Honest launch boundaries</p>
                <ul class="mt-4 grid gap-3 text-sm text-neutral-300">{scope_rows}</ul>
            </section>
        </section>
    </main>
</page>
"#,
        score = report.score,
        passed = report.passed,
        generated_at = escape_page_text(&report.generated_at),
        package_count = report.package_count,
        docs_present = report.package_docs_present,
        receipt_count = report.receipt_count,
        dx_check_score = report.dx_check.score,
        strict_forge = report.dx_check.strict_forge_passed,
        bundle_score = report.release_bundle.score,
        bundle_routes = report.release_bundle.route_count,
        bundle_artifacts = report.release_bundle.artifact_count,
        no_node_modules = report.no_node_modules,
        package_rows = package_rows,
        route_rows = route_rows,
        source_manifest = escape_page_text(&report.source_manifest_path.display().to_string()),
        package_docs = escape_page_text(&report.package_docs_dir.display().to_string()),
        public_dir = escape_page_text(&report.public_dir.display().to_string()),
        release_bundle = escape_page_text(&report.release_bundle.bundle_dir.display().to_string()),
        finding_rows = finding_rows,
        scope_rows = scope_rows
    )
}

fn forge_adoption_claims_manifest(report: &DxForgeAdoptionReport) -> DxForgeLaunchClaimsManifest {
    DxForgeLaunchClaimsManifest {
        version: 1,
        route: "/forge/adoption".to_string(),
        generated_at: report.generated_at.clone(),
        claims: vec![
            launch_claim(
                "adoption-no-node-modules",
                format!(
                    "The adoption report found no node_modules in the project or release bundle: {}.",
                    report.no_node_modules
                ),
                "DxForgeAdoptionReport",
                "no_node_modules",
                if report.no_node_modules {
                    "verified"
                } else {
                    "needs-review"
                },
                "Rendered from DxForgeAdoptionReport.no_node_modules.",
            ),
            launch_claim(
                "adoption-source-owned-packages",
                format!(
                    "The adoption project tracks {} source-owned packages with {} Forge receipts.",
                    report.package_count, report.receipt_count
                ),
                "DxForgeAdoptionReport",
                "packages[]; receipt_count",
                if report.package_count > 0 && report.receipt_count >= report.package_count {
                    "verified"
                } else {
                    "needs-review"
                },
                "Rendered from DxForgeAdoptionReport package and receipt metrics.",
            ),
            launch_claim(
                "adoption-package-docs",
                format!(
                    "{} package docs are present and {} are missing.",
                    report.package_docs_present, report.package_docs_missing
                ),
                "DxForgeAdoptionReport",
                "package_docs_present; package_docs_missing",
                if report.package_docs_missing == 0 {
                    "verified"
                } else {
                    "needs-review"
                },
                "Rendered from DxForgeAdoptionReport package-doc metrics.",
            ),
            launch_claim(
                "adoption-public-routes",
                format!(
                    "{} public route artifact sets are recorded for the adoption project.",
                    report.public_routes.len()
                ),
                "DxForgeAdoptionReport",
                "public_routes[]",
                if report.public_routes.iter().all(|route| route.passed) {
                    "verified"
                } else {
                    "needs-review"
                },
                "Rendered from DxForgeAdoptionReport.public_routes.",
            ),
            launch_claim(
                "adoption-release-bundle",
                format!(
                    "The adoption release bundle score is {} / 100 and passed status is {}.",
                    report.release_bundle.score, report.release_bundle.passed
                ),
                "DxForgeAdoptionReport",
                "release_bundle",
                if report.release_bundle.passed {
                    "verified"
                } else {
                    "needs-review"
                },
                "Rendered from DxForgeAdoptionReport.release_bundle.",
            ),
            launch_claim(
                "adoption-honest-scope",
                "The route states that Forge adoption evidence is local and not a universal npm replacement claim.",
                "DxForgeAdoptionReport",
                "honest_scope[]",
                "declared",
                "Rendered from DxForgeAdoptionReport.honest_scope.",
            ),
        ],
    }
}

fn forge_scorecard_page_source(report: &DxForgePackageScorecardReport) -> String {
    let package_cards = report
        .packages
        .iter()
        .map(|package| {
            format!(
                r#"
                <article class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                    <p class="text-sm font-medium text-neutral-400">{package_id}</p>
                    <h2 class="mt-2 text-xl font-semibold">{description}</h2>
                    <p class="mt-3 text-sm text-neutral-300">Version {version}, {files} files, {bytes} source bytes, integrity {integrity}.</p>
                    <p class="mt-3 text-sm text-neutral-300">Provenance {provenance_source}, verified {provenance_verified}. Advisories {advisory_provider}, live coverage {advisory_live}. License review {license_reviewed}.</p>
                    <p class="mt-3 text-sm text-neutral-300">{claim}</p>
                    <p class="mt-3 text-sm text-neutral-400">{boundary}</p>
                </article>
"#,
                package_id = escape_page_text(&package.package_id),
                description = escape_page_text(&package.description),
                version = escape_page_text(&package.version),
                files = package.file_count,
                bytes = package.total_bytes,
                integrity = if package.integrity_verified {
                    "verified"
                } else {
                    "failed"
                },
                provenance_source = escape_page_text(&package.provenance.source),
                provenance_verified = if package.provenance.verified {
                    "yes"
                } else {
                    "no"
                },
                advisory_provider = escape_page_text(&package.advisory_review.provider),
                advisory_live = if package.advisory_review.live_coverage {
                    "yes"
                } else {
                    "no"
                },
                license_reviewed = if package.license_review.reviewed {
                    "reviewed"
                } else {
                    "declared only"
                },
                claim = escape_page_text(&package.public_claim),
                boundary = escape_page_text(&package.launch_boundary)
            )
        })
        .collect::<String>();

    let boundaries = report
        .honest_boundaries
        .iter()
        .map(|boundary| {
            format!(
                r#"
                    <li class="rounded-md border border-neutral-800 bg-neutral-950 p-3">{}</li>
"#,
                escape_page_text(boundary)
            )
        })
        .collect::<String>();

    format!(
        r#"
<page>
    <main class="min-h-screen bg-neutral-950 px-6 py-10 text-neutral-50">
        <section class="mx-auto grid max-w-5xl gap-8">
            <div class="grid gap-4 border-b border-neutral-800 pb-8">
                <p class="text-sm font-medium uppercase text-neutral-400">DX Forge launch evidence</p>
                <h1 class="max-w-3xl text-5xl font-semibold">DX Forge Package Scorecard</h1>
                <p class="max-w-3xl text-lg text-neutral-300">
                    This compiler-generated route renders the current Forge package scorecard from the same data model used by dx forge scorecard.
                </p>
                <p class="text-sm text-neutral-300">Score: {score}. Packages tracked: {package_count}. Generated: {generated_at}.</p>
            </div>

            <section class="grid gap-4 md:grid-cols-3">
                {package_cards}
            </section>

            <section class="rounded-lg border border-neutral-800 bg-neutral-900 p-5">
                <p class="text-sm font-medium text-neutral-400">Honest launch boundaries</p>
                <h2 class="mt-2 text-2xl font-semibold">A source-owned package firewall, not a universal npm replacement yet</h2>
                <ul class="mt-4 grid gap-3 text-sm text-neutral-300">
                    {boundaries}
                </ul>
            </section>
        </section>
    </main>
</page>
"#,
        score = report.score,
        package_count = report.packages.len(),
        generated_at = escape_page_text(&report.generated_at),
        package_cards = package_cards,
        boundaries = boundaries
    )
}

fn escape_page_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
