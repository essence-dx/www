//! # CLI Commands
//!
//! This module provides CLI command handlers for the DX WWW Framework.
//!
//! Commands:
//! - `dx new <name>` - Create a new project
//! - `dx create <name>` - Alias for `dx new`
//! - `dx dev` - Start development server
//! - `dx preview --production-contract` - Serve build output through deploy contract
//! - `dx build` - Build for production
//! - `dx style build|watch|check` - Generate and verify dx-style CSS
//! - `dx icons sync|check` - Generate and verify source-owned icon wrappers
//! - `dx imports sync|check` - Generate and verify readable import maps
//! - `dx deploy vercel` - Write the static Vercel deploy manifest
//! - `dx generate <type> <name>` - Generate new files
//! - `dx add <package-or-component>` - Add a source-owned package or component
//! - `dx forge remove <package>` - Archive and remove a source-owned Forge package

#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};
use chrono::Utc;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

// Component library
use dx_compiler::components::{ComponentDef, get_all_components, get_component};
use dx_compiler::delivery::{
    DxReactComponentSource, DxReactForgeOwnedFile, DxReactImportAlias, DxReactImportResolverConfig,
    DxReactResolvedImport, DxReactReviewedAdapter, DxReactRouteHandlerRequest, DxReactServerSource,
    DxReactServerSourceKind, DxReactStyleSource, execute_react_route_handler,
    resolve_react_imports,
};
use dx_compiler::ecosystem::{
    DX_FORGE_IMPORT_ECOSYSTEM_ALIASES_HELP, DX_FORGE_IMPORT_ECOSYSTEMS_HELP, DxCheckFinding,
    DxCheckLatestPanelReport, DxCheckMetric, DxCheckOptions, DxCheckReport, DxCheckSection,
    DxForgeAddOutcome, DxForgeAuditReport, DxForgeFileTransaction, DxForgeImportEcosystem,
    DxForgeLocalSourceFile, DxForgeLocalSourcePackage, DxForgePackageScorecardReport,
    DxForgeRegistryOperationReport, DxForgeRemoteObjectHeadExecutionApproval,
    DxForgeRemoteReadIntent, DxForgeUiRegistryCatalog, DxForgeUiRegistryContentEmbeddingReport,
    DxForgeUiRegistryItem, DxForgeUiRegistryItemDocsReport, DxForgeUiRegistryItemPlanReport,
    DxForgeUiRegistryItemType, DxForgeUiRegistryPlanAction, DxForgeUiRegistryPlanDecisionKind,
    DxForgeUiRegistryValidationReport, DxForgeUpdateApproval, DxSourceKind, DxSourceManifest,
    DxSourcePackage, DxSupplyChainSeverity, DxUpdateTraffic, add_outcome_markdown,
    audit_report_markdown, audit_supply_chain, build_forge_package_scorecard,
    build_forge_package_scorecard_for_project, canonical_package_id, check_dx_project,
    check_dx_project_with_options, describe_forge_ui_registry_item, dx_check_report_markdown,
    embed_forge_ui_registry_catalog_file_contents, evaluate_r2_remote_object_head_receipt_health,
    execute_r2_remote_object_head_checks_from_env, forge_docs_outcome_markdown,
    forge_launch_gate_findings, forge_package_scorecard_markdown, init_local_registry,
    latest_local_registry_package_version, load_forge_ui_registry_catalog_from_path,
    plan_forge_add_from_local_registry, plan_forge_add_selected_exports, plan_forge_add_variant,
    plan_forge_docs, plan_forge_rollback, plan_forge_ui_registry_item, plan_forge_update_variant,
    plan_r2_remote_object_head_execution_receipt,
    plan_r2_remote_read_only_install_from_manifest_fixture, publish_registry_package_to_r2,
    publish_root_dx_package_to_local_registry, publish_root_dx_package_to_r2_dry_run,
    pull_registry_package_from_r2, r2_registry_status, read_dx_check_latest_panel,
    registry_operation_markdown, registry_package, remove_outcome_markdown,
    resolve_forge_ui_registry_reference, rollback_outcome_markdown, root_dx_registry_package,
    update_outcome_markdown, validate_forge_ui_registry_catalog,
    validate_forge_ui_registry_dependency_graphs, verify_registry_package_integrity,
    write_forge_add_from_local_registry, write_forge_add_selected_exports, write_forge_add_variant,
    write_forge_docs, write_forge_local_source, write_forge_package_scorecard_history,
    write_forge_remove_dry_run_variant, write_forge_remove_variant, write_forge_rollback,
    write_forge_update_dry_run_from_local_registry, write_forge_update_dry_run_variant,
    write_forge_update_from_local_registry, write_forge_update_reviewed_variant,
    write_forge_update_variant,
};

include!("mod_parts/next_familiar_template.rs");
const FORGE_RELEASE_EVIDENCE_HISTORY_DIR: &str = ".dx/forge/release-proof-history";

const FORGE_BETA_UPGRADE_LOCAL_EDIT_PACKAGE_ID: &str = "shadcn/ui/button";
const FORGE_BETA_UPGRADE_LOCAL_EDIT_PATH: &str = "components/ui/button.tsx";
const FORGE_BETA_UPGRADE_LOCAL_EDIT_MARKER: &str = "dx-forge-beta-upgrade-smoke-local-edit";

mod add_args;
mod agent_context;
mod app_api_routes;
mod app_page_route_diagnostics;
mod app_page_routes;
mod app_route_diagnostics;
mod app_route_handler_build_output;
mod app_route_handler_receipt;
mod app_router_build_command;
mod app_router_build_output;
mod app_router_execution;
mod app_router_paths;
mod app_router_runtime_command;
mod app_router_semantics;
mod app_router_server_data;
mod app_router_style_assets;
mod app_segment_files;
mod app_server_data_manifest;
mod build_command;
mod build_observability;
mod build_options;
mod sandbox_generator;
mod build_promotion;
mod build_rollback_verification;
mod command_output;
mod config_diagnostics;
mod css_diagnostics;
mod default_template_contract;
mod default_template_materializer;
mod default_template_sources;
mod deploy_adapter_contract;
mod dev_bridge;
mod dev_command;
mod dev_hot_reload_client;
mod dev_http;
mod dev_options;
mod dev_response;
mod dev_server_mode;
mod dev_tiny_server;
mod dev_wire;
mod devtools;
mod docs_doctor;
mod dx_check_latest_receipt;
mod dx_style_support;
mod env_firewall;
mod extension_orchestrator;
mod forge_acquire_options;
mod forge_add_options;
mod forge_adoption_options;
mod forge_audit_options;
mod forge_beta_diagnostics;
mod forge_beta_options;
mod forge_ci_snippets;
mod forge_ci_snippets_options;
mod forge_doctor;
mod forge_evidence_options;
mod forge_failure_triage;
mod forge_hosted_registry_smoke;
mod forge_hosting_manifest;
mod forge_import_options;
mod forge_import_plan;
mod forge_init_app_options;
mod forge_launch_changelog;
mod forge_launch_copy_review;
mod forge_launch_page;
mod forge_migrated_route_benchmark;
mod forge_migration_audit;
mod forge_migration_workflow;
mod forge_npm_acquisition;
mod forge_npm_archive;
mod forge_operator_dashboard;
mod forge_packages_command;
mod forge_packages_options;
mod forge_provenance;
mod forge_provenance_command;
mod forge_provenance_options;
mod forge_public_add;
mod forge_public_evidence;
mod forge_public_evidence_options;
mod forge_public_status;
mod forge_publish_options;
mod forge_publish_plan_options;
mod forge_publisher_key_command;
mod forge_publisher_key_options;
mod forge_react_starter_benchmark;
mod forge_registry_options;
mod forge_release_bundle_inspect;
mod forge_release_candidate;
mod forge_release_candidate_command;
mod forge_release_dashboard;
mod forge_release_dashboard_command;
mod forge_release_history;
mod forge_release_operations_options;
mod forge_release_proof;
mod forge_release_review_options;
mod forge_release_trend;
mod forge_release_triage;
mod forge_remote_lifecycle;
mod forge_smoke_options;
mod forge_static_asset_materialization;
mod forge_static_migration_plan;
mod forge_static_migration_smoke;
mod forge_static_page_assets;
mod forge_static_page_migration;
mod forge_static_page_policy;
mod forge_static_page_preview;
mod forge_trust_policy_command;
mod forge_trust_policy_options;
mod forge_trust_regression;
mod forge_trust_regression_command;
mod forge_trust_regression_options;
mod forge_ui_registry_apply_receipt;
mod forge_ui_registry_build_receipt;
mod forge_ui_registry_parity;
mod forge_update_options;
mod formatting;
mod generate_command;
mod help_text;
mod hosted_preview_contract;
mod launch_adoption_report;
mod launch_companion_receipts;
mod launch_evidence_acceptance_digest;
mod launch_evidence_acceptance_index;
mod launch_evidence_archive_index;
mod launch_evidence_archive_ledger;
mod launch_evidence_archive_receipt;
mod launch_evidence_closure_memo;
mod launch_evidence_completion_ledger;
mod launch_evidence_continuation_packet;
mod launch_evidence_final_brief;
mod launch_evidence_friday_baton;
mod launch_evidence_handoff_capsule;
mod launch_evidence_handoff_digest;
mod launch_evidence_operator_index;
mod launch_evidence_operator_resume_card;
mod launch_evidence_operator_runbook;
mod launch_evidence_operator_summary;
mod launch_evidence_packet;
mod launch_evidence_recovery_brief;
mod launch_evidence_release_checklist;
mod launch_evidence_release_seal;
mod launch_evidence_restart_brief;
mod launch_evidence_restart_checklist;
mod launch_evidence_restart_closeout;
mod launch_evidence_restart_dispatch;
mod launch_evidence_restart_ledger;
mod launch_evidence_restart_manifest;
mod launch_evidence_restart_receipt;
mod launch_evidence_restart_signoff;
mod launch_evidence_restart_snapshot;
mod launch_evidence_restart_summary;
mod launch_evidence_resumption_index;
mod launch_evidence_retention_policy;
mod launch_evidence_retention_review;
mod launch_evidence_share_manifest;
mod launch_evidence_status_timeline;
mod launch_manifest_drift;
mod launch_readiness_bundle;
mod launch_report_options;
mod launch_runtime_approval_request;
mod launch_runtime_checklist;
mod launch_runtime_evidence;
mod launch_runtime_evidence_completeness;
mod launch_runtime_evidence_finalization;
mod launch_runtime_evidence_import_plan;
mod launch_runtime_evidence_review;
mod launch_verification_lane;
mod migrate_command;
mod migrate_options;
mod naming;
mod native_android_build_command;
mod native_android_build_disk;
mod native_android_build_environment;
mod native_android_build_gradle;
mod native_android_build_plan;
mod native_android_build_preflight;
mod native_android_build_process;
mod native_android_build_receipt;
mod native_shell_command;
mod native_shell_materializer;
mod native_shell_naming;
mod native_shell_options;
mod native_shell_plan;
mod native_shell_render;
mod native_shell_templates;
mod native_shell_validation;
mod new_command;
mod next_adapter_fixtures;
mod next_familiar_fixtures;
mod next_migration;
mod next_migration_plan;
mod next_rust_status;
mod options;
mod preview_command;
mod preview_contract;
mod preview_options;
mod project_contract_hints;
mod promote_command;
mod promote_options;
mod prove;
mod prove_fixtures;
mod prove_runtime;
mod public_framework_tools;
mod react_migration_plan;
pub(crate) mod readiness;
mod rollback_command;
mod rollback_options;
mod route_handler_runtime_env;
mod route_request_values;
mod script_runner;
mod serializer_artifacts;
mod server_action_runtime;
mod studio_command;
mod studio_json_surface;
mod studio_manifest;
mod template_options;
mod template_readiness;
mod templates_command;
mod update_command;
mod update_options;
mod utils;
mod www_output_presence;
mod www_root;

use self::add_args::{first_dx_add_subject, is_source_owned_add_candidate};
use self::app_server_data_manifest::{
    collect_app_server_data_manifest, summarize_app_server_data_manifest_routes,
};
use self::build_command::{
    BuildManifestInput, compile_legacy_pages, copy_build_asset_tree, ensure_build_output_dirs,
    run_build_command, write_build_manifest_and_deploy_adapter, write_import_build_artifacts,
    write_next_migration_build_artifacts, write_server_build_artifacts,
};
use self::build_options::{DxBuildCommandOptions, DxBuildTarget};
#[cfg(test)]
use self::dev_http::with_dev_hot_reload;
use self::dev_http::{DxCliHttpRequest, DxCliHttpResponse};
#[cfg(test)]
use self::dev_options::{bind_dev_listener, parse_dev_options};
use self::dx_check_latest_receipt::write_dx_check_latest_receipt;
#[rustfmt::skip]
use self::forge_add_options::{
    parse_forge_add_options,
    DxForgeAddCommandOptions,
};
#[rustfmt::skip]
use self::forge_adoption_options::{
    parse_forge_adoption_report_options,
    parse_forge_adoption_smoke_options,
    DxForgeAdoptionReportCommandOptions,
    DxForgeAdoptionSmokeCommandOptions,
};
#[rustfmt::skip]
use self::forge_audit_options::{
    parse_forge_audit_options,
    DxForgeAuditCommandOptions,
};
#[rustfmt::skip]
use self::forge_beta_options::{
    parse_forge_beta_install_options,
    parse_forge_beta_upgrade_smoke_options,
    DxForgeBetaInstallCommandOptions,
    DxForgeBetaUpgradeSmokeCommandOptions,
};
#[rustfmt::skip]
use self::forge_ci_snippets_options::{
    parse_forge_ci_snippets_options,
    DxForgeCiSnippetsCommandOptions,
};
use self::forge_doctor::{
    DxForgeDoctorRegistryCheck, build_forge_doctor_report, forge_doctor_package_doc_name,
    run_forge_doctor,
};
#[rustfmt::skip]
use self::forge_evidence_options::{
    parse_forge_evidence_options,
    DxForgeEvidenceCommandOptions,
};
use self::forge_failure_triage::forge_failure_triage_markdown;
use self::forge_hosted_registry_smoke::{
    build_forge_hosted_registry_smoke_report, forge_hosted_registry_smoke_failure_summary,
    forge_hosted_registry_smoke_markdown, forge_hosted_registry_smoke_terminal,
};
#[rustfmt::skip]
use self::forge_init_app_options::{
    parse_forge_init_app_options,
    DxForgeInitAppCommandOptions,
};
use self::forge_public_add::{dx_add_outcome_terminal, parse_public_forge_add_request};
use self::forge_public_status::{
    run_forge_public_receipts, run_forge_public_remote, run_forge_public_remotes,
    run_forge_public_status,
};
#[rustfmt::skip]
use self::forge_publish_options::{
    parse_forge_publish_options,
    DxForgePublishCommandOptions,
};
#[rustfmt::skip]
use self::forge_publish_plan_options::{
    parse_forge_publish_plan_options,
    DxForgePublishPlanCommandOptions,
};
use self::forge_registry_options::{
    DxForgeRegistryApplyOptions, DxForgeRegistryBuildOptions, DxForgeRegistryDocsOptions,
    DxForgeRegistryInitOptions, DxForgeRegistryListOptions, DxForgeRegistryPlanOptions,
    DxForgeRegistryPublishOptions, DxForgeRegistryPullOptions, DxForgeRegistrySmokeOptions,
    DxForgeRegistryStatusOptions, DxForgeRegistryValidateOptions,
    parse_forge_registry_apply_options, parse_forge_registry_build_options,
    parse_forge_registry_docs_options, parse_forge_registry_init_options,
    parse_forge_registry_list_options, parse_forge_registry_plan_options,
    parse_forge_registry_publish_options, parse_forge_registry_pull_options,
    parse_forge_registry_smoke_options, parse_forge_registry_status_options,
    parse_forge_registry_validate_options,
};
use self::forge_release_candidate::{
    DxForgeReleaseCandidateNoNodeModules, verify_release_candidate_no_node_modules,
    verify_release_candidate_secret_markers,
};
use self::forge_release_proof::{
    build_forge_release_evidence_report, forge_benchmark_snapshot_markdown,
    forge_release_evidence_markdown,
};
#[rustfmt::skip]
use self::forge_release_operations_options::{
    parse_forge_release_operations_options,
    DxForgeReleaseOperationsCommandOptions,
};
#[rustfmt::skip]
use self::forge_release_review_options::{
    parse_forge_release_review_options,
    DxForgeReleaseReviewCommandOptions,
};
use self::forge_remote_lifecycle::{
    DxForgeRemoteHeadCliReport, DxForgeRemoteLifecycleAction, forge_remote_head_receipt_path,
    forge_remote_lifecycle_dry_run, print_forge_remote_head_report,
    print_forge_remote_lifecycle_plans, write_forge_remote_head_report,
};
use self::forge_ui_registry_apply_receipt::{
    build_forge_ui_registry_apply_receipt, forge_ui_registry_apply_mark_written,
    forge_ui_registry_apply_rendered, forge_ui_registry_apply_write_ready,
    write_forge_ui_registry_apply_receipt_artifacts,
    write_forge_ui_registry_apply_receipt_artifacts_with_transaction,
};
#[rustfmt::skip]
use self::forge_smoke_options::{
    parse_forge_badge_options,
    parse_forge_ci_options,
    parse_forge_smoke_options,
    DxForgeBadgeCommandOptions,
    DxForgeCiCommandOptions,
    DxForgeSmokeCommandOptions,
};
#[rustfmt::skip]
use self::forge_update_options::{
    parse_forge_update_options,
    DxForgeUpdateCommandOptions,
};
use self::formatting::{
    count_substrings, html_href_values, markdown_table_cell, optional_f64, optional_string,
    optional_u64,
};
use self::help_text::{
    forge_unknown_command_message, is_help_arg, print_check_help, print_check_web_perf_help,
    print_dev_help, print_forge_help, print_forge_registry_help, print_forge_ui_help, print_help,
    print_icons_help, print_imports_help, print_new_help, print_serializer_help, print_style_help,
    print_www_help, print_www_native_shell_help,
};
use self::launch_readiness_bundle::{
    build_launch_readiness_bundle_report, launch_readiness_bundle_failure_summary,
    launch_readiness_bundle_markdown, launch_readiness_bundle_terminal,
};
#[rustfmt::skip]
use self::launch_report_options::{
    parse_launch_report_options,
    DxLaunchReportCommandOptions,
};
use self::migrate_command::cmd_migrate;
use self::naming::{dx_new_project_name, toml_basic_string_escape};
use self::native_android_build_command::cmd_www_build_android;
use self::native_shell_command::cmd_www_native_shell;
#[rustfmt::skip]
use self::options::{
    parse_score_threshold,
    resolve_cli_path,
    DxOutputFormat,
};
use self::promote_command::cmd_promote;
use self::rollback_command::cmd_rollback;
use self::update_command::{cmd_update, default_update_reviewer};
use app_route_handler_build_output::write_app_route_handler_receipts;
use app_route_handler_receipt::{
    APP_ROUTE_HANDLER_RECEIPT_SCHEMA, DxAppRouteHandlerReceiptInput,
    app_route_handler_receipt_headers, build_app_route_handler_receipt,
};
use default_template_contract::default_www_template_architecture_contract;
use default_template_materializer::{
    DEFAULT_TEMPLATE_CORE_SOURCE_RECEIPT_FILE, read_default_template_source_text,
    write_default_template_source_files,
};
use default_template_sources::{
    DEFAULT_TEMPLATE_APP_ROUTE_SOURCES, DEFAULT_TEMPLATE_HOME_ROUTE_SOURCE_FILE,
};
use forge_acquire_options::parse_forge_acquire_options;
use forge_beta_diagnostics::{
    build_forge_beta_diagnostics_report, forge_beta_diagnostics_failure_summary,
    forge_beta_diagnostics_markdown, forge_beta_diagnostics_terminal,
};
use forge_ci_snippets::{
    build_forge_ci_snippets_report, forge_ci_snippets_markdown, forge_ci_snippets_terminal,
};
use forge_import_options::parse_forge_import_options;
use forge_import_plan::{
    build_forge_import_plan_report_with_selection, build_forge_import_write_report,
    forge_import_plan_failure_summary, forge_import_plan_markdown, forge_import_plan_terminal,
};
use forge_launch_changelog::{
    DxForgeLaunchChangelogInput, build_forge_launch_changelog_report,
    build_forge_launch_changelog_report_from_history, forge_launch_changelog_markdown,
    forge_launch_changelog_terminal,
};
use forge_launch_copy_review::cmd_forge_launch_copy_review as run_forge_launch_copy_review_command;
use forge_launch_page::cmd_forge_launch_page as run_forge_launch_page_command;
use forge_migrated_route_benchmark::{
    build_forge_migrated_route_benchmark_report, forge_migrated_route_benchmark_failure_summary,
    forge_migrated_route_benchmark_markdown, forge_migrated_route_benchmark_terminal,
};
use forge_migration_audit::{
    build_forge_migration_audit_report, forge_migration_audit_failure_summary,
    forge_migration_audit_markdown, forge_migration_audit_terminal,
};
use forge_migration_workflow::{
    build_forge_migration_guide_report, build_forge_package_gallery_report,
    forge_migration_guide_failure_summary, forge_migration_guide_markdown,
    forge_migration_guide_terminal, forge_migration_package_receipts,
    forge_package_gallery_failure_summary, forge_package_gallery_markdown,
    forge_package_gallery_terminal, write_forge_package_gallery_hosted_index,
};
use forge_npm_acquisition::{acquire_npm_package, forge_acquire_markdown, forge_acquire_terminal};
use forge_operator_dashboard::{
    build_forge_operator_dashboard_report, forge_operator_dashboard_failure_summary,
    forge_operator_dashboard_markdown, forge_operator_dashboard_terminal,
};
use forge_provenance::{build_forge_provenance_report, forge_provenance_markdown};
use forge_public_evidence::{
    DxForgePublicEvidenceReport, build_forge_public_evidence_report, run_forge_public_evidence,
    verify_forge_public_evidence_export,
};
use forge_react_starter_benchmark::cmd_forge_react_starter_benchmark as run_forge_react_starter_benchmark_command;
use forge_release_bundle_inspect::{
    build_forge_release_bundle_inspect_report, forge_release_bundle_inspect_failure_summary,
    forge_release_bundle_inspect_markdown, forge_release_bundle_inspect_rollback_inputs,
    forge_release_bundle_inspect_terminal,
};
use forge_release_history::{
    DxForgePublicReleaseDashboardCheckSnapshot, DxForgePublicReleaseDashboardSnapshot,
    DxForgePublicReleaseHistory, DxForgePublicReleaseRecord,
    DxForgePublicReleaseRouteComparisonSnapshot, DxForgePublicReleaseRouteSnapshot,
    build_forge_public_release_record, forge_public_release_history_markdown,
    run_forge_release_history,
};
use forge_release_trend::{
    DxForgeReleaseReadinessTrendInput, build_forge_release_readiness_trend_report,
    forge_release_readiness_trend_markdown, forge_release_readiness_trend_terminal,
};
use forge_release_triage::{
    build_forge_release_triage_report, forge_release_triage_markdown, forge_release_triage_terminal,
};
use forge_static_asset_materialization::cmd_forge_materialize_static_assets as run_forge_materialize_static_assets_command;
use forge_static_migration_plan::{
    build_forge_static_migration_plan_report, forge_static_migration_plan_failure_summary,
    forge_static_migration_plan_markdown, forge_static_migration_plan_terminal,
};
use forge_static_migration_smoke::{
    build_forge_static_migration_smoke_report, forge_static_migration_smoke_failure_summary,
    forge_static_migration_smoke_markdown, forge_static_migration_smoke_terminal,
};
use forge_static_page_migration::{
    build_forge_static_page_migration_report, forge_static_page_migration_failure_summary,
    forge_static_page_migration_markdown, forge_static_page_migration_terminal,
};
use forge_trust_regression::{
    build_forge_trust_regression_report, forge_trust_regression_markdown,
};
use next_familiar_fixtures::write_next_familiar_fixtures;
use next_migration::DxNextProjectMigrationInput;
use project_contract_hints::build_project_contract_hint_artifact;
use public_framework_tools::{
    ensure_dx_imports_current_for_build, print_public_tool_report, run_dx_deploy, run_dx_doctor,
    run_dx_explain, run_dx_export_analyze, run_dx_icons, run_dx_imports, run_dx_packages_check,
    run_dx_style, run_dx_web_perf_check,
};
use script_runner::run_dx_script;
// =============================================================================
// CLI
// =============================================================================

/// CLI command runner.
#[derive(Debug)]
pub struct Cli {
    /// Current working directory
    cwd: PathBuf,
}

fn remove_failed_build_output(output_dir: &Path) {
    match std::fs::remove_dir_all(output_dir) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            eprintln!(
                "warning: failed to clean partial build output {}: {}",
                output_dir.display(),
                error
            );
        }
    }
}

include!("mod_parts/cli_core_impl.rs");
include!("mod_parts/cli_forge_commands_a.rs");
include!("mod_parts/cli_forge_commands_b.rs");
include!("mod_parts/cli_forge_commands_c.rs");

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Helpers
// =============================================================================

include!("mod_parts/forge_release_reports.rs");
include!("mod_parts/forge_adoption_beta.rs");
include!("mod_parts/forge_release_bundle.rs");
include!("mod_parts/forge_release_operations.rs");
include!("mod_parts/forge_verify_and_terminal.rs");

#[cfg(test)]
mod tests;
