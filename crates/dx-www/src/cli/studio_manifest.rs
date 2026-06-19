mod hot_reload_manifest;

use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn build_studio_preview_manifest(generated_at: &str) -> Value {
    let routes = studio_routes();
    json!({
        "schema": "dx.studio.preview_manifest",
        "generated_at": generated_at,
        "owner": "dx-www",
        "consumer": "zed-web-preview",
        "purpose": "One Zed-readable manifest for DX-WWW routes, Forge package slices, source files, assets, markers, and hot reload targets.",
        "commands": {
            "routes": "dx www routes --json",
            "preview_manifest": "dx www preview-manifest --json",
            "forge_packages": "dx forge packages --json",
            "templates": "dx www templates --json",
            "automations_connectors": "dx automations connectors --json",
            "automations_credentials": "dx automations credentials --json"
        },
        "preview": {
            "dev_command": "dx dev",
            "target_origin": "http://127.0.0.1:3000",
            "port_policy": "use 3000 first, then fall forward to 3001+ when busy",
            "hot_reload": hot_reload_manifest::studio_hot_reload_contract()
        },
        "studio_contract": {
            "route_key": "route",
            "open_preview_url": "preview.url",
            "source_files": "source_files",
            "forge_packages_used": "forge_packages",
            "data_dx_markers": "data_dx_markers",
            "package_surfaces": "routes[].package_surfaces",
            "source_guard_index": "source_guard_index",
            "source_guard_runbook_index": "source_guard_runbook_index",
            "preview_watch_index": "preview_watch_index",
            "route_readiness_index": "route_readiness_index",
            "forge_readiness_index": "forge_readiness_index",
            "source_selection_index": "source_selection_index",
            "editable_surface_index": "editable_surface_index",
            "edit_operation_index": "edit_operation_index",
            "env_contract_index": "env_contract_index",
            "forge_receipt_index": "forge_receipt_index",
            "check_panel": "check_panel",
            "assets": "assets",
            "hot_reload_target": "preview.hot_reload_target",
            "hot_reload_poll_receipt": "preview.hot_reload.poll_receipt",
            "package_surface_fields": [
                "package",
                "source_file",
                "materialized_file",
                "api_surface",
                "readiness",
                "data_dx_markers",
                "interaction_selectors"
            ],
            "zed_behavior": [
                "Render routes from this manifest in the Web Preview route picker.",
                "Open preview.url for the selected route.",
                "Watch source_files, assets, and Forge package materialized files for smart reload.",
                "Index package_surfaces so package readiness can be shown without scanning generated bundles.",
                "Use data_dx_markers to map preview DOM selections back to DX-owned route surfaces.",
                "Use source_guard_index to offer safe route/package checks without starting servers.",
                "Use source_guard_runbook_index to show the exact source-only command set for each route and proof contract.",
                "Use preview_watch_index for route-scoped Web Preview reloads.",
                "Use route_readiness_index and forge_readiness_index for sidebar readiness badges without scanning source.",
                "Use source_selection_index to map Web Preview DOM selections to source-owned files and package surfaces.",
                "Use env_contract_index to show app-owned public and secret environment requirements without reading local env files.",
                "Use forge_receipt_index to open source-owned receipts, manifests, and provenance proof without reading runtime evidence.",
                "Use check_panel to render the latest dx-check receipt without running expensive checks by default.",
                "Use editable_surface_index and edit_operation_index to drive source-owned DX Studio edits from stable responsive selectors.",
                "Use preview.hot_reload.poll_receipt to read route-scoped reload state without adopting upstream protocol names."
            ]
        },
        "data_dx_marker_index": studio_marker_index(),
        "source_guard_index": studio_source_guard_index(),
        "source_guard_runbook_index": studio_source_guard_runbook_index(&routes),
        "preview_watch_index": studio_preview_watch_index(&routes),
        "route_readiness_index": studio_route_readiness_index(&routes),
        "forge_readiness_index": studio_forge_readiness_index(&routes),
        "source_selection_index": studio_source_selection_index(&routes),
        "editable_surface_index": studio_editable_surface_index(&routes),
        "edit_operation_index": studio_edit_operation_index(),
        "env_contract_index": studio_env_contract_index(&routes),
        "forge_receipt_index": studio_forge_receipt_index(&routes),
        "check_panel": studio_dx_check_panel_contract(),
        "studio_edit_contract": studio_edit_contract(),
        "zed_web_preview_contract": {
            "schema": "dx.zed.web_preview_contract",
            "project_detection": {
                "strong_signals": [
                    "dx",
                    "app/",
                    "components/",
                    "server/",
                    "styles/",
                    ".dx/forge/"
                ],
                "manifest_command": "dx www preview-manifest --json",
                "routes_command": "dx www routes --json",
                "packages_command": "dx forge packages --json",
                "activation_scope": "DX powers only activate when this manifest schema or project signals are present"
            },
            "route_picker": {
                "route_field": "routes[].route",
                "label_field": "routes[].label",
                "preview_url_field": "routes[].preview.url",
                "source_files_field": "routes[].source_files",
                "package_surfaces_field": "routes[].package_surfaces",
                "fallback_route": "/"
            },
            "dom_selection": {
                "route_marker": "data-dx-route",
                "source_marker": "data-dx-source",
                "forge_marker": "data-dx-forge",
                "package_marker": "data-dx-package",
                "package_role_marker": "data-dx-package-role",
                "section_marker": "data-dx-section",
                "component_marker": "data-dx-component",
                "dashboard_workflow_marker": "data-dx-dashboard-workflow",
                "trpc_workflow_marker": "data-dx-trpc-workflow",
                "trpc_action_marker": "data-dx-trpc-action",
                "trpc_interaction_marker": "data-trpc-interaction",
                "motion_interaction_marker": "data-dx-motion-interaction",
                "motion_reduced_marker": "data-dx-motion-reduced",
                "editable_marker": "data-dx-editable",
                "design_token_scope_marker": "data-dx-token-scope",
                "hot_reload_marker": "data-dx-hot-reload-target",
                "automation_route_marker": "data-dx-automation-route",
                "automation_dashboard_card_marker": "data-dx-automation-dashboard-card",
                "automation_intent_input_marker": "data-dx-automation-intent-input",
                "automation_receipt_path_marker": "data-dx-automation-receipt-path",
                "automation_receipt_intent_marker": "data-dx-automation-receipt-intent",
                "automation_run_receipt_path_marker": "data-dx-automation-run-receipt-path",
                "automation_run_receipt_intent_marker": "data-dx-automation-run-receipt-intent",
                "automation_dashboard_state_marker": "data-dx-automation-dashboard-state",
                "automation_selected_connector_marker": "data-dx-automation-selected-connector",
                "automation_credential_schema_marker": "data-dx-automation-credential-schema",
                "automation_auth_kind_marker": "data-dx-automation-auth-kind",
                "automation_credential_type_marker": "data-dx-automation-credential-type",
                "automation_required_env_marker": "data-dx-automation-required-env",
                "automation_workflow_node_readiness_marker": "data-dx-automation-workflow-node-readiness",
                "automation_usable_as_tool_marker": "data-dx-automation-usable-as-tool",
                "check_panel_marker": "data-dx-check-panel",
                "check_command_marker": "data-dx-check-command",
                "check_receipt_path_marker": "data-dx-check-receipt-path",
                "check_schema_marker": "data-dx-check-schema",
                "check_score_max_marker": "data-dx-check-score-max",
                "check_view_model_schema_marker": "data-dx-check-view-model-schema",
                "check_view_model_status_marker": "data-dx-check-view-model-status",
                "check_score_state_marker": "data-dx-check-score-state",
                "check_empty_state_marker": "data-dx-check-empty-state",
                "check_bucket_count_marker": "data-dx-check-bucket-count",
                "check_package_lane_count_marker": "data-dx-check-package-lane-count",
                "check_style_evidence_drift_marker": "data-dx-check-style-evidence-drift",
                "check_style_evidence_drift_state_marker": "data-dx-check-style-evidence-drift-state",
                "check_style_evidence_drift_loader_marker": "data-dx-check-style-evidence-drift-loader",
                "check_style_evidence_drift_helper_marker": "data-dx-check-style-evidence-drift-helper",
                "check_style_evidence_drift_states_marker": "data-dx-check-style-evidence-drift-states",
                "style_package_panel_marker": "data-dx-style-package-panel",
                "style_package_panel_read_model_marker": "data-dx-style-package-panel-read-model",
                "style_package_panel_drift_state_marker": "data-dx-style-package-panel-drift-state",
                "style_package_panel_drift_status_marker": "data-dx-style-package-panel-drift-status",
                "style_package_panel_drift_mismatch_fields_marker": "data-dx-style-package-panel-drift-mismatch-fields",
                "style_package_panel_readiness_receipt_marker": "data-dx-style-package-panel-readiness-receipt",
                "style_package_ownership_read_model_marker": "data-dx-style-package-ownership-read-model",
                "style_package_ownership_packages_marker": "data-dx-style-package-ownership-packages",
                "style_package_ownership_generated_classes_marker": "data-dx-style-package-ownership-generated-classes",
                "style_package_ownership_unsupported_classes_marker": "data-dx-style-package-ownership-unsupported-classes",
                "check_package_lane_row_marker": "data-dx-check-package-lane-row",
                "check_package_lane_status_marker": "data-dx-check-package-lane-status",
                "check_package_lane_receipt_status_marker": "data-dx-check-package-lane-receipt-status",
                "check_package_lane_name_marker": "data-dx-check-package-lane-name",
                "check_package_lane_upstream_package_marker": "data-dx-check-package-lane-upstream-package",
                "check_package_lane_source_mirror_marker": "data-dx-check-package-lane-source-mirror",
                "check_package_lane_receipt_path_marker": "data-dx-check-package-lane-receipt-path",
                "check_package_lane_dx_style_status_marker": "data-dx-check-package-lane-dx-style-status",
                "check_package_lane_hash_refresh_status_marker": "data-dx-check-package-lane-hash-refresh-status",
                "check_package_lane_hash_refresh_helper_marker": "data-dx-check-package-lane-hash-refresh-helper",
                "check_package_lane_hash_refresh_json_command_marker": "data-dx-check-package-lane-hash-refresh-json-command",
                "check_package_lane_hash_refresh_zed_marker": "data-dx-check-package-lane-hash-refresh-zed",
                "check_package_lane_hash_refresh_tracked_files_marker": "data-dx-check-package-lane-hash-refresh-tracked-files",
                "check_package_lane_hash_refresh_stale_files_marker": "data-dx-check-package-lane-hash-refresh-stale-files",
                "check_package_lane_hash_refresh_missing_files_marker": "data-dx-check-package-lane-hash-refresh-missing-files",
                "check_package_lane_hash_refresh_current_file_list_marker": "data-dx-check-package-lane-hash-refresh-current-file-list",
                "check_package_lane_hash_refresh_stale_file_list_marker": "data-dx-check-package-lane-hash-refresh-stale-file-list",
                "check_package_lane_hash_refresh_missing_file_list_marker": "data-dx-check-package-lane-hash-refresh-missing-file-list",
                "check_package_lane_hash_refresh_stale_mirror_file_list_marker": "data-dx-check-package-lane-hash-refresh-stale-mirror-file-list",
                "check_package_lane_hash_refresh_missing_mirror_file_list_marker": "data-dx-check-package-lane-hash-refresh-missing-mirror-file-list",
                "check_package_lane_hash_refresh_current_metric_marker": "data-dx-check-package-lane-hash-refresh-current-metric",
                "check_package_lane_hash_refresh_stale_metric_marker": "data-dx-check-package-lane-hash-refresh-stale-metric",
                "check_package_lane_hash_refresh_missing_metric_marker": "data-dx-check-package-lane-hash-refresh-missing-metric",
                "check_blocker_count_marker": "data-dx-check-blocker-count",
                "check_warning_count_marker": "data-dx-check-warning-count",
                "check_quick_fix_count_marker": "data-dx-check-quick-fix-count",
                "check_last_run_marker": "data-dx-check-last-run",
                "forge_safety_archive_contract_marker": "data-dx-safety-archive-contract",
                "forge_safety_archive_state_marker": "data-dx-safety-archive-state",
                "forge_safety_archive_safe_delete_marker": "data-dx-safety-archive-safe-delete",
                "forge_safety_archive_package_count_marker": "data-dx-safety-archive-package-count",
                "forge_safety_archive_covered_packages_marker": "data-dx-safety-archive-covered-packages",
                "forge_safety_archive_missing_packages_marker": "data-dx-safety-archive-missing-packages",
                "forge_safety_archive_rollback_coverage_marker": "data-dx-safety-archive-rollback-coverage",
                "forge_safety_archive_receipt_count_marker": "data-dx-safety-archive-receipt-count",
                "forge_safety_archive_directory_marker": "data-dx-safety-archive-directory",
                "forge_safety_archive_boundary_marker": "data-dx-safety-archive-boundary",
                "edit_id_marker": "data-dx-edit-id",
                "edit_kind_marker": "data-dx-edit-kind",
                "edit_ops_marker": "data-dx-edit-ops",
                "edit_order_marker": "data-dx-edit-order",
                "editable_section_marker": "data-dx-editable-section",
                "insert_slot_marker": "data-dx-insert-slot",
                "reorder_group_marker": "data-dx-reorder-group",
                "design_token_marker": "data-dx-design-token",
                "content_key_marker": "data-dx-content-key",
                "editable_text_marker": "data-dx-editable-text",
                "media_slot_marker": "data-dx-media-slot",
                "visual_audit_marker": "data-visual-audit"
            },
            "semantic_actions": [
                {
                    "id": "open-source-file",
                    "requires_marker": "data-dx-source",
                    "source_field": "source_files"
                },
                {
                    "id": "show-forge-package-readiness",
                    "requires_marker": "data-dx-package",
                    "source_field": "package_surfaces"
                },
                {
                    "id": "reload-route-scope",
                    "requires_marker": "data-dx-hot-reload-target",
                    "source_field": "preview.hot_reload_target"
                },
                {
                    "id": "show-source-guard",
                    "requires_marker": "data-dx-route",
                    "source_field": "source_guard_index"
                },
                {
                    "id": "show-dx-check-panel",
                    "requires_marker": "data-dx-check-panel",
                    "source_field": "check_panel"
                },
                {
                    "id": "show-dx-check-package-lane-row",
                    "requires_marker": "data-dx-check-package-lane-row",
                    "source_field": "check_panel.view_model.package_lane_rows"
                },
                {
                    "id": "show-forge-safety-archive-status",
                    "requires_marker": "data-dx-safety-archive-contract",
                    "source_field": "studio_edit_contract.surfaces"
                },
                {
                    "id": "insert-component",
                    "requires_marker": "data-dx-insert-slot",
                    "source_field": "studio_edit_contract.operations"
                },
                {
                    "id": "move-reorder-section",
                    "requires_marker": "data-dx-editable-section",
                    "source_field": "studio_edit_contract.surfaces"
                },
                {
                    "id": "update-design-token",
                    "requires_marker": "data-dx-design-token",
                    "source_field": "studio_edit_contract.operations"
                },
                {
                    "id": "update-text-content",
                    "requires_marker": "data-dx-editable-text",
                    "source_field": "studio_edit_contract.operations"
                },
                {
                    "id": "insert-icon-media",
                    "requires_marker": "data-dx-media-slot",
                    "source_field": "studio_edit_contract.operations"
                }
            ],
            "edit_operations": {
                "manifest_field": "studio_edit_contract",
                "edit_policy": "source-owned-edit-contract-explicit-user-action",
                "operation_field": "studio_edit_contract.operations",
                "surface_field": "studio_edit_contract.surfaces",
                "allowed_operations": [
                    "insert_component",
                    "move_reorder_section",
                    "update_design_token",
                    "update_text_content",
                    "insert_icon_media"
                ],
                "layout_policy": "responsive-design-system-grid",
                "absolute_positioning": false,
                "writes_files": true,
                "writes_only_source_owned_files": true,
                "requires_node_modules": false,
                "requires_explicit_user_action": true
            },
            "semantic_selection": {
                "index_field": "source_selection_index",
                "route_marker": "data-dx-route",
                "source_marker": "data-dx-source",
                "package_marker": "data-dx-package",
                "hot_reload_marker": "data-dx-hot-reload-target",
                "open_file_field": "primary_source_file",
                "candidate_files_field": "source_files",
                "package_surfaces_field": "package_surfaces",
                "source_guard_field": "source_guard_ids",
                "editable_surface_field": "editable_surfaces",
                "edit_operation_ids_field": "edit_operation_ids",
                "edit_policy": "source-owned-edit-contract-explicit-user-action",
                "writes_files": false,
                "edit_operations_write_files": true,
                "future_hooks": [
                    "semantic-edit-preview",
                    "insert-component",
                    "move-reorder-section",
                    "update-design-token",
                    "update-text-content",
                    "insert-icon-media",
                    "drop-to-code",
                    "package-receipt-inspection"
                ]
            },
            "edit_contract": {
                "schema": "dx.studio.launch_edit_contract",
                "source_manifest_file": "examples/template/dx-studio-edit-contract.ts",
                "materialized_manifest_file": "components/template-app/dx-studio-edit-contract.ts",
                "editable_surface_index_field": "editable_surface_index",
                "edit_operation_index_field": "edit_operation_index",
                "operation_ids": [
                    "insert_component",
                    "move_reorder_section",
                    "update_design_token",
                    "update_text_content",
                    "insert_icon_media"
                ],
                "layout_policy": "responsive-design-system-grid",
                "absolute_positioning": false,
                "design_token_scope_marker": "data-dx-token-scope",
                "writes_after_explicit_user_command": true,
                "no_node_modules_required": true
            },
            "source_guards": {
                "index_field": "source_guard_index",
                "runbook_index_field": "source_guard_runbook_index",
                "command_field": "commands",
                "contract_field": "contracts",
                "default_action": "show-source-only-runbook",
                "default_mode": "show-before-run",
                "allowed_scope": "source-only",
                "disallowed_by_default": [
                    "local servers",
                    "npm installs",
                    "full builds",
                    "broad cargo builds"
                ]
            },
            "file_watch": {
                "index_field": "preview_watch_index",
                "route_field": "route",
                "watch_fields": [
                    "watch_files",
                    "source_files",
                    "materialized_files",
                    "assets",
                    "forge_packages"
                ],
                "reload_target_field": "hot_reload_target",
                "default_reload": "route-scoped",
                "ignored_roots": [
                    "node_modules",
                    ".dx/build",
                    "target",
                    ".next"
                ]
            },
            "package_readiness": {
                "index_field": "forge_readiness_index",
                "route_index_field": "route_readiness_index",
                "route_field": "routes",
                "package_field": "package",
                "readiness_field": "readiness",
                "api_surface_field": "api_surface",
                "source_guard_field": "source_guard_ids",
                "dom_marker": "data-dx-package",
                "default_badge": "source-owned-runtime-gated",
                "no_node_modules_field": "node_modules_required"
            },
            "env_contracts": {
                "index_field": "env_contract_index",
                "package_field": "package",
                "env_field": "env",
                "visibility_field": "visibility",
                "status_field": "status",
                "reads_environment": false,
                "writes_files": false,
                "default_badge": "app-owned-env-boundary"
            },
            "forge_receipts": {
                "index_field": "forge_receipt_index",
                "route_field": "route",
                "route_receipts_field": "route_receipts",
                "package_receipts_field": "package_receipts",
                "source_guard_field": "source_guard_ids",
                "reads_runtime_artifacts": false,
                "writes_files": false,
                "default_badge": "source-owned-proof"
            },
            "safety": {
                "no_runtime_execution": true,
                "no_node_modules_required": true,
                "normal_web_preview_behavior_preserved": true
            }
        },
        "project_contract": {
            "config_file": "dx",
            "package_policy": "forge-first-source-owned",
            "node_modules_policy": "strict launch templates must not create node_modules",
            "no_node_modules_required": true,
            "source_owned_packages": true,
            "runtime_proof": "explicit-approval-required",
            "source_guard": "route manifests must name every package slice, source file, data-dx marker, and hot reload target consumed by Zed Web Preview"
        },
        "integrations": {
            "website_conversion": {
                "routes": ["/ui", "/database", "/backend"],
                "source_root": "examples/conversion-proof",
                "manifest_glob": "examples/conversion-proof/forge/conversion-manifests/*.json",
                "receipt_glob": "examples/conversion-proof/.dx/forge/receipts/*.json"
            },
            "n8n_automations": {
                "routes": ["/automations", "/automations/connectors", "/automations/credentials", "/automations/workflows"],
                "source_root": "examples/template/automations",
                "generated_manifest_root": "integrations/n8n-nodes-base/generated",
                "cli_bridge": "dx automations connectors --json"
            },
            "forge_packages": {
                "command": "dx forge packages --json",
                "catalog_source": "examples/template/package-catalog.ts",
                "template_contract": "examples/template/template-route-contract.ts"
            }
        },
        "routes": routes,
        "assets": studio_assets(),
        "forge_package_count": unique_forge_packages(&routes).len(),
        "forge_packages_used": unique_forge_packages(&routes),
        "no_execution": true
    })
}
pub(super) fn build_www_routes_report(generated_at: &str) -> Value {
    let routes = studio_routes();
    json!({
        "schema": "dx.www.routes",
        "generated_at": generated_at,
        "owner": "dx-www",
        "source": "dx-studio-preview-manifest",
        "command": "dx www routes --json",
        "preview_manifest_command": "dx www preview-manifest --json",
        "route_count": routes.len(),
        "routes": routes,
        "no_execution": true
    })
}
pub(super) fn studio_preview_manifest_terminal(report: &Value) -> String {
    let route_count = report
        .get("routes")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let package_count = report
        .get("forge_packages_used")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    format!(
        "DX Studio preview manifest\nSchema: {}\nRoutes: {}\nForge packages: {}\nCommand: dx www preview-manifest --json\n",
        report
            .get("schema")
            .and_then(Value::as_str)
            .unwrap_or("dx.studio.preview_manifest"),
        route_count,
        package_count
    )
}

pub(super) fn www_routes_terminal(report: &Value) -> String {
    let mut output = format!(
        "DX-WWW routes\nSchema: {}\nRoutes: {}\n",
        report
            .get("schema")
            .and_then(Value::as_str)
            .unwrap_or("dx.www.routes"),
        report
            .get("routes")
            .and_then(Value::as_array)
            .map_or(0, Vec::len)
    );

    for route in report
        .get("routes")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        output.push_str(&format!(
            "- {} -> {}\n",
            route.get("route").and_then(Value::as_str).unwrap_or("/"),
            route
                .pointer("/preview/url")
                .and_then(Value::as_str)
                .unwrap_or("http://127.0.0.1:3000/")
        ));
    }

    output
}

pub(super) fn studio_preview_manifest_markdown(report: &Value) -> String {
    let mut output = String::from(
        "# DX Studio Preview Manifest\n\n| Route | Source | Preview |\n| --- | --- | --- |\n",
    );
    for route in report
        .get("routes")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` |\n",
            route.get("route").and_then(Value::as_str).unwrap_or("/"),
            route
                .get("source")
                .and_then(Value::as_str)
                .unwrap_or("dx-www"),
            route
                .pointer("/preview/url")
                .and_then(Value::as_str)
                .unwrap_or("http://127.0.0.1:3000/")
        ));
    }
    output
}

pub(super) fn www_routes_markdown(report: &Value) -> String {
    let mut output =
        String::from("# DX-WWW Routes\n\n| Route | Role | Preview |\n| --- | --- | --- |\n");
    for route in report
        .get("routes")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` |\n",
            route.get("route").and_then(Value::as_str).unwrap_or("/"),
            route.get("role").and_then(Value::as_str).unwrap_or("route"),
            route
                .pointer("/preview/url")
                .and_then(Value::as_str)
                .unwrap_or("http://127.0.0.1:3000/")
        ));
    }
    output
}

fn studio_routes() -> Vec<Value> {
    vec![
        json!({
            "route": "/",
            "label": "DX Studio www dashboard",
            "role": "main-dashboard",
            "source": "forge-www-template",
            "status": "source-ready-runtime-gated",
            "source_files": [
                "examples/template/app/page.tsx",
                "examples/template/template-shell.tsx",
                "examples/template/template-dashboard-nav.tsx",
                "examples/template/dx-studio-edit-contract.ts",
                "examples/template/automations-status.tsx",
                "examples/template/automation-mission-summary.tsx",
                "examples/template/automations/automations-metadata.ts",
                "examples/template/template-lead-form.tsx",
                "examples/template/package-catalog.ts",
                "examples/template/template-route-contract.ts",
                "examples/template/auth-session-status.tsx",
                "examples/template/ai-chat-status.tsx",
                "examples/template/data-status.tsx",
                "examples/template/payments-status.tsx",
                "examples/template/docs-status.tsx",
                "examples/template/instantdb-status.tsx",
                "examples/template/icon-status.tsx",
                "examples/template/next-intl-dashboard-locale.tsx",
                "examples/template/next-intl-status.tsx",
                "examples/template/query-cache-status.tsx",
                "examples/template/query-dashboard-read-model.ts",
                "examples/template/react-markdown-preview.tsx",
                "examples/template/state-zustand-counter.tsx",
                "examples/template/state-zustand-dashboard.tsx",
                "examples/template/state-zustand-dashboard.tsx",
                "examples/template/trpc-launch-contract.ts",
                "examples/template/trpc-launch-health.tsx",
                "examples/template/wasm-interop-status.tsx",
                "examples/template/zod-validation-status.tsx",
                "examples/template/launch-scene.tsx",
                "examples/template/scene/types.ts",
                "examples/template/scene/preset.ts",
                "examples/template/scene/dashboard-workflow.ts",
                "examples/template/scene/dashboard-controls.ts",
                "examples/template/scene/frame-sample.ts",
                "examples/template/scene/capability-report.ts",
                "examples/template/scene/viewport-report.ts",
                "examples/template/scene/bounds-report.ts",
                "examples/template/scene/raycast-report.ts",
                "examples/template/scene/webgl-runtime.ts",
                "examples/template/scene/metadata.ts",
                "examples/template/scene/preview-readiness.ts",
                "examples/template/scene/README.md"
            ],
            "materialized_files": [
                "app/page.tsx",
                "components/template-app/template-shell.tsx",
                "components/template-app/template-dashboard-nav.tsx",
                "components/template-app/dx-studio-edit-contract.ts",
                "components/template-app/automations-status.tsx",
                "components/template-app/automation-mission-summary.tsx",
                "components/template-app/automations/automations-metadata.ts",
                "components/template-app/template-lead-form.tsx",
                "components/template-app/package-catalog.ts",
                "components/template-app/template-route-contract.ts",
                "components/template-app/auth-session-status.tsx",
                "components/template-app/ai-chat-status.tsx",
                "components/template-app/data-status.tsx",
                "components/template-app/payments-status.tsx",
                "components/template-app/docs-status.tsx",
                "components/template-app/instantdb-status.tsx",
                "components/template-app/icon-status.tsx",
                "components/template-app/next-intl-dashboard-locale.tsx",
                "components/template-app/next-intl-status.tsx",
                "components/template-app/query-cache-status.tsx",
                "components/template-app/query-dashboard-read-model.ts",
                "components/template-app/react-markdown-preview.tsx",
                "components/template-app/state-zustand-counter.tsx",
                "components/template-app/state-zustand-dashboard.tsx",
                "components/template-app/state-zustand-dashboard.tsx",
                "components/template-app/trpc-launch-contract.ts",
                "components/template-app/trpc-launch-health.tsx",
                "components/template-app/wasm-interop-status.tsx",
                "components/template-app/zod-validation-status.tsx",
                "components/scene/launch-scene.tsx",
                "lib/scene/index.ts",
                "lib/scene/types.ts",
                "lib/scene/preset.ts",
                "lib/scene/dashboard-workflow.ts",
                "lib/scene/dashboard-controls.ts",
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
                ".dx/forge/template-readiness/launch-route.json",
                ".dx/forge/template-readiness/launch-readiness-bundle.json",
                "server/templateCatalog.ts"
            ],
            "forge_packages": [
                "shadcn/ui/button",
                "shadcn/ui/card",
                "shadcn/ui/alert",
                "shadcn/ui/avatar",
                "shadcn/ui/skeleton",
                "shadcn/ui/separator",
                "shadcn/ui/input",
                "shadcn/ui/textarea",
                "dx/icon/search",
                "auth/better-auth",
                "animation/motion",
                "forms/react-hook-form",
                "i18n/next-intl",
                "tanstack/query",
                "reactive/store",
                "validation/zod",
                "payments/stripe-js",
                "automations/n8n",
                "state/zustand",
                "ai/vercel-ai",
                "api/trpc",
                "content/fumadocs-next",
                "content/react-markdown",
                "supabase/client",
                "db/drizzle-sqlite",
                "instantdb/react",
                "wasm/bindgen",
                "3d/launch-scene"
            ],
            "assets": [
                "examples/template/scene/README.md",
                "examples/template/scene/preset.ts"
            ],
            "package_surfaces": launch_package_surfaces(),
            "no_node_modules_proof": {
                "checked_path": "examples/template/node_modules",
                "expected": "absent",
                "guard": "benchmarks/dx-studio-preview-manifest.test.ts",
                "policy": "launch template package slices are source-owned and must not materialize node_modules"
            },
            "data_dx_markers": [
                "data-dx-route",
                "data-dx-source",
                "data-dx-forge",
                "data-dx-package",
                "data-dx-package-role",
                "data-dx-hot-reload-target",
                "data-dx-ready",
                "data-dx-node-modules",
                "data-dx-edit-contract",
                "data-dx-section",
                "data-dx-component",
                "data-dx-editable",
                "data-dx-token-scope",
                "data-dx-edit-id",
                "data-dx-edit-kind",
                "data-dx-edit-ops",
                "data-dx-edit-order",
                "data-dx-editable-section",
                "data-dx-insert-slot",
                "data-dx-reorder-group",
                "data-dx-design-token",
                "data-dx-content-key",
                "data-dx-editable-text",
                "data-dx-media-slot",
                "data-dx-data-status",
                "data-dx-payment-status",
                "data-dx-checkout-submit-state",
                "data-launch-i18n-phase",
                "data-dx-docs-status",
                "data-dx-instant-status",
                "data-dx-zod-status",
                "data-dx-scene-status",
                "data-dx-scene-preview-readiness",
                "data-dx-scene-quality-profile",
                "data-dx-scene-material-palette",
                "data-dx-scene-camera-rig",
                "data-dx-scene-frame-sample",
                "data-dx-scene-capability-report",
                "data-dx-scene-capability-status",
                "data-dx-scene-viewport-report",
                "data-dx-scene-viewport-status",
                "data-dx-scene-bounds-report",
                "data-dx-scene-bounds-status",
                "data-dx-scene-raycast-report",
                "data-dx-scene-raycast-status",
                "data-dx-scene-workflow-active",
                "data-dx-scene-workflow-receipt-state",
                "data-dx-dashboard-workflow",
                "data-dx-trpc-workflow",
                "data-dx-trpc-action",
                "data-trpc-interaction",
                "data-trpc-mutation-state",
                "data-dx-trpc-receipt-state",
                "data-dx-trpc-request-id",
                "data-dx-motion-interaction",
                "data-dx-motion-reduced",
                "data-dx-motion-state",
                "data-dx-motion-progress",
                "data-dx-motion-order",
                "data-dx-motion-order-available",
                "data-dx-motion-keyboard-reorder",
                "data-dx-motion-keyboard-state",
                "data-dx-motion-preference-storage",
                "data-dx-motion-storage-key",
                "data-dx-zustand-store",
                "data-dx-zustand-persist-key",
                "data-dx-zustand-action",
                "data-dx-automation-dashboard-card",
                "data-dx-automation-intent-input",
                "data-dx-automation-receipt-path",
                "data-dx-automation-receipt-intent",
                "data-dx-automation-run-receipt-intent",
                "data-dx-automation-required-env",
                "data-dx-automation-view",
                "data-dx-automation-route"
            ],
            "preview": preview_for("/")
        }),
        json!({
            "route": "/automations",
            "label": "DX Automations",
            "role": "n8n-automation-dashboard",
            "source": "n8n-automations-bridge",
            "status": "source-ready-cli-bridge",
            "source_files": [
                "examples/template/app/automations/page.tsx",
                "examples/template/app/automations/connectors/page.tsx",
                "examples/template/app/automations/credentials/page.tsx",
                "examples/template/app/automations/workflows/page.tsx",
                "examples/template/automations/automations-shell.tsx",
                "examples/template/automations/automations-metadata.ts",
                "integrations/n8n-nodes-base/generated/dx-automations-connectors.json",
                "integrations/n8n-nodes-base/generated/dx-automations-credentials.json",
                "integrations/n8n-nodes-base/generated/dx-automations-readiness.json"
            ],
            "materialized_files": [
                "app/automations/page.tsx",
                "app/automations/connectors/page.tsx",
                "app/automations/credentials/page.tsx",
                "app/automations/workflows/page.tsx",
                "components/automations/automations-shell.tsx",
                "components/automations/automations-metadata.ts"
            ],
            "forge_packages": ["automations/n8n"],
            "assets": [],
            "package_surfaces": [
                {
                    "package": "automations/n8n",
                    "source_file": "examples/template/automations/automations-shell.tsx",
                    "materialized_file": "components/automations/automations-shell.tsx",
                    "api_surface": [
                        "automationRoutes",
                        "automationSummary",
                        "connectorMetadata",
                        "credentialMetadata"
                    ],
                    "readiness": "source-owned-automation-metadata",
                    "data_dx_markers": [
                        "data-dx-automation-view",
                        "data-dx-automation-route"
                    ]
                }
            ],
            "no_node_modules_proof": {
                "checked_path": "examples/template/node_modules",
                "expected": "absent",
                "guard": "benchmarks/dx-studio-preview-manifest.test.ts",
                "policy": "automation metadata is source-owned and must not materialize node_modules"
            },
            "data_dx_markers": [
                "data-dx-route",
                "data-dx-source",
                "data-dx-forge",
                "data-dx-package",
                "data-dx-hot-reload-target",
                "data-dx-node-modules",
                "data-dx-section",
                "data-dx-component",
                "data-dx-editable",
                "data-dx-token-scope",
                "data-dx-edit-id",
                "data-dx-edit-kind",
                "data-dx-edit-ops",
                "data-dx-editable-section",
                "data-dx-insert-slot",
                "data-dx-reorder-group",
                "data-dx-design-token",
                "data-dx-editable-text",
                "data-dx-automation-view",
                "data-dx-automation-route"
            ],
            "preview": preview_for("/automations")
        }),
        converted_route(
            "/ui",
            "Converted shadcn UI route",
            "website-conversion-shadcn",
            "examples/conversion-proof/pages/ui.html",
            "examples/conversion-proof/forge/conversion-manifests/shadcn-ui.json",
            "examples/conversion-proof/.dx/forge/receipts/2026-05-21-shadcn-ui-to-ui.json",
            &[
                "shadcn/ui/button",
                "shadcn/ui/card",
                "shadcn/ui/input",
                "shadcn/ui/textarea",
            ],
            &["examples/conversion-proof/public/vendor/shadcn-favicon-32x32.png"],
        ),
        converted_route(
            "/database",
            "Converted Supabase database route",
            "website-conversion-supabase",
            "examples/conversion-proof/pages/database.html",
            "examples/conversion-proof/forge/conversion-manifests/supabase.json",
            "examples/conversion-proof/.dx/forge/receipts/2026-05-21-supabase-to-database.json",
            &["supabase/client", "db/drizzle-sqlite"],
            &["examples/conversion-proof/public/vendor/supabase-logo.svg"],
        ),
        converted_route(
            "/backend",
            "Converted Convex backend route",
            "website-conversion-convex",
            "examples/conversion-proof/pages/backend.html",
            "examples/conversion-proof/forge/conversion-manifests/convex-backend.json",
            "examples/conversion-proof/.dx/forge/receipts/2026-05-21-convex-to-backend.json",
            &["api/trpc", "ai/vercel-ai", "supabase/client"],
            &["examples/conversion-proof/public/vendor/convex-logo.svg"],
        ),
    ]
}

fn studio_marker_index() -> Vec<Value> {
    vec![
        studio_marker(
            "data-dx-route",
            "route-owner",
            &["/", "/automations", "/ui", "/database", "/backend"],
            "routes[].route",
            "Map a preview DOM node to the DX-WWW route that owns it.",
        ),
        studio_marker(
            "data-dx-source",
            "source-owner",
            &["/", "/automations", "/ui", "/database", "/backend"],
            "routes[].source_files",
            "Open the source file or source project that owns the selected surface.",
        ),
        studio_marker(
            "data-dx-forge",
            "forge-owner",
            &["/", "/automations", "/ui", "/database", "/backend"],
            "routes[].source",
            "Show the Forge or conversion lane responsible for the surface.",
        ),
        studio_marker(
            "data-dx-package",
            "forge-package",
            &["/", "/automations", "/ui", "/database", "/backend"],
            "routes[].package_surfaces",
            "Show package readiness, source file, materialized file, and API surface metadata.",
        ),
        studio_marker(
            "data-dx-package-role",
            "package-role",
            &["/"],
            "routes[].package_surfaces[].readiness",
            "Group launch package rows by capability role for sidebar filters.",
        ),
        studio_marker(
            "data-dx-hot-reload-target",
            "hot-reload-target",
            &["/", "/automations", "/ui", "/database", "/backend"],
            "routes[].preview.hot_reload_target",
            "Reload only the route scope that owns the changed source surface.",
        ),
        studio_marker(
            "data-dx-ready",
            "route-readiness",
            &["/"],
            "routes[].status",
            "Show source-owned route readiness for the template shell.",
        ),
        studio_marker(
            "data-dx-node-modules",
            "safety-policy",
            &["/", "/automations"],
            "project_contract.node_modules_policy",
            "Show that the launch and automation templates are source-owned and no-node-modules by default.",
        ),
        studio_marker(
            "data-dx-check-panel",
            "dx-check-panel",
            &["/"],
            "check_panel",
            "Open the latest dx-check receipt as a source-owned project health panel.",
        ),
        studio_marker(
            "data-dx-check-command",
            "dx-check-command",
            &["/"],
            "check_panel.command",
            "Show the read-only command DX-WWW uses to load the latest dx-check receipt.",
        ),
        studio_marker(
            "data-dx-check-receipt-path",
            "dx-check-receipt-path",
            &["/"],
            "check_panel.receipt_path",
            "Show the stable machine-readable receipt path for DX-WWW and future GPUI panels.",
        ),
        studio_marker(
            "data-dx-check-schema",
            "dx-check-schema",
            &["/"],
            "check_panel.panel_schema",
            "Show the DX-WWW check panel schema consumed by Web Preview.",
        ),
        studio_marker(
            "data-dx-check-score-max",
            "dx-check-score-max",
            &["/"],
            "check_panel.score_max",
            "Show the 500-point project health score maximum.",
        ),
        studio_marker(
            "data-dx-check-view-model-schema",
            "dx-check-view-model-schema",
            &["/"],
            "check_panel.view_model_schema",
            "Show the render-ready DX-WWW check panel view-model schema.",
        ),
        studio_marker(
            "data-dx-check-view-model-status",
            "dx-check-view-model-status",
            &["/"],
            "check_panel.view_model.status",
            "Show whether the latest dx-check receipt panel is ready, missing, or malformed.",
        ),
        studio_marker(
            "data-dx-check-score-state",
            "dx-check-score-state",
            &["/"],
            "check_panel.view_model.score_meter",
            "Show whether the panel has a receipt-backed score meter.",
        ),
        studio_marker(
            "data-dx-check-empty-state",
            "dx-check-empty-state",
            &["/"],
            "check_panel.view_model.empty_state",
            "Show the explicit missing or malformed receipt message without inventing a score.",
        ),
        studio_marker(
            "data-dx-check-bucket-count",
            "dx-check-bucket-count",
            &["/"],
            "check_panel.view_model.bucket_rows",
            "Show how many bucket rows the latest dx-check receipt can render.",
        ),
        studio_marker(
            "data-dx-check-package-lane-count",
            "dx-check-package-lane-count",
            &["/"],
            "check_panel.view_model.package_lane_rows",
            "Show how many Forge package-lane rows the latest dx-check receipt can render.",
        ),
        studio_marker(
            "data-dx-check-package-lane-row",
            "dx-check-package-lane-row",
            &["/"],
            "check_panel.view_model.package_lane_rows",
            "Open a receipt-backed Forge package-lane row from the dx-check panel.",
        ),
        studio_marker(
            "data-dx-check-package-lane-status",
            "dx-check-package-lane-status",
            &["/"],
            "check_panel.view_model.package_lane_rows[].status",
            "Show the present, stale, missing-receipt, blocked, or unsupported-surface state for a Forge package lane.",
        ),
        studio_marker(
            "data-dx-check-package-lane-receipt-status",
            "dx-check-package-lane-receipt-status",
            &["/"],
            "check_panel.view_model.package_lane_rows[].receipt_status",
            "Show the package receipt state used to derive the dx-check package-lane row.",
        ),
        studio_marker(
            "data-dx-check-package-lane-name",
            "dx-check-package-lane-name",
            &["/"],
            "check_panel.view_model.package_lane_rows[].official_package_name",
            "Show the official DX package lane name rendered in the dx-check panel.",
        ),
        studio_marker(
            "data-dx-check-package-lane-upstream-package",
            "dx-check-package-lane-upstream-package",
            &["/"],
            "check_panel.view_model.package_lane_rows[].upstream_package",
            "Show upstream package provenance without branding the DX lane by npm package name.",
        ),
        studio_marker(
            "data-dx-check-package-lane-source-mirror",
            "dx-check-package-lane-source-mirror",
            &["/"],
            "check_panel.view_model.package_lane_rows[].source_mirror",
            "Show the inspected source mirror used for Forge package-lane provenance.",
        ),
        studio_marker(
            "data-dx-check-package-lane-receipt-path",
            "dx-check-package-lane-receipt-path",
            &["/"],
            "check_panel.view_model.package_lane_rows[].package_receipt_path",
            "Show the package receipt path backing the dx-check package-lane row.",
        ),
        studio_marker(
            "data-dx-check-package-lane-dx-style-status",
            "dx-check-package-lane-dx-style-status",
            &["/"],
            "check_panel.view_model.package_lane_rows[].metrics",
            "Show whether a Forge package-lane row carries dx-style compatibility evidence.",
        ),
        studio_marker(
            "data-dx-check-package-lane-hash-refresh-status",
            "dx-check-package-lane-hash-refresh-status",
            &["/"],
            "check_panel.view_model.package_lane_rows[].receipt_hash_refresh.status",
            "Show package-owned receipt-hash helper freshness beside dx-check package-lane metrics.",
        ),
        studio_marker(
            "data-dx-check-package-lane-hash-refresh-helper",
            "dx-check-package-lane-hash-refresh-helper",
            &["/"],
            "check_panel.view_model.package_lane_rows[].receipt_hash_refresh.helper_path",
            "Open the package-owned helper that checks selected receipt hashes.",
        ),
        studio_marker(
            "data-dx-check-package-lane-hash-refresh-json-command",
            "dx-check-package-lane-hash-refresh-json-command",
            &["/"],
            "check_panel.view_model.package_lane_rows[].receipt_hash_refresh.json_check_command",
            "Show the JSON helper command Studio and Zed can run after reviewed source edits.",
        ),
        studio_marker(
            "data-dx-check-package-lane-hash-refresh-zed",
            "dx-check-package-lane-hash-refresh-zed",
            &["/"],
            "check_panel.view_model.package_lane_rows[].receipt_hash_refresh.zed_visibility",
            "Expose the Zed helper surface for package-owned receipt freshness.",
        ),
        studio_marker(
            "data-dx-check-package-lane-hash-refresh-tracked-files",
            "dx-check-package-lane-hash-refresh-tracked-files",
            &["/"],
            "check_panel.view_model.package_lane_rows[].receipt_hash_refresh.tracked_file_count",
            "Show how many selected files the package-owned receipt helper tracks.",
        ),
        studio_marker(
            "data-dx-check-package-lane-hash-refresh-stale-files",
            "dx-check-package-lane-hash-refresh-stale-files",
            &["/"],
            "check_panel.view_model.package_lane_rows[].receipt_hash_refresh.stale_file_count",
            "Show stale selected-file count beside dx-check package-lane metrics.",
        ),
        studio_marker(
            "data-dx-check-package-lane-hash-refresh-missing-files",
            "dx-check-package-lane-hash-refresh-missing-files",
            &["/"],
            "check_panel.view_model.package_lane_rows[].receipt_hash_refresh.missing_file_count",
            "Show missing selected-file count beside dx-check package-lane metrics.",
        ),
        studio_marker(
            "data-dx-check-package-lane-hash-refresh-current-file-list",
            "dx-check-package-lane-hash-refresh-current-file-list",
            &["/"],
            "check_panel.view_model.package_lane_rows[].receipt_hash_refresh.current_files",
            "Expose exact current helper file paths for package-owned receipt freshness.",
        ),
        studio_marker(
            "data-dx-check-package-lane-hash-refresh-stale-file-list",
            "dx-check-package-lane-hash-refresh-stale-file-list",
            &["/"],
            "check_panel.view_model.package_lane_rows[].receipt_hash_refresh.stale_files",
            "Expose exact stale helper file paths for package-owned receipt freshness.",
        ),
        studio_marker(
            "data-dx-check-package-lane-hash-refresh-missing-file-list",
            "dx-check-package-lane-hash-refresh-missing-file-list",
            &["/"],
            "check_panel.view_model.package_lane_rows[].receipt_hash_refresh.missing_files",
            "Expose exact missing helper file paths for package-owned receipt freshness.",
        ),
        studio_marker(
            "data-dx-check-package-lane-hash-refresh-stale-mirror-file-list",
            "dx-check-package-lane-hash-refresh-stale-mirror-file-list",
            &["/"],
            "check_panel.view_model.package_lane_rows[].receipt_hash_refresh.stale_mirror_files",
            "Expose exact stale source-mirror file paths for package-owned receipt freshness.",
        ),
        studio_marker(
            "data-dx-check-package-lane-hash-refresh-missing-mirror-file-list",
            "dx-check-package-lane-hash-refresh-missing-mirror-file-list",
            &["/"],
            "check_panel.view_model.package_lane_rows[].receipt_hash_refresh.missing_mirror_files",
            "Expose exact missing source-mirror file paths for package-owned receipt freshness.",
        ),
        studio_marker(
            "data-dx-check-package-lane-hash-refresh-current-metric",
            "dx-check-package-lane-hash-refresh-current-metric",
            &["/"],
            "check_panel.view_model.package_lane_rows[].metrics",
            "Expose the metric name that marks the package-owned receipt helper current.",
        ),
        studio_marker(
            "data-dx-check-package-lane-hash-refresh-stale-metric",
            "dx-check-package-lane-hash-refresh-stale-metric",
            &["/"],
            "check_panel.view_model.package_lane_rows[].metrics",
            "Expose the metric name that marks the package-owned receipt helper stale.",
        ),
        studio_marker(
            "data-dx-check-package-lane-hash-refresh-missing-metric",
            "dx-check-package-lane-hash-refresh-missing-metric",
            &["/"],
            "check_panel.view_model.package_lane_rows[].metrics",
            "Expose the metric name that marks the package-owned receipt helper missing.",
        ),
        studio_marker(
            "data-dx-check-blocker-count",
            "dx-check-blocker-count",
            &["/"],
            "check_panel.view_model.blocker_rows",
            "Show how many blocker rows the latest dx-check receipt can render.",
        ),
        studio_marker(
            "data-dx-check-warning-count",
            "dx-check-warning-count",
            &["/"],
            "check_panel.view_model.warning_rows",
            "Show how many warning rows the latest dx-check receipt can render.",
        ),
        studio_marker(
            "data-dx-check-quick-fix-count",
            "dx-check-quick-fix-count",
            &["/"],
            "check_panel.view_model.quick_fix_rows",
            "Show how many quick-fix rows the latest dx-check receipt can render.",
        ),
        studio_marker(
            "data-dx-check-last-run",
            "dx-check-last-run",
            &["/"],
            "check_panel.view_model.last_run_unix_ms",
            "Show the last dx-check run timestamp when a receipt is available.",
        ),
        studio_marker(
            "data-dx-zed-panel-schema",
            "dx-check-zed-panel-schema",
            &["/"],
            "check_panel.embedded_zed_schema",
            "Show the embedded Zed-facing panel schema for future GPUI rendering.",
        ),
        studio_marker(
            "data-dx-safety-archive-contract",
            "forge-safety-archive-contract",
            &["/"],
            "studio_edit_contract.surfaces",
            "Show the Forge safety/archive contract for receipt-less rollback coverage discovery.",
        ),
        studio_marker(
            "data-dx-safety-archive-state",
            "forge-safety-archive-state",
            &["/"],
            "studio_edit_contract.surfaces",
            "Show whether archive-before-delete rollback coverage is ready, partial, missing, or source-only.",
        ),
        studio_marker(
            "data-dx-safety-archive-safe-delete",
            "forge-safety-archive-safe-delete",
            &["/"],
            "studio_edit_contract.surfaces",
            "Show whether destructive Forge package paths are safe to run without extra operator review.",
        ),
        studio_marker(
            "data-dx-safety-archive-package-count",
            "forge-safety-archive-package-count",
            &["/"],
            "studio_edit_contract.surfaces",
            "Show how many Forge package slices are represented in safety/archive coverage.",
        ),
        studio_marker(
            "data-dx-safety-archive-covered-packages",
            "forge-safety-archive-covered-packages",
            &["/"],
            "studio_edit_contract.surfaces",
            "Show how many tracked Forge package slices have archive-backed rollback coverage.",
        ),
        studio_marker(
            "data-dx-safety-archive-missing-packages",
            "forge-safety-archive-missing-packages",
            &["/"],
            "studio_edit_contract.surfaces",
            "Show how many tracked Forge package slices still lack archive-backed rollback coverage.",
        ),
        studio_marker(
            "data-dx-safety-archive-rollback-coverage",
            "forge-safety-archive-rollback-coverage",
            &["/"],
            "studio_edit_contract.surfaces",
            "Show the receipt-backed rollback coverage percentage before a fresh package-status receipt loads.",
        ),
        studio_marker(
            "data-dx-safety-archive-receipt-count",
            "forge-safety-archive-receipt-count",
            &["/"],
            "studio_edit_contract.surfaces",
            "Show how many archive-before-delete receipts are available for rollback review.",
        ),
        studio_marker(
            "data-dx-safety-archive-directory",
            "forge-safety-archive-directory",
            &["/"],
            "studio_edit_contract.surfaces",
            "Open the local receipt directory Studio should inspect for archive-before-delete evidence.",
        ),
        studio_marker(
            "data-dx-safety-archive-boundary",
            "forge-safety-archive-boundary",
            &["/"],
            "studio_edit_contract.surfaces",
            "Keep rollback coverage honest when the static runtime row is source-only and not live restore proof.",
        ),
        studio_marker(
            "data-dx-safety-archive-runbook-source",
            "forge-safety-archive-runbook-source",
            &["/"],
            "source_guard_runbook_index",
            "Show the generated preview-manifest file Studio should read for the rollback proof command.",
        ),
        studio_marker(
            "data-dx-safety-archive-runbook-fixture",
            "forge-safety-archive-runbook-fixture",
            &["/"],
            "source_guard_runbook_index.fixture_paths",
            "Open the source-owned safety/archive runbook fixture for rollback coverage proof.",
        ),
        studio_marker(
            "data-dx-safety-archive-runbook-guard",
            "forge-safety-archive-runbook-guard",
            &["/"],
            "source_guard_runbook_index.contracts",
            "Show the source guard id that owns the safety/archive rollback proof.",
        ),
        studio_marker(
            "data-dx-safety-archive-runbook-command",
            "forge-safety-archive-runbook-command",
            &["/"],
            "source_guard_runbook_index.commands",
            "Show the exact source-only command for validating safety/archive rollback coverage.",
        ),
        studio_marker(
            "data-dx-safety-archive-runbook-policy",
            "forge-safety-archive-runbook-policy",
            &["/"],
            "source_guard_runbook_index.commands",
            "Show that the rollback proof is source-only and does not run servers, installs, live restore, or live R2/S3 writes.",
        ),
        studio_marker(
            "data-dx-edit-contract",
            "studio-edit-contract",
            &["/"],
            "studio_edit_contract",
            "Open the source-owned edit manifest that defines allowed Zed Web Preview operations.",
        ),
        studio_marker(
            "data-dx-section",
            "studio-section",
            &["/", "/automations"],
            "editable_surface_index",
            "Map a preview selection to a route-level responsive section.",
        ),
        studio_marker(
            "data-dx-component",
            "studio-component",
            &["/", "/automations"],
            "editable_surface_index",
            "Map a preview selection to a source-owned launch component.",
        ),
        studio_marker(
            "data-dx-dashboard-workflow",
            "dashboard-workflow",
            &["/"],
            "routes[].package_surfaces",
            "Map a preview selection to the product workflow powered by a source-owned package slice.",
        ),
        studio_marker(
            "data-dx-motion-interaction",
            "motion-dashboard-action",
            &["/"],
            "editable_surface_index",
            "Map Motion advance, reorder, reset, and reduced-motion controls to the source-owned Motion proof component.",
        ),
        studio_marker(
            "data-dx-motion-reduced",
            "motion-reduced-policy",
            &["/"],
            "forge_receipt_index",
            "Show the app-owned reduced-motion preview state and Motion dashboard workflow receipt.",
        ),
        studio_marker(
            "data-dx-intl-dashboard-workflow",
            "intl-dashboard-workflow",
            &["/"],
            "routes[].package_surfaces",
            "Map next-intl locale/copy workflow selections to the source-owned dashboard workflow.",
        ),
        studio_marker(
            "data-dx-intl-action",
            "intl-dashboard-action",
            &["/"],
            "routes[].package_surfaces",
            "Map locale switch and receipt buttons to the next-intl dashboard source surface.",
        ),
        studio_marker(
            "data-dx-dashboard-copy-locale",
            "intl-copy-locale",
            &["/"],
            "routes[].package_surfaces",
            "Show the selected dashboard copy locale for mission-control and route preview copy.",
        ),
        studio_marker(
            "data-dx-intl-receipt-state",
            "intl-receipt-state",
            &["/"],
            "forge_receipt_index",
            "Map local locale receipt state to the source-owned next-intl receipt handoff.",
        ),
        studio_marker(
            "data-dx-intl-message-namespace",
            "intl-message-namespace",
            &["/"],
            "routes[].package_surfaces",
            "Map LaunchDashboard message namespace usage to the i18n package slice.",
        ),
        studio_marker(
            "data-dx-intl-copy-target",
            "intl-copy-target",
            &["/"],
            "source_selection_index.package_surfaces",
            "Map visible next-intl dashboard copy targets back to the locale workflow source.",
        ),
        studio_marker(
            "data-dx-intl-readiness-copy",
            "intl-readiness-copy",
            &["/"],
            "source_selection_index.package_surfaces",
            "Expose the current translated readiness copy on the dashboard locale card for Web Preview selection.",
        ),
        studio_marker(
            "data-dx-intl-locale-option",
            "intl-locale-option",
            &["/"],
            "source_selection_index.package_surfaces",
            "Map locale switch options to the source-owned next-intl dashboard action.",
        ),
        studio_marker(
            "data-dx-intl-preview-locale",
            "intl-preview-locale",
            &["/"],
            "source_selection_index.package_surfaces",
            "Show the selected preview locale used by the next-intl workflow.",
        ),
        studio_marker(
            "data-dx-intl-provider-locale",
            "intl-provider-locale",
            &["/"],
            "source_selection_index.package_surfaces",
            "Show the provider locale handed to the source-owned next-intl workflow.",
        ),
        studio_marker(
            "data-dx-intl-route-preview",
            "intl-route-preview",
            &["/"],
            "routes[].package_surfaces",
            "Map route preview copy back to the next-intl dashboard messages.",
        ),
        studio_marker(
            "data-dx-intl-format-preview",
            "intl-format-preview",
            &["/"],
            "routes[].package_surfaces",
            "Map formatted launch-window text back to the next-intl useFormatter.dateTime contract.",
        ),
        studio_marker(
            "data-dx-intl-format-source-api",
            "intl-format-source-api",
            &["/"],
            "routes[].package_surfaces",
            "Show which next-intl formatter API produced the visible dashboard preview.",
        ),
        studio_marker(
            "data-dx-intl-format-time-zone",
            "intl-format-time-zone",
            &["/"],
            "routes[].package_surfaces",
            "Show the explicit time zone used by the source-owned formatter preview.",
        ),
        studio_marker(
            "data-dx-intl-number-preview",
            "intl-number-preview",
            &["/"],
            "routes[].package_surfaces",
            "Map localized plan-price text back to the next-intl useFormatter.number contract.",
        ),
        studio_marker(
            "data-dx-intl-number-source-api",
            "intl-number-source-api",
            &["/"],
            "routes[].package_surfaces",
            "Show which next-intl number formatter API produced the visible plan-price preview.",
        ),
        studio_marker(
            "data-dx-intl-number-currency",
            "intl-number-currency",
            &["/"],
            "routes[].package_surfaces",
            "Show the explicit currency used by the source-owned number formatter preview.",
        ),
        studio_marker(
            "data-dx-intl-alternate-links",
            "intl-alternate-links",
            &["/"],
            "routes[].package_surfaces",
            "Map visible alternate-link review surfaces back to the next-intl dashboard route contract.",
        ),
        studio_marker(
            "data-dx-intl-alternate-locale",
            "intl-alternate-locale",
            &["/"],
            "routes[].package_surfaces",
            "Map alternate-link locale ownership back to the next-intl dashboard route contract.",
        ),
        studio_marker(
            "data-dx-intl-alternate-href",
            "intl-alternate-href",
            &["/"],
            "routes[].package_surfaces",
            "Map alternate-link href ownership back to the next-intl dashboard route contract.",
        ),
        studio_marker(
            "data-dx-intl-hreflang",
            "intl-route-hreflang",
            &["/"],
            "routes[].package_surfaces",
            "Map route preview hreflang back to the next-intl dashboard route contract.",
        ),
        studio_marker(
            "data-dx-intl-locale-prefix",
            "intl-route-locale-prefix",
            &["/"],
            "routes[].package_surfaces",
            "Map route preview prefix mode back to the next-intl dashboard route contract.",
        ),
        studio_marker(
            "data-dx-intl-plan-label",
            "intl-plan-label",
            &["/"],
            "routes[].package_surfaces",
            "Map plan label copy back to the next-intl dashboard messages.",
        ),
        studio_marker(
            "data-dx-intl-support-sla",
            "intl-support-sla",
            &["/"],
            "routes[].package_surfaces",
            "Map support SLA copy back to the next-intl dashboard messages.",
        ),
        studio_marker(
            "data-dx-intl-receipt-locale",
            "intl-receipt-locale",
            &["/"],
            "forge_receipt_index",
            "Map locale receipt output to the source-owned next-intl receipt handoff.",
        ),
        studio_marker(
            "data-dx-intl-receipt-route",
            "intl-receipt-route",
            &["/"],
            "forge_receipt_index",
            "Map locale receipt route output to the source-owned next-intl receipt handoff.",
        ),
        studio_marker(
            "data-dx-intl-receipt-hreflang",
            "intl-receipt-hreflang",
            &["/"],
            "forge_receipt_index",
            "Map locale receipt hreflang output to the source-owned next-intl receipt handoff.",
        ),
        studio_marker(
            "data-dx-zustand-store",
            "zustand-store",
            &["/"],
            "routes[].package_surfaces",
            "Map persisted Zustand store identity to the source-owned dashboard state workflow.",
        ),
        studio_marker(
            "data-dx-zustand-persist-key",
            "zustand-persist-key",
            &["/"],
            "routes[].package_surfaces",
            "Map browser persistence keys to the package-owned dashboard state boundary.",
        ),
        studio_marker(
            "data-dx-zustand-action",
            "zustand-action",
            &["/"],
            "routes[].package_surfaces",
            "Map dashboard state controls to Zustand action markers.",
        ),
        studio_marker(
            "data-dx-editable",
            "studio-editable-kind",
            &["/", "/automations"],
            "edit_operation_index",
            "Show the edit kind for text, layout, token, and media surfaces.",
        ),
        studio_marker(
            "data-dx-token-scope",
            "studio-design-token-scope",
            &["/", "/automations"],
            "edit_operation_index",
            "Scope design-token updates to the responsive launch template contract.",
        ),
        studio_marker(
            "data-dx-edit-id",
            "stable-edit-id",
            &["/"],
            "studio_edit_contract.surfaces",
            "Map selected preview elements to stable source-owned editable surfaces.",
        ),
        studio_marker(
            "data-dx-edit-kind",
            "edit-surface-kind",
            &["/"],
            "studio_edit_contract.surfaces",
            "Describe whether a selected surface is a section, proof card, package row, media slot, or route root.",
        ),
        studio_marker(
            "data-dx-edit-ops",
            "allowed-edit-operations",
            &["/"],
            "studio_edit_contract.operations",
            "List the edit operations allowed for a selected responsive source surface.",
        ),
        studio_marker(
            "data-dx-edit-order",
            "responsive-section-order",
            &["/"],
            "studio_edit_contract.surfaces",
            "Reorder sections by source order while preserving CSS grid/flex layout.",
        ),
        studio_marker(
            "data-dx-editable-section",
            "editable-section",
            &["/", "/automations"],
            "studio_edit_contract.surfaces",
            "Map a Web Preview section selection to an allowed source-owned edit surface.",
        ),
        studio_marker(
            "data-dx-insert-slot",
            "insert-slot",
            &["/", "/automations"],
            "studio_edit_contract.operations",
            "Allow component insertion only inside declared responsive section slots.",
        ),
        studio_marker(
            "data-dx-reorder-group",
            "reorder-group",
            &["/", "/automations"],
            "editable_surface_index",
            "Move sections only by source order inside declared responsive groups.",
        ),
        studio_marker(
            "data-dx-design-token",
            "design-token",
            &["/", "/automations"],
            "studio_edit_contract.operations",
            "Update named design-token class targets without absolute positioning.",
        ),
        studio_marker(
            "data-dx-content-key",
            "content-key",
            &["/"],
            "studio_edit_contract.operations",
            "Map copy and markdown updates to source-owned content keys.",
        ),
        studio_marker(
            "data-dx-editable-text",
            "editable-text",
            &["/", "/automations"],
            "studio_edit_contract.operations",
            "Update declared text content while preserving component structure.",
        ),
        studio_marker(
            "data-dx-media-slot",
            "media-slot",
            &["/"],
            "studio_edit_contract.operations",
            "Insert registered icon or media assets into declared source-owned slots.",
        ),
        studio_marker(
            "data-dx-automation-view",
            "automation-view",
            &["/", "/automations"],
            "routes[].package_surfaces",
            "Map the automation shell to overview, connector, credential, or workflow metadata.",
        ),
        studio_marker(
            "data-dx-automation-route",
            "automation-route",
            &["/", "/automations"],
            "integrations.n8n_automations.routes",
            "Open nested automation preview routes from the route navigation.",
        ),
        studio_marker(
            "data-dx-automation-dashboard-card",
            "automation-dashboard-card",
            &["/"],
            "routes[].package_surfaces",
            "Map the launch automation card to its source-owned n8n workflow readiness surface.",
        ),
        studio_marker(
            "data-dx-automation-intent-input",
            "automation-intent-input",
            &["/"],
            "routes[].package_surfaces",
            "Map release-notification intent edits back to the launch automation workflow source.",
        ),
        studio_marker(
            "data-dx-automation-receipt-intent",
            "automation-receipt-intent",
            &["/"],
            "routes[].package_surfaces",
            "Map the normalized local automation receipt intent back to the source-owned receipt helper.",
        ),
        studio_marker(
            "data-dx-automation-run-receipt-intent",
            "automation-run-receipt-intent",
            &["/"],
            "routes[].package_surfaces",
            "Map the normalized Zed run-handoff receipt intent back to the source-owned receipt helper.",
        ),
        studio_marker(
            "data-dx-automation-required-env",
            "automation-required-env",
            &["/", "/automations"],
            "routes[].package_surfaces",
            "Map connector-specific missing configuration back to the app-owned env boundary without reading secrets.",
        ),
        studio_marker(
            "data-dx-automation-receipt-path",
            "automation-receipt-path",
            &["/"],
            "forge_receipt_index",
            "Open the redacted automation receipt handoff path without reading provider secrets.",
        ),
        studio_marker(
            "data-dx-data-status",
            "data-boundary",
            &["/"],
            "routes[].package_surfaces",
            "Show Drizzle, Supabase, and InstantDB data boundary readiness.",
        ),
        studio_marker(
            "data-dx-drizzle-action",
            "drizzle-dashboard-action",
            &["/"],
            "editable_surface_index",
            "Map SQLite read-model selection, query-plan preview, and apply actions back to the Drizzle dashboard workflow source.",
        ),
        studio_marker(
            "data-dx-drizzle-read-model",
            "drizzle-read-model",
            &["/"],
            "routes[].package_surfaces",
            "Show the selected Drizzle dashboard read model without connecting to production SQLite.",
        ),
        studio_marker(
            "data-dx-drizzle-query-plan-id",
            "drizzle-query-plan-id",
            &["/"],
            "routes[].package_surfaces",
            "Show the selected Drizzle query-plan id used by the package-owned by-id selector.",
        ),
        studio_marker(
            "data-dx-drizzle-sql-preview",
            "drizzle-query-plan-preview",
            &["/"],
            "forge_receipt_index",
            "Map the SQL preview surface to the source-owned Drizzle query-plan receipt boundary.",
        ),
        studio_marker(
            "data-dx-drizzle-receipt-path",
            "drizzle-dashboard-receipt",
            &["/"],
            "forge_receipt_index",
            "Open the source-owned Drizzle dashboard workflow receipt without reading runtime evidence.",
        ),
        studio_marker(
            "data-dx-drizzle-runtime-dependencies",
            "drizzle-runtime-dependencies",
            &["/"],
            "routes[].package_surfaces",
            "Show the app-owned Drizzle runtime dependencies required before SQLite execution.",
        ),
        studio_marker(
            "data-dx-backend-status",
            "dashboard-backend-status",
            &["/"],
            "editable_surface_index",
            "Map backend mission-control status updates to source-owned package workflow surfaces.",
        ),
        studio_marker(
            "data-dx-backend-detail",
            "dashboard-backend-detail",
            &["/"],
            "editable_surface_index",
            "Map backend mission-control detail text to the package workflow that updated it.",
        ),
        studio_marker(
            "data-dx-trpc-workflow",
            "trpc-typed-api-workflow",
            &["/"],
            "editable_surface_index",
            "Map the launch Type-Safe API dashboard workflow back to the source-owned upstream tRPC account/data surface.",
        ),
        studio_marker(
            "data-dx-trpc-action",
            "trpc-dashboard-action",
            &["/"],
            "editable_surface_index",
            "Map Type-Safe API health query and launch event actions back to the upstream tRPC dashboard workflow source.",
        ),
        studio_marker(
            "data-trpc-interaction",
            "trpc-local-interaction",
            &["/"],
            "routes[].package_surfaces",
            "Show the safe local Type-Safe API query/mutation interactions without claiming a hosted runtime.",
        ),
        studio_marker(
            "data-trpc-mutation-state",
            "trpc-mutation-state",
            &["/"],
            "forge_receipt_index",
            "Map local launchEvent mutation state to the source-owned tRPC dashboard receipt.",
        ),
        studio_marker(
            "data-dx-trpc-receipt-state",
            "trpc-dashboard-receipt-state",
            &["/"],
            "forge_receipt_index",
            "Open the Type-Safe API dashboard workflow receipt state for Zed without reading runtime evidence.",
        ),
        studio_marker(
            "data-dx-trpc-request-id",
            "trpc-dashboard-request-id",
            &["/"],
            "routes[].package_surfaces",
            "Show the request id emitted by the safe local tRPC readiness interaction.",
        ),
        studio_marker(
            "data-dx-payment-status",
            "payment-boundary",
            &["/"],
            "routes[].package_surfaces",
            "Show Stripe.js public key and checkout adapter readiness.",
        ),
        studio_marker(
            "data-dx-checkout-submit-state",
            "form-submit-state",
            &["/"],
            "routes[].package_surfaces",
            "Map payment form submit state to app-owned checkout boundaries.",
        ),
        studio_marker(
            "data-launch-i18n-phase",
            "i18n-phase",
            &["/"],
            "routes[].package_surfaces",
            "Map next-intl locale phase readiness to the source-owned i18n boundary.",
        ),
        studio_marker(
            "data-dx-docs-status",
            "docs-boundary",
            &["/"],
            "routes[].package_surfaces",
            "Show Fumadocs and markdown source-owned documentation readiness.",
        ),
        studio_marker(
            "data-dx-instant-status",
            "realtime-boundary",
            &["/"],
            "routes[].package_surfaces",
            "Show InstantDB auth, storage, room, topic, and presence readiness.",
        ),
        studio_marker(
            "data-dx-zod-status",
            "validation-boundary",
            &["/"],
            "routes[].package_surfaces",
            "Show Zod validation, JSON Schema, and codec readiness.",
        ),
        studio_marker(
            "data-dx-scene-status",
            "scene-runtime",
            &["/"],
            "routes[].package_surfaces",
            "Show the source-owned WebGL scene runtime state.",
        ),
        studio_marker(
            "data-dx-scene-preview-readiness",
            "scene-preview-readiness",
            &["/"],
            "routes[].package_surfaces",
            "Show Web Preview-safe 3D scene readiness without a server-side build.",
        ),
        studio_marker(
            "data-dx-scene-quality-profile",
            "scene-quality-profile",
            &["/"],
            "routes[].package_surfaces",
            "Show the active 3D scene quality profile selected from the package-owned render budgets.",
        ),
        studio_marker(
            "data-dx-scene-material-palette",
            "scene-material-palette",
            &["/"],
            "routes[].package_surfaces",
            "Show the active source-owned 3D material palette selected from the launch dashboard.",
        ),
        studio_marker(
            "data-dx-scene-camera-rig",
            "scene-camera-rig",
            &["/"],
            "routes[].package_surfaces",
            "Show the active source-owned 3D camera rig selected from the launch dashboard.",
        ),
        studio_marker(
            "data-dx-scene-frame-sample",
            "scene-frame-sample",
            &["/"],
            "routes[].package_surfaces",
            "Show the latest local 3D canvas frame sample captured by the source-owned launch scene.",
        ),
        studio_marker(
            "data-dx-scene-capability-report",
            "scene-capability-report",
            &["/"],
            "routes[].package_surfaces",
            "Show the latest WebGL context capability report captured by the source-owned launch scene.",
        ),
        studio_marker(
            "data-dx-scene-capability-status",
            "scene-capability-status",
            &["/"],
            "routes[].package_surfaces",
            "Show whether the launch scene renderer capability inspection is ready, missing, or unavailable.",
        ),
        studio_marker(
            "data-dx-scene-viewport-report",
            "scene-viewport-report",
            &["/"],
            "routes[].package_surfaces",
            "Show the latest viewport and DPR budget report from the source-owned launch scene.",
        ),
        studio_marker(
            "data-dx-scene-viewport-status",
            "scene-viewport-status",
            &["/"],
            "routes[].package_surfaces",
            "Show whether the launch scene viewport/DPR report is ready or missing.",
        ),
        studio_marker(
            "data-dx-scene-bounds-report",
            "scene-bounds-report",
            &["/"],
            "routes[].package_surfaces",
            "Show the latest Box3-style scene bounds and camera fit report from the launch scene.",
        ),
        studio_marker(
            "data-dx-scene-bounds-status",
            "scene-bounds-status",
            &["/"],
            "routes[].package_surfaces",
            "Show whether the launch scene bounds fit report is ready or pending.",
        ),
        studio_marker(
            "data-dx-scene-raycast-report",
            "scene-raycast-report",
            &["/"],
            "routes[].package_surfaces",
            "Show the latest Raycaster-style scene hit report from the source-owned launch scene.",
        ),
        studio_marker(
            "data-dx-scene-raycast-status",
            "scene-raycast-status",
            &["/"],
            "routes[].package_surfaces",
            "Show whether the launch scene raycast hit report found a node or has a pending scene.",
        ),
        studio_marker(
            "data-visual-audit",
            "conversion-visual-audit",
            &["/ui", "/database", "/backend"],
            "examples/conversion-proof/forge/visual-audits/*.json",
            "Open visual audit proof for converted website routes.",
        ),
    ]
}

fn studio_edit_contract() -> Value {
    json!({
        "schema": "dx.studio.launch_edit_contract",
        "route": "/",
        "source_manifest": "examples/template/dx-studio-edit-contract.ts",
        "source_manifest_file": "examples/template/dx-studio-edit-contract.ts",
        "materialized_manifest": "components/template-app/dx-studio-edit-contract.ts",
        "materialized_manifest_file": "components/template-app/dx-studio-edit-contract.ts",
        "source_guard": "dx run --test .\\benchmarks\\dx-studio-preview-manifest.test.ts",
        "edit_policy": "source-owned-edit-contract-explicit-user-action",
        "selector_policy": "stable-data-dx-selectors-source-owned-files-only",
        "layout_policy": "responsive-design-system-grid",
        "no_node_modules_required": true,
        "absolute_positioning": false,
        "operation_ids": [
            "insert_component",
            "move_reorder_section",
            "update_design_token",
            "update_text_content",
            "insert_icon_media"
        ],
        "operations": [
            studio_edit_operation(
                "insert_component",
                "data-dx-insert-slot",
                "Insert a Forge-owned component into a declared responsive section slot.",
            ),
            studio_edit_operation(
                "move_reorder_section",
                "data-dx-editable-section",
                "Reorder declared sections by updating source order instead of absolute coordinates.",
            ),
            studio_edit_operation(
                "update_design_token",
                "data-dx-design-token",
                "Update named design-token classes on declared token targets.",
            ),
            studio_edit_operation(
                "update_text_content",
                "data-dx-editable-text",
                "Update content inside source-owned text markers while preserving component structure.",
            ),
            studio_edit_operation(
                "insert_icon_media",
                "data-dx-media-slot",
                "Insert a registered icon or media asset into a declared media slot.",
            ),
        ],
        "surfaces": route_editable_surfaces("/")
    })
}

fn studio_edit_operation(
    id: &'static str,
    marker: &'static str,
    description: &'static str,
) -> Value {
    json!({
        "id": id,
        "operation": id,
        "marker": marker,
        "required_marker": marker,
        "description": description,
        "layout_policy": "responsive-design-system-grid",
        "absolute_positioning": false,
        "writes_files": true,
        "writes_only_source_owned_files": true,
        "requires_node_modules": false,
        "requires_server_restart": false,
        "requires_package_install": false,
        "requires_explicit_user_action": true
    })
}

fn studio_edit_surface(
    id: &'static str,
    selector: &'static str,
    source_file: &'static str,
    materialized_file: &'static str,
    package_ids: &'static [&'static str],
    operations: &'static [&'static str],
    responsive_policy: &'static str,
) -> Value {
    json!({
        "id": id,
        "selector": selector,
        "source_file": source_file,
        "materialized_file": materialized_file,
        "package_ids": package_ids,
        "operations": operations,
        "responsive_policy": responsive_policy,
        "layout_policy": "responsive-design-system-grid",
        "no_node_modules_required": true,
        "absolute_positioning": false
    })
}

fn studio_dx_check_panel_contract() -> Value {
    json!({
        "schema": "dx.studio.check_panel_contract",
        "panel_schema": "dx.www.check_panel",
        "view_model_schema": "dx.www.check_panel_view_model",
        "embedded_zed_schema": "dx.check.zed_panel",
        "command": "dx check --latest-receipt --json",
        "refresh_command": "dx check --json",
        "detail_command": "dx check score --json",
        "receipt_path": ".dx/receipts/check/check-latest.json",
        "reader_api": "dx_compiler::ecosystem::read_dx_check_latest_panel",
        "score_model": "500-point",
        "score_max": 500,
        "renders": [
            "score_meter",
            "bucket_breakdown",
            "package_lane_rows",
            "style_evidence_rows",
            "blockers",
            "warnings",
            "quick_fixes",
            "last_run_time"
        ],
        "missing_state": "Run `dx check --json` from the project root to create check-latest.json.",
        "malformed_state": "Re-run `dx check --json`; update the DX-WWW receipt reader if schema drift persists.",
        "runs_expensive_checks": false,
        "starts_server": false,
        "runs_package_install": false,
        "runs_full_build": false,
        "writes_receipts": false
    })
}

fn studio_dx_check_edit_surface() -> Value {
    let mut surface = studio_edit_surface(
        "dx-check-health-panel",
        "[data-dx-component=\"dx-check-health-panel\"]",
        "examples/template/template-shell.tsx",
        "components/template-app/template-shell.tsx",
        &[
            "dx-www/template-shell",
            "shadcn/ui/button",
            "reactive/store",
            "payments/stripe-js",
            "validation/zod",
            "forms/react-hook-form",
            "automations/n8n",
            "supabase/client",
            "instantdb/react",
            "api/trpc",
            "wasm/bindgen",
            "3d/launch-scene",
            "content/fumadocs-next",
        ],
        &["move_reorder_section", "update_text_content"],
        "responsive-section-grid",
    );

    if let Some(surface) = surface.as_object_mut() {
        surface.insert(
            "interaction_selectors".to_string(),
            json!([
                "[data-dx-check-command=\"dx check --latest-receipt --json\"]",
                "[data-dx-check-panel=\"latest-receipt\"]"
            ]),
        );
        surface.insert(
            "state_markers".to_string(),
            json!([
                "data-dx-check-panel",
                "data-dx-check-command",
                "data-dx-check-receipt-path",
                "data-dx-check-schema",
                "data-dx-check-score-max",
                "data-dx-check-view-model-schema",
                "data-dx-check-view-model-status",
                "data-dx-check-score-state",
                "data-dx-check-empty-state",
                "data-dx-check-bucket-count",
                "data-dx-check-package-lane-count",
                "data-dx-check-style-evidence-count",
                "data-dx-check-style-evidence-row",
                "data-dx-check-style-evidence-status",
                "data-dx-check-style-evidence-receipt-path",
                "data-dx-check-style-evidence-fixture-path",
                "data-dx-check-style-evidence-zed",
                "data-dx-check-style-evidence-class-count",
                "data-dx-check-style-evidence-selector-class-count",
                "data-dx-check-style-evidence-selector-class-examples",
                "data-dx-check-style-evidence-state-alias-count",
                "data-dx-check-style-evidence-state-alias-examples",
                "data-dx-check-style-evidence-full-autoprefixer-parity",
                "data-dx-check-style-evidence-full-tailwind-postcss-output-parity",
                "data-dx-check-style-evidence-drift",
                "data-dx-check-style-evidence-drift-state",
                "data-dx-check-style-evidence-drift-loader",
                "data-dx-check-style-evidence-drift-helper",
                "data-dx-check-style-evidence-drift-states",
                "data-dx-style-package-panel",
                "data-dx-style-package-panel-read-model",
                "data-dx-style-package-panel-drift-state",
                "data-dx-style-package-panel-drift-status",
                "data-dx-style-package-panel-drift-mismatch-fields",
                "data-dx-style-package-panel-readiness-receipt",
                "data-dx-style-package-ownership-read-model",
                "data-dx-style-package-ownership-packages",
                "data-dx-style-package-ownership-generated-classes",
                "data-dx-style-package-ownership-unsupported-classes",
                "data-dx-check-package-lane-row",
                "data-dx-check-package-lane-status",
                "data-dx-check-package-lane-receipt-status",
                "data-dx-check-package-lane-name",
                "data-dx-check-package-lane-upstream-package",
                "data-dx-check-package-lane-source-mirror",
                "data-dx-check-package-lane-receipt-path",
                "data-dx-check-package-lane-dx-style-status",
                "data-dx-check-package-lane-hash-refresh-status",
                "data-dx-check-package-lane-hash-refresh-helper",
                "data-dx-check-package-lane-hash-refresh-json-command",
                "data-dx-check-package-lane-hash-refresh-zed",
                "data-dx-check-package-lane-hash-refresh-tracked-files",
                "data-dx-check-package-lane-hash-refresh-stale-files",
                "data-dx-check-package-lane-hash-refresh-missing-files",
                "data-dx-check-package-lane-hash-refresh-current-file-list",
                "data-dx-check-package-lane-hash-refresh-stale-file-list",
                "data-dx-check-package-lane-hash-refresh-missing-file-list",
                "data-dx-check-package-lane-hash-refresh-stale-mirror-file-list",
                "data-dx-check-package-lane-hash-refresh-missing-mirror-file-list",
                "data-dx-check-package-lane-hash-refresh-current-metric",
                "data-dx-check-package-lane-hash-refresh-stale-metric",
                "data-dx-check-package-lane-hash-refresh-missing-metric",
                "data-dx-style-surface",
                "data-dx-token-scope",
                "data-dx-check-package-lane-hash-refresh-status",
                "data-dx-check-package-lane-hash-refresh-helper",
                "data-dx-check-package-lane-hash-refresh-json-command",
                "data-dx-check-package-lane-hash-refresh-zed",
                "data-dx-check-package-lane-hash-refresh-tracked-files",
                "data-dx-check-package-lane-hash-refresh-stale-files",
                "data-dx-check-package-lane-hash-refresh-missing-files",
                "data-dx-check-package-lane-hash-refresh-current-file-list",
                "data-dx-check-package-lane-hash-refresh-stale-file-list",
                "data-dx-check-package-lane-hash-refresh-missing-file-list",
                "data-dx-check-package-lane-hash-refresh-stale-mirror-file-list",
                "data-dx-check-package-lane-hash-refresh-missing-mirror-file-list",
                "data-dx-check-package-lane-hash-refresh-current-metric",
                "data-dx-check-package-lane-hash-refresh-stale-metric",
                "data-dx-check-package-lane-hash-refresh-missing-metric",
                "data-dx-check-blocker-count",
                "data-dx-check-warning-count",
                "data-dx-check-quick-fix-count",
                "data-dx-check-last-run",
                "data-dx-zed-panel-schema"
            ]),
        );
        surface.insert(
            "receipt_path".to_string(),
            json!(".dx/receipts/check/check-latest.json"),
        );
    }

    surface
}

fn studio_forge_safety_archive_edit_surface() -> Value {
    let mut surface = studio_edit_surface(
        "forge-safety-archive-status",
        "[data-dx-component=\"forge-safety-archive-status\"]",
        "tools/launch/runtime-template/pages/index.html",
        "pages/index.html",
        &["dx-www/template-shell"],
        &["move_reorder_section", "update_text_content"],
        "responsive-section-grid",
    );

    if let Some(surface) = surface.as_object_mut() {
        surface.insert(
            "state_markers".to_string(),
            json!([
                "data-dx-safety-archive-contract",
                "data-dx-forge-status-surface",
                "data-dx-zed-surface",
                "data-dx-safety-archive-operation",
                "data-dx-safety-archive-state",
                "data-dx-safety-archive-safe-delete",
                "data-dx-safety-archive-package-count",
                "data-dx-safety-archive-covered-packages",
                "data-dx-safety-archive-missing-packages",
                "data-dx-safety-archive-rollback-coverage",
                "data-dx-safety-archive-receipt-count",
                "data-dx-safety-archive-directory",
                "data-dx-safety-archive-boundary",
                "data-dx-safety-archive-runbook-source",
                "data-dx-safety-archive-runbook-fixture",
                "data-dx-safety-archive-runbook-guard",
                "data-dx-safety-archive-runbook-command",
                "data-dx-safety-archive-runbook-policy"
            ]),
        );
        surface.insert(
            "receipt_path".to_string(),
            json!(".dx/forge/receipts/safety"),
        );
    }

    surface
}

fn studio_motion_edit_surface(
    id: &'static str,
    selector: &'static str,
    source_file: &'static str,
    materialized_file: &'static str,
    operations: &'static [&'static str],
    responsive_policy: &'static str,
    include_policy_status_marker: bool,
) -> Value {
    let mut surface = studio_edit_surface(
        id,
        selector,
        source_file,
        materialized_file,
        &["animation/motion"],
        operations,
        responsive_policy,
    );

    if let Some(surface) = surface.as_object_mut() {
        let state_markers = if include_policy_status_marker {
            json!([
                "data-dx-motion-state",
                "data-dx-motion-progress",
                "data-dx-motion-order",
                "data-dx-motion-order-available",
                "data-dx-motion-keyboard-reorder",
                "data-dx-motion-keyboard-state",
                "data-dx-motion-preference-storage",
                "data-dx-motion-storage-key",
                "data-dx-motion-reduced",
                "data-dx-motion-policy-status"
            ])
        } else {
            json!([
                "data-dx-motion-state",
                "data-dx-motion-progress",
                "data-dx-motion-order",
                "data-dx-motion-order-available",
                "data-dx-motion-keyboard-reorder",
                "data-dx-motion-keyboard-state",
                "data-dx-motion-preference-storage",
                "data-dx-motion-storage-key",
                "data-dx-motion-reduced"
            ])
        };

        surface.insert(
            "interaction_selectors".to_string(),
            json!([
                "[data-dx-motion-interaction=\"advance-stage\"]",
                "[data-dx-motion-interaction=\"reverse-order\"]",
                "[data-dx-motion-interaction=\"move-stage-previous\"]",
                "[data-dx-motion-interaction=\"move-stage-next\"]",
                "[data-dx-motion-interaction=\"reset-proof\"]",
                "[data-dx-motion-interaction=\"toggle-reduced-motion\"]"
            ]),
        );
        surface.insert("state_markers".to_string(), state_markers);
        surface.insert(
            "receipt_path".to_string(),
            json!("examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json"),
        );
    }

    surface
}

fn studio_drizzle_edit_surface(
    id: &'static str,
    selector: &'static str,
    source_file: &'static str,
    materialized_file: &'static str,
    operations: &'static [&'static str],
    responsive_policy: &'static str,
) -> Value {
    let mut surface = studio_edit_surface(
        id,
        selector,
        source_file,
        materialized_file,
        &["db/drizzle-sqlite"],
        operations,
        responsive_policy,
    );

    if let Some(surface) = surface.as_object_mut() {
        surface.insert(
            "interaction_selectors".to_string(),
            json!([
                "[data-dx-drizzle-action=\"select-read-model\"]",
                "[data-dx-drizzle-action=\"preview-query-plan\"]",
                "[data-dx-drizzle-action=\"apply-read-model\"]",
                "[data-dx-dashboard-target=\"mission-control-database\"]"
            ]),
        );
        surface.insert(
            "state_markers".to_string(),
            json!([
                "data-dx-drizzle-status",
                "data-dx-drizzle-read-model",
                "data-dx-drizzle-query-plan-id",
                "data-dx-backend-status",
                "data-dx-backend-detail",
                "data-dx-drizzle-receipt-path",
                "data-dx-drizzle-receipt-state",
                "data-dx-drizzle-runtime-dependencies"
            ]),
        );
        surface.insert(
            "receipt_path".to_string(),
            json!("examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json"),
        );
    }

    surface
}

fn studio_trpc_edit_surface(
    id: &'static str,
    selector: &'static str,
    source_file: &'static str,
    materialized_file: &'static str,
    operations: &'static [&'static str],
    responsive_policy: &'static str,
) -> Value {
    let mut surface = studio_edit_surface(
        id,
        selector,
        source_file,
        materialized_file,
        &["api/trpc"],
        operations,
        responsive_policy,
    );

    if let Some(surface) = surface.as_object_mut() {
        surface.insert(
            "interaction_selectors".to_string(),
            json!([
                "[data-dx-trpc-action=\"check-health\"]",
                "[data-dx-trpc-action=\"prepare-launch-event\"]",
                "[data-trpc-interaction=\"health-query\"]",
                "[data-trpc-interaction=\"local-launch-event-mutation\"]"
            ]),
        );
        surface.insert(
            "state_markers".to_string(),
            json!([
                "data-dx-trpc-workflow",
                "data-dx-trpc-action",
                "data-trpc-interaction",
                "data-trpc-mutation-state",
                "data-dx-trpc-receipt-state",
                "data-dx-trpc-request-id"
            ]),
        );
        surface.insert(
            "receipt_path".to_string(),
            json!(
                "examples/template/.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json"
            ),
        );
    }

    surface
}

fn studio_editable_surface_index(routes: &[Value]) -> Vec<Value> {
    routes
        .iter()
        .filter_map(|route| route.get("route").and_then(Value::as_str))
        .flat_map(route_editable_surfaces)
        .collect()
}

fn route_editable_surfaces(route_path: &str) -> Vec<Value> {
    match route_path {
        "/" => vec![
            studio_edit_surface(
                "launch-hero",
                "[data-dx-section=\"hero\"]",
                "examples/template/template-shell.tsx",
                "components/template-app/template-shell.tsx",
                &[
                    "dx-www/template-shell",
                    "animation/motion",
                    "3d/launch-scene",
                ],
                &[
                    "move_reorder_section",
                    "update_design_token",
                    "update_text_content",
                    "insert_icon_media",
                ],
                "responsive-section-grid",
            ),
            studio_edit_surface(
                "launch-studio-proof-flow",
                "[data-dx-section=\"studio-proof-flow\"]",
                "examples/template/template-shell.tsx",
                "components/template-app/template-shell.tsx",
                &["dx-www/template-shell", "automations/n8n"],
                &[
                    "insert_component",
                    "move_reorder_section",
                    "update_design_token",
                    "update_text_content",
                    "insert_icon_media",
                ],
                "responsive-section-grid",
            ),
            studio_dx_check_edit_surface(),
            studio_forge_safety_archive_edit_surface(),
            studio_edit_surface(
                "launch-connected-capability-map",
                "[data-dx-component=\"launch-connected-capability-map\"]",
                "examples/template/template-shell.tsx",
                "components/template-app/template-shell.tsx",
                &[
                    "auth/better-auth",
                    "state/zustand",
                    "tanstack/query",
                    "forms/react-hook-form",
                    "validation/zod",
                    "payments/stripe-js",
                    "3d/launch-scene",
                    "animation/motion",
                    "content/fumadocs-next",
                    "content/react-markdown",
                    "automations/n8n",
                    "supabase/client",
                    "db/drizzle-sqlite",
                    "instantdb/react",
                    "api/trpc",
                ],
                &[
                    "insert_component",
                    "move_reorder_section",
                    "update_text_content",
                ],
                "capability-map",
            ),
            studio_edit_surface(
                "launch-proof-grid",
                "[data-dx-section=\"proof-grid\"]",
                "examples/template/template-shell.tsx",
                "components/template-app/template-shell.tsx",
                &[
                    "auth/better-auth",
                    "tanstack/query",
                    "state/zustand",
                    "validation/zod",
                    "forms/react-hook-form",
                    "payments/stripe-js",
                    "supabase/client",
                    "db/drizzle-sqlite",
                    "instantdb/react",
                    "automations/n8n",
                ],
                &[
                    "insert_component",
                    "move_reorder_section",
                    "update_design_token",
                    "update_text_content",
                ],
                "proof-card-grid",
            ),
            studio_edit_surface(
                "launch-auth-session-proof",
                "[data-dx-component=\"auth-session-proof\"]",
                "examples/template/template-shell.tsx",
                "components/template-app/template-shell.tsx",
                &["auth/better-auth"],
                &["move_reorder_section", "update_text_content"],
                "proof-card-grid",
            ),
            studio_edit_surface(
                "launch-form-validation-proof",
                "[data-dx-component=\"form-zod-proof\"]",
                "examples/template/template-shell.tsx",
                "components/template-app/template-shell.tsx",
                &["forms/react-hook-form", "validation/zod"],
                &["move_reorder_section", "update_text_content"],
                "proof-card-grid",
            ),
            studio_edit_surface(
                "next-intl-dashboard-locale-workflow",
                "[data-dx-component=\"next-intl-dashboard-locale-workflow\"]",
                "examples/template/next-intl-dashboard-locale.tsx",
                "components/template-app/next-intl-dashboard-locale.tsx",
                &["i18n/next-intl"],
                &[
                    "move_reorder_section",
                    "update_design_token",
                    "update_text_content",
                    "insert_icon_media",
                ],
                "locale-copy-dashboard",
            ),
            studio_edit_surface(
                "launch-billing-checkout-workflow",
                "[data-dx-component=\"launch-billing-checkout-workflow\"]",
                "examples/template/payments-status.tsx",
                "components/template-app/payments-status.tsx",
                &["payments/stripe-js"],
                &["move_reorder_section", "update_text_content"],
                "billing-workflow",
            ),
            studio_motion_edit_surface(
                "motion-dashboard-workflow",
                "[data-dx-component=\"launch-motion-dashboard-workflow\"]",
                "examples/template/template-shell.tsx",
                "components/template-app/template-shell.tsx",
                &[
                    "move_reorder_section",
                    "update_design_token",
                    "update_text_content",
                    "insert_icon_media",
                ],
                "motion-panel-orchestration",
                false,
            ),
            studio_motion_edit_surface(
                "motion-interaction-proof",
                "[data-dx-component=\"motion-interaction-proof\"]",
                "examples/template/motion-interaction-proof.tsx",
                "components/template-app/motion-interaction-proof.tsx",
                &[
                    "move_reorder_section",
                    "update_design_token",
                    "update_text_content",
                ],
                "motion-panel-orchestration",
                true,
            ),
            studio_edit_surface(
                "launch-automation-dashboard-workflow",
                "[data-dx-component=\"launch-automation-dashboard-workflow\"]",
                "examples/template/template-shell.tsx",
                "components/template-app/template-shell.tsx",
                &["automations/n8n"],
                &["move_reorder_section", "update_text_content"],
                "launch-main",
            ),
            studio_edit_surface(
                "launch-automation-mission-summary",
                "[data-dx-component=\"launch-automation-mission-summary\"]",
                "examples/template/automation-mission-summary.tsx",
                "components/template-app/automation-mission-summary.tsx",
                &["automations/n8n"],
                &["move_reorder_section", "update_text_content"],
                "launch-main",
            ),
            studio_edit_surface(
                "launch-database-backend-proof",
                "[data-dx-component=\"database-backend-proof\"]",
                "examples/template/template-shell.tsx",
                "components/template-app/template-shell.tsx",
                &[
                    "supabase/client",
                    "db/drizzle-sqlite",
                    "instantdb/react",
                    "api/trpc",
                ],
                &["move_reorder_section", "update_text_content"],
                "proof-card-grid",
            ),
            studio_drizzle_edit_surface(
                "launch-drizzle-data-workflow",
                "[data-dx-component=\"launch-drizzle-data-workflow\"]",
                "examples/template/drizzle-query-proof.tsx",
                "components/template-app/drizzle-query-proof.tsx",
                &["move_reorder_section", "update_text_content"],
                "sqlite-read-model-dashboard",
            ),
            studio_trpc_edit_surface(
                "launch-trpc-api-dashboard-workflow",
                "[data-dx-component=\"launch-trpc-api-dashboard-workflow\"]",
                "examples/template/template-shell.tsx",
                "components/template-app/template-shell.tsx",
                &[
                    "move_reorder_section",
                    "update_text_content",
                    "insert_icon_media",
                ],
                "typed-api-readiness",
            ),
            studio_edit_surface(
                "launch-trpc-health-workflow",
                "[data-dx-component=\"trpc-launch-health-workflow\"]",
                "examples/template/trpc-launch-health.tsx",
                "components/template-app/trpc-launch-health.tsx",
                &["api/trpc"],
                &[
                    "move_reorder_section",
                    "update_design_token",
                    "update_text_content",
                ],
                "typed-api-health-workflow",
            ),
            studio_edit_surface(
                "launch-scene-rendering-proof",
                "[data-dx-media-slot=\"launch-scene\"]",
                "examples/template/template-shell.tsx",
                "components/template-app/template-shell.tsx",
                &["3d/launch-scene", "animation/motion"],
                &["update_design_token", "insert_icon_media"],
                "media-slot",
            ),
            studio_edit_surface(
                "launch-scene-dashboard-workflow",
                "[data-dx-component=\"launch-scene-dashboard-workflow\"]",
                "examples/template/launch-scene.tsx",
                "components/scene/launch-scene.tsx",
                &["3d/launch-scene"],
                &[
                    "move_reorder_section",
                    "update_text_content",
                    "insert_icon_media",
                ],
                "visual-operations-dashboard",
            ),
            studio_edit_surface(
                "launch-docs-content",
                "[data-dx-component=\"launch-fumadocs-docs-workflow\"]",
                "examples/template/docs-status.tsx",
                "components/template-app/docs-status.tsx",
                &["content/fumadocs-next", "content/react-markdown"],
                &[
                    "insert_component",
                    "move_reorder_section",
                    "update_text_content",
                    "insert_icon_media",
                ],
                "responsive-section-grid",
            ),
            studio_edit_surface(
                "launch-motion-metrics",
                "[data-dx-editable-section=\"launch-motion-metrics\"]",
                "examples/template/template-shell.tsx",
                "components/template-app/template-shell.tsx",
                &["animation/motion", "state/zustand", "tanstack/query"],
                &["move_reorder_section", "update_design_token"],
                "responsive-section-grid",
            ),
            studio_edit_surface(
                "launch-dashboard-state-workflow",
                "[data-dx-component=\"launch-dashboard-state-workflow\"]",
                "examples/template/state-zustand-dashboard.tsx",
                "components/template-app/state-zustand-dashboard.tsx",
                &["state/zustand"],
                &[
                    "move_reorder_section",
                    "update_design_token",
                    "update_text_content",
                ],
                "ui-state-persistence-dashboard",
            ),
            studio_edit_surface(
                "launch-dashboard-state-shell",
                "[data-dx-component=\"launch-dashboard-state-shell\"]",
                "examples/template/template-shell.tsx",
                "components/template-app/template-shell.tsx",
                &["state/zustand"],
                &[
                    "move_reorder_section",
                    "update_design_token",
                    "update_text_content",
                ],
                "ui-state-persistence-shell",
            ),
            studio_edit_surface(
                "launch-package-catalog",
                "[data-dx-section=\"package-catalog\"]",
                "examples/template/template-shell.tsx",
                "components/template-app/template-shell.tsx",
                &[
                    "dx-www/template-shell",
                    "animation/motion",
                    "shadcn/ui/item",
                ],
                &[
                    "insert_component",
                    "move_reorder_section",
                    "update_text_content",
                    "insert_icon_media",
                ],
                "proof-card-grid",
            ),
            studio_edit_surface(
                "launch-studio-contract",
                "[data-dx-section=\"studio-contract\"]",
                "examples/template/dx-studio-edit-contract.ts",
                "components/template-app/dx-studio-edit-contract.ts",
                &["dx-www/template-shell"],
                &["update_text_content", "update_design_token"],
                "design-token-surface",
            ),
        ],
        "/automations" => vec![
            studio_edit_surface(
                "automations-shell",
                "[data-dx-section=\"automations\"]",
                "examples/template/automations/automations-shell.tsx",
                "components/automations/automations-shell.tsx",
                &["automations/n8n"],
                &[
                    "move_reorder_section",
                    "update_design_token",
                    "update_text_content",
                ],
                "responsive-section-grid",
            ),
            studio_edit_surface(
                "automations-route-nav",
                "[data-dx-insert-slot=\"automations-route-nav\"]",
                "examples/template/automations/automations-shell.tsx",
                "components/automations/automations-shell.tsx",
                &["automations/n8n"],
                &["insert_component", "update_text_content"],
                "responsive-section-grid",
            ),
        ],
        _ => Vec::new(),
    }
}
fn studio_edit_operation_index() -> Vec<Value> {
    vec![
        studio_edit_operation_detail(
            "insert_component",
            "Insert component",
            "data-dx-insert-slot",
            &["/", "/automations"],
            "Add a Forge-owned import and JSX call inside a declared responsive slot.",
        ),
        studio_edit_operation_detail(
            "move_reorder_section",
            "Move or reorder section",
            "data-dx-editable-section",
            &["/", "/automations"],
            "Move declared section JSX in source order while preserving responsive grid classes.",
        ),
        studio_edit_operation_detail(
            "update_design_token",
            "Update design token",
            "data-dx-design-token",
            &["/", "/automations"],
            "Update semantic Tailwind or shadcn token classes scoped by data-dx-token-scope.",
        ),
        studio_edit_operation_detail(
            "update_text_content",
            "Update text or content",
            "data-dx-editable-text",
            &["/", "/automations"],
            "Update source-owned literals, translation references, markdown, or metadata copy.",
        ),
        studio_edit_operation_detail(
            "insert_icon_media",
            "Insert icon or media",
            "data-dx-media-slot",
            &["/"],
            "Insert a registered DX icon or source-owned media component into a declared media slot.",
        ),
    ]
}

fn studio_edit_operation_detail(
    id: &'static str,
    label: &'static str,
    marker: &'static str,
    routes: &'static [&'static str],
    source_update: &'static str,
) -> Value {
    json!({
        "id": id,
        "label": label,
        "required_marker": marker,
        "selector": format!("[{marker}]"),
        "routes": routes,
        "editable_surfaces_field": "editable_surface_index",
        "source_update": source_update,
        "layout_policy": "responsive-design-system-grid",
        "design_system_policy": "semantic-tailwind-shadcn-and-forge-primitives",
        "absolute_positioning": false,
        "writes_files": true,
        "writes_only_source_owned_files": true,
        "requires_explicit_user_action": true,
        "requires_node_modules": false,
        "starts_server": false,
        "runs_package_install": false,
        "runs_full_build": false
    })
}

fn edit_operation_ids_for_route(route: &str) -> Vec<&'static str> {
    match route {
        "/" => vec![
            "insert_component",
            "move_reorder_section",
            "update_design_token",
            "update_text_content",
            "insert_icon_media",
        ],
        "/automations" => vec![
            "insert_component",
            "move_reorder_section",
            "update_design_token",
            "update_text_content",
        ],
        _ => Vec::new(),
    }
}

fn studio_marker(
    marker: &'static str,
    selector_kind: &'static str,
    routes: &'static [&'static str],
    lookup: &'static str,
    purpose: &'static str,
) -> Value {
    json!({
        "marker": marker,
        "selector": format!("[{marker}]"),
        "selector_kind": selector_kind,
        "routes": routes,
        "lookup": lookup,
        "purpose": purpose
    })
}

fn studio_source_guard_index() -> Vec<Value> {
    vec![
        studio_source_guard(
            "studio-preview-manifest",
            &["/", "/automations", "/ui", "/database", "/backend"],
            "benchmarks/dx-studio-preview-manifest.test.ts",
            "dx run --test .\\benchmarks\\dx-studio-preview-manifest.test.ts",
            &[
                "dx.studio.preview_manifest",
                "dx.zed.web_preview_contract",
                "data_dx_marker_index",
                "source_guard_index",
                "routes[].package_surfaces",
            ],
        ),
        studio_source_guard(
            "template-shell-package-slices",
            &["/"],
            "benchmarks/template-shell.test.ts",
            "dx run --test .\\benchmarks\\template-shell.test.ts",
            &[
                "launch package catalog consumption",
                "source-owned package imports",
                "data-dx launch markers",
                "no local node_modules",
            ],
        ),
        studio_source_guard_with_fixture(
            "forge-safety-archive-rollback-coverage",
            &["/"],
            "benchmarks/www-forge-package-lock.test.ts",
            "dx run --test .\\benchmarks\\www-forge-package-lock.test.ts",
            &[
                "Forge safety/archive rollback coverage guard",
                "tracked launch packages expose rollback receipt paths",
                "safety archive receipts exist under .dx/forge/receipts/safety",
                "archive receipts name cache files used for restore planning",
                "package-status safety_archive reports 100% local rollback coverage",
                "docs/packages/forge-safety-archive.source-guard-runbook.json",
                "without claiming remote rollback, live restore, or object-store writes",
                "Forge archive-before-delete source-only Studio discovery",
            ],
            "docs/packages/forge-safety-archive.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "authentication-package-lane-panel",
            &["/"],
            "benchmarks/authentication-dx-check-package-lane-panel.test.ts",
            "dx run --test .\\benchmarks\\authentication-dx-check-package-lane-panel.test.ts",
            &[
                "Authentication package-lane panel row",
                "data-dx-check-package-lane-row=\"auth/better-auth\"",
                "data-dx-token-scope=\"auth/better-auth\"",
                "authentication:receipt-hash-refresh",
                "data-dx-check-package-lane-hash-refresh-current-file-list",
                "data-dx-check-package-lane-hash-refresh-stale-file-list",
                "data-dx-check-package-lane-hash-refresh-missing-file-list",
                "data-dx-check-package-lane-hash-refresh-stale-mirror-file-list",
                "data-dx-check-package-lane-hash-refresh-missing-mirror-file-list",
                "authentication_receipt_hash_refresh_current",
                "authentication_receipt_hash_refresh_stale",
                "authentication_receipt_hash_refresh_missing",
                "docs/packages/authentication.source-guard-runbook.json",
                "without claiming live OAuth, deployed cookies, or hosted session runtime proof",
                "auth/better-auth adapter-boundary Studio discovery",
            ],
            "docs/packages/authentication.source-guard-runbook.json",
        ),
        studio_source_guard(
            "internationalization-launch-package-lane-template",
            &["/"],
            "benchmarks/next-intl-launch-package-lane-template.test.ts",
            "dx run --test .\\benchmarks\\next-intl-launch-package-lane-template.test.ts",
            &[
                "Internationalization www-template package-lane template",
                "data-dx-check-package-lane-template",
                "data-dx-style-surface=\"internationalization\"",
                "i18n/next-intl source-only Studio discovery",
                "docs/packages/next-intl.source-guard-runbook.json",
                "examples/template/internationalization-receipt-hashes.ts",
                "node tools/launch/run-template-receipt-helper.js examples/template/internationalization-receipt-hashes.ts --check --json",
                "internationalization:receipt-hash-refresh",
                "data-dx-check-package-lane-hash-refresh-helper",
                "data-dx-check-package-lane-hash-refresh-json-command",
                "data-dx-check-package-lane-hash-refresh-zed",
                "internationalization_receipt_hash_refresh_current",
                "internationalization_receipt_hash_refresh_stale",
                "internationalization_receipt_hash_refresh_missing",
            ],
        ),
        studio_source_guard(
            "state-management-generated-starter-materialization",
            &["/"],
            "benchmarks/zustand-launch-materialized.test.ts",
            "dx run --test .\\benchmarks\\zustand-launch-materialized.test.ts",
            &[
                "State Management generated-starter materialization guard",
                "data-dx-check-package-lane-row=\"state/zustand\"",
                "data-dx-token-scope=\"state/zustand\"",
                "state-management:receipt-hash-refresh",
                "docs/packages/state-zustand.source-guard-runbook.json",
                "without claiming browser storage or visual runtime proof",
                "state/zustand source-only Studio discovery",
            ],
        ),
        studio_source_guard_with_fixture(
            "state-management-check-panel-stale-helper-attribution",
            &["/"],
            "core/src/ecosystem/dx_check_receipt.rs",
            "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_state_management_stale_helper_file_attribution --lib",
            &[
                "State Management check-panel stale helper attribution guard",
                "receipt_hash_refresh.stale_files carries tools/launch/materialize-www-template.ts",
                "selected State Management source files remain in receipt_hash_refresh.current_files",
                "state_management_receipt_hash_refresh_stale",
                "state_management_receipt_stale",
                "docs/packages/state-zustand.source-guard-runbook.json",
                "without claiming browser storage or visual runtime proof",
                "state/zustand source-only Studio discovery",
            ],
            "docs/packages/state-zustand.source-guard-runbook.json",
        ),
        studio_source_guard(
            "data-fetching-cache-generated-starter-materialization",
            &["/"],
            "benchmarks/tanstack-query-dx-check-package-lane-panel.test.ts",
            "dx run --test .\\benchmarks\\tanstack-query-dx-check-package-lane-panel.test.ts",
            &[
                "Data Fetching & Cache generated-starter materialization guard",
                "data-dx-check-package-lane-row=\"tanstack/query\"",
                "data-dx-token-scope=\"tanstack/query\"",
                "data-fetching-cache:receipt-hash-refresh",
                "docs/packages/data-fetching-cache.source-guard-runbook.json",
                "tanstack/query source-only Studio discovery",
                "without live QueryClient execution proof",
                "without claiming live QueryClient execution",
            ],
        ),
        studio_source_guard(
            "backend-platform-client-dx-style-rust-check-output",
            &["/"],
            "core/src/ecosystem/project_check/backend_platform_client_dx_check.rs",
            "cargo test -q -p dx-www-compiler backend_platform_client_dx_style_missing_metric_and_finding_flip --lib",
            &[
                "Backend Platform Client Rust dx-style check output",
                "backend_platform_client_dx_style_compatibility_present",
                "backend_platform_client_dx_style_compatibility_missing",
                "backend-platform-client-missing-dx-style-compatibility",
                "docs/packages/backend-platform-client.source-guard-runbook.json",
                "without claiming hosted Supabase runtime proof",
                "supabase/client source-only Studio discovery",
            ],
        ),
        studio_source_guard_with_fixture(
            "backend-platform-client-lower-dx-check-helper-freshness",
            &["/"],
            "core/src/ecosystem/project_check/backend_platform_client_dx_check.rs",
            "cargo test -q -p dx-www-compiler backend_platform_client_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
            &[
                "Backend Platform Client lower-level dx-check helper freshness fixture",
                "supabase/client source-only Studio discovery",
                "docs/packages/backend-platform-client.source-guard-runbook.json",
                "backend_platform_client_receipt_hash_refresh_stale",
                "backend_platform_client_hash_mismatch stays byte-derived",
                "without claiming hosted Supabase runtime proof",
            ],
            "docs/packages/backend-platform-client.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "ui-components-generated-starter-materialization",
            &["/"],
            "benchmarks/ui-components-dx-check-package-lane-panel.test.ts",
            "dx run --test .\\benchmarks\\ui-components-dx-check-package-lane-panel.test.ts",
            &[
                "UI Components generated-starter materialization guard",
                "data-dx-check-package-lane-row=\"shadcn/ui/button\"",
                "data-dx-token-scope=\"shadcn/ui/button\"",
                "ui-components:receipt-hash-refresh",
                "docs/packages/ui-components.source-guard-runbook.json",
                "without claiming browser UI runtime proof",
                "shadcn/ui/button source-only Studio discovery",
            ],
            "docs/packages/ui-components.source-guard-runbook.json",
        ),
        studio_source_guard(
            "payments-generated-starter-materialization",
            &["/"],
            "benchmarks/payments-dx-check-package-lane-panel.test.ts",
            "dx run --test .\\benchmarks\\payments-dx-check-package-lane-panel.test.ts",
            &[
                "Payments generated-starter materialization guard",
                "data-dx-check-package-lane-row=\"payments/stripe-js\"",
                "data-dx-token-scope=\"payments/stripe-js\"",
                "payments:receipt-hash-refresh",
                "docs/packages/payments.source-guard-runbook.json",
                "without claiming live Stripe Checkout or webhook runtime proof",
                "payments/stripe-js source-only Studio discovery",
            ],
        ),
        studio_source_guard_with_fixture(
            "payments-lower-dx-check-helper-freshness",
            &["/"],
            "core/src/ecosystem/project_check/payments_dx_check.rs",
            "cargo test -q -p dx-www-compiler payments_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
            &[
                "Payments lower-level dx-check helper freshness fixture",
                "payments_receipt_hash_refresh_stale",
                "payments_hash_mismatch stays byte-derived",
                "docs/packages/payments.source-guard-runbook.json",
                "without claiming live Stripe Checkout or webhook runtime proof",
                "payments/stripe-js source-only Studio discovery",
            ],
            "docs/packages/payments.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "payments-check-panel-helper-freshness",
            &["/"],
            "core/src/ecosystem/dx_check_receipt.rs",
            "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_payments_stale_helper_without_source_hash_drift --lib",
            &[
                "Payments DX Studio/check-panel helper freshness fixture",
                "payments_receipt_hash_refresh_stale",
                "payments_hash_mismatch stays byte-derived",
                "docs/packages/payments.source-guard-runbook.json",
                "without claiming live Stripe Checkout or webhook runtime proof",
                "payments/stripe-js source-only Studio discovery",
            ],
            "docs/packages/payments.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "forms-generated-starter-materialization",
            &["/"],
            "benchmarks/forms-dx-check-package-lane-panel.test.ts",
            "dx run --test .\\benchmarks\\forms-dx-check-package-lane-panel.test.ts",
            &[
                "Forms generated-starter materialization guard",
                "data-dx-check-package-lane-row=\"forms/react-hook-form\"",
                "data-dx-token-scope=\"forms/react-hook-form\"",
                "forms:receipt-hash-refresh",
                "docs/packages/forms.source-guard-runbook.json",
                "without claiming browser submission proof",
                "forms/react-hook-form source-only Studio discovery",
            ],
            "docs/packages/forms.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "forms-package-metrics-helper-freshness-path-arrays",
            &["/"],
            "core/src/ecosystem/project_check/forms_dx_check.rs",
            "cargo test -q -p dx-www-compiler forms_package_metrics_reports_helper_freshness_from_path_arrays --lib",
            &[
                "Forms lower dx-check helper freshness fixture",
                "forms_receipt_hash_refresh_current",
                "forms_receipt_hash_refresh_stale",
                "forms_receipt_hash_refresh_missing",
                "forms_hash_mismatch stays byte-derived",
                "docs/packages/forms.source-guard-runbook.json",
                "without claiming browser submission proof",
                "forms/react-hook-form source-only Studio discovery",
            ],
            "docs/packages/forms.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "validation-schemas-generated-starter-materialization",
            &["/"],
            "benchmarks/zod-dx-check-package-lane-panel.test.ts",
            "dx run --test .\\benchmarks\\zod-dx-check-package-lane-panel.test.ts",
            &[
                "Validation & Schemas generated-starter materialization guard",
                "data-dx-check-package-lane-row=\"validation/zod\"",
                "data-dx-token-scope=\"validation/zod\"",
                "validation-schemas:receipt-hash-refresh",
                "docs/packages/validation-schemas.source-guard-runbook.json",
                "without claiming live Validation & Schemas runtime proof",
                "validation/zod source-only Studio discovery",
            ],
            "docs/packages/validation-schemas.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "webassembly-bridge-generated-starter-materialization",
            &["/"],
            "benchmarks/wasm-bindgen-dx-check-package-lane-panel.test.ts",
            "dx run --test .\\benchmarks\\wasm-bindgen-dx-check-package-lane-panel.test.ts",
            &[
                "WebAssembly Bridge generated-starter materialization guard",
                "data-dx-check-package-lane-row=\"wasm/bindgen\"",
                "data-dx-token-scope=\"wasm/bindgen\"",
                "docs/packages/wasm-bindgen.source-guard-runbook.json",
                "without claiming live generated-Wasm runtime proof",
                "wasm-bindgen 0.2.121 source-only Studio discovery",
                "live generated-Wasm runtime proof remains app-owned",
            ],
            "docs/packages/wasm-bindgen.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "webassembly-bridge-lower-dx-check-helper-freshness",
            &["/"],
            "core/src/ecosystem/project_check/wasm_bindgen_dx_check.rs",
            "cargo test -q -p dx-www-compiler webassembly_bridge_package_metrics_reports_helper_freshness_from_path_arrays --lib",
            &[
                "WebAssembly Bridge lower dx-check helper freshness fixture",
                "webassembly_bridge_receipt_hash_refresh_current",
                "webassembly_bridge_receipt_hash_refresh_stale",
                "webassembly_bridge_receipt_hash_refresh_missing",
                "webassembly_bridge_hash_mismatch stays byte-derived",
                "docs/packages/wasm-bindgen.source-guard-runbook.json",
                "without claiming live generated-Wasm runtime proof",
                "wasm-bindgen 0.2.121 source-only Studio discovery",
            ],
            "docs/packages/wasm-bindgen.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "webassembly-bridge-check-panel-helper-freshness",
            &["/"],
            "core/src/ecosystem/dx_check_receipt.rs",
            "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_webassembly_bridge_package_lane_hash_refresh_row --lib",
            &[
                "WebAssembly Bridge check-panel helper freshness fixture",
                "webassembly_bridge_receipt_hash_refresh_current",
                "webassembly_bridge_receipt_hash_refresh_stale",
                "webassembly_bridge_receipt_hash_refresh_missing",
                "webassembly_bridge_hash_mismatch stays byte-derived",
                "docs/packages/wasm-bindgen.source-guard-runbook.json",
                "without claiming live generated-Wasm runtime proof",
                "wasm-bindgen 0.2.121 source-only Studio discovery",
            ],
            "docs/packages/wasm-bindgen.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "motion-animation-generated-starter-materialization",
            &["/"],
            "benchmarks/motion-dx-check-package-lane-panel.test.ts",
            "dx run --test .\\benchmarks\\motion-dx-check-package-lane-panel.test.ts",
            &[
                "Motion & Animation generated-starter materialization guard",
                "data-dx-check-package-lane-row=\"animation/motion\"",
                "data-dx-token-scope=\"animation/motion\"",
                "motion-animation:receipt-hash-refresh",
                "docs/packages/motion-animation.source-guard-runbook.json",
                "without claiming live Motion browser animation proof",
                "animation/motion source-only Studio discovery",
            ],
            "docs/packages/motion-animation.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "three-scene-system-lower-dx-check-helper-freshness",
            &["/"],
            "core/src/ecosystem/project_check/three_scene_system_dx_check.rs",
            "cargo test -q -p dx-www-compiler three_scene_system_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
            &[
                "3D Scene System lower-level dx-check helper freshness fixture",
                "3d/launch-scene source-only Studio discovery",
                "docs/packages/3d-scene-system.source-guard-runbook.json",
                "three_scene_system_receipt_hash_refresh_stale",
                "three_scene_system_hash_mismatch stays byte-derived",
                "without claiming live browser/WebGL proof",
            ],
            "docs/packages/3d-scene-system.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "documentation-system-generated-starter-materialization",
            &["/"],
            "benchmarks/fumadocs-dx-check-package-lane-panel.test.ts",
            "dx run --test .\\benchmarks\\fumadocs-dx-check-package-lane-panel.test.ts",
            &[
                "Documentation System generated-starter materialization guard",
                "data-dx-check-package-lane-row=\"content/fumadocs-next\"",
                "data-dx-token-scope=\"content/fumadocs-next\"",
                "documentation-system:receipt-hash-refresh",
                "docs/packages/content-fumadocs-next.source-guard-runbook.json",
                "examples/template/documentation-system-receipt-hashes.ts",
                "node tools/launch/run-template-receipt-helper.js examples/template/documentation-system-receipt-hashes.ts --check --json",
                "without claiming live Fumadocs renderer runtime proof",
                "content/fumadocs-next source-only Studio discovery",
            ],
            "docs/packages/content-fumadocs-next.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "markdown-mdx-content-materialized-source-fixture",
            &["/"],
            "core/src/ecosystem/dx_check_receipt.rs",
            "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_materialized_source_row --lib",
            &[
                "Markdown & MDX Content materialized-source fixture",
                "content/react-markdown source-only Studio discovery",
                "markdown_mdx_content_materialized_source_present",
                "markdown_mdx_content_materialized_source_missing",
                "docs/packages/content-react-markdown.source-guard-runbook.json",
                "no live Markdown/MDX renderer proof",
            ],
            "docs/packages/content-react-markdown.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "markdown-mdx-content-check-panel-helper-freshness",
            &["/"],
            "core/src/ecosystem/dx_check_receipt.rs",
            "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_hash_refresh_row --lib",
            &[
                "Markdown & MDX Content check-panel helper freshness fixture",
                "content/react-markdown source-only Studio discovery",
                "docs/packages/content-react-markdown.source-guard-runbook.json",
                "markdown_mdx_content_receipt_hash_refresh_current",
                "markdown_mdx_content_receipt_hash_refresh_stale",
                "markdown_mdx_content_receipt_hash_refresh_missing",
                "markdown_mdx_content_hash_mismatch stays byte-derived",
                "without claiming live Markdown/MDX renderer proof",
            ],
            "docs/packages/content-react-markdown.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "ai-sdk-check-panel-helper-freshness",
            &["/"],
            "core/src/ecosystem/dx_check_receipt.rs",
            "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_ai_sdk_package_lane_hash_refresh_row --lib",
            &[
                "AI SDK check-panel helper freshness fixture",
                "ai/vercel-ai source-only Studio discovery",
                "docs/packages/ai-sdk.source-guard-runbook.json",
                "ai_sdk_receipt_hash_refresh_stale",
                "ai_sdk_hash_mismatch stays byte-derived",
                "without claiming live model streaming proof",
            ],
            "docs/packages/ai-sdk.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "realtime-app-database-check-panel-helper-freshness",
            &["/"],
            "core/src/ecosystem/dx_check_receipt.rs",
            "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_realtime_app_database_package_lane_hash_refresh_row --lib",
            &[
                "Realtime App Database check-panel helper freshness fixture",
                "instantdb/react source-only Studio discovery",
                "docs/packages/instantdb-react.source-guard-runbook.json",
                "realtime_app_database_receipt_hash_refresh_stale",
                "realtime_app_database_hash_mismatch stays byte-derived",
                "without claiming hosted Instant runtime proof",
            ],
            "docs/packages/instantdb-react.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "reactive-store-lower-dx-check-helper-freshness",
            &["/"],
            "core/src/ecosystem/project_check/reactive_store_dx_check.rs",
            "cargo test -q -p dx-www-compiler reactive_store_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
            &[
                "Reactive Store lower-level dx-check helper freshness fixture",
                "reactive/store source-only Studio discovery",
                "docs/packages/reactive-store.source-guard-runbook.json",
                "reactive_store_receipt_hash_refresh_stale",
                "reactive_store_hash_mismatch stays byte-derived",
                "no live React runtime proof",
            ],
            "docs/packages/reactive-store.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "database-orm-lower-dx-check-helper-freshness",
            &["/"],
            "core/src/ecosystem/project_check/database_orm_dx_check.rs",
            "cargo test -q -p dx-www-compiler database_orm_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
            &[
                "Database ORM lower-level dx-check helper freshness fixture",
                "db/drizzle-sqlite source-only Studio discovery",
                "docs/packages/database-orm.source-guard-runbook.json",
                "database_orm_receipt_hash_refresh_stale",
                "database_orm_hash_mismatch stays byte-derived",
                "no live SQLite read proof",
            ],
            "docs/packages/database-orm.source-guard-runbook.json",
        ),
        studio_source_guard_with_fixture(
            "automation-connectors-package-lane-panel",
            &["/"],
            "benchmarks/automations-dx-check-package-lane-panel.test.ts",
            "dx run --test .\\benchmarks\\automations-dx-check-package-lane-panel.test.ts",
            &[
                "Automation Connectors package-lane panel row",
                "data-dx-check-package-lane-row=\"automations/n8n\"",
                "data-dx-token-scope=\"automations/n8n\"",
                "automation-connectors:receipt-hash-refresh",
                "docs/packages/automation-connectors.source-guard-runbook.json",
                "without claiming live n8n workflow execution, provider credentials, webhook registration, or browser visual proof",
                "automations/n8n adapter-boundary Studio discovery",
            ],
            "docs/packages/automation-connectors.source-guard-runbook.json",
        ),
        studio_source_guard(
            "automations-route-bridge",
            &["/automations"],
            "benchmarks/automations-bridge.test.ts",
            "dx run --test .\\benchmarks\\automations-bridge.test.ts",
            &[
                "n8n connector metadata",
                "automation App Router surfaces",
                "automation data-dx markers",
                "credential redaction boundary",
            ],
        ),
        studio_source_guard(
            "website-conversion-routes",
            &["/ui", "/database", "/backend"],
            "benchmarks/dx-www-conversion-proof.test.ts",
            "dx run --test .\\benchmarks\\dx-www-conversion-proof.test.ts",
            &[
                "converted route ownership",
                "visual audit receipts",
                "copied asset provenance",
                "source-owned launch shims",
            ],
        ),
        studio_source_guard(
            "diff-whitespace-hygiene",
            &["/", "/automations", "/ui", "/database", "/backend"],
            "git diff --check",
            "git diff --check",
            &[
                "whitespace-safe patch",
                "source-only diff hygiene",
                "no generated node_modules",
            ],
        ),
        studio_source_guard(
            "conflict-marker-scan",
            &["/", "/automations", "/ui", "/database", "/backend"],
            "ripgrep conflict marker scan",
            "rg -n \"^(<<<<<<<|=======|>>>>>>>)\" dx-www/src/cli/studio_manifest.rs benchmarks/dx-studio-preview-manifest.test.ts benchmarks/template-shell.test.ts benchmarks/dx-www-conversion-proof.test.ts benchmarks/automations-bridge.test.ts examples/template examples/conversion-proof DX.md TODO.md CHANGELOG.md",
            &["no merge conflict markers", "safe shared-worktree handoff"],
        ),
    ]
}

fn studio_source_guard(
    id: &'static str,
    routes: &'static [&'static str],
    guard_file: &'static str,
    command: &'static str,
    proves: &'static [&'static str],
) -> Value {
    json!({
        "id": id,
        "routes": routes,
        "guard_file": guard_file,
        "command": command,
        "proves": proves,
        "execution_policy": "source-only",
        "writes_files": false,
        "starts_server": false,
        "runs_package_install": false,
        "runs_full_build": false,
        "node_modules_required": false
    })
}

fn studio_source_guard_with_fixture(
    id: &'static str,
    routes: &'static [&'static str],
    guard_file: &'static str,
    command: &'static str,
    proves: &'static [&'static str],
    fixture_path: &'static str,
) -> Value {
    let mut guard = studio_source_guard(id, routes, guard_file, command, proves);
    if let Some(object) = guard.as_object_mut() {
        object.insert("fixture_path".to_string(), json!(fixture_path));
    }
    guard
}

fn studio_source_guard_runbook_index(routes: &[Value]) -> Vec<Value> {
    routes
        .iter()
        .map(|route| {
            let route_path = route.get("route").and_then(Value::as_str).unwrap_or("/");

            json!({
                "route": route_path,
                "label": route.get("label").and_then(Value::as_str).unwrap_or(route_path),
                "source_guard_ids": source_guard_ids_for_route(route_path),
                "fixture_paths": source_guard_fixture_paths_for_route(route_path),
                "editable_surfaces": route_editable_surfaces(route_path),
                "edit_operation_ids": edit_operation_ids_for_route(route_path),
                "contracts": source_guard_contracts_for_route(route_path),
                "commands": source_guard_commands_for_route(route_path),
                "default_action": "show-source-only-runbook",
                "execution_policy": "source-only",
                "requires_server": false,
                "requires_package_install": false,
                "requires_full_build": false,
                "writes_files": false,
                "node_modules_required": false
            })
        })
        .collect()
}

fn source_guard_fixture_paths_for_route(route: &str) -> Vec<Value> {
    match route {
        "/" => vec![
            json!({
                "source_guard_id": "forge-safety-archive-rollback-coverage",
                "package_id": "forge/safety-archive",
                "fixture_path": "docs/packages/forge-safety-archive.source-guard-runbook.json",
                "schema": "dx.forge.safety_archive.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "authentication-package-lane-panel",
                "package_id": "auth/better-auth",
                "fixture_path": "docs/packages/authentication.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "state-management-check-panel-stale-helper-attribution",
                "package_id": "state/zustand",
                "fixture_path": "docs/packages/state-zustand.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "markdown-mdx-content-materialized-source-fixture",
                "package_id": "content/react-markdown",
                "fixture_path": "docs/packages/content-react-markdown.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "markdown-mdx-content-check-panel-helper-freshness",
                "package_id": "content/react-markdown",
                "fixture_path": "docs/packages/content-react-markdown.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "ui-components-generated-starter-materialization",
                "package_id": "shadcn/ui/button",
                "fixture_path": "docs/packages/ui-components.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "ai-sdk-check-panel-helper-freshness",
                "package_id": "ai/vercel-ai",
                "fixture_path": "docs/packages/ai-sdk.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "realtime-app-database-check-panel-helper-freshness",
                "package_id": "instantdb/react",
                "fixture_path": "docs/packages/instantdb-react.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "reactive-store-lower-dx-check-helper-freshness",
                "package_id": "reactive/store",
                "fixture_path": "docs/packages/reactive-store.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "database-orm-lower-dx-check-helper-freshness",
                "package_id": "db/drizzle-sqlite",
                "fixture_path": "docs/packages/database-orm.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "backend-platform-client-lower-dx-check-helper-freshness",
                "package_id": "supabase/client",
                "fixture_path": "docs/packages/backend-platform-client.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "payments-lower-dx-check-helper-freshness",
                "package_id": "payments/stripe-js",
                "fixture_path": "docs/packages/payments.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "payments-check-panel-helper-freshness",
                "package_id": "payments/stripe-js",
                "fixture_path": "docs/packages/payments.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "three-scene-system-lower-dx-check-helper-freshness",
                "package_id": "3d/launch-scene",
                "fixture_path": "docs/packages/3d-scene-system.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "forms-generated-starter-materialization",
                "package_id": "forms/react-hook-form",
                "fixture_path": "docs/packages/forms.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "forms-package-metrics-helper-freshness-path-arrays",
                "package_id": "forms/react-hook-form",
                "fixture_path": "docs/packages/forms.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "validation-schemas-generated-starter-materialization",
                "package_id": "validation/zod",
                "fixture_path": "docs/packages/validation-schemas.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "webassembly-bridge-generated-starter-materialization",
                "package_id": "wasm/bindgen",
                "fixture_path": "docs/packages/wasm-bindgen.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "webassembly-bridge-lower-dx-check-helper-freshness",
                "package_id": "wasm/bindgen",
                "fixture_path": "docs/packages/wasm-bindgen.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "webassembly-bridge-check-panel-helper-freshness",
                "package_id": "wasm/bindgen",
                "fixture_path": "docs/packages/wasm-bindgen.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "motion-animation-generated-starter-materialization",
                "package_id": "animation/motion",
                "fixture_path": "docs/packages/motion-animation.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "documentation-system-generated-starter-materialization",
                "package_id": "content/fumadocs-next",
                "fixture_path": "docs/packages/content-fumadocs-next.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
            json!({
                "source_guard_id": "automation-connectors-package-lane-panel",
                "package_id": "automations/n8n",
                "fixture_path": "docs/packages/automation-connectors.source-guard-runbook.json",
                "schema": "dx.forge.package.source_guard_runbook_fixture"
            }),
        ],
        _ => Vec::new(),
    }
}

fn source_guard_contracts_for_route(route: &str) -> Vec<Value> {
    let mut contracts = vec![
        source_guard_contract(
            "studio-preview-manifest",
            "DX Studio manifest schema, Zed Web Preview contract, marker index, source guards, and package surfaces are discoverable.",
            "dx.studio.preview_manifest",
        ),
        source_guard_contract(
            "diff-whitespace-hygiene",
            "The shared worktree patch stays whitespace-clean before Friday or Zed consume it.",
            "git diff --check",
        ),
        source_guard_contract(
            "conflict-marker-scan",
            "The manifest, tests, docs, converted routes, and launch template have no merge conflict markers.",
            "ripgrep conflict marker scan",
        ),
    ];

    match route {
        "/" => contracts.extend([
            source_guard_contract(
                "template-shell-package-slices",
                "The template shell imports real Forge package slices together and keeps the template node_modules-free.",
                "benchmarks/template-shell.test.ts",
            ),
            source_guard_contract(
                "launch-package-slice-readiness",
                "The launch package catalog, registry, scorecard, and trust-policy surfaces stay wired to real source-owned package APIs.",
                "benchmarks/launch-package-slices.test.ts",
            ),
            source_guard_contract_with_fixture(
                "forge-safety-archive-rollback-coverage",
                "The launch safety/archive row proves archive-before-delete receipts and local rollback coverage for tracked Forge packages without claiming remote rollback or live restore execution.",
                "benchmarks/www-forge-package-lock.test.ts",
                "docs/packages/forge-safety-archive.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "authentication-package-lane-panel",
                "The static launch fixture exposes the Authentication package-lane row, selected source markers, receipt-hash helper freshness, and dx-style markers without live OAuth or hosted session proof.",
                "benchmarks/authentication-dx-check-package-lane-panel.test.ts",
                "docs/packages/authentication.source-guard-runbook.json",
            ),
            source_guard_contract(
                "internationalization-launch-package-lane-template",
                "The static dx-check panel exposes the Internationalization package-lane template with official naming, upstream provenance, dx-style markers, and helper freshness markers before a receipt is loaded.",
                "benchmarks/next-intl-launch-package-lane-template.test.ts",
            ),
            source_guard_contract(
                "state-management-generated-starter-materialization",
                "The generated starter preserves the State Management package-lane row, helper freshness markers, and package-scoped dx-check panel without browser storage or visual runtime proof.",
                "benchmarks/zustand-launch-materialized.test.ts",
            ),
            source_guard_contract_with_fixture(
                "state-management-check-panel-stale-helper-attribution",
                "The State Management check-panel row preserves exact stale helper path attribution while selected State Management source files remain current.",
                "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_state_management_stale_helper_file_attribution --lib",
                "docs/packages/state-zustand.source-guard-runbook.json",
            ),
            source_guard_contract(
                "data-fetching-cache-generated-starter-materialization",
                "The generated starter preserves the Data Fetching & Cache package-lane row, helper freshness markers, and package-scoped dx-check panel without live QueryClient execution proof.",
                "benchmarks/tanstack-query-dx-check-package-lane-panel.test.ts",
            ),
            source_guard_contract(
                "backend-platform-client-dx-style-rust-check-output",
                "The lower-level Backend Platform Client dx-check producer reports dx-style compatibility present and missing metrics with a missing-metadata finding flip, without claiming hosted Supabase runtime proof.",
                "cargo test -q -p dx-www-compiler backend_platform_client_dx_style_missing_metric_and_finding_flip --lib",
            ),
            source_guard_contract_with_fixture(
                "backend-platform-client-lower-dx-check-helper-freshness",
                "The lower-level Backend Platform Client dx-check producer reports backend_platform_client_receipt_hash_refresh_stale while keeping backend_platform_client_hash_mismatch byte-derived.",
                "cargo test -q -p dx-www-compiler backend_platform_client_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
                "docs/packages/backend-platform-client.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "ui-components-generated-starter-materialization",
                "The generated starter preserves the UI Components package-lane row, helper freshness markers, and package-scoped dx-check panel without claiming browser UI runtime proof.",
                "benchmarks/ui-components-dx-check-package-lane-panel.test.ts",
                "docs/packages/ui-components.source-guard-runbook.json",
            ),
            source_guard_contract(
                "payments-generated-starter-materialization",
                "The generated starter preserves the Payments package-lane row, helper freshness markers, and package-scoped dx-check panel without live Stripe Checkout or webhook runtime proof.",
                "benchmarks/payments-dx-check-package-lane-panel.test.ts",
            ),
            source_guard_contract_with_fixture(
                "payments-lower-dx-check-helper-freshness",
                "The lower-level Payments dx-check producer reports payments_receipt_hash_refresh_stale while keeping payments_hash_mismatch byte-derived.",
                "cargo test -q -p dx-www-compiler payments_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
                "docs/packages/payments.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "payments-check-panel-helper-freshness",
                "The DX Studio/check-panel Payments row reports helper freshness metrics while keeping selected source hash mismatch byte-derived.",
                "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_payments_stale_helper_without_source_hash_drift --lib",
                "docs/packages/payments.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "forms-generated-starter-materialization",
                "The generated starter preserves the Forms package-lane row, helper freshness markers, and package-scoped dx-check panel without browser submission proof.",
                "benchmarks/forms-dx-check-package-lane-panel.test.ts",
                "docs/packages/forms.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "forms-package-metrics-helper-freshness-path-arrays",
                "The lower-level Forms dx-check producer reports helper freshness path arrays while keeping forms_hash_mismatch byte-derived.",
                "cargo test -q -p dx-www-compiler forms_package_metrics_reports_helper_freshness_from_path_arrays --lib",
                "docs/packages/forms.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "validation-schemas-generated-starter-materialization",
                "The generated starter preserves the Validation & Schemas package-lane row, helper freshness markers, dx-style markers, and package-scoped dx-check panel without live Validation & Schemas runtime proof.",
                "benchmarks/zod-dx-check-package-lane-panel.test.ts",
                "docs/packages/validation-schemas.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "webassembly-bridge-generated-starter-materialization",
                "The generated starter preserves the WebAssembly Bridge package-lane row, provenance, dx-style markers, and package-scoped dx-check panel without live generated-Wasm runtime proof.",
                "benchmarks/wasm-bindgen-dx-check-package-lane-panel.test.ts",
                "docs/packages/wasm-bindgen.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "webassembly-bridge-lower-dx-check-helper-freshness",
                "The lower-level WebAssembly Bridge dx-check producer reports helper freshness path arrays while keeping webassembly_bridge_hash_mismatch byte-derived.",
                "cargo test -q -p dx-www-compiler webassembly_bridge_package_metrics_reports_helper_freshness_from_path_arrays --lib",
                "docs/packages/wasm-bindgen.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "webassembly-bridge-check-panel-helper-freshness",
                "The DX Studio/check-panel WebAssembly Bridge row reports helper freshness path arrays while keeping webassembly_bridge_hash_mismatch byte-derived.",
                "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_webassembly_bridge_package_lane_hash_refresh_row --lib",
                "docs/packages/wasm-bindgen.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "motion-animation-generated-starter-materialization",
                "The generated starter preserves the Motion & Animation package-lane row, helper freshness markers, and package-scoped dx-check panel without live Motion browser animation proof.",
                "benchmarks/motion-dx-check-package-lane-panel.test.ts",
                "docs/packages/motion-animation.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "three-scene-system-lower-dx-check-helper-freshness",
                "The lower-level 3D Scene System dx-check producer reports three_scene_system_receipt_hash_refresh_stale while keeping three_scene_system_hash_mismatch byte-derived.",
                "cargo test -q -p dx-www-compiler three_scene_system_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
                "docs/packages/3d-scene-system.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "documentation-system-generated-starter-materialization",
                "The generated starter preserves the Documentation System package-lane row, scoped receipt hash helper, dx-style markers, and package-scoped dx-check panel without live Fumadocs renderer runtime proof.",
                "benchmarks/fumadocs-dx-check-package-lane-panel.test.ts",
                "docs/packages/content-fumadocs-next.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "automation-connectors-package-lane-panel",
                "The static launch fixture exposes the Automation Connectors package-lane row, upstream n8n provenance, selected source markers, dx-style markers, and receipt-hash helper freshness without live provider execution.",
                "benchmarks/automations-dx-check-package-lane-panel.test.ts",
                "docs/packages/automation-connectors.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "markdown-mdx-content-materialized-source-fixture",
                "The check-panel row proves Markdown & MDX Content materialized-source present/missing metrics from package-status evidence without live renderer proof.",
                "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_materialized_source_row --lib",
                "docs/packages/content-react-markdown.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "markdown-mdx-content-check-panel-helper-freshness",
                "The check-panel row proves Markdown & MDX Content receipt-helper freshness metrics while keeping markdown_mdx_content_hash_mismatch byte-derived from selected source files.",
                "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_hash_refresh_row --lib",
                "docs/packages/content-react-markdown.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "ai-sdk-check-panel-helper-freshness",
                "The shared check-panel AI SDK row reports ai_sdk_receipt_hash_refresh_stale while keeping ai_sdk_hash_mismatch byte-derived from selected app files.",
                "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_ai_sdk_package_lane_hash_refresh_row --lib",
                "docs/packages/ai-sdk.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "realtime-app-database-check-panel-helper-freshness",
                "The shared check-panel Realtime App Database row reports receipt-hash helper freshness metrics while keeping selected source hash mismatch byte-derived and hosted Instant runtime proof app-owned.",
                "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_realtime_app_database_package_lane_hash_refresh_row --lib",
                "docs/packages/instantdb-react.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "reactive-store-lower-dx-check-helper-freshness",
                "The lower-level Reactive Store dx-check producer reports reactive_store_receipt_hash_refresh_stale while keeping reactive_store_hash_mismatch byte-derived.",
                "cargo test -q -p dx-www-compiler reactive_store_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
                "docs/packages/reactive-store.source-guard-runbook.json",
            ),
            source_guard_contract_with_fixture(
                "database-orm-lower-dx-check-helper-freshness",
                "The lower-level Database ORM dx-check producer reports database_orm_receipt_hash_refresh_stale while keeping database_orm_hash_mismatch byte-derived.",
                "cargo test -q -p dx-www-compiler database_orm_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
                "docs/packages/database-orm.source-guard-runbook.json",
            ),
        ]),
        "/automations" => contracts.push(source_guard_contract(
            "automations-route-bridge",
            "The n8n automation route, generated connector metadata, credential redaction boundary, and data-dx markers stay discoverable.",
            "benchmarks/automations-bridge.test.ts",
        )),
        "/ui" | "/database" | "/backend" => contracts.extend([
            source_guard_contract(
                "website-conversion-routes",
                "Converted shadcn, Supabase, and Convex routes remain source-owned with launch shims and visual audit receipts.",
                "benchmarks/dx-www-conversion-proof.test.ts",
            ),
            source_guard_contract(
                "rendered-proof-validator",
                "Rendered-proof evidence remains schema-valid without running the converted application.",
                "examples/conversion-proof/forge/acceptance/validate-rendered-proof-evidence.ts",
            ),
            source_guard_contract(
                "rendered-proof-import-plan",
                "External runtime evidence can be mapped into stable Forge targets without copying files.",
                "examples/conversion-proof/forge/acceptance/prepare-rendered-proof-import.ts",
            ),
            source_guard_contract(
                "rendered-proof-runtime-approval",
                "Runtime rendered-proof capture stays blocked until an operator approves the exact route scope.",
                "examples/conversion-proof/forge/acceptance/request-rendered-proof-runtime-approval.ts",
            ),
            source_guard_contract(
                "rendered-proof-completeness-review",
                "Converted route launch readiness stays honest by separating collected and missing runtime proof.",
                "examples/conversion-proof/forge/acceptance/review-rendered-proof-completeness.ts",
            ),
        ]),
        _ => {}
    }

    contracts
}

fn source_guard_commands_for_route(route: &str) -> Vec<Value> {
    let mut commands = vec![
        source_guard_command(
            "dx run --test .\\benchmarks\\dx-studio-preview-manifest.test.ts",
            "Validate the Zed-facing DX Studio manifest contract and route/source/package discovery fields.",
        ),
        source_guard_command(
            "git diff --check",
            "Validate whitespace hygiene before handoff.",
        ),
        source_guard_command(
            r#"rg -n "^(<<<<<<<|=======|>>>>>>>)" dx-www/src/cli/studio_manifest.rs benchmarks/dx-studio-preview-manifest.test.ts benchmarks/template-shell.test.ts benchmarks/launch-package-slices.test.ts benchmarks/dx-www-conversion-proof.test.ts benchmarks/automations-bridge.test.ts examples/template examples/conversion-proof DX.md TODO.md CHANGELOG.md"#,
            "Scan source-owned manifest, tests, docs, launch template, and converted-route proofs for merge conflict markers.",
        ),
    ];

    match route {
        "/" => commands.extend([
            source_guard_command(
                "dx run --test .\\benchmarks\\template-shell.test.ts",
                "Validate the launch dashboard shell, route markers, source-owned package usage, and no-node_modules contract.",
            ),
            source_guard_command(
                "dx run --test .\\benchmarks\\launch-package-slices.test.ts",
                "Validate launch package slices across registry, scorecard, trust policy, and template surfaces.",
            ),
            source_guard_command_with_fixture(
                "dx run --test .\\benchmarks\\www-forge-package-lock.test.ts",
                "Validate source-owned Forge package lock, archive receipt, rollback receipt, cache-file, and safety_archive status coverage for the launch template.",
                "docs/packages/forge-safety-archive.source-guard-runbook.json",
            ),
            source_guard_command_with_fixture(
                "dx run --test .\\benchmarks\\authentication-dx-check-package-lane-panel.test.ts",
                "Validate the source-only Authentication package-lane row, selected source markers, receipt-hash helper freshness markers, and dx-style package scope without live OAuth, cookies, database adapters, or hosted sessions.",
                "docs/packages/authentication.source-guard-runbook.json",
            ),
            source_guard_command(
                "dx run --test .\\benchmarks\\next-intl-launch-package-lane-template.test.ts",
                "Validate the source-only Internationalization package-lane template, provenance, dx-style markers, and helper freshness markers.",
            ),
            source_guard_command(
                "node tools/launch/run-template-receipt-helper.js examples/template/internationalization-receipt-hashes.ts --check --json",
                "Check the source-only Internationalization receipt-hash helper payload without browser locale routing proof.",
            ),
            source_guard_command(
                "dx run --test .\\benchmarks\\zustand-launch-materialized.test.ts",
                "Validate the source-only State Management package-lane row, generated starter materialization, helper freshness markers, and dx-check panel package scope without browser storage or visual runtime proof.",
            ),
            source_guard_command_with_fixture(
                "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_state_management_stale_helper_file_attribution --lib",
                "Validate the source-only State Management stale-helper path attribution in the DX Studio/check-panel row without browser storage or visual runtime proof.",
                "docs/packages/state-zustand.source-guard-runbook.json",
            ),
            source_guard_command(
                "dx run --test .\\benchmarks\\tanstack-query-dx-check-package-lane-panel.test.ts",
                "Validate the source-only Data Fetching & Cache package-lane row, generated starter materialization, helper freshness markers, and dx-check panel package scope without live QueryClient execution proof.",
            ),
            source_guard_command(
                "cargo test -q -p dx-www-compiler backend_platform_client_dx_style_missing_metric_and_finding_flip --lib",
                "Validate the source-only Backend Platform Client dx-style metric and missing-metadata finding flip without hosted Supabase runtime proof.",
            ),
            source_guard_command_with_fixture(
                "cargo test -q -p dx-www-compiler backend_platform_client_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
                "Validate the source-only Backend Platform Client lower-level helper-freshness metrics without hosted Supabase runtime proof.",
                "docs/packages/backend-platform-client.source-guard-runbook.json",
            ),
            source_guard_command_with_fixture(
                "dx run --test .\\benchmarks\\ui-components-dx-check-package-lane-panel.test.ts",
                "Validate the source-only UI Components package-lane row, generated starter materialization, helper freshness markers, and dx-check panel package scope.",
                "docs/packages/ui-components.source-guard-runbook.json",
            ),
            source_guard_command(
                "dx run --test .\\benchmarks\\payments-dx-check-package-lane-panel.test.ts",
                "Validate the source-only Payments package-lane row, generated starter materialization, helper freshness markers, and dx-check panel package scope without live Stripe Checkout or webhook runtime proof.",
            ),
            source_guard_command_with_fixture(
                "cargo test -q -p dx-www-compiler payments_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
                "Validate the source-only Payments lower-level helper-freshness metrics without live Stripe Checkout, webhook delivery, secrets, or provider runtime proof.",
                "docs/packages/payments.source-guard-runbook.json",
            ),
            source_guard_command_with_fixture(
                "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_payments_stale_helper_without_source_hash_drift --lib",
                "Validate the source-only Payments check-panel helper-freshness metrics without live Stripe Checkout, webhook delivery, secrets, or provider runtime proof.",
                "docs/packages/payments.source-guard-runbook.json",
            ),
            source_guard_command_with_fixture(
                "dx run --test .\\benchmarks\\forms-dx-check-package-lane-panel.test.ts",
                "Validate the source-only Forms package-lane row, generated starter materialization, helper freshness markers, and dx-check panel package scope without browser submission proof.",
                "docs/packages/forms.source-guard-runbook.json",
            ),
            source_guard_command_with_fixture(
                "cargo test -q -p dx-www-compiler forms_package_metrics_reports_helper_freshness_from_path_arrays --lib",
                "Validate the source-only Forms lower dx-check helper freshness metrics and path-array attribution without browser submission proof.",
                "docs/packages/forms.source-guard-runbook.json",
            ),
            source_guard_command_with_fixture(
                "dx run --test .\\benchmarks\\zod-dx-check-package-lane-panel.test.ts",
                "Validate the source-only Validation & Schemas package-lane row, generated starter materialization, helper freshness markers, dx-style markers, and dx-check panel package scope without live Validation & Schemas runtime proof.",
                "docs/packages/validation-schemas.source-guard-runbook.json",
            ),
            source_guard_command_with_fixture(
                "dx run --test .\\benchmarks\\wasm-bindgen-dx-check-package-lane-panel.test.ts",
                "Validate the source-only WebAssembly Bridge package-lane row, generated starter materialization, provenance, dx-style markers, and dx-check panel package scope without live generated-Wasm runtime proof.",
                "docs/packages/wasm-bindgen.source-guard-runbook.json",
            ),
            source_guard_command_with_fixture(
                "cargo test -q -p dx-www-compiler webassembly_bridge_package_metrics_reports_helper_freshness_from_path_arrays --lib",
                "Validate the source-only WebAssembly Bridge lower dx-check helper freshness metrics and path-array attribution without live generated-Wasm runtime proof.",
                "docs/packages/wasm-bindgen.source-guard-runbook.json",
            ),
            source_guard_command_with_fixture(
                "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_webassembly_bridge_package_lane_hash_refresh_row --lib",
                "Validate the source-only WebAssembly Bridge check-panel helper freshness metrics and path-array attribution without live generated-Wasm runtime proof.",
                "docs/packages/wasm-bindgen.source-guard-runbook.json",
            ),
            source_guard_command_with_fixture(
                "dx run --test .\\benchmarks\\motion-dx-check-package-lane-panel.test.ts",
                "Validate the source-only Motion & Animation package-lane row, generated starter materialization, helper freshness markers, and dx-check panel package scope without live Motion browser animation proof.",
                "docs/packages/motion-animation.source-guard-runbook.json",
            ),
            source_guard_command_with_fixture(
                "cargo test -q -p dx-www-compiler three_scene_system_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
                "Validate the source-only 3D Scene System helper-freshness metrics without running a browser, opening WebGL, installing dependencies, or claiming screenshot proof.",
                "docs/packages/3d-scene-system.source-guard-runbook.json",
            ),
            source_guard_command_with_fixture(
                "dx run --test .\\benchmarks\\fumadocs-dx-check-package-lane-panel.test.ts",
                "Validate the source-only Documentation System package-lane row, generated starter materialization, scoped helper freshness markers, dx-style markers, and dx-check panel package scope without live Fumadocs renderer runtime proof.",
                "docs/packages/content-fumadocs-next.source-guard-runbook.json",
            ),
            source_guard_command_with_fixture(
                "dx run --test .\\benchmarks\\automations-dx-check-package-lane-panel.test.ts",
                "Validate the source-only Automation Connectors package-lane row, upstream n8n provenance, selected source markers, receipt-hash helper freshness markers, and dx-style package scope without live n8n execution, credentials, webhooks, or browser proof.",
                "docs/packages/automation-connectors.source-guard-runbook.json",
            ),
            source_guard_command(
                "node tools/launch/run-template-receipt-helper.js examples/template/documentation-system-receipt-hashes.ts --check --json",
                "Check the source-only Documentation System receipt-hash helper payload without live Fumadocs rendering, hosted search, or OpenAPI proxy proof.",
            ),
            source_guard_command_with_fixture(
                "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_materialized_source_row --lib",
                "Validate the source-only Markdown & MDX Content materialized-source check-panel metrics without live renderer proof.",
                "docs/packages/content-react-markdown.source-guard-runbook.json",
            ),
            source_guard_command_with_fixture(
                "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_markdown_mdx_content_package_lane_hash_refresh_row --lib",
                "Validate the source-only Markdown & MDX Content helper-freshness check-panel metrics without live renderer proof.",
                "docs/packages/content-react-markdown.source-guard-runbook.json",
            ),
            source_guard_command_with_fixture(
                "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_ai_sdk_package_lane_hash_refresh_row --lib",
                "Validate the source-only AI SDK check-panel helper-freshness metrics without running providers, installing dependencies, reading secrets, or claiming model streaming proof.",
                "docs/packages/ai-sdk.source-guard-runbook.json",
            ),
            source_guard_command_with_fixture(
                "cargo test -q -p dx-www-compiler dx_check_latest_panel_exposes_realtime_app_database_package_lane_hash_refresh_row --lib",
                "Validate the source-only Realtime App Database check-panel helper-freshness metrics without hosted Instant credentials, browser proof, dependency installation, or live subscriptions.",
                "docs/packages/instantdb-react.source-guard-runbook.json",
            ),
            source_guard_command(
                "cargo test -q -p dx-www-compiler reactive_store_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
                "Validate the source-only Reactive Store lower-level dx-check helper freshness metrics without live React runtime proof.",
            ),
            source_guard_command_with_fixture(
                "cargo test -q -p dx-www-compiler database_orm_hash_refresh_stale_helper_keeps_source_hash_clean --lib",
                "Validate the source-only Database ORM lower-level dx-check helper freshness metrics without live SQLite read proof.",
                "docs/packages/database-orm.source-guard-runbook.json",
            ),
        ]),
        "/automations" => commands.push(source_guard_command(
            "dx run --test .\\benchmarks\\automations-bridge.test.ts",
            "Validate n8n automation route metadata, generated connector catalog, credentials boundary, and preview markers.",
        )),
        "/ui" | "/database" | "/backend" => commands.extend([
            source_guard_command(
                "dx run --test .\\benchmarks\\dx-www-conversion-proof.test.ts",
                "Validate converted shadcn, Supabase, and Convex route proof files without running the app.",
            ),
            source_guard_command(
                "node .\\examples\\conversion-proof\\forge\\acceptance\\validate-rendered-proof-evidence.ts --json",
                "Validate rendered-proof evidence against the source-owned schema without starting a server.",
            ),
            source_guard_command(
                "node .\\examples\\conversion-proof\\forge\\acceptance\\prepare-rendered-proof-import.ts --json",
                "Plan how externally collected runtime evidence would materialize into Forge-owned targets without copying files.",
            ),
            source_guard_command(
                "node .\\examples\\conversion-proof\\forge\\acceptance\\request-rendered-proof-runtime-approval.ts --json",
                "Report the approval request required before runtime rendered-proof capture can collect evidence.",
            ),
            source_guard_command(
                "node .\\examples\\conversion-proof\\forge\\acceptance\\review-rendered-proof-completeness.ts --json",
                "Review which converted-route runtime evidence is collected, blocked, or still pending without faking proof.",
            ),
        ]),
        _ => {}
    }

    commands
}

fn source_guard_command(command: &'static str, purpose: &'static str) -> Value {
    json!({
        "command": command,
        "purpose": purpose,
        "scope": "source-only",
        "starts_server": false,
        "runs_package_install": false,
        "runs_full_build": false,
        "writes_files": false,
        "node_modules_required": false
    })
}

fn source_guard_command_with_fixture(
    command: &'static str,
    purpose: &'static str,
    fixture_path: &'static str,
) -> Value {
    let mut command = source_guard_command(command, purpose);
    if let Some(object) = command.as_object_mut() {
        object.insert("fixture_path".to_string(), json!(fixture_path));
    }
    command
}

fn source_guard_contract(
    id: &'static str,
    purpose: &'static str,
    evidence_field: &'static str,
) -> Value {
    json!({
        "id": id,
        "purpose": purpose,
        "evidence_field": evidence_field,
        "scope": "source-only",
        "reads_runtime_artifacts": false,
        "writes_files": false,
        "node_modules_required": false
    })
}

fn source_guard_contract_with_fixture(
    id: &'static str,
    purpose: &'static str,
    evidence_field: &'static str,
    fixture_path: &'static str,
) -> Value {
    let mut contract = source_guard_contract(id, purpose, evidence_field);
    if let Some(object) = contract.as_object_mut() {
        object.insert("fixture_path".to_string(), json!(fixture_path));
    }
    contract
}

fn studio_preview_watch_index(routes: &[Value]) -> Vec<Value> {
    routes
        .iter()
        .map(|route| {
            let route_path = route
                .get("route")
                .and_then(Value::as_str)
                .unwrap_or("/");
            let source_files = route_string_array(route, "source_files");
            let materialized_files = route_string_array(route, "materialized_files");
            let assets = route_string_array(route, "assets");
            let forge_packages = route_string_array(route, "forge_packages");
            let data_dx_markers = route_string_array(route, "data_dx_markers");
            let mut watch_files = source_files
                .iter()
                .chain(materialized_files.iter())
                .chain(assets.iter())
                .cloned()
                .collect::<Vec<_>>();
            watch_files.sort();
            watch_files.dedup();

            json!({
                "route": route_path,
                "preview_url": route.pointer("/preview/url").and_then(Value::as_str).unwrap_or("http://127.0.0.1:3000/"),
                "hot_reload_target": route.pointer("/preview/hot_reload_target").and_then(Value::as_str).unwrap_or("route:/"),
                "watch_files": watch_files,
                "source_files": source_files,
                "materialized_files": materialized_files,
                "assets": assets,
                "forge_packages": forge_packages,
                "marker_selectors": data_dx_markers
                    .iter()
                    .map(|marker| format!("[{marker}]"))
                    .collect::<Vec<_>>(),
                "source_guard_ids": source_guard_ids_for_route(route_path),
                "editable_surfaces": route_editable_surfaces(route_path),
                "edit_operation_ids": edit_operation_ids_for_route(route_path),
                "reload_strategy": "route-scoped",
                "requires_node_modules": false,
                "requires_server_restart": false,
                "requires_package_install": false
            })
        })
        .collect()
}

fn route_string_array(route: &Value, key: &str) -> Vec<String> {
    route
        .get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

fn source_guard_ids_for_route(route: &str) -> Vec<&'static str> {
    let mut guards = vec![
        "studio-preview-manifest",
        "diff-whitespace-hygiene",
        "conflict-marker-scan",
    ];
    match route {
        "/" => guards.extend([
            "template-shell-package-slices",
            "forge-safety-archive-rollback-coverage",
            "authentication-package-lane-panel",
            "internationalization-launch-package-lane-template",
            "state-management-generated-starter-materialization",
            "state-management-check-panel-stale-helper-attribution",
            "data-fetching-cache-generated-starter-materialization",
            "backend-platform-client-dx-style-rust-check-output",
            "backend-platform-client-lower-dx-check-helper-freshness",
            "ui-components-generated-starter-materialization",
            "payments-generated-starter-materialization",
            "payments-lower-dx-check-helper-freshness",
            "payments-check-panel-helper-freshness",
            "forms-generated-starter-materialization",
            "forms-package-metrics-helper-freshness-path-arrays",
            "webassembly-bridge-generated-starter-materialization",
            "webassembly-bridge-lower-dx-check-helper-freshness",
            "webassembly-bridge-check-panel-helper-freshness",
            "motion-animation-generated-starter-materialization",
            "validation-schemas-generated-starter-materialization",
            "documentation-system-generated-starter-materialization",
            "markdown-mdx-content-materialized-source-fixture",
            "markdown-mdx-content-check-panel-helper-freshness",
            "ai-sdk-check-panel-helper-freshness",
            "realtime-app-database-check-panel-helper-freshness",
            "reactive-store-lower-dx-check-helper-freshness",
            "database-orm-lower-dx-check-helper-freshness",
            "three-scene-system-lower-dx-check-helper-freshness",
        ]),
        "/automations" => guards.push("automations-route-bridge"),
        "/ui" | "/database" | "/backend" => guards.push("website-conversion-routes"),
        _ => {}
    }
    guards
}

fn studio_route_readiness_index(routes: &[Value]) -> Vec<Value> {
    routes
        .iter()
        .map(|route| {
            let route_path = route
                .get("route")
                .and_then(Value::as_str)
                .unwrap_or("/");
            let source_files = route_string_array(route, "source_files");
            let forge_packages = route_string_array(route, "forge_packages");
            let data_dx_markers = route_string_array(route, "data_dx_markers");

            json!({
                "route": route_path,
                "label": route.get("label").and_then(Value::as_str).unwrap_or(route_path),
                "role": route.get("role").and_then(Value::as_str).unwrap_or("route"),
                "status": route.get("status").and_then(Value::as_str).unwrap_or("source-ready-runtime-gated"),
                "preview_url": route.pointer("/preview/url").and_then(Value::as_str).unwrap_or("http://127.0.0.1:3000/"),
                "hot_reload_target": route.pointer("/preview/hot_reload_target").and_then(Value::as_str).unwrap_or("route:/"),
                "source_file_count": source_files.len(),
                "forge_package_count": forge_packages.len(),
                "data_dx_marker_count": data_dx_markers.len(),
                "source_guard_ids": source_guard_ids_for_route(route_path),
                "editable_surfaces": route_editable_surfaces(route_path),
                "edit_operation_ids": edit_operation_ids_for_route(route_path),
                "editable_surface_count": route_editable_surfaces(route_path).len(),
                "edit_operation_count": edit_operation_ids_for_route(route_path).len(),
                "node_modules_required": false,
                "package_install_required": false,
                "server_restart_required": false,
                "runtime_evidence": "explicit-approval-gated"
            })
        })
        .collect()
}

#[derive(Default)]
struct ForgeReadinessAccumulator {
    routes: BTreeSet<String>,
    source_files: BTreeSet<String>,
    materialized_files: BTreeSet<String>,
    api_surface: BTreeSet<String>,
    readiness: BTreeSet<String>,
    data_dx_markers: BTreeSet<String>,
    source_guard_ids: BTreeSet<String>,
}

#[derive(Default)]
struct EnvContractAccumulator {
    routes: BTreeSet<String>,
    source_files: BTreeSet<String>,
    materialized_files: BTreeSet<String>,
    source_guard_ids: BTreeSet<String>,
}

fn studio_forge_readiness_index(routes: &[Value]) -> Vec<Value> {
    let mut index = BTreeMap::<String, ForgeReadinessAccumulator>::new();

    for route in routes {
        let route_path = route.get("route").and_then(Value::as_str).unwrap_or("/");
        let route_source_files = route_string_array(route, "source_files");
        let route_materialized_files = route_string_array(route, "materialized_files");
        let route_guards = source_guard_ids_for_route(route_path);
        let route_status = route
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("source-ready-runtime-gated");

        for package in route_string_array(route, "forge_packages") {
            let entry = index.entry(package).or_default();
            entry.routes.insert(route_path.to_string());
            entry.readiness.insert(route_status.to_string());
            for guard in &route_guards {
                entry.source_guard_ids.insert((*guard).to_string());
            }
        }

        for surface in route
            .get("package_surfaces")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let Some(package) = surface.get("package").and_then(Value::as_str) else {
                continue;
            };
            let entry = index.entry(package.to_string()).or_default();
            entry.routes.insert(route_path.to_string());
            for source_file in route_string_array(surface, "source_file") {
                entry.source_files.insert(source_file);
            }
            if let Some(source_file) = surface.get("source_file").and_then(Value::as_str) {
                entry.source_files.insert(source_file.to_string());
            }
            if let Some(materialized_file) =
                surface.get("materialized_file").and_then(Value::as_str)
            {
                entry
                    .materialized_files
                    .insert(materialized_file.to_string());
            }
            for api in route_string_array(surface, "api_surface") {
                entry.api_surface.insert(api);
            }
            if let Some(readiness) = surface.get("readiness").and_then(Value::as_str) {
                entry.readiness.insert(readiness.to_string());
            }
            for marker in route_string_array(surface, "data_dx_markers") {
                entry.data_dx_markers.insert(marker);
            }
            for guard in &route_guards {
                entry.source_guard_ids.insert((*guard).to_string());
            }
        }

        for package in route_string_array(route, "forge_packages") {
            let entry = index.entry(package).or_default();
            if entry.source_files.is_empty() {
                entry
                    .source_files
                    .extend(route_source_files.iter().cloned());
            }
            if entry.materialized_files.is_empty() {
                entry
                    .materialized_files
                    .extend(route_materialized_files.iter().cloned());
            }
        }
    }

    index
        .into_iter()
        .map(|(package, entry)| {
            let readiness = entry.readiness.iter().cloned().collect::<Vec<_>>();
            json!({
                "package": package,
                "routes": entry.routes.into_iter().collect::<Vec<_>>(),
                "source_files": entry.source_files.into_iter().collect::<Vec<_>>(),
                "materialized_files": entry.materialized_files.into_iter().collect::<Vec<_>>(),
                "api_surface": entry.api_surface.into_iter().collect::<Vec<_>>(),
                "readiness": readiness,
                "runtime_policy": readiness_policy(&readiness),
                "data_dx_markers": entry.data_dx_markers.into_iter().collect::<Vec<_>>(),
                "source_guard_ids": entry.source_guard_ids.into_iter().collect::<Vec<_>>(),
                "node_modules_required": false,
                "package_install_required": false,
                "server_restart_required": false
            })
        })
        .collect()
}

fn studio_env_contract_index(routes: &[Value]) -> Vec<Value> {
    let mut index = BTreeMap::<String, EnvContractAccumulator>::new();

    for route in routes {
        let route_path = route.get("route").and_then(Value::as_str).unwrap_or("/");
        let route_source_files = route_string_array(route, "source_files");
        let route_materialized_files = route_string_array(route, "materialized_files");
        let route_guards = source_guard_ids_for_route(route_path);
        let mut packages = route_string_array(route, "forge_packages");

        for surface in route
            .get("package_surfaces")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            if let Some(package) = surface.get("package").and_then(Value::as_str) {
                packages.push(package.to_string());
            }
        }

        packages.sort();
        packages.dedup();

        for package in packages {
            if package_env_contracts(&package).is_empty() {
                continue;
            }

            let entry = index.entry(package).or_default();
            entry.routes.insert(route_path.to_string());
            entry
                .source_files
                .extend(route_source_files.iter().cloned());
            entry
                .materialized_files
                .extend(route_materialized_files.iter().cloned());
            for guard in &route_guards {
                entry.source_guard_ids.insert((*guard).to_string());
            }
        }
    }

    index
        .into_iter()
        .map(|(package, entry)| {
            let env = package_env_contracts(&package);
            json!({
                "package": package,
                "routes": entry.routes.into_iter().collect::<Vec<_>>(),
                "env": env,
                "source_files": entry.source_files.into_iter().collect::<Vec<_>>(),
                "materialized_files": entry.materialized_files.into_iter().collect::<Vec<_>>(),
                "source_guard_ids": entry.source_guard_ids.into_iter().collect::<Vec<_>>(),
                "status": "declared-app-owned-not-read",
                "runtime_policy": "app-owned-env-boundary",
                "reads_environment": false,
                "writes_files": false,
                "starts_server": false,
                "runs_package_install": false,
                "runs_full_build": false,
                "node_modules_required": false
            })
        })
        .collect()
}

fn package_env_contracts(package: &str) -> Vec<Value> {
    match package {
        "auth/better-auth" => vec![
            env_contract(
                "BETTER_AUTH_SECRET",
                "server-secret",
                "Signs and verifies Better Auth session material.",
            ),
            env_contract(
                "BETTER_AUTH_URL",
                "server-public-origin",
                "Defines the trusted Better Auth origin for callbacks and cookies.",
            ),
        ],
        "payments/stripe-js" => vec![
            env_contract(
                "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY",
                "browser-public",
                "Loads Stripe.js in the browser for checkout confirmation.",
            ),
            env_contract(
                "STRIPE_SECRET_KEY",
                "server-secret",
                "Creates Checkout Sessions or PaymentIntents on an app-owned server boundary.",
            ),
            env_contract(
                "STRIPE_PRICE_ID",
                "server-public-catalog",
                "Fallback app-owned Stripe Price ID for generated Checkout Sessions.",
            ),
            env_contract(
                "STRIPE_PRICE_ID_STARTER",
                "server-public-catalog",
                "Optional starter-plan Stripe Price ID selected by the launch billing workflow.",
            ),
            env_contract(
                "STRIPE_PRICE_ID_TEAM",
                "server-public-catalog",
                "Optional team-plan Stripe Price ID selected by the launch billing workflow.",
            ),
            env_contract(
                "STRIPE_PRICE_ID_SCALE",
                "server-public-catalog",
                "Optional scale-plan Stripe Price ID selected by the launch billing workflow.",
            ),
        ],
        "ai/vercel-ai" => vec![
            env_contract(
                "AI_PROVIDER_API_KEY",
                "server-secret",
                "Authorizes app-owned AI provider calls behind the AI SDK slice.",
            ),
            env_contract(
                "AI_GATEWAY_API_KEY",
                "server-secret",
                "Enables app-owned AI Gateway routing for the launch assistant workflow.",
            ),
        ],
        "supabase/client" => vec![
            env_contract(
                "NEXT_PUBLIC_SUPABASE_URL",
                "browser-public",
                "Points source-owned Supabase clients at the app-owned Supabase project.",
            ),
            env_contract(
                "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY",
                "browser-public",
                "Configures browser-safe Supabase client access under RLS.",
            ),
        ],
        "instantdb/react" => vec![env_contract(
            "NEXT_PUBLIC_INSTANT_APP_ID",
            "browser-public",
            "Connects InstantDB React helpers to an app-owned Instant application.",
        )],
        _ => Vec::new(),
    }
}

fn env_contract(name: &'static str, visibility: &'static str, purpose: &'static str) -> Value {
    json!({
        "name": name,
        "visibility": visibility,
        "purpose": purpose,
        "owner": "app",
        "source_only": true,
        "read_by_manifest": false,
        "placeholder_allowed": true,
        "runtime_required_for_real_service": true
    })
}

fn studio_forge_receipt_index(routes: &[Value]) -> Vec<Value> {
    routes
        .iter()
        .map(|route| {
            let route_path = route.get("route").and_then(Value::as_str).unwrap_or("/");

            json!({
                "route": route_path,
                "route_receipts": route_receipt_artifacts(route),
                "package_receipts": route_package_receipts(route, route_path),
                "source_guard_ids": source_guard_ids_for_route(route_path),
                "provenance_policy": "source-owned-declaration-no-runtime-read",
                "reads_runtime_artifacts": false,
                "reads_receipt_files": false,
                "writes_files": false,
                "starts_server": false,
                "runs_package_install": false,
                "runs_full_build": false,
                "node_modules_required": false
            })
        })
        .collect()
}

fn route_receipt_artifacts(route: &Value) -> Vec<Value> {
    let mut paths = route_string_array(route, "source_files");
    paths.extend(route_string_array(route, "materialized_files"));
    paths.sort();
    paths.dedup();

    paths
        .into_iter()
        .filter(|path| is_receipt_artifact_path(path))
        .map(|path| {
            let kind = receipt_artifact_kind(&path);
            json!({
                "path": path,
                "kind": kind,
                "source_owned": true,
                "runtime_artifact": false
            })
        })
        .collect()
}

fn route_package_receipts(route: &Value, route_path: &str) -> Vec<Value> {
    let route_guards = source_guard_ids_for_route(route_path);
    let mut seen = BTreeSet::<String>::new();
    let mut receipts = Vec::new();

    for surface in route
        .get("package_surfaces")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let Some(package) = surface.get("package").and_then(Value::as_str) else {
            continue;
        };
        seen.insert(package.to_string());
        receipts.push(json!({
            "package": package,
            "route": route_path,
            "source_file": surface.get("source_file").and_then(Value::as_str).unwrap_or(""),
            "materialized_file": surface.get("materialized_file").and_then(Value::as_str).unwrap_or(""),
            "readiness": surface.get("readiness").and_then(Value::as_str).unwrap_or("source-owned-ready"),
            "api_surface": route_string_array(surface, "api_surface"),
            "receipt_family": "launch-package-surface",
            "source_guard_ids": route_guards.clone(),
            "source_owned": true,
            "runtime_artifact": false,
            "reads_receipt_files": false,
            "writes_files": false,
            "node_modules_required": false
        }));
    }

    for package in route_string_array(route, "forge_packages") {
        if seen.contains(&package) {
            continue;
        }
        receipts.push(json!({
            "package": package,
            "route": route_path,
            "source_file": route
                .get("source_files")
                .and_then(Value::as_array)
                .and_then(|files| files.first())
                .and_then(Value::as_str)
                .unwrap_or(""),
            "materialized_file": route
                .get("materialized_files")
                .and_then(Value::as_array)
                .and_then(|files| files.first())
                .and_then(Value::as_str)
                .unwrap_or(""),
            "readiness": route.get("status").and_then(Value::as_str).unwrap_or("source-ready-runtime-gated"),
            "api_surface": [],
            "receipt_family": "route-declared-forge-package",
            "source_guard_ids": route_guards.clone(),
            "source_owned": true,
            "runtime_artifact": false,
            "reads_receipt_files": false,
            "writes_files": false,
            "node_modules_required": false
        }));
    }

    receipts
}

fn is_receipt_artifact_path(path: &str) -> bool {
    path.contains(".dx/forge")
        || path.contains("forge/conversion-manifests")
        || path.contains("forge/route-discovery")
        || path.contains("forge/acceptance")
        || path.contains("integrations/n8n-nodes-base/generated")
}

fn receipt_artifact_kind(path: &str) -> &'static str {
    if path.contains(".dx/forge/receipts") {
        "forge-receipt"
    } else if path.contains("forge/conversion-manifests") {
        "conversion-manifest"
    } else if path.contains("forge/route-discovery") {
        "route-discovery"
    } else if path.contains("no-runtime-route-acceptance") {
        "acceptance-checklist"
    } else if path.contains("rendered-proof-evidence.schema") {
        "rendered-proof-schema"
    } else if path.contains("validate-rendered-proof-evidence") {
        "rendered-proof-validator"
    } else if path.contains("rendered-proof-import-plan") {
        "rendered-proof-import-plan"
    } else if path.contains("prepare-rendered-proof-import") {
        "rendered-proof-import-tool"
    } else if path.contains("rendered-proof-runtime-approval-request") {
        "rendered-proof-runtime-approval-request"
    } else if path.contains("request-rendered-proof-runtime-approval") {
        "rendered-proof-runtime-approval-tool"
    } else if path.contains("review-rendered-proof-completeness") {
        "rendered-proof-completeness-review"
    } else if path.contains("forge/acceptance") {
        "acceptance-proof"
    } else if path.contains("integrations/n8n-nodes-base/generated") {
        "automation-generated-manifest"
    } else if path.contains(".dx/forge") {
        "forge-artifact"
    } else {
        "source-artifact"
    }
}

fn readiness_policy(readiness: &[String]) -> &'static str {
    if readiness.iter().any(|item| {
        item.contains("env-gated")
            || item.contains("runtime")
            || item.contains("adapter")
            || item.contains("cli-bridge")
    }) {
        "source-owned-runtime-gated"
    } else {
        "source-owned-ready"
    }
}

fn studio_source_selection_index(routes: &[Value]) -> Vec<Value> {
    routes
        .iter()
        .map(|route| {
            let route_path = route.get("route").and_then(Value::as_str).unwrap_or("/");
            let source_files = route_string_array(route, "source_files");
            let materialized_files = route_string_array(route, "materialized_files");
            let data_dx_markers = route_string_array(route, "data_dx_markers");
            let primary_source_file = source_files
                .first()
                .cloned()
                .unwrap_or_else(|| "dx".to_string());

            json!({
                "route": route_path,
                "route_selector": format!("[data-dx-route=\"{route_path}\"]"),
                "primary_source_file": primary_source_file,
                "source_files": source_files,
                "materialized_files": materialized_files,
                "package_surfaces": source_selection_package_surfaces(route, route_path),
                "editable_surfaces": route_editable_surfaces(route_path),
                "edit_operation_ids": edit_operation_ids_for_route(route_path),
                "marker_selectors": data_dx_markers
                    .iter()
                    .map(|marker| format!("[{marker}]"))
                    .collect::<Vec<_>>(),
                "source_guard_ids": source_guard_ids_for_route(route_path),
                "hot_reload_target": route.pointer("/preview/hot_reload_target").and_then(Value::as_str).unwrap_or("route:/"),
                "open_policy": "source-owned-open-only",
                "writes_files": false,
                "edit_operations_write_files": !edit_operation_ids_for_route(route_path).is_empty(),
                "starts_server": false,
                "runs_package_install": false,
                "runs_full_build": false,
                "node_modules_required": false,
                "semantic_editing": {
                    "drop_to_code_ready": true,
                    "default_action": "open-source-file",
                    "edit_operation_index_field": "edit_operation_index",
                    "editable_surface_index_field": "editable_surface_index",
                    "write_gate": "explicit-user-edit-command-required"
                }
            })
        })
        .collect()
}

fn source_selection_package_surfaces(route: &Value, route_path: &str) -> Vec<Value> {
    let route_guards = source_guard_ids_for_route(route_path);
    let mut seen = BTreeSet::<String>::new();
    let mut surfaces = Vec::new();

    for surface in route
        .get("package_surfaces")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let Some(package) = surface.get("package").and_then(Value::as_str) else {
            continue;
        };
        seen.insert(package.to_string());
        surfaces.push(json!({
            "package": package,
            "selector": format!("[data-dx-package=\"{package}\"]"),
            "source_file": surface.get("source_file").and_then(Value::as_str).unwrap_or(""),
            "materialized_file": surface.get("materialized_file").and_then(Value::as_str).unwrap_or(""),
            "api_surface": route_string_array(surface, "api_surface"),
            "readiness": surface.get("readiness").and_then(Value::as_str).unwrap_or("source-owned-ready"),
            "data_dx_markers": route_string_array(surface, "data_dx_markers"),
            "interaction_selectors": route_string_array(surface, "interaction_selectors"),
            "source_guard_ids": route_guards.clone(),
            "open_policy": "source-owned-open-only",
            "writes_files": false,
            "node_modules_required": false
        }));
    }

    for package in route_string_array(route, "forge_packages") {
        if seen.contains(&package) {
            continue;
        }
        surfaces.push(json!({
            "package": package,
            "selector": format!("[data-dx-package=\"{package}\"]"),
            "source_file": route
                .get("source_files")
                .and_then(Value::as_array)
                .and_then(|files| files.first())
                .and_then(Value::as_str)
                .unwrap_or(""),
            "materialized_file": route
                .get("materialized_files")
                .and_then(Value::as_array)
                .and_then(|files| files.first())
                .and_then(Value::as_str)
                .unwrap_or(""),
            "api_surface": [],
            "readiness": route.get("status").and_then(Value::as_str).unwrap_or("source-ready-runtime-gated"),
            "data_dx_markers": ["data-dx-package"],
            "source_guard_ids": route_guards.clone(),
            "open_policy": "source-owned-open-only",
            "writes_files": false,
            "node_modules_required": false
        }));
    }

    surfaces
}

fn launch_package_surfaces() -> Vec<Value> {
    vec![
        json!({
            "package": "shadcn/ui/button",
            "source_file": "examples/template/template-dashboard-nav.tsx",
            "materialized_file": "components/template-app/template-dashboard-nav.tsx",
            "api_surface": ["Button"],
            "readiness": "source-owned-ui-primitive",
            "data_dx_markers": [
                "data-dx-package",
                "data-dx-check-package-lane-row",
                "data-dx-check-package-lane-hash-refresh-status",
                "data-dx-check-package-lane-hash-refresh-helper",
                "data-dx-check-package-lane-hash-refresh-json-command",
                "data-dx-check-package-lane-hash-refresh-zed"
            ]
        }),
        json!({
            "package": "shadcn/ui/input",
            "source_file": "examples/template/template-lead-form.tsx",
            "materialized_file": "components/template-app/template-lead-form.tsx",
            "api_surface": ["DxInputField"],
            "readiness": "source-owned-form-field",
            "data_dx_markers": ["data-dx-package"]
        }),
        json!({
            "package": "shadcn/ui/separator",
            "source_file": "examples/template/template-shell.tsx",
            "materialized_file": "components/template-app/template-shell.tsx",
            "api_surface": ["Separator"],
            "readiness": "source-owned-layout-primitive",
            "data_dx_markers": ["data-dx-package"]
        }),
        json!({
            "package": "shadcn/ui/textarea",
            "source_file": "examples/template/template-lead-form.tsx",
            "materialized_file": "components/template-app/template-lead-form.tsx",
            "api_surface": ["Textarea"],
            "readiness": "source-owned-form-field",
            "data_dx_markers": ["data-dx-package"]
        }),
        json!({
            "package": "dx/icon/search",
            "source_file": "examples/template/icon-status.tsx",
            "materialized_file": "components/template-app/icon-status.tsx",
            "api_surface": ["Icon"],
            "readiness": "dx-icons-registry",
            "data_dx_markers": ["data-dx-package"]
        }),
        json!({
            "package": "auth/better-auth",
            "source_file": "examples/template/auth-session-status.tsx",
            "materialized_file": "components/template-app/auth-session-status.tsx",
            "api_surface": ["useSession", "signOut"],
            "readiness": "env-gated-real-adapter",
            "data_dx_markers": ["data-dx-package"]
        }),
        json!({
            "package": "animation/motion",
            "front_facing_name": "Motion Dashboard Choreography",
            "dx_icon": "pack:motion",
            "source_file": "examples/template/template-shell.tsx",
            "materialized_file": "components/template-app/template-shell.tsx",
            "api_surface": [
                "MotionConfig",
                "LazyMotion",
                "AnimatePresence",
                "LayoutGroup",
                "Reorder",
                "useReducedMotion",
                "useAnimate"
            ],
            "readiness": "visible-motion-dashboard-workflow",
            "dashboard_workflow": "motion-panel-orchestration",
            "source_mirror": "G:/WWW/inspirations/motion",
            "receipt_paths": [
                "examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json",
                "docs/packages/animation-motion.md"
            ],
            "data_dx_markers": [
                "data-dx-package",
                "data-dx-component",
                "data-dx-dashboard-workflow",
                "data-dx-motion-interaction",
                "data-dx-motion-reduced"
            ]
        }),
        json!({
            "package": "forms/react-hook-form",
            "source_file": "examples/template/template-lead-form.tsx",
            "materialized_file": "components/template-app/template-lead-form.tsx",
            "api_surface": ["DxHookForm", "handleSubmit"],
            "readiness": "source-owned-form-adapter",
            "data_dx_markers": ["data-dx-package"]
        }),
        json!({
            "package": "i18n/next-intl",
            "front_facing_name": "Internationalization Dashboard Locale Workflow",
            "dx_icon": "pack:i18n",
            "source_file": "examples/template/next-intl-dashboard-locale.tsx",
            "materialized_file": "components/template-app/next-intl-dashboard-locale.tsx",
            "api_surface": [
                "NextIntlClientProvider",
                "useTranslations",
                "useLocale",
                "useFormatter",
                "createDxDashboardIntlReceipt",
                "createDxDashboardIntlFormatPreview",
                "createDxDashboardIntlNumberPreview",
                "getDxDashboardLocaleAlternateLinks"
            ],
            "readiness": "visible-dashboard-locale-workflow",
            "dashboard_workflow": "locale-copy-boundary",
            "source_mirror": "G:/WWW/inspirations/next-intl",
            "receipt_paths": [
                ".dx/forge/receipts/*-i18n-next-intl.json",
                "docs/packages/next-intl.md",
                "docs/packages/next-intl.source-guard-runbook.json",
                "examples/template/.dx/forge/receipts/2026-05-22-i18n-next-intl-dashboard-locale.json"
            ],
            "data_dx_markers": [
                "data-dx-package",
                "data-dx-component",
                "data-dx-intl-dashboard-workflow",
                "data-dx-intl-action",
                "data-dx-dashboard-copy-locale",
                "data-dx-intl-receipt-state",
                "data-dx-intl-message-namespace",
                "data-dx-intl-copy-target",
                "data-dx-intl-readiness-copy",
                "data-dx-check-package-lane-template",
                "data-dx-check-package-lane-row",
                "data-dx-check-package-lane-status",
                "data-dx-check-package-lane-receipt-status",
                "data-dx-check-package-lane-upstream-package",
                "data-dx-check-package-lane-source-mirror",
                "data-dx-check-package-lane-receipt-path",
                "data-dx-check-package-lane-dx-style-status",
                "data-dx-check-package-lane-hash-refresh-status",
                "data-dx-check-package-lane-hash-refresh-helper",
                "data-dx-check-package-lane-hash-refresh-json-command",
                "data-dx-check-package-lane-hash-refresh-zed",
                "data-dx-check-package-lane-hash-refresh-tracked-files",
                "data-dx-check-package-lane-hash-refresh-stale-files",
                "data-dx-check-package-lane-hash-refresh-missing-files",
                "data-dx-check-package-lane-hash-refresh-current-file-list",
                "data-dx-check-package-lane-hash-refresh-stale-file-list",
                "data-dx-check-package-lane-hash-refresh-missing-file-list",
                "data-dx-check-package-lane-hash-refresh-stale-mirror-file-list",
                "data-dx-check-package-lane-hash-refresh-missing-mirror-file-list",
                "data-dx-check-package-lane-hash-refresh-current-metric",
                "data-dx-check-package-lane-hash-refresh-stale-metric",
                "data-dx-check-package-lane-hash-refresh-missing-metric",
                "data-dx-style-surface",
                "data-dx-token-scope",
                "data-dx-intl-locale-option",
                "data-dx-intl-preview-locale",
                "data-dx-intl-provider-locale",
                "data-dx-intl-route-preview",
                "data-dx-intl-format-preview",
                "data-dx-intl-format-source-api",
                "data-dx-intl-format-time-zone",
                "data-dx-intl-number-preview",
                "data-dx-intl-number-source-api",
                "data-dx-intl-number-currency",
                "data-dx-intl-alternate-links",
                "data-dx-intl-alternate-locale",
                "data-dx-intl-alternate-href",
                "data-dx-intl-hreflang",
                "data-dx-intl-locale-prefix",
                "data-dx-intl-plan-label",
                "data-dx-intl-support-sla",
                "data-dx-intl-receipt-locale",
                "data-dx-intl-receipt-route",
                "data-dx-intl-receipt-number-source",
                "data-dx-intl-receipt-hreflang"
            ],
            "interaction_selectors": [
                "[data-dx-intl-action]",
                "[data-dx-intl-copy-target]",
                "[data-dx-intl-readiness-copy]",
                "[data-dx-intl-locale-option]",
                "[data-dx-intl-receipt-state]",
                "[data-dx-intl-alternate-links]",
                "[data-dx-intl-format-preview]",
                "[data-dx-intl-number-preview]",
                "[data-dx-intl-hreflang]"
            ]
        }),
        json!({
            "package": "tanstack/query",
            "source_file": "examples/template/query-cache-status.tsx",
            "read_model_source_file": "examples/template/query-dashboard-read-model.ts",
            "materialized_file": "components/template-app/query-cache-status.tsx",
            "read_model_materialized_file": "components/template-app/query-dashboard-read-model.ts",
            "api_surface": ["useQuery", "dxQueryOptions"],
            "readiness": "source-owned-cache-adapter",
            "data_dx_markers": ["data-dx-package"]
        }),
        json!({
            "package": "reactive/store",
            "front_facing_name": "Reactive Store",
            "official_dx_package_name": "Reactive Store",
            "upstream_package": "@tanstack/store",
            "based_on": "@tanstack/react-store",
            "upstream_version": "0.11.0",
            "dx_icon": "pack:state",
            "source_file": "examples/template/lib/forge/state/reactive-store/context.tsx",
            "materialized_file": "lib/forge/state/reactive-store/context.tsx",
            "secondary_source_file": "tools/launch/runtime-template/pages/index.html",
            "secondary_materialized_file": "pages/index.html",
            "api_surface": [
                "createStoreContext",
                "StoreProvider",
                "useStoreContext",
                "Store",
                "ReadonlyStore",
                "createStore"
            ],
            "readiness": "source-owned-react-context-package-lane-status",
            "selected_surfaces": ["react-context"],
            "source_mirror": "G:/WWW/inspirations/tanstack-store",
            "receipt_paths": [
                "examples/template/.dx/forge/receipts/packages/reactive-store.json",
                "docs/packages/reactive-store.md"
            ],
            "data_dx_markers": [
                "data-dx-check-package-lane-row",
                "data-dx-check-package-lane-status",
                "data-dx-check-package-lane-receipt-status",
                "data-dx-check-package-lane-upstream-package",
                "data-dx-check-package-lane-source-mirror",
                "data-dx-check-package-lane-receipt-path",
                "data-dx-check-package-lane-dx-style-status"
            ]
        }),
        json!({
            "package": "validation/zod",
            "source_file": "examples/template/zod-validation-status.tsx",
            "materialized_file": "components/template-app/zod-validation-status.tsx",
            "api_surface": ["validateDxInput", "dxToJsonSchema"],
            "readiness": "source-owned-validation-adapter",
            "data_dx_markers": ["data-dx-zod-status"]
        }),
        json!({
            "package": "payments/stripe-js",
            "front_facing_name": "Payments Billing Workflow",
            "dx_icon": "pack:payments",
            "source_file": "examples/template/payments-status.tsx",
            "materialized_file": "components/template-app/payments-status.tsx",
            "api_surface": [
                "readDxStripeClientConfig",
                "submitDxStripeCheckoutContact",
                "createDxStripeDashboardCheckoutRequest",
                "createDxStripeDashboardMissingConfigReceipt",
                "dxStripeDashboardPlans",
                "createDxStripeEmbeddedCheckoutClientSecretFetcher"
            ],
            "readiness": "visible-billing-workflow-missing-config-receipt",
            "dashboard_workflow": "billing-checkout",
            "source_mirror": "G:/WWW/inspirations/stripe-js",
            "required_env": [
                "NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY",
                "STRIPE_SECRET_KEY",
                "STRIPE_PRICE_ID",
                "STRIPE_PRICE_ID_STARTER",
                "STRIPE_PRICE_ID_TEAM",
                "STRIPE_PRICE_ID_SCALE"
            ],
            "receipt_paths": [
                ".dx/forge/receipts/payments-stripe-js.json",
                "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
                ".dx/forge/docs/dashboard-stripe-plan-checkout.md",
                "benchmarks/stripe-payment-launch-proof.test.ts"
            ],
            "data_dx_markers": [
                "data-dx-package",
                "data-dx-component",
                "data-dx-dashboard-flow",
                "data-dx-payment-status",
                "data-dx-checkout-submit-state",
                "data-dx-stripe-dashboard-workflow",
                "data-dx-stripe-action",
                "data-dx-stripe-plan-id",
                "data-dx-stripe-price-env",
                "data-dx-stripe-checkout-mode",
                "data-dx-stripe-local-receipt",
                "data-dx-stripe-receipt-path",
                "data-dx-stripe-submit-state",
                "data-dx-check-package-lane-row",
                "data-dx-check-package-lane-status",
                "data-dx-check-package-lane-receipt-status",
                "data-dx-check-package-lane-upstream-package",
                "data-dx-check-package-lane-source-mirror",
                "data-dx-check-package-lane-receipt-path",
                "data-dx-check-package-lane-dx-style-status",
                "data-dx-check-package-lane-hash-refresh-status",
                "data-dx-check-package-lane-hash-refresh-helper",
                "data-dx-check-package-lane-hash-refresh-json-command",
                "data-dx-check-package-lane-hash-refresh-zed",
                "data-dx-check-package-lane-hash-refresh-tracked-files",
                "data-dx-check-package-lane-hash-refresh-stale-files",
                "data-dx-check-package-lane-hash-refresh-missing-files",
                "data-dx-check-package-lane-hash-refresh-current-file-list",
                "data-dx-check-package-lane-hash-refresh-stale-file-list",
                "data-dx-check-package-lane-hash-refresh-missing-file-list",
                "data-dx-check-package-lane-hash-refresh-stale-mirror-file-list",
                "data-dx-check-package-lane-hash-refresh-missing-mirror-file-list",
                "data-dx-style-surface",
                "data-dx-token-scope"
            ]
        }),
        json!({
            "package": "automations/n8n",
            "front_facing_name": "Automation Connectors",
            "official_dx_package_name": "Automation Connectors",
            "upstream_package": "n8n-nodes-base",
            "upstream_version": "2.22.0",
            "source_file": "examples/template/template-shell.tsx",
            "materialized_file": "components/template-app/template-shell.tsx",
            "mission_summary_source_file": "examples/template/automation-mission-summary.tsx",
            "mission_summary_materialized_file": "components/template-app/automation-mission-summary.tsx",
            "api_surface": [
                "LaunchAutomationBridgeStatus",
                "LaunchAutomationDashboardState",
                "automationRoutes",
                "automationSummary",
                "connectorMetadata",
                "credentialMetadata",
                "buildDxN8nCredentialReadiness",
                "requiredEnvForDxN8nConnector",
                "createDxN8nRunReceipt"
            ],
            "readiness": "dashboard-release-automation-workflow",
            "dashboard_workflow": "automation-release-receipt",
            "source_mirror": "G:/WWW/inspirations/n8n/packages/nodes-base",
            "selected_surfaces": [
                "connector-catalog",
                "credential-readiness",
                "redacted-run-receipt",
                "launch-dashboard-workflow",
                "starter-dashboard-workflow",
                "zed-run-handoff"
            ],
            "honesty_label": "ADAPTER-BOUNDARY",
            "receipt_paths": [
                "G:/Dx/.dx/receipts/automations/launch-release-notification.json",
                "G:/Dx/.dx/receipts/automations/run-latest.json",
                ".dx/forge/receipts/automations/readiness.json"
            ],
            "data_dx_markers": [
                "data-dx-package",
                "data-dx-component",
                "data-dx-dashboard-workflow",
                "data-dx-check-package-lane-row",
                "data-dx-check-package-lane-dx-style-status",
                "data-dx-style-surface=\"automation-connectors\"",
                "data-dx-token-scope=\"automations/n8n\"",
                "data-dx-automation-dashboard-card",
                "data-dx-component=\"launch-automation-connector-workflow\"",
                "data-dx-component=\"launch-automation-catalog-summary\"",
                "data-dx-component=\"launch-automation-mission-summary\"",
                "data-dx-automation-workflow=\"connector-readiness\"",
                "data-dx-automation-dashboard-state",
                "data-dx-automation-selected-connector",
                "data-dx-automation-intent-input",
                "data-dx-automation-receipt-path",
                "data-dx-automation-run-receipt-path",
                "data-dx-automation-receipt-intent",
                "data-dx-automation-run-receipt-intent",
                "data-dx-automation-credential-schema",
                "data-dx-automation-auth-kind",
                "data-dx-automation-credential-type",
                "data-dx-automation-required-env",
                "data-dx-automation-workflow-node-readiness",
                "data-dx-automation-usable-as-tool"
            ]
        }),
        json!({
            "package": "state/zustand",
            "front_facing_name": "State Management",
            "dx_icon": "pack:state",
            "source_file": "examples/template/state-zustand-dashboard.tsx",
            "materialized_file": "components/template-app/state-zustand-dashboard.tsx",
            "api_surface": [
                "createWithEqualityFn",
                "persist",
                "rehydrate",
                "onHydrate",
                "onFinishHydration",
                "LaunchDashboardStateControl",
                "LaunchDashboardStateShell",
                "dx-template-dashboard-settings"
            ],
            "readiness": "visible-dashboard-state-persistence-workflow",
            "dashboard_workflow": "ui-state-persistence",
            "shell_source_file": "examples/template/template-shell.tsx",
            "shell_materialized_file": "components/template-app/template-shell.tsx",
            "source_mirror": "G:/WWW/inspirations/zustand",
            "receipt_paths": [
                "examples/template/.dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json",
                "benchmarks/zustand-launch-materialized.test.ts"
            ],
            "data_dx_markers": [
                "data-dx-package",
                "data-dx-component",
                "data-dx-dashboard-workflow",
                "data-dx-component=\"launch-dashboard-state-shell\"",
                "data-dx-zustand-store",
                "data-dx-zustand-persist-key",
                "data-dx-zustand-action",
                "data-dx-zustand-action=\"rehydrate-dashboard-settings\"",
                "data-dx-zustand-hydration-event",
                "data-dx-zustand-rehydrate-state",
                "data-dx-check-package-lane-row"
            ]
        }),
        json!({
            "package": "ai/vercel-ai",
            "front_facing_name": "AI SDK Launch Assistant",
            "official_dx_package_name": "AI SDK",
            "upstream_package": "ai",
            "upstream_version": "7.0.0-canary.146",
            "dx_icon": "pack:ai",
            "source_file": "examples/template/ai-chat-status.tsx",
            "materialized_file": "components/template-app/ai-chat-status.tsx",
            "api_surface": [
                "DxAIClientChat",
                "streamText",
                "convertToModelMessages",
                "tool",
                "gateway",
                "createProviderRegistry"
            ],
            "readiness": "visible-prompt-action-provider-missing-route-contract",
            "dashboard_workflow": "prompt-action-provider-readiness",
            "source_mirror": "G:/WWW/inspirations/vercel-ai",
            "data_dx_markers": [
                "data-dx-package",
                "data-dx-component",
                "data-dx-dashboard-workflow",
                "data-dx-ai-route-contract",
                "data-dx-ai-action",
                "data-dx-ai-action-state",
                "data-dx-ai-config-state",
                "data-dx-ai-local-response",
                "data-dx-ai-prompt-state",
                "data-dx-ai-request-id",
                "data-dx-check-package-lane-row",
                "data-dx-check-package-lane-dx-style-status"
            ]
        }),
        json!({
            "package": "api/trpc",
            "front_facing_name": "Type-Safe API Dashboard",
            "upstream_package": "@trpc/server",
            "dx_icon": "api:trpc",
            "source_file": "examples/template/template-shell.tsx",
            "materialized_file": "components/template-app/template-shell.tsx",
            "secondary_source_file": "examples/template/trpc-launch-health.tsx",
            "secondary_materialized_file": "components/template-app/trpc-launch-health.tsx",
            "api_surface": [
                "initTRPC.create",
                "fetchRequestHandler",
                "createTRPCClient",
                "httpBatchLink",
                "createTRPCOptionsProxy",
                "trpc.health.queryOptions",
                "trpc.launchEvent.mutationOptions",
                "trpc.health.queryFilter"
            ],
            "readiness": "visible-typed-api-dashboard-workflow",
            "dashboard_workflow": "typed-api-readiness",
            "source_mirror": "G:/WWW/inspirations/trpc",
            "receipt_paths": [
                "docs/packages/api-trpc.md",
                "examples/template/.dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json",
                "benchmarks/trpc-launch-runtime-proof.test.ts"
            ],
            "data_dx_markers": [
                "data-dx-package",
                "data-dx-component",
                "data-dx-dashboard-card",
                "data-dx-dashboard-workflow",
                "data-dx-trpc-workflow",
                "data-dx-trpc-action",
                "data-trpc-interaction",
                "data-trpc-mutation-state",
                "data-dx-trpc-receipt-state",
                "data-dx-trpc-request-id",
                "data-dx-check-package-lane-row",
                "data-dx-check-package-lane-hash-refresh-status",
                "data-dx-check-package-lane-hash-refresh-helper",
                "data-dx-check-package-lane-hash-refresh-json-command",
                "data-dx-check-package-lane-hash-refresh-zed",
                "data-dx-check-package-lane-hash-refresh-current-file-list",
                "data-dx-check-package-lane-hash-refresh-stale-file-list",
                "data-dx-check-package-lane-hash-refresh-missing-file-list",
                "data-dx-check-package-lane-hash-refresh-stale-mirror-file-list",
                "data-dx-check-package-lane-hash-refresh-missing-mirror-file-list"
            ],
            "interaction_selectors": [
                "[data-dx-trpc-action=\"check-health\"]",
                "[data-dx-trpc-action=\"prepare-launch-event\"]",
                "[data-trpc-interaction=\"health-query\"]",
                "[data-trpc-interaction=\"local-launch-event-mutation\"]"
            ]
        }),
        json!({
            "package": "content/fumadocs-next",
            "front_facing_name": "Fumadocs Docs Help Workflow",
            "dx_icon": "pack:fumadocs",
            "source_file": "examples/template/docs-status.tsx",
            "materialized_file": "components/template-app/docs-status.tsx",
            "api_surface": [
                "dxFumadocsRouteContract",
                "dxFumadocsDashboardPages",
                "createFumadocsNavigationReceipt",
                "dxFumadocsOpenAPICodeUsageContract"
            ],
            "readiness": "dashboard-help-changelog-workflow",
            "dashboard_workflow": "docs-help-changelog",
            "source_mirror": "G:/WWW/inspirations/fumadocs",
            "receipt_paths": [
                ".dx/forge/docs/content-fumadocs-next.md",
                ".dx/forge/receipts/*-content-fumadocs-next.json",
                "docs/packages/content-fumadocs-next.md",
                "docs/packages/content-fumadocs-next.source-guard-runbook.json",
                "examples/template/.dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json"
            ],
            "data_dx_markers": [
                "data-dx-package",
                "data-dx-component",
                "data-dx-dashboard-workflow",
                "data-dx-dashboard-card",
                "data-dx-product-surface",
                "data-dx-fumadocs-dashboard-target",
                "data-dx-fumadocs-interaction",
                "data-dx-fumadocs-action",
                "data-dx-fumadocs-page-option",
                "data-dx-fumadocs-rendered-markdown",
                "data-dx-fumadocs-changelog",
                "data-dx-fumadocs-rendered-route",
                "data-dx-fumadocs-selected-page",
                "data-dx-fumadocs-toc-count",
                "data-dx-fumadocs-local-response",
                "data-dx-fumadocs-receipt-route",
                "data-dx-fumadocs-missing-config",
                "data-dx-docs-status",
                "data-dx-docs-openapi-code-usage",
                "data-dx-docs-openapi-proxy"
            ],
            "interaction_selectors": [
                "[data-dx-fumadocs-interaction=\"page-tree-selector\"]",
                "[data-dx-fumadocs-action=\"safe-local-route-preview\"]",
                "[data-dx-fumadocs-page-option]",
                "[data-dx-fumadocs-rendered-markdown=\"active-page\"]",
                "[data-dx-fumadocs-changelog=\"launch-docs\"]"
            ]
        }),
        json!({
            "package": "content/react-markdown",
            "source_file": "examples/template/react-markdown-preview.tsx",
            "materialized_file": "components/template-app/react-markdown-preview.tsx",
            "api_surface": ["DxMarkdown", "skipHtml"],
            "readiness": "source-owned-markdown-boundary",
            "data_dx_markers": ["data-dx-package"]
        }),
        json!({
            "package": "supabase/client",
            "front_facing_name": "Backend Platform Client",
            "official_dx_package_name": "Backend Platform Client",
            "dx_icon": "database:supabase",
            "source_file": "examples/template/supabase-profile-workflow.tsx",
            "materialized_file": "components/template-app/supabase-profile-workflow.tsx",
            "secondary_source_file": "examples/template/data-status.tsx",
            "secondary_materialized_file": "components/template-app/data-status.tsx",
            "api_surface": [
                "readSupabasePublicConfig",
                "getDxSupabaseCurrentProfile",
                "upsertDxSupabaseProfile",
                "readDxSupabaseProfileConfigStatus",
                "createDxSupabaseProfilePreview",
                "createDxSupabaseProfileUpsertReceipt",
                "readDxSupabaseProfilesReadModel",
                "LaunchSupabaseProfileWorkflow"
            ],
            "readiness": "visible-account-profile-and-schema-query-workflow",
            "dashboard_workflow": "account-profile-settings",
            "secondary_dashboard_workflow": "supabase-schema-query",
            "source_mirror": "G:/WWW/inspirations/supabase",
            "receipt_paths": [
                ".dx/forge/docs/supabase-client.md",
                ".dx/forge/receipts/*-supabase-client.json",
                "examples/template/.dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json",
                ".dx/forge/receipts/2026-05-22-supabase-client-dashboard-workflow.json",
                "docs/packages/supabase-client.md"
            ],
            "app_owned_boundaries": [
                "Supabase project provisioning",
                "public env configuration",
                "profile table and RLS policy",
                "auth redirect and provider credentials",
                "service-role secret handling"
            ],
            "data_dx_markers": [
                "data-dx-package",
                "data-dx-component",
                "data-dx-dashboard-workflow",
                "data-dx-dashboard-card",
                "data-dx-supabase-workflow",
                "data-dx-supabase-action",
                "data-dx-supabase-profile-field",
                "data-dx-supabase-config-status",
                "data-dx-supabase-query-state",
                "data-dx-supabase-query-operation",
                "data-dx-supabase-receipt-path"
            ],
            "interaction_selectors": [
                "[data-dx-supabase-action=\"load-profile-fixture\"]",
                "[data-dx-supabase-action=\"prepare-profile-upsert\"]",
                "[data-dx-supabase-action=\"run-local-schema-query\"]",
                "[data-dx-supabase-query-operation]",
                "[data-dx-component=\"supabase-schema-query-workflow\"]",
                "[data-dx-supabase-receipt-path]"
            ]
        }),
        json!({
            "package": "db/drizzle-sqlite",
            "front_facing_name": "Database ORM",
            "dx_icon": "pack:database",
            "source_file": "examples/template/drizzle-query-proof.tsx",
            "materialized_file": "components/template-app/drizzle-query-proof.tsx",
            "api_surface": [
                "LaunchDrizzleDashboardData",
                "readDrizzleDashboardOverview",
                "readDrizzleDashboardQueryPlan",
                "readDrizzleDashboardQueryPlanById",
                "sqliteTable",
                "relations",
                "drizzle"
            ],
            "readiness": "visible-sqlite-read-model-workflow",
            "dashboard_workflow": "sqlite-read-model",
            "source_mirror": "G:/WWW/inspirations/drizzle-orm",
            "receipt_paths": [
                "docs/packages/db-drizzle-sqlite.md",
                "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
                "benchmarks/drizzle-launch-proof.test.ts"
            ],
            "data_dx_markers": [
                "data-dx-package",
                "data-dx-component",
                "data-dx-dashboard-workflow",
                "data-dx-product-surface",
                "data-dx-dashboard-target",
                "data-dx-drizzle-action",
                "data-dx-drizzle-status",
                "data-dx-drizzle-read-model",
                "data-dx-drizzle-query-plan-id",
                "data-dx-backend-status",
                "data-dx-backend-detail",
                "data-dx-drizzle-receipt-path",
                "data-dx-drizzle-runtime-dependencies",
                "data-dx-check-package-lane-row",
                "data-dx-check-package-lane-status",
                "data-dx-check-package-lane-receipt-status",
                "data-dx-check-package-lane-upstream-package",
                "data-dx-check-package-lane-source-mirror",
                "data-dx-check-package-lane-receipt-path",
                "data-dx-check-package-lane-dx-style-status"
            ],
            "interaction_selectors": [
                "[data-dx-drizzle-action=\"select-read-model\"]",
                "[data-dx-drizzle-action=\"preview-query-plan\"]",
                "[data-dx-drizzle-action=\"apply-read-model\"]",
                "[data-dx-dashboard-target=\"mission-control-database\"]"
            ]
        }),
        json!({
            "package": "instantdb/react",
            "source_file": "examples/template/instantdb-status.tsx",
            "materialized_file": "components/template-app/instantdb-status.tsx",
            "api_surface": ["db.useQuery", "db.rooms.usePresence"],
            "readiness": "env-gated-real-adapter",
            "data_dx_markers": ["data-dx-instant-status"]
        }),
        json!({
            "package": "wasm/bindgen",
            "official_name": "WebAssembly Bridge",
            "upstream_package": "wasm-bindgen",
            "source_file": "examples/template/wasm-interop-status.tsx",
            "materialized_file": "components/template-app/wasm-interop-status.tsx",
            "api_surface": ["useWasmBindgenModule", "WasmBindgenFactory"],
            "readiness": "source-owned-wasm-boundary",
            "data_dx_markers": ["data-dx-package"]
        }),
        json!({
            "package": "3d/launch-scene",
            "front_facing_name": "3D Scene System",
            "dx_icon": "pack:cube",
            "source_file": "examples/template/launch-scene.tsx",
            "materialized_file": "components/scene/launch-scene.tsx",
            "api_surface": ["createDxLaunchScenePreset", "dxSceneMaterialPalettes", "resolveDxSceneMaterialPalette", "createDxSceneDashboardWorkflow", "createDxSceneDashboardReceipt", "cycleDxSceneQualityProfile", "cycleDxSceneMaterialPalette", "cycleDxSceneCameraRig", "captureDxSceneFrameSample", "createDxSceneCapabilityReport", "createDxSceneViewportReport", "createDxSceneBoundsReport", "createDxSceneRaycastReport", "dxSceneDashboardCameraRigs", "resolveDxSceneDashboardCameraRig", "mountDxWebGLScene"],
            "readiness": "source-owned-webgl-scene-dashboard-workflow",
            "data_dx_markers": ["data-dx-package", "data-dx-dashboard-workflow", "data-dx-scene-action", "data-dx-scene-status", "data-dx-scene-preview-readiness", "data-dx-scene-quality-profile", "data-dx-scene-material-palette", "data-dx-scene-camera-rig", "data-dx-scene-frame-sample", "data-dx-scene-capability-report", "data-dx-scene-capability-status", "data-dx-scene-viewport-report", "data-dx-scene-viewport-status", "data-dx-scene-bounds-report", "data-dx-scene-bounds-status", "data-dx-scene-raycast-report", "data-dx-scene-raycast-status", "data-dx-scene-workflow-active", "data-dx-scene-workflow-receipt-state", "data-dx-check-package-lane-row", "data-dx-check-package-lane-status", "data-dx-check-package-lane-receipt-status", "data-dx-check-package-lane-dx-style-status"]
        }),
    ]
}

#[allow(clippy::too_many_arguments)]
fn converted_route(
    route: &'static str,
    label: &'static str,
    source: &'static str,
    page: &'static str,
    manifest: &'static str,
    receipt: &'static str,
    packages: &[&'static str],
    assets: &[&'static str],
) -> Value {
    json!({
        "route": route,
        "label": label,
        "role": "converted-route",
        "source": source,
        "status": "source-ready-runtime-gated",
        "source_files": [
            page,
            manifest,
            receipt,
            "examples/conversion-proof/forge/acceptance/no-runtime-route-acceptance.json",
            "examples/conversion-proof/forge/acceptance/rendered-proof-evidence.schema.json",
            "examples/conversion-proof/forge/acceptance/validate-rendered-proof-evidence.ts",
                "examples/conversion-proof/forge/acceptance/rendered-proof-import-plan.json",
                "examples/conversion-proof/forge/acceptance/prepare-rendered-proof-import.ts",
                "examples/conversion-proof/forge/acceptance/rendered-proof-runtime-approval-request.json",
                "examples/conversion-proof/forge/acceptance/request-rendered-proof-runtime-approval.ts",
                "examples/conversion-proof/forge/acceptance/review-rendered-proof-completeness.ts"
            ],
        "materialized_files": [page],
        "forge_packages": packages,
        "assets": assets,
        "data_dx_markers": [
            "data-dx-route",
            "data-dx-source",
            "data-dx-forge",
            "data-dx-package",
            "data-dx-hot-reload-target"
        ],
        "preview": preview_for(route)
    })
}

fn preview_for(route: &'static str) -> Value {
    let hot_reload_target = hot_reload_manifest::route_hot_reload_target(route);

    json!({
        "url": format!("http://127.0.0.1:3000{route}"),
        "route_path": route,
        "hot_reload_target": hot_reload_target,
        "hot_reload_version_endpoint": hot_reload_manifest::DX_HOT_RELOAD_VERSION_ENDPOINT,
        "hot_reload_protocol": hot_reload_manifest::DX_HOT_RELOAD_PROTOCOL,
        "hot_reload_protocol_format": hot_reload_manifest::DX_HOT_RELOAD_PROTOCOL_FORMAT,
        "hot_reload_transport": hot_reload_manifest::DX_HOT_RELOAD_TRANSPORT,
        "hot_reload_resource_query_param": hot_reload_manifest::DX_HOT_RELOAD_RESOURCE_QUERY_PARAM,
        "hot_reload_poll_receipt_schema": hot_reload_manifest::DX_HOT_RELOAD_POLL_RECEIPT_SCHEMA,
        "hot_reload_poll_receipt_format": hot_reload_manifest::DX_HOT_RELOAD_POLL_RECEIPT_FORMAT,
        "hot_reload": hot_reload_manifest::studio_route_hot_reload_read_model(route),
        "requires_running_dev_server": true,
        "dev_command": "dx dev",
        "no_execution": true
    })
}

fn studio_assets() -> Vec<Value> {
    vec![
        json!({"path": "examples/conversion-proof/public/vendor/shadcn-favicon-32x32.png", "route": "/ui", "kind": "converted-vendor-asset"}),
        json!({"path": "examples/conversion-proof/public/vendor/supabase-logo.svg", "route": "/database", "kind": "converted-vendor-asset"}),
        json!({"path": "examples/conversion-proof/public/vendor/convex-logo.svg", "route": "/backend", "kind": "converted-vendor-asset"}),
        json!({"path": "examples/template/scene/preset.ts", "route": "/", "kind": "source-owned-scene-preset"}),
        json!({"path": "examples/template/scene/README.md", "route": "/", "kind": "scene-license-boundary"}),
    ]
}

fn unique_forge_packages(routes: &[Value]) -> Vec<String> {
    let mut packages = routes
        .iter()
        .flat_map(|route| {
            route
                .get("forge_packages")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
        })
        .map(str::to_string)
        .collect::<Vec<_>>();
    packages.sort();
    packages.dedup();
    packages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_manifest_exposes_studio_routes_for_zed() {
        let report = build_studio_preview_manifest("2026-05-21T00:00:00Z");
        let routes = report["routes"].as_array().expect("routes");
        let paths = routes
            .iter()
            .map(|route| route["route"].as_str().expect("route"))
            .collect::<Vec<_>>();

        assert!(paths.contains(&"/"));
        assert!(paths.contains(&"/automations"));
        assert!(paths.contains(&"/ui"));
        assert!(paths.contains(&"/database"));
        assert!(paths.contains(&"/backend"));
        assert_eq!(report["commands"]["routes"], "dx www routes --json");
        assert_eq!(report["project_contract"]["no_node_modules_required"], true);
        assert_eq!(
            report["preview"]["hot_reload"]["version_endpoint"],
            "/_dx/hot-reload/version"
        );
        assert_eq!(
            report["preview"]["hot_reload"]["protocol"],
            "dx.hot-reload.poll"
        );
        assert_eq!(report["preview"]["hot_reload"]["protocol_format"], 1);
        assert_eq!(report["preview"]["hot_reload"]["transport"], "poll");
        assert_eq!(
            report["preview"]["hot_reload"]["poll_receipt"]["schema"],
            "dx.dev.hot_reload.poll_receipt"
        );
        assert_eq!(report["preview"]["hot_reload"]["poll_receipt"]["format"], 1);
        assert_eq!(
            report["preview"]["hot_reload"]["poll_receipt"]["boundary_field"],
            "receipt.boundaries"
        );
        assert_eq!(
            routes[0]["preview"]["hot_reload_poll_receipt_schema"],
            "dx.dev.hot_reload.poll_receipt"
        );
        assert_eq!(routes[0]["preview"]["hot_reload_poll_receipt_format"], 1);
        assert_eq!(
            routes[0]["preview"]["hot_reload_resource_query_param"],
            "resource"
        );
        assert_eq!(
            routes[0]["preview"]["hot_reload"]["schema"],
            "dx.studio.hot_reload_route_read_model"
        );
        assert_eq!(routes[0]["preview"]["hot_reload"]["format"], 1);
        assert_eq!(
            routes[0]["preview"]["hot_reload"]["target"],
            routes[0]["preview"]["hot_reload_target"]
        );
        assert_eq!(
            routes[0]["preview"]["hot_reload"]["partial_module_updates"],
            false
        );
        assert!(
            report["forge_packages_used"]
                .as_array()
                .expect("forge packages")
                .iter()
                .any(|package| package.as_str() == Some("auth/better-auth"))
        );
    }

    #[test]
    fn routes_report_is_the_compact_manifest_slice() {
        let report = build_www_routes_report("2026-05-21T00:00:00Z");
        assert_eq!(report["schema"], "dx.www.routes");
        assert_eq!(report["command"], "dx www routes --json");
        assert_eq!(report["route_count"], 5);
        assert_eq!(
            report["routes"][0]["preview"]["url"],
            "http://127.0.0.1:3000/"
        );
    }
}
