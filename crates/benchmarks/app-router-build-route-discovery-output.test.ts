const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("App Router build emits route discovery and skipped-route diagnostics", () => {
  const buildCommand = read("dx-www/src/cli/app_router_build_command.rs");
  const pageRoutes = read("dx-www/src/cli/app_page_routes.rs");
  const diagnostics = read("dx-www/src/cli/app_page_route_diagnostics.rs");

  assert.match(buildCommand, /const APP_ROUTE_DISCOVERY_SUMMARY_JSON: &str = "app-route-discovery\.json";/);
  assert.match(
    buildCommand,
    /if app_route_roots\.is_empty\(\) \{\s*remove_stale_app_route_discovery_summary\(input\.output_dir\)\?;\s*return Ok\(DxAppRouterBuildCommandOutput::default\(\)\);/s,
  );
  assert.match(buildCommand, /fn remove_stale_app_route_discovery_summary\(output_dir: &Path\) -> DxResult<\(\)>/);
  assert.match(buildCommand, /std::fs::remove_file\(&path\)/);
  assert.match(buildCommand, /std::io::ErrorKind::NotFound => Ok\(\(\)\)/);
  assert.match(
    buildCommand,
    /let route_summaries = app_page_routes::discover_page_route_summaries\(input\.cwd\);/,
  );
  assert.match(
    buildCommand,
    /let skipped_route_summaries =\s+app_page_route_diagnostics::discover_skipped_page_route_summaries\(input\.cwd\);/,
  );
  assert.match(
    buildCommand,
    /write_app_route_discovery_summary\(\s*input\.output_dir,\s*&route_summaries,\s*&skipped_route_summaries,\s*\)\?;/,
  );
  assert.match(buildCommand, /"schema": "dx\.app-router\.route-discovery"/);
  assert.match(buildCommand, /"format": 1/);
  assert.match(buildCommand, /"routes": route_summaries/);
  assert.match(buildCommand, /"skipped_routes": skipped_routes/);
  assert.match(buildCommand, /UnsupportedAppRouteSegmentReason::DuplicateParamName => "duplicate-param-name"/);
  assert.match(buildCommand, /UnsupportedAppRouteSegmentReason::NonTerminalCatchAll => "non-terminal-catch-all"/);

  assert.match(pageRoutes, /use serde::Serialize;/);
  assert.match(pageRoutes, /derive\([^)]*Serialize[^)]*\)\][\s\r\n]+pub\(super\) struct AppDiscoveredRouteSummary/);
  assert.match(pageRoutes, /derive\([^)]*Serialize[^)]*\)\][\s\r\n]+pub\(super\) enum AppDiscoveredSegmentKind/);
  assert.match(pageRoutes, /route_segments_have_duplicate_param_names\(&segments\)/);
  assert.match(pageRoutes, /route_segments_have_nonterminal_catch_all\(&segments\)/);
  assert.match(pageRoutes, /is_malformed_app_route_parameter_segment\(segment\)/);

  assert.match(diagnostics, /pub\(super\) fn discover_skipped_page_route_summaries/);
});
