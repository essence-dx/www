impl Cli {
    /// Create a CLI command runner for the current working directory.
    pub fn new() -> Self {
        Self {
            cwd: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    /// Create with a specific working directory.
    pub fn with_cwd(cwd: PathBuf) -> Self {
        Self { cwd }
    }

    /// Run the CLI.
    ///
    /// Parses command-line arguments and executes the appropriate command.
    pub fn run() -> DxResult<()> {
        let args: Vec<String> = std::env::args().collect();

        if args.len() < 2 {
            print_help();
            return Ok(());
        }

        let cli = Cli::new();

        match args[1].as_str() {
            "new" | "create" => {
                if args.len() < 3 {
                    eprintln!("Error: Project name required");
                    eprintln!("Usage: dx new <name>");
                    return Err(DxError::ConfigValidationError {
                        message: "Project name required".to_string(),
                        field: Some("name".to_string()),
                    });
                }
                if is_help_arg(args.get(2)) {
                    print_new_help();
                    return Ok(());
                }
                if args[2].starts_with('-') {
                    eprintln!("Error: Unknown option for dx new: {}", args[2]);
                    print_new_help();
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown option for dx new: {}", args[2]),
                        field: Some("name".to_string()),
                    });
                }
                cli.cmd_new(&args[2])?;
            }
            "dev" => {
                if is_help_arg(args.get(2)) {
                    print_dev_help();
                    return Ok(());
                }
                cli.cmd_dev(&args[2..])?;
            }
            "preview" => {
                cli.cmd_preview(&args[2..])?;
            }
            "promote" => {
                cli.cmd_promote(&args[2..])?;
            }
            "rollback" => {
                cli.cmd_rollback(&args[2..])?;
            }
            "build" => {
                run_build_command(&args[2..], "dx build", |options| {
                    cli.cmd_build_with_options(options, "dx build")
                })?;
            }
            "run" => {
                run_dx_script(&cli.cwd, &args[2..])?;
            }
            "style" => {
                if is_help_arg(args.get(2)) {
                    print_style_help();
                    return Ok(());
                }
                cli.cmd_style(&args[2..])?;
            }
            "icons" => {
                if is_help_arg(args.get(2)) {
                    print_icons_help();
                    return Ok(());
                }
                cli.cmd_icons(&args[2..])?;
            }
            "imports" => {
                if is_help_arg(args.get(2)) {
                    print_imports_help();
                    return Ok(());
                }
                cli.cmd_imports(&args[2..])?;
            }
            "env" => {
                env_firewall::cmd_env(&cli.cwd, &args[2..])?;
            }
            "explain" => {
                cli.cmd_explain(&args[2..])?;
            }
            "doctor" => {
                cli.cmd_doctor(&args[2..])?;
            }
            "export" => {
                cli.cmd_export(&args[2..])?;
            }
            "deploy" => {
                cli.cmd_deploy(&args[2..])?;
            }
            "serializer" => {
                if args.len() < 3 || is_help_arg(args.get(2)) {
                    print_serializer_help();
                    if args.len() < 3 {
                        return Err(DxError::ConfigValidationError {
                            message: "serializer source path required".to_string(),
                            field: Some("serializer".to_string()),
                        });
                    }
                    return Ok(());
                }
                cli.cmd_serializer(&args[2..])?;
            }
            "generate" | "g" => {
                if args.len() < 4 {
                    eprintln!("Error: Type and name required");
                    eprintln!("Usage: dx generate <type> <name>");
                    return Err(DxError::ConfigValidationError {
                        message: "Type and name required".to_string(),
                        field: Some("type/name".to_string()),
                    });
                }
                cli.cmd_generate(&args[2], &args[3])?;
            }
            "add" => {
                if args.len() < 3 {
                    eprintln!("Error: Component name required");
                    eprintln!("Usage: dx add <component>");
                    eprintln!("       dx add --all");
                    eprintln!("       dx add --list");
                    return Err(DxError::ConfigValidationError {
                        message: "Component name required".to_string(),
                        field: Some("component".to_string()),
                    });
                }
                let components: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();
                cli.cmd_add(&components)?;
            }
            "update" => {
                cli.cmd_update(&args[2..])?;
            }
            "forge" => {
                cli.cmd_forge(&args[2..])?;
            }
            "templates" | "list-templates" => {
                cli.cmd_templates(&args[2..])?;
            }
            "agent-context" => {
                agent_context::cmd_agent_context(&cli.cwd, &args[2..])?;
            }
            "routes" => {
                cli.cmd_routes(&args[2..])?;
            }
            "preview-manifest" | "studio-manifest" => {
                cli.cmd_preview_manifest(&args[2..])?;
            }
            "www" => {
                cli.cmd_www(&args[2..])?;
            }
            "next-rust" => {
                cli.cmd_www_next_rust(&args[2..])?;
            }
            "migrate" => {
                cli.cmd_migrate(&args[2..])?;
            }
            "prove" => {
                cli.cmd_prove(&args[2..])?;
            }
            "check" => {
                cli.cmd_check(&args[2..])?;
            }
            "help" | "--help" | "-h" => {
                print_help();
            }
            "version" | "--version" | "-v" => {
                Self::print_version();
            }
            cmd => {
                eprintln!("Unknown command: {}", cmd);
                print_help();
                return Err(DxError::ConfigValidationError {
                    message: format!("Unknown command: {}", cmd),
                    field: None,
                });
            }
        }

        Ok(())
    }

    /// Generate or verify dx-style CSS for the public TSX launch path.
    pub fn cmd_style(&self, args: &[String]) -> DxResult<()> {
        let report = run_dx_style(&self.cwd, args).map_err(forge_error)?;
        print_public_tool_report(report).map_err(forge_error)
    }

    /// Generate or verify source-owned icon wrappers for static TSX icon tags.
    pub fn cmd_icons(&self, args: &[String]) -> DxResult<()> {
        let report = run_dx_icons(&self.cwd, args).map_err(forge_error)?;
        print_public_tool_report(report).map_err(forge_error)
    }

    /// Generate or verify readable auto-import maps.
    pub fn cmd_imports(&self, args: &[String]) -> DxResult<()> {
        let report = run_dx_imports(&self.cwd, args).map_err(forge_error)?;
        print_public_tool_report(report).map_err(forge_error)
    }

    /// Explain a TSX route and write AI/Zed-readable route contracts.
    pub fn cmd_explain(&self, args: &[String]) -> DxResult<()> {
        let report = run_dx_explain(&self.cwd, args).map_err(forge_error)?;
        print_public_tool_report(report).map_err(forge_error)
    }

    /// Run the source-only DX launch doctor.
    pub fn cmd_doctor(&self, args: &[String]) -> DxResult<()> {
        let report = run_dx_doctor(&self.cwd, args).map_err(forge_error)?;
        print_public_tool_report(report).map_err(forge_error)
    }

    /// Analyze static export output without running a heavy build.
    pub fn cmd_export(&self, args: &[String]) -> DxResult<()> {
        let report = run_dx_export_analyze(&self.cwd, args).map_err(forge_error)?;
        print_public_tool_report(report).map_err(forge_error)
    }

    /// Prepare a static Vercel deploy manifest for DX-WWW.
    pub fn cmd_deploy(&self, args: &[String]) -> DxResult<()> {
        let report = run_dx_deploy(&self.cwd, args).map_err(forge_error)?;
        print_public_tool_report(report).map_err(forge_error)
    }

    fn cmd_templates(&self, args: &[String]) -> DxResult<()> {
        templates_command::cmd_templates(&self.cwd, args)
    }
    fn cmd_forge_launch_readiness_bundle(&self, args: &[String]) -> DxResult<()> {
        let options: DxLaunchReportCommandOptions =
            parse_launch_report_options(&self.cwd, args, "launch-readiness-bundle", 100)?;
        let project = options.project;
        let output = options.output;
        let format = options.format;
        let fail_under = options.fail_under;
        let quiet = options.quiet;

        let report =
            build_launch_readiness_bundle_report(&project, fail_under).map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => launch_readiness_bundle_markdown(&report),
            DxOutputFormat::Terminal => launch_readiness_bundle_terminal(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
            if !quiet {
                println!("{}", output.display());
            }
        } else if !quiet {
            println!("{rendered}");
        }

        if !report.passed() {
            return Err(DxError::ConfigValidationError {
                message: launch_readiness_bundle_failure_summary(&report),
                field: Some("forge.launch-readiness-bundle".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_forge_launch_adoption_report(&self, args: &[String]) -> DxResult<()> {
        let options: DxLaunchReportCommandOptions =
            parse_launch_report_options(&self.cwd, args, "launch-adoption-report", 100)?;
        let project = options.project;
        let output = options.output;
        let format = options.format;
        let fail_under = options.fail_under;
        let quiet = options.quiet;

        let report = launch_adoption_report::build_launch_adoption_report(&project, fail_under)
            .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => {
                launch_adoption_report::launch_adoption_report_markdown(&report)
            }
            DxOutputFormat::Terminal => {
                launch_adoption_report::launch_adoption_report_terminal(&report)
            }
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
            if !quiet {
                println!("{}", output.display());
            }
        } else if !quiet {
            println!("{rendered}");
        }

        if !report.passed() {
            return Err(DxError::ConfigValidationError {
                message: launch_adoption_report::launch_adoption_report_failure_summary(&report),
                field: Some("forge.launch-adoption-report".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_forge_launch_manifest_drift(&self, args: &[String]) -> DxResult<()> {
        let options: DxLaunchReportCommandOptions =
            parse_launch_report_options(&self.cwd, args, "launch-manifest-drift", 100)?;
        let project = options.project;
        let output = options.output;
        let format = options.format;
        let fail_under = options.fail_under;
        let quiet = options.quiet;

        let source_template = launch_manifest_drift_source_contract();
        let report = launch_manifest_drift::build_launch_manifest_drift_report(
            &project,
            fail_under,
            &source_template,
        )
        .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => {
                launch_manifest_drift::launch_manifest_drift_markdown(&report)
            }
            DxOutputFormat::Terminal => {
                launch_manifest_drift::launch_manifest_drift_terminal(&report)
            }
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
            if !quiet {
                println!("{}", output.display());
            }
        } else if !quiet {
            println!("{rendered}");
        }

        if !report.passed() {
            return Err(DxError::ConfigValidationError {
                message: launch_manifest_drift::launch_manifest_drift_failure_summary(&report),
                field: Some("forge.launch-manifest-drift".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_forge_launch_companion_receipts(&self, args: &[String]) -> DxResult<()> {
        let options: DxLaunchReportCommandOptions =
            parse_launch_report_options(&self.cwd, args, "launch-companion-receipts", 100)?;
        let project = options.project;
        let output = options.output;
        let format = options.format;
        let fail_under = options.fail_under;
        let quiet = options.quiet;

        let report =
            launch_companion_receipts::build_launch_companion_receipts_report(&project, fail_under)
                .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => {
                launch_companion_receipts::launch_companion_receipts_markdown(&report)
            }
            DxOutputFormat::Terminal => {
                launch_companion_receipts::launch_companion_receipts_terminal(&report)
            }
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
            if !quiet {
                println!("{}", output.display());
            }
        } else if !quiet {
            println!("{rendered}");
        }

        if !report.passed() {
            return Err(DxError::ConfigValidationError {
                message: launch_companion_receipts::launch_companion_receipts_failure_summary(
                    &report,
                ),
                field: Some("forge.launch-companion-receipts".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_forge_launch_runtime_checklist(&self, args: &[String]) -> DxResult<()> {
        let options: DxLaunchReportCommandOptions =
            parse_launch_report_options(&self.cwd, args, "launch-runtime-checklist", 100)?;
        let project = options.project;
        let output = options.output;
        let format = options.format;
        let fail_under = options.fail_under;
        let quiet = options.quiet;

        let report =
            launch_runtime_checklist::build_launch_runtime_checklist_report(&project, fail_under)
                .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => {
                launch_runtime_checklist::launch_runtime_checklist_markdown(&report)
            }
            DxOutputFormat::Terminal => {
                launch_runtime_checklist::launch_runtime_checklist_terminal(&report)
            }
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
            if !quiet {
                println!("{}", output.display());
            }
        } else if !quiet {
            println!("{rendered}");
        }

        if !report.passed() {
            return Err(DxError::ConfigValidationError {
                message: launch_runtime_checklist::launch_runtime_checklist_failure_summary(
                    &report,
                ),
                field: Some("forge.launch-runtime-checklist".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_routes(&self, args: &[String]) -> DxResult<()> {
        studio_command::cmd_routes(&self.cwd, args)
    }

    fn cmd_preview_manifest(&self, args: &[String]) -> DxResult<()> {
        studio_command::cmd_preview_manifest(&self.cwd, args)
    }

    fn cmd_www(&self, args: &[String]) -> DxResult<()> {
        match args.first().map(String::as_str) {
            Some("new") | Some("create") => {
                let name = args.get(1).ok_or_else(|| DxError::ConfigValidationError {
                    message: "Project name required".to_string(),
                    field: Some("www new".to_string()),
                })?;
                self.cmd_new(name)
            }
            Some("dev") => self.cmd_dev(&args[1..]),
            Some("build") => {
                run_build_command(&args[1..], "dx www build", |options| {
                    self.cmd_build_with_options(options, "dx www build")
                })
            }
            Some("check") => self.cmd_check(&args[1..]),
            Some("routes") => self.cmd_routes(&args[1..]),
            Some("preview-manifest") | Some("studio-manifest") => {
                self.cmd_preview_manifest(&args[1..])
            }
            Some("templates") | Some("list-templates") => self.cmd_templates(&args[1..]),
            Some("native-shell") | Some("native") => cmd_www_native_shell(&self.cwd, &args[1..]),
            Some("agent-context") => agent_context::cmd_agent_context(&self.cwd, &args[1..]),
            Some("docs-doctor") => docs_doctor::cmd_docs_doctor(&self.cwd, &args[1..]),
            Some("readiness") => readiness::cmd_readiness(&self.cwd, &args[1..]),
            Some("next-rust") | Some("vendor") => self.cmd_www_next_rust(&args[1..]),
            Some("help") | Some("--help") | Some("-h") | None => {
                print_www_help();
                Ok(())
            }
            Some(command) => Err(DxError::ConfigValidationError {
                message: format!("Unknown www command: {command}"),
                field: Some("www".to_string()),
            }),
        }
    }

    fn cmd_www_next_rust(&self, args: &[String]) -> DxResult<()> {
        next_rust_status::cmd_www_next_rust(args)
    }

    /// Print version.
    fn print_version() {
        eprintln!("dx-www {}", env!("CARGO_PKG_VERSION"));
    }

    /// Preview source-owned package updates.
    pub fn cmd_update(&self, args: &[String]) -> DxResult<()> {
        cmd_update(&self.cwd, args)
    }

    /// Plan a source-owned migration from another framework into DX-WWW.
    pub fn cmd_migrate(&self, args: &[String]) -> DxResult<()> {
        cmd_migrate(&self.cwd, args)
    }

    /// Create a new project.
    pub fn cmd_new(&self, name: &str) -> DxResult<()> {
        new_command::cmd_new(&self.cwd, name)
    }

    /// Start development server.
    pub fn cmd_dev(&self, args: &[String]) -> DxResult<()> {
        dev_command::cmd_dev(
            &self.cwd,
            args,
            || self.load_translations(),
            Self::handle_parsed_http_response,
        )
    }

    /// Serve production build output through the deploy-adapter contract.
    pub fn cmd_preview(&self, args: &[String]) -> DxResult<()> {
        preview_command::cmd_preview(
            &self.cwd,
            args,
            server_action_runtime::execute_production_contract_server_action,
        )
    }

    /// Sign and verify a production build manifest before hosted release.
    pub fn cmd_promote(&self, args: &[String]) -> DxResult<()> {
        cmd_promote(&self.cwd, args)
    }

    /// Verify that a current build can roll back to a previous immutable build.
    pub fn cmd_rollback(&self, args: &[String]) -> DxResult<()> {
        cmd_rollback(&self.cwd, args)
    }

    /// Handle HTTP request
    #[cfg(test)]
    fn handle_request(
        cwd: &PathBuf,
        path: &str,
        translations: &HashMap<String, String>,
    ) -> (String, String, String) {
        Self::handle_parsed_request(
            cwd,
            &DxCliHttpRequest {
                method: "GET".to_string(),
                path: path.to_string(),
                headers: BTreeMap::new(),
                body: serde_json::Value::Null,
            },
            translations,
        )
    }

    fn route_handler_http_response(
        cwd: &Path,
        request: &DxCliHttpRequest,
    ) -> Option<DxCliHttpResponse> {
        let path = request.path.as_str();
        let route_handler_match = Self::app_api_route_handler_match(cwd, path)?;
        if !route_handler_match.path.exists() {
            return None;
        }
        Some(
            match Self::execute_app_route_handler(cwd, request, &route_handler_match) {
                Ok(response) => response,
                Err(error) if Self::is_route_handler_boundary_error(&error) => {
                    Self::route_handler_boundary_response(cwd, request, &route_handler_match, error)
                }
                Err(error) => DxCliHttpResponse {
                    status: "500 Internal Server Error".to_string(),
                    content_type: "application/json; charset=utf-8".to_string(),
                    headers: BTreeMap::new(),
                    body: serde_json::json!({
                        "error": "route-handler-failed",
                        "message": error
                    })
                    .to_string(),
                },
            },
        )
    }

    fn is_route_handler_boundary_error(error: &str) -> bool {
        error.contains("route handler must return an object literal")
            || error.contains("unsupported route handler literal")
            || error.contains("json response helper body must be an object literal")
            || error.contains("unsupported route handler request body reader")
            || error.contains("unsupported redirect URL expression")
    }

    fn route_handler_boundary_response(
        cwd: &Path,
        request: &DxCliHttpRequest,
        route_handler_match: &app_api_routes::AppApiRouteMatch,
        message: String,
    ) -> DxCliHttpResponse {
        let source_path = Self::relative_cli_path(cwd, &route_handler_match.path);
        DxCliHttpResponse {
            status: "501 Not Implemented".to_string(),
            content_type: "application/json; charset=utf-8".to_string(),
            headers: BTreeMap::from([
                (
                    "x-dx-route-handler-receipt".to_string(),
                    APP_ROUTE_HANDLER_RECEIPT_SCHEMA.to_string(),
                ),
                ("x-dx-node-modules-required".to_string(), "false".to_string()),
                (
                    "x-dx-route-handler-source-owned".to_string(),
                    "true".to_string(),
                ),
                (
                    "x-dx-external-runtime-required".to_string(),
                    "false".to_string(),
                ),
                (
                    "x-dx-external-runtime-executed".to_string(),
                    "false".to_string(),
                ),
            ]),
            body: serde_json::json!({
                "ok": false,
                "status": "route-handler-boundary",
                "method": request.method,
                "path": request.path,
                "sourcePath": source_path,
                "message": message,
                "next": "Materialize a source-owned literal response or add a dedicated Forge interpreter before claiming broad route-handler runtime coverage.",
                "lifecycleScriptsExecuted": false,
            })
            .to_string(),
        }
    }

    fn handle_parsed_request(
        cwd: &PathBuf,
        request: &DxCliHttpRequest,
        translations: &HashMap<String, String>,
    ) -> (String, String, String) {
        let lookup_path = Self::dev_lookup_path(&request.path);
        let path = lookup_path.as_str();
        if Self::is_server_action_request(path) {
            return match Self::execute_project_server_action_request(cwd, request) {
                Ok(body) => (
                    "200 OK".to_string(),
                    "application/json; charset=utf-8".to_string(),
                    body,
                ),
                Err(error) => (
                    server_action_runtime::server_action_error_status(&error).to_string(),
                    "application/json; charset=utf-8".to_string(),
                    serde_json::json!({
                        "error": "server-action-failed",
                        "message": server_action_runtime::server_action_redacted_error(&error)
                    })
                    .to_string(),
                ),
            };
        }

        if path.starts_with("/_dx/styles/") {
            return match app_router_style_assets::render_generated_style_asset(cwd, path) {
                Ok(css) => (
                    "200 OK".to_string(),
                    "text/css; charset=utf-8".to_string(),
                    css,
                ),
                Err(error) => (
                    "404 Not Found".to_string(),
                    "application/json; charset=utf-8".to_string(),
                    error.to_response_body(),
                ),
            };
        }

        // Serve static files
        if path.starts_with("/styles/")
            || path.starts_with("/public/")
            || Self::public_root_asset_path(cwd, path).is_some()
        {
            let file_path = if path == "/favicon.svg" {
                cwd.join("public/favicon.svg")
            } else if let Some(file_path) = Self::public_root_asset_path(cwd, path) {
                file_path
            } else {
                cwd.join(path.trim_start_matches('/'))
            };
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                let content_type = if path.ends_with(".css") {
                    "text/css; charset=utf-8"
                } else if path.ends_with(".js") || path.ends_with(".ts") || path.ends_with(".tsx") {
                    "application/javascript; charset=utf-8"
                } else if path.ends_with(".json") {
                    "application/json; charset=utf-8"
                } else if path.ends_with(".svg") {
                    "image/svg+xml"
                } else if path.ends_with(".png") {
                    "image/png"
                } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
                    "image/jpeg"
                } else if path.ends_with(".woff2") {
                    "font/woff2"
                } else if path.ends_with(".woff") {
                    "font/woff"
                } else {
                    "text/plain; charset=utf-8"
                };
                return ("200 OK".to_string(), content_type.to_string(), content);
            }
        }

        if Self::is_project_contract_hint_request(path) {
            return match Self::render_project_contract_hints(cwd) {
                Ok(body) => (
                    "200 OK".to_string(),
                    "application/json; charset=utf-8".to_string(),
                    body,
                ),
                Err(error) => (
                    "500 Internal Server Error".to_string(),
                    "application/json; charset=utf-8".to_string(),
                    serde_json::json!({
                        "error": "project-contract-hints-failed",
                        "message": error
                    })
                    .to_string(),
                ),
            };
        }

        if let Some(response) = Self::route_handler_http_response(cwd, request) {
            return (response.status, response.content_type, response.body);
        }

        if let Some(app_route_match) = app_page_routes::route_match(cwd, &request.path) {
            if app_route_match.path.exists() {
                match app_router_runtime_command::render_app_route(cwd, &app_route_match) {
                    Ok(html) => {
                        return (
                            "200 OK".to_string(),
                            "text/html; charset=utf-8".to_string(),
                            html,
                        );
                    }
                    Err(e) => {
                        return (
                            "500 Internal Server Error".to_string(),
                            "text/html; charset=utf-8".to_string(),
                            Self::render_error_page("500", &format!("Render Error: {}", e)),
                        );
                    }
                }
            }
        }

        // Render page
        let page_name = if path == "/" {
            "index"
        } else {
            path.trim_start_matches('/').trim_end_matches('/')
        };
        let page_path = cwd.join("pages").join(format!("{}.html", page_name));

        if page_path.exists() {
            match Self::render_page(cwd, &page_path, translations) {
                Ok(html) => (
                    "200 OK".to_string(),
                    "text/html; charset=utf-8".to_string(),
                    html,
                ),
                Err(e) => (
                    "500 Internal Server Error".to_string(),
                    "text/html; charset=utf-8".to_string(),
                    Self::render_error_page("500", &format!("Render Error: {}", e)),
                ),
            }
        } else {
            (
                "404 Not Found".to_string(),
                "text/html; charset=utf-8".to_string(),
                Self::render_error_page("404", &format!("Page not found: {}", path)),
            )
        }
    }

    fn public_root_asset_path(cwd: &Path, path: &str) -> Option<PathBuf> {
        let relative = path.strip_prefix('/')?;
        if relative.is_empty()
            || relative.contains('/')
            || relative.contains('\\')
            || relative.contains("..")
        {
            return None;
        }

        let asset_path = cwd.join("public").join(relative);
        asset_path.is_file().then_some(asset_path)
    }

    fn is_project_contract_hint_request(path: &str) -> bool {
        matches!(
            path.trim_end_matches('/'),
            "/.dx/project-contract-hints.json"
                | "/.dx/dev/project-contract-hints.json"
                | "/.dx/lsp/project-contract-hints.json"
        )
    }

    fn render_project_contract_hints(cwd: &Path) -> Result<String, String> {
        let report = check_dx_project_with_options(
            cwd,
            DxCheckOptions {
                project_contract: true,
            },
        )
        .map_err(|error| format!("Failed to check project contract: {error}"))?;
        let artifact = build_project_contract_hint_artifact(cwd, &report);
        serde_json::to_string_pretty(&artifact)
            .map_err(|error| format!("Failed to serialize project contract hints: {error}"))
    }

    fn app_api_route_handler_match(
        cwd: &Path,
        path: &str,
    ) -> Option<app_api_routes::AppApiRouteMatch> {
        app_api_routes::route_handler_match(cwd, path)
    }

    fn execute_app_route_handler(
        cwd: &Path,
        request: &DxCliHttpRequest,
        route_handler_match: &app_api_routes::AppApiRouteMatch,
    ) -> Result<DxCliHttpResponse, String> {
        let source = std::fs::read_to_string(&route_handler_match.path)
            .map_err(|error| format!("Failed to read route handler: {error}"))?;
        let request_path = request
            .path
            .as_str()
            .split('?')
            .next()
            .unwrap_or(request.path.as_str())
            .trim_end_matches('/')
            .to_string();
        let request_path = if request_path.is_empty() {
            "/".to_string()
        } else {
            request_path
        };
        if let Some(response) = Self::try_metasearch_route_handler_response(
            cwd,
            request,
            route_handler_match,
            &source,
            &request_path,
        )? {
            return Ok(response);
        }
        let source_path = Self::relative_cli_path(cwd, &route_handler_match.path);
        let response = execute_react_route_handler(
            &DxReactServerSource {
                kind: DxReactServerSourceKind::RouteHandler,
                source_path: source_path.clone(),
                source,
            },
            DxReactRouteHandlerRequest {
                method: request.method.clone(),
                path: request_path.clone(),
                headers: request.headers.clone(),
                body: request.body.clone(),
                route_params: route_handler_match.params.clone(),
                search_params: route_handler_match.search_params.clone(),
                runtime_env: route_handler_runtime_env::route_handler_runtime_env(),
            },
        )?;
        let receipt = build_app_route_handler_receipt(DxAppRouteHandlerReceiptInput {
            source_path: &source_path,
            method: &request.method,
            request_path: &request_path,
            route_params: &route_handler_match.params,
            search_params: &route_handler_match.search_params,
            status: response.status,
            content_type: &response.content_type,
            response_headers: &response.headers,
            execution_model: &response.execution_model,
            lifecycle_scripts_executed: response.lifecycle_scripts_executed,
            node_modules_present: cwd.join("node_modules").exists(),
        });
        let mut headers = response.headers.clone();
        headers.extend(app_route_handler_receipt_headers(&receipt));
        let body = serde_json::to_string(&response.body)
            .map_err(|error| format!("Failed to serialize route response: {error}"))?;
        Ok(DxCliHttpResponse {
            status: Self::http_status_line(response.status),
            content_type: response.content_type,
            headers,
            body,
        })
    }

    fn try_metasearch_route_handler_response(
        cwd: &Path,
        request: &DxCliHttpRequest,
        route_handler_match: &app_api_routes::AppApiRouteMatch,
        source: &str,
        request_path: &str,
    ) -> Result<Option<DxCliHttpResponse>, String> {
        if request_path != "/api/v1/search"
            || !source.contains("createDxMetasearchSearchResponse")
        {
            return Ok(None);
        }

        if request.method != "GET" {
            return Ok(Some(Self::metasearch_json_response(
                405,
                serde_json::json!({
                    "error": "method_not_allowed",
                    "message": "DX Metasearch search accepts GET requests.",
                }),
            )));
        }

        let Some(query) = route_handler_match
            .search_params
            .get("q")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        else {
            return Ok(Some(Self::metasearch_json_response(
                400,
                serde_json::json!({
                    "error": "missing_query",
                    "message": "The `q` query parameter is required.",
                }),
            )));
        };

        if let Some(format) = route_handler_match.search_params.get("format") {
            if !format.eq_ignore_ascii_case("json") {
                return Ok(Some(Self::metasearch_json_response(
                    400,
                    serde_json::json!({
                        "error": "unsupported_format",
                        "message": "Only `format=json` is supported on the JSON API.",
                        "received": format,
                    }),
                )));
            }
        }

        let Some(root) = Self::metasearch_project_root(cwd) else {
            return Ok(Some(Self::metasearch_json_response(
                503,
                serde_json::json!({
                    "error": "metasearch_project_not_found",
                    "message": "DX WWW could not locate the Metasearch project root for this route.",
                }),
            )));
        };
        let Some(binary) = Self::metasearch_binary_path(&root) else {
            return Ok(Some(Self::metasearch_json_response(
                503,
                serde_json::json!({
                    "error": "metasearch_binary_missing",
                    "message": "Build the metasearch CLI before starting the DX WWW search route.",
                }),
            )));
        };

        let mut command = std::process::Command::new(binary);
        command
            .current_dir(&root)
            .arg("search")
            .arg("--allow-network")
            .arg("--format")
            .arg("json")
            .arg("--query")
            .arg(query)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        Self::append_optional_metasearch_arg(
            &mut command,
            "--categories",
            route_handler_match
                .search_params
                .get("categories")
                .or_else(|| route_handler_match.search_params.get("category")),
        );
        Self::append_optional_metasearch_arg(
            &mut command,
            "--language",
            route_handler_match.search_params.get("language"),
        );
        Self::append_optional_metasearch_arg(
            &mut command,
            "--page",
            route_handler_match.search_params.get("page"),
        );
        Self::append_optional_metasearch_arg(
            &mut command,
            "--safe-search",
            route_handler_match.search_params.get("safe_search"),
        );
        Self::append_optional_metasearch_arg(
            &mut command,
            "--time-range",
            route_handler_match.search_params.get("time_range"),
        );
        Self::append_optional_metasearch_arg(
            &mut command,
            "--engines",
            route_handler_match.search_params.get("engines"),
        );

        let output = command
            .output()
            .map_err(|error| format!("Failed to execute metasearch CLI: {error}"))?;
        if !output.status.success() {
            return Ok(Some(Self::metasearch_json_response(
                502,
                serde_json::json!({
                    "error": "metasearch_search_failed",
                    "message": Self::short_process_output(&output.stderr),
                }),
            )));
        }

        let stdout = String::from_utf8(output.stdout)
            .map_err(|error| format!("Metasearch CLI returned non-UTF8 JSON: {error}"))?;
        let payload: serde_json::Value = serde_json::from_str(stdout.trim())
            .map_err(|error| format!("Metasearch CLI returned invalid JSON: {error}"))?;

        Ok(Some(Self::metasearch_json_response(200, payload)))
    }

    fn append_optional_metasearch_arg(
        command: &mut std::process::Command,
        flag: &str,
        value: Option<&String>,
    ) {
        if let Some(value) = value.map(|value| value.trim()).filter(|value| !value.is_empty()) {
            command.arg(flag).arg(value);
        }
    }

    fn metasearch_json_response(status: u16, body: serde_json::Value) -> DxCliHttpResponse {
        DxCliHttpResponse {
            status: Self::http_status_line(status),
            content_type: "application/json; charset=utf-8".to_string(),
            headers: BTreeMap::from([("cache-control".to_string(), "no-store".to_string())]),
            body: body.to_string(),
        }
    }

    fn metasearch_project_root(cwd: &Path) -> Option<PathBuf> {
        cwd.ancestors().find_map(|candidate| {
            candidate
                .join("crates/metasearch-cli/Cargo.toml")
                .is_file()
                .then(|| candidate.to_path_buf())
        })
    }

    fn metasearch_binary_path(root: &Path) -> Option<PathBuf> {
        let binary_name = if cfg!(windows) {
            "metasearch.exe"
        } else {
            "metasearch"
        };
        ["debug", "release"]
            .into_iter()
            .map(|profile| root.join("target").join(profile).join(binary_name))
            .find(|candidate| candidate.is_file())
    }

    fn short_process_output(bytes: &[u8]) -> String {
        const MAX_LEN: usize = 600;
        let text = String::from_utf8_lossy(bytes)
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if text.is_empty() {
            "The metasearch CLI exited without a diagnostic.".to_string()
        } else if text.len() <= MAX_LEN {
            text
        } else {
            format!("{}...", text.chars().take(MAX_LEN).collect::<String>())
        }
    }

    fn is_server_action_request(path: &str) -> bool {
        path.split('?')
            .next()
            .unwrap_or(path)
            .trim_end_matches('/')
            .starts_with("/.dx/actions/")
    }

    fn execute_project_server_action_request(
        cwd: &Path,
        request: &DxCliHttpRequest,
    ) -> Result<String, String> {
        let sources = Self::react_server_sources(cwd)
            .into_iter()
            .filter(|source| source.kind == DxReactServerSourceKind::Action)
            .collect::<Vec<_>>();
        server_action_runtime::execute_project_server_action_request(&sources, request)
    }

    fn http_status_line(status: u16) -> String {
        let reason = match status {
            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            204 => "No Content",
            301 => "Moved Permanently",
            302 => "Found",
            303 => "See Other",
            307 => "Temporary Redirect",
            308 => "Permanent Redirect",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            409 => "Conflict",
            422 => "Unprocessable Entity",
            500 => "Internal Server Error",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            _ => "OK",
        };
        format!("{status} {reason}")
    }

    fn react_component_sources(cwd: &Path) -> Vec<DxReactComponentSource> {
        let package_ids = Self::forge_package_ids_by_file(cwd);
        let mut sources = Vec::new();
        for root in ["components", "lib/stores"] {
            Self::collect_react_component_sources(cwd, &cwd.join(root), &package_ids, &mut sources);
        }
        sources.sort_by(|left, right| left.source_path.cmp(&right.source_path));
        sources
    }

    fn collect_react_component_sources(
        cwd: &Path,
        dir: &Path,
        package_ids: &HashMap<String, String>,
        sources: &mut Vec<DxReactComponentSource>,
    ) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                Self::collect_react_component_sources(cwd, &path, package_ids, sources);
                continue;
            }
            if !path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| matches!(extension, "tsx" | "jsx" | "ts"))
            {
                continue;
            }
            let Ok(source) = std::fs::read_to_string(&path) else {
                continue;
            };
            let source_path = Self::relative_cli_path(cwd, &path);
            let name = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(str::to_string)
                .unwrap_or_else(|| "Component".to_string());
            sources.push(DxReactComponentSource {
                name,
                package_id: package_ids.get(&source_path).cloned(),
                source_path,
                source,
            });
        }
    }

    fn react_style_sources(cwd: &Path) -> Vec<DxReactStyleSource> {
        let mut sources = Vec::new();
        for root in ["styles", "app", "src/app", "components"] {
            let root_path = cwd.join(root);
            if !root_path.exists() {
                continue;
            }
            for entry in walkdir::WalkDir::new(&root_path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry.file_type().is_file()
                        && entry
                            .path()
                            .extension()
                            .and_then(|extension| extension.to_str())
                            .is_some_and(|extension| extension == "css")
                        && entry.path().components().all(|component| {
                            component.as_os_str().to_string_lossy() != "node_modules"
                        })
                })
            {
                let path = entry.path();
                let Ok(source) = std::fs::read_to_string(path) else {
                    continue;
                };
                sources.push(DxReactStyleSource {
                    source_path: Self::relative_cli_path(cwd, path),
                    source,
                });
            }
        }
        sources.sort_by(|left, right| left.source_path.cmp(&right.source_path));
        sources.dedup_by(|left, right| left.source_path == right.source_path);
        sources
    }

    fn react_server_sources(cwd: &Path) -> Vec<DxReactServerSource> {
        let mut sources = Vec::new();
        Self::push_react_server_source(
            cwd,
            &mut sources,
            &cwd.join("server/loaders.ts"),
            DxReactServerSourceKind::Loader,
        );
        Self::push_react_server_source(
            cwd,
            &mut sources,
            &cwd.join("server/actions.ts"),
            DxReactServerSourceKind::Action,
        );

        for app_root in app_segment_files::app_route_roots(cwd) {
            let api_dir = app_root.join("api");
            if api_dir.exists() {
                for entry in walkdir::WalkDir::new(&api_dir)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|entry| {
                        entry.file_type().is_file()
                            && matches!(
                                entry.file_name().to_string_lossy().as_ref(),
                                "route.ts" | "route.tsx" | "route.js" | "route.jsx"
                            )
                    })
                {
                    Self::push_react_server_source(
                        cwd,
                        &mut sources,
                        entry.path(),
                        DxReactServerSourceKind::RouteHandler,
                    );
                }
            }
        }

        sources.sort_by(|left, right| left.source_path.cmp(&right.source_path));
        sources
    }

    fn push_react_server_source(
        cwd: &Path,
        sources: &mut Vec<DxReactServerSource>,
        path: &Path,
        kind: DxReactServerSourceKind,
    ) {
        let Ok(source) = std::fs::read_to_string(path) else {
            return;
        };
        sources.push(DxReactServerSource {
            kind,
            source_path: Self::relative_cli_path(cwd, path),
            source,
        });
    }

    fn react_import_resolutions(cwd: &Path) -> Vec<DxReactResolvedImport> {
        let config = Self::react_import_resolver_config(cwd);
        let mut resolutions = Vec::new();
        for root in ["app", "components", "server"] {
            let dir = cwd.join(root);
            if !dir.exists() {
                continue;
            }
            for entry in walkdir::WalkDir::new(dir)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry.file_type().is_file()
                        && entry.path().components().all(|component| {
                            component.as_os_str().to_string_lossy() != "node_modules"
                        })
                        && entry
                            .path()
                            .extension()
                            .and_then(|extension| extension.to_str())
                            .is_some_and(|extension| {
                                matches!(extension, "tsx" | "jsx" | "ts" | "js")
                            })
                })
            {
                let Ok(source) = std::fs::read_to_string(entry.path()) else {
                    continue;
                };
                let source_path = Self::relative_cli_path(cwd, entry.path());
                resolutions.extend(resolve_react_imports(&source_path, &source, config.clone()));
            }
        }
        resolutions.sort_by(|left, right| {
            left.importer_path
                .cmp(&right.importer_path)
                .then(left.specifier.cmp(&right.specifier))
        });
        resolutions
    }

    fn react_import_resolver_config(cwd: &Path) -> DxReactImportResolverConfig {
        let manifest = std::fs::read(cwd.join(".dx/forge/source-manifest.json"))
            .ok()
            .and_then(|bytes| serde_json::from_slice::<DxSourceManifest>(&bytes).ok())
            .unwrap_or_default();
        let mut forge_files = Vec::new();
        let mut reviewed_adapters = Vec::new();
        for package in manifest.packages {
            for file in &package.files {
                forge_files.push(DxReactForgeOwnedFile {
                    import_specifier: package.package_id.clone(),
                    source_path: file.path.clone(),
                    package_id: package.package_id.clone(),
                });
            }
            reviewed_adapters.extend(Self::reviewed_javascript_adapters_for_package(&package));
        }
        DxReactImportResolverConfig {
            aliases: vec![DxReactImportAlias {
                prefix: "@/".to_string(),
                target_root: "".to_string(),
            }],
            forge_files,
            reviewed_adapters,
            strict_no_node_modules: true,
        }
    }

    fn reviewed_javascript_adapters_for_package(
        package: &DxSourcePackage,
    ) -> Vec<DxReactReviewedAdapter> {
        let Some((ecosystem_segment, package_name)) =
            Self::reviewed_javascript_adapter_origin(package)
        else {
            return Vec::new();
        };
        let mut seen = HashSet::new();
        let mut adapters = Vec::new();
        for file in &package.files {
            if let Some(specifier) = Self::reviewed_javascript_subpath_specifier(
                ecosystem_segment,
                &package_name,
                &file.path,
            ) {
                Self::push_reviewed_javascript_adapter(
                    &mut adapters,
                    &mut seen,
                    &specifier,
                    &file.path,
                    &package.package_id,
                );
            }
        }
        adapters
    }

    fn reviewed_javascript_adapter_origin(
        package: &DxSourcePackage,
    ) -> Option<(&'static str, String)> {
        let (ecosystem_segment, package_name) =
            Self::javascript_package_name_from_reference(&package.upstream_name)
                .or_else(|| Self::javascript_package_name_from_reference(&package.package_id))?;
        let source_kind_matches = match ecosystem_segment {
            "npm" => package.source_kind == DxSourceKind::NpmSnapshot,
            "jsr" => package.source_kind == DxSourceKind::ExternalSnapshot,
            _ => false,
        };
        source_kind_matches.then_some((ecosystem_segment, package_name))
    }

    fn push_reviewed_javascript_adapter(
        adapters: &mut Vec<DxReactReviewedAdapter>,
        seen: &mut HashSet<(String, String)>,
        package_name: &str,
        adapter_path: &str,
        package_id: &str,
    ) {
        let key = (package_name.to_string(), adapter_path.to_string());
        if !seen.insert(key) {
            return;
        }
        adapters.push(DxReactReviewedAdapter {
            package_name: package_name.to_string(),
            adapter_path: adapter_path.to_string(),
            package_id: package_id.to_string(),
            reviewed: true,
        });
    }

    fn javascript_package_name_from_reference(reference: &str) -> Option<(&'static str, String)> {
        for (prefix, ecosystem_segment) in [
            ("npm:", "npm"),
            ("jsr:", "jsr"),
            ("npm/", "npm"),
            ("jsr/", "jsr"),
        ] {
            if let Some(raw) = reference.strip_prefix(prefix) {
                let package_name = Self::javascript_package_name_without_version(raw);
                return (!package_name.is_empty()).then_some((ecosystem_segment, package_name));
            }
        }
        None
    }

    fn javascript_package_name_without_version(raw: &str) -> String {
        if let Some(rest) = raw.strip_prefix('@') {
            if let Some((scope, name_and_version)) = rest.split_once('/') {
                let name = name_and_version
                    .rsplit_once('@')
                    .map(|(name, _)| name)
                    .unwrap_or(name_and_version);
                return format!("@{scope}/{name}");
            }
            return raw.to_string();
        }
        raw.rsplit_once('@')
            .map(|(name, _)| name)
            .unwrap_or(raw)
            .to_string()
    }

    fn reviewed_javascript_subpath_specifier(
        ecosystem_segment: &str,
        package_name: &str,
        materialized_path: &str,
    ) -> Option<String> {
        let path = materialized_path.replace('\\', "/");
        let source_root = format!("lib/forge/{ecosystem_segment}/");
        let source_path = path.strip_prefix(&source_root)?;
        let (_, source_relative) = source_path.split_once('/')?;
        let source_relative = Self::strip_js_ts_extension(source_relative)?;
        if source_relative == "index" {
            return Some(package_name.to_string());
        }
        let subpath = source_relative
            .strip_suffix("/index")
            .unwrap_or(source_relative);
        Some(format!("{package_name}/{subpath}"))
    }

    fn strip_js_ts_extension(path: &str) -> Option<&str> {
        [".tsx", ".ts", ".jsx", ".js", ".mjs", ".cjs"]
            .iter()
            .find_map(|extension| path.strip_suffix(extension))
    }

    fn forge_package_ids_by_file(cwd: &Path) -> HashMap<String, String> {
        let manifest_path = cwd.join(".dx/forge/source-manifest.json");
        std::fs::read(&manifest_path)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<DxSourceManifest>(&bytes).ok())
            .map(|manifest| {
                manifest
                    .packages
                    .into_iter()
                    .flat_map(|package| {
                        let package_id = package.package_id;
                        package
                            .files
                            .into_iter()
                            .map(move |file| (file.path, package_id.clone()))
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn source_manifest_hash(cwd: &Path) -> Option<String> {
        std::fs::read(cwd.join(".dx/forge/source-manifest.json"))
            .ok()
            .map(|bytes| blake3::hash(&bytes).to_hex().to_string())
    }

    fn relative_cli_path(root: &Path, path: &Path) -> String {
        path.strip_prefix(root)
            .unwrap_or(path)
            .components()
            .map(|component| component.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/")
    }

    /// Render a .html page file to HTML
    fn render_page(
        cwd: &Path,
        page_path: &Path,
        translations: &HashMap<String, String>,
    ) -> Result<String, String> {
        let content = std::fs::read_to_string(page_path)
            .map_err(|e| format!("Failed to read page: {}", e))?;

        // Extract template content - supports <page>, <template>, or raw content
        let template = if let Some(start) = content.find("<page>") {
            if let Some(end) = content.rfind("</page>") {
                content[start + 6..end].to_string()
            } else {
                return Err("Missing </page> tag".to_string());
            }
        } else if let Some(start) = content.find("<template>") {
            if let Some(end) = content.rfind("</template>") {
                content[start + 10..end].to_string()
            } else {
                return Err("Missing </template> tag".to_string());
            }
        } else {
            // Try to extract just the content between body or main tags or use everything
            content.clone()
        };

        // Load layout
        let layout_path = cwd.join("pages/_layout.html");
        let layout_content = if layout_path.exists() {
            let layout_raw = std::fs::read_to_string(&layout_path)
                .map_err(|e| format!("Failed to read layout: {}", e))?;
            if let Some(start) = layout_raw.find("<page>") {
                if let Some(end) = layout_raw.rfind("</page>") {
                    layout_raw[start + 6..end].to_string()
                } else {
                    Self::default_layout()
                }
            } else if let Some(start) = layout_raw.find("<template>") {
                if let Some(end) = layout_raw.rfind("</template>") {
                    layout_raw[start + 10..end].to_string()
                } else {
                    Self::default_layout()
                }
            } else {
                Self::default_layout()
            }
        } else {
            Self::default_layout()
        };

        // Process template
        let mut html = template;

        // Replace i18n calls: {t!("key")}
        let re = regex::Regex::new(r#"\{t!\("([^"]+)"\)\}"#).unwrap();
        html = re
            .replace_all(&html, |caps: &regex::Captures| {
                let key = &caps[1];
                translations
                    .get(key)
                    .cloned()
                    .unwrap_or_else(|| format!("[{}]", key))
            })
            .to_string();

        // Replace variables with defaults
        html = html.replace(
            "{title}",
            translations
                .get("home.title")
                .unwrap_or(&"DX WWW".to_string()),
        );
        html = html.replace(
            "{description}",
            translations
                .get("home.description")
                .unwrap_or(&"Rust-owned App Router TSX framework".to_string()),
        );
        html = html.replace("{total_users}", "10,000+");
        html = html.replace("{github_stars}", "5,000");

        // Process components - render them inline
        html = Self::process_components(cwd, &html, translations)?;

        // Process dx-icon elements
        html = Self::process_icons(&html);

        // Remove dx-animate attributes for static rendering
        let attr_re = regex::Regex::new(r#"\s*dx-animate[^=]*="[^"]*""#).unwrap();
        html = attr_re.replace_all(&html, "").to_string();

        // Insert into layout
        let final_html = layout_content.replace("{children}", &html);

        // Process layout i18n
        let final_html = re
            .replace_all(&final_html, |caps: &regex::Captures| {
                let key = &caps[1];
                translations
                    .get(key)
                    .cloned()
                    .unwrap_or_else(|| format!("[{}]", key))
            })
            .to_string();

        Ok(Self::wrap_with_html_shell(&final_html))
    }

    /// Process component references in template
    fn process_components(
        cwd: &Path,
        html: &str,
        translations: &HashMap<String, String>,
    ) -> Result<String, String> {
        let mut result = html.to_string();

        // List of known components to process
        let components = [
            ("Button", "ui/Button"),
            ("Card", "ui/Card"),
            ("Badge", "ui/Badge"),
            ("Input", "ui/Input"),
            ("FeatureCard", "FeatureCard"),
            ("StatCard", "StatCard"),
            ("GlowButton", "ui/GlowButton"),
            ("AnimatedCard", "ui/AnimatedCard"),
            ("ParticleBackground", "ui/ParticleBackground"),
            ("Textarea", "ui/Textarea"),
        ];

        for (name, path) in components {
            let component_path = cwd.join("components").join(format!("{}.tsx", path));
            if component_path.exists() {
                result = Self::expand_component(&result, name, &component_path, translations)?;
            }
        }

        Ok(result)
    }

    /// Expand a component tag into its template
    fn expand_component(
        html: &str,
        name: &str,
        _component_path: &PathBuf,
        _translations: &HashMap<String, String>,
    ) -> Result<String, String> {
        let mut result = html.to_string();

        // Simple component expansion - replace self-closing and regular tags
        // For a production system, this would be much more sophisticated

        match name {
            "Button" => {
                // Expand Button component
                let re = regex::Regex::new(r#"<Button\s+([^>]*)>([^<]*)</Button>"#).unwrap();
                result = re.replace_all(&result, |caps: &regex::Captures| {
                    let attrs = &caps[1];
                    let content = &caps[2];
                    let href = Self::extract_attr(attrs, "href");
                    let variant = Self::extract_attr(attrs, "variant").unwrap_or("primary".to_string());
                    let size = Self::extract_attr(attrs, "size").unwrap_or("md".to_string());

                    let variant_class = match variant.as_str() {
                        "primary" => "bg-emerald-600 text-white hover:bg-emerald-700",
                        "secondary" => "bg-slate-800 text-white hover:bg-slate-700",
                        "white" => "bg-white text-slate-900 hover:bg-slate-100",
                        _ => "bg-emerald-600 text-white hover:bg-emerald-700",
                    };
                    let size_class = match size.as_str() {
                        "sm" => "px-3 py-1.5 text-sm",
                        "md" => "px-4 py-2 text-base",
                        "lg" => "px-6 py-3 text-lg",
                        _ => "px-4 py-2 text-base",
                    };

                    if let Some(href) = href {
                        format!(r#"<a href="{}" class="inline-flex items-center justify-center font-medium rounded-lg transition-all {} {}">{}</a>"#,
                            href, variant_class, size_class, content)
                    } else {
                        format!(r#"<button class="inline-flex items-center justify-center font-medium rounded-lg transition-all {} {}">{}</button>"#,
                            variant_class, size_class, content)
                    }
                }).to_string();
            }
            "Card" => {
                let re = regex::Regex::new(r#"<Card\s*([^>]*)>([\s\S]*?)</Card>"#).unwrap();
                result = re.replace_all(&result, |caps: &regex::Captures| {
                    let attrs = &caps[1];
                    let content = &caps[2];
                    let class = Self::extract_attr(attrs, "class").unwrap_or_default();
                    format!(r#"<div class="bg-slate-900 border border-slate-800 rounded-xl shadow-xl {}">{}</div>"#, class, content)
                }).to_string();
            }
            "Badge" => {
                let re = regex::Regex::new(r#"<Badge\s*([^>]*)>([\s\S]*?)</Badge>"#).unwrap();
                result = re.replace_all(&result, |caps: &regex::Captures| {
                    let attrs = &caps[1];
                    let content = &caps[2];
                    let variant = Self::extract_attr(attrs, "variant").unwrap_or("primary".to_string());
                    let class = match variant.as_str() {
                        "emerald" => "bg-emerald-500/20 text-emerald-400 border border-emerald-500/30",
                        "teal" => "bg-teal-500/20 text-teal-400 border border-teal-500/30",
                        _ => "bg-emerald-600 text-white",
                    };
                    format!(r#"<span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium {}">{}</span>"#, class, content)
                }).to_string();
            }
            "FeatureCard" => {
                let re = regex::Regex::new(r#"<FeatureCard\s+([^/]*)/?>"#).unwrap();
                result = re.replace_all(&result, |caps: &regex::Captures| {
                    let attrs = &caps[1];
                    let icon = Self::extract_attr(attrs, "icon").unwrap_or("zap".to_string());
                    let title = Self::extract_attr(attrs, "title").unwrap_or("Feature".to_string());
                    let description = Self::extract_attr(attrs, "description").unwrap_or_default();
                    format!(r#"<div class="bg-slate-900 border border-slate-800 rounded-xl p-6 hover:border-emerald-500 transition-all group">
                        <svg class="w-12 h-12 mb-4 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><title>{}</title><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/></svg>
                        <h3 class="text-xl font-bold text-white mb-2">{}</h3>
                        <p class="text-slate-300">{}</p>
                    </div>"#, icon, title, description)
                }).to_string();
            }
            "StatCard" => {
                let re = regex::Regex::new(r#"<StatCard\s+([^/]*)/?>"#).unwrap();
                result = re.replace_all(&result, |caps: &regex::Captures| {
                    let attrs = &caps[1];
                    let number = Self::extract_attr(attrs, "number").unwrap_or("0".to_string());
                    let label = Self::extract_attr(attrs, "label").unwrap_or("Label".to_string());
                    format!(r#"<div class="text-center p-8">
                        <div class="text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 to-teal-400 mb-2">{}</div>
                        <div class="text-slate-400 text-sm uppercase tracking-wider">{}</div>
                    </div>"#, number, label)
                }).to_string();
            }
            "GlowButton" => {
                let re =
                    regex::Regex::new(r#"<GlowButton\s+([^>]*)>([\s\S]*?)</GlowButton>"#).unwrap();
                result = re.replace_all(&result, |caps: &regex::Captures| {
                    let attrs = &caps[1];
                    let content = &caps[2];
                    let href = Self::extract_attr(attrs, "href").unwrap_or("#".to_string());
                    format!(r#"<a href="{}" class="group relative inline-flex items-center justify-center overflow-hidden rounded-lg p-0.5 text-sm font-medium text-white">
                        <span class="absolute h-full w-full bg-gradient-to-br from-emerald-600 to-emerald-400 opacity-70 group-hover:opacity-100 transition-opacity"></span>
                        <span class="relative flex items-center gap-2 rounded-md bg-slate-950 px-6 py-3 transition-all group-hover:bg-transparent">{}</span>
                    </a>"#, href, content)
                }).to_string();
            }
            "AnimatedCard" => {
                let re = regex::Regex::new(r#"<AnimatedCard\s*([^>]*)>([\s\S]*?)</AnimatedCard>"#)
                    .unwrap();
                result = re
                    .replace_all(&result, |caps: &regex::Captures| {
                        let content = &caps[2];
                        format!(r#"<div class="animate-fade-in">{}</div>"#, content)
                    })
                    .to_string();
            }
            "ParticleBackground" => {
                let re = regex::Regex::new(r#"<ParticleBackground\s*[^/]*/>"#).unwrap();
                result = re.replace_all(&result, r#"<div class="absolute inset-0 overflow-hidden pointer-events-none">
                    <div class="absolute inset-0 bg-gradient-to-t from-slate-950 via-transparent to-transparent"></div>
                </div>"#).to_string();
            }
            _ => {}
        }

        Ok(result)
    }

    /// Extract attribute value from attribute string
    fn extract_attr(attrs: &str, name: &str) -> Option<String> {
        let pattern = format!(r#"{}=\{{([^}}]+)\}}"#, name);
        if let Ok(re) = regex::Regex::new(&pattern) {
            if let Some(caps) = re.captures(attrs) {
                return Some(caps[1].to_string());
            }
        }
        let pattern2 = format!(r#"{}="([^"]*)""#, name);
        if let Ok(re) = regex::Regex::new(&pattern2) {
            if let Some(caps) = re.captures(attrs) {
                return Some(caps[1].to_string());
            }
        }
        None
    }

    /// Process Icon and dx-icon elements into SVG through DX Icons.
    fn process_icons(html: &str) -> String {
        let re = regex::Regex::new(r#"<(?:icon|dx-icon|Icon)\s+([^>]*)/?>"#).unwrap();
        re.replace_all(html, |caps: &regex::Captures| {
            let attrs = &caps[1];
            let name = Self::extract_attr(attrs, "name").unwrap_or("pack:logo".to_string());
            let class = Self::extract_attr(attrs, "class")
                .or_else(|| Self::extract_attr(attrs, "className"))
                .unwrap_or("w-6 h-6".to_string());
            let aria_label = Self::extract_attr(attrs, "aria-label");
            let aria_hidden = Self::extract_attr(attrs, "aria-hidden")
                .unwrap_or_else(|| {
                    if aria_label.is_some() {
                        "false".to_string()
                    } else {
                        "true".to_string()
                    }
                });
            let (set, icon_name) = Self::split_icon_name(&name);
            let mut reader = dx_icon::icons();
            let body = reader
                .get(&set, &icon_name)
                .map(|icon| Self::svg_inner(&icon.to_svg(24)))
                .unwrap_or_else(|| {
                    format!(
                        r#"<path d="M4 4h16v16H4z" data-dx-icon-missing="{}"/>"#,
                        Self::escape_attr(&name)
                    )
                });
            let label_attr = aria_label
                .as_deref()
                .map(|label| format!(r#" aria-label="{}""#, Self::escape_attr(label)))
                .unwrap_or_default();

            format!(
                r#"<svg class="{}" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="{}" data-icon-source="dx-icons" data-dx-icon="{}" data-dx-icon-set="{}" data-dx-icon-name="{}"{}>{}</svg>"#,
                Self::escape_attr(&class),
                Self::escape_attr(&aria_hidden),
                Self::escape_attr(&name),
                Self::escape_attr(&set),
                Self::escape_attr(&icon_name),
                label_attr,
                body
            )
        }).to_string()
    }

    fn split_icon_name(name: &str) -> (String, String) {
        name.split_once(':')
            .map(|(set, icon)| (set.to_string(), icon.to_string()))
            .unwrap_or_else(|| ("pack".to_string(), name.to_string()))
    }

    fn svg_inner(svg: &str) -> String {
        let Some(start) = svg.find('>') else {
            return svg.to_string();
        };
        let Some(end) = svg.rfind("</svg>") else {
            return svg[start + 1..].to_string();
        };
        svg[start + 1..end].to_string()
    }

    fn escape_attr(value: &str) -> String {
        value
            .replace('&', "&amp;")
            .replace('"', "&quot;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    }

    /// Default layout HTML
    fn default_layout() -> String {
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DX WWW</title>
    <link rel="stylesheet" href="./styles/globals.css">
</head>
<body class="dx-www-shell" data-dx-renderer="dx-www-shell">
    <main class="dx-www-shell-main" data-dx-shell-content="page">
        {children}
    </main>
</body>
</html>"#
            .to_string()
    }

    /// Wrap content with full HTML document
    fn wrap_with_html_shell(content: &str) -> String {
        if content.contains("<!DOCTYPE html>") || content.contains("<html") {
            return content.to_string();
        }

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DX WWW</title>
    <link rel="stylesheet" href="./styles/globals.css">
    <style>
        body {{
            margin: 0;
            background: var(--dx-bg, Canvas);
            color: var(--dx-text, CanvasText);
            font-family: var(--font-mono, ui-monospace, SFMono-Regular, Consolas, monospace);
        }}
        .dx-www-shell-main {{
            min-height: 100vh;
        }}
    </style>
</head>
<body class="dx-www-shell" data-dx-renderer="dx-www-shell">
    <main class="dx-www-shell-main" data-dx-shell-content="page">
        {}
    </main>
</body>
</html>"#,
            content
        )
    }

    /// Render error page
    fn render_error_page(code: &str, message: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<!-- DX_DEV_ERROR: {} -->
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - DX WWW</title>
    <link rel="stylesheet" href="./styles/globals.css">
    <style>
        body {{
            margin: 0;
            background: var(--dx-bg, Canvas);
            color: var(--dx-text, CanvasText);
            font-family: var(--font-mono, ui-monospace, SFMono-Regular, Consolas, monospace);
        }}
        .dx-www-error {{
            min-height: 100vh;
            display: grid;
            place-items: center;
            padding: 2rem;
            text-align: center;
        }}
        .dx-www-error-card {{
            max-width: 48rem;
            width: 100%;
            border: 1px solid var(--dx-border, currentColor);
            border-radius: var(--radius, 8px);
            padding: 2rem;
            background: var(--dx-panel, transparent);
            box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
        }}
        .dx-www-error-details {{
            margin-top: 1.5rem;
            padding: 1.5rem;
            background: rgba(220, 38, 38, 0.1);
            border: 1px solid rgba(220, 38, 38, 0.3);
            border-radius: 6px;
            color: #ef4444;
            text-align: left;
            white-space: pre-wrap;
            word-break: break-word;
            font-size: 0.9rem;
            overflow-x: auto;
        }}
        .dx-www-error-card h1 {{ margin-top: 0; color: #ef4444; }}
        a {{ color: inherit; margin-top: 1.5rem; display: inline-block; }}
    </style>
</head>
<body class="dx-www-shell" data-dx-renderer="dx-www-shell">
    <main class="dx-www-error" data-dx-shell-content="error" data-dx-error-code="{}">
        <section class="dx-www-error-card">
            <h1>Error {}</h1>
            <div class="dx-www-error-details">{}</div>
            <a href="/">Go Home</a>
        </section>
    </main>
</body>
</html>"#,
            message.replace("-->", ""), code, code, code, message
        )
    }

    /// Load translations from locale files
    fn load_translations(&self) -> DxResult<HashMap<String, String>> {
        let mut translations = HashMap::new();

        let locale_path = self.cwd.join("locales/en-US.sr");
        if locale_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&locale_path) {
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                        continue;
                    }
                    if let Some((key, value)) = line.split_once('=') {
                        translations.insert(
                            key.trim().to_string(),
                            value.trim().trim_matches('"').to_string(),
                        );
                    }
                }
            }
        }

        // Provide defaults
        if translations.is_empty() {
            translations.insert(
                "home.title".to_string(),
                "Binary-First Web Framework".to_string(),
            );
            translations.insert(
                "home.description".to_string(),
                "Build blazing-fast web applications.".to_string(),
            );
        }

        Ok(translations)
    }

    /// Build for production.
    pub fn cmd_build(&self) -> DxResult<()> {
        self.cmd_build_with_options(DxBuildCommandOptions::default(), "dx build")
    }

    fn cmd_build_with_options(
        &self,
        options: DxBuildCommandOptions,
        invoked_as: &'static str,
    ) -> DxResult<()> {
        match options.target {
            DxBuildTarget::Web => self.cmd_build_web(),
            DxBuildTarget::Android => {
                self.cmd_build_web()?;
                cmd_www_build_android(&self.cwd, invoked_as)
            }
        }
    }

    fn cmd_build_web(&self) -> DxResult<()> {
        use console::style;
        eprintln!();
        eprintln!("  ◆ {} {}", style("dx build").bold().white(), style("· production").dim());
        eprintln!();

        let config = config_diagnostics::load_project_config_with_diagnostics(&self.cwd)?;
        ensure_dx_imports_current_for_build(&self.cwd).map_err(forge_error)?;

        let output_dir = self.cwd.join(&config.build.output_dir);
        ensure_build_output_dirs(&self.cwd, &output_dir)?;

        let translations = self.load_translations()?;
        let mut compiled_count = 0;
        let mut app_routes_compiled = 0;
        let mut app_router_execution_contracts_compiled = 0;
        let mut client_islands_compiled = 0;
        let mut generated_style_assets_compiled = 0;
        let mut streaming_plans_compiled = 0;
        let mut server_data_entries_compiled = 0;
        let mut total_size = 0usize;
        let server_sources = Self::react_server_sources(&self.cwd);

        let legacy_pages = compile_legacy_pages(
            &self.cwd,
            &output_dir,
            &translations,
            |page_path, translations| self.compile_to_binary(page_path, translations),
        )?;
        compiled_count += legacy_pages.compiled_count;
        total_size += legacy_pages.total_size;

        let app_router_build = app_router_build_command::compile_app_router_build_outputs(
            app_router_build_command::DxAppRouterBuildCommandInput {
                cwd: &self.cwd,
                output_dir: &output_dir,
                server_sources: &server_sources,
            },
        )?;
        app_routes_compiled += app_router_build.app_routes_compiled;
        app_router_execution_contracts_compiled +=
            app_router_build.app_router_execution_contracts_compiled;
        client_islands_compiled += app_router_build.client_islands_compiled;
        generated_style_assets_compiled += app_router_build.generated_style_assets_compiled;
        streaming_plans_compiled += app_router_build.streaming_plans_compiled;
        server_data_entries_compiled += app_router_build.server_data_entries_compiled;
        total_size += app_router_build.total_size;
        compiled_count += app_router_build.app_routes_compiled;

        let server_build_artifacts = write_server_build_artifacts(&output_dir, &server_sources)?;
        let server_contracts_compiled = server_build_artifacts.server_contracts_compiled;
        let server_action_protocols_compiled =
            server_build_artifacts.server_action_protocols_compiled;

        if !server_sources.is_empty() {
            write_app_route_handler_receipts(
                &output_dir,
                &server_sources,
                self.cwd.join("node_modules").exists(),
            )?;
        }

        let import_resolutions = Self::react_import_resolutions(&self.cwd);
        let import_artifacts = write_import_build_artifacts(&output_dir, &import_resolutions)?;
        let import_resolutions_compiled = import_artifacts.import_resolutions_compiled;
        let next_adapter_fixtures_emitted = import_artifacts.next_adapter_fixtures_emitted;

        copy_build_asset_tree(&self.cwd, &output_dir, "styles")?;
        copy_build_asset_tree(&self.cwd, &output_dir, "public")?;

        let next_migration_input = DxNextProjectMigrationInput {
            project_dir: &self.cwd,
            output_dir: &output_dir,
            app_routes_compiled,
            app_router_execution_contracts_compiled,
            client_islands_compiled,
            generated_style_assets_compiled,
            streaming_plans_compiled,
            server_contracts_compiled,
            server_action_protocols_compiled,
            import_resolutions: &import_resolutions,
        };
        let next_migration_artifacts = write_next_migration_build_artifacts(next_migration_input)?;
        let next_migration_proof_emitted = next_migration_artifacts.next_migration_proof_emitted;
        let next_familiar_compatibility_evidence_emitted =
            next_migration_artifacts.next_familiar_compatibility_evidence_emitted;
        let next_familiar_fixtures_emitted = write_next_familiar_fixtures(&self.cwd, &output_dir)
            .map_err(forge_error)?
            .is_some();
        let source_build_report =
            match crate::build::SourceBuildEngine::new(crate::build::SourceBuildOptions {
                output_dir: Some(output_dir.clone()),
                ..Default::default()
            })
            .build(&self.cwd)
            {
                Ok(report) => report,
                Err(error) => {
                    remove_failed_build_output(&output_dir);
                    return Err(error);
                }
            };
        let route_handler_receipts_compiled = source_build_report
            .manifest
            .route_handler_receipts
            .receipt_count;
        let server_data_routes = collect_app_server_data_manifest(
            &self.cwd,
            &output_dir,
            &source_build_report.server_data_routes,
        )?;
        let server_data_route_manifest = summarize_app_server_data_manifest_routes(
            &server_data_routes,
            &source_build_report.server_data_routes,
        );

        write_build_manifest_and_deploy_adapter(
            &output_dir,
            BuildManifestInput {
                compiled_count,
                app_routes_compiled,
                tsx_app_router_entrypoint: app_router_build.entrypoint_compiled,
                compatibility_pages_fallback_compiled: self.cwd.join("pages/index.html").is_file()
                    && output_dir.join("pages/index.dxob").is_file(),
                app_router_execution_contracts_compiled,
                client_islands_compiled,
                generated_style_assets_compiled,
                streaming_plans_compiled,
                server_data_entries_compiled,
                server_data_routes: &server_data_routes,
                server_data_route_manifest: &server_data_route_manifest,
                server_contracts_compiled,
                route_handler_receipts_compiled,
                server_action_protocols_compiled,
                import_resolutions_compiled,
                next_adapter_fixtures_emitted,
                next_migration_proof_emitted,
                next_familiar_compatibility_evidence_emitted,
                next_familiar_fixtures_emitted,
                source_build_report: &source_build_report,
                total_size,
            },
            |manifest_json| {
                deploy_adapter_contract::write_deploy_adapter_contract(
                    &self.cwd,
                    &output_dir,
                    &server_sources,
                    manifest_json,
                )
            },
        )?;

        let size_str = if total_size >= 1024 {
            format!("{:.1} kB", total_size as f64 / 1024.0)
        } else {
            format!("{} B", total_size)
        };
        eprintln!();
        eprintln!("  {} Build complete", style("✓").green());
        eprintln!("  {} Routes compiled:  {}", style("·").dim(), style(compiled_count).cyan());
        eprintln!("  {} Output size:      {}", style("·").dim(), style(&size_str).cyan());
        eprintln!("  {} Output:           {}", style("·").dim(), style(config.build.output_dir.display()).white().dim());

        Ok(())
    }

    /// Generate DX Serializer machine cache files for an LLM-format source file.
    pub fn cmd_serializer(&self, args: &[String]) -> DxResult<()> {
        if args.is_empty()
            || args
                .iter()
                .any(|arg| matches!(arg.as_str(), "--help" | "-h"))
        {
            print_serializer_help();
            return if args.is_empty() {
                Err(DxError::ConfigValidationError {
                    message: "serializer source path required".to_string(),
                    field: Some("serializer".to_string()),
                })
            } else {
                Ok(())
            };
        }

        let source = PathBuf::from(&args[0]);
        let source = if source.is_absolute() {
            source
        } else {
            self.cwd.join(source)
        };
        if !source.exists() {
            return Err(DxError::IoError {
                path: Some(source),
                message: "serializer source path does not exist".to_string(),
            });
        }

        let config = serializer::SerializerOutputConfig::new()
            .with_output_dir(self.cwd.join(".dx/serializer"))
            .with_llm(false)
            .with_machine(true);
        let serializer = serializer::SerializerOutput::with_config(config);
        if source.is_dir() {
            let results = serializer.process_directory(&source).map_err(|error| {
                DxError::ConfigValidationError {
                    message: format!("Failed to serialize {}: {error}", source.display()),
                    field: Some("serializer".to_string()),
                }
            })?;
            eprintln!("Generated serializer cache for {} file(s)", results.len());
        } else {
            let result = serializer.process_file(&source).map_err(|error| {
                DxError::ConfigValidationError {
                    message: format!("Failed to serialize {}: {error}", source.display()),
                    field: Some("serializer".to_string()),
                }
            })?;
            eprintln!("Generated {}", result.paths.machine.display());
        }

        Ok(())
    }

    /// Compile a .html or .tsx file to DXOB binary format
    fn compile_to_binary(
        &self,
        path: &Path,
        _translations: &HashMap<String, String>,
    ) -> Result<Vec<u8>, String> {
        let content =
            std::fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

        // DXOB Binary Format:
        // Magic: "DXOB" (4 bytes)
        // Version: u16 (2 bytes)
        // Flags: u16 (2 bytes)
        // Template offset: u32 (4 bytes)
        // Template length: u32 (4 bytes)
        // Script offset: u32 (4 bytes)
        // Script length: u32 (4 bytes)
        // Style offset: u32 (4 bytes)
        // Style length: u32 (4 bytes)
        // String table offset: u32 (4 bytes)
        // String table length: u32 (4 bytes)
        // [Data sections...]

        let mut binary = Vec::new();

        // Magic
        binary.extend_from_slice(b"DXOB");
        // Version
        binary.extend_from_slice(&1u16.to_le_bytes());
        // Flags (0 = page, 1 = component, 2 = layout)
        let flags: u16 = if path.extension().map(|e| e == "pg").unwrap_or(false) {
            0
        } else {
            1
        };
        binary.extend_from_slice(&flags.to_le_bytes());

        // Extract sections
        let template = Self::extract_section(&content, "<page>", "</page>")
            .or_else(|| Self::extract_section(&content, "<component>", "</component>"))
            .unwrap_or_default();
        let script = Self::extract_section(&content, "<script", "</script>").unwrap_or_default();
        let style = Self::extract_section(&content, "<style>", "</style>").unwrap_or_default();

        // Header size = 4 + 2 + 2 + 4*8 = 40 bytes
        let header_size = 40u32;

        let template_bytes = template.as_bytes();
        let script_bytes = script.as_bytes();
        let style_bytes = style.as_bytes();

        // Calculate offsets
        let template_offset = header_size;
        let script_offset = template_offset + template_bytes.len() as u32;
        let style_offset = script_offset + script_bytes.len() as u32;
        let string_table_offset = style_offset + style_bytes.len() as u32;

        // Build string table (simple: just collect all string literals)
        let string_table = Self::build_string_table(&template);
        let string_table_bytes = string_table.join("\0").into_bytes();

        // Write offsets and lengths
        binary.extend_from_slice(&template_offset.to_le_bytes());
        binary.extend_from_slice(&(template_bytes.len() as u32).to_le_bytes());
        binary.extend_from_slice(&script_offset.to_le_bytes());
        binary.extend_from_slice(&(script_bytes.len() as u32).to_le_bytes());
        binary.extend_from_slice(&style_offset.to_le_bytes());
        binary.extend_from_slice(&(style_bytes.len() as u32).to_le_bytes());
        binary.extend_from_slice(&string_table_offset.to_le_bytes());
        binary.extend_from_slice(&(string_table_bytes.len() as u32).to_le_bytes());

        // Write data sections
        binary.extend_from_slice(template_bytes);
        binary.extend_from_slice(script_bytes);
        binary.extend_from_slice(style_bytes);
        binary.extend_from_slice(&string_table_bytes);

        // Add content hash at the end
        let hash = blake3::hash(&binary);
        binary.extend_from_slice(hash.as_bytes());

        Ok(binary)
    }

    /// Extract section content from source
    fn extract_section(content: &str, start_tag: &str, end_tag: &str) -> Option<String> {
        let start = content.find(start_tag)?;
        let end = content.rfind(end_tag)?;

        // Find the end of the start tag
        let content_start = if start_tag.ends_with('>') {
            start + start_tag.len()
        } else {
            content[start..].find('>')? + start + 1
        };

        if content_start < end {
            Some(content[content_start..end].to_string())
        } else {
            None
        }
    }

    /// Build string table from template
    fn build_string_table(template: &str) -> Vec<String> {
        let mut strings = Vec::new();

        // Extract class names
        let class_re = regex::Regex::new(r#"class="([^"]*)""#).unwrap();
        for caps in class_re.captures_iter(template) {
            for class in caps[1].split_whitespace() {
                if !strings.contains(&class.to_string()) {
                    strings.push(class.to_string());
                }
            }
        }

        strings
    }

    /// Generate a new file.
    pub fn cmd_generate(&self, gen_type: &str, name: &str) -> DxResult<()> {
        generate_command::cmd_generate(&self.cwd, gen_type, name)
    }

    // =========================================================================
    // FORGE COMMAND - source-owned package firewall
    // =========================================================================
}
