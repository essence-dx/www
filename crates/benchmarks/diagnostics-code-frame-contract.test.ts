import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const workspaceRoot = join(dirname(fileURLToPath(import.meta.url)), "..");

function readWorkspaceFile(path) {
  return readFileSync(join(workspaceRoot, path), "utf8");
}

test("DX diagnostic code-frame contract is public and adapter-boundary only", () => {
  const libSource = readWorkspaceFile("dx-www/src/lib.rs");
  const diagnosticsSource = readWorkspaceFile("dx-www/src/diagnostics.rs");
  const contractSource = readWorkspaceFile("dx-www/src/diagnostics/contract.rs");
  const codeFrameSource = readWorkspaceFile("dx-www/src/diagnostics/code_frame.rs");
  const errorSource = readWorkspaceFile("dx-www/src/error.rs");
  const mainSource = readWorkspaceFile("dx-www/src/main.rs");
  const cliSource = readWorkspaceFile("dx-www/src/cli/mod.rs");
  const cliAppRouteDiagnosticsSource = readWorkspaceFile(
    "dx-www/src/cli/app_route_diagnostics.rs",
  );
  const cliCssDiagnosticsSource = readWorkspaceFile(
    "dx-www/src/cli/css_diagnostics.rs",
  );
  const cliConfigDiagnosticsSource = readWorkspaceFile(
    "dx-www/src/cli/config_diagnostics.rs",
  );
  const basicErrorOverlaySource = readWorkspaceFile("dx-www/src/dev/error_overlay.rs");
  const cliDiagnosticsTestSource = readWorkspaceFile("dx-www/tests/diagnostics_cli.rs");
  const crateRootDiagnosticsExport = libSource.match(/pub use diagnostics::\{([\s\S]*?)\};/);
  const diagnosticsContractExport = diagnosticsSource.match(/pub use contract::\{([\s\S]*?)\};/);

  assert.ok(crateRootDiagnosticsExport, "crate root should re-export diagnostics");
  assert.ok(diagnosticsContractExport, "diagnostics module should re-export its contract");
  for (const requiredExport of [
    "dx_diagnostic_code_frame_contract",
    "DxDiagnosticCodeFrameContract",
    "DX_DIAGNOSTIC_CODE_FRAME_CONTRACT",
  ]) {
    assert.match(crateRootDiagnosticsExport[1], new RegExp(`\\b${requiredExport}\\b`));
    assert.match(diagnosticsContractExport[1], new RegExp(`\\b${requiredExport}\\b`));
  }
  for (const requiredDiagnosticsExport of [
    "dx_diagnostic_code_frame_receipt_view",
    "DxDiagnosticCodeFrameReceiptView",
    "DX_DIAGNOSTIC_CODE_FRAME_RECEIPT_VIEW",
  ]) {
    assert.match(
      crateRootDiagnosticsExport[1],
      new RegExp(`\\b${requiredDiagnosticsExport}\\b`),
    );
    assert.match(
      diagnosticsContractExport[1],
      new RegExp(`\\b${requiredDiagnosticsExport}\\b`),
    );
  }
  assert.match(contractSource, /renderer:\s*"dx-www\.diagnostics\.code-frame"/);
  assert.match(contractSource, /format:\s*1/);
  assert.match(
    contractSource,
    /upstream_reference:\s*"vendor\/next-rust\/crates\/next-code-frame"/,
  );
  assert.match(
    contractSource,
    /boundary:\s*"adapter-boundary: diagnostics formatting only"/,
  );
  assert.match(contractSource, /source_of_truth:\s*"miette \/ dx-check \/ source receipts"/);
  assert.match(contractSource, /requires_react:\s*false/);
  assert.match(contractSource, /requires_rsc:\s*false/);
  assert.match(contractSource, /requires_node:\s*false/);
  assert.match(contractSource, /requires_napi:\s*false/);
  assert.match(contractSource, /requires_npm:\s*false/);
  assert.match(contractSource, /requires_node_modules:\s*false/);
  assert.match(contractSource, /requires_turborepo:\s*false/);
  assert.match(contractSource, /public_turbopack_dependency:\s*false/);
  assert.match(contractSource, /runtime_takeover:\s*false/);
  assert.match(contractSource, /next_code_frame_parity_claimed:\s*false/);
  assert.match(contractSource, /DxDiagnosticCodeFrameReceiptView/);
  assert.match(contractSource, /dx_diagnostic_code_frame_receipt_view/);
  assert.match(contractSource, /schema:\s*"dx\.diagnostics\.code_frame\.contract"/);
  assert.match(contractSource, /forbidden_foundations/);
  assert.match(contractSource, /supported_features/);
  assert.match(codeFrameSource, /visible_end_column\.max\(clamped_start \+ 1\)/);
  assert.match(codeFrameSource, /const TAB_WIDTH: usize = 2;/);
  assert.match(codeFrameSource, /render_source_char/);
  assert.match(codeFrameSource, /display_widths/);
  assert.match(codeFrameSource, /code_frame_expands_tabs_for_terminal_alignment/);
  assert.match(
    codeFrameSource,
    /code_frame_marks_end_of_line_parse_spans_without_panicking/,
  );
  assert.match(errorSource, /pub fn render_dx_error_terminal\(error: &DxError\) -> String/);
  assert.match(errorSource, /fn to_dx_diagnostic\(&self\) -> Option<DxDiagnostic>/);
  assert.match(diagnosticsSource, /pub fn next_action\(&self\) -> Option<&str>/);
  assert.match(diagnosticsSource, /next action: \{next_action\}/);
  assert.match(diagnosticsSource, /render_terminal_includes_dx_next_action/);
  assert.match(diagnosticsSource, /pub code: Option<String>/);
  assert.match(
    diagnosticsSource,
    /pub fn with_code\(mut self, code: impl Into<String>\) -> Self/,
  );
  assert.match(diagnosticsSource, /render_terminal_includes_dx_diagnostic_code/);
  assert.match(
    diagnosticsSource,
    /#\[derive\([^\)]*serde::Serialize[^\)]*\)\]\s*#\[serde\(rename_all = "snake_case"\)\]\s*pub enum DxDiagnosticSeverity/,
  );
  assert.match(
    diagnosticsSource,
    /serializes_dx_diagnostic_severity_as_snake_case/,
  );
  assert.match(diagnosticsSource, /pub fn warning\(title: impl Into<String>, message: impl Into<String>\) -> Self/);
  assert.match(errorSource, /pub severity: DxDiagnosticSeverity/);
  assert.match(errorSource, /pub title: String/);
  assert.match(errorSource, /pub diagnostic_code: Option<String>/);
  assert.match(errorSource, /pub next_action: Option<String>/);
  assert.match(errorSource, /Diagnostic,/);
  assert.match(errorSource, /pub fn from_diagnostic\(diagnostic: &DxDiagnostic\) -> Self/);
  assert.match(errorSource, /title: diagnostic\.title\.clone\(\)/);
  assert.match(errorSource, /fn dx_error_title\(error: &DxError\) -> String/);
  assert.match(errorSource, /overlay\.title = dx_error_title\(error\)/);
  assert.match(errorSource, /fn dx_error_diagnostic_code\(error: &DxError\) -> Option<String>/);
  assert.match(errorSource, /diagnostic_overlay_data_preserves_warning_payload/);
  assert.match(errorSource, /diagnostic_overlay_data_preserves_warning_title/);
  assert.match(errorSource, /error_overlay_data_from_error_preserves_diagnostic_title/);
  assert.match(
    errorSource,
    /fn diagnostic_severity_for_error\(error: &DxError\) -> DxDiagnosticSeverity/,
  );
  assert.match(errorSource, /overlay\.severity = diagnostic_severity_for_error\(error\)/);
  assert.match(errorSource, /overlay\.diagnostic_code = dx_error_diagnostic_code\(error\)/);
  assert.match(errorSource, /fn dx_error_next_action\(error: &DxError\) -> Option<String>/);
  assert.match(errorSource, /parse_error_terminal_and_overlay_include_next_action/);
  assert.match(errorSource, /parse_error_overlay_includes_machine_readable_severity/);
  assert.match(errorSource, /parse_error_overlay_includes_machine_readable_diagnostic_code/);
  assert.match(errorSource, /diagnostic_overlay_data_preserves_warning_code/);
  for (const expectedCode of [
    "dx.config.not_found",
    "dx.config.parse_error",
    "dx.router.duplicate_route",
    "dx.source.parse_error",
    "dx.source.syntax_error",
    "dx.config.validation_error",
    "dx.data.loader_error",
    "dx.dev.port_in_use",
    "dx.dev.hot_reload_failed",
    "dx.assets.not_found",
    "dx.assets.optimization_failed",
    "dx.project.not_found",
    "dx.project.invalid_structure",
    "dx.router.not_found",
    "dx.router.invalid_pattern",
    "dx.parser.missing_section",
    "dx.parser.invalid_language",
    "dx.build.failed",
    "dx.build.dependency_error",
    "dx.build.binary_format_error",
    "dx.build.cache_error",
    "dx.data.timeout",
    "dx.api.handler_error",
    "dx.api.invalid_method",
    "dx.dev.server_error",
    "dx.io.error",
    "dx.io.read_error",
    "dx.io.write_error",
    "dx.internal",
    "dx.not_implemented",
  ]) {
    assert.match(errorSource, new RegExp(`\\.with_code\\("${expectedCode.replaceAll(".", "\\.")}"\\)`));
  }
  assert.match(errorSource, /fn compilation_diagnostic_metadata\(message: &str\) -> CompilationDiagnosticMetadata/);
  assert.match(
    errorSource,
    /CompilationDiagnosticMetadata \{[\s\S]*?code: "dx\.source\.compilation_error"/,
  );
  assert.match(errorSource, /code: "dx\.style\.unsupported_class"/);
  assert.match(
    errorSource,
    /let metadata = compilation_diagnostic_metadata\(message\)[\s\S]*?\.with_code\(metadata\.code\)/,
  );
  assert.match(errorSource, /config_validation_terminal_and_overlay_include_actionable_code/);
  assert.match(errorSource, /data_loader_terminal_and_overlay_include_route_action/);
  assert.match(errorSource, /dev_runtime_terminal_and_overlay_include_actionable_codes/);
  assert.match(errorSource, /asset_errors_terminal_and_overlay_include_actionable_codes/);
  assert.match(errorSource, /project_and_route_terminal_and_overlay_include_actionable_codes/);
  assert.match(errorSource, /parser_boundary_terminal_and_overlay_include_actionable_codes/);
  assert.match(errorSource, /build_and_api_terminal_and_overlay_include_actionable_codes/);
  assert.match(errorSource, /io_and_internal_terminal_and_overlay_include_actionable_codes/);
  assert.match(errorSource, /fn diagnostic_overlay_type_for_error\(error: &DxError\) -> ErrorType/);
  assert.match(errorSource, /overlay\.error_type = diagnostic_overlay_type_for_error\(error\)/);
  assert.match(errorSource, /fn source_range_from_span\(/);
  assert.match(errorSource, /with_source_range\(/);
  assert.match(errorSource, /fn diagnostic_code_frame_for_error\(error: &DxError\) -> Option<String>/);
  assert.match(errorSource, /code_frame:\s*Option<String>/);
  assert.match(errorSource, /DxError::ConfigNotFound/);
  assert.match(errorSource, /DxDiagnostic::error\(\s*"Configuration file not found"/);
  assert.match(
    errorSource,
    /DxError::ConfigNotFound[\s\S]*?error_type:\s*ErrorType::Config/,
  );
  assert.match(
    errorSource,
    /config_not_found_terminal_and_overlay_are_dx_branded/,
  );
  assert.match(errorSource, /unwrap_or_else\(\|\| format!\("DX-WWW error: \{error\}\\n"\)\)/);
  assert.match(
    errorSource,
    /terminal_rendering_falls_back_to_dx_branded_message_without_debug_dump/,
  );
  assert.match(errorSource, /DxError::DuplicateRoute/);
  assert.match(errorSource, /DxDiagnostic::error\(\s*"Duplicate route"/);
  assert.match(
    errorSource,
    /DxError::DuplicateRoute[\s\S]*?error_type:\s*ErrorType::Compilation/,
  );
  assert.match(
    errorSource,
    /duplicate_route_terminal_and_overlay_name_both_route_sources/,
  );
  assert.match(errorSource, /DxError::SyntaxError/);
  assert.match(errorSource, /DxDiagnostic::error\("Syntax error"/);
  assert.match(
    errorSource,
    /DxError::SyntaxError[\s\S]*?error_type:\s*ErrorType::Compilation/,
  );
  assert.match(
    errorSource,
    /syntax_error_terminal_and_overlay_preserve_location_without_source/,
  );
  assert.match(errorSource, /DxError::ParseError/);
  assert.match(errorSource, /DxDiagnostic::error\("Parse failed"/);
  assert.match(
    errorSource,
    /DxError::ParseError[\s\S]*?source_range_from_span\(source,\s*\*span\)/,
  );
  assert.doesNotMatch(errorSource, /\+\s*column\.saturating_sub\(1\)/);
  assert.match(errorSource, /parse_error_terminal_rendering_uses_dx_code_frame/);
  assert.match(
    errorSource,
    /parse_error_overlay_and_terminal_prefer_source_span_location/,
  );
  assert.match(errorSource, /source_span_locations_use_utf8_character_columns/);
  assert.match(errorSource, /parse_error_with_context_uses_utf8_character_columns/);
  assert.match(
    errorSource,
    /parse_error_with_context_marks_end_of_line_without_next_line_span/,
  );
  assert.match(errorSource, /source_offset_from_location\(src,\s*line,\s*column\)/);
  assert.match(errorSource, /source_char_len_at_offset\(src,\s*offset\)/);
  assert.match(errorSource, /source\.char_indices\(\)/);
  assert.match(errorSource, /ch\.len_utf8\(\)/);
  assert.match(errorSource, /compilation_error_terminal_rendering_marks_dx_source_span_range/);
  assert.match(errorSource, /compilation_error_without_span_still_has_diagnostic_code_and_source/);
  assert.match(errorSource, /error_overlay_data_includes_dx_code_frame_for_compilation_error/);
  assert.match(
    errorSource,
    /DxError::CompilationError[\s\S]*?source_location_from_offset\(source,\s*span\.offset\(\)\)/,
  );
  assert.match(
    errorSource,
    /DxError::ConfigParseError[\s\S]*?error_type:\s*ErrorType::Config/,
  );
  assert.match(
    errorSource,
    /error_overlay_data_includes_dx_code_frame_for_config_parse_error/,
  );
  assert.match(mainSource, /fn render_cli_error\(error: &dx_www::DxError\) -> String/);
  assert.match(mainSource, /render_dx_error_terminal\(error\)/);
  assert.match(mainSource, /render_cli_error\(&e\)/);
  assert.match(mainSource, /render_cli_parse_error_uses_dx_code_frame/);
  assert.doesNotMatch(mainSource, /eprintln!\("\{e:\?\}"\)/);
  assert.match(cliSource, /mod app_route_diagnostics;/);
  assert.match(cliSource, /mod css_diagnostics;/);
  assert.match(
    cliSource,
    /app_route_diagnostics::validate_app_route_source\(&self\.cwd,\s*page_path\)\?/,
  );
  assert.match(
    cliSource,
    /app_route_diagnostics::validate_app_route_handlers\(&self\.cwd\)\?/,
  );
  assert.match(cliSource, /app_route_diagnostics::app_route_compile_error/);
  assert.match(cliAppRouteDiagnosticsSource, /parse_tsx_module/);
  assert.match(cliAppRouteDiagnosticsSource, /extract_class_attribute_tokens/);
  assert.match(cliAppRouteDiagnosticsSource, /unsupported_scanned_classes/);
  assert.match(cliAppRouteDiagnosticsSource, /DxError::ParseError/);
  assert.match(cliAppRouteDiagnosticsSource, /DxError::CompilationError/);
  assert.match(cliAppRouteDiagnosticsSource, /dx_style_source_error/);
  assert.match(cliAppRouteDiagnosticsSource, /validate_related_app_route_sources/);
  assert.match(cliAppRouteDiagnosticsSource, /validate_app_route_handlers/);
  assert.match(cliAppRouteDiagnosticsSource, /css_diagnostics::validate_style_sources/);
  assert.match(cliAppRouteDiagnosticsSource, /app_segment_diagnostic_paths/);
  assert.match(cliAppRouteDiagnosticsSource, /app_route_handler_diagnostic_paths/);
  assert.match(cliAppRouteDiagnosticsSource, /use super::app_segment_files;/);
  assert.match(cliAppRouteDiagnosticsSource, /app_segment_files::app_route_roots/);
  assert.match(cliAppRouteDiagnosticsSource, /app_segment_files::app_root_for_route/);
  assert.match(cliAppRouteDiagnosticsSource, /src\/app\/layout\.tsx/);
  assert.match(cliAppRouteDiagnosticsSource, /src\/app\/api\/health\/route\.ts/);
  assert.match(cliAppRouteDiagnosticsSource, /app_route_diagnostics_validate_src_app_segment_files/);
  assert.match(cliAppRouteDiagnosticsSource, /app_route_handler_diagnostics_scan_src_app_handlers/);
  assert.match(
    cliAppRouteDiagnosticsSource,
    /app_route_handler_diagnostics_scan_handlers_and_skip_node_modules/,
  );
  assert.match(
    cliAppRouteDiagnosticsSource,
    /app_route_handler_diagnostics_report_parse_errors_with_source_span/,
  );
  assert.match(
    basicErrorOverlaySource,
    /use crate::diagnostics::\{DxDiagnostic, DxDiagnosticSeverity\};/,
  );
  assert.match(basicErrorOverlaySource, /use crate::error::\{DxError, ErrorOverlayData\};/);
  assert.match(
    basicErrorOverlaySource,
    /pub fn show_diagnostic\(&mut self, diagnostic: &DxDiagnostic\)/,
  );
  assert.match(
    basicErrorOverlaySource,
    /pub fn show_payload\(&mut self, payload: &ErrorOverlayData\)/,
  );
  assert.match(basicErrorOverlaySource, /pub severity: DxDiagnosticSeverity/);
  assert.match(basicErrorOverlaySource, /pub diagnostic_code: Option<String>/);
  assert.match(basicErrorOverlaySource, /pub next_action: Option<String>/);
  assert.match(basicErrorOverlaySource, /pub suggestions: Vec<String>/);
  assert.match(
    basicErrorOverlaySource,
    /pub fn from_diagnostic\(diagnostic: &DxDiagnostic\) -> Self/,
  );
  assert.match(
    basicErrorOverlaySource,
    /pub fn from_payload\(payload: &ErrorOverlayData\) -> Self/,
  );
  assert.match(basicErrorOverlaySource, /let title = title_for_payload\(&payload\);/);
  assert.match(basicErrorOverlaySource, /title: title_for_payload\(payload\)/);
  assert.match(basicErrorOverlaySource, /payload\.title\.trim\(\)/);
  assert.match(basicErrorOverlaySource, /fn title_from_diagnostic_code\(code: &str\) -> Option<String>/);
  assert.match(basicErrorOverlaySource, /show_payload_infers_title_from_diagnostic_code_when_missing/);
  assert.match(basicErrorOverlaySource, /function normalizeOverlayPayload\(payload\)/);
  assert.match(basicErrorOverlaySource, /function titleFromDiagnosticCode\(code\)/);
  assert.match(basicErrorOverlaySource, /titleFromDiagnosticCode\(diagnosticCode\)/);
  assert.match(basicErrorOverlaySource, /overlay_script_normalizes_issue_payload_title_and_code/);
  assert.match(basicErrorOverlaySource, /const DX_FEEDBACK_ERRORS_ENDPOINT = '\/_dx\/feedback\/errors';/);
  assert.match(basicErrorOverlaySource, /function payloadValueAtPath\(payload, path\)/);
  assert.match(basicErrorOverlaySource, /function firstPayloadTextAtPath\(payload, paths\)/);
  assert.match(basicErrorOverlaySource, /function firstPayloadIntegerAtPath\(payload, paths\)/);
  assert.match(basicErrorOverlaySource, /function severityText\(value\)/);
  assert.match(basicErrorOverlaySource, /function normalizeSeverityName\(value\)/);
  assert.match(basicErrorOverlaySource, /function overlaySeverityName\(value\)/);
  assert.match(
    basicErrorOverlaySource,
    /function normalizeOverlayPayload\(payload\) \{[\s\S]*?const severity = overlaySeverityName\(normalized\);/,
  );
  assert.match(
    basicErrorOverlaySource,
    /function severityText\(value\) \{[\s\S]*?\['diagnostic', 'severity'\][\s\S]*?\['level'\]/,
  );
  assert.match(
    basicErrorOverlaySource,
    /function normalizeSeverityName\(value\) \{[\s\S]*?severity === 'warn'[\s\S]*?return 'warning'/,
  );
  assert.match(
    basicErrorOverlaySource,
    /function overlaySeverityName\(value\) \{[\s\S]*?return severity === 'info' \|\| severity === 'warning' \? 'warning' : 'error'/,
  );
  assert.match(basicErrorOverlaySource, /function suggestionText\(value\)/);
  assert.match(basicErrorOverlaySource, /function suggestionTextsFromValue\(value\)/);
  assert.match(basicErrorOverlaySource, /function collectPayloadSuggestions\(payload, paths\)/);
  assert.match(
    basicErrorOverlaySource,
    /function normalizeOverlayPayload\(payload\) \{[\s\S]*?const suggestions = collectPayloadSuggestions\(normalized, \[[\s\S]*?\['suggestions'\][\s\S]*?\['fixes'\][\s\S]*?\['next_actions'\][\s\S]*?\['diagnostic', 'hints'\][\s\S]*?\]\);/,
  );
  assert.match(
    basicErrorOverlaySource,
    /function issueToOverlayPayload\(issue, nextAction\) \{[\s\S]*?const suggestions = collectPayloadSuggestions\(issue, \[[\s\S]*?\['suggestions'\][\s\S]*?\['fixes'\][\s\S]*?\['nextActions'\][\s\S]*?\]\);/,
  );
  assert.doesNotMatch(
    basicErrorOverlaySource,
    /normalized\.suggestions\.filter\(\(suggestion\) => typeof suggestion === 'string'/,
  );
  assert.match(basicErrorOverlaySource, /const directFilePath = firstPayloadTextAtPath\(normalized, \[/);
  assert.match(basicErrorOverlaySource, /\['source_location', 'path'\]/);
  assert.match(basicErrorOverlaySource, /\['sourceLocation', 'file'\]/);
  assert.match(basicErrorOverlaySource, /const directLine = firstPayloadIntegerAtPath\(normalized, \[/);
  assert.match(basicErrorOverlaySource, /\['source_location', 'line'\]/);
  assert.match(basicErrorOverlaySource, /const directColumn = firstPayloadIntegerAtPath\(normalized, \[/);
  assert.match(basicErrorOverlaySource, /\['source_location', 'column'\]/);
  assert.match(basicErrorOverlaySource, /const directCodeFrame = firstPayloadTextAtPath\(normalized, \[/);
  assert.match(basicErrorOverlaySource, /\['diagnostic', 'code_frame'\]/);
  assert.match(basicErrorOverlaySource, /const directCodeContext = firstPayloadTextAtPath\(normalized, \[/);
  assert.match(basicErrorOverlaySource, /\['source', 'snippet'\]/);
  assert.match(basicErrorOverlaySource, /const directNextAction = firstPayloadTextAtPath\(normalized, \[/);
  assert.match(basicErrorOverlaySource, /\['hint', 'message'\]/);
  assert.match(basicErrorOverlaySource, /normalized\.file_path = directFilePath/);
  assert.match(basicErrorOverlaySource, /normalized\.line = directLine/);
  assert.match(basicErrorOverlaySource, /normalized\.column = directColumn/);
  assert.match(basicErrorOverlaySource, /normalized\.code_frame = directCodeFrame \|\| optionalPayloadText\(normalized\.code_frame\)/);
  assert.match(basicErrorOverlaySource, /normalized\.code_context = directCodeContext \|\| optionalPayloadText\(normalized\.code_context\)/);
  assert.match(basicErrorOverlaySource, /normalized\.next_action = directNextAction \|\| optionalPayloadText\(normalized\.next_action\)/);
  assert.match(
    basicErrorOverlaySource,
    /function normalizeOverlayPayload\(payload\) \{[\s\S]*?const directFilePath = firstPayloadTextAtPath\(normalized, \[[\s\S]*?\['source_location', 'path'\][\s\S]*?const directCodeFrame = firstPayloadTextAtPath\(normalized, \[[\s\S]*?\['diagnostic', 'code_frame'\][\s\S]*?normalized\.file_path = directFilePath[\s\S]*?normalized\.code_frame = directCodeFrame \|\| optionalPayloadText\(normalized\.code_frame\)[\s\S]*?return normalized;\n    \}/,
  );
  assert.match(basicErrorOverlaySource, /function issueSeverityName\(issue\)/);
  assert.match(basicErrorOverlaySource, /function issueSeverityRank\(issue\)/);
  assert.match(basicErrorOverlaySource, /function issueShouldOpenOverlay\(issue\)/);
  assert.match(basicErrorOverlaySource, /return issueSeverityName\(issue\) !== 'info'/);
  assert.match(basicErrorOverlaySource, /if \(!issueShouldOpenOverlay\(issue\)\)/);
  assert.match(basicErrorOverlaySource, /function highestSeverityIssue\(issues\)/);
  assert.match(basicErrorOverlaySource, /function issueToOverlayPayload\(issue, nextAction\)/);
  assert.match(basicErrorOverlaySource, /const filePath = firstPayloadTextAtPath\(issue, \[/);
  assert.match(basicErrorOverlaySource, /\['source', 'path'\]/);
  assert.match(basicErrorOverlaySource, /\['span', 'source', 'path'\]/);
  assert.match(basicErrorOverlaySource, /const line = firstPayloadIntegerAtPath\(issue, \[/);
  assert.match(basicErrorOverlaySource, /\['location', 'line'\]/);
  assert.match(basicErrorOverlaySource, /\['span', 'start', 'line'\]/);
  assert.match(basicErrorOverlaySource, /const column = firstPayloadIntegerAtPath\(issue, \[/);
  assert.match(basicErrorOverlaySource, /\['location', 'column'\]/);
  assert.match(basicErrorOverlaySource, /\['span', 'start', 'column'\]/);
  assert.match(basicErrorOverlaySource, /const codeFrame = firstPayloadTextAtPath\(issue, \[/);
  assert.match(basicErrorOverlaySource, /\['codeFrame', 'rendered'\]/);
  assert.match(basicErrorOverlaySource, /\['code_frame', 'rendered'\]/);
  assert.match(basicErrorOverlaySource, /function overlayPayloadFromFeedbackErrors\(snapshot\)/);
  assert.match(
    basicErrorOverlaySource,
    /function feedbackSnapshotHasAuthoritativeEmptyIssueList\(snapshot\)/,
  );
  assert.match(basicErrorOverlaySource, /function parseFeedbackEventPayload\(value\)/);
  assert.match(basicErrorOverlaySource, /function feedbackErrorsSnapshotFromEventPayload\(payload\)/);
  assert.match(basicErrorOverlaySource, /window\.__DX_APPLY_FEEDBACK_ERRORS__ = function\(snapshot\)/);
  assert.match(basicErrorOverlaySource, /window\.__DX_APPLY_DEV_FEEDBACK__ = function\(payload\)/);
  assert.match(basicErrorOverlaySource, /window\.__DX_APPLY_FEEDBACK_ERRORS__\(snapshot\)/);
  assert.match(basicErrorOverlaySource, /return window\.__DX_APPLY_FEEDBACK_ERRORS__\(snapshot\)/);
  assert.match(
    basicErrorOverlaySource,
    /feedbackSnapshotClearsOverlay\(snapshot\) \|\| feedbackSnapshotHasAuthoritativeEmptyIssueList\(snapshot\)/,
  );
  assert.match(basicErrorOverlaySource, /window\.__DX_HIDE_ERROR__\(\)/);
  assert.match(basicErrorOverlaySource, /window\.addEventListener\('dx:feedback-errors', function\(event\)/);
  assert.match(basicErrorOverlaySource, /window\.__DX_APPLY_FEEDBACK_ERRORS__\(event\.detail\)/);
  assert.match(basicErrorOverlaySource, /window\.addEventListener\('dx-dev-feedback', function\(event\)/);
  assert.match(basicErrorOverlaySource, /window\.__DX_APPLY_DEV_FEEDBACK__\(event\.detail \|\| event\)/);
  assert.match(basicErrorOverlaySource, /window\.__DX_REFRESH_ERROR_OVERLAY__ = async function\(\)/);
  assert.match(basicErrorOverlaySource, /fetch\(DX_FEEDBACK_ERRORS_ENDPOINT, \{ cache: 'no-store'/);
  assert.match(basicErrorOverlaySource, /window\.__DX_SHOW_ERROR__\(payload\)/);
  assert.match(basicErrorOverlaySource, /overlay_script_applies_feedback_error_snapshots_without_fetch/);
  assert.match(basicErrorOverlaySource, /overlay_script_accepts_nested_dev_feedback_event_payloads/);
  assert.match(basicErrorOverlaySource, /overlay_script_clears_stale_feedback_errors_after_recovery/);
  assert.match(basicErrorOverlaySource, /overlay_script_bridges_feedback_errors_to_basic_overlay/);
  assert.match(basicErrorOverlaySource, /show_error_preserves_diagnostic_overlay_title/);
  assert.match(basicErrorOverlaySource, /show_payload_preserves_structured_error_overlay_title/);
  assert.match(basicErrorOverlaySource, /data-dx-error-severity/);
  assert.match(basicErrorOverlaySource, /data-dx-error-code/);
  assert.match(basicErrorOverlaySource, /data-dx-error-title/);
  assert.match(basicErrorOverlaySource, /data-dx-error-file/);
  assert.match(basicErrorOverlaySource, /window\.__DX_LAST_ERROR_PAYLOAD__ = normalized/);
  assert.match(basicErrorOverlaySource, /window\.__DX_LAST_ERROR_PAYLOAD__ = null/);
  assert.match(basicErrorOverlaySource, /overlay_script_exposes_browser_visible_payload_state/);
  assert.match(basicErrorOverlaySource, /overlay_script_does_not_escalate_info_feedback_to_error/);
  assert.match(basicErrorOverlaySource, /overlay_script_normalizes_warning_aliases_without_error_escalation/);
  assert.match(basicErrorOverlaySource, /window\.__DX_SHOW_ERROR__ = function\(payload\)/);
  assert.doesNotMatch(
    basicErrorOverlaySource,
    /codeNode\.textContent = normalized\.code_frame \|\| normalized\.code \|\|/,
  );
  assert.match(basicErrorOverlaySource, /normalized\.next_action/);
  assert.match(basicErrorOverlaySource, /fn suggestion_items_html\(suggestions: &\[String\]\) -> String/);
  assert.match(basicErrorOverlaySource, /class="dx-error-suggestions"/);
  assert.match(basicErrorOverlaySource, /const suggestions = collectPayloadSuggestions\(normalized, \[/);
  assert.match(basicErrorOverlaySource, /const suggestionsNode = overlay\.querySelector\('\.dx-error-suggestions'\)/);
  assert.match(basicErrorOverlaySource, /document\.createElement\('li'\)/);
  assert.match(basicErrorOverlaySource, /item\.textContent = suggestion/);
  assert.match(basicErrorOverlaySource, /function optionalPayloadText\(value\)/);
  assert.match(basicErrorOverlaySource, /function setOptionalText\(node, value\)/);
  assert.match(basicErrorOverlaySource, /node\.hidden = text\.length === 0/);
  assert.match(
    basicErrorOverlaySource,
    /messageNode\.textContent = optionalPayloadText\(normalized\.message\) \|\| 'No diagnostic message was provided\.'/,
  );
  assert.match(
    basicErrorOverlaySource,
    /setOptionalText\(diagnosticCodeNode, normalized\.diagnostic_code\)/,
  );
  assert.match(basicErrorOverlaySource, /setOptionalText\(fileNode, formatPayloadLocation\(normalized\)\)/);
  assert.match(
    basicErrorOverlaySource,
    /setOptionalText\(codeNode, normalized\.code_frame \|\| normalized\.code_context \|\| normalized\.source\)/,
  );
  assert.match(
    basicErrorOverlaySource,
    /hintNode\.parentElement\.hidden = hintNode\.hidden/,
  );
  assert.match(basicErrorOverlaySource, /function formatPayloadLocation\(normalized\)/);
  assert.match(basicErrorOverlaySource, /Number\.isInteger\(normalized\.line\)/);
  assert.match(basicErrorOverlaySource, /Number\.isInteger\(normalized\.column\)/);
  assert.doesNotMatch(
    basicErrorOverlaySource,
    /fileNode\.textContent = formatPayloadLocation\(normalized\)/,
  );
  assert.match(
    basicErrorOverlaySource,
    /show_diagnostic_preserves_warning_overlay_payload/,
  );
  assert.match(
    basicErrorOverlaySource,
    /show_payload_preserves_structured_error_overlay_data/,
  );
  assert.match(
    basicErrorOverlaySource,
    /overlay_html_and_script_render_suggestions_without_interpolation/,
  );
  assert.match(
    basicErrorOverlaySource,
    /overlay_script_accepts_structured_payload_without_raw_message_interpolation/,
  );
  assert.match(
    basicErrorOverlaySource,
    /overlay_script_formats_file_line_column_payloads/,
  );
  assert.match(
    basicErrorOverlaySource,
    /overlay_script_hides_empty_optional_payload_fields/,
  );
  assert.doesNotMatch(basicErrorOverlaySource, /\$\{message\}/);
  assert.match(cliAppRouteDiagnosticsSource, /component_diagnostic_paths/);
  assert.doesNotMatch(cliAppRouteDiagnosticsSource, /fn style_diagnostic_paths/);
  assert.doesNotMatch(cliAppRouteDiagnosticsSource, /fn css_source_error/);
  assert.match(cliAppRouteDiagnosticsSource, /project_relative_slash_path/);
  assert.match(cliCssDiagnosticsSource, /pub\(super\) fn validate_style_sources/);
  assert.match(cliCssDiagnosticsSource, /fn style_diagnostic_paths/);
  assert.match(cliCssDiagnosticsSource, /fn css_source_error/);
  assert.match(cliCssDiagnosticsSource, /DxError::ParseError/);
  assert.match(cliCssDiagnosticsSource, /CSS block is missing a closing `}`\./);
  assert.match(
    cliCssDiagnosticsSource,
    /css_diagnostics_reports_unclosed_blocks_with_source_span/,
  );
  assert.match(
    cliCssDiagnosticsSource,
    /css_diagnostics_ignores_braces_inside_strings_and_comments/,
  );
  assert.match(cliSource, /mod config_diagnostics;/);
  assert.match(
    cliSource,
    /pub fn cmd_build\(&self\) -> DxResult<\(\)>[\s\S]*?config_diagnostics::load_project_config_with_diagnostics\(&self\.cwd\)/,
  );
  assert.match(
    cliConfigDiagnosticsSource,
    /pub\(super\) fn load_project_config_with_diagnostics/,
  );
  assert.match(cliConfigDiagnosticsSource, /DxError::ConfigParseError/);
  assert.match(cliConfigDiagnosticsSource, /legacy_toml_config_error/);
  assert.match(cliConfigDiagnosticsSource, /ConfigError::ParseError\(error\)[\s\S]*?\.span\(\)/);
  assert.match(cliConfigDiagnosticsSource, /dx\.config\.toml/);
  assert.match(cliConfigDiagnosticsSource, /miette::SourceSpan::new/);
  assert.match(cliDiagnosticsTestSource, /build_process_emits_dx_code_frame_for_invalid_dx_config/);
  assert.match(
    cliDiagnosticsTestSource,
    /build_process_emits_dx_code_frame_for_invalid_legacy_toml_config/,
  );
  assert.match(
    cliDiagnosticsTestSource,
    /build_process_emits_dx_code_frame_for_invalid_app_route_tsx/,
  );
  assert.match(
    cliDiagnosticsTestSource,
    /build_process_emits_dx_code_frame_for_invalid_dx_style_class/,
  );
  assert.match(
    cliDiagnosticsTestSource,
    /build_process_emits_dx_code_frame_for_invalid_component_dx_style_class/,
  );
  assert.match(
    cliDiagnosticsTestSource,
    /build_process_emits_dx_code_frame_for_invalid_route_handler_ts/,
  );
  assert.match(
    cliDiagnosticsTestSource,
    /build_process_emits_dx_code_frame_for_invalid_css_source/,
  );
  assert.match(cliDiagnosticsTestSource, /CARGO_BIN_EXE_dx-www/);
  assert.match(cliDiagnosticsTestSource, /\.arg\("build"\)/);
  assert.match(cliDiagnosticsTestSource, /DX-WWW error: Config parse failed/);
  assert.match(cliDiagnosticsTestSource, /-->\s*dx:/);
  assert.match(cliDiagnosticsTestSource, /-->\s*dx\.config\.toml:/);
  assert.match(cliDiagnosticsTestSource, /-->\s*app\/page\.tsx:/);
  assert.match(cliDiagnosticsTestSource, /grouped classname syntax is invalid/);
  assert.match(cliDiagnosticsTestSource, /components\/BrokenCard\.tsx/);
  assert.match(cliDiagnosticsTestSource, /-->\s*app\/api\/health\/route\.ts:/);
  assert.match(cliDiagnosticsTestSource, /-->\s*styles\/app\.css:/);
  assert.match(cliDiagnosticsTestSource, /CSS block is missing a closing `}`\./);
});
