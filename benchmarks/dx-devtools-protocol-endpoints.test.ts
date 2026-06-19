import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath: string): string {
  const fullPath = path.join(repoRoot, relativePath);
  assert.ok(fs.existsSync(fullPath), `expected ${relativePath} to exist`);
  return fs.readFileSync(fullPath, "utf8");
}

function extractBalancedBlock(source: string, signaturePattern: RegExp, label: string): string {
  const match = signaturePattern.exec(source);
  assert.ok(match, `expected to find ${label}`);
  const start = match.index;
  const openBrace = source.indexOf("{", start);
  assert.notEqual(openBrace, -1, `expected ${label} to have a body`);

  let depth = 0;
  let inLineComment = false;
  let inBlockComment = false;
  let inString = false;
  let escaped = false;

  for (let index = openBrace; index < source.length; index += 1) {
    const current = source[index];
    const next = source[index + 1];

    if (inLineComment) {
      if (current === "\n") inLineComment = false;
      continue;
    }
    if (inBlockComment) {
      if (current === "*" && next === "/") {
        inBlockComment = false;
        index += 1;
      }
      continue;
    }
    if (inString) {
      if (escaped) {
        escaped = false;
      } else if (current === "\\") {
        escaped = true;
      } else if (current === '"') {
        inString = false;
      }
      continue;
    }

    if (current === "/" && next === "/") {
      inLineComment = true;
      index += 1;
      continue;
    }
    if (current === "/" && next === "*") {
      inBlockComment = true;
      index += 1;
      continue;
    }
    if (current === '"') {
      inString = true;
      continue;
    }
    if (current === "{") {
      depth += 1;
    } else if (current === "}") {
      depth -= 1;
      if (depth === 0) {
        return source.slice(start, index + 1);
      }
    }
  }

  assert.fail(`expected to find the end of ${label}`);
}

test("dev response dispatches real devtools protocol data before app routing", () => {
  const devResponse = read("dx-www/src/cli/dev_response.rs");
  const dispatch = extractBalancedBlock(
    devResponse,
    /pub\(super\)\s+fn\s+handle_parsed_http_response\s*\(/,
    "handle_parsed_http_response",
  );

  assert.match(
    dispatch,
    /devtools::devtools_cli_response\(\s*cwd,\s*request,\s*true\s*\)/,
    "dev response should delegate devtools paths to the real protocol surface",
  );
  assert.doesNotMatch(
    dispatch,
    /disabled_devtools_response\(request\)/,
    "dev response must not hard-disable devtools endpoints before routing",
  );
  assert.ok(
    dispatch.indexOf("devtools::devtools_cli_response") < dispatch.indexOf("is_hot_reload_version_request"),
    "devtools protocol dispatch should run before hot reload and app route fallbacks",
  );
});

test("devtools protocol endpoints return real session, route, diagnostics, source, style, and undo data", () => {
  const protocol = read("dx-www/src/cli/devtools/protocol.rs");
  const sourceMap = read("dx-www/src/cli/devtools/source_map.rs");
  const styleOps = read("dx-www/src/cli/devtools/style_ops.rs");
  const response = extractBalancedBlock(
    protocol,
    /pub\(super\)\s+fn\s+devtools_protocol_response\s*\(/,
    "devtools_protocol_response",
  );
  const session = extractBalancedBlock(protocol, /fn\s+session_payload\s*\(/, "session_payload");
  const route = extractBalancedBlock(protocol, /fn\s+route_payload\s*\(/, "route_payload");
  const diagnostics = extractBalancedBlock(protocol, /fn\s+diagnostics_payload\s*\(/, "diagnostics_payload");
  const sourceMapPayload = extractBalancedBlock(protocol, /fn\s+source_map_payload\s*\(/, "source_map_payload");
  const stylePreview = extractBalancedBlock(protocol, /fn\s+style_preview_payload\s*\(/, "style_preview_payload");
  const styleApply = extractBalancedBlock(protocol, /fn\s+style_apply_payload\s*\(/, "style_apply_payload");
  const styleUndo = extractBalancedBlock(protocol, /fn\s+style_undo_payload\s*\(/, "style_undo_payload");

  for (const endpoint of [
    "SESSION_ENDPOINT",
    "ROUTE_ENDPOINT",
    "DIAGNOSTICS_ENDPOINT",
    "SOURCE_MAP_ENDPOINT",
    "STYLE_PREVIEW_ENDPOINT",
    "STYLE_APPLY_ENDPOINT",
    "STYLE_UNDO_ENDPOINT",
  ]) {
    assert.match(response, new RegExp(endpoint), `${endpoint} should be routed by the protocol`);
  }

  assert.match(session, /source_map::collect_project_routes\(project_root\)/, "session should read real route data");
  assert.match(session, /diagnostics_payload\(project_root\)/, "session should summarize real diagnostics");
  assert.match(route, /source_map::route_for_request\(project_root,\s*route\)/, "route endpoint should resolve real routes");
  assert.match(diagnostics, /\.dx\/diagnostics\/latest\.json/, "diagnostics should read the latest diagnostics artifact");
  assert.match(diagnostics, /\.dx\/receipts\/check\/check-latest\.json/, "diagnostics should include the check receipt");
  assert.match(sourceMapPayload, /source_map::resolve_source_location\(project_root,\s*request_path,\s*body\)/, "source-map should resolve project source");
  assert.match(sourceMap, /collect_project_routes\(project_root\)/, "source map should discover app and pages routes");
  assert.match(stylePreview, /style_ops::preview_style_change_json\(&request\)/, "style-preview should use the structured style operation engine");
  assert.match(styleApply, /style_ops::apply_style_change_json\(project_root,\s*&request\)/, "style-apply should use the structured style operation engine");
  assert.match(styleApply, /write_visual_edit_receipt\(/, "safe style-apply should persist durable receipts");
  assert.match(styleUndo, /style_ops::undo_style_change_json\(project_root,\s*&latest_receipt\)/, "style-undo should use the undo operation engine");
  assert.match(styleUndo, /write_visual_edit_undo_receipt\(/, "style-undo should persist durable undo receipts");
  assert.match(styleOps, /expected_text_after[\s\S]*restore_text_before[\s\S]*undo-source-range-mismatch/, "undo should restore only the exact captured source range");
});

test("style preview stays read-only and style apply writes only exact authored CSS targets", () => {
  const protocol = read("dx-www/src/cli/devtools/protocol.rs");
  const styleOps = read("dx-www/src/cli/devtools/style_ops.rs");
  const stylePreview = extractBalancedBlock(protocol, /fn\s+style_preview_payload\s*\(/, "style_preview_payload");
  const previewOperation = extractBalancedBlock(
    styleOps,
    /pub\(crate\)\s+fn\s+preview_style_change\s*\(/,
    "preview_style_change",
  );
  const applyOperation = extractBalancedBlock(
    styleOps,
    /pub\(crate\)\s+fn\s+apply_style_change\s*\(/,
    "apply_style_change",
  );
  const sourceEligibility = extractBalancedBlock(
    styleOps,
    /pub\(crate\)\s+fn\s+source_write_eligibility\s*\(/,
    "source_write_eligibility",
  );
  const safeProjectPath = extractBalancedBlock(styleOps, /fn\s+safe_project_path\s*\(/, "safe_project_path");

  assert.match(stylePreview, /"mutates_source": false/, "style-preview response should advertise no source mutation");
  assert.match(previewOperation, /mutated:\s*false/, "style-preview operation should never mutate source");
  assert.doesNotMatch(
    `${stylePreview}\n${previewOperation}`,
    /\b(?:std::fs::write|fs::write|File::create|OpenOptions::new|write_all)\b/,
    "style-preview code path must not perform filesystem writes",
  );
  assert.match(sourceEligibility, /target\.kind\s*!=\s*DxSourceTargetKind::AuthoredCss/, "style-apply should reject non-authored-CSS source kinds");
  assert.match(sourceEligibility, /!is_authored_css_path\(&relative_path\)/, "style-apply should reject non-CSS source paths");
  assert.match(sourceEligibility, /target\.range\.is_none\(\)/, "style-apply should require an exact source range");
  assert.match(sourceEligibility, /!looks_like_css_declaration\(&range\.expected_text\)/, "style-apply should require a declaration-shaped expected range");
  assert.match(applyOperation, /current_text\s*!=\s*range\.expected_text/, "style-apply should verify the exact source text before writing");
  assert.match(applyOperation, /declaration_property_matches_validation\(&range\.expected_text,\s*&validation\)/, "style-apply should match the requested property to the source declaration");
  assert.match(applyOperation, /\bfs::write\(&project_path,\s*updated\)\?;/, "style-apply should perform exactly one bounded source write");
  assert.match(safeProjectPath, /canonicalize\(project_root\)/, "style-apply should canonicalize the project root");
  assert.match(safeProjectPath, /starts_with\(&root\)/, "style-apply should keep writes inside the project root");
});

test("production build and preview surfaces exclude devtools endpoints and assets", () => {
  const cliCoreImpl = read("dx-www/src/cli/mod_parts/cli_core_impl.rs");
  const previewContract = read("dx-www/src/cli/preview_contract.rs");
  const cmdBuild = extractBalancedBlock(
    cliCoreImpl,
    /pub\s+fn\s+cmd_build\s*\(&self\)\s*->\s*DxResult<\(\)>/,
    "DxCli::cmd_build",
  );
  const productionSurface = `${cmdBuild}\n${previewContract}`;

  assert.doesNotMatch(
    productionSurface,
    /\/_dx\/devtools|style-preview|style-apply|style-undo|devtools\.css|runtime\.js|data-dx-devtools/i,
    "production build and production preview should not expose devtools endpoints or assets",
  );
  assert.match(
    previewContract,
    /is not listed in deploy-adapter\.json/,
    "production preview should serve only deploy-adapter-listed routes and assets",
  );
});
