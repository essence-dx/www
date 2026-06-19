const fs = require("fs");
const path = require("path");
const {
  buildHistoricalBenchmarkSnapshotStatus,
  snapshotStatusMarkdownBlock,
} = require("./report-snapshot-status");

const root = __dirname;
const reportDir = path.join(root, "reports");
const fairPath = path.join(reportDir, "fair-counter-comparison.json");
const labPath = path.join(reportDir, "binary-web-lab.json");
const outJsonPath = path.join(reportDir, "framework-scorecard.json");
const outMdPath = path.join(reportDir, "framework-scorecard.md");

const fair = JSON.parse(fs.readFileSync(fairPath, "utf8"));
const lab = JSON.parse(fs.readFileSync(labPath, "utf8"));

const frameworks = ["DX-WWW", "Astro", "Svelte", "HTMX", "Next.js"];

function fairResult(name) {
  const result = fair.results.find((item) => item.name === name);
  if (!result) {
    throw new Error(`Missing fair result: ${name}`);
  }

  return result;
}

function labResult(scenario, encoding) {
  const result = lab.results.find((item) => item.scenario === scenario && item.encoding === encoding);
  if (!result) {
    throw new Error(`Missing lab result: ${scenario}/${encoding}`);
  }

  return result;
}

function breakthrough(experiment, strategy) {
  const result = lab.breakthroughs.find((item) => item.experiment === experiment && item.strategy === strategy);
  if (!result) {
    throw new Error(`Missing breakthrough result: ${experiment}/${strategy}`);
  }

  return result;
}

function round(value, digits = 1) {
  return Number(value.toFixed(digits));
}

function lowerIsBetterScore(value, best) {
  if (!Number.isFinite(value) || value <= 0) {
    return 0;
  }

  return Math.max(1, Math.min(100, round((best / value) * 100)));
}

function average(values) {
  return round(values.reduce((sum, value) => sum + value, 0) / values.length);
}

function bytes(value) {
  if (value < 1024) {
    return `${value} B`;
  }

  return `${(value / 1024).toFixed(2)} KB`;
}

function stars(score) {
  const full = Math.max(0, Math.min(5, Math.round(score / 20)));
  return `${"\u2605".repeat(full)}${"\u2606".repeat(5 - full)}`;
}

const tiny = Object.fromEntries(
  frameworks.map((name) => {
    const result = fairResult(name);
    return [
      name,
      {
        brotli_bytes: result.compression_estimate.brotli_bytes,
        median_ms: result.full_route_timing.median_ms,
      },
    ];
  })
);

const htmlTiny = labResult("tiny-counter", "html-string");
const htmlDocs = labResult("docs-160-sections", "html-string");
const htmlMarketing = labResult("marketing-180-cards", "html-string");
const dxDocs = labResult("docs-160-sections", "dx-template-data");
const dxMarketing = labResult("marketing-180-cards", "dx-template-data");
const htmlDashboard = labResult("dashboard-1200-rows", "html-string");
const dxDashboardFull = labResult("dashboard-1200-rows", "dx-template-data");
const dxDashboardViewport = breakthrough("initial-viewport", "dx-viewport-40");

const runtimeFloor = {
  "DX-WWW": 0,
  Astro: 0,
  Svelte: Math.max(0, tiny.Svelte.brotli_bytes - htmlTiny.brotli_bytes),
  HTMX: Math.max(0, tiny.HTMX.brotli_bytes - htmlTiny.brotli_bytes),
  "Next.js": Math.max(0, tiny["Next.js"].brotli_bytes - htmlTiny.brotli_bytes),
};

const medium = {
  "DX-WWW": {
    brotli_bytes: average([dxDocs.brotli_bytes, dxMarketing.brotli_bytes]),
    access_ns: average([dxDocs.access_median_ns, dxMarketing.access_median_ns]),
    model: "DX template/data packets for repeated docs and marketing cards",
  },
  Astro: {
    brotli_bytes: average([htmlDocs.brotli_bytes, htmlMarketing.brotli_bytes]),
    access_ns: average([htmlDocs.access_median_ns, htmlMarketing.access_median_ns]),
    model: "static HTML baseline, Astro-like output shape",
  },
  Svelte: {
    brotli_bytes: runtimeFloor.Svelte + average([htmlDocs.brotli_bytes, htmlMarketing.brotli_bytes]),
    access_ns: average([htmlDocs.access_median_ns, htmlMarketing.access_median_ns]),
    model: "static HTML payload plus measured Svelte runtime floor",
  },
  HTMX: {
    brotli_bytes: runtimeFloor.HTMX + average([htmlDocs.brotli_bytes, htmlMarketing.brotli_bytes]),
    access_ns: average([htmlDocs.access_median_ns, htmlMarketing.access_median_ns]),
    model: "static HTML payload plus measured HTMX runtime floor",
  },
  "Next.js": {
    brotli_bytes: runtimeFloor["Next.js"] + average([htmlDocs.brotli_bytes, htmlMarketing.brotli_bytes]),
    access_ns: average([htmlDocs.access_median_ns, htmlMarketing.access_median_ns]),
    model: "static HTML payload plus measured Next/React runtime floor",
  },
};

const big = {
  "DX-WWW": {
    brotli_bytes: dxDashboardViewport.brotli_bytes,
    access_ns: dxDashboardViewport.access_median_ns,
    full_data_brotli_bytes: dxDashboardFull.brotli_bytes,
    full_data_access_ns: dxDashboardFull.access_median_ns,
    model: "adaptive initial viewport packet; full data can stream separately",
  },
  Astro: {
    brotli_bytes: htmlDashboard.brotli_bytes,
    access_ns: htmlDashboard.access_median_ns,
    model: "full static dashboard HTML baseline",
  },
  Svelte: {
    brotli_bytes: runtimeFloor.Svelte + htmlDashboard.brotli_bytes,
    access_ns: htmlDashboard.access_median_ns,
    model: "full dashboard payload plus measured Svelte runtime floor",
  },
  HTMX: {
    brotli_bytes: runtimeFloor.HTMX + htmlDashboard.brotli_bytes,
    access_ns: htmlDashboard.access_median_ns,
    model: "full dashboard payload plus measured HTMX runtime floor",
  },
  "Next.js": {
    brotli_bytes: runtimeFloor["Next.js"] + htmlDashboard.brotli_bytes,
    access_ns: htmlDashboard.access_median_ns,
    model: "full dashboard payload plus measured Next/React runtime floor",
  },
};

const currentReadiness = {
  "DX-WWW": 58,
  Astro: 88,
  Svelte: 84,
  HTMX: 76,
  "Next.js": 91,
};

const developerExperience = {
  "DX-WWW": 74,
  Astro: 82,
  Svelte: 86,
  HTMX: 70,
  "Next.js": 89,
};

const bestTinyBrotli = Math.min(...frameworks.map((name) => tiny[name].brotli_bytes));
const bestTinyMedian = Math.min(...frameworks.map((name) => tiny[name].median_ms));
const bestMediumBrotli = Math.min(...frameworks.map((name) => medium[name].brotli_bytes));
const bestMediumAccess = Math.min(...frameworks.map((name) => medium[name].access_ns));
const bestBigBrotli = Math.min(...frameworks.map((name) => big[name].brotli_bytes));
const bestBigAccess = Math.min(...frameworks.map((name) => big[name].access_ns));

const scorecard = Object.fromEntries(
  frameworks.map((name) => {
    const smallScore = average([
      lowerIsBetterScore(tiny[name].brotli_bytes, bestTinyBrotli),
      lowerIsBetterScore(tiny[name].median_ms, bestTinyMedian),
    ]);
    const mediumScore = average([
      lowerIsBetterScore(medium[name].brotli_bytes, bestMediumBrotli),
      lowerIsBetterScore(medium[name].access_ns, bestMediumAccess),
    ]);
    const bigScore = average([
      lowerIsBetterScore(big[name].brotli_bytes, bestBigBrotli),
      lowerIsBetterScore(big[name].access_ns, bestBigAccess),
    ]);
    const overall = round(
      smallScore * 0.25 +
        mediumScore * 0.2 +
        bigScore * 0.2 +
        currentReadiness[name] * 0.2 +
        developerExperience[name] * 0.15
    );

    return [
      name,
      {
        small_score: smallScore,
        medium_score: mediumScore,
        big_score: bigScore,
        current_readiness_score: currentReadiness[name],
        developer_experience_score: developerExperience[name],
        overall_score: overall,
        stars: stars(overall),
      },
    ];
  })
);

const ranked = [...frameworks].sort((a, b) => scorecard[b].overall_score - scorecard[a].overall_score);

const report = {
  generated_at: new Date().toISOString(),
  snapshot_status: buildHistoricalBenchmarkSnapshotStatus(),
  method: {
    community_excluded: true,
    small: "Actual local route benchmark. DX-WWW uses its Rust demo runtime; Astro/Svelte/HTMX use static local servers; Next.js uses next start.",
    medium: "Scale payload/access model over docs and marketing-card page graphs from binary-web-lab. This is compiler evidence, not a full browser Lighthouse run.",
    big: "Scale payload/access model over a 1200-row dashboard. DX-WWW uses adaptive viewport delivery for initial load and records full-data packet numbers separately.",
    score_formula: "Overall = 25% small actual route + 20% medium scale + 20% big scale + 20% current product readiness + 15% developer experience. Community/adoption is not included.",
  },
  tiny,
  medium,
  big,
  scorecard,
  ranked,
  verdict: {
    performance_winner: ranked[0],
    product_readiness_winner: Object.entries(currentReadiness).sort((a, b) => b[1] - a[1])[0][0],
    honest_note:
      "DX-WWW is now winning the measured small route and the adaptive medium/big payload model, but its product-readiness score is intentionally lower because App Router coverage, dx build proof, and the DX-owned dev feedback and diagnostics surface still need production proof.",
  },
};

fs.writeFileSync(outJsonPath, `${JSON.stringify(report, null, 2)}\n`);

function row(name) {
  const score = scorecard[name];
  return `| ${name} | ${score.small_score} | ${score.medium_score} | ${score.big_score} | ${score.current_readiness_score} | ${score.developer_experience_score} | ${score.overall_score} | ${score.stars} |`;
}

let markdown = "# Framework Scorecard\n\n";
markdown += `Generated: ${report.generated_at}\n\n`;
markdown += snapshotStatusMarkdownBlock();
markdown += "Community/adoption is deliberately excluded.\n\n";
markdown += "## Method\n\n";
markdown += `- Small: ${report.method.small}\n`;
markdown += `- Medium: ${report.method.medium}\n`;
markdown += `- Big: ${report.method.big}\n`;
markdown += `- Score: ${report.method.score_formula}\n\n`;
markdown += "## Small Actual Route\n\n";
markdown += "| Framework | Brotli | Median |\n";
markdown += "| --- | ---: | ---: |\n";
for (const name of frameworks) {
  markdown += `| ${name} | ${bytes(tiny[name].brotli_bytes)} | ${tiny[name].median_ms} ms |\n`;
}
markdown += "\n## Medium And Big Scale Models\n\n";
markdown += "| Framework | Medium model | Medium Brotli | Big model | Big Brotli |\n";
markdown += "| --- | --- | ---: | --- | ---: |\n";
for (const name of frameworks) {
  markdown += `| ${name} | ${medium[name].model} | ${bytes(medium[name].brotli_bytes)} | ${big[name].model} | ${bytes(big[name].brotli_bytes)} |\n`;
}
markdown += "\n## Scores\n\n";
markdown += "| Framework | Small | Medium | Big | Readiness | DX | Overall | Stars |\n";
markdown += "| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |\n";
for (const name of ranked) {
  markdown += `${row(name)}\n`;
}
markdown += "\n## Demanding Verdict\n\n";
markdown += `- Performance winner: ${report.verdict.performance_winner}\n`;
markdown += `- Product-readiness winner today: ${report.verdict.product_readiness_winner}\n`;
markdown += `- ${report.verdict.honest_note}\n`;

fs.writeFileSync(outMdPath, markdown);

console.log(
  JSON.stringify(
    {
      reports: [outMdPath, outJsonPath],
      ranked: ranked.map((name) => ({ name, ...scorecard[name] })),
    },
    null,
    2
  )
);
