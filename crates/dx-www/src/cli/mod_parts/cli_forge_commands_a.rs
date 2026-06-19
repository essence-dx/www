impl Cli {
    /// Run DX Forge package-governance commands.
    pub fn cmd_forge(&self, args: &[String]) -> DxResult<()> {
        if args.is_empty() || is_help_arg(args.first()) {
            print_forge_help();
            return Ok(());
        }

        match args[0].as_str() {
            "audit" => self.cmd_forge_audit(&args[1..]),
            "add" => self.cmd_forge_add(&args[1..]),
            "acquire" | "fetch" => self.cmd_forge_acquire(&args[1..]),
            "adoption-report" => self.cmd_forge_adoption_report(&args[1..]),
            "adoption-smoke" => self.cmd_forge_adoption_smoke(&args[1..]),
            "badge" => self.cmd_forge_badge(&args[1..]),
            "beta-artifact-verify" | "downloaded-beta-verify" | "verify-beta-artifacts" => {
                self.cmd_forge_beta_artifact_verify(&args[1..])
            }
            "beta-install" => self.cmd_forge_beta_install(&args[1..]),
            "beta-upgrade-smoke" | "upgrade-smoke" => self.cmd_forge_beta_upgrade_smoke(&args[1..]),
            "beta-diagnostics" | "diagnostics" => self.cmd_forge_beta_diagnostics(&args[1..]),
            "ci" => self.cmd_forge_ci(&args[1..]),
            "ci-snippets" | "portable-ci" | "ci-templates" => {
                self.cmd_forge_ci_snippets(&args[1..])
            }
            "doctor" => run_forge_doctor(&self.cwd, &args[1..]),
            "docs" => self.cmd_forge_docs(&args[1..]),
            "evidence" => self.cmd_forge_evidence(&args[1..]),
            "review" | "source-review" | "package-review" | "import" => {
                self.cmd_forge_import(&args[1..])
            }
            "init-app" => self.cmd_forge_init_app(&args[1..]),
            "launch-copy-review" | "copy-review" => self.cmd_forge_launch_copy_review(&args[1..]),
            "launch-page" => self.cmd_forge_launch_page(&args[1..]),
            "migration-audit" | "static-migration-audit" | "wordpress-migration-audit" => {
                self.cmd_forge_migration_audit(&args[1..])
            }
            "migrate-static-page" | "static-page-migrate" | "migrate-page" => {
                self.cmd_forge_migrate_static_page(&args[1..])
            }
            "static-migration-plan" | "migration-plan" | "plan-static-migration" => {
                self.cmd_forge_static_migration_plan(&args[1..])
            }
            "static-migration-smoke" | "migration-smoke" | "migrate-static-page-smoke" => {
                self.cmd_forge_static_migration_smoke(&args[1..])
            }
            "materialize-static-assets" | "static-assets" | "copy-static-assets" => {
                self.cmd_forge_materialize_static_assets(&args[1..])
            }
            "migrated-route-benchmark" | "migration-benchmark" | "migrated-benchmark" => {
                self.cmd_forge_migrated_route_benchmark(&args[1..])
            }
            "react-starter-benchmark" | "starter-benchmark" | "app-starter-benchmark" => {
                self.cmd_forge_react_starter_benchmark(&args[1..])
            }
            "template-readiness" | "www-template-readiness" | "verify-template-readiness" => {
                templates_command::cmd_templates_verify_readiness(&self.cwd, &args[1..])
            }
            "launch-readiness-bundle" | "readiness-bundle" => {
                self.cmd_forge_launch_readiness_bundle(&args[1..])
            }
            "launch-adoption-report" | "launch-adoption" => {
                self.cmd_forge_launch_adoption_report(&args[1..])
            }
            "launch-manifest-drift" | "manifest-drift" => {
                self.cmd_forge_launch_manifest_drift(&args[1..])
            }
            "launch-companion-receipts" | "companion-receipts" => {
                self.cmd_forge_launch_companion_receipts(&args[1..])
            }
            "launch-runtime-checklist" | "runtime-checklist" => {
                self.cmd_forge_launch_runtime_checklist(&args[1..])
            }
            "launch-runtime-approval-request" | "runtime-approval-request" => {
                launch_runtime_approval_request::run_launch_runtime_approval_request(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-runtime-evidence" | "runtime-evidence" => {
                launch_runtime_evidence::run_launch_runtime_evidence(&self.cwd, &args[1..])
            }
            "launch-runtime-evidence-import-plan" | "runtime-evidence-import-plan" => {
                launch_runtime_evidence_import_plan::run_launch_runtime_evidence_import_plan(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-runtime-evidence-completeness" | "runtime-evidence-completeness" => {
                launch_runtime_evidence_completeness::run_launch_runtime_evidence_completeness(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-runtime-evidence-finalize"
            | "runtime-evidence-finalize"
            | "launch-runtime-evidence-finalization"
            | "runtime-evidence-finalization" => {
                launch_runtime_evidence_finalization::run_launch_runtime_evidence_finalization(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-runtime-evidence-review" | "runtime-evidence-review" => {
                launch_runtime_evidence_review::run_launch_runtime_evidence_review(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-packet" | "evidence-packet" => {
                launch_evidence_packet::run_launch_evidence_packet(&self.cwd, &args[1..])
            }
            "launch-evidence-operator-index" | "evidence-operator-index" | "operator-index" => {
                launch_evidence_operator_index::run_launch_evidence_operator_index(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-status-timeline" | "evidence-status-timeline" | "status-timeline" => {
                launch_evidence_status_timeline::run_launch_evidence_status_timeline(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-handoff-digest" | "evidence-handoff-digest" | "handoff-digest" => {
                launch_evidence_handoff_digest::run_launch_evidence_handoff_digest(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-release-checklist"
            | "evidence-release-checklist"
            | "release-checklist" => {
                launch_evidence_release_checklist::run_launch_evidence_release_checklist(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-share-manifest" | "evidence-share-manifest" | "share-manifest" => {
                launch_evidence_share_manifest::run_launch_evidence_share_manifest(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-archive-index" | "evidence-archive-index" | "archive-index" => {
                launch_evidence_archive_index::run_launch_evidence_archive_index(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-archive-receipt" | "evidence-archive-receipt" | "archive-receipt" => {
                launch_evidence_archive_receipt::run_launch_evidence_archive_receipt(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-archive-ledger" | "evidence-archive-ledger" | "archive-ledger" => {
                launch_evidence_archive_ledger::run_launch_evidence_archive_ledger(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-retention-policy"
            | "evidence-retention-policy"
            | "retention-policy" => {
                launch_evidence_retention_policy::run_launch_evidence_retention_policy(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-retention-review"
            | "evidence-retention-review"
            | "retention-review" => {
                launch_evidence_retention_review::run_launch_evidence_retention_review(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-release-seal" | "evidence-release-seal" | "release-seal" => {
                launch_evidence_release_seal::run_launch_evidence_release_seal(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-operator-summary"
            | "evidence-operator-summary"
            | "operator-summary" => {
                launch_evidence_operator_summary::run_launch_evidence_operator_summary(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-completion-ledger"
            | "evidence-completion-ledger"
            | "completion-ledger" => {
                launch_evidence_completion_ledger::run_launch_evidence_completion_ledger(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-closure-memo" | "evidence-closure-memo" | "closure-memo" => {
                launch_evidence_closure_memo::run_launch_evidence_closure_memo(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-final-brief" | "evidence-final-brief" | "final-brief" => {
                launch_evidence_final_brief::run_launch_evidence_final_brief(&self.cwd, &args[1..])
            }
            "launch-evidence-operator-runbook"
            | "evidence-operator-runbook"
            | "operator-runbook" => {
                launch_evidence_operator_runbook::run_launch_evidence_operator_runbook(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-handoff-capsule" | "evidence-handoff-capsule" | "handoff-capsule" => {
                launch_evidence_handoff_capsule::run_launch_evidence_handoff_capsule(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-resumption-index"
            | "evidence-resumption-index"
            | "resumption-index" => {
                launch_evidence_resumption_index::run_launch_evidence_resumption_index(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-recovery-brief" | "evidence-recovery-brief" | "recovery-brief" => {
                launch_evidence_recovery_brief::run_launch_evidence_recovery_brief(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-continuation-packet"
            | "evidence-continuation-packet"
            | "continuation-packet" => {
                launch_evidence_continuation_packet::run_launch_evidence_continuation_packet(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-operator-resume-card"
            | "evidence-operator-resume-card"
            | "operator-resume-card"
            | "resume-card" => {
                launch_evidence_operator_resume_card::run_launch_evidence_operator_resume_card(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-restart-ledger" | "evidence-restart-ledger" | "restart-ledger" => {
                launch_evidence_restart_ledger::run_launch_evidence_restart_ledger(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-restart-checklist"
            | "evidence-restart-checklist"
            | "restart-checklist" => {
                launch_evidence_restart_checklist::run_launch_evidence_restart_checklist(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-restart-brief" | "evidence-restart-brief" | "restart-brief" => {
                launch_evidence_restart_brief::run_launch_evidence_restart_brief(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-restart-manifest"
            | "evidence-restart-manifest"
            | "restart-manifest" => {
                launch_evidence_restart_manifest::run_launch_evidence_restart_manifest(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-restart-receipt" | "evidence-restart-receipt" | "restart-receipt" => {
                launch_evidence_restart_receipt::run_launch_evidence_restart_receipt(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-restart-summary" | "evidence-restart-summary" | "restart-summary" => {
                launch_evidence_restart_summary::run_launch_evidence_restart_summary(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-restart-snapshot"
            | "evidence-restart-snapshot"
            | "restart-snapshot" => {
                launch_evidence_restart_snapshot::run_launch_evidence_restart_snapshot(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-restart-dispatch"
            | "evidence-restart-dispatch"
            | "restart-dispatch" => {
                launch_evidence_restart_dispatch::run_launch_evidence_restart_dispatch(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-restart-closeout"
            | "evidence-restart-closeout"
            | "restart-closeout" => {
                launch_evidence_restart_closeout::run_launch_evidence_restart_closeout(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-restart-signoff" | "evidence-restart-signoff" | "restart-signoff" => {
                launch_evidence_restart_signoff::run_launch_evidence_restart_signoff(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-acceptance-index"
            | "evidence-acceptance-index"
            | "acceptance-index" => {
                launch_evidence_acceptance_index::run_launch_evidence_acceptance_index(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-acceptance-digest"
            | "evidence-acceptance-digest"
            | "acceptance-digest" => {
                launch_evidence_acceptance_digest::run_launch_evidence_acceptance_digest(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-evidence-friday-baton" | "evidence-friday-baton" | "friday-baton" => {
                launch_evidence_friday_baton::run_launch_evidence_friday_baton(
                    &self.cwd,
                    &args[1..],
                )
            }
            "launch-verification-lane" | "verification-lane" => {
                launch_verification_lane::run_launch_verification_lane(&self.cwd, &args[1..])
            }
            "migration-guide" => self.cmd_forge_migration_guide(&args[1..]),
            "operator-dashboard" | "beta-dashboard" | "review-dashboard" => {
                self.cmd_forge_operator_dashboard(&args[1..])
            }
            "package-gallery" => self.cmd_forge_package_gallery(&args[1..]),
            "packages" | "package-catalog" => {
                forge_packages_command::run_forge_packages(&self.cwd, &args[1..])
            }
            #[rustfmt::skip]
            "provenance" => {
                forge_provenance_command::run_forge_provenance(&self.cwd, &args[1..])
            },
            "public-evidence" => run_forge_public_evidence(&self.cwd, &args[1..]),
            "publish" => self.cmd_forge_public_publish(&args[1..]),
            "publish-plan" => self.cmd_forge_publish_plan(&args[1..]),
            "publisher-key" | "publisher" => {
                forge_publisher_key_command::run_forge_publisher_key(&self.cwd, &args[1..])
            }
            "receipts" => run_forge_public_receipts(&self.cwd, &args[1..]),
            "remote" => run_forge_public_remote(&self.cwd, &args[1..]),
            "remote-head" | "remote-head-health" | "r2-head" => {
                self.cmd_forge_remote_head(&args[1..])
            }
            "remotes" => run_forge_public_remotes(&self.cwd, &args[1..]),
            "release-history" => run_forge_release_history(&self.cwd, &args[1..]),
            "release-trend" => self.cmd_forge_release_trend(&args[1..]),
            "launch-changelog" => self.cmd_forge_launch_changelog(&args[1..]),
            "release-notes" => self.cmd_forge_release_notes(&args[1..]),
            "release-dashboard" => {
                forge_release_dashboard_command::run_forge_release_dashboard(&self.cwd, &args[1..])
            }
            "release-candidate" => {
                forge_release_candidate_command::run_forge_release_candidate(&self.cwd, &args[1..])
            }
            "release-bundle" => self.cmd_forge_release_bundle(&args[1..]),
            "release-bundle-inspect" | "bundle-inspect" | "inspect-bundle" => {
                self.cmd_forge_release_bundle_inspect(&args[1..])
            }
            "release-triage" | "shipping-triage" | "operator-triage" => {
                self.cmd_forge_release_triage(&args[1..])
            }
            "release-operations" | "operations" => self.cmd_forge_release_operations(&args[1..]),
            "release-review" => self.cmd_forge_release_review(&args[1..]),
            "trust-policy" => {
                forge_trust_policy_command::run_forge_trust_policy(&self.cwd, &args[1..])
            }
            "trust-regression" => {
                forge_trust_regression_command::run_forge_trust_regression(&self.cwd, &args[1..])
            }
            "smoke" => self.cmd_forge_smoke(&args[1..]),
            "status" => run_forge_public_status(&self.cwd, &args[1..]),
            "validate" => run_forge_doctor(&self.cwd, &args[1..]),
            "verify-package" => self.cmd_forge_verify_package(&args[1..]),
            "scorecard" => self.cmd_forge_scorecard(&args[1..]),
            "ui" => match args.get(1).map(String::as_str) {
                None => {
                    print_forge_ui_help();
                    Ok(())
                }
                Some("--help" | "-h" | "help") => {
                    print_forge_ui_help();
                    Ok(())
                }
                Some("parity") => {
                    forge_ui_registry_parity::run_forge_ui_registry_parity(&self.cwd, &args[2..])
                }
                Some(command) => Err(DxError::ConfigValidationError {
                    message: format!("Unknown forge ui command `{command}`. Expected: parity"),
                    field: Some("forge ui".to_string()),
                }),
            },
            "ui-parity" | "registry-parity" => {
                forge_ui_registry_parity::run_forge_ui_registry_parity(&self.cwd, &args[1..])
            }
            "update" => self.cmd_forge_update(&args[1..]),
            "remove" => self.cmd_forge_remove(&args[1..]),
            "rollback" => self.cmd_forge_rollback(&args[1..]),
            "registry" => self.cmd_forge_registry(&args[1..]),
            command => Err(DxError::ConfigValidationError {
                message: forge_unknown_command_message(command),
                field: Some("forge".to_string()),
            }),
        }
    }

    fn cmd_forge_acquire(&self, args: &[String]) -> DxResult<()> {
        if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
            eprintln!(
                "Usage: dx forge acquire <{}> <package> [--version <version>]",
                DX_FORGE_IMPORT_ECOSYSTEMS_HELP
            );
            eprintln!("       dx forge acquire npm three --json");
            eprintln!("       compatibility alias: dx forge add npm three --json");
            eprintln!(
                "       [--registry-url <url>] [--project <path>] [--output <path>] [--format terminal|json|markdown] [--json] [--fail-under <score>] [--quiet]"
            );
            eprintln!(
                "       Acquisition fetches registry metadata and package archives into .dx/cache without package-manager installs, node_modules, or lifecycle scripts."
            );
            eprintln!(
                "       npm live acquisition is supported first; other ecosystems remain modeled by dx forge import until their registry fetchers are implemented."
            );
            return Ok(());
        }

        let options = parse_forge_acquire_options(&self.cwd, args)?;
        let report = match options.ecosystem.as_str() {
            "npm" => acquire_npm_package(&options).map_err(forge_error)?,
            ecosystem => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "dx forge acquire {ecosystem} is not live yet; use dx forge import {ecosystem} <package> --plan with reviewed source-dir evidence"
                    ),
                    field: Some("forge acquire".to_string()),
                });
            }
        };
        let rendered = match options.format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Terminal => forge_acquire_terminal(&report),
            DxOutputFormat::Markdown => forge_acquire_markdown(&report),
        };

        if let Some(output) = options.output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        if !options.quiet {
            println!("{rendered}");
        }

        Ok(())
    }

    fn cmd_forge_import(&self, args: &[String]) -> DxResult<()> {
        if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
            eprintln!(
                "Usage: dx forge review <{}> <package> --plan|--write",
                DX_FORGE_IMPORT_ECOSYSTEMS_HELP
            );
            eprintln!(
                "       compatibility alias: dx forge import <{}> <package> --plan|--write",
                DX_FORGE_IMPORT_ECOSYSTEMS_HELP
            );
            eprintln!("       aliases: {}", DX_FORGE_IMPORT_ECOSYSTEM_ALIASES_HELP);
            eprintln!(
                "       [--source-dir <path>] [--file <package-path>] [--from-plan <path>] [--project <path>]"
            );
            eprintln!(
                "       [--output <path>] [--format terminal|json|markdown] [--json] [--fail-under <score>] [--quiet]"
            );
            eprintln!(
                "       Import is a Forge review gate; it does not run package-manager installs or lifecycle/setup/build scripts."
            );
            eprintln!(
                "       Ecosystem support means modeled review surfaces, not universal or live package-manager compatibility."
            );
            eprintln!("       Ecosystem aliases normalize into canonical Forge receipt paths.");
            eprintln!(
                "       Package names are validated per ecosystem before they become package ids, receipt paths, or materialized source paths."
            );
            eprintln!(
                "       --plan writes evidence receipts only; no app source, node_modules, or package code is created."
            );
            eprintln!("       Outcomes are materialize, slice, bridge, or reject.");
            eprintln!(
                "       --write materializes inspected source directories into source-owned Forge files after --from-plan validates the accepted import plan; reject mode never overwrites different local source."
            );
            eprintln!("       --from-plan is required for reviewed source materialization.");
            eprintln!(
                "       Accepted source snapshots are written under lib/forge/<ecosystem>/<package>/; clean package-name imports require a compatible reviewed adapter or bridge."
            );
            eprintln!(
                "       --file may be repeated or comma-separated to materialize a reviewed package-relative source slice."
            );
            eprintln!(
                "       Bridge requires adapter/manual wrapper evidence before app code can depend on the package boundary."
            );
            return Ok(());
        }

        let options = parse_forge_import_options(&self.cwd, args)?;
        if let Some(output) = options.output.as_ref() {
            preflight_forge_import_output(output).map_err(forge_error)?;
        }
        let report = if options.write {
            build_forge_import_write_report(
                &options.project,
                &options.ecosystem,
                &options.package_name,
                options.source_dir.as_deref(),
                &options.selected_files,
                options.accepted_plan.as_deref(),
                options.fail_under,
            )
        } else if options.plan {
            build_forge_import_plan_report_with_selection(
                &options.project,
                &options.ecosystem,
                &options.package_name,
                options.source_dir.as_deref(),
                &options.selected_files,
                options.fail_under,
            )
        } else {
            unreachable!("forge import parser requires plan or write mode")
        }
        .map_err(forge_error)?;
        let rendered = match options.format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Terminal => forge_import_plan_terminal(&report),
            DxOutputFormat::Markdown => forge_import_plan_markdown(&report),
        };

        if let Some(output) = options.output {
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).map_err(forge_error)?;
            }
            std::fs::write(&output, &rendered).map_err(forge_error)?;
        }

        if !options.quiet {
            println!("{rendered}");
        }

        if options.write && !report.passed {
            return Err(DxError::ConfigValidationError {
                message: forge_import_plan_failure_summary(&report),
                field: Some("forge import".to_string()),
            });
        }

        if options.plan && options.fail_under_explicit && !report.passed {
            return Err(DxError::ConfigValidationError {
                message: forge_import_plan_failure_summary(&report),
                field: Some("forge import".to_string()),
            });
        }

        if options.write && report.score < options.fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge import score {} is below fail-under threshold {}",
                    report.score, options.fail_under
                ),
                field: Some("forge import".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_forge_launch_copy_review(&self, args: &[String]) -> DxResult<()> {
        run_forge_launch_copy_review_command(&self.cwd, args)
    }

    fn cmd_forge_launch_page(&self, args: &[String]) -> DxResult<()> {
        run_forge_launch_page_command(&self.cwd, args)
    }

    fn cmd_forge_docs(&self, args: &[String]) -> DxResult<()> {
        let mut project: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut write = false;
        let mut dry_run = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index].as_str() {
                "--project" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--project requires a value".to_string(),
                                field: Some("project".to_string()),
                            })?;
                    project = Some(resolve_cli_path(&self.cwd, value));
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
                "--write" => {
                    write = true;
                    index += 1;
                }
                "--dry-run" => {
                    dry_run = true;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge docs option: {value}"),
                        field: Some("forge docs".to_string()),
                    });
                }
                value => {
                    if project.is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: format!("Unexpected forge docs path: {value}"),
                            field: Some("project".to_string()),
                        });
                    }
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 1;
                }
            }
        }

        if write && dry_run {
            return Err(DxError::ConfigValidationError {
                message: "Choose either --dry-run or --write, not both".to_string(),
                field: Some("forge docs".to_string()),
            });
        }

        let project = project.unwrap_or_else(|| self.cwd.clone());
        let outcome = if write {
            write_forge_docs(&project).map_err(forge_error)?
        } else {
            plan_forge_docs(&project).map_err(forge_error)?
        };

        match format {
            DxOutputFormat::Terminal | DxOutputFormat::Markdown => {
                println!("{}", forge_docs_outcome_markdown(&outcome));
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

    fn cmd_forge_verify_package(&self, args: &[String]) -> DxResult<()> {
        let mut package_id: Option<String> = None;
        let mut all = false;
        let mut project: Option<PathBuf> = None;
        let mut variant = "default".to_string();
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under: Option<u8> = None;
        let mut index = 0usize;

        while index < args.len() {
            match args[index].as_str() {
                "--all" => {
                    all = true;
                    index += 1;
                }
                "--project" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--project requires a path".to_string(),
                                field: Some("project".to_string()),
                            })?;
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--variant" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--variant requires a value".to_string(),
                                field: Some("variant".to_string()),
                            })?;
                    variant = value.to_string();
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
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("fail-under".to_string()),
                            })?;
                    fail_under = Some(parse_score_threshold(value)?);
                    index += 2;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge verify-package option: {value}"),
                        field: Some("forge verify-package".to_string()),
                    });
                }
                value => {
                    if package_id.replace(value.to_string()).is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: "forge verify-package accepts one package id at a time"
                                .to_string(),
                            field: Some("forge verify-package".to_string()),
                        });
                    }
                    index += 1;
                }
            }
        }

        let project = project.unwrap_or_else(|| self.cwd.clone());
        if all {
            if package_id.is_some() {
                return Err(DxError::ConfigValidationError {
                    message: "Use either a package id or --all, not both".to_string(),
                    field: Some("forge verify-package".to_string()),
                });
            }
            let report = build_forge_verify_all_packages_report(&project).map_err(forge_error)?;

            match format {
                DxOutputFormat::Terminal => print_forge_verify_all_packages_terminal(&report),
                DxOutputFormat::Json => println!(
                    "{}",
                    serde_json::to_string_pretty(&report).map_err(forge_error)?
                ),
                DxOutputFormat::Markdown => {
                    println!("{}", forge_verify_all_packages_markdown(&report))
                }
            }

            if let Some(threshold) = fail_under {
                if report.score < threshold {
                    return Err(DxError::ConfigValidationError {
                        message: format!(
                            "DX Forge verify-package --all score {} is below fail-under threshold {}",
                            report.score, threshold
                        ),
                        field: Some("forge verify-package".to_string()),
                    });
                }
            }

            if !report.passed {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "DX Forge verify-package --all failed: packages={}, missing={}",
                        report.packages.len(),
                        report.missing_packages.len()
                    ),
                    field: Some("forge verify-package".to_string()),
                });
            }

            return Ok(());
        }

        let package_id = package_id.ok_or_else(|| DxError::ConfigValidationError {
            message: "forge verify-package requires a package id or --all".to_string(),
            field: Some("forge verify-package".to_string()),
        })?;
        let report = build_forge_verify_package_report(&project, &package_id, &variant)
            .map_err(forge_error)?;

        match format {
            DxOutputFormat::Terminal => print_forge_verify_package_terminal(&report),
            DxOutputFormat::Json => println!(
                "{}",
                serde_json::to_string_pretty(&report).map_err(forge_error)?
            ),
            DxOutputFormat::Markdown => println!("{}", forge_verify_package_markdown(&report)),
        }

        if let Some(threshold) = fail_under {
            if report.score < threshold {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "DX Forge verify-package score {} is below fail-under threshold {}",
                        report.score, threshold
                    ),
                    field: Some("forge verify-package".to_string()),
                });
            }
        }

        if !report.passed {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge verify-package failed for `{}`: {}",
                    report.package_id,
                    forge_verify_package_failure_summary(&report)
                ),
                field: Some("forge verify-package".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_forge_scorecard(&self, args: &[String]) -> DxResult<()> {
        let mut project: Option<PathBuf> = None;
        let mut history: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut output: Option<PathBuf> = None;
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
                    project = Some(resolve_cli_path(&self.cwd, value));
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
                "--history" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--history requires a value".to_string(),
                                field: Some("history".to_string()),
                            })?;
                    history = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--output" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--output requires a path".to_string(),
                                field: Some("output".to_string()),
                            })?;
                    output = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge scorecard option: {value}"),
                        field: Some("forge scorecard".to_string()),
                    });
                }
                value => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unexpected forge scorecard path: {value}"),
                        field: Some("forge scorecard".to_string()),
                    });
                }
            }
        }

        let history_root = project.clone().unwrap_or_else(|| self.cwd.clone());
        let report = if let Some(project) = project {
            build_forge_package_scorecard_for_project(&project).map_err(forge_error)?
        } else {
            build_forge_package_scorecard().map_err(forge_error)?
        };
        let benchmark_history_path = history.unwrap_or_else(|| {
            self.cwd
                .join("benchmarks/reports/vertical-proof-history/index.json")
        });
        let latest_forge_route_benchmark =
            load_latest_forge_route_benchmark_snapshot(&benchmark_history_path)
                .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal | DxOutputFormat::Markdown => {
                forge_scorecard_cli_markdown(&report, latest_forge_route_benchmark.as_ref())
            }
            DxOutputFormat::Json => serde_json::to_string_pretty(&DxForgeScorecardCliReport {
                scorecard: report.clone(),
                latest_forge_route_benchmark,
            })
            .map_err(forge_error)?,
        };

        if let Some(path) = output {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
                    path: Some(parent.to_path_buf()),
                    message: error.to_string(),
                })?;
            }
            std::fs::write(&path, rendered).map_err(|error| DxError::IoError {
                path: Some(path),
                message: error.to_string(),
            })?;
            write_forge_package_scorecard_history(&history_root, &report).map_err(forge_error)?;
        } else {
            println!("{rendered}");
        }

        Ok(())
    }

    fn cmd_forge_migration_audit(&self, args: &[String]) -> DxResult<()> {
        let mut project: Option<PathBuf> = None;
        let mut input: Option<PathBuf> = None;
        let mut output: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under = 70u8;
        let mut quiet = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index].as_str() {
                "--project" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--project requires a value".to_string(),
                                field: Some("project".to_string()),
                            })?;
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--input" | "--source" | "--export" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--input requires a file or directory".to_string(),
                                field: Some("input".to_string()),
                            })?;
                    input = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--output" | "--out" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--output requires a value".to_string(),
                                field: Some("output".to_string()),
                            })?;
                    output = Some(resolve_cli_path(&self.cwd, value));
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
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("fail-under".to_string()),
                            })?;
                    fail_under = parse_score_threshold(value)?;
                    index += 2;
                }
                "--quiet" => {
                    quiet = true;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge migration-audit option: {value}"),
                        field: Some("forge migration-audit".to_string()),
                    });
                }
                value => {
                    if input.is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: format!("Unexpected forge migration-audit path: {value}"),
                            field: Some("input".to_string()),
                        });
                    }
                    input = Some(resolve_cli_path(&self.cwd, value));
                    index += 1;
                }
            }
        }

        let project = project.unwrap_or_else(|| self.cwd.clone());
        let input = input.ok_or_else(|| DxError::ConfigValidationError {
            message: "dx forge migration-audit requires --input <file-or-dir>".to_string(),
            field: Some("input".to_string()),
        })?;
        let report = build_forge_migration_audit_report(&project, &input, fail_under)
            .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Terminal => forge_migration_audit_terminal(&report),
            DxOutputFormat::Markdown => forge_migration_audit_markdown(&report),
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
                    "DX Forge migration-audit score {} is below fail-under threshold {}",
                    report.score, fail_under
                ),
                field: Some("forge migration-audit".to_string()),
            });
        }

        if !report.passed {
            return Err(DxError::ConfigValidationError {
                message: forge_migration_audit_failure_summary(&report),
                field: Some("forge migration-audit".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_forge_migrate_static_page(&self, args: &[String]) -> DxResult<()> {
        let mut project: Option<PathBuf> = None;
        let mut input: Option<PathBuf> = None;
        let mut route: Option<String> = None;
        let mut unsafe_html_review: Option<String> = None;
        let mut output: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under = 90u8;
        let mut write = false;
        let mut dry_run = false;
        let mut quiet = false;
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
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--input" | "--source" | "--export" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--input requires a file or directory".to_string(),
                                field: Some("input".to_string()),
                            })?;
                    input = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--route" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--route requires a route path".to_string(),
                                field: Some("route".to_string()),
                            })?;
                    route = Some(value.clone());
                    index += 2;
                }
                "--unsafe-html-review" | "--unsafe-review" | "--manual-review-decision" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--unsafe-html-review requires a review decision"
                                    .to_string(),
                                field: Some("unsafe-html-review".to_string()),
                            })?;
                    unsafe_html_review = Some(value.clone());
                    index += 2;
                }
                "--output" | "--out" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--output requires a path".to_string(),
                                field: Some("output".to_string()),
                            })?;
                    output = Some(resolve_cli_path(&self.cwd, value));
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
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("fail-under".to_string()),
                            })?;
                    fail_under = parse_score_threshold(value)?;
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
                "--quiet" => {
                    quiet = true;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge migrate-static-page option: {value}"),
                        field: Some("forge migrate-static-page".to_string()),
                    });
                }
                value => {
                    if input.is_none() {
                        input = Some(resolve_cli_path(&self.cwd, value));
                    } else if route.is_none() {
                        route = Some(value.to_string());
                    } else {
                        return Err(DxError::ConfigValidationError {
                            message: format!(
                                "Unexpected forge migrate-static-page argument: {value}"
                            ),
                            field: Some("forge migrate-static-page".to_string()),
                        });
                    }
                    index += 1;
                }
            }
        }

        if write && dry_run {
            return Err(DxError::ConfigValidationError {
                message: "Choose either --dry-run or --write, not both".to_string(),
                field: Some("forge migrate-static-page".to_string()),
            });
        }

        let project = project.unwrap_or_else(|| self.cwd.clone());
        let input = input.ok_or_else(|| DxError::ConfigValidationError {
            message: "dx forge migrate-static-page requires --input <html-or-dir>".to_string(),
            field: Some("input".to_string()),
        })?;
        let route = route.ok_or_else(|| DxError::ConfigValidationError {
            message: "dx forge migrate-static-page requires --route <route>".to_string(),
            field: Some("route".to_string()),
        })?;
        let report = build_forge_static_page_migration_report(
            &project,
            &input,
            &route,
            write,
            unsafe_html_review.as_deref(),
            fail_under,
        )
        .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Terminal => forge_static_page_migration_terminal(&report),
            DxOutputFormat::Markdown => forge_static_page_migration_markdown(&report),
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

        if !report.passed {
            return Err(DxError::ConfigValidationError {
                message: forge_static_page_migration_failure_summary(&report),
                field: Some("forge migrate-static-page".to_string()),
            });
        }

        if report.score < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge migrate-static-page score {} is below fail-under threshold {}",
                    report.score, fail_under
                ),
                field: Some("forge migrate-static-page".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_forge_static_migration_plan(&self, args: &[String]) -> DxResult<()> {
        let mut project: Option<PathBuf> = None;
        let mut input: Option<PathBuf> = None;
        let mut route_prefix = "/migrated".to_string();
        let mut output: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under = 90u8;
        let mut quiet = false;
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
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--input" | "--source" | "--export" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--input requires a file or directory".to_string(),
                                field: Some("input".to_string()),
                            })?;
                    input = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--route-prefix" | "--prefix" | "--base-route" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--route-prefix requires a route path".to_string(),
                                field: Some("route-prefix".to_string()),
                            })?;
                    route_prefix = value.clone();
                    index += 2;
                }
                "--output" | "--out" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--output requires a path".to_string(),
                                field: Some("output".to_string()),
                            })?;
                    output = Some(resolve_cli_path(&self.cwd, value));
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
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("fail-under".to_string()),
                            })?;
                    fail_under = parse_score_threshold(value)?;
                    index += 2;
                }
                "--quiet" => {
                    quiet = true;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge static-migration-plan option: {value}"),
                        field: Some("forge static-migration-plan".to_string()),
                    });
                }
                value => {
                    if input.is_none() {
                        input = Some(resolve_cli_path(&self.cwd, value));
                    } else {
                        return Err(DxError::ConfigValidationError {
                            message: format!(
                                "Unexpected forge static-migration-plan argument: {value}"
                            ),
                            field: Some("forge static-migration-plan".to_string()),
                        });
                    }
                    index += 1;
                }
            }
        }

        let project = project.unwrap_or_else(|| self.cwd.clone());
        let input = input.ok_or_else(|| DxError::ConfigValidationError {
            message: "dx forge static-migration-plan requires --input <html-or-dir>".to_string(),
            field: Some("input".to_string()),
        })?;
        let report =
            build_forge_static_migration_plan_report(&project, &input, &route_prefix, fail_under)
                .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Terminal => forge_static_migration_plan_terminal(&report),
            DxOutputFormat::Markdown => forge_static_migration_plan_markdown(&report),
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

        if !report.passed {
            return Err(DxError::ConfigValidationError {
                message: forge_static_migration_plan_failure_summary(&report),
                field: Some("forge static-migration-plan".to_string()),
            });
        }

        if report.score < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge static-migration-plan score {} is below fail-under threshold {}",
                    report.score, fail_under
                ),
                field: Some("forge static-migration-plan".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_forge_static_migration_smoke(&self, args: &[String]) -> DxResult<()> {
        let mut project: Option<PathBuf> = None;
        let mut artifact_dir: Option<PathBuf> = None;
        let mut output: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under = 90u8;
        let mut quiet = false;
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
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--artifacts" | "--artifact-dir" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--artifacts requires a directory".to_string(),
                                field: Some("artifacts".to_string()),
                            })?;
                    artifact_dir = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--output" | "--out" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--output requires a path".to_string(),
                                field: Some("output".to_string()),
                            })?;
                    output = Some(resolve_cli_path(&self.cwd, value));
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
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("fail-under".to_string()),
                            })?;
                    fail_under = parse_score_threshold(value)?;
                    index += 2;
                }
                "--quiet" => {
                    quiet = true;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge static-migration-smoke option: {value}"),
                        field: Some("forge static-migration-smoke".to_string()),
                    });
                }
                value => {
                    if project.is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: format!(
                                "Unexpected forge static-migration-smoke path: {value}"
                            ),
                            field: Some("project".to_string()),
                        });
                    }
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 1;
                }
            }
        }

        let project = project.unwrap_or_else(|| self.cwd.clone());
        let artifact_dir =
            artifact_dir.unwrap_or_else(|| project.join(".dx/forge/static-migration-smoke"));
        let report = build_forge_static_migration_smoke_report(&project, &artifact_dir, fail_under)
            .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Terminal => forge_static_migration_smoke_terminal(&report),
            DxOutputFormat::Markdown => forge_static_migration_smoke_markdown(&report),
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

        if !report.passed {
            return Err(DxError::ConfigValidationError {
                message: forge_static_migration_smoke_failure_summary(&report),
                field: Some("forge static-migration-smoke".to_string()),
            });
        }

        if report.score < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge static-migration-smoke score {} is below fail-under threshold {}",
                    report.score, fail_under
                ),
                field: Some("forge static-migration-smoke".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_forge_materialize_static_assets(&self, args: &[String]) -> DxResult<()> {
        run_forge_materialize_static_assets_command(&self.cwd, args)
    }

    fn cmd_forge_migrated_route_benchmark(&self, args: &[String]) -> DxResult<()> {
        let mut project: Option<PathBuf> = None;
        let mut output: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under = 90u8;
        let mut quiet = false;
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
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--output" | "--out" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--output requires a path".to_string(),
                                field: Some("output".to_string()),
                            })?;
                    output = Some(resolve_cli_path(&self.cwd, value));
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
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("fail-under".to_string()),
                            })?;
                    fail_under = parse_score_threshold(value)?;
                    index += 2;
                }
                "--quiet" => {
                    quiet = true;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge migrated-route-benchmark option: {value}"),
                        field: Some("forge migrated-route-benchmark".to_string()),
                    });
                }
                value => {
                    if project.is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: format!(
                                "Unexpected forge migrated-route-benchmark path: {value}"
                            ),
                            field: Some("project".to_string()),
                        });
                    }
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 1;
                }
            }
        }

        let project = project.unwrap_or_else(|| self.cwd.clone());
        let report = build_forge_migrated_route_benchmark_report(&project, fail_under)
            .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Terminal => forge_migrated_route_benchmark_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_migrated_route_benchmark_markdown(&report),
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
                    "DX Forge migrated-route-benchmark score {} is below fail-under threshold {}",
                    report.score, fail_under
                ),
                field: Some("forge migrated-route-benchmark".to_string()),
            });
        }

        if !report.passed {
            return Err(DxError::ConfigValidationError {
                message: forge_migrated_route_benchmark_failure_summary(&report),
                field: Some("forge migrated-route-benchmark".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_forge_react_starter_benchmark(&self, args: &[String]) -> DxResult<()> {
        run_forge_react_starter_benchmark_command(&self.cwd, args)
    }

    fn cmd_forge_migration_guide(&self, args: &[String]) -> DxResult<()> {
        let mut project: Option<PathBuf> = None;
        let mut package_id = "ui/button".to_string();
        let mut output: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under = 90u8;
        let mut quiet = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index].as_str() {
                "--project" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--project requires a value".to_string(),
                                field: Some("project".to_string()),
                            })?;
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--package" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--package requires a package id".to_string(),
                                field: Some("package".to_string()),
                            })?;
                    package_id = value.to_string();
                    index += 2;
                }
                "--output" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--output requires a value".to_string(),
                                field: Some("output".to_string()),
                            })?;
                    output = Some(resolve_cli_path(&self.cwd, value));
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
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("fail-under".to_string()),
                            })?;
                    fail_under = parse_score_threshold(value)?;
                    index += 2;
                }
                "--quiet" => {
                    quiet = true;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge migration-guide option: {value}"),
                        field: Some("forge migration-guide".to_string()),
                    });
                }
                value => {
                    if project.is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: format!("Unexpected forge migration-guide path: {value}"),
                            field: Some("project".to_string()),
                        });
                    }
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 1;
                }
            }
        }

        let project = project.unwrap_or_else(|| self.cwd.clone());
        let report = build_forge_migration_guide_report(&project, &package_id, fail_under)
            .map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Terminal => forge_migration_guide_terminal(&report),
            DxOutputFormat::Markdown => forge_migration_guide_markdown(&report),
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
                    "DX Forge migration-guide score {} is below fail-under threshold {}",
                    report.score, fail_under
                ),
                field: Some("forge migration-guide".to_string()),
            });
        }

        if !report.passed {
            return Err(DxError::ConfigValidationError {
                message: forge_migration_guide_failure_summary(&report),
                field: Some("forge migration-guide".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_forge_beta_diagnostics(&self, args: &[String]) -> DxResult<()> {
        let mut project: Option<PathBuf> = None;
        let mut output: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under = 90u8;
        let mut quiet = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index].as_str() {
                "--project" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--project requires a value".to_string(),
                                field: Some("project".to_string()),
                            })?;
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--output" | "--out" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--output requires a value".to_string(),
                                field: Some("output".to_string()),
                            })?;
                    output = Some(resolve_cli_path(&self.cwd, value));
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
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("fail-under".to_string()),
                            })?;
                    fail_under = parse_score_threshold(value)?;
                    index += 2;
                }
                "--quiet" => {
                    quiet = true;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge beta-diagnostics option: {value}"),
                        field: Some("forge beta-diagnostics".to_string()),
                    });
                }
                value => {
                    if project.is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: format!("Unexpected forge beta-diagnostics path: {value}"),
                            field: Some("project".to_string()),
                        });
                    }
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 1;
                }
            }
        }

        let project = project.unwrap_or_else(|| self.cwd.clone());
        let report =
            build_forge_beta_diagnostics_report(&project, fail_under).map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Terminal => forge_beta_diagnostics_terminal(&report),
            DxOutputFormat::Markdown => forge_beta_diagnostics_markdown(&report),
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

        if report.score() < fail_under {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge beta-diagnostics score {} is below fail-under threshold {}",
                    report.score(),
                    fail_under
                ),
                field: Some("forge beta-diagnostics".to_string()),
            });
        }

        if !report.passed() {
            return Err(DxError::ConfigValidationError {
                message: forge_beta_diagnostics_failure_summary(&report),
                field: Some("forge beta-diagnostics".to_string()),
            });
        }

        Ok(())
    }

    fn cmd_forge_package_gallery(&self, args: &[String]) -> DxResult<()> {
        let mut project: Option<PathBuf> = None;
        let mut output: Option<PathBuf> = None;
        let mut public_index: Option<PathBuf> = None;
        let mut format = DxOutputFormat::Terminal;
        let mut fail_under = 90u8;
        let mut quiet = false;
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
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--output" | "--out" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--output requires a path".to_string(),
                                field: Some("output".to_string()),
                            })?;
                    output = Some(resolve_cli_path(&self.cwd, value));
                    index += 2;
                }
                "--public-index" | "--hosted-index" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--public-index requires an output directory".to_string(),
                                field: Some("public-index".to_string()),
                            })?;
                    public_index = Some(resolve_cli_path(&self.cwd, value));
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
                "--fail-under" => {
                    let value =
                        args.get(index + 1)
                            .ok_or_else(|| DxError::ConfigValidationError {
                                message: "--fail-under requires a score".to_string(),
                                field: Some("fail-under".to_string()),
                            })?;
                    fail_under = parse_score_threshold(value)?;
                    index += 2;
                }
                "--quiet" => {
                    quiet = true;
                    index += 1;
                }
                value if value.starts_with('-') => {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unknown forge package-gallery option: {value}"),
                        field: Some("forge package-gallery".to_string()),
                    });
                }
                value => {
                    if project.is_some() {
                        return Err(DxError::ConfigValidationError {
                            message: format!("Unexpected forge package-gallery path: {value}"),
                            field: Some("project".to_string()),
                        });
                    }
                    project = Some(resolve_cli_path(&self.cwd, value));
                    index += 1;
                }
            }
        }

        let project = project.unwrap_or_else(|| self.cwd.clone());
        let mut report =
            build_forge_package_gallery_report(&project, fail_under).map_err(forge_error)?;
        if let Some(public_index) = public_index {
            let hosted_index = write_forge_package_gallery_hosted_index(&public_index, &report)
                .map_err(forge_error)?;
            report.hosted_index = Some(hosted_index);
        }
        let rendered = match format {
            DxOutputFormat::Terminal => forge_package_gallery_terminal(&report),
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Markdown => forge_package_gallery_markdown(&report),
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
                    "DX Forge package gallery score {} is below fail-under threshold {}",
                    report.score, fail_under
                ),
                field: Some("forge package-gallery".to_string()),
            });
        }

        if !report.passed {
            return Err(DxError::ConfigValidationError {
                message: forge_package_gallery_failure_summary(&report),
                field: Some("forge package-gallery".to_string()),
            });
        }

        Ok(())
    }
}

fn preflight_forge_import_output(output: &std::path::Path) -> anyhow::Result<()> {
    if output.is_dir() {
        anyhow::bail!(
            "Forge import output `{}` is a directory; choose a writable report file path",
            output.display()
        );
    }
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent).map_err(|error| {
            anyhow::anyhow!(
                "create Forge import output directory `{}`: {error}",
                parent.display()
            )
        })?;
        if !parent.is_dir() {
            anyhow::bail!(
                "Forge import output parent `{}` is not a directory",
                parent.display()
            );
        }
    }
    if output.exists() {
        std::fs::OpenOptions::new()
            .write(true)
            .open(output)
            .map_err(|error| {
                anyhow::anyhow!(
                    "open Forge import output `{}` for writing: {error}",
                    output.display()
                )
            })?;
    }
    Ok(())
}
