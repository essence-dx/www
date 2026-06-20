use std::io::Write as _;

impl Cli {
    fn cmd_forge_ci(&self, args: &[String]) -> DxResult<()> {
        let DxForgeCiCommandOptions {
            project,
            out,
            verify_artifacts,
            verify_pages,
            format,
            fail_under,
            quiet,
        } = parse_forge_ci_options(&self.cwd, args)?;

        if let Some(artifact_dir) = verify_artifacts {
            let report = verify_forge_ci_artifacts(&artifact_dir).map_err(forge_error)?;
            if !quiet {
                match format {
                    DxOutputFormat::Terminal => print_forge_ci_artifact_verification(&report),
                    DxOutputFormat::Json => println!(
                        "{}",
                        serde_json::to_string_pretty(&report).map_err(forge_error)?
                    ),
                    DxOutputFormat::Markdown => {
                        print!("{}", forge_ci_artifact_verification_markdown(&report));
                    }
                }
            }

            if let Some(minimum) = fail_under {
                if report.score < minimum {
                    return Err(DxError::ConfigValidationError {
                        message: format!(
                            "Forge CI artifact score {} is below required threshold {minimum}",
                            report.score
                        ),
                        field: Some("fail-under".to_string()),
                    });
                }
            }

            if !report.passed {
                return Err(DxError::InternalError {
                    message: forge_ci_artifact_verification_failure_summary(&report),
                });
            }

            return Ok(());
        }

        if let Some(pages_dir) = verify_pages {
            let report = verify_forge_pages_bundle(&pages_dir).map_err(forge_error)?;
            if !quiet {
                match format {
                    DxOutputFormat::Terminal => print_forge_pages_bundle_verification(&report),
                    DxOutputFormat::Json => println!(
                        "{}",
                        serde_json::to_string_pretty(&report).map_err(forge_error)?
                    ),
                    DxOutputFormat::Markdown => {
                        print!("{}", forge_pages_bundle_verification_markdown(&report));
                    }
                }
            }

            if let Some(minimum) = fail_under {
                if report.score < minimum {
                    return Err(DxError::ConfigValidationError {
                        message: format!(
                            "Forge Pages bundle score {} is below required threshold {minimum}",
                            report.score
                        ),
                        field: Some("fail-under".to_string()),
                    });
                }
            }

            if !report.passed {
                return Err(DxError::InternalError {
                    message: forge_pages_bundle_verification_failure_summary(&report),
                });
            }

            return Ok(());
        }

        let project = project.unwrap_or_else(default_forge_smoke_project);
        let out = out.unwrap_or_else(|| self.cwd.join(".dx/ci"));
        let report = build_forge_smoke_report(&project).map_err(forge_error)?;
        let artifacts = write_forge_ci_command_artifacts(&report, &out, fail_under.unwrap_or(0))
            .map_err(forge_error)?;

        if !quiet {
            match format {
                DxOutputFormat::Terminal => {
                    println!("DX Forge CI");
                    println!("Project: {}", report.project.display());
                    println!("Artifacts: {}", out.display());
                    println!("Passed: {}", report.passed);
                    println!("Score: {}", report.score);
                    println!("Files: {}", artifacts.len());
                    for artifact in &artifacts {
                        println!("- {}", artifact.display());
                    }
                }
                DxOutputFormat::Json => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&forge_ci_summary_json(
                            &report, &out, &artifacts
                        ))
                        .map_err(forge_error)?
                    );
                }
                DxOutputFormat::Markdown => {
                    print!("{}", forge_ci_summary_markdown(&report, &out, &artifacts));
                }
            }
        }

        if let Some(minimum) = fail_under {
            if report.score < minimum {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Forge CI score {} is below required threshold {minimum}",
                        report.score
                    ),
                    field: Some("fail-under".to_string()),
                });
            }
        }

        if !report.passed {
            return Err(DxError::InternalError {
                message: forge_smoke_failure_summary(&report),
            });
        }

        Ok(())
    }

    fn cmd_forge_ci_snippets(&self, args: &[String]) -> DxResult<()> {
        let DxForgeCiSnippetsCommandOptions {
            out,
            publisher_key,
            artifact_dir,
            pages_dir,
            output,
            format,
            fail_under,
            quiet,
        } = parse_forge_ci_snippets_options(&self.cwd, args)?;

        let out = out.unwrap_or_else(|| self.cwd.join(".dx/forge-ci-snippets"));
        let report = build_forge_ci_snippets_report(
            &out,
            &artifact_dir,
            &pages_dir,
            fail_under,
            publisher_key.as_deref(),
        )
        .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal => forge_ci_snippets_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_ci_snippets_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        Ok(())
    }

    fn cmd_forge_audit(&self, args: &[String]) -> DxResult<()> {
        let DxForgeAuditCommandOptions {
            path,
            format,
            fail_under,
        } = parse_forge_audit_options(&self.cwd, args)?;
        let report = audit_supply_chain(&path).map_err(forge_error)?;

        match format {
            DxOutputFormat::Terminal => print_forge_audit_terminal(&report),
            DxOutputFormat::Json => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).map_err(forge_error)?
                );
            }
            DxOutputFormat::Markdown => println!("{}", audit_report_markdown(&report)),
        }

        if fail_under.is_some_and(|threshold| report.risk_score < threshold) {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge audit score {} is below fail-under threshold {}",
                    report.risk_score,
                    fail_under.unwrap_or_default()
                ),
                field: Some("forge audit".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_forge_add(&self, args: &[String]) -> DxResult<()> {
        if forge_help_requested(args) {
            print_forge_add_help();
            return Ok(());
        }
        if args.len() >= 2
            && DxForgeImportEcosystem::from_segment(&args[0]).is_some()
            && !args[1].starts_with('-')
        {
            return self.cmd_forge_acquire(args);
        }

        let DxForgeAddCommandOptions {
            package_id,
            project,
            variant,
            write,
            dry_run,
            only,
            registry,
            local,
            remote_manifest,
            version,
            format,
        } = parse_forge_add_options(&self.cwd, args)?;
        let package_id = package_id.as_str();

        let request = parse_public_forge_add_request(package_id, only.as_deref())?;
        if registry.as_deref() != Some("r2")
            && request.surface_packages
            && format != DxOutputFormat::Json
        {
            println!(
                "DX Forge selected exports: {}",
                request.selected_exports.join(", ")
            );
            println!(
                "DX Forge materializes selected source-owned packages into visible project folders."
            );
        }

        if let Some("r2") = registry.as_deref() {
            if write {
                return Err(DxError::ConfigValidationError {
                    message: "dx forge add --registry r2 is dry-run only; publish a package dry-run first and wait for governed remote materialization support before writes".to_string(),
                    field: Some("forge add".to_string()),
                });
            }
            if local.is_some() {
                return Err(DxError::ConfigValidationError {
                    message: "--local is only valid with --registry local".to_string(),
                    field: Some("forge add".to_string()),
                });
            }
            if remote_manifest.is_some() && !dry_run {
                return Err(DxError::ConfigValidationError {
                    message: "--remote-manifest is a dry-run-only R2 preview input".to_string(),
                    field: Some("forge add".to_string()),
                });
            }
            let plans = request
                .package_ids
                .iter()
                .map(|package_id| {
                    forge_remote_lifecycle_dry_run(
                        DxForgeRemoteLifecycleAction::Install,
                        package_id,
                        &request.selected_exports,
                        version.as_deref(),
                        &project,
                        remote_manifest.as_deref(),
                    )
                })
                .collect::<DxResult<Vec<_>>>()?;
            return print_forge_remote_lifecycle_plans(&plans, format);
        }

        for package_id in &request.package_ids {
            let local_registry = match registry.as_deref() {
                Some("local") => Some(
                    local
                        .clone()
                        .unwrap_or_else(|| project.join(".dx/forge/registry/local")),
                ),
                Some(other) => {
                    return Err(DxError::ConfigValidationError {
                        message: format!(
                            "dx forge add --registry supports local or r2 dry-run today, got {other}"
                        ),
                        field: Some("forge add".to_string()),
                    });
                }
                None => None,
            };
            let registry_selected_exports = if request.surface_packages {
                &[][..]
            } else {
                request.selected_exports.as_slice()
            };
            let outcome = if write {
                if let Some(local_registry) = local_registry.as_ref() {
                    let version = resolve_local_registry_version(
                        local_registry,
                        package_id,
                        version.as_deref(),
                    )?;
                    write_forge_add_from_local_registry(
                        package_id,
                        &version,
                        registry_selected_exports,
                        local_registry,
                        &project,
                    )
                    .map_err(forge_error)?
                } else if !request.surface_packages && !request.selected_exports.is_empty() {
                    write_forge_add_selected_exports(
                        package_id,
                        &request.selected_exports,
                        &project,
                    )
                    .map_err(forge_error)?
                } else {
                    write_forge_add_variant(package_id, &variant, &project).map_err(forge_error)?
                }
            } else {
                if let Some(local_registry) = local_registry.as_ref() {
                    let version = resolve_local_registry_version(
                        local_registry,
                        package_id,
                        version.as_deref(),
                    )?;
                    plan_forge_add_from_local_registry(
                        package_id,
                        &version,
                        registry_selected_exports,
                        local_registry,
                        &project,
                    )
                    .map_err(forge_error)?
                } else if !request.surface_packages && !request.selected_exports.is_empty() {
                    plan_forge_add_selected_exports(package_id, &request.selected_exports, &project)
                        .map_err(forge_error)?
                } else {
                    plan_forge_add_variant(package_id, &variant, &project).map_err(forge_error)?
                }
            };

            println!("{}", add_outcome_markdown(&outcome));
            if !write {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&outcome.receipt).map_err(forge_error)?
                );
            }
        }
        if write {
            new_command::refresh_forge_package_status_receipts(&project)?;
        }

        Ok(())
    }

    fn cmd_forge_update(&self, args: &[String]) -> DxResult<()> {
        if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
            eprintln!(
                "Usage: dx forge update <package> [--project <path>] [--variant <name>] [--registry local] [--local <path>] [--version <version>] [--only <exports>] [--dry-run|--write] [--format terminal|json|markdown]"
            );
            eprintln!("       dx forge update ui/button --dry-run");
            eprintln!("       dx forge update ui#button,card --registry local --dry-run");
            eprintln!(
                "       dx forge update package#client,server --registry local --version 0.1.0 --write"
            );
            return Ok(());
        }

        let DxForgeUpdateCommandOptions {
            package_spec,
            project,
            variant,
            format,
            dry_run: _dry_run,
            write,
            accept_yellow,
            review_note,
            reviewer,
            only,
            registry,
            local,
            version,
        } = parse_forge_update_options(&self.cwd, args)?;
        let request = parse_public_forge_add_request(&package_spec, only.as_deref())?;
        if let Some("r2") = registry.as_deref() {
            if write || accept_yellow {
                return Err(DxError::ConfigValidationError {
                    message: "dx forge update --registry r2 is dry-run only; publish a package dry-run first and wait for remote update support before writes".to_string(),
                    field: Some("forge update".to_string()),
                });
            }
            if local.is_some() {
                return Err(DxError::ConfigValidationError {
                    message: "--local is only valid with --registry local".to_string(),
                    field: Some("forge update".to_string()),
                });
            }
            let plans = request
                .package_ids
                .iter()
                .map(|package_id| {
                    forge_remote_lifecycle_dry_run(
                        DxForgeRemoteLifecycleAction::Update,
                        package_id,
                        &request.selected_exports,
                        version.as_deref(),
                        &project,
                        None,
                    )
                })
                .collect::<DxResult<Vec<_>>>()?;
            return print_forge_remote_lifecycle_plans(&plans, format);
        }
        let mut outcomes = Vec::new();

        for package_id in &request.package_ids {
            let outcome = match registry.as_deref() {
                Some("local") => {
                    if accept_yellow {
                        return Err(DxError::ConfigValidationError {
                            message: "dx forge update --registry local writes only green updates today; resolve local edits before writing".to_string(),
                            field: Some("forge update".to_string()),
                        });
                    }
                    let local_registry = local
                        .clone()
                        .unwrap_or_else(|| project.join(".dx/forge/registry/local"));
                    let version = resolve_local_registry_version(
                        &local_registry,
                        package_id,
                        version.as_deref(),
                    )?;
                    let registry_selected_exports = if request.surface_packages {
                        &[][..]
                    } else {
                        request.selected_exports.as_slice()
                    };

                    if write {
                        write_forge_update_from_local_registry(
                            package_id,
                            &version,
                            registry_selected_exports,
                            &local_registry,
                            &project,
                        )
                        .map_err(forge_error)?
                    } else {
                        write_forge_update_dry_run_from_local_registry(
                            package_id,
                            &version,
                            registry_selected_exports,
                            &local_registry,
                            &project,
                        )
                        .map_err(forge_error)?
                    }
                }
                Some(other) => {
                    return Err(DxError::ConfigValidationError {
                        message: format!(
                            "dx forge update --registry supports local or r2 dry-run today, got {other}"
                        ),
                        field: Some("forge update".to_string()),
                    });
                }
                None => {
                    if !request.surface_packages && !request.selected_exports.is_empty() {
                        return Err(DxError::ConfigValidationError {
                            message: "selected root dx update requires --registry local so Forge can resolve a published export map".to_string(),
                            field: Some("forge update".to_string()),
                        });
                    }
                    if write {
                        if accept_yellow {
                            let note = review_note.clone().ok_or_else(|| {
                                DxError::ConfigValidationError {
                                    message: "--accept-yellow requires --review-note".to_string(),
                                    field: Some("review-note".to_string()),
                                }
                            })?;
                            let approval = DxForgeUpdateApproval {
                                reviewer: reviewer.clone().unwrap_or_else(default_update_reviewer),
                                note,
                            };
                            write_forge_update_reviewed_variant(
                                package_id, &variant, &project, approval,
                            )
                            .map_err(forge_error)?
                        } else {
                            write_forge_update_variant(package_id, &variant, &project)
                                .map_err(forge_error)?
                        }
                    } else {
                        write_forge_update_dry_run_variant(package_id, &variant, &project)
                            .map_err(forge_error)?
                    }
                }
            };
            outcomes.push(outcome);
        }
        if write {
            new_command::refresh_forge_package_status_receipts(&project)?;
        }

        match format {
            DxOutputFormat::Terminal | DxOutputFormat::Markdown => {
                for outcome in &outcomes {
                    println!("{}", update_outcome_markdown(outcome));
                }
            }
            DxOutputFormat::Json => {
                if outcomes.len() == 1 {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&outcomes[0]).map_err(forge_error)?
                    );
                } else {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&outcomes).map_err(forge_error)?
                    );
                }
            }
        }

        Ok(())
    }

    fn cmd_forge_public_publish(&self, args: &[String]) -> DxResult<()> {
        if forge_help_requested(args) {
            print_forge_publish_help();
            return Ok(());
        }

        let DxForgePublishCommandOptions {
            registry,
            package,
            local,
            write,
            dry_run,
            confirmed,
            format,
        } = parse_forge_publish_options(&self.cwd, args)?;

        match registry.as_str() {
            "local" => {
                let local = local.unwrap_or_else(|| self.cwd.join(".dx/forge/registry/local"));
                let report = if dry_run {
                    if let Some(requested_package) = package.as_deref() {
                        ensure_root_dx_publish_package_matches(&self.cwd, requested_package)?;
                        publish_root_dx_package_to_local_registry(&self.cwd, &local, true)
                            .map_err(forge_error)?
                    } else {
                        forge_local_registry_publish_plan(&local, package.as_deref())
                    }
                } else {
                    if let Some(requested_package) = package.as_deref() {
                        ensure_root_dx_publish_package_matches(&self.cwd, requested_package)?;
                        publish_root_dx_package_to_local_registry(&self.cwd, &local, false)
                            .map_err(forge_error)?
                    } else {
                        init_local_registry(&local).map_err(forge_error)?
                    }
                };
                print_registry_report(&report, format)?;
                Ok(())
            }
            "r2" => {
                let package = package.ok_or_else(|| DxError::ConfigValidationError {
                    message: "dx forge publish --registry r2 requires --package <id>".to_string(),
                    field: Some("forge publish".to_string()),
                })?;
                let root_package_matches = root_dx_publish_package_matches(&self.cwd, &package);
                if root_package_matches && write && !dry_run {
                    return Err(DxError::ConfigValidationError {
                        message: "root dx package R2 publishing is dry-run only in this pass; run --dry-run and keep live upload gated for operator approval".to_string(),
                        field: Some("forge publish".to_string()),
                    });
                }
                if root_package_matches && (!write || dry_run) {
                    let report =
                        publish_root_dx_package_to_r2_dry_run(&self.cwd).map_err(forge_error)?;
                    return print_registry_report(&report, format);
                }
                let remote_args =
                    forge_registry_publish_args("r2", &package, write, dry_run, confirmed);
                if format == DxOutputFormat::Terminal || format == DxOutputFormat::Markdown {
                    self.cmd_forge_registry_publish(&remote_args)
                } else {
                    let report = forge_r2_publish_report(&package, write, dry_run, confirmed)?;
                    print_registry_report(&report, format)
                }
            }
            other => Err(DxError::ConfigValidationError {
                message: format!("dx forge publish supports --registry local or r2, got {other}"),
                field: Some("forge publish".to_string()),
            }),
        }
    }

    fn cmd_forge_remote_head(&self, args: &[String]) -> DxResult<()> {
        if forge_help_requested(args) {
            print_forge_remote_head_help();
            return Ok(());
        }

        let mut package_spec: Option<String> = None;
        let mut project = self.cwd.clone();
        let mut format = DxOutputFormat::Terminal;
        let mut registry = "r2".to_string();
        let mut remote_manifest: Option<PathBuf> = None;
        let mut requested_version: Option<String> = None;
        let mut only: Option<String> = None;
        let mut approved_by: Option<String> = None;
        let mut provider_mode = "r2-head".to_string();
        let mut execute_live = false;
        let mut dry_run = false;
        let mut write_receipt = false;
        let mut output: Option<PathBuf> = None;
        let mut index = 0usize;

        while index < args.len() {
            match args[index].as_str() {
                "--project" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--project requires a path".to_string(),
                                field: Some("forge remote-head".to_string()),
                            })?;
                    project = resolve_cli_path(&self.cwd, value);
                    index += 2;
                }
                "--registry" | "--remote" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--registry requires r2".to_string(),
                                field: Some("forge remote-head".to_string()),
                            })?;
                    registry = value.clone();
                    index += 2;
                }
                "--remote-manifest" | "--manifest" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--remote-manifest requires a .dx/build-cache/manifest.json path"
                                    .to_string(),
                                field: Some("forge remote-head".to_string()),
                            })?;
                    remote_manifest = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--version" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--version requires a package version".to_string(),
                                field: Some("forge remote-head".to_string()),
                            })?;
                    requested_version = Some(value.clone());
                    index += 2;
                }
                "--only" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--only requires an export list".to_string(),
                                field: Some("forge remote-head".to_string()),
                            })?;
                    only = Some(value.clone());
                    index += 2;
                }
                "--approved-by" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--approved-by requires an operator name".to_string(),
                                field: Some("forge remote-head".to_string()),
                            })?;
                    approved_by = Some(value.clone());
                    index += 2;
                }
                "--provider-mode" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--provider-mode requires a label".to_string(),
                                field: Some("forge remote-head".to_string()),
                            })?;
                    provider_mode = value.clone();
                    index += 2;
                }
                "--yes" | "--confirm" => {
                    execute_live = true;
                    index += 1;
                }
                "--dry-run" => {
                    dry_run = true;
                    index += 1;
                }
                "--write-receipt" => {
                    write_receipt = true;
                    index += 1;
                }
                "--output" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--output requires a receipt path".to_string(),
                                field: Some("forge remote-head".to_string()),
                            })?;
                    output = Some(resolve_cli_path(&self.cwd, value));
                    write_receipt = true;
                    index += 2;
                }
                "--format" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--format requires terminal, json, or markdown"
                                    .to_string(),
                                field: Some("forge remote-head".to_string()),
                            })?;
                    format = DxOutputFormat::parse(value)?;
                    index += 2;
                }
                "--json" => {
                    format = DxOutputFormat::Json;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge remote-head option: {value}"),
                        field: Some("forge remote-head".to_string()),
                    });
                }
                value => {
                    if package_spec.replace(value.to_string()).is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: format!("Unexpected extra package id: {value}"),
                            field: Some("forge remote-head".to_string()),
                        });
                    }
                    index += 1;
                }
            }
        }

        if registry != "r2" {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "dx forge remote-head supports only --registry r2, got {registry}"
                ),
                field: Some("forge remote-head".to_string()),
            });
        }
        if dry_run && execute_live {
            return Err(DxError::ConfigValidationError {
                message: "Choose either --dry-run or --yes, not both".to_string(),
                field: Some("forge remote-head".to_string()),
            });
        }
        let package_spec = package_spec.ok_or_else(|| DxError::ConfigValidationError {
            message: "dx forge remote-head requires a package id or package#surface".to_string(),
            field: Some("forge remote-head".to_string()),
        })?;
        let remote_manifest = remote_manifest.ok_or_else(|| DxError::ConfigValidationError {
            message: "dx forge remote-head requires --remote-manifest <.dx/build-cache/manifest.json> so HEAD checks come from a real Forge package manifest".to_string(),
            field: Some("forge remote-head".to_string()),
        })?;
        let request = parse_public_forge_add_request(&package_spec, only.as_deref())?;
        if request.package_ids.len() != 1 {
            return Err(DxError::ConfigValidationError {
                message: "dx forge remote-head measures one package manifest at a time; run it once per selected package surface".to_string(),
                field: Some("forge remote-head".to_string()),
            });
        }
        let package_id = canonical_package_id(&request.package_ids[0]).to_string();

        if execute_live
            && approved_by
                .as_deref()
                .map(str::trim)
                .unwrap_or_default()
                .is_empty()
        {
            return Err(DxError::ConfigValidationError {
                message: "live dx forge remote-head requires --approved-by <operator>".to_string(),
                field: Some("forge remote-head".to_string()),
            });
        }

        let remote_read_plan = plan_r2_remote_read_only_install_from_manifest_fixture(
            DxForgeRemoteReadIntent::InstallDryRun,
            &package_id,
            &request.selected_exports,
            requested_version.as_deref(),
            &remote_manifest,
            &project,
        )
        .map_err(forge_error)?;
        let metadata_plan = remote_read_plan
            .object_metadata_plan
            .clone()
            .ok_or_else(|| DxError::ConfigValidationError {
                message: "remote manifest did not produce an R2 object metadata plan".to_string(),
                field: Some("forge remote-head".to_string()),
            })?;

        let execution_receipt = if execute_live {
            let approval = DxForgeRemoteObjectHeadExecutionApproval {
                approved_by: approved_by
                    .as_deref()
                    .map(str::trim)
                    .unwrap_or_default()
                    .to_string(),
                provider_mode: provider_mode.trim().to_string(),
                network_allowed: true,
            };
            block_on_registry(execute_r2_remote_object_head_checks_from_env(
                &metadata_plan,
                approval,
            ))?
        } else {
            remote_read_plan
                .object_head_execution_receipt
                .clone()
                .unwrap_or_else(|| plan_r2_remote_object_head_execution_receipt(&metadata_plan))
        };
        let health_evaluation = evaluate_r2_remote_object_head_receipt_health(&execution_receipt);
        let mut warnings = remote_read_plan.warnings.clone();
        warnings.extend(execution_receipt.warnings.clone());
        warnings.extend(health_evaluation.warnings.clone());
        if !execute_live {
            warnings.push(
                "remote HEAD command stayed in planned mode; no live R2/S3 network request was executed"
                    .to_string(),
            );
        }
        warnings.sort();
        warnings.dedup();

        let mut next_actions = health_evaluation.next_actions.clone();
        if execute_live {
            next_actions.push(
                "Feed this measured HEAD health receipt into dx-check/Zed before enabling governed remote materialization."
                    .to_string(),
            );
        } else {
            next_actions.push(
                "For a governed live metadata probe, rerun with --yes --approved-by <operator>; this still performs no writes or blob fetches."
                    .to_string(),
            );
        }
        next_actions.push(
            "Use --write-receipt to persist the JSON report under .dx/forge/receipts/remotes."
                .to_string(),
        );
        next_actions.sort();
        next_actions.dedup();

        let mut report = DxForgeRemoteHeadCliReport {
            schema_version: "dx.forge.remote_head_cli_report",
            command: format!("dx forge remote-head {package_spec} --registry r2"),
            package_id: package_id.to_string(),
            selected_exports: request.selected_exports,
            registry: "r2",
            provider_kind: "s3-compatible-object-storage",
            project,
            remote_manifest,
            version: metadata_plan.version.clone(),
            approved: execution_receipt.approved,
            executed: execution_receipt.checks.iter().any(|check| check.executed),
            dry_run: execution_receipt.dry_run,
            network_allowed: execution_receipt.network_allowed,
            remote_write_allowed: false,
            local_receipt_write_requested: write_receipt,
            receipt_path: None,
            remote_read_plan,
            metadata_plan,
            execution_receipt,
            health_evaluation,
            warnings,
            next_actions,
        };

        if write_receipt {
            let receipt_path = output.unwrap_or_else(|| {
                forge_remote_head_receipt_path(&report.project, &report.package_id, &report.version)
            });
            report.receipt_path = Some(receipt_path.clone());
            let receipt_path = write_forge_remote_head_report(
                &report.project,
                &report.package_id,
                &report.version,
                &report,
                Some(&receipt_path),
            )?;
            report.receipt_path = Some(receipt_path);
        }

        print_forge_remote_head_report(&report, format)
    }

    fn cmd_forge_remove(&self, args: &[String]) -> DxResult<()> {
        if forge_help_requested(args) {
            print_forge_remove_help();
            return Ok(());
        }

        let mut package_id: Option<String> = None;
        let mut project = self.cwd.clone();
        let mut variant = "default".to_string();
        let mut write = false;
        let mut dry_run = false;
        let mut format = DxOutputFormat::Terminal;
        let mut registry: Option<String> = None;
        let mut version: Option<String> = None;
        let mut index = 0usize;

        while index < args.len() {
            match args[index].as_str() {
                "--project" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--project requires a path".to_string(),
                                field: Some("project".to_string()),
                            })?;
                    project = resolve_cli_path(&self.cwd, value);
                    index += 2;
                }
                "--variant" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--variant requires a name".to_string(),
                                field: Some("variant".to_string()),
                            })?;
                    variant = value.clone();
                    index += 2;
                }
                "--write" => {
                    write = true;
                    index += 1;
                }
                "--dry-run" => {
                    dry_run = true;
                    index += 1;
                }
                "--format" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--format requires terminal, json, or markdown"
                                    .to_string(),
                                field: Some("format".to_string()),
                            })?;
                    format = DxOutputFormat::parse(value)?;
                    index += 2;
                }
                "--json" => {
                    format = DxOutputFormat::Json;
                    index += 1;
                }
                "--registry" | "--remote" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--registry requires r2".to_string(),
                                field: Some("forge remove".to_string()),
                            })?;
                    registry = Some(value.clone());
                    index += 2;
                }
                "--version" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--version requires a package version".to_string(),
                                field: Some("forge remove".to_string()),
                            })?;
                    version = Some(value.clone());
                    index += 2;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge remove option: {value}"),
                        field: Some("forge remove".to_string()),
                    });
                }
                value => {
                    if package_id.replace(value.to_string()).is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: format!("Unexpected extra package id: {value}"),
                            field: Some("forge remove".to_string()),
                        });
                    }
                    index += 1;
                }
            }
        }

        if write && dry_run {
            return Err(DxError::ConfigValidationError {
                message: "Choose either --dry-run or --write, not both".to_string(),
                field: Some("forge remove".to_string()),
            });
        }

        let package_id = package_id.ok_or_else(|| DxError::ConfigValidationError {
            message: "Forge remove requires a package id".to_string(),
            field: Some("package".to_string()),
        })?;
        if registry.is_none() && version.is_some() {
            return Err(DxError::ConfigValidationError {
                message: "--version requires --registry r2 for remote remove dry-run planning"
                    .to_string(),
                field: Some("forge remove".to_string()),
            });
        }
        if registry.as_deref() == Some("r2") {
            if write {
                return Err(DxError::ConfigValidationError {
                    message: "dx forge remove --registry r2 is dry-run only; remote uninstall waits for an approved provider adapter".to_string(),
                    field: Some("forge remove".to_string()),
                });
            }
            let request = parse_public_forge_add_request(&package_id, None)?;
            let plans = request
                .package_ids
                .iter()
                .map(|package_id| {
                    forge_remote_lifecycle_dry_run(
                        DxForgeRemoteLifecycleAction::Uninstall,
                        package_id,
                        &request.selected_exports,
                        version.as_deref(),
                        &project,
                        None,
                    )
                })
                .collect::<DxResult<Vec<_>>>()?;
            return print_forge_remote_lifecycle_plans(&plans, format);
        }
        if let Some(other) = registry.as_deref() {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "dx forge remove --registry supports r2 dry-run today, got {other}"
                ),
                field: Some("forge remove".to_string()),
            });
        }
        let outcome = if write {
            write_forge_remove_variant(&package_id, &variant, &project).map_err(forge_error)?
        } else {
            write_forge_remove_dry_run_variant(&package_id, &variant, &project)
                .map_err(forge_error)?
        };

        match format {
            DxOutputFormat::Terminal | DxOutputFormat::Markdown => {
                println!("{}", remove_outcome_markdown(&outcome));
            }
            DxOutputFormat::Json => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&outcome).map_err(forge_error)?
                );
            }
        }

        Ok(())
    }

    fn cmd_forge_rollback(&self, args: &[String]) -> DxResult<()> {
        let mut receipt_path: Option<PathBuf> = None;
        let mut project = self.cwd.clone();
        let mut write = false;
        let mut dry_run = false;
        let mut format = DxOutputFormat::Terminal;
        let mut index = 0usize;

        while index < args.len() {
            match args[index].as_str() {
                "--project" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--project requires a path".to_string(),
                                field: Some("project".to_string()),
                            })?;
                    project = resolve_cli_path(&self.cwd, value);
                    index += 2;
                }
                "--write" => {
                    write = true;
                    index += 1;
                }
                "--dry-run" => {
                    dry_run = true;
                    index += 1;
                }
                "--format" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--format requires terminal, json, or markdown"
                                    .to_string(),
                                field: Some("format".to_string()),
                            })?;
                    format = DxOutputFormat::parse(value)?;
                    index += 2;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge rollback option: {value}"),
                        field: Some("forge rollback".to_string()),
                    });
                }
                value => {
                    if receipt_path
                        .replace(resolve_cli_path(&self.cwd, value))
                        .is_some()
                    {
                        return Err(DxError::ConfigValidationError {
                            message: format!("Unexpected extra rollback receipt path: {value}"),
                            field: Some("receipt".to_string()),
                        });
                    }
                    index += 1;
                }
            }
        }

        if write && dry_run {
            return Err(DxError::ConfigValidationError {
                message: "Choose either --dry-run or --write, not both".to_string(),
                field: Some("forge rollback".to_string()),
            });
        }

        let receipt_path = receipt_path.ok_or_else(|| DxError::ConfigValidationError {
            message: "Forge rollback requires a receipt path".to_string(),
            field: Some("receipt".to_string()),
        })?;
        let outcome = if write {
            write_forge_rollback(&receipt_path, &project).map_err(forge_error)?
        } else {
            plan_forge_rollback(&receipt_path, &project).map_err(forge_error)?
        };

        match format {
            DxOutputFormat::Terminal | DxOutputFormat::Markdown => {
                println!("{}", rollback_outcome_markdown(&outcome));
            }
            DxOutputFormat::Json => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&outcome).map_err(forge_error)?
                );
            }
        }

        Ok(())
    }

    fn cmd_forge_beta_upgrade_smoke(&self, args: &[String]) -> DxResult<()> {
        let DxForgeBetaUpgradeSmokeCommandOptions {
            project,
            from_release_bundle,
            to_release_bundle,
            artifacts,
            output,
            format,
            fail_under,
            write,
            dry_run: _dry_run,
            quiet,
        } = parse_forge_beta_upgrade_smoke_options(&self.cwd, args)?;

        let project = project.unwrap_or_else(|| self.cwd.join(".dx/forge-beta-app"));
        let from_release_bundle = from_release_bundle
            .unwrap_or_else(|| self.cwd.join(".dx/forge-release-bundle-adoption"));
        let to_release_bundle =
            to_release_bundle.unwrap_or_else(|| self.cwd.join(".dx/forge-release-bundle-next"));
        let artifacts = artifacts.unwrap_or_else(|| project.join(".dx/forge/beta-upgrade-smoke"));
        let report = build_forge_beta_upgrade_smoke_report(
            &project,
            &from_release_bundle,
            &to_release_bundle,
            &artifacts,
            write,
            fail_under,
        )
        .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal => forge_beta_upgrade_smoke_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_beta_upgrade_smoke_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        if report.score < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge beta-upgrade-smoke score {} is below fail-under threshold {fail_under}",
                    report.score
                ),
                field: Some("forge beta-upgrade-smoke".to_string()),
            });
        }

        if !report.passed {
            return Err(DxError::InternalError {
                message: forge_beta_upgrade_smoke_failure_summary(&report),
            });
        }

        Ok(())
    }

    fn cmd_forge_registry(&self, args: &[String]) -> DxResult<()> {
        if args.is_empty() || is_help_arg(args.first()) {
            print_forge_registry_help();
            return Ok(());
        }
        if args.get(1).is_some_and(|arg| is_help_arg(Some(arg))) {
            print_forge_registry_subcommand_help(&args[0]);
            return Ok(());
        }

        match args[0].as_str() {
            "build" => self.cmd_forge_registry_build(&args[1..]),
            "docs" | "view" => self.cmd_forge_registry_docs(&args[1..]),
            "init" => self.cmd_forge_registry_init(&args[1..]),
            "list" | "items" | "search" => self.cmd_forge_registry_list(&args[1..]),
            "plan" => self.cmd_forge_registry_plan(&args[1..]),
            "apply" | "materialize" => self.cmd_forge_registry_apply(&args[1..]),
            "parity" => {
                forge_ui_registry_parity::run_forge_ui_registry_parity(&self.cwd, &args[1..])
            }
            "publish" => self.cmd_forge_registry_publish(&args[1..]),
            "pull" => self.cmd_forge_registry_pull(&args[1..]),
            "smoke" => self.cmd_forge_registry_smoke(&args[1..]),
            "status" => self.cmd_forge_registry_status(&args[1..]),
            "validate" => self.cmd_forge_registry_validate(&args[1..]),
            command => Err(DxError::ConfigValidationError {
                message: format!(
                    "Unknown forge registry command `{command}`. Expected one of: validate, build, list, docs, plan, apply, parity, init, smoke, publish, pull, status"
                ),
                field: Some("forge registry".to_string()),
            }),
        }
    }

    fn cmd_forge_registry_validate(&self, args: &[String]) -> DxResult<()> {
        let DxForgeRegistryValidateOptions {
            file,
            output,
            format,
            quiet,
        } = parse_forge_registry_validate_options(&self.cwd, args)?;
        let catalog =
            load_forge_ui_registry_catalog_from_path(&file).map_err(forge_ui_registry_error)?;
        validate_forge_ui_registry_dependency_graphs(&catalog).map_err(forge_ui_registry_error)?;
        let report =
            validate_forge_ui_registry_catalog(&catalog).map_err(forge_ui_registry_error)?;
        let rendered =
            forge_ui_registry_validation_rendered(&report, &file, None, None, None, format)?;

        if let Some(output) = output {
            write_forge_ui_registry_output(&output, &rendered)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        Ok(())
    }

    fn cmd_forge_registry_build(&self, args: &[String]) -> DxResult<()> {
        let DxForgeRegistryBuildOptions {
            file,
            output,
            receipt_output,
            embed_content,
            source_root,
            format,
            quiet,
        } = parse_forge_registry_build_options(&self.cwd, args)?;
        let mut catalog =
            load_forge_ui_registry_catalog_from_path(&file).map_err(forge_ui_registry_error)?;
        let embedding = if embed_content {
            let source_root = source_root.unwrap_or_else(|| {
                file.parent()
                    .map(Path::to_path_buf)
                    .unwrap_or_else(|| self.cwd.clone())
            });
            let (embedded, report) =
                embed_forge_ui_registry_catalog_file_contents(&catalog, source_root)
                    .map_err(forge_ui_registry_error)?;
            catalog = embedded;
            Some(report)
        } else {
            None
        };
        let report =
            validate_forge_ui_registry_catalog(&catalog).map_err(forge_ui_registry_error)?;
        let json = serde_json::to_string_pretty(&catalog).map_err(forge_error)?;

        write_forge_ui_registry_output(&output, &json)?;
        let build_receipt = if let Some(receipt_output) = receipt_output {
            let mut receipt =
                forge_ui_registry_build_receipt::build_forge_ui_registry_build_receipt(
                    forge_ui_registry_build_receipt::DxForgeUiRegistryBuildReceiptInput {
                        registry_file: &file,
                        built_output: &output,
                        output_json: &json,
                        embed_content,
                        content_embedding: embedding.as_ref(),
                        validation: &report,
                    },
                );
            forge_ui_registry_build_receipt::write_forge_ui_registry_build_receipt_artifacts(
                &self.cwd,
                &receipt_output,
                &mut receipt,
            )
            .map_err(forge_error)?;
            Some(receipt)
        } else {
            None
        };

        let rendered = forge_ui_registry_validation_rendered(
            &report,
            &file,
            Some(&output),
            embedding.as_ref(),
            build_receipt.as_ref(),
            format,
        )?;
        if !quiet {
            println!("{rendered}");
        }

        Ok(())
    }

    fn cmd_forge_registry_plan(&self, args: &[String]) -> DxResult<()> {
        let DxForgeRegistryPlanOptions {
            file,
            item,
            project,
            output,
            format,
            quiet,
        } = parse_forge_registry_plan_options(&self.cwd, args)?;
        let resolved =
            resolve_forge_ui_registry_reference(&file, &item).map_err(forge_ui_registry_error)?;
        let report = plan_forge_ui_registry_item(&resolved.catalog, &resolved.item_name, &project)
            .map_err(forge_ui_registry_error)?;
        let rendered = forge_ui_registry_plan_rendered(&report, &resolved.registry_file, format)?;

        if let Some(output) = output {
            write_forge_ui_registry_output(&output, &rendered)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        Ok(())
    }

    fn cmd_forge_registry_docs(&self, args: &[String]) -> DxResult<()> {
        let DxForgeRegistryDocsOptions {
            file,
            item,
            output,
            format,
            quiet,
        } = parse_forge_registry_docs_options(&self.cwd, args)?;
        let resolved =
            resolve_forge_ui_registry_reference(&file, &item).map_err(forge_ui_registry_error)?;
        let report = describe_forge_ui_registry_item(&resolved.catalog, &resolved.item_name)
            .map_err(forge_ui_registry_error)?;
        let rendered = forge_ui_registry_docs_rendered(&report, &resolved.registry_file, format)?;

        if let Some(output) = output {
            write_forge_ui_registry_output(&output, &rendered)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        Ok(())
    }

    fn cmd_forge_registry_list(&self, args: &[String]) -> DxResult<()> {
        let DxForgeRegistryListOptions {
            file,
            item_type,
            query,
            output,
            format,
            quiet,
        } = parse_forge_registry_list_options(&self.cwd, args)?;
        let catalog =
            load_forge_ui_registry_catalog_from_path(&file).map_err(forge_ui_registry_error)?;
        let report =
            validate_forge_ui_registry_catalog(&catalog).map_err(forge_ui_registry_error)?;
        let rendered = forge_ui_registry_list_rendered(
            &catalog,
            &report,
            &file,
            item_type.as_deref(),
            query.as_deref(),
            format,
        )?;

        if let Some(output) = output {
            write_forge_ui_registry_output(&output, &rendered)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        Ok(())
    }

    fn cmd_forge_registry_apply(&self, args: &[String]) -> DxResult<()> {
        let DxForgeRegistryApplyOptions {
            file,
            item,
            project,
            receipt_output,
            output,
            format,
            write,
            dry_run,
            quiet,
        } = parse_forge_registry_apply_options(&self.cwd, args)?;
        let resolved =
            resolve_forge_ui_registry_reference(&file, &item).map_err(forge_ui_registry_error)?;
        let plan = plan_forge_ui_registry_item(&resolved.catalog, &resolved.item_name, &project)
            .map_err(forge_ui_registry_error)?;
        let (mut report, writes) = build_forge_ui_registry_apply_receipt(
            &resolved.catalog,
            &plan,
            &resolved.registry_file,
            write,
            dry_run,
        )?;

        let mut file_transaction = if write && forge_ui_registry_apply_write_ready(&report) {
            Some(DxForgeFileTransaction::new(&plan.project))
        } else {
            None
        };
        if let Some(file_transaction) = file_transaction.as_mut() {
            let write_result = (|| -> DxResult<()> {
                for file_write in &writes {
                    write_forge_ui_registry_source_file_new(
                        &plan.project,
                        &file_write.target_path,
                        &file_write.content,
                        file_transaction,
                    )?;
                }
                Ok(())
            })();
            if let Err(error) = write_result {
                let _rollback_findings = file_transaction.rollback();
                return Err(error);
            }
            forge_ui_registry_apply_mark_written(&mut report, &writes);
        }

        let artifact_result = (|| -> DxResult<()> {
            if let Some(receipt_output) = receipt_output {
                if let Some(file_transaction) = file_transaction.as_mut() {
                    write_forge_ui_registry_apply_receipt_artifacts_with_transaction(
                        &project,
                        &receipt_output,
                        &mut report,
                        file_transaction,
                    )
                    .map_err(forge_error)?;
                } else {
                    write_forge_ui_registry_apply_receipt_artifacts(
                        &project,
                        &receipt_output,
                        &mut report,
                    )
                    .map_err(forge_error)?;
                }
            }
            Ok(())
        })();
        if let Err(error) = artifact_result {
            if let Some(file_transaction) = file_transaction.as_mut() {
                let _rollback_findings = file_transaction.rollback();
            }
            return Err(error);
        }
        if let Some(file_transaction) = file_transaction.as_mut() {
            file_transaction.commit();
        }

        let rendered = forge_ui_registry_apply_rendered(&report, format)?;
        if let Some(output) = output {
            write_forge_ui_registry_output(&output, &rendered)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        if write && !forge_ui_registry_apply_write_ready(&report) {
            return Err(DxError::ConfigValidationError {
                message:
                    "Forge registry apply refused to write because the receipt contains blockers"
                        .to_string(),
                field: Some("forge registry apply".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_forge_registry_init(&self, args: &[String]) -> DxResult<()> {
        let DxForgeRegistryInitOptions { local } =
            parse_forge_registry_init_options(&self.cwd, args)?;
        let report = init_local_registry(local).map_err(forge_error)?;

        println!("{}", registry_operation_markdown(&report));
        Ok(())
    }

    fn cmd_forge_registry_smoke(&self, args: &[String]) -> DxResult<()> {
        let DxForgeRegistrySmokeOptions {
            local,
            remote,
            package,
            output,
            format,
            fail_under,
            quiet,
        } = parse_forge_registry_smoke_options(&self.cwd, args)?;
        ensure_r2_remote(Some(&remote), "smoke")?;
        let publish = block_on_registry(publish_registry_package_to_r2(&package, true))?;
        let report = build_forge_hosted_registry_smoke_report(
            &self.cwd, &local, &remote, &package, publish, fail_under,
        )
        .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal => forge_hosted_registry_smoke_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_hosted_registry_smoke_markdown(&report),
        };

        if let Some(output) = output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        if !quiet {
            println!("{rendered}");
        }

        if report.score < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge hosted registry smoke score {} is below fail-under threshold {}",
                    report.score, fail_under
                ),
                field: Some("forge registry smoke".to_string()),
            });
        }

        if !report.passed {
            return Err(DxError::InternalError {
                message: forge_hosted_registry_smoke_failure_summary(&report),
            });
        }

        Ok(())
    }

    fn cmd_forge_registry_publish(&self, args: &[String]) -> DxResult<()> {
        let DxForgeRegistryPublishOptions {
            remote,
            package,
            dry_run,
            confirmed: _confirmed,
        } = parse_forge_registry_publish_options(args)?;
        ensure_r2_remote(remote.as_deref(), "publish")?;
        let report = block_on_registry(publish_registry_package_to_r2(&package, dry_run))?;

        println!("{}", registry_operation_markdown(&report));
        Ok(())
    }

    fn cmd_forge_registry_pull(&self, args: &[String]) -> DxResult<()> {
        let DxForgeRegistryPullOptions {
            remote,
            package,
            version,
            dry_run,
        } = parse_forge_registry_pull_options(args)?;
        ensure_r2_remote(remote.as_deref(), "pull")?;
        let report = block_on_registry(pull_registry_package_from_r2(&package, &version, dry_run))?;

        println!("{}", registry_operation_markdown(&report));
        Ok(())
    }

    fn cmd_forge_registry_status(&self, args: &[String]) -> DxResult<()> {
        let DxForgeRegistryStatusOptions { remote } = parse_forge_registry_status_options(args)?;
        ensure_r2_remote(remote.as_deref(), "status")?;
        let report = r2_registry_status();

        println!("{}", registry_operation_markdown(&report));
        Ok(())
    }

    /// Run project-wide DX checks.
    pub fn cmd_check(&self, args: &[String]) -> DxResult<()> {
        if args.iter().any(|arg| is_help_arg(Some(arg))) {
            if args.first().map(String::as_str) == Some("web-perf") {
                print_check_web_perf_help();
            } else {
                print_check_help();
            }
            return Ok(());
        }

        if args.first().map(String::as_str) == Some("web-perf") {
            let report = run_dx_web_perf_check(&self.cwd, &args[1..]).map_err(forge_error)?;
            return print_public_tool_report(report).map_err(forge_error);
        }
        if args.first().map(String::as_str) == Some("packages") {
            let report = run_dx_packages_check(&self.cwd, &args[1..]).map_err(forge_error)?;
            return print_public_tool_report(report).map_err(forge_error);
        }

        let mut path: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under: Option<u8> = None;
        let mut strict_forge = false;
        let mut project_contract = false;
        let mut strict_project_contract = false;
        let mut latest_receipt = false;
        let mut hints_output: Option<PathBuf> = None;
        let mut index = 0usize;

        while index < args.len() {
            match args[index].as_str() {
                "--strict-forge" | "--forge-launch-gate" => {
                    strict_forge = true;
                    index += 1;
                }
                "--project-contract" | "--dx-www-contract" => {
                    project_contract = true;
                    index += 1;
                }
                "--strict-project-contract" => {
                    project_contract = true;
                    strict_project_contract = true;
                    index += 1;
                }
                "--latest-receipt" | "--check-panel" => {
                    latest_receipt = true;
                    index += 1;
                }
                "--hints-output" | "--ide-hints-output" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--hints-output requires a path".to_string(),
                                field: Some("hints-output".to_string()),
                            })?;
                    project_contract = true;
                    hints_output = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--format" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--format requires a value".to_string(),
                                field: Some("format".to_string()),
                            })?;
                    format = DxOutputFormat::parse(value)?;
                    index += 2;
                }
                "--json" => {
                    format = DxOutputFormat::Json;
                    index += 1;
                }
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("fail-under".to_string()),
                            })?;
                    fail_under =
                        Some(
                            value
                                .parse::<u8>()
                                .map_err(|_| DxError::ConfigValidationError {
                                    message: format!("Invalid fail-under score: {value}"),
                                    field: Some("fail-under".to_string()),
                                })?,
                        );
                    index += 2;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown dx check option: {value}"),
                        field: Some("check".to_string()),
                    });
                }
                value => {
                    if path.is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: format!("Unexpected extra path: {value}"),
                            field: Some("path".to_string()),
                        });
                    }
                    path = Some(resolve_cli_path(&self.cwd, value));
                    index += 1;
                }
            }
        }

        let path = path.unwrap_or_else(|| self.cwd.clone());
        if latest_receipt {
            let panel = read_dx_check_latest_panel(&path);
            match format {
                DxOutputFormat::Terminal => print_dx_check_latest_panel_terminal(&panel),
                DxOutputFormat::Json => {
                    let mut panel_json = serde_json::to_value(&panel).map_err(forge_error)?;
                    if let Some(panel_object) = panel_json.as_object_mut() {
                        let readiness_replay_commands =
                            serde_json::json!(readiness::readiness_replay_commands());
                        panel_object.insert("release_ready".to_string(), serde_json::json!(false));
                        panel_object
                            .insert("fastest_world_claim".to_string(), serde_json::json!(false));
                        panel_object.insert(
                            "static_readiness_gate_advisory".to_string(),
                            readiness::readiness_gate_status(),
                        );
                        panel_object.insert(
                            "static_readiness_replay_commands".to_string(),
                            readiness_replay_commands.clone(),
                        );
                        panel_object
                            .entry("readiness_replay_commands".to_string())
                            .or_insert_with(|| readiness_replay_commands.clone());
                        panel_object
                            .entry("replay_commands".to_string())
                            .or_insert(readiness_replay_commands);
                    }
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&panel_json).map_err(forge_error)?
                    );
                }
                DxOutputFormat::Markdown => println!("{}", dx_check_latest_panel_markdown(&panel)),
            }

            if let Some(threshold) = fail_under {
                match panel.summary.score_percent {
                    Some(score) if score < threshold => {
                        return Err(DxError::ConfigValidationError {
                            message: format!(
                                "DX check latest receipt score {score} is below fail-under threshold {threshold}"
                            ),
                            field: Some("check".to_string()),
                        });
                    }
                    None => {
                        return Err(DxError::ConfigValidationError {
                            message: "DX check latest receipt has no score for fail-under"
                                .to_string(),
                            field: Some("check".to_string()),
                        });
                    }
                    _ => {}
                }
            }

            return Ok(());
        }

        let mut report = if project_contract {
            check_dx_project_with_options(
                &path,
                DxCheckOptions {
                    project_contract: true,
                },
            )
        } else {
            check_dx_project(&path)
        }
        .map_err(forge_error)?;
        let imports_check = run_dx_imports(&path, &["check".to_string(), "--json".to_string()])
            .map_err(forge_error)?
            .json;
        if imports_check
            .get("passed")
            .and_then(serde_json::Value::as_bool)
            == Some(false)
        {
            report.score = report.score.min(85);
            report.traffic = DxUpdateTraffic::Red;
            report.sections.push(DxCheckSection {
                name: "auto-imports".to_string(),
                score: 0,
                traffic: DxUpdateTraffic::Red,
                metrics: vec![
                    DxCheckMetric {
                        name: "stale_barrel".to_string(),
                        value: imports_check["stale_barrel"].as_bool().unwrap_or(false) as u64,
                    },
                    DxCheckMetric {
                        name: "stale_import_map".to_string(),
                        value: imports_check["stale_import_map"].as_bool().unwrap_or(false) as u64,
                    },
                    DxCheckMetric {
                        name: "stale_declarations".to_string(),
                        value: imports_check["stale_declarations"]
                            .as_bool()
                            .unwrap_or(false) as u64,
                    },
                    DxCheckMetric {
                        name: "stale_sync_receipt".to_string(),
                        value: imports_check["stale_sync_receipt"]
                            .as_bool()
                            .unwrap_or(false) as u64,
                    },
                ],
                findings: vec![DxCheckFinding {
                    severity: DxSupplyChainSeverity::High,
                    code: "auto-import-artifacts-stale".to_string(),
                    message: "DX auto-import artifacts are stale or missing.".to_string(),
                    evidence_path: Some(".dx/imports/import-map.json".to_string()),
                    remediation: "Run dx imports sync, then rerun dx imports check.".to_string(),
                }],
            });
        }

        write_dx_check_latest_receipt(&path, &report)?;

        match format {
            DxOutputFormat::Terminal => print_dx_check_terminal(&report),
            DxOutputFormat::Json => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).map_err(forge_error)?
                );
            }
            DxOutputFormat::Markdown => println!("{}", dx_check_report_markdown(&report)),
        }

        if let Some(hints_output) = hints_output {
            let hints = build_project_contract_hint_artifact(&path, &report);
            if let Some(parent) = hints_output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(
                &hints_output,
                serde_json::to_string_pretty(&hints).map_err(forge_error)?,
            )
            .map_err(forge_error)?;
        }

        if fail_under.is_some_and(|threshold| report.score < threshold) {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX check score {} is below fail-under threshold {}",
                    report.score,
                    fail_under.unwrap_or_default()
                ),
                field: Some("check".to_string()),
            });
        }

        if strict_forge {
            let gate_findings = forge_launch_gate_findings(&report);
            if !gate_findings.is_empty() {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "DX Forge strict release check failed: {}",
                        strict_forge_failure_summary(&gate_findings)
                    ),
                    field: Some("check".to_string()),
                });
            }
        }

        if strict_project_contract {
            let contract = report
                .sections
                .iter()
                .find(|section| section.name == "project-contract");
            if contract.is_none_or(|section| section.traffic == DxUpdateTraffic::Red) {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "DX-WWW project contract failed: {}",
                        project_contract_failure_summary(contract)
                    ),
                    field: Some("check".to_string()),
                });
            }
        }

        Ok(())
    }

    // =========================================================================
    // ADD COMMAND - Forge UI component materialization
    // =========================================================================

    /// Add components to the project.
    pub fn cmd_add(&self, components: &[&str]) -> DxResult<()> {
        // Handle flags
        if components.contains(&"--list") || components.contains(&"-l") {
            return self.list_components();
        }

        if components.contains(&"--all") || components.contains(&"-a") {
            return self.add_all_components();
        }

        if first_dx_add_subject(components).is_some_and(|subject| {
            is_source_owned_add_candidate(
                subject,
                canonical_package_id(subject),
                &FORGE_WWW_TEMPLATE_PACKAGE_IDS,
            )
        }) {
            return self.cmd_add_source_owned_package(components);
        }

        // Add specific components
        for component_name in components {
            if component_name.starts_with('-') {
                continue; // Skip flags
            }
            self.add_component(component_name)?;
        }

        Ok(())
    }

    /// Add a source-owned package through the polished `dx add` path.
    fn cmd_add_source_owned_package(&self, args: &[&str]) -> DxResult<()> {
        let mut package_id: Option<String> = None;
        let mut project = self.cwd.clone();
        let mut variant = "default".to_string();
        let mut write = true;
        let mut dry_run = false;
        let mut explicit_write = false;
        let mut format = DxOutputFormat::Terminal;
        let mut index = 0usize;

        while index < args.len() {
            match args[index] {
                "--project" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--project requires a path".to_string(),
                                field: Some("project".to_string()),
                            })?;
                    project = resolve_cli_path(&self.cwd, value);
                    index += 2;
                }
                "--variant" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--variant requires a name".to_string(),
                                field: Some("variant".to_string()),
                            })?;
                    variant = (*value).to_string();
                    index += 2;
                }
                "--write" => {
                    explicit_write = true;
                    write = true;
                    index += 1;
                }
                "--dry-run" => {
                    dry_run = true;
                    write = false;
                    index += 1;
                }
                "--format" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--format requires terminal, json, or markdown"
                                    .to_string(),
                                field: Some("format".to_string()),
                            })?;
                    format = DxOutputFormat::parse(value)?;
                    index += 2;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown dx add option for source-owned package: {value}"),
                        field: Some("dx add".to_string()),
                    });
                }
                "icon" => {
                    let icon_name =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "dx add icon requires an icon name, for example `dx add icon search`".to_string(),
                                field: Some("package".to_string()),
                            })?;
                    if icon_name.starts_with('-') {
                        return Err(DxError::ConfigValidationError {
                            message: "dx add icon requires an icon name before flags".to_string(),
                            field: Some("package".to_string()),
                        });
                    }
                    if package_id.replace(format!("icon/{icon_name}")).is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: "dx add accepts one source-owned package id at a time for now"
                                .to_string(),
                            field: Some("package".to_string()),
                        });
                    }
                    index += 2;
                }
                value => {
                    if package_id.replace(value.to_string()).is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: "dx add accepts one source-owned package id at a time for now"
                                .to_string(),
                            field: Some("package".to_string()),
                        });
                    }
                    index += 1;
                }
            }
        }

        if explicit_write && dry_run {
            return Err(DxError::ConfigValidationError {
                message: "Choose either --dry-run or --write, not both".to_string(),
                field: Some("dx add".to_string()),
            });
        }

        let package_id = package_id.ok_or_else(|| DxError::ConfigValidationError {
            message: "Source-owned package id required".to_string(),
            field: Some("package".to_string()),
        })?;
        let package_id = canonical_package_id(&package_id).to_string();

        let outcome = if write {
            write_forge_add_variant(&package_id, &variant, &project).map_err(forge_error)?
        } else {
            plan_forge_add_variant(&package_id, &variant, &project).map_err(forge_error)?
        };

        let rendered = match format {
            DxOutputFormat::Terminal => dx_add_outcome_terminal(&outcome),
            DxOutputFormat::Json => serde_json::to_string_pretty(&outcome).map_err(forge_error)?,
            DxOutputFormat::Markdown => add_outcome_markdown(&outcome),
        };

        println!("{rendered}");

        Ok(())
    }

    /// List all available components.
    fn list_components(&self) -> DxResult<()> {
        let components = get_all_components();

        eprintln!("Available components ({}):", components.len());
        eprintln!();

        // Group by category
        let categories = [
            ("Primitives", "primitive"),
            ("Layout", "layout"),
            ("Navigation", "navigation"),
            ("Feedback", "feedback"),
            ("Overlay", "overlay"),
            ("Data Display", "data-display"),
            ("Form", "form"),
        ];

        for (category_name, category_slug) in categories {
            let category_components: Vec<_> = components
                .iter()
                .filter(|c| c.category.as_str() == category_slug)
                .collect();

            if !category_components.is_empty() {
                eprintln!("  {}:", category_name);
                for comp in category_components {
                    eprintln!("    {} - {}", comp.name, comp.description);
                }
                eprintln!();
            }
        }

        eprintln!("Usage: dx add <component-name>");
        eprintln!("       dx add button card modal");
        eprintln!("       dx add --all");

        Ok(())
    }

    /// Add all components to the project.
    fn add_all_components(&self) -> DxResult<()> {
        let components = get_all_components();
        eprintln!("Adding all {} components...", components.len());
        eprintln!();

        for component in &components {
            self.add_component_def(component)?;
        }

        eprintln!();
        eprintln!("? Added {} components to components/", components.len());
        Ok(())
    }

    /// Add a single component by name.
    fn add_component(&self, name: &str) -> DxResult<()> {
        match get_component(name) {
            Some(component) => self.add_component_def(&component),
            None => {
                eprintln!("Component '{}' not found.", name);
                eprintln!();
                eprintln!("Run `dx add --list` to see available components.");
                Err(DxError::ConfigValidationError {
                    message: format!("Unknown component: {}", name),
                    field: Some("component".to_string()),
                })
            }
        }
    }

    /// Add a component definition to the project.
    fn add_component_def(&self, component: &ComponentDef) -> DxResult<()> {
        let components_dir = self.cwd.join("components");
        std::fs::create_dir_all(&components_dir).map_err(|e| DxError::IoError {
            path: Some(components_dir.clone()),
            message: format!("{}:{}: {}", file!(), line!(), e),
        })?;

        let path = components_dir.join(format!("{}.tsx", component.name));

        // Check if already exists
        if path.exists() {
            eprintln!("  ? {} already exists, skipping", component.name);
            return Ok(());
        }

        // Add dependencies first
        for dep_name in &component.dependencies {
            if let Some(dep) = get_component(dep_name) {
                let dep_path = components_dir.join(format!("{}.tsx", dep.name));
                if !dep_path.exists() {
                    self.add_component_def(&dep)?;
                }
            }
        }

        // Write the component file
        std::fs::write(&path, &component.source).map_err(|e| DxError::IoError {
            path: Some(path.clone()),
            message: format!("{}:{}: {}", file!(), line!(), e),
        })?;

        eprintln!("  ? Added {}", component.name);
        Ok(())
    }
}

fn forge_ui_registry_validation_rendered(
    report: &DxForgeUiRegistryValidationReport,
    file: &Path,
    built_output: Option<&Path>,
    embedding: Option<&DxForgeUiRegistryContentEmbeddingReport>,
    build_receipt: Option<&forge_ui_registry_build_receipt::DxForgeUiRegistryBuildReceipt>,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Terminal => Ok(forge_ui_registry_validation_terminal(
            report,
            file,
            built_output,
            embedding,
            build_receipt,
        )),
        DxOutputFormat::Json => serde_json::to_string_pretty(&serde_json::json!({
            "registry_file": file,
            "built_output": built_output,
            "content_embedding": embedding,
            "build_receipt": build_receipt,
            "validation": report,
        }))
        .map_err(forge_error),
        DxOutputFormat::Markdown => Ok(forge_ui_registry_validation_markdown(
            report,
            file,
            built_output,
            embedding,
            build_receipt,
        )),
    }
}

fn forge_ui_registry_plan_rendered(
    report: &DxForgeUiRegistryItemPlanReport,
    file: &Path,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Terminal => Ok(forge_ui_registry_plan_terminal(report, file)),
        DxOutputFormat::Json => serde_json::to_string_pretty(&serde_json::json!({
            "registry_file": file,
            "plan": report,
        }))
        .map_err(forge_error),
        DxOutputFormat::Markdown => Ok(forge_ui_registry_plan_markdown(report, file)),
    }
}

fn forge_ui_registry_docs_rendered(
    report: &DxForgeUiRegistryItemDocsReport,
    file: &Path,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Terminal => Ok(forge_ui_registry_docs_terminal(report, file)),
        DxOutputFormat::Json => serde_json::to_string_pretty(&serde_json::json!({
            "schema": "dx.forge.registry_docs",
            "registry_file": file,
            "no_package_manager_execution": true,
            "docs": report,
        }))
        .map_err(forge_error),
        DxOutputFormat::Markdown => Ok(forge_ui_registry_docs_markdown(report, file)),
    }
}

fn forge_ui_registry_list_rendered(
    catalog: &DxForgeUiRegistryCatalog,
    validation: &DxForgeUiRegistryValidationReport,
    file: &Path,
    item_type: Option<&str>,
    query: Option<&str>,
    format: DxOutputFormat,
) -> DxResult<String> {
    let items = forge_ui_registry_list_items(catalog, item_type, query);
    match format {
        DxOutputFormat::Terminal => Ok(forge_ui_registry_list_terminal(
            validation, file, item_type, query, &items,
        )),
        DxOutputFormat::Json => serde_json::to_string_pretty(&serde_json::json!({
            "schema": "dx.forge.registry_list",
            "registry_file": file,
            "total_items": catalog.items.len(),
            "matched_items": items.len(),
            "type_filter": item_type,
            "query": query,
            "no_package_manager_execution": true,
            "validation": validation,
            "items": items.iter().map(forge_ui_registry_list_item_json).collect::<Vec<_>>(),
        }))
        .map_err(forge_error),
        DxOutputFormat::Markdown => Ok(forge_ui_registry_list_markdown(
            validation, file, item_type, query, &items,
        )),
    }
}

fn forge_ui_registry_list_items<'a>(
    catalog: &'a DxForgeUiRegistryCatalog,
    item_type: Option<&str>,
    query: Option<&str>,
) -> Vec<&'a DxForgeUiRegistryItem> {
    let normalized_type = item_type.map(|value| value.trim().to_ascii_lowercase());
    let normalized_query = query.map(|value| value.trim().to_ascii_lowercase());
    catalog
        .items
        .iter()
        .filter(|item| {
            normalized_type.as_ref().is_none_or(|item_type| {
                forge_ui_registry_item_type_display(item.item_type) == item_type
            })
        })
        .filter(|item| {
            normalized_query
                .as_ref()
                .is_none_or(|query| forge_ui_registry_item_matches_query(item, query))
        })
        .collect()
}

fn forge_ui_registry_item_matches_query(item: &DxForgeUiRegistryItem, query: &str) -> bool {
    let haystack = [
        item.name.as_str(),
        item.title.as_deref().unwrap_or_default(),
        item.description.as_deref().unwrap_or_default(),
    ]
    .join("\n")
    .to_ascii_lowercase();
    haystack.contains(query)
}

fn forge_ui_registry_list_item_json(item: &&DxForgeUiRegistryItem) -> serde_json::Value {
    serde_json::json!({
        "name": item.name,
        "title": item.title.as_deref(),
        "description": item.description.as_deref(),
        "type": forge_ui_registry_item_type_display(item.item_type),
        "files": item.files.len(),
        "dependencies": &item.dependencies,
        "dev_dependencies": &item.dev_dependencies,
        "registry_dependencies": &item.registry_dependencies,
        "categories": &item.categories,
    })
}

fn forge_ui_registry_docs_terminal(
    report: &DxForgeUiRegistryItemDocsReport,
    file: &Path,
) -> String {
    let mut lines = vec![
        "DX Forge Registry Docs".to_string(),
        format!("Registry: {}", file.display()),
        format!("Item: {}", report.item_name),
        format!(
            "Type: {}",
            forge_ui_registry_item_type_display(report.item_type)
        ),
        format!("Title: {}", report.title.as_deref().unwrap_or("none")),
        format!(
            "Description: {}",
            report.description.as_deref().unwrap_or("none")
        ),
        format!(
            "Docs: {}",
            report
                .docs
                .as_deref()
                .unwrap_or("no reviewed docs recorded")
        ),
        format!("Files: {}", report.file_count),
        format!(
            "Registry order: {}",
            forge_ui_registry_registry_order_label(&report.registry_dependency_order)
        ),
        format!("External dependencies: {}", report.dependencies.len()),
        format!(
            "Dev dependencies recorded: {}",
            report.dev_dependencies.len()
        ),
        format!("Env vars: {}", report.env_vars.len()),
        format!("CSS vars: {}", report.css_var_count),
        format!("CSS rules: {}", report.css_rule_count),
        "Package-manager execution: disabled".to_string(),
    ];
    if let Some(style) = &report.base_style {
        lines.push(format!("Base style: {style}"));
    }
    if let Some(icon_library) = &report.base_icon_library {
        lines.push(format!("Base icon library: {icon_library}"));
    }
    if let Some(base_color) = &report.base_color {
        lines.push(format!("Base color: {base_color}"));
    }
    if let Some(theme) = &report.base_theme {
        lines.push(format!("Base theme: {theme}"));
    }

    if !report.files.is_empty() {
        lines.push("Reviewed files:".to_string());
        for file in &report.files {
            lines.push(format!(
                "  - {} -> {} [{}]",
                file.source_path,
                file.target_path,
                forge_ui_registry_plan_action_label(file.action)
            ));
        }
    }

    if !report.dependencies.is_empty() {
        lines.push("External dependency bridges:".to_string());
        for dependency in &report.dependencies {
            lines.push(format!("  - {dependency}"));
        }
    }

    if !report.next_actions.is_empty() {
        lines.push("Next actions:".to_string());
        for action in &report.next_actions {
            lines.push(format!("  - {action}"));
        }
    }

    lines.join("\n")
}

fn forge_ui_registry_docs_markdown(
    report: &DxForgeUiRegistryItemDocsReport,
    file: &Path,
) -> String {
    let mut markdown = String::new();
    markdown.push_str("# DX Forge Registry Docs\n\n");
    markdown.push_str(&format!("- Registry: `{}`\n", file.display()));
    markdown.push_str(&format!("- Item: `{}`\n", report.item_name));
    markdown.push_str(&format!(
        "- Type: `{}`\n",
        forge_ui_registry_item_type_display(report.item_type)
    ));
    if let Some(title) = &report.title {
        markdown.push_str(&format!(
            "- Title: `{}`\n",
            forge_ui_registry_markdown_cell(title)
        ));
    }
    if let Some(description) = &report.description {
        markdown.push_str(&format!(
            "- Description: {}\n",
            forge_ui_registry_markdown_cell(description)
        ));
    }
    markdown.push_str(&format!(
        "- Package-manager execution: `{}`\n",
        if report.no_package_manager_execution {
            "disabled"
        } else {
            "enabled"
        }
    ));
    markdown.push_str(&format!(
        "- Registry order: `{}`\n",
        forge_ui_registry_registry_order_label(&report.registry_dependency_order)
    ));
    markdown.push('\n');

    if report.base_style.is_some()
        || report.base_icon_library.is_some()
        || report.base_color.is_some()
        || report.base_theme.is_some()
    {
        markdown.push_str("## Base Metadata\n\n");
        if let Some(style) = &report.base_style {
            markdown.push_str(&format!(
                "- Style: `{}`\n",
                forge_ui_registry_markdown_cell(style)
            ));
        }
        if let Some(icon_library) = &report.base_icon_library {
            markdown.push_str(&format!(
                "- Icon library: `{}`\n",
                forge_ui_registry_markdown_cell(icon_library)
            ));
        }
        if let Some(base_color) = &report.base_color {
            markdown.push_str(&format!(
                "- Base color: `{}`\n",
                forge_ui_registry_markdown_cell(base_color)
            ));
        }
        if let Some(theme) = &report.base_theme {
            markdown.push_str(&format!(
                "- Theme: `{}`\n",
                forge_ui_registry_markdown_cell(theme)
            ));
        }
        markdown.push('\n');
    }

    markdown.push_str("## Reviewed Docs\n\n");
    markdown.push_str(
        report
            .docs
            .as_deref()
            .filter(|docs| !docs.trim().is_empty())
            .unwrap_or("No reviewed docs are recorded for this registry item."),
    );
    markdown.push_str("\n\n");

    if !report.files.is_empty() {
        markdown.push_str("## Reviewed Files\n\n");
        markdown.push_str("| Source | Target | Action |\n");
        markdown.push_str("|---|---|---|\n");
        for file in &report.files {
            markdown.push_str(&format!(
                "| `{}` | `{}` | `{}` |\n",
                forge_ui_registry_markdown_cell(&file.source_path),
                forge_ui_registry_markdown_cell(&file.target_path),
                forge_ui_registry_plan_action_label(file.action)
            ));
        }
        markdown.push('\n');
    }

    if !report.dependencies.is_empty() {
        markdown.push_str("## External Dependency Bridges\n\n");
        for dependency in &report.dependencies {
            markdown.push_str(&format!(
                "- `{}`\n",
                forge_ui_registry_markdown_cell(dependency)
            ));
        }
        markdown.push('\n');
    }

    if !report.next_actions.is_empty() {
        markdown.push_str("## Next Actions\n\n");
        for action in &report.next_actions {
            markdown.push_str(&format!(
                "- `{}`\n",
                forge_ui_registry_markdown_cell(action)
            ));
        }
    }

    markdown
}

fn forge_ui_registry_list_terminal(
    validation: &DxForgeUiRegistryValidationReport,
    file: &Path,
    item_type: Option<&str>,
    query: Option<&str>,
    items: &[&DxForgeUiRegistryItem],
) -> String {
    let mut lines = vec![
        "DX Forge Registry Items".to_string(),
        format!("Registry: {}", file.display()),
        format!("Total items: {}", validation.item_count),
        format!("Matched items: {}", items.len()),
        format!(
            "Type filter: {}",
            item_type.filter(|value| !value.is_empty()).unwrap_or("all")
        ),
        format!(
            "Query: {}",
            query.filter(|value| !value.is_empty()).unwrap_or("none")
        ),
        "Package-manager execution: disabled".to_string(),
    ];

    if items.is_empty() {
        lines.push("Items: none matched the current filters".to_string());
    } else {
        lines.push("Items:".to_string());
        for item in items {
            lines.push(format!(
                "  - {} [{}] files={} registryDeps={}",
                item.name,
                forge_ui_registry_item_type_display(item.item_type),
                item.files.len(),
                item.registry_dependencies.len()
            ));
        }
    }

    lines.join("\n")
}

fn forge_ui_registry_list_markdown(
    validation: &DxForgeUiRegistryValidationReport,
    file: &Path,
    item_type: Option<&str>,
    query: Option<&str>,
    items: &[&DxForgeUiRegistryItem],
) -> String {
    let mut markdown = String::new();
    markdown.push_str("# DX Forge Registry Items\n\n");
    markdown.push_str(&format!("- Registry: `{}`\n", file.display()));
    markdown.push_str(&format!("- Total items: `{}`\n", validation.item_count));
    markdown.push_str(&format!("- Matched items: `{}`\n", items.len()));
    markdown.push_str(&format!(
        "- Type filter: `{}`\n",
        item_type.filter(|value| !value.is_empty()).unwrap_or("all")
    ));
    markdown.push_str(&format!(
        "- Query: `{}`\n",
        query.filter(|value| !value.is_empty()).unwrap_or("none")
    ));
    markdown.push_str("- Package-manager execution: `disabled`\n\n");

    if items.is_empty() {
        markdown.push_str("No registry items matched the current filters.\n");
        return markdown;
    }

    markdown.push_str("| Item | Type | Files | Registry Dependencies |\n");
    markdown.push_str("|---|---|---:|---:|\n");
    for item in items {
        markdown.push_str(&format!(
            "| `{}` | `{}` | {} | {} |\n",
            forge_ui_registry_markdown_cell(&item.name),
            forge_ui_registry_item_type_display(item.item_type),
            item.files.len(),
            item.registry_dependencies.len()
        ));
    }

    markdown
}

fn forge_ui_registry_plan_terminal(
    report: &DxForgeUiRegistryItemPlanReport,
    file: &Path,
) -> String {
    let mut lines = vec![
        "DX Forge Registry Plan".to_string(),
        format!(
            "Status: {}",
            if report.passed {
                "write-ready"
            } else {
                "review-required"
            }
        ),
        format!("Registry: {}", file.display()),
        format!("Project: {}", report.project.display()),
        format!("Item: {}", report.item_name),
        format!(
            "Type: {}",
            forge_ui_registry_item_type_display(report.item_type)
        ),
        format!("Score: {}", report.score),
        format!(
            "Package-manager execution: {}",
            if report.no_package_manager_execution {
                "disabled"
            } else {
                "enabled"
            }
        ),
        format!("Files: {}", report.file_count),
        format!("Write files: {}", report.write_file_count),
        format!("Inline content files: {}", report.inline_content_file_count),
        format!(
            "Missing reviewed content: {}",
            report.missing_inline_content_count
        ),
        format!("External dependencies: {}", report.dependency_count),
        format!(
            "Registry dependencies: {}",
            report.registry_dependency_count
        ),
        format!("Dev dependencies recorded: {}", report.dev_dependency_count),
    ];

    if !report.registry_dependency_order.is_empty() {
        lines.push(format!(
            "Registry order: {}",
            report.registry_dependency_order.join(" -> ")
        ));
    }

    if !report.registry_dependency_edges.is_empty() {
        lines.push("Registry dependency graph:".to_string());
        for edge in &report.registry_dependency_edges {
            lines.push(format!("  - {} -> {}", edge.from, edge.to));
        }
    }

    if !report.files.is_empty() {
        lines.push("Planned files:".to_string());
        for file in &report.files {
            lines.push(format!(
                "  - {}: {} -> {} [{}]",
                file.item_name,
                file.source_path,
                file.target_path,
                forge_ui_registry_plan_action_label(file.action)
            ));
        }
    }

    if !report.decisions.is_empty() {
        lines.push("Decisions:".to_string());
        for decision in &report.decisions {
            lines.push(format!(
                "  - {}: {} - {}",
                decision.subject,
                forge_ui_registry_plan_decision_label(decision.decision),
                decision.reason
            ));
        }
    }

    if !report.warnings.is_empty() {
        lines.push("Warnings:".to_string());
        for warning in &report.warnings {
            lines.push(format!("  - {warning}"));
        }
    }

    if !report.next_actions.is_empty() {
        lines.push("Next actions:".to_string());
        for action in &report.next_actions {
            lines.push(format!("  - {action}"));
        }
    }

    lines.join("\n")
}

fn forge_ui_registry_plan_markdown(
    report: &DxForgeUiRegistryItemPlanReport,
    file: &Path,
) -> String {
    let mut markdown = String::new();
    markdown.push_str("# DX Forge Registry Plan\n\n");
    markdown.push_str(&format!(
        "- Status: `{}`\n",
        if report.passed {
            "write-ready"
        } else {
            "review-required"
        }
    ));
    markdown.push_str(&format!("- Registry: `{}`\n", file.display()));
    markdown.push_str(&format!("- Project: `{}`\n", report.project.display()));
    markdown.push_str(&format!("- Item: `{}`\n", report.item_name));
    markdown.push_str(&format!(
        "- Type: `{}`\n",
        forge_ui_registry_item_type_display(report.item_type)
    ));
    markdown.push_str(&format!("- Score: `{}`\n", report.score));
    markdown.push_str(&format!(
        "- Package-manager execution: `{}`\n",
        if report.no_package_manager_execution {
            "disabled"
        } else {
            "enabled"
        }
    ));
    markdown.push_str(&format!("- Files: `{}`\n", report.file_count));
    markdown.push_str(&format!("- Write files: `{}`\n", report.write_file_count));
    markdown.push_str(&format!(
        "- Registry order: `{}`\n",
        report.registry_dependency_order.join(" -> ")
    ));
    markdown.push_str(&format!(
        "- Missing reviewed content: `{}`\n",
        report.missing_inline_content_count
    ));
    markdown.push('\n');

    if !report.registry_dependency_edges.is_empty() {
        markdown.push_str("## Registry Dependency Graph\n\n");
        for edge in &report.registry_dependency_edges {
            markdown.push_str(&format!(
                "- `{}` -> `{}`\n",
                forge_ui_registry_markdown_cell(&edge.from),
                forge_ui_registry_markdown_cell(&edge.to)
            ));
        }
        markdown.push('\n');
    }

    if !report.files.is_empty() {
        markdown.push_str("## Planned Files\n\n");
        markdown.push_str("| Item | Source | Target | Action |\n");
        markdown.push_str("|---|---|---|---|\n");
        for file in &report.files {
            markdown.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` |\n",
                forge_ui_registry_markdown_cell(&file.item_name),
                forge_ui_registry_markdown_cell(&file.source_path),
                forge_ui_registry_markdown_cell(&file.target_path),
                forge_ui_registry_plan_action_label(file.action)
            ));
        }
        markdown.push('\n');
    }

    if !report.decisions.is_empty() {
        markdown.push_str("## Decisions\n\n");
        for decision in &report.decisions {
            markdown.push_str(&format!(
                "- `{}`: `{}` - {}\n",
                forge_ui_registry_markdown_cell(&decision.subject),
                forge_ui_registry_plan_decision_label(decision.decision),
                decision.reason
            ));
        }
        markdown.push('\n');
    }

    if !report.warnings.is_empty() {
        markdown.push_str("## Warnings\n\n");
        for warning in &report.warnings {
            markdown.push_str(&format!("- {warning}\n"));
        }
        markdown.push('\n');
    }

    if !report.next_actions.is_empty() {
        markdown.push_str("## Next Actions\n\n");
        for action in &report.next_actions {
            markdown.push_str(&format!("- {action}\n"));
        }
    }

    markdown
}

fn forge_help_requested(args: &[String]) -> bool {
    args.is_empty()
        || args
            .iter()
            .any(|arg| matches!(arg.as_str(), "--help" | "-h" | "help"))
}

fn print_forge_add_help() {
    eprintln!(
        "Usage: dx forge add <package> [--project <path>] [--variant <name>] [--registry local|r2] [--local <path>] [--version <version>] [--only <exports>] [--dry-run|--write] [--format terminal|json|markdown]"
    );
    eprintln!("       dx forge add ui/button --write");
    eprintln!("       dx forge add npm three --json");
    eprintln!("       dx forge add package#client,server --registry local --dry-run");
    eprintln!(
        "       Adds reviewed source-owned package files; R2 registry access is dry-run only until governed remote materialization is approved."
    );
    eprintln!(
        "       npm ecosystem form is an alias for dx forge acquire npm <package>; it writes Forge cache/evidence, not node_modules."
    );
}

fn print_forge_publish_help() {
    eprintln!(
        "Usage: dx forge publish [--registry local|r2] [--package <id>] [--local <path>] [--dry-run|--write] [--yes] [--format terminal|json|markdown]"
    );
    eprintln!("       dx forge publish --registry local --dry-run");
    eprintln!("       dx forge publish --registry r2 --package ui/button --dry-run");
    eprintln!(
        "       Publish requires an explicit --dry-run or --write mode; live remote writes require operator confirmation."
    );
}

fn print_forge_remote_head_help() {
    eprintln!(
        "Usage: dx forge remote-head <package[#exports]> --registry r2 --remote-manifest <.dx/build-cache/manifest.json> [--version <version>] [--project <path>] [--dry-run|--yes] [--approved-by <operator>] [--write-receipt] [--output <path>] [--format terminal|json|markdown]"
    );
    eprintln!(
        "       Plans or measures remote object HEAD health without fetching package blobs or writing remote state."
    );
    eprintln!(
        "       Live measurement requires --yes --approved-by <operator>; planned mode performs no network request."
    );
}

fn print_forge_remove_help() {
    eprintln!(
        "Usage: dx forge remove <package> [--project <path>] [--variant <name>] [--registry r2] [--version <version>] [--dry-run|--write] [--format terminal|json|markdown]"
    );
    eprintln!("       dx forge remove ui/button --dry-run");
    eprintln!("       dx forge remove ui/button --write");
    eprintln!(
        "       Removes local source-owned package files or produces a dry-run remote uninstall plan."
    );
}

fn print_forge_registry_subcommand_help(command: &str) {
    match command {
        "validate" => {
            eprintln!(
                "Usage: dx forge registry validate [--file <path>] [--output <path>] [--format terminal|json|markdown] [--json] [--quiet]"
            );
            eprintln!(
                "Validates an authored Forge UI registry without network access or package-manager execution."
            );
        }
        "build" => {
            eprintln!(
                "Usage: dx forge registry build [--file <path>] --output <path> [--embed-content] [--source-root <path>] [--receipt <path>] [--format terminal|json|markdown] [--json] [--quiet]"
            );
            eprintln!(
                "Flattens authored registry.json include graphs and can embed reviewed local source file contents."
            );
        }
        "plan" => {
            eprintln!(
                "Usage: dx forge registry plan --item <name> [--file <path>] [--project <path>] [--output <path>] [--format terminal|json|markdown] [--json] [--quiet]"
            );
            eprintln!(
                "Plans source-owned UI registry materialization and dependency order without writing files or executing package managers."
            );
        }
        "list" | "items" | "search" => {
            eprintln!(
                "Usage: dx forge registry list [--file <path>] [--type <registry:type>] [--query <text>] [--output <path>] [--format terminal|json|markdown] [--json] [--quiet]"
            );
            eprintln!(
                "List source-owned Forge registry items before planning or materializing them."
            );
            eprintln!(
                "Registry list is discovery; plan, parity, and apply receipts are the capability and scoring truth."
            );
        }
        "docs" | "view" => {
            eprintln!(
                "Usage: dx forge registry docs --item <name> [--file <path>] [--output <path>] [--format terminal|json|markdown] [--json] [--quiet]"
            );
            eprintln!("Read reviewed registry item docs without writing files.");
        }
        "apply" | "materialize" => {
            eprintln!(
                "Usage: dx forge registry apply --item <name> [--file <path>] [--project <path>] [--dry-run|--write] [--receipt <path>] [--output <path>] [--format terminal|json|markdown] [--json] [--quiet]"
            );
            eprintln!(
                "Materializes reviewed inline Forge registry content only; no package manager is executed."
            );
        }
        "parity" => {
            eprintln!(
                "Usage: dx forge registry parity [--format terminal|json|markdown] [--json] [--output <path>] [--quiet]"
            );
            eprintln!(
                "Reports Forge UI registry capability coverage against the component registry reference set."
            );
        }
        "init" => eprintln!("Usage: dx forge registry init --local <path>"),
        "smoke" => eprintln!(
            "Usage: dx forge registry smoke [--remote r2] [--local <path>] [--package <id>] [--output <path>] [--format terminal|json|markdown] [--fail-under <score>] [--quiet]"
        ),
        "publish" => {
            eprintln!("Usage: dx forge registry publish --remote r2 --package <id> --dry-run");
            eprintln!("       dx forge registry publish --remote r2 --package <id> --write --yes");
        }
        "pull" => eprintln!(
            "Usage: dx forge registry pull --remote r2 --package <id> --version <version> [--dry-run]"
        ),
        "status" => eprintln!("Usage: dx forge registry status --remote r2"),
        _ => print_forge_registry_help(),
    }
}

fn write_forge_ui_registry_output(path: &Path, content: &str) -> DxResult<()> {
    let parent = if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent).map_err(forge_error)?;
        parent
    } else {
        Path::new(".")
    };
    let temp_path = parent.join(format!(
        ".forge-registry-{}-{}.tmp",
        std::process::id(),
        forge_registry_output_nonce()
    ));
    let mut temp = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp_path)
        .map_err(forge_error)?;
    temp.write_all(content.as_bytes()).map_err(forge_error)?;
    temp.flush().map_err(forge_error)?;
    temp.sync_all().map_err(forge_error)?;
    drop(temp);

    replace_forge_registry_output(&temp_path, path)
}

fn write_forge_ui_registry_source_file_new(
    project: &Path,
    path: &Path,
    content: &str,
    transaction: &mut DxForgeFileTransaction,
) -> DxResult<()> {
    let project_root = project.canonicalize().map_err(forge_error)?;
    if path.strip_prefix(project).is_err() {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Forge registry apply target `{}` is outside project `{}`",
                path.display(),
                project.display()
            ),
            field: Some("forge registry apply".to_string()),
        });
    }

    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent).map_err(forge_error)?;
        let canonical_parent = parent.canonicalize().map_err(forge_error)?;
        if !canonical_parent.starts_with(&project_root) {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "Forge registry apply target parent `{}` escapes project `{}`",
                    canonical_parent.display(),
                    project_root.display()
                ),
                field: Some("forge registry apply".to_string()),
            });
        }
    }

    if path.exists() {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Forge registry apply target `{}` already exists; rerun as a dry-run and review the receipt before overwriting",
                path.display()
            ),
            field: Some("forge registry apply".to_string()),
        });
    }
    transaction
        .write_bytes_atomic(path, content.as_bytes())
        .map_err(forge_error)?;

    let written = std::fs::read_to_string(path).map_err(forge_error)?;
    if written != content {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Forge registry apply wrote `{}` but verification did not match reviewed content",
                path.display()
            ),
            field: Some("forge registry apply".to_string()),
        });
    }

    Ok(())
}

#[cfg(windows)]
fn replace_forge_registry_output(temp_path: &Path, path: &Path) -> DxResult<()> {
    use std::os::windows::ffi::OsStrExt;

    const MOVEFILE_REPLACE_EXISTING: u32 = 0x1;
    const MOVEFILE_WRITE_THROUGH: u32 = 0x8;

    unsafe extern "system" {
        fn MoveFileExW(
            lp_existing_file_name: *const u16,
            lp_new_file_name: *const u16,
            dw_flags: u32,
        ) -> i32;
    }

    let temp_wide: Vec<u16> = temp_path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let path_wide: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    // SAFETY: Both paths are null-terminated UTF-16 buffers that remain alive for the call.
    let replaced = unsafe {
        MoveFileExW(
            temp_wide.as_ptr(),
            path_wide.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if replaced == 0 {
        let _ = std::fs::remove_file(temp_path);
        return Err(forge_error(std::io::Error::last_os_error()));
    }
    Ok(())
}

#[cfg(not(windows))]
fn replace_forge_registry_output(temp_path: &Path, path: &Path) -> DxResult<()> {
    if let Err(error) = std::fs::rename(temp_path, path) {
        let _ = std::fs::remove_file(temp_path);
        return Err(forge_error(error));
    }
    Ok(())
}

fn forge_registry_output_nonce() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default()
}

fn forge_ui_registry_error(error: impl std::fmt::Display) -> DxError {
    DxError::ConfigValidationError {
        message: format!("{}:{}: {}", file!(), line!(), error),
        field: Some("forge registry".to_string()),
    }
}

fn forge_ui_registry_validation_terminal(
    report: &DxForgeUiRegistryValidationReport,
    file: &Path,
    built_output: Option<&Path>,
    embedding: Option<&DxForgeUiRegistryContentEmbeddingReport>,
    build_receipt: Option<&forge_ui_registry_build_receipt::DxForgeUiRegistryBuildReceipt>,
) -> String {
    let mut lines = vec![
        "DX Forge UI Registry".to_string(),
        format!("Status: {}", if report.valid { "valid" } else { "invalid" }),
        format!("Registry: {}", file.display()),
        format!("Schema: {}", report.schema_version),
    ];
    if let Some(output) = built_output {
        lines.push(format!("Built output: {}", output.display()));
    }
    if let Some(embedding) = embedding {
        lines.push(format!(
            "Content source root: {}",
            embedding.source_root.display()
        ));
        lines.push(format!(
            "Embedded content files: {}",
            embedding.embedded_file_count
        ));
        lines.push(format!(
            "Preserved inline content files: {}",
            embedding.preserved_inline_content_file_count
        ));
    }
    if let Some(receipt) = build_receipt {
        if let Some(path) = &receipt.artifacts.receipt_json_path {
            lines.push(format!("Build receipt: {}", path.display()));
        }
        if let Some(path) = &receipt.artifacts.receipt_sr_path {
            lines.push(format!("Build receipt .sr: {}", path.display()));
        }
        if let Some(path) = &receipt.artifacts.receipt_json_machine_path {
            lines.push(format!("Build receipt machine: {}", path.display()));
        }
    }
    lines.extend([
        format!("Items: {}", report.item_count),
        format!("Files: {}", report.file_count),
        format!("Includes: {}", report.include_count),
        format!("Dependencies: {}", report.dependency_count),
        format!("Dev dependencies: {}", report.dev_dependency_count),
        format!(
            "Registry dependencies: {}",
            report.registry_dependency_count
        ),
        format!("Env vars: {}", report.env_var_count),
        format!("Docs: {}", report.docs_count),
    ]);
    if !report.item_types.is_empty() {
        lines.push("Types:".to_string());
        for (item_type, count) in &report.item_types {
            lines.push(format!(
                "  - {}: {count}",
                forge_ui_registry_item_type_display(*item_type)
            ));
        }
    }
    lines.join("\n")
}

fn forge_ui_registry_validation_markdown(
    report: &DxForgeUiRegistryValidationReport,
    file: &Path,
    built_output: Option<&Path>,
    embedding: Option<&DxForgeUiRegistryContentEmbeddingReport>,
    build_receipt: Option<&forge_ui_registry_build_receipt::DxForgeUiRegistryBuildReceipt>,
) -> String {
    let mut markdown = String::new();
    markdown.push_str("# DX Forge UI Registry\n\n");
    markdown.push_str(&format!(
        "- Status: {}\n",
        if report.valid { "valid" } else { "invalid" }
    ));
    markdown.push_str(&format!("- Registry: `{}`\n", file.display()));
    markdown.push_str(&format!("- Schema: `{}`\n", report.schema_version));
    if let Some(output) = built_output {
        markdown.push_str(&format!("- Built output: `{}`\n", output.display()));
    }
    if let Some(embedding) = embedding {
        markdown.push_str(&format!(
            "- Content source root: `{}`\n",
            embedding.source_root.display()
        ));
        markdown.push_str(&format!(
            "- Embedded content files: `{}`\n",
            embedding.embedded_file_count
        ));
        markdown.push_str(&format!(
            "- Preserved inline content files: `{}`\n",
            embedding.preserved_inline_content_file_count
        ));
    }
    if let Some(receipt) = build_receipt {
        if let Some(path) = &receipt.artifacts.receipt_json_path {
            markdown.push_str(&format!("- Build receipt: `{}`\n", path.display()));
        }
        if let Some(path) = &receipt.artifacts.receipt_sr_path {
            markdown.push_str(&format!("- Build receipt `.sr`: `{}`\n", path.display()));
        }
        if let Some(path) = &receipt.artifacts.receipt_json_machine_path {
            markdown.push_str(&format!("- Build receipt machine: `{}`\n", path.display()));
        }
    }
    markdown.push_str(&format!("- Items: `{}`\n", report.item_count));
    markdown.push_str(&format!("- Files: `{}`\n", report.file_count));
    markdown.push_str(&format!("- Includes: `{}`\n", report.include_count));
    markdown.push_str(&format!("- Dependencies: `{}`\n", report.dependency_count));
    markdown.push_str(&format!(
        "- Dev dependencies: `{}`\n",
        report.dev_dependency_count
    ));
    markdown.push_str(&format!(
        "- Registry dependencies: `{}`\n",
        report.registry_dependency_count
    ));
    markdown.push_str(&format!("- Env vars: `{}`\n", report.env_var_count));
    markdown.push_str(&format!("- Docs: `{}`\n", report.docs_count));
    if !report.item_types.is_empty() {
        markdown.push_str("\n## Types\n\n");
        for (item_type, count) in &report.item_types {
            markdown.push_str(&format!(
                "- `{}`: `{count}`\n",
                forge_ui_registry_item_type_display(*item_type)
            ));
        }
    }
    markdown
}

fn forge_ui_registry_plan_action_label(action: DxForgeUiRegistryPlanAction) -> &'static str {
    match action {
        DxForgeUiRegistryPlanAction::Materialize => "materialize",
        DxForgeUiRegistryPlanAction::NeedsReviewedContent => "needs-reviewed-content",
    }
}

fn forge_ui_registry_plan_decision_label(
    decision: DxForgeUiRegistryPlanDecisionKind,
) -> &'static str {
    match decision {
        DxForgeUiRegistryPlanDecisionKind::Materialize => "materialize",
        DxForgeUiRegistryPlanDecisionKind::ResolveRegistryDependency => {
            "resolve-registry-dependency"
        }
        DxForgeUiRegistryPlanDecisionKind::BridgeDependency => "bridge-dependency",
        DxForgeUiRegistryPlanDecisionKind::IgnoreDevDependency => "ignore-dev-dependency",
        DxForgeUiRegistryPlanDecisionKind::RequireEnvironment => "require-environment",
        DxForgeUiRegistryPlanDecisionKind::MergeStyle => "merge-style",
        DxForgeUiRegistryPlanDecisionKind::MergeConfig => "merge-config",
        DxForgeUiRegistryPlanDecisionKind::RegisterFont => "register-font",
    }
}

fn forge_ui_registry_markdown_cell(value: &str) -> String {
    value.replace('|', "\\|")
}

fn forge_ui_registry_registry_order_label(order: &[String]) -> String {
    if order.is_empty() {
        "none".to_string()
    } else {
        order.join(" -> ")
    }
}

fn forge_ui_registry_item_type_display(item_type: DxForgeUiRegistryItemType) -> &'static str {
    match item_type {
        DxForgeUiRegistryItemType::Lib => "registry:lib",
        DxForgeUiRegistryItemType::Block => "registry:block",
        DxForgeUiRegistryItemType::Component => "registry:component",
        DxForgeUiRegistryItemType::Ui => "registry:ui",
        DxForgeUiRegistryItemType::Hook => "registry:hook",
        DxForgeUiRegistryItemType::Page => "registry:page",
        DxForgeUiRegistryItemType::File => "registry:file",
        DxForgeUiRegistryItemType::Theme => "registry:theme",
        DxForgeUiRegistryItemType::Style => "registry:style",
        DxForgeUiRegistryItemType::Item => "registry:item",
        DxForgeUiRegistryItemType::Base => "registry:base",
        DxForgeUiRegistryItemType::Font => "registry:font",
        DxForgeUiRegistryItemType::Example => "registry:example",
        DxForgeUiRegistryItemType::Internal => "registry:internal",
    }
}
