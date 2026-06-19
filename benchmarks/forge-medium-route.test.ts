const assert = require("assert");
const test = require("node:test");

const {
  buildReport,
  renderMarkdown,
  mediumPageModel,
} = require("./compare-forge-medium-route.ts");

test("medium route comparison covers real medium-page shape and honest scope", () => {
  const report = buildReport({
    generatedAt: "2026-05-17T00:00:00.000Z",
  });
  const markdown = renderMarkdown(report);

  assert.equal(report.scope.not_full_framework_benchmark, true);
  assert.equal(report.scope.competitor_builds_not_run, true);
  assert.equal(report.scope.no_package_install, true);
  assert.equal(report.scope.no_node_modules_created, true);
  assert.equal(report.fixture.route, "/forge/medium");
  assert.equal(report.fixture.cards, mediumPageModel.cards.length);
  assert.ok(report.fixture.cards >= 12);
  assert.ok(report.fixture.form_fields >= 6);
  assert.ok(report.fixture.route_links >= 8);
  assert.equal(report.fixture.static_evidence, true);
  assert.deepEqual(
    report.frameworks.map((framework) => framework.framework),
    ["DX-WWW", "Astro", "Svelte", "HTMX", "Next.js"]
  );
  assert.ok(
    report.frameworks.every((framework) => framework.route_count === 1),
    "each framework should describe the same single medium route"
  );
  assert.ok(report.rankings.brotli.length === report.frameworks.length);
  assert.match(markdown, /medium-route fixture/i);
  assert.match(markdown, /repeated cards/);
  assert.match(markdown, /form fields/);
  assert.match(markdown, /HTMX/);
  assert.match(markdown, /not a full framework benchmark/);
});

test("medium route comparison keeps static evidence linked to every framework row", () => {
  const report = buildReport({
    generatedAt: "2026-05-17T00:00:00.000Z",
  });

  for (const framework of report.frameworks) {
    assert.equal(framework.static_evidence.route, "/forge/medium");
    assert.ok(framework.static_evidence.html_bytes > 0);
    assert.ok(framework.static_evidence.brotli_bytes > 0);
    assert.ok(framework.static_evidence.contains_cards);
    assert.ok(framework.static_evidence.contains_form);
    assert.ok(framework.static_evidence.contains_route_links);
  }
});
