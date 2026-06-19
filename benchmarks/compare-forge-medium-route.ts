const crypto = require("crypto");
const fs = require("fs");
const path = require("path");
const zlib = require("zlib");

const reportDir = path.join(__dirname, "reports");
const outJsonPath = path.join(reportDir, "forge-medium-route-comparison.json");
const outMdPath = path.join(reportDir, "forge-medium-route-comparison.md");

const mediumPageModel = {
  route: "/forge/medium",
  title: "DX Forge medium route evidence",
  eyebrow: "Source-owned package adoption",
  summary:
    "A medium static route with repeated cards, route navigation, a review form, and public evidence links.",
  stats: [
    ["Packages", "3"],
    ["Receipts", "12"],
    ["Routes", "6"],
    ["Runtime", "0"],
  ],
  navLinks: [
    ["/forge", "Launch"],
    ["/forge/scorecard", "Scorecard"],
    ["/forge/ci", "CI"],
    ["/forge/evidence", "Evidence"],
    ["/forge/releases", "Releases"],
    ["/forge/changelog", "Changelog"],
    ["/docs/forge-public-launch-handoff", "Handoff"],
    ["/benchmarks/forge-medium", "Medium benchmark"],
  ],
  cards: [
    ["shadcn/ui/button", "Source-owned UI package with editable local files and receipt coverage."],
    ["dx/icon/search", "Selected icon package proving one-icon materialization instead of bulk icons."],
    ["auth/better-auth", "OAuth starter package with env docs and source-owned route handlers."],
    ["Launch bundle", "Verified six-route public bundle with BLAKE3 artifact manifest."],
    ["Release dashboard", "Single gate for CI artifacts, public evidence, route budgets, and changelog."],
    ["Release history", "Reviewed route-comparison records with payload deltas and regression findings."],
    ["Public changelog", "Generated launch notes from release history without adoption overclaims."],
    ["Adoption smoke", "Clean temp app proof that public Forge routes build without node_modules."],
    ["Package docs", "Local .dx/forge docs explain source ownership and update boundaries."],
    ["Rollback receipts", "Restorable package files with reviewable before and after hashes."],
    ["Registry integrity", "Local registry manifests verify package file hashes before materialization."],
    ["Credibility evidence", "Benchmarks state limitations instead of claiming universal framework replacement."],
  ],
  formFields: [
    ["project", "Project path"],
    ["package", "Package id"],
    ["variant", "Variant"],
    ["reviewer", "Reviewer"],
    ["risk", "Risk note"],
    ["decision", "Decision"],
  ],
};

const profiles = [
  {
    framework: "DX-WWW",
    baseline_id: "dx-www-medium-static-route",
    baseline_kind: "generated-static-evidence",
    runtime_claim: "static first route with DXPK proof artifact, no route runtime",
    note:
      "DX-WWW fixture renders the medium route as source-owned static evidence plus a non-blocking proof packet model.",
    render: renderDxWwwHtml,
    packet: true,
  },
  {
    framework: "Astro",
    baseline_id: "astro-medium-static-floor",
    baseline_kind: "static-floor",
    runtime_claim: "no island hydration included in this fixture",
    note:
      "Generous Astro-style static floor for the same medium content. No astro build was run.",
    render: (model) => renderFrameworkHtml(model, "Astro", "astro-island-free"),
  },
  {
    framework: "Svelte",
    baseline_id: "svelte-medium-prerender-floor",
    baseline_kind: "static-floor",
    runtime_claim: "no Svelte client bundle included in this fixture",
    note:
      "Generous Svelte prerender floor for the same medium content. No Vite or SvelteKit build was run.",
    render: (model) => renderFrameworkHtml(model, "Svelte", "svelte-prerender"),
  },
  {
    framework: "HTMX",
    baseline_id: "htmx-medium-static-floor",
    baseline_kind: "static-floor-with-htmx-attributes",
    runtime_claim: "HTML carries htmx-style attributes; external htmx runtime bytes are not fetched",
    note:
      "HTMX-shaped static fixture for the same medium content. It does not fetch htmx.org runtime code.",
    render: renderHtmxHtml,
  },
  {
    framework: "Next.js",
    baseline_id: "next-medium-static-export-floor",
    baseline_kind: "static-floor",
    runtime_claim: "no React, RSC, router, font, image, or prefetch runtime included",
    note:
      "Generous Next.js static-export floor for the same medium content. No next build was run.",
    render: (model) => renderFrameworkHtml(model, "Next.js", "next-static-export"),
  },
];

function buildReport(options = {}) {
  const generatedAt = options.generatedAt || new Date().toISOString();
  const model = options.model || mediumPageModel;
  const frameworks = profiles.map((profile) => frameworkReport(profile, model));
  const rankings = {
    brotli: rankFrameworks(frameworks, "total_brotli_bytes"),
    decoded: rankFrameworks(frameworks, "total_decoded_bytes"),
  };

  return {
    generated_at: generatedAt,
    report_id: "forge-medium-route-comparison-v1",
    source: "benchmarks/compare-forge-medium-route.ts",
    fixture: {
      route: model.route,
      cards: model.cards.length,
      form_fields: model.formFields.length,
      route_links: model.navLinks.length,
      stat_blocks: model.stats.length,
      static_evidence: true,
      content_parity:
        "Every framework row renders the same medium-page model: repeated cards, route links, stats, and review form fields.",
    },
    scope: {
      community_excluded: true,
      not_full_framework_benchmark: true,
      competitor_builds_not_run: true,
      no_package_install: true,
      no_node_modules_created: true,
      browser_timings_not_measured: true,
      safe_public_claim:
        "This medium fixture compares deterministic static route payloads for the same content shape. It does not prove broad framework replacement.",
    },
    frameworks,
    rankings,
    honest_findings: [
      "This is a medium route fixture, not a real Astro/Svelte/HTMX/Next production build.",
      "The competitor rows are intentionally generous static floors and exclude common framework runtime, router, devtool, image, font, and hydration overhead.",
      "DX-WWW should use this evidence to find payload direction, not to claim it beats every framework in every scenario.",
      "A future browser suite must add real framework builds, real route navigation, hydration or interaction tests, and production CDN transfer measurements.",
    ],
    conclusion:
      "DX-WWW now has a reproducible medium-route evidence fixture beyond tiny public pages. The next proof should run real framework builds for the same page model.",
  };
}

function frameworkReport(profile, model) {
  const html = profile.render(model);
  const htmlBuffer = Buffer.from(html);
  const proof = profile.packet ? dxPacketProof(model) : null;
  const proofBuffer = proof ? Buffer.from(JSON.stringify(proof)) : Buffer.alloc(0);
  const htmlBytes = htmlBuffer.length;
  const brotliBytes = brotliSize(htmlBuffer);
  const proofBytes = proofBuffer.length;

  return {
    framework: profile.framework,
    baseline_id: profile.baseline_id,
    baseline_kind: profile.baseline_kind,
    runtime_claim: profile.runtime_claim,
    route_count: 1,
    total_decoded_bytes: htmlBytes,
    total_brotli_bytes: brotliBytes,
    proof_artifact_bytes: proofBytes,
    static_evidence: {
      route: model.route,
      html_bytes: htmlBytes,
      brotli_bytes: brotliBytes,
      sha256: sha256(htmlBuffer),
      contains_cards: containsAll(html, model.cards.map(([title]) => title)),
      contains_form: containsAll(html, model.formFields.map(([name]) => name)),
      contains_route_links: containsAll(html, model.navLinks.map(([href]) => href)),
    },
    note: profile.note,
  };
}

function renderDxWwwHtml(model) {
  return htmlShell(
    model,
    "DX-WWW",
    "dx-medium-route",
    "Static route generated from source-owned package evidence with no client runtime.",
    {
      bodyAttrs: 'data-dx-route="medium" data-dx-delivery="static"',
      cardAttrs: (index) => `data-dx-card="${index}"`,
      formAttrs: 'data-dx-form="review"',
    }
  );
}

function renderFrameworkHtml(model, framework, marker) {
  return htmlShell(
    model,
    framework,
    marker,
    `${framework} static floor for the same medium route model.`,
    {
      bodyAttrs: `data-framework="${escapeAttr(marker)}"`,
      cardAttrs: (index) => `data-card="${index}"`,
      formAttrs: `data-form="${escapeAttr(marker)}"`,
    }
  );
}

function renderHtmxHtml(model) {
  return htmlShell(
    model,
    "HTMX",
    "htmx-medium-floor",
    "HTMX-shaped static route with htmx attributes but no fetched runtime in this fixture.",
    {
      bodyAttrs: 'data-framework="htmx" hx-boost="true"',
      cardAttrs: (index) => `data-card="${index}" hx-get="/forge/evidence" hx-trigger="revealed once"`,
      formAttrs: 'hx-post="/forge/review" hx-swap="outerHTML"',
    }
  );
}

function htmlShell(model, framework, marker, description, attrs) {
  return [
    "<!doctype html>",
    '<html lang="en">',
    "<head>",
    '<meta charset="utf-8">',
    '<meta name="viewport" content="width=device-width,initial-scale=1">',
    `<title>${escapeHtml(framework)} medium-route fixture</title>`,
    "<style>",
    "body{margin:0;font-family:Inter,ui-sans-serif,system-ui,sans-serif;background:#fafafa;color:#111827}",
    "main{max-width:1120px;margin:auto;padding:32px}",
    "nav{display:flex;flex-wrap:wrap;gap:10px;margin:22px 0}",
    "a{color:#111827;text-decoration:none;border:1px solid #d1d5db;border-radius:7px;padding:8px 10px}",
    ".hero{border-bottom:1px solid #e5e7eb;padding-bottom:20px}.eyebrow{font-size:13px;color:#4b5563;text-transform:uppercase}",
    ".stats,.cards,.formgrid{display:grid;gap:12px}.stats{grid-template-columns:repeat(4,minmax(0,1fr));margin:18px 0}",
    ".stat,.card,form{border:1px solid #e5e7eb;background:white;border-radius:8px;padding:14px}",
    ".cards{grid-template-columns:repeat(3,minmax(0,1fr))}.card h2{font-size:16px;margin:0 0 8px}",
    "label{display:grid;gap:6px;font-size:13px;color:#374151}input,textarea,select{border:1px solid #d1d5db;border-radius:7px;padding:9px;background:white}",
    "button{border:0;border-radius:7px;background:#111827;color:white;padding:10px 14px}.meta{margin-top:20px;color:#4b5563;font-size:13px}",
    "@media(max-width:760px){main{padding:20px}.stats,.cards{grid-template-columns:1fr}}",
    "</style>",
    "</head>",
    `<body ${attrs.bodyAttrs}>`,
    `<main id="${escapeAttr(marker)}">`,
    '<section class="hero">',
    `<p class="eyebrow">${escapeHtml(model.eyebrow)}</p>`,
    `<h1>${escapeHtml(model.title)}</h1>`,
    `<p>${escapeHtml(model.summary)}</p>`,
    `<p class="meta">${escapeHtml(description)}</p>`,
    "</section>",
    "<nav aria-label=\"Forge medium route links\">",
    ...model.navLinks.map(([href, label]) => `<a href="${escapeAttr(href)}">${escapeHtml(label)}</a>`),
    "</nav>",
    '<section class="stats" aria-label="Release statistics">',
    ...model.stats.map(
      ([label, value]) =>
        `<article class="stat"><strong>${escapeHtml(value)}</strong><span>${escapeHtml(label)}</span></article>`
    ),
    "</section>",
    '<section class="cards" aria-label="Repeated Forge evidence cards">',
    ...model.cards.map(
      ([title, text], index) =>
        `<article class="card" ${attrs.cardAttrs(index)}><h2>${escapeHtml(title)}</h2><p>${escapeHtml(text)}</p></article>`
    ),
    "</section>",
    `<form ${attrs.formAttrs} aria-label="Forge review form">`,
    "<h2>Review this Forge route</h2>",
    '<div class="formgrid">',
    ...model.formFields.map(([name, label]) =>
      name === "decision"
        ? `<label>${escapeHtml(label)}<select name="${escapeAttr(name)}"><option>approve</option><option>needs-review</option></select></label>`
        : `<label>${escapeHtml(label)}<input name="${escapeAttr(name)}" value="${escapeAttr(name === "package" ? "shadcn/ui/button" : "")}"></label>`
    ),
    "</div>",
    "<button type=\"submit\">Record review</button>",
    "</form>",
    '<p class="meta">Static evidence: repeated cards, form fields, route links, and source-owned package review copy are present in this single route payload.</p>',
    "</main>",
    "</body>",
    "</html>",
  ].join("");
}

function dxPacketProof(model) {
  return {
    magic: "DXPK",
    version: 1,
    route: model.route,
    sections: {
      cards: model.cards.length,
      form_fields: model.formFields.length,
      route_links: model.navLinks.length,
      stats: model.stats.length,
    },
    delivery: "static",
    source_owned: true,
    node_modules_created: false,
  };
}

function renderMarkdown(report) {
  const lines = [
    "# Forge Medium Route Comparison",
    "",
    `Generated: ${report.generated_at}`,
    `Fixture route: \`${report.fixture.route}\``,
    "",
    "This medium-route fixture is not a full framework benchmark.",
    "It compares deterministic static route payloads for the same repeated cards, route links, and form fields.",
    "",
    "## Fixture Shape",
    "",
    `- Repeated cards: \`${report.fixture.cards}\``,
    `- Form fields: \`${report.fixture.form_fields}\``,
    `- Route links: \`${report.fixture.route_links}\``,
    `- Static evidence: \`${report.fixture.static_evidence}\``,
    `- Content parity: ${report.fixture.content_parity}`,
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
    "| Framework | Baseline | Kind | Decoded | Brotli | Proof artifact | Runtime claim |",
    "| --- | --- | --- | ---: | ---: | ---: | --- |",
    ...report.frameworks.map((framework) =>
      [
        framework.framework,
        framework.baseline_id,
        framework.baseline_kind,
        `${framework.total_decoded_bytes} B`,
        `${framework.total_brotli_bytes} B`,
        `${framework.proof_artifact_bytes} B`,
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

function containsAll(html, needles) {
  return needles.every((needle) => html.includes(needle));
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
        frameworks: report.frameworks.map((framework) => framework.framework),
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
    mediumPageModel,
    renderMarkdown,
    renderDxWwwHtml,
    renderFrameworkHtml,
    renderHtmxHtml,
  };
}
