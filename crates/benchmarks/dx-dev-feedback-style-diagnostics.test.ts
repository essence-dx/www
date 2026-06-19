import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("dx-style unsupported class diagnostics stay style-specific and actionable", () => {
  const errorSource = read("dx-www/src/error.rs");
  const appRouteDiagnostics = read("dx-www/src/cli/app_route_diagnostics.rs");

  assert.match(
    appRouteDiagnostics,
    /"dx-style unsupported class `\{\}`: \{\}"[\s\S]*unsupported\.class_name[\s\S]*unsupported\.reason/,
    "app route diagnostics should preserve the unsupported class and reason in one parseable message",
  );
  assert.match(
    errorSource,
    /const DX_STYLE_UNSUPPORTED_CLASS_PREFIX: &str = "dx-style unsupported class `";/,
    "error rendering needs a stable dx-style unsupported-class prefix",
  );
  assert.match(
    errorSource,
    /fn dx_style_unsupported_class_name\(message: &str\) -> Option<&str>/,
    "error rendering should parse the unsupported class out of the diagnostic message",
  );
  assert.match(
    errorSource,
    /fn compilation_diagnostic_metadata\(message: &str\) -> CompilationDiagnosticMetadata/,
    "compilation errors should be classified before terminal and overlay rendering",
  );
  assert.match(errorSource, /title: "Unsupported dx-style class"/);
  assert.match(errorSource, /code: "dx\.style\.unsupported_class"/);
  assert.match(
    errorSource,
    /Use a supported dx-style utility, add engine support for `\{class_name\}`, or move this styling into authored CSS\./,
    "unsupported class diagnostics should tell the developer what to do next",
  );
  assert.match(
    errorSource,
    /DxDiagnostic::error\(metadata\.title, message\.clone\(\)\)[\s\S]*\.with_code\(metadata\.code\)[\s\S]*\.with_hint\(metadata\.hint\)/,
    "diagnostic metadata should feed both the stable code and next action",
  );
});

test("dx-style receipt diagnostics keep class and remediation through dev feedback", () => {
  const devFeedback = read("dx-www/src/dev/dev_feedback.rs");
  const errorOverlay = read("dx-www/src/dev/error_overlay.rs");
  const diagnosticSnapshot = read("dx-www/src/dev/diagnostic_snapshot.rs");

  assert.match(
    devFeedback,
    /fn dx_style_issue_class_name\(issue: &Value\) -> Option<String>/,
    "dev feedback should recognize class_name-bearing dx-style receipts",
  );
  assert.match(
    devFeedback,
    /fn dx_style_issue_message\(issue: &Value\) -> Option<String>[\s\S]*"dx-style unsupported class `\{\}`: \{\}"/,
    "dev feedback should reconstruct the actionable class-specific message when receipts split class_name and reason",
  );
  assert.match(
    devFeedback,
    /fn diagnostic_issue_suggestions\(issue: &Value\) -> Value[\s\S]*"remediation"/,
    "receipt remediation should be promoted into overlay suggestions",
  );
  assert.match(
    devFeedback,
    /entry\("diagnostic_code"[\s\S]*diagnostic_issue_code/,
    "dev feedback should preserve rule-derived diagnostic codes for overlay titles",
  );
  assert.match(
    devFeedback,
    /entry\("next_action"[\s\S]*diagnostic_issue_next_action_value/,
    "dev feedback should preserve remediation as the next action when explicit next_action is missing",
  );

  assert.match(
    errorOverlay,
    /const diagnosticCode = firstPayloadText\(issue, \['diagnostic_code', 'code', 'rule'\]\);/,
    "direct overlay issue rendering should accept rule as a diagnostic code",
  );
  assert.match(
    errorOverlay,
    /\['remediation'\]/,
    "overlay suggestion and next-action extraction should include remediation fields",
  );
  assert.match(
    errorOverlay,
    /message: issueMessageText\(issue\)/,
    "direct overlay issue rendering should use class-aware message normalization",
  );

  assert.match(
    diagnosticSnapshot,
    /fn diagnostic_issue_message_from_value\(value: &Value\) -> String/,
    "hot reload diagnostics should normalize receipt-shaped style messages before publishing",
  );
  assert.match(
    diagnosticSnapshot,
    /&\["diagnostic_code"\][\s\S]*&\["code"\][\s\S]*&\["rule"\]/,
    "hot reload diagnostic codes should preserve diagnostic_code/code/rule precedence",
  );
  assert.match(
    diagnosticSnapshot,
    /"dx-style unsupported class `\{\}`: \{\}"/,
    "hot reload diagnostics should publish the unsupported class name in the message",
  );
});

test("diagnostic issue payloads preserve nested source locations and rendered frames", () => {
  const errorOverlay = read("dx-www/src/dev/error_overlay.rs");
  const diagnosticSnapshot = read("dx-www/src/dev/diagnostic_snapshot.rs");
  const devFeedback = read("dx-www/src/dev/dev_feedback.rs");

  assert.match(
    errorOverlay,
    /const filePath = firstPayloadTextAtPath\(issue, \[[\s\S]*?\['source_location', 'path'\][\s\S]*?\]\);/,
    "direct overlay issue rendering should preserve source_location.path",
  );
  assert.match(
    errorOverlay,
    /const line = firstPayloadIntegerAtPath\(issue, \[[\s\S]*?\['source_location', 'line'\][\s\S]*?\]\);/,
    "direct overlay issue rendering should preserve source_location.line",
  );
  assert.match(
    errorOverlay,
    /const column = firstPayloadIntegerAtPath\(issue, \[[\s\S]*?\['source_location', 'column'\][\s\S]*?\]\);/,
    "direct overlay issue rendering should preserve source_location.column",
  );
  assert.match(
    errorOverlay,
    /const codeFrame = firstPayloadTextAtPath\(issue, \[[\s\S]*?\['diagnostic', 'code_frame'\][\s\S]*?\]\);/,
    "direct overlay issue rendering should preserve nested diagnostic code frames",
  );

  assert.match(
    diagnosticSnapshot,
    /fn diagnostic_file_from_issue_value\(value: &Value\) -> Option<String> \{[\s\S]*?&\["source_location", "path"\]/,
    "hot reload diagnostic snapshots should preserve source_location.path",
  );
  assert.match(
    diagnosticSnapshot,
    /let line = diagnostic_u64_from_paths\([\s\S]*?&\["source_location", "line"\]/,
    "hot reload diagnostic snapshots should preserve source_location.line",
  );
  assert.match(
    diagnosticSnapshot,
    /let column = diagnostic_u64_from_paths\([\s\S]*?&\["source_location", "column"\]/,
    "hot reload diagnostic snapshots should preserve source_location.column",
  );
  assert.match(
    diagnosticSnapshot,
    /fn diagnostic_code_frame_from_value\(value: &Value\) -> Option<&str> \{[\s\S]*?&\["diagnostic", "code_frame"\]/,
    "hot reload diagnostic snapshots should preserve nested diagnostic code frames",
  );

  assert.match(
    devFeedback,
    /fn diagnostic_issue_source_path\(issue: &Value\) -> Option<String> \{[\s\S]*?&\["source_location", "path"\]/,
    "dev feedback errors should preserve source_location.path for code frames and summaries",
  );
  assert.match(
    devFeedback,
    /fn diagnostic_issue_line\(issue: &Value\) -> Option<usize> \{[\s\S]*?&\["source_location", "line"\]/,
    "dev feedback errors should preserve source_location.line for code frames and summaries",
  );
  assert.match(
    devFeedback,
    /fn diagnostic_issue_column\(issue: &Value\) -> Option<usize> \{[\s\S]*?&\["source_location", "column"\]/,
    "dev feedback errors should preserve source_location.column for code frames and summaries",
  );
  assert.match(
    devFeedback,
    /fn diagnostic_issue_existing_code_frame\(issue: &Value\) -> Option<String> \{[\s\S]*?&\["diagnostic", "code_frame"\]/,
    "dev feedback errors should preserve nested diagnostic code frames",
  );
  assert.match(
    devFeedback,
    /fn dev_feedback_code_frame_for_issue\(project_root: &Path, issue: &Value\) -> Option<String> \{[\s\S]*?let file = diagnostic_issue_source_path\(issue\)\?;[\s\S]*?let line = diagnostic_issue_line\(issue\)\?;/,
    "dev feedback code-frame rendering should use normalized nested source locations",
  );
});

test("dev feedback reports missing or stale diagnostics artifacts without node_modules", () => {
  const devFeedback = read("dx-www/src/dev/dev_feedback.rs");
  const diagnosticsModule = read("dx-www/src/dev/dev_feedback_diagnostics.rs");

  assert.match(
    diagnosticsModule,
    /pub\(super\) struct DxDevFeedbackDiagnosticsArtifact/,
    "dev feedback should model diagnostics artifact freshness explicitly",
  );
  assert.match(
    diagnosticsModule,
    /pub\(super\) fn diagnostics_artifact_status\(\s*project_root: &Path,\s*diagnostics_path: &Path,\s*\) -> DxDevFeedbackDiagnosticsArtifact/,
    "errors snapshot should compute diagnostics artifact status from source and receipt mtimes",
  );
  assert.match(
    diagnosticsModule,
    /pub\(super\) fn diagnostic_artifact_issue\(\s*artifact: &DxDevFeedbackDiagnosticsArtifact,\s*\) -> Option<Value>/,
    "missing or stale diagnostics should become a visible DX diagnostic issue",
  );
  assert.match(
    diagnosticsModule,
    /"dx\.dev_feedback\.diagnostics_stale"/,
    "stale diagnostics should have a stable DX diagnostic code",
  );
  assert.match(
    diagnosticsModule,
    /"dx\.dev_feedback\.diagnostics_missing"/,
    "missing diagnostics should have a stable DX diagnostic code",
  );
  assert.match(
    devFeedback,
    /"diagnostics_artifact": diagnostics_artifact\.to_json\(\)/,
    "errors snapshot should expose artifact freshness metadata alongside issues",
  );
  assert.match(
    devFeedback,
    /fn errors_snapshot_preserves_style_receipt_and_stale_artifact_issue\(\)[\s\S]*"class_name":"text-muted-foreground"[\s\S]*"remediation":"Use a supported dx-style utility or add engine support\."[\s\S]*"diagnostics_artifact"\]\["status"\], "stale"[\s\S]*"dx\.dev_feedback\.diagnostics_stale"/,
    "a focused Rust fixture should prove style receipt payloads and stale diagnostics share the same errors snapshot",
  );
  assert.match(
    diagnosticsModule,
    /fn newest_source_artifact\(project_root: &Path\) -> Option<DxDevFeedbackSourceArtifact>/,
    "stale checks should compare diagnostics against source-owned project files",
  );
  assert.match(
    diagnosticsModule,
    /matches!\([\s\S]*part,[\s\S]*"\.git" \| "\.dx" \| "node_modules" \| "target" \| "dist" \| "build"[\s\S]*\)/,
    "diagnostics artifact scanning must not depend on node_modules or generated output trees",
  );
});

test("dev feedback diagnostics artifact helpers are isolated from endpoint assembly", () => {
  const devModule = read("dx-www/src/dev/mod.rs");
  const devFeedback = read("dx-www/src/dev/dev_feedback.rs");
  const diagnosticsModule = read("dx-www/src/dev/dev_feedback_diagnostics.rs");

  assert.match(
    devModule,
    /mod dev_feedback_diagnostics;/,
    "dev module should declare the focused diagnostics artifact helper module",
  );
  assert.match(
    devFeedback,
    /use super::dev_feedback_diagnostics::\{[\s\S]*diagnostic_artifact_issue[\s\S]*diagnostics_artifact_status[\s\S]*DX_DEV_FEEDBACK_CHECK_LATEST_PATH[\s\S]*DX_DEV_FEEDBACK_DIAGNOSTICS_LATEST_PATH[\s\S]*\};/,
    "dev feedback endpoint assembly should import diagnostics artifact helpers instead of owning them",
  );
  assert.doesNotMatch(
    devFeedback,
    /struct DxDevFeedbackSourceArtifact/,
    "source artifact mtime scanning should live outside the endpoint assembly module",
  );
  assert.doesNotMatch(
    devFeedback,
    /fn newest_source_artifact/,
    "source artifact freshness scanning should live outside the endpoint assembly module",
  );
  assert.match(
    diagnosticsModule,
    /fn diagnostics_relative_source_path\(project_root: &Path, path: &Path\) -> Option<String>/,
    "the diagnostics module should own the small path-normalization helper it needs",
  );
});

test("dev feedback errors snapshots publish an explicit overlay recovery contract", () => {
  const devFeedback = read("dx-www/src/dev/dev_feedback.rs");
  const errorOverlay = read("dx-www/src/dev/error_overlay.rs");

  assert.match(
    devFeedback,
    /fn diagnostic_issue_recovery_state\(\s*highest_severity: Option<&str>,\s*issue_count: usize,\s*diagnostics_artifact_status: &str,\s*\) -> Value/,
    "errors snapshot assembly should model overlay recovery state explicitly",
  );
  assert.match(
    devFeedback,
    /let recovery = diagnostic_issue_recovery_state\(\s*highest_severity\.as_deref\(\),\s*issues\.len\(\),\s*diagnostics_artifact\.status,\s*\);/,
    "errors snapshot should compute recovery after diagnostics and artifact issues are known",
  );
  assert.match(
    devFeedback,
    /"recovery": recovery,/,
    "errors endpoint payload should expose the recovery contract",
  );
  assert.match(
    devFeedback,
    /"recovery": errors\["recovery"\]\.clone\(\),/,
    "dev feedback SSE payload should forward recovery without requiring another fetch",
  );
  assert.match(
    devFeedback,
    /"status": "recovered"[\s\S]*"overlay_action": "clear-overlay"[\s\S]*"clears_overlay": true[\s\S]*"requires_full_reload": false/,
    "a current empty diagnostics snapshot should tell the overlay to clear without forcing a full reload",
  );
  assert.match(
    errorOverlay,
    /function feedbackSnapshotClearsOverlay\(snapshot\)[\s\S]*snapshot\.recovery\.clears_overlay === true/,
    "browser overlay should consume the explicit recovery clear contract",
  );
});

test("dev feedback next actions use source locations when a code frame is unavailable", () => {
  const devFeedback = read("dx-www/src/dev/dev_feedback.rs");

  assert.match(
    devFeedback,
    /fn diagnostic_issue_next_action_message\(\s*kind: &str,\s*first_issue: &Value\s*\) -> String/,
    "next-action message construction should be explicit enough to distinguish code-frame and location-only diagnostics",
  );
  assert.match(
    devFeedback,
    /"message": diagnostic_issue_next_action_message\("error", &first_issue\),/,
    "error next actions should be computed from the focused issue instead of using an unconditional code-frame message",
  );
  assert.match(
    devFeedback,
    /diagnostic_issue_location_label\(first_issue\)/,
    "next-action messages should reuse the normalized source path, line, and column from the focused issue",
  );
  assert.match(
    devFeedback,
    /format!\(\s*"Fix the first DX error at \{\}, then let hot reload recover\.",\s*location\s*\)/,
    "location-only errors should tell the developer exactly where to start",
  );
  assert.match(
    devFeedback,
    /"Fix the first DX error at app\/dashboard\/page\.tsx:4:12, then let hot reload recover\."/,
    "the Rust snapshot test should lock the location-only remediation text",
  );
});

test("dev feedback next actions preserve nested hint and remediation payloads", () => {
  const devFeedback = read("dx-www/src/dev/dev_feedback.rs");
  const errorOverlay = read("dx-www/src/dev/error_overlay.rs");
  const hotReloadProtocol = read("dx-www/src/hot_reload_protocol.rs");
  const diagnosticSnapshot = read("dx-www/src/dev/diagnostic_snapshot.rs");

  assert.match(
    devFeedback,
    /fn diagnostic_issue_next_action_value\(issue: &Value\) -> Value \{[\s\S]*issue_string_at_paths\(\s*issue,[\s\S]*&\["nextAction"\][\s\S]*&\["hint", "message"\][\s\S]*&\["diagnostic", "remediation"\]/,
    "Rust dev feedback should promote nested hint/remediation shapes into next_action",
  );
  assert.match(
    devFeedback,
    /fn collect_diagnostic_issue_suggestion_text[\s\S]*issue_string_at_paths\(\s*value,[\s\S]*&\["hint", "message"\][\s\S]*&\["fix", "title"\][\s\S]*&\["nextAction"\]/,
    "structured suggestion objects should preserve nested hint/action/fix text",
  );
  assert.match(
    devFeedback,
    /fn errors_snapshot_promotes_nested_hint_next_action\(\)[\s\S]*"hint":\{"message":"Fix the handler export and save the file\."\}[\s\S]*"diagnostic":\{"remediation":"Regenerate diagnostics after saving\."\}[\s\S]*errors\["issues"\]\[0\]\["next_action"\][\s\S]*"Fix the handler export and save the file\."/,
    "a Rust fixture should prove nested hints survive the errors snapshot",
  );

  assert.match(
    errorOverlay,
    /const issueNextAction = firstPayloadTextAtPath\(issue, \[[\s\S]*\['nextAction'\][\s\S]*\['hint', 'message'\][\s\S]*\['diagnostic', 'remediation'\][\s\S]*\]\);/,
    "browser overlay feedback issues should read nested next-action shapes",
  );
  assert.match(
    errorOverlay,
    /next_action: issueNextAction \|\| feedbackNextActionText\(nextAction\)/,
    "feedback issue next actions should prefer issue-specific guidance before snapshot fallbacks",
  );
  assert.match(
    errorOverlay,
    /fn overlay_script_promotes_nested_issue_next_actions\(\)[\s\S]*\['hint', 'message'\][\s\S]*\['diagnostic', 'remediation'\]/,
    "the overlay script test should lock nested issue next-action extraction",
  );

  assert.match(
    hotReloadProtocol,
    /next_action: Option<String>/,
    "hot reload diagnostic issue payloads should carry issue-specific next actions",
  );
  assert.match(
    hotReloadProtocol,
    /pub\(crate\) fn with_next_action\(mut self, next_action: impl Into<String>\) -> Self/,
    "hot reload diagnostics need an explicit builder for next actions",
  );
  assert.match(
    hotReloadProtocol,
    /"next_action": issue\.next_action\.as_deref\(\)/,
    "hot reload issue receipts should serialize next_action for overlay consumers",
  );
  assert.match(
    hotReloadProtocol,
    /issue_payload_reports_diagnostics_without_claiming_module_hmr[\s\S]*\.with_next_action\("Fix the route source and save the file\."\)[\s\S]*payload\["issue_receipt"\]\["issues"\]\[0\]\["next_action"\]/,
    "the hot reload protocol unit fixture should lock next_action serialization",
  );
  assert.match(
    diagnosticSnapshot,
    /fn diagnostic_next_action_from_value\(value: &Value\) -> Option<&str> \{[\s\S]*&\["hint", "message"\][\s\S]*&\["diagnostic", "remediation"\]/,
    "diagnostics snapshots should promote nested hint/remediation into hot reload issues",
  );
  assert.match(
    diagnosticSnapshot,
    /if let Some\(next_action\) = next_action \{[\s\S]*issue = issue\.with_next_action\(next_action\);/,
    "hot reload snapshot conversion should attach parsed next actions to issues",
  );
  assert.match(
    diagnosticSnapshot,
    /diagnostic_snapshot_preserves_nested_source_location_and_code_frame[\s\S]*"hint": \{[\s\S]*"message": "Fix the JSX token and save the file\."[\s\S]*payload\["issue_receipt"\]\["issues"\]\[0\]\["next_action"\]/,
    "the diagnostics snapshot fixture should prove nested next actions survive hot reload publication",
  );
});
