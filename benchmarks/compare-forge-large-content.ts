const crypto = require("crypto");
const fs = require("fs");
const path = require("path");
const zlib = require("zlib");

const reportDir = path.join(__dirname, "reports");
const outJsonPath = path.join(reportDir, "forge-large-content-comparison.json");
const outMdPath = path.join(reportDir, "forge-large-content-comparison.md");

const packageMetadata = [
  {
    id: "shadcn/ui/button",
    version: "0.1.0-dx",
    license: "MIT",
    files: ["components/ui/button.tsx", "lib/utils.ts", "styles/dx.css"],
    receipts: 4,
    risk: "green",
  },
  {
    id: "dx/icon/search",
    version: "0.1.0-dx",
    license: "MIT",
    files: ["components/icons/search.tsx", "components/icons/icon.tsx"],
    receipts: 3,
    risk: "green",
  },
  {
    id: "auth/better-auth",
    version: "0.1.0-dx",
    license: "MIT",
    files: ["lib/auth/better-auth.ts", "routes/auth/better-auth/start.ts", "routes/auth/better-auth/callback.ts"],
    receipts: 5,
    risk: "yellow-review",
  },
];

const largeContentModel = {
  route: "/forge/large-content",
  title: "DX Forge large content evidence",
  summary:
    "A large static route stressing repeated release sections, source-owned package metadata, and first-route payload budgets.",
  packageMetadata,
  sections: Array.from({ length: 8 }, (_, sectionIndex) => ({
    title: `Release evidence section ${sectionIndex + 1}`,
    eyebrow: ["packages", "receipts", "routes", "budgets"][sectionIndex % 4],
    rows: Array.from({ length: 12 }, (_, rowIndex) => ({
      title: `Evidence row ${sectionIndex + 1}.${rowIndex + 1}`,
      package: packageMetadata[(sectionIndex + rowIndex) % packageMetadata.length].id,
      status: ["verified", "reviewed", "tracked", "budgeted"][(sectionIndex + rowIndex) % 4],
      text:
        "Source-owned package files stay editable, receipt-backed, and measurable without installing node_modules.",
    })),
  })),
};

const profiles = [
  {
    framework: "DX-WWW",
    baseline_id: "dx-www-large-static-route",
    baseline_kind: "generated-static-evidence",
    runtime_claim: "static first route with package metadata and no route runtime",
    note:
      "DX-WWW large-content fixture keeps source-owned package metadata in the route and records a first-route budget.",
    render: renderDxWwwHtml,
  },
  {
    framework: "Astro",
    baseline_id: "astro-large-static-floor",
    baseline_kind: "static-floor",
    runtime_claim: "no Astro island hydration included",
    note:
      "Generous Astro-style large static floor for the same content model. No astro build was run.",
    render: (model) => renderFrameworkHtml(model, "Astro", "astro-large-floor"),
  },
  {
    framework: "Svelte",
    baseline_id: "svelte-large-prerender-floor",
    baseline_kind: "static-floor",
    runtime_claim: "no Svelte client runtime included",
    note:
      "Generous Svelte prerender floor for the same content model. No SvelteKit build was run.",
    render: (model) => renderFrameworkHtml(model, "Svelte", "svelte-large-floor"),
  },
  {
    framework: "HTMX",
    baseline_id: "htmx-large-static-floor",
    baseline_kind: "static-floor-with-htmx-attributes",
    runtime_claim: "htmx-shaped attributes included; external runtime bytes not fetched",
    note:
      "HTMX-shaped large static fixture for the same content model. No external htmx runtime was fetched.",
    render: renderHtmxHtml,
  },
  {
    framework: "Next.js",
    baseline_id: "next-large-static-export-floor",
    baseline_kind: "static-floor",
    runtime_claim: "no React, RSC, router, font, image, or prefetch runtime included",
    note:
      "Generous Next.js static export floor for the same content model. No next build was run.",
    render: (model) => renderFrameworkHtml(model, "Next.js", "next-large-floor"),
  },
];

const largeRouteBudget = {
  max_decoded_bytes: 64_000,
  max_brotli_bytes: 9_000,
  max_package_metadata_bytes: 3_000,
};

function buildReport(options = {}) {
  const generatedAt = options.generatedAt || new Date().toISOString();
  const model = options.model || largeContentModel;
  const frameworks = profiles.map((profile) => frameworkReport(profile, model));
  const dxWww = frameworks.find((framework) => framework.framework === "DX-WWW");
  const packageMetadataBytes = Buffer.byteLength(JSON.stringify(model.packageMetadata));
  const firstRouteBudget = {
    ...largeRouteBudget,
    dxwww_decoded_bytes: dxWww.total_decoded_bytes,
    dxwww_brotli_bytes: dxWww.total_brotli_bytes,
    package_metadata_bytes: packageMetadataBytes,
    passed:
      dxWww.total_decoded_bytes <= largeRouteBudget.max_decoded_bytes &&
      dxWww.total_brotli_bytes <= largeRouteBudget.max_brotli_bytes &&
      packageMetadataBytes <= largeRouteBudget.max_package_metadata_bytes,
    caveat:
      "Budget applies only to this deterministic large static route fixture, not to arbitrary application pages.",
  };

  return {
    generated_at: generatedAt,
    report_id: "forge-large-content-comparison-v1",
    source: "benchmarks/compare-forge-large-content.ts",
    fixture: {
      route: model.route,
      sections: model.sections.length,
      rows_per_section: model.sections[0]?.rows.length || 0,
      repeated_items: model.sections.reduce((total, section) => total + section.rows.length, 0),
      package_metadata_count: model.packageMetadata.length,
      package_file_count: model.packageMetadata.reduce(
        (total, packageInfo) => total + packageInfo.files.length,
        0
      ),
      static_evidence: true,
      content_parity:
        "Every framework row renders the same repeated release sections and the same source-owned package metadata.",
    },
    scope: {
      community_excluded: true,
      not_full_framework_benchmark: true,
      competitor_builds_not_run: true,
      no_package_install: true,
      no_node_modules_created: true,
      browser_timings_not_measured: true,
      safe_public_claim:
        "This large-content fixture checks static payload direction and first-route budget pressure. It does not prove broad framework replacement.",
    },
    first_route_budget: firstRouteBudget,
    frameworks,
    rankings: {
      brotli: rankFrameworks(frameworks, "total_brotli_bytes"),
      decoded: rankFrameworks(frameworks, "total_decoded_bytes"),
    },
    honest_findings: [
      "Large repeated static content compresses well in every framework row, so small differences here are not a universal framework verdict.",
      "Competitor rows remain generous static floors and exclude normal framework runtime, hydration, router, image, font, and data-loading overhead.",
      "DX-WWW should treat this as a first-route budget stress test for package metadata and repeated content, not as proof of market victory.",
      "The next credible benchmark step is a real build suite that renders this same model in each framework and measures browser navigation.",
    ],
    conclusion:
      "DX-WWW now has a large-content stress fixture with repeated sections and source-owned package metadata. The budget result is useful only inside this scoped static fixture.",
  };
}

function frameworkReport(profile, model) {
  const html = profile.render(model);
  const htmlBuffer = Buffer.from(html);
  return {
    framework: profile.framework,
    baseline_id: profile.baseline_id,
    baseline_kind: profile.baseline_kind,
    runtime_claim: profile.runtime_claim,
    route_count: 1,
    total_decoded_bytes: htmlBuffer.length,
    total_brotli_bytes: brotliSize(htmlBuffer),
    package_metadata_bytes: Buffer.byteLength(JSON.stringify(model.packageMetadata)),
    static_evidence: {
      route: model.route,
      html_bytes: htmlBuffer.length,
      brotli_bytes: brotliSize(htmlBuffer),
      sha256: sha256(htmlBuffer),
      contains_repeated_sections: model.sections.every((section) => html.includes(section.title)),
      contains_package_metadata: model.packageMetadata.every((packageInfo) =>
        html.includes(packageInfo.id)
      ),
      contains_budget_copy: html.includes("first-route payload budget"),
    },
    note: profile.note,
  };
}

function renderDxWwwHtml(model) {
  return htmlShell(model, "DX-WWW", "dx-large-content", {
    bodyAttrs: 'data-dx-route="large-content" data-dx-delivery="static"',
    sectionAttrs: (index) => `data-dx-section="${index}"`,
    rowAttrs: (sectionIndex, rowIndex) =>
      `data-dx-row="${sectionIndex}-${rowIndex}" data-source-owned="true"`,
  });
}

function renderFrameworkHtml(model, framework, marker) {
  return htmlShell(model, framework, marker, {
    bodyAttrs: `data-framework="${escapeAttr(marker)}"`,
    sectionAttrs: (index) => `data-section="${index}"`,
    rowAttrs: (sectionIndex, rowIndex) => `data-row="${sectionIndex}-${rowIndex}"`,
  });
}

function renderHtmxHtml(model) {
  return htmlShell(model, "HTMX", "htmx-large-content", {
    bodyAttrs: 'data-framework="htmx" hx-boost="true"',
    sectionAttrs: (index) => `data-section="${index}" hx-get="/forge/evidence" hx-trigger="revealed once"`,
    rowAttrs: (sectionIndex, rowIndex) =>
      `data-row="${sectionIndex}-${rowIndex}" hx-target="closest article"`,
  });
}

function htmlShell(model, framework, marker, attrs) {
  return [
    "<!doctype html>",
    '<html lang="en">',
    "<head>",
    '<meta charset="utf-8">',
    '<meta name="viewport" content="width=device-width,initial-scale=1">',
    `<title>${escapeHtml(framework)} large-content fixture</title>`,
    "<style>",
    "body{margin:0;font-family:Inter,ui-sans-serif,system-ui,sans-serif;background:#f8fafc;color:#111827}",
    "main{max-width:1180px;margin:auto;padding:28px}.hero{padding-bottom:18px;border-bottom:1px solid #e5e7eb}",
    ".packages,.sections{display:grid;gap:14px}.packages{grid-template-columns:repeat(3,minmax(0,1fr));margin:18px 0}",
    ".pkg,.section,.row{border:1px solid #e5e7eb;background:white;border-radius:8px;padding:12px}",
    ".section{margin-top:14px}.rows{display:grid;grid-template-columns:repeat(3,minmax(0,1fr));gap:10px}",
    ".row h3,.pkg h2{font-size:15px;margin:0 0 6px}.meta{font-size:13px;color:#475569}.budget{margin:18px 0;padding:12px;border:1px solid #cbd5e1;background:#fff}",
    "@media(max-width:820px){main{padding:18px}.packages,.rows{grid-template-columns:1fr}}",
    "</style>",
    "</head>",
    `<body ${attrs.bodyAttrs}>`,
    `<main id="${escapeAttr(marker)}">`,
    '<section class="hero">',
    `<p class="meta">large-content fixture</p>`,
    `<h1>${escapeHtml(model.title)}</h1>`,
    `<p>${escapeHtml(model.summary)}</p>`,
    "</section>",
    '<section class="budget" aria-label="First-route payload budget">',
    "<h2>first-route payload budget</h2>",
    "<p>This fixture keeps package metadata in the first route and checks decoded, Brotli, and metadata bytes against explicit thresholds.</p>",
    "</section>",
    '<section class="packages" aria-label="Source-owned package metadata">',
    ...model.packageMetadata.map(
      (packageInfo) =>
        `<article class="pkg"><h2>${escapeHtml(packageInfo.id)}</h2><p class="meta">version ${escapeHtml(packageInfo.version)} - ${escapeHtml(packageInfo.license)} - risk ${escapeHtml(packageInfo.risk)}</p><p>Files: ${escapeHtml(packageInfo.files.join(", "))}</p><p>Receipts: ${packageInfo.receipts}</p></article>`
    ),
    "</section>",
    '<section class="sections" aria-label="Repeated release proof sections">',
    ...model.sections.map(
      (section, sectionIndex) =>
        `<article class="section" ${attrs.sectionAttrs(sectionIndex)}><p class="meta">${escapeHtml(section.eyebrow)}</p><h2>${escapeHtml(section.title)}</h2><div class="rows">${section.rows
          .map(
            (row, rowIndex) =>
              `<article class="row" ${attrs.rowAttrs(sectionIndex, rowIndex)}><h3>${escapeHtml(row.title)}</h3><p class="meta">${escapeHtml(row.package)} - ${escapeHtml(row.status)}</p><p>${escapeHtml(row.text)}</p></article>`
          )
          .join("")}</div></article>`
    ),
    "</section>",
    "</main>",
    "</body>",
    "</html>",
  ].join("");
}

function renderMarkdown(report) {
  const lines = [
    "# Forge Large Content Comparison",
    "",
    `Generated: ${report.generated_at}`,
    `Fixture route: \`${report.fixture.route}\``,
    "",
    "This large-content fixture is not a full framework benchmark.",
    "It stresses repeated sections, source-owned package metadata, and first-route payload budget pressure.",
    "",
    "## Fixture Shape",
    "",
    `- Sections: \`${report.fixture.sections}\``,
    `- Rows per section: \`${report.fixture.rows_per_section}\``,
    `- Repeated items: \`${report.fixture.repeated_items}\``,
    `- Source-owned package metadata entries: \`${report.fixture.package_metadata_count}\``,
    `- Package files represented: \`${report.fixture.package_file_count}\``,
    `- Content parity: ${report.fixture.content_parity}`,
    "",
    "## First-Route Payload Budget",
    "",
    `- Passed: \`${report.first_route_budget.passed}\``,
    `- DX-WWW decoded: \`${report.first_route_budget.dxwww_decoded_bytes}\` B / max \`${report.first_route_budget.max_decoded_bytes}\` B`,
    `- DX-WWW Brotli: \`${report.first_route_budget.dxwww_brotli_bytes}\` B / max \`${report.first_route_budget.max_brotli_bytes}\` B`,
    `- Package metadata: \`${report.first_route_budget.package_metadata_bytes}\` B / max \`${report.first_route_budget.max_package_metadata_bytes}\` B`,
    `- Caveat: ${report.first_route_budget.caveat}`,
    "",
    "## Scope",
    "",
    `- Competitor builds run: \`${!report.scope.competitor_builds_not_run}\``,
    `- Package installs run: \`${!report.scope.no_package_install}\``,
    `- Created node_modules: \`${!report.scope.no_node_modules_created}\``,
    `- Browser timings measured: \`${!report.scope.browser_timings_not_measured}\``,
    `- Safe public claim: ${report.scope.safe_public_claim}`,
    "",
    "## Framework Rows",
    "",
    "| Framework | Baseline | Kind | Decoded | Brotli | Metadata | Runtime claim |",
    "| --- | --- | --- | ---: | ---: | ---: | --- |",
    ...report.frameworks.map((framework) =>
      [
        framework.framework,
        framework.baseline_id,
        framework.baseline_kind,
        `${framework.total_decoded_bytes} B`,
        `${framework.total_brotli_bytes} B`,
        `${framework.package_metadata_bytes} B`,
        framework.runtime_claim,
      ].join(" | ")
    ).map((row) => `| ${row} |`),
    "",
    "## Brotli Ranking",
    "",
    ...report.rankings.brotli.map(
      (row) => `${row.rank}. ${row.framework}: ${row.bytes} B`
    ),
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

function writeReport(report) {
  fs.mkdirSync(reportDir, { recursive: true });
  fs.writeFileSync(outJsonPath, `${JSON.stringify(report, null, 2)}\n`);
  fs.writeFileSync(outMdPath, renderMarkdown(report));
}

function rankFrameworks(frameworks, key) {
  return frameworks
    .map((framework) => ({
      framework: framework.framework,
      bytes: framework[key],
    }))
    .sort((left, right) => left.bytes - right.bytes)
    .map((row, index) => ({
      rank: index + 1,
      ...row,
    }));
}

function brotliSize(buffer) {
  return zlib.brotliCompressSync(buffer, {
    params: {
      [zlib.constants.BROTLI_PARAM_QUALITY]: 11,
    },
  }).length;
}

function sha256(buffer) {
  return crypto.createHash("sha256").update(buffer).digest("hex");
}

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

function escapeAttr(value) {
  return escapeHtml(value).replaceAll("'", "&#39;");
}

function main() {
  const report = buildReport();
  writeReport(report);
  console.log(
    JSON.stringify(
      {
        report: [outJsonPath, outMdPath],
        route: report.fixture.route,
        repeated_items: report.fixture.repeated_items,
        package_metadata_count: report.fixture.package_metadata_count,
        first_route_budget: report.first_route_budget,
        brotli_ranking: report.rankings.brotli,
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
    largeContentModel,
    renderMarkdown,
    renderDxWwwHtml,
    renderFrameworkHtml,
    renderHtmxHtml,
  };
}
