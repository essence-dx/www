const assert = require("assert");
const test = require("node:test");

const { buildReport, renderMarkdown } = require("./compare-forge-static-competitors.ts");

function measuredRoute(route, role, fixtureMode, decodedBytes, brotliBytes) {
  return {
    route,
    role,
    fixture_mode: fixtureMode,
    status: "measured",
    route_delivery: "static",
    decoded_bytes: decodedBytes,
    brotli_bytes: brotliBytes,
    http_route_median_ms: 1.5,
    chrome_load_event_ms: 10,
    budget_passed: true,
  };
}

test("static competitor evidence stays scoped and honest", () => {
  const routeComparison = {
    generated_at: "2026-05-17T00:00:00.000Z",
    route_count: 7,
    routes: [
      measuredRoute("/forge", "Launch evidence", "forge-site", 5100, 1200),
      measuredRoute("/forge/scorecard", "Package scorecard", "forge-scorecard", 4100, 1000),
      measuredRoute("/forge/ci", "CI evidence", "forge-ci", 3300, 850),
      measuredRoute("/forge/evidence", "Evidence index", "forge-evidence", 6900, 1100),
      measuredRoute("/forge/releases", "Release history", "forge-releases", 4500, 850),
      measuredRoute("/forge/changelog", "Launch changelog", "forge-changelog", 4500, 900),
      measuredRoute("/forge/adoption", "Adoption evidence", "forge-adoption", 7200, 1300),
    ],
  };

  const report = buildReport({
    generatedAt: "2026-05-17T00:00:00.000Z",
    routeComparison,
  });
  const markdown = renderMarkdown(report);

  assert.equal(report.scope.not_full_framework_benchmark, true);
  assert.equal(report.scope.competitor_builds_not_run, true);
  assert.equal(report.scope.no_package_install, true);
  assert.equal(report.scope.no_node_modules_created, true);
  assert.deepEqual(
    report.frameworks.map((framework) => framework.framework),
    ["DX-WWW", "Astro", "Svelte", "HTMX", "Next.js"]
  );
  assert.equal(report.route_comparisons.length, 7);
  assert.ok(report.required_routes.includes("/forge/adoption"));
  assert.equal(report.adoption_route_browser_benchmark.route, "/forge/adoption");
  assert.equal(report.adoption_route_browser_benchmark.no_package_install, true);
  assert.equal(report.adoption_route_browser_benchmark.dxwww.chrome_load_event_ms, 10);
  assert.deepEqual(
    report.adoption_route_browser_benchmark.static_floors.map((floor) => floor.framework),
    ["Astro", "Svelte", "HTMX", "Next.js"]
  );
  assert.ok(report.frameworks.slice(1).every((framework) => framework.baseline_kind === "static-floor"));
  assert.match(markdown, /not a full framework benchmark/);
  assert.match(markdown, /does not prove broad framework replacement/);
  assert.match(markdown, /Astro floor/);
  assert.match(markdown, /Svelte floor/);
  assert.match(markdown, /HTMX floor/);
  assert.match(markdown, /Next\.js floor/);
  assert.match(markdown, /\/forge\/changelog/);
  assert.match(markdown, /\/forge\/adoption/);
  assert.match(markdown, /Adoption Route Browser Benchmark/);
});

test("static competitor evidence fails when a required public route is missing", () => {
  assert.throws(
    () =>
      buildReport({
        routeComparison: {
          routes: [
            measuredRoute("/forge", "Launch evidence", "forge-site", 5100, 1200),
            measuredRoute("/forge/scorecard", "Package scorecard", "forge-scorecard", 4100, 1000),
          ],
        },
      }),
    /missing measured routes/
  );
});
