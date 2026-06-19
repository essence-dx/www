const assert = require("assert");
const test = require("node:test");

const {
  buildReport,
  largeContentModel,
  renderMarkdown,
} = require("./compare-forge-large-content.ts");

test("large content comparison stresses repeated sections and package metadata", () => {
  const report = buildReport({
    generatedAt: "2026-05-17T00:00:00.000Z",
  });
  const markdown = renderMarkdown(report);

  assert.equal(report.scope.not_full_framework_benchmark, true);
  assert.equal(report.scope.competitor_builds_not_run, true);
  assert.equal(report.scope.no_package_install, true);
  assert.equal(report.scope.no_node_modules_created, true);
  assert.equal(report.fixture.route, "/forge/large-content");
  assert.equal(report.fixture.sections, largeContentModel.sections.length);
  assert.ok(report.fixture.sections >= 8);
  assert.ok(report.fixture.repeated_items >= 96);
  assert.equal(report.fixture.package_metadata_count, 3);
  assert.equal(report.fixture.static_evidence, true);
  assert.ok(report.first_route_budget.max_brotli_bytes > 0);
  assert.ok(report.first_route_budget.dxwww_brotli_bytes > 0);
  assert.equal(typeof report.first_route_budget.passed, "boolean");
  assert.deepEqual(
    report.frameworks.map((framework) => framework.framework),
    ["DX-WWW", "Astro", "Svelte", "HTMX", "Next.js"]
  );
  assert.match(markdown, /large-content fixture/i);
  assert.match(markdown, /source-owned package metadata/);
  assert.match(markdown, /first-route payload budget/);
  assert.match(markdown, /not a full framework benchmark/);
});

test("large content comparison proves every row includes repeated content and packages", () => {
  const report = buildReport({
    generatedAt: "2026-05-17T00:00:00.000Z",
  });

  for (const framework of report.frameworks) {
    assert.equal(framework.static_evidence.route, "/forge/large-content");
    assert.ok(framework.static_evidence.html_bytes > 0);
    assert.ok(framework.static_evidence.brotli_bytes > 0);
    assert.ok(framework.static_evidence.contains_repeated_sections);
    assert.ok(framework.static_evidence.contains_package_metadata);
    assert.ok(framework.static_evidence.contains_budget_copy);
  }
});
