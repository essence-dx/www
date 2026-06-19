const fs = require("fs");
const path = require("path");
const zlib = require("zlib");

const reportDir = path.join(__dirname, "reports");
const defaultRouteComparisonPath = path.join(reportDir, "forge-public-route-comparison.json");
const outJsonPath = path.join(reportDir, "forge-static-competitor-evidence.json");
const outMdPath = path.join(reportDir, "forge-static-competitor-evidence.md");

const requiredRoutes = [
  "/forge",
  "/forge/scorecard",
  "/forge/ci",
  "/forge/evidence",
  "/forge/releases",
  "/forge/changelog",
  "/forge/adoption",
];

const competitorProfiles = [
  {
    framework: "Astro",
    baseline_id: "astro-static-html-floor",
    baseline_kind: "static-floor",
    runtime_claim: "no hydrated runtime in this fixture",
    note:
      "A generous Astro-style static HTML floor. This does not run astro build and should not be presented as an Astro framework benchmark.",
  },
  {
    framework: "Svelte",
    baseline_id: "svelte-prerendered-static-floor",
    baseline_kind: "static-floor",
    runtime_claim: "no hydrated runtime in this fixture",
    note:
      "A generous Svelte prerender/static floor. This excludes the normal Vite/Svelte client bundle and is not a SvelteKit or CSR measurement.",
  },
  {
    framework: "HTMX",
    baseline_id: "htmx-static-html-floor",
    baseline_kind: "static-floor",
    runtime_claim: "no htmx runtime in this fixture",
    note:
      "A generous HTMX-style static HTML floor. This excludes htmx.js and server behavior, so it is not a production HTMX app benchmark.",
  },
  {
    framework: "Next.js",
    baseline_id: "next-static-export-floor",
    baseline_kind: "static-floor",
    runtime_claim: "no React, RSC, or router runtime in this fixture",
    note:
      "A generous Next.js static-export floor. This excludes next start, React, RSC, image/font/runtime assets, and is not a production Next.js app benchmark.",
  },
];

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

function buildReport(options = {}) {
  const generatedAt = options.generatedAt || new Date().toISOString();
  const routeComparison =
    options.routeComparison || readJson(options.routeComparisonPath || defaultRouteComparisonPath);
  const routes = Array.isArray(routeComparison.routes) ? routeComparison.routes : [];
  const missingRoutes = requiredRoutes.filter(
    (requiredRoute) => !routes.some((route) => route.route === requiredRoute && route.status === "measured")
  );

  if (missingRoutes.length > 0) {
    throw new Error(`Forge public route comparison is missing measured routes: ${missingRoutes.join(", ")}`);
  }

  const dxWwwRoutes = routes.map((route) => ({
    route: route.route,
    role: route.role,
    fixture_mode: route.fixture_mode,
    delivery: route.route_delivery || route.delivery || "unknown",
    decoded_bytes: route.decoded_bytes,
    brotli_bytes: route.brotli_bytes,
    http_route_median_ms: route.http_route_median_ms,
    chrome_load_event_ms: route.chrome_load_event_ms,
    budget_passed: route.budget_passed,
  }));

  const competitorFrameworks = competitorProfiles.map((profile) =>
    competitorFrameworkReport(profile, dxWwwRoutes)
  );
  const dxWwwFramework = {
    framework: "DX-WWW",
    baseline_id: "measured-forge-public-routes",
    baseline_kind: "measured-static-routes",
    route_count: dxWwwRoutes.length,
    total_decoded_bytes: sum(dxWwwRoutes, "decoded_bytes"),
    total_brotli_bytes: sum(dxWwwRoutes, "brotli_bytes"),
    routes: dxWwwRoutes,
    note:
      "Measured public Forge compiler output from forge-public-route-comparison.json. It includes real launch evidence, claims, proof links, and route-specific content.",
  };

  const frameworks = [dxWwwFramework, ...competitorFrameworks];
  const routeComparisons = dxWwwRoutes.map((route) => routeComparisonRow(route, competitorFrameworks));
  const adoptionRouteBrowserBenchmark = adoptionRouteBrowserComparison(dxWwwRoutes, competitorFrameworks);

  return {
    generated_at: generatedAt,
    source_route_comparison: "benchmarks/reports/forge-public-route-comparison.json",
    scope: {
      community_excluded: true,
      not_full_framework_benchmark: true,
      competitor_builds_not_run: true,
      no_package_install: true,
      no_node_modules_created: true,
      content_parity:
        "Competitor rows are summary-level static floors for the same public route roles, not byte-identical recreations of the full DX-WWW evidence pages.",
      safe_public_claim:
        "This fixture checks whether DX-WWW public Forge routes stay reasonably small against generous static HTML floors. It does not prove broad framework replacement.",
    },
    required_routes: requiredRoutes,
    frameworks,
    route_comparisons: routeComparisons,
    adoption_route_browser_benchmark: adoptionRouteBrowserBenchmark,
    honest_findings: [
      "Minimal static HTML floors can be smaller than DX-WWW when they omit Forge evidence, claims, receipts, proof metadata, and generated public review copy.",
      "Astro, Svelte, HTMX, and Next.js rows in this report are deliberately generous static floors; they are not live framework builds, dev-server timings, or production app measurements.",
      "DX-WWW should only claim this public surface is static, measured, source-owned, and no-runtime; broader framework wins require separate real app suites.",
      "Use real-route and framework scorecard reports for broader comparisons, and keep this fixture scoped to public Forge launch evidence routes.",
    ],
    conclusion:
      "DX-WWW public Forge routes are verified static/no-runtime compiler outputs. This fixture adds an intentionally conservative competitor floor so launch copy stays honest when a plain static page could be smaller.",
  };
}

function adoptionRouteBrowserComparison(dxWwwRoutes, competitorFrameworks) {
  const route = dxWwwRoutes.find((candidate) => candidate.route === "/forge/adoption");
  if (!route) {
    throw new Error("Forge public route comparison is missing /forge/adoption for adoption browser benchmark");
  }

  const staticFloors = competitorFrameworks.map((framework) => {
    const floor = framework.routes.find((candidate) => candidate.route === "/forge/adoption");
    return {
      framework: framework.framework,
      baseline_id: framework.baseline_id,
      decoded_bytes: floor.decoded_bytes,
      brotli_bytes: floor.brotli_bytes,
      resource_count: floor.resource_count,
      delivery: floor.delivery,
      runtime_asset_written: floor.runtime_asset_written,
      chrome_load_event_ms: null,
      browser_timing_source: "not-run deterministic static-floor fixture",
    };
  });
  const winner = [
    { framework: "DX-WWW", brotli_bytes: route.brotli_bytes },
    ...staticFloors.map((floor) => ({
      framework: floor.framework,
      brotli_bytes: floor.brotli_bytes,
    })),
  ].sort((left, right) => left.brotli_bytes - right.brotli_bytes)[0];

  return {
    route: "/forge/adoption",
    source_route_comparison: "benchmarks/reports/forge-public-route-comparison.json",
    no_package_install: true,
    competitor_builds_run: false,
    browser_scope:
      "DX-WWW browser timing comes from the measured /forge/adoption route; competitor rows are deterministic static-floor payload fixtures and do not run package installs.",
    dxwww: {
      decoded_bytes: route.decoded_bytes,
      brotli_bytes: route.brotli_bytes,
      http_route_median_ms: route.http_route_median_ms,
      chrome_load_event_ms: route.chrome_load_event_ms,
      delivery: route.delivery,
      budget_passed: route.budget_passed,
    },
    static_floors: staticFloors,
    winner_by_brotli: winner,
    caveat:
      "Static floors can be smaller because they omit adoption report copy, claims manifests, DXPK proof artifacts, source-owned package evidence, and reviewer context.",
  };
}

function competitorFrameworkReport(profile, dxWwwRoutes) {
  const routes = dxWwwRoutes.map((route) => competitorRoute(profile, route));
  return {
    framework: profile.framework,
    baseline_id: profile.baseline_id,
    baseline_kind: profile.baseline_kind,
    runtime_claim: profile.runtime_claim,
    route_count: routes.length,
    total_decoded_bytes: sum(routes, "decoded_bytes"),
    total_brotli_bytes: sum(routes, "brotli_bytes"),
    routes,
    note: profile.note,
  };
}

function competitorRoute(profile, dxRoute) {
  const html = renderStaticFloorHtml(profile, dxRoute);
  const raw = Buffer.from(html);
  return {
    route: dxRoute.route,
    role: dxRoute.role,
    fixture_mode: dxRoute.fixture_mode,
    decoded_bytes: raw.length,
    brotli_bytes: brotliSize(raw),
    resource_count: 1,
    delivery: "static",
    runtime_asset_written: false,
    content_parity: "summary-level route-role fixture",
  };
}

function renderStaticFloorHtml(profile, route) {
  const title = `${profile.framework} static floor for ${route.route}`;
  return [
    "<!doctype html>",
    '<html lang="en">',
    "<head>",
    '<meta charset="utf-8">',
    '<meta name="viewport" content="width=device-width,initial-scale=1">',
    `<title>${escapeHtml(title)}</title>`,
    "<style>body{font-family:system-ui,sans-serif;margin:0;background:#fff;color:#111827}main{max-width:760px;margin:auto;padding:32px}h1{font-size:28px;margin:0 0 12px}p,li{line-height:1.5}.meta{border-top:1px solid #e5e7eb;margin-top:24px;padding-top:16px;color:#4b5563}</style>",
    "</head>",
    "<body>",
    "<main>",
    `<h1>${escapeHtml(profile.framework)} static floor</h1>`,
    `<p>Route: <strong>${escapeHtml(route.route)}</strong></p>`,
    `<p>Role: ${escapeHtml(route.role || "public Forge evidence")}</p>`,
    "<ul>",
    "<li>One static HTML resource.</li>",
    "<li>No hydrated runtime in this fixture.</li>",
    "<li>No package install or node_modules created by this comparison.</li>",
    "</ul>",
    `<p class="meta">Compared with DX-WWW fixture ${escapeHtml(route.fixture_mode)} at ${route.decoded_bytes} decoded bytes and ${route.brotli_bytes} Brotli-estimated bytes. This is a static floor, not a full framework benchmark.</p>`,
    "</main>",
    "</body>",
    "</html>",
  ].join("");
}

function routeComparisonRow(dxRoute, competitorFrameworks) {
  const competitors = competitorFrameworks.map((framework) => {
    const route = framework.routes.find((candidate) => candidate.route === dxRoute.route);
    return {
      framework: framework.framework,
      decoded_bytes: route.decoded_bytes,
      brotli_bytes: route.brotli_bytes,
      brotli_vs_dxwww_bytes: route.brotli_bytes - dxRoute.brotli_bytes,
      brotli_vs_dxwww_percent: percentDelta(dxRoute.brotli_bytes, route.brotli_bytes),
    };
  });
  const winner = [
    { framework: "DX-WWW", brotli_bytes: dxRoute.brotli_bytes },
    ...competitors.map((competitor) => ({
      framework: competitor.framework,
      brotli_bytes: competitor.brotli_bytes,
    })),
  ].sort((left, right) => left.brotli_bytes - right.brotli_bytes)[0];

  return {
    route: dxRoute.route,
    role: dxRoute.role,
    dxwww_brotli_bytes: dxRoute.brotli_bytes,
    winner,
    competitors,
    caveat:
      "Winner only applies to this static-floor fixture. It is not a claim about framework completeness, routing, data loading, hydration, auth, ecosystem, or production DX.",
  };
}

function renderMarkdown(report) {
  const adoptionBenchmarkLines = report.adoption_route_browser_benchmark
    ? renderAdoptionRouteBrowserBenchmarkMarkdown(report.adoption_route_browser_benchmark)
    : [];
  const lines = [
    "# Forge Static Competitor Evidence",
    "",
    `Generated: ${report.generated_at}`,
    `Source: \`${report.source_route_comparison}\``,
    "",
    "This is not a full framework benchmark and does not prove broad framework replacement.",
    "It compares measured DX-WWW public Forge static routes against generous static HTML floors for Astro, Svelte, HTMX, and Next.js.",
    "",
    "## Scope",
    "",
    `- Community/adoption excluded: \`${report.scope.community_excluded}\``,
    `- Competitor builds run: \`${!report.scope.competitor_builds_not_run}\``,
    `- Package installs run: \`${!report.scope.no_package_install}\``,
    `- Created node_modules: \`${!report.scope.no_node_modules_created}\``,
    `- Content parity: ${report.scope.content_parity}`,
    `- Safe public claim: ${report.scope.safe_public_claim}`,
    "",
    "## Totals",
    "",
    "| Framework | Baseline | Kind | Routes | Decoded | Brotli | Note |",
    "| --- | --- | --- | ---: | ---: | ---: | --- |",
    ...report.frameworks.map((framework) =>
      [
        framework.framework,
        framework.baseline_id,
        framework.baseline_kind,
        framework.route_count,
        `${framework.total_decoded_bytes} B`,
        `${framework.total_brotli_bytes} B`,
        framework.note,
      ].join(" | ")
    ).map((row) => `| ${row} |`),
    "",
    "## Route Comparison",
    "",
    "| Route | Role | DX-WWW Brotli | Astro floor | Svelte floor | HTMX floor | Next.js floor | Winner |",
    "| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |",
    ...report.route_comparisons.map((route) => {
      const astro = route.competitors.find((competitor) => competitor.framework === "Astro");
      const svelte = route.competitors.find((competitor) => competitor.framework === "Svelte");
      const htmx = route.competitors.find((competitor) => competitor.framework === "HTMX");
      const next = route.competitors.find((competitor) => competitor.framework === "Next.js");
      return [
        route.route,
        route.role,
        `${route.dxwww_brotli_bytes} B`,
        formatMaybeBytes(astro?.brotli_bytes),
        formatMaybeBytes(svelte?.brotli_bytes),
        formatMaybeBytes(htmx?.brotli_bytes),
        formatMaybeBytes(next?.brotli_bytes),
        `${route.winner.framework} (${route.winner.brotli_bytes} B)`,
      ].join(" | ");
    }).map((row) => `| ${row} |`),
    ...adoptionBenchmarkLines,
    "",
    "## Honest Findings",
    "",
    ...report.honest_findings.map((finding) => `- ${finding}`),
    "",
    report.conclusion,
    "",
  ];
  return lines.join("\n");
}

function renderAdoptionRouteBrowserBenchmarkMarkdown(benchmark) {
  return [
    "",
    "## Adoption Route Browser Benchmark",
    "",
    `Route: \`${benchmark.route}\``,
    `No package installs: \`${benchmark.no_package_install}\``,
    `Competitor builds run: \`${benchmark.competitor_builds_run}\``,
    "",
    "| Framework | Evidence | Decoded | Brotli | HTTP median | Browser load |",
    "| --- | --- | ---: | ---: | ---: | ---: |",
    `| DX-WWW | measured /forge/adoption | ${benchmark.dxwww.decoded_bytes} B | ${benchmark.dxwww.brotli_bytes} B | ${formatMaybeMs(benchmark.dxwww.http_route_median_ms)} | ${formatMaybeMs(benchmark.dxwww.chrome_load_event_ms)} |`,
    ...benchmark.static_floors.map((floor) =>
      `| ${floor.framework} | ${floor.baseline_id} | ${floor.decoded_bytes} B | ${floor.brotli_bytes} B | n/a | n/a |`
    ),
    "",
    benchmark.caveat,
  ];
}

function formatMaybeBytes(value) {
  return Number.isFinite(Number(value)) ? `${value} B` : "n/a";
}

function formatMaybeMs(value) {
  return Number.isFinite(Number(value)) ? `${value} ms` : "n/a";
}

function writeReport(report) {
  fs.mkdirSync(reportDir, { recursive: true });
  fs.writeFileSync(outJsonPath, `${JSON.stringify(report, null, 2)}\n`);
  fs.writeFileSync(outMdPath, renderMarkdown(report));
}

function brotliSize(buffer) {
  return zlib.brotliCompressSync(buffer, {
    params: {
      [zlib.constants.BROTLI_PARAM_QUALITY]: 11,
    },
  }).length;
}

function sum(rows, key) {
  return rows.reduce((total, row) => total + (Number(row[key]) || 0), 0);
}

function percentDelta(left, right) {
  if (!Number.isFinite(left) || left === 0 || !Number.isFinite(right)) {
    return null;
  }
  return Number((((right - left) / left) * 100).toFixed(1));
}

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

function main() {
  const report = buildReport();
  writeReport(report);
  console.log(
    JSON.stringify(
      {
        report: [outJsonPath, outMdPath],
        route_count: report.required_routes.length,
        dxwww_brotli_bytes: report.frameworks[0].total_brotli_bytes,
        competitor_floor_brotli_bytes: Object.fromEntries(
          report.frameworks.slice(1).map((framework) => [framework.framework, framework.total_brotli_bytes])
        ),
        adoption_route_browser_benchmark: report.adoption_route_browser_benchmark,
        safe_public_claim: report.scope.safe_public_claim,
      },
      null,
      2
    )
  );
}

if (require.main === module) {
  main();
} else {
  module.exports = {
    buildReport,
    adoptionRouteBrowserComparison,
    renderMarkdown,
    renderStaticFloorHtml,
  };
}
