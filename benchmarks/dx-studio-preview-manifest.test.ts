const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  const fullPath = path.join(repoRoot, relativePath);
  assert.ok(fs.existsSync(fullPath), `expected ${relativePath} to exist`);
  return fs.readFileSync(fullPath, "utf8");
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function selectorMarker(selector) {
  const match = selector.match(/^\[([^=\]]+)="([^"]+)"\]$/);
  assert.ok(match, `expected selector ${selector} to be a stable data-dx attribute selector`);
  return `${match[1]}="${match[2]}"`;
}

function assertEditableSurfaceSourceOwnsSelector(editContract, surface) {
  const source = read(surface.sourceFile);
  assert.match(
    source,
    new RegExp(escapeRegExp(selectorMarker(surface.selector))),
    `${surface.sourceFile} must contain ${surface.selector}`,
  );
  assert.match(
    editContract,
    new RegExp(
      `id: "${escapeRegExp(surface.id)}"[\\s\\S]*selector: '${escapeRegExp(
        surface.selector,
      )}'[\\s\\S]*sourceFile: "${escapeRegExp(surface.sourceFile)}"`,
    ),
    `${surface.id} must point at the source file that owns its selector`,
  );
}

test("DX Studio manifest exposes Zed preview routes and JSON commands", () => {
  const studio = read("dx-www/src/cli/studio_manifest.rs");
  const hotReloadManifest = read("dx-www/src/cli/studio_manifest/hot_reload_manifest.rs");
  const cli = read("dx-www/src/cli/mod.rs");
  const launchShell = read("examples/template/template-shell.tsx");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const runtimeLaunch = read("tools/launch/runtime-template/pages/index.html");
  const materializer = read("tools/launch/materialize-www-template.ts");
  const automationsStatus = read("examples/template/automations-status.tsx");
  const wwwTemplateNodeModules = path.join(
    repoRoot,
    "examples",
    "www-template",
    "node_modules",
  );

  assert.match(studio, /"schema": "dx\.studio\.preview_manifest"/);
  assert.match(studio, /"routes": "dx www routes --json"/);
  assert.match(studio, /"preview_manifest": "dx www preview-manifest --json"/);
  assert.match(studio, /"forge_packages": "dx forge packages --json"/);
  assert.match(studio, /"package_surfaces": "routes\[\]\.package_surfaces"/);
  assert.match(studio, /"package_surface_fields": \[/);
  assert.match(studio, /"data_dx_marker_index": studio_marker_index\(\)/);
  assert.match(studio, /"source_guard_index": studio_source_guard_index\(\)/);
  assert.match(studio, /"source_guard_runbook_index": studio_source_guard_runbook_index\(&routes\)/);
  assert.match(studio, /"preview_watch_index": studio_preview_watch_index\(&routes\)/);
  assert.match(studio, /"route_readiness_index": studio_route_readiness_index\(&routes\)/);
  assert.match(studio, /"forge_readiness_index": studio_forge_readiness_index\(&routes\)/);
  assert.match(studio, /"source_selection_index": studio_source_selection_index\(&routes\)/);
  assert.match(studio, /"editable_surface_index": studio_editable_surface_index\(&routes\)/);
  assert.match(studio, /"edit_operation_index": studio_edit_operation_index\(\)/);
  assert.match(studio, /"env_contract_index": studio_env_contract_index\(&routes\)/);
  assert.match(studio, /"forge_receipt_index": studio_forge_receipt_index\(&routes\)/);
  assert.match(studio, /"check_panel": studio_dx_check_panel_contract\(\)/);
  assert.match(studio, /"check_panel": "check_panel"/);
  assert.match(studio, /"schema": "dx\.studio\.check_panel_contract"/);
  assert.match(studio, /"panel_schema": "dx\.www\.check_panel"/);
  assert.match(studio, /"view_model_schema": "dx\.www\.check_panel_view_model"/);
  assert.match(studio, /"embedded_zed_schema": "dx\.check\.zed_panel"/);
  assert.match(studio, /"command": "dx check --latest-receipt --json"/);
  assert.match(studio, /"receipt_path": "\.dx\/receipts\/check\/check-latest\.json"/);
  assert.match(studio, /"runs_expensive_checks": false/);
  assert.match(studio, /"writes_receipts": false/);
  assert.match(studio, /fn studio_marker_index\(\) -> Vec<Value>/);
  assert.match(studio, /fn studio_marker\(/);
  assert.match(studio, /fn studio_source_guard_index\(\) -> Vec<Value>/);
  assert.match(studio, /fn studio_source_guard\(/);
  assert.match(studio, /fn studio_source_guard_runbook_index\(routes: &\[Value\]\) -> Vec<Value>/);
  assert.match(studio, /fn source_guard_contracts_for_route\(route: &str\) -> Vec<Value>/);
  assert.match(studio, /fn source_guard_commands_for_route\(route: &str\) -> Vec<Value>/);
  assert.match(studio, /fn source_guard_command\(command: &'static str, purpose: &'static str\) -> Value/);
  assert.match(
    studio,
    /fn source_guard_contract\(\s*id: &'static str,\s*purpose: &'static str,\s*evidence_field: &'static str,\s*\) -> Value/s,
  );
  assert.match(studio, /fn studio_preview_watch_index\(routes: &\[Value\]\) -> Vec<Value>/);
  assert.match(studio, /fn studio_route_readiness_index\(routes: &\[Value\]\) -> Vec<Value>/);
  assert.match(studio, /fn studio_forge_readiness_index\(routes: &\[Value\]\) -> Vec<Value>/);
  assert.match(studio, /fn studio_source_selection_index\(routes: &\[Value\]\) -> Vec<Value>/);
  assert.match(studio, /fn source_selection_package_surfaces\(route: &Value, route_path: &str\) -> Vec<Value>/);
  assert.match(studio, /fn studio_editable_surface_index\(routes: &\[Value\]\) -> Vec<Value>/);
  assert.match(studio, /fn studio_edit_operation_index\(\) -> Vec<Value>/);
  assert.match(studio, /fn studio_edit_operation\(/);
  assert.match(studio, /fn studio_env_contract_index\(routes: &\[Value\]\) -> Vec<Value>/);
  assert.match(studio, /fn package_env_contracts\(package: &str\) -> Vec<Value>/);
  assert.match(studio, /fn env_contract\(name: &'static str, visibility: &'static str, purpose: &'static str\) -> Value/);
  assert.match(studio, /fn studio_forge_receipt_index\(routes: &\[Value\]\) -> Vec<Value>/);
  assert.match(studio, /fn route_receipt_artifacts\(route: &Value\) -> Vec<Value>/);
  assert.match(studio, /fn route_package_receipts\(route: &Value, route_path: &str\) -> Vec<Value>/);
  assert.match(studio, /fn receipt_artifact_kind\(path: &str\) -> &'static str/);
  assert.match(studio, /fn readiness_policy\(readiness: &\[String\]\) -> &'static str/);
  assert.match(studio, /fn route_string_array\(route: &Value, key: &str\) -> Vec<String>/);
  assert.match(studio, /fn source_guard_ids_for_route\(route: &str\) -> Vec<&'static str>/);
  assert.match(studio, /"selector": format!\("\[\{marker\}\]"\)/);
  assert.match(studio, /studio_marker\(\s*"data-dx-route"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-ready"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-check-panel"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-check-command"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-check-receipt-path"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-zustand-store"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-zustand-persist-key"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-zustand-action"/);
  assert.match(studio, /studio_marker\(\s*"data-launch-i18n-phase"/);
  assert.match(studio, /studio_marker\(\s*"data-visual-audit"/);
  assert.match(studio, /"id": id/);
  assert.match(studio, /"execution_policy": "source-only"/);
  assert.match(studio, /"starts_server": false/);
  assert.match(studio, /"runs_package_install": false/);
  assert.match(studio, /"runs_full_build": false/);
  assert.match(studio, /"scope": "source-only"/);
  assert.match(studio, /"requires_server": false/);
  assert.match(studio, /"requires_package_install": false/);
  assert.match(studio, /"requires_full_build": false/);
  assert.match(studio, /"default_action": "show-source-only-runbook"/);
  assert.match(studio, /studio_source_guard\(\s*"studio-preview-manifest"/);
  assert.match(studio, /studio_source_guard\(\s*"template-shell-package-slices"/);
  assert.match(studio, /studio_source_guard\(\s*"automations-route-bridge"/);
  assert.match(studio, /studio_source_guard\(\s*"website-conversion-routes"/);
  assert.match(studio, /studio_source_guard\(\s*"diff-whitespace-hygiene"/);
  assert.match(studio, /studio_source_guard\(\s*"conflict-marker-scan"/);
  assert.match(studio, /"zed_web_preview_contract": \{/);
  assert.match(studio, /"schema": "dx\.zed\.web_preview_contract"/);
  assert.match(studio, /"project_detection": \{/);
  assert.match(studio, /"strong_signals": \[/);
  assert.match(studio, /"manifest_command": "dx www preview-manifest --json"/);
  assert.match(studio, /"routes_command": "dx www routes --json"/);
  assert.match(studio, /"packages_command": "dx forge packages --json"/);
  assert.match(studio, /"activation_scope": "DX powers only activate when this manifest schema or project signals are present"/);
  assert.match(studio, /"route_picker": \{/);
  assert.match(studio, /"route_field": "routes\[\]\.route"/);
  assert.match(studio, /"package_surfaces_field": "routes\[\]\.package_surfaces"/);
  assert.match(studio, /"dom_selection": \{/);
  assert.match(studio, /"route_marker": "data-dx-route"/);
  assert.match(studio, /"check_panel_marker": "data-dx-check-panel"/);
  assert.match(studio, /"check_view_model_schema_marker": "data-dx-check-view-model-schema"/);
  assert.match(studio, /"check_view_model_status_marker": "data-dx-check-view-model-status"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-check-view-model-schema"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-check-view-model-status"/);
  assert.match(studio, /"visual_audit_marker": "data-visual-audit"/);
  assert.match(studio, /"semantic_actions": \[/);
  assert.match(studio, /"id": "open-source-file"/);
  assert.match(studio, /"id": "show-forge-package-readiness"/);
  assert.match(studio, /"id": "reload-route-scope"/);
  assert.match(studio, /"id": "show-source-guard"/);
  assert.match(studio, /"id": "show-dx-check-panel"/);
  assert.match(studio, /"requires_marker": "data-dx-check-panel"/);
  assert.match(studio, /"source_field": "check_panel"/);
  assert.match(studio, /"semantic_selection": \{/);
  assert.match(studio, /"index_field": "source_selection_index"/);
  assert.match(studio, /"edit_policy": "source-owned-edit-contract-explicit-user-action"/);
  assert.match(studio, /"drop-to-code"/);
  assert.match(studio, /"edit_contract": \{/);
  assert.match(studio, /"schema": "dx\.studio\.launch_edit_contract"/);
  assert.match(studio, /"source_manifest_file": "examples\/template\/dx-studio-edit-contract\.ts"/);
  assert.match(studio, /"editable_surface_index_field": "editable_surface_index"/);
  assert.match(studio, /"edit_operation_index_field": "edit_operation_index"/);
  assert.match(studio, /"operation_ids": \[/);
  assert.match(studio, /"insert_component"/);
  assert.match(studio, /"move_reorder_section"/);
  assert.match(studio, /"update_design_token"/);
  assert.match(studio, /"update_text_content"/);
  assert.match(studio, /"insert_icon_media"/);
  assert.match(studio, /"layout_policy": "responsive-design-system-grid"/);
  assert.match(studio, /"absolute_positioning": false/);
  assert.match(studio, /"design_token_scope_marker": "data-dx-token-scope"/);
  assert.match(studio, /"source_guards": \{/);
  assert.match(studio, /"index_field": "source_guard_index"/);
  assert.match(studio, /"runbook_index_field": "source_guard_runbook_index"/);
  assert.match(studio, /"command_field": "commands"/);
  assert.match(studio, /"contract_field": "contracts"/);
  assert.match(studio, /"default_mode": "show-before-run"/);
  assert.match(studio, /"allowed_scope": "source-only"/);
  assert.match(studio, /"file_watch": \{/);
  assert.match(studio, /"index_field": "preview_watch_index"/);
  assert.match(studio, /"watch_fields": \[/);
  assert.match(studio, /"reload_target_field": "hot_reload_target"/);
  assert.match(studio, /"default_reload": "route-scoped"/);
  assert.match(studio, /"ignored_roots": \[/);
  assert.match(studio, /"package_readiness": \{/);
  assert.match(studio, /"index_field": "forge_readiness_index"/);
  assert.match(studio, /"route_index_field": "route_readiness_index"/);
  assert.match(studio, /"dom_marker": "data-dx-package"/);
  assert.match(studio, /"default_badge": "source-owned-runtime-gated"/);
  assert.match(studio, /"env_contracts": \{/);
  assert.match(studio, /"index_field": "env_contract_index"/);
  assert.match(studio, /"default_badge": "app-owned-env-boundary"/);
  assert.match(studio, /"forge_receipts": \{/);
  assert.match(studio, /"index_field": "forge_receipt_index"/);
  assert.match(studio, /"route_receipts_field": "route_receipts"/);
  assert.match(studio, /"package_receipts_field": "package_receipts"/);
  assert.match(studio, /"default_badge": "source-owned-proof"/);
  assert.match(studio, /"reads_environment": false/);
  assert.match(studio, /"reads_runtime_artifacts": false/);
  assert.match(studio, /"normal_web_preview_behavior_preserved": true/);
  assert.match(studio, /"project_contract": \{/);
  assert.match(studio, /"config_file": "dx"/);
  assert.match(studio, /"no_node_modules_required": true/);
  assert.match(studio, /"package_policy": "forge-first-source-owned"/);
  assert.match(studio, /"hot_reload": hot_reload_manifest::studio_hot_reload_contract\(\)/);
  assert.match(hotReloadManifest, /"version_endpoint": DX_HOT_RELOAD_VERSION_ENDPOINT/);
  assert.match(studio, /"route": "\/"/);
  assert.match(studio, /"route": "\/automations"/);
  assert.match(studio, /"route": "\/ui"/);
  assert.match(studio, /"route": "\/database"/);
  assert.match(studio, /"route": "\/backend"/);
  assert.match(studio, /"examples\/template\/query-cache-status\.tsx"/);
  assert.match(studio, /"examples\/template\/automations-status\.tsx"/);
  assert.match(studio, /"examples\/template\/automations\/automations-metadata\.ts"/);
  assert.match(studio, /"examples\/template\/state-zustand-counter\.tsx"/);
  assert.match(studio, /"examples\/template\/state-zustand-dashboard\.tsx"/);
  assert.match(studio, /"examples\/template\/trpc-launch-health\.tsx"/);
  assert.match(studio, /"examples\/template\/wasm-interop-status\.tsx"/);
  assert.match(studio, /"examples\/template\/icon-status\.tsx"/);
  assert.match(studio, /"examples\/template\/next-intl-status\.tsx"/);
  assert.match(studio, /"examples\/template\/react-markdown-preview\.tsx"/);
  assert.match(studio, /"examples\/template\/scene\/webgl-runtime\.ts"/);
  assert.match(studio, /"components\/template-app\/query-cache-status\.tsx"/);
  assert.match(studio, /"components\/template-app\/state-zustand-counter\.tsx"/);
  assert.match(studio, /"components\/template-app\/state-zustand-dashboard\.tsx"/);
  assert.match(studio, /"lib\/scene\/webgl-runtime\.ts"/);
  assert.match(studio, /"package_surfaces": launch_package_surfaces\(\)/);
  assert.match(studio, /"no_node_modules_proof": \{/);
  assert.match(studio, /"guard": "benchmarks\/dx-studio-preview-manifest\.test\.ts"/);
  assert.match(studio, /"watch_files": watch_files/);
  assert.match(studio, /"marker_selectors": data_dx_markers/);
  assert.match(studio, /"source_guard_ids": source_guard_ids_for_route\(route_path\)/);
  assert.match(studio, /"editable_surfaces": route_editable_surfaces\(route_path\)/);
  assert.match(studio, /"contracts": source_guard_contracts_for_route\(route_path\)/);
  assert.match(studio, /"commands": source_guard_commands_for_route\(route_path\)/);
  assert.match(studio, /"source_file_count": source_files\.len\(\)/);
  assert.match(studio, /"forge_package_count": forge_packages\.len\(\)/);
  assert.match(studio, /"runtime_evidence": "explicit-approval-gated"/);
  assert.match(studio, /"runtime_policy": readiness_policy\(&readiness\)/);
  assert.match(studio, /"route_selector": format!\("\[data-dx-route=\\\"\{route_path\}\\\"\]"\)/);
  assert.match(studio, /"selector": format!\("\[data-dx-package=\\\"\{package\}\\\"\]"\)/);
  assert.match(studio, /"drop_to_code_ready": true/);
  assert.match(studio, /"open_policy": "source-owned-open-only"/);
  assert.match(studio, /"status": "declared-app-owned-not-read"/);
  assert.match(studio, /"runtime_policy": "app-owned-env-boundary"/);
  assert.match(studio, /"provenance_policy": "source-owned-declaration-no-runtime-read"/);
  assert.match(studio, /"receipt_family": "launch-package-surface"/);
  assert.match(studio, /"receipt_family": "route-declared-forge-package"/);
  assert.match(studio, /"reads_receipt_files": false/);
  assert.match(studio, /"forge-receipt"/);
  assert.match(studio, /"conversion-manifest"/);
  assert.match(studio, /"acceptance-checklist"/);
  assert.match(studio, /"rendered-proof-schema"/);
  assert.match(studio, /"rendered-proof-validator"/);
  assert.match(studio, /"rendered-proof-import-plan"/);
  assert.match(studio, /"rendered-proof-import-tool"/);
  assert.match(studio, /"rendered-proof-runtime-approval-request"/);
  assert.match(studio, /"rendered-proof-runtime-approval-tool"/);
  assert.match(studio, /"rendered-proof-completeness-review"/);
  assert.match(studio, /dx-studio-preview-manifest\.test\.ts/);
  assert.match(studio, /template-shell\.test\.ts/);
  assert.match(studio, /launch-package-slices\.test\.ts/);
  assert.match(studio, /automations-bridge\.test\.ts/);
  assert.match(studio, /dx-www-conversion-proof\.test\.ts/);
  assert.match(studio, /validate-rendered-proof-evidence\.ts --json/);
  assert.match(studio, /prepare-rendered-proof-import\.ts --json/);
  assert.match(studio, /request-rendered-proof-runtime-approval\.ts --json/);
  assert.match(studio, /review-rendered-proof-completeness\.ts --json/);
  assert.match(studio, /"automation-generated-manifest"/);
  assert.match(studio, /\.dx\/forge/);
  assert.match(studio, /forge\/conversion-manifests/);
  assert.match(studio, /forge\/acceptance\/review-rendered-proof-completeness\.ts/);
  assert.match(studio, /integrations\/n8n-nodes-base\/generated/);
  assert.match(studio, /"BETTER_AUTH_SECRET"/);
  assert.match(studio, /"NEXT_PUBLIC_STRIPE_PUBLISHABLE_KEY"/);
  assert.match(studio, /"AI_PROVIDER_API_KEY"/);
  assert.match(studio, /"NEXT_PUBLIC_SUPABASE_URL"/);
  assert.match(studio, /"NEXT_PUBLIC_INSTANT_APP_ID"/);
  assert.match(studio, /"node_modules_required": false/);
  assert.match(studio, /"package_install_required": false/);
  assert.match(studio, /"server_restart_required": false/);
  assert.match(studio, /"requires_node_modules": false/);
  assert.match(studio, /"requires_server_restart": false/);
  assert.match(studio, /"requires_package_install": false/);
  assert.match(studio, /"package": "auth\/better-auth"/);
  assert.match(studio, /"api_surface": \["useSession", "signOut"\]/);
  assert.match(studio, /"package": "shadcn\/ui\/separator"/);
  assert.match(studio, /"api_surface": \["Separator"\]/);
  assert.match(studio, /"package": "tanstack\/query"/);
  assert.match(studio, /"api_surface": \["useQuery", "dxQueryOptions"\]/);
  assert.match(studio, /"package": "state\/zustand"/);
  assert.match(
    studio,
    /"api_surface": \[\s*"createWithEqualityFn",\s*"persist",\s*"rehydrate",\s*"onHydrate",\s*"onFinishHydration",\s*"LaunchDashboardStateControl",\s*"LaunchDashboardStateShell",\s*"dx-template-dashboard-settings"\s*\]/,
  );
  assert.match(studio, /"data-dx-zustand-store"/);
  assert.match(studio, /"data-dx-zustand-persist-key"/);
  assert.match(studio, /"data-dx-zustand-action"/);
  assert.match(studio, /"package": "payments\/stripe-js"/);
  assert.match(
    studio,
    /"api_surface": \[\s*"readDxStripeClientConfig",\s*"submitDxStripeCheckoutContact",\s*"createDxStripeDashboardCheckoutRequest",\s*"createDxStripeDashboardMissingConfigReceipt",\s*"dxStripeDashboardPlans",\s*"createDxStripeEmbeddedCheckoutClientSecretFetcher"\s*\]/,
  );
  assert.match(studio, /"package": "3d\/launch-scene"/);
  assert.match(
    studio,
    /"package": "3d\/launch-scene"[\s\S]*"api_surface": \[[\s\S]*"createDxLaunchScenePreset"[\s\S]*"createDxSceneDashboardWorkflow"[\s\S]*"createDxSceneDashboardReceipt"[\s\S]*"mountDxWebGLScene"[\s\S]*\]/,
  );
  assert.match(studio, /"data-dx-route"/);
  assert.match(studio, /"data-dx-source"/);
  assert.match(studio, /"data-dx-forge"/);
  assert.match(studio, /"data-dx-package"/);
  assert.match(studio, /"data-dx-package-role"/);
  assert.match(studio, /"data-dx-hot-reload-target"/);
  assert.match(studio, /"data-dx-ready"/);
  assert.match(studio, /"data-dx-node-modules"/);
  assert.match(studio, /"data-dx-section"/);
  assert.match(studio, /"data-dx-component"/);
  assert.match(studio, /"data-dx-editable"/);
  assert.match(studio, /"data-dx-token-scope"/);
  assert.match(studio, /"data-dx-insert-slot"/);
  assert.match(studio, /"data-dx-payment-status"/);
  assert.match(studio, /"data-launch-i18n-phase"/);
  assert.match(studio, /"data-dx-dashboard-workflow"/);
  assert.match(studio, /"data-dx-check-panel"/);
  assert.match(studio, /"data-dx-check-command"/);
  assert.match(studio, /"data-dx-check-receipt-path"/);
  assert.match(studio, /"data-dx-check-schema"/);
  assert.match(studio, /"data-dx-check-score-max"/);
  assert.match(studio, /"data-dx-motion-interaction"/);
  assert.match(studio, /"data-dx-motion-reduced"/);
  assert.match(studio, /"data-dx-automation-view"/);
  assert.match(studio, /"data-dx-automation-dashboard-card"/);
  assert.match(studio, /"data-dx-automation-intent-input"/);
  assert.match(studio, /"data-dx-automation-receipt-path"/);
  assert.match(studio, /"supabase\/client"/);
  assert.match(studio, /"automations\/n8n"/);
  assert.match(studio, /"package": "automations\/n8n"/);
  assert.match(
    studio,
    /"api_surface": \[\s*"automationRoutes",\s*"automationSummary",\s*"connectorMetadata",\s*"credentialMetadata"\s*\]/,
  );
  assert.match(studio, /"data-dx-automation-route"/);
  assert.match(launchShell, /route = "\/"/);
  assert.match(launchShell, /data-dx-route=\{route\}/);
  assert.match(
    launchShell,
    /data-dx-source="examples\/template\/template-shell\.tsx"/,
  );
  assert.match(launchShell, /data-dx-hot-reload-target=\{`route:\$\{route\}`\}/);
  assert.match(launchShell, /data-dx-node-modules="forbidden"/);
  assert.match(launchShell, /data-dx-edit-contract=\{launchStudioEditContract\.schema\}/);
  assert.match(launchShell, /data-dx-token-scope="template"/);
  assert.match(launchShell, /data-dx-edit-id="launch\.root"/);
  assert.match(launchShell, /data-dx-edit-ops="insert_component,move_reorder_section,update_design_token,update_text_content,insert_icon_media"/);
  assert.match(launchShell, /data-dx-section="hero"/);
  assert.match(launchShell, /data-dx-section="proof-grid"/);
  assert.match(launchShell, /data-dx-section="package-catalog"/);
  assert.match(launchShell, /data-dx-component="launch-hero"/);
  assert.match(launchShell, /data-dx-component="dx-check-health-panel"/);
  assert.match(launchShell, /data-dx-check-panel="latest-receipt"/);
  assert.match(launchShell, /type DxCheckPanelViewModel = \{/);
  assert.match(launchShell, /const dxCheckPanelMissingViewModel: DxCheckPanelViewModel = \{/);
  assert.match(launchShell, /function DxCheckHealthPanel\(\{\s*viewModel = dxCheckPanelMissingViewModel/s);
  assert.match(launchShell, /data-dx-check-view-model-schema=\{viewModel\.schema_version\}/);
  assert.match(launchShell, /data-dx-check-view-model-status=\{viewModel\.status\}/);
  assert.match(launchShell, /data-dx-check-score-state=\{viewModel\.score_meter \? "available" : "missing"\}/);
  assert.match(launchShell, /viewModel\.bucket_rows\.map/);
  assert.match(launchShell, /viewModel\.blocker_rows\.map/);
  assert.match(launchShell, /viewModel\.warning_rows\.map/);
  assert.match(launchShell, /viewModel\.quick_fix_rows\.map/);
  assert.match(launchShell, /risk_level: string/);
  assert.match(launchShell, /requires_user_approval: boolean/);
  assert.match(launchShell, /writes_receipts: boolean/);
  assert.match(launchShell, /data-dx-check-quick-fix-risk=\{quickFix\.risk_level\}/);
  assert.match(
    launchShell,
    /data-dx-check-quick-fix-approval=\{\s*quickFix\.requires_user_approval \? "required" : "not-required"\s*\}/s,
  );
  assert.match(
    launchShell,
    /data-dx-check-quick-fix-writes-receipts=\{\s*quickFix\.writes_receipts \? "true" : "false"\s*\}/s,
  );
  assert.match(launchShell, /viewModel\.empty_state/);
  assert.match(launchShell, /data-dx-check-command=\{dxCheckPanelContract\.command\}/);
  assert.match(launchShell, /data-dx-check-receipt-path=\{dxCheckPanelContract\.receiptPath\}/);
  assert.match(launchShell, /data-dx-check-schema=\{dxCheckPanelContract\.schema\}/);
  assert.match(launchShell, /data-dx-zed-panel-schema=\{dxCheckPanelContract\.zedPanelSchema\}/);
  assert.match(launchShell, /data-dx-check-score-max=\{dxCheckPanelContract\.scoreMax\}/);
  assert.match(launchShell, /data-dx-check-expensive-default="skipped"/);
  assert.match(runtimeLaunch, /data-dx-component="dx-check-health-panel"/);
  assert.match(runtimeLaunch, /data-dx-check-panel="latest-receipt"/);
  assert.match(runtimeLaunch, /data-dx-check-view-model-schema="dx\.www\.check_panel_view_model"/);
  assert.match(runtimeLaunch, /data-dx-check-view-model-status="missing"/);
  assert.match(runtimeLaunch, /data-dx-check-score-state="missing"/);
  assert.match(runtimeLaunch, /data-dx-check-empty-state="missing"/);
  assert.match(runtimeLaunch, /data-dx-check-command="dx check --latest-receipt --json"/);
  assert.match(runtimeLaunch, /data-dx-check-receipt-path="\.dx\/receipts\/check\/check-latest\.json"/);
  assert.match(runtimeLaunch, /data-dx-component="forge-safety-archive-status"/);
  assert.match(
    runtimeLaunch,
    /data-dx-safety-archive-contract="dx\.forge\.safety_archive_contract"/,
  );
  assert.match(runtimeLaunch, /data-dx-safety-archive-state="covered"/);
  assert.match(runtimeLaunch, /data-dx-safety-archive-safe-delete="true"/);
  assert.match(runtimeLaunch, /data-dx-safety-archive-rollback-coverage="100"/);
  assert.match(runtimeLaunch, /data-dx-safety-archive-receipt-count="3"/);
  assert.match(
    runtimeLaunch,
    /data-dx-safety-archive-boundary="local-cache-restore-inputs-no-remote-rollback"/,
  );
  assert.match(materializer, /"launch-runtime-dx-check-panel"/);
  assert.match(materializer, /\[data-dx-component="dx-check-health-panel"\]/);
  assert.match(materializer, /"data-dx-check-view-model-schema"/);
  assert.match(materializer, /"data-dx-check-view-model-status"/);
  assert.match(materializer, /receiptPath: "\.dx\/receipts\/check\/check-latest\.json"/);
  assert.match(launchShell, /data-dx-editable="text"/);
  assert.match(launchShell, /data-dx-content-key="launch\.studio-proof-flow"/);
  assert.match(launchShell, /data-dx-insert-slot="proof-card-grid"/);
  assert.match(launchShell, /LaunchAutomationBridgeStatus/);
  assert.match(launchShell, /data-dx-section="launch-automation-ops"/);
  assert.match(launchShell, /data-dx-component="launch-automation-dashboard-workflow"/);
  assert.match(launchShell, /data-dx-dashboard-workflow="automation-release-receipt"/);
  assert.match(launchShell, /data-dx-package=\{item\.packageId\}/);
  assert.match(launchShell, /data-dx-package-role=\{item\.role\}/);
  assert.match(automationsStatus, /automationSummary/);
  assert.match(automationsStatus, /data-dx-automation-view="launch-bridge"/);
  assert.match(automationsStatus, /data-dx-package="automations\/n8n"/);
  assert.match(editContract, /schema: "dx\.studio\.launch_edit_contract"/);
  assert.match(editContract, /operation: "insert_component"/);
  assert.match(editContract, /operation: "move_reorder_section"/);
  assert.match(editContract, /operation: "update_design_token"/);
  assert.match(editContract, /operation: "update_text_content"/);
  assert.match(editContract, /operation: "insert_icon_media"/);
  assert.match(editContract, /layoutPolicy: "responsive-design-system-grid"/);
  assert.match(editContract, /absolutePositioning: false/);
  assert.match(editContract, /selector: '\[data-dx-section="hero"\]'/);
  assert.match(editContract, /selector: '\[data-dx-component="launch-connected-capability-map"\]'/);
  assert.match(editContract, /selector: '\[data-dx-section="package-catalog"\]'/);
  assert.match(editContract, /id: "dx-check-health-panel"/);
  assert.match(editContract, /selector: '\[data-dx-component="dx-check-health-panel"\]'/);
  assert.match(editContract, /"data-dx-check-view-model-schema"/);
  assert.match(editContract, /"data-dx-check-view-model-status"/);
  assert.match(editContract, /receiptPath: "\.dx\/receipts\/check\/check-latest\.json"/);
  assert.match(editContract, /const forgeSafetyArchiveStateMarkers = \[/);
  assert.match(editContract, /id: "forge-safety-archive-status"/);
  assert.match(editContract, /selector: '\[data-dx-component="forge-safety-archive-status"\]'/);
  assert.match(editContract, /sourceFile: "tools\/launch\/runtime-template\/pages\/index\.html"/);
  assert.match(editContract, /materializedFile: "pages\/index\.html"/);
  assert.match(editContract, /stateMarkers: forgeSafetyArchiveStateMarkers/);
  assert.match(editContract, /receiptPath: "\.dx\/forge\/receipts\/safety"/);
  assert.match(editContract, /"data-dx-safety-archive-contract"/);
  assert.match(editContract, /"data-dx-safety-archive-rollback-coverage"/);
  assert.match(editContract, /"data-dx-safety-archive-boundary"/);
  assert.match(studio, /studio_forge_safety_archive_edit_surface\(\)/);
  assert.match(studio, /studio_marker\(\s*"data-dx-safety-archive-contract"/);
  assert.match(studio, /studio_marker\(\s*"data-dx-safety-archive-rollback-coverage"/);
  assert.match(studio, /"forge_safety_archive_contract_marker": "data-dx-safety-archive-contract"/);
  assert.match(studio, /"forge_safety_archive_rollback_coverage_marker": "data-dx-safety-archive-rollback-coverage"/);
  assert.match(editContract, /id: "motion-interaction-proof"/);
  assert.match(editContract, /const motionDashboardInteractionSelectors = \[/);
  assert.match(editContract, /const motionDashboardStateMarkers = \[/);
  assert.match(
    editContract,
    /const motionDashboardReceiptPath =\s*"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-animation-motion-dashboard-workflow\.json"/,
  );
  assert.match(editContract, /interactionSelectors: motionDashboardInteractionSelectors/);
  assert.match(editContract, /stateMarkers: motionDashboardStateMarkers/);
  assert.match(
    editContract,
    /stateMarkers: \[\s*\.\.\.motionDashboardStateMarkers,\s*"data-dx-motion-policy-status",\s*\]/s,
  );
  assert.match(editContract, /data-dx-motion-interaction="toggle-reduced-motion"/);
  assert.match(editContract, /receiptPath: motionDashboardReceiptPath/);
  assert.match(studio, /studio_motion_edit_surface\(\s*"motion-dashboard-workflow"/);
  assert.match(studio, /studio_motion_edit_surface\(\s*"motion-interaction-proof"/);
  assert.match(
    studio,
    /fn studio_motion_edit_surface\([\s\S]*include_policy_status_marker: bool/s,
  );
  assert.match(studio, /let state_markers = if include_policy_status_marker/);
  assert.match(
    studio,
    /studio_motion_edit_surface\(\s*"motion-dashboard-workflow"[\s\S]*"motion-panel-orchestration",\s*false,\s*\)/,
  );
  assert.match(
    studio,
    /studio_motion_edit_surface\(\s*"motion-interaction-proof"[\s\S]*"motion-panel-orchestration",\s*true,\s*\)/,
  );
  assert.match(
    studio,
    /"receipt_path"\.to_string\(\)[\s\S]*2026-05-22-animation-motion-dashboard-workflow\.json/,
  );
  assert.match(editContract, /noNodeModulesRequired: true/);
  for (const surface of [
    {
      id: "dx-check-health-panel",
      selector: '[data-dx-component="dx-check-health-panel"]',
      sourceFile: "examples/template/template-shell.tsx",
    },
    {
      id: "forge-safety-archive-status",
      selector: '[data-dx-component="forge-safety-archive-status"]',
      sourceFile: "tools/launch/runtime-template/pages/index.html",
    },
    {
      id: "better-auth-account-dashboard-workflow",
      selector: '[data-dx-component="better-auth-account-dashboard-workflow"]',
      sourceFile: "examples/template/template-shell.tsx",
    },
    {
      id: "better-auth-session-status-panel",
      selector: '[data-dx-component="better-auth-session-status-panel"]',
      sourceFile: "examples/template/auth-session-status.tsx",
    },
    {
      id: "scene-rendering-proof",
      selector: '[data-dx-media-slot="launch-scene"]',
      sourceFile: "examples/template/template-shell.tsx",
    },
    {
      id: "form-validation-proof",
      selector: '[data-dx-component="form-zod-proof"]',
      sourceFile: "examples/template/template-shell.tsx",
    },
    {
      id: "launch-billing-checkout-workflow",
      selector: '[data-dx-component="launch-billing-checkout-workflow"]',
      sourceFile: "examples/template/payments-status.tsx",
    },
    {
      id: "motion-dashboard-workflow",
      selector: '[data-dx-component="launch-motion-dashboard-workflow"]',
      sourceFile: "examples/template/template-shell.tsx",
    },
    {
      id: "motion-interaction-proof",
      selector: '[data-dx-component="motion-interaction-proof"]',
      sourceFile: "examples/template/motion-interaction-proof.tsx",
    },
    {
      id: "wasm-compute-dashboard-workflow",
      selector: '[data-dx-component="launch-wasm-compute-dashboard-workflow"]',
      sourceFile: "examples/template/template-shell.tsx",
    },
    {
      id: "automation-dashboard-workflow",
      selector: '[data-dx-component="launch-automation-dashboard-workflow"]',
      sourceFile: "examples/template/template-shell.tsx",
    },
    {
      id: "database-backend-proof",
      selector: '[data-dx-component="database-backend-proof"]',
      sourceFile: "examples/template/template-shell.tsx",
    },
    {
      id: "supabase-schema-query-workflow",
      selector: '[data-dx-component="supabase-schema-query-workflow"]',
      sourceFile: "examples/template/data-status.tsx",
    },
    {
      id: "launch-drizzle-data-workflow",
      selector: '[data-dx-component="launch-drizzle-data-workflow"]',
      sourceFile: "examples/template/drizzle-query-proof.tsx",
    },
    {
      id: "launch-trpc-api-dashboard-workflow",
      selector: '[data-dx-component="launch-trpc-api-dashboard-workflow"]',
      sourceFile: "examples/template/template-shell.tsx",
    },
    {
      id: "launch-dashboard-state-workflow",
      selector: '[data-dx-component="launch-dashboard-state-workflow"]',
      sourceFile: "examples/template/state-zustand-dashboard.tsx",
    },
    {
      id: "launch-dashboard-state-shell",
      selector: '[data-dx-component="launch-dashboard-state-shell"]',
      sourceFile: "examples/template/template-shell.tsx",
    },
    {
      id: "tanstack-query-dashboard-data",
      selector: '[data-dx-component="tanstack-query-dashboard-data-workflow"]',
      sourceFile: "examples/template/query-cache-status.tsx",
    },
    {
      id: "trpc-launch-health-workflow",
      selector: '[data-dx-component="trpc-launch-health-workflow"]',
      sourceFile: "examples/template/trpc-launch-health.tsx",
    },
    {
      id: "docs-help-changelog-workflow",
      selector: '[data-dx-component="launch-fumadocs-docs-workflow"]',
      sourceFile: "examples/template/docs-status.tsx",
    },
    {
      id: "package-catalog",
      selector: '[data-dx-section="package-catalog"]',
      sourceFile: "examples/template/template-shell.tsx",
    },
  ]) {
    assertEditableSurfaceSourceOwnsSelector(editContract, surface);
  }
  assert.ok(
    !fs.existsSync(wwwTemplateNodeModules),
    "launch template must not contain a local node_modules folder",
  );
  assert.match(cli, /"routes" => \{\s*cli\.cmd_routes\(&args\[2\.\.\]\)\?/s);
  assert.match(cli, /"preview-manifest" \| "studio-manifest"/);
  assert.match(cli, /fn cmd_www\(&self, args: &\[String\]\) -> DxResult<\(\)>/);
});

test("converted and automation routes publish data-dx markers for preview selection", () => {
  const automationsShell = read("examples/template/automations/automations-shell.tsx");
  const uiRoute = read("examples/conversion-proof/pages/ui.html");
  const databaseRoute = read("examples/conversion-proof/pages/database.html");
  const backendRoute = read("examples/conversion-proof/pages/backend.html");

  assert.match(automationsShell, /data-dx-route="\/automations"/);
  assert.match(
    automationsShell,
    /data-dx-source="examples\/template\/automations\/automations-shell\.tsx"/,
  );
  assert.match(automationsShell, /data-dx-forge="automations\/n8n"/);
  assert.match(automationsShell, /data-dx-package="automations\/n8n"/);
  assert.match(automationsShell, /data-dx-hot-reload-target="route:\/automations"/);
  assert.match(automationsShell, /data-dx-node-modules="forbidden"/);
  assert.match(automationsShell, /data-dx-automation-view=\{view\}/);
  assert.match(automationsShell, /data-dx-automation-route=\{route\.href\}/);
  assert.match(uiRoute, /data-dx-route="\/ui"/);
  assert.match(uiRoute, /data-dx-source="shadcn-ui"/);
  assert.match(uiRoute, /data-dx-forge="website-conversion-shadcn"/);
  assert.match(uiRoute, /data-dx-package="shadcn\/ui\/card"/);
  assert.match(uiRoute, /data-dx-hot-reload-target="route:\/ui"/);
  assert.match(databaseRoute, /data-dx-route="\/database"/);
  assert.match(databaseRoute, /data-dx-source="supabase"/);
  assert.match(databaseRoute, /data-dx-forge="website-conversion-supabase"/);
  assert.match(databaseRoute, /data-dx-package="supabase\/client"/);
  assert.match(databaseRoute, /data-dx-hot-reload-target="route:\/database"/);
  assert.match(backendRoute, /data-dx-route="\/backend"/);
  assert.match(backendRoute, /data-dx-source="convex-backend"/);
  assert.match(backendRoute, /data-dx-forge="website-conversion-convex"/);
  assert.match(backendRoute, /data-dx-package="supabase\/client"/);
  assert.match(backendRoute, /data-dx-hot-reload-target="route:\/backend"/);
});
