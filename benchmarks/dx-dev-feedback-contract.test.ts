import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

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

test("DX dev feedback replaces the old DevTools clone target while preserving source-owned feedback", () => {
  const devMod = read("dx-www/src/dev/mod.rs");
  const lib = read("dx-www/src/lib.rs");
  const axumServer = read("dx-www/src/dev/axum_server.rs");
  const devResponse = read("dx-www/src/cli/dev_response.rs");
  const feedback = read("dx-www/src/dev/dev_feedback.rs");

  assert.ok(
    !fs.existsSync(path.join(repoRoot, "dx-www/src/dev/devtools.rs")),
    "the active dev feedback module must not keep the DevTools clone target name",
  );

  assert.match(lib, /#\[path = "dev\/dev_feedback_diagnostics\.rs"\]\s*mod dev_feedback_diagnostics;/);
  assert.match(lib, /#\[path = "dev\/dev_feedback\.rs"\]\s*pub\(crate\) mod dev_feedback;/);
  assert.doesNotMatch(devMod, /mod dev_feedback;/);
  assert.match(axumServer, /use crate::dev_feedback::dev_feedback_response;/);
  assert.match(devResponse, /use crate::dev_feedback::dev_feedback_response;/);
  assert.match(devResponse, /fn dev_feedback_cli_response/);
  assert.match(devResponse, /dev_feedback_response\(/);
  assert.match(axumServer, /return response_from_dev_feedback\(response\)/);

  for (const endpoint of [
    "/_dx/feedback",
    "/_dx/feedback/events",
    "/_dx/feedback/errors",
    "/_dx/feedback/routes",
    "/_dx/feedback/hmr",
    "/_dx/feedback/receipts",
    "/_dx/feedback/dx-check",
    "/_dx/feedback/source-frame",
    "/_dx/feedback/open-in-editor",
  ]) {
    assert.match(feedback, new RegExp(endpoint.replaceAll("/", "\\/")));
  }

  assert.match(feedback, /pub\(super\) fn dev_feedback_response/);
  assert.match(feedback, /fn dev_feedback_html/);
  assert.match(feedback, /fn route_graph_snapshot/);
  assert.match(feedback, /fn app_route_roots\(project_root: &Path\) -> Vec<PathBuf>/);
  assert.match(feedback, /fn pages_route_roots\(project_root: &Path\) -> Vec<PathBuf>/);
  assert.match(feedback, /fn collect_pages_route_files_from_root/);
  assert.match(feedback, /fn route_has_dynamic_params\(route: &str\) -> bool/);
  assert.match(feedback, /fn route_params\(route: &str\) -> Vec<Value>/);
  assert.match(feedback, /fn route_param\(segment: &str\) -> Option<Value>/);
  assert.match(feedback, /fn hmr_snapshot/);
  assert.match(feedback, /fn hmr_watched_roots\(project_root: &Path\) -> Vec<String>/);
  assert.match(feedback, /fn errors_snapshot/);
  assert.match(feedback, /fn receipts_snapshot/);
  assert.match(feedback, /fn dx_check_snapshot/);
  assert.match(feedback, /fn source_frame_snapshot/);
  assert.match(feedback, /fn collect_receipt_files/);
  assert.match(feedback, /fn diagnostic_issues_with_code_frames/);
  assert.match(feedback, /fn dev_feedback_code_frame_for_issue/);
  assert.match(feedback, /DxDiagnostic::error/);
  assert.match(feedback, /code_frame_with_options/);
  assert.match(feedback, /fn dev_feedback_event_stream/);
  assert.match(feedback, /fn open_in_editor_snapshot/);
  assert.match(feedback, /data-dx-dev-feedback-overlay/);
  assert.match(feedback, /new EventSource\("\{DX_DEV_FEEDBACK_EVENTS_ENDPOINT\}"\)/);
  assert.match(feedback, /new EventSource\("\{DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT\}"/);
  assert.match(feedback, /addEventListener\("\{DX_HOT_RELOAD_EVENT_NAME\}"/);
  assert.match(feedback, /"schema": "dx\.dev_feedback\.routes"/);
  assert.match(feedback, /"dynamic": route_has_dynamic_params\(route\.route\.as_str\(\)\)/);
  assert.match(feedback, /"params": route_params\(route\.route\.as_str\(\)\)/);
  assert.match(feedback, /"methods": route\.methods\.as_slice\(\)/);
  assert.match(feedback, /"metadata": route\.metadata\.to_json\(\)/);
  assert.match(feedback, /fn route_handler_methods\(source: &str\) -> Vec<String>/);
  assert.match(feedback, /fn route_metadata_signals\(source: &str\) -> DxDevFeedbackRouteMetadata/);
  assert.match(feedback, /"route_groups": route\.route_groups\.as_slice\(\)/);
  assert.match(feedback, /"parallel_slots": route\.parallel_slots\.as_slice\(\)/);
  assert.match(feedback, /fn app_route_context\(relative_parent: &Path\) -> AppRouteContext/);
  assert.match(feedback, /struct AppRouteContext/);
  assert.match(feedback, /optional-catch-all/);
  assert.match(feedback, /route_graph_exposes_app_router_params_without_next_runtime/);
  assert.match(feedback, /route_graph_exposes_route_group_and_parallel_slot_context_without_next_runtime/);
  assert.match(feedback, /"schema": "dx\.dev_feedback\.hmr"/);
  assert.match(feedback, /"watched_roots": hmr_watched_roots\(project_root\)/);
  assert.match(feedback, /"refresh_capabilities": \{/);
  assert.match(feedback, /"css_stylesheet_refresh": hot_reload_enabled/);
  assert.match(feedback, /"route_refresh": hot_reload_enabled/);
  assert.match(feedback, /hmr_snapshot_reports_observed_source_roots_without_turbopack/);
  assert.match(feedback, /"schema": "dx\.dev_feedback\.errors"/);
  assert.match(feedback, /fn diagnostic_issue_severity_counts/);
  assert.match(feedback, /fn diagnostic_issue_highest_severity/);
  assert.match(feedback, /fn diagnostic_issue_next_action/);
  assert.match(feedback, /"severity_counts": diagnostic_issue_severity_counts\(issues\.as_slice\(\)\)/);
  assert.match(feedback, /let next_action = diagnostic_issue_next_action\(highest_severity\.as_deref\(\), issues\.as_slice\(\)\)/);
  assert.match(feedback, /"highest_severity": highest_severity\.clone\(\)/);
  assert.match(feedback, /"next_action": next_action/);
  assert.match(feedback, /"severity_counts": errors\["severity_counts"\]\.clone\(\)/);
  assert.match(feedback, /"highest_severity": errors\["highest_severity"\]\.clone\(\)/);
  assert.match(feedback, /"next_action": errors\["next_action"\]\.clone\(\)/);
  assert.match(feedback, /severityCounts: data\?\.severity_counts/);
  assert.match(feedback, /nextAction: data\?\.next_action/);
  assert.match(feedback, /if \(payload\?\.errors\) render\("\[data-dx-dev-feedback-errors\]", payload\.errors\)/);
  assert.match(feedback, /errors_snapshot_summarizes_severity_and_next_action/);
  assert.match(feedback, /"schema": "dx\.dev_feedback\.receipts"/);
  assert.match(feedback, /"schema": "dx\.dev_feedback\.dx_check"/);
  assert.match(feedback, /"schema": "dx\.dev_feedback\.source_frame"/);
  assert.match(feedback, /"schema": "dx\.dev_feedback\.open_in_editor"/);
  assert.match(feedback, /"code_frame_source"/);
  assert.match(feedback, /"unsafe-or-missing-source-location"/);
  assert.match(feedback, /"format": 1/);
  assert.match(feedback, /"turbopack_hmr": false/);
  assert.match(feedback, /"next_runtime": false/);
  assert.match(feedback, /"node_modules_required": false/);
  assert.match(feedback, /"editor_adapter_boundary"/);
  assert.match(feedback, /WalkDir::new/);
  assert.match(feedback, /project_root\.join\("src"\)\.join\("app"\)/);
  assert.match(feedback, /project_root\.join\("src"\)\.join\("pages"\)/);
  assert.match(feedback, /route_graph_discovers_src_app_routes/);
  assert.match(feedback, /route_graph_discovers_src_pages_routes_without_node_modules/);
  assert.match(feedback, /route_graph_reports_source_detected_route_handler_methods_without_next_runtime/);
  assert.match(feedback, /route_graph_reports_app_metadata_exports_without_next_runtime/);

  assert.doesNotMatch(feedback, /Next DevTools|DX-WWW DevTools|next-devtools|dev-overlay/);
  assert.doesNotMatch(feedback, /DX_DEVTOOLS|devtools_response|dx\.devtools|\/_dx\/devtools/);
  assert.doesNotMatch(feedback, /data-dx-devtools|x-dx-devtools|event: dx-devtools/);
  assert.doesNotMatch(feedback, /project_root\.join\("node_modules"\)|_next\/static|_next\/hmr/);
  assert.doesNotMatch(feedback, /turbopack-subscribe|turbopack-hmr|Turbopack powers/);
  assert.doesNotMatch(feedback, /=\s*"[^"]*\.v1"/);
});
