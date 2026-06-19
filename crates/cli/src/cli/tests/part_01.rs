#[test]
fn test_cli_new() {
    let cli = Cli::new();
    assert!(cli.cwd.exists() || cli.cwd == PathBuf::from("."));
}

#[test]
fn dx_new_creates_react_familiar_no_node_modules_starter() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("contract-app").expect("dx new");

    let project = dir.path().join("contract-app");
    for path in [
        "app/page.tsx",
        "app/layout.tsx",
        "components/local/WelcomeCard.tsx",
        "components/ui/button.tsx",
        "components/ui/card.tsx",
        "components/ui/scroll-area.tsx",
        "server/actions.ts",
        "styles/globals.css",
        "styles/theme.css",
        "styles/generated.css",
        ".dx/forge/source-manifest.json",
        ".dx/forge/docs/dx-www-starter-ui.md",
        ".dx/forge",
        ".dx/serializer/dx.machine",
        "dx",
    ] {
        assert!(project.join(path).exists(), "{path}");
    }
    for path in [
        "package.json",
        "package-lock.json",
        "pnpm-lock.yaml",
        "yarn.lock",
        "bun.lock",
        "bun.lockb",
        "next.config.js",
        "next.config.mjs",
        "next.config.ts",
        "source.config.ts",
        "next-env.d.ts",
        "biome.json",
        "tailwind.config.ts",
        "postcss.config.mjs",
        "components.json",
        "tsconfig.json",
        "dx-env.d.ts",
    ] {
        assert!(!project.join(path).exists(), "{path}");
    }
    assert!(!project.join("dx.config.toml").exists());
    assert!(!project.join(".dx/serializer/dx.llm").exists());
    assert!(!project.join("node_modules").exists());
    let report = check_dx_project_with_options(
        &project,
        DxCheckOptions {
            project_contract: true,
        },
    )
    .expect("project contract check");
    let contract = report
        .sections
        .iter()
        .find(|section| section.name == "project-contract")
        .expect("project-contract");
    assert_eq!(contract.traffic, DxUpdateTraffic::Green);
}

#[test]
fn dx_new_absolute_path_uses_dx_safe_folder_name() {
    let dir = tempdir().expect("tempdir");
    let project = dir.path().join("absolute-starter");
    let cli = Cli::with_cwd(PathBuf::from("."));

    cli.cmd_new(project.to_str().expect("utf-8 temp path"))
        .expect("dx new absolute path");

    let config = DxConfig::load_project(&project).expect("parse generated dx config");
    assert_eq!(config.project.name, "absolute-starter");
    assert_eq!(config.tooling.biome.version, "2.4.15");
    assert_eq!(config.tooling.dx_style.mode, "generated-css");
    assert_eq!(config.tooling.forge_ui.style, "new-york");
    assert!(!project.join("dx.config.toml").exists());
    assert!(!project.join("node_modules").exists());
}

#[test]
fn www_routes_report_merges_project_app_router_files() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        r#"
project.name="routes-app"
dev.port=3939
"#,
    )
    .expect("write dx");
    for path in [
        "app/page.tsx",
        "app/blog/[slug]/page.tsx",
        "app/(dashboard)/settings/page.tsx",
        "app/api/health/route.ts",
    ] {
        let path = dir.path().join(path);
        fs::create_dir_all(path.parent().expect("parent")).expect("create parent");
        fs::write(path, "export default function Page() { return null; }\n")
            .expect("write route file");
    }

    let mut report = build_www_routes_report("test");
    attach_local_www_routes(dir.path(), &mut report).expect("merge local app routes");
    let routes = report["routes"].as_array().expect("routes");
    let route_paths: BTreeSet<_> = routes
        .iter()
        .filter_map(|route| route.get("route").and_then(serde_json::Value::as_str))
        .collect();

    assert!(route_paths.contains("/"));
    assert!(route_paths.contains("/blog/:slug"));
    assert!(route_paths.contains("/settings"));
    assert!(route_paths.contains("/api/health"));
    assert_eq!(report["local_project"]["app_dir_present"], true);
    let root = routes
        .iter()
        .find(|route| route.get("route").and_then(serde_json::Value::as_str) == Some("/"))
        .expect("root route");
    assert_eq!(
        root.pointer("/preview/url")
            .and_then(serde_json::Value::as_str),
        Some("http://localhost:3939/")
    );
    assert_eq!(
        root.get("source_files")
            .and_then(serde_json::Value::as_array)
            .and_then(|files| files.first())
            .and_then(serde_json::Value::as_str),
        Some("app/page.tsx")
    );
}

#[test]
fn dx_serializer_command_generates_machine_cache_for_dx_file() {
    let dir = tempdir().expect("tempdir");
    fs::write(
        dir.path().join("dx"),
        r#"
project.name="serializer-app"
project.version="0.1.0"
dev.port=3030
"#,
    )
    .expect("write dx");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_serializer(&["dx".to_string()])
        .expect("serialize dx file");

    assert!(!dir.path().join(".dx/serializer/dx.llm").exists());
    assert!(dir.path().join(".dx/serializer/dx.machine").exists());
}

#[test]
fn public_forge_add_selector_expands_shadcn_surfaces_to_package_ids() {
    let request = parse_public_forge_add_request("shadcn/ui#button,card,input", None)
        .expect("parse surface selector");

    assert_eq!(
        request.package_ids,
        vec!["shadcn/ui/button", "shadcn/ui/card", "shadcn/ui/input"]
    );
    assert_eq!(request.selected_exports, vec!["button", "card", "input"]);
    assert!(request.surface_packages);
}

#[test]
fn public_forge_add_only_expands_ui_surface_aliases() {
    let request = parse_public_forge_add_request("ui", Some("button,input")).expect("parse --only");

    assert_eq!(request.package_ids, vec!["ui/button", "ui/input"]);
    assert_eq!(request.selected_exports, vec!["button", "input"]);
    assert!(request.surface_packages);
}

#[test]
fn public_forge_add_keeps_generic_root_dx_export_selection() {
    let request = parse_public_forge_add_request("auth/better-auth#client", None)
        .expect("generic package selectors are delegated to root dx export maps");

    assert_eq!(request.package_ids, vec!["auth/better-auth"]);
    assert_eq!(request.selected_exports, vec!["client"]);
    assert!(!request.surface_packages);
}

#[test]
fn forge_registry_publish_write_requires_yes() {
    let cli = Cli::new();
    let error = cli
        .cmd_forge_registry_publish(&[
            "--remote".to_string(),
            "r2".to_string(),
            "--package".to_string(),
            "shadcn/ui/button".to_string(),
            "--write".to_string(),
        ])
        .expect_err("write without --yes must be blocked");

    assert!(error.to_string().contains("--yes"));
}

#[test]
fn dx_new_creates_next_familiar_template_surface_without_node_modules() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("next-familiar-app").expect("dx new");

    let project = dir.path().join("next-familiar-app");
    for path in [
        "app/loading.tsx",
        "app/error.tsx",
        "app/not-found.tsx",
        "app/page.tsx",
        "app/api/health/route.ts",
        "components/template-app/template-console.tsx",
        "components/template-app/template-shell.tsx",
        "components/template-app/template-route-contract.ts",
        "components/template-app/auth-session-status.tsx",
        "components/template-app/ai-chat-status.tsx",
        "components/template-app/instantdb-status.tsx",
        "components/template-app/icon-status.tsx",
        "components/template-app/next-intl-status.tsx",
        "components/template-app/query-cache-status.tsx",
        "components/template-app/query-dashboard-read-model.ts",
        "components/template-app/dx-check-style-evidence-read-model.ts",
        "components/template-app/template-shell-evidence-loader.ts",
        "components/template-app/template-shell-style-evidence-drift.ts",
        "components/template-app/preview-style-evidence-read-model.ts",
        "components/template-app/preview-style-package-panel-read-model.ts",
        "components/template-app/forge-package-status.ts",
        "components/template-app/forge-package-status-read-model.ts",
        "components/template-app/forge-safety-archive-contract.ts",
        "components/template-app/forge-safety-archive-runbook.ts",
        "components/template-app/forge-remote-head-health-contract.ts",
        "components/template-app/forge-remote-head-health-panel.tsx",
        "components/template-app/wasm-interop-status.tsx",
        "components/template-app/zod-validation-status.tsx",
        "components/template-app/package-catalog.ts",
        "components/template-app/react-markdown-preview.tsx",
        "components/template-app/state-zustand-counter.tsx",
        "components/template-app/state-zustand-dashboard.tsx",
        ".dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json",
        "components/template-app/trpc-launch-contract.ts",
        "components/template-app/trpc-launch-health.tsx",
        "server/templateCatalog.ts",
        "server/loaders.ts",
        "public/favicon.svg",
        "public/d-logo.svg",
        "public/og-image.svg",
        "public/robots.txt",
        "README.md",
        ".gitignore",
        ".dx/forge/template-manifest.json",
        ".dx/forge/source-manifest.json",
        ".dx/forge/docs/dx-www-starter-ui.md",
        ".dx/forge/docs/dx-www-template-shell--variant-next-familiar.md",
        NEXT_FAMILIAR_LAUNCH_ZED_TEMPLATE_HANDOFF_FILE,
        ".dx/serializer/dx.machine",
    ] {
        assert!(project.join(path).exists(), "{path}");
    }
    for path in [
        "package.json",
        "package-lock.json",
        "pnpm-lock.yaml",
        "yarn.lock",
        "bun.lock",
        "bun.lockb",
        "next.config.js",
        "next.config.mjs",
        "next.config.ts",
        "source.config.ts",
        "next-env.d.ts",
        "biome.json",
        "tailwind.config.ts",
        "postcss.config.mjs",
        "components.json",
        "tsconfig.json",
        "dx-env.d.ts",
    ] {
        assert!(!project.join(path).exists(), "{path}");
    }
    assert!(!project.join("source.config.ts").exists());

    let dx_config = DxConfig::load_project(&project).expect("parse generated dx");
    assert!(dx_config.framework.www.app_router);
    assert_eq!(dx_config.framework.www.config_owner_file, "dx");
    assert!(dx_config.framework.www.config_files.is_empty());
    assert_eq!(dx_config.framework.www.route_root, "/");
    assert!(!dx_config.framework.www.turbopack_runtime);
    assert!(dx_config.framework.fumadocs.enabled);
    assert_eq!(dx_config.framework.fumadocs.docs_route, "/docs");
    assert_eq!(
        dx_config.framework.fumadocs.readiness_route,
        "/docs/readiness"
    );
    assert_eq!(
        dx_config.framework.fumadocs.openapi_allowed_origins_env,
        "DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS"
    );
    assert!(
        dx_config
            .framework
            .fumadocs
            .generated_routes
            .iter()
            .any(|route| route == "/docs/readiness")
    );

    let manifest = read_json_value(project.join(".dx/forge/template-manifest.json"));
    assert_eq!(manifest["template"], "next-familiar");
    assert_eq!(manifest["node_modules_required"], false);
    assert_eq!(manifest["entrypoint"], "app/page.tsx");
    assert_eq!(manifest["compatibility_pages_fallback"], false);
    assert_eq!(manifest["app_router_first"], true);
    assert_eq!(manifest["tooling_config_source"], "dx");
    assert!(
        manifest["tooling"]
            .as_array()
            .expect("tooling")
            .iter()
            .any(|tool| tool == "biome")
    );
    assert!(
        manifest["local_source"]
            .as_array()
            .expect("local source")
            .iter()
            .any(|path| path == "components/ui/button.tsx")
    );
    assert!(
        manifest["forge_artifacts"]
            .as_array()
            .expect("forge artifacts")
            .iter()
            .any(|path| path == ".dx/forge/source-manifest.json")
    );
    assert!(
        manifest["app_router_files"]
            .as_array()
            .expect("app router files")
            .iter()
            .any(|path| path == "app/not-found.tsx")
    );
    assert!(
        manifest["app_router_files"]
            .as_array()
            .expect("app router files")
            .iter()
            .any(|path| path == "app/page.tsx")
    );
    assert!(
        manifest["local_source"]
            .as_array()
            .expect("local source")
            .iter()
            .any(|path| path == "components/template-app/template-shell.tsx")
    );
    assert_eq!(manifest["www_template_entrypoint"]["route"], "/");
    assert_eq!(
        manifest["www_template_entrypoint"]["materialized_file"],
        "app/page.tsx"
    );
    assert_eq!(
        manifest["www_template_entrypoint"]["contract_materialized_file"],
        "components/template-app/template-route-contract.ts"
    );
    assert!(
        manifest["www_template_entrypoint"]["component_materialized_files"]
            .as_array()
            .expect("component materialized files")
            .iter()
            .any(|path| path == "components/template-app/next-intl-status.tsx")
    );
    assert!(
        manifest["www_template_entrypoint"]["component_materialized_files"]
            .as_array()
            .expect("component materialized files")
            .iter()
            .any(|path| path == "components/template-app/state-zustand-dashboard.tsx")
    );
    assert_eq!(
        manifest["www_template_entrypoint"]["template_readiness_receipt"]["package"],
        "dx-www/template-shell"
    );
    assert_eq!(
        manifest["www_template_entrypoint"]["template_readiness_receipt"]["status"],
        "source-materialized-runtime-pending"
    );
    assert_eq!(
        manifest["zed_template_handoff"]["schema"],
        "dx.zed.template_handoff"
    );
    assert_eq!(
        manifest["zed_template_handoff"]["file"],
        NEXT_FAMILIAR_LAUNCH_ZED_TEMPLATE_HANDOFF_FILE
    );
    assert_eq!(
        manifest["zed_template_handoff"]["architecture_contract"]["schema"],
        "dx.www.default_template.architecture_contract"
    );
    assert_eq!(
        manifest["zed_template_handoff"]["readiness_receipt"],
        NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE
    );
    assert!(
        manifest["zed_template_handoff"]["open_files"]
            .as_array()
            .expect("handoff open files")
            .iter()
            .any(|file| file["path"] == "app/page.tsx")
    );
    let zed_handoff = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_ZED_TEMPLATE_HANDOFF_FILE));
    assert_eq!(zed_handoff["schema"], "dx.zed.template_handoff");
    assert_eq!(zed_handoff["route"], "/");
    assert_eq!(
        zed_handoff["route_aliases"]
            .as_array()
            .expect("handoff route aliases")
            .len(),
        0
    );
    assert_eq!(zed_handoff["entrypoint_file"], "app/page.tsx");
    assert_eq!(
        zed_handoff["source_entrypoint_file"],
        "examples/template/app/page.tsx"
    );
    assert!(zed_handoff["secondary_entrypoint_file"].is_null());
    assert!(zed_handoff["secondary_source_entrypoint_file"].is_null());
    assert_eq!(
        zed_handoff["architecture_contract"]["runtime_model"]["foundation"],
        "dx-www"
    );
    assert_eq!(
        zed_handoff["template_readiness_receipt"]["architecture_contract"]["schema"],
        "dx.www.default_template.architecture_contract"
    );
    assert_eq!(
        manifest["launch_readiness_bundle"]["schema"],
        "dx.launch.readiness_bundle"
    );
    assert_eq!(
        manifest["launch_readiness_bundle"]["readiness_receipts"]["template_readiness"],
        NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE
    );
    assert!(
        manifest["launch_readiness_bundle"]["source_guards"]
            .as_array()
            .expect("bundle source guards")
            .iter()
            .any(|guard| guard["kind"] == "source_level_package_guard")
    );
    assert!(!project.join("pages").exists());
    assert!(!project.join("pages/index.html").exists());
    assert!(!project.join(".dx/serializer/dx.llm").exists());
    assert!(!project.join("node_modules").exists());

    let project_cli = Cli::with_cwd(project.clone());
    project_cli.cmd_build().expect("dx build");

    let build_manifest = read_json_value(project.join(".dx/www/output/manifest.json"));
    assert_eq!(build_manifest["app_routes_compiled"], 7);
    assert_eq!(build_manifest["tsx_app_router_entrypoint"], true);
    assert_eq!(
        build_manifest["compatibility_pages_fallback_compiled"],
        false
    );
    assert!(project.join(".dx/www/output/app/index.html").is_file());
    assert!(!project.join(".dx/www/output/pages/index.dxob").exists());
}

#[test]
fn dx_new_materializes_every_launch_manifest_source_file() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("launch-materialized-app").expect("dx new");

    let project = dir.path().join("launch-materialized-app");
    let manifest = read_json_value(project.join(".dx/forge/template-manifest.json"));
    let mut manifest_files = Vec::new();
    for key in ["app_router_files", "local_source"] {
        manifest_files.extend(
            manifest[key]
                .as_array()
                .unwrap_or_else(|| panic!("{key} should be an array"))
                .iter()
                .filter_map(|value| value.as_str())
                .map(str::to_string),
        );
    }
    manifest_files.extend(
        manifest["generated_files"]
            .as_array()
            .expect("generated files")
            .iter()
            .filter_map(|entry| entry["materialized_file"].as_str())
            .map(str::to_string),
    );
    manifest_files.sort();
    manifest_files.dedup();

    for relative_path in manifest_files {
        assert!(
            project.join(&relative_path).is_file(),
            "template manifest references missing generated file {relative_path}"
        );
    }

    let package_catalog = fs::read_to_string(project.join("components/template-app/package-catalog.ts"))
        .expect("package catalog");
    for package_id in FORGE_WWW_TEMPLATE_PACKAGE_IDS {
        assert!(
            package_catalog.contains(&format!("packageId: \"{package_id}\"")),
            "launch package catalog should include {package_id}"
        );
    }

    assert_eq!(
        manifest["www_template_entrypoint"]["template_readiness_receipt"]["package"],
        NEXT_FAMILIAR_LAUNCH_RECEIPT_PACKAGE_ID
    );
    assert_eq!(
        manifest["www_template_entrypoint"]["template_readiness_receipt"]["variant"],
        NEXT_FAMILIAR_LAUNCH_RECEIPT_VARIANT
    );
    for component in [
        "components/template-app/auth-session-status.tsx",
        "components/template-app/ai-chat-status.tsx",
        "components/template-app/instantdb-status.tsx",
        "components/template-app/wasm-interop-status.tsx",
        "components/template-app/zod-validation-status.tsx",
    ] {
        assert!(
            manifest["www_template_entrypoint"]["component_materialized_files"]
                .as_array()
                .expect("component materialized files")
                .iter()
                .any(|path| path == component),
            "launch component manifest should include {component}"
        );
    }
    assert_eq!(
        manifest["www_template_entrypoint"]["template_readiness_receipt"]["receipt_glob"],
        NEXT_FAMILIAR_LAUNCH_RECEIPT_GLOB
    );
    assert_eq!(
        manifest["www_template_entrypoint"]["template_readiness_receipt"]["file"],
        NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE
    );
    assert!(
        manifest["forge_artifacts"]
            .as_array()
            .expect("forge artifacts")
            .iter()
            .any(|path| path == NEXT_FAMILIAR_LAUNCH_RECEIPT_DOC)
    );
    assert!(
        manifest["forge_artifacts"]
            .as_array()
            .expect("forge artifacts")
            .iter()
            .any(|path| path == NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE)
    );
    assert!(
        manifest["forge_artifacts"]
            .as_array()
            .expect("forge artifacts")
            .iter()
            .any(|path| path == NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE)
    );

    let source_manifest = read_json_value(project.join(".dx/forge/source-manifest.json"));
    let launch_package = source_manifest["packages"]
        .as_array()
        .expect("source manifest packages")
        .iter()
        .find(|package| {
            package["package_id"] == NEXT_FAMILIAR_LAUNCH_RECEIPT_PACKAGE_ID
                && package["variant"] == NEXT_FAMILIAR_LAUNCH_RECEIPT_VARIANT
        })
        .expect("launch template source package");
    assert!(
        launch_package["files"]
            .as_array()
            .expect("launch package files")
            .iter()
            .any(|file| file["path"] == "app/page.tsx")
    );
    assert!(project.join(NEXT_FAMILIAR_LAUNCH_RECEIPT_DOC).is_file());
    let launch_receipt = fs::read_dir(project.join(".dx/forge/receipts"))
        .expect("receipt dir")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .find(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    name.ends_with("dx-www-template-shell--variant-next-familiar.json")
                })
        })
        .expect("launch template receipt");
    let receipt = read_json_value(launch_receipt);
    assert_eq!(
        receipt["package"]["package_id"],
        NEXT_FAMILIAR_LAUNCH_RECEIPT_PACKAGE_ID
    );
    assert_eq!(
        receipt["package"]["variant"],
        NEXT_FAMILIAR_LAUNCH_RECEIPT_VARIANT
    );
    let readiness_receipt =
        read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE));
    assert_eq!(readiness_receipt["schema"], "dx.www.template_readiness");
    assert_eq!(readiness_receipt["route"], "/");
    assert_eq!(
        readiness_receipt["summary"]["required_package_count"].as_u64(),
        Some(FORGE_WWW_TEMPLATE_PACKAGE_IDS.len() as u64)
    );
    assert_eq!(
        readiness_receipt["summary"]["materialized_file_count"].as_u64(),
        readiness_receipt["materialized_files"]
            .as_array()
            .map(|files| files.len() as u64)
    );
    assert_eq!(
        readiness_receipt["summary"]["checks_pending"].as_u64(),
        Some(1)
    );
    assert_eq!(
        readiness_receipt["runtime_verification_requires_explicit_permission"].as_bool(),
        Some(true)
    );
    assert_eq!(
        readiness_receipt["app_router_entrypoint"]["materialized_file"],
        "app/page.tsx"
    );
}

#[test]
fn dx_new_writes_launch_companion_documentation_receipts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("companion-docs-app").expect("dx new");

    let project = dir.path().join("companion-docs-app");
    let receipts_path = ".dx/forge/template-readiness/launch-companion-doc-receipts.json";
    let receipts = read_json_value(project.join(receipts_path));

    assert_eq!(receipts["schema"], "dx.launch.companion_doc_receipts");
    assert_eq!(receipts["template_id"], "next-familiar-www-template");
    assert_eq!(receipts["route"], "/");
    assert_eq!(receipts["no_execution"], true);

    let companions = receipts["companions"].as_array().expect("companion docs");
    assert_eq!(companions.len(), 11);
    for package_id in [
        "auth/better-auth",
        "ai/vercel-ai",
        "db/drizzle-sqlite",
        "supabase/client",
        "payments/stripe-js",
        "content/fumadocs-next",
        "validation/zod",
        "instantdb/react",
        "i18n/next-intl",
        "tanstack/query",
        "wasm/bindgen",
    ] {
        assert!(
            companions
                .iter()
                .any(|companion| companion["package_id"] == package_id
                    && companion["source_file"]
                        .as_str()
                        .is_some_and(|path| path.starts_with("examples/template/"))
                    && companion["materialized_file"]
                        .as_str()
                        .is_some_and(|path| project.join(path).is_file())
                    && companion["open_files"]
                        .as_array()
                        .expect("open files")
                        .iter()
                        .any(|file| file["kind"] == "materialized-proof")),
            "missing companion documentation receipt for {package_id}"
        );
    }

    assert!(companions.iter().any(|companion| {
        companion["package_id"] == "validation/zod"
            && companion["docs_file"] == ".dx/forge/docs/validation-zod.md"
            && companion["proof_export"] == "LaunchZodValidationStatus"
            && companion["public_api"]
                .as_array()
                .expect("public api")
                .iter()
                .any(|api| api == "validateDxInput")
    }));

    let manifest = read_json_value(project.join(".dx/forge/template-manifest.json"));
    assert_eq!(
        manifest["launch_companion_doc_receipts"]["file"],
        receipts_path
    );
    assert!(
        manifest["forge_artifacts"]
            .as_array()
            .expect("forge artifacts")
            .iter()
            .any(|artifact| artifact == receipts_path)
    );

    let readiness_receipt =
        read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE));
    assert_eq!(
        readiness_receipt["companion_documentation_receipts"]["file"],
        receipts_path
    );

    let readiness_bundle =
        read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE));
    assert_eq!(
        readiness_bundle["companion_documentation_receipts"]["companion_count"],
        11
    );
}

#[test]
fn forge_template_readiness_reports_generated_launch_receipt_without_runtime() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("readiness-app").expect("dx new");

    let project = dir.path().join("readiness-app");
    let output_path = dir.path().join("template-readiness.json");
    cli.cmd_forge(&[
        "template-readiness".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("template readiness report");

    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.www.template_readiness_verification");
    assert_eq!(report["passed"], true);
    assert_eq!(report["score"], 100);
    assert_eq!(
        report["receipt_path"],
        NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("checks")
            .iter()
            .any(|check| check["name"] == "readiness-receipt-schema" && check["passed"] == true)
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("checks")
            .iter()
            .any(|check| check["name"] == "materialized-files-present" && check["passed"] == true)
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("checks")
            .iter()
            .any(|check| check["name"] == "runtime-verification-gated"
                && check["passed"] == true
                && check["requires_explicit_permission"] == true)
    );
    assert_eq!(
        report["runtime_verification_requires_explicit_permission"].as_bool(),
        Some(true)
    );
    assert!(
        report["next_commands"]
            .as_array()
            .expect("next commands")
            .iter()
            .any(|command| command == "dx check . --project-contract")
    );
}

#[test]
fn source_readiness_reports_stay_green_without_runtime_approval_artifacts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("readiness-no-approval-app").expect("dx new");

    let project = dir.path().join("readiness-no-approval-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE,
        NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE,
        NEXT_FAMILIAR_LAUNCH_VERIFICATION_LANE_FILE,
    ] {
        fs::remove_file(project.join(path))
            .unwrap_or_else(|error| panic!("remove optional runtime artifact {path}: {error}"));
    }

    let readiness_output = dir.path().join("template-readiness-no-approval.json");
    cli.cmd_forge(&[
        "template-readiness".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        readiness_output.to_string_lossy().into_owned(),
    ])
    .expect("template readiness without approval artifacts");

    let readiness = read_json_value(readiness_output);
    assert_eq!(readiness["passed"], true);
    assert_eq!(readiness["score"], 100);
    assert!(
        readiness["findings"]
            .as_array()
            .expect("findings")
            .is_empty()
    );

    let bundle_output = dir.path().join("readiness-bundle-no-approval.json");
    cli.cmd_forge(&[
        "launch-readiness-bundle".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        bundle_output.to_string_lossy().into_owned(),
    ])
    .expect("readiness bundle without approval artifacts");

    let bundle = read_json_value(bundle_output);
    assert_eq!(bundle["passed"], true);
    assert_eq!(bundle["score"], 100);
    assert_eq!(bundle["template_readiness"]["passed"], true);
    assert_eq!(bundle["runtime_gate"]["requires_explicit_permission"], true);
}

#[test]
fn forge_launch_readiness_bundle_aggregates_generated_project_evidence_without_runtime() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("bundle-app").expect("dx new");

    let project = dir.path().join("bundle-app");
    let output_path = dir.path().join("launch-readiness-bundle.json");
    cli.cmd_forge(&[
        "launch-readiness-bundle".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("launch readiness bundle");

    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.forge.launch_readiness_bundle");
    assert_eq!(report["passed"], true);
    assert_eq!(report["score"], 100);
    assert_eq!(report["no_execution"], true);
    assert_eq!(report["template_readiness"]["passed"], true);
    assert_eq!(
        report["template_readiness"]["receipt_path"],
        NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE
    );
    assert_eq!(
        report["package_receipts"]["www_template_receipt_present"],
        true
    );
    assert_eq!(
        report["companion_documentation_receipts"]["schema"],
        "dx.launch.companion_doc_receipts"
    );
    assert_eq!(
        report["companion_documentation_receipts"]["companion_count"],
        11
    );
    assert_eq!(
        report["companion_documentation_receipts"]["materialized_proofs_present"],
        report["companion_documentation_receipts"]["materialized_proofs_total"]
    );
    assert_eq!(
        report["runtime_checklist"]["schema"],
        "dx.launch.runtime_checklist"
    );
    assert_eq!(
        report["runtime_checklist"]["approval_status"],
        "requires-explicit-permission"
    );
    assert_eq!(report["runtime_checklist"]["blocked_by_default"], true);
    assert_eq!(
        report["runtime_checklist"]["commands_requiring_approval"],
        report["runtime_checklist"]["commands_total"]
    );
    assert_eq!(
        report["runtime_evidence_review"]["runtime_evidence_path"],
        NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE
    );
    assert_eq!(
        report["runtime_evidence_review"]["final_receipt_path"],
        NEXT_FAMILIAR_LAUNCH_RUNTIME_FINALIZATION_RECEIPT_FILE
    );
    assert_eq!(
        report["runtime_evidence_review"]["review_report_path"],
        NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE
    );
    assert_eq!(
        report["runtime_evidence_review"]["review_command"],
        "dx forge launch-runtime-evidence-review --project . --json"
    );
    assert_eq!(report["zed_handoff"]["present"], true);
    assert_eq!(
        report["zed_handoff"]["readiness_receipt"],
        NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE
    );
    assert_eq!(report["source_guards"]["package_guard_declared"], true);
    assert_eq!(report["runtime_gate"]["requires_explicit_permission"], true);
    assert!(
        report["next_commands"]
            .as_array()
            .expect("next commands")
            .iter()
            .any(|command| command == "dx forge launch-runtime-checklist --project . --json")
    );
}

#[test]
fn forge_launch_adoption_report_summarizes_no_build_template_adoption() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("adoption-report-app").expect("dx new");

    let project = dir.path().join("adoption-report-app");
    let output_path = dir.path().join("launch-adoption-report.json");
    cli.cmd_forge(&[
        "launch-adoption-report".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("launch adoption report");

    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.forge.launch_adoption_report");
    assert_eq!(report["passed"], true);
    assert_eq!(report["score"], 100);
    assert_eq!(report["no_execution"], true);
    assert_eq!(report["readiness_receipt"]["present"], true);
    assert_eq!(
        report["readiness_receipt"]["schema"],
        "dx.www.template_readiness"
    );
    assert_eq!(
        report["companion_files"]["present"],
        report["companion_files"]["total"]
    );
    assert!(
        report["companion_files"]["files"]
            .as_array()
            .expect("companion files")
            .iter()
            .any(|file| file["kind"] == "validation-status"
                && file["source_file"] == "examples/template/zod-validation-status.tsx"
                && file["materialized_file"] == "components/template-app/zod-validation-status.tsx"
                && file["present"] == true)
    );
    assert!(
        report["app_owned_dependencies"]["packages"]
            .as_array()
            .expect("app-owned packages")
            .iter()
            .any(|package| package["package_id"] == "instantdb/react"
                && package["required_env"]
                    .as_array()
                    .expect("required env")
                    .iter()
                    .any(|env| env == "NEXT_PUBLIC_INSTANT_APP_ID")
                && package["app_owned_boundaries"]
                    .as_array()
                    .expect("app owned boundaries")
                    .iter()
                    .any(|boundary| boundary
                        .as_str()
                        .is_some_and(|value| value.contains("production schema"))))
    );
    assert!(
        report["app_owned_dependencies"]["required_env"]
            .as_array()
            .expect("required env")
            .iter()
            .any(|env| env == "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY")
    );
    assert_eq!(
        report["runtime_proofs"]["requires_explicit_permission"],
        true
    );
    assert!(
        report["runtime_proofs"]["expected_evidence"]
            .as_array()
            .expect("runtime evidence")
            .iter()
            .any(|evidence| evidence == "final-launch-evidence-receipt")
    );
    assert!(
        report["next_commands"]
            .as_array()
            .expect("next commands")
            .iter()
            .any(|command| command == "dx forge launch-readiness-bundle --project . --json")
    );
}

#[test]
fn forge_launch_manifest_drift_compares_generated_template_artifacts_without_runtime() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("manifest-drift-app").expect("dx new");

    let project = dir.path().join("manifest-drift-app");
    let output_path = dir.path().join("launch-manifest-drift.json");
    cli.cmd_forge(&[
        "launch-manifest-drift".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("launch manifest drift report");

    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.forge.launch_manifest_drift");
    assert_eq!(report["passed"], true);
    assert_eq!(report["score"], 100);
    assert_eq!(report["drift_count"], 0);
    assert_eq!(report["no_execution"], true);
    assert_eq!(
        report["package_catalog"]["manifest_package_count"],
        report["package_catalog"]["readiness_required_package_count"]
    );
    assert_eq!(
        report["generated_manifest"]["manifest_file_count"],
        report["generated_manifest"]["readiness_materialized_file_count"]
    );
    assert_eq!(
        report["generated_manifest"]["files_match_readiness_receipt"],
        true
    );
    assert_eq!(
        report["companion_coverage"]["present"],
        report["companion_coverage"]["total"]
    );
    assert!(
        report["companion_coverage"]["files"]
            .as_array()
            .expect("companion files")
            .iter()
            .any(
                |file| file["materialized_file"] == "components/template-app/zod-validation-status.tsx"
                    && file["covered_by_generated_files"] == true
                    && file["covered_by_readiness_receipt"] == true
                    && file["present"] == true
            )
    );
    assert_eq!(report["runtime_gate"]["requires_explicit_permission"], true);
    assert!(
        report["next_commands"]
            .as_array()
            .expect("next commands")
            .iter()
            .any(|command| command == "dx forge launch-adoption-report --project . --json")
    );
}

#[test]
fn forge_launch_companion_receipts_expose_source_proof_docs_without_runtime() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("companion-receipts-app").expect("dx new");

    let project = dir.path().join("companion-receipts-app");
    let output_path = dir.path().join("launch-companion-receipts.json");
    cli.cmd_forge(&[
        "launch-companion-receipts".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("launch companion receipts report");

    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.forge.launch_companion_receipts");
    assert_eq!(report["passed"], true);
    assert_eq!(report["score"], 100);
    assert_eq!(report["no_execution"], true);
    assert_eq!(
        report["companion_receipts"]["present"],
        report["companion_receipts"]["total"]
    );
    assert!(
        report["companion_receipts"]["total"]
            .as_u64()
            .is_some_and(|total| total >= 8)
    );
    let auth_receipt = report["companion_receipts"]["receipts"]
        .as_array()
        .expect("companion receipts")
        .iter()
        .find(|receipt| receipt["package_id"] == "auth/better-auth")
        .expect("auth companion receipt");
    assert_eq!(auth_receipt["kind"], "auth-session-status");
    assert_eq!(
        auth_receipt["source_file"],
        "examples/template/auth-session-status.tsx"
    );
    assert_eq!(
        auth_receipt["materialized_file"],
        "components/template-app/auth-session-status.tsx"
    );
    assert_eq!(
        auth_receipt["docs_file"],
        ".dx/forge/docs/launch-companions/auth-session-status.md"
    );
    assert_eq!(auth_receipt["docs_present"], true);

    let auth_doc = std::fs::read_to_string(
        project.join(".dx/forge/docs/launch-companions/auth-session-status.md"),
    )
    .expect("auth companion doc");
    assert!(auth_doc.contains("Source proof"));
    assert!(auth_doc.contains("examples/template/auth-session-status.tsx"));
    assert!(auth_doc.contains("components/template-app/auth-session-status.tsx"));

    let manifest = read_json_value(project.join(".dx/forge/template-manifest.json"));
    assert_eq!(
        manifest["launch_companion_receipts"]["schema"],
        "dx.launch.companion_receipts"
    );
    assert!(
        manifest["launch_companion_receipts"]["receipts"]
            .as_array()
            .expect("manifest companion receipts")
            .iter()
            .any(|receipt| receipt["package_id"] == "validation/zod"
                && receipt["docs_file"] == ".dx/forge/docs/launch-companions/validation-status.md")
    );
}

#[test]
fn forge_launch_runtime_checklist_requires_explicit_approval_without_execution() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("runtime-checklist-app").expect("dx new");

    let project = dir.path().join("runtime-checklist-app");
    let checklist_path = project.join(".dx/forge/template-readiness/launch-runtime-checklist.json");
    let checklist = read_json_value(checklist_path);
    assert_eq!(checklist["schema"], "dx.launch.runtime_checklist");
    assert_eq!(
        checklist["approval"]["status"],
        "requires-explicit-permission"
    );
    assert_eq!(
        checklist["approval"]["default_action"],
        "skip-runtime-build-preview"
    );
    assert_eq!(checklist["no_execution"], true);
    assert!(
        checklist["commands"]
            .as_array()
            .expect("runtime commands")
            .iter()
            .any(|command| command["command"] == "dx build"
                && command["requires_explicit_approval"] == true
                && command["default_action"] == "skip")
    );
    assert!(
        checklist["commands"]
            .as_array()
            .expect("runtime commands")
            .iter()
            .any(
                |command| command["command"] == "dx preview --production-contract"
                    && command["requires_explicit_approval"] == true
                    && command["default_action"] == "skip"
            )
    );

    let output_path = dir.path().join("launch-runtime-checklist-report.json");
    cli.cmd_forge(&[
        "launch-runtime-checklist".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("launch runtime checklist report");

    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.forge.launch_runtime_checklist");
    assert_eq!(report["passed"], true);
    assert_eq!(report["score"], 100);
    assert_eq!(report["no_execution"], true);
    assert_eq!(report["approval"]["requires_explicit_permission"], true);
    assert_eq!(report["approval"]["status"], "requires-explicit-permission");
    assert_eq!(report["runtime_commands"]["blocked_by_default"], true);
    assert_eq!(
        report["runtime_commands"]["requires_explicit_approval"],
        report["runtime_commands"]["total"]
    );
    assert!(
        report["expected_evidence"]["items"]
            .as_array()
            .expect("expected evidence")
            .iter()
            .any(|evidence| evidence == "final-launch-evidence-receipt")
    );

    let manifest = read_json_value(project.join(".dx/forge/template-manifest.json"));
    assert_eq!(
        manifest["launch_runtime_checklist"]["file"],
        ".dx/forge/template-readiness/launch-runtime-checklist.json"
    );
}

#[test]
fn forge_public_status_reports_root_dx_package_and_local_registry_export_readiness() {
    let dir = tempdir().expect("tempdir");
    let project = dir.path().join("forge-status-app");
    fs::create_dir_all(project.join("src/auth")).expect("source dir");
    fs::write(
        project.join("src/auth/client.ts"),
        "export const betterAuthClient = true;\n",
    )
    .expect("client source");
    fs::write(
        project.join("dx"),
        r#"
[package]
name = "auth/better-auth"
version = "0.1.0"
description = "Authentication front-facing package based on upstream better-auth"
license = "MIT"
source = "."

[forge]
package = true
visibility = "public"
registry = "local"

[[forge.files]]
from = "src/auth/client.ts"
to = "lib/auth/better-auth/client.ts"
surface = "client"

[[forge.exports]]
name = "client"
files = ["lib/auth/better-auth/client.ts"]

[forge.install]
default_exports = ["client"]
allow_selective_imports = true
"#,
    )
    .expect("dx manifest");

    publish_root_dx_package_to_local_registry(
        &project,
        project.join(".dx/forge/registry/local"),
        false,
    )
    .expect("local publish");

    let report = forge_public_status_report(&project, "dx forge status");

    assert_eq!(report.status, "ready");
    let root_package = report.root_package.as_ref().expect("root package status");
    assert_eq!(root_package.status, "declared");
    assert_eq!(root_package.package_id.as_deref(), Some("auth/better-auth"));
    assert_eq!(root_package.version.as_deref(), Some("0.1.0"));
    assert_eq!(root_package.export_count, 1);
    assert_eq!(root_package.default_exports, vec!["client"]);
    assert_eq!(root_package.allow_selective_imports, Some(true));

    let local_package = report
        .local_registry_package
        .as_ref()
        .expect("local registry package status");
    assert_eq!(local_package.status, "published");
    assert_eq!(
        local_package.package_id.as_deref(),
        Some("auth/better-auth")
    );
    assert_eq!(local_package.version.as_deref(), Some("0.1.0"));
    assert_eq!(local_package.export_count, 1);
    assert!(local_package.verified);
    assert!(local_package.manifest_path.ends_with(Path::new(
        "packages/js/auth/better-auth/0.1.0/manifest.json"
    )));
    assert!(
        report
            .warnings
            .iter()
            .all(|warning| !warning.contains("Local Forge registry is missing"))
    );
}

#[test]
fn forge_public_status_writes_canonical_latest_receipt_for_cli_consumers() {
    let dir = tempdir().expect("tempdir");
    let project = dir.path().join("forge-status-receipt-app");
    fs::create_dir_all(project.join("src/auth")).expect("source dir");
    fs::write(
        project.join("src/auth/client.ts"),
        "export const betterAuthClient = true;\n",
    )
    .expect("client source");
    fs::write(
        project.join("dx"),
        r#"
[package]
name = "auth/better-auth"
version = "0.1.0"
description = "Authentication front-facing package based on upstream better-auth"
license = "MIT"
source = "."

[forge]
package = true
visibility = "public"
registry = "local"

[[forge.files]]
from = "src/auth/client.ts"
to = "lib/auth/better-auth/client.ts"
surface = "client"

[[forge.exports]]
name = "client"
files = ["lib/auth/better-auth/client.ts"]

[forge.install]
default_exports = ["client"]
allow_selective_imports = true
"#,
    )
    .expect("dx manifest");

    publish_root_dx_package_to_local_registry(
        &project,
        project.join(".dx/forge/registry/local"),
        false,
    )
    .expect("local publish");

    let cli = Cli::with_cwd(project.clone());
    cli.cmd_forge(&["status".to_string(), "--json".to_string()])
        .expect("forge status");

    let latest = project.join(".dx/receipts/forge/status-latest.json");
    assert!(latest.is_file(), "status latest receipt should exist");
    let receipt = read_json_value(latest);

    assert_eq!(receipt["schema_version"], "dx.forge.status");
    assert_eq!(receipt["status"], "ready");
    assert_eq!(
        receipt["root_package"]["package_id"].as_str(),
        Some("auth/better-auth")
    );
    assert_eq!(
        receipt["local_registry_package"]["status"].as_str(),
        Some("published")
    );
    assert!(
        receipt["receipt_path"]
            .as_str()
            .expect("receipt path")
            .ends_with(".dx/receipts/forge/status-latest.json")
    );
}

#[test]
fn dx_new_writes_runtime_approval_request_receipt() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("runtime-approval-app").expect("dx new");

    let project = dir.path().join("runtime-approval-app");
    let request_path = ".dx/forge/template-readiness/launch-runtime-approval-request.json";
    let request = read_json_value(project.join(request_path));

    assert_eq!(request["schema"], "dx.launch.runtime_approval_request");
    assert_eq!(request["template_id"], "next-familiar-www-template");
    assert_eq!(request["route"], "/");
    assert_eq!(request["no_execution"], true);
    assert_eq!(
        request["approval_record"]["status"],
        "pending-explicit-approval"
    );
    assert_eq!(
        request["approval_record"]["approved_by"],
        serde_json::Value::Null
    );
    assert!(
        request["requested_commands"]
            .as_array()
            .expect("requested commands")
            .iter()
            .any(|command| command["command"] == "dx build"
                && command["approved"] == false
                && command["requires_explicit_permission"] == true)
    );
    assert!(
        request["requested_evidence"]["items"]
            .as_array()
            .expect("requested evidence")
            .iter()
            .any(|evidence| evidence == "final-launch-evidence-receipt")
    );

    let manifest = read_json_value(project.join(".dx/forge/template-manifest.json"));
    assert_eq!(
        manifest["launch_runtime_approval_request"]["file"],
        request_path
    );
    assert_eq!(
        manifest["launch_runtime_checklist"]["approval_request"]["file"],
        request_path
    );
    assert!(
        manifest["forge_artifacts"]
            .as_array()
            .expect("forge artifacts")
            .iter()
            .any(|artifact| artifact == request_path)
    );
}

#[test]
fn forge_launch_runtime_approval_request_reports_pending_approval_without_execution() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("runtime-approval-report-app").expect("dx new");

    let project = dir.path().join("runtime-approval-report-app");
    let output_path = dir.path().join("launch-runtime-approval-request.json");
    cli.cmd_forge(&[
        "launch-runtime-approval-request".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("runtime approval request report");

    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.forge.launch_runtime_approval_request");
    assert_eq!(report["passed"], true);
    assert_eq!(report["score"], 100);
    assert_eq!(report["no_execution"], true);
    assert_eq!(report["approval"]["status"], "pending-explicit-approval");
    assert_eq!(report["approval"]["approved_by"], serde_json::Value::Null);
    assert_eq!(report["requested_commands"]["approved"], 0);
    assert_eq!(
        report["requested_commands"]["requires_explicit_permission"],
        report["requested_commands"]["total"]
    );
    assert_eq!(report["requested_evidence"]["total"], 3);
    assert!(
        report["checks"]
            .as_array()
            .expect("checks")
            .iter()
            .any(|check| check["name"] == "approval-pending" && check["passed"] == true)
    );
    assert!(
        report["next_commands"]
            .as_array()
            .expect("next commands")
            .iter()
            .any(|command| command == "dx forge launch-runtime-checklist --project . --json")
    );
}

#[test]
fn dx_new_writes_runtime_evidence_schema_stub_without_fake_proof() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("runtime-evidence-app").expect("dx new");

    let project = dir.path().join("runtime-evidence-app");
    let evidence_path = ".dx/forge/template-readiness/launch-runtime-evidence.json";
    let evidence = read_json_value(project.join(evidence_path));

    assert_eq!(evidence["schema"], "dx.launch.runtime_evidence");
    assert_eq!(evidence["status"], "awaiting-approved-runtime-run");
    assert_eq!(evidence["no_execution"], true);
    assert_eq!(evidence["fake_proof"], false);
    assert_eq!(
        evidence["approval_request"],
        ".dx/forge/template-readiness/launch-runtime-approval-request.json"
    );
    assert!(
        evidence["required_evidence"]
            .as_array()
            .expect("required evidence")
            .iter()
            .any(|item| item["id"] == "governed-runtime-route-response"
                && item["status"] == "not-collected"
                && item["required"] == true)
    );
    assert!(
        evidence["required_evidence"]
            .as_array()
            .expect("required evidence")
            .iter()
            .any(|item| item["id"] == "production-contract-route-proof"
                && item["status"] == "not-collected"
                && item["required"] == true)
    );
    assert!(
        evidence["required_evidence"]
            .as_array()
            .expect("required evidence")
            .iter()
            .any(|item| item["id"] == "final-launch-evidence-receipt"
                && item["status"] == "not-collected"
                && item["required"] == true)
    );

    let manifest = read_json_value(project.join(".dx/forge/template-manifest.json"));
    assert_eq!(manifest["launch_runtime_evidence"]["file"], evidence_path);
    assert_eq!(
        manifest["launch_runtime_approval_request"]["requested_evidence"]["receipt"],
        evidence_path
    );
    assert!(
        manifest["forge_artifacts"]
            .as_array()
            .expect("forge artifacts")
            .iter()
            .any(|artifact| artifact == evidence_path)
    );
}

#[test]
fn forge_launch_runtime_evidence_reports_required_runtime_artifacts_without_fake_proof() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("runtime-evidence-report-app").expect("dx new");

    let project = dir.path().join("runtime-evidence-report-app");
    let output_path = dir.path().join("launch-runtime-evidence.json");
    cli.cmd_forge(&[
        "launch-runtime-evidence".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("runtime evidence report");

    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.forge.launch_runtime_evidence");
    assert_eq!(report["passed"], true);
    assert_eq!(report["score"], 100);
    assert_eq!(report["no_execution"], true);
    assert_eq!(report["fake_proof"], false);
    assert_eq!(report["collected_evidence"]["present"], 0);
    assert_eq!(
        report["required_evidence"]["not_collected"],
        report["required_evidence"]["total"]
    );
    assert!(
        report["required_evidence"]["items"]
            .as_array()
            .expect("required evidence")
            .iter()
            .any(|item| item["id"] == "final-launch-evidence-receipt"
                && item["status"] == "not-collected")
    );
    assert!(
            report["next_commands"]
                .as_array()
                .expect("next commands")
                .iter()
                .any(|command| command
                    == "dx forge launch-runtime-approval-request --project . --json")
        );
}

#[test]
fn approved_runtime_evidence_report_requires_real_collected_artifacts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("approved-runtime-missing-evidence-app")
        .expect("dx new");

    let project = dir.path().join("approved-runtime-missing-evidence-app");
    let approval_path = project.join(NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE);
    let mut approval = read_json_value(approval_path.clone());
    approval["approval_record"]["status"] = serde_json::json!("approved");
    approval["approval_record"]["approved"] = serde_json::json!(true);
    approval["approval_record"]["approved_by"] = serde_json::json!("launch-operator");
    approval["approval_record"]["approved_at"] = serde_json::json!("2026-05-21T00:00:00Z");
    for command in approval["requested_commands"]
        .as_array_mut()
        .expect("requested commands")
    {
        command["approved"] = serde_json::json!(true);
        command["approval_status"] = serde_json::json!("approved");
    }
    fs::write(
        &approval_path,
        serde_json::to_string_pretty(&approval).expect("approval json"),
    )
    .expect("write approved request");

    let output_path = dir.path().join("approved-runtime-evidence.json");
    let result = cli.cmd_forge(&[
        "launch-runtime-evidence".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ]);

    assert!(
        result.is_err(),
        "approved runtime evidence without collected artifacts must fail"
    );
    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.forge.launch_runtime_evidence");
    assert_eq!(report["passed"], false);
    assert_eq!(report["runtime_approved"], true);
    assert_eq!(report["collected_evidence"]["present"], 0);
    assert!(
        report["checks"]
            .as_array()
            .expect("checks")
            .iter()
            .any(|check| check["name"] == "approved-runtime-evidence" && check["passed"] == false)
    );
}

#[test]
fn dx_new_writes_launch_verification_lane_metadata() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("launch-lane-app").expect("dx new");

    let project = dir.path().join("launch-lane-app");
    let lane_path = ".dx/forge/template-readiness/launch-verification-lane.json";
    let lane = read_json_value(project.join(lane_path));

    assert_eq!(lane["schema"], "dx.launch.verification_lane");
    assert_eq!(lane["lane_id"], "governed-runtime-verification");
    assert_eq!(lane["route"], "/");
    assert_eq!(lane["no_execution"], true);
    assert_eq!(lane["requires_explicit_permission"], true);
    assert_eq!(
        lane["runtime_artifacts"]["checklist"],
        ".dx/forge/template-readiness/launch-runtime-checklist.json"
    );
    assert_eq!(
        lane["runtime_artifacts"]["approval_request"],
        ".dx/forge/template-readiness/launch-runtime-approval-request.json"
    );
    assert_eq!(
        lane["runtime_artifacts"]["runtime_evidence"],
        ".dx/forge/template-readiness/launch-runtime-evidence.json"
    );
    assert!(
        lane["operator_steps"]
            .as_array()
            .expect("operator steps")
            .iter()
            .any(|step| step["id"] == "collect-runtime-evidence"
                && step["file"] == ".dx/forge/template-readiness/launch-runtime-evidence.json"
                && step["status"] == "awaiting-approved-runtime-run")
    );

    let manifest = read_json_value(project.join(".dx/forge/template-manifest.json"));
    assert_eq!(manifest["launch_verification_lane"]["file"], lane_path);
    assert_eq!(
        manifest["zed_template_handoff"]["launch_verification_lane"],
        lane_path
    );
    assert!(
        manifest["forge_artifacts"]
            .as_array()
            .expect("forge artifacts")
            .iter()
            .any(|artifact| artifact == lane_path)
    );
}

#[test]
fn forge_launch_verification_lane_reports_operator_sequence_without_runtime() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("launch-lane-report-app").expect("dx new");

    let project = dir.path().join("launch-lane-report-app");
    let output_path = dir.path().join("launch-verification-lane.json");
    cli.cmd_forge(&[
        "launch-verification-lane".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("launch verification lane report");

    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.forge.launch_verification_lane");
    assert_eq!(report["passed"], true);
    assert_eq!(report["score"], 100);
    assert_eq!(report["no_execution"], true);
    assert_eq!(report["lane"]["lane_id"], "governed-runtime-verification");
    assert_eq!(report["lane"]["step_count"], 13);
    assert_eq!(report["lane"]["present_files"], 3);
    assert_eq!(report["lane"]["requires_explicit_permission"], true);
    assert!(!report["lane"]["runtime_approved"].as_bool().unwrap_or(true));
    assert!(
        report["open_files"]
            .as_array()
            .expect("open files")
            .iter()
            .any(|file| file["kind"] == "runtime-evidence"
                && file["path"] == ".dx/forge/template-readiness/launch-runtime-evidence.json")
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("checks")
            .iter()
            .any(|check| check["name"] == "operator-sequence" && check["passed"] == true)
    );
}

#[test]
fn launch_readiness_reports_green_without_runtime_artifacts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("readiness-no-runtime-artifacts-app")
        .expect("dx new");

    let project = dir.path().join("readiness-no-runtime-artifacts-app");
    assert!(!project.join(".dx/forge/runtime").exists());

    let bundle_output = dir.path().join("launch-readiness-bundle.json");
    cli.cmd_forge(&[
        "launch-readiness-bundle".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        bundle_output.to_string_lossy().into_owned(),
    ])
    .expect("launch readiness bundle");
    let bundle = read_json_value(bundle_output);

    assert_eq!(bundle["passed"], true);
    assert_eq!(
        bundle["runtime_gate"]["status"],
        "pending-governed-runtime-pass"
    );

    let lane_output = dir.path().join("launch-verification-lane.json");
    cli.cmd_forge(&[
        "launch-verification-lane".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        lane_output.to_string_lossy().into_owned(),
    ])
    .expect("launch verification lane");
    let lane = read_json_value(lane_output);

    assert_eq!(lane["passed"], true);
    assert_eq!(lane["lane"]["runtime_approved"], false);
}

#[test]
fn forge_launch_runtime_evidence_requires_real_artifacts_after_approval() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("approved-runtime-evidence-app")
        .expect("dx new");

    let project = dir.path().join("approved-runtime-evidence-app");
    let evidence_path = project.join(".dx/forge/template-readiness/launch-runtime-evidence.json");
    let mut evidence = read_json_value(evidence_path.clone());
    evidence["approval_gate"]["status"] = serde_json::json!("approved");
    evidence["approval_gate"]["approved_by"] = serde_json::json!("essencefromexistence");
    for item in evidence["required_evidence"]
        .as_array_mut()
        .expect("required evidence")
    {
        item["status"] = serde_json::json!("collected");
    }
    evidence["collected_evidence"]["present"] = serde_json::json!(3);
    evidence["collected_evidence"]["artifacts"] = serde_json::json!([
        ".dx/forge/runtime/launch-route-response.json",
        ".dx/forge/runtime/production-contract-route-proof.json",
        ".dx/forge/runtime/final-launch-evidence-receipt.json"
    ]);
    fs::write(
        &evidence_path,
        serde_json::to_string_pretty(&evidence).expect("evidence json"),
    )
    .expect("write approved evidence stub");

    let output_path = dir.path().join("approved-runtime-evidence.json");
    let result = cli.cmd_forge(&[
        "launch-runtime-evidence".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ]);

    assert!(
        result.is_err(),
        "approved runtime evidence must fail until real runtime artifacts are collected"
    );
    let report = read_json_value(output_path);
    assert_eq!(report["runtime_approved"], true);
    assert_eq!(report["collected_evidence"]["present"], 3);
    assert_eq!(report["collected_evidence"]["existing_artifacts"], 0);
    assert!(
        report["checks"]
            .as_array()
            .expect("checks")
            .iter()
            .any(|check| check["name"] == "approved-runtime-evidence" && check["passed"] == false)
    );
}

#[test]
fn forge_launch_runtime_evidence_import_plan_requires_explicit_approval() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("unapproved-runtime-import-app")
        .expect("dx new");

    let external = dir.path().join("external-evidence");
    fs::create_dir_all(&external).expect("external evidence dir");
    let build_log = external.join("build.log");
    let route_response = external.join("route-response.json");
    let preview_proof = external.join("preview-proof.json");
    fs::write(&build_log, "build completed").expect("build log");
    fs::write(&route_response, "{\"status\":200}").expect("route response");
    fs::write(&preview_proof, "{\"preview\":\"ok\"}").expect("preview proof");

    let project = dir.path().join("unapproved-runtime-import-app");
    let output_path = dir.path().join("runtime-import-plan.json");
    let result = cli.cmd_forge(&[
        "launch-runtime-evidence-import-plan".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--build-log".to_string(),
        build_log.to_string_lossy().into_owned(),
        "--route-response".to_string(),
        route_response.to_string_lossy().into_owned(),
        "--preview-proof".to_string(),
        preview_proof.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ]);

    assert!(
        result.is_err(),
        "runtime evidence import plan must require approval"
    );
    let report = read_json_value(output_path);
    assert_eq!(
        report["schema"],
        "dx.forge.launch_runtime_evidence_import_plan"
    );
    assert_eq!(report["passed"], false);
    assert_eq!(report["no_execution"], true);
    assert_eq!(report["approval"]["approved"], false);
    assert!(
        report["checks"]
            .as_array()
            .expect("checks")
            .iter()
            .any(|check| check["name"] == "runtime-approval-recorded" && check["passed"] == false)
    );
}

#[test]
fn forge_launch_runtime_evidence_import_plan_maps_external_files_after_approval() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("approved-runtime-import-app").expect("dx new");

    let project = dir.path().join("approved-runtime-import-app");
    let approval_path = project.join(NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE);
    let mut approval = read_json_value(approval_path.clone());
    approval["approval_record"]["status"] = serde_json::json!("approved");
    approval["approval_record"]["approved"] = serde_json::json!(true);
    approval["approval_record"]["approved_by"] = serde_json::json!("launch-operator");
    approval["approval_record"]["approved_at"] = serde_json::json!("2026-05-21T00:00:00Z");
    for command in approval["requested_commands"]
        .as_array_mut()
        .expect("requested commands")
    {
        command["approved"] = serde_json::json!(true);
        command["approval_status"] = serde_json::json!("approved");
    }
    fs::write(
        &approval_path,
        serde_json::to_string_pretty(&approval).expect("approval json"),
    )
    .expect("write approved request");

    let external = dir.path().join("external-evidence");
    fs::create_dir_all(&external).expect("external evidence dir");
    let build_log = external.join("build.log");
    let route_response = external.join("route-response.json");
    let preview_proof = external.join("preview-proof.json");
    fs::write(&build_log, "build completed").expect("build log");
    fs::write(&route_response, "{\"status\":200}").expect("route response");
    fs::write(&preview_proof, "{\"preview\":\"ok\"}").expect("preview proof");

    let output_path = dir.path().join("runtime-import-plan.json");
    cli.cmd_forge(&[
        "launch-runtime-evidence-import-plan".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--build-log".to_string(),
        build_log.to_string_lossy().into_owned(),
        "--route-response".to_string(),
        route_response.to_string_lossy().into_owned(),
        "--preview-proof".to_string(),
        preview_proof.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("runtime import plan");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["no_execution"], true);
    assert_eq!(report["approval"]["approved"], true);
    assert_eq!(report["imports"]["total"], 3);
    assert_eq!(report["imports"]["source_present"], 3);
    assert!(
        report["imports"]["items"]
            .as_array()
            .expect("import items")
            .iter()
            .any(|item| item["id"] == "governed-runtime-route-response"
                && item["source_exists"] == true
                && item["target_path"] == ".dx/forge/runtime/launch-route-response.json")
    );
    assert!(
        report["next_commands"]
            .as_array()
            .expect("next commands")
            .iter()
            .any(|command| command == "dx forge launch-runtime-evidence --project . --json")
    );
}

#[test]
fn forge_launch_runtime_evidence_import_plan_reports_source_hash_metadata() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("hashed-runtime-import-app").expect("dx new");

    let project = dir.path().join("hashed-runtime-import-app");
    let approval_path = project.join(NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE);
    let mut approval = read_json_value(approval_path.clone());
    approval["approval_record"]["status"] = serde_json::json!("approved");
    approval["approval_record"]["approved"] = serde_json::json!(true);
    approval["approval_record"]["approved_by"] = serde_json::json!("launch-operator");
    approval["approval_record"]["approved_at"] = serde_json::json!("2026-05-21T00:00:00Z");
    for command in approval["requested_commands"]
        .as_array_mut()
        .expect("requested commands")
    {
        command["approved"] = serde_json::json!(true);
        command["approval_status"] = serde_json::json!("approved");
    }
    fs::write(
        &approval_path,
        serde_json::to_string_pretty(&approval).expect("approval json"),
    )
    .expect("write approved request");

    let external = dir.path().join("external-evidence");
    fs::create_dir_all(&external).expect("external evidence dir");
    let build_log = external.join("build.log");
    let route_response = external.join("route-response.json");
    let preview_proof = external.join("preview-proof.json");
    fs::write(&build_log, "build completed").expect("build log");
    fs::write(&route_response, "{\"status\":200}").expect("route response");
    fs::write(&preview_proof, "{\"preview\":\"ok\"}").expect("preview proof");

    let output_path = dir.path().join("runtime-import-plan.json");
    cli.cmd_forge(&[
        "launch-runtime-evidence-import-plan".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--build-log".to_string(),
        build_log.to_string_lossy().into_owned(),
        "--route-response".to_string(),
        route_response.to_string_lossy().into_owned(),
        "--preview-proof".to_string(),
        preview_proof.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("runtime import plan");

    let report = read_json_value(output_path);
    let build_item = report["imports"]["items"]
        .as_array()
        .expect("import items")
        .iter()
        .find(|item| item["id"] == "production-contract-build-log")
        .expect("build log item");
    assert_eq!(
        build_item["source_path"],
        build_log.to_string_lossy().as_ref()
    );
    assert_eq!(build_item["source_bytes"], "build completed".len() as u64);
    assert_eq!(build_item["hash_algorithm"], "blake3");
    assert_eq!(
        build_item["source_hash"],
        format!("blake3:{}", blake3::hash(b"build completed").to_hex())
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("checks")
            .iter()
            .any(|check| check["name"] == "source-hashes-present" && check["passed"] == true)
    );

    fs::write(&build_log, "build completed after replacement").expect("replace build log");
    let replaced_output_path = dir.path().join("runtime-import-plan-replaced.json");
    cli.cmd_forge(&[
        "launch-runtime-evidence-import-plan".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--build-log".to_string(),
        build_log.to_string_lossy().into_owned(),
        "--route-response".to_string(),
        route_response.to_string_lossy().into_owned(),
        "--preview-proof".to_string(),
        preview_proof.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        replaced_output_path.to_string_lossy().into_owned(),
    ])
    .expect("runtime import plan after replacement");
    let replaced_report = read_json_value(replaced_output_path);
    let replaced_build_item = replaced_report["imports"]["items"]
        .as_array()
        .expect("replaced import items")
        .iter()
        .find(|item| item["id"] == "production-contract-build-log")
        .expect("replaced build log item");
    assert_ne!(
        replaced_build_item["source_hash"],
        build_item["source_hash"]
    );
}

fn approve_runtime_request_receipt(project: &Path) {
    let approval_path = project.join(NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE);
    let mut approval = read_json_value(approval_path.clone());
    approval["approval_record"]["status"] = serde_json::json!("approved");
    approval["approval_record"]["approved"] = serde_json::json!(true);
    approval["approval_record"]["approved_by"] = serde_json::json!("launch-operator");
    approval["approval_record"]["approved_at"] = serde_json::json!("2026-05-21T00:00:00Z");
    for command in approval["requested_commands"]
        .as_array_mut()
        .expect("requested commands")
    {
        command["approved"] = serde_json::json!(true);
        command["approval_status"] = serde_json::json!("approved");
    }
    fs::write(
        &approval_path,
        serde_json::to_string_pretty(&approval).expect("approval json"),
    )
    .expect("write approved request");
}

#[test]
fn forge_launch_runtime_evidence_completeness_fails_incomplete_import_plan() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("incomplete-runtime-evidence-app")
        .expect("dx new");

    let project = dir.path().join("incomplete-runtime-evidence-app");
    approve_runtime_request_receipt(&project);

    let external = dir.path().join("external-evidence");
    fs::create_dir_all(&external).expect("external evidence dir");
    let build_log = external.join("build.log");
    let route_response = external.join("route-response.json");
    let preview_proof = external.join("preview-proof.json");
    fs::write(&build_log, "build completed").expect("build log");
    fs::write(&preview_proof, "{\"preview\":\"ok\"}").expect("preview proof");

    let import_plan_path = dir.path().join("runtime-import-plan-incomplete.json");
    let import_result = cli.cmd_forge(&[
        "launch-runtime-evidence-import-plan".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--build-log".to_string(),
        build_log.to_string_lossy().into_owned(),
        "--route-response".to_string(),
        route_response.to_string_lossy().into_owned(),
        "--preview-proof".to_string(),
        preview_proof.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        import_plan_path.to_string_lossy().into_owned(),
    ]);
    assert!(
        import_result.is_err(),
        "incomplete import plan should still write a reviewable report"
    );

    let output_path = dir.path().join("runtime-evidence-completeness.json");
    let result = cli.cmd_forge(&[
        "launch-runtime-evidence-completeness".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--import-plan".to_string(),
        import_plan_path.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ]);

    assert!(
        result.is_err(),
        "missing runtime artifacts must fail completeness"
    );
    let report = read_json_value(output_path);
    assert_eq!(
        report["schema"],
        "dx.forge.launch_runtime_evidence_completeness"
    );
    assert_eq!(report["passed"], false);
    assert_eq!(report["no_execution"], true);
    assert_eq!(report["import_plan"]["passed"], false);
    assert_eq!(report["artifacts"]["total"], 3);
    assert_eq!(report["artifacts"]["present"], 2);
    assert_eq!(report["artifacts"]["missing"], 1);
    assert_eq!(report["artifacts"]["hashes_present"], 2);
    assert_eq!(
        report["lane"]["path"],
        ".dx/forge/template-readiness/launch-verification-lane.json"
    );
    assert!(
        report["artifacts"]["items"]
            .as_array()
            .expect("artifact items")
            .iter()
            .any(|item| item["id"] == "governed-runtime-route-response"
                && item["source_exists"] == false
                && item["source_hash"] == serde_json::Value::Null
                && item["target_path"] == ".dx/forge/runtime/launch-route-response.json")
    );
    assert!(
            report["checks"]
                .as_array()
                .expect("checks")
                .iter()
                .any(|check| check["name"] == "required-artifacts-complete"
                    && check["passed"] == false)
        );
}

#[test]
fn forge_launch_runtime_evidence_completeness_passes_complete_import_plan() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("complete-runtime-evidence-app")
        .expect("dx new");

    let project = dir.path().join("complete-runtime-evidence-app");
    approve_runtime_request_receipt(&project);

    let external = dir.path().join("external-evidence");
    fs::create_dir_all(&external).expect("external evidence dir");
    let build_log = external.join("build.log");
    let route_response = external.join("route-response.json");
    let preview_proof = external.join("preview-proof.json");
    fs::write(&build_log, "build completed").expect("build log");
    fs::write(&route_response, "{\"status\":200}").expect("route response");
    fs::write(&preview_proof, "{\"preview\":\"ok\"}").expect("preview proof");

    let import_plan_path = dir.path().join("runtime-import-plan-complete.json");
    cli.cmd_forge(&[
        "launch-runtime-evidence-import-plan".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--build-log".to_string(),
        build_log.to_string_lossy().into_owned(),
        "--route-response".to_string(),
        route_response.to_string_lossy().into_owned(),
        "--preview-proof".to_string(),
        preview_proof.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        import_plan_path.to_string_lossy().into_owned(),
    ])
    .expect("runtime import plan");

    let output_path = dir.path().join("runtime-evidence-completeness.json");
    cli.cmd_forge(&[
        "launch-runtime-evidence-completeness".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--import-plan".to_string(),
        import_plan_path.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("runtime evidence completeness");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["no_execution"], true);
    assert_eq!(report["import_plan"]["passed"], true);
    assert_eq!(report["artifacts"]["present"], 3);
    assert_eq!(report["artifacts"]["missing"], 0);
    assert_eq!(report["artifacts"]["hashes_present"], 3);
    assert!(
        report["artifacts"]["items"]
            .as_array()
            .expect("artifact items")
            .iter()
            .any(|item| item["id"] == "governed-runtime-route-response"
                && item["source_path"] == route_response.to_string_lossy().as_ref()
                && item["source_exists"] == true
                && item["target_path"] == ".dx/forge/runtime/launch-route-response.json"
                && item["hash_algorithm"] == "blake3"
                && item["source_hash"]
                    .as_str()
                    .is_some_and(|hash| hash.starts_with("blake3:")))
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("checks")
            .iter()
            .any(|check| check["name"] == "required-artifacts-complete" && check["passed"] == true)
    );
    assert!(
        report["next_commands"]
            .as_array()
            .expect("next commands")
            .iter()
            .any(|command| command == "dx forge launch-runtime-evidence --project . --json")
    );
}

struct RuntimeImportPlanFixture {
    build_log: PathBuf,
    route_response: PathBuf,
    preview_proof: PathBuf,
    import_plan: PathBuf,
}

fn write_complete_runtime_import_plan(
    cli: &Cli,
    project: &Path,
    root: &Path,
    name: &str,
) -> RuntimeImportPlanFixture {
    let external = root.join(format!("{name}-external-evidence"));
    fs::create_dir_all(&external).expect("external evidence dir");
    let build_log = external.join("build.log");
    let route_response = external.join("route-response.json");
    let preview_proof = external.join("preview-proof.json");
    fs::write(&build_log, "build completed").expect("build log");
    fs::write(&route_response, "{\"status\":200}").expect("route response");
    fs::write(&preview_proof, "{\"preview\":\"ok\"}").expect("preview proof");

    let import_plan = root.join(format!("{name}-runtime-import-plan.json"));
    cli.cmd_forge(&[
        "launch-runtime-evidence-import-plan".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--build-log".to_string(),
        build_log.to_string_lossy().into_owned(),
        "--route-response".to_string(),
        route_response.to_string_lossy().into_owned(),
        "--preview-proof".to_string(),
        preview_proof.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        import_plan.to_string_lossy().into_owned(),
    ])
    .expect("runtime import plan");

    RuntimeImportPlanFixture {
        build_log,
        route_response,
        preview_proof,
        import_plan,
    }
}

#[test]
fn forge_launch_runtime_evidence_completeness_rejects_stale_source_hashes() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("stale-completeness-app").expect("dx new");

    let project = dir.path().join("stale-completeness-app");
    approve_runtime_request_receipt(&project);
    let fixture =
        write_complete_runtime_import_plan(&cli, &project, dir.path(), "stale-completeness");
    fs::write(&fixture.build_log, "build replaced after import").expect("replace build log");

    let output_path = dir.path().join("stale-completeness.json");
    let result = cli.cmd_forge(&[
        "launch-runtime-evidence-completeness".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--import-plan".to_string(),
        fixture.import_plan.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ]);

    assert!(
        result.is_err(),
        "stale source hashes must block completeness"
    );
    let report = read_json_value(output_path);
    assert_eq!(report["passed"], false);
    assert!(
        report["artifacts"]["items"]
            .as_array()
            .expect("completeness artifacts")
            .iter()
            .any(|item| item["id"] == "production-contract-build-log"
                && item["source_hash_matches"] == false)
    );
}

#[test]
fn forge_launch_runtime_evidence_finalization_blocks_incomplete_import_plan() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("incomplete-finalization-app").expect("dx new");

    let project = dir.path().join("incomplete-finalization-app");
    approve_runtime_request_receipt(&project);

    let external = dir.path().join("incomplete-finalization-evidence");
    fs::create_dir_all(&external).expect("external evidence dir");
    let build_log = external.join("build.log");
    let route_response = external.join("route-response.json");
    let preview_proof = external.join("preview-proof.json");
    fs::write(&build_log, "build completed").expect("build log");
    fs::write(&preview_proof, "{\"preview\":\"ok\"}").expect("preview proof");

    let import_plan = dir.path().join("incomplete-finalization-import-plan.json");
    let import_result = cli.cmd_forge(&[
        "launch-runtime-evidence-import-plan".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--build-log".to_string(),
        build_log.to_string_lossy().into_owned(),
        "--route-response".to_string(),
        route_response.to_string_lossy().into_owned(),
        "--preview-proof".to_string(),
        preview_proof.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        import_plan.to_string_lossy().into_owned(),
    ]);
    assert!(import_result.is_err(), "incomplete import plan should fail");

    let evidence_path = project.join(NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE);
    let before = read_json_value(evidence_path.clone());
    let output_path = dir.path().join("incomplete-finalization.json");
    let result = cli.cmd_forge(&[
        "launch-runtime-evidence-finalize".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--import-plan".to_string(),
        import_plan.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ]);

    assert!(result.is_err(), "incomplete evidence must not finalize");
    let report = read_json_value(output_path);
    assert_eq!(
        report["schema"],
        "dx.forge.launch_runtime_evidence_finalization"
    );
    assert_eq!(report["passed"], false);
    assert_eq!(report["runtime_evidence"]["updated"], false);
    assert_eq!(read_json_value(evidence_path), before);
    assert!(
        !project
            .join(".dx/forge/runtime/final-launch-evidence-receipt.json")
            .exists()
    );
}

#[test]
fn forge_launch_runtime_evidence_finalization_rejects_stale_source_hashes() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("stale-finalization-app").expect("dx new");

    let project = dir.path().join("stale-finalization-app");
    approve_runtime_request_receipt(&project);
    let fixture =
        write_complete_runtime_import_plan(&cli, &project, dir.path(), "stale-finalization");
    fs::write(&fixture.build_log, "build replaced after import").expect("replace build log");

    let output_path = dir.path().join("stale-finalization.json");
    let result = cli.cmd_forge(&[
        "launch-runtime-evidence-finalize".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--import-plan".to_string(),
        fixture.import_plan.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ]);

    assert!(
        result.is_err(),
        "stale source hashes must block finalization"
    );
    let report = read_json_value(output_path);
    assert_eq!(report["passed"], false);
    assert_eq!(report["runtime_evidence"]["updated"], false);
    assert!(
        report["artifacts"]["items"]
            .as_array()
            .expect("finalization artifacts")
            .iter()
            .any(|item| item["id"] == "production-contract-build-log"
                && item["source_hash_matches"] == false)
    );
}

#[test]
fn forge_launch_runtime_evidence_finalization_updates_evidence_after_completeness() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("complete-finalization-app").expect("dx new");

    let project = dir.path().join("complete-finalization-app");
    approve_runtime_request_receipt(&project);
    let fixture =
        write_complete_runtime_import_plan(&cli, &project, dir.path(), "complete-finalization");

    let output_path = dir.path().join("complete-finalization.json");
    cli.cmd_forge(&[
        "launch-runtime-evidence-finalize".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--import-plan".to_string(),
        fixture.import_plan.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("runtime evidence finalization");

    let report = read_json_value(output_path);
    assert_eq!(
        report["schema"],
        "dx.forge.launch_runtime_evidence_finalization"
    );
    assert_eq!(report["passed"], true);
    assert_eq!(report["no_execution"], true);
    assert_eq!(report["write_mode"], true);
    assert_eq!(report["runtime_evidence"]["updated"], true);
    assert_eq!(
        report["runtime_evidence"]["receipt_path"],
        ".dx/forge/runtime/final-launch-evidence-receipt.json"
    );

    let receipt_path = project.join(".dx/forge/runtime/final-launch-evidence-receipt.json");
    let receipt = read_json_value(receipt_path);
    assert_eq!(
        receipt["schema"],
        "dx.launch.runtime_evidence_finalization_receipt"
    );
    assert_eq!(receipt["artifacts"]["present"], 3);

    let evidence = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE));
    assert_eq!(evidence["status"], "complete");
    assert_eq!(evidence["finalized"], true);
    assert_eq!(
        evidence["finalization_receipt"],
        ".dx/forge/runtime/final-launch-evidence-receipt.json"
    );
    assert_eq!(evidence["collected_evidence"]["present"], 4);
    assert_eq!(evidence["collected_evidence"]["existing_artifacts"], 4);
    assert!(
        evidence["collected_evidence"]["artifacts"]
            .as_array()
            .expect("collected artifacts")
            .iter()
            .any(|artifact| artifact == fixture.route_response.to_string_lossy().as_ref())
    );
    assert!(
        evidence["collected_evidence"]["artifacts"]
            .as_array()
            .expect("collected artifacts")
            .iter()
            .any(|artifact| artifact == fixture.build_log.to_string_lossy().as_ref())
    );
    assert!(
        evidence["collected_evidence"]["artifacts"]
            .as_array()
            .expect("collected artifacts")
            .iter()
            .any(|artifact| artifact == fixture.preview_proof.to_string_lossy().as_ref())
    );
    assert!(
        evidence["required_evidence"]
            .as_array()
            .expect("required evidence")
            .iter()
            .all(|item| item["status"] == "collected")
    );

    cli.cmd_forge(&[
        "launch-runtime-evidence".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        dir.path()
            .join("final-runtime-evidence-report.json")
            .to_string_lossy()
            .into_owned(),
    ])
    .expect("runtime evidence report after finalization");
}

#[test]
fn forge_launch_runtime_evidence_review_fails_without_final_receipt() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("missing-review-receipt-app").expect("dx new");

    let project = dir.path().join("missing-review-receipt-app");
    let output_path = dir.path().join("runtime-evidence-review-missing.json");
    let result = cli.cmd_forge(&[
        "launch-runtime-evidence-review".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ]);

    assert!(
        result.is_err(),
        "review should fail until final runtime evidence receipt exists"
    );
    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.forge.launch_runtime_evidence_review");
    assert_eq!(report["passed"], false);
    assert_eq!(report["no_execution"], true);
    assert_eq!(report["finalization_receipt"]["present"], false);
    assert_eq!(
        report["runtime_evidence"]["status"],
        "awaiting-approved-runtime-run"
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("checks")
            .iter()
            .any(|check| check["name"] == "finalization-receipt" && check["passed"] == false)
    );
}

#[test]
fn forge_launch_runtime_evidence_review_rejects_tampered_finalized_artifacts() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("tampered-review-app").expect("dx new");

    let project = dir.path().join("tampered-review-app");
    approve_runtime_request_receipt(&project);
    let fixture = write_complete_runtime_import_plan(&cli, &project, dir.path(), "tampered-review");
    cli.cmd_forge(&[
        "launch-runtime-evidence-finalize".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--import-plan".to_string(),
        fixture.import_plan.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--json".to_string(),
        "--output".to_string(),
        dir.path()
            .join("tampered-review-finalization.json")
            .to_string_lossy()
            .into_owned(),
    ])
    .expect("runtime evidence finalization");
    fs::write(&fixture.route_response, "{\"status\":500}").expect("tamper route response");

    let output_path = dir.path().join("runtime-evidence-review-tampered.json");
    let result = cli.cmd_forge(&[
        "launch-runtime-evidence-review".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ]);

    assert!(
        result.is_err(),
        "tampered finalized evidence must fail review"
    );
    let report = read_json_value(output_path);
    assert_eq!(report["passed"], false);
    assert_eq!(report["finalization_receipt"]["present"], true);
    assert_eq!(report["runtime_evidence"]["status"], "complete");
    assert!(
        report["artifacts"]["items"]
            .as_array()
            .expect("review artifacts")
            .iter()
            .any(|item| item["id"] == "governed-runtime-route-response"
                && item["source_hash_matches"] == false)
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("checks")
            .iter()
            .any(|check| check["name"] == "finalization-receipt-hashes-match"
                && check["passed"] == false)
    );
}

#[test]
fn forge_launch_runtime_evidence_review_passes_complete_finalized_evidence() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("complete-review-app").expect("dx new");

    let project = dir.path().join("complete-review-app");
    approve_runtime_request_receipt(&project);
    let fixture = write_complete_runtime_import_plan(&cli, &project, dir.path(), "complete-review");
    cli.cmd_forge(&[
        "launch-runtime-evidence-finalize".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--import-plan".to_string(),
        fixture.import_plan.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--json".to_string(),
        "--output".to_string(),
        dir.path()
            .join("complete-review-finalization.json")
            .to_string_lossy()
            .into_owned(),
    ])
    .expect("runtime evidence finalization");

    let output_path = dir.path().join("runtime-evidence-review-complete.json");
    cli.cmd_forge(&[
        "launch-runtime-evidence-review".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("runtime evidence review");

    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.forge.launch_runtime_evidence_review");
    assert_eq!(report["passed"], true);
    assert_eq!(report["no_execution"], true);
    assert_eq!(report["finalization_receipt"]["present"], true);
    assert_eq!(report["runtime_evidence"]["finalized"], true);
    assert_eq!(report["artifacts"]["present"], 3);
    assert_eq!(report["artifacts"]["hashes_matched"], 3);
    assert!(
        report["checks"]
            .as_array()
            .expect("checks")
            .iter()
            .any(|check| check["name"] == "runtime-evidence-finalized" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_packet_fails_without_review_report() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("missing-packet-review-app").expect("dx new");

    let project = dir.path().join("missing-packet-review-app");
    let output_path = dir.path().join("missing-packet-review.json");
    let result = cli.cmd_forge(&[
        "launch-evidence-packet".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ]);

    assert!(
        result.is_err(),
        "release packet must require a written final evidence review report"
    );
    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.forge.launch_evidence_packet");
    assert_eq!(report["passed"], false);
    assert_eq!(report["no_execution"], true);
    assert_eq!(
        report["final_evidence_review"]["review_report"]["present"],
        false
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("packet checks")
            .iter()
            .any(
                |check| check["name"] == "final-evidence-review-report" && check["passed"] == false
            )
    );
}

#[test]
fn forge_launch_evidence_packet_rejects_tampered_packet_inputs() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("tampered-packet-app").expect("dx new");

    let project = dir.path().join("tampered-packet-app");
    approve_runtime_request_receipt(&project);
    let fixture = write_complete_runtime_import_plan(&cli, &project, dir.path(), "tampered-packet");

    cli.cmd_forge(&[
        "launch-runtime-evidence-finalize".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--import-plan".to_string(),
        fixture.import_plan.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--json".to_string(),
        "--output".to_string(),
        dir.path()
            .join("tampered-packet-finalization.json")
            .to_string_lossy()
            .into_owned(),
    ])
    .expect("runtime evidence finalization");

    let review_path = project.join(".dx/forge/runtime/final-launch-evidence-review.json");
    cli.cmd_forge(&[
        "launch-runtime-evidence-review".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        review_path.to_string_lossy().into_owned(),
    ])
    .expect("runtime evidence review");

    fs::write(&fixture.preview_proof, "{\"preview\":\"replaced\"}").expect("tamper preview proof");

    let output_path = dir.path().join("tampered-packet.json");
    let result = cli.cmd_forge(&[
        "launch-evidence-packet".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ]);

    assert!(
        result.is_err(),
        "packet must reject stale reviewed evidence"
    );
    let report = read_json_value(output_path);
    assert_eq!(report["passed"], false);
    assert_eq!(
        report["final_evidence_review"]["review_report"]["passed"],
        true
    );
    assert_eq!(report["final_evidence_review"]["current"]["passed"], false);
    assert!(
            report["checks"]
                .as_array()
                .expect("packet checks")
                .iter()
                .any(|check| check["name"] == "fresh-final-evidence-review"
                    && check["passed"] == false)
        );
}

#[test]
fn forge_launch_evidence_packet_passes_complete_reviewed_evidence() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("complete-packet-app").expect("dx new");

    let project = dir.path().join("complete-packet-app");
    approve_runtime_request_receipt(&project);
    let fixture = write_complete_runtime_import_plan(&cli, &project, dir.path(), "complete-packet");

    cli.cmd_forge(&[
        "launch-runtime-evidence-finalize".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--import-plan".to_string(),
        fixture.import_plan.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--json".to_string(),
        "--output".to_string(),
        dir.path()
            .join("complete-packet-finalization.json")
            .to_string_lossy()
            .into_owned(),
    ])
    .expect("runtime evidence finalization");

    let review_path = project.join(".dx/forge/runtime/final-launch-evidence-review.json");
    cli.cmd_forge(&[
        "launch-runtime-evidence-review".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        review_path.to_string_lossy().into_owned(),
    ])
    .expect("runtime evidence review");

    let output_path = dir.path().join("complete-packet.json");
    cli.cmd_forge(&[
        "launch-evidence-packet".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("launch evidence packet");

    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.forge.launch_evidence_packet");
    assert_eq!(report["passed"], true);
    assert_eq!(report["no_execution"], true);
    assert_eq!(
        report["final_evidence_review"]["review_report"]["passed"],
        true
    );
    assert_eq!(report["final_evidence_review"]["current"]["passed"], true);
    assert!(
        report["contracts"]["items"]
            .as_array()
            .expect("packet contracts")
            .iter()
            .any(|item| item["path"] == ".dx/forge/template-manifest.json"
                && item["hash_algorithm"] == "blake3"
                && item["source_hash"]
                    .as_str()
                    .is_some_and(|hash| hash.starts_with("blake3:")))
    );
    assert!(
        report["source_receipts"]["receipt_count"]
            .as_u64()
            .is_some_and(|count| count > 0)
    );
    assert!(
        report["next_commands"]
            .as_array()
            .expect("next commands")
            .iter()
            .any(|command| command == "open .dx/forge/release/launch-evidence-packet.json")
    );
}

#[test]
fn forge_launch_evidence_operator_index_reports_empty_starter_hints() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("operator-empty-app").expect("dx new");

    let project = dir.path().join("operator-empty-app");
    let output_path = dir.path().join("operator-empty-index.json");
    cli.cmd_forge(&[
        "launch-evidence-operator-index".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("operator index for empty starter");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["schema"], "dx.forge.launch_evidence_operator_index");
    assert_eq!(report["no_execution"], true);
    assert_eq!(report["index"]["step_count"], 8);
    assert_eq!(report["index"]["reads_runtime_artifact_contents"], false);
    assert!(
        report["index"]["present_steps"]
            .as_u64()
            .is_some_and(|count| count >= 5)
    );
    assert!(
        report["stale_step_hints"]
            .as_array()
            .expect("stale hints")
            .iter()
            .any(|hint| hint["step"] == "final-runtime-review"
                && hint["rerun_command"]
                    .as_str()
                    .is_some_and(|command| command.contains("launch-runtime-evidence-review")))
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("operator checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_operator_index_reports_partial_finalization_hints() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("operator-partial-app").expect("dx new");

    let project = dir.path().join("operator-partial-app");
    approve_runtime_request_receipt(&project);
    let fixture =
        write_complete_runtime_import_plan(&cli, &project, dir.path(), "operator-partial");

    cli.cmd_forge(&[
        "launch-runtime-evidence-finalize".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--import-plan".to_string(),
        fixture.import_plan.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--json".to_string(),
        "--output".to_string(),
        dir.path()
            .join("operator-partial-finalization.json")
            .to_string_lossy()
            .into_owned(),
    ])
    .expect("runtime evidence finalization");

    let output_path = dir.path().join("operator-partial-index.json");
    cli.cmd_forge(&[
        "launch-evidence-operator-index".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("operator index should surface missing runtime-review hints");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert!(
        report["steps"]
            .as_array()
            .expect("operator steps")
            .iter()
            .any(|step| step["id"] == "final-launch-evidence-receipt" && step["present"] == true)
    );
    assert!(
        report["stale_step_hints"]
            .as_array()
            .expect("stale hints")
            .iter()
            .any(|hint| hint["step"] == "final-runtime-review"
                && hint["rerun_command"]
                    .as_str()
                    .is_some_and(|command| command.contains("launch-runtime-evidence-review")))
    );
}

#[test]
fn forge_launch_evidence_operator_index_passes_packeted_launch_evidence() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());

    cli.cmd_new("operator-complete-app").expect("dx new");

    let project = dir.path().join("operator-complete-app");
    approve_runtime_request_receipt(&project);
    let fixture =
        write_complete_runtime_import_plan(&cli, &project, dir.path(), "operator-complete");

    cli.cmd_forge(&[
        "launch-runtime-evidence-finalize".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--import-plan".to_string(),
        fixture.import_plan.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--json".to_string(),
        "--output".to_string(),
        dir.path()
            .join("operator-complete-finalization.json")
            .to_string_lossy()
            .into_owned(),
    ])
    .expect("runtime evidence finalization");

    let review_path = project.join(".dx/forge/runtime/final-launch-evidence-review.json");
    cli.cmd_forge(&[
        "launch-runtime-evidence-review".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        review_path.to_string_lossy().into_owned(),
    ])
    .expect("runtime evidence review");

    cli.cmd_forge(&[
        "launch-evidence-packet".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("launch evidence packet");

    cli.cmd_forge(&[
        "launch-evidence-operator-index".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("launch evidence operator index");

    let report =
        read_json_value(project.join(".dx/forge/release/launch-evidence-operator-index.json"));
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["index"]["present_steps"],
        report["index"]["step_count"]
    );
    assert!(
        report["stale_step_hints"]
            .as_array()
            .expect("stale hints")
            .is_empty()
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("operator checks")
            .iter()
            .any(|check| check["name"] == "packet-evidence-indexed" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_status_timeline_reports_empty_project() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("timeline-empty-app");
    fs::create_dir_all(&project).expect("project dir");

    let output_path = dir.path().join("timeline-empty.json");
    let result = cli.cmd_forge(&[
        "launch-evidence-status-timeline".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ]);

    assert!(
        result.is_err(),
        "timeline should block until launch evidence exists"
    );
    let report = read_json_value(output_path);
    assert_eq!(report["schema"], "dx.forge.launch_evidence_status_timeline");
    assert_eq!(report["passed"], false);
    assert_eq!(report["no_execution"], true);
    assert!(report["timeline"]["latest_completed_step"].is_null());
    assert_eq!(
        report["timeline"]["next_blocked_step"],
        "template-readiness"
    );
    assert_eq!(report["timeline"]["reads_runtime_artifact_contents"], false);
}

#[test]
fn forge_launch_evidence_status_timeline_reports_partial_finalization() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("timeline-partial-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
        NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE,
        NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE,
        NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE,
        NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE,
        NEXT_FAMILIAR_LAUNCH_RUNTIME_FINALIZATION_RECEIPT_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    let output_path = dir.path().join("timeline-partial.json");
    let result = cli.cmd_forge(&[
        "launch-evidence-status-timeline".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ]);

    assert!(
        result.is_err(),
        "timeline should report the next blocked launch evidence step"
    );
    let report = read_json_value(output_path);
    assert_eq!(
        report["timeline"]["latest_completed_step"],
        "final-launch-evidence-receipt"
    );
    assert_eq!(
        report["timeline"]["next_blocked_step"],
        "final-runtime-review"
    );
    assert!(
        report["next_commands"]
            .as_array()
            .expect("next commands")
            .iter()
            .any(|command| command
                .as_str()
                .is_some_and(|command| command.contains("launch-runtime-evidence-review")))
    );
}

#[test]
fn forge_launch_evidence_status_timeline_passes_fresh_packeted_evidence() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("timeline-complete-app");
    for path in launch_evidence_timeline_step_paths() {
        write_launch_evidence_timeline_marker(&project, path);
    }
    std::thread::sleep(std::time::Duration::from_millis(5));
    write_launch_evidence_timeline_marker(
        &project,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE,
    );

    cli.cmd_forge(&[
        "launch-evidence-status-timeline".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("fresh status timeline");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE));
    assert_eq!(report["passed"], true);
    assert_eq!(report["timeline"]["release_ready"], true);
    assert_eq!(
        report["timeline"]["latest_completed_step"],
        "launch-evidence-packet"
    );
    assert_eq!(
        report["freshness"]["operator_index_not_older_than_packet"],
        true
    );
}

#[test]
fn forge_launch_evidence_status_timeline_flags_stale_operator_index() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("timeline-stale-app");
    for path in launch_evidence_timeline_step_paths() {
        write_launch_evidence_timeline_marker(&project, path);
    }
    write_launch_evidence_timeline_marker(
        &project,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE,
    );
    std::thread::sleep(std::time::Duration::from_millis(5));
    write_launch_evidence_timeline_marker(&project, NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE);

    let output_path = dir.path().join("timeline-stale.json");
    let result = cli.cmd_forge(&[
        "launch-evidence-status-timeline".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ]);

    assert!(
        result.is_err(),
        "timeline should flag a packet newer than the operator index"
    );
    let report = read_json_value(output_path);
    assert_eq!(
        report["freshness"]["operator_index_not_older_than_packet"],
        false
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("timeline checks")
            .iter()
            .any(|check| check["name"] == "operator-index-current" && check["passed"] == false)
    );
}

#[test]
fn forge_launch_evidence_handoff_digest_writes_fresh_markdown() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("handoff-digest-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
        NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-handoff-digest".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write handoff digest");

    let digest_path = project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE);
    let digest = fs::read_to_string(&digest_path).expect("handoff digest markdown");
    assert!(digest.contains("DX Forge Launch Evidence Handoff Digest"));

    let output_path = dir.path().join("handoff-digest.json");
    cli.cmd_forge(&[
        "launch-evidence-handoff-digest".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--json".to_string(),
        "--output".to_string(),
        output_path.to_string_lossy().into_owned(),
    ])
    .expect("fresh handoff digest report");

    let report = read_json_value(output_path);
    assert_eq!(report["passed"], true);
    assert_eq!(report["digest"]["zed_openable"], true);
    assert_eq!(report["freshness"]["digest_not_older_than_inputs"], true);
    assert!(
        report["checks"]
            .as_array()
            .expect("digest checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_release_checklist_writes_fresh_signoff_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("release-checklist-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-release-checklist".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write release checklist");

    let report =
        read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE));
    assert_eq!(
        report["schema"],
        "dx.forge.launch_evidence_release_checklist"
    );
    assert_eq!(report["passed"], true);
    assert_eq!(report["checklist"]["release_ready"], true);
    assert_eq!(report["freshness"]["checklist_not_older_than_inputs"], true);
    assert!(
        report["checks"]
            .as_array()
            .expect("checklist checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_share_manifest_writes_fresh_export_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("share-manifest-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_STATUS_TIMELINE_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_INDEX_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-share-manifest".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write share manifest");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE));
    assert_eq!(report["schema"], "dx.forge.launch_evidence_share_manifest");
    assert_eq!(report["passed"], true);
    assert_eq!(report["manifest"]["export_target"], "dx-cli-zed");
    assert_eq!(
        report["freshness"]["manifest_not_older_than_release_artifacts"],
        true
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("share manifest checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_archive_index_writes_fresh_archive_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("archive-index-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-archive-index".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write archive index");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE));
    assert_eq!(report["schema"], "dx.forge.launch_evidence_archive_index");
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["archive"]["archive_target"],
        "long-term-launch-handoff"
    );
    assert_eq!(
        report["freshness"]["archive_not_older_than_share_manifest"],
        true
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("archive index checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_archive_receipt_writes_fresh_receipt_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("archive-receipt-app");
    write_launch_evidence_timeline_marker(
        &project,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE,
    );

    cli.cmd_forge(&[
        "launch-evidence-archive-receipt".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write archive receipt");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE));
    assert_eq!(report["schema"], "dx.forge.launch_evidence_archive_receipt");
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["receipt"]["operator_handoff_target"],
        "dx-cli-zed-archive"
    );
    assert_eq!(
        report["freshness"]["receipt_not_older_than_archive_index"],
        true
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("archive receipt checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_archive_ledger_writes_fresh_ledger_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("archive-ledger-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-archive-ledger".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write archive ledger");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_LEDGER_FILE));
    assert_eq!(report["schema"], "dx.forge.launch_evidence_archive_ledger");
    assert_eq!(report["passed"], true);
    assert_eq!(report["ledger"]["ledger_target"], "durable-release-ledger");
    assert_eq!(
        report["freshness"]["ledger_not_older_than_archive_receipt"],
        true
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("archive ledger checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_retention_policy_writes_fresh_policy_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("retention-policy-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_LEDGER_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-retention-policy".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write retention policy");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_POLICY_FILE));
    assert_eq!(
        report["schema"],
        "dx.forge.launch_evidence_retention_policy"
    );
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["policy"]["policy_target"],
        "release-proof-retention"
    );
    assert_eq!(
        report["freshness"]["policy_not_older_than_archive_ledger"],
        true
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("retention policy checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_retention_review_writes_fresh_review_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("retention-review-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_POLICY_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_LEDGER_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_RECEIPT_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_INDEX_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_SHARE_MANIFEST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_CHECKLIST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_DIGEST_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-retention-review".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write retention review");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_REVIEW_FILE));
    assert_eq!(
        report["schema"],
        "dx.forge.launch_evidence_retention_review"
    );
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["review"]["review_target"],
        "post-retention-release-proof"
    );
    assert_eq!(
        report["freshness"]["review_not_older_than_retention_policy"],
        true
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("retention review checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_release_seal_writes_fresh_seal_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("release-seal-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_REVIEW_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_POLICY_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_ARCHIVE_LEDGER_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-release-seal".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write release seal");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE));
    assert_eq!(report["schema"], "dx.forge.launch_evidence_release_seal");
    assert_eq!(report["passed"], true);
    assert_eq!(report["seal"]["seal_target"], "final-launch-handoff-seal");
    assert_eq!(
        report["freshness"]["seal_not_older_than_retention_review"],
        true
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("release seal checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_operator_summary_writes_fresh_summary_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("operator-summary-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_REVIEW_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_PACKET_FILE,
        NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-operator-summary".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write operator summary");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE));
    assert_eq!(
        report["schema"],
        "dx.forge.launch_evidence_operator_summary"
    );
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["summary"]["summary_target"],
        "terminal-friendly-launch-handoff"
    );
    assert_eq!(
        report["freshness"]["summary_not_older_than_release_seal"],
        true
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("operator summary checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_completion_ledger_writes_fresh_ledger_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("completion-ledger-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RETENTION_REVIEW_FILE,
        NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-completion-ledger".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write completion ledger");

    let report =
        read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE));
    assert_eq!(
        report["schema"],
        "dx.forge.launch_evidence_completion_ledger"
    );
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["ledger"]["completion_target"],
        "final-launch-evidence-completion-map"
    );
    assert_eq!(
        report["freshness"]["ledger_not_older_than_operator_summary"],
        true
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("completion ledger checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_closure_memo_writes_fresh_markdown() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("closure-memo-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
        NEXT_FAMILIAR_LAUNCH_RUNTIME_REVIEW_REPORT_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-closure-memo".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write closure memo");

    let memo = fs::read_to_string(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_CLOSURE_MEMO_FILE))
        .expect("closure memo markdown");
    assert!(memo.contains("# DX Forge Launch Evidence Closure Memo"));
    assert!(memo.contains("human-readable-launch-release-closeout"));
    assert!(memo.contains("Closeout Artifacts"));
}

#[test]
fn forge_launch_evidence_final_brief_writes_fresh_brief_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("final-brief-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_CLOSURE_MEMO_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-final-brief".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write final brief");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE));
    assert_eq!(report["schema"], "dx.forge.launch_evidence_final_brief");
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["brief"]["brief_target"],
        "dx-cli-zed-launch-closeout-pointer"
    );
    assert_eq!(
        report["freshness"]["brief_not_older_than_closure_memo"],
        true
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("final brief checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_operator_runbook_writes_fresh_runbook_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("operator-runbook-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_CLOSURE_MEMO_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_SUMMARY_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RELEASE_SEAL_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-operator-runbook".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write operator runbook");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE));
    assert_eq!(
        report["schema"],
        "dx.forge.launch_evidence_operator_runbook"
    );
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["runbook"]["runbook_target"],
        "restartable-dx-worker-checklist"
    );
    assert_eq!(
        report["freshness"]["runbook_not_older_than_final_brief"],
        true
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("operator runbook checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_handoff_capsule_writes_fresh_capsule_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("handoff-capsule-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_CLOSURE_MEMO_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_COMPLETION_LEDGER_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-handoff-capsule".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write handoff capsule");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_CAPSULE_FILE));
    assert_eq!(report["schema"], "dx.forge.launch_evidence_handoff_capsule");
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["capsule"]["capsule_target"],
        "dx-cli-zed-restart-artifact"
    );
    assert_eq!(
        report["freshness"]["capsule_not_older_than_operator_runbook"],
        true
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("handoff capsule checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_resumption_index_writes_fresh_index_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("resumption-index-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_CAPSULE_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_CLOSURE_MEMO_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-resumption-index".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write resumption index");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESUMPTION_INDEX_FILE));
    assert_eq!(
        report["schema"],
        "dx.forge.launch_evidence_resumption_index"
    );
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["index"]["resumption_target"],
        "ordered-dx-cli-zed-restart-lanes"
    );
    assert_eq!(
        report["freshness"]["index_not_older_than_handoff_capsule"],
        true
    );
    assert!(
        report["lanes"]
            .as_array()
            .expect("resumption lanes")
            .iter()
            .any(|lane| lane["id"] == "runtime-approved")
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("resumption index checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_recovery_brief_writes_fresh_markdown() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("recovery-brief-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESUMPTION_INDEX_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_CAPSULE_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_FINAL_BRIEF_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-recovery-brief".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write recovery brief");

    let markdown =
        fs::read_to_string(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_RECOVERY_BRIEF_FILE))
            .expect("recovery brief markdown");
    assert!(markdown.contains("# DX Forge Launch Evidence Recovery Brief"));
    assert!(markdown.contains("human-readable-dx-worker-restart-brief"));
    assert!(markdown.contains("Source-only lane"));
}

#[test]
fn forge_launch_evidence_continuation_packet_writes_fresh_packet_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("continuation-packet-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RECOVERY_BRIEF_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESUMPTION_INDEX_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_HANDOFF_CAPSULE_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-continuation-packet".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write continuation packet");

    let report =
        read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_CONTINUATION_PACKET_FILE));
    assert_eq!(
        report["schema"],
        "dx.forge.launch_evidence_continuation_packet"
    );
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["packet"]["continuation_target"],
        "dx-cli-zed-continuation-packet"
    );
    assert_eq!(
        report["freshness"]["packet_not_older_than_recovery_brief"],
        true
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("continuation packet checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_operator_resume_card_writes_fresh_card_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("operator-resume-card-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_CONTINUATION_PACKET_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RECOVERY_BRIEF_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESUMPTION_INDEX_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RUNBOOK_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-operator-resume-card".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write operator resume card");

    let report =
        read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RESUME_CARD_FILE));
    assert_eq!(
        report["schema"],
        "dx.forge.launch_evidence_operator_resume_card"
    );
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["card"]["resume_target"],
        "terminal-first-dx-resume-card"
    );
    assert_eq!(
        report["freshness"]["card_not_older_than_continuation_packet"],
        true
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("operator resume card checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}

#[test]
fn forge_launch_evidence_restart_ledger_writes_fresh_ledger_json() {
    let dir = tempdir().expect("tempdir");
    let cli = Cli::with_cwd(dir.path().to_path_buf());
    let project = dir.path().join("restart-ledger-app");
    for path in [
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_OPERATOR_RESUME_CARD_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_CONTINUATION_PACKET_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RECOVERY_BRIEF_FILE,
        NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESUMPTION_INDEX_FILE,
    ] {
        write_launch_evidence_timeline_marker(&project, path);
    }

    cli.cmd_forge(&[
        "launch-evidence-restart-ledger".to_string(),
        "--project".to_string(),
        project.to_string_lossy().into_owned(),
        "--write".to_string(),
        "--quiet".to_string(),
    ])
    .expect("write restart ledger");

    let report = read_json_value(project.join(NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_LEDGER_FILE));
    assert_eq!(report["schema"], "dx.forge.launch_evidence_restart_ledger");
    assert_eq!(report["passed"], true);
    assert_eq!(
        report["ledger"]["ledger_target"],
        "durable-dx-restart-ledger"
    );
    assert_eq!(
        report["freshness"]["ledger_not_older_than_operator_resume_card"],
        true
    );
    assert!(
        report["checks"]
            .as_array()
            .expect("restart ledger checks")
            .iter()
            .any(|check| check["name"] == "no-runtime-content-read" && check["passed"] == true)
    );
}
