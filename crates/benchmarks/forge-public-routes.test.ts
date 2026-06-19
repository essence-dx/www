const assert = require("node:assert/strict");
const test = require("node:test");

const {
  buildReport,
  publicRoutes,
  renderMarkdown,
} = require("./compare-forge-public-routes.ts");

function measuredSnapshot(fixtureMode, index) {
  return {
    fixture_mode: fixtureMode,
    generated_at: `2026-05-17T00:00:0${index}.000Z`,
    route_delivery: "static",
    runtime_asset_written: false,
    packet_artifact_written: true,
    http_resources: 1,
    forge_packages: fixtureMode === "forge-adoption" ? 3 : 0,
    forge_files_tracked: fixtureMode === "forge-adoption" ? 12 : 0,
    decoded_bytes: 3000 + index,
    brotli_bytes: 800 + index,
    http_route_median_ms: 1 + index / 10,
    chrome_load_event_ms: 8 + index,
    dx_packet_applied: false,
    interaction_works: false,
    budget_passed: true,
    markdown: `vertical-proof-history/${fixtureMode}.md`,
    json: `vertical-proof-history/${fixtureMode}.json`,
    full: {},
  };
}

test("public route comparison includes adoption evidence without overclaiming", () => {
  const history = publicRoutes.map((route, index) =>
    measuredSnapshot(route.fixture_mode, index)
  );
  const report = buildReport(history);
  const routes = report.routes.map((route) => route.route);
  const markdown = renderMarkdown(report);

  assert.equal(report.route_count, 7);
  assert.ok(routes.includes("/forge/adoption"));
  assert.equal(
    report.routes.find((route) => route.route === "/forge/adoption").role,
    "Adoption evidence"
  );
  assert.match(report.conclusion, /adoption evidence/i);
  assert.doesNotMatch(report.conclusion, /live customer/i);
  assert.doesNotMatch(report.conclusion, /production adoption/i);
  assert.match(markdown, /\| \/forge\/adoption \| Adoption evidence \| forge-adoption \|/);
});
