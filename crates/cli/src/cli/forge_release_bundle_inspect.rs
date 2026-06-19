use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::Serialize;

use super::{
    DxForgePublishPlanArtifactTarget, DxForgePublishPlanCacheHeader,
    DxForgePublishPlanRollbackInput, DxForgeReleaseBundleManifest, DxForgeReleaseBundleReport,
    DxForgeReleaseCandidateNoNodeModules, DxForgeReleaseOperationsCheck,
    DxForgeReleaseOperationsPackageGallery, DxForgeReleaseOperationsSignedManifest,
    FORGE_RELEASE_BUNDLE_LAUNCH_CHANGELOG_JSON, FORGE_RELEASE_BUNDLE_MANIFEST_JSON,
    FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_JSON, append_release_operations_check_finding,
    forge_publish_plan_artifact_targets, forge_publish_plan_cache_headers,
    forge_publish_plan_rollback_input, markdown_table_cell, release_operations_check,
    release_operations_manifest_score, summarize_forge_release_operations_manifest,
    summarize_forge_release_operations_package_gallery, verify_forge_release_bundle_with_options,
    verify_release_candidate_no_node_modules,
};

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeReleaseBundleInspectReport {
    version: u32,
    bundle: PathBuf,
    generated_at: String,
    pub(super) passed: bool,
    pub(super) score: u8,
    status: String,
    fail_under: u8,
    checks: DxForgeReleaseBundleInspectChecks,
    release_bundle: DxForgeReleaseBundleReport,
    signed_manifest: DxForgeReleaseOperationsSignedManifest,
    hosted_artifacts: Vec<DxForgePublishPlanArtifactTarget>,
    cache_headers: Vec<DxForgePublishPlanCacheHeader>,
    rollback_inputs: Vec<DxForgePublishPlanRollbackInput>,
    package_gallery: DxForgeReleaseOperationsPackageGallery,
    no_node_modules: DxForgeReleaseCandidateNoNodeModules,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeReleaseBundleInspectChecks {
    release_bundle: DxForgeReleaseOperationsCheck,
    signed_manifest: DxForgeReleaseOperationsCheck,
    hosted_artifacts: DxForgeReleaseOperationsCheck,
    rollback_inputs: DxForgeReleaseOperationsCheck,
    cache_policy: DxForgeReleaseOperationsCheck,
    package_gallery: DxForgeReleaseOperationsCheck,
    no_node_modules: DxForgeReleaseOperationsCheck,
}

pub(super) fn build_forge_release_bundle_inspect_report(
    bundle: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeReleaseBundleInspectReport> {
    let release_bundle = verify_forge_release_bundle_with_options(bundle, false)?;
    let manifest_path = bundle.join(FORGE_RELEASE_BUNDLE_MANIFEST_JSON);
    let signed_manifest = summarize_forge_release_operations_manifest(&manifest_path)?;
    let manifest = match std::fs::read(&manifest_path) {
        Ok(raw) => serde_json::from_slice::<DxForgeReleaseBundleManifest>(&raw).ok(),
        Err(_) => None,
    };
    let hosted_artifacts = manifest
        .as_ref()
        .map(|manifest| forge_release_bundle_inspect_hosted_artifacts(bundle, manifest))
        .unwrap_or_default();
    let cache_headers = forge_publish_plan_cache_headers();
    let rollback_inputs = forge_release_bundle_inspect_rollback_inputs(bundle, &manifest_path);
    let package_gallery =
        summarize_forge_release_operations_package_gallery(&manifest_path, bundle);
    let no_node_modules = verify_release_candidate_no_node_modules(&[bundle]);

    let hosted_target_count = hosted_artifacts
        .iter()
        .filter(|target| target.channel == "pages")
        .count();
    let hosted_targets_passed =
        !hosted_artifacts.is_empty() && hosted_artifacts.iter().all(|target| target.passed);
    let has_package_gallery_target = hosted_artifacts.iter().any(|target| {
        target.channel == "pages" && target.route.as_deref() == Some("/forge/package-gallery/")
    });
    let rollback_inputs_passed = rollback_inputs
        .iter()
        .filter(|input| input.required)
        .all(|input| input.passed);
    let hosted_targets_have_cache = hosted_artifacts
        .iter()
        .all(|target| !target.cache_control.trim().is_empty());
    let cache_policy_passed =
        hosted_targets_have_cache && cache_headers.iter().all(|header| header.passed);

    let checks = DxForgeReleaseBundleInspectChecks {
        release_bundle: release_operations_check(
            release_bundle.passed && release_bundle.score >= fail_under,
            release_bundle.score,
            format!(
                "release bundle has {} artifact(s), {} public route(s), no_node_modules={}.",
                release_bundle.artifact_count,
                release_bundle.route_count,
                release_bundle.no_node_modules
            ),
            Some(bundle.display().to_string()),
        ),
        signed_manifest: release_operations_check(
            signed_manifest.exists
                && signed_manifest.signed
                && signed_manifest.signature_verified
                && signed_manifest.manifest_digest_verified
                && signed_manifest.artifact_integrity_verified,
            release_operations_manifest_score(&signed_manifest),
            format!(
                "publisher status `{}` with signature_verified={} and artifact_integrity={}.",
                signed_manifest.publisher_status,
                signed_manifest.signature_verified,
                signed_manifest.artifact_integrity_verified
            ),
            Some(manifest_path.display().to_string()),
        ),
        hosted_artifacts: release_operations_check(
            hosted_targets_passed && has_package_gallery_target && hosted_target_count >= 6,
            if hosted_targets_passed && has_package_gallery_target {
                100
            } else {
                0
            },
            format!(
                "{} hosted Pages target(s), package_gallery_target={}.",
                hosted_target_count, has_package_gallery_target
            ),
            Some(bundle.display().to_string()),
        ),
        rollback_inputs: release_operations_check(
            rollback_inputs_passed,
            if rollback_inputs_passed { 100 } else { 0 },
            format!(
                "{} required rollback input(s) checked.",
                rollback_inputs
                    .iter()
                    .filter(|input| input.required)
                    .count()
            ),
            Some(bundle.display().to_string()),
        ),
        cache_policy: release_operations_check(
            cache_policy_passed,
            if cache_policy_passed { 100 } else { 0 },
            format!(
                "{} cache policy row(s), hosted target cache coverage={}.",
                cache_headers.len(),
                hosted_targets_have_cache
            ),
            None,
        ),
        package_gallery: release_operations_check(
            package_gallery.passed && package_gallery.score >= fail_under,
            package_gallery.score,
            format!(
                "package gallery has {} artifact(s), {} package(s), and {} migration guide(s).",
                package_gallery.artifact_count,
                package_gallery.package_count,
                package_gallery.migration_guide_count
            ),
            Some(package_gallery.html_path.display().to_string()),
        ),
        no_node_modules: release_operations_check(
            no_node_modules.passed,
            no_node_modules.score,
            format!(
                "{} release-bundle boundary path(s) checked.",
                no_node_modules.checked_paths.len()
            ),
            None,
        ),
    };

    let score = [
        checks.release_bundle.score,
        checks.signed_manifest.score,
        checks.hosted_artifacts.score,
        checks.rollback_inputs.score,
        checks.cache_policy.score,
        checks.package_gallery.score,
        checks.no_node_modules.score,
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let mut findings = Vec::new();
    append_release_operations_check_finding(
        "release-bundle",
        &checks.release_bundle,
        &mut findings,
    );
    append_release_operations_check_finding(
        "signed-manifest",
        &checks.signed_manifest,
        &mut findings,
    );
    append_release_operations_check_finding(
        "hosted-artifacts",
        &checks.hosted_artifacts,
        &mut findings,
    );
    append_release_operations_check_finding(
        "rollback-inputs",
        &checks.rollback_inputs,
        &mut findings,
    );
    append_release_operations_check_finding("cache-policy", &checks.cache_policy, &mut findings);
    append_release_operations_check_finding(
        "package-gallery",
        &checks.package_gallery,
        &mut findings,
    );
    append_release_operations_check_finding(
        "no-node-modules",
        &checks.no_node_modules,
        &mut findings,
    );
    findings.extend(
        release_bundle
            .findings
            .iter()
            .map(|finding| format!("release-bundle: {finding}")),
    );
    findings.extend(
        signed_manifest
            .findings
            .iter()
            .map(|finding| format!("signed-manifest: {finding}")),
    );
    findings.extend(
        hosted_artifacts
            .iter()
            .filter(|target| !target.passed)
            .map(|target| format!("hosted-artifacts: {}", target.message)),
    );
    findings.extend(
        rollback_inputs
            .iter()
            .filter(|input| input.required && !input.passed)
            .map(|input| format!("rollback-inputs: {}", input.message)),
    );
    findings.extend(
        package_gallery
            .findings
            .iter()
            .map(|finding| format!("package-gallery: {finding}")),
    );
    findings.extend(
        no_node_modules
            .findings
            .iter()
            .map(|finding| format!("no-node-modules: {finding}")),
    );

    let passed = findings.is_empty()
        && score >= fail_under
        && checks.release_bundle.passed
        && checks.signed_manifest.passed
        && checks.hosted_artifacts.passed
        && checks.rollback_inputs.passed
        && checks.cache_policy.passed
        && checks.package_gallery.passed
        && checks.no_node_modules.passed;
    let status = if passed {
        "ready-for-beta-review"
    } else {
        "needs-review"
    }
    .to_string();

    Ok(DxForgeReleaseBundleInspectReport {
        version: 1,
        bundle: bundle.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        status,
        fail_under,
        checks,
        release_bundle,
        signed_manifest,
        hosted_artifacts,
        cache_headers,
        rollback_inputs,
        package_gallery,
        no_node_modules,
        findings,
        next_commands: vec![
            format!(
                "dx forge release-bundle-inspect --bundle {} --format markdown --fail-under {}",
                bundle.display(),
                fail_under
            ),
            format!(
                "dx forge beta-install --release-bundle {} --dry-run --format markdown",
                bundle.display()
            ),
            "dx forge publish-plan --project . --release-bundle <bundle> --pages <pages> --registry-smoke <json> --release-operations <json> --format markdown".to_string(),
        ],
    })
}

fn forge_release_bundle_inspect_hosted_artifacts(
    bundle: &Path,
    manifest: &DxForgeReleaseBundleManifest,
) -> Vec<DxForgePublishPlanArtifactTarget> {
    forge_publish_plan_artifact_targets(bundle, manifest, &serde_json::Value::Null)
        .into_iter()
        .filter(|target| target.channel == "pages")
        .collect()
}

pub(super) fn forge_release_bundle_inspect_rollback_inputs(
    bundle: &Path,
    manifest_path: &Path,
) -> Vec<DxForgePublishPlanRollbackInput> {
    vec![
        forge_publish_plan_rollback_input(
            "release_bundle",
            bundle.to_path_buf(),
            true,
            "Verified release bundle folder can be promoted or rolled back as one unit.",
        ),
        forge_publish_plan_rollback_input(
            "signed_release_manifest",
            manifest_path.to_path_buf(),
            true,
            "Signed manifest pins every hosted artifact and publisher identity.",
        ),
        forge_publish_plan_rollback_input(
            "public_route_comparison",
            bundle.join("forge-public-route-comparison.json"),
            true,
            "Route comparison records the public route payload and budget state.",
        ),
        forge_publish_plan_rollback_input(
            "public_release_history",
            bundle.join("forge-public-release-history.json"),
            true,
            "Release history preserves the reviewed public route and dashboard state.",
        ),
        forge_publish_plan_rollback_input(
            "launch_changelog",
            bundle.join(FORGE_RELEASE_BUNDLE_LAUNCH_CHANGELOG_JSON),
            true,
            "Launch changelog explains the promoted release proof for rollback review.",
        ),
        forge_publish_plan_rollback_input(
            "package_gallery",
            bundle.join(FORGE_RELEASE_BUNDLE_PACKAGE_GALLERY_JSON),
            true,
            "Package gallery keeps source-owned package coverage reviewable before install.",
        ),
    ]
}

pub(super) fn forge_release_bundle_inspect_terminal(
    report: &DxForgeReleaseBundleInspectReport,
) -> String {
    let mut output = format!(
        "DX Forge release bundle inspector\nBundle: {}\nStatus: {} ({} / 100)\nPassed: {}\nSigned manifest: {}\nHosted targets: {}\nRollback inputs: {}\nPackage gallery packages: {}\nNo node_modules: {}\n",
        report.bundle.display(),
        report.status,
        report.score,
        report.passed,
        report.signed_manifest.signature_verified,
        report.hosted_artifacts.len(),
        report.rollback_inputs.len(),
        report.package_gallery.package_count,
        report.no_node_modules.passed
    );
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

pub(super) fn forge_release_bundle_inspect_markdown(
    report: &DxForgeReleaseBundleInspectReport,
) -> String {
    let mut output = format!(
        "# DX Forge Release Bundle Inspector\n\n- Bundle: `{}`\n- Generated: `{}`\n- Status: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Signed manifest verified: `{}`\n- no `node_modules`: `{}`\n\n",
        report.bundle.display(),
        report.generated_at,
        report.status,
        report.passed,
        report.score,
        report.fail_under,
        report.signed_manifest.signature_verified,
        report.no_node_modules.passed
    );

    output.push_str("## Checks\n\n");
    output.push_str("| Check | Passed | Score | Message |\n");
    output.push_str("| --- | --- | ---: | --- |\n");
    for (label, check) in [
        ("release bundle", &report.checks.release_bundle),
        ("signed manifest", &report.checks.signed_manifest),
        ("hosted artifacts", &report.checks.hosted_artifacts),
        ("rollback inputs", &report.checks.rollback_inputs),
        ("cache policy", &report.checks.cache_policy),
        ("package gallery", &report.checks.package_gallery),
        ("no node_modules", &report.checks.no_node_modules),
    ] {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | {} |\n",
            label,
            check.passed,
            check.score,
            markdown_table_cell(&check.message)
        ));
    }

    output.push_str("\n## Hosted Artifacts\n\n");
    output.push_str("| Channel | Artifact | Route | Cache-Control | Source |\n");
    output.push_str("| --- | --- | --- | --- | --- |\n");
    for target in &report.hosted_artifacts {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            target.channel,
            markdown_table_cell(&target.artifact),
            markdown_table_cell(target.route.as_deref().unwrap_or("global")),
            markdown_table_cell(&target.cache_control),
            markdown_table_cell(&target.source)
        ));
    }

    output.push_str("\n## Rollback Inputs\n\n");
    output.push_str("| Input | Exists | Path | Why |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for input in &report.rollback_inputs {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} |\n",
            input.name,
            input.exists,
            markdown_table_cell(&input.path.display().to_string()),
            markdown_table_cell(&input.message)
        ));
    }

    output.push_str("\n## Cache Policy\n\n");
    output.push_str("| Channel | Pattern | Cache-Control | Reason |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for header in &report.cache_headers {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} |\n",
            header.channel,
            markdown_table_cell(&header.pattern),
            markdown_table_cell(&header.cache_control),
            markdown_table_cell(&header.reason)
        ));
    }

    output.push_str("\n## Package Gallery\n\n");
    output.push_str(&format!(
        "- Route: `{}`\n- Passed: `{}`\n- Packages: `{}`\n- Migration guides: `{}`\n- HTML: `{}`\n\n",
        report.package_gallery.route,
        report.package_gallery.passed,
        report.package_gallery.package_count,
        report.package_gallery.migration_guide_count,
        report.package_gallery.html_path.display()
    ));

    output.push_str("## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- No release-bundle inspector findings for the configured threshold.\n");
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

pub(super) fn forge_release_bundle_inspect_failure_summary(
    report: &DxForgeReleaseBundleInspectReport,
) -> String {
    if report.findings.is_empty() {
        return format!(
            "DX Forge release-bundle-inspect did not pass: score {} / 100, required {} / 100",
            report.score, report.fail_under
        );
    }
    format!(
        "DX Forge release-bundle-inspect did not pass: {}",
        report.findings.join("; ")
    )
}
