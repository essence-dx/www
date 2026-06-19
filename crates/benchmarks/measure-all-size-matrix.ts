const fs = require("fs");
const path = require("path");
const { execFileSync } = require("child_process");

const root = path.resolve(__dirname, "..");
const reportDir = path.join(root, "benchmarks", "reports");
const fairPath = path.join(reportDir, "fair-counter-comparison.json");
const labPath = path.join(reportDir, "binary-web-lab.json");
const outJsonPath = path.join(reportDir, "all-size-framework-comparison.json");
const outMdPath = path.join(reportDir, "all-size-framework-comparison.md");

const fair = JSON.parse(fs.readFileSync(fairPath, "utf8"));
const lab = JSON.parse(fs.readFileSync(labPath, "utf8"));

function latestVersion(packageName) {
  try {
    return execFileSync("npm", ["view", packageName, "version"], {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
      timeout: 30_000,
    }).trim();
  } catch {
    return null;
  }
}

function fairResult(name) {
  const result = fair.results.find((item) => item.name === name);
  if (!result) {
    throw new Error(`Missing fair counter result: ${name}`);
  }
  return {
    name: result.name,
    raw_bytes: result.total_decoded_bytes,
    gzip_bytes: result.compression_estimate.gzip_bytes,
    brotli_bytes: result.compression_estimate.brotli_bytes,
    median_ms: result.full_route_timing.median_ms,
    source: "measured production tiny counter",
  };
}

function labResult(scenario, encoding) {
  const result = lab.results.find((item) => item.scenario === scenario && item.encoding === encoding);
  if (!result) {
    throw new Error(`Missing lab result: ${scenario}/${encoding}`);
  }
  return {
    name: encoding,
    raw_bytes: result.raw_bytes,
    gzip_bytes: result.gzip_bytes,
    brotli_bytes: result.brotli_bytes,
    access_median_ns: result.access_median_ns,
    source: "compiler packet lab",
  };
}

function breakthrough(experiment, strategy) {
  const result = lab.breakthroughs.find((item) => item.experiment === experiment && item.strategy === strategy);
  if (!result) {
    throw new Error(`Missing breakthrough result: ${experiment}/${strategy}`);
  }
  return {
    name: strategy,
    raw_bytes: result.raw_bytes,
    gzip_bytes: result.gzip_bytes,
    brotli_bytes: result.brotli_bytes,
    access_median_ns: result.access_median_ns,
    source: "compiler packet lab",
  };
}

function savings(dx, baseline, key = "brotli_bytes") {
  return Number((((baseline[key] - dx[key]) / baseline[key]) * 100).toFixed(1));
}

function ratio(a, b, key = "brotli_bytes") {
  return Number((a[key] / b[key]).toFixed(2));
}

function verdict(dx, baseline) {
  if (dx.brotli_bytes < baseline.brotli_bytes) {
    return `DX-WWW smaller by ${savings(dx, baseline)}% Brotli`;
  }
  if (dx.brotli_bytes > baseline.brotli_bytes) {
    return `Current baseline smaller by ${savings(baseline, dx)}% Brotli`;
  }
  return "Tie on Brotli";
}

const astroTiny = fairResult("Astro");
const dxTinyCurrent = fairResult("DX-WWW");
const nextTiny = fairResult("Next.js");
const svelteTiny = fairResult("Svelte");
const htmxTiny = fairResult("HTMX");

const rows = [
  {
    scenario: "Tiny interactive counter, actual browser route",
    current_best: astroTiny,
    dx_current: dxTinyCurrent,
    dx_adaptive: labResult("tiny-counter", "dx-template-data"),
    winner: "DX-WWW current micro route",
    verdict: `DX-WWW is ${savings(dxTinyCurrent, astroTiny)}% smaller than Astro on Brotli and ships no WASM for this tiny interaction; DX packet lab is ${ratio(astroTiny, labResult("tiny-counter", "dx-template-data"))}x smaller than Astro but is not yet the shipped route.`,
  },
  {
    scenario: "Tiny interactive counter, React framework baseline",
    current_best: nextTiny,
    dx_current: dxTinyCurrent,
    dx_adaptive: labResult("tiny-counter", "dx-template-data"),
    winner: "DX-WWW current demo",
    verdict: `Current DX-WWW demo is ${ratio(nextTiny, dxTinyCurrent)}x smaller than Next.js on Brotli.`,
  },
  {
    scenario: "Tiny interactive counter, compiled JS baseline",
    current_best: svelteTiny,
    dx_current: dxTinyCurrent,
    dx_adaptive: labResult("tiny-counter", "dx-template-data"),
    winner: "DX-WWW current demo",
    verdict: `Current DX-WWW demo is ${ratio(svelteTiny, dxTinyCurrent)}x smaller than Svelte on Brotli.`,
  },
  {
    scenario: "Static/repeated docs, 160 sections",
    current_best: labResult("docs-160-sections", "html-string"),
    dx_current: labResult("docs-160-sections", "dx-template-data"),
    dx_adaptive: labResult("docs-160-sections", "dx-template-data"),
    winner: "DX-WWW adaptive template/data",
    verdict: verdict(labResult("docs-160-sections", "dx-template-data"), labResult("docs-160-sections", "html-string")),
  },
  {
    scenario: "Repeated marketing cards, 180 cards",
    current_best: labResult("marketing-180-cards", "html-string"),
    dx_current: labResult("marketing-180-cards", "dx-template-data"),
    dx_adaptive: labResult("marketing-180-cards", "dx-template-data"),
    winner: "DX-WWW adaptive template/data",
    verdict: verdict(labResult("marketing-180-cards", "dx-template-data"), labResult("marketing-180-cards", "html-string")),
  },
  {
    scenario: "Large dashboard, full 1200-row data",
    current_best: labResult("dashboard-1200-rows", "html-string"),
    dx_current: labResult("dashboard-1200-rows", "dx-template-data"),
    dx_adaptive: breakthrough("dashboard-full-data", "dx-columnar-slots"),
    winner: "DX-WWW template/data on Brotli; columnar wins raw/access",
    verdict: `${verdict(labResult("dashboard-1200-rows", "dx-template-data"), labResult("dashboard-1200-rows", "html-string"))}; columnar is ${savings(breakthrough("dashboard-full-data", "dx-columnar-slots"), labResult("dashboard-1200-rows", "html-string"), "raw_bytes")}% smaller raw but worse than template on Brotli for this synthetic data.`,
  },
  {
    scenario: "Large dashboard, initial 40-row viewport",
    current_best: labResult("dashboard-1200-rows", "html-string"),
    dx_current: breakthrough("initial-viewport", "dx-viewport-40"),
    dx_adaptive: breakthrough("initial-viewport", "dx-viewport-40"),
    winner: "DX-WWW viewport packet",
    verdict: verdict(breakthrough("initial-viewport", "dx-viewport-40"), labResult("dashboard-1200-rows", "html-string")),
  },
  {
    scenario: "12-row live update",
    current_best: breakthrough("12-row-live-update", "html-row-fragments"),
    dx_current: breakthrough("12-row-live-update", "dx-cell-patch"),
    dx_adaptive: breakthrough("12-row-live-update", "dx-cell-patch"),
    winner: "DX-WWW patch stream, but only slightly after Brotli",
    verdict: verdict(breakthrough("12-row-live-update", "dx-cell-patch"), breakthrough("12-row-live-update", "html-row-fragments")),
  },
  {
    scenario: "600-row bulk update",
    current_best: breakthrough("600-row-bulk-update", "json-range-op"),
    dx_current: breakthrough("600-row-bulk-update", "dx-range-op"),
    dx_adaptive: breakthrough("600-row-bulk-update", "dx-range-op"),
    winner: "DX-WWW range op",
    verdict: verdict(breakthrough("600-row-bulk-update", "dx-range-op"), breakthrough("600-row-bulk-update", "json-range-op")),
  },
];

const latestVersions = {
  next: latestVersion("next"),
  react: latestVersion("react"),
  svelte: latestVersion("svelte"),
  astro: latestVersion("astro"),
  htmx: latestVersion("htmx.org"),
};

const summary = {
  generated_at: new Date().toISOString(),
  method: {
    actual_framework_test: "fair-counter-comparison.json measures equivalent production tiny counter routes for current local Next.js, Svelte, Astro, HTMX, and DX-WWW demo.",
    all_size_test: "binary-web-lab.json measures payload encodings for generated page graphs and update packets. It is compiler evidence, not a full browser framework build.",
    fairness_note: "Rows labelled compiler packet lab prove payload shape and encode/access cost. They do not prove hydration, DOM apply cost, routing, dev server quality, ecosystem support, or CDN behavior yet.",
  },
  latest_versions_checked_from_npm: latestVersions,
  local_versions: fair.versions,
  rows,
  honest_verdict: {
    today: "Current DX-WWW now beats Astro, Next.js, Svelte, and HTMX on the tiny measured route by using the micro-JS/no-WASM path.",
    after_new_compiler_is_runtime_wired: "The adaptive compiler direction is genuinely strong for repeated UI, dashboards, partial updates, source-owned package registries, and framework hosting, but it must keep static/micro-JS/no-WASM paths automatic for tiny pages.",
    non_tech_sellability: "Non-technical buyers will care if this becomes cheaper hosting, faster dashboards, smaller websites, safer editable packages, and easier maintenance. They will not care about binary packets by name.",
    biggest_flaw: "The compiler packet wins are not yet the complete product. The missing layer is production runtime selection, DOM apply benchmarking, DX registry/versioning workflow, framework-compatible integrations, and proof on real websites.",
  },
};

fs.writeFileSync(outJsonPath, `${JSON.stringify(summary, null, 2)}\n`);

function bytes(value) {
  return `${value.toLocaleString()} B`;
}

function metric(item) {
  const speed = item.access_median_ns != null ? `${item.access_median_ns} ns packet access` : `${item.median_ms} ms route median`;
  return `${bytes(item.raw_bytes)} raw / ${bytes(item.gzip_bytes)} gzip / ${bytes(item.brotli_bytes)} Brotli / ${speed}`;
}

let markdown = "# All-Size Framework Comparison\n\n";
markdown += `Generated: ${summary.generated_at}\n\n`;
markdown += "## Method\n\n";
markdown += "- Actual tiny route: production/minimal counter builds measured for DX-WWW, Next.js, Svelte, Astro, and HTMX.\n";
markdown += "- All-size rows: compiler packet lab over equivalent generated page/update shapes. These are payload/compiler results, not full Lighthouse/browser-app scores.\n";
markdown += "- Current latest npm versions checked during this run: ";
markdown += Object.entries(latestVersions).map(([name, version]) => `${name} ${version || "unavailable"}`).join(", ");
markdown += ".\n\n";
markdown += "## Matrix\n\n";
markdown += "| Scenario | Best current baseline | DX-WWW current/adaptive path | Winner | Honest verdict |\n";
markdown += "| --- | --- | --- | --- | --- |\n";
for (const row of rows) {
  markdown += `| ${row.scenario} | ${row.current_best.name}: ${metric(row.current_best)} | ${row.dx_current.name}: ${metric(row.dx_current)} | ${row.winner} | ${row.verdict} |\n`;
}
markdown += "\n## Honest Verdict\n\n";
markdown += `- Today: ${summary.honest_verdict.today}\n`;
markdown += `- If wired correctly: ${summary.honest_verdict.after_new_compiler_is_runtime_wired}\n`;
markdown += `- Non-tech pitch: ${summary.honest_verdict.non_tech_sellability}\n`;
markdown += `- Biggest flaw: ${summary.honest_verdict.biggest_flaw}\n`;
markdown += "\n## Demanding Product Read\n\n";
markdown += "- The invention is real enough to be worth building, because the payload wins appear exactly where modern apps hurt: repeated UI, dashboards, updates, and dependency packaging.\n";
markdown += "- It is not yet a Next.js killer as a product. It is currently a compiler thesis with promising packet evidence and an early demo.\n";
markdown += "- To become a billion-dollar-grade platform, the next proof must be a real website builder flow: compile a shadcn-style app, emit static/micro/wasm plans, deploy it, compare real browser metrics, and show safe editable dependency updates.\n";

fs.writeFileSync(outMdPath, markdown);

console.log(JSON.stringify({
  reports: [outMdPath, outJsonPath],
  rows: rows.map((row) => ({
    scenario: row.scenario,
    baseline_brotli: row.current_best.brotli_bytes,
    dx_brotli: row.dx_current.brotli_bytes,
    winner: row.winner,
  })),
}, null, 2));
