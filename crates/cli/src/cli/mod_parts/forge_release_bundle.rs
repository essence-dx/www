const FORGE_CI_REQUIRED_ARTIFACTS: &[&str] = &[
    "forge-smoke.json",
    "forge-smoke.md",
    "forge-adoption-smoke.json",
    "forge-adoption-report.json",
    "forge-adoption-report.md",
    "forge-adoption-page.html",
    "forge-benchmark-history.json",
    "forge-evidence.json",
    "forge-page.html",
    "forge-proof.json",
    "forge-readiness-badge.json",
    "forge-scorecard.json",
    "forge-triage.md",
    "forge.claims.json",
    "forge.dxp",
    "forge.evidence.json",
    "forge.html",
    "forge/adoption.html",
    "forge/adoption/index.html",
    "forge/adoption.claims.json",
    "forge/adoption.dxp",
    "forge/adoption.proof.json",
];

fn verify_forge_ci_artifacts(
    artifact_dir: &Path,
) -> anyhow::Result<DxForgeCiArtifactVerificationReport> {
    let mut findings = Vec::new();
    let mut artifacts = Vec::new();
    let mut json_values = std::collections::BTreeMap::<String, serde_json::Value>::new();
    let mut penalty = 0u16;

    if !artifact_dir.is_dir() {
        findings.push(format!(
            "Artifact directory does not exist: {}",
            artifact_dir.display()
        ));
        penalty = 100;
    }

    for name in FORGE_CI_REQUIRED_ARTIFACTS {
        let path = artifact_dir.join(name);
        let exists = path.is_file();
        let bytes = path.metadata().map(|metadata| metadata.len()).unwrap_or(0);
        let mut messages = Vec::new();
        let mut valid_json = None;

        if !exists {
            messages.push("missing".to_string());
            penalty = penalty.saturating_add(12);
        } else {
            if bytes == 0 {
                messages.push("empty".to_string());
                penalty = penalty.saturating_add(12);
            }

            if name.ends_with(".json") {
                valid_json = Some(false);
                match std::fs::read(&path) {
                    Ok(raw) => match serde_json::from_slice::<serde_json::Value>(&raw) {
                        Ok(value) => {
                            valid_json = Some(true);
                            json_values.insert((*name).to_string(), value);
                        }
                        Err(error) => {
                            messages.push(format!("invalid json: {error}"));
                            penalty = penalty.saturating_add(12);
                        }
                    },
                    Err(error) => {
                        messages.push(format!("unreadable: {error}"));
                        penalty = penalty.saturating_add(12);
                    }
                }
            }

            if forge_ci_text_artifact(name) {
                match std::fs::read_to_string(&path) {
                    Ok(text) => {
                        if FORGE_PUBLIC_SECRET_MARKERS
                            .iter()
                            .any(|marker| text.contains(marker))
                        {
                            messages.push("contains secret-environment marker".to_string());
                            penalty = penalty.saturating_add(10);
                        }
                    }
                    Err(error) => {
                        messages.push(format!("unreadable text artifact: {error}"));
                        penalty = penalty.saturating_add(8);
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
        artifacts.push(DxForgeCiArtifactCheck {
            name: (*name).to_string(),
            path,
            exists,
            bytes,
            valid_json,
            passed,
            message,
        });
    }

    let routes = forge_ci_route_artifact_checks(artifact_dir, &mut findings, &mut penalty);

    forge_ci_json_gate(
        &json_values,
        "forge-smoke.json",
        "passed",
        true,
        "forge-smoke.json did not pass",
        &mut findings,
        &mut penalty,
    );
    forge_ci_json_gate(
        &json_values,
        "forge-smoke.json",
        "no_node_modules",
        true,
        "forge-smoke.json does not prove no node_modules",
        &mut findings,
        &mut penalty,
    );
    forge_ci_json_gate(
        &json_values,
        "forge-readiness-badge.json",
        "passed",
        true,
        "forge-readiness-badge.json did not pass",
        &mut findings,
        &mut penalty,
    );
    forge_ci_json_gate(
        &json_values,
        "forge-readiness-badge.json",
        "no_node_modules",
        true,
        "forge-readiness-badge.json does not prove no node_modules",
        &mut findings,
        &mut penalty,
    );
    forge_ci_json_gate(
        &json_values,
        "forge-adoption-smoke.json",
        "passed",
        true,
        "forge-adoption-smoke.json did not pass",
        &mut findings,
        &mut penalty,
    );
    forge_ci_json_gate(
        &json_values,
        "forge-adoption-smoke.json",
        "no_node_modules",
        true,
        "forge-adoption-smoke.json does not prove no node_modules",
        &mut findings,
        &mut penalty,
    );
    forge_ci_json_gate(
        &json_values,
        "forge-adoption-report.json",
        "passed",
        true,
        "forge-adoption-report.json did not pass",
        &mut findings,
        &mut penalty,
    );
    forge_ci_json_gate(
        &json_values,
        "forge-adoption-report.json",
        "no_node_modules",
        true,
        "forge-adoption-report.json does not prove no node_modules",
        &mut findings,
        &mut penalty,
    );

    let claims_valid = json_values
        .get("forge.claims.json")
        .and_then(|value| value.get("claims"))
        .and_then(|claims| claims.as_array())
        .is_some_and(|claims| {
            !claims.is_empty()
                && claims.iter().all(|claim| {
                    matches!(
                        claim
                            .get("verification_status")
                            .and_then(|status| status.as_str()),
                        Some("verified" | "declared" | "needs-review")
                    )
                })
        });
    if !claims_valid {
        findings.push("forge.claims.json has missing or invalid claim statuses".to_string());
        penalty = penalty.saturating_add(10);
    }

    let adoption_claims_valid = forge_pages_route_value(&json_values, "forge/adoption.claims.json")
        == Some("/forge/adoption")
        && forge_pages_claim_statuses_valid(&json_values, "forge/adoption.claims.json");
    if !adoption_claims_valid {
        findings.push(
            "forge/adoption.claims.json must target /forge/adoption with reviewable statuses"
                .to_string(),
        );
        penalty = penalty.saturating_add(10);
    }

    let adoption_proof_valid = forge_pages_route_value(&json_values, "forge/adoption.proof.json")
        == Some("/forge/adoption")
        && forge_pages_json_string_contains(
            &json_values,
            "forge/adoption.proof.json",
            "forge/adoption.html",
        )
        && forge_pages_json_string_contains(
            &json_values,
            "forge/adoption.proof.json",
            "forge/adoption.dxp",
        );
    if !adoption_proof_valid {
        findings.push(
            "forge/adoption.proof.json must target /forge/adoption and reference HTML plus DXPK"
                .to_string(),
        );
        penalty = penalty.saturating_add(10);
    }

    let score = 100u8.saturating_sub(penalty.min(100) as u8);
    let passed = findings.is_empty();

    Ok(DxForgeCiArtifactVerificationReport {
        artifact_dir: artifact_dir.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        artifacts,
        routes,
        findings,
    })
}

const FORGE_PAGES_REQUIRED_ARTIFACTS: &[&str] = &[
    "forge-readiness-badge.json",
    "forge/ci.html",
    "forge/ci/index.html",
    "forge/ci.claims.json",
    "forge/ci.dxp",
    "forge/ci.proof.json",
    "forge/releases.html",
    "forge/releases/index.html",
    "forge/releases.claims.json",
    "forge/releases.dxp",
    "forge/releases.proof.json",
    "forge/changelog.html",
    "forge/changelog/index.html",
    "forge/changelog.claims.json",
    "forge/changelog.dxp",
    "forge/changelog.proof.json",
    "forge/adoption.html",
    "forge/adoption/index.html",
    "forge/adoption.claims.json",
    "forge/adoption.dxp",
    "forge/adoption.proof.json",
    "proof.json",
];

#[derive(Debug, Clone, Copy)]
struct DxForgeReleaseBundleRouteSpec {
    route: &'static str,
    fixture: Option<&'static str>,
    html: &'static str,
    clean_index: &'static str,
    packet: &'static str,
    claims: Option<&'static str>,
    evidence: Option<&'static str>,
    proof: &'static str,
}

const FORGE_RELEASE_BUNDLE_ROUTES: &[DxForgeReleaseBundleRouteSpec] = &[
    DxForgeReleaseBundleRouteSpec {
        route: "/forge",
        fixture: None,
        html: "forge.html",
        clean_index: "forge/index.html",
        packet: "forge.dxp",
        claims: Some("forge.claims.json"),
        evidence: Some("forge.evidence.json"),
        proof: "forge-proof.json",
    },
    DxForgeReleaseBundleRouteSpec {
        route: "/forge/scorecard",
        fixture: Some("forge-scorecard"),
        html: "forge/scorecard.html",
        clean_index: "forge/scorecard/index.html",
        packet: "forge/scorecard.dxp",
        claims: None,
        evidence: None,
        proof: "forge/scorecard.proof.json",
    },
    DxForgeReleaseBundleRouteSpec {
        route: "/forge/ci",
        fixture: Some("forge-ci"),
        html: "forge/ci.html",
        clean_index: "forge/ci/index.html",
        packet: "forge/ci.dxp",
        claims: Some("forge/ci.claims.json"),
        evidence: None,
        proof: "forge/ci.proof.json",
    },
    DxForgeReleaseBundleRouteSpec {
        route: "/forge/evidence",
        fixture: Some("forge-evidence"),
        html: "forge/evidence.html",
        clean_index: "forge/evidence/index.html",
        packet: "forge/evidence.dxp",
        claims: Some("forge/evidence.claims.json"),
        evidence: None,
        proof: "forge/evidence.proof.json",
    },
    DxForgeReleaseBundleRouteSpec {
        route: "/forge/releases",
        fixture: Some("forge-releases"),
        html: "forge/releases.html",
        clean_index: "forge/releases/index.html",
        packet: "forge/releases.dxp",
        claims: Some("forge/releases.claims.json"),
        evidence: None,
        proof: "forge/releases.proof.json",
    },
    DxForgeReleaseBundleRouteSpec {
        route: "/forge/changelog",
        fixture: Some("forge-changelog"),
        html: "forge/changelog.html",
        clean_index: "forge/changelog/index.html",
        packet: "forge/changelog.dxp",
        claims: Some("forge/changelog.claims.json"),
        evidence: None,
        proof: "forge/changelog.proof.json",
    },
];

const FORGE_RELEASE_BUNDLE_ADOPTION_ROUTE: DxForgeReleaseBundleRouteSpec =
    DxForgeReleaseBundleRouteSpec {
        route: "/forge/adoption",
        fixture: None,
        html: "forge/adoption.html",
        clean_index: "forge/adoption/index.html",
        packet: "forge/adoption.dxp",
        claims: Some("forge/adoption.claims.json"),
        evidence: None,
        proof: "forge/adoption.proof.json",
    };

const FORGE_REQUIRED_PUBLIC_ROUTES: &[&str] = &[
    "/forge",
    "/forge/scorecard",
    "/forge/ci",
    "/forge/evidence",
    "/forge/releases",
    "/forge/changelog",
];

const FORGE_RELEASE_BUNDLE_ADOPTION_REPORT_ARTIFACTS: &[&str] = &[
    "forge-adoption-smoke.json",
    "forge-adoption-report.json",
    "forge-adoption-report.md",
    "forge-adoption-page.html",
];

const FORGE_RELEASE_BUNDLE_REPORTS: &[(&str, &str)] = &[
    (
        "benchmarks/reports/forge-public-route-comparison.json",
        "forge-public-route-comparison.json",
    ),
    (
        "benchmarks/reports/forge-public-route-comparison.md",
        "forge-public-route-comparison.md",
    ),
    (
        "benchmarks/reports/forge-public-release-history.json",
        "forge-public-release-history.json",
    ),
    (
        "benchmarks/reports/forge-public-release-history.md",
        "forge-public-release-history.md",
    ),
];

const FORGE_RELEASE_BUNDLE_LAUNCH_CHANGELOG_JSON: &str = "forge-public-launch-changelog.json";
const FORGE_RELEASE_BUNDLE_LAUNCH_CHANGELOG_MD: &str = "forge-public-launch-changelog.md";
const FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_HTML: &str = "forge/package-gallery/index.html";
const FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_JSON: &str = "forge/package-gallery.json";
const FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_MD: &str = "forge/package-gallery.md";
const FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_HTML: &str = "forge/migration-gallery/index.html";
const FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_JSON: &str = "forge/migration-gallery.json";
const FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_MD: &str = "forge/migration-gallery.md";
const FORGE_RELEASE_BUNDLE_MANIFEST_JSON: &str = "forge-release-manifest.json";
const FORGE_RELEASE_BUNDLE_MANIFEST_MD: &str = "forge-release-manifest.md";
const FORGE_RELEASE_BUNDLE_MANIFEST_INTEGRITY_SCHEME: &str = "dx-forge-release-manifest-v1";
const FORGE_RELEASE_BUNDLE_ARTIFACT_INTEGRITY_SCHEME: &str =
    "dx-forge-release-artifact-integrity-v1";
const FORGE_RELEASE_BUNDLE_PUBLISHER_IDENTITY_SCHEME: &str =
    "dx-forge-release-publisher-identity-v1";
const FORGE_RELEASE_BUNDLE_SIGNATURE_STATUS_UNSIGNED: &str = "unsigned";
const FORGE_RELEASE_BUNDLE_SIGNATURE_STATUS_SIGNED: &str = "signed";
const FORGE_PUBLISHER_KEY_SCHEME: &str = "dx-forge-release-publisher-key-v1";
const FORGE_PUBLISHER_PRIVATE_KEY_FILE: &str = "publisher-key.private.json";
const FORGE_PUBLISHER_PUBLIC_KEY_FILE: &str = "publisher-key.public.json";

const FORGE_PUBLIC_SECRET_MARKERS: &[&str] = &[
    "CLOUDFLARE_R2_",
    "DX_FORGE_R2_LIVE",
    "R2_SECRET",
    "SECRET_ACCESS_KEY",
];

fn build_forge_release_bundle(
    project: &Path,
    bundle_dir: &Path,
    fail_under: u8,
    include_adoption: bool,
) -> anyhow::Result<DxForgeReleaseBundleReport> {
    std::fs::create_dir_all(bundle_dir)?;

    let smoke = build_forge_smoke_report(project)?;
    write_forge_ci_artifacts(&smoke, bundle_dir, fail_under)?;
    copy_forge_release_bundle_public_reports(project, bundle_dir)?;
    write_forge_release_bundle_launch_changelog(project, bundle_dir)?;
    write_forge_release_bundle_package_gallery(project, bundle_dir, fail_under)?;
    if include_adoption {
        write_forge_ci_adoption_artifacts(project, bundle_dir, fail_under)?;
    }

    for route in forge_release_bundle_routes(include_adoption) {
        if let Some(fixture) = route.fixture {
            let fixture_project = prepare_forge_release_bundle_fixture_project(project, fixture)?;
            Cli::with_cwd(fixture_project)
                .cmd_prove(&[
                    "vertical".to_string(),
                    "--fixture".to_string(),
                    fixture.to_string(),
                    "--out".to_string(),
                    bundle_dir.to_string_lossy().into_owned(),
                    "--write".to_string(),
                    "--format".to_string(),
                    "json".to_string(),
                    "--quiet".to_string(),
                ])
                .map_err(|error| anyhow::anyhow!("dx prove vertical {fixture} failed: {error}"))?;
            copy_bundle_file(bundle_dir, "proof.json", route.proof)?;
        }
        copy_bundle_file(bundle_dir, route.html, route.clean_index)?;
    }

    let transient_proof = bundle_dir.join("proof.json");
    if transient_proof.is_file() {
        std::fs::remove_file(transient_proof)?;
    }
    write_forge_release_bundle_manifest(bundle_dir, include_adoption)?;

    verify_forge_release_bundle_with_options(bundle_dir, include_adoption)
}

fn write_forge_release_bundle_package_gallery(
    project: &Path,
    bundle_dir: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgePackageGalleryHostedIndex> {
    let report = build_forge_package_gallery_report(project, fail_under)?;
    write_forge_package_gallery_hosted_index(bundle_dir, &report)
}

fn forge_release_bundle_routes(include_adoption: bool) -> Vec<DxForgeReleaseBundleRouteSpec> {
    let mut routes = FORGE_RELEASE_BUNDLE_ROUTES.to_vec();
    if include_adoption {
        routes.push(FORGE_RELEASE_BUNDLE_ADOPTION_ROUTE);
    }
    routes
}

fn forge_required_public_routes(include_adoption: bool) -> Vec<&'static str> {
    let mut routes = FORGE_REQUIRED_PUBLIC_ROUTES.to_vec();
    if include_adoption {
        routes.push(FORGE_RELEASE_BUNDLE_ADOPTION_ROUTE.route);
    }
    routes
}

fn prepare_forge_release_bundle_fixture_project(
    source_project: &Path,
    fixture: &str,
) -> anyhow::Result<PathBuf> {
    let fixture_project = std::env::temp_dir().join(format!(
        "dx-forge-release-bundle-{}-{}",
        fixture,
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(&fixture_project)?;

    if matches!(fixture, "forge-releases" | "forge-changelog") {
        let source = source_project.join("benchmarks/reports/forge-public-release-history.json");
        let destination =
            fixture_project.join("benchmarks/reports/forge-public-release-history.json");
        if let Some(parent) = destination.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::copy(&source, &destination)?;
    }

    Ok(fixture_project)
}

fn copy_forge_release_bundle_public_reports(
    project: &Path,
    bundle_dir: &Path,
) -> anyhow::Result<Vec<PathBuf>> {
    let mut written = Vec::new();
    for (source_relative, destination_relative) in FORGE_RELEASE_BUNDLE_REPORTS {
        let source = project.join(source_relative);
        let destination = bundle_dir.join(destination_relative);
        if let Some(parent) = destination.parent() {
            std::fs::create_dir_all(parent)?;
        }
        if source.is_file() {
            std::fs::copy(&source, &destination)?;
        } else if *destination_relative == "forge-public-route-comparison.md" {
            let json_path = project.join("benchmarks/reports/forge-public-route-comparison.json");
            std::fs::write(
                &destination,
                forge_public_route_comparison_bundle_markdown(&json_path)?,
            )?;
        } else if *destination_relative == "forge-public-release-history.md" {
            let json_path = project.join("benchmarks/reports/forge-public-release-history.json");
            let history: DxForgePublicReleaseHistory =
                serde_json::from_slice(&std::fs::read(&json_path)?)?;
            std::fs::write(
                &destination,
                forge_public_release_history_markdown(&history),
            )?;
        } else {
            anyhow::bail!(
                "Release bundle source artifact is missing: {}",
                source.display()
            );
        }
        written.push(destination);
    }
    Ok(written)
}

fn write_forge_release_bundle_launch_changelog(
    project: &Path,
    bundle_dir: &Path,
) -> anyhow::Result<()> {
    let history_path = project.join("benchmarks/reports/forge-public-release-history.json");
    let report = build_forge_launch_changelog_report(DxForgeLaunchChangelogInput { history_path })?;
    std::fs::write(
        bundle_dir.join(FORGE_RELEASE_BUNDLE_LAUNCH_CHANGELOG_JSON),
        serde_json::to_vec_pretty(&report)?,
    )?;
    std::fs::write(
        bundle_dir.join(FORGE_RELEASE_BUNDLE_LAUNCH_CHANGELOG_MD),
        forge_launch_changelog_markdown(&report),
    )?;
    Ok(())
}

fn forge_public_route_comparison_bundle_markdown(json_path: &Path) -> anyhow::Result<String> {
    let value: serde_json::Value = serde_json::from_slice(&std::fs::read(json_path)?)?;
    let generated_at = value
        .get("generated_at")
        .and_then(|value| value.as_str())
        .unwrap_or("unknown");
    let route_count = value
        .get("route_count")
        .and_then(|value| value.as_u64())
        .unwrap_or_default();
    let total_decoded = value
        .get("total_decoded_bytes")
        .and_then(|value| value.as_u64())
        .unwrap_or_default();
    let total_brotli = value
        .get("total_brotli_bytes")
        .and_then(|value| value.as_u64())
        .unwrap_or_default();
    let mut output = format!(
        "# Forge Public Route Comparison\n\nGenerated: {generated_at}\n\n- Routes: {route_count}\n- Total decoded bytes: {total_decoded} B\n- Total Brotli estimate: {total_brotli} B\n\n"
    );
    output.push_str("| Route | Fixture | Delivery | Decoded | Brotli | Budget |\n");
    output.push_str("| --- | --- | --- | ---: | ---: | --- |\n");
    if let Some(routes) = value.get("routes").and_then(|value| value.as_array()) {
        for route in routes {
            output.push_str(&format!(
                "| {} | {} | {} | {} B | {} B | {} |\n",
                route
                    .get("route")
                    .and_then(|value| value.as_str())
                    .unwrap_or("unknown"),
                route
                    .get("fixture_mode")
                    .and_then(|value| value.as_str())
                    .unwrap_or("unknown"),
                route
                    .get("route_delivery")
                    .and_then(|value| value.as_str())
                    .unwrap_or("unknown"),
                route
                    .get("decoded_bytes")
                    .and_then(|value| value.as_u64())
                    .unwrap_or_default(),
                route
                    .get("brotli_bytes")
                    .and_then(|value| value.as_u64())
                    .unwrap_or_default(),
                route
                    .get("budget_passed")
                    .and_then(|value| value.as_bool())
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "n/a".to_string())
            ));
        }
    }
    Ok(output)
}

fn copy_bundle_file(
    bundle_dir: &Path,
    source_relative: &str,
    destination_relative: &str,
) -> anyhow::Result<PathBuf> {
    let source = bundle_dir.join(source_relative);
    let destination = bundle_dir.join(destination_relative);
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::copy(&source, &destination)?;
    Ok(destination)
}

fn write_forge_release_bundle_manifest(
    bundle_dir: &Path,
    include_adoption: bool,
) -> anyhow::Result<PathBuf> {
    let manifest = build_forge_release_bundle_manifest(bundle_dir, include_adoption)?;
    let json_path = bundle_dir.join(FORGE_RELEASE_BUNDLE_MANIFEST_JSON);
    let markdown_path = bundle_dir.join(FORGE_RELEASE_BUNDLE_MANIFEST_MD);
    std::fs::write(&json_path, serde_json::to_vec_pretty(&manifest)?)?;
    std::fs::write(
        &markdown_path,
        forge_release_bundle_manifest_markdown(&manifest),
    )?;
    Ok(json_path)
}

fn build_forge_release_bundle_manifest(
    bundle_dir: &Path,
    include_adoption: bool,
) -> anyhow::Result<DxForgeReleaseBundleManifest> {
    let mut artifacts = Vec::new();
    for relative in forge_release_bundle_manifest_artifact_paths(include_adoption) {
        let path = bundle_dir.join(&relative);
        let raw = std::fs::read(&path).map_err(|error| {
            anyhow::anyhow!(
                "Release bundle manifest could not read `{}`: {error}",
                relative
            )
        })?;
        artifacts.push(DxForgeReleaseBundleManifestArtifact {
            artifact_type: forge_release_bundle_artifact_type(&relative).to_string(),
            route: forge_release_bundle_artifact_route(&relative, include_adoption),
            bytes: raw.len() as u64,
            blake3: blake3::hash(&raw).to_hex().to_string(),
            path: relative,
        });
    }
    artifacts.sort_by(|left, right| left.path.cmp(&right.path));
    let digest = forge_release_bundle_manifest_digest(&artifacts)?;
    let artifact_count = artifacts.len();
    Ok(DxForgeReleaseBundleManifest {
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        artifact_count,
        hash_algorithm: "blake3".to_string(),
        artifacts,
        integrity: DxForgeReleaseBundleManifestIntegrity {
            scheme: FORGE_RELEASE_BUNDLE_MANIFEST_INTEGRITY_SCHEME.to_string(),
            signed: false,
            digest: digest.clone(),
            signature: None,
            message: "Unsigned review manifest: BLAKE3 artifact hashes are verified locally before publish."
                .to_string(),
        },
        artifact_integrity: DxForgeReleaseBundleArtifactIntegrity {
            scheme: FORGE_RELEASE_BUNDLE_ARTIFACT_INTEGRITY_SCHEME.to_string(),
            hash_algorithm: "blake3".to_string(),
            digest,
            artifact_count,
            verified_locally: true,
            message: "Local BLAKE3 artifact integrity covers every release-bundle file before publish."
                .to_string(),
        },
        publisher_identity: DxForgeReleaseBundlePublisherIdentity {
            scheme: FORGE_RELEASE_BUNDLE_PUBLISHER_IDENTITY_SCHEME.to_string(),
            status: FORGE_RELEASE_BUNDLE_SIGNATURE_STATUS_UNSIGNED.to_string(),
            signer: None,
            key_id: None,
            algorithm: None,
            public_key: None,
            signature: None,
            signed_at: None,
            message: "Publisher identity is not attached yet; this manifest records BLAKE3 artifact integrity only."
                .to_string(),
        },
    })
}

fn forge_release_bundle_manifest_digest(
    artifacts: &[DxForgeReleaseBundleManifestArtifact],
) -> anyhow::Result<String> {
    Ok(blake3::hash(&serde_json::to_vec(artifacts)?)
        .to_hex()
        .to_string())
}

fn forge_release_bundle_manifest_markdown(manifest: &DxForgeReleaseBundleManifest) -> String {
    let mut output = format!(
        "# DX Forge Release Bundle Manifest\n\nGenerated: `{}`\n\n- Artifacts: `{}`\n- Hash algorithm: `{}`\n- Integrity scheme: `{}`\n- Signed: `{}`\n- Manifest digest: `{}`\n- Artifact integrity scheme: `{}`\n- Artifact integrity digest: `{}`\n- Artifact integrity verified locally: `{}`\n- Publisher identity scheme: `{}`\n- Publisher identity status: `{}`\n- Publisher signer: `{}`\n- Publisher key id: `{}`\n- Publisher algorithm: `{}`\n- Publisher public key: `{}`\n- Publisher signature: `{}`\n\n",
        manifest.generated_at,
        manifest.artifact_count,
        manifest.hash_algorithm,
        manifest.integrity.scheme,
        manifest.integrity.signed,
        manifest.integrity.digest,
        manifest.artifact_integrity.scheme,
        manifest.artifact_integrity.digest,
        manifest.artifact_integrity.verified_locally,
        manifest.publisher_identity.scheme,
        manifest.publisher_identity.status,
        manifest
            .publisher_identity
            .signer
            .as_deref()
            .unwrap_or("not attached"),
        manifest
            .publisher_identity
            .key_id
            .as_deref()
            .unwrap_or("not attached"),
        manifest
            .publisher_identity
            .algorithm
            .as_deref()
            .unwrap_or("not attached"),
        manifest
            .publisher_identity
            .public_key
            .as_deref()
            .unwrap_or("not attached"),
        manifest
            .publisher_identity
            .signature
            .as_deref()
            .unwrap_or("not attached")
    );
    output.push_str("| Artifact | Type | Route | Bytes | BLAKE3 |\n");
    output.push_str("| --- | --- | --- | ---: | --- |\n");
    for artifact in &manifest.artifacts {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} | `{}` |\n",
            artifact.path,
            artifact.artifact_type,
            artifact.route.as_deref().unwrap_or("global"),
            artifact.bytes,
            artifact.blake3
        ));
    }
    output
}

fn forge_release_bundle_manifest_artifact_paths(include_adoption: bool) -> Vec<String> {
    let mut artifacts = forge_release_bundle_required_artifacts(include_adoption)
        .into_iter()
        .filter(|artifact| {
            artifact != FORGE_RELEASE_BUNDLE_MANIFEST_JSON
                && artifact != FORGE_RELEASE_BUNDLE_MANIFEST_MD
        })
        .collect::<Vec<_>>();
    artifacts.sort();
    artifacts.dedup();
    artifacts
}

fn forge_release_bundle_artifact_type(relative: &str) -> &'static str {
    if relative.ends_with(".dxp") {
        "dxpk-packet"
    } else if relative.ends_with(".claims.json") {
        "claims"
    } else if relative.contains("release-history") {
        "release-history"
    } else if relative.contains("launch-changelog") {
        "launch-changelog"
    } else if relative.starts_with("forge/migration-gallery") {
        "migration-gallery"
    } else if relative.starts_with("forge/package-gallery") {
        "package-gallery"
    } else if relative.contains("route-comparison") {
        "route-comparison"
    } else if relative.contains("evidence") {
        "evidence"
    } else if relative.ends_with(".proof.json") || relative == "forge-proof.json" {
        "proof"
    } else if relative.ends_with(".html") {
        "route-html"
    } else if relative.ends_with(".md") {
        "markdown-report"
    } else if relative.ends_with(".json") {
        "json-report"
    } else if relative.ends_with(".html") {
        "source-page"
    } else {
        "artifact"
    }
}

fn forge_release_bundle_artifact_route(relative: &str, include_adoption: bool) -> Option<String> {
    if matches!(
        relative,
        FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_HTML
            | FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_JSON
            | FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_MD
    ) {
        return Some("/forge/package-gallery/".to_string());
    }
    if matches!(
        relative,
        FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_HTML
            | FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_JSON
            | FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_MD
    ) {
        return Some("/forge/migration-gallery/".to_string());
    }

    for route in forge_release_bundle_routes(include_adoption) {
        if relative == route.html
            || relative == route.clean_index
            || relative == route.packet
            || relative == route.proof
            || route.claims == Some(relative)
            || route.evidence == Some(relative)
        {
            return Some(route.route.to_string());
        }
    }
    None
}

fn verify_forge_release_bundle(bundle_dir: &Path) -> anyhow::Result<DxForgeReleaseBundleReport> {
    verify_forge_release_bundle_with_options(bundle_dir, false)
}

fn verify_forge_release_bundle_with_options(
    bundle_dir: &Path,
    require_adoption: bool,
) -> anyhow::Result<DxForgeReleaseBundleReport> {
    let mut findings = Vec::new();
    let mut artifacts = Vec::new();
    let mut json_values = std::collections::BTreeMap::<String, serde_json::Value>::new();
    let mut penalty = 0u16;
    let include_adoption = require_adoption || forge_release_bundle_includes_adoption(bundle_dir);

    if !bundle_dir.is_dir() {
        findings.push(format!(
            "Release bundle directory does not exist: {}",
            bundle_dir.display()
        ));
        penalty = 100;
    }

    for name in forge_release_bundle_required_artifacts(include_adoption) {
        let path = bundle_dir.join(&name);
        let exists = path.is_file();
        let bytes = path.metadata().map(|metadata| metadata.len()).unwrap_or(0);
        let mut messages = Vec::new();
        let mut valid_json = None;

        if !exists {
            messages.push("missing".to_string());
            penalty = penalty.saturating_add(8);
        } else {
            if bytes == 0 {
                messages.push("empty".to_string());
                penalty = penalty.saturating_add(8);
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
                            penalty = penalty.saturating_add(8);
                        }
                    },
                    Err(error) => {
                        messages.push(format!("unreadable: {error}"));
                        penalty = penalty.saturating_add(8);
                    }
                }
            }
            if name.ends_with(".dxp") {
                match std::fs::read(&path) {
                    Ok(raw) if raw.starts_with(b"DXPK") => {}
                    Ok(_) => {
                        messages.push("missing DXPK header".to_string());
                        penalty = penalty.saturating_add(8);
                    }
                    Err(error) => {
                        messages.push(format!("unreadable DXPK artifact: {error}"));
                        penalty = penalty.saturating_add(8);
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
        artifacts.push(DxForgeCiArtifactCheck {
            name,
            path,
            exists,
            bytes,
            valid_json,
            passed,
            message,
        });
    }

    let mut routes = forge_release_bundle_shape_checks(
        bundle_dir,
        &json_values,
        include_adoption,
        &mut findings,
        &mut penalty,
    );
    routes.extend(forge_release_bundle_secret_marker_checks(
        bundle_dir,
        &mut findings,
        &mut penalty,
    ));

    let score = 100u8.saturating_sub(penalty.min(100) as u8);
    let no_node_modules = !bundle_dir.join("node_modules").exists();
    let passed = findings.is_empty() && no_node_modules;

    Ok(DxForgeReleaseBundleReport {
        version: 1,
        bundle_dir: bundle_dir.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        artifact_count: artifacts.len(),
        route_count: forge_release_bundle_routes(include_adoption).len(),
        no_node_modules,
        artifacts,
        routes,
        findings,
    })
}

fn forge_release_bundle_includes_adoption(bundle_dir: &Path) -> bool {
    [
        "forge-adoption-report.json",
        "forge/adoption.html",
        "forge/adoption.claims.json",
        "forge/adoption.dxp",
        "forge/adoption.proof.json",
    ]
    .iter()
    .any(|relative| bundle_dir.join(relative).is_file())
}

fn forge_release_bundle_required_artifacts(include_adoption: bool) -> Vec<String> {
    let mut artifacts = vec![
        "forge-smoke.json",
        "forge-smoke.md",
        "forge-benchmark-history.json",
        "forge-evidence.json",
        "forge-page.html",
        "forge-proof.json",
        "forge-readiness-badge.json",
        "forge-scorecard.json",
        "forge-triage.md",
        "forge-public-route-comparison.json",
        "forge-public-route-comparison.md",
        "forge-public-release-history.json",
        "forge-public-release-history.md",
        FORGE_RELEASE_BUNDLE_LAUNCH_CHANGELOG_JSON,
        FORGE_RELEASE_BUNDLE_LAUNCH_CHANGELOG_MD,
        FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_HTML,
        FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_JSON,
        FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_MD,
        FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_HTML,
        FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_JSON,
        FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_MD,
        FORGE_RELEASE_BUNDLE_MANIFEST_JSON,
        FORGE_RELEASE_BUNDLE_MANIFEST_MD,
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<Vec<_>>();

    if include_adoption {
        artifacts.extend(
            FORGE_RELEASE_BUNDLE_ADOPTION_REPORT_ARTIFACTS
                .iter()
                .map(|artifact| (*artifact).to_string()),
        );
    }

    for route in forge_release_bundle_routes(include_adoption) {
        artifacts.push(route.html.to_string());
        artifacts.push(route.clean_index.to_string());
        artifacts.push(route.packet.to_string());
        if let Some(claims) = route.claims {
            artifacts.push(claims.to_string());
        }
        if let Some(evidence) = route.evidence {
            artifacts.push(evidence.to_string());
        }
        artifacts.push(route.proof.to_string());
    }
    artifacts
}

fn forge_release_bundle_shape_checks(
    bundle_dir: &Path,
    json_values: &std::collections::BTreeMap<String, serde_json::Value>,
    include_adoption: bool,
    findings: &mut Vec<String>,
    penalty: &mut u16,
) -> Vec<DxForgeCiRouteArtifactCheck> {
    let mut checks = Vec::new();

    for route in forge_release_bundle_routes(include_adoption) {
        let html_path = bundle_dir.join(route.html);
        let index_path = bundle_dir.join(route.clean_index);
        let clean_route_matches = match (std::fs::read(&html_path), std::fs::read(&index_path)) {
            (Ok(html), Ok(index)) => html == index,
            _ => false,
        };
        checks.push(forge_pages_named_check(
            route.route,
            vec![route.html, route.clean_index],
            clean_route_matches,
            "clean-route index matches route HTML",
            "clean-route index must exist and match route HTML",
            findings,
            penalty,
        ));

        if let Some(claims) = route.claims {
            checks.push(forge_pages_named_check(
                claims,
                vec![claims],
                forge_pages_route_value(json_values, claims) == Some(route.route)
                    && forge_pages_claim_statuses_valid(json_values, claims),
                "claims manifest route and statuses are reviewable",
                "claims manifest must target the route and use reviewable statuses",
                findings,
                penalty,
            ));
        }
    }

    checks.push(forge_pages_named_check(
        "readiness badge",
        vec!["forge-readiness-badge.json"],
        forge_pages_json_bool(json_values, "forge-readiness-badge.json", "passed")
            && forge_pages_json_bool(json_values, "forge-readiness-badge.json", "no_node_modules"),
        "badge reports passed and no node_modules",
        "forge-readiness-badge.json must report passed=true and no_node_modules=true",
        findings,
        penalty,
    ));

    checks.push(forge_pages_named_check(
        "public route comparison",
        vec!["forge-public-route-comparison.json"],
        forge_release_bundle_route_comparison_ready(json_values, include_adoption),
        "route comparison covers the required public surface",
        "route comparison must include all required public routes and passing budgets",
        findings,
        penalty,
    ));

    checks.push(forge_pages_named_check(
        "public release history",
        vec!["forge-public-release-history.json"],
        forge_release_bundle_history_ready(json_values, include_adoption),
        "release history has records and the latest required public route comparison",
        "release history must include at least one record with all required public routes",
        findings,
        penalty,
    ));

    checks.push(forge_pages_named_check(
        "public launch changelog",
        vec![FORGE_RELEASE_BUNDLE_LAUNCH_CHANGELOG_JSON],
        forge_release_bundle_launch_changelog_ready(json_values),
        "launch changelog is passing and keeps honest scope limits",
        "launch changelog must be generated from release history and include honest scope limits",
        findings,
        penalty,
    ));

    checks.push(forge_pages_named_check(
        "/forge/package-gallery/",
        vec![
            FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_HTML,
            FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_JSON,
            FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_MD,
        ],
        forge_release_bundle_package_gallery_ready(bundle_dir, json_values),
        "hosted package-gallery artifacts are public-review ready",
        "hosted package-gallery must publish HTML, JSON, Markdown, trust signals, and migration guides",
        findings,
        penalty,
    ));

    checks.push(forge_pages_named_check(
        "/forge/migration-gallery/",
        vec![
            FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_HTML,
            FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_JSON,
            FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_MD,
        ],
        forge_release_bundle_migration_gallery_ready(bundle_dir, json_values),
        "hosted migration-gallery artifacts are public-review ready",
        "hosted migration-gallery must publish supported scope, manual gaps, package evidence, and payload comparison boundaries",
        findings,
        penalty,
    ));

    checks.push(forge_release_bundle_manifest_check(
        bundle_dir,
        json_values,
        include_adoption,
        findings,
        penalty,
    ));

    checks.push(forge_pages_named_check(
        "publish bundle dependency boundary",
        vec!["node_modules"],
        !bundle_dir.join("node_modules").exists(),
        "no node_modules directory in release bundle",
        "release bundle must not include node_modules",
        findings,
        penalty,
    ));

    checks
}

fn forge_release_bundle_package_gallery_ready(
    bundle_dir: &Path,
    json_values: &std::collections::BTreeMap<String, serde_json::Value>,
) -> bool {
    let json_ready = json_values
        .get(FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_JSON)
        .is_some_and(|value| {
            let package_count = value
                .get("package_count")
                .and_then(|count| count.as_u64())
                .unwrap_or_default();
            let packages_len = value
                .get("packages")
                .and_then(|packages| packages.as_array())
                .map(|packages| packages.len() as u64)
                .unwrap_or_default();
            let migration_guides = value
                .get("migration_guides")
                .and_then(|guides| guides.as_array())
                .map(|guides| guides.len())
                .unwrap_or_default();
            let honest_scope = value
                .get("honest_scope")
                .and_then(|items| items.as_array())
                .is_some_and(|items| {
                    items.iter().any(|item| {
                        item.as_str()
                            .is_some_and(|scope| scope.contains("not a universal npm replacement"))
                    })
                });

            value.get("route").and_then(|route| route.as_str()) == Some("/forge/package-gallery/")
                && value.get("passed").and_then(|passed| passed.as_bool()) == Some(true)
                && value
                    .get("no_node_modules")
                    .and_then(|no_node_modules| no_node_modules.as_bool())
                    == Some(true)
                && package_count >= FORGE_WWW_TEMPLATE_PACKAGE_IDS.len() as u64
                && packages_len >= FORGE_WWW_TEMPLATE_PACKAGE_IDS.len() as u64
                && migration_guides > 0
                && value
                    .get("migration_gallery")
                    .and_then(|gallery| gallery.get("route"))
                    .and_then(|route| route.as_str())
                    == Some("/forge/migration-gallery/")
                && honest_scope
        });

    let html_ready = forge_release_bundle_text_contains_all(
        bundle_dir,
        FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_HTML,
        &[
            "DX Forge Package Gallery",
            "Trust signals",
            "Migration guides",
            "shadcn/ui/button",
            "auth/better-auth",
            "migration/static-site",
            "Migration gallery",
            "/forge/migration-gallery/",
            "not a universal npm replacement",
            "no node_modules",
        ],
    );
    let markdown_ready = forge_release_bundle_text_contains_all(
        bundle_dir,
        FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_MD,
        &[
            "# DX Forge Hosted Package Gallery",
            "## Migration Guides",
            "## Migration Gallery",
            "dx forge migration-guide --package ui/button",
        ],
    );

    json_ready && html_ready && markdown_ready && !bundle_dir.join("node_modules").exists()
}

fn forge_release_bundle_migration_gallery_ready(
    bundle_dir: &Path,
    json_values: &std::collections::BTreeMap<String, serde_json::Value>,
) -> bool {
    let json_ready = json_values
        .get(FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_JSON)
        .is_some_and(|value| {
            let evidence_ready = value
                .get("package_evidence")
                .and_then(|evidence| evidence.as_array())
                .is_some_and(|items| {
                    items.iter().any(|item| {
                        item.get("name").and_then(|name| name.as_str())
                            == Some("migration-static-site-manual-review")
                    })
                });
            let supported_scope_ready = value
                .get("supported_scope")
                .and_then(|items| items.as_array())
                .is_some_and(|items| {
                    items.iter().any(|item| {
                        item.as_str()
                            .is_some_and(|text| text.contains("static WordPress or HTML page"))
                    })
                });
            let manual_gaps_ready = value
                .get("manual_gaps")
                .and_then(|items| items.as_array())
                .is_some_and(|items| {
                    items.iter().any(|item| {
                        item.as_str()
                            .is_some_and(|text| text.contains("forms, comments, search"))
                    })
                });
            let payload_boundaries_ready = value
                .get("payload_comparison_boundaries")
                .and_then(|items| items.as_array())
                .is_some_and(|items| {
                    items.iter().any(|item| {
                        item.as_str()
                            .is_some_and(|text| text.contains("scoped static migrated route"))
                    })
                });

            value.get("route").and_then(|route| route.as_str()) == Some("/forge/migration-gallery/")
                && value.get("package_id").and_then(|package| package.as_str())
                    == Some("migration/static-site")
                && value.get("passed").and_then(|passed| passed.as_bool()) == Some(true)
                && value
                    .get("no_node_modules")
                    .and_then(|no_node_modules| no_node_modules.as_bool())
                    == Some(true)
                && evidence_ready
                && supported_scope_ready
                && manual_gaps_ready
                && payload_boundaries_ready
        });

    let html_ready = forge_release_bundle_text_contains_all(
        bundle_dir,
        FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_HTML,
        &[
            "DX Forge Migration Gallery",
            "migration/static-site",
            "Supported scope",
            "Manual gaps",
            "Package evidence",
            "Payload comparison boundaries",
            "dx forge migration-audit",
            "not a full WordPress plugin or theme migration",
            "no node_modules",
        ],
    );
    let markdown_ready = forge_release_bundle_text_contains_all(
        bundle_dir,
        FORGE_RELEASE_BUNDLE_MIGRATION_GALLERY_MD,
        &[
            "# DX Forge Migration Gallery",
            "## Payload Comparison Boundaries",
            "migration-static-site-source-files",
        ],
    );

    json_ready && html_ready && markdown_ready && !bundle_dir.join("node_modules").exists()
}

fn forge_release_bundle_text_contains_all(
    bundle_dir: &Path,
    relative: &str,
    expected: &[&str],
) -> bool {
    std::fs::read_to_string(bundle_dir.join(relative))
        .map(|text| expected.iter().all(|needle| text.contains(needle)))
        .unwrap_or(false)
}

fn forge_release_bundle_manifest_check(
    bundle_dir: &Path,
    json_values: &std::collections::BTreeMap<String, serde_json::Value>,
    include_adoption: bool,
    findings: &mut Vec<String>,
    penalty: &mut u16,
) -> DxForgeCiRouteArtifactCheck {
    let mut issues = Vec::new();
    let manifest = match json_values
        .get(FORGE_RELEASE_BUNDLE_MANIFEST_JSON)
        .cloned()
        .map(serde_json::from_value::<DxForgeReleaseBundleManifest>)
    {
        Some(Ok(manifest)) => Some(manifest),
        Some(Err(error)) => {
            issues.push(format!("manifest schema is invalid: {error}"));
            None
        }
        None => {
            issues.push("manifest json is missing".to_string());
            None
        }
    };

    if let Some(manifest) = manifest {
        if manifest.version != 1 {
            issues.push(format!("unsupported manifest version {}", manifest.version));
        }
        if manifest.hash_algorithm != "blake3" {
            issues.push(format!(
                "unexpected hash algorithm {}",
                manifest.hash_algorithm
            ));
        }
        if manifest.integrity.scheme != FORGE_RELEASE_BUNDLE_MANIFEST_INTEGRITY_SCHEME {
            issues.push(format!(
                "unexpected integrity scheme {}",
                manifest.integrity.scheme
            ));
        }
        if manifest.artifact_integrity.scheme != FORGE_RELEASE_BUNDLE_ARTIFACT_INTEGRITY_SCHEME {
            issues.push(format!(
                "unexpected artifact integrity scheme {}",
                manifest.artifact_integrity.scheme
            ));
        }
        if manifest.artifact_integrity.hash_algorithm != "blake3" {
            issues.push(format!(
                "unexpected artifact integrity hash algorithm {}",
                manifest.artifact_integrity.hash_algorithm
            ));
        }
        if !manifest.artifact_integrity.verified_locally {
            issues.push("artifact integrity must be locally verified".to_string());
        }
        if manifest.publisher_identity.scheme != FORGE_RELEASE_BUNDLE_PUBLISHER_IDENTITY_SCHEME {
            issues.push(format!(
                "unexpected publisher identity scheme {}",
                manifest.publisher_identity.scheme
            ));
        }
        if manifest.artifact_count != manifest.artifacts.len() {
            issues.push(format!(
                "artifact_count {} does not match {} manifest rows",
                manifest.artifact_count,
                manifest.artifacts.len()
            ));
        }
        if manifest.artifact_integrity.artifact_count != manifest.artifacts.len() {
            issues.push(format!(
                "artifact integrity count {} does not match {} manifest rows",
                manifest.artifact_integrity.artifact_count,
                manifest.artifacts.len()
            ));
        }
        if manifest.artifact_integrity.digest != manifest.integrity.digest {
            issues.push(format!(
                "artifact integrity digest {} does not match manifest digest {}",
                manifest.artifact_integrity.digest, manifest.integrity.digest
            ));
        }
        let computed_manifest_digest = forge_release_bundle_manifest_digest(&manifest.artifacts);
        match &computed_manifest_digest {
            Ok(digest) if digest == &manifest.integrity.digest => {}
            Ok(digest) => issues.push(format!(
                "manifest digest mismatch: expected {}, computed {}",
                manifest.integrity.digest, digest
            )),
            Err(error) => issues.push(format!("could not recompute manifest digest: {error}")),
        }
        match &computed_manifest_digest {
            Ok(digest) if digest == &manifest.artifact_integrity.digest => {}
            Ok(digest) => issues.push(format!(
                "artifact integrity digest mismatch: expected {}, computed {}",
                manifest.artifact_integrity.digest, digest
            )),
            Err(_) => {}
        }
        match manifest.publisher_identity.status.as_str() {
            FORGE_RELEASE_BUNDLE_SIGNATURE_STATUS_UNSIGNED => {
                if manifest.publisher_identity.signer.is_some()
                    || manifest.publisher_identity.key_id.is_some()
                    || manifest.publisher_identity.algorithm.is_some()
                    || manifest.publisher_identity.public_key.is_some()
                    || manifest.publisher_identity.signature.is_some()
                    || manifest.publisher_identity.signed_at.is_some()
                {
                    issues.push(
                        "unsigned publisher identity must not include signer, key, algorithm, public_key, signature, or signed_at"
                            .to_string(),
                    );
                }
                if manifest.integrity.signed || manifest.integrity.signature.is_some() {
                    issues.push(
                        "unsigned publisher identity must keep compatibility signature fields unsigned"
                            .to_string(),
                    );
                }
            }
            FORGE_RELEASE_BUNDLE_SIGNATURE_STATUS_SIGNED => {
                if manifest.publisher_identity.signer.is_none()
                    || manifest.publisher_identity.key_id.is_none()
                    || manifest.publisher_identity.algorithm.is_none()
                    || manifest.publisher_identity.public_key.is_none()
                    || manifest.publisher_identity.signature.is_none()
                    || manifest.publisher_identity.signed_at.is_none()
                {
                    issues.push(
                        "signed publisher identity must include signer, key_id, algorithm, public_key, signature, and signed_at"
                            .to_string(),
                    );
                }
                if !manifest.integrity.signed || manifest.integrity.signature.is_none() {
                    issues.push(
                        "signed publisher identity must keep compatibility signature fields signed"
                            .to_string(),
                    );
                }
                if let Err(error) = verify_forge_release_bundle_manifest_signature(&manifest) {
                    issues.push(error);
                }
            }
            other => issues.push(format!("unknown publisher identity status `{other}`")),
        }

        let expected = forge_release_bundle_manifest_artifact_paths(include_adoption)
            .into_iter()
            .collect::<HashSet<_>>();
        let listed = manifest
            .artifacts
            .iter()
            .map(|artifact| artifact.path.clone())
            .collect::<HashSet<_>>();
        for missing in expected.difference(&listed) {
            issues.push(format!("manifest does not list `{missing}`"));
        }
        for unexpected in listed.difference(&expected) {
            issues.push(format!("manifest lists unexpected artifact `{unexpected}`"));
        }

        let mut has_claims = false;
        let mut has_dxp = false;
        let mut has_evidence = false;
        let mut has_release_history = false;
        let mut has_launch_changelog = false;
        let mut has_package_gallery = false;
        let mut has_migration_gallery = false;
        let mut has_route_comparison = false;

        for artifact in &manifest.artifacts {
            if !forge_release_bundle_manifest_path_safe(&artifact.path) {
                issues.push(format!("manifest path is unsafe: `{}`", artifact.path));
                continue;
            }

            has_claims |= artifact.artifact_type == "claims";
            has_dxp |= artifact.artifact_type == "dxpk-packet";
            has_evidence |= artifact.artifact_type == "evidence";
            has_release_history |= artifact.artifact_type == "release-history";
            has_launch_changelog |= artifact.artifact_type == "launch-changelog";
            has_package_gallery |= artifact.artifact_type == "package-gallery";
            has_migration_gallery |= artifact.artifact_type == "migration-gallery";
            has_route_comparison |= artifact.artifact_type == "route-comparison";

            match std::fs::read(bundle_dir.join(&artifact.path)) {
                Ok(raw) => {
                    if raw.len() as u64 != artifact.bytes {
                        issues.push(format!(
                            "`{}` byte count changed: manifest {}, actual {}",
                            artifact.path,
                            artifact.bytes,
                            raw.len()
                        ));
                    }
                    let actual_hash = blake3::hash(&raw).to_hex().to_string();
                    if actual_hash != artifact.blake3 {
                        issues.push(format!(
                            "`{}` hash changed: manifest {}, actual {}",
                            artifact.path, artifact.blake3, actual_hash
                        ));
                    }
                }
                Err(error) => issues.push(format!("`{}` is unreadable: {error}", artifact.path)),
            }
        }

        if !has_claims {
            issues.push("manifest does not include claims artifacts".to_string());
        }
        if !has_dxp {
            issues.push("manifest does not include DXPK packet artifacts".to_string());
        }
        if !has_evidence {
            issues.push("manifest does not include evidence artifacts".to_string());
        }
        if !has_release_history {
            issues.push("manifest does not include release-history artifacts".to_string());
        }
        if !has_launch_changelog {
            issues.push("manifest does not include launch-changelog artifacts".to_string());
        }
        if !has_package_gallery {
            issues.push("manifest does not include package-gallery artifacts".to_string());
        }
        if !has_migration_gallery {
            issues.push("manifest does not include migration-gallery artifacts".to_string());
        }
        if !has_route_comparison {
            issues.push("manifest does not include route-comparison artifacts".to_string());
        }
    }

    let passed = issues.is_empty();
    let message = if passed {
        "manifest covers and verifies public launch artifacts".to_string()
    } else {
        let message = issues.join("; ");
        findings.push(format!("release manifest: {message}"));
        *penalty = penalty.saturating_add(10);
        message
    };

    DxForgeCiRouteArtifactCheck {
        route: "release manifest".to_string(),
        artifacts: vec![
            FORGE_RELEASE_BUNDLE_MANIFEST_JSON.to_string(),
            FORGE_RELEASE_BUNDLE_MANIFEST_MD.to_string(),
        ],
        passed,
        message,
    }
}

fn verify_forge_release_bundle_manifest_signature(
    manifest: &DxForgeReleaseBundleManifest,
) -> Result<(), String> {
    let identity = &manifest.publisher_identity;
    let signer = required_publisher_identity_field(identity.signer.as_deref(), "signer")?;
    let key_id = required_publisher_identity_field(identity.key_id.as_deref(), "key_id")?;
    let algorithm = required_publisher_identity_field(identity.algorithm.as_deref(), "algorithm")?;
    let public_key =
        required_publisher_identity_field(identity.public_key.as_deref(), "public_key")?;
    let signature = required_publisher_identity_field(identity.signature.as_deref(), "signature")?;
    let signed_at = required_publisher_identity_field(identity.signed_at.as_deref(), "signed_at")?;
    let integrity_signature = required_publisher_identity_field(
        manifest.integrity.signature.as_deref(),
        "integrity.signature",
    )?;

    if algorithm != "ed25519" {
        return Err(format!(
            "unsupported publisher identity algorithm `{algorithm}`"
        ));
    }
    if signature != integrity_signature {
        return Err(
            "publisher identity signature must match integrity compatibility signature".to_string(),
        );
    }

    let public_key_bytes = decode_prefixed_hex(public_key, "ed25519:", "public_key", 32)?;
    let expected_key_id = forge_release_bundle_publisher_key_id(&public_key_bytes);
    if key_id != expected_key_id {
        return Err(format!(
            "publisher identity key_id `{key_id}` does not match public_key fingerprint `{expected_key_id}`"
        ));
    }
    let signature_bytes = decode_prefixed_hex(signature, "ed25519:", "signature", 64)?;
    let public_key_array: [u8; 32] = public_key_bytes
        .try_into()
        .map_err(|_| "publisher public_key must decode to 32 bytes".to_string())?;
    let signature_array: [u8; 64] = signature_bytes
        .try_into()
        .map_err(|_| "publisher signature must decode to 64 bytes".to_string())?;
    let verifying_key = VerifyingKey::from_bytes(&public_key_array)
        .map_err(|error| format!("publisher public_key is invalid: {error}"))?;
    let signature = Signature::from_bytes(&signature_array);
    let payload = forge_release_bundle_manifest_signing_payload(
        manifest, signer, key_id, public_key, signed_at,
    );

    verifying_key
        .verify(payload.as_bytes(), &signature)
        .map_err(|error| format!("publisher identity signature verification failed: {error}"))
}

fn required_publisher_identity_field<'a>(
    value: Option<&'a str>,
    field: &str,
) -> Result<&'a str, String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("signed publisher identity missing `{field}`"))
}

fn forge_release_bundle_manifest_signing_payload(
    manifest: &DxForgeReleaseBundleManifest,
    signer: &str,
    key_id: &str,
    public_key: &str,
    signed_at: &str,
) -> String {
    format!(
        "dx-forge-release-publisher-identity-v1\nversion={}\nhash_algorithm={}\nintegrity_scheme={}\nartifact_integrity_scheme={}\nmanifest_digest={}\nartifact_integrity_digest={}\nartifact_count={}\nsigner={}\nkey_id={}\npublic_key={}\nsigned_at={}\n",
        manifest.version,
        manifest.hash_algorithm,
        manifest.integrity.scheme,
        manifest.artifact_integrity.scheme,
        manifest.integrity.digest,
        manifest.artifact_integrity.digest,
        manifest.artifact_count,
        signer,
        key_id,
        public_key,
        signed_at
    )
}

fn forge_release_bundle_publisher_key_id(public_key: &[u8]) -> String {
    format!("ed25519-blake3:{}", blake3::hash(public_key).to_hex())
}

fn decode_prefixed_hex(
    value: &str,
    prefix: &str,
    field: &str,
    expected_len: usize,
) -> Result<Vec<u8>, String> {
    let Some(hex) = value.strip_prefix(prefix) else {
        return Err(format!(
            "publisher identity `{field}` must start with `{prefix}`"
        ));
    };
    let bytes = decode_hex(hex).map_err(|error| format!("publisher identity `{field}` {error}"))?;
    if bytes.len() != expected_len {
        return Err(format!(
            "publisher identity `{field}` must decode to {expected_len} bytes, got {}",
            bytes.len()
        ));
    }
    Ok(bytes)
}

fn decode_hex(value: &str) -> Result<Vec<u8>, &'static str> {
    if value.len() % 2 != 0 {
        return Err("hex length must be even");
    }

    let mut bytes = Vec::with_capacity(value.len() / 2);
    for pair in value.as_bytes().chunks_exact(2) {
        let high = hex_nibble(pair[0]).ok_or("hex contains a non-hex character")?;
        let low = hex_nibble(pair[1]).ok_or("hex contains a non-hex character")?;
        bytes.push((high << 4) | low);
    }
    Ok(bytes)
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn hex_nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

fn forge_release_bundle_manifest_path_safe(relative: &str) -> bool {
    !relative.contains('\\')
        && !Path::new(relative).is_absolute()
        && !Path::new(relative).components().any(|component| {
            matches!(
                component,
                std::path::Component::ParentDir
                    | std::path::Component::RootDir
                    | std::path::Component::Prefix(_)
            )
        })
}

fn forge_release_bundle_route_comparison_ready(
    json_values: &std::collections::BTreeMap<String, serde_json::Value>,
    include_adoption: bool,
) -> bool {
    let Some(value) = json_values.get("forge-public-route-comparison.json") else {
        return false;
    };
    let required_routes = forge_required_public_routes(include_adoption);
    let route_count_ok = value
        .get("route_count")
        .and_then(|value| value.as_u64())
        .is_some_and(|route_count| route_count >= required_routes.len() as u64);
    let Some(routes) = value.get("routes").and_then(|value| value.as_array()) else {
        return false;
    };
    route_count_ok
        && required_routes.iter().all(|required| {
            routes.iter().any(|route| {
                route.get("route").and_then(|value| value.as_str()) == Some(*required)
                    && route.get("status").and_then(|value| value.as_str()) == Some("measured")
                    && route.get("route_delivery").and_then(|value| value.as_str())
                        == Some("static")
                    && route.get("budget_passed").and_then(|value| value.as_bool()) == Some(true)
            })
        })
        && routes.iter().all(|route| {
            route
                .get("budget_passed")
                .and_then(|value| value.as_bool())
                .unwrap_or(true)
        })
}

fn forge_release_bundle_history_ready(
    json_values: &std::collections::BTreeMap<String, serde_json::Value>,
    include_adoption: bool,
) -> bool {
    let Some(value) = json_values.get("forge-public-release-history.json") else {
        return false;
    };
    let required_routes = forge_required_public_routes(include_adoption);
    value
        .get("records")
        .and_then(|value| value.as_array())
        .and_then(|records| records.first())
        .and_then(|record| record.get("route_comparison"))
        .and_then(|route_comparison| route_comparison.get("route_count"))
        .and_then(|value| value.as_u64())
        .is_some_and(|route_count| route_count >= required_routes.len() as u64)
}

fn forge_release_bundle_launch_changelog_ready(
    json_values: &std::collections::BTreeMap<String, serde_json::Value>,
) -> bool {
    let Some(value) = json_values.get(FORGE_RELEASE_BUNDLE_LAUNCH_CHANGELOG_JSON) else {
        return false;
    };
    let passing = value.get("passed").and_then(|value| value.as_bool()) == Some(true)
        && value
            .get("score")
            .and_then(|value| value.as_u64())
            .is_some_and(|score| score >= 90)
        && value.get("latest").is_some_and(|latest| !latest.is_null());
    let honest_scope = value
        .get("honest_scope")
        .and_then(|value| value.as_array())
        .is_some_and(|items| {
            items.iter().any(|item| {
                item.as_str()
                    .is_some_and(|scope| scope.contains("does not claim live production traffic"))
            }) && items.iter().any(|item| {
                item.as_str()
                    .is_some_and(|scope| scope.contains("universal npm replacement coverage"))
            })
        });
    passing && honest_scope
}

fn forge_release_bundle_secret_marker_checks(
    bundle_dir: &Path,
    findings: &mut Vec<String>,
    penalty: &mut u16,
) -> Vec<DxForgeCiRouteArtifactCheck> {
    let mut leaked = Vec::new();
    let mut pending = vec![bundle_dir.to_path_buf()];

    while let Some(current) = pending.pop() {
        let entries = match std::fs::read_dir(&current) {
            Ok(entries) => entries,
            Err(error) => {
                findings.push(format!(
                    "secret scan could not read {}: {error}",
                    current.display()
                ));
                *penalty = penalty.saturating_add(10);
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
                        path.strip_prefix(bundle_dir).unwrap_or(&path).display()
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

    vec![DxForgeCiRouteArtifactCheck {
        route: "secret-free release bundle".to_string(),
        artifacts: vec!["release-bundle".to_string()],
        passed,
        message,
    }]
}

fn verify_forge_pages_bundle(
    bundle_dir: &Path,
) -> anyhow::Result<DxForgePagesBundleVerificationReport> {
    let mut findings = Vec::new();
    let mut artifacts = Vec::new();
    let mut json_values = std::collections::BTreeMap::<String, serde_json::Value>::new();
    let mut penalty = 0u16;

    if !bundle_dir.is_dir() {
        findings.push(format!(
            "Pages bundle directory does not exist: {}",
            bundle_dir.display()
        ));
        penalty = 100;
    }

    for name in FORGE_PAGES_REQUIRED_ARTIFACTS {
        let path = bundle_dir.join(name);
        let exists = path.is_file();
        let bytes = path.metadata().map(|metadata| metadata.len()).unwrap_or(0);
        let mut messages = Vec::new();
        let mut valid_json = None;

        if !exists {
            messages.push("missing".to_string());
            penalty = penalty.saturating_add(14);
        } else {
            if bytes == 0 {
                messages.push("empty".to_string());
                penalty = penalty.saturating_add(14);
            }

            if name.ends_with(".json") {
                valid_json = Some(false);
                match std::fs::read(&path) {
                    Ok(raw) => match serde_json::from_slice::<serde_json::Value>(&raw) {
                        Ok(value) => {
                            valid_json = Some(true);
                            json_values.insert((*name).to_string(), value);
                        }
                        Err(error) => {
                            messages.push(format!("invalid json: {error}"));
                            penalty = penalty.saturating_add(14);
                        }
                    },
                    Err(error) => {
                        messages.push(format!("unreadable: {error}"));
                        penalty = penalty.saturating_add(14);
                    }
                }
            }

            if name.ends_with(".dxp") {
                match std::fs::read(&path) {
                    Ok(raw) if raw.starts_with(b"DXPK") => {}
                    Ok(_) => {
                        messages.push("missing DXPK header".to_string());
                        penalty = penalty.saturating_add(14);
                    }
                    Err(error) => {
                        messages.push(format!("unreadable DXPK artifact: {error}"));
                        penalty = penalty.saturating_add(14);
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
        artifacts.push(DxForgeCiArtifactCheck {
            name: (*name).to_string(),
            path,
            exists,
            bytes,
            valid_json,
            passed,
            message,
        });
    }

    let mut checks =
        forge_pages_bundle_checks(bundle_dir, &json_values, &mut findings, &mut penalty);
    checks.extend(forge_pages_secret_marker_checks(
        bundle_dir,
        &mut findings,
        &mut penalty,
    ));

    let score = 100u8.saturating_sub(penalty.min(100) as u8);
    let passed = findings.is_empty();

    Ok(DxForgePagesBundleVerificationReport {
        bundle_dir: bundle_dir.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        artifacts,
        checks,
        findings,
    })
}

fn forge_pages_bundle_checks(
    bundle_dir: &Path,
    json_values: &std::collections::BTreeMap<String, serde_json::Value>,
    findings: &mut Vec<String>,
    penalty: &mut u16,
) -> Vec<DxForgeCiRouteArtifactCheck> {
    let mut checks = vec![forge_pages_named_check(
        "readiness badge",
        vec!["forge-readiness-badge.json"],
        forge_pages_json_bool(json_values, "forge-readiness-badge.json", "passed")
            && forge_pages_json_bool(json_values, "forge-readiness-badge.json", "no_node_modules"),
        "badge reports passed and no node_modules",
        "forge-readiness-badge.json must report passed=true and no_node_modules=true",
        findings,
        penalty,
    )];

    checks.push(forge_pages_named_check(
        "/forge/ci claims",
        vec!["forge/ci.claims.json"],
        forge_pages_route_value(json_values, "forge/ci.claims.json") == Some("/forge/ci")
            && forge_pages_claim_statuses_valid(json_values, "forge/ci.claims.json"),
        "claims manifest targets /forge/ci and has reviewable statuses",
        "forge/ci.claims.json must target /forge/ci and use reviewable claim statuses",
        findings,
        penalty,
    ));

    checks.push(forge_pages_named_check(
        "/forge/ci proof",
        vec!["forge/ci.proof.json"],
        forge_pages_route_value(json_values, "forge/ci.proof.json") == Some("/forge/ci")
            && forge_pages_json_string_contains(
                json_values,
                "forge/ci.proof.json",
                "forge/ci.html",
            )
            && forge_pages_json_string_contains(json_values, "forge/ci.proof.json", "forge/ci.dxp"),
        "proof summary targets /forge/ci and references HTML plus DXPK",
        "forge/ci.proof.json must target /forge/ci and reference forge/ci.html plus forge/ci.dxp",
        findings,
        penalty,
    ));

    checks.push(forge_pages_named_check(
        "/forge/ci legacy proof",
        vec!["proof.json"],
        forge_pages_route_value(json_values, "proof.json") == Some("/forge/ci")
            && forge_pages_json_string_contains(json_values, "proof.json", "forge/ci.html")
            && forge_pages_json_string_contains(json_values, "proof.json", "forge/ci.dxp"),
        "legacy proof.json still targets /forge/ci",
        "proof.json must remain a /forge/ci compatibility proof",
        findings,
        penalty,
    ));

    let ci_html_path = bundle_dir.join("forge/ci.html");
    let ci_index_path = bundle_dir.join("forge/ci/index.html");
    let ci_clean_route_matches = match (std::fs::read(&ci_html_path), std::fs::read(&ci_index_path))
    {
        (Ok(html), Ok(index)) => html == index,
        _ => false,
    };
    checks.push(forge_pages_named_check(
        "/forge/ci clean route",
        vec!["forge/ci.html", "forge/ci/index.html"],
        ci_clean_route_matches,
        "clean-route index matches forge/ci.html",
        "forge/ci/index.html must exist and match forge/ci.html",
        findings,
        penalty,
    ));

    checks.push(forge_pages_named_check(
        "/forge/releases claims",
        vec!["forge/releases.claims.json"],
        forge_pages_route_value(json_values, "forge/releases.claims.json")
            == Some("/forge/releases")
            && forge_pages_claim_statuses_valid(json_values, "forge/releases.claims.json"),
        "claims manifest targets /forge/releases and has reviewable statuses",
        "forge/releases.claims.json must target /forge/releases and use reviewable claim statuses",
        findings,
        penalty,
    ));

    checks.push(forge_pages_named_check(
        "/forge/releases proof",
        vec!["forge/releases.proof.json"],
        forge_pages_route_value(json_values, "forge/releases.proof.json")
            == Some("/forge/releases")
            && forge_pages_json_string_contains(
                json_values,
                "forge/releases.proof.json",
                "forge/releases.html",
            )
            && forge_pages_json_string_contains(
                json_values,
                "forge/releases.proof.json",
                "forge/releases.dxp",
            ),
        "proof summary targets /forge/releases and references HTML plus DXPK",
        "forge/releases.proof.json must target /forge/releases and reference forge/releases.html plus forge/releases.dxp",
        findings,
        penalty,
    ));

    let releases_html_path = bundle_dir.join("forge/releases.html");
    let releases_index_path = bundle_dir.join("forge/releases/index.html");
    let releases_clean_route_matches = match (
        std::fs::read(&releases_html_path),
        std::fs::read(&releases_index_path),
    ) {
        (Ok(html), Ok(index)) => html == index,
        _ => false,
    };
    checks.push(forge_pages_named_check(
        "/forge/releases clean route",
        vec!["forge/releases.html", "forge/releases/index.html"],
        releases_clean_route_matches,
        "clean-route index matches forge/releases.html",
        "forge/releases/index.html must exist and match forge/releases.html",
        findings,
        penalty,
    ));

    checks.push(forge_pages_named_check(
        "/forge/changelog claims",
        vec!["forge/changelog.claims.json"],
        forge_pages_route_value(json_values, "forge/changelog.claims.json")
            == Some("/forge/changelog")
            && forge_pages_claim_statuses_valid(json_values, "forge/changelog.claims.json"),
        "claims manifest targets /forge/changelog and has reviewable statuses",
        "forge/changelog.claims.json must target /forge/changelog and use reviewable claim statuses",
        findings,
        penalty,
    ));

    checks.push(forge_pages_named_check(
        "/forge/changelog proof",
        vec!["forge/changelog.proof.json"],
        forge_pages_route_value(json_values, "forge/changelog.proof.json")
            == Some("/forge/changelog")
            && forge_pages_json_string_contains(
                json_values,
                "forge/changelog.proof.json",
                "forge/changelog.html",
            )
            && forge_pages_json_string_contains(
                json_values,
                "forge/changelog.proof.json",
                "forge/changelog.dxp",
            ),
        "proof summary targets /forge/changelog and references HTML plus DXPK",
        "forge/changelog.proof.json must target /forge/changelog and reference forge/changelog.html plus forge/changelog.dxp",
        findings,
        penalty,
    ));

    let changelog_html_path = bundle_dir.join("forge/changelog.html");
    let changelog_index_path = bundle_dir.join("forge/changelog/index.html");
    let changelog_clean_route_matches = match (
        std::fs::read(&changelog_html_path),
        std::fs::read(&changelog_index_path),
    ) {
        (Ok(html), Ok(index)) => html == index,
        _ => false,
    };
    checks.push(forge_pages_named_check(
        "/forge/changelog clean route",
        vec!["forge/changelog.html", "forge/changelog/index.html"],
        changelog_clean_route_matches,
        "clean-route index matches forge/changelog.html",
        "forge/changelog/index.html must exist and match forge/changelog.html",
        findings,
        penalty,
    ));

    checks.push(forge_pages_named_check(
        "/forge/adoption claims",
        vec!["forge/adoption.claims.json"],
        forge_pages_route_value(json_values, "forge/adoption.claims.json")
            == Some("/forge/adoption")
            && forge_pages_claim_statuses_valid(json_values, "forge/adoption.claims.json"),
        "claims manifest targets /forge/adoption and has reviewable statuses",
        "forge/adoption.claims.json must target /forge/adoption and use reviewable claim statuses",
        findings,
        penalty,
    ));

    checks.push(forge_pages_named_check(
        "/forge/adoption proof",
        vec!["forge/adoption.proof.json"],
        forge_pages_route_value(json_values, "forge/adoption.proof.json")
            == Some("/forge/adoption")
            && forge_pages_json_string_contains(
                json_values,
                "forge/adoption.proof.json",
                "forge/adoption.html",
            )
            && forge_pages_json_string_contains(
                json_values,
                "forge/adoption.proof.json",
                "forge/adoption.dxp",
            ),
        "proof summary targets /forge/adoption and references HTML plus DXPK",
        "forge/adoption.proof.json must target /forge/adoption and reference forge/adoption.html plus forge/adoption.dxp",
        findings,
        penalty,
    ));

    let adoption_html_path = bundle_dir.join("forge/adoption.html");
    let adoption_index_path = bundle_dir.join("forge/adoption/index.html");
    let adoption_clean_route_matches = match (
        std::fs::read(&adoption_html_path),
        std::fs::read(&adoption_index_path),
    ) {
        (Ok(html), Ok(index)) => html == index,
        _ => false,
    };
    checks.push(forge_pages_named_check(
        "/forge/adoption clean route",
        vec!["forge/adoption.html", "forge/adoption/index.html"],
        adoption_clean_route_matches,
        "clean-route index matches forge/adoption.html",
        "forge/adoption/index.html must exist and match forge/adoption.html",
        findings,
        penalty,
    ));

    checks.push(forge_pages_named_check(
        "publish bundle dependency boundary",
        vec!["node_modules"],
        !bundle_dir.join("node_modules").exists(),
        "no node_modules directory in publish bundle",
        "publish bundle must not include node_modules",
        findings,
        penalty,
    ));

    checks
}

fn forge_pages_named_check(
    route: &str,
    artifacts: Vec<&str>,
    passed: bool,
    ok_message: &str,
    failed_message: &str,
    findings: &mut Vec<String>,
    penalty: &mut u16,
) -> DxForgeCiRouteArtifactCheck {
    let message = if passed {
        ok_message.to_string()
    } else {
        findings.push(format!("{route}: {failed_message}"));
        *penalty = penalty.saturating_add(10);
        failed_message.to_string()
    };

    DxForgeCiRouteArtifactCheck {
        route: route.to_string(),
        artifacts: artifacts.into_iter().map(str::to_string).collect(),
        passed,
        message,
    }
}

fn forge_pages_json_bool(
    json_values: &std::collections::BTreeMap<String, serde_json::Value>,
    artifact: &str,
    key: &str,
) -> bool {
    json_values
        .get(artifact)
        .and_then(|value| value.get(key))
        .and_then(|value| value.as_bool())
        == Some(true)
}

fn forge_pages_route_value<'a>(
    json_values: &'a std::collections::BTreeMap<String, serde_json::Value>,
    artifact: &str,
) -> Option<&'a str> {
    json_values
        .get(artifact)
        .and_then(|value| value.get("route"))
        .and_then(|value| value.as_str())
}

fn forge_pages_claim_statuses_valid(
    json_values: &std::collections::BTreeMap<String, serde_json::Value>,
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

fn forge_pages_json_string_contains(
    json_values: &std::collections::BTreeMap<String, serde_json::Value>,
    artifact: &str,
    needle: &str,
) -> bool {
    json_values.get(artifact).is_some_and(|value| {
        let normalized = value.to_string().replace("\\\\", "/").replace('\\', "/");
        normalized.contains(needle)
    })
}

fn forge_pages_secret_marker_checks(
    bundle_dir: &Path,
    findings: &mut Vec<String>,
    penalty: &mut u16,
) -> Vec<DxForgeCiRouteArtifactCheck> {
    let mut leaked = Vec::new();
    let mut pending = vec![bundle_dir.to_path_buf()];

    while let Some(current) = pending.pop() {
        let entries = match std::fs::read_dir(&current) {
            Ok(entries) => entries,
            Err(error) => {
                findings.push(format!(
                    "secret scan could not read {}: {error}",
                    current.display()
                ));
                *penalty = penalty.saturating_add(10);
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
                        path.strip_prefix(bundle_dir).unwrap_or(&path).display()
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

    vec![DxForgeCiRouteArtifactCheck {
        route: "secret-free public bundle".to_string(),
        artifacts: FORGE_PAGES_REQUIRED_ARTIFACTS
            .iter()
            .map(|artifact| (*artifact).to_string())
            .collect(),
        passed,
        message,
    }]
}

fn forge_ci_text_artifact(name: &str) -> bool {
    name.ends_with(".json")
        || name.ends_with(".md")
        || name.ends_with(".html")
        || name.ends_with(".html")
}

fn forge_ci_json_gate(
    json_values: &std::collections::BTreeMap<String, serde_json::Value>,
    artifact: &str,
    key: &str,
    expected: bool,
    finding: &str,
    findings: &mut Vec<String>,
    penalty: &mut u16,
) {
    let passed = json_values
        .get(artifact)
        .and_then(|value| value.get(key))
        .and_then(|value| value.as_bool())
        == Some(expected);
    if !passed {
        findings.push(finding.to_string());
        *penalty = penalty.saturating_add(10);
    }
}

fn forge_ci_route_artifact_checks(
    artifact_dir: &Path,
    findings: &mut Vec<String>,
    penalty: &mut u16,
) -> Vec<DxForgeCiRouteArtifactCheck> {
    let route_artifacts = [
        ("/forge", vec!["forge.html"]),
        ("/forge claims", vec!["forge.claims.json"]),
        ("/forge evidence model", vec!["forge.evidence.json"]),
        ("/forge proof packet", vec!["forge.dxp"]),
        ("/forge source page", vec!["forge-page.html"]),
        (
            "/forge/ci source model",
            vec!["forge-smoke.json", "forge-readiness-badge.json"],
        ),
        (
            "/forge/adoption report",
            vec!["forge-adoption-report.json", "forge-adoption-report.md"],
        ),
        (
            "/forge/adoption route",
            vec![
                "forge/adoption.html",
                "forge/adoption/index.html",
                "forge/adoption.claims.json",
                "forge/adoption.dxp",
                "forge/adoption.proof.json",
                "forge-adoption-page.html",
            ],
        ),
    ];

    route_artifacts
        .into_iter()
        .map(|(route, names)| {
            let missing = names
                .iter()
                .filter(|name| !artifact_dir.join(name).is_file())
                .copied()
                .collect::<Vec<_>>();
            let passed = missing.is_empty();
            let message = if passed {
                "ok".to_string()
            } else {
                let message = format!("missing {}", missing.join(", "));
                findings.push(format!("{route}: {message}"));
                *penalty = penalty.saturating_add(8);
                message
            };
            DxForgeCiRouteArtifactCheck {
                route: route.to_string(),
                artifacts: names.into_iter().map(str::to_string).collect(),
                passed,
                message,
            }
        })
        .collect()
}

fn print_forge_ci_artifact_verification(report: &DxForgeCiArtifactVerificationReport) {
    println!("DX Forge CI artifact verification");
    println!("Artifacts: {}", report.artifact_dir.display());
    println!("Passed: {}", report.passed);
    println!("Score: {}", report.score);
    println!(
        "Files: {}/{}",
        report
            .artifacts
            .iter()
            .filter(|artifact| artifact.passed)
            .count(),
        report.artifacts.len()
    );
    if !report.findings.is_empty() {
        println!("Findings:");
        for finding in &report.findings {
            println!("- {finding}");
        }
    }
}

fn print_forge_pages_bundle_verification(report: &DxForgePagesBundleVerificationReport) {
    println!("DX Forge Pages bundle verification");
    println!("Bundle: {}", report.bundle_dir.display());
    println!("Passed: {}", report.passed);
    println!("Score: {}", report.score);
    println!(
        "Files: {}/{}",
        report
            .artifacts
            .iter()
            .filter(|artifact| artifact.passed)
            .count(),
        report.artifacts.len()
    );
    if !report.findings.is_empty() {
        println!("Findings:");
        for finding in &report.findings {
            println!("- {finding}");
        }
    }
}

fn forge_ci_artifact_verification_markdown(report: &DxForgeCiArtifactVerificationReport) -> String {
    let mut output = format!(
        "# DX Forge CI Artifact Verification\n\n- Artifact directory: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}`\n\n",
        report.artifact_dir.display(),
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

    output.push_str("\n## Route And Evidence Map\n\n");
    output.push_str("| Route or evidence | Artifacts | Status |\n");
    output.push_str("| --- | --- | --- |\n");
    for route in &report.routes {
        let links = route
            .artifacts
            .iter()
            .map(|artifact| markdown_artifact_link(artifact))
            .collect::<Vec<_>>()
            .join(", ");
        output.push_str(&format!(
            "| `{}` | {} | {} |\n",
            route.route, links, route.message
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

fn forge_pages_bundle_verification_markdown(
    report: &DxForgePagesBundleVerificationReport,
) -> String {
    let mut output = format!(
        "# DX Forge Pages Bundle Verification\n\n- Bundle directory: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}`\n\n",
        report.bundle_dir.display(),
        report.generated_at,
        report.passed,
        report.score
    );

    output.push_str("## Publish Artifact Checks\n\n");
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

    output.push_str("\n## Publish Shape Checks\n\n");
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

fn forge_ci_artifact_verification_failure_summary(
    report: &DxForgeCiArtifactVerificationReport,
) -> String {
    if report.findings.is_empty() {
        return format!(
            "Forge CI artifact verification failed with score {}",
            report.score
        );
    }
    format!(
        "Forge CI artifact verification failed: {}",
        report.findings.join("; ")
    )
}

fn forge_pages_bundle_verification_failure_summary(
    report: &DxForgePagesBundleVerificationReport,
) -> String {
    if report.findings.is_empty() {
        return format!(
            "Forge Pages bundle verification failed with score {}",
            report.score
        );
    }
    format!(
        "Forge Pages bundle verification failed: {}",
        report.findings.join("; ")
    )
}
