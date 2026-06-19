const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("App Router invalid page-route segments carry explicit discovery diagnostics", () => {
  const classifier = read("dx-www/src/app_router_segments.rs");
  const cliRoutes = read("dx-www/src/cli/app_page_routes.rs");
  const cliRouteDiagnostics = read("dx-www/src/cli/app_page_route_diagnostics.rs");
  const cliMod = read("dx-www/src/cli/mod.rs");
  const discovery = read("dx-www/src/build/source_engine/discovery.rs");

  assert.match(classifier, /pub\(crate\) enum UnsupportedAppRouteSegmentReason/);
  assert.match(classifier, /MalformedSegment/);
  assert.match(classifier, /DuplicateParamName/);
  assert.match(classifier, /NonTerminalCatchAll/);
  assert.match(classifier, /pub\(crate\) fn unsupported_app_page_route_segment/);
  assert.match(classifier, /reports_unsupported_app_page_route_segment_reasons/);

  assert.match(cliMod, /mod app_page_route_diagnostics;/);
  assert.doesNotMatch(cliRoutes, /pub\(super\) struct AppSkippedPageRouteSummary/);
  assert.doesNotMatch(cliRoutes, /pub\(super\) fn discover_skipped_page_route_summaries/);
  assert.match(cliRoutes, /pub\(super\) fn raw_page_route_segments_from_source_path/);
  assert.match(cliRouteDiagnostics, /pub\(super\) struct AppSkippedPageRouteSummary/);
  assert.match(cliRouteDiagnostics, /pub\(super\) fn discover_skipped_page_route_summaries/);
  assert.match(cliRouteDiagnostics, /discover_page_routes_reports_skipped_route_diagnostics/);
  assert.match(cliRouteDiagnostics, /UnsupportedAppRouteSegmentReason::MalformedSegment/);
  assert.match(cliRouteDiagnostics, /UnsupportedAppRouteSegmentReason::DuplicateParamName/);
  assert.match(cliRouteDiagnostics, /UnsupportedAppRouteSegmentReason::NonTerminalCatchAll/);

  assert.match(discovery, /pub struct SourceSkippedAppRoute/);
  assert.match(discovery, /pub skipped_routes: Vec<SourceSkippedAppRoute>/);
  assert.match(discovery, /inputs\s*\.skipped_routes\s*\.sort_by/);
  assert.match(discovery, /source_discovery_reports_skipped_unsupported_app_page_route_diagnostics/);
  assert.match(discovery, /UnsupportedAppRouteSegmentReason::MalformedSegment/);
  assert.match(discovery, /UnsupportedAppRouteSegmentReason::DuplicateParamName/);
  assert.match(discovery, /UnsupportedAppRouteSegmentReason::NonTerminalCatchAll/);

  assert.doesNotMatch(
    [classifier, cliRoutes, cliRouteDiagnostics, discovery].join("\n"),
    /Next DevTools|Turbopack powers|full Next\.js parity/,
  );
});
