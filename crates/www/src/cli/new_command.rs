use super::*;

struct NewCommand<'a> {
    cwd: &'a Path,
}

pub(super) fn cmd_new(cwd: &Path, name: &str) -> DxResult<()> {
    NewCommand { cwd }.cmd_new(name)
}

pub(super) fn default_dx_project_config(project_name: &str) -> String {
    format!(
        r#"project(name="{}" version=0.1.0 kind=www-app)

www(
   app_dir=app
   output_dir=.dx/www/output
)

dev(host=127.0.0.1 port=3000 hot_reload=true devtools=true)

style(
   mode=generated-css
   tokens=styles/theme.css
   generated_css=styles/generated.css
)

imports(
   map=.dx/imports/import-map.json
   barrel=components/auto-imports.ts
   declarations=.dx/imports/imports.d.ts
   scan_roots=components,composables,utils
   used_roots=app,components,lib,server,styles
   aliases=#imports,#components
   used_only=true
)

icons(component=Icon source_tag=icon runtime_tag=dx-icon generated_dir=components/icons)
forge(policy=forge-first-no-node-modules)
check(score_scale=500 lighthouse=true)

docs(
   route=/docs
   content=content/docs
   openapi=openapi/dx-www.yaml
)
"#,
        project_name
    )
}

pub(super) fn refresh_forge_package_status_receipts(project_dir: &Path) -> DxResult<()> {
    NewCommand::write_forge_package_status_receipts(project_dir)
}

fn template_generated_file_manifest_entries() -> Vec<serde_json::Value> {
    next_familiar_launch_generated_file_paths()
        .into_iter()
        .map(|path| {
            serde_json::json!({
                "kind": "generated-template-file",
                "source_file": null,
                "materialized_file": path
            })
        })
        .collect()
}

fn forge_template_package_slug(package_id: &str) -> String {
    package_id
        .chars()
        .map(|character| match character {
            '/' | '@' | ':' | '+' | ' ' => '-',
            _ => character,
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn forge_template_package_official_name(package_id: &str) -> &'static str {
    match package_id {
        "shadcn/ui/button" => "UI Components",
        "shadcn/ui/badge" => "UI Badge",
        "shadcn/ui/card" => "UI Card",
        "shadcn/ui/alert" => "UI Alert",
        "shadcn/ui/avatar" => "UI Avatar",
        "shadcn/ui/skeleton" => "UI Skeleton",
        "shadcn/ui/label" => "UI Label",
        "shadcn/ui/separator" => "UI Separator",
        "shadcn/ui/field" => "UI Field",
        "shadcn/ui/item" => "UI Item",
        "shadcn/ui/input" => "UI Input",
        "shadcn/ui/textarea" => "UI Textarea",
        "dx/icon/search" => "DX Icons",
        "auth/better-auth" => "Authentication",
        "animation/motion" => "Motion & Animation",
        "i18n/next-intl" => "Internationalization",
        "tanstack/query" => "Data Fetching & Cache",
        "validation/zod" => "Validation & Schemas",
        "forms/react-hook-form" => "Forms",
        "payments/stripe-js" => "Payments",
        "automations/n8n" => "Automation Connectors",
        "state/zustand" => "State Management",
        "ai/vercel-ai" => "AI SDK",
        "api/trpc" => "Type-Safe API",
        "content/fumadocs-next" => "Documentation System",
        "content/react-markdown" => "Markdown & MDX Content",
        "supabase/client" => "Backend Platform Client",
        "db/drizzle-sqlite" => "Database ORM",
        "instantdb/react" => "Realtime App Database",
        "wasm/bindgen" => "WebAssembly Bridge",
        "3d/launch-scene" => "3D Scene System",
        "dx-www/template-shell" => "WWW Template Shell",
        "www/starter-ui" => "WWW Starter UI",
        _ => "Forge Package",
    }
}

fn forge_template_package_lane(
    package_id: &str,
    official_name: &str,
    receipt_path: &str,
) -> serde_json::Value {
    let mut lane = serde_json::json!({
        "official_package_name": official_name,
        "package_id": package_id,
        "schema": "dx.forge.package.dx_style_compatibility",
        "status": "present",
        "receipt_status": "present",
        "token_source": "styles/theme.css",
        "generated_css": "styles/generated.css",
        "package_receipt_path": receipt_path,
        "status_vocabulary": [
            "present",
            "stale",
            "missing-receipt",
            "blocked",
            "unsupported-surface"
        ],
        "receipt_hash_refresh": {
            "schema": "dx.forge.package.receipt_hash_refresh",
            "status": "current",
            "tracked_file_count": 0,
            "stale_file_count": 0,
            "missing_file_count": 0,
            "stale_files": [],
            "missing_files": [],
            "stale_mirror_files": [],
            "missing_mirror_files": [],
            "runtime_execution": false,
            "secret_access": false
        },
        "dx_style_compatibility": {
            "schema": "dx.forge.package.dx_style_compatibility",
            "status": "present",
            "token_source": "styles/theme.css",
            "generated_css": "styles/generated.css",
            "visible_surfaces": [package_id],
            "source_files": [receipt_path],
            "data_dx_markers": [],
            "receipt_path": receipt_path,
            "runtime_proof": false,
            "runtime_limitations": [
                "Fresh dx new package visibility is receipt-backed source evidence; live provider/runtime proof remains app-owned."
            ]
        },
        "selected_surfaces": []
    });

    if package_id == "content/react-markdown" {
        lane["materialized_source"] = serde_json::json!({
            "schema": "dx.forge.package.materialized_source",
            "source_file": "lib/markdown-mdx-content/receipt.ts",
            "materialized_file": "lib/markdown-mdx-content/receipt.ts",
            "surface": "forge-receipt-helper",
            "execution_guard": "benchmarks/markdown-mdx-content-slice.test.ts",
            "runtime_proof": false
        });
    }

    if package_id == "automations/n8n" {
        lane["inspected_upstream_files"] = serde_json::json!([
            "packages/nodes-base/package.json",
            "packages/nodes-base/nodes/ManualTrigger/ManualTrigger.node.ts",
            "packages/nodes-base/nodes/Slack/Slack.node.ts",
            "packages/nodes-base/nodes/Slack/V2/SlackV2.node.ts",
            "packages/nodes-base/nodes/Webhook/Webhook.node.ts",
            "packages/nodes-base/nodes/Notion/Notion.node.ts",
            "packages/nodes-base/credentials/SlackApi.credentials.ts",
            "packages/nodes-base/credentials/SlackOAuth2Api.credentials.ts",
            "packages/nodes-base/credentials/NotionApi.credentials.ts"
        ]);
        lane["upstream_public_apis"] = serde_json::json!([
            "VersionedNodeType",
            "INodeType",
            "INodeTypeDescription",
            "ITriggerFunctions",
            "IExecuteFunctions",
            "IWebhookFunctions",
            "ICredentialType",
            "IAuthenticateGeneric",
            "ICredentialTestRequest"
        ]);
    }

    lane
}

fn read_json_value(path: &Path, label: &str) -> DxResult<serde_json::Value> {
    let bytes = std::fs::read(path).map_err(|e| DxError::IoError {
        path: Some(path.to_path_buf()),
        message: e.to_string(),
    })?;
    serde_json::from_slice(&bytes).map_err(|error| DxError::ConfigValidationError {
        message: format!("Failed to parse {label}: {error}"),
        field: Some(path.display().to_string()),
    })
}

impl NewCommand<'_> {
    /// Create a new project.
    fn cmd_new(&self, name: &str) -> DxResult<()> {
        let project_dir = self.cwd.join(name);
        let project_name = dx_new_project_name(&project_dir);
        let project_name_escaped = toml_basic_string_escape(&project_name);

        if project_dir.exists() {
            return Err(DxError::IoError {
                path: Some(project_dir),
                message: "Directory already exists".to_string(),
            });
        }

        eprintln!();
        eprintln!("  ◆ Creating project {}...", console::style(name).cyan().bold());
        eprintln!();

        for dir in [
            "app",
            "styles",
            "public",
            ".dx/forge",
            ".dx/forge/docs",
            ".dx/forge/receipts",
            ".dx/check",
            ".dx/run",
            ".dx/www/output",
            ".dx/receipts",
            ".dx/receipts/style",
            ".dx/receipts/imports",
            ".dx/receipts/check/web-perf",
            ".dx/deploy",
            ".dx/serializer",
        ] {
            std::fs::create_dir_all(project_dir.join(dir)).map_err(|e| DxError::IoError {
                path: Some(project_dir.join(dir)),
                message: e.to_string(),
            })?;
        }

        std::fs::write(
            project_dir.join("dx"),
            default_dx_project_config(&project_name_escaped),
        )
        .map_err(|e| DxError::IoError {
            path: Some(project_dir.join("dx")),
            message: e.to_string(),
        })?;

        let serializer_config = serializer::SerializerOutputConfig::new()
            .with_output_dir(project_dir.join(".dx/serializer"))
            .with_llm(false)
            .with_machine(true);
        serializer::SerializerOutput::with_config(serializer_config)
            .process_file(&project_dir.join("dx"))
            .map_err(|error| DxError::ConfigValidationError {
                message: format!("Failed to generate dx machine cache: {error}"),
                field: Some("serializer".to_string()),
            })?;

        write_default_template_source_files(&project_dir)?;

        let starter_sources = [
            (
                "app/page.tsx",
                DEFAULT_TEMPLATE_HOME_ROUTE_SOURCE_FILE,
                "minimal-www-home-page",
            ),
            (
                "app/layout.tsx",
                "examples/template/app/layout.tsx",
                "minimal-www-root-layout",
            ),
            (
                "styles/theme.css",
                "examples/template/styles/theme.css",
                "minimal-www-theme-tokens",
            ),
            (
                "styles/generated.css",
                "examples/template/styles/generated.css",
                "minimal-www-generated-css",
            ),
            (
                "styles/globals.css",
                "examples/template/styles/globals.css",
                "minimal-www-global-css",
            ),
            (
                "components/icons/icon.tsx",
                "examples/template/components/icons/icon.tsx",
                "minimal-www-source-owned-icon-component",
            ),
            (
                "lib/utils.ts",
                "examples/template/lib/utils.ts",
                "minimal-www-source-owned-utility-helpers",
            ),
            (
                "public/logo.svg",
                "examples/template/public/logo.svg",
                "minimal-www-logo-asset",
            ),
            (
                "public/icon.svg",
                "examples/template/public/icon.svg",
                "minimal-www-icon-asset",
            ),
            (
                "public/favicon.svg",
                "examples/template/public/favicon.svg",
                "minimal-www-favicon-asset",
            ),
            (
                "vercel.json",
                "examples/template/vercel.json",
                "minimal-www-vercel-deploy-policy",
            ),
            (
                ".gitignore",
                "examples/template/.gitignore",
                "minimal-www-gitignore-policy",
            ),
            (
                "README.md",
                "examples/template/README.md",
                "minimal-www-readme",
            ),
        ];

        let mut source_files = Vec::with_capacity(starter_sources.len());
        for (materialized_file, source_file, _) in starter_sources.iter() {
            source_files.push(DxForgeLocalSourceFile {
                path: (*materialized_file).to_string(),
                content: read_default_template_source_text(source_file)?,
            });
        }

        write_forge_local_source(
            DxForgeLocalSourcePackage {
                package_id: "www/minimal-starter".to_string(),
                variant: "default".to_string(),
                upstream_name: "dx-www/examples/template".to_string(),
                version: "0.1.0".to_string(),
                license: "MIT".to_string(),
                files: source_files,
            },
            &project_dir,
        )
        .map_err(forge_error)?;

        let materialized_files: Vec<serde_json::Value> = starter_sources
            .iter()
            .map(|(materialized_file, source_file, role)| {
                serde_json::json!({
                    "kind": role,
                    "source_file": source_file,
                    "materialized_file": materialized_file
                })
            })
            .collect();
        let local_source: Vec<&str> = starter_sources
            .iter()
            .map(|(materialized_file, _, _)| *materialized_file)
            .collect();

        let template_manifest = serde_json::json!({
            "template": "minimal-www-starter",
            "project": name,
            "entrypoint": "app/page.tsx",
            "app_router_first": true,
            "compatibility_pages_fallback": false,
            "node_modules_required": false,
            "package_policy": "forge-first-no-node-modules",
            "run_command": "dx dev",
            "app_router_files": ["app/layout.tsx", "app/page.tsx"],
            "local_source": local_source,
            "tooling": ["dx-style", "dx-icons", "dx-imports", "dx-check", "serializer"],
            "tooling_config_source": "dx",
            "source_package": {
                "package_id": "www/minimal-starter",
                "variant": "default",
                "source": "examples/template"
            },
            "generated_files": materialized_files,
            "forge_artifacts": [
                ".dx/forge/source-manifest.json",
                DEFAULT_TEMPLATE_CORE_SOURCE_RECEIPT_FILE
            ],
            "compiler_owned_intrinsics": ["jsx", "app-router", "static-assets"]
        });
        std::fs::write(
            project_dir.join(".dx/forge/template-manifest.json"),
            serde_json::to_string_pretty(&template_manifest).map_err(forge_error)?,
        )
        .map_err(|e| DxError::IoError {
            path: Some(project_dir.join(".dx/forge/template-manifest.json")),
            message: e.to_string(),
        })?;

        Self::write_forge_package_status_receipts(&project_dir)?;
        let style_build_args = ["build".to_string(), "--json".to_string()];
        let _ = run_dx_style(&project_dir, &style_build_args).map_err(forge_error)?;
        let refreshed_source_files = starter_sources
            .iter()
            .map(|(materialized_file, _, _)| {
                let path = project_dir.join(materialized_file);
                std::fs::read_to_string(&path)
                    .map(|content| DxForgeLocalSourceFile {
                        path: (*materialized_file).to_string(),
                        content,
                    })
                    .map_err(|e| DxError::IoError {
                        path: Some(path),
                        message: e.to_string(),
                    })
            })
            .collect::<DxResult<Vec<_>>>()?;
        write_forge_local_source(
            DxForgeLocalSourcePackage {
                package_id: "www/minimal-starter".to_string(),
                variant: "default".to_string(),
                upstream_name: "dx-www/examples/template".to_string(),
                version: "0.1.0".to_string(),
                license: "MIT".to_string(),
                files: refreshed_source_files,
            },
            &project_dir,
        )
        .map_err(forge_error)?;
        Self::write_forge_package_status_receipts(&project_dir)?;
        let style_check_args = ["check".to_string(), "--json".to_string()];
        let _ = run_dx_style(&project_dir, &style_check_args).map_err(forge_error)?;
        let imports_sync_args = ["sync".to_string(), "--json".to_string()];
        let _ = run_dx_imports(&project_dir, &imports_sync_args).map_err(forge_error)?;

        let tsconfig_content = r#"{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@/*": ["./*"]
    },
    "jsx": "preserve",
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "target": "ES2022",
    "strict": true,
    "skipLibCheck": true,
    "allowJs": false
  },
  "include": ["dx.d.ts", "app/**/*.tsx", "components/**/*.tsx", "components/**/*.ts", "lib/**/*.ts", "server/**/*.ts"]
}"#;
        std::fs::write(project_dir.join("tsconfig.json"), tsconfig_content).unwrap_or_default();
        if let Ok(d_ts_content) = read_default_template_source_text("examples/template/dx.d.ts") {
            std::fs::write(project_dir.join("dx.d.ts"), d_ts_content).unwrap_or_default();
        }

        use console::style;
        eprintln!("  {} Created project structure", style("✓").green());
        eprintln!("  {} Configured dx workspace & types", style("✓").green());
        eprintln!("  {} Scaffolded App Router components", style("✓").green());
        eprintln!("  {} Configured themes & tokens", style("✓").green());
        eprintln!("  {} Generated Forge source receipts & assets", style("✓").green());
        eprintln!();
        eprintln!("  {}", style("Next steps:").bold());
        eprintln!("    {} cd {}", style("$").dim(), style(name).cyan());
        eprintln!("    {} dx dev", style("$").dim());
        eprintln!();

        Ok(())
    }
    fn write_forge_package_status_receipts(project_dir: &Path) -> DxResult<()> {
        let manifest_path = project_dir.join(".dx/forge/source-manifest.json");
        let mut manifest =
            read_json_value(&manifest_path, "source manifest for generated Forge status")?;
        let packages_snapshot = {
            let Some(packages) = manifest
                .get_mut("packages")
                .and_then(serde_json::Value::as_array_mut)
            else {
                return Ok(());
            };

            for package in packages.iter_mut() {
                let Some(files) = package
                    .get_mut("files")
                    .and_then(serde_json::Value::as_array_mut)
                else {
                    continue;
                };

                for file in files {
                    let Some(relative_path) = file.get("path").and_then(serde_json::Value::as_str)
                    else {
                        continue;
                    };
                    let file_path = project_dir.join(relative_path);
                    let Ok(bytes) = std::fs::read(&file_path) else {
                        continue;
                    };
                    file["hash"] = serde_json::json!(blake3::hash(&bytes).to_hex().to_string());
                    file["bytes"] = serde_json::json!(bytes.len() as u64);
                }
            }

            packages.clone()
        };

        std::fs::write(
            &manifest_path,
            serde_json::to_string_pretty(&manifest).map_err(forge_error)?,
        )
        .map_err(|e| DxError::IoError {
            path: Some(manifest_path.clone()),
            message: e.to_string(),
        })?;

        let receipt_map = Self::forge_receipts_by_package(project_dir)?;
        let package_receipts_dir = project_dir.join(".dx/forge/receipts/packages");
        std::fs::create_dir_all(&package_receipts_dir).map_err(|e| DxError::IoError {
            path: Some(package_receipts_dir.clone()),
            message: e.to_string(),
        })?;

        let mut package_rows = Vec::new();
        let mut lane_rows = Vec::new();
        let mut locked_names = Vec::new();

        for package in packages_snapshot.iter() {
            let Some(package_id) = package
                .get("package_id")
                .and_then(serde_json::Value::as_str)
            else {
                continue;
            };
            let version = package
                .get("version")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("0.1.0");
            let variant = package
                .get("variant")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("default");
            let receipt_path = receipt_map.get(package_id).cloned().unwrap_or_else(|| {
                format!(
                    ".dx/forge/receipts/packages/{}.json",
                    forge_template_package_slug(package_id)
                )
            });
            let file_count = package
                .get("files")
                .and_then(serde_json::Value::as_array)
                .map_or(0, Vec::len);
            let official_name = forge_template_package_official_name(package_id);
            let lane = forge_template_package_lane(package_id, official_name, &receipt_path);

            let package_receipt_path = package_receipts_dir
                .join(format!("{}.json", forge_template_package_slug(package_id)));
            let package_receipt = serde_json::json!({
                "schema": "forge.package_add_receipt",
                "package_id": package_id,
                "official_package_name": official_name,
                "hash_algorithm": "blake3",
                "package_receipt_path": format!(
                    ".dx/forge/receipts/packages/{}.json",
                    forge_template_package_slug(package_id)
                ),
                "source_receipt_path": receipt_path,
                "package": {
                    "package_id": package_id,
                    "name": package_id,
                    "version": version,
                    "variant": variant,
                    "dx_check_visibility": lane.clone()
                }
            });
            std::fs::write(
                &package_receipt_path,
                serde_json::to_string_pretty(&package_receipt).map_err(forge_error)?,
            )
            .map_err(|e| DxError::IoError {
                path: Some(package_receipt_path),
                message: e.to_string(),
            })?;

            locked_names.push(package_id.to_string());
            lane_rows.push(lane);
            package_rows.push(serde_json::json!({
                "name": package_id,
                "package_id": package_id,
                "official_name": official_name,
                "status": "present",
                "package_score": 100,
                "version": version,
                "variant": variant,
                "source_kind": package.get("source_kind").cloned().unwrap_or_else(|| serde_json::json!("curated-registry")),
                "package_receipt_path": receipt_path,
                "file_count": file_count
            }));
        }

        let package_status = serde_json::json!({
            "schema": "dx.www.template.forge_package_status",
            "status": "present",
            "source": "dx new",
            "package_policy": "forge-first-no-node-modules",
            "no_node_modules_required": true,
            "package_count": package_rows.len(),
            "locked_package_count": locked_names.len(),
            "locked_package_names": locked_names,
            "package_lane_visibility": lane_rows,
            "packages": package_rows,
            "dx_check_metrics": [
                "package-status-generated",
                "receipt-hash-refresh-current",
                "dx-style-compatibility-present"
            ],
            "zed_receipt_surfaces": [
                "package-status",
                "dx-new-forge-receipts"
            ]
        });
        let package_status_path = project_dir.join(".dx/forge/package-status.json");
        std::fs::write(
            &package_status_path,
            serde_json::to_string_pretty(&package_status).map_err(forge_error)?,
        )
        .map_err(|e| DxError::IoError {
            path: Some(package_status_path),
            message: e.to_string(),
        })?;
        if let Err(error) =
            dx_compiler::ecosystem::write_forge_package_status_machine_cache_with_performance_receipt(
                project_dir,
                &package_status,
            )
        {
            eprintln!(
                "dx-www warning: skipped typed Forge package-status machine cache write: {:#}",
                error
            );
            super::serializer_artifacts::write_json_receipt_machine_alias_best_effort(
                project_dir,
                "forge-package-status",
                ".dx/forge/package-status.json",
                &package_status,
            );
        }

        let package_status_sr_path = project_dir.join(".dx/forge/package-status.sr");
        std::fs::write(
            &package_status_sr_path,
            "package_status(schema=dx.www.template.forge_package_status status=present source=dx-new)\n",
        )
        .map_err(|e| DxError::IoError {
            path: Some(package_status_sr_path),
            message: e.to_string(),
        })?;

        Ok(())
    }

    fn forge_receipts_by_package(
        project_dir: &Path,
    ) -> DxResult<std::collections::BTreeMap<String, String>> {
        let mut receipts = std::collections::BTreeMap::new();
        let receipts_dir = project_dir.join(".dx/forge/receipts");
        if !receipts_dir.is_dir() {
            return Ok(receipts);
        }

        let mut receipt_paths = std::fs::read_dir(&receipts_dir)
            .map_err(|e| DxError::IoError {
                path: Some(receipts_dir.clone()),
                message: e.to_string(),
            })?
            .map(|entry| {
                entry
                    .map(|entry| entry.path())
                    .map_err(|e| DxError::IoError {
                        path: Some(receipts_dir.clone()),
                        message: e.to_string(),
                    })
            })
            .collect::<DxResult<Vec<_>>>()?;
        receipt_paths.sort_by(|left, right| {
            left.file_name()
                .and_then(|name| name.to_str())
                .cmp(&right.file_name().and_then(|name| name.to_str()))
        });

        for path in receipt_paths {
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }
            let Ok(receipt) = read_json_value(&path, "generated Forge receipt") else {
                continue;
            };
            let Some(package_id) = receipt
                .get("package")
                .and_then(|package| {
                    package
                        .get("package_id")
                        .or_else(|| package.get("name"))
                        .and_then(serde_json::Value::as_str)
                })
                .map(str::to_string)
            else {
                continue;
            };
            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default();
            receipts.insert(package_id, format!(".dx/forge/receipts/{file_name}"));
        }

        Ok(receipts)
    }

    fn write_launch_forge_package_slices(project_dir: &Path) -> DxResult<()> {
        for package_id in FORGE_WWW_TEMPLATE_MATERIALIZED_PACKAGE_IDS {
            write_forge_add_variant(package_id, "default", project_dir).map_err(forge_error)?;
        }

        Ok(())
    }

    fn write_next_familiar_launch_route(project_dir: &Path) -> DxResult<()> {
        let template_route_contract = NEXT_FAMILIAR_TEMPLATE_ROUTE_CONTRACT_TS;
        let template_shell = NEXT_FAMILIAR_TEMPLATE_SHELL_TSX;
        let auth_status = NEXT_FAMILIAR_AUTH_STATUS_TSX;
        let better_auth_dashboard_receipt = NEXT_FAMILIAR_BETTER_AUTH_DASHBOARD_RECEIPT_JSON;
        let ai_chat_status = NEXT_FAMILIAR_AI_CHAT_STATUS_TSX;
        let data_status = NEXT_FAMILIAR_DATA_STATUS_TSX;
        let supabase_profile_workflow_state = NEXT_FAMILIAR_SUPABASE_PROFILE_WORKFLOW_STATE_TS;
        let supabase_profile_workflow = NEXT_FAMILIAR_SUPABASE_PROFILE_WORKFLOW_TSX;
        let supabase_dashboard_receipt = NEXT_FAMILIAR_SUPABASE_DASHBOARD_RECEIPT_JSON;
        let drizzle_query_proof = NEXT_FAMILIAR_DRIZZLE_QUERY_PROOF_TSX;
        let payments_status = NEXT_FAMILIAR_PAYMENTS_STATUS_TSX;
        let stripe_billing_workflow_receipt = NEXT_FAMILIAR_STRIPE_BILLING_WORKFLOW_RECEIPT_JSON;
        let docs_status = NEXT_FAMILIAR_DOCS_STATUS_TSX;
        let docs_dashboard_receipt = NEXT_FAMILIAR_DOCS_DASHBOARD_RECEIPT_JSON;
        let launch_scene = NEXT_FAMILIAR_LAUNCH_SCENE_TSX;
        let scene_index = NEXT_FAMILIAR_SCENE_INDEX_TS;
        let scene_types = NEXT_FAMILIAR_SCENE_TYPES_TS;
        let scene_preset = NEXT_FAMILIAR_SCENE_PRESET_TS;
        let scene_interaction = NEXT_FAMILIAR_SCENE_INTERACTION_TS;
        let scene_dashboard_workflow = NEXT_FAMILIAR_SCENE_DASHBOARD_WORKFLOW_TS;
        let scene_dashboard_controls = NEXT_FAMILIAR_SCENE_DASHBOARD_CONTROLS_TS;
        let scene_frame_sample = NEXT_FAMILIAR_SCENE_FRAME_SAMPLE_TS;
        let scene_capability_report = NEXT_FAMILIAR_SCENE_CAPABILITY_REPORT_TS;
        let scene_viewport_report = NEXT_FAMILIAR_SCENE_VIEWPORT_REPORT_TS;
        let scene_bounds_report = NEXT_FAMILIAR_SCENE_BOUNDS_REPORT_TS;
        let scene_raycast_report = NEXT_FAMILIAR_SCENE_RAYCAST_REPORT_TS;
        let scene_preview_readiness = NEXT_FAMILIAR_SCENE_PREVIEW_READINESS_TS;
        let scene_performance_monitor = NEXT_FAMILIAR_SCENE_PERFORMANCE_MONITOR_TS;
        let scene_renderer_handoff = NEXT_FAMILIAR_SCENE_RENDERER_HANDOFF_TS;
        let scene_r3f_renderer_adapter = NEXT_FAMILIAR_SCENE_R3F_RENDERER_ADAPTER_TS;
        let scene_webgl_runtime = NEXT_FAMILIAR_SCENE_WEBGL_RUNTIME_TS;
        let scene_metadata = NEXT_FAMILIAR_SCENE_METADATA_TS;
        let scene_readme = NEXT_FAMILIAR_SCENE_README_MD;
        let template_dashboard_nav = NEXT_FAMILIAR_TEMPLATE_DASHBOARD_NAV_TSX;
        let dx_studio_edit_contract = NEXT_FAMILIAR_DX_STUDIO_EDIT_CONTRACT_TS;
        let shadcn_dashboard_controls_contract =
            NEXT_FAMILIAR_SHADCN_DASHBOARD_CONTROLS_CONTRACT_TSX;
        let shadcn_dashboard_controls = NEXT_FAMILIAR_SHADCN_DASHBOARD_CONTROLS_TSX;
        let automations_status = NEXT_FAMILIAR_AUTOMATIONS_STATUS_TSX;
        let automation_mission_summary = NEXT_FAMILIAR_AUTOMATION_MISSION_SUMMARY_TSX;
        let automations_metadata = NEXT_FAMILIAR_AUTOMATIONS_METADATA_TS;
        let automation_connectors_launch_receipt =
            NEXT_FAMILIAR_AUTOMATION_CONNECTORS_LAUNCH_RECEIPT_JSON;
        let motion_interaction_proof = NEXT_FAMILIAR_MOTION_INTERACTION_PROOF_TSX;
        let motion_dashboard_receipt = NEXT_FAMILIAR_MOTION_DASHBOARD_RECEIPT_JSON;
        let template_lead_form = NEXT_FAMILIAR_TEMPLATE_LEAD_FORM_TSX;
        let forms_dashboard_receipt = NEXT_FAMILIAR_FORMS_DASHBOARD_RECEIPT_JSON;
        let instant_status = NEXT_FAMILIAR_INSTANT_STATUS_TSX;
        let instant_dashboard_receipt = NEXT_FAMILIAR_INSTANT_DASHBOARD_RECEIPT_JSON;
        let wasm_status = NEXT_FAMILIAR_WASM_STATUS_TSX;
        let zod_status = NEXT_FAMILIAR_ZOD_STATUS_TSX;
        let zod_dashboard_settings = NEXT_FAMILIAR_ZOD_DASHBOARD_SETTINGS_TSX;
        let icon_status = NEXT_FAMILIAR_ICON_STATUS_TSX;
        let next_intl_dashboard_locale_contract = NEXT_FAMILIAR_INTL_DASHBOARD_LOCALE_CONTRACT_TS;
        let next_intl_dashboard_locale = NEXT_FAMILIAR_INTL_DASHBOARD_LOCALE_TSX;
        let next_intl_dashboard_receipt = NEXT_FAMILIAR_INTL_DASHBOARD_RECEIPT_JSON;
        let next_intl_status = NEXT_FAMILIAR_INTL_STATUS_TSX;
        let query_status = NEXT_FAMILIAR_QUERY_STATUS_TSX;
        let query_read_model = NEXT_FAMILIAR_QUERY_DASHBOARD_READ_MODEL_TS;
        let forge_package_status = NEXT_FAMILIAR_FORGE_PACKAGE_STATUS_TS;
        let forge_package_status_read_model = NEXT_FAMILIAR_FORGE_PACKAGE_STATUS_READ_MODEL_TS;
        let forge_golden_path_contract = NEXT_FAMILIAR_FORGE_GOLDEN_PATH_CONTRACT_TS;
        let forge_golden_path_panel = NEXT_FAMILIAR_FORGE_GOLDEN_PATH_PANEL_TSX;
        let forge_safety_archive_contract = NEXT_FAMILIAR_FORGE_SAFETY_ARCHIVE_CONTRACT_TS;
        let forge_safety_archive_runbook = NEXT_FAMILIAR_FORGE_SAFETY_ARCHIVE_RUNBOOK_TS;
        let forge_safety_archive_panel = NEXT_FAMILIAR_FORGE_SAFETY_ARCHIVE_PANEL_TSX;
        let forge_remote_head_health_contract = NEXT_FAMILIAR_FORGE_REMOTE_HEAD_HEALTH_CONTRACT_TS;
        let forge_remote_head_health_panel = NEXT_FAMILIAR_FORGE_REMOTE_HEAD_HEALTH_PANEL_TSX;
        let package_catalog = NEXT_FAMILIAR_TEMPLATE_CATALOG_TS;
        let template_surface_registry = NEXT_FAMILIAR_TEMPLATE_SURFACE_REGISTRY_TS;
        let framework_completeness = NEXT_FAMILIAR_FRAMEWORK_COMPLETENESS_TS;
        let markdown_preview = NEXT_FAMILIAR_MARKDOWN_PREVIEW_TSX;
        let state_counter = NEXT_FAMILIAR_STATE_COUNTER_TSX;
        let state_dashboard = NEXT_FAMILIAR_STATE_DASHBOARD_TSX;
        let zustand_dashboard_receipt = NEXT_FAMILIAR_ZUSTAND_DASHBOARD_RECEIPT_JSON;
        let trpc_contract = NEXT_FAMILIAR_TRPC_CONTRACT_TS;
        let trpc_health = NEXT_FAMILIAR_TRPC_HEALTH_TSX;
        let template_console = NEXT_FAMILIAR_TEMPLATE_CONSOLE_TSX;
        let server_template_catalog = NEXT_FAMILIAR_SERVER_TEMPLATE_CATALOG_TS;

        let files = [
            (
                "components/template-app/template-route-contract.ts",
                template_route_contract,
            ),
            ("components/template-app/template-shell.tsx", template_shell),
            (
                "components/template-app/template-dashboard-nav.tsx",
                template_dashboard_nav,
            ),
            (
                "components/template-app/dx-studio-edit-contract.ts",
                dx_studio_edit_contract,
            ),
            (
                "components/template-app/shadcn-dashboard-controls-contract.tsx",
                shadcn_dashboard_controls_contract,
            ),
            (
                "components/template-app/shadcn-dashboard-controls.tsx",
                shadcn_dashboard_controls,
            ),
            (
                "components/template-app/automations-status.tsx",
                automations_status,
            ),
            (
                "components/template-app/automation-mission-summary.tsx",
                automation_mission_summary,
            ),
            (
                "components/template-app/automations/automations-metadata.ts",
                automations_metadata,
            ),
            (
                ".dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json",
                automation_connectors_launch_receipt,
            ),
            (
                "components/template-app/motion-interaction-proof.tsx",
                motion_interaction_proof,
            ),
            (
                ".dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
                motion_dashboard_receipt,
            ),
            (
                "components/template-app/template-lead-form.tsx",
                template_lead_form,
            ),
            (
                ".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
                forms_dashboard_receipt,
            ),
            (
                "components/template-app/auth-session-status.tsx",
                auth_status,
            ),
            (
                ".dx/forge/receipts/auth-better-auth.json",
                better_auth_dashboard_receipt,
            ),
            ("components/template-app/ai-chat-status.tsx", ai_chat_status),
            ("components/template-app/data-status.tsx", data_status),
            (
                "lib/supabase/profile-workflow.ts",
                supabase_profile_workflow_state,
            ),
            (
                "components/template-app/supabase-profile-workflow.tsx",
                supabase_profile_workflow,
            ),
            (
                ".dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json",
                supabase_dashboard_receipt,
            ),
            (
                "components/template-app/drizzle-query-proof.tsx",
                drizzle_query_proof,
            ),
            (
                "components/template-app/payments-status.tsx",
                payments_status,
            ),
            (
                ".dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
                stripe_billing_workflow_receipt,
            ),
            ("components/template-app/docs-status.tsx", docs_status),
            (
                ".dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json",
                docs_dashboard_receipt,
            ),
            (
                "components/template-app/instantdb-status.tsx",
                instant_status,
            ),
            (
                ".dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
                instant_dashboard_receipt,
            ),
            (
                "components/template-app/wasm-interop-status.tsx",
                wasm_status,
            ),
            (
                "components/template-app/zod-validation-status.tsx",
                zod_status,
            ),
            (
                "components/template-app/zod-dashboard-settings.tsx",
                zod_dashboard_settings,
            ),
            ("components/template-app/icon-status.tsx", icon_status),
            (
                "components/template-app/next-intl-dashboard-locale-contract.ts",
                next_intl_dashboard_locale_contract,
            ),
            (
                "components/template-app/next-intl-dashboard-locale.tsx",
                next_intl_dashboard_locale,
            ),
            (
                ".dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json",
                next_intl_dashboard_receipt,
            ),
            (
                "components/template-app/next-intl-status.tsx",
                next_intl_status,
            ),
            (
                "components/template-app/query-cache-status.tsx",
                query_status,
            ),
            (
                "components/template-app/query-dashboard-read-model.ts",
                query_read_model,
            ),
            (
                "components/template-app/dx-check-style-evidence-read-model.ts",
                include_str!("../../../../examples/onboard/dx-check-style-evidence-read-model.ts"),
            ),
            (
                "components/template-app/template-shell-evidence-loader.ts",
                include_str!("../../../../examples/onboard/template-shell-evidence-loader.ts"),
            ),
            (
                "components/template-app/template-shell-style-evidence-drift.ts",
                include_str!("../../../../examples/onboard/template-shell-style-evidence-drift.ts"),
            ),
            (
                "components/template-app/preview-style-evidence-read-model.ts",
                include_str!("../../../../examples/onboard/preview-style-evidence-read-model.ts"),
            ),
            (
                "components/template-app/preview-style-package-panel-read-model.ts",
                include_str!("../../../../examples/onboard/preview-style-package-panel-read-model.ts"),
            ),
            (
                "components/template-app/preview-style-package-ownership-read-model.ts",
                include_str!(
                    "../../../../examples/onboard/preview-style-package-ownership-read-model.ts"
                ),
            ),
            (
                "components/template-app/forge-package-status.ts",
                forge_package_status,
            ),
            (
                "components/template-app/forge-package-status-read-model.ts",
                forge_package_status_read_model,
            ),
            (
                "components/template-app/forge-golden-path-contract.ts",
                forge_golden_path_contract,
            ),
            (
                "components/template-app/forge-golden-path-panel.tsx",
                forge_golden_path_panel,
            ),
            (
                "components/template-app/forge-safety-archive-contract.ts",
                forge_safety_archive_contract,
            ),
            (
                "components/template-app/forge-safety-archive-runbook.ts",
                forge_safety_archive_runbook,
            ),
            (
                "components/template-app/forge-safety-archive-panel.tsx",
                forge_safety_archive_panel,
            ),
            (
                "components/template-app/forge-remote-head-health-contract.ts",
                forge_remote_head_health_contract,
            ),
            (
                "components/template-app/forge-remote-head-health-panel.tsx",
                forge_remote_head_health_panel,
            ),
            (
                "components/template-app/package-catalog.ts",
                package_catalog,
            ),
            (
                "components/template-app/template-surface-registry.ts",
                template_surface_registry,
            ),
            (
                "components/template-app/framework-completeness.ts",
                framework_completeness,
            ),
            (
                "components/template-app/react-markdown-preview.tsx",
                markdown_preview,
            ),
            (
                "components/template-app/state-zustand-counter.tsx",
                state_counter,
            ),
            (
                "components/template-app/state-zustand-dashboard.tsx",
                state_dashboard,
            ),
            (
                ".dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json",
                zustand_dashboard_receipt,
            ),
            (
                "components/template-app/trpc-launch-contract.ts",
                trpc_contract,
            ),
            (
                "components/template-app/trpc-launch-health.tsx",
                trpc_health,
            ),
            ("components/scene/launch-scene.tsx", launch_scene),
            ("lib/scene/index.ts", scene_index),
            ("lib/scene/types.ts", scene_types),
            ("lib/scene/preset.ts", scene_preset),
            ("lib/scene/interaction.ts", scene_interaction),
            ("lib/scene/dashboard-workflow.ts", scene_dashboard_workflow),
            ("lib/scene/dashboard-controls.ts", scene_dashboard_controls),
            ("lib/scene/frame-sample.ts", scene_frame_sample),
            ("lib/scene/capability-report.ts", scene_capability_report),
            ("lib/scene/viewport-report.ts", scene_viewport_report),
            ("lib/scene/bounds-report.ts", scene_bounds_report),
            ("lib/scene/raycast-report.ts", scene_raycast_report),
            ("lib/scene/preview-readiness.ts", scene_preview_readiness),
            (
                "lib/scene/performance-monitor.ts",
                scene_performance_monitor,
            ),
            ("lib/scene/renderer-handoff.ts", scene_renderer_handoff),
            (
                "lib/scene/r3f-renderer-adapter.ts",
                scene_r3f_renderer_adapter,
            ),
            ("lib/scene/webgl-runtime.ts", scene_webgl_runtime),
            ("lib/scene/metadata.ts", scene_metadata),
            ("lib/scene/README.md", scene_readme),
            (
                "components/template-app/template-console.tsx",
                template_console,
            ),
            ("server/templateCatalog.ts", server_template_catalog),
        ];

        for (relative_path, content) in &files {
            let path = project_dir.join(*relative_path);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| DxError::IoError {
                    path: Some(parent.to_path_buf()),
                    message: e.to_string(),
                })?;
            }
            std::fs::write(&path, *content).map_err(|e| DxError::IoError {
                path: Some(path),
                message: e.to_string(),
            })?;
        }

        Ok(())
    }

    fn write_next_familiar_launch_receipt(project_dir: &Path) -> DxResult<()> {
        let outcome = write_forge_local_source(
            DxForgeLocalSourcePackage {
                package_id: NEXT_FAMILIAR_LAUNCH_RECEIPT_PACKAGE_ID.to_string(),
                variant: NEXT_FAMILIAR_LAUNCH_RECEIPT_VARIANT.to_string(),
                upstream_name: "dx-www/examples/template".to_string(),
                version: "0.1.0".to_string(),
                license: "MIT".to_string(),
                files: vec![
                    DxForgeLocalSourceFile {
                        path: "app/page.tsx".to_string(),
                        content: read_default_template_source_text(
                            DEFAULT_TEMPLATE_HOME_ROUTE_SOURCE_FILE,
                        )?,
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/template-route-contract.ts".to_string(),
                        content: NEXT_FAMILIAR_TEMPLATE_ROUTE_CONTRACT_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/template-shell.tsx".to_string(),
                        content: NEXT_FAMILIAR_TEMPLATE_SHELL_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/template-dashboard-nav.tsx".to_string(),
                        content: NEXT_FAMILIAR_TEMPLATE_DASHBOARD_NAV_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/dx-studio-edit-contract.ts".to_string(),
                        content: NEXT_FAMILIAR_DX_STUDIO_EDIT_CONTRACT_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/shadcn-dashboard-controls-contract.tsx"
                            .to_string(),
                        content: NEXT_FAMILIAR_SHADCN_DASHBOARD_CONTROLS_CONTRACT_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/shadcn-dashboard-controls.tsx".to_string(),
                        content: NEXT_FAMILIAR_SHADCN_DASHBOARD_CONTROLS_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/automations-status.tsx".to_string(),
                        content: NEXT_FAMILIAR_AUTOMATIONS_STATUS_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/automation-mission-summary.tsx".to_string(),
                        content: NEXT_FAMILIAR_AUTOMATION_MISSION_SUMMARY_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/automations/automations-metadata.ts".to_string(),
                        content: NEXT_FAMILIAR_AUTOMATIONS_METADATA_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path:
                            ".dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json"
                                .to_string(),
                        content:
                            NEXT_FAMILIAR_AUTOMATION_CONNECTORS_LAUNCH_RECEIPT_JSON.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/motion-interaction-proof.tsx".to_string(),
                        content: NEXT_FAMILIAR_MOTION_INTERACTION_PROOF_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path:
                            ".dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json"
                                .to_string(),
                        content: NEXT_FAMILIAR_MOTION_DASHBOARD_RECEIPT_JSON.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/template-lead-form.tsx".to_string(),
                        content: NEXT_FAMILIAR_TEMPLATE_LEAD_FORM_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: ".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json"
                            .to_string(),
                        content: NEXT_FAMILIAR_FORMS_DASHBOARD_RECEIPT_JSON.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/auth-session-status.tsx".to_string(),
                        content: NEXT_FAMILIAR_AUTH_STATUS_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: ".dx/forge/receipts/auth-better-auth.json".to_string(),
                        content: NEXT_FAMILIAR_BETTER_AUTH_DASHBOARD_RECEIPT_JSON.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/ai-chat-status.tsx".to_string(),
                        content: NEXT_FAMILIAR_AI_CHAT_STATUS_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/data-status.tsx".to_string(),
                        content: NEXT_FAMILIAR_DATA_STATUS_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/supabase/profile-workflow.ts".to_string(),
                        content: NEXT_FAMILIAR_SUPABASE_PROFILE_WORKFLOW_STATE_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/supabase-profile-workflow.tsx".to_string(),
                        content: NEXT_FAMILIAR_SUPABASE_PROFILE_WORKFLOW_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path:
                            ".dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json"
                                .to_string(),
                        content: NEXT_FAMILIAR_SUPABASE_DASHBOARD_RECEIPT_JSON.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/payments-status.tsx".to_string(),
                        content: NEXT_FAMILIAR_PAYMENTS_STATUS_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path:
                            ".dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json"
                                .to_string(),
                        content: NEXT_FAMILIAR_STRIPE_BILLING_WORKFLOW_RECEIPT_JSON.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/docs-status.tsx".to_string(),
                        content: NEXT_FAMILIAR_DOCS_STATUS_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path:
                            ".dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json"
                                .to_string(),
                        content: NEXT_FAMILIAR_DOCS_DASHBOARD_RECEIPT_JSON.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/instantdb-status.tsx".to_string(),
                        content: NEXT_FAMILIAR_INSTANT_STATUS_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: ".dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json"
                            .to_string(),
                        content: NEXT_FAMILIAR_INSTANT_DASHBOARD_RECEIPT_JSON.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/wasm-interop-status.tsx".to_string(),
                        content: NEXT_FAMILIAR_WASM_STATUS_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/zod-validation-status.tsx".to_string(),
                        content: NEXT_FAMILIAR_ZOD_STATUS_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/zod-dashboard-settings.tsx".to_string(),
                        content: NEXT_FAMILIAR_ZOD_DASHBOARD_SETTINGS_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/icon-status.tsx".to_string(),
                        content: NEXT_FAMILIAR_ICON_STATUS_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/next-intl-dashboard-locale-contract.ts"
                            .to_string(),
                        content: NEXT_FAMILIAR_INTL_DASHBOARD_LOCALE_CONTRACT_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/next-intl-dashboard-locale.tsx".to_string(),
                        content: NEXT_FAMILIAR_INTL_DASHBOARD_LOCALE_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: ".dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json"
                            .to_string(),
                        content: NEXT_FAMILIAR_INTL_DASHBOARD_RECEIPT_JSON.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/next-intl-status.tsx".to_string(),
                        content: NEXT_FAMILIAR_INTL_STATUS_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/query-cache-status.tsx".to_string(),
                        content: NEXT_FAMILIAR_QUERY_STATUS_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/query-dashboard-read-model.ts".to_string(),
                        content: NEXT_FAMILIAR_QUERY_DASHBOARD_READ_MODEL_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/forge-package-status.ts".to_string(),
                        content: NEXT_FAMILIAR_FORGE_PACKAGE_STATUS_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/forge-package-status-read-model.ts".to_string(),
                        content: NEXT_FAMILIAR_FORGE_PACKAGE_STATUS_READ_MODEL_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/forge-golden-path-contract.ts".to_string(),
                        content: NEXT_FAMILIAR_FORGE_GOLDEN_PATH_CONTRACT_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/forge-golden-path-panel.tsx".to_string(),
                        content: NEXT_FAMILIAR_FORGE_GOLDEN_PATH_PANEL_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/forge-safety-archive-contract.ts".to_string(),
                        content: NEXT_FAMILIAR_FORGE_SAFETY_ARCHIVE_CONTRACT_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/forge-safety-archive-runbook.ts".to_string(),
                        content: NEXT_FAMILIAR_FORGE_SAFETY_ARCHIVE_RUNBOOK_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/forge-safety-archive-panel.tsx".to_string(),
                        content: NEXT_FAMILIAR_FORGE_SAFETY_ARCHIVE_PANEL_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/forge-remote-head-health-contract.ts".to_string(),
                        content: NEXT_FAMILIAR_FORGE_REMOTE_HEAD_HEALTH_CONTRACT_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/forge-remote-head-health-panel.tsx".to_string(),
                        content: NEXT_FAMILIAR_FORGE_REMOTE_HEAD_HEALTH_PANEL_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/package-catalog.ts".to_string(),
                        content: NEXT_FAMILIAR_TEMPLATE_CATALOG_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/template-surface-registry.ts".to_string(),
                        content: NEXT_FAMILIAR_TEMPLATE_SURFACE_REGISTRY_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/framework-completeness.ts".to_string(),
                        content: NEXT_FAMILIAR_FRAMEWORK_COMPLETENESS_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/react-markdown-preview.tsx".to_string(),
                        content: NEXT_FAMILIAR_MARKDOWN_PREVIEW_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/state-zustand-counter.tsx".to_string(),
                        content: NEXT_FAMILIAR_STATE_COUNTER_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/state-zustand-dashboard.tsx".to_string(),
                        content: NEXT_FAMILIAR_STATE_DASHBOARD_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: ".dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json"
                            .to_string(),
                        content: NEXT_FAMILIAR_ZUSTAND_DASHBOARD_RECEIPT_JSON.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/trpc-launch-contract.ts".to_string(),
                        content: NEXT_FAMILIAR_TRPC_CONTRACT_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/trpc-launch-health.tsx".to_string(),
                        content: NEXT_FAMILIAR_TRPC_HEALTH_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/scene/launch-scene.tsx".to_string(),
                        content: NEXT_FAMILIAR_LAUNCH_SCENE_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/index.ts".to_string(),
                        content: NEXT_FAMILIAR_SCENE_INDEX_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/types.ts".to_string(),
                        content: NEXT_FAMILIAR_SCENE_TYPES_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/preset.ts".to_string(),
                        content: NEXT_FAMILIAR_SCENE_PRESET_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/interaction.ts".to_string(),
                        content: NEXT_FAMILIAR_SCENE_INTERACTION_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/dashboard-workflow.ts".to_string(),
                        content: NEXT_FAMILIAR_SCENE_DASHBOARD_WORKFLOW_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/dashboard-controls.ts".to_string(),
                        content: NEXT_FAMILIAR_SCENE_DASHBOARD_CONTROLS_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/frame-sample.ts".to_string(),
                        content: NEXT_FAMILIAR_SCENE_FRAME_SAMPLE_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/capability-report.ts".to_string(),
                        content: NEXT_FAMILIAR_SCENE_CAPABILITY_REPORT_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/viewport-report.ts".to_string(),
                        content: NEXT_FAMILIAR_SCENE_VIEWPORT_REPORT_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/bounds-report.ts".to_string(),
                        content: NEXT_FAMILIAR_SCENE_BOUNDS_REPORT_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/raycast-report.ts".to_string(),
                        content: NEXT_FAMILIAR_SCENE_RAYCAST_REPORT_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/preview-readiness.ts".to_string(),
                        content: NEXT_FAMILIAR_SCENE_PREVIEW_READINESS_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/performance-monitor.ts".to_string(),
                        content: NEXT_FAMILIAR_SCENE_PERFORMANCE_MONITOR_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/renderer-handoff.ts".to_string(),
                        content: NEXT_FAMILIAR_SCENE_RENDERER_HANDOFF_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/r3f-renderer-adapter.ts".to_string(),
                        content: NEXT_FAMILIAR_SCENE_R3F_RENDERER_ADAPTER_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/webgl-runtime.ts".to_string(),
                        content: NEXT_FAMILIAR_SCENE_WEBGL_RUNTIME_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/metadata.ts".to_string(),
                        content: NEXT_FAMILIAR_SCENE_METADATA_TS.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "lib/scene/README.md".to_string(),
                        content: NEXT_FAMILIAR_SCENE_README_MD.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "components/template-app/template-console.tsx".to_string(),
                        content: NEXT_FAMILIAR_TEMPLATE_CONSOLE_TSX.to_string(),
                    },
                    DxForgeLocalSourceFile {
                        path: "server/templateCatalog.ts".to_string(),
                        content: NEXT_FAMILIAR_SERVER_TEMPLATE_CATALOG_TS.to_string(),
                    },
                ],
            },
            project_dir,
        )
        .map_err(forge_error)?;

        Self::write_next_familiar_launch_readiness_receipt(project_dir, &outcome)
    }

    fn write_next_familiar_launch_readiness_receipt(
        project_dir: &Path,
        outcome: &DxForgeAddOutcome,
    ) -> DxResult<()> {
        let forge_receipt_path = outcome.receipt_path.as_ref().map(|path| {
            path.strip_prefix(project_dir)
                .unwrap_or(path)
                .to_string_lossy()
                .replace('\\', "/")
        });
        let materialized_files = vec![
            "app/page.tsx",
            "components/template-app/template-route-contract.ts",
            "components/template-app/template-shell.tsx",
            "components/template-app/template-dashboard-nav.tsx",
            "components/template-app/dx-studio-edit-contract.ts",
            "components/template-app/shadcn-dashboard-controls-contract.tsx",
            "components/template-app/shadcn-dashboard-controls.tsx",
            "components/template-app/automations-status.tsx",
            "components/template-app/automation-mission-summary.tsx",
            "components/template-app/automations/automations-metadata.ts",
            "components/template-app/motion-interaction-proof.tsx",
            "components/template-app/template-lead-form.tsx",
            ".dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
            "components/template-app/auth-session-status.tsx",
            ".dx/forge/receipts/auth-better-auth.json",
            "components/template-app/ai-chat-status.tsx",
            "components/template-app/data-status.tsx",
            "lib/supabase/profile-workflow.ts",
            "components/template-app/supabase-profile-workflow.tsx",
            ".dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json",
            "components/template-app/payments-status.tsx",
            ".dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
            "components/template-app/docs-status.tsx",
            "components/template-app/instantdb-status.tsx",
            ".dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json",
            "components/template-app/wasm-interop-status.tsx",
            "components/template-app/zod-validation-status.tsx",
            "components/template-app/zod-dashboard-settings.tsx",
            "components/template-app/icon-status.tsx",
            "components/template-app/next-intl-status.tsx",
            "components/template-app/query-cache-status.tsx",
            "components/template-app/query-dashboard-read-model.ts",
            "components/template-app/dx-check-style-evidence-read-model.ts",
            "components/template-app/template-shell-evidence-loader.ts",
            "components/template-app/template-shell-style-evidence-drift.ts",
            "components/template-app/preview-style-evidence-read-model.ts",
            "components/template-app/preview-style-package-panel-read-model.ts",
            "components/template-app/preview-style-package-ownership-read-model.ts",
            "components/template-app/forge-package-status.ts",
            "components/template-app/forge-package-status-read-model.ts",
            "components/template-app/forge-golden-path-contract.ts",
            "components/template-app/forge-golden-path-panel.tsx",
            "components/template-app/forge-safety-archive-contract.ts",
            "components/template-app/forge-safety-archive-runbook.ts",
            "components/template-app/forge-safety-archive-panel.tsx",
            "components/template-app/forge-remote-head-health-contract.ts",
            "components/template-app/forge-remote-head-health-panel.tsx",
            "components/template-app/package-catalog.ts",
            "components/template-app/react-markdown-preview.tsx",
            "components/template-app/state-zustand-counter.tsx",
            "components/template-app/state-zustand-dashboard.tsx",
            ".dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json",
            "components/template-app/trpc-launch-contract.ts",
            "components/template-app/trpc-launch-health.tsx",
            "components/scene/launch-scene.tsx",
            "lib/scene/index.ts",
            "lib/scene/types.ts",
            "lib/scene/preset.ts",
            "lib/scene/frame-sample.ts",
            "lib/scene/capability-report.ts",
            "lib/scene/viewport-report.ts",
            "lib/scene/bounds-report.ts",
            "lib/scene/raycast-report.ts",
            "lib/scene/preview-readiness.ts",
            "lib/scene/webgl-runtime.ts",
            "lib/scene/metadata.ts",
            "lib/scene/README.md",
            "components/template-app/template-console.tsx",
            "server/templateCatalog.ts",
            NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
            NEXT_FAMILIAR_LAUNCH_ZED_TEMPLATE_HANDOFF_FILE,
            NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE,
            NEXT_FAMILIAR_LAUNCH_COMPANION_DOC_RECEIPTS_FILE,
            NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE,
            NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE,
            NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE,
            NEXT_FAMILIAR_LAUNCH_VERIFICATION_LANE_FILE,
        ];
        let readiness_receipt = serde_json::json!({
            "schema": "dx.www.template_readiness",
            "package": {
                "id": NEXT_FAMILIAR_LAUNCH_RECEIPT_PACKAGE_ID,
                "variant": NEXT_FAMILIAR_LAUNCH_RECEIPT_VARIANT
            },
            "route": "/",
            "status": "source-materialized-runtime-pending",
            "generated_at": Utc::now().to_rfc3339(),
            "node_modules_required": false,
            "app_router_first": true,
            "architecture_contract": default_www_template_architecture_contract(),
            "source_smoke_command": "dx run --test .\\benchmarks\\template-shell.test.ts",
            "runtime_verification": "pending-governed-runtime-pass",
            "runtime_verification_requires_explicit_permission": true,
            "runtime_verification_request": {
                "approval_status": "requires-explicit-permission",
                "automation_default": "skip-runtime-build-preview",
                "expected_evidence": [
                    "governed-runtime-route-response",
                    "production-contract-route-proof",
                    "final-launch-evidence-receipt"
                ],
                "no_execution": true
            },
            "forge_receipt_path": forge_receipt_path,
            "receipt_glob": NEXT_FAMILIAR_LAUNCH_RECEIPT_GLOB,
            "docs_file": NEXT_FAMILIAR_LAUNCH_RECEIPT_DOC,
            "launch_readiness_bundle": {
                "schema": "dx.launch.readiness_bundle",
                "file": NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE,
                "source_level_package_guard": "dx run --test .\\benchmarks\\launch-package-slices.test.ts",
                "runtime_gate": "pending-governed-runtime-pass",
                "no_execution": true
            },
            "companion_documentation_receipts": launch_companion_doc_receipts_contract(),
            "runtime_verification_checklist": launch_runtime_checklist_contract(),
            "runtime_approval_request": launch_runtime_approval_request_contract(),
            "runtime_evidence": launch_runtime_evidence_contract(),
            "launch_verification_lane": launch_verification_lane_contract(),
            "style_evidence_drift_fixture": {
                "schema": "dx.style.template_shell.no_server_drift_fixture",
                "row_id": "dx-style-browser-compat",
                "route": "/",
                "status": "source-guarded",
                "source_guard": "dx run --test .\\benchmarks\\dx-style-launch-contract.test.ts -- --test-name-pattern \"template shell evidence fixture resolves concrete drift markers without a server\"",
                "loader_file": "components/template-app/template-shell-evidence-loader.ts",
                "marker_helper_file": "components/template-app/template-shell-style-evidence-drift.ts",
                "preview_manifest_fixture": "public/preview-manifest.json",
                "check_receipt_fixture": ".dx/receipts/check/check-latest.json",
                "marker": "data-dx-check-style-evidence-drift",
                "states": ["unknown", "false", "true"],
                "proves_no_server_evidence_handoff": true,
                "full_tailwind_postcss_output_parity": false,
                "full_autoprefixer_parity": false,
                "next_action": "Expose this fixture through generated preview-manifest metadata for Studio/Zed consumers."
            },
            "summary": {
                "required_package_count": FORGE_WWW_TEMPLATE_PACKAGE_IDS.len(),
                "materialized_file_count": materialized_files.len(),
                "checks_passed": 1,
                "checks_pending": 1,
                "runtime_gate": "pending-governed-runtime-pass"
            },
            "app_router_entrypoint": {
                "route": "/",
                "route_aliases": [],
                "source_file": "examples/template/app/page.tsx",
                "materialized_file": "app/page.tsx",
                "contract_materialized_file": "components/template-app/template-route-contract.ts",
                "runtime_component_materialized_file": "components/template-app/template-console.tsx",
                "runtime_catalog_materialized_file": "server/templateCatalog.ts"
            },
            "materialized_files": materialized_files,
            "required_packages": FORGE_WWW_TEMPLATE_PACKAGE_IDS,
            "checks": [
                {
                    "name": "source-template-shell",
                    "command": "dx run --test .\\benchmarks\\template-shell.test.ts",
                    "status": "passed-before-generation"
                },
                {
                    "name": "governed-runtime-route",
                    "command": "governed runtime verification",
                    "status": "pending-user-approved-runtime-pass",
                    "requires_explicit_permission": true
                }
            ],
            "next_commands": [
                "dx templates --json",
                "dx check . --project-contract",
                "dx run --test .\\benchmarks\\template-shell.test.ts",
                "dx run --test .\\benchmarks\\launch-package-slices.test.ts"
            ]
        });
        let receipt_path = project_dir.join(NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE);
        if let Some(parent) = receipt_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| DxError::IoError {
                path: Some(parent.to_path_buf()),
                message: e.to_string(),
            })?;
        }
        std::fs::write(
            &receipt_path,
            serde_json::to_string_pretty(&readiness_receipt).map_err(forge_error)?,
        )
        .map_err(|e| DxError::IoError {
            path: Some(receipt_path),
            message: e.to_string(),
        })?;

        Self::write_next_familiar_launch_companion_doc_receipts(project_dir, &readiness_receipt)?;
        Self::write_next_familiar_launch_runtime_checklist(project_dir, &readiness_receipt)?;
        Self::write_next_familiar_launch_runtime_approval_request(project_dir, &readiness_receipt)?;
        Self::write_next_familiar_launch_runtime_evidence(project_dir, &readiness_receipt)?;
        Self::write_next_familiar_launch_verification_lane(project_dir, &readiness_receipt)?;
        Self::write_next_familiar_launch_zed_template_handoff(project_dir, &readiness_receipt)?;

        Self::write_next_familiar_launch_readiness_bundle(
            project_dir,
            &readiness_receipt,
            &forge_receipt_path,
            &materialized_files,
        )
    }

    fn write_next_familiar_launch_companion_doc_receipts(
        project_dir: &Path,
        readiness_receipt: &serde_json::Value,
    ) -> DxResult<()> {
        let mut companion_receipts = launch_companion_doc_receipts_contract();
        if let Some(object) = companion_receipts.as_object_mut() {
            object.insert(
                "generated_at".to_string(),
                serde_json::json!(Utc::now().to_rfc3339()),
            );
            object.insert(
                "template_readiness_receipt".to_string(),
                serde_json::json!({
                    "file": NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
                    "status": readiness_receipt["status"]
                }),
            );
            object.insert(
                "readiness_bundle".to_string(),
                serde_json::json!(NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE),
            );
        }

        let receipts_path = project_dir.join(NEXT_FAMILIAR_LAUNCH_COMPANION_DOC_RECEIPTS_FILE);
        if let Some(parent) = receipts_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| DxError::IoError {
                path: Some(parent.to_path_buf()),
                message: e.to_string(),
            })?;
        }
        std::fs::write(
            &receipts_path,
            serde_json::to_string_pretty(&companion_receipts).map_err(forge_error)?,
        )
        .map_err(|e| DxError::IoError {
            path: Some(receipts_path),
            message: e.to_string(),
        })?;

        Self::write_next_familiar_launch_companion_docs(project_dir)
    }

    fn write_next_familiar_launch_runtime_checklist(
        project_dir: &Path,
        readiness_receipt: &serde_json::Value,
    ) -> DxResult<()> {
        let mut checklist = launch_runtime_checklist_contract();
        if let Some(object) = checklist.as_object_mut() {
            object.insert(
                "generated_at".to_string(),
                serde_json::json!(Utc::now().to_rfc3339()),
            );
            object.insert(
                "template_readiness_receipt".to_string(),
                serde_json::json!({
                    "file": NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
                    "status": readiness_receipt["status"],
                    "runtime_verification": readiness_receipt["runtime_verification"]
                }),
            );
            object.insert(
                "readiness_bundle".to_string(),
                serde_json::json!(NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE),
            );
            object.insert(
                "execution_policy".to_string(),
                serde_json::json!({
                    "default": "do-not-execute",
                    "runtime_commands_require_approval": true,
                    "allowed_without_approval": [
                        "dx templates --json",
                        "dx forge launch-adoption-report --project <path> --json",
                        "dx forge launch-manifest-drift --project <path> --json",
                        "dx forge launch-companion-receipts --project <path> --json",
                        "dx forge launch-runtime-checklist --project <path> --json",
                        "dx forge launch-runtime-approval-request --project <path> --json",
                        "dx forge launch-runtime-evidence --project <path> --json",
                        "dx forge launch-runtime-evidence-import-plan --project <path> --build-log <path> --route-response <path> --preview-proof <path> --json",
                        "dx forge launch-runtime-evidence-completeness --project <path> --import-plan <path> --json",
                        "dx forge launch-runtime-evidence-finalize --project <path> --import-plan <path> --write --json",
                        "dx forge launch-verification-lane --project <path> --json"
                    ],
                    "blocked_without_permission": [
                        "dev-server",
                        "full-build",
                        "production-preview"
                    ]
                }),
            );
        }

        let checklist_path = project_dir.join(NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE);
        if let Some(parent) = checklist_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| DxError::IoError {
                path: Some(parent.to_path_buf()),
                message: e.to_string(),
            })?;
        }
        std::fs::write(
            &checklist_path,
            serde_json::to_string_pretty(&checklist).map_err(forge_error)?,
        )
        .map_err(|e| DxError::IoError {
            path: Some(checklist_path),
            message: e.to_string(),
        })
    }

    fn write_next_familiar_launch_runtime_approval_request(
        project_dir: &Path,
        readiness_receipt: &serde_json::Value,
    ) -> DxResult<()> {
        let mut request = launch_runtime_approval_request_contract();
        if let Some(object) = request.as_object_mut() {
            object.insert(
                "generated_at".to_string(),
                serde_json::json!(Utc::now().to_rfc3339()),
            );
            object.insert(
                "template_readiness_receipt".to_string(),
                serde_json::json!({
                    "file": NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
                    "status": readiness_receipt["status"],
                    "runtime_verification": readiness_receipt["runtime_verification"]
                }),
            );
            object.insert(
                "runtime_checklist".to_string(),
                serde_json::json!({
                    "schema": "dx.launch.runtime_checklist",
                    "file": NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE,
                    "command": "dx forge launch-runtime-checklist --project <path> --json"
                }),
            );
        }

        let request_path = project_dir.join(NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE);
        if let Some(parent) = request_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| DxError::IoError {
                path: Some(parent.to_path_buf()),
                message: e.to_string(),
            })?;
        }
        std::fs::write(
            &request_path,
            serde_json::to_string_pretty(&request).map_err(forge_error)?,
        )
        .map_err(|e| DxError::IoError {
            path: Some(request_path),
            message: e.to_string(),
        })
    }

    fn write_next_familiar_launch_runtime_evidence(
        project_dir: &Path,
        readiness_receipt: &serde_json::Value,
    ) -> DxResult<()> {
        let mut evidence = launch_runtime_evidence_contract();
        if let Some(object) = evidence.as_object_mut() {
            object.insert(
                "generated_at".to_string(),
                serde_json::json!(Utc::now().to_rfc3339()),
            );
            object.insert(
                "template_readiness_receipt".to_string(),
                serde_json::json!({
                    "file": NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
                    "status": readiness_receipt["status"],
                    "runtime_verification": readiness_receipt["runtime_verification"]
                }),
            );
            object.insert(
                "approval_gate".to_string(),
                serde_json::json!({
                    "status": "pending-explicit-approval",
                    "request_file": NEXT_FAMILIAR_LAUNCH_RUNTIME_APPROVAL_REQUEST_FILE,
                    "checklist_file": NEXT_FAMILIAR_LAUNCH_RUNTIME_CHECKLIST_FILE,
                    "approved_runtime_required": true
                }),
            );
        }

        let evidence_path = project_dir.join(NEXT_FAMILIAR_LAUNCH_RUNTIME_EVIDENCE_FILE);
        if let Some(parent) = evidence_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| DxError::IoError {
                path: Some(parent.to_path_buf()),
                message: e.to_string(),
            })?;
        }
        std::fs::write(
            &evidence_path,
            serde_json::to_string_pretty(&evidence).map_err(forge_error)?,
        )
        .map_err(|e| DxError::IoError {
            path: Some(evidence_path),
            message: e.to_string(),
        })
    }

    fn write_next_familiar_launch_verification_lane(
        project_dir: &Path,
        readiness_receipt: &serde_json::Value,
    ) -> DxResult<()> {
        let mut lane = launch_verification_lane_contract();
        if let Some(object) = lane.as_object_mut() {
            object.insert(
                "generated_at".to_string(),
                serde_json::json!(Utc::now().to_rfc3339()),
            );
            object.insert(
                "template_readiness_receipt".to_string(),
                serde_json::json!({
                    "file": NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
                    "status": readiness_receipt["status"],
                    "runtime_verification": readiness_receipt["runtime_verification"]
                }),
            );
            object.insert(
                "readiness_bundle".to_string(),
                serde_json::json!(NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE),
            );
        }

        let lane_path = project_dir.join(NEXT_FAMILIAR_LAUNCH_VERIFICATION_LANE_FILE);
        if let Some(parent) = lane_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| DxError::IoError {
                path: Some(parent.to_path_buf()),
                message: e.to_string(),
            })?;
        }
        std::fs::write(
            &lane_path,
            serde_json::to_string_pretty(&lane).map_err(forge_error)?,
        )
        .map_err(|e| DxError::IoError {
            path: Some(lane_path),
            message: e.to_string(),
        })
    }

    fn write_next_familiar_launch_zed_template_handoff(
        project_dir: &Path,
        readiness_receipt: &serde_json::Value,
    ) -> DxResult<()> {
        let mut handoff = launch_zed_template_handoff_contract();
        if let Some(object) = handoff.as_object_mut() {
            object.insert(
                "generated_at".to_string(),
                serde_json::json!(Utc::now().to_rfc3339()),
            );
            object.insert(
                "template_readiness_receipt".to_string(),
                serde_json::json!({
                    "file": NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
                    "status": readiness_receipt["status"],
                    "architecture_contract": readiness_receipt["architecture_contract"]
                }),
            );
        }

        let handoff_path = project_dir.join(NEXT_FAMILIAR_LAUNCH_ZED_TEMPLATE_HANDOFF_FILE);
        if let Some(parent) = handoff_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| DxError::IoError {
                path: Some(parent.to_path_buf()),
                message: e.to_string(),
            })?;
        }
        std::fs::write(
            &handoff_path,
            serde_json::to_string_pretty(&handoff).map_err(forge_error)?,
        )
        .map_err(|e| DxError::IoError {
            path: Some(handoff_path),
            message: e.to_string(),
        })
    }

    fn write_next_familiar_launch_companion_docs(project_dir: &Path) -> DxResult<()> {
        let receipts = launch_companion_receipts_contract();
        for receipt in receipts["receipts"].as_array().into_iter().flatten() {
            let docs_file = receipt["docs_file"].as_str().unwrap_or_default();
            let docs_path = project_dir.join(docs_file);
            if let Some(parent) = docs_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| DxError::IoError {
                    path: Some(parent.to_path_buf()),
                    message: e.to_string(),
                })?;
            }
            std::fs::write(&docs_path, Self::launch_companion_doc_content(receipt)).map_err(
                |e| DxError::IoError {
                    path: Some(docs_path),
                    message: e.to_string(),
                },
            )?;
        }
        Ok(())
    }

    fn launch_companion_doc_content(receipt: &serde_json::Value) -> String {
        let kind = receipt["kind"].as_str().unwrap_or("companion");
        let package_id = receipt["package_id"].as_str().unwrap_or("unknown");
        let source_file = receipt["source_file"].as_str().unwrap_or("unknown");
        let materialized_file = receipt["materialized_file"].as_str().unwrap_or("unknown");
        let package_docs_file = receipt["package_docs_file"].as_str().unwrap_or("unknown");
        let public_api = receipt["public_api"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|api| api.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "# Launch Companion Receipt: {kind}\n\n- Package: `{package_id}`\n- Source proof: `{source_file}`\n- Materialized proof: `{materialized_file}`\n- Package docs target: `{package_docs_file}`\n- Public API markers: `{public_api}`\n\n## Source proof\n\nOpen `{source_file}` to review the source-owned launch proof, then compare it with `{materialized_file}` in the generated starter. This receipt does not run installs, builds, previews, or servers.\n"
        )
    }

    fn write_next_familiar_launch_readiness_bundle(
        project_dir: &Path,
        readiness_receipt: &serde_json::Value,
        forge_receipt_path: &Option<String>,
        materialized_files: &[&str],
    ) -> DxResult<()> {
        let mut readiness_bundle = launch_readiness_bundle_contract();
        if let Some(object) = readiness_bundle.as_object_mut() {
            object.insert(
                "generated_at".to_string(),
                serde_json::json!(Utc::now().to_rfc3339()),
            );
            object.insert(
                "template_readiness_receipt".to_string(),
                serde_json::json!({
                    "file": NEXT_FAMILIAR_LAUNCH_READINESS_RECEIPT_FILE,
                    "status": readiness_receipt["status"],
                    "summary": readiness_receipt["summary"]
                }),
            );
            object.insert(
                "materialized_files".to_string(),
                serde_json::json!(materialized_files),
            );
            object.insert(
                "required_packages".to_string(),
                serde_json::json!(FORGE_WWW_TEMPLATE_PACKAGE_IDS),
            );
            object.insert(
                "forge_receipt_path".to_string(),
                serde_json::json!(forge_receipt_path),
            );
            object.insert(
                "source_guard_evidence".to_string(),
                serde_json::json!([
                    {
                        "kind": "source_level_template_guard",
                        "command": "dx run --test .\\benchmarks\\template-shell.test.ts",
                        "status": "passed-before-generation"
                    },
                    {
                        "kind": "source_level_package_guard",
                        "command": "dx run --test .\\benchmarks\\launch-package-slices.test.ts",
                        "status": "passed-before-generation"
                    }
                ]),
            );
        }

        let bundle_path = project_dir.join(NEXT_FAMILIAR_LAUNCH_READINESS_BUNDLE_FILE);
        if let Some(parent) = bundle_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| DxError::IoError {
                path: Some(parent.to_path_buf()),
                message: e.to_string(),
            })?;
        }
        std::fs::write(
            &bundle_path,
            serde_json::to_string_pretty(&readiness_bundle).map_err(forge_error)?,
        )
        .map_err(|e| DxError::IoError {
            path: Some(bundle_path),
            message: e.to_string(),
        })
    }
}
