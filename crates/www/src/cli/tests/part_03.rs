#[test]
fn forge_materialize_static_assets_copies_reviewed_assets_with_cache_evidence() {
    let dir = tempdir().expect("tempdir");
    let export_dir = dir.path().join("exports");
    let upload_dir = export_dir.join("wp-content/uploads");
    fs::create_dir_all(&upload_dir).expect("upload dir");
    let input_path = export_dir.join("landing.html");
    let hero_path = upload_dir.join("hero.jpg");
    fs::write(&hero_path, b"local hero bytes").expect("hero asset");
    fs::write(
        &input_path,
        r#"<!doctype html>
<html>
  <head>
    <title>Landing Page</title>
    <meta name="description" content="Asset materialization fixture">
    <link rel="canonical" href="https://example.test/landing/">
  </head>
  <body>
    <article>
      <h1>Landing Page</h1>
      <img src="/wp-content/uploads/hero.jpg" alt="Hero image">
      <img src="/wp-content/uploads/missing.jpg" alt="">
    </article>
  </body>
</html>"#,
    )
    .expect("html fixture");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let migration_report_path = dir.path().join("migrate-static-page-assets.json");
    cli.cmd_forge(&[
        "migrate-static-page".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--input".to_string(),
        input_path.to_string_lossy().into_owned(),
        "--route".to_string(),
        "/migrated/landing".to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        migration_report_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge migrate-static-page assets");

    let manifest_path = dir
        .path()
        .join("migrations/static-site/generated/landing/asset-.dx/build-cache/manifest.json");
    let public_dir = dir.path().join("public");
    let output_path = dir.path().join("materialize-static-assets.json");
    cli.cmd_forge(&[
        "materialize-static-assets".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--manifest".to_string(),
        manifest_path.to_string_lossy().into_owned(),
        "--public-dir".to_string(),
        public_dir.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
        "--fail-under".to_string(),
        "90".to_string(),
    ])
    .expect("forge materialize static assets");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["mode"], "write");
    assert_eq!(report["asset_count"], 2);
    assert_eq!(report["copied_asset_count"], 1);
    assert_eq!(report["unresolved_asset_count"], 1);
    assert_eq!(
        report["cache_policy"]["long_lived_header"],
        "public, max-age=31536000, immutable"
    );
    assert_eq!(report["no_node_modules"], true);
    assert_eq!(report["package_installs_run"], false);

    let copied_asset = public_dir.join("assets/migrated/landing/hero.jpg");
    assert!(copied_asset.exists());
    assert_eq!(
        fs::read(&copied_asset).expect("copied asset"),
        b"local hero bytes"
    );
    assert!(report["assets"]
        .as_array()
        .expect("assets")
        .iter()
        .any(
            |asset| asset["source_url"] == "/wp-content/uploads/hero.jpg"
                && asset["write_state"] == "copied"
                && asset["hash_verified"] == true
                && asset["cache_header"] == "public, max-age=31536000, immutable"
        ));
    assert!(report["assets"]
        .as_array()
        .expect("assets")
        .iter()
        .any(
            |asset| asset["source_url"] == "/wp-content/uploads/missing.jpg"
                && asset["write_state"] == "unresolved"
        ));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_migrate_static_page_blocks_unsafe_html_without_manual_review_decision() {
    let dir = tempdir().expect("tempdir");
    let export_dir = dir.path().join("exports");
    fs::create_dir_all(&export_dir).expect("export dir");
    let input_path = export_dir.join("unsafe.html");
    fs::write(
        &input_path,
        r#"<!doctype html>
<html>
  <head>
    <title>Unsafe Legacy Page</title>
    <meta name="description" content="Unsafe migration fixture">
    <link rel="canonical" href="https://example.test/unsafe/">
  </head>
  <body>
    <article>
      <h1>Unsafe Legacy Page</h1>
      <a onclick="trackClick()" href="/legacy">Legacy CTA</a>
      <form action="/wp-json/contact-form-7/v1/contact-forms/42/feedback"></form>
      <iframe src="https://player.example.test/embed/1"></iframe>
      [contact-form-7 id="42"]
      <script src="/wp-content/plugins/gallery/gallery.js"></script>
    </article>
  </body>
</html>"#,
    )
    .expect("html fixture");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let report_path = dir.path().join("unsafe-policy-blocked.json");
    let error = cli
        .cmd_forge(&[
            "migrate-static-page".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--input".to_string(),
            input_path.to_string_lossy().into_owned(),
            "--route".to_string(),
            "/migrated/unsafe".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--output".to_string(),
            report_path.to_string_lossy().into_owned(),
            "--quiet".to_string(),
        ])
        .expect_err("unsafe HTML should require an explicit review decision");

    assert!(error.to_string().contains("unsafe HTML policy gate"));
    let report = read_json_value(report_path);
    assert_eq!(report["passed"], false);
    assert_eq!(report["wrote_files"], false);
    assert_eq!(report["unsafe_html_policy"]["blocked"], true);
    assert_eq!(
        report["unsafe_html_policy"]["decision"],
        serde_json::Value::Null
    );
    assert_eq!(report["unsafe_html_policy"]["review_count"], 5);
    assert!(report["unsafe_html_policy"]["reviews"]
        .as_array()
        .expect("unsafe reviews")
        .iter()
        .any(|review| review["code"] == "script-tag"));
    assert!(report["unsafe_html_policy"]["reviews"]
        .as_array()
        .expect("unsafe reviews")
        .iter()
        .any(|review| review["code"] == "inline-event-handler"));
    assert!(report["source_files"]
        .as_array()
        .expect("source files")
        .iter()
        .all(|file| file["write_state"] == "blocked"));
    assert!(!dir
        .path()
        .join("migrations/static-site/generated/unsafe/content.ts")
        .exists());
    assert!(!dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_migrate_static_page_records_manual_review_decision_for_unsafe_html() {
    let dir = tempdir().expect("tempdir");
    let export_dir = dir.path().join("exports");
    fs::create_dir_all(&export_dir).expect("export dir");
    let input_path = export_dir.join("reviewed.html");
    fs::write(
        &input_path,
        r#"<!doctype html>
<html>
  <head>
    <title>Reviewed Legacy Page</title>
    <meta name="description" content="Reviewed unsafe fixture">
    <link rel="canonical" href="https://example.test/reviewed/">
  </head>
  <body>
    <article>
      <h1>Reviewed Legacy Page</h1>
      <button onmouseover="legacyHover()">Legacy button</button>
      <form action="/legacy-contact"></form>
      [gallery ids="1,2"]
      <script>legacyGallery()</script>
    </article>
  </body>
</html>"#,
    )
    .expect("html fixture");

    let decision = "Reviewed by migration lead: remove scripts, replace form, and rebuild shortcode before production publish.";
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let report_path = dir.path().join("unsafe-policy-reviewed.json");
    cli.cmd_forge(&[
        "migrate-static-page".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--input".to_string(),
        input_path.to_string_lossy().into_owned(),
        "--route".to_string(),
        "/migrated/reviewed".to_string(),
        "--unsafe-html-review".to_string(),
        decision.to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        report_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("reviewed unsafe HTML should write with recorded decision");

    let report = read_json_value(report_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["status"], "needs-review");
    assert_eq!(report["unsafe_html_policy"]["blocked"], false);
    assert_eq!(report["unsafe_html_policy"]["decision"], decision);
    assert!(report["unsafe_html_policy"]["reviews"]
        .as_array()
        .expect("unsafe reviews")
        .iter()
        .any(|review| review["code"] == "wordpress-shortcode-leftover"));

    let content_path = dir
        .path()
        .join("migrations/static-site/generated/reviewed/content.ts");
    let readme_path = dir
        .path()
        .join("migrations/static-site/generated/reviewed/README.md");
    assert!(content_path.exists());
    let generated_content = fs::read_to_string(&content_path).expect("generated content");
    let generated_readme = fs::read_to_string(&readme_path).expect("generated readme");
    assert!(generated_content.contains("unsafeHtmlManualReviewDecision"));
    assert!(generated_content.contains(decision));
    assert!(generated_readme.contains(decision));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_migrate_static_page_writes_hosted_preview_artifact_for_reviewers() {
    let dir = tempdir().expect("tempdir");
    let export_dir = dir.path().join("exports");
    fs::create_dir_all(&export_dir).expect("export dir");
    let input_path = export_dir.join("preview.html");
    fs::write(
        &input_path,
        r#"<!doctype html>
<html>
  <head>
    <title>Previewed Legacy Page</title>
    <meta name="description" content="Preview artifact fixture">
    <link rel="canonical" href="https://example.test/preview/">
  </head>
  <body>
    <article>
      <h1>Previewed Legacy Page</h1>
      <p>This route needs public beta review before launch.</p>
      <form action="/legacy-contact"></form>
      <script>legacyWidget()</script>
    </article>
  </body>
</html>"#,
    )
    .expect("html fixture");

    let decision =
        "Reviewed for hosted preview: replace the form and legacy widget before production.";
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let report_path = dir.path().join("preview-artifact-report.json");
    cli.cmd_forge(&[
        "migrate-static-page".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--input".to_string(),
        input_path.to_string_lossy().into_owned(),
        "--route".to_string(),
        "/migrated/preview".to_string(),
        "--unsafe-html-review".to_string(),
        decision.to_string(),
        "--write".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        report_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge migrate-static-page preview artifact");

    let report = read_json_value(report_path);
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["preview_artifact"]["route"],
        "/forge/migrated-route-preview/preview/"
    );
    assert_eq!(report["preview_artifact"]["write_state"], "written");
    assert_eq!(report["preview_artifact"]["manual_review_warning_count"], 6);
    for kind in [
        "migration-audit",
        "generated-source",
        "asset-manifest",
        "benchmark-fixture",
        "manual-review",
    ] {
        assert!(
            report["preview_artifact"]["links"]
                .as_array()
                .expect("preview links")
                .iter()
                .any(|link| link["kind"] == kind),
            "missing preview link kind {kind}"
        );
    }
    assert!(report["source_files"]
        .as_array()
        .expect("source files")
        .iter()
        .any(|file| file["kind"] == "hosted-preview-index" && file["write_state"] == "written"));

    let generated_dir = dir.path().join("migrations/static-site/generated/preview");
    let preview_index = generated_dir.join("preview/index.html");
    let preview_json = generated_dir.join("preview/preview.json");
    let audit_json = generated_dir.join("migration-audit.json");
    let benchmark_json = generated_dir.join("benchmark-fixture.json");
    assert!(preview_index.exists());
    assert!(preview_json.exists());
    assert!(audit_json.exists());
    assert!(benchmark_json.exists());

    let preview_html = fs::read_to_string(&preview_index).expect("preview html");
    assert!(preview_html.contains("DX Forge Migrated Route Preview"));
    assert!(preview_html.contains("../migration-audit.json"));
    assert!(preview_html.contains("../content.ts"));
    assert!(preview_html.contains("../asset-.dx/build-cache/manifest.json"));
    assert!(preview_html.contains("../benchmark-fixture.json"));
    assert!(preview_html.contains(decision));

    let preview = read_json_value(preview_json);
    assert_eq!(preview["links"][0]["kind"], "migration-audit");
    assert!(preview["manual_review_warnings"]
        .as_array()
        .expect("warnings")
        .iter()
        .any(|warning| warning
            .as_str()
            .is_some_and(|value| value.contains("Replace legacy forms"))));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_static_migration_smoke_runs_end_to_end_temp_project_pipeline() {
    let dir = tempdir().expect("tempdir");
    let output_path = dir.path().join("static-migration-smoke.json");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_forge(&[
        "static-migration-smoke".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--quiet".to_string(),
        "--fail-under".to_string(),
        "90".to_string(),
    ])
    .expect("forge static migration smoke");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert!(report["score"].as_u64().expect("score") >= 90);
    assert_eq!(report["no_node_modules"], true);
    assert_eq!(report["package_installs_run"], false);
    assert!(report["temp_project"]
        .as_str()
        .is_some_and(|path| path.contains("static-migration-smoke")));

    let steps = report["steps"].as_array().expect("steps");
    let names = steps
        .iter()
        .map(|step| step["name"].as_str().expect("step name"))
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        vec![
            "migration-audit",
            "migrate-static-page",
            "verify-package",
            "migrated-route-benchmark",
            "package-gallery"
        ]
    );
    assert!(steps.iter().all(|step| step["passed"] == true));

    let artifacts_dir = dir.path().join(".dx/forge/static-migration-smoke");
    let temp_project = artifacts_dir.join("temp-project");
    for artifact in [
        "reports/migration-audit.json",
        "reports/migrate-static-page.json",
        "reports/verify-package.json",
        "reports/migrated-route-benchmark.json",
        "reports/package-gallery.json",
        "reports/static-migration-smoke.json",
    ] {
        assert!(
            artifacts_dir.join(artifact).exists(),
            "static migration smoke missing artifact `{artifact}`"
        );
    }
    assert!(temp_project
        .join("migrations/static-site/generated/smoke/preview/index.html")
        .exists());
    assert!(temp_project
        .join("migrations/static-site/generated/smoke/asset-.dx/build-cache/manifest.json")
        .exists());
    assert!(temp_project.join(".dx/forge/source-.dx/build-cache/manifest.json").exists());
    assert!(!temp_project.join("node_modules").exists());
    assert!(!artifacts_dir.join("node_modules").exists());
}

#[test]
fn forge_static_migration_plan_batches_routes_before_writes() {
    let dir = tempdir().expect("tempdir");
    let export_dir = dir.path().join("exports");
    fs::create_dir_all(&export_dir).expect("export dir");
    fs::write(
        export_dir.join("about.html"),
        r#"<!doctype html>
<html>
  <head>
    <title>About DX Forge</title>
    <meta name="description" content="Source-owned static migration">
  </head>
  <body><main><h1>About DX Forge</h1></main></body>
</html>"#,
    )
    .expect("about fixture");
    fs::write(
        export_dir.join("pricing.html"),
        r#"<!doctype html>
<html>
  <head>
    <title>Pricing</title>
    <meta name="description" content="Plan pricing page">
  </head>
  <body><main><h1>Pricing</h1><script>legacyPricing()</script></main></body>
</html>"#,
    )
    .expect("pricing fixture");
    fs::create_dir_all(export_dir.join("contact")).expect("contact dir");
    fs::write(
        export_dir.join("contact/index.html"),
        r#"<!doctype html>
<html>
  <head>
    <title>Contact</title>
    <meta name="description" content="Contact page">
  </head>
  <body><main><h1>Contact</h1></main></body>
</html>"#,
    )
    .expect("contact fixture");
    fs::write(
        dir.path().join("package.json"),
        r#"{"scripts":{"postinstall":"node -e \"require('fs').writeFileSync('sentinel','bad')\"}}"#,
    )
    .expect("package json");

    let output_path = dir.path().join("static-migration-plan.json");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "static-migration-plan".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--input".to_string(),
        export_dir.to_string_lossy().into_owned(),
        "--route-prefix".to_string(),
        "/migrated".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--quiet".to_string(),
        "--fail-under".to_string(),
        "90".to_string(),
    ])
    .expect("forge static migration plan");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert!(report["score"].as_u64().expect("score") >= 90);
    assert_eq!(report["page_count"], 3);
    assert_eq!(report["ready_count"], 2);
    assert_eq!(report["manual_review_count"], 1);
    assert_eq!(report["blocked_count"], 0);
    assert_eq!(report["no_node_modules"], true);
    assert_eq!(report["package_installs_run"], false);

    let routes = report["routes"].as_array().expect("planned routes");
    let route_values = routes
        .iter()
        .map(|route| route["route"].as_str().expect("route"))
        .collect::<Vec<_>>();
    assert_eq!(
        route_values,
        vec!["/migrated/about", "/migrated/contact", "/migrated/pricing"]
    );
    assert!(routes
        .iter()
        .any(|route| { route["slug"] == "pricing" && route["review_state"] == "needs-review" }));
    assert!(routes.iter().all(|route| route["write_state"] == "planned"));
    assert!(report["batch_commands"]
        .as_array()
        .expect("batch commands")
        .iter()
        .any(|command| command
            .as_str()
            .is_some_and(|value| value.contains("migrate-static-page")
                && value.contains("--route /migrated/pricing"))));
    assert!(!dir
        .path()
        .join("migrations/static-site/generated/about")
        .exists());
    assert!(!dir.path().join("sentinel").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_add_auth_google_writes_source_owned_auth_package() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let args = vec![
        "add".to_string(),
        "auth/better-auth".to_string(),
        "--write".to_string(),
    ];

    cli.cmd_forge(&args).expect("forge add auth/better-auth");

    assert!(dir.path().join("auth/better-auth/options.ts").exists());
    assert!(dir.path().join("auth/better-auth/server.ts").exists());
    assert!(dir.path().join("auth/better-auth/client.ts").exists());
    assert!(dir.path().join("auth/better-auth/route.ts").exists());
    assert!(dir.path().join("auth/better-auth/metadata.ts").exists());
    assert!(dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_add_source_owned_package_supports_project_option() {
    let workspace = tempdir().expect("tempdir");
    let project = workspace.path().join("project");
    fs::create_dir_all(&project).expect("project dir");
    let cli = Cli::with_cwd(workspace.path().to_path_buf());

    cli.cmd_add(&["ui/button", "--project", "project"])
        .expect("dx add ui/button --project");

    assert!(project.join("components/ui/button.tsx").exists());
    assert!(project.join(".dx/forge/source-.dx/build-cache/manifest.json").exists());
    assert!(!workspace.path().join("components/ui/button.tsx").exists());
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_add_dry_run_terminal_output_stays_human_readable() {
    let dir = tempdir().expect("tempdir");
    let outcome =
        plan_forge_add_variant("ui/button", "default", dir.path()).expect("dx add dry-run");

    let rendered = dx_add_outcome_terminal(&outcome);

    assert!(rendered.contains("DX add dry-run"));
    assert!(rendered.contains("Files planned"));
    assert!(rendered.contains("No files were written"));
    assert!(rendered.contains("--format json"));
    assert!(!rendered.contains("\"files_written\""));
    assert!(!rendered.contains("\"policy_decisions\""));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_add_source_owned_package_accepts_explicit_json_format() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_add(&["ui/button", "--dry-run", "--format", "json"])
        .expect("dx add ui/button --dry-run --format json");

    assert!(!dir.path().join("components/ui/button.tsx").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_update_ui_button_dry_run_reports_change_set_without_writing() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["ui/button"]).expect("dx add ui/button");
    fs::write(
        dir.path().join("components/ui/button.tsx"),
        "export const Button = 'local update preview';",
    )
    .expect("local edit");

    cli.cmd_update(&[
        "ui/button".to_string(),
        "--dry-run".to_string(),
        "--format".to_string(),
        "json".to_string(),
    ])
    .expect("dx update ui/button --dry-run");

    let content = fs::read_to_string(dir.path().join("components/ui/button.tsx"))
        .expect("button still exists");
    assert!(content.contains("local update preview"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_update_ui_button_write_records_green_update_without_node_modules() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["ui/button"]).expect("dx add ui/button");

    cli.cmd_update(&["ui/button".to_string(), "--write".to_string()])
        .expect("dx update ui/button --write");

    let receipts = fs::read_dir(dir.path().join(".dx/forge/receipts"))
        .expect("receipts dir")
        .count();
    assert!(receipts >= 2);
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_update_ui_button_variant_targets_only_named_fork() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["ui/button"]).expect("dx add ui/button");
    cli.cmd_add(&["ui/button", "--variant", "marketing"])
        .expect("dx add ui/button variant");
    fs::write(
        dir.path().join("components/ui/button.tsx"),
        "export const Button = 'default fork changed';",
    )
    .expect("edit default");

    cli.cmd_update(&[
        "ui/button".to_string(),
        "--variant".to_string(),
        "marketing".to_string(),
        "--dry-run".to_string(),
    ])
    .expect("dx update ui/button --variant marketing");

    let default_content =
        fs::read_to_string(dir.path().join("components/ui/button.tsx")).expect("default button");
    assert!(default_content.contains("default fork changed"));
    assert!(dir
        .path()
        .join("components/ui/variants/marketing/button.tsx")
        .exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_update_ui_button_write_blocks_local_edits() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["ui/button"]).expect("dx add ui/button");
    fs::write(
        dir.path().join("components/ui/button.tsx"),
        "export const Button = 'local write block';",
    )
    .expect("local edit");

    assert!(cli
        .cmd_update(&["ui/button".to_string(), "--write".to_string()])
        .is_err());

    let content = fs::read_to_string(dir.path().join("components/ui/button.tsx"))
        .expect("button still exists");
    assert!(content.contains("local write block"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn dx_update_accept_yellow_records_review_receipt() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["ui/button"]).expect("dx add ui/button");
    fs::write(
        dir.path().join("components/ui/button.tsx"),
        "export const Button = 'approved cli edit';\n",
    )
    .expect("local edit");

    cli.cmd_update(&[
        "ui/button".to_string(),
        "--write".to_string(),
        "--accept-yellow".to_string(),
        "--reviewer".to_string(),
        "essencefromexistence".to_string(),
        "--review-note".to_string(),
        "Approved local UI edit after reviewing the update preview.".to_string(),
    ])
    .expect("reviewed yellow update");

    let content = fs::read_to_string(dir.path().join("components/ui/button.tsx"))
        .expect("button still exists");
    let receipts_dir = dir.path().join(".dx/forge/receipts");
    let receipt_text = fs::read_dir(&receipts_dir)
        .expect("receipt dir")
        .filter_map(|entry| entry.ok())
        .map(|entry| fs::read_to_string(entry.path()).expect("receipt"))
        .find(|receipt| receipt.contains("explicit-yellow-review"))
        .expect("review receipt");

    assert!(content.contains("approved cli edit"));
    assert!(receipt_text.contains("essencefromexistence"));
    assert!(receipt_text.contains("Approved local UI edit"));
    assert!(receipt_text.contains("local-edit"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_audit_fail_under_returns_error_for_red_fixture() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("package.json"),
        r#"{"scripts":{"prepare":"node router_init.js"}}"#,
    )
    .expect("write package");
    fs::write(dir.path().join("router_init.js"), "filev2.getsession.org").expect("write ioc");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let args = vec![
        "audit".to_string(),
        ".".to_string(),
        "--fail-under".to_string(),
        "80".to_string(),
    ];

    assert!(cli.cmd_forge(&args).is_err());
}

#[test]
fn forge_registry_init_creates_local_index() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let args = vec![
        "registry".to_string(),
        "init".to_string(),
        "--local".to_string(),
        "registry".to_string(),
    ];

    cli.cmd_forge(&args).expect("registry init");

    assert!(dir.path().join("registry/index.json").exists());
    assert!(dir
        .path()
        .join("registry/packages/js/shadcn/ui/button/0.1.0/.dx/build-cache/manifest.json")
        .exists());
    assert!(dir
        .path()
        .join("registry/packages/js/shadcn/ui/card/0.1.0/.dx/build-cache/manifest.json")
        .exists());
}

#[test]
fn forge_registry_smoke_proves_dry_run_publish_pull_without_secrets() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("hosted-registry-smoke.json");

    cli.cmd_forge(&[
        "registry".to_string(),
        "smoke".to_string(),
        "--package".to_string(),
        "ui/button".to_string(),
        "--local".to_string(),
        "registry-smoke".to_string(),
        "--remote".to_string(),
        "r2".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "100".to_string(),
        "--quiet".to_string(),
    ])
    .expect("registry smoke");

    let report = read_json_value(output_path.clone());
    assert_eq!(report["passed"], true);
    assert_eq!(report["score"], 100);
    assert_eq!(report["package_id"], "ui/button");
    assert_eq!(report["remote"], "r2");
    assert_eq!(report["requires_secrets"], false);
    assert_eq!(report["no_node_modules"], true);
    assert!(dir
        .path()
        .join("registry-smoke/packages/js/shadcn/ui/button/0.1.0/.dx/build-cache/manifest.json")
        .is_file());

    let operations = report["operations"].as_array().expect("operations");
    let publish = registry_smoke_operation(operations, "registry-publish");
    assert_eq!(publish["dry_run"], true);
    assert_eq!(publish["package_id"], "ui/button");
    assert!(publish["objects"]
        .as_array()
        .expect("publish objects")
        .iter()
        .any(|object| object
            .as_str()
            .is_some_and(|value| value.contains(".dx/build-cache/manifest.json"))));
    let pull = registry_smoke_operation(operations, "registry-pull");
    assert_eq!(pull["dry_run"], true);
    assert_eq!(pull["package_id"], "ui/button");
    assert!(pull["objects"]
        .as_array()
        .expect("pull objects")
        .iter()
        .any(|object| object
            .as_str()
            .is_some_and(|value| value.contains("files/"))));

    let report_text = fs::read_to_string(output_path).expect("registry smoke report");
    assert!(!report_text.contains(r#""package_id": "shadcn/ui/button""#));
    for marker in [
        "CLOUDFLARE_R2_",
        "DX_FORGE_R2_LIVE",
        "R2_SECRET",
        "SECRET_ACCESS_KEY",
    ] {
        assert!(
            !report_text.contains(marker),
            "registry smoke leaked secret marker {marker}"
        );
    }
    assert!(!dir.path().join("node_modules").exists());
    assert!(!dir.path().join("registry-smoke/node_modules").exists());
}

#[test]
fn forge_registry_smoke_defaults_to_r2_remote() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("hosted-registry-smoke-default.json");

    cli.cmd_forge(&[
        "registry".to_string(),
        "smoke".to_string(),
        "--local".to_string(),
        "registry-smoke".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "100".to_string(),
        "--quiet".to_string(),
    ])
    .expect("registry smoke");

    let report = read_json_value(output_path);
    assert_eq!(report["remote"], "r2");
    assert_eq!(report["passed"], true);
}

#[test]
fn forge_add_alias_still_creates_canonical_manifest() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let args = vec![
        "add".to_string(),
        "ui/button".to_string(),
        "--write".to_string(),
    ];

    cli.cmd_forge(&args).expect("forge add alias");

    let manifest =
        fs::read_to_string(dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json")).expect("manifest");
    assert!(manifest.contains("shadcn/ui/button"));
}

#[test]
fn dx_check_returns_combined_score() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "add".to_string(),
        "shadcn/ui/button".to_string(),
        "--write".to_string(),
    ])
    .expect("forge add");

    cli.cmd_check(&[".".to_string(), "--format".to_string(), "json".to_string()])
        .expect("dx check");
}

#[test]
fn dx_check_project_contract_flag_accepts_next_familiar_app() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("dx"), "project.name = contract\n").expect("config");
    fs::create_dir_all(dir.path().join("app/dashboard")).expect("app route");
    fs::create_dir_all(dir.path().join("components/ui")).expect("components");
    fs::create_dir_all(dir.path().join("server")).expect("server");
    fs::create_dir_all(dir.path().join("styles")).expect("styles");
    fs::write(
        dir.path().join("app/dashboard/page.tsx"),
        "export default function Page() { return <main />; }\n",
    )
    .expect("page");
    fs::write(
        dir.path().join("components/ui/Button.tsx"),
        "export function Button() { return <button />; }\n",
    )
    .expect("button");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_check(&[
        ".".to_string(),
        "--project-contract".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--fail-under".to_string(),
        "1".to_string(),
    ])
    .expect("dx check project contract");
}

#[test]
fn dx_check_strict_project_contract_fails_for_node_modules() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("dx"), "project.name = contract\n").expect("config");
    for path in ["app", "components", "server", "styles", "node_modules"] {
        fs::create_dir_all(dir.path().join(path)).expect("project dir");
    }

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let error = cli
        .cmd_check(&[
            ".".to_string(),
            "--strict-project-contract".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--fail-under".to_string(),
            "1".to_string(),
        ])
        .expect_err("strict project contract should fail");

    assert!(error.to_string().contains("project contract failed"));
    assert!(error
        .to_string()
        .contains("project-contract-node-modules-present"));
}

#[test]
fn dx_check_project_contract_writes_hints_without_source_writes() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("dx"), "project.name = contract\n").expect("config");
    fs::create_dir_all(dir.path().join("app")).expect("app");
    fs::write(
        dir.path().join("app/page.tsx"),
        "export default function Page() { return <main />; }\n",
    )
    .expect("page");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir
        .path()
        .join(".dx/forge/hints/project-contract-hints.json");
    cli.cmd_check(&[
        ".".to_string(),
        "--project-contract".to_string(),
        "--hints-output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
    ])
    .expect("dx check hints");

    let report = read_json_value(output_path);
    assert_eq!(report["source_files_written"], false);
    assert_eq!(report["auto_write_on_save"], false);
    assert!(report["hints"].as_array().expect("hints").len() >= 3);
    assert!(!dir.path().join("components").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_import_npm_plan_reports_without_installing_or_running_scripts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("npm-import-plan.json");

    cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "react".to_string(),
        "--plan".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge import npm plan");

    let report = read_json_value(output_path);
    assert_eq!(report["ecosystem"], "npm");
    assert_eq!(report["package_name"], "react");
    assert_eq!(report["mode"], "plan-only");
    assert_eq!(report["source_kind"], "registry-plan-review-required");
    assert_eq!(report["source_dir_ready"], false);
    assert_eq!(report["disposition"]["kind"], "bridge");
    assert_eq!(
        report["disposition"]["route"],
        "bridge-uninspected-external-boundary"
    );
    assert_eq!(
        report["disposition"]["ownership_claim"],
        "external-boundary"
    );
    assert_eq!(report["disposition"]["importable_source"], false);
    assert_eq!(report["materialization_ready"], false);
    assert_eq!(report["materialized"], false);
    assert_eq!(report["package_installs_run"], false);
    assert_eq!(report["lifecycle_scripts_executed"], false);
    assert_eq!(report["forge_import_gate"], true);
    assert!(report["review_required"].as_array().expect("review").len() >= 3);
    assert!(report_artifact_path(dir.path(), &report, "import_plan_path").is_file());
    assert!(report_artifact_path(dir.path(), &report, "import_plan_sr_path").is_file());
    assert!(report_artifact_path(dir.path(), &report, "import_plan_machine_path").is_file());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_acquire_npm_downloads_reviewed_source_without_node_modules() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("dx"), "project.name = forge-npm-acquire\n").expect("config");
    let tarball = npm_tgz_fixture(&[
        (
            "package/index.ts",
            b"export function startCase(value: string): string { return value.toUpperCase(); }\nexport default { startCase };\n",
        ),
        (
            "package/package.json",
            br#"{"name":"tiny-package","version":"1.2.3","license":"MIT"}"#,
        ),
        ("package/LICENSE", b"MIT fixture license\n"),
    ]);
    let integrity = npm_integrity_sha512(&tarball);
    let registry_url = start_mock_npm_registry("tiny-package", "1.2.3", tarball, &integrity);
    let output_path = dir.path().join("npm-acquire.json");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "acquire".to_string(),
        "npm".to_string(),
        "tiny-package".to_string(),
        "--registry-url".to_string(),
        registry_url,
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge acquire npm");

    let source_dir = dir.path().join(".dx/cache/npm/tiny-package/package");
    assert!(source_dir.join("index.ts").is_file());
    assert!(source_dir.join("package.json").is_file());
    assert!(source_dir.join("LICENSE").is_file());
    assert!(source_dir.join("dx-forge-evidence.sr").is_file());
    assert!(!dir.path().join("node_modules").exists());

    let evidence =
        fs::read_to_string(source_dir.join("dx-forge-evidence.sr")).expect("evidence");
    assert!(evidence.contains("integrity=true"));
    assert!(evidence.contains("provenance_verified=true"));
    assert!(evidence.contains("license=MIT"));

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["ecosystem"], "npm");
    assert_eq!(report["package_name"], "tiny-package");
    assert_eq!(report["package_id"], "npm/tiny-package");
    assert_eq!(report["version"], "1.2.3");
    assert_eq!(report["source_dir_ready"], true);
    assert_eq!(report["integrity_verified"], true);
    assert_eq!(report["package_installs_run"], false);
    assert_eq!(report["lifecycle_scripts_executed"], false);
    assert_eq!(report["package_manager_execution_allowed"], false);
    assert_eq!(report["import_plan"]["source_dir_ready"], true);
    assert_eq!(report["import_plan"]["package_installs_run"], false);
    assert_eq!(report["import_plan"]["lifecycle_scripts_executed"], false);
    assert!(report["next_commands"]
        .as_array()
        .expect("next commands")
        .iter()
        .any(|command| command
            .as_str()
            .unwrap_or_default()
            .contains("dx forge import npm tiny-package --plan")));
}

#[test]
fn forge_import_external_ecosystem_plan_exposes_serializer_artifacts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("cargo-serde-plan.json");

    cli.cmd_forge(&[
        "import".to_string(),
        "cargo".to_string(),
        "serde".to_string(),
        "--plan".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge import cargo serde plan");

    let report = read_json_value(output_path);
    assert_eq!(report["ecosystem"], "cargo");
    assert_eq!(report["package_name"], "serde");
    assert_eq!(report["package_id"], "cargo/serde");
    assert_eq!(report["mode"], "plan-only");
    assert_eq!(report["import_alias"], "cargo/serde");
    assert_eq!(report["source_kind"], "registry-plan-review-required");
    assert_eq!(report["disposition"]["kind"], "bridge");
    assert_eq!(
        report["disposition"]["bridge_kind"],
        "package-manager-boundary"
    );
    assert_eq!(report["disposition"]["materializes_source"], false);
    assert_eq!(
        report["materialization_status"],
        "bridge-required-before-materialization"
    );
    assert_eq!(report["materialization_ready"], false);
    assert_eq!(report["materialized"], false);
    assert_eq!(report["package_installs_run"], false);
    assert_eq!(report["lifecycle_scripts_executed"], false);
    assert!(
        report["next_commands"]
            .as_array()
            .expect("next commands")
            .len()
            >= 3
    );
    assert!(report_artifact_path(dir.path(), &report, "import_plan_path").is_file());
    assert!(report_artifact_path(dir.path(), &report, "import_plan_sr_path").is_file());
    assert!(report_artifact_path(dir.path(), &report, "import_plan_machine_path").is_file());
    assert!(!dir.path().join("lib/forge/cargo/serde/lib.rs").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_import_jsr_plan_keeps_clean_javascript_import_alias() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("jsr-std-path-plan.json");

    cli.cmd_forge(&[
        "import".to_string(),
        "jsr".to_string(),
        "@std/path".to_string(),
        "--plan".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge import jsr plan");

    let report = read_json_value(output_path);
    assert_eq!(report["ecosystem"], "jsr");
    assert_eq!(report["package_name"], "@std/path");
    assert_eq!(report["package_id"], "jsr/@std/path");
    assert_eq!(report["import_alias"], "@std/path");
    assert_eq!(report["origin"]["registry"], "jsr.io");
    assert_eq!(report["source_kind"], "registry-plan-review-required");
    assert_eq!(report["disposition"]["kind"], "bridge");
    assert_eq!(report["package_installs_run"], false);
    assert_eq!(report["lifecycle_scripts_executed"], false);
    assert!(report_artifact_path(dir.path(), &report, "import_plan_path").is_file());
    assert!(report_artifact_path(dir.path(), &report, "import_plan_sr_path").is_file());
    assert!(report_artifact_path(dir.path(), &report, "import_plan_machine_path").is_file());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_import_jsr_source_dir_materializes_reviewed_javascript_adapters() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("dx"), "project.name = forge-jsr-adapter\n").expect("config");
    let source_dir = dir.path().join(".dx/cache/jsr/std-path/package");
    fs::create_dir_all(&source_dir).expect("source dir");
    fs::write(
        source_dir.join("index.ts"),
        "export function join(...parts: string[]): string { return parts.join('/'); }\nexport default { join };\n",
    )
    .expect("jsr root source");
    fs::write(
        source_dir.join("join.ts"),
        "export function join(...parts: string[]): string { return parts.join('/'); }\n",
    )
    .expect("jsr join source");
    fs::write(
        source_dir.join("package.json"),
        r#"{"name":"@std/path","version":"0.0.0-forge-fixture","license":"MIT"}"#,
    )
    .expect("package metadata");
    fs::write(source_dir.join("LICENSE"), "MIT fixture license\n").expect("license");
    fs::write(
        source_dir.join("dx-forge-evidence.sr"),
        "integrity=true\nadvisory=true\npopularity=true\nlicense=MIT\n",
    )
    .expect("forge evidence");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let plan_path =
        write_reviewed_import_plan(&cli, dir.path(), "jsr", "@std/path", &source_dir, &[], 90);
    let output_path = dir.path().join("jsr-import-write.json");
    cli.cmd_forge(&[
        "import".to_string(),
        "jsr".to_string(),
        "@std/path".to_string(),
        "--write".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
        "--from-plan".to_string(),
        plan_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge import jsr write");

    assert!(dir.path().join("lib/forge/jsr/std-path/index.ts").exists());
    assert!(dir.path().join("lib/forge/jsr/std-path/join.ts").exists());
    assert!(!dir.path().join("node_modules").exists());

    let report = read_json_value(output_path);
    assert_eq!(report["ecosystem"], "jsr");
    assert_eq!(report["package_name"], "@std/path");
    assert_eq!(report["package_id"], "jsr/@std/path");
    assert_eq!(report["import_alias"], "@std/path");
    assert_eq!(report["materialized_package_id"], "jsr/@std/path");
    assert_eq!(report["materialized"], true);
    assert_eq!(report["package_installs_run"], false);
    assert_eq!(report["lifecycle_scripts_executed"], false);
    let reviewed_adapters = report["reviewed_adapters"]
        .as_array()
        .expect("reviewed adapters");
    assert!(reviewed_adapters.iter().any(|adapter| {
        adapter["specifier"] == "@std/path"
            && adapter["package_id"] == "jsr/@std/path"
            && adapter["materialized_path"] == "lib/forge/jsr/std-path/index.ts"
            && adapter["root"] == true
    }));
    assert!(reviewed_adapters.iter().any(|adapter| {
        adapter["specifier"] == "@std/path/join"
            && adapter["package_id"] == "jsr/@std/path"
            && adapter["materialized_path"] == "lib/forge/jsr/std-path/join.ts"
            && adapter["root"] == false
    }));
}

#[test]
fn forge_import_hex_and_cran_plans_are_non_executing_and_receipt_gated() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    for (ecosystem, package_name, package_id, registry) in [
        ("hex", "jason", "hex/jason", "hex.pm"),
        ("cran", "dplyr", "cran/dplyr", "cran.r-project.org"),
    ] {
        let output_path = dir
            .path()
            .join(format!("forge-import-{ecosystem}-{package_name}.json"));
        cli.cmd_forge(&[
            "import".to_string(),
            ecosystem.to_string(),
            package_name.to_string(),
            "--plan".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--output".to_string(),
            output_path.to_string_lossy().into_owned(),
            "--quiet".to_string(),
        ])
        .unwrap_or_else(|error| panic!("forge import {ecosystem} plan failed: {error}"));

        let report = read_json_value(output_path);
        assert_eq!(report["ecosystem"], ecosystem);
        assert_eq!(report["package_name"], package_name);
        assert_eq!(report["package_id"], package_id);
        assert_eq!(report["origin"]["registry"], registry);
        assert_eq!(report["source_kind"], "registry-plan-review-required");
        assert_eq!(report["disposition"]["kind"], "bridge");
        assert_eq!(report["package_installs_run"], false);
        assert_eq!(report["lifecycle_scripts_executed"], false);
        assert!(report_artifact_path(dir.path(), &report, "import_plan_path").is_file());
        assert!(report_artifact_path(dir.path(), &report, "import_plan_sr_path").is_file());
        assert!(report_artifact_path(dir.path(), &report, "import_plan_machine_path").is_file());
    }
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_import_npm_large_dependency_graph_is_visible_and_capped() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("dx"), "project.name = forge-large-graph\n").expect("config");
    let source_dir = dir.path().join(".dx/cache/npm/large-graph/package");
    fs::create_dir_all(&source_dir).expect("source dir");
    fs::write(
        source_dir.join("index.ts"),
        "export const largeGraphFixture = 'reviewed source';\n",
    )
    .expect("source");
    fs::write(source_dir.join("LICENSE"), "MIT fixture license\n").expect("license");
    fs::write(
        source_dir.join("dx-forge-evidence.sr"),
        "integrity=true\nadvisory=true\npopularity=true\nlicense=MIT\n",
    )
    .expect("forge evidence");
    let dependencies = (0..49)
        .map(|index| format!(r#""dep-{index}":"1.0.0""#))
        .collect::<Vec<_>>()
        .join(",");
    fs::write(
        source_dir.join("package.json"),
        format!(
            r#"{{"name":"large-graph","version":"0.0.0-forge-fixture","license":"MIT","dependencies":{{{dependencies}}}}}"#
        ),
    )
    .expect("package metadata");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("large-graph-plan.json");
    cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "large-graph".to_string(),
        "--plan".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge import npm large graph plan");

    let report = read_json_value(output_path);
    assert_eq!(report["source_dependency_count"], 49);
    assert_eq!(report["score_ceiling"], 72);
    assert!(report["risk_flags"]
        .as_array()
        .expect("risk flags")
        .iter()
        .any(|risk| risk == "huge-dependency-graph"));
    assert!(report["applied_caps"]
        .as_array()
        .expect("applied caps")
        .iter()
        .any(|cap| cap["id"] == "large-dependency-graph"));
    assert!(report_artifact_path(dir.path(), &report, "import_plan_sr_path").is_file());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_import_plan_explicit_fail_under_returns_error_with_evidence() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        "project.name = forge-plan-threshold\n",
    )
    .expect("config");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("npm-react-plan.json");

    let result = cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "react".to_string(),
        "--plan".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "100".to_string(),
        "--quiet".to_string(),
    ]);

    assert!(result.is_err());
    let report = read_json_value(output_path);
    assert_eq!(report["passed"], false);
    assert_eq!(report["mode"], "plan-only");
    assert!(report_artifact_path(dir.path(), &report, "import_plan_path").is_file());
    assert!(report_artifact_path(dir.path(), &report, "import_plan_sr_path").is_file());
    assert!(report_artifact_path(dir.path(), &report, "import_plan_machine_path").is_file());
}

#[test]
fn forge_import_npm_source_dir_materializes_lodash_without_node_modules() {
    let dir = tempdir().expect("tempdir");
    fs::write(dir.path().join("dx"), "project.name = forge-npm-adapter\n").expect("config");
    for path in ["app", "components/local", "server", "styles"] {
        fs::create_dir_all(dir.path().join(path)).expect("project dir");
    }
    fs::write(
            dir.path().join("app/page.tsx"),
            "import { Hero } from '../components/local/Hero';\nexport default function Page() { return <Hero />; }\n",
        )
        .expect("page");
    fs::write(
        dir.path().join("components/local/Hero.tsx"),
        "export function Hero() { return <section />; }\n",
    )
    .expect("hero");
    let source_dir = dir.path().join(".dx/cache/npm/lodash/package");
    fs::create_dir_all(&source_dir).expect("source dir");
    fs::write(
        source_dir.join("index.ts"),
        "export function startCase(value: string): string { return value.replace(/(^|\\s)\\S/g, (part) => part.toUpperCase()); }\nexport default { startCase };\n",
    )
    .expect("lodash source");
    fs::write(
        source_dir.join("fp.ts"),
        "export function flow<T>(value: T): T { return value; }\nexport default { flow };\n",
    )
    .expect("lodash fp source");
    fs::write(
        source_dir.join("package.json"),
        r#"{"name":"lodash","version":"0.0.0-forge-fixture","license":"MIT"}"#,
    )
    .expect("package metadata");
    fs::write(source_dir.join("LICENSE"), "MIT fixture license\n").expect("license");
    fs::write(
        source_dir.join("dx-forge-evidence.sr"),
        "integrity=true\nadvisory=true\npopularity=true\nlicense=MIT\n",
    )
    .expect("forge evidence");
    fs::write(
        dir.path().join("app/page.tsx"),
        "import { startCase } from 'lodash';\nimport { flow } from 'lodash/fp';\nexport default function Page() { return <main data-forge-package=\"npm/lodash\">Lodash Forge proof</main>; }\n",
    )
    .expect("page");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let plan_path =
        write_reviewed_import_plan(&cli, dir.path(), "npm", "lodash", &source_dir, &[], 90);
    let output_path = dir.path().join("npm-import-write.json");
    cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "lodash".to_string(),
        "--write".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
        "--from-plan".to_string(),
        plan_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge import npm write");

    let materialized_path = dir.path().join("lib/forge/npm/lodash/index.ts");
    assert!(materialized_path.exists());
    let materialized_subpath = dir.path().join("lib/forge/npm/lodash/fp.ts");
    assert!(materialized_subpath.exists());
    assert!(!dir.path().join("node_modules").exists());
    assert!(dir
        .path()
        .join(".dx/forge/import-plans/npm-lodash.json")
        .exists());
    assert!(dir.path().join(".dx/forge/docs/npm-lodash.md").exists());

    let report = read_json_value(output_path);
    assert_eq!(report["ecosystem"], "npm");
    assert_eq!(report["package_name"], "lodash");
    assert_eq!(report["mode"], "write");
    assert_eq!(report["materialized"], true);
    assert_eq!(report["materialized_package_id"], "npm/lodash");
    assert_eq!(report["import_alias"], "lodash");
    assert_eq!(report["source_kind"], "external-source-snapshot-ready");
    assert_eq!(report["source_dir_ready"], true);
    assert_eq!(report["disposition"]["kind"], "materialize");
    assert_eq!(report["disposition"]["route"], "materialize-source-owned");
    assert_eq!(report["disposition"]["ownership_claim"], "source-owned");
    assert_eq!(report["disposition"]["importable_source"], true);
    assert_eq!(
        report["materialization_status"],
        "materialized-source-owned-adapter"
    );
    assert_eq!(report["package_installs_run"], false);
    assert_eq!(report["lifecycle_scripts_executed"], false);
    assert!(report_artifact_path(dir.path(), &report, "import_plan_path").is_file());
    assert!(report_artifact_path(dir.path(), &report, "import_plan_sr_path").is_file());
    assert!(report_artifact_path(dir.path(), &report, "import_plan_machine_path").is_file());
    assert!(report["materialized_files"]
        .as_array()
        .expect("materialized files")
        .iter()
        .any(|path| path == "lib/forge/npm/lodash/index.ts"));
    let reviewed_adapters = report["reviewed_adapters"]
        .as_array()
        .expect("reviewed adapters");
    assert!(reviewed_adapters.iter().any(|adapter| {
        adapter["specifier"] == "lodash"
            && adapter["package_id"] == "npm/lodash"
            && adapter["materialized_path"] == "lib/forge/npm/lodash/index.ts"
            && adapter["root"] == true
    }));
    assert!(reviewed_adapters.iter().any(|adapter| {
        adapter["specifier"] == "lodash/fp"
            && adapter["package_id"] == "npm/lodash"
            && adapter["materialized_path"] == "lib/forge/npm/lodash/fp.ts"
            && adapter["root"] == false
    }));

    let manifest = read_json_value(dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json"));
    assert!(manifest["packages"]
        .as_array()
        .expect("packages")
        .iter()
        .any(|package| package["package_id"] == "npm/lodash"
            && package["upstream_name"] == "npm:lodash"
            && package["files"]
                .as_array()
                .expect("files")
                .iter()
                .any(|file| file["path"] == "lib/forge/npm/lodash/index.ts")
            && package["files"]
                .as_array()
                .expect("files")
                .iter()
                .any(|file| file["path"] == "lib/forge/npm/lodash/fp.ts")));
    cli.cmd_imports(&["sync".to_string()])
        .expect("imports sync");
    cli.cmd_build().expect("dx build");
    let import_resolution =
        read_json_value(dir.path().join(".dx/www/output/.dx/build-cache/import-resolution.json"));
    assert!(import_resolution
        .as_array()
        .expect("import resolutions")
        .iter()
        .any(|resolution| resolution["specifier"] == "lodash"
            && resolution["kind"] == "reviewed-adapter"
            && resolution["package_id"] == "npm/lodash"
            && resolution["resolved_path"] == "lib/forge/npm/lodash/index.ts"
            && resolution["requires_node_modules"] == false));
    assert!(import_resolution
        .as_array()
        .expect("import resolutions")
        .iter()
        .any(|resolution| resolution["specifier"] == "lodash/fp"
            && resolution["kind"] == "reviewed-adapter"
            && resolution["package_id"] == "npm/lodash"
            && resolution["resolved_path"] == "lib/forge/npm/lodash/fp.ts"
            && resolution["requires_node_modules"] == false));

    cli.cmd_check(&[
        ".".to_string(),
        "--strict-project-contract".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--fail-under".to_string(),
        "1".to_string(),
    ])
    .expect("strict project contract check");
}

#[test]
fn forge_import_npm_selected_subpath_does_not_invent_root_adapter() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        "project.name = forge-npm-subpath-adapter\n",
    )
    .expect("config");
    fs::create_dir_all(dir.path().join("app")).expect("app dir");
    let source_dir = dir.path().join(".dx/cache/npm/lodash/package");
    fs::create_dir_all(&source_dir).expect("source dir");
    fs::write(
        source_dir.join("index.ts"),
        "export function startCase(value: string): string { return value; }\n",
    )
    .expect("lodash root source");
    fs::write(
        source_dir.join("fp.ts"),
        "export function flow<T>(value: T): T { return value; }\nexport default { flow };\n",
    )
    .expect("lodash fp source");
    fs::write(
        source_dir.join("package.json"),
        r#"{"name":"lodash","version":"0.0.0-forge-fixture","license":"MIT"}"#,
    )
    .expect("package metadata");
    fs::write(source_dir.join("LICENSE"), "MIT fixture license\n").expect("license");
    fs::write(
        source_dir.join("dx-forge-evidence.sr"),
        "integrity=true\nadvisory=true\npopularity=true\nlicense=MIT\n",
    )
    .expect("forge evidence");
    fs::write(
        dir.path().join("app/page.tsx"),
        "import { flow } from 'lodash/fp';\nexport default function Page() { return <main>{flow('reviewed')}</main>; }\n",
    )
    .expect("page");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let plan_path =
        write_reviewed_import_plan(&cli, dir.path(), "npm", "lodash", &source_dir, &["fp.ts"], 90);
    cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "lodash".to_string(),
        "--write".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
        "--file".to_string(),
        "fp.ts".to_string(),
        "--from-plan".to_string(),
        plan_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--quiet".to_string(),
    ])
    .expect("forge import selected npm subpath write");

    assert!(!dir.path().join("lib/forge/npm/lodash/index.ts").exists());
    assert!(dir.path().join("lib/forge/npm/lodash/fp.ts").exists());
    cli.cmd_imports(&["sync".to_string()])
        .expect("imports sync");
    cli.cmd_build().expect("dx build");

    let import_resolution =
        read_json_value(dir.path().join(".dx/www/output/.dx/build-cache/import-resolution.json"));
    let resolutions = import_resolution
        .as_array()
        .expect("import resolutions");
    assert!(resolutions.iter().any(|resolution| resolution["specifier"] == "lodash/fp"
        && resolution["kind"] == "reviewed-adapter"
        && resolution["package_id"] == "npm/lodash"
        && resolution["resolved_path"] == "lib/forge/npm/lodash/fp.ts"
        && resolution["requires_node_modules"] == false));
    assert!(resolutions
        .iter()
        .all(|resolution| resolution["specifier"] != "lodash"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_import_write_accepts_reviewed_import_plan_before_materializing() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        "project.name = forge-accepted-plan\n",
    )
    .expect("config");
    let source_dir = write_reviewed_npm_import_fixture(dir.path(), "accepted-plan-tool");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let plan_path = dir.path().join("accepted-plan-tool-plan.json");

    cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "accepted-plan-tool".to_string(),
        "--plan".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        plan_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge import npm accepted plan");

    let output_path = dir.path().join("accepted-plan-tool-write.json");
    cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "accepted-plan-tool".to_string(),
        "--write".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
        "--from-plan".to_string(),
        plan_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge import npm write from accepted plan");

    let report = read_json_value(output_path);
    assert_eq!(report["materialized"], true);
    assert_eq!(report["accepted_plan_status"], "validated");
    assert_eq!(
        report["accepted_plan_path"],
        serde_json::Value::String(plan_path.to_string_lossy().into_owned())
    );
    assert!(dir
        .path()
        .join("lib/forge/npm/accepted-plan-tool/index.ts")
        .exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_import_write_rejects_stale_accepted_plan_when_source_hash_changes() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        "project.name = forge-stale-accepted-plan\n",
    )
    .expect("config");
    let source_dir = write_reviewed_npm_import_fixture(dir.path(), "stale-plan-tool");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let plan_path = dir.path().join("stale-plan-tool-plan.json");

    cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "stale-plan-tool".to_string(),
        "--plan".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        plan_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge import npm stale plan");

    fs::write(
        source_dir.join("index.ts"),
        "export const forgeImportFixture = 'mutated after plan';\n",
    )
    .expect("mutated source");

    let output_path = dir.path().join("stale-plan-tool-write.json");
    let result = cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "stale-plan-tool".to_string(),
        "--write".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
        "--from-plan".to_string(),
        plan_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ]);

    assert!(result.is_err());
    assert!(!dir
        .path()
        .join("lib/forge/npm/stale-plan-tool/index.ts")
        .exists());
    assert!(!dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json").exists());

    let report = read_json_value(output_path);
    assert_eq!(report["mode"], "write");
    assert_eq!(report["materialized"], false);
    assert_eq!(report["accepted_plan_status"], "mismatch");
    assert!(report["accepted_plan_findings"]
        .as_array()
        .expect("accepted plan findings")
        .iter()
        .any(|finding| finding
            .as_str()
            .unwrap_or_default()
            .contains("source snapshot hash")));
}

#[test]
fn forge_import_write_requires_reviewed_plan_before_materializing() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        "project.name = forge-plan-required\n",
    )
    .expect("config");
    let source_dir = write_reviewed_npm_import_fixture(dir.path(), "plan-required-tool");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("plan-required-tool-write.json");

    let result = cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "plan-required-tool".to_string(),
        "--write".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ]);

    assert!(result.is_err());
    assert!(!dir
        .path()
        .join("lib/forge/npm/plan-required-tool/index.ts")
        .exists());
    assert!(!dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json").exists());

    let report = read_json_value(output_path);
    assert_eq!(report["mode"], "write");
    assert_eq!(report["materialized"], false);
    assert_eq!(report["materialization_ready"], false);
    assert_eq!(report["accepted_plan_status"], "missing-reviewed-plan");
    assert!(report["findings"]
        .as_array()
        .expect("findings")
        .iter()
        .any(|finding| finding.as_str().unwrap_or_default().contains("--from-plan")));
}

#[test]
fn forge_import_write_rolls_back_source_files_when_receipt_dir_is_blocked() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        "project.name = forge-receipt-rollback\n",
    )
    .expect("config");
    let source_dir = write_reviewed_npm_import_fixture(dir.path(), "rollback-tool");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let plan_path = write_reviewed_import_plan(
        &cli,
        dir.path(),
        "npm",
        "rollback-tool",
        &source_dir,
        &[],
        90,
    );
    fs::create_dir_all(dir.path().join(".dx/forge")).expect("forge dir");
    fs::write(
        dir.path().join(".dx/forge/receipts"),
        "blocks receipt directory creation\n",
    )
    .expect("receipt blocker");

    let result = cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "rollback-tool".to_string(),
        "--write".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
        "--from-plan".to_string(),
        plan_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ]);

    assert!(result.is_err());
    assert!(!dir
        .path()
        .join("lib/forge/npm/rollback-tool/index.ts")
        .exists());
    assert!(!dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json").exists());
    assert!(!dir
        .path()
        .join(".dx/forge/docs/npm-rollback-tool.md")
        .exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_import_write_rolls_back_state_when_import_plan_artifacts_fail() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        "project.name = forge-import-plan-rollback\n",
    )
    .expect("config");
    let source_dir = write_reviewed_npm_import_fixture(dir.path(), "plan-blocked");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let plan_path = write_reviewed_import_plan(
        &cli,
        dir.path(),
        "npm",
        "plan-blocked",
        &source_dir,
        &[],
        90,
    );
    fs::remove_dir_all(dir.path().join(".dx/forge/import-plans")).expect("remove plan dir");
    fs::write(
        dir.path().join(".dx/forge/import-plans"),
        "blocks import-plan artifact directory creation\n",
    )
    .expect("import plan blocker");

    let result = cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "plan-blocked".to_string(),
        "--write".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
        "--from-plan".to_string(),
        plan_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ]);

    assert!(result.is_err());
    assert!(!dir
        .path()
        .join("lib/forge/npm/plan-blocked/index.ts")
        .exists());
    assert!(!dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json").exists());
    assert!(!dir
        .path()
        .join(".dx/forge/docs/npm-plan-blocked.md")
        .exists());
    assert!(!dir
        .path()
        .join(".dx/forge/import-plans/npm-plan-blocked.json")
        .exists());
    let receipt_dir = dir.path().join(".dx/forge/receipts");
    assert!(
        !receipt_dir.exists()
            || fs::read_dir(&receipt_dir)
                .expect("receipt dir")
                .next()
                .is_none()
    );
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_import_output_preflight_runs_before_source_materialization() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        "project.name = forge-output-preflight\n",
    )
    .expect("config");
    let source_dir = write_reviewed_npm_import_fixture(dir.path(), "output-blocked");
    let output_dir = dir.path().join("forge-report-output");
    fs::create_dir_all(&output_dir).expect("output dir");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let result = cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "output-blocked".to_string(),
        "--write".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_dir.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ]);

    assert!(result.is_err());
    assert!(!dir
        .path()
        .join("lib/forge/npm/output-blocked/index.ts")
        .exists());
    assert!(!dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json").exists());
    assert!(!dir
        .path()
        .join(".dx/forge/import-plans/npm-output-blocked.json")
        .exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_import_www_conversion_packages_materialize_reviewed_adapters() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        "project.name = forge-www-conversion-packages\n",
    )
    .expect("config");
    fs::create_dir_all(dir.path().join("app/package-proof")).expect("app dir");
    fs::write(
        dir.path().join("app/package-proof/page.tsx"),
        conversion_package_page_source(),
    )
    .expect("page");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let packages = conversion_package_fixtures();
    for package in &packages {
        let source_dir = write_conversion_package_source(dir.path(), package);
        let plan_path =
            write_reviewed_import_plan(&cli, dir.path(), "npm", package.name, &source_dir, &[], 90);
        let output_path = dir
            .path()
            .join(format!("forge-import-{}.json", package.slug));
        cli.cmd_forge(&[
            "import".to_string(),
            "npm".to_string(),
            package.name.to_string(),
            "--write".to_string(),
            "--source-dir".to_string(),
            source_dir.to_string_lossy().into_owned(),
            "--from-plan".to_string(),
            plan_path.to_string_lossy().into_owned(),
            "--format".to_string(),
            "json".to_string(),
            "--output".to_string(),
            output_path.to_string_lossy().into_owned(),
            "--quiet".to_string(),
            "--fail-under".to_string(),
            "90".to_string(),
        ])
        .unwrap_or_else(|error| panic!("forge import {} failed: {error}", package.name));

        let report = read_json_value(output_path);
        assert_eq!(report["passed"], true, "package {}", package.name);
        assert_eq!(report["mode"], "write");
        assert_eq!(report["materialized"], true);
        assert_eq!(report["package_installs_run"], false);
        assert_eq!(report["lifecycle_scripts_executed"], false);
        assert_eq!(report["import_alias"], package.name);
        assert_eq!(report["materialized_package_id"], package.package_id);
        assert!(report_artifact_path(dir.path(), &report, "import_plan_path").is_file());
        assert!(report_artifact_path(dir.path(), &report, "import_plan_sr_path").is_file());
        assert!(report_artifact_path(dir.path(), &report, "import_plan_machine_path").is_file());
    }
    let bridge_package = conversion_bridge_package_fixture();
    let bridge_source_dir = write_conversion_package_source(dir.path(), &bridge_package);
    let bridge_output_path = dir
        .path()
        .join(format!("forge-import-{}.json", bridge_package.slug));
    let bridge_result = cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        bridge_package.name.to_string(),
        "--write".to_string(),
        "--source-dir".to_string(),
        bridge_source_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        bridge_output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
        "--fail-under".to_string(),
        "90".to_string(),
    ]);
    assert!(
        bridge_result.is_err(),
        "native runtime package should require a Forge bridge"
    );
    let bridge_report = read_json_value(bridge_output_path);
    assert_eq!(bridge_report["mode"], "write");
    assert_eq!(bridge_report["materialized"], false);
    assert_eq!(bridge_report["disposition"]["kind"], "bridge");
    assert_eq!(
        bridge_report["disposition"]["route"],
        "bridge-native-runtime"
    );
    assert_eq!(
        bridge_report["materialized_package_id"],
        serde_json::Value::Null
    );
    assert!(report_artifact_path(dir.path(), &bridge_report, "import_plan_path").is_file());
    assert!(report_artifact_path(dir.path(), &bridge_report, "import_plan_sr_path").is_file());
    assert!(report_artifact_path(dir.path(), &bridge_report, "import_plan_machine_path").is_file());
    assert!(!dir
        .path()
        .join(format!("lib/forge/npm/{}/index.ts", bridge_package.slug))
        .exists());

    let manifest = read_json_value(dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json"));
    let manifest_packages = manifest["packages"].as_array().expect("packages");
    for package in &packages {
        let materialized_path = format!("lib/forge/npm/{}/index.ts", package.slug);
        assert!(
            dir.path().join(&materialized_path).is_file(),
            "materialized source missing for {}",
            package.name
        );
        assert!(
            manifest_packages.iter().any(|entry| {
                entry["package_id"] == package.package_id
                    && entry["source_kind"] == "npm-snapshot"
                    && entry["files"]
                        .as_array()
                        .expect("files")
                        .iter()
                        .any(|file| file["path"] == materialized_path)
            }),
            "source manifest missing package {}",
            package.name
        );
        assert!(
            dir.path()
                .join(format!(
                    ".dx/forge/docs/{}.md",
                    package.package_id.replace('/', "-")
                ))
                .is_file(),
            "package docs missing for {}",
            package.name
        );
    }
    assert!(manifest["receipts"].as_array().expect("receipts").len() >= packages.len());
    assert!(!dir.path().join("node_modules").exists());

    let config = Cli::react_import_resolver_config(dir.path());
    let page_source =
        fs::read_to_string(dir.path().join("app/package-proof/page.tsx")).expect("page source");
    let resolutions = dx_compiler::delivery::resolve_react_imports(
        "app/package-proof/page.tsx",
        &page_source,
        config,
    );
    for package in &packages {
        let materialized_path = format!("lib/forge/npm/{}/index.ts", package.slug);
        assert!(
            resolutions.iter().any(|resolution| {
                resolution.specifier == package.name
                    && resolution.kind
                        == dx_compiler::delivery::DxReactImportResolutionKind::ReviewedAdapter
                    && resolution.package_id.as_deref() == Some(package.package_id)
                    && resolution.resolved_path.as_deref() == Some(materialized_path.as_str())
                    && !resolution.requires_node_modules
            }),
            "import resolver did not review {} through Forge: {resolutions:#?}",
            package.name
        );
    }
}

#[test]
fn forge_import_popular_ecosystems_materialize_reviewed_source_snapshots() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        "project.name = forge-popular-ecosystems\n",
    )
    .expect("config");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let packages = popular_ecosystem_import_fixtures();
    for package in &packages {
        let source_dir = write_popular_ecosystem_source(dir.path(), package);
        let plan_path = write_reviewed_import_plan(
            &cli,
            dir.path(),
            package.ecosystem,
            package.name,
            &source_dir,
            &[],
            90,
        );
        let output_path = dir.path().join(format!(
            "forge-import-{}-{}.json",
            package.ecosystem, package.slug
        ));
        cli.cmd_forge(&[
            "import".to_string(),
            package.ecosystem.to_string(),
            package.name.to_string(),
            "--write".to_string(),
            "--source-dir".to_string(),
            source_dir.to_string_lossy().into_owned(),
            "--from-plan".to_string(),
            plan_path.to_string_lossy().into_owned(),
            "--format".to_string(),
            "json".to_string(),
            "--output".to_string(),
            output_path.to_string_lossy().into_owned(),
            "--quiet".to_string(),
            "--fail-under".to_string(),
            "90".to_string(),
        ])
        .unwrap_or_else(|error| {
            panic!(
                "forge import {} package {} failed: {error}",
                package.ecosystem, package.name
            )
        });

        let report = read_json_value(output_path);
        assert_eq!(report["passed"], true, "package {}", package.name);
        assert_eq!(report["mode"], "write");
        assert_eq!(report["ecosystem"], package.ecosystem);
        assert_eq!(report["materialized"], true);
        assert_eq!(report["package_installs_run"], false);
        assert_eq!(report["lifecycle_scripts_executed"], false);
        assert_eq!(report["materialized_package_id"], package.package_id);
        assert_eq!(report["origin"]["registry"], package.registry);
        assert!(report_artifact_path(dir.path(), &report, "import_plan_path").is_file());
        assert!(report_artifact_path(dir.path(), &report, "import_plan_sr_path").is_file());
        assert!(report_artifact_path(dir.path(), &report, "import_plan_machine_path").is_file());

        let materialized_path = format!(
            "lib/forge/{}/{}/{}",
            package.ecosystem, package.slug, package.source_path
        );
        assert!(
            dir.path().join(&materialized_path).is_file(),
            "materialized source missing for {}",
            package.name
        );
    }

    let manifest = read_json_value(dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json"));
    let manifest_packages = manifest["packages"].as_array().expect("packages");
    for package in &packages {
        let materialized_path = format!(
            "lib/forge/{}/{}/{}",
            package.ecosystem, package.slug, package.source_path
        );
        assert!(
            manifest_packages.iter().any(|entry| {
                entry["package_id"] == package.package_id
                    && entry["source_kind"] == "external-snapshot"
                    && entry["generator"]
                        == format!("dx-forge/{}-external-source-snapshot", package.ecosystem)
                    && entry["files"]
                        .as_array()
                        .expect("files")
                        .iter()
                        .any(|file| file["path"] == materialized_path)
            }),
            "source manifest missing package {}",
            package.name
        );
        assert!(
            dir.path()
                .join(format!(
                    ".dx/forge/docs/{}.md",
                    package.package_id.replace('/', "-")
                ))
                .is_file(),
            "package docs missing for {}",
            package.name
        );
    }
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_import_python_rust_go_discovers_public_exports_from_reviewed_snapshots() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        "project.name = forge-python-rust-go-exports\n",
    )
    .expect("config");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let packages = python_rust_go_export_fixtures();
    for package in &packages {
        let source_dir = write_python_rust_go_export_source(dir.path(), package);
        let plan_path = write_reviewed_import_plan(
            &cli,
            dir.path(),
            package.ecosystem,
            package.name,
            &source_dir,
            &[],
            90,
        );
        let output_path = dir.path().join(format!(
            "forge-import-{}-{}.json",
            package.ecosystem, package.slug
        ));
        cli.cmd_forge(&[
            "import".to_string(),
            package.ecosystem.to_string(),
            package.name.to_string(),
            "--write".to_string(),
            "--source-dir".to_string(),
            source_dir.to_string_lossy().into_owned(),
            "--from-plan".to_string(),
            plan_path.to_string_lossy().into_owned(),
            "--format".to_string(),
            "json".to_string(),
            "--output".to_string(),
            output_path.to_string_lossy().into_owned(),
            "--quiet".to_string(),
            "--fail-under".to_string(),
            "90".to_string(),
        ])
        .unwrap_or_else(|error| {
            panic!(
                "forge import {} package {} failed: {error}",
                package.ecosystem, package.name
            )
        });

        let report = read_json_value(output_path);
        assert_eq!(report["passed"], true, "package {}", package.name);
        assert_eq!(report["materialized"], true, "package {}", package.name);
        assert!(
            report["source_files_inspected_count"]
                .as_u64()
                .unwrap_or_default()
                >= 2,
            "package {} should inspect package source plus metadata evidence",
            package.name
        );
        let export_names = report["exports"]
            .as_array()
            .expect("exports")
            .iter()
            .map(|export| export["name"].as_str().expect("export name"))
            .collect::<Vec<_>>();
        for expected_export in package.expected_exports {
            assert!(
                export_names.contains(expected_export),
                "package {} missing export {expected_export}: {export_names:?}",
                package.name
            );
        }
        assert!(
            !export_names.contains(&package.private_export),
            "package {} leaked private export {}",
            package.name,
            package.private_export
        );
        assert!(!dir.path().join("node_modules").exists());
    }
}

fn write_reviewed_npm_import_fixture(
    project: &std::path::Path,
    package_name: &str,
) -> std::path::PathBuf {
    let source_dir = project
        .join(".dx/cache/npm")
        .join(package_name)
        .join("package");
    fs::create_dir_all(&source_dir).expect("source dir");
    fs::write(
        source_dir.join("index.ts"),
        "export const forgeImportFixture = 'reviewed source';\nexport default { forgeImportFixture };\n",
    )
    .expect("source");
    fs::write(
        source_dir.join("package.json"),
        format!(r#"{{"name":"{package_name}","version":"0.0.0-forge-fixture","license":"MIT"}}"#),
    )
    .expect("package metadata");
    fs::write(source_dir.join("LICENSE"), "MIT fixture license\n").expect("license");
    fs::write(
        source_dir.join("dx-forge-evidence.sr"),
        "integrity=true\nadvisory=true\npopularity=true\nlicense=MIT\n",
    )
    .expect("forge evidence");
    source_dir
}

fn write_reviewed_import_plan(
    cli: &Cli,
    project: &std::path::Path,
    ecosystem: &str,
    package_name: &str,
    source_dir: &std::path::Path,
    selected_files: &[&str],
    fail_under: u8,
) -> std::path::PathBuf {
    let plan_path = project.join(format!(
        "forge-import-{}-{}-reviewed-plan.json",
        ecosystem,
        forge_import_test_slug(package_name)
    ));
    let mut args = vec![
        "import".to_string(),
        ecosystem.to_string(),
        package_name.to_string(),
        "--plan".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
    ];
    for selected_file in selected_files {
        args.push("--file".to_string());
        args.push((*selected_file).to_string());
    }
    args.extend([
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        plan_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
        "--fail-under".to_string(),
        fail_under.to_string(),
    ]);
    cli.cmd_forge(&args).unwrap_or_else(|error| {
        panic!("forge import plan for {ecosystem} package {package_name} failed: {error}")
    });
    plan_path
}

fn forge_import_test_slug(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn npm_integrity_sha512(bytes: &[u8]) -> String {
    use base64::Engine as _;
    use sha2::Digest as _;

    let digest = sha2::Sha512::digest(bytes);
    format!(
        "sha512-{}",
        base64::engine::general_purpose::STANDARD.encode(digest)
    )
}

fn start_mock_npm_registry(
    package_name: &str,
    version: &str,
    tarball: Vec<u8>,
    integrity: &str,
) -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("mock npm registry");
    let base_url = format!("http://{}", listener.local_addr().expect("registry addr"));
    let package_path = format!("/{package_name}");
    let tarball_path = format!("/{package_name}/-/{package_name}-{version}.tgz");
    let packument = format!(
        r#"{{
  "name": "{package_name}",
  "dist-tags": {{"latest": "{version}"}},
  "versions": {{
    "{version}": {{
      "name": "{package_name}",
      "version": "{version}",
      "license": "MIT",
      "dist": {{
        "tarball": "{base_url}{tarball_path}",
        "integrity": "{integrity}"
      }}
    }}
  }}
}}"#
    );
    let tarball_response = tarball.clone();

    std::thread::spawn(move || {
        for _ in 0..2 {
            let Ok((mut stream, _)) = listener.accept() else {
                return;
            };
            let mut buffer = [0_u8; 4096];
            let Ok(read) = std::io::Read::read(&mut stream, &mut buffer) else {
                return;
            };
            let request = String::from_utf8_lossy(&buffer[..read]);
            let path = request.split_whitespace().nth(1).unwrap_or("/");
            if path == package_path {
                write_mock_http_response(&mut stream, "application/json", packument.as_bytes());
            } else if path == tarball_path {
                write_mock_http_response(
                    &mut stream,
                    "application/octet-stream",
                    &tarball_response,
                );
            } else {
                write_mock_http_response(&mut stream, "text/plain", b"not found");
            }
        }
    });

    base_url
}

fn write_mock_http_response(
    stream: &mut std::net::TcpStream,
    content_type: &str,
    body: &[u8],
) {
    let header = format!(
        "HTTP/1.1 200 OK\r\ncontent-type: {content_type}\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
        body.len()
    );
    std::io::Write::write_all(stream, header.as_bytes()).expect("mock header");
    std::io::Write::write_all(stream, body).expect("mock body");
}

fn npm_tgz_fixture(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let mut tar = Vec::new();
    for (path, bytes) in entries {
        append_tar_file(&mut tar, path, bytes);
    }
    tar.extend_from_slice(&[0_u8; 1024]);

    let mut encoder =
        flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    std::io::Write::write_all(&mut encoder, &tar).expect("write gzip tar");
    encoder.finish().expect("finish gzip tar")
}

fn append_tar_file(output: &mut Vec<u8>, path: &str, content: &[u8]) {
    assert!(path.len() <= 100, "test tar path too long: {path}");
    let mut header = [0_u8; 512];
    header[..path.len()].copy_from_slice(path.as_bytes());
    write_tar_octal(&mut header[100..108], 0o644);
    write_tar_octal(&mut header[108..116], 0);
    write_tar_octal(&mut header[116..124], 0);
    write_tar_octal(&mut header[124..136], content.len() as u64);
    write_tar_octal(&mut header[136..148], 0);
    header[148..156].fill(b' ');
    header[156] = b'0';
    header[257..263].copy_from_slice(b"ustar\0");
    header[263..265].copy_from_slice(b"00");
    let checksum = header.iter().map(|byte| u32::from(*byte)).sum::<u32>();
    write_tar_checksum(&mut header[148..156], checksum);

    output.extend_from_slice(&header);
    output.extend_from_slice(content);
    let padding = (512 - (content.len() % 512)) % 512;
    output.extend(std::iter::repeat(0_u8).take(padding));
}

fn write_tar_octal(field: &mut [u8], value: u64) {
    field.fill(0);
    let text = format!("{value:0width$o}", width = field.len() - 1);
    field[..text.len()].copy_from_slice(text.as_bytes());
}

fn write_tar_checksum(field: &mut [u8], value: u32) {
    field.fill(b' ');
    let text = format!("{value:06o}\0 ");
    field[..text.len()].copy_from_slice(text.as_bytes());
}

struct ForgeConversionPackageFixture {
    name: &'static str,
    package_id: &'static str,
    slug: &'static str,
    source: &'static str,
}

fn conversion_package_fixtures() -> Vec<ForgeConversionPackageFixture> {
    vec![
        ForgeConversionPackageFixture {
            name: "three",
            package_id: "npm/three",
            slug: "three",
            source: "export class Vector3 { constructor(public x = 0, public y = 0, public z = 0) {} toArray() { return [this.x, this.y, this.z]; } }\nexport class Color { constructor(public value = '#ffffff') {} }\nexport default { Vector3, Color };\n",
        },
        ForgeConversionPackageFixture {
            name: "xlsx",
            package_id: "npm/xlsx",
            slug: "xlsx",
            source: "export const utils = { book_new: () => ({ SheetNames: [], Sheets: {} }), aoa_to_sheet: (rows: unknown[][]) => ({ rows }) };\nexport function read(bytes: Uint8Array) { return { bytes, SheetNames: [], Sheets: {} }; }\nexport function writeFile(workbook: unknown, filename: string) { return { workbook, filename }; }\nexport default { utils, read, writeFile };\n",
        },
        ForgeConversionPackageFixture {
            name: "pptxgenjs",
            package_id: "npm/pptxgenjs",
            slug: "pptxgenjs",
            source: "export default class PptxGenJS { slides: unknown[] = []; addSlide() { const slide = { shapes: [] as unknown[] }; this.slides.push(slide); return slide; } writeFile() { return Promise.resolve({ slides: this.slides.length }); } }\n",
        },
        ForgeConversionPackageFixture {
            name: "jszip",
            package_id: "npm/jszip",
            slug: "jszip",
            source: "export default class JSZip { private files = new Map<string, string>(); file(name: string, content: string) { this.files.set(name, content); return this; } async generateAsync() { return Array.from(this.files.entries()); } }\n",
        },
        ForgeConversionPackageFixture {
            name: "fabric",
            package_id: "npm/fabric",
            slug: "fabric",
            source: "export class Canvas { objects: unknown[] = []; constructor(public element: string) {} add(object: unknown) { this.objects.push(object); return this; } }\nexport const fabric = { Canvas };\nexport default fabric;\n",
        },
        ForgeConversionPackageFixture {
            name: "konva",
            package_id: "npm/konva",
            slug: "konva",
            source: "export class Stage { constructor(public options: Record<string, unknown>) {} }\nexport class Layer { children: unknown[] = []; add(node: unknown) { this.children.push(node); return this; } }\nexport class Rect { constructor(public options: Record<string, unknown>) {} }\nexport default { Stage, Layer, Rect };\n",
        },
        ForgeConversionPackageFixture {
            name: "fflate",
            package_id: "npm/fflate",
            slug: "fflate",
            source: "export function zipSync(files: Record<string, Uint8Array>) { return new Uint8Array(Object.keys(files).length); }\nexport function unzipSync(bytes: Uint8Array) { return { bytes }; }\nexport default { zipSync, unzipSync };\n",
        },
        ForgeConversionPackageFixture {
            name: "hyperformula",
            package_id: "npm/hyperformula",
            slug: "hyperformula",
            source: "export class HyperFormula { static buildEmpty() { return new HyperFormula(); } calculateFormula(formula: string) { return { formula, value: null }; } }\nexport default HyperFormula;\n",
        },
    ]
}

fn conversion_bridge_package_fixture() -> ForgeConversionPackageFixture {
    ForgeConversionPackageFixture {
        name: "@ffmpeg/ffmpeg",
        package_id: "npm/@ffmpeg/ffmpeg",
        slug: "ffmpeg-ffmpeg",
        source: "export class FFmpeg { loaded = false; async load() { this.loaded = true; } async exec(args: string[]) { return { args, loaded: this.loaded }; } }\nexport default { FFmpeg };\n",
    }
}

fn write_conversion_package_source(
    project: &std::path::Path,
    package: &ForgeConversionPackageFixture,
) -> std::path::PathBuf {
    let source_dir = project
        .join(".dx/reviewed-package-sources/npm")
        .join(package.slug)
        .join("package");
    fs::create_dir_all(&source_dir).expect("source dir");
    fs::write(source_dir.join("index.ts"), package.source).expect("source");
    fs::write(
        source_dir.join("package.json"),
        format!(
            r#"{{"name":"{}","version":"0.0.0-forge-conversion-proof","license":"MIT"}}"#,
            package.name
        ),
    )
    .expect("package metadata");
    fs::write(source_dir.join("LICENSE"), "MIT fixture license\n").expect("license");
    fs::write(
        source_dir.join("dx-forge-evidence.sr"),
        "integrity=true\nadvisory=true\npopularity=true\nlicense=MIT\n",
    )
    .expect("forge evidence");
    source_dir
}

fn conversion_package_page_source() -> &'static str {
    r##"import { Vector3, Color } from "three";
import { utils as xlsxUtils, read as readWorkbook } from "xlsx";
import PptxGenJS from "pptxgenjs";
import JSZip from "jszip";
import { Canvas } from "fabric";
import { Stage } from "konva";
import { zipSync } from "fflate";
import { HyperFormula } from "hyperformula";

export default function PackageProofPage() {
  const vector = new Vector3(1, 2, 3).toArray().join(",");
  const color = new Color("#ffffff").value;
  const workbook = xlsxUtils.book_new();
  const parsed = readWorkbook(new Uint8Array());
  const deck = new PptxGenJS();
  const archive = new JSZip().file("proof.txt", "dx");
  const canvas = new Canvas("canvas");
  const stage = new Stage({ container: "stage" });
  const zipped = zipSync({ "proof.txt": new Uint8Array() });
  const formulas = HyperFormula.buildEmpty();
  return (
    <main data-forge-conversion-packages="true">
      {vector}
      {color}
      {workbook.SheetNames.length}
      {parsed.SheetNames.length}
      {deck.slides.length}
      {archive}
      {canvas.objects.length}
      {stage.options.container}
      {zipped.length}
      {String(formulas.calculateFormula("=1+1").value)}
    </main>
  );
}
"##
}

struct ForgePopularEcosystemImportFixture {
    ecosystem: &'static str,
    name: &'static str,
    package_id: &'static str,
    slug: &'static str,
    registry: &'static str,
    metadata_path: &'static str,
    metadata: &'static str,
    source_path: &'static str,
    source: &'static str,
}

struct ForgePythonRustGoExportFixture {
    ecosystem: &'static str,
    name: &'static str,
    slug: &'static str,
    metadata_path: &'static str,
    metadata: &'static str,
    source_path: &'static str,
    source: &'static str,
    expected_exports: &'static [&'static str],
    private_export: &'static str,
}

fn python_rust_go_export_fixtures() -> Vec<ForgePythonRustGoExportFixture> {
    vec![
        ForgePythonRustGoExportFixture {
            ecosystem: "pip",
            name: "requests",
            slug: "requests",
            metadata_path: "pyproject.toml",
            metadata: "[project]\nname = \"requests\"\nversion = \"0.0.0-forge-fixture\"\nlicense = \"MIT\"\n",
            source_path: "requests/__init__.py",
            source: "class Client:\n    pass\n\ndef request(url):\n    return url\n\ndef _private_request():\n    return None\n",
            expected_exports: &["Client", "request"],
            private_export: "_private_request",
        },
        ForgePythonRustGoExportFixture {
            ecosystem: "cargo",
            name: "serde",
            slug: "serde",
            metadata_path: "Cargo.toml",
            metadata: "[package]\nname = \"serde\"\nversion = \"0.0.0-forge-fixture\"\nlicense = \"MIT\"\n",
            source_path: "src/lib.rs",
            source: "pub struct Serializer;\npub enum Format { Json }\npub fn serialize() {}\nfn private_helper() {}\n",
            expected_exports: &["Serializer", "Format", "serialize"],
            private_export: "private_helper",
        },
        ForgePythonRustGoExportFixture {
            ecosystem: "go",
            name: "github.com/dx/router",
            slug: "github-com-dx-router",
            metadata_path: "go.mod",
            metadata: "module github.com/dx/router\n\ngo 1.24\n",
            source_path: "router.go",
            source: "package router\n\ntype Router struct{}\nfunc NewRouter() Router { return Router{} }\nfunc privateRouter() Router { return Router{} }\n",
            expected_exports: &["Router", "NewRouter"],
            private_export: "privateRouter",
        },
    ]
}

fn write_python_rust_go_export_source(
    project: &std::path::Path,
    package: &ForgePythonRustGoExportFixture,
) -> std::path::PathBuf {
    let source_dir = project
        .join(".dx/reviewed-package-sources")
        .join(package.ecosystem)
        .join(package.slug)
        .join("package");
    fs::create_dir_all(
        source_dir.join(
            std::path::Path::new(package.source_path)
                .parent()
                .unwrap_or_else(|| std::path::Path::new("")),
        ),
    )
    .expect("source dir");
    fs::write(source_dir.join(package.source_path), package.source).expect("source");
    fs::write(source_dir.join(package.metadata_path), package.metadata).expect("package metadata");
    fs::write(source_dir.join("LICENSE"), "MIT fixture license\n").expect("license");
    fs::write(
        source_dir.join("dx-forge-evidence.sr"),
        "integrity=true\nadvisory=true\npopularity=true\nlicense=MIT\n",
    )
    .expect("forge evidence");
    source_dir
}

fn popular_ecosystem_import_fixtures() -> Vec<ForgePopularEcosystemImportFixture> {
    vec![
        ForgePopularEcosystemImportFixture {
            ecosystem: "pub",
            name: "vector_math",
            package_id: "pub/vector_math",
            slug: "vector-math",
            registry: "pub.dev",
            metadata_path: "pubspec.yaml",
            metadata: "name: vector_math\nversion: 0.0.0-forge-fixture\nlicense: MIT\n",
            source_path: "lib/vector_math.dart",
            source: "class Vector3 { final double x; final double y; final double z; const Vector3(this.x, this.y, this.z); }\n",
        },
        ForgePopularEcosystemImportFixture {
            ecosystem: "maven",
            name: "org.jetbrains.kotlinx.kotlinx-coroutines-core",
            package_id: "maven/org.jetbrains.kotlinx.kotlinx-coroutines-core",
            slug: "org-jetbrains-kotlinx-kotlinx-coroutines-core",
            registry: "maven-central",
            metadata_path: "pom.xml",
            metadata: "<project><groupId>org.jetbrains.kotlinx</groupId><artifactId>kotlinx-coroutines-core</artifactId><version>0.0.0-forge-fixture</version><licenses><license><name>MIT</name></license></licenses></project>\n",
            source_path: "src/main/kotlin/CoroutineScope.kt",
            source: "package kotlinx.coroutines\nclass CoroutineScope\n",
        },
        ForgePopularEcosystemImportFixture {
            ecosystem: "nuget",
            name: "Humanizer",
            package_id: "nuget/Humanizer",
            slug: "humanizer",
            registry: "nuget.org",
            metadata_path: "Humanizer.nuspec",
            metadata: "<package><metadata><id>Humanizer</id><version>0.0.0-forge-fixture</version><license type=\"expression\">MIT</license></metadata></package>\n",
            source_path: "src/Humanizer.cs",
            source: "namespace Humanizer { public static class Words { public static string Title(string value) => value; } }\n",
        },
        ForgePopularEcosystemImportFixture {
            ecosystem: "composer",
            name: "symfony/console",
            package_id: "composer/symfony/console",
            slug: "symfony-console",
            registry: "packagist",
            metadata_path: "composer.json",
            metadata: r#"{"name":"symfony/console","version":"0.0.0-forge-fixture","license":"MIT"}"#,
            source_path: "src/Application.php",
            source: "<?php\nnamespace Symfony\\Component\\Console;\nfinal class Application {}\n",
        },
        ForgePopularEcosystemImportFixture {
            ecosystem: "gem",
            name: "rails",
            package_id: "gem/rails",
            slug: "rails",
            registry: "rubygems.org",
            metadata_path: "rails.gemspec",
            metadata: "Gem::Specification.new do |spec|\n  spec.name = 'rails'\n  spec.version = '0.0.0-forge-fixture'\n  spec.license = 'MIT'\nend\n",
            source_path: "lib/rails.rb",
            source: "module Rails\n  VERSION = '0.0.0-forge-fixture'\nend\n",
        },
        ForgePopularEcosystemImportFixture {
            ecosystem: "swift",
            name: "swift-algorithms",
            package_id: "swift/swift-algorithms",
            slug: "swift-algorithms",
            registry: "swift-package-index",
            metadata_path: "Package.swift",
            metadata: "// swift-tools-version: 6.0\nimport PackageDescription\nlet package = Package(name: \"swift-algorithms\")\n",
            source_path: "Sources/Algorithms/Chunked.swift",
            source: "public struct ChunkedSequence<Element> { public let elements: [Element] }\n",
        },
        ForgePopularEcosystemImportFixture {
            ecosystem: "hex",
            name: "jason",
            package_id: "hex/jason",
            slug: "jason",
            registry: "hex.pm",
            metadata_path: "mix.exs",
            metadata: "defmodule Jason.MixProject do\n  use Mix.Project\n  def project do\n    [app: :jason, version: \"0.0.0-forge-fixture\"]\n  end\nend\n",
            source_path: "lib/jason.ex",
            source: "defmodule Jason do\n  def encode(value), do: value\nend\n",
        },
        ForgePopularEcosystemImportFixture {
            ecosystem: "cran",
            name: "dplyr",
            package_id: "cran/dplyr",
            slug: "dplyr",
            registry: "cran.r-project.org",
            metadata_path: "DESCRIPTION",
            metadata: "Package: dplyr\nVersion: 0.0.0-forge-fixture\nLicense: MIT\n",
            source_path: "R/dplyr.R",
            source: "filter <- function(data) data\nmutate <- function(data) data\n",
        },
    ]
}

fn write_popular_ecosystem_source(
    project: &std::path::Path,
    package: &ForgePopularEcosystemImportFixture,
) -> std::path::PathBuf {
    let source_dir = project
        .join(".dx/reviewed-package-sources")
        .join(package.ecosystem)
        .join(package.slug)
        .join("package");
    fs::create_dir_all(
        source_dir.join(std::path::Path::new(package.source_path).parent().unwrap()),
    )
    .expect("source dir");
    fs::write(source_dir.join(package.source_path), package.source).expect("source");
    fs::write(source_dir.join(package.metadata_path), package.metadata).expect("package metadata");
    fs::write(source_dir.join("LICENSE"), "MIT fixture license\n").expect("license");
    fs::write(
        source_dir.join("dx-forge-evidence.sr"),
        "integrity=true\nadvisory=true\npopularity=true\nlicense=MIT\n",
    )
    .expect("forge evidence");
    source_dir
}

#[test]
fn forge_import_npm_write_materializes_selected_source_files_only() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        "project.name = forge-selected-source-slice\n",
    )
    .expect("config");
    let source_dir = dir.path().join(".dx/cache/npm/tiny-tools/package");
    fs::create_dir_all(source_dir.join("src")).expect("source dir");
    fs::write(
        source_dir.join("src/index.ts"),
        "export function selectedTool() { return 'selected'; }\n",
    )
    .expect("selected source");
    fs::write(
        source_dir.join("src/unused.ts"),
        "export function unusedTool() { return 'unused'; }\n",
    )
    .expect("unused source");
    fs::write(
        source_dir.join("package.json"),
        r#"{"name":"tiny-tools","version":"0.0.0-forge-fixture","license":"MIT"}"#,
    )
    .expect("package metadata");
    fs::write(source_dir.join("LICENSE"), "MIT fixture license\n").expect("license");
    fs::write(
        source_dir.join("dx-forge-evidence.sr"),
        "integrity=true\nadvisory=true\npopularity=true\n",
    )
    .expect("forge evidence");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let plan_path = write_reviewed_import_plan(
        &cli,
        dir.path(),
        "npm",
        "tiny-tools",
        &source_dir,
        &["src/index.ts"],
        90,
    );
    let output_path = dir.path().join("npm-tiny-tools-write.json");
    cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "tiny-tools".to_string(),
        "--write".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
        "--file".to_string(),
        "src/index.ts".to_string(),
        "--from-plan".to_string(),
        plan_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("selected source slice write");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["selected_files"][0], "src/index.ts");
    assert!(report["materialized_files"]
        .as_array()
        .expect("materialized files")
        .iter()
        .any(|path| path == "lib/forge/npm/tiny-tools/src/index.ts"));
    assert!(!report["materialized_files"]
        .as_array()
        .expect("materialized files")
        .iter()
        .any(|path| path == "lib/forge/npm/tiny-tools/src/unused.ts"));
    assert!(dir
        .path()
        .join("lib/forge/npm/tiny-tools/src/index.ts")
        .is_file());
    assert!(!dir
        .path()
        .join("lib/forge/npm/tiny-tools/src/unused.ts")
        .exists());
    assert_eq!(report["exports"][0]["source_path"], "src/index.ts");
}

#[test]
fn forge_import_npm_write_bridges_install_hooks_without_materializing() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        "project.name = forge-bridge-adapter\n",
    )
    .expect("config");
    let source_dir = dir.path().join(".dx/cache/npm/native-tool/package");
    fs::create_dir_all(&source_dir).expect("source dir");
    fs::write(
        source_dir.join("index.ts"),
        "export function runNativeTool() { return 'bridge required'; }\n",
    )
    .expect("source");
    fs::write(
        source_dir.join("package.json"),
        r#"{"name":"native-tool","version":"0.0.0-forge-fixture","license":"MIT","scripts":{"install":"node scripts/install.js"}}"#,
    )
    .expect("package metadata");
    fs::write(source_dir.join("LICENSE"), "MIT fixture license\n").expect("license");
    fs::write(
        source_dir.join("dx-forge-evidence.sr"),
        "advisory=true\npopularity=true\n",
    )
    .expect("forge evidence");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("npm-native-tool-write.json");
    let result = cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "native-tool".to_string(),
        "--write".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ]);

    assert!(result.is_err());
    assert!(!dir
        .path()
        .join("lib/forge/npm/native-tool/index.ts")
        .exists());
    assert!(!dir.path().join("node_modules").exists());

    let report = read_json_value(output_path);
    assert_eq!(report["mode"], "write");
    assert_eq!(report["materialized"], false);
    assert_eq!(report["materialization_ready"], false);
    assert_eq!(report["disposition"]["kind"], "bridge");
    assert_eq!(report["disposition"]["bridge_kind"], "tool");
    assert_eq!(
        report["materialization_status"],
        "bridge-required-before-materialization"
    );
    assert!(report["risk_flags"]
        .as_array()
        .expect("risk flags")
        .iter()
        .any(|risk| risk == "install-hook"));
    assert!(report_artifact_path(dir.path(), &report, "import_plan_path").is_file());
    assert!(report_artifact_path(dir.path(), &report, "import_plan_sr_path").is_file());
    assert!(report_artifact_path(dir.path(), &report, "import_plan_machine_path").is_file());
}

#[test]
fn forge_import_npm_write_bridges_package_side_effects_without_materializing() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        "project.name = forge-side-effect-adapter\n",
    )
    .expect("config");
    let source_dir = dir.path().join(".dx/cache/npm/side-effect-tool/package");
    fs::create_dir_all(source_dir.join("src")).expect("source dir");
    fs::write(
        source_dir.join("src/index.ts"),
        "export function sideEffectTool() { return 'review side effects'; }\n",
    )
    .expect("source");
    fs::write(
        source_dir.join("package.json"),
        r#"{"name":"side-effect-tool","version":"0.0.0-forge-fixture","license":"MIT","sideEffects":["src/register.ts"]}"#,
    )
    .expect("package metadata");
    fs::write(source_dir.join("LICENSE"), "MIT fixture license\n").expect("license");
    fs::write(
        source_dir.join("dx-forge-evidence.sr"),
        "integrity=true\nadvisory=true\npopularity=true\n",
    )
    .expect("forge evidence");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("npm-side-effect-tool-write.json");
    let result = cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "side-effect-tool".to_string(),
        "--write".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ]);

    assert!(result.is_err());
    assert!(!dir
        .path()
        .join("lib/forge/npm/side-effect-tool/src/index.ts")
        .exists());
    assert!(!dir.path().join("node_modules").exists());

    let report = read_json_value(output_path);
    assert_eq!(report["mode"], "write");
    assert_eq!(report["materialized"], false);
    assert_eq!(report["materialization_ready"], false);
    assert_eq!(report["disposition"]["kind"], "bridge");
    assert_eq!(
        report["materialization_status"],
        "bridge-required-before-materialization"
    );
    assert!(report["risk_flags"]
        .as_array()
        .expect("risk flags")
        .iter()
        .any(|risk| risk == "side-effect-import"));
    assert!(report_artifact_path(dir.path(), &report, "import_plan_path").is_file());
    assert!(report_artifact_path(dir.path(), &report, "import_plan_sr_path").is_file());
    assert!(report_artifact_path(dir.path(), &report, "import_plan_machine_path").is_file());
}

#[test]
fn forge_import_npm_write_requires_exact_advisory_evidence_before_materializing() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        "project.name = forge-advisory-gate\n",
    )
    .expect("config");
    let source_dir = dir.path().join(".dx/cache/npm/reviewed-tool/package");
    fs::create_dir_all(&source_dir).expect("source dir");
    fs::write(
        source_dir.join("index.ts"),
        "export function reviewedTool() { return 'needs advisory proof'; }\n",
    )
    .expect("source");
    fs::write(
        source_dir.join("package.json"),
        r#"{"name":"reviewed-tool","version":"0.0.0-forge-fixture","license":"MIT"}"#,
    )
    .expect("package metadata");
    fs::write(source_dir.join("LICENSE"), "MIT fixture license\n").expect("license");
    fs::write(
        source_dir.join("dx-forge-evidence.sr"),
        "integrity=true\nadvisory=false\npopularity=true\n",
    )
    .expect("forge evidence");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("npm-reviewed-tool-write.json");
    let result = cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "reviewed-tool".to_string(),
        "--write".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ]);

    assert!(result.is_err());
    assert!(!dir
        .path()
        .join("lib/forge/npm/reviewed-tool/index.ts")
        .exists());

    let report = read_json_value(output_path);
    assert_eq!(report["materialized"], false);
    assert_eq!(report["materialized_package_id"], serde_json::Value::Null);
    assert_eq!(report["materialization_ready"], false);
    assert_eq!(
        report["materialization_status"],
        "preflight-review-required"
    );
    assert!(report["applied_caps"]
        .as_array()
        .expect("caps")
        .iter()
        .any(|cap| cap["id"] == "advisory-evidence-incomplete"));
}

#[test]
fn forge_import_npm_write_requires_explicit_integrity_evidence_before_materializing() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        "project.name = forge-integrity-gate\n",
    )
    .expect("config");
    let source_dir = dir.path().join(".dx/cache/npm/popular-tool/package");
    fs::create_dir_all(&source_dir).expect("source dir");
    fs::write(
        source_dir.join("index.ts"),
        "export function popularTool() { return 'needs integrity proof'; }\n",
    )
    .expect("source");
    fs::write(
        source_dir.join("package.json"),
        r#"{"name":"popular-tool","version":"0.0.0-forge-fixture","license":"MIT"}"#,
    )
    .expect("package metadata");
    fs::write(source_dir.join("LICENSE"), "MIT fixture license\n").expect("license");
    fs::write(
        source_dir.join("dx-forge-evidence.sr"),
        "advisory=true\npopularity=true\nlicense=MIT\n",
    )
    .expect("forge evidence");

    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let output_path = dir.path().join("npm-popular-tool-write.json");
    let result = cli.cmd_forge(&[
        "import".to_string(),
        "npm".to_string(),
        "popular-tool".to_string(),
        "--write".to_string(),
        "--source-dir".to_string(),
        source_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ]);

    assert!(result.is_err());
    assert!(!dir
        .path()
        .join("lib/forge/npm/popular-tool/index.ts")
        .exists());
    assert!(
        output_path.is_file(),
        "expected Forge to write refusal report to `{}` before returning error; result: {result:?}",
        output_path.display()
    );

    let report = read_json_value(output_path);
    assert_eq!(report["materialized"], false);
    assert_eq!(report["materialized_package_id"], serde_json::Value::Null);
    assert_eq!(report["materialization_ready"], false);
    assert_eq!(
        report["materialization_status"],
        "preflight-review-required"
    );
    assert!(report["applied_caps"]
        .as_array()
        .expect("caps")
        .iter()
        .any(|cap| cap["id"] == "artifact-integrity-incomplete"));
    assert!(report["materialization_blocker_ids"]
        .as_array()
        .expect("blockers")
        .iter()
        .any(|blocker| blocker == "artifact-integrity-incomplete"));
    assert!(report["refusal_reasons"]
        .as_array()
        .expect("refusal reasons")
        .iter()
        .any(|reason| reason["code"] == "artifact-integrity-incomplete"));
}

fn report_artifact_path(
    project: &std::path::Path,
    report: &serde_json::Value,
    field: &str,
) -> std::path::PathBuf {
    let value = report[field]
        .as_str()
        .unwrap_or_else(|| panic!("{field} should be exposed as a path string"));
    let path = std::path::PathBuf::from(value);
    if path.is_absolute() {
        path
    } else {
        project.join(path)
    }
}

#[test]
fn dx_migrate_next_plan_inventories_project_without_installing_packages() {
    let dir = tempdir().expect("tempdir");
    let project = dir.path().join("next-app");
    fs::create_dir_all(project.join("app/dashboard")).expect("dashboard dir");
    fs::create_dir_all(project.join("app/api/health")).expect("api dir");
    fs::create_dir_all(project.join("components")).expect("components dir");
    fs::create_dir_all(project.join("server")).expect("server dir");
    fs::write(
        project.join("package.json"),
        r#"{
  "name": "next-plan-fixture",
  "private": true,
  "dependencies": {
    "next": "16.0.0",
    "react": "19.0.0",
    "react-dom": "19.0.0",
    "next-auth": "latest"
  }
}
"#,
    )
    .expect("package");
    fs::write(project.join("next.config.mjs"), "export default {};\n").expect("next config");
    fs::write(
            project.join("app/page.tsx"),
            r#"import Image from "next/image";
import Link from "next/link";
import { Counter } from "@/components/Counter";

export default function Page() {
  return <main><Image src="/hero.png" alt="Hero" width={640} height={360} /><Link href="/dashboard">Dashboard</Link><Counter /></main>;
}
"#,
        )
        .expect("page");
    fs::write(
        project.join("app/dashboard/page.tsx"),
        r#"import { redirect } from "next/navigation";

export default function Dashboard() {
  redirect("/");
}
"#,
    )
    .expect("dashboard page");
    fs::write(
        project.join("components/Counter.tsx"),
        r#""use client";

import { useState } from "react";

export function Counter() {
  const [count, setCount] = useState(0);
  return <button onClick={() => setCount(count + 1)}>Count {count}</button>;
}
"#,
    )
    .expect("counter");
    fs::write(
        project.join("app/api/health/route.ts"),
        r#"import { NextResponse } from "next/server";

export function GET() {
  return NextResponse.json({ ok: true });
}
"#,
    )
    .expect("route handler");
    fs::write(
        project.join("server/actions.ts"),
        r#"export async function recordWelcomeView() {
  return { ok: true };
}
"#,
    )
    .expect("action");
    fs::write(
        project.join("middleware.ts"),
        "export function middleware() { return Response.redirect('/login'); }\n",
    )
    .expect("middleware");
    fs::write(
            project.join("components/LazyChart.tsx"),
            "import dynamic from 'next/dynamic';\nexport const LazyChart = dynamic(() => import('./Chart'));\n",
        )
        .expect("dynamic component");

    let output_path = dir.path().join("next-migration-plan.json");
    let cli = Cli::with_cwd(project.clone());
    cli.cmd_migrate(&[
        "next".to_string(),
        "--plan".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "50".to_string(),
        "--quiet".to_string(),
    ])
    .expect("dx migrate next plan");

    let report = read_json_value(output_path);
    assert_eq!(report["command"], "dx migrate next --plan");
    assert_eq!(report["mode"], "plan-only");
    assert_eq!(report["source_framework"], "nextjs-app-router");
    assert_eq!(report["package_installs_run"], false);
    assert_eq!(report["lifecycle_scripts_executed"], false);
    assert_eq!(report["source_files_written"], false);
    assert_eq!(report["node_modules_present"], false);
    assert_eq!(report["inventory"]["page_route_count"], 2);
    assert_eq!(report["inventory"]["route_handler_count"], 1);
    assert_eq!(report["inventory"]["client_component_count"], 1);
    assert_eq!(report["inventory"]["server_action_count"], 1);
    assert!(report["score"].as_u64().expect("score") >= 50);
    assert!(report["migration_steps"]
        .as_array()
        .expect("steps")
        .iter()
        .any(|step| step["source"] == "app/page.tsx"
            && step["target"] == "app/page.tsx"
            && step["status"] == "ready"));
    assert!(report["migration_steps"]
        .as_array()
        .expect("steps")
        .iter()
        .any(|step| step["source"] == "next/image"
            && step["target"] == "forge/adapters/next-image.tsx"));
    assert!(report["unsupported_api_findings"]
        .as_array()
        .expect("findings")
        .iter()
        .any(|finding| finding["api"] == "middleware.ts" && finding["severity"] == "red"));
    assert!(report["unsupported_api_findings"]
        .as_array()
        .expect("findings")
        .iter()
        .any(|finding| finding["api"] == "next/dynamic"));
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_migrate_react_plan_inventories_vite_project_without_writes() {
    let dir = tempdir().expect("tempdir");
    let project = dir.path().join("vite-react-app");
    fs::create_dir_all(project.join("src/components")).expect("components dir");
    fs::write(
        project.join("package.json"),
        r#"{
  "name": "vite-react-fixture",
  "private": true,
  "dependencies": {
    "react": "19.0.0",
    "react-dom": "19.0.0",
    "react-router-dom": "7.1.1",
    "lucide-react": "0.468.0"
  },
  "devDependencies": {
    "@vitejs/plugin-react": "4.3.4",
    "vite": "6.0.7",
    "typescript": "5.7.2"
  }
}
"#,
    )
    .expect("package");
    fs::write(
            project.join("src/main.tsx"),
            "import { createRoot } from 'react-dom/client';\nimport App from './App';\ncreateRoot(document.getElementById('root')!).render(<App />);\n",
        )
        .expect("main");
    fs::write(
        project.join("src/App.tsx"),
        r#"import { BrowserRouter } from "react-router-dom";
import { Counter } from "./components/Counter";

export default function App() {
  return <BrowserRouter><Counter /></BrowserRouter>;
}
"#,
    )
    .expect("app");
    fs::write(
        project.join("src/components/Counter.tsx"),
        r#"import React from "react";

export function Counter() {
  const [count, setCount] = React.useState<number>(() => 2);
  const [open, setOpen] = React.useState(false);
  return (
    <section>
      <button onClick={() => setCount((value) => value + 3)}>Add</button>
      <button onClick={() => setOpen((value) => !value)}>Toggle</button>
      <p>{open ? count : "closed"}</p>
    </section>
  );
}
"#,
    )
    .expect("counter");

    let output_path = dir.path().join("react-migration-plan.json");
    let cli = Cli::with_cwd(project.clone());
    cli.cmd_migrate(&[
        "react".to_string(),
        "--plan".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "1".to_string(),
        "--quiet".to_string(),
    ])
    .expect("dx migrate react plan");

    let report = read_json_value(output_path);
    assert_eq!(report["command"], "dx migrate react --plan");
    assert_eq!(report["mode"], "plan-only");
    assert_eq!(report["source_framework"], "vite-react");
    assert_eq!(report["package_installs_run"], false);
    assert_eq!(report["lifecycle_scripts_executed"], false);
    assert_eq!(report["source_files_written"], false);
    assert_eq!(report["node_modules_present"], false);
    assert_eq!(report["inventory"]["tsx_file_count"], 3);
    assert_eq!(report["inventory"]["state_hook_count"], 2);
    assert_eq!(report["compatibility"]["can_scan_without_breaking"], true);
    assert_eq!(
        report["compatibility"]["supports_current_state_subset"],
        true
    );
    assert!(report["compatibility"]["missing_adapters"]
        .as_array()
        .expect("missing adapters")
        .iter()
        .any(|adapter| adapter == "react-router-dom"));
    assert!(report["migration_steps"]
        .as_array()
        .expect("steps")
        .iter()
        .any(
            |step| step["source"] == "src/App.tsx" && step["target"] == "components/local/App.tsx"
        ));
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_migrate_react_recursive_plan_discovers_inspiration_like_web_apps_without_writes() {
    let dir = tempdir().expect("tempdir");
    let inspiration_root = dir.path().join("inspirations");
    let agent_web = inspiration_root.join("agent-archive/web");
    let hermes_web = inspiration_root.join("hermes-agent/web");
    let rust_only = inspiration_root.join("openclaw");
    fs::create_dir_all(agent_web.join("src/components")).expect("agent web dirs");
    fs::create_dir_all(hermes_web.join("src")).expect("hermes web dirs");
    fs::create_dir_all(&rust_only).expect("rust only dir");
    fs::write(
        agent_web.join("package.json"),
        r#"{
  "name": "agent-web",
  "dependencies": {
    "react": "19.0.0",
    "react-dom": "19.0.0",
    "react-router-dom": "7.1.1"
  },
  "devDependencies": {
    "vite": "6.0.7",
    "@vitejs/plugin-react": "4.3.4"
  }
}
"#,
    )
    .expect("agent package");
    fs::write(
        agent_web.join("src/App.tsx"),
        "export function App() { return <main>Agent</main>; }\n",
    )
    .expect("agent app");
    fs::write(
        agent_web.join("src/main.tsx"),
        "import { App } from './App';\n<App />;\n",
    )
    .expect("agent main");
    fs::write(
        hermes_web.join("package.json"),
        r#"{
  "name": "hermes-web",
  "dependencies": {
    "react": "19.0.0",
    "react-dom": "19.0.0",
    "@xterm/xterm": "5.5.0",
    "@react-three/fiber": "9.0.0"
  },
  "devDependencies": {
    "vite": "6.0.7"
  }
}
"#,
    )
    .expect("hermes package");
    fs::write(
        hermes_web.join("src/App.tsx"),
        r#"import { useEffect, useState } from "react";

export function App() {
  const [ready, setReady] = useState(false);
  useEffect(() => setReady(true), []);
  return <main>{ready ? "Hermes" : "Loading"}</main>;
}
"#,
    )
    .expect("hermes app");
    fs::write(
        rust_only.join("package.json"),
        r#"{ "name": "openclaw", "dependencies": { "serde": "latest" } }"#,
    )
    .expect("rust only package");

    let output_path = dir.path().join("recursive-react-plan.json");
    let cli = Cli::with_cwd(inspiration_root.clone());
    cli.cmd_migrate(&[
        "react".to_string(),
        "--plan".to_string(),
        "--recursive".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "1".to_string(),
        "--quiet".to_string(),
    ])
    .expect("recursive react migration plan");

    let report = read_json_value(output_path);
    assert_eq!(report["command"], "dx migrate react --plan --recursive");
    assert_eq!(report["mode"], "plan-only");
    assert_eq!(report["project_count"], 2);
    assert_eq!(report["package_installs_run"], false);
    assert_eq!(report["lifecycle_scripts_executed"], false);
    assert_eq!(report["source_files_written"], false);
    assert!(report["projects"]
        .as_array()
        .expect("projects")
        .iter()
        .any(|project| project["project_path"]
            .as_str()
            .expect("project path")
            .ends_with("agent-archive\\web")
            || project["project_path"]
                .as_str()
                .expect("project path")
                .ends_with("agent-archive/web")));
    assert!(report["projects"]
        .as_array()
        .expect("projects")
        .iter()
        .any(|project| project["compatibility"]["missing_adapters"]
            .as_array()
            .expect("adapters")
            .iter()
            .any(|adapter| adapter == "@xterm/xterm")));
    assert!(!agent_web.join("node_modules").exists());
    assert!(!hermes_web.join("node_modules").exists());
}

#[test]
fn dx_migrate_react_recursive_web_only_ignores_react_tui_packages() {
    let dir = tempdir().expect("tempdir");
    let inspiration_root = dir.path().join("inspirations");
    let web_app = inspiration_root.join("hermes-agent/web");
    let tui_package = inspiration_root.join("hermes-agent/ui-tui");
    fs::create_dir_all(web_app.join("src")).expect("web app dir");
    fs::create_dir_all(tui_package.join("src")).expect("tui package dir");
    fs::write(
        web_app.join("package.json"),
        r#"{
  "name": "hermes-web",
  "dependencies": {
    "react": "19.0.0",
    "react-dom": "19.0.0"
  },
  "devDependencies": {
    "vite": "6.0.7"
  }
}
"#,
    )
    .expect("web package");
    fs::write(
        web_app.join("src/App.tsx"),
        "export function App() { return <main>Web</main>; }\n",
    )
    .expect("web app");
    fs::write(
        tui_package.join("package.json"),
        r#"{
  "name": "hermes-ui-tui",
  "dependencies": {
    "react": "19.0.0",
    "ink": "5.0.0"
  }
}
"#,
    )
    .expect("tui package");
    fs::write(
        tui_package.join("src/App.tsx"),
        "export function App() { return <box>TUI</box>; }\n",
    )
    .expect("tui app");

    let output_path = dir.path().join("recursive-react-web-plan.json");
    let cli = Cli::with_cwd(inspiration_root.clone());
    cli.cmd_migrate(&[
        "react".to_string(),
        "--plan".to_string(),
        "--recursive".to_string(),
        "--web-only".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "1".to_string(),
        "--quiet".to_string(),
    ])
    .expect("recursive web-only react migration plan");

    let report = read_json_value(output_path);
    assert_eq!(
        report["command"],
        "dx migrate react --plan --recursive --web-only"
    );
    assert_eq!(report["scope"], "web-apps");
    assert_eq!(report["project_count"], 1);
    assert!(report["projects"]
        .as_array()
        .expect("projects")
        .iter()
        .all(|project| !project["project_path"]
            .as_str()
            .expect("project path")
            .contains("ui-tui")));
    assert!(report["projects"]
        .as_array()
        .expect("projects")
        .iter()
        .any(|project| project["project_path"]
            .as_str()
            .expect("project path")
            .contains("web")));
    assert!(!web_app.join("node_modules").exists());
    assert!(!tui_package.join("node_modules").exists());
}

#[test]
fn dx_migrate_react_plan_reports_complexity_ceiling_for_heavy_apps() {
    let dir = tempdir().expect("tempdir");
    let project = dir.path().join("heavy-react-app");
    fs::create_dir_all(project.join("src")).expect("src dir");
    fs::write(
        project.join("package.json"),
        r#"{
  "name": "heavy-react-fixture",
  "dependencies": {
    "react": "19.0.0",
    "react-dom": "19.0.0",
    "react-router-dom": "7.1.1",
    "@xterm/xterm": "5.5.0",
    "@react-three/fiber": "9.0.0"
  },
  "devDependencies": {
    "vite": "6.0.7"
  }
}
"#,
    )
    .expect("package");
    fs::write(
        project.join("src/main.tsx"),
        "import { App } from './App';\n<App />;\n",
    )
    .expect("main");
    fs::write(
        project.join("src/App.tsx"),
        r#"import { useEffect, useState } from "react";
import { BrowserRouter } from "react-router-dom";

export function App() {
  const [ready, setReady] = useState(false);
  useEffect(() => setReady(true), []);
  return <BrowserRouter><main>{ready ? "Ready" : "Loading"}</main></BrowserRouter>;
}
"#,
    )
    .expect("app");

    let output_path = dir.path().join("react-complexity-plan.json");
    let cli = Cli::with_cwd(project.clone());
    cli.cmd_migrate(&[
        "react".to_string(),
        "--plan".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "1".to_string(),
        "--quiet".to_string(),
    ])
    .expect("react complexity migration plan");

    let report = read_json_value(output_path);
    assert_eq!(report["complexity"]["band"], "heavy-client-runtime");
    assert_eq!(
        report["complexity"]["current_www_handling"],
        "safe-plan-only"
    );
    assert_eq!(report["complexity"]["readiness_lane"], "adapter-first");
    assert_eq!(
        report["complexity"]["recommended_next_action"],
        "Plan Forge adapters for advanced client runtimes before route compilation."
    );
    assert_eq!(report["complexity"]["safe_to_analyze_without_writes"], true);
    assert!(report["complexity"]["direct_compile_blockers"]
        .as_array()
        .expect("blockers")
        .iter()
        .any(|blocker| blocker == "react-router-adapter-required"));
    assert!(report["complexity"]["direct_compile_blockers"]
        .as_array()
        .expect("blockers")
        .iter()
        .any(|blocker| blocker == "advanced-client-runtime-adapter-required"));
    assert!(report["complexity"]["next_required_work"]
        .as_array()
        .expect("next work")
        .iter()
        .any(|work| work == "state-graph-effects-context-reducer-abi"));
    assert!(!project.join("node_modules").exists());
}

#[test]
fn dx_migrate_next_diagnostics_explain_strict_compile_blockers_and_fixes() {
    let dir = tempdir().expect("tempdir");
    let project = dir.path().join("next-diagnostics-app");
    fs::create_dir_all(project.join("app")).expect("app dir");
    fs::create_dir_all(project.join("components")).expect("components dir");
    fs::create_dir_all(project.join("node_modules/lodash")).expect("node_modules fixture");
    fs::write(
        project.join("package.json"),
        r#"{
  "name": "next-diagnostics-fixture",
  "private": true,
  "dependencies": {
    "next": "16.0.0",
    "react": "19.0.0",
    "react-dom": "19.0.0",
    "lodash": "latest",
    "next-auth": "latest"
  }
}
"#,
    )
    .expect("package");
    fs::write(project.join("next.config.mjs"), "export default {};\n").expect("next config");
    fs::write(
        project.join("app/page.tsx"),
        r#"import Script from "next/script";
import { LazyChart } from "@/components/LazyChart";

export default function Page() {
  return <main><Script id="boot" /><LazyChart /></main>;
}
"#,
    )
    .expect("page");
    fs::write(
            project.join("components/LazyChart.tsx"),
            "import dynamic from 'next/dynamic';\nexport const LazyChart = dynamic(() => import('./Chart'));\n",
        )
        .expect("dynamic component");
    fs::write(
        project.join("middleware.ts"),
        "export function middleware() { return Response.redirect('/login'); }\n",
    )
    .expect("middleware");

    let output_path = dir.path().join("next-diagnostics.json");
    let cli = Cli::with_cwd(project.clone());
    cli.cmd_migrate(&[
        "next".to_string(),
        "--plan".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "1".to_string(),
        "--quiet".to_string(),
    ])
    .expect("dx migrate next diagnostics");

    let report = read_json_value(output_path);
    assert_eq!(report["strict_compile_ready"], false);
    assert_eq!(report["strict_compile_diagnostics"]["ready"], false);
    assert_eq!(
        report["strict_compile_diagnostics"]["blocked_reason_count"]
            .as_u64()
            .expect("blocked count"),
        5
    );
    let diagnostics = report["strict_compile_diagnostics"]["diagnostics"]
        .as_array()
        .expect("diagnostics");
    for code in [
        "strict-no-node-modules",
        "next-middleware-needs-route-policy",
        "next-dynamic-needs-client-island",
        "next-unsupported-import",
        "npm-package-requires-forge-import",
    ] {
        assert!(
            diagnostics.iter().any(|diagnostic| {
                diagnostic["code"] == code
                    && diagnostic["can_compile_under_strict_dx_www"] == false
                    && diagnostic["why"]
                        .as_str()
                        .expect("why")
                        .contains("strict DX-WWW")
                    && diagnostic["fix"].as_str().expect("fix").len() > 16
                    && diagnostic["command"]
                        .as_str()
                        .expect("command")
                        .starts_with("dx ")
            }),
            "missing diagnostic {code}: {diagnostics:#?}"
        );
    }
    assert!(diagnostics
        .iter()
        .any(|diagnostic| diagnostic["code"] == "next-unsupported-import"
            && diagnostic["api"] == "next/script"
            && diagnostic["path"] == "app/page.tsx"));
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic["code"] == "npm-package-requires-forge-import"
            && diagnostic["api"] == "lodash"
            && diagnostic["source_owned_target"]
                .as_str()
                .expect("source owned target")
                .ends_with("lodash.json")
    }));
}

#[test]
fn dx_check_strict_forge_passes_for_clean_source_owned_package() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "add".to_string(),
        "shadcn/ui/button".to_string(),
        "--write".to_string(),
    ])
    .expect("forge add");

    cli.cmd_check(&[".".to_string(), "--strict-forge".to_string()])
        .expect("strict forge check");
}

#[test]
fn dx_check_strict_forge_fails_for_red_package_traffic() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "add".to_string(),
        "shadcn/ui/button".to_string(),
        "--write".to_string(),
    ])
    .expect("forge add");
    fs::write(
        dir.path().join("components/ui/button.tsx"),
        "fetch('https://filev2.getsession.org/session')\n",
    )
    .expect("security edit");

    let error = cli
        .cmd_check(&[".".to_string(), "--strict-forge".to_string()])
        .expect_err("strict forge should fail");

    assert!(error.to_string().contains("strict release check failed"));
}

#[test]
fn dx_check_strict_forge_fails_for_missing_rollback_coverage() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "add".to_string(),
        "shadcn/ui/button".to_string(),
        "--write".to_string(),
    ])
    .expect("forge add");

    let manifest_path = dir.path().join(".dx/forge/source-.dx/build-cache/manifest.json");
    let mut manifest: dx_compiler::ecosystem::DxSourceManifest =
        serde_json::from_slice(&fs::read(&manifest_path).expect("manifest bytes"))
            .expect("manifest json");
    manifest.packages[0].last_accepted_update = Some("2026-05-16T00:00:00Z".to_string());
    manifest.packages[0].rollback_receipt = None;
    fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&manifest).expect("manifest json"),
    )
    .expect("write manifest");

    let error = cli
        .cmd_check(&[".".to_string(), "--strict-forge".to_string()])
        .expect_err("strict forge should fail");

    assert!(error
        .to_string()
        .contains("forge-launch-gate-missing-rollback"));
}

#[test]
fn forge_doctor_passes_for_documented_source_owned_package() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "add".to_string(),
        "shadcn/ui/button".to_string(),
        "--write".to_string(),
    ])
    .expect("forge add");

    cli.cmd_forge(&[
        "doctor".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
    ])
    .expect("forge doctor");
}

#[test]
fn forge_doctor_fails_when_package_docs_are_missing() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "add".to_string(),
        "shadcn/ui/button".to_string(),
        "--write".to_string(),
    ])
    .expect("forge add");
    fs::remove_file(dir.path().join(".dx/forge/docs/shadcn-ui-button.md")).expect("remove docs");

    let error = cli
        .cmd_forge(&[
            "doctor".to_string(),
            "--project".to_string(),
            ".".to_string(),
        ])
        .expect_err("forge doctor should fail");

    assert!(error.to_string().contains("missing_docs=1"));
}

#[test]
fn forge_docs_write_regenerates_missing_package_docs() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_forge(&[
        "add".to_string(),
        "shadcn/ui/button".to_string(),
        "--write".to_string(),
    ])
    .expect("forge add");
    fs::remove_file(dir.path().join(".dx/forge/docs/shadcn-ui-button.md")).expect("remove docs");

    cli.cmd_forge(&[
        "docs".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--write".to_string(),
    ])
    .expect("forge docs write");

    assert!(dir
        .path()
        .join(".dx/forge/docs/shadcn-ui-button.md")
        .exists());
    assert!(dir.path().join("components/ui/button.tsx").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_evidence_writes_release_report() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["ui/button", "--write"])
        .expect("dx add ui/button");
    cli.cmd_add(&["ui/card", "--write"])
        .expect("dx add ui/card");
    cli.cmd_add(&["icon", "search", "--write"])
        .expect("dx add icon search");
    let history_dir = dir.path().join("benchmarks/reports/vertical-proof-history");
    fs::create_dir_all(&history_dir).expect("history dir");
    let history_path = history_dir.join("index.json");
    fs::write(
        &history_path,
        r#"{
  "updated_at": "2026-05-16T00:00:00Z",
  "snapshots": [
    {
      "generated_at": "2026-05-16T00:00:00Z",
      "fixture_mode": "forge-combo",
      "markdown": "vertical-proof-history/snapshot.md",
      "forge_packages": 2,
      "forge_files_tracked": 6,
      "decoded_bytes": 12000,
      "brotli_bytes": 3200,
      "http_route_median_ms": 2.5,
      "chrome_load_event_ms": 35.0,
      "dx_packet_applied": true,
      "interaction_works": true
    }
  ]
}"#,
    )
    .expect("history index");
    let output_path = dir.path().join("release-proof.md");

    cli.cmd_forge(&[
        "evidence".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--history".to_string(),
        history_path.to_string_lossy().into_owned(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("forge evidence");

    let report = fs::read_to_string(output_path).expect("evidence report");
    assert!(report.contains("DX Forge Release Proof"));
    assert!(report.contains("Package Scorecard"));
    assert!(report.contains("Scorecard score: `100`"));
    assert!(report.contains("auth/better-auth"));
    assert!(report.contains("forge-combo"));
    assert!(report.contains("Package docs coverage: `100%`"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_evidence_output_preserves_history_snapshot() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["ui/button", "--write"])
        .expect("dx add ui/button");
    cli.cmd_add(&["ui/card", "--write"])
        .expect("dx add ui/card");
    cli.cmd_add(&["ui/input", "--write"])
        .expect("dx add ui/input");
    cli.cmd_add(&["icon", "search", "--write"])
        .expect("dx add icon search");
    cli.cmd_add(&["auth/better-auth", "--write"])
        .expect("dx add auth/better-auth");
    cli.cmd_add(&["auth/better-auth", "--write"])
        .expect("dx add auth/better-auth");
    cli.cmd_add(&["db/drizzle", "--write"])
        .expect("dx add db/drizzle");
    cli.cmd_add(&["migration/static-site", "--write"])
        .expect("dx add migration/static-site");
    let history_path = write_passing_forge_benchmark_history(dir.path());
    let output_path = dir.path().join("release-proof.json");

    cli.cmd_forge(&[
        "evidence".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--history".to_string(),
        history_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("forge evidence");

    let evidence_history = dir.path().join(".dx/forge/release-proof-history");
    let index = fs::read_to_string(evidence_history.join("index.json")).expect("index json");
    let index_markdown =
        fs::read_to_string(evidence_history.join("index.md")).expect("index markdown");
    assert!(index.contains("\"snapshots\""));
    assert!(index.contains("\"check_score\""));
    assert!(index.contains("\"snapshot_file\""));
    assert!(index_markdown.contains("DX Forge Release Proof History"));

    let snapshot_path = fs::read_dir(&evidence_history)
        .expect("evidence history dir")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .find(|path| {
            path.extension().and_then(|value| value.to_str()) == Some("json")
                && path.file_name().and_then(|value| value.to_str()) != Some("index.json")
        })
        .expect("evidence snapshot");
    let snapshot = fs::read_to_string(snapshot_path).expect("snapshot json");
    assert!(snapshot.contains("\"package_scorecard\""));
    assert!(snapshot.contains("\"latest_benchmark\""));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_public_evidence_exports_json_and_markdown() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let json_path = dir.path().join("public-evidence.json");
    let markdown_path = dir.path().join("public-evidence.md");

    cli.cmd_forge(&[
        "public-evidence".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        json_path.to_string_lossy().into_owned(),
    ])
    .expect("public evidence json");
    cli.cmd_forge(&[
        "public-evidence".to_string(),
        "--format".to_string(),
        "markdown".to_string(),
        "--output".to_string(),
        markdown_path.to_string_lossy().into_owned(),
    ])
    .expect("public evidence markdown");

    let json = fs::read_to_string(json_path).expect("public evidence json");
    let markdown = fs::read_to_string(markdown_path).expect("public evidence markdown");
    assert!(json.contains("\"route\": \"/forge/evidence\""));
    assert!(json.contains("forge-readiness-badge.json"));
    assert!(json.contains("forge-public-route-comparison.md"));
    assert!(json.contains("forge/scorecard.html"));
    assert!(json.contains("forge/ci.html"));
    assert!(markdown.contains("# DX Forge Public Evidence"));
    assert!(markdown.contains("[`forge-readiness-badge.json`](forge-readiness-badge.json)"));
    assert!(markdown.contains("[`forge/ci.html`](forge/ci.html)"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_public_evidence_fixture_links_existing_secret_free_artifacts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let public_dir = dir.path().join("public");
    let ci_dir = tempdir().expect("ci tempdir");
    let ci_cli = Cli::with_cwd(ci_dir.path().to_path_buf());
    let ci_artifact_dir = ci_dir.path().join("ci-artifacts");

    ci_cli
        .cmd_forge(&[
            "ci".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--out".to_string(),
            ci_artifact_dir.to_string_lossy().into_owned(),
            "--quiet".to_string(),
        ])
        .expect("forge ci public artifacts");
    for fixture in ["forge-scorecard", "forge-ci", "forge-evidence"] {
        cli.cmd_prove(&[
            "vertical".to_string(),
            "--fixture".to_string(),
            fixture.to_string(),
            "--out".to_string(),
            "public".to_string(),
            "--write".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ])
        .unwrap_or_else(|error| panic!("prove fixture {fixture}: {error}"));
    }
    fs::copy(
        ci_artifact_dir.join("forge-readiness-badge.json"),
        public_dir.join("forge-readiness-badge.json"),
    )
    .expect("copy readiness badge");
    copy_public_evidence_benchmark_reports(&public_dir);

    let evidence_html =
        fs::read_to_string(public_dir.join("forge/evidence.html")).expect("evidence html");
    let html_hrefs = html_href_values(&evidence_html);
    for link in forge_public_evidence::forge_public_evidence_links() {
        assert!(
            html_hrefs.iter().any(|href| href == link.href),
            "public evidence route should link `{}`",
            link.href
        );
        assert!(
            public_dir.join(link.href).exists(),
            "public evidence link `{}` should resolve to an artifact",
            link.href
        );
    }

    assert_secret_markers_absent(&public_dir);
    assert!(!dir.path().join("node_modules").exists());
    assert!(!ci_dir.path().join("node_modules").exists());
}

#[test]
fn forge_public_evidence_verify_validates_existing_export_without_regenerating() {
    let dir = tempdir().expect("tempdir");
    let public_dir = write_public_evidence_export_fixture(dir.path());
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let report_path = dir.path().join("public-evidence-verify.json");

    cli.cmd_forge(&[
        "public-evidence".to_string(),
        "--verify".to_string(),
        public_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        report_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "100".to_string(),
        "--quiet".to_string(),
    ])
    .expect("verify public evidence export");

    let report = fs::read_to_string(report_path).expect("verification report");
    assert!(report.contains("\"passed\": true"));
    assert!(report.contains("\"score\": 100"));
    assert!(report.contains("forge/evidence.html"));
    assert!(report.contains("forge-readiness-badge.json"));
    assert!(report.contains("public route comparison"));
    assert!(!public_dir.join("node_modules").exists());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_public_evidence_verify_fails_for_missing_export_artifacts() {
    let dir = tempdir().expect("tempdir");
    let public_dir = dir.path().join("public");
    fs::create_dir_all(&public_dir).expect("public dir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    let error = cli
        .cmd_forge(&[
            "public-evidence".to_string(),
            "--verify".to_string(),
            public_dir.to_string_lossy().into_owned(),
            "--quiet".to_string(),
            "--no-fail-under".to_string(),
        ])
        .expect_err("empty export should fail verification");

    assert!(error.to_string().contains("forge/evidence.html"));
    assert!(!public_dir.join("node_modules").exists());
}

#[test]
fn forge_release_notes_summarize_readiness_scorecard_routes_and_limitations() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    cli.cmd_add(&["ui/button", "--write"])
        .expect("dx add ui/button");
    cli.cmd_add(&["icon", "search", "--write"])
        .expect("dx add icon search");
    cli.cmd_add(&["auth/better-auth", "--write"])
        .expect("dx add auth/better-auth");
    let history_path = write_passing_forge_benchmark_history(dir.path());
    let notes_path = dir.path().join("forge-release-notes.md");

    cli.cmd_forge(&[
        "release-notes".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--history".to_string(),
        history_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "markdown".to_string(),
        "--output".to_string(),
        notes_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("forge release notes");

    let notes = fs::read_to_string(notes_path).expect("release notes");
    assert!(notes.contains("# DX Forge Release Notes"));
    assert!(notes.contains("## CI Readiness"));
    assert!(notes.contains("## Package Scorecard"));
    assert!(notes.contains("## Route Measurements"));
    assert!(notes.contains("## Honest Launch Limitations"));
    assert!(notes.contains("shadcn/ui/button"));
    assert!(notes.contains("dx/icon/search"));
    assert!(notes.contains("auth/better-auth"));
    assert!(notes.contains("decoded bytes"));
    assert!(notes.contains("not a universal npm replacement"));
    assert!(!notes.contains("CLOUDFLARE_R2_"));
    assert!(!notes.contains("DX_FORGE_R2_LIVE"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_release_dashboard_verifies_public_launch_artifacts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let ci_project = tempdir().expect("ci project");
    let ci_cli = Cli::with_cwd(ci_project.path().to_path_buf());
    let pages_project = tempdir().expect("pages project");
    let pages_cli = Cli::with_cwd(pages_project.path().to_path_buf());
    let ci_artifact_dir = ci_project.path().join(".dx/ci");
    let pages_dir = pages_project.path().join(".dx/forge-pages");
    let history_path = write_passing_forge_benchmark_history(dir.path());
    let route_comparison_path = write_passing_public_route_comparison(dir.path());
    let dashboard_path = dir.path().join("release-dashboard.json");

    ci_cli
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

    cli.cmd_forge(&[
        "release-dashboard".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--ci-artifacts".to_string(),
        ci_artifact_dir.to_string_lossy().into_owned(),
        "--pages".to_string(),
        pages_dir.to_string_lossy().into_owned(),
        "--history".to_string(),
        history_path.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        route_comparison_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        dashboard_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("release dashboard");

    let dashboard = read_json_value(dashboard_path);
    assert_eq!(dashboard["passed"], true);
    assert!(dashboard["score"].as_u64().expect("dashboard score") >= 90);
    assert_eq!(dashboard["checks"]["ci_artifacts"]["passed"], true);
    assert_eq!(dashboard["checks"]["pages_bundle"]["passed"], true);
    assert_eq!(dashboard["checks"]["release_notes"]["passed"], true);
    assert_eq!(dashboard["checks"]["launch_changelog"]["passed"], true);
    assert_eq!(dashboard["checks"]["public_evidence"]["passed"], true);
    assert_eq!(dashboard["checks"]["route_comparison"]["passed"], true);
    assert_eq!(dashboard["launch_changelog"]["passed"], true);
    assert!(
        dashboard["launch_changelog"]["honest_scope_count"]
            .as_u64()
            .expect("launch changelog honest scope")
            >= 4
    );
    assert!(dashboard["checks"]["launch_changelog"]["message"]
        .as_str()
        .expect("launch changelog message")
        .contains("reviewed release-history"));
    assert_eq!(dashboard["route_comparison"]["route_count"], 6);
    assert!(
        dashboard["public_evidence"]["links"]
            .as_u64()
            .expect("links")
            >= 9
    );
    assert!(
        dashboard["release_notes"]["score"]
            .as_u64()
            .expect("notes score")
            >= 90
    );
    assert!(!dir.path().join("node_modules").exists());
    assert!(!ci_project.path().join("node_modules").exists());
    assert!(!pages_project.path().join("node_modules").exists());
}

#[test]
fn forge_release_candidate_gate_joins_public_beta_evidence() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let fixture = write_release_candidate_fixture(dir.path());
    let output_path = dir.path().join("forge-release-candidate.json");

    cli.cmd_forge(&[
        "release-candidate".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--ci-artifacts".to_string(),
        fixture.ci_artifact_dir.to_string_lossy().into_owned(),
        "--pages".to_string(),
        fixture.pages_dir.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        fixture.route_comparison_path.to_string_lossy().into_owned(),
        "--source-review".to_string(),
        fixture.source_review_path.to_string_lossy().into_owned(),
        "--static-evidence".to_string(),
        fixture.static_evidence_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("forge release-candidate");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert!(
        report["score"].as_u64().expect("release candidate score") >= 90,
        "release candidate score should pass: {report:#?}"
    );
    assert_eq!(report["checks"]["ci_artifacts"]["passed"], true);
    assert_eq!(report["checks"]["pages_bundle"]["passed"], true);
    assert_eq!(report["checks"]["route_comparison"]["passed"], true);
    assert_eq!(report["checks"]["source_owned_review"]["passed"], true);
    assert_eq!(
        report["checks"]["static_competitor_evidence"]["passed"],
        true
    );
    assert_eq!(report["checks"]["secret_markers"]["passed"], true);
    assert_eq!(report["checks"]["no_node_modules"]["passed"], true);
    assert_eq!(report["source_owned_review"]["no_node_modules"], true);
    assert_eq!(
        report["static_competitor_evidence"]["scope"]["not_full_framework_benchmark"],
        true
    );
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_release_candidate_gate_fails_secret_marker_leaks() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let fixture = write_release_candidate_fixture(dir.path());
    let leaked_page = fixture.pages_dir.join("forge/ci.html");
    let mut leaked_text = fs::read_to_string(&leaked_page).expect("pages html");
    leaked_text.push_str("\n<!-- CLOUDFLARE_R2_SECRET_ACCESS_KEY -->\n");
    fs::write(&leaked_page, leaked_text).expect("leak marker");
    let output_path = dir.path().join("forge-release-candidate.json");

    let error = cli
        .cmd_forge(&[
            "release-candidate".to_string(),
            "--project".to_string(),
            ".".to_string(),
            "--ci-artifacts".to_string(),
            fixture.ci_artifact_dir.to_string_lossy().into_owned(),
            "--pages".to_string(),
            fixture.pages_dir.to_string_lossy().into_owned(),
            "--route-comparison".to_string(),
            fixture.route_comparison_path.to_string_lossy().into_owned(),
            "--source-review".to_string(),
            fixture.source_review_path.to_string_lossy().into_owned(),
            "--static-evidence".to_string(),
            fixture.static_evidence_path.to_string_lossy().into_owned(),
            "--format".to_string(),
            "json".to_string(),
            "--output".to_string(),
            output_path.to_string_lossy().into_owned(),
            "--fail-under".to_string(),
            "90".to_string(),
            "--quiet".to_string(),
        ])
        .expect_err("secret marker leak should fail release candidate");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], false);
    assert_eq!(report["checks"]["secret_markers"]["passed"], false);
    assert!(
        report["secret_markers"]["findings"]
            .as_array()
            .expect("secret findings")
            .iter()
            .any(|finding| finding
                .as_str()
                .is_some_and(|finding| finding.contains("CLOUDFLARE_R2_"))),
        "secret marker finding should name the leaked marker: {report:#?}"
    );
    assert!(
        error.to_string().contains("secret marker")
            || error.to_string().contains("release-candidate")
    );
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_release_operations_gate_links_signed_manifest_and_shipping_evidence() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let dashboard_path = write_passing_release_dashboard(dir.path());
    let route_comparison_path = write_passing_public_route_comparison(dir.path());
    let history_path = dir
        .path()
        .join("benchmarks/reports/forge-public-release-history.json");
    let bundle_dir = dir.path().join("release-bundle");

    cli.cmd_forge(&[
        "release-history".to_string(),
        "--dashboard".to_string(),
        dashboard_path.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        route_comparison_path.to_string_lossy().into_owned(),
        "--output".to_string(),
        history_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("release history");
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

    let manifest_path = bundle_dir.join("forge-release-.dx/build-cache/manifest.json");
    let manifest: DxForgeReleaseBundleManifest =
        serde_json::from_slice(&fs::read(&manifest_path).expect("manifest bytes"))
            .expect("manifest json");
    fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&signed_release_manifest_value_for_test(&manifest))
            .expect("signed manifest json"),
    )
    .expect("write signed manifest");

    let trust_regression_path = dir.path().join("forge-trust-regression.json");
    cli.cmd_forge(&[
        "trust-regression".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        trust_regression_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "100".to_string(),
        "--quiet".to_string(),
    ])
    .expect("trust regression");

    let fixture = write_release_candidate_fixture(dir.path());
    let release_candidate_path = dir.path().join("forge-release-candidate.json");
    cli.cmd_forge(&[
        "release-candidate".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--ci-artifacts".to_string(),
        fixture.ci_artifact_dir.to_string_lossy().into_owned(),
        "--pages".to_string(),
        fixture.pages_dir.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        fixture.route_comparison_path.to_string_lossy().into_owned(),
        "--source-review".to_string(),
        fixture.source_review_path.to_string_lossy().into_owned(),
        "--static-evidence".to_string(),
        fixture.static_evidence_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        release_candidate_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("release candidate");

    let public_evidence_project = tempdir().expect("public evidence project");
    let public_evidence_dir = write_public_evidence_export_fixture(public_evidence_project.path());
    let output_path = dir.path().join("forge-release-operations.json");
    cli.cmd_forge(&[
        "release-operations".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--release-manifest".to_string(),
        manifest_path.to_string_lossy().into_owned(),
        "--trust-regression".to_string(),
        trust_regression_path.to_string_lossy().into_owned(),
        "--release-candidate".to_string(),
        release_candidate_path.to_string_lossy().into_owned(),
        "--ci-artifacts".to_string(),
        fixture.ci_artifact_dir.to_string_lossy().into_owned(),
        "--public-evidence".to_string(),
        public_evidence_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("release operations");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["status"], "ready-to-ship");
    assert_eq!(report["checks"]["signed_manifest"]["passed"], true);
    assert_eq!(report["checks"]["trust_regression"]["passed"], true);
    assert_eq!(report["checks"]["release_candidate"]["passed"], true);
    assert_eq!(report["checks"]["ci_artifacts"]["passed"], true);
    assert_eq!(report["checks"]["public_evidence"]["passed"], true);
    assert_eq!(report["checks"]["package_gallery"]["passed"], true);
    assert_eq!(report["signed_manifest"]["publisher_status"], "signed");
    assert_eq!(report["signed_manifest"]["signature_verified"], true);
    assert_eq!(
        report["package_gallery"]["route"],
        "/forge/package-gallery/"
    );
    assert_eq!(report["package_gallery"]["passed"], true);
    assert_eq!(report["package_gallery"]["artifact_count"], 6);
    assert_eq!(
        report["package_gallery"]["package_count"],
        FORGE_WWW_TEMPLATE_PACKAGE_IDS.len()
    );
    assert_eq!(report["trust_regression"]["case_count"], 6);
    assert_eq!(report["release_candidate"]["passed"], true);
    assert!(report["shipping_gate"]
        .as_array()
        .expect("shipping gate")
        .iter()
        .any(|item| item["label"] == "Publish signed release manifest"));
    assert!(report["shipping_gate"]
        .as_array()
        .expect("shipping gate")
        .iter()
        .any(|item| item["label"] == "Publish hosted package gallery"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_publish_plan_maps_public_beta_artifacts_before_deployment() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let dashboard_path = write_passing_release_dashboard(dir.path());
    let route_comparison_path = write_passing_public_route_comparison(dir.path());
    let history_path = dir
        .path()
        .join("benchmarks/reports/forge-public-release-history.json");
    let bundle_dir = dir.path().join("release-bundle");

    cli.cmd_forge(&[
        "release-history".to_string(),
        "--dashboard".to_string(),
        dashboard_path.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        route_comparison_path.to_string_lossy().into_owned(),
        "--output".to_string(),
        history_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("release history");
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

    let manifest_path = bundle_dir.join("forge-release-.dx/build-cache/manifest.json");
    let manifest: DxForgeReleaseBundleManifest =
        serde_json::from_slice(&fs::read(&manifest_path).expect("manifest bytes"))
            .expect("manifest json");
    fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&signed_release_manifest_value_for_test(&manifest))
            .expect("signed manifest json"),
    )
    .expect("write signed manifest");

    let trust_regression_path = dir.path().join("forge-trust-regression.json");
    cli.cmd_forge(&[
        "trust-regression".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        trust_regression_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "100".to_string(),
        "--quiet".to_string(),
    ])
    .expect("trust regression");

    let fixture = write_release_candidate_fixture(dir.path());
    let release_candidate_path = dir.path().join("forge-release-candidate.json");
    cli.cmd_forge(&[
        "release-candidate".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--ci-artifacts".to_string(),
        fixture.ci_artifact_dir.to_string_lossy().into_owned(),
        "--pages".to_string(),
        fixture.pages_dir.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        fixture.route_comparison_path.to_string_lossy().into_owned(),
        "--source-review".to_string(),
        fixture.source_review_path.to_string_lossy().into_owned(),
        "--static-evidence".to_string(),
        fixture.static_evidence_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        release_candidate_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("release candidate");

    let public_evidence_project = tempdir().expect("public evidence project");
    let public_evidence_dir = write_public_evidence_export_fixture(public_evidence_project.path());
    let operations_path = dir.path().join("forge-release-operations.json");
    cli.cmd_forge(&[
        "release-operations".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--release-manifest".to_string(),
        manifest_path.to_string_lossy().into_owned(),
        "--trust-regression".to_string(),
        trust_regression_path.to_string_lossy().into_owned(),
        "--release-candidate".to_string(),
        release_candidate_path.to_string_lossy().into_owned(),
        "--ci-artifacts".to_string(),
        fixture.ci_artifact_dir.to_string_lossy().into_owned(),
        "--public-evidence".to_string(),
        public_evidence_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        operations_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("release operations");

    let registry_smoke_path = dir.path().join("forge-registry-smoke.json");
    cli.cmd_forge(&[
        "registry".to_string(),
        "smoke".to_string(),
        "--remote".to_string(),
        "r2".to_string(),
        "--local".to_string(),
        dir.path()
            .join(".dx/forge-registry-smoke")
            .to_string_lossy()
            .into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        registry_smoke_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("registry smoke");

    let output_path = dir.path().join("forge-publish-plan.json");
    cli.cmd_forge(&[
        "publish-plan".to_string(),
        "--project".to_string(),
        ".".to_string(),
        "--release-bundle".to_string(),
        bundle_dir.to_string_lossy().into_owned(),
        "--pages".to_string(),
        fixture.pages_dir.to_string_lossy().into_owned(),
        "--registry-smoke".to_string(),
        registry_smoke_path.to_string_lossy().into_owned(),
        "--release-operations".to_string(),
        operations_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("publish plan");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["status"], "ready-to-publish-plan");
    assert_eq!(report["checks"]["pages_artifacts"]["passed"], true);
    assert_eq!(report["checks"]["r2_artifacts"]["passed"], true);
    assert_eq!(report["checks"]["local_artifacts"]["passed"], true);
    assert_eq!(report["checks"]["cache_headers"]["passed"], true);
    assert_eq!(report["checks"]["rollback_inputs"]["passed"], true);
    assert_eq!(report["checks"]["no_secret_requirements"]["passed"], true);
    assert_eq!(report["checks"]["no_node_modules"]["passed"], true);
    assert!(report["artifact_targets"]
        .as_array()
        .expect("artifact targets")
        .iter()
        .any(|target| target["channel"] == "pages"
            && target["route"] == "/forge/package-gallery/"
            && target["cache_control"] == "public, max-age=0, must-revalidate"));
    assert!(report["artifact_targets"]
        .as_array()
        .expect("artifact targets")
        .iter()
        .any(|target| target["channel"] == "r2"
            && target["cache_control"] == "public, max-age=31536000, immutable"));
    assert!(report["rollback_inputs"]
        .as_array()
        .expect("rollback inputs")
        .iter()
        .any(|input| input["name"] == "signed_release_manifest" && input["passed"] == true));
    assert_eq!(report["secret_requirements"]["requires_secrets"], false);
    assert!(report["secret_requirements"]["blocked_markers"]
        .as_array()
        .expect("blocked markers")
        .iter()
        .any(|marker| marker == "CLOUDFLARE_R2_"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_release_triage_groups_release_operations_and_publish_plan_failures_for_operators() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let release_operations_path = dir.path().join("forge-release-operations.json");
    let publish_plan_path = dir.path().join("forge-publish-plan.json");
    let output_path = dir.path().join("forge-release-triage.json");

    fs::write(
            &release_operations_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "version": 1,
                "passed": false,
                "score": 62,
                "status": "needs-review",
                "fail_under": 90,
                "checks": {
                    "signed_manifest": {
                        "passed": false,
                        "score": 0,
                        "message": "publisher status `missing` with 0 artifact(s), signature verified=false, artifact integrity=false.",
                        "evidence": "release-bundle/forge-release-.dx/build-cache/manifest.json"
                    },
                    "package_gallery": {
                        "passed": false,
                        "score": 0,
                        "message": "hosted package-gallery route `/forge/package-gallery/` is missing.",
                        "evidence": "release-bundle/forge/package-gallery/index.html"
                    },
                    "no_node_modules": {
                        "passed": false,
                        "score": 75,
                        "message": "node_modules exists at release-bundle/node_modules",
                        "evidence": null
                    }
                },
                "signed_manifest": {
                    "exists": false,
                    "findings": ["release manifest is missing or unreadable"]
                },
                "package_gallery": {
                    "passed": false,
                    "findings": ["package-gallery html missing"]
                },
                "no_node_modules": {
                    "passed": false,
                    "score": 75,
                    "checked_paths": ["release-bundle/node_modules"],
                    "findings": ["node_modules exists at release-bundle/node_modules"]
                },
                "findings": [
                    "signed-manifest: release manifest is missing or unreadable",
                    "package-gallery: package-gallery html missing",
                    "no-node-modules: node_modules exists at release-bundle/node_modules"
                ]
            }))
            .expect("release operations json"),
        )
        .expect("write release operations");
    fs::write(
            &publish_plan_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "version": 1,
                "passed": false,
                "score": 55,
                "status": "needs-review",
                "fail_under": 90,
                "checks": {
                    "local_artifacts": {
                        "passed": false,
                        "score": 0,
                        "message": "release bundle has 0 artifact(s), route_count=0, operations_ready=false.",
                        "evidence": "release-bundle"
                    },
                    "cache_headers": {
                        "passed": false,
                        "score": 0,
                        "message": "4 cache policy row(s) cover Pages HTML, JSON/Markdown, DXPK packets, and R2 objects.",
                        "evidence": null
                    },
                    "rollback_inputs": {
                        "passed": false,
                        "score": 0,
                        "message": "2 required rollback input(s) checked.",
                        "evidence": "forge-release-operations.json"
                    },
                    "no_secret_requirements": {
                        "passed": false,
                        "score": 0,
                        "message": "requires_secrets=true, dry_run=false, scanned 3 path(s).",
                        "evidence": "forge-registry-smoke.json"
                    },
                    "no_node_modules": {
                        "passed": false,
                        "score": 75,
                        "message": "node_modules exists at pages/node_modules",
                        "evidence": null
                    }
                },
                "artifact_targets": [
                    {
                        "channel": "pages",
                        "artifact": "package-gallery",
                        "source": "release-bundle/forge/package-gallery/index.html",
                        "destination": "pages://forge/package-gallery/index.html",
                        "route": "/forge/package-gallery/",
                        "cache_control": "",
                        "required": true,
                        "passed": false,
                        "message": "Publish from the verified release bundle to the Pages public surface. Missing source."
                    }
                ],
                "cache_headers": [
                    {
                        "channel": "pages",
                        "pattern": "**/*.html",
                        "cache_control": "",
                        "required": true,
                        "passed": false,
                        "reason": "HTML routes need rollback-friendly cache policy."
                    }
                ],
                "rollback_inputs": [
                    {
                        "name": "signed_release_manifest",
                        "path": "release-bundle/forge-release-.dx/build-cache/manifest.json",
                        "required": true,
                        "exists": false,
                        "passed": false,
                        "message": "Signed manifest pins the publish artifact set and publisher identity. Missing input."
                    }
                ],
                "secret_requirements": {
                    "requires_secrets": true,
                    "registry_operations_dry_run": false,
                    "blocked_markers": ["CLOUDFLARE_R2_"],
                    "scanned_paths": ["release-bundle", "pages", "forge-registry-smoke.json"],
                    "passed": false,
                    "score": 0,
                    "findings": [
                        "registry smoke requires secrets",
                        "registry operations are not all dry-run"
                    ]
                },
                "no_node_modules": {
                    "passed": false,
                    "score": 75,
                    "checked_paths": ["pages/node_modules"],
                    "findings": ["node_modules exists at pages/node_modules"]
                },
                "findings": [
                    "local-artifacts: release bundle has 0 artifact(s)",
                    "cache-headers: target cache coverage=false",
                    "rollback-inputs: Signed manifest pins the publish artifact set and publisher identity. Missing input.",
                    "secret-requirements: registry smoke requires secrets",
                    "no-node-modules: node_modules exists at pages/node_modules"
                ]
            }))
            .expect("publish plan json"),
        )
        .expect("write publish plan");

    cli.cmd_forge(&[
        "release-triage".to_string(),
        "--release-operations".to_string(),
        release_operations_path.to_string_lossy().into_owned(),
        "--publish-plan".to_string(),
        publish_plan_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("release triage");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], false);
    assert_eq!(report["status"], "operator-action-required");
    assert_eq!(report["shipping_ready"], false);
    assert_eq!(report["groups"]["missing_artifacts"]["passed"], false);
    assert_eq!(report["groups"]["secret_risk"]["passed"], false);
    assert_eq!(report["groups"]["cache_policy"]["passed"], false);
    assert_eq!(report["groups"]["rollback_readiness"]["passed"], false);
    assert_eq!(report["groups"]["dependency_boundary"]["passed"], false);
    assert!(report["groups"]["missing_artifacts"]["findings"]
        .as_array()
        .expect("missing artifact findings")
        .iter()
        .any(|finding| finding
            .as_str()
            .is_some_and(|finding| finding.contains("signed manifest"))));
    assert!(report["groups"]["secret_risk"]["findings"]
        .as_array()
        .expect("secret risk findings")
        .iter()
        .any(|finding| finding
            .as_str()
            .is_some_and(|finding| finding.contains("requires secrets"))));
    assert!(report["first_actions"]
        .as_array()
        .expect("first actions")
        .iter()
        .any(|action| action
            .as_str()
            .is_some_and(|action| action.contains("Restore missing artifacts"))));
}

#[test]
fn forge_ci_snippets_write_portable_beta_promotion_templates() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let out_dir = dir.path().join("forge-ci-snippets");
    let output_path = dir.path().join("forge-ci-snippets.json");
    let key_dir = dir.path().join("publisher");

    cli.cmd_forge(&[
        "publisher-key".to_string(),
        "generate".to_string(),
        "--out".to_string(),
        key_dir.to_string_lossy().into_owned(),
        "--signer".to_string(),
        "essencefromexistence".to_string(),
        "--quiet".to_string(),
    ])
    .expect("publisher key");

    cli.cmd_forge(&[
        "ci-snippets".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().into_owned(),
        "--publisher-key".to_string(),
        key_dir
            .join("publisher-key.private.json")
            .to_string_lossy()
            .into_owned(),
        "--artifact-dir".to_string(),
        ".dx/ci".to_string(),
        "--pages-dir".to_string(),
        ".dx/forge-pages".to_string(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("forge ci snippets");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["status"], "ready-to-copy-ci-snippets");
    assert_eq!(report["snippet_count"], 3);
    assert_eq!(report["provenance"]["signed"], true);
    assert_eq!(report["provenance"]["signature_verified"], true);
    assert_eq!(report["provenance"]["artifact_count"], 4);
    for kind in ["github_actions", "powershell", "generic_runner"] {
        assert!(
            report["snippets"]
                .as_array()
                .expect("snippets")
                .iter()
                .any(|snippet| snippet["kind"] == kind && snippet["passed"] == true),
            "missing snippet kind {kind}: {report:#?}"
        );
    }

    let github = fs::read_to_string(out_dir.join("github-actions/forge-ci.yml"))
        .expect("github actions snippet");
    let powershell =
        fs::read_to_string(out_dir.join("powershell/forge-ci.ps1")).expect("ps snippet");
    let generic = fs::read_to_string(out_dir.join("generic/forge-ci.sh")).expect("generic snippet");
    let readme = fs::read_to_string(out_dir.join("README.md")).expect("snippet readme");

    for content in [&github, &powershell, &generic, &readme] {
        assert!(content.contains("scripts/ci/forge-ci.ps1"));
        assert!(content.contains("forge release-triage"));
        assert!(content.contains("forge beta-artifact-verify"));
        assert!(content.contains("forge-release-operations.json"));
        assert!(content.contains("forge-publish-plan.json"));
        assert!(content.contains("forge-beta-artifact-verify"));
        assert!(content.contains("FailUnder 90") || content.contains("FAIL_UNDER=90"));
        assert!(!content.contains("CLOUDFLARE_R2_SECRET"));
    }
    assert!(github.contains("actions/upload-artifact@v4"));
    assert!(powershell.contains("$ArtifactDir = \".dx/ci\""));
    assert!(generic.contains("pwsh ./scripts/ci/forge-ci.ps1"));
    let provenance = read_json_value(out_dir.join("forge-ci-snippets-provenance.json"));
    assert_eq!(provenance["publisher_identity"]["status"], "signed");
    assert_eq!(provenance["artifact_integrity"]["verified_locally"], true);
    assert!(provenance["artifacts"]
        .as_array()
        .expect("provenance artifacts")
        .iter()
        .any(|artifact| artifact["path"] == "github-actions/forge-ci.yml"
            && artifact["artifact_type"] == "ci-snippet"));
    assert!(provenance["artifacts"]
        .as_array()
        .expect("provenance artifacts")
        .iter()
        .any(|artifact| artifact["path"] == "README.md"
            && artifact["artifact_type"] == "snippet-index"));
    assert!(out_dir.join("forge-ci-snippets-provenance.md").is_file());
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_operator_dashboard_joins_beta_review_artifacts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let release_triage_path = dir.path().join("forge-release-triage.json");
    let beta_verify_path = dir.path().join("forge-beta-artifact-verify.json");
    let ci_snippets_path = dir.path().join("forge-ci-snippets.json");
    let installability_path = dir.path().join("forge-installability-snapshot.json");
    let installability_history_path = dir.path().join("forge-installability-history.json");
    let output_path = dir.path().join("forge-operator-dashboard.json");

    fs::write(
        &release_triage_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "passed": true,
            "shipping_ready": true,
            "status": "ready-for-operator-review",
            "score": 94,
            "first_actions": [],
            "findings": [],
            "groups": {
                "missing_artifacts": { "passed": true, "score": 100, "finding_count": 0 },
                "secret_risk": { "passed": true, "score": 100, "finding_count": 0 },
                "cache_policy": { "passed": true, "score": 100, "finding_count": 0 },
                "rollback_readiness": { "passed": true, "score": 100, "finding_count": 0 },
                "dependency_boundary": { "passed": true, "score": 100, "finding_count": 0 }
            }
        }))
        .expect("release triage json"),
    )
    .expect("write release triage");
    fs::write(
        &beta_verify_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "passed": true,
            "status": "ready-for-downloaded-beta-install",
            "score": 97,
            "requires_rebuild": false,
            "artifact_targets": [
                { "channel": "pages", "passed": true },
                { "channel": "r2", "passed": true }
            ],
            "secret_requirements": { "requires_secrets": false },
            "no_node_modules": { "passed": true },
            "findings": []
        }))
        .expect("beta verify json"),
    )
    .expect("write beta verify");
    fs::write(
        &ci_snippets_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "passed": true,
            "status": "ready-to-copy-ci-snippets",
            "score": 100,
            "snippet_count": 3,
            "provenance": {
                "signed": true,
                "signature_verified": true,
                "artifact_count": 4
            },
            "findings": []
        }))
        .expect("ci snippets json"),
    )
    .expect("write ci snippets");
    fs::write(
            &installability_path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "passed": true,
                "score": 100,
                "scope": {
                    "no_package_installs_run": true,
                    "no_node_modules_created": true,
                    "not_live_npm_or_shadcn_benchmark": true
                },
                "rows": [
                    { "id": "dx-forge-beta-install", "time_ms": 1200, "artifact_bytes": 180000, "package_install_ran": false },
                    { "id": "dx-forge-beta-upgrade", "time_ms": 90, "artifact_bytes": 36000, "package_install_ran": false }
                ],
                "findings": []
            }))
            .expect("installability json"),
        )
        .expect("write installability");
    fs::write(
        &installability_history_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "snapshot_count": 2,
            "latest": {
                "generated_at": "2026-05-19T00:00:00.000Z",
                "trend": "improved",
                "delta": {
                    "install_time_ms": -220,
                    "upgrade_time_ms": -27
                }
            },
            "checks": {
                "latest_passed": { "passed": true },
                "no_package_installs": { "passed": true }
            }
        }))
        .expect("installability history json"),
    )
    .expect("write installability history");

    cli.cmd_forge(&[
        "operator-dashboard".to_string(),
        "--release-triage".to_string(),
        release_triage_path.to_string_lossy().into_owned(),
        "--beta-artifact-verify".to_string(),
        beta_verify_path.to_string_lossy().into_owned(),
        "--ci-snippets".to_string(),
        ci_snippets_path.to_string_lossy().into_owned(),
        "--installability".to_string(),
        installability_path.to_string_lossy().into_owned(),
        "--installability-history".to_string(),
        installability_history_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("operator dashboard");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["status"], "ready-for-beta-operator-review");
    assert_eq!(report["score"], 94);
    assert_eq!(report["checks"]["release_triage"]["passed"], true);
    assert_eq!(report["checks"]["beta_artifacts"]["passed"], true);
    assert_eq!(report["checks"]["ci_snippet_provenance"]["passed"], true);
    assert_eq!(report["checks"]["installability"]["passed"], true);
    assert_eq!(report["checks"]["installability_history"]["passed"], true);
    assert_eq!(report["summary"]["pages_targets"], 1);
    assert_eq!(report["summary"]["r2_targets"], 1);
    assert_eq!(report["summary"]["ci_snippets"], 3);
    assert_eq!(report["summary"]["install_time_ms"], 1200);
    assert_eq!(report["summary"]["upgrade_time_ms"], 90);
    assert_eq!(report["summary"]["installability_trend"], "improved");
    assert_eq!(report["findings"].as_array().expect("findings").len(), 0);
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_release_bundle_inspector_summarizes_signed_bundle_for_operator_review() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let bundle_dir =
        write_signed_adoption_release_bundle_fixture(dir.path(), "release-bundle-adoption");
    let output_path = dir.path().join("forge-release-bundle-inspect.json");

    cli.cmd_forge(&[
        "release-bundle-inspect".to_string(),
        "--bundle".to_string(),
        bundle_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("release bundle inspector");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["status"], "ready-for-beta-review");
    assert_eq!(report["checks"]["signed_manifest"]["passed"], true);
    assert_eq!(report["checks"]["hosted_artifacts"]["passed"], true);
    assert_eq!(report["checks"]["rollback_inputs"]["passed"], true);
    assert_eq!(report["checks"]["cache_policy"]["passed"], true);
    assert_eq!(report["checks"]["package_gallery"]["passed"], true);
    assert_eq!(report["checks"]["no_node_modules"]["passed"], true);
    assert_eq!(report["signed_manifest"]["publisher_status"], "signed");
    assert_eq!(report["signed_manifest"]["signature_verified"], true);
    assert_eq!(
        report["package_gallery"]["package_count"],
        FORGE_WWW_TEMPLATE_PACKAGE_IDS.len()
    );
    assert!(report["hosted_artifacts"]
        .as_array()
        .expect("hosted artifacts")
        .iter()
        .any(|target| target["channel"] == "pages"
            && target["route"] == "/forge/package-gallery/"
            && target["cache_control"] == "public, max-age=0, must-revalidate"));
    assert!(report["rollback_inputs"]
        .as_array()
        .expect("rollback inputs")
        .iter()
        .any(|input| input["name"] == "signed_release_manifest" && input["passed"] == true));
    assert!(report["cache_headers"]
        .as_array()
        .expect("cache headers")
        .iter()
        .any(|header| header["pattern"] == "**/*.dxp"
            && header["cache_control"] == "public, max-age=31536000, immutable"));
    assert!(!dir.path().join("node_modules").exists());
    assert!(!bundle_dir.join("node_modules").exists());
}

#[test]
fn forge_beta_artifact_verify_validates_downloaded_bundle_pages_and_r2_evidence_without_rebuild() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let bundle_dir =
        write_signed_adoption_release_bundle_fixture(dir.path(), "release-bundle-adoption");
    let pages_project = tempdir().expect("pages project");
    let pages_cli = Cli::with_cwd(pages_project.path().to_path_buf());
    let ci_project = tempdir().expect("ci project");
    let ci_cli = Cli::with_cwd(ci_project.path().to_path_buf());
    let ci_artifact_dir = ci_project.path().join("ci-artifacts");
    let pages_dir = pages_project.path().join("forge-pages");
    let registry_smoke_path = dir.path().join("forge-registry-smoke.json");
    let output_path = dir.path().join("forge-beta-artifact-verify.json");

    ci_cli
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
    cli.cmd_forge(&[
        "registry".to_string(),
        "smoke".to_string(),
        "--remote".to_string(),
        "r2".to_string(),
        "--local".to_string(),
        dir.path()
            .join(".dx/forge-registry-smoke")
            .to_string_lossy()
            .into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        registry_smoke_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("registry smoke");

    cli.cmd_forge(&[
        "beta-artifact-verify".to_string(),
        "--release-bundle".to_string(),
        bundle_dir.to_string_lossy().into_owned(),
        "--pages".to_string(),
        pages_dir.to_string_lossy().into_owned(),
        "--registry-smoke".to_string(),
        registry_smoke_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("downloaded beta artifact verifier");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["status"], "ready-for-downloaded-beta-install");
    assert_eq!(report["requires_rebuild"], false);
    assert_eq!(report["checks"]["release_bundle"]["passed"], true);
    assert_eq!(report["checks"]["pages_bundle"]["passed"], true);
    assert_eq!(report["checks"]["r2_evidence"]["passed"], true);
    assert_eq!(report["checks"]["cache_policy"]["passed"], true);
    assert_eq!(report["checks"]["rollback_inputs"]["passed"], true);
    assert_eq!(report["checks"]["no_secret_requirements"]["passed"], true);
    assert_eq!(report["checks"]["no_node_modules"]["passed"], true);
    assert_eq!(report["signed_manifest"]["signature_verified"], true);
    assert!(report["artifact_targets"]
        .as_array()
        .expect("artifact targets")
        .iter()
        .any(|target| target["channel"] == "pages"
            && target["route"] == "/forge/package-gallery/"
            && target["cache_control"] == "public, max-age=0, must-revalidate"));
    assert!(report["artifact_targets"]
        .as_array()
        .expect("artifact targets")
        .iter()
        .any(|target| target["channel"] == "r2"
            && target["cache_control"] == "public, max-age=31536000, immutable"));
    assert_eq!(report["secret_requirements"]["requires_secrets"], false);
    assert!(!dir.path().join("node_modules").exists());
    assert!(!bundle_dir.join("node_modules").exists());
    assert!(!pages_dir.join("node_modules").exists());
}

#[test]
fn forge_route_comparison_guard_requires_changelog_route() {
    let dir = tempdir().expect("tempdir");
    let route_comparison_path = write_public_route_comparison_without_changelog(dir.path());

    let report = verify_release_dashboard_route_comparison(&route_comparison_path)
        .expect("route comparison guard report");

    assert_eq!(report.passed, false);
    assert!(report
        .missing_routes
        .contains(&"/forge/changelog".to_string()));
    assert!(
        report
            .findings
            .iter()
            .any(|finding| finding.contains("/forge/changelog")),
        "guard findings should name the missing changelog route: {:?}",
        report.findings
    );
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_release_history_records_dashboard_and_route_comparison() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let dashboard_path = write_passing_release_dashboard(dir.path());
    let route_comparison_path = write_passing_public_route_comparison(dir.path());
    let history_path = dir
        .path()
        .join("benchmarks/reports/forge-public-release-history.json");

    for _ in 0..2 {
        cli.cmd_forge(&[
            "release-history".to_string(),
            "--dashboard".to_string(),
            dashboard_path.to_string_lossy().into_owned(),
            "--route-comparison".to_string(),
            route_comparison_path.to_string_lossy().into_owned(),
            "--output".to_string(),
            history_path.to_string_lossy().into_owned(),
            "--format".to_string(),
            "json".to_string(),
            "--quiet".to_string(),
        ])
        .expect("forge release history");
    }

    let history = read_json_value(history_path.clone());
    assert_eq!(history["records"].as_array().expect("records").len(), 1);
    assert_eq!(history["records"][0]["dashboard"]["score"], 93);
    assert_eq!(history["records"][0]["dashboard"]["passed"], true);
    assert_eq!(history["records"][0]["dashboard"]["no_node_modules"], true);
    assert_eq!(history["records"][0]["route_comparison"]["route_count"], 6);
    assert!(history["records"][0]["regression_findings"]
        .as_array()
        .expect("regression findings")
        .is_empty());
    assert_eq!(
        history["records"][0]["route_comparison"]["total_brotli_bytes"],
        6014
    );
    assert_eq!(
        history["records"][0]["route_comparison"]["routes"]
            .as_array()
            .expect("routes")
            .len(),
        6
    );

    let markdown_path = history_path.with_extension("md");
    let markdown = fs::read_to_string(markdown_path).expect("release history markdown");
    assert!(markdown.contains("# Forge Public Release History"));
    assert!(markdown.contains("Dashboard score: 93 / 100"));
    assert!(markdown.contains("| /forge/evidence | forge-evidence | static |"));
    assert!(markdown.contains("| /forge/releases | forge-releases | static |"));
    assert!(!markdown.contains("CLOUDFLARE_R2_"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_release_history_detects_release_regressions() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let dashboard_path = write_passing_release_dashboard(dir.path());
    let route_comparison_path = write_passing_public_route_comparison(dir.path());
    let history_path = dir
        .path()
        .join("benchmarks/reports/forge-public-release-history.json");

    cli.cmd_forge(&[
        "release-history".to_string(),
        "--dashboard".to_string(),
        dashboard_path.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        route_comparison_path.to_string_lossy().into_owned(),
        "--output".to_string(),
        history_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("baseline release history");

    std::thread::sleep(std::time::Duration::from_millis(2));
    let regressing_dashboard_path = write_regressing_release_dashboard(dir.path());
    let regressing_route_comparison_path = write_regressing_public_route_comparison(dir.path());
    cli.cmd_forge(&[
        "release-history".to_string(),
        "--dashboard".to_string(),
        regressing_dashboard_path.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        regressing_route_comparison_path
            .to_string_lossy()
            .into_owned(),
        "--output".to_string(),
        history_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("regressing release history");

    let history = read_json_value(history_path);
    assert_eq!(history["records"].as_array().expect("records").len(), 2);
    let findings = history["records"][0]["regression_findings"]
        .as_array()
        .expect("regression findings")
        .iter()
        .filter_map(|finding| finding.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    for expected in [
        "Release dashboard failed",
        "Route `/forge/ci` failed its configured public delivery budget.",
        "Dashboard score dropped from 93 to 88.",
        "Total Brotli public payload grew from 6014 B to 7000 B.",
        "Public route `/forge/evidence` disappeared from release history.",
        "Route `/forge` Brotli payload grew from 1240 B to 1500 B.",
    ] {
        assert!(
            findings.contains(expected),
            "release regression findings missing `{expected}` in {findings}"
        );
    }
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_release_history_allows_configured_expected_route_additions() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    fs::write(
            dir.path().join("dx"),
            "forge.release_history.expected_route_additions=2\nforge.release_history.max_added_route_decoded_bytes=10000\nforge.release_history.max_added_route_brotli_bytes=2000\n",
        )
        .expect("release-history policy config");
    let dashboard_path = write_passing_release_dashboard(dir.path());
    let baseline_route_comparison_path = write_four_route_public_route_comparison(dir.path());
    let history_path = dir
        .path()
        .join("benchmarks/reports/forge-public-release-history.json");

    cli.cmd_forge(&[
        "release-history".to_string(),
        "--dashboard".to_string(),
        dashboard_path.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        baseline_route_comparison_path
            .to_string_lossy()
            .into_owned(),
        "--output".to_string(),
        history_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("baseline release history");

    std::thread::sleep(std::time::Duration::from_millis(2));
    let added_route_comparison_path = write_passing_public_route_comparison(dir.path());
    cli.cmd_forge(&[
        "release-history".to_string(),
        "--dashboard".to_string(),
        dashboard_path.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        added_route_comparison_path.to_string_lossy().into_owned(),
        "--output".to_string(),
        history_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("release history with expected route addition");

    let history = read_json_value(history_path);
    assert_eq!(history["records"].as_array().expect("records").len(), 2);
    assert_eq!(history["records"][0]["route_comparison"]["route_count"], 6);
    assert!(
        history["records"][0]["regression_findings"]
            .as_array()
            .expect("regression findings")
            .is_empty(),
        "expected route addition should not create payload regression findings: {}",
        history["records"][0]["regression_findings"]
    );
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_launch_changelog_generates_honest_notes_from_release_history() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    fs::write(
            dir.path().join("dx"),
            "forge.release_history.expected_route_additions=2\nforge.release_history.max_added_route_decoded_bytes=10000\nforge.release_history.max_added_route_brotli_bytes=2000\n",
        )
        .expect("release-history policy config");
    let dashboard_path = write_passing_release_dashboard(dir.path());
    let baseline_route_comparison_path = write_four_route_public_route_comparison(dir.path());
    let history_path = dir
        .path()
        .join("benchmarks/reports/forge-public-release-history.json");

    cli.cmd_forge(&[
        "release-history".to_string(),
        "--dashboard".to_string(),
        dashboard_path.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        baseline_route_comparison_path
            .to_string_lossy()
            .into_owned(),
        "--output".to_string(),
        history_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("baseline release history");

    std::thread::sleep(std::time::Duration::from_millis(2));
    let added_route_comparison_path = write_passing_public_route_comparison(dir.path());
    cli.cmd_forge(&[
        "release-history".to_string(),
        "--dashboard".to_string(),
        dashboard_path.to_string_lossy().into_owned(),
        "--route-comparison".to_string(),
        added_route_comparison_path.to_string_lossy().into_owned(),
        "--output".to_string(),
        history_path.to_string_lossy().into_owned(),
        "--quiet".to_string(),
    ])
    .expect("release history with expected route addition");

    let changelog_path = dir.path().join("forge-launch-changelog.md");
    cli.cmd_forge(&[
        "launch-changelog".to_string(),
        "--history".to_string(),
        history_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "markdown".to_string(),
        "--output".to_string(),
        changelog_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("forge launch changelog");
    let changelog_json_path = dir.path().join("forge-launch-changelog.json");
    cli.cmd_forge(&[
        "launch-changelog".to_string(),
        "--history".to_string(),
        history_path.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--output".to_string(),
        changelog_json_path.to_string_lossy().into_owned(),
        "--fail-under".to_string(),
        "90".to_string(),
        "--quiet".to_string(),
    ])
    .expect("forge launch changelog json");

    let changelog = fs::read_to_string(changelog_path).expect("launch changelog");
    assert_forge_json_snapshot(
        "launch-changelog-shape.json",
        forge_launch_changelog_shape_contract(&read_json_value(changelog_json_path)),
    );
    assert_forge_launch_changelog_markdown_snapshot(
        "forge-public-launch-changelog.md",
        &changelog,
        dir.path(),
    );
    assert!(changelog.contains("# DX Forge Public Launch Changelog"));
    assert!(changelog.contains("- Status: `passing`"));
    assert!(changelog.contains("Added public routes: `/forge/changelog`, `/forge/releases`"));
    assert!(changelog.contains("No release-regression findings were recorded"));
    assert!(changelog.contains("does not claim live production traffic"));
    assert!(changelog.contains("universal npm replacement"));
    assert!(!changelog.contains("CLOUDFLARE_R2_"));
    assert!(!changelog.contains("DX_FORGE_R2_LIVE"));
    assert!(!dir.path().join("node_modules").exists());
}

#[test]
fn forge_release_bundle_assembles_and_verifies_public_surface() {
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

    for artifact in [
        "forge.html",
        "forge/index.html",
        "forge/scorecard.html",
        "forge/scorecard/index.html",
        "forge/ci.html",
        "forge/ci/index.html",
        "forge/evidence.html",
        "forge/evidence/index.html",
        "forge/releases.html",
        "forge/releases/index.html",
        "forge/changelog.html",
        "forge/changelog/index.html",
        "forge/changelog.claims.json",
        "forge/changelog.dxp",
        "forge/changelog.proof.json",
        "forge-readiness-badge.json",
        "forge-public-route-comparison.json",
        "forge-public-release-history.json",
        "forge-public-launch-changelog.json",
        "forge-public-launch-changelog.md",
        "forge/package-gallery/index.html",
        "forge/package-gallery.json",
        "forge/package-gallery.md",
        "forge/migration-gallery/index.html",
        "forge/migration-gallery.json",
        "forge/migration-gallery.md",
        "forge-release-.dx/build-cache/manifest.json",
        "forge-release-manifest.md",
    ] {
        assert!(
            bundle_dir.join(artifact).is_file(),
            "release bundle missing `{artifact}`"
        );
    }

    let manifest: DxForgeReleaseBundleManifest = serde_json::from_slice(
        &std::fs::read(bundle_dir.join("forge-release-.dx/build-cache/manifest.json")).unwrap(),
    )
    .expect("release manifest json");
    assert_eq!(manifest.hash_algorithm, "blake3");
    assert!(manifest.artifact_count >= 30);
    assert_eq!(
        manifest.artifact_integrity.scheme,
        "dx-forge-release-artifact-integrity-v1"
    );
    assert_eq!(manifest.artifact_integrity.hash_algorithm, "blake3");
    assert_eq!(
        manifest.artifact_integrity.digest,
        manifest.integrity.digest
    );
    assert_eq!(
        manifest.artifact_integrity.artifact_count,
        manifest.artifacts.len()
    );
    assert!(manifest.artifact_integrity.verified_locally);
    assert_eq!(
        manifest.publisher_identity.scheme,
        "dx-forge-release-publisher-identity-v1"
    );
    assert_eq!(manifest.publisher_identity.status, "unsigned");
    assert_eq!(manifest.publisher_identity.signature, None);
    assert!(manifest.publisher_identity.message.contains("BLAKE3"));
    assert!(manifest.artifacts.iter().any(|artifact| {
        artifact.path == "forge.dxp" && artifact.artifact_type == "dxpk-packet"
    }));
    assert!(manifest.artifacts.iter().any(|artifact| {
        artifact.path == "forge.claims.json" && artifact.artifact_type == "claims"
    }));
    assert!(manifest.artifacts.iter().any(|artifact| {
        artifact.path == "forge-public-route-comparison.json"
            && artifact.artifact_type == "route-comparison"
    }));
    assert!(manifest.artifacts.iter().any(|artifact| {
        artifact.path == "forge-public-release-history.json"
            && artifact.artifact_type == "release-history"
    }));
    assert!(manifest.artifacts.iter().any(|artifact| {
        artifact.path == "forge-public-launch-changelog.json"
            && artifact.artifact_type == "launch-changelog"
    }));
    assert!(manifest.artifacts.iter().any(|artifact| {
        artifact.path == "forge/package-gallery/index.html"
            && artifact.artifact_type == "package-gallery"
            && artifact.route.as_deref() == Some("/forge/package-gallery/")
    }));
    assert!(manifest.artifacts.iter().any(|artifact| {
        artifact.path == "forge/package-gallery.json"
            && artifact.artifact_type == "package-gallery"
            && artifact.route.as_deref() == Some("/forge/package-gallery/")
    }));
    assert!(manifest.artifacts.iter().any(|artifact| {
        artifact.path == "forge/migration-gallery/index.html"
            && artifact.artifact_type == "migration-gallery"
            && artifact.route.as_deref() == Some("/forge/migration-gallery/")
    }));
    assert!(manifest.artifacts.iter().any(|artifact| {
        artifact.path == "forge/migration-gallery.json"
            && artifact.artifact_type == "migration-gallery"
            && artifact.route.as_deref() == Some("/forge/migration-gallery/")
    }));
    assert!(manifest.artifacts.iter().any(|artifact| {
        artifact.path == "forge/changelog.html"
            && artifact.artifact_type == "route-html"
            && artifact.route.as_deref() == Some("/forge/changelog")
    }));
    let changelog = fs::read_to_string(bundle_dir.join("forge-public-launch-changelog.md"))
        .expect("bundle launch changelog");
    assert!(changelog.contains("# DX Forge Public Launch Changelog"));
    assert!(changelog.contains("does not claim live production traffic"));
    let route_changelog =
        fs::read_to_string(bundle_dir.join("forge/changelog.html")).expect("route changelog");
    assert!(route_changelog.contains("DX Forge Public Launch Changelog"));
    assert!(route_changelog.contains("forge-public-launch-changelog.json"));
    let hosted_gallery = fs::read_to_string(bundle_dir.join("forge/package-gallery/index.html"))
        .expect("hosted package gallery");
    for expected in [
        "DX Forge Package Gallery",
        "Trust signals",
        "Migration guides",
        "shadcn/ui/button",
        "auth/better-auth",
        "not a universal npm replacement",
    ] {
        assert!(
            hosted_gallery.contains(expected),
            "hosted package gallery missing `{expected}`"
        );
    }
    let hosted_gallery_json = read_json_value(bundle_dir.join("forge/package-gallery.json"));
    assert_eq!(hosted_gallery_json["route"], "/forge/package-gallery/");
    assert_eq!(hosted_gallery_json["passed"], true);
    assert_eq!(
        hosted_gallery_json["migration_gallery"]["route"],
        "/forge/migration-gallery/"
    );
    assert_eq!(
        hosted_gallery_json["packages"]
            .as_array()
            .expect("hosted gallery packages")
            .len(),
        FORGE_WWW_TEMPLATE_PACKAGE_IDS.len()
    );
    let hosted_migration_gallery =
        fs::read_to_string(bundle_dir.join("forge/migration-gallery/index.html"))
            .expect("hosted migration gallery");
    for expected in [
        "DX Forge Migration Gallery",
        "Supported scope",
        "Manual gaps",
        "Package evidence",
        "Payload comparison boundaries",
        "migration/static-site",
    ] {
        assert!(
            hosted_migration_gallery.contains(expected),
            "hosted migration gallery missing `{expected}`"
        );
    }
    let hosted_migration_gallery_json =
        read_json_value(bundle_dir.join("forge/migration-gallery.json"));
    assert_eq!(
        hosted_migration_gallery_json["route"],
        "/forge/migration-gallery/"
    );
    assert_eq!(hosted_migration_gallery_json["passed"], true);
    assert!(!bundle_dir.join("forge/changelog.dxp.js").exists());

    cli.cmd_forge(&[
        "release-bundle".to_string(),
        "--verify".to_string(),
        bundle_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        "json".to_string(),
        "--quiet".to_string(),
    ])
    .expect("release bundle verify");

    let manifest_path = bundle_dir.join("forge-release-.dx/build-cache/manifest.json");
    let signed_manifest = signed_release_manifest_value_for_test(&manifest);
    std::fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&signed_manifest).unwrap(),
    )
    .expect("write valid signed manifest");
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
    .expect("signed release manifest should verify publisher identity");

    let mut signed_without_identity = serde_json::to_value(&manifest).expect("manifest json value");
    signed_without_identity["publisher_identity"]["status"] =
        serde_json::Value::String("signed".to_string());
    std::fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&signed_without_identity).unwrap(),
    )
    .expect("write signed manifest without identity");
    let error = cli
        .cmd_forge(&[
            "release-bundle".to_string(),
            "--verify".to_string(),
            bundle_dir.to_string_lossy().into_owned(),
            "--format".to_string(),
            "json".to_string(),
            "--fail-under".to_string(),
            "0".to_string(),
            "--quiet".to_string(),
        ])
        .expect_err("signed manifest without identity should fail verification");
    assert!(
        format!("{error:?}").contains("publisher identity"),
        "unsigned signature upgrade error should mention publisher identity: {error:?}"
    );
    std::fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&manifest).unwrap(),
    )
    .expect("restore manifest");

    let mut unverifiable_signed_manifest =
        serde_json::to_value(&manifest).expect("manifest json value");
    unverifiable_signed_manifest["publisher_identity"]["status"] =
        serde_json::Value::String("signed".to_string());
    unverifiable_signed_manifest["publisher_identity"]["signer"] =
        serde_json::Value::String("dx-forge-test-signer".to_string());
    unverifiable_signed_manifest["publisher_identity"]["key_id"] =
        serde_json::Value::String("dx-forge-test-key".to_string());
    unverifiable_signed_manifest["publisher_identity"]["signature"] =
        serde_json::Value::String("not-a-real-signature".to_string());
    unverifiable_signed_manifest["publisher_identity"]["signed_at"] =
        serde_json::Value::String("2026-05-18T00:00:00Z".to_string());
    unverifiable_signed_manifest["integrity"]["signed"] = serde_json::Value::Bool(true);
    unverifiable_signed_manifest["integrity"]["signature"] =
        serde_json::Value::String("not-a-real-signature".to_string());
    std::fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&unverifiable_signed_manifest).unwrap(),
    )
    .expect("write unverifiable signed manifest");
    let error = cli
        .cmd_forge(&[
            "release-bundle".to_string(),
            "--verify".to_string(),
            bundle_dir.to_string_lossy().into_owned(),
            "--format".to_string(),
            "json".to_string(),
            "--fail-under".to_string(),
            "0".to_string(),
            "--quiet".to_string(),
        ])
        .expect_err("unverifiable signed manifest should fail verification");
    assert!(
        format!("{error:?}").contains("signature"),
        "unverified signature error should mention signature verification: {error:?}"
    );
    std::fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&manifest).unwrap(),
    )
    .expect("restore manifest");

    std::fs::write(bundle_dir.join("forge.html"), "<h1>tampered</h1>")
        .expect("tamper bundle artifact");
    let error = cli
        .cmd_forge(&[
            "release-bundle".to_string(),
            "--verify".to_string(),
            bundle_dir.to_string_lossy().into_owned(),
            "--format".to_string(),
            "json".to_string(),
            "--fail-under".to_string(),
            "0".to_string(),
            "--quiet".to_string(),
        ])
        .expect_err("tampered release bundle should fail manifest verification");
    assert!(
        format!("{error:?}").contains("release manifest"),
        "tampered bundle error should mention release manifest: {error:?}"
    );

    assert!(!project.path().join("node_modules").exists());
    assert!(!bundle_dir.join("node_modules").exists());
}
