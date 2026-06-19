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

test("DX hot reload publishes diagnostics changes as issue and recovery frames", () => {
  const protocol = read("dx-www/src/hot_reload_protocol.rs");
  const watcher = read("dx-www/src/dev/watcher.rs");
  const devMod = read("dx-www/src/dev/mod.rs");
  const stream = read("dx-www/src/dev/hot_reload_stream.rs");
  const diagnosticSnapshot = read("dx-www/src/dev/diagnostic_snapshot.rs");
  const axum = read("dx-www/src/dev/axum_server.rs");

  assert.match(watcher, /"\.dx\/diagnostics"/);
  assert.match(watcher, /parts\[1\] == "diagnostics"/);

  assert.match(stream, /DxHotReloadIssue/);
  assert.match(stream, /broadcast::Sender<Value>/);
  assert.match(stream, /fn payload_with_subscription/);
  assert.match(stream, /let requested_resource_for_events = requested_resource\.clone\(\);/);
  assert.match(stream, /dx_hot_reload_sse_frame\(&payload_with_subscription/);
  assert.match(stream, /current_resource_for\(version, requested_resource\.clone\(\)\)/);
  assert.match(stream, /"requested_resource"\.to_string\(\)[\s\S]*json!\(requested_resource\)/);
  assert.match(stream, /"resource_scoped"\.to_string\(\)[\s\S]*json!\(true\)/);
  assert.match(stream, /dx_hot_reload_issue_payload/);
  assert.match(stream, /dx_hot_reload_issue_recovery_payload/);
  assert.match(stream, /pub\(super\) fn publish_issues/);
  assert.match(stream, /pub\(super\) fn publish_diagnostics_for_changed_paths/);
  assert.match(devMod, /mod diagnostic_snapshot;/);
  assert.match(stream, /use super::diagnostic_snapshot::\{[\s\S]*diagnostic_snapshot_from_json_str/);
  assert.match(
    stream,
    /diagnostic_snapshot_from_json_str\(&content, route_resource_for_changed_relative_path\)/,
  );
  assert.match(stream, /latest_diagnostic_snapshot: Arc<Mutex<Option<DxHotReloadDiagnosticSnapshot>>>/);
  assert.match(stream, /fn latest_diagnostic_snapshot\(&self\) -> Option<DxHotReloadDiagnosticSnapshot>/);
  assert.match(stream, /fn latest_diagnostic_resource\(&self\) -> Option<String>/);
  assert.match(
    stream,
    /dx_hot_reload_issue_payload\(\s*self\.enabled,\s*token,\s*version,\s*&snapshot\.resource,\s*&snapshot\.issues,\s*\)/,
  );
  assert.match(
    stream,
    /snapshot\.issues\.is_empty\(\) && snapshot\.resource == DX_HOT_RELOAD_DEFAULT_RESOURCE/,
  );
  assert.doesNotMatch(stream, /fn diagnostic_snapshot_from_value/);
  assert.doesNotMatch(stream, /fn diagnostic_issue_from_value/);
  assert.doesNotMatch(stream, /fn diagnostic_string_from_paths/);
  assert.match(diagnosticSnapshot, /pub\(super\) struct DxHotReloadDiagnosticSnapshot/);
  assert.match(diagnosticSnapshot, /pub\(super\) fn diagnostic_snapshot_from_json_str/);
  assert.match(diagnosticSnapshot, /fn diagnostic_snapshot_from_value/);
  assert.match(diagnosticSnapshot, /fn diagnostic_snapshot_schema_issue/);
  assert.match(diagnosticSnapshot, /fn diagnostic_issue_from_value/);
  assert.match(diagnosticSnapshot, /fn diagnostic_resource_from_snapshot_value/);
  assert.match(diagnosticSnapshot, /fn diagnostic_resource_from_issue_value/);
  assert.match(diagnosticSnapshot, /fn diagnostic_string_from_paths/);
  assert.match(diagnosticSnapshot, /fn diagnostic_u64_from_paths/);
  assert.match(diagnosticSnapshot, /fn diagnostic_code_frame_from_value/);
  assert.match(diagnosticSnapshot, /DxHotReloadIssue::new/);
  assert.match(stream, /DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA/);
  assert.match(stream, /hub_event_stream_emits_diagnostic_issue_frames_without_axum_server/);
  assert.match(stream, /polling_fallback_version_payload_reports_latest_diagnostic_issue/);
  assert.match(stream, /diagnostics_nested_source_locations_publish_issue_frames/);
  assert.match(stream, /diagnostics_without_issues_publish_recovery_frame/);
  assert.match(stream, /diagnostics_recovery_uses_latest_issue_resource_when_snapshot_is_empty/);
  assert.match(stream, /diagnostics_malformed_snapshot_shape_reports_issue_instead_of_clearing_overlay/);
  assert.match(diagnosticSnapshot, /dx\.dev\.diagnostics\.invalid_snapshot/);
  assert.match(diagnosticSnapshot, /dx\.dev\.diagnostics\.invalid_issues/);
  assert.match(diagnosticSnapshot, /value\.as_object\(\)/);
  assert.match(diagnosticSnapshot, /\.get\("issues"\)/);
  assert.match(stream, /contains\("\\"type\\":\\"clear-issue\\""\)/);
  assert.match(stream, /contains\("\\"id\\":\\"route:\/dashboard\\""\)/);
  assert.match(diagnosticSnapshot, /&\["source", "path"\]/);
  assert.match(diagnosticSnapshot, /&\["location", "line"\]/);
  assert.match(diagnosticSnapshot, /&\["span", "start", "line"\]/);
  assert.match(diagnosticSnapshot, /&\["codeFrame", "rendered"\]/);
  assert.match(diagnosticSnapshot, /diagnostic_resource_from_snapshot_value\(value\)/);
  assert.match(
    diagnosticSnapshot,
    /diagnostic_resource_from_issue_value\(\s*issue,\s*route_resource_for_changed_relative_path,\s*\)/,
  );
  assert.match(stream, /return self\.publish_issue_recovery\(resource\);/);
  assert.doesNotMatch(stream, /return self\.publish\(resource\);/);
  assert.match(protocol, /pub\(crate\) fn dx_hot_reload_normalize_resource_id/);

  assert.match(
    axum,
    /hot_reload[\s\S]*\.publish_diagnostics_for_changed_paths\(&change\.paths\)[\s\S]*\.is_some\(\)/,
  );
  assert.match(
    axum,
    /publish_diagnostics_for_changed_paths\(&change\.paths\)[\s\S]*continue;/,
  );

  assert.doesNotMatch(
    stream + axum,
    /project_root\.join\("node_modules"\)|_next\/hmr|turbopack-subscribe|turbopack-hmr/,
  );
  assert.doesNotMatch(
    diagnosticSnapshot,
    /project_root\.join\("node_modules"\)|_next\/hmr|turbopack-subscribe|turbopack-hmr/,
  );
});
