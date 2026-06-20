#[test]
fn forge_publisher_key_commands_generate_and_sign_release_manifest() {
    let project = tempdir().expect("project tempdir");
    let cli = Cli::with_cwd(project.path().to_path_buf());
    let dashboard_path = write_passing_release_dashboard(project.path());
    let route_comparison_path = write_passing_public_route_comparison(project.path());
    let release_history_path = project
        .path()
        .join("benchmarks/reports/forge-public-release-history.json");
    let bundle_dir = project.path().join("release-bundle");

    cli.cmd_forge(&[
        "release-history".to_string(),
        "--dashboard".to_string(),
        dashboard_path.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        route_comparison_path.to_string_lossy().into_owned(),
        "--output".to_string(),
        release_history_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("release history fixture");
    cli.cmd_forge(&[
        "release-bundle".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--out".to_string(),
        bundle_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--quiet".to_string(),
    ])
    .expect("release bundle");

    let key_dir = project.path().join(".dx/forge/publisher");
    let key_report_path = project.path().join("publisher-key.json");
    cli.cmd_forge(&[
        "publisher-key".to_string(),
        "generate".to_string(),
        "--out".to_string(),
        key_dir.to_string_lossy().into_owned(),
        "--signer".to_string(),
        "dx-forge-test-publisher".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        key_report_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("publisher key generate");

    let key_report_text = fs::read_to_string(&key_report_path).expect("publisher key report");
    assert!(!key_report_text.contains("ed25519-seed:"));
    let key_report: serde_json::Value =
        serde_json::from_str(&key_report_text).expect("publisher key report json");
    assert_eq!(key_report["passed"], true);
    assert_eq!(key_report["signer"], "dx-forge-test-publisher");
    assert!(
        key_report["key_id"]
            .as_str()
            .expect("key id")
            .starts_with("ed25519-blake3:")
    );
    let private_key_path = PathBuf::from(
        key_report["private_key_path"]
            .as_str()
            .expect("private key path"),
    );
    let public_key_path = PathBuf::from(
        key_report["public_key_path"]
            .as_str()
            .expect("public key path"),
    );
    assert!(private_key_path.is_file());
    assert!(public_key_path.is_file());
    assert!(
        fs::read_to_string(&private_key_path)
            .expect("private key")
            .contains("private_key")
    );
    assert!(
        !fs::read_to_string(&public_key_path)
            .expect("public key")
            .contains("private_key")
    );

    let manifest_path = bundle_dir.join(FORGE_RELEASE_BUNDLE_MANIFEST_JSON);
    let sign_report_path = project.path().join("publisher-sign.json");
    cli.cmd_forge(&[
        "publisher-key".to_string(),
        "sign".to_string(),
        "--key".to_string(),
        private_key_path.to_string_lossy().into_owned(),
        "--manifest".to_string(),
        manifest_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        sign_report_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("publisher key sign");

    let sign_report_text = fs::read_to_string(&sign_report_path).expect("sign report");
    assert!(!sign_report_text.contains("ed25519-seed:"));
    let sign_report: serde_json::Value =
        serde_json::from_str(&sign_report_text).expect("sign report json");
    assert_eq!(sign_report["passed"], true);
    assert_eq!(sign_report["signature_verified"], true);
    assert_eq!(sign_report["wrote_manifest"], true);
    assert_eq!(sign_report["signer"], "dx-forge-test-publisher");

    let signed_manifest: DxForgeReleaseBundleManifest =
        serde_json::from_slice(&fs::read(&manifest_path).expect("signed manifest bytes"))
            .expect("signed manifest json");
    assert_eq!(signed_manifest.publisher_identity.status, "signed");
    assert_eq!(
        signed_manifest.publisher_identity.signer.as_deref(),
        Some("dx-forge-test-publisher")
    );
    assert!(signed_manifest.publisher_identity.signature.is_some());

    cli.cmd_forge(&[
        "release-bundle".to_string(),
        "--verify".to_string(),
        bundle_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("signed release bundle verifies");

    assert!(!project.path().join("node_modules").exists());
    assert!(!bundle_dir.join("node_modules").exists());
}

#[test]
fn forge_release_bundle_can_optionally_include_adoption_route() {
    let default_project = tempdir().expect("default project tempdir");
    let default_cli = Cli::with_cwd(default_project.path().to_path_buf());
    let default_dashboard_path = write_passing_release_dashboard(default_project.path());
    let default_route_comparison_path =
        write_passing_public_route_comparison_with_adoption(default_project.path());
    let default_release_history_path = default_project
        .path()
        .join("benchmarks/reports/forge-public-release-history.json");
    let default_bundle_dir = default_project.path().join("release-bundle-default");

    default_cli
        .cmd_forge(&[
            "release-history".to_string(),
            "--dashboard".to_string(),
            default_dashboard_path.to_string_lossy().into_owned(),
            "--route-comparison".to_string(),
            default_route_comparison_path.to_string_lossy().into_owned(),
            "--output".to_string(),
            default_release_history_path.to_string_lossy().into_owned(),
            "--quiet".to_string(),
        ])
        .expect("default release history fixture");

    default_cli
        .cmd_forge(&[
            "release-bundle".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--out".to_string(),
            default_bundle_dir.to_string_lossy().into_owned(),
            "--format".to_string(),
            "json".to_string(),
            "--quiet".to_string(),
        ])
        .expect("default release bundle");
    let default_report =
        verify_forge_release_bundle(&default_bundle_dir).expect("default bundle verify");
    assert_eq!(default_report.route_count, 6);
    assert!(!default_bundle_dir.join("forge/adoption.html").exists());
    assert!(
        !default_bundle_dir
            .join("forge-adoption-report.json")
            .exists()
    );

    let adoption_project = tempdir().expect("adoption project tempdir");
    let adoption_cli = Cli::with_cwd(adoption_project.path().to_path_buf());
    let adoption_dashboard_path = write_passing_release_dashboard(adoption_project.path());
    let adoption_route_comparison_path =
        write_passing_public_route_comparison_with_adoption(adoption_project.path());
    let adoption_release_history_path = adoption_project
        .path()
        .join("benchmarks/reports/forge-public-release-history.json");
    let adoption_bundle_dir = adoption_project.path().join("release-bundle-adoption");

    adoption_cli
        .cmd_forge(&[
            "release-history".to_string(),
            "--dashboard".to_string(),
            adoption_dashboard_path.to_string_lossy().into_owned(),
            "--route-comparison".to_string(),
            adoption_route_comparison_path
                .to_string_lossy()
                .into_owned(),
            "--output".to_string(),
            adoption_release_history_path.to_string_lossy().into_owned(),
            "--quiet".to_string(),
        ])
        .expect("adoption release history fixture");

    adoption_cli
        .cmd_forge(&[
            "release-bundle".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--out".to_string(),
            adoption_bundle_dir.to_string_lossy().into_owned(),
            "--include-adoption".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--quiet".to_string(),
        ])
        .expect("adoption release bundle");

    for artifact in [
        "forge-adoption-smoke.json",
        "forge-adoption-report.json",
        "forge-adoption-report.md",
        "forge-adoption-page.html",
        "forge/adoption.html",
        "forge/adoption/index.html",
        "forge/adoption.claims.json",
        "forge/adoption.dxp",
        "forge/adoption.proof.json",
    ] {
        assert!(
            adoption_bundle_dir.join(artifact).is_file(),
            "adoption release bundle missing `{artifact}`"
        );
    }

    let adoption_report =
        verify_forge_release_bundle(&adoption_bundle_dir).expect("adoption bundle verify");
    assert_eq!(adoption_report.route_count, 7);
    assert!(adoption_report.routes.iter().any(|route| {
        route.route == "/forge/adoption"
            && route.passed
            && route.artifacts.contains(&"forge/adoption.html".to_string())
    }));
    let manifest: DxForgeReleaseBundleManifest = serde_json::from_slice(
        &std::fs::read(adoption_bundle_dir.join("forge-release-.dx/build-cache/manifest.json")).unwrap(),
    )
    .expect("adoption release manifest json");
    assert!(manifest.artifacts.iter().any(|artifact| {
        artifact.path == "forge/adoption.html"
            && artifact.artifact_type == "route-html"
            && artifact.route.as_deref() == Some("/forge/adoption")
    }));
    assert!(!default_project.path().join("node_modules").exists());
    assert!(!adoption_project.path().join("node_modules").exists());
    assert!(!default_bundle_dir.join("node_modules").exists());
    assert!(!adoption_bundle_dir.join("node_modules").exists());
}

#[test]
fn forge_release_bundle_snapshots_cover_manifest_and_pages_shape() {
    let project = tempdir().expect("project tempdir");
    let cli = Cli::with_cwd(project.path().to_path_buf());
    let dashboard_path = write_passing_release_dashboard(project.path());
    let route_comparison_path = write_passing_public_route_comparison(project.path());
    let release_history_path = project
        .path()
        .join("benchmarks/reports/forge-public-release-history.json");
    let bundle_dir = project.path().join("release-bundle");

    cli.cmd_forge(&[
        "release-history".to_string(),
        "--dashboard".to_string(),
        dashboard_path.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        route_comparison_path.to_string_lossy().into_owned(),
        "--output".to_string(),
        release_history_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("release history fixture");

    cli.cmd_forge(&[
        "release-bundle".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--out".to_string(),
        bundle_dir.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("release bundle");

    let manifest: DxForgeReleaseBundleManifest = serde_json::from_slice(
        &fs::read(bundle_dir.join(FORGE_RELEASE_BUNDLE_MANIFEST_JSON)).unwrap(),
    )
    .expect("release bundle manifest");
    assert_forge_json_snapshot(
        "release-bundle-manifest-shape.json",
        forge_release_manifest_shape_contract(&manifest),
    );
    assert_forge_release_manifest_markdown_snapshot(
        "forge-release-manifest.md",
        &fs::read_to_string(bundle_dir.join(FORGE_RELEASE_BUNDLE_MANIFEST_MD))
            .expect("release manifest markdown"),
    );

    let pages_project = tempdir().expect("pages project");
    let pages_cli = Cli::with_cwd(pages_project.path().to_path_buf());
    let badge_project = tempdir().expect("badge project");
    let badge_cli = Cli::with_cwd(badge_project.path().to_path_buf());
    let ci_artifact_dir = badge_project.path().join("ci-artifacts");
    let pages_dir = pages_project.path().join("forge-pages");
    badge_cli
        .cmd_forge(&[
            "ci".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--out".to_string(),
            ci_artifact_dir.to_string_lossy().into_owned(),
            "--quiet".to_string(),
        ])
        .expect("forge ci artifacts");
    write_forge_pages_publish_bundle(
        &pages_cli,
        pages_project.path(),
        &pages_dir,
        &ci_artifact_dir,
    );

    let pages_report = verify_forge_pages_bundle(&pages_dir).expect("pages bundle report");
    assert_forge_json_snapshot(
        "pages-bundle-shape.json",
        forge_pages_bundle_shape_contract(&pages_report),
    );
    assert_forge_pages_shape_markdown_snapshot(
        "forge-pages-bundle-shape.md",
        &forge_pages_bundle_shape_markdown(&pages_report),
    );

    assert!(!project.path().join("node_modules").exists());
    assert!(!bundle_dir.join("node_modules").exists());
    assert!(!pages_project.path().join("node_modules").exists());
    assert!(!badge_project.path().join("node_modules").exists());
}

#[test]
fn forge_release_review_joins_operator_signoff_evidence() {
    let project = tempdir().expect("project tempdir");
    let cli = Cli::with_cwd(project.path().to_path_buf());
    let dashboard_path = write_passing_release_dashboard(project.path());
    let route_comparison_path = write_passing_public_route_comparison(project.path());
    let release_history_path = project
        .path()
        .join("benchmarks/reports/forge-public-release-history.json");
    let bundle_dir = project.path().join("release-bundle");

    cli.cmd_forge(&[
        "release-history".to_string(),
        "--dashboard".to_string(),
        dashboard_path.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        route_comparison_path.to_string_lossy().into_owned(),
        "--output".to_string(),
        release_history_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("release history fixture");

    cli.cmd_forge(&[
        "release-bundle".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--out".to_string(),
        bundle_dir.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("release bundle fixture");

    let review_json_path = project.path().join("release-review.json");
    cli.cmd_forge(&[
        "release-review".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--bundle".to_string(),
        bundle_dir.to_string_lossy().into_owned(),
        "--dashboard".to_string(),
        dashboard_path.to_string_lossy().into_owned(),
        "--history".to_string(),
        release_history_path.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        route_comparison_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        review_json_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("release review json");

    let review = read_json_value(review_json_path);
    assert_eq!(review["passed"], true);
    assert_eq!(review["status"], "ready-for-human-signoff");
    assert!(review["score"].as_u64().expect("review score") >= 90);
    assert_eq!(review["checks"]["release_dashboard"]["passed"], true);
    assert_eq!(review["checks"]["release_bundle"]["passed"], true);
    assert_eq!(review["checks"]["bundle_manifest"]["passed"], true);
    assert_eq!(review["checks"]["launch_changelog"]["passed"], true);
    assert_eq!(review["checks"]["route_comparison"]["passed"], true);
    assert_eq!(review["checks"]["release_history"]["passed"], true);
    assert_eq!(review["release_bundle"]["manifest_digest_verified"], true);
    assert!(
        review["signoff_items"]
            .as_array()
            .expect("signoff items")
            .iter()
            .any(|item| item["label"] == "Review release bundle manifest digest")
    );

    let review_markdown_path = project.path().join("release-review.md");
    cli.cmd_forge(&[
        "release-review".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--bundle".to_string(),
        bundle_dir.to_string_lossy().into_owned(),
        "--dashboard".to_string(),
        dashboard_path.to_string_lossy().into_owned(),
        "--history".to_string(),
        release_history_path.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        route_comparison_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "markdown".to_string(),
        "--output".to_string(),
        review_markdown_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("release review markdown");
    let review_markdown = fs::read_to_string(review_markdown_path).expect("review markdown");
    assert!(review_markdown.contains("# DX Forge Release Review"));
    assert!(review_markdown.contains("## Human Signoff"));
    assert!(review_markdown.contains("release-dashboard"));
    assert!(review_markdown.contains("release bundle manifest"));
    assert!(review_markdown.contains("launch changelog"));
    assert!(!review_markdown.contains("CLOUDFLARE_R2_"));
    assert!(!review_markdown.contains("DX_FORGE_R2_LIVE"));
    assert!(!project.path().join("node_modules").exists());
    assert!(!bundle_dir.join("node_modules").exists());
}

#[test]
fn forge_launch_copy_review_accepts_honest_beta_copy_with_verified_highlights() {
    let project = tempdir().expect("project tempdir");
    let cli = Cli::with_cwd(project.path().to_path_buf());
    let copy_path = project.path().join("public-beta-copy.md");
    fs::write(
            &copy_path,
            r#"# DX Forge Public Beta

DX Forge materializes curated source-owned packages into editable app files with receipts, rollback evidence, scorecards, strict launch gates, no `node_modules`, and blocked install-time package scripts.

The public Forge surface is static/no-runtime compiler output measured by route comparison evidence, with seven public routes, Brotli budget checks, and local browser timing notes.

This is not a universal npm replacement, not a full framework benchmark, does not claim live customer traffic, and does not prevent every supply-chain attack.
"#,
        )
        .expect("copy fixture");
    let route_comparison_path = write_passing_public_route_comparison_with_adoption(project.path());
    let source_review_path = write_passing_source_owned_review_report(project.path());
    let static_evidence_path = write_passing_static_competitor_evidence_report(project.path());
    let output_path = project.path().join("launch-copy-review.json");

    cli.cmd_forge(&[
        "launch-copy-review".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--copy".to_string(),
        copy_path.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        route_comparison_path.to_string_lossy().into_owned(),
        "--source-review".to_string(),
        source_review_path.to_string_lossy().into_owned(),
        "--static-evidence".to_string(),
        static_evidence_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("launch copy review");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert!(report["score"].as_u64().expect("copy review score") >= 90);
    assert_eq!(report["checks"]["blocked_claims"]["passed"], true);
    assert_eq!(report["checks"]["required_caveats"]["passed"], true);
    assert_eq!(report["checks"]["source_owned_security"]["passed"], true);
    assert_eq!(report["checks"]["static_route_performance"]["passed"], true);
    assert!(
        report["evidence"]["route_comparison"]["route_count"]
            .as_u64()
            .expect("route count")
            >= FORGE_REQUIRED_PUBLIC_ROUTES.len() as u64
    );
    assert!(
        report["approved_claims"]
            .as_array()
            .expect("approved claims")
            .iter()
            .any(|claim| claim
                .as_str()
                .is_some_and(|claim| claim.contains("source-owned packages")))
    );
    assert!(
        report["approved_claims"]
            .as_array()
            .expect("approved claims")
            .iter()
            .any(|claim| claim
                .as_str()
                .is_some_and(|claim| claim.contains("static/no-runtime")))
    );
    assert!(!project.path().join("node_modules").exists());
}

#[test]
fn forge_launch_copy_review_blocks_universal_replacement_claims() {
    let project = tempdir().expect("project tempdir");
    let cli = Cli::with_cwd(project.path().to_path_buf());
    let copy_path = project.path().join("bad-public-beta-copy.md");
    fs::write(
            &copy_path,
            "DX Forge replaces npm and Next.js, beats every frontend framework, and prevents every supply-chain attack.",
        )
        .expect("bad copy fixture");
    let route_comparison_path = write_passing_public_route_comparison_with_adoption(project.path());
    let source_review_path = write_passing_source_owned_review_report(project.path());
    let static_evidence_path = write_passing_static_competitor_evidence_report(project.path());
    let output_path = project.path().join("launch-copy-review.json");

    let error = cli
        .cmd_forge(&[
            "launch-copy-review".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--copy".to_string(),
            copy_path.to_string_lossy().into_owned(),
            "--route-comparison".to_string(),
            route_comparison_path.to_string_lossy().into_owned(),
            "--source-review".to_string(),
            source_review_path.to_string_lossy().into_owned(),
            "--static-evidence".to_string(),
            static_evidence_path.to_string_lossy().into_owned(),
            "--format".to_string(),
            "json".to_string(),
            "--output".to_string(),
            output_path.to_string_lossy().into_owned(),
            "--fail-under".to_string(),
            "90".to_string(),
            "--quiet".to_string(),
        ])
        .expect_err("overclaim copy should fail review");

    assert!(error.to_string().contains("launch-copy-review"));
    let report = read_json_value(output_path);
    assert_eq!(report["passed"], false);
    assert_eq!(report["checks"]["blocked_claims"]["passed"], false);
    assert!(
        report["blocked_claims"]
            .as_array()
            .expect("blocked claims")
            .iter()
            .any(|claim| claim["pattern"] == "replaces npm")
    );
    assert!(
        report["findings"]
            .as_array()
            .expect("findings")
            .iter()
            .any(|finding| finding
                .as_str()
                .is_some_and(|finding| finding.contains("blocked public claim")))
    );
    assert!(!project.path().join("node_modules").exists());
}

#[test]
fn forge_launch_bundle_temp_project_smoke_runs_release_commands() {
    let project = tempdir().expect("project tempdir");
    let cli = Cli::with_cwd(project.path().to_path_buf());
    let dashboard_path = write_passing_release_dashboard(project.path());
    let route_comparison_path = write_passing_public_route_comparison(project.path());
    let release_history_path = project
        .path()
        .join("benchmarks/reports/forge-public-release-history.json");
    let bundle_dir = project.path().join("release-bundle");

    let route_guard = verify_release_dashboard_route_comparison(&route_comparison_path)
        .expect("route comparison guard");
    assert!(
        route_guard.passed,
        "route comparison guard should pass: {:?}",
        route_guard.findings
    );
    assert_eq!(route_guard.route_count, FORGE_REQUIRED_PUBLIC_ROUTES.len());
    assert!(route_guard.missing_routes.is_empty());

    cli.cmd_forge(&[
        "release-history".to_string(),
        "--dashboard".to_string(),
        dashboard_path.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        route_comparison_path.to_string_lossy().into_owned(),
        "--output".to_string(),
        release_history_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("release history fixture");

    let launch_changelog_path = project.path().join("launch-changelog.json");
    cli.cmd_forge(&[
        "launch-changelog".to_string(),
        "--history".to_string(),
        release_history_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        launch_changelog_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("launch changelog");
    let launch_changelog = read_json_value(launch_changelog_path);
    assert_eq!(launch_changelog["passed"], true);
    assert_eq!(
        launch_changelog["latest"]["route_count"],
        serde_json::json!(FORGE_REQUIRED_PUBLIC_ROUTES.len() as u64)
    );
    assert!(
        launch_changelog["honest_scope"]
            .as_array()
            .expect("honest scope")
            .len()
            >= 4
    );

    cli.cmd_forge(&[
        "release-bundle".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--out".to_string(),
        bundle_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("release bundle");

    cli.cmd_forge(&[
        "release-bundle".to_string(),
        "--verify".to_string(),
        bundle_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("release bundle verify");

    let pages_project = tempdir().expect("pages project");
    let pages_cli = Cli::with_cwd(pages_project.path().to_path_buf());
    let pages_dir = pages_project.path().join("forge-pages");
    write_forge_pages_publish_bundle(&pages_cli, pages_project.path(), &pages_dir, &bundle_dir);

    let ci_project = tempdir().expect("release dashboard CI project");
    let ci_cli = Cli::with_cwd(ci_project.path().to_path_buf());
    let ci_artifact_dir = ci_project.path().join("ci-artifacts");
    ci_cli
        .cmd_forge(&[
            "ci".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--out".to_string(),
            ci_artifact_dir.to_string_lossy().into_owned(),
            "--fail-under".to_string(),
            "90".to_string(),
            "--quiet".to_string(),
        ])
        .expect("release dashboard CI artifacts");

    let dashboard_project = tempdir().expect("release dashboard project");
    let dashboard_cli = Cli::with_cwd(dashboard_project.path().to_path_buf());
    let release_dashboard_path = dashboard_project.path().join("release-dashboard.json");
    dashboard_cli
        .cmd_forge(&[
            "release-dashboard".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--ci-artifacts".to_string(),
            ci_artifact_dir.to_string_lossy().into_owned(),
            "--pages".to_string(),
            pages_dir.to_string_lossy().into_owned(),
            "--history".to_string(),
            bundle_dir
                .join("forge-benchmark-history.json")
                .to_string_lossy()
                .into_owned(),
            "--route-comparison".to_string(),
            route_comparison_path.to_string_lossy().into_owned(),
            "--format".to_string(),
            "json".to_string(),
            "--output".to_string(),
            release_dashboard_path.to_string_lossy().into_owned(),
            "--fail-under".to_string(),
            "90".to_string(),
            "--quiet".to_string(),
        ])
        .expect("release dashboard");
    let release_dashboard = read_json_value(release_dashboard_path);
    assert_eq!(release_dashboard["passed"], true);
    assert_eq!(
        release_dashboard["checks"]["route_comparison"]["passed"],
        true
    );
    assert_eq!(
        release_dashboard["checks"]["launch_changelog"]["passed"],
        true
    );
    assert_eq!(
        release_dashboard["route_comparison"]["route_count"],
        serde_json::json!(FORGE_REQUIRED_PUBLIC_ROUTES.len() as u64)
    );

    assert!(bundle_dir.join(FORGE_RELEASE_BUNDLE_MANIFEST_JSON).exists());
    assert!(
        bundle_dir
            .join(FORGE_RELEASE_BUNDLE_LAUNCH_CHANGELOG_JSON)
            .exists()
    );
    assert!(bundle_dir.join("forge/changelog.html").exists());
    assert_secret_markers_absent(&bundle_dir);
    assert_secret_markers_absent(&ci_artifact_dir);
    assert_secret_markers_absent(&pages_dir);
    assert!(!project.path().join("node_modules").exists());
    assert!(!bundle_dir.join("node_modules").exists());
    assert!(!ci_project.path().join("node_modules").exists());
    assert!(!ci_artifact_dir.join("node_modules").exists());
    assert!(!dashboard_project.path().join("node_modules").exists());
    assert!(!pages_project.path().join("node_modules").exists());
    assert!(!pages_dir.join("node_modules").exists());
}

#[test]
fn forge_launch_temp_project_smoke_covers_current_launch_packages() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_add(&["ui/button", "--write"])
        .expect("dx add ui/button");
    cli.cmd_add(&["ui/card", "--write"])
        .expect("dx add ui/card");
    cli.cmd_add(&["ui/input", "--write"])
        .expect("dx add ui/input");
    cli.cmd_add(&["ui/textarea", "--write"])
        .expect("dx add ui/textarea");
    cli.cmd_add(&["icon", "search", "--write"])
        .expect("dx add icon search");
    cli.cmd_add(&["auth/better-auth", "--write"])
        .expect("dx add auth/better-auth");
    cli.cmd_add(&["db/drizzle", "--write"])
        .expect("dx add db/drizzle");
    cli.cmd_add(&["migration/static-site", "--write"])
        .expect("dx add migration/static-site");

    cli.cmd_check(&[
        ".".to_string(),
        "--strict-forge".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--fail-under".to_string(),
        "90".to_string(),
    ])
    .expect("strict forge check");

    cli.cmd_forge(&[
        "doctor".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--fail-under".to_string(),
        "90".to_string(),
    ])
    .expect("forge doctor");

    let scorecard_path = dir.path().join("forge-scorecard.json");
    cli.cmd_forge(&[
        "scorecard".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        scorecard_path.to_string_lossy().into_owned(),
    ])
    .expect("forge scorecard");

    let manifest: dx_compiler::ecosystem::DxSourceManifest = serde_json::from_slice(
        &fs::read(dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json")).expect("manifest bytes"),
    )
    .expect("manifest json");
    let packages = manifest
        .packages
        .iter()
        .map(|package| package.package_id.as_str())
        .collect::<Vec<_>>();
    assert!(packages.contains(&"shadcn/ui/button"));
    assert!(packages.contains(&"shadcn/ui/card"));
    assert!(packages.contains(&"shadcn/ui/input"));
    assert!(packages.contains(&"shadcn/ui/textarea"));
    assert!(packages.contains(&"dx/icon/search"));
    assert!(packages.contains(&"auth/better-auth"));
    assert!(packages.contains(&"auth/better-auth"));
    assert!(packages.contains(&"db/drizzle-sqlite"));
    assert!(packages.contains(&"migration/static-site"));

    let scorecard = fs::read_to_string(scorecard_path).expect("scorecard report");
    assert!(scorecard.contains("\"score\": 100"));
    assert!(scorecard.contains("shadcn/ui/button"));
    assert!(scorecard.contains("shadcn/ui/card"));
    assert!(scorecard.contains("shadcn/ui/input"));
    assert!(scorecard.contains("shadcn/ui/textarea"));
    assert!(scorecard.contains("dx/icon/search"));
    assert!(scorecard.contains("auth/better-auth"));
    assert!(scorecard.contains("auth/better-auth"));
    assert!(scorecard.contains("db/drizzle-sqlite"));
    assert!(scorecard.contains("migration/static-site"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_init_app_dry_run_reports_without_writing_project_files() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("init-app.json");

    cli.cmd_forge(&[
        "init-app".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--dry-run".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("forge init-app dry-run");

    let report = read_json_value(output_path);
    assert_eq!(report["mode"], "dry-run");
    assert_eq!(report["passed"], true);
    assert_eq!(report["no_node_modules"], true);
    assert_eq!(
        report["package_ids"].as_array().expect("package ids").len(),
        FORGE_WWW_TEMPLATE_PACKAGE_IDS.len()
    );
    assert!(report["planned_files"].as_array().expect("files").len() >= 4);
    assert!(!dir.path().join("dx").exists());
    assert!(!dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_init_app_write_creates_beta_project_without_node_modules() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("init-app.json");

    cli.cmd_forge(&[
        "init-app".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("forge init-app write");

    let report = read_json_value(output_path);
    assert_eq!(report["mode"], "write");
    assert_eq!(report["passed"], true);
    assert_eq!(report["no_node_modules"], true);
    assert!(report["score"].as_u64().expect("score") >= 90);
    for path in [
        "dx",
        "README.md",
        "app/page.tsx",
        "app/layout.tsx",
        "components/local/WelcomeCard.tsx",
        "server/actions.ts",
        "styles/tokens.css",
        "pages/index.html",
        "pages/forge-adoption.html",
        "components/ui/button.tsx",
        "components/ui/card.tsx",
        "components/ui/input.tsx",
        ".dx/forge/source-.dx/build-cache/manifest.json",
        ".dx/forge/docs/shadcn-ui-button.md",
        ".dx/forge/docs/shadcn-ui-card.md",
        ".dx/forge/docs/shadcn-ui-input.md",
        ".dx/forge/docs/dx-icon-search.md",
        ".dx/forge/docs/auth-better-auth.md",
        ".dx/forge/docs/supabase-client.md",
        ".dx/forge/docs/db-drizzle-sqlite.md",
        ".dx/forge/docs/migration-static-site.md",
        ".dx/serializer/dx.machine",
        "migrations/static-site/content.ts",
        "migrations/static-site/page.tsx",
        "migrations/static-site/sample-wordpress-export.json",
        "migrations/static-site/README.md",
    ] {
        assert!(dir.path().join(path).exists(), "{path}");
    }
    let manifest: DxSourceManifest = serde_json::from_slice(
        &fs::read(dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json")).expect("manifest bytes"),
    )
    .expect("manifest json");
    let package_ids = manifest
        .packages
        .iter()
        .map(|package| package.package_id.as_str())
        .collect::<HashSet<_>>();
    for package_id in FORGE_WWW_TEMPLATE_PACKAGE_IDS {
        assert!(package_ids.contains(package_id), "{package_id}");
    }
    let check = check_dx_project(dir.path()).expect("dx check");
    assert!(check.score >= 90, "check score should be beta-ready");
    assert!(
        forge_launch_gate_findings(&check).is_empty(),
        "init app should pass strict Forge launch gate"
    );
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_package_gallery_reports_curated_launch_package_boundaries() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "init-app".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--quiet".to_string(),
    ])
    .expect("init app");

    let output_path = dir.path().join("package-gallery.json");
    cli.cmd_forge(&[
        "package-gallery".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("package gallery");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["no_node_modules"], true);
    assert!(report["score"].as_u64().expect("score") >= 90);
    assert_eq!(
        report["package_count"].as_u64().expect("package count"),
        FORGE_WWW_TEMPLATE_PACKAGE_IDS.len() as u64
    );
    let packages = report["packages"].as_array().expect("packages");
    for package_id in FORGE_WWW_TEMPLATE_PACKAGE_IDS {
        let package = packages
            .iter()
            .find(|package| package["package_id"] == package_id)
            .unwrap_or_else(|| panic!("missing gallery package {package_id}"));
        assert!(
            package["ownership_boundary"]
                .as_str()
                .expect("ownership boundary")
                .contains("editable local source")
        );
        assert!(
            package["file_map"].as_array().expect("file map").len() > 0,
            "{package_id} file map"
        );
        assert_eq!(
            package["advisory"]["provider"],
            "dx-forge-curated-advisory-fixture"
        );
        assert_eq!(package["advisory"]["placeholder_present"], true);
        assert_eq!(package["advisory"]["live_coverage"], false);
        assert_eq!(package["docs_status"]["passed"], true);
        assert_eq!(package["scorecard_status"]["passed"], true);
        assert!(package["update_status"]["traffic"].is_string());
        assert!(package["rollback_status"]["traffic"].is_string());
    }
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_package_gallery_markdown_covers_review_surface() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "init-app".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--quiet".to_string(),
    ])
    .expect("init app");

    let output_path = dir.path().join("package-gallery.md");
    cli.cmd_forge(&[
        "package-gallery".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "markdown".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("package gallery markdown");

    let markdown = fs::read_to_string(output_path).expect("gallery markdown");
    assert!(markdown.contains("# DX Forge Source-Owned Package Gallery"));
    assert!(markdown.contains("shadcn/ui/button"));
    assert!(markdown.contains("shadcn/ui/card"));
    assert!(markdown.contains("shadcn/ui/input"));
    assert!(markdown.contains("shadcn/ui/textarea"));
    assert!(markdown.contains("dx/icon/search"));
    assert!(markdown.contains("auth/better-auth"));
    assert!(markdown.contains("auth/better-auth"));
    assert!(markdown.contains("supabase/client"));
    assert!(markdown.contains("db/drizzle-sqlite"));
    assert!(markdown.contains("migration/static-site"));
    assert!(markdown.contains("File Map"));
    assert!(markdown.contains("Advisory Placeholders"));
    assert!(markdown.contains("Update And Rollback"));
    assert!(markdown.contains("editable local source"));
    assert!(markdown.contains("no `node_modules`"));
    assert!(markdown.contains("dx-forge-curated-advisory-fixture"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_package_gallery_distinguishes_offline_advisory_coverage_from_placeholder_review() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "init-app".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--quiet".to_string(),
    ])
    .expect("init app");
    fs::write(
        dir.path().join(".dx/forge/advisories.json"),
        r#"{
  "version": 1,
  "packages": [
    {
      "package_id": "ui/button",
      "provider": "dx-forge-offline-osv-snapshot",
      "finding_count": 1,
      "reviewed_at": "2026-05-18T00:00:00Z",
      "note": "Offline OSV snapshot records one known advisory for regression testing."
    }
  ]
}"#,
    )
    .expect("advisory metadata");

    let output_path = dir.path().join("package-gallery.json");
    cli.cmd_forge(&[
        "package-gallery".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("package gallery");

    let report = read_json_value(output_path);
    let packages = report["packages"].as_array().expect("packages");
    let button = packages
        .iter()
        .find(|package| package["package_id"] == "shadcn/ui/button")
        .expect("button package");
    assert_eq!(button["advisory"]["coverage_kind"], "offline-snapshot");
    assert_eq!(
        button["advisory"]["provider"],
        "dx-forge-offline-osv-snapshot"
    );
    assert_eq!(button["advisory"]["finding_count"], 1);
    assert_eq!(button["advisory"]["placeholder_present"], false);

    let icon = packages
        .iter()
        .find(|package| package["package_id"] == "dx/icon/search")
        .expect("icon package");
    assert_eq!(icon["advisory"]["coverage_kind"], "curated-fixture");
    assert_eq!(icon["advisory"]["placeholder_present"], true);
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_package_gallery_writes_hosted_public_index_artifacts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "init-app".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--quiet".to_string(),
    ])
    .expect("init app");

    let output_path = dir.path().join("package-gallery.json");
    cli.cmd_forge(&[
        "package-gallery".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--public-index".to_string(),
        "public".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("package gallery public index");

    let report = read_json_value(output_path.clone());
    assert_eq!(report["hosted_index"]["route"], "/forge/package-gallery/");
    assert_eq!(report["hosted_index"]["passed"], true);
    assert_eq!(report["hosted_index"]["artifact_count"], 6);

    for artifact in [
        "public/forge/package-gallery/index.html",
        "public/forge/package-gallery.json",
        "public/forge/package-gallery.md",
        "public/forge/migration-gallery/index.html",
        "public/forge/migration-gallery.json",
        "public/forge/migration-gallery.md",
    ] {
        assert!(
            dir.path().join(artifact).is_file(),
            "hosted gallery missing `{artifact}`"
        );
    }

    let html = fs::read_to_string(dir.path().join("public/forge/package-gallery/index.html"))
        .expect("hosted gallery html");
    for expected in [
        "DX Forge Package Gallery",
        "Trust signals",
        "Migration guides",
        "shadcn/ui/button",
        "shadcn/ui/card",
        "shadcn/ui/input",
        "shadcn/ui/textarea",
        "dx/icon/search",
        "auth/better-auth",
        "auth/better-auth",
        "supabase/client",
        "db/drizzle-sqlite",
        "migration/static-site",
        "dx-forge-curated-advisory-fixture",
        "not a universal npm replacement",
        "no node_modules",
    ] {
        assert!(
            html.contains(expected),
            "hosted gallery html missing `{expected}`"
        );
    }

    let markdown = fs::read_to_string(dir.path().join("public/forge/package-gallery.md"))
        .expect("hosted gallery markdown");
    assert!(markdown.contains("## Migration Guides"));
    assert!(markdown.contains("dx forge migration-guide --package ui/button"));
    assert!(markdown.contains("dx forge migration-guide --package ui/input"));
    assert!(markdown.contains("dx forge migration-guide --package ui/textarea"));

    let hosted_json = read_json_value(dir.path().join("public/forge/package-gallery.json"));
    assert_eq!(hosted_json["route"], "/forge/package-gallery/");
    assert_eq!(
        hosted_json["packages"]
            .as_array()
            .expect("hosted packages")
            .len(),
        FORGE_WWW_TEMPLATE_PACKAGE_IDS.len()
    );

    let report_text = fs::read_to_string(output_path).expect("package gallery report");
    for marker in FORGE_PUBLIC_SECRET_MARKERS {
        assert!(
            !report_text.contains(marker),
            "hosted package gallery leaked secret marker {marker}"
        );
    }
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_package_gallery_writes_hosted_static_migration_gallery_artifacts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "init-app".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--quiet".to_string(),
    ])
    .expect("init app");

    let output_path = dir.path().join("package-gallery.json");
    cli.cmd_forge(&[
        "package-gallery".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--public-index".to_string(),
        "public".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("package gallery public index");

    let report = read_json_value(output_path.clone());
    assert_eq!(
        report["hosted_index"]["migration_gallery"]["route"],
        "/forge/migration-gallery/"
    );
    assert_eq!(report["hosted_index"]["artifact_count"], 6);

    for artifact in [
        "public/forge/migration-gallery/index.html",
        "public/forge/migration-gallery.json",
        "public/forge/migration-gallery.md",
    ] {
        assert!(
            dir.path().join(artifact).is_file(),
            "hosted migration gallery missing `{artifact}`"
        );
    }

    let hosted_json = read_json_value(dir.path().join("public/forge/migration-gallery.json"));
    assert_eq!(hosted_json["route"], "/forge/migration-gallery/");
    assert_eq!(hosted_json["package_id"], "migration/static-site");
    assert_eq!(hosted_json["passed"], true);
    assert_eq!(hosted_json["no_node_modules"], true);
    assert!(
        hosted_json["supported_scope"]
            .as_array()
            .expect("supported scope")
            .iter()
            .any(|item| item
                .as_str()
                .is_some_and(|value| value.contains("static WordPress or HTML page")))
    );
    assert!(
        hosted_json["manual_gaps"]
            .as_array()
            .expect("manual gaps")
            .iter()
            .any(|item| item
                .as_str()
                .is_some_and(|value| value.contains("forms, comments, search")))
    );
    assert!(
        hosted_json["package_evidence"]
            .as_array()
            .expect("package evidence")
            .iter()
            .any(|item| item["name"] == "migration-static-site-manual-review")
    );
    assert!(
        hosted_json["payload_comparison_boundaries"]
            .as_array()
            .expect("payload boundaries")
            .iter()
            .any(|item| item
                .as_str()
                .is_some_and(|value| value.contains("scoped static migrated route")))
    );

    let html = fs::read_to_string(dir.path().join("public/forge/migration-gallery/index.html"))
        .expect("hosted migration gallery html");
    for expected in [
        "DX Forge Migration Gallery",
        "migration/static-site",
        "Supported scope",
        "Manual gaps",
        "Package evidence",
        "Payload comparison boundaries",
        "dx forge migration-audit",
        "not a full WordPress plugin or theme migration",
        "no node_modules",
    ] {
        assert!(
            html.contains(expected),
            "hosted migration gallery html missing `{expected}`"
        );
    }

    let markdown = fs::read_to_string(dir.path().join("public/forge/migration-gallery.md"))
        .expect("hosted migration gallery markdown");
    assert!(markdown.contains("# DX Forge Migration Gallery"));
    assert!(markdown.contains("## Payload Comparison Boundaries"));
    assert!(markdown.contains("migration-static-site-source-files"));

    let report_text = fs::read_to_string(output_path).expect("package gallery report");
    for marker in FORGE_PUBLIC_SECRET_MARKERS {
        assert!(
            !report_text.contains(marker),
            "hosted migration gallery leaked secret marker {marker}"
        );
    }
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_migrated_route_benchmark_compares_scoped_fixtures_without_framework_claims() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    let json_path = dir.path().join("migrated-route-benchmark.json");
    cli.cmd_forge(&[
        "migrated-route-benchmark".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        json_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("migrated route benchmark");

    let report = read_json_value(json_path);
    assert_eq!(report["passed"], true);
    assert!(report["score"].as_u64().expect("score") >= 90);
    assert_eq!(report["route"], "/migrated/hello-world");
    assert_eq!(report["scope"]["scoped_static_route_only"], true);
    assert_eq!(report["scope"]["not_full_framework_benchmark"], true);
    assert_eq!(report["scope"]["no_package_install"], true);
    assert_eq!(report["no_node_modules"], true);
    assert_eq!(report["package_installs_run"], false);

    let fixtures = report["fixtures"].as_array().expect("fixtures");
    for fixture_id in [
        "dx-static-migrated",
        "wordpress-style-static",
        "nextjs-style-static",
    ] {
        assert!(
            fixtures.iter().any(|fixture| fixture["id"] == fixture_id),
            "missing fixture `{fixture_id}`"
        );
    }
    let dx_fixture = fixtures
        .iter()
        .find(|fixture| fixture["id"] == "dx-static-migrated")
        .expect("dx fixture");
    let wordpress_fixture = fixtures
        .iter()
        .find(|fixture| fixture["id"] == "wordpress-style-static")
        .expect("wordpress fixture");
    let next_fixture = fixtures
        .iter()
        .find(|fixture| fixture["id"] == "nextjs-style-static")
        .expect("next fixture");
    assert!(dx_fixture["decoded_bytes"].as_u64().expect("dx decoded") > 0);
    assert!(
        dx_fixture["decoded_bytes"].as_u64().expect("dx decoded")
            < wordpress_fixture["decoded_bytes"]
                .as_u64()
                .expect("wordpress decoded")
    );
    assert!(
        dx_fixture["brotli_bytes"].as_u64().expect("dx brotli")
            < next_fixture["brotli_bytes"].as_u64().expect("next brotli")
    );
    assert!(
        report["claim_boundaries"]
            .as_array()
            .expect("claim boundaries")
            .iter()
            .any(|boundary| boundary
                .as_str()
                .is_some_and(|text| text.contains("not a broad framework replacement claim")))
    );

    let markdown_path = dir.path().join("migrated-route-benchmark.md");
    cli.cmd_forge(&[
        "migrated-route-benchmark".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "markdown".to_string(),
        "--output".to_string(),
        markdown_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("migrated route benchmark markdown");

    let markdown = fs::read_to_string(markdown_path).expect("benchmark markdown");
    for expected in [
        "Scoped Static Migrated Route Benchmark",
        "WordPress-style baseline",
        "Next.js-style baseline",
        "not a broad framework replacement claim",
        "no `node_modules`",
    ] {
        assert!(
            markdown.contains(expected),
            "benchmark markdown missing `{expected}`"
        );
    }
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_migration_workflow_builders_are_split_from_cli_mod() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cli_mod = fs::read_to_string(manifest_dir.join("src/cli/mod.rs")).expect("cli mod");
    let production_cli_mod = cli_mod.split("#[cfg(test)]").next().unwrap_or(&cli_mod);
    let workflow_module = manifest_dir.join("src/cli/forge_migration_workflow.rs");

    assert!(
        workflow_module.is_file(),
        "migration workflow builders should live in a focused CLI module"
    );
    for builder in [
        "fn build_forge_migration_guide_report",
        "fn build_forge_package_gallery_report",
        "fn write_forge_package_gallery_hosted_index",
        "fn forge_migration_guide_markdown",
        "fn forge_package_gallery_markdown",
    ] {
        assert!(
            !production_cli_mod.contains(builder),
            "`{builder}` should not live in cli/mod.rs"
        );
    }
}

#[test]
fn forge_provenance_joins_manifest_receipts_licenses_updates_and_rollback() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "init-app".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--quiet".to_string(),
    ])
    .expect("init app");
    cli.cmd_forge(&[
        "update".to_string(),
        "ui/button".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
    ])
    .expect("forge update");

    let output_path = dir.path().join("forge-provenance.json");
    cli.cmd_forge(&[
        "provenance".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("forge provenance");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["no_node_modules"], true);
    assert!(report["score"].as_u64().expect("score") >= 90);
    assert_eq!(report["checks"]["registry_manifest"]["passed"], true);
    assert_eq!(report["checks"]["receipt_hashes"]["passed"], true);
    assert_eq!(report["checks"]["license_metadata"]["passed"], true);
    assert_eq!(report["checks"]["accepted_update_receipts"]["passed"], true);
    assert_eq!(report["checks"]["rollback_coverage"]["passed"], true);
    assert!(
        report["receipt_hash_count"]
            .as_u64()
            .expect("receipt hash count")
            >= 4
    );

    let packages = report["packages"].as_array().expect("packages");
    for package_id in FORGE_WWW_TEMPLATE_PACKAGE_IDS {
        let package = packages
            .iter()
            .find(|package| package["package_id"] == package_id)
            .unwrap_or_else(|| panic!("missing provenance package {package_id}"));
        assert_eq!(package["registry_manifest_present"], true);
        assert!(package["receipt_count"].as_u64().expect("receipt count") >= 1);
        assert!(
            package["receipt_hashes"]
                .as_array()
                .expect("receipt hashes")
                .iter()
                .all(|receipt| receipt["hash"]
                    .as_str()
                    .is_some_and(|hash| hash.len() == 64))
        );
        assert_eq!(
            package["license"]["declared"],
            package["registry_license"]["declared"]
        );
    }

    let button = packages
        .iter()
        .find(|package| package["package_id"] == "shadcn/ui/button")
        .expect("button package");
    assert_eq!(button["accepted_update_receipt_present"], true);
    assert_eq!(button["rollback_covered"], true);
    assert!(
        button["last_accepted_update"]
            .as_str()
            .is_some_and(|timestamp| timestamp.contains('T'))
    );
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_provenance_markdown_is_reviewable_and_secret_free() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "init-app".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--quiet".to_string(),
    ])
    .expect("init app");
    cli.cmd_forge(&[
        "update".to_string(),
        "ui/button".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
    ])
    .expect("forge update");

    let output_path = dir.path().join("forge-provenance.md");
    cli.cmd_forge(&[
        "provenance".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "markdown".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge provenance markdown");

    let markdown = fs::read_to_string(output_path).expect("provenance markdown");
    assert!(markdown.contains("# DX Forge Source-Owned Package Provenance"));
    assert!(markdown.contains("Registry Manifest"));
    assert!(markdown.contains("Receipt Hashes"));
    assert!(markdown.contains("Accepted Updates"));
    assert!(markdown.contains("License Metadata"));
    assert!(markdown.contains("Rollback Coverage"));
    assert!(markdown.contains("shadcn/ui/button"));
    assert!(markdown.contains("dx/icon/search"));
    assert!(markdown.contains("auth/better-auth"));
    assert!(markdown.contains("source-.dx/build-cache/manifest.json"));
    assert!(markdown.contains("no `node_modules`"));
    for marker in FORGE_PUBLIC_SECRET_MARKERS {
        assert!(
            !markdown.contains(marker),
            "provenance report leaked secret marker {marker}"
        );
    }
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_trust_regression_fixture_catches_package_trust_drift() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("forge-trust-regression.json");

    cli.cmd_forge(&[
        "trust-regression".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "100".to_string(),
        "--quiet".to_string(),
    ])
    .expect("forge trust regression");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["score"], 100);
    assert_eq!(report["no_node_modules"], true);
    assert_eq!(report["case_count"], 6);

    let cases = report["cases"].as_array().expect("cases");
    let baseline = trust_regression_case(cases, "green-baseline");
    assert_eq!(baseline["expected_traffic"], "green");
    assert_eq!(baseline["observed_traffic"], "green");
    assert_eq!(baseline["passed"], true);

    let local_edit = trust_regression_case(cases, "yellow-local-edit");
    assert_eq!(local_edit["expected_traffic"], "yellow");
    assert_eq!(local_edit["observed_traffic"], "yellow");
    assert!(
        local_edit["evidence"]
            .as_array()
            .expect("local edit evidence")
            .iter()
            .any(|value| value.as_str().is_some_and(|text| text.contains("update")))
    );

    let provenance = trust_regression_case(cases, "red-provenance-mismatch");
    assert_eq!(provenance["expected_traffic"], "red");
    assert_eq!(provenance["observed_traffic"], "red");
    assert!(
        provenance["evidence"]
            .as_array()
            .expect("provenance evidence")
            .iter()
            .any(|value| value
                .as_str()
                .is_some_and(|text| text.contains("provenance_metadata")))
    );

    let license = trust_regression_case(cases, "red-license-mismatch");
    assert_eq!(license["expected_traffic"], "red");
    assert_eq!(license["observed_traffic"], "red");
    assert!(
        license["evidence"]
            .as_array()
            .expect("license evidence")
            .iter()
            .any(|value| value
                .as_str()
                .is_some_and(|text| text.contains("license_metadata")))
    );

    let receipt = trust_regression_case(cases, "red-receipt-missing");
    assert_eq!(receipt["expected_traffic"], "red");
    assert_eq!(receipt["observed_traffic"], "red");
    assert!(
        receipt["evidence"]
            .as_array()
            .expect("receipt evidence")
            .iter()
            .any(|value| value
                .as_str()
                .is_some_and(|text| text.contains("receipt_hashes")))
    );

    let advisory = trust_regression_case(cases, "yellow-offline-advisory");
    assert_eq!(advisory["expected_traffic"], "yellow");
    assert_eq!(advisory["observed_traffic"], "yellow");
    assert!(
        advisory["evidence"]
            .as_array()
            .expect("advisory evidence")
            .iter()
            .any(|value| value
                .as_str()
                .is_some_and(|text| text.contains("offline-snapshot")))
    );

    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_migration_guide_maps_ui_button_to_source_owned_flow() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["ui/button", "--write"])
        .expect("dx add ui/button");
    let output_path = dir.path().join("ui-button-migration.json");

    cli.cmd_forge(&[
        "migration-guide".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--package".to_string(),
        "ui/button".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("migration guide");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["package_id"], "ui/button");
    assert_eq!(report["checks"]["materialized"]["passed"], true);
    assert_eq!(report["checks"]["docs"]["passed"], true);
    assert_eq!(report["checks"]["receipts"]["passed"], true);
    assert_eq!(report["checks"]["verify_package"]["passed"], true);
    assert_eq!(report["checks"]["no_node_modules"]["passed"], true);
    assert!(
        report["upstream_command"]
            .as_str()
            .expect("upstream command")
            .contains("shadcn@latest add button")
    );
    assert_eq!(
        report["forge_commands"]["write"],
        "dx add ui/button --write"
    );
    assert!(
        report["file_map"]
            .as_array()
            .expect("file map")
            .iter()
            .any(|file| file["materialized_path"] == "components/ui/button.tsx")
    );
    assert!(
        report["expectation_map"]
            .as_array()
            .expect("expectation map")
            .iter()
            .any(|item| item["upstream_expectation"]
                .as_str()
                .is_some_and(|value| value.contains("Component source files")))
    );
    assert!(
        report["ownership_boundaries"]
            .as_array()
            .expect("ownership boundaries")
            .iter()
            .any(|item| item
                .as_str()
                .is_some_and(|value| value.contains("app owns the copied files")))
    );
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_migration_guide_maps_ui_card_to_source_owned_flow() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["ui/card", "--write"])
        .expect("dx add ui/card");
    let output_path = dir.path().join("ui-card-migration.json");

    cli.cmd_forge(&[
        "migration-guide".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--package".to_string(),
        "ui/card".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("card migration guide");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["package_id"], "ui/card");
    assert_eq!(report["upstream_command"], "npx shadcn@latest add card");
    assert_eq!(report["forge_commands"]["write"], "dx add ui/card --write");
    assert!(
        report["file_map"]
            .as_array()
            .expect("file map")
            .iter()
            .any(|file| file["materialized_path"] == "components/ui/card.tsx")
    );
    assert!(
        report["next_commands"]
            .as_array()
            .expect("next commands")
            .iter()
            .any(|command| command
                .as_str()
                .is_some_and(|value| value == "dx update ui/card --dry-run"))
    );
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_migration_guide_markdown_covers_ui_components_developer_path() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["ui/button", "--write"])
        .expect("dx add ui/button");
    let output_path = dir.path().join("ui-components-migration.md");

    cli.cmd_forge(&[
        "migration-guide".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "markdown".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("migration guide markdown");

    let markdown = fs::read_to_string(output_path).expect("migration markdown");
    assert!(markdown.contains("# DX Forge UI Components Migration Guide"));
    assert!(markdown.contains("npx shadcn@latest add button"));
    assert!(markdown.contains("dx add ui/button --write"));
    assert!(markdown.contains("Receipts And Manifest"));
    assert!(markdown.contains("Local Ownership"));
    assert!(markdown.contains("Update And Rollback"));
    assert!(markdown.contains("components/ui/button.tsx"));
    assert!(markdown.contains("No `node_modules`"));
    assert!(markdown.contains("not a universal npm replacement"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_beta_diagnostics_reports_local_versions_durations_and_skips_without_secrets() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("beta-diagnostics.json");

    cli.cmd_forge(&[
        "beta-diagnostics".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("beta diagnostics");

    let report_text = fs::read_to_string(&output_path).expect("diagnostics json");
    for marker in FORGE_PUBLIC_SECRET_MARKERS {
        assert!(
            !report_text.contains(marker),
            "diagnostics report leaked secret marker {marker}"
        );
    }
    let report: serde_json::Value =
        serde_json::from_str(&report_text).expect("diagnostics json parse");
    assert_eq!(report["passed"], true);
    assert_eq!(report["telemetry_free"], true);
    assert_eq!(report["secret_policy"]["env_values_collected"], false);
    assert_eq!(report["secret_policy"]["serialized_environment_keys"], 0);
    assert!(
        report["tool_versions"]
            .as_array()
            .expect("tool versions")
            .iter()
            .any(|tool| tool["name"] == "cargo" && tool["duration_ms"].as_u64().is_some())
    );
    assert!(
        report["command_durations"]
            .as_array()
            .expect("command durations")
            .iter()
            .any(|duration| duration["label"] == "cargo --version")
    );
    assert!(report["cargo_cache"]["total_observed_bytes"].is_u64());
    assert!(
        report["skipped_optional_checks"]
            .as_array()
            .expect("skipped checks")
            .iter()
            .any(|check| check["name"] == "browser-timing")
    );
    assert!(
        report["privacy_notes"]
            .as_array()
            .expect("privacy notes")
            .iter()
            .any(|note| note
                .as_str()
                .is_some_and(|note| note.contains("No environment variable values")))
    );
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_beta_diagnostics_markdown_is_reviewable_and_secret_free() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("beta-diagnostics.md");

    cli.cmd_forge(&[
        "beta-diagnostics".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "markdown".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("beta diagnostics markdown");

    let markdown = fs::read_to_string(&output_path).expect("diagnostics markdown");
    assert!(markdown.contains("# DX Forge Beta Diagnostics"));
    assert!(markdown.contains("Tool Versions"));
    assert!(markdown.contains("Cargo Cache Use"));
    assert!(markdown.contains("Command Durations"));
    assert!(markdown.contains("Skipped Optional Checks"));
    assert!(markdown.contains("No environment variable values"));
    assert!(markdown.contains("browser-timing"));
    for marker in FORGE_PUBLIC_SECRET_MARKERS {
        assert!(
            !markdown.contains(marker),
            "diagnostics markdown leaked secret marker {marker}"
        );
    }
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_smoke_command_runs_launch_path() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("forge-smoke.json");

    cli.cmd_forge(&[
        "smoke".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--ci".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
    ])
    .expect("forge smoke");

    let report = fs::read_to_string(output_path).expect("smoke report");
    assert!(report.trim_start().starts_with('{'));
    assert!(report.contains("\"passed\": true"));
    assert!(report.contains("\"no_node_modules\": true"));
    assert!(report.contains("\"shadcn/ui/button\""));
    assert!(report.contains("\"dx/icon/search\""));
    assert!(report.contains("\"auth/better-auth\""));
    assert!(report.contains("\"evidence_report_path\""));
    assert!(report.contains("\"scorecard_report_path\""));
    assert!(report.contains("\"launch_summary_path\""));
    assert!(report.contains("\"launch_claims_path\""));
    assert!(report.contains("\"launch_evidence_model_path\""));
    assert!(report.contains("\"launch_page_quality\""));
    assert!(report.contains("\"claims_manifest\""));
    assert!(report.contains("\"headings\""));

    let report_json: serde_json::Value = serde_json::from_str(&report).expect("smoke report json");
    assert!(report_json["score"].as_u64().expect("score") >= 90);
    let launch_page_quality = &report_json["launch_page_quality"];
    assert_eq!(
        launch_page_quality["passed"].as_bool(),
        Some(true),
        "launch page quality should pass: {launch_page_quality:#?}"
    );
    assert!(
        launch_page_quality["score"]
            .as_u64()
            .expect("quality score")
            >= 90,
        "launch page quality score should stay release-ready"
    );
    assert_eq!(
        launch_page_quality["claims_manifest"]["passed"].as_bool(),
        Some(true)
    );
    assert_eq!(
        launch_page_quality["headings"]["passed"].as_bool(),
        Some(true)
    );
    let evidence_report_path = report_json["evidence_report_path"]
        .as_str()
        .expect("evidence path");
    let scorecard_report_path = report_json["scorecard_report_path"]
        .as_str()
        .expect("scorecard path");
    let launch_summary_path = report_json["launch_summary_path"]
        .as_str()
        .expect("launch summary path");
    let launch_html_path = report_json["launch_html_path"]
        .as_str()
        .expect("launch html path");
    let launch_claims_path = report_json["launch_claims_path"]
        .as_str()
        .expect("launch claims path");
    let launch_evidence_model_path = report_json["launch_evidence_model_path"]
        .as_str()
        .expect("launch evidence model path");

    assert!(std::path::Path::new(evidence_report_path).exists());
    assert!(std::path::Path::new(scorecard_report_path).exists());
    assert!(std::path::Path::new(launch_summary_path).exists());
    assert!(std::path::Path::new(launch_html_path).exists());
    assert!(std::path::Path::new(launch_claims_path).exists());
    assert!(std::path::Path::new(launch_evidence_model_path).exists());
    assert!(
        fs::read_to_string(evidence_report_path)
            .expect("evidence report")
            .contains("\"passed\": true")
    );
    let scorecard_report = fs::read_to_string(scorecard_report_path).expect("scorecard report");
    assert!(scorecard_report.contains("\"score\": 100"));
    assert!(scorecard_report.contains("\"latest_forge_route_benchmark\""));
    assert!(scorecard_report.contains("\"fixture_mode\": \"forge-site\""));
    assert!(dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json").exists());
    assert!(dir.path().join("public/forge.html").exists());
    assert!(dir.path().join("public/forge.claims.json").exists());
    assert!(dir.path().join("public/forge.evidence.json").exists());
    assert!(dir.path().join("public/proof.json").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_adoption_smoke_generates_real_project_public_routes() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("forge-adoption-smoke.json");

    cli.cmd_forge(&[
        "adoption-smoke".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--fail-under".to_string(),
        "90".to_string(),
    ])
    .expect("forge adoption smoke");

    let report_text = fs::read_to_string(&output_path).expect("adoption smoke report");
    let report: serde_json::Value =
        serde_json::from_str(&report_text).expect("adoption smoke json");

    assert_eq!(report["passed"], true);
    assert_eq!(report["no_node_modules"], true);
    assert_eq!(report["route_count"], 6);
    assert!(report["score"].as_u64().expect("score") >= 90);
    assert!(dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json").exists());
    assert!(
        dir.path()
            .join(".dx/forge/adoption-smoke/forge-smoke.json")
            .exists()
    );
    assert!(
        dir.path()
            .join(".dx/forge/adoption-smoke/release-bundle/forge-release-.dx/build-cache/manifest.json")
            .exists()
    );
    let example_route_path = dir.path().join("pages/forge-adoption.html");
    let example_route =
        fs::read_to_string(&example_route_path).expect("forge adoption example route");
    for required in [
        "data-forge-package=\"shadcn/ui/button\"",
        "data-forge-package=\"shadcn/ui/card\"",
        "data-forge-package=\"dx/icon/search\"",
        "data-forge-package=\"auth/better-auth\"",
        "data-forge-package=\"auth/better-auth\"",
        "data-forge-package=\"animation/motion\"",
        "data-forge-package=\"migration/static-site\"",
        "<Button",
        "<Card",
        "<SearchIcon",
        "/auth/better-auth/route",
    ] {
        assert!(
            example_route.contains(required),
            "adoption example route is missing `{required}`"
        );
    }
    for docs in [
        ".dx/forge/docs/shadcn-ui-button.md",
        ".dx/forge/docs/shadcn-ui-card.md",
        ".dx/forge/docs/dx-icon-search.md",
        ".dx/forge/docs/auth-better-auth.md",
        ".dx/forge/docs/animation-motion.md",
        ".dx/forge/docs/migration-static-site.md",
    ] {
        assert!(dir.path().join(docs).exists(), "{docs}");
    }
    cli.cmd_check(&[
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
    ])
    .expect("dx check for adoption example app");

    for route in [
        "forge.html",
        "forge/scorecard.html",
        "forge/ci.html",
        "forge/evidence.html",
        "forge/releases.html",
        "forge/changelog.html",
    ] {
        assert!(dir.path().join("public").join(route).exists(), "{route}");
    }

    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_adoption_report_summarizes_existing_project_evidence() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "adoption-smoke".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--quiet".to_string(),
        "--fail-under".to_string(),
        "90".to_string(),
    ])
    .expect("forge adoption smoke");

    let output_path = dir.path().join("forge-adoption-report.json");
    cli.cmd_forge(&[
        "adoption-report".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--fail-under".to_string(),
        "90".to_string(),
    ])
    .expect("forge adoption report");

    let report_text = fs::read_to_string(&output_path).expect("adoption report json");
    let report: serde_json::Value =
        serde_json::from_str(&report_text).expect("adoption report parse");

    assert_eq!(report["passed"], true);
    assert_eq!(report["no_node_modules"], true);
    assert!(
        report["package_count"].as_u64().expect("package count")
            >= FORGE_WWW_TEMPLATE_PACKAGE_IDS.len() as u64
    );
    let package_ids = report["packages"]
        .as_array()
        .expect("packages")
        .iter()
        .filter_map(|package| package["package_id"].as_str())
        .collect::<HashSet<_>>();
    for package_id in FORGE_WWW_TEMPLATE_PACKAGE_IDS {
        assert!(
            package_ids.contains(package_id),
            "adoption report missing package {package_id}"
        );
    }
    assert!(
        report["receipt_count"].as_u64().expect("receipt count")
            >= FORGE_WWW_TEMPLATE_PACKAGE_IDS.len() as u64,
        "expected package receipts in adoption report"
    );
    assert!(
        report["package_docs_present"].as_u64().expect("docs")
            >= FORGE_WWW_TEMPLATE_PACKAGE_IDS.len() as u64
    );
    assert_eq!(report["public_routes"].as_array().expect("routes").len(), 6);
    assert_eq!(report["release_bundle"]["passed"], true);
    assert!(report["score"].as_u64().expect("score") >= 90);
    assert!(
        report["project_structure"]["app_route_path"]
            .as_str()
            .expect("app route path")
            .ends_with("pages/forge-adoption.html")
    );
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_beta_install_bootstraps_clean_project_from_release_bundle_with_trust_artifacts() {
    let release_project = tempdir().expect("release project tempdir");
    let release_cli = Cli::with_cwd(release_project.path().to_path_buf());
    let dashboard_path = write_passing_release_dashboard(release_project.path());
    let route_comparison_path =
        write_passing_public_route_comparison_with_adoption(release_project.path());
    let release_history_path = release_project
        .path()
        .join("benchmarks/reports/forge-public-release-history.json");
    let release_bundle_dir = release_project.path().join("release-bundle-adoption");

    release_cli
        .cmd_forge(&[
            "release-history".to_string(),
            "--dashboard".to_string(),
            dashboard_path.to_string_lossy().into_owned(),
            "--route-comparison".to_string(),
            route_comparison_path.to_string_lossy().into_owned(),
            "--output".to_string(),
            release_history_path.to_string_lossy().into_owned(),
            "--quiet".to_string(),
        ])
        .expect("release history fixture");
    release_cli
        .cmd_forge(&[
            "release-bundle".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--out".to_string(),
            release_bundle_dir.to_string_lossy().into_owned(),
            "--include-adoption".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--quiet".to_string(),
        ])
        .expect("adoption release bundle");

    let install_project = tempdir().expect("install project tempdir");
    let install_cli = Cli::with_cwd(install_project.path().to_path_buf());
    let output_path = install_project.path().join("beta-install.json");

    install_cli
        .cmd_forge(&[
            "beta-install".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--release-bundle".to_string(),
            release_bundle_dir.to_string_lossy().into_owned(),
            "--artifacts".to_string(),
            ".dx/forge/beta-install".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--output".to_string(),
            output_path.to_string_lossy().into_owned(),
            "--fail-under".to_string(),
            "90".to_string(),
            "--quiet".to_string(),
        ])
        .expect("forge beta install");

    let report = read_json_value(output_path.clone());
    assert_eq!(report["passed"], true);
    assert_eq!(report["mode"], "write");
    assert_eq!(report["release_bundle"]["passed"], true);
    assert_eq!(report["init_app"]["passed"], true);
    assert_eq!(report["provenance"]["passed"], true);
    assert_eq!(report["trust_regression"]["passed"], true);
    assert_eq!(report["adoption_report"]["passed"], true);
    assert_eq!(report["no_node_modules"], true);
    assert!(
        report["routes"]
            .as_array()
            .expect("routes")
            .iter()
            .any(|route| route["route"] == "/forge/adoption" && route["exists"] == true)
    );

    for artifact in [
        ".dx/forge/beta-install/forge-beta-install.ps1",
        ".dx/forge/beta-install/forge-provenance.json",
        ".dx/forge/beta-install/forge-provenance.md",
        ".dx/forge/beta-install/forge-trust-regression.json",
        ".dx/forge/beta-install/forge-trust-regression.md",
        ".dx/forge/beta-install/forge-adoption-report.json",
        ".dx/forge/beta-install/forge-adoption-report.md",
        ".dx/forge/beta-install/forge-release-bundle.json",
        ".dx/forge/beta-install/forge-beta-install.json",
        "public/forge/adoption.html",
        ".dx/forge/source-.dx/build-cache/manifest.json",
    ] {
        assert!(
            install_project.path().join(artifact).is_file(),
            "beta install missing `{artifact}`"
        );
    }

    let report_text = fs::read_to_string(output_path).expect("beta install report");
    for marker in FORGE_PUBLIC_SECRET_MARKERS {
        assert!(
            !report_text.contains(marker),
            "beta install leaked secret marker {marker}"
        );
    }
    assert!(!install_project.path().join("node_modules").exists());
    assert!(!release_bundle_dir.join("node_modules").exists());
}

#[test]
fn forge_beta_upgrade_smoke_preserves_local_source_owned_edits_between_signed_bundles() {
    let first_release = tempdir().expect("first release project tempdir");
    let first_bundle =
        write_signed_adoption_release_bundle_fixture(first_release.path(), "release-bundle-v1");
    let second_release = tempdir().expect("second release project tempdir");
    let second_bundle =
        write_signed_adoption_release_bundle_fixture(second_release.path(), "release-bundle-v2");
    let install_project = tempdir().expect("install project tempdir");
    let install_cli = Cli::with_cwd(install_project.path().to_path_buf());
    let output_path = install_project.path().join("beta-upgrade-smoke.json");

    install_cli
        .cmd_forge(&[
            "beta-upgrade-smoke".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--from-release-bundle".to_string(),
            first_bundle.to_string_lossy().into_owned(),
            "--to-release-bundle".to_string(),
            second_bundle.to_string_lossy().into_owned(),
            "--artifacts".to_string(),
            ".dx/forge/beta-upgrade-smoke".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--output".to_string(),
            output_path.to_string_lossy().into_owned(),
            "--fail-under".to_string(),
            "90".to_string(),
            "--quiet".to_string(),
        ])
        .expect("forge beta upgrade smoke");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["mode"], "write");
    assert_eq!(report["from_release_manifest"]["passed"], true);
    assert_eq!(report["to_release_manifest"]["passed"], true);
    assert_eq!(report["initial_install"]["passed"], true);
    assert_eq!(report["reviewed_update"]["passed"], true);
    assert_eq!(report["provenance"]["passed"], true);
    assert_eq!(report["adoption_report"]["passed"], true);
    assert_eq!(report["local_edit"]["preserved"], true);
    assert_eq!(report["local_edit"]["path"], "components/ui/button.tsx");

    let button = fs::read_to_string(install_project.path().join("components/ui/button.tsx"))
        .expect("button source");
    assert!(button.contains("dx-forge-beta-upgrade-smoke-local-edit"));
    for artifact in [
        ".dx/forge/beta-upgrade-smoke/forge-beta-upgrade-smoke.json",
        ".dx/forge/beta-upgrade-smoke/forge-provenance.json",
        ".dx/forge/beta-upgrade-smoke/forge-adoption-report.json",
        ".dx/forge/beta-upgrade-smoke/forge-trust-regression.json",
        "public/forge/adoption.html",
    ] {
        assert!(
            install_project.path().join(artifact).is_file(),
            "beta upgrade smoke missing `{artifact}`"
        );
    }
    assert!(!install_project.path().join("node_modules").exists());
    assert!(!first_bundle.join("node_modules").exists());
    assert!(!second_bundle.join("node_modules").exists());
}

#[test]
fn forge_ci_command_writes_reviewable_artifacts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let artifact_dir = dir.path().join("ci-artifacts");

    cli.cmd_forge(&[
        "ci".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--out".to_string(),
        artifact_dir.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("forge ci");

    assert!(artifact_dir.join("forge-smoke.json").exists());
    assert!(artifact_dir.join("forge-smoke.md").exists());
    assert!(artifact_dir.join("forge-triage.md").exists());
    assert!(artifact_dir.join("forge-readiness-badge.json").exists());
    assert!(artifact_dir.join("forge-evidence.json").exists());
    assert!(artifact_dir.join("forge-scorecard.json").exists());
    assert!(artifact_dir.join("forge-benchmark-history.json").exists());
    assert!(artifact_dir.join("forge-adoption-smoke.json").exists());
    assert!(artifact_dir.join("forge-adoption-report.json").exists());
    assert!(artifact_dir.join("forge-adoption-report.md").exists());
    assert!(artifact_dir.join("forge-adoption-page.html").exists());
    assert!(artifact_dir.join("forge.html").exists());
    assert!(artifact_dir.join("forge.claims.json").exists());
    assert!(artifact_dir.join("forge.evidence.json").exists());
    assert!(artifact_dir.join("forge.dxp").exists());
    assert!(artifact_dir.join("forge-proof.json").exists());
    assert!(artifact_dir.join("forge/adoption.html").exists());
    assert!(artifact_dir.join("forge/adoption/index.html").exists());
    assert!(artifact_dir.join("forge/adoption.claims.json").exists());
    assert!(artifact_dir.join("forge/adoption.dxp").exists());
    assert!(artifact_dir.join("forge/adoption.proof.json").exists());

    let smoke = fs::read_to_string(artifact_dir.join("forge-smoke.json")).expect("smoke json");
    assert!(smoke.contains("\"passed\": true"));
    assert!(smoke.contains("\"launch_page_quality\""));
    let adoption_report = fs::read_to_string(artifact_dir.join("forge-adoption-report.json"))
        .expect("adoption report json");
    assert!(adoption_report.contains("\"passed\": true"));
    assert!(adoption_report.contains("\"no_node_modules\": true"));
    let adoption_route =
        fs::read_to_string(artifact_dir.join("forge/adoption.html")).expect("adoption html");
    assert!(adoption_route.contains("DX Forge adoption evidence"));
    let adoption_claims = fs::read_to_string(artifact_dir.join("forge/adoption.claims.json"))
        .expect("adoption claims");
    assert!(adoption_claims.contains("\"route\": \"/forge/adoption\""));
    let triage = fs::read_to_string(artifact_dir.join("forge-triage.md")).expect("triage markdown");
    assert!(triage.contains("# DX Forge Failure Triage"));
    assert!(triage.contains("## Launch Page Quality"));
    assert!(triage.contains("## Budget Gate"));
    let badge = fs::read_to_string(artifact_dir.join("forge-readiness-badge.json")).expect("badge");
    let badge_json: serde_json::Value = serde_json::from_str(&badge).expect("badge json");
    assert_eq!(badge_json["label"], "DX Forge");
    assert_eq!(badge_json["status"], "passing");
    assert!(
        badge_json["message"]
            .as_str()
            .expect("badge message")
            .starts_with("ready ")
    );
    assert_eq!(badge_json["smoke"]["passed"], true);
    assert_eq!(badge_json["evidence"]["passed"], true);
    assert_eq!(badge_json["scorecard"]["score"], 100);
    assert_eq!(
        badge_json["latest_forge_route_benchmark"]["fixture_mode"],
        "forge-site"
    );
    assert_eq!(
        badge_json["latest_forge_route_benchmark"]["route_delivery"],
        "static"
    );
    assert_eq!(badge_json["no_node_modules"], true);
    assert_eq!(
        artifact_names(&artifact_dir),
        BTreeSet::from([
            "forge".to_string(),
            "forge-adoption-page.html".to_string(),
            "forge-adoption-report.json".to_string(),
            "forge-adoption-report.md".to_string(),
            "forge-adoption-smoke.json".to_string(),
            "forge-benchmark-history.json".to_string(),
            "forge-readiness-badge.json".to_string(),
            "forge-scorecard.json".to_string(),
            "forge-smoke.json".to_string(),
            "forge-smoke.md".to_string(),
            "forge-triage.md".to_string(),
            "forge.claims.json".to_string(),
            "forge.dxp".to_string(),
            "forge.evidence.json".to_string(),
            "forge.html".to_string(),
            "forge-evidence.json".to_string(),
            "forge-page.html".to_string(),
            "forge-proof.json".to_string(),
        ])
    );
    for file_name in [
        "forge-smoke.json",
        "forge-smoke.md",
        "forge-triage.md",
        "forge-readiness-badge.json",
        "forge-adoption-report.json",
        "forge-adoption-report.md",
        "forge/adoption.html",
        "forge/adoption.claims.json",
    ] {
        let text = fs::read_to_string(artifact_dir.join(file_name)).expect("text artifact");
        assert!(!text.contains("CLOUDFLARE_R2_"));
        assert!(!text.contains("DX_FORGE_R2_LIVE"));
    }
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_ci_markdown_summary_links_artifacts_and_routes() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let artifact_dir = dir.path().join("ci-artifacts");

    cli.cmd_forge(&[
        "ci".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--out".to_string(),
        artifact_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "markdown".to_string(),
        "--fail-under".to_string(),
        "90".to_string(),
    ])
    .expect("forge ci markdown");

    let summary_project = tempdir().expect("summary tempdir");
    let report = build_forge_smoke_report(summary_project.path()).expect("forge smoke report");
    let artifacts = forge_ci_artifact_paths_from_dir(&artifact_dir).expect("ci artifact paths");
    let summary = forge_ci_summary_markdown(&report, &artifact_dir, &artifacts);

    assert!(summary.contains("# DX Forge CI Summary"));
    assert!(summary.contains("[`forge-smoke.json`](forge-smoke.json)"));
    assert!(summary.contains("[`forge-triage.md`](forge-triage.md)"));
    assert!(summary.contains("[`forge-readiness-badge.json`](forge-readiness-badge.json)"));
    assert!(summary.contains("[`forge-adoption-report.json`](forge-adoption-report.json)"));
    assert!(summary.contains("[`forge/adoption.html`](forge/adoption.html)"));
    assert!(summary.contains("| `/forge` | [`forge.html`](forge.html) |"));
    assert!(summary.contains("| `/forge` claims | [`forge.claims.json`](forge.claims.json) |"));
    assert!(summary.contains("| `/forge/ci` source model | [`forge-smoke.json`](forge-smoke.json), [`forge-readiness-badge.json`](forge-readiness-badge.json) |"));
    assert!(
        summary
            .contains("| `/forge/adoption` route | [`forge/adoption.html`](forge/adoption.html)")
    );
    assert!(!summary.contains("CLOUDFLARE_R2_"));
    assert!(!summary.contains("DX_FORGE_R2_LIVE"));
    assert!(!dir.path().join("node_modules").exists());
    assert!(!summary_project.path().join("node_modules").exists());
}

#[test]
fn forge_ci_verify_artifacts_reuses_existing_bundle_without_smoke_rerun() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let artifact_dir = dir.path().join("ci-artifacts");

    cli.cmd_forge(&[
        "ci".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--out".to_string(),
        artifact_dir.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge ci artifacts");

    let verify_dir = tempdir().expect("verify tempdir");
    let verify_cli = Cli::with_cwd(verify_dir.path().to_path_buf());
    verify_cli
        .cmd_forge(&[
            "ci".to_string(),
            "--verify-artifacts".to_string(),
            artifact_dir.to_string_lossy().into_owned(),
            "--format".to_string(),
            "markdown".to_string(),
            "--fail-under".to_string(),
            "90".to_string(),
        ])
        .expect("verify artifacts");

    assert!(!verify_dir.path().join("pages").exists());
    assert!(!verify_dir.path().join(".dx").exists());
    assert!(!verify_dir.path().join("node_modules").exists());
}

#[test]
fn forge_ci_verify_artifacts_fails_for_missing_required_bundle_file() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let artifact_dir = dir.path().join("ci-artifacts");

    cli.cmd_forge(&[
        "ci".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--out".to_string(),
        artifact_dir.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge ci artifacts");

    std::fs::remove_file(artifact_dir.join("forge-readiness-badge.json"))
        .expect("remove readiness badge");

    let verify_dir = tempdir().expect("verify tempdir");
    let verify_cli = Cli::with_cwd(verify_dir.path().to_path_buf());
    let error = verify_cli
        .cmd_forge(&[
            "ci".to_string(),
            "--verify-artifacts".to_string(),
            artifact_dir.to_string_lossy().into_owned(),
            "--no-fail-under".to_string(),
            "--quiet".to_string(),
        ])
        .expect_err("missing artifact fails");
    let message = format!("{error:?}");
    assert!(message.contains("forge-readiness-badge.json"));
    assert!(message.contains("missing"));
}

#[test]
fn forge_ci_verify_pages_bundle_checks_public_publish_shape() {
    let pages_project = tempdir().expect("pages project");
    let cli = Cli::with_cwd(pages_project.path().to_path_buf());
    let badge_project = tempdir().expect("badge project");
    let badge_cli = Cli::with_cwd(badge_project.path().to_path_buf());
    let ci_artifact_dir = badge_project.path().join("ci-artifacts");
    let pages_dir = pages_project.path().join("forge-pages");

    badge_cli
        .cmd_forge(&[
            "ci".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--out".to_string(),
            ci_artifact_dir.to_string_lossy().into_owned(),
            "--quiet".to_string(),
        ])
        .expect("forge ci artifacts");
    write_forge_pages_publish_bundle(&cli, pages_project.path(), &pages_dir, &ci_artifact_dir);

    for artifact in [
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
    ] {
        assert!(
            pages_dir.join(artifact).is_file(),
            "Pages publish bundle is missing artifact `{artifact}`"
        );
    }

    cli.cmd_forge(&[
        "ci".to_string(),
        "--verify-pages".to_string(),
        pages_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("verify pages bundle");

    fs::remove_file(pages_dir.join("forge/ci/index.html")).expect("remove clean route index");
    let error = cli
        .cmd_forge(&[
            "ci".to_string(),
            "--verify-pages".to_string(),
            pages_dir.to_string_lossy().into_owned(),
            "--no-fail-under".to_string(),
            "--quiet".to_string(),
        ])
        .expect_err("missing clean route should fail");
    let message = format!("{error:?}");
    assert!(message.contains("forge/ci/index.html"));
    assert!(!pages_project.path().join("node_modules").exists());
    assert!(!badge_project.path().join("node_modules").exists());
}

#[test]
fn forge_badge_command_writes_release_readiness_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let badge_path = dir.path().join("badge.json");

    cli.cmd_forge(&[
        "badge".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--output".to_string(),
        badge_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("forge badge");

    let badge = fs::read_to_string(&badge_path).expect("badge");
    let badge_json: serde_json::Value = serde_json::from_str(&badge).expect("badge json");

    assert_eq!(badge_json["schemaVersion"], 1);
    assert_eq!(badge_json["isError"], false);
    assert_eq!(badge_json["label"], "DX Forge");
    assert_eq!(badge_json["status"], "passing");
    assert!(badge_json["score"].as_u64().expect("score") >= 90);
    assert!(badge_json["smoke"]["score"].as_u64().expect("smoke score") >= 90);
    assert!(
        badge_json["evidence"]["score"]
            .as_u64()
            .expect("evidence score")
            >= 90
    );
    assert_eq!(badge_json["scorecard"]["score"], 100);
    assert_eq!(badge_json["launch_page_quality"]["passed"], true);
    assert_eq!(
        badge_json["latest_forge_route_benchmark"]["fixture_mode"],
        "forge-site"
    );
    assert_eq!(
        std::path::Path::new(
            badge_json["artifacts"]["release_evidence"]
                .as_str()
                .expect("release proof path")
        )
        .exists(),
        true
    );
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_launch_page_command_writes_public_route() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_forge(&[
        "launch-page".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
    ])
    .expect("forge launch page");

    let source = fs::read_to_string(dir.path().join("pages/forge.html")).expect("source");
    let html = fs::read_to_string(dir.path().join("public/forge.html")).expect("html");
    let claims = fs::read_to_string(dir.path().join("public/forge.claims.json")).expect("claims");
    let evidence =
        fs::read_to_string(dir.path().join("public/forge.evidence.json")).expect("evidence");
    let summary = fs::read_to_string(dir.path().join("public/proof.json")).expect("proof summary");

    assert!(source.contains("DX Forge launch evidence"));
    assert!(source.contains("forge.evidence.json"));
    assert!(html.contains("DX Forge launch evidence"));
    assert!(html.contains("forge.evidence.json"));
    assert!(claims.contains("\"route\": \"/forge\""));
    assert!(claims.contains("\"source_field\": \"package_scorecard.packages[].source_owned\""));
    assert!(claims.contains("\"verification_status\": \"verified\""));
    assert!(claims.contains("auth/better-auth"));
    assert!(evidence.contains("\"route\": \"/forge\""));
    assert!(evidence.contains("\"packages\""));
    assert!(evidence.contains("\"provenance_source\""));
    assert!(evidence.contains("auth/better-auth"));
    assert!(dir.path().join("public/forge.dxp").exists());
    assert!(!dir.path().join("public/forge.dxp.js").exists());
    assert!(dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json").exists());
    assert!(dir.path().join(".dx/forge/receipts").exists());
    assert!(summary.contains("\"route\": \"/forge\""));
    assert!(summary.contains("forge.html"));
    assert!(summary.contains("forge.dxp"));
    assert!(!summary.contains("forge.dxp.js"));
    assert!(summary.contains("forge.claims.json"));
    assert!(summary.contains("forge.evidence.json"));
    assert!(summary.contains("source-.dx/build-cache/manifest.json"));
    assert!(summary.contains("receipts"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_launch_page_dry_run_writes_nothing() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_forge(&[
        "launch-page".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--dry-run".to_string(),
        "--format".to_string(),
        "json".to_string(),
    ])
    .expect("forge launch page dry-run");

    assert!(!dir.path().join("pages/forge.html").exists());
    assert!(!dir.path().join("public/forge.html").exists());
    assert!(!dir.path().join("public/forge.claims.json").exists());
    assert!(!dir.path().join("public/forge.evidence.json").exists());
    assert!(!dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_public_markdown_snapshots_match_launch_copy() {
    let scorecard =
        forge_package_scorecard_markdown(&build_forge_package_scorecard().expect("scorecard"));
    assert_forge_markdown_snapshot("scorecard-public.md", &scorecard, None);

    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["ui/button", "--write"])
        .expect("dx add ui/button");
    cli.cmd_add(&["icon", "search", "--write"])
        .expect("dx add icon search");
    cli.cmd_add(&["auth/better-auth", "--write"])
        .expect("dx add auth/better-auth");
    cli.cmd_add(&["ui/card", "--write"])
        .expect("dx add ui/card");
    cli.cmd_add(&["migration/static-site", "--write"])
        .expect("dx add migration/static-site");
    let history_path = write_passing_forge_benchmark_history(dir.path());

    let evidence = forge_release_evidence_markdown(
        &build_forge_release_evidence_report(dir.path(), &history_path).expect("release proof"),
    );
    assert_forge_markdown_snapshot("release-proof-project.md", &evidence, Some(dir.path()));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_public_markdown_snapshots_cover_evidence_and_route_comparison() {
    let scorecard = build_forge_package_scorecard().expect("scorecard");
    let public_evidence =
        forge_public_evidence_markdown(&build_forge_public_evidence_report(&scorecard));
    assert_forge_markdown_snapshot("public-evidence.md", &public_evidence, None);

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .expect("repo root");
    let route_comparison_path = repo_root
        .join("benchmarks/reports")
        .join("forge-public-route-comparison.md");
    let route_comparison = fs::read_to_string(&route_comparison_path).unwrap_or_else(|error| {
        panic!(
            "read route comparison markdown {}: {error}",
            route_comparison_path.display()
        )
    });
    assert_forge_route_comparison_markdown_snapshot(
        "forge-public-route-comparison.md",
        &route_comparison,
    );
    let route_comparison_json_path = repo_root
        .join("benchmarks/reports")
        .join("forge-public-route-comparison.json");
    let route_comparison_json = read_json_value(route_comparison_json_path);
    assert_forge_json_snapshot(
        "public-route-comparison-shape.json",
        forge_public_route_comparison_shape_contract(&route_comparison_json),
    );
    let changelog_budget = read_forge_changelog_budget_evidence(repo_root, &route_comparison_json);
    assert_forge_json_snapshot(
        "forge-changelog-budget-shape.json",
        forge_changelog_budget_shape_contract(&changelog_budget),
    );

    let dir = tempdir().expect("release history snapshot project");
    let dashboard_path = write_passing_release_dashboard(dir.path());
    let route_comparison_path = write_passing_public_route_comparison(dir.path());
    let history_path = dir
        .path()
        .join("benchmarks/reports/forge-public-release-history.json");
    record_forge_public_release_history(DxForgePublicReleaseHistoryInput {
        root: dir.path().to_path_buf(),
        dashboard_path,
        route_comparison_path,
        history_path: history_path.clone(),
    })
    .expect("public release history");
    let release_history =
        fs::read_to_string(history_path.with_extension("md")).expect("release history md");
    assert_forge_release_history_markdown_snapshot(
        "forge-public-release-history.md",
        &release_history,
    );
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_adoption_snapshots_cover_report_and_route_contracts() {
    let dir = tempdir().expect("adoption snapshot project");
    let report = write_forge_adoption_contract_artifacts(dir.path());
    let report_json = serde_json::to_value(&report).expect("adoption report json");

    assert_forge_json_snapshot(
        "adoption-report-shape.json",
        forge_adoption_report_shape_contract(&report_json),
    );
    assert_forge_markdown_snapshot(
        "forge-adoption-report.md",
        &forge_adoption_report_markdown(&report),
        Some(dir.path()),
    );
    assert_forge_json_snapshot(
        "adoption-claims-shape.json",
        forge_adoption_claims_shape_contract(&read_json_value(
            dir.path().join("public/forge/adoption.claims.json"),
        )),
    );
    assert_forge_json_snapshot(
        "adoption-proof-shape.json",
        forge_adoption_proof_shape_contract(&read_json_value(dir.path().join("public/proof.json"))),
    );
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_public_launch_checklist_documents_operator_flow() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .expect("repo root");
    let checklist_path = repo_root.join("docs/forge-public-launch-checklist.md");
    let handoff_path = repo_root.join("docs/forge-public-launch-handoff.md");
    let checklist = fs::read_to_string(&checklist_path).unwrap_or_else(|error| {
        panic!(
            "read public launch checklist {}: {error}",
            checklist_path.display()
        )
    });
    let handoff = fs::read_to_string(&handoff_path)
        .unwrap_or_else(|error| panic!("read launch handoff {}: {error}", handoff_path.display()));

    for required in [
        "docs\\forge-public-launch-handoff.md",
        "scripts\\ci\\forge-ci.ps1 -ArtifactDir .\\.dx\\ci -PagesDir .\\.dx\\forge-pages -FailUnder 90",
        "dx-www -- forge ci --verify-artifacts .\\.dx\\ci --fail-under 90",
        "dx-www -- forge ci --verify-pages .\\.dx\\forge-pages --fail-under 90",
        "dx-www -- forge release-notes --project .",
        "dx-www -- forge public-evidence --project .",
        "dx-www -- forge release-dashboard --project .",
        "dx-www -- forge release-history --dashboard",
        "dx-www -- prove vertical --fixture forge-releases",
        "benchmarks\\measure-vertical-proof.ts",
        "benchmarks\\compare-forge-launch-delivery.ts",
        "benchmarks\\reports\\forge-public-route-comparison.json",
        "benchmarks\\reports\\forge-public-release-history.json",
        "benchmarks\\reports\\forge-public-release-history.md",
        "--include-adoption",
        "forge-readiness-badge.json",
        "forge-release-dashboard.md",
        "forge-public-release-history.json",
        "forge-public-release-history.md",
        "node_modules",
        "CLOUDFLARE_R2_",
    ] {
        assert!(
            checklist.contains(required),
            "public launch checklist is missing `{required}`"
        );
    }

    for required in [
        "forge release-bundle --verify .\\.dx\\forge-release-bundle",
        "forge-release-bundle-adoption --include-adoption",
        "forge release-review --project .",
        "forge-release-.dx/build-cache/manifest.json",
        "hash_algorithm = blake3",
        "dx-forge-release-manifest-v1",
        "forge-public-launch-changelog.json",
        "forge-public-route-comparison.json",
        ".dx\\forge-pages\\forge\\changelog\\index.html",
        "CLOUDFLARE_R2_",
        "node_modules",
        "Forge replaces npm, cargo, pip, or all package managers today.",
    ] {
        assert!(
            handoff.contains(required),
            "public launch handoff is missing `{required}`"
        );
    }
}

#[test]
fn forge_real_project_adoption_docs_cover_reproducible_path() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .expect("repo root");
    let docs_path = repo_root.join("docs/forge-real-project-adoption.md");
    let checklist_path = repo_root.join("docs/forge-public-launch-checklist.md");
    let docs = fs::read_to_string(&docs_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", docs_path.display()));
    let checklist = fs::read_to_string(&checklist_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", checklist_path.display()));

    for required in [
        "# DX Forge Real-Project Adoption Path",
        "cargo run --manifest-path .\\www\\Cargo.toml -p dx-www --bin dx-www -- forge adoption-smoke --project .\\.dx\\adoption-app --format markdown --output .\\.dx\\adoption-app\\.dx\\forge\\adoption-smoke\\adoption-smoke.md --fail-under 90",
        "cargo run --manifest-path .\\www\\Cargo.toml -p dx-www --bin dx-www -- check .\\.dx\\adoption-app --strict-forge --format markdown --fail-under 90",
        "cargo run --manifest-path .\\www\\Cargo.toml -p dx-www --bin dx-www -- forge release-bundle --project .\\.dx\\adoption-app --out .\\.dx\\adoption-app\\.dx\\forge\\adoption-smoke\\release-bundle --format markdown --fail-under 90",
        "cargo run --manifest-path .\\www\\Cargo.toml -p dx-www --bin dx-www -- forge release-bundle --verify .\\.dx\\adoption-app\\.dx\\forge\\adoption-smoke\\release-bundle --format markdown --fail-under 90",
        "--include-adoption",
        "cargo run --manifest-path .\\www\\Cargo.toml -p dx-www --bin dx-www -- forge release-trend --write-history --format markdown --fail-under 90",
        ".dx\\forge\\adoption-smoke\\forge-smoke.json",
        ".dx\\forge\\source-.dx/build-cache/manifest.json",
        ".dx\\forge\\receipts",
        "public\\forge.html",
        "public\\forge\\scorecard.html",
        "public\\forge\\ci.html",
        "public\\forge\\evidence.html",
        "public\\forge\\releases.html",
        "public\\forge\\changelog.html",
        "benchmarks\\reports\\forge-release-readiness-trend.json",
        "not a universal npm replacement",
        "not a full framework benchmark",
        "no `node_modules`",
        "CLOUDFLARE_R2_",
    ] {
        assert!(
            docs.contains(required),
            "real-project adoption docs are missing `{required}`"
        );
    }

    assert!(
        checklist.contains("docs\\forge-real-project-adoption.md"),
        "public launch checklist should link the real-project adoption guide"
    );
}

#[test]
fn forge_adoption_launch_checklist_documents_operator_flow() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .expect("repo root");
    let docs_path = repo_root.join("docs/forge-adoption-launch-checklist.md");
    let public_checklist_path = repo_root.join("docs/forge-public-launch-checklist.md");
    let ci_docs_path = repo_root.join("docs/forge-ci-smoke.md");
    let docs = fs::read_to_string(&docs_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", docs_path.display()));
    let public_checklist = fs::read_to_string(&public_checklist_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", public_checklist_path.display()));
    let ci_docs = fs::read_to_string(&ci_docs_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", ci_docs_path.display()));

    for required in [
        "# DX Forge Adoption Launch Checklist",
        "dx forge ci",
        "forge adoption-smoke --project .\\.dx\\adoption-app",
        "forge adoption-report --project .\\.dx\\adoption-app --format markdown",
        "prove vertical --fixture forge-adoption --out .\\.dx\\adoption-app\\public --write",
        "scripts\\ci\\forge-ci.ps1 -ArtifactDir .\\.dx\\ci -PagesDir .\\.dx\\forge-pages -FailUnder 90",
        "forge ci --verify-artifacts .\\.dx\\ci --fail-under 90",
        "forge ci --verify-pages .\\.dx\\forge-pages --fail-under 90",
        "benchmarks\\measure-forge-adoption-browser-smoke.ts",
        "benchmarks\\measure-forge-package-update-rehearsal.ts",
        "benchmarks\\measure-vertical-proof.ts",
        "forge release-bundle --project . --out .\\.dx\\forge-release-bundle-adoption --include-adoption",
        "forge beta-install --project .\\.dx\\forge-beta-app --release-bundle .\\.dx\\forge-release-bundle-adoption",
        ".dx\\ci\\forge-adoption-report.json",
        ".dx\\forge-pages\\forge\\adoption\\index.html",
        "benchmarks\\reports\\forge-adoption-browser-smoke.md",
        "benchmarks\\reports\\forge-package-update-rehearsal.md",
        "benchmarks\\reports\\forge-public-route-comparison.md",
        "no `node_modules`",
        "CLOUDFLARE_R2_",
        "universal npm, cargo, pip, or package-manager replacement",
        "full Astro, Svelte, HTMX, Next.js, WordPress, or React benchmark dominance",
    ] {
        assert!(
            docs.contains(required),
            "adoption launch checklist is missing `{required}`"
        );
    }

    assert!(
        public_checklist.contains("docs\\forge-adoption-launch-checklist.md"),
        "public launch checklist should link the adoption launch checklist"
    );
    assert!(
        ci_docs.contains("docs/forge-adoption-launch-checklist.md"),
        "CI smoke docs should link the adoption launch checklist"
    );
}

#[test]
fn forge_public_beta_quickstart_docs_cover_onboarding_flow() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .expect("repo root");
    let quickstart_path = repo_root.join("docs/forge-public-beta-quickstart.md");
    let adoption_checklist_path = repo_root.join("docs/forge-adoption-launch-checklist.md");
    let public_checklist_path = repo_root.join("docs/forge-public-launch-checklist.md");
    let quickstart = fs::read_to_string(&quickstart_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", quickstart_path.display()));
    let adoption_checklist = fs::read_to_string(&adoption_checklist_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", adoption_checklist_path.display()));
    let public_checklist = fs::read_to_string(&public_checklist_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", public_checklist_path.display()));

    for required in [
        "# DX Forge Public Beta Quickstart",
        "dx forge init-app --project .\\.dx\\forge-beta-app --write",
        "dx forge beta-install --project .\\.dx\\forge-beta-app --release-bundle .\\.dx\\forge-release-bundle-adoption --write",
        "dx check .\\.dx\\forge-beta-app --strict-forge --format markdown --fail-under 90",
        "dx prove vertical --fixture forge-quickstart --out .\\.dx\\forge-beta-app\\public --write",
        "dx forge ci --project .\\.dx\\forge-beta-app",
        "dx forge package-gallery --project .\\.dx\\forge-beta-app --public-index .\\.dx\\forge-beta-app\\public",
        "dx forge publisher-key generate --out .\\.dx\\forge-publisher --signer essencefromexistence",
        "dx forge publisher-key sign --key .\\.dx\\forge-publisher\\publisher-key.private.json",
        "dx forge release-operations --project . --release-manifest .\\.dx\\forge-release-bundle-adoption\\forge-release-.dx/build-cache/manifest.json",
        "dx forge publish-plan --project . --release-bundle .\\.dx\\forge-release-bundle-adoption",
        ".dx\\forge-release-bundle-adoption\\forge\\package-gallery\\index.html",
        "package_gallery",
        "ready-to-publish-plan",
        "cache_headers",
        ".dx\\ci\\forge-publish-plan.md",
        "node .\\benchmarks\\measure-forge-source-owned-package-review.ts",
        ".dx\\forge-beta-app\\.dx\\forge\\init-app\\dx-check.json",
        ".dx\\forge-beta-app\\.dx\\forge\\init-app\\forge-scorecard.json",
        ".dx\\forge-beta-app\\.dx\\forge\\beta-install\\forge-beta-install.json",
        ".dx\\ci\\forge-release-candidate.json",
        ".dx\\forge-beta-app\\public\\forge\\quickstart.html",
        ".dx\\forge-beta-app\\public\\forge\\package-gallery\\index.html",
        ".dx\\ci\\forge-publisher-key.md",
        ".dx\\ci\\forge-manifest-signing.md",
        ".dx\\ci\\forge-release-operations.md",
        "no `node_modules`",
        "not a universal npm replacement",
        "full Next.js, npm, WordPress, or framework replacement",
        "CLOUDFLARE_R2_",
    ] {
        assert!(
            quickstart.contains(required),
            "public beta quickstart is missing `{required}`"
        );
    }

    assert!(
        adoption_checklist.contains("docs\\forge-public-beta-quickstart.md"),
        "adoption checklist should link the public beta quickstart"
    );
    assert!(
        public_checklist.contains("docs\\forge-public-beta-quickstart.md"),
        "public launch checklist should link the public beta quickstart"
    );
}

#[test]
fn forge_ci_templates_wire_release_dashboard_gate() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .expect("repo root");
    let script_path = repo_root.join("scripts/ci/forge-ci.ps1");
    let workflow_path = repo_root.join(".github/workflows/forge-ci.yml");
    let docs_path = repo_root.join("docs/forge-ci-smoke.md");
    let script = fs::read_to_string(&script_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", script_path.display()));
    let workflow = fs::read_to_string(&workflow_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", workflow_path.display()));
    let docs = fs::read_to_string(&docs_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", docs_path.display()));

    for required in [
        "forge release-dashboard gate",
        "forge\", \"release-dashboard\"",
        "--ci-artifacts\", $artifactRoot",
        "--pages\", $pagesRoot",
        "--history\", (Join-Path $artifactRoot \"forge-benchmark-history.json\")",
        "--route-comparison\", $artifactRouteComparison",
        "forge release-dashboard json evidence",
        "forge-release-dashboard.md",
        "forge-release-dashboard.json",
        "Forge release-dashboard project created node_modules",
        "Forge release-dashboard JSON project created node_modules",
        "forge release-history record",
        "forge\", \"release-history\"",
        "--dashboard\", (Join-Path $artifactRoot \"forge-release-dashboard.json\")",
        "--output\", (Join-Path $artifactRoot \"forge-public-release-history.json\")",
        "Forge release-history artifact lane created node_modules",
        "forge launch-changelog json evidence",
        "forge\", \"launch-changelog\"",
        "--history\", (Join-Path $artifactRoot \"forge-public-release-history.json\")",
        "--output\", (Join-Path $artifactRoot \"forge-public-launch-changelog.json\")",
        "forge launch-changelog markdown evidence",
        "--output\", (Join-Path $artifactRoot \"forge-public-launch-changelog.md\")",
        "Forge launch-changelog artifact lane created node_modules",
        "Assert-ForgePublicRouteComparison -Path $routeComparisonPath",
        "missing public route $required",
        "forge releases public route",
        "--fixture\", \"forge-releases\"",
        "forge\\releases.proof.json",
        "forge\\releases",
        "forge changelog public route",
        "--fixture\", \"forge-changelog\"",
        "forge\\changelog.proof.json",
        "forge\\changelog",
        "forge adoption app evidence",
        "forge\", \"adoption-smoke\"",
        "forge adoption public route",
        "--fixture\", \"forge-adoption\"",
        "forge\\adoption.proof.json",
        "forge\\adoption",
    ] {
        assert!(
            script.contains(required),
            "forge CI script is missing `{required}`"
        );
    }

    assert!(workflow.contains(
        "./scripts/ci/forge-ci.ps1 -ArtifactDir .dx/ci -PagesDir .dx/forge-pages -FailUnder 90"
    ));
    assert!(workflow.contains("Verify release-history artifacts"));
    assert!(workflow.contains("forge-public-release-history.json"));
    assert!(workflow.contains("Verify launch-changelog artifacts"));
    assert!(workflow.contains("forge-public-launch-changelog.json"));
    assert!(workflow.contains("forge-public-launch-changelog.md"));
    assert!(workflow.contains("Verify adoption artifacts"));
    assert!(workflow.contains("forge-adoption-report.json"));
    assert!(workflow.contains("/forge/adoption"));
    assert!(workflow.contains("dx-forge-ci"));
    assert!(workflow.contains("/forge/releases"));
    assert!(workflow.contains("/forge/changelog"));
    assert!(docs.contains("forge release-dashboard --project <temp>"));
    assert!(docs.contains("forge-public-route-comparison.json"));
    assert!(docs.contains("/forge/changelog"));
    assert!(docs.contains("explicit public-route guard"));
    assert!(docs.contains("forge-release-dashboard.md"));
    assert!(docs.contains("forge-release-dashboard.json"));
    assert!(docs.contains(".dx/forge-pages/forge/releases.html"));
    assert!(docs.contains(".dx/forge-pages/forge/changelog.html"));
    assert!(docs.contains(".dx/forge-pages/forge/adoption.html"));
    assert!(docs.contains("https://<owner>.github.io/<repo>/forge/releases/"));
    assert!(docs.contains("https://<owner>.github.io/<repo>/forge/changelog/"));
    assert!(docs.contains("https://<owner>.github.io/<repo>/forge/adoption/"));
    assert!(docs.contains(".dx/ci/forge-public-release-history.json"));
    assert!(docs.contains(".dx/ci/forge-public-release-history.md"));
    assert!(docs.contains(".dx/ci/forge-public-launch-changelog.json"));
    assert!(docs.contains(".dx/ci/forge-public-launch-changelog.md"));
    assert!(docs.contains(".dx/ci/forge-adoption-report.json"));
    assert!(docs.contains(".dx/ci/forge/adoption.html"));
    assert!(docs.contains("forge release-history --dashboard"));
    assert!(docs.contains("forge launch-changelog --history"));
    assert!(docs.contains("Release History Preservation"));
    assert!(docs.contains("benchmarks/reports/forge-public-release-history.json"));
    assert!(docs.contains("prove vertical --fixture forge-releases"));
    assert!(docs.contains("prove vertical --fixture forge-changelog"));
    assert!(docs.contains("prove vertical --fixture forge-adoption"));
}

#[test]
fn forge_scorecard_writes_public_package_report() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("forge-scorecard.md");

    cli.cmd_forge(&[
        "scorecard".to_string(),
        "--format".to_string(),
        "markdown".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("forge scorecard");

    let report = fs::read_to_string(output_path).expect("scorecard report");
    assert!(report.contains("DX Forge Package Scorecard"));
    assert!(report.contains("shadcn/ui/button"));
    assert!(report.contains("dx/icon/search"));
    assert!(report.contains("auth/better-auth"));
    assert!(report.contains("not a universal npm replacement"));
    assert!(report.contains("node_modules"));
    assert!(report.contains("Latest /forge Payload And Browser Timing"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_packages_json_reports_launch_discovery_metadata() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("forge-packages.json");

    cli.cmd_forge(&[
        "packages".to_string(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("forge packages json");

    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.forge.packages");
    assert_eq!(report["discovery"]["schema"], "dx.discovery");
    assert_eq!(
        report["discovery"]["metadata_commands"]["packages"],
        "dx forge packages --json"
    );
    assert!(
        report["discovery"]["consumers"]
            .as_array()
            .expect("discovery consumers")
            .iter()
            .any(|consumer| consumer == "zed")
    );
    assert_eq!(
        report["packages"].as_array().expect("packages").len(),
        FORGE_WWW_TEMPLATE_PACKAGE_IDS.len()
    );
    let public_packages = report["packages"].as_array().expect("packages");
    assert!(
        public_packages.iter().all(|package| !package["package_id"]
                .as_str()
                .unwrap_or_default()
                .starts_with("shadcn/ui/")),
        "dx forge packages should expose Forge-native package ids, not internal registry ancestry"
    );
    assert!(public_packages.iter().any(|package| {
        package["package_id"] == "ui/input"
            && package["discoverable"] == true
            && package["metadata_command"] == "dx forge packages --json"
            && package["discoverable_by"]
                .as_array()
                .expect("discoverable_by")
                .iter()
                .any(|consumer| consumer == "zed")
            && package["cli_add"] == "dx add ui/input --write"
            && package["public_api"]
                .as_array()
                .expect("public_api")
                .iter()
                .any(|api| api == "Input")
    }));
    assert!(public_packages.iter().any(|package| {
        package["package_id"] == "ui/textarea"
            && package["discoverable"] == true
            && package["cli_add"] == "dx add ui/textarea --write"
            && package["public_api"]
                .as_array()
                .expect("public_api")
                .iter()
                .any(|api| api == "Textarea")
    }));
    assert!(
        report["packages"].as_array().expect("packages").iter().any(
            |package| package["package_id"] == "animation/motion"
                && package["discoverable"] == true
                && package["cli_add"] == "dx add motion-animation --write"
                && package["public_api"]
                    .as_array()
                    .expect("public_api")
                    .iter()
                    .any(|api| api == "MotionReveal")
        )
    );
    assert!(
        report["packages"].as_array().expect("packages").iter().any(
            |package| package["package_id"] == "content/fumadocs-next"
                && package["discoverable"] == true
                && package["cli_add"] == "dx add fumadocs --write"
                && package["template_role"] == "docs"
                && package["public_api"]
                    .as_array()
                    .expect("public_api")
                    .iter()
                    .any(|api| api == "DocsPage")
                && package["public_api"]
                    .as_array()
                    .expect("public_api")
                    .iter()
                    .any(|api| api == "dxFumadocsRouteContract")
        )
    );
    assert!(
        report["packages"].as_array().expect("packages").iter().any(
            |package| package["package_id"] == "forms/react-hook-form"
                && package["discoverable"] == true
                && package["cli_add"] == "dx add forms --write"
                && package["template_role"] == "forms"
                && package["public_api"]
                    .as_array()
                    .expect("public_api")
                    .iter()
                    .any(|api| api == "DxHookForm")
        )
    );
}

#[test]
fn forge_registry_validate_rejects_missing_registry_dependencies() {
    let dir = tempdir().expect("tempdir");
    let registry_path = dir.path().join("registry.json");
    fs::write(
        &registry_path,
        r#"{
          "items": [
            {
              "name": "dialog",
              "type": "registry:ui",
              "registryDependencies": ["button"]
            }
          ]
        }"#,
    )
    .expect("registry");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let error = cli
        .cmd_forge(&[
            "registry".to_string(),
            "validate".to_string(),
            "--file".to_string(),
            registry_path.to_string_lossy().into_owned(),
            "--json".to_string(),
            "--quiet".to_string(),
        ])
        .expect_err("registry validate should reject missing authored dependencies");

    assert!(
        error
            .to_string()
            .contains("depends on missing registry item `button`"),
        "{error}"
    );
}

#[test]
fn www_templates_json_reports_www_template_metadata() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("www-templates.json");

    cli.cmd_templates(&[
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("www templates json");

    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.www.templates");
    assert_eq!(report["discovery"]["schema"], "dx.discovery");
    assert_eq!(
        report["discovery"]["metadata_commands"]["templates"],
        "dx templates --json"
    );
    assert!(
        report["discovery"]["entrypoints"]
            .as_array()
            .expect("entrypoints")
            .iter()
            .any(|entrypoint| entrypoint["route"] == "/"
                && entrypoint["materialized_file"] == "app/page.tsx"
                && entrypoint["readiness_receipt"] == NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE)
    );
    assert_eq!(
        report["zed_template_handoff"]["schema"],
        "dx.zed.template_handoff"
    );
    assert_eq!(
        report["zed_template_handoff"]["entrypoint_file"],
        "app/page.tsx"
    );
    assert_eq!(
        report["zed_template_handoff"]["runtime_actions"]["requires_explicit_permission"],
        true
    );
    assert!(
        report["zed_template_handoff"]["safe_source_checks"]
            .as_array()
            .expect("safe source checks")
            .iter()
            .any(|command| command == "dx run --test .\\benchmarks\\template-shell.test.ts")
    );
    assert_eq!(
        report["launch_readiness_bundle"]["schema"],
        "dx.launch.readiness_bundle"
    );
    assert_eq!(
        report["launch_companion_doc_receipts"]["schema"],
        "dx.launch.companion_doc_receipts"
    );
    assert_eq!(
        report["launch_companion_doc_receipts"]["companion_count"],
        11
    );
    assert_eq!(
        report["launch_runtime_checklist"]["schema"],
        "dx.launch.runtime_checklist"
    );
    assert_eq!(
        report["launch_runtime_checklist"]["approval"]["status"],
        "requires-explicit-permission"
    );
    assert_eq!(
        report["launch_runtime_evidence"]["schema"],
        "dx.launch.runtime_evidence"
    );
    assert_eq!(
        report["launch_runtime_evidence"]["status"],
        "awaiting-approved-runtime-run"
    );
    assert_eq!(
        report["launch_verification_lane"]["schema"],
        "dx.launch.verification_lane"
    );
    assert_eq!(
        report["launch_verification_lane"]["runtime_artifacts"]["runtime_evidence"],
        ".dx/forge/template-readiness/launch-runtime-evidence.json"
    );
    assert_eq!(
        report["launch_readiness_bundle"]["metadata_commands"]["forge_template_readiness"],
        "dx forge template-readiness --project <path> --json"
    );
    assert!(
        report["launch_readiness_bundle"]["source_guards"]
            .as_array()
            .expect("bundle source guards")
            .iter()
            .any(|guard| guard["kind"] == "source_level_package_guard")
    );
    assert!(
            report["templates"]
                .as_array()
                .expect("templates")
                .iter()
                .any(|template| template["id"] == "next-familiar-www-template"
                    && template["discovery"]["schema"] == "dx.discovery"
                    && template["discovery"]["consumers"]
                        .as_array()
                        .expect("template discovery consumers")
                        .iter()
                        .any(|consumer| consumer == "zed")
                    && template["zed_template_handoff"]["schema"]
                        == "dx.zed.template_handoff"
                    && template["zed_template_handoff"]["open_files"]
                        .as_array()
                        .expect("handoff open files")
                        .iter()
                        .any(|file| file["path"] == "components/template-app/template-route-contract.ts")
                    && template["launch_readiness_bundle"]["schema"]
                        == "dx.launch.readiness_bundle"
                    && template["launch_companion_doc_receipts"]["schema"]
                        == "dx.launch.companion_doc_receipts"
                    && template["launch_runtime_checklist"]["schema"]
                        == "dx.launch.runtime_checklist"
                    && template["launch_runtime_evidence"]["schema"]
                        == "dx.launch.runtime_evidence"
                    && template["launch_verification_lane"]["schema"]
                        == "dx.launch.verification_lane"
                    && template["launch_runtime_checklist"]["approval"]["default_action"]
                        == "skip-runtime-build-preview"
                    && template["launch_companion_doc_receipts"]["companions"]
                        .as_array()
                        .expect("companion doc receipts")
                        .iter()
                        .any(|companion| companion["package_id"] == "wasm/bindgen"
                            && companion["materialized_file"]
                                == "components/template-app/wasm-interop-status.tsx"
                            && companion["proof_export"] == "LaunchWasmInteropStatus")
                    && template["launch_readiness_bundle"]["package_receipts"]["required_packages"]
                        .as_array()
                        .expect("bundle packages")
                        .iter()
                        .any(|package| package == "content/react-markdown")
                    && template["forge_packages"]
                        .as_array()
                        .expect("forge packages")
                        .iter()
                        .any(|package| package == "shadcn/ui/input")
                    && template["forge_packages"]
                        .as_array()
                        .expect("forge packages")
                        .iter()
                        .any(|package| package == "shadcn/ui/textarea")
                    && template["forge_packages"]
                        .as_array()
                        .expect("forge packages")
                        .iter()
                        .any(|package| package == "animation/motion")
                    && template["forge_packages"]
                        .as_array()
                        .expect("forge packages")
                        .iter()
                        .any(|package| package == "supabase/client")
                    && template["forge_packages"]
                        .as_array()
                        .expect("forge packages")
                        .iter()
                        .any(|package| package == "content/fumadocs-next")
                    && template["forge_packages"]
                        .as_array()
                        .expect("forge packages")
                        .iter()
                        .any(|package| package == "forms/react-hook-form")
                    && template["recommended_commands"]
                        .as_array()
                        .expect("commands")
                        .iter()
                        .any(|command| command == "dx add ui/input --write")
                    && template["recommended_commands"]
                        .as_array()
                        .expect("commands")
                        .iter()
                        .any(|command| command == "dx add ui/textarea --write")
                    && template["recommended_commands"]
                        .as_array()
                        .expect("commands")
                        .iter()
                        .any(|command| command == "dx add motion-animation --write")
                    && template["recommended_commands"]
                        .as_array()
                        .expect("commands")
                        .iter()
                        .any(|command| command == "dx add supabase/client --write")
                    && template["recommended_commands"]
                        .as_array()
                        .expect("commands")
                        .iter()
                        .any(|command| command == "dx add next-intl --write")
                    && template["recommended_commands"]
                        .as_array()
                        .expect("commands")
                        .iter()
                        .any(|command| command == "dx add content/fumadocs-next --write")
                    && template["recommended_commands"]
                        .as_array()
                        .expect("commands")
                        .iter()
                        .any(|command| command == "dx add forms --write")
                    && template["recommended_commands"]
                        .as_array()
                        .expect("commands")
                        .iter()
                        .any(|command| command == "dx add trpc --write")
                    && template["usage_examples"]
                        .as_array()
                        .expect("usage examples")
                        .iter()
                        .any(|example| example["package"] == "shadcn/ui/textarea"
                            && example["imports"]
                                .as_array()
                                .expect("imports")
                                .iter()
                                .any(|import| import
                                    == "import { Textarea } from \"@/components/ui/textarea\";"))
                    && template["usage_examples"]
                        .as_array()
                        .expect("usage examples")
                        .iter()
                        .any(|example| example["package"] == "animation/motion"
                            && example["imports"]
                                .as_array()
                                .expect("imports")
                                .iter()
                                .any(|import| import
                                    == "import { LaunchMotionInteractionProof } from \"@/components/template-app/motion-interaction-proof\";"))
                    && template["usage_examples"]
                        .as_array()
                        .expect("usage examples")
                        .iter()
                        .any(|example| example["package"] == "supabase/client"
                            && example["imports"]
                                .as_array()
                                .expect("imports")
                                .iter()
                                .any(|import| import
                                    == "import { readDxSupabaseProfileConfigStatus, createDxSupabaseProfilePreview, createDxSupabaseProfileUpsertReceipt } from \"@/lib/supabase/profile-workflow\";"))
                    && template["usage_examples"]
                        .as_array()
                        .expect("usage examples")
                        .iter()
                        .any(|example| example["package"] == "i18n/next-intl"
                            && example["imports"]
                                .as_array()
                                .expect("imports")
                                .iter()
                                .any(|import| import
                                    == "import { useTranslations } from \"next-intl\";"))
                    && template["usage_examples"]
                        .as_array()
                        .expect("usage examples")
                        .iter()
                        .any(|example| example["package"] == "content/fumadocs-next"
                            && example["imports"]
                                .as_array()
                                .expect("imports")
                                .iter()
                                .any(|import| import
                                    == "import { dxFumadocsRouteContract } from \"@/lib/fumadocs/route-contract\";"))
                    && template["usage_examples"]
                        .as_array()
                        .expect("usage examples")
                        .iter()
                        .any(|example| example["package"] == "forms/react-hook-form"
                            && example["imports"]
                                .as_array()
                                .expect("imports")
                                .iter()
                                .any(|import| import
                                    == "import { DxHookForm } from \"@/lib/forms/react-hook-form/form\";"))
                    && template["usage_examples"]
                        .as_array()
                        .expect("usage examples")
                        .iter()
                        .any(|example| example["package"] == "api/trpc"
                            && example["file"] == "examples/template/trpc-launch-health.tsx"
                            && example["imports"]
                                .as_array()
                                .expect("imports")
                                .iter()
                                .any(|import| import
                                    == "import { useTRPC } from \"@/lib/trpc/provider\";"))
                    && template["app_router_entrypoint"]["materialized_file"]
                        == "app/page.tsx"
                    && template["app_router_entrypoint"]["component_materialized_files"]
                        .as_array()
                        .expect("component files")
                        .iter()
                        .any(|file| file == "components/template-app/package-catalog.ts")
                    && template["app_router_entrypoint"]["component_materialized_files"]
                        .as_array()
                        .expect("component files")
                        .iter()
                        .any(|file| file == "components/template-app/trpc-launch-contract.ts")
                    && template["app_router_entrypoint"]["component_materialized_files"]
                        .as_array()
                        .expect("component files")
                        .iter()
                        .any(|file| file == "components/template-app/react-markdown-preview.tsx")
                    && template["example_files"]
                        .as_array()
                        .expect("example files")
                        .iter()
                        .any(|file| file == "examples/template/template-shell.tsx")
                    && template["example_files"]
                        .as_array()
                        .expect("example files")
                        .iter()
                        .any(|file| file == "examples/template/trpc-launch-contract.ts")
                    && template["example_files"]
                        .as_array()
                        .expect("example files")
                        .iter()
                        .any(|file| file == "examples/template/trpc-launch-health.tsx")
                    && template["www_package_catalog"]
                        .as_array()
                        .expect("launch package catalog")
                        .iter()
                        .any(|package| package["package_id"] == "instantdb/react"
                            && package["role"] == "realtime-data"
                            && package["command"] == "dx add instantdb/react --write"
                            && package["env"]
                                .as_array()
                                .expect("env")
                                .iter()
                                .any(|env| env == "NEXT_PUBLIC_INSTANT_APP_ID")
                            && package["app_owned_boundaries"]
                                .as_array()
                                .expect("boundaries")
                                .iter()
                                .any(|boundary| boundary.as_str().map_or(false, |boundary| boundary
                                    .contains("Instant dashboard app")
                                    && boundary.contains("NEXT_PUBLIC_INSTANT_APP_ID"))))
                    && template["www_package_catalog"]
                        .as_array()
                        .expect("launch package catalog")
                        .iter()
                        .any(|package| package["package_id"] == "i18n/next-intl"
                            && package["role"] == "i18n"
                            && package["command"] == "dx add next-intl --write"
                            && package["app_owned_boundaries"]
                                .as_array()
                                .expect("boundaries")
                                .iter()
                                .any(|boundary| boundary.as_str().map_or(false, |boundary| boundary
                                    .contains("Translated message quality")
                                    && boundary.contains("runtime dependency installation")))))
        );
}

#[test]
fn www_templates_verify_readiness_reports_generated_launch_receipts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("readiness-app").expect("dx new");

    let project = dir.path().join("readiness-app");
    let output_path = dir.path().join("launch-readiness.json");
    cli.cmd_templates(&[
        "verify-readiness".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("verify readiness");

    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.www.template_readiness_verification");
    assert_eq!(report["passed"], true);
    assert_eq!(report["score"], 100);
    assert_eq!(report["route"], "/");
    assert_eq!(
        report["runtime_verification_requires_explicit_permission"].as_bool(),
        Some(true)
    );
    assert_eq!(
        report["materialized_files"]["present"],
        report["materialized_files"]["total"]
    );
    assert_eq!(
        report["required_packages"]["present"],
        report["required_packages"]["total"]
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("checks")
            .iter()
            .any(|check| {
                check["name"] == "runtime-verification-gated"
                    && check["passed"] == true
                    && check["requires_explicit_permission"] == true
            })
    );
    assert!(
        report["next_commands"]
            .as_array()
            .expect("next commands")
            .iter()
            .any(|command| command == "governed runtime verification")
    );
}

#[test]
fn forge_scorecard_project_mode_merges_local_evidence() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["ui/button", "--write"])
        .expect("dx add ui/button");
    let output_path = dir.path().join("forge-scorecard.json");
    let history_path = write_passing_forge_benchmark_history(dir.path());

    cli.cmd_forge(&[
        "scorecard".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--history".to_string(),
        history_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("forge scorecard project mode");

    let report = fs::read_to_string(output_path).expect("scorecard report");
    assert!(report.contains("\"project\""));
    assert!(report.contains("\"dx_check_score\""));
    assert!(report.contains("\"package_docs_coverage_percent\""));
    assert!(report.contains("\"rollback_coverage_percent\""));
    assert!(report.contains("\"manifest_variant_count\""));
    assert!(report.contains("\"latest_forge_route_benchmark\""));
    assert!(report.contains("\"fixture_mode\": \"forge-site\""));
    assert!(report.contains("\"decoded_bytes\": 5200"));
    assert!(report.contains("\"shadcn/ui/button\""));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_scorecard_output_preserves_history_snapshot() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("forge-scorecard.json");

    cli.cmd_forge(&[
        "scorecard".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("forge scorecard");

    let history_dir = dir.path().join(".dx/forge/scorecard-history");
    let index_path = history_dir.join("index.json");
    let index_markdown_path = history_dir.join("index.md");
    let index = fs::read_to_string(&index_path).expect("history index");
    let index_markdown = fs::read_to_string(&index_markdown_path).expect("history markdown");

    assert!(index.contains("\"snapshots\""));
    assert!(index.contains("\"score_delta\""));
    assert!(index.contains("\"package_count\""));
    assert!(index_markdown.contains("DX Forge Package Scorecard History"));

    let snapshot_count = fs::read_dir(&history_dir)
        .expect("history dir")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let path = entry.path();
            path.extension().and_then(|value| value.to_str()) == Some("json")
                && path.file_name().and_then(|value| value.to_str()) != Some("index.json")
        })
        .count();
    assert_eq!(snapshot_count, 1);
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_trust_policy_writes_policy_file_and_report() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("forge-trust-policy.md");

    cli.cmd_forge(&[
        "trust-policy".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--write-policy".to_string(),
        "--format".to_string(),
        "markdown".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "80".to_string(),
        "--quiet".to_string(),
    ])
    .expect("forge trust-policy");

    let policy =
        fs::read_to_string(dir.path().join(".dx/forge/trust-policy.json")).expect("trust policy");
    let report = fs::read_to_string(output_path).expect("trust policy report");
    let model = build_forge_trust_policy_report(dir.path()).expect("trust policy model");

    assert!(policy.contains("\"allowed_packages\""));
    assert!(policy.contains("\"blocked_shapes\""));
    assert!(policy.contains("lifecycle-postinstall"));
    assert!(report.contains("DX Forge Trust Policy"));
    assert!(report.contains("Allowed Packages"));
    assert!(report.contains("Blocked Shapes"));
    assert!(report.contains("Package Owner Responsibilities"));
    assert!(report.contains("advisory-fixture-no-live-feed"));
    assert!(report.contains("curated-fixture"));
    assert!(model.policy_file_present);
    assert!(model.policy_file_matches_current);
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_release_trend_reports_current_evidence_against_previous_snapshot() {
    let dir = tempdir().expect("tempdir");
    let reports_dir = dir.path().join("benchmarks/reports");
    fs::create_dir_all(&reports_dir).expect("reports dir");
    fs::write(
        reports_dir.join("forge-public-release-history.json"),
        r#"{
  "updated_at": "2026-05-17T00:00:00Z",
  "records": [
    {
      "generated_at": "2026-05-17T00:00:00Z",
      "dashboard": { "score": 94, "passed": true },
      "route_comparison": {
        "route_count": 6,
        "total_decoded_bytes": 28731,
        "total_brotli_bytes": 6014,
        "routes": []
      }
    }
  ]
}"#,
    )
    .expect("release history");
    fs::write(
        reports_dir.join("forge-medium-route-comparison.json"),
        r#"{
  "generated_at": "2026-05-17T00:00:00Z",
  "scope": {
    "not_full_framework_benchmark": true,
    "no_node_modules_created": true
  },
  "frameworks": [
    { "framework": "DX-WWW", "total_decoded_bytes": 5057, "total_brotli_bytes": 1454 },
    { "framework": "Astro", "total_decoded_bytes": 4986, "total_brotli_bytes": 1438 }
  ]
}"#,
    )
    .expect("medium");
    fs::write(
        reports_dir.join("forge-large-content-comparison.json"),
        r#"{
  "generated_at": "2026-05-17T00:00:00Z",
  "first_route_budget": {
    "passed": true,
    "dxwww_decoded_bytes": 28008,
    "dxwww_brotli_bytes": 1534
  },
  "frameworks": [
    { "framework": "DX-WWW", "total_decoded_bytes": 28008, "total_brotli_bytes": 1534 }
  ]
}"#,
    )
    .expect("large");
    fs::write(
        reports_dir.join("forge-release-readiness-trend.json"),
        r#"{
  "updated_at": "2026-05-16T00:00:00Z",
  "records": [
    {
      "generated_at": "2026-05-16T00:00:00Z",
      "score": 88,
      "signals": {
        "public_bundle": { "score": 90, "brotli_bytes": 6500, "passed": true },
        "medium_route": { "score": 90, "brotli_bytes": 1600, "passed": true },
        "large_route": { "score": 85, "brotli_bytes": 1900, "passed": true },
        "trust_policy": { "score": 88, "traffic": "yellow", "passed": true }
      }
    }
  ]
}"#,
    )
    .expect("trend history");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("release-trend.md");
    cli.cmd_forge(&[
        "release-trend".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--write-history".to_string(),
        "--format".to_string(),
        "markdown".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "80".to_string(),
        "--quiet".to_string(),
    ])
    .expect("forge release-trend");

    let report = fs::read_to_string(output_path).expect("trend report");
    let history = fs::read_to_string(reports_dir.join("forge-release-readiness-trend.json"))
        .expect("trend history");

    assert!(report.contains("DX Forge Release Readiness Trend"));
    assert!(report.contains("public-bundle"));
    assert!(report.contains("medium-route"));
    assert!(report.contains("large-route"));
    assert!(report.contains("trust-policy"));
    assert!(report.contains("Score delta"));
    assert!(report.contains("Brotli delta"));
    assert!(report.contains("curated-fixture"));
    assert!(history.contains("\"records\""));
    assert!(history.contains("\"previous_score\""));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_verify_package_reports_single_package_evidence() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["ui/button", "--write"])
        .expect("dx add ui/button");

    cli.cmd_forge(&[
        "verify-package".to_string(),
        "ui/button".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--fail-under".to_string(),
        "80".to_string(),
    ])
    .expect("forge verify-package");

    let report = build_forge_verify_package_report(dir.path(), "ui/button", "default")
        .expect("verify report");
    assert!(report.passed);
    assert!(report.rollback.passed);
    assert_eq!(report.rollback.traffic, DxUpdateTraffic::Yellow);
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_verify_package_migration_static_site_checks_migration_artifacts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["migration/static-site", "--write"])
        .expect("dx add migration/static-site");

    let report = build_forge_verify_package_report(dir.path(), "migration/static-site", "default")
        .expect("verify report");
    let markdown = forge_verify_package_markdown(&report);

    assert!(report.passed);
    assert!(markdown.contains("migration-static-site-docs"));
    assert!(markdown.contains("migration-static-site-receipts"));
    assert!(markdown.contains("migration-static-site-asset-mapping"));
    assert!(markdown.contains("migration-static-site-manual-review"));
    assert!(markdown.contains("migration-static-site-source-files"));
    assert!(markdown.contains("migration-static-site-no-node-modules"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_verify_package_fails_when_required_docs_are_missing() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["ui/button", "--write"])
        .expect("dx add ui/button");
    fs::remove_file(dir.path().join(".dx/forge/docs/shadcn-ui-button.md")).expect("remove docs");

    let error = cli
        .cmd_forge(&[
            "verify-package".to_string(),
            "ui/button".to_string(),
            "--project".to_string(),
            ".".to_string(),
        ])
        .expect_err("verify-package should fail");

    assert!(error.to_string().contains("missing docs"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_verify_package_all_covers_launch_packages() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    for package_id in FORGE_WWW_TEMPLATE_PACKAGE_IDS {
        cli.cmd_add(&[package_id, "--write"])
            .unwrap_or_else(|error| panic!("dx add {package_id}: {error}"));
    }

    cli.cmd_forge(&[
        "verify-package".to_string(),
        "--all".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--fail-under".to_string(),
        "90".to_string(),
    ])
    .expect("forge verify-package --all");

    let report = build_forge_verify_all_packages_report(dir.path()).expect("verify all");
    assert!(report.passed);
    assert_eq!(report.packages.len(), FORGE_WWW_TEMPLATE_PACKAGE_IDS.len());
    assert!(report.missing_packages.is_empty());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_verify_package_all_reports_missing_launch_packages() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["ui/button", "--write"])
        .expect("dx add ui/button");

    let error = cli
        .cmd_forge(&[
            "verify-package".to_string(),
            "--all".to_string(),
            "--project".to_string(),
            ".".to_string(),
        ])
        .expect_err("verify all should fail when launch packages are missing");

    let report = build_forge_verify_all_packages_report(dir.path()).expect("verify all");
    assert!(!report.passed);
    assert_eq!(
        report.missing_packages.len(),
        FORGE_WWW_TEMPLATE_PACKAGE_IDS.len() - 1
    );
    assert!(
        error
            .to_string()
            .contains(&format!("missing={}", FORGE_WWW_TEMPLATE_PACKAGE_IDS.len() - 1))
    );
    assert!(!dir.path().join("node_modules").exists());
}
