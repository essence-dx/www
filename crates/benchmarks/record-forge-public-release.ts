const fs = require("fs");
const path = require("path");

const root = path.resolve(__dirname, "..");
const reportDir = path.join(__dirname, "reports");
const dashboardPath = resolveInput(
  process.env.DX_FORGE_RELEASE_DASHBOARD,
  path.join(root, ".dx", "ci", "forge-release-dashboard.json")
);
const routeComparisonPath = resolveInput(
  process.env.DX_FORGE_ROUTE_COMPARISON,
  path.join(reportDir, "forge-public-route-comparison.json")
);
const historyPath = resolveInput(
  process.env.DX_FORGE_PUBLIC_RELEASE_HISTORY,
  path.join(reportDir, "forge-public-release-history.json")
);
const historyMarkdownPath = historyPath.replace(/\.json$/i, ".md");

function resolveInput(value, fallback) {
  if (!value) {
    return fallback;
  }
  return path.isAbsolute(value) ? value : path.join(root, value);
}

function readJson(filePath, hint) {
  if (!fs.existsSync(filePath)) {
    throw new Error(`${hint} is missing: ${filePath}`);
  }
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

function relativeToRoot(filePath) {
  return path.relative(root, filePath).replace(/\\/g, "/");
}

function buildRecord(dashboard, routeComparison) {
  return {
    generated_at: new Date().toISOString(),
    source_dashboard: relativeToRoot(dashboardPath),
    source_route_comparison: relativeToRoot(routeComparisonPath),
    dashboard: {
      generated_at: dashboard.generated_at,
      score: dashboard.score,
      fail_under: dashboard.fail_under,
      passed: dashboard.passed,
      no_node_modules: dashboard.release_notes?.no_node_modules ?? null,
      public_evidence_links: dashboard.public_evidence?.links ?? null,
      findings: dashboard.findings || [],
      checks: Object.fromEntries(
        Object.entries(dashboard.checks || {}).map(([name, check]) => [
          name,
          {
            passed: check.passed,
            score: check.score,
            message: check.message,
          },
        ])
      ),
    },
    route_comparison: {
      generated_at: routeComparison.generated_at,
      route_count: routeComparison.route_count,
      total_decoded_bytes: routeComparison.total_decoded_bytes,
      total_brotli_bytes: routeComparison.total_brotli_bytes,
      lowest_brotli_route: routeComparison.lowest_brotli_route,
      routes: (routeComparison.routes || []).map((route) => ({
        route: route.route,
        fixture_mode: route.fixture_mode,
        delivery: route.route_delivery,
        decoded_bytes: route.decoded_bytes,
        brotli_bytes: route.brotli_bytes,
        http_route_median_ms: route.http_route_median_ms,
        chrome_load_event_ms: route.chrome_load_event_ms,
        budget_passed: route.budget_passed,
      })),
    },
  };
}

function readHistory() {
  if (!fs.existsSync(historyPath)) {
    return { updated_at: null, records: [] };
  }

  const parsed = readJson(historyPath, "Forge public release history");
  return {
    updated_at: parsed.updated_at || null,
    records: Array.isArray(parsed.records) ? parsed.records : [],
  };
}

function sameEvidence(left, right) {
  return (
    left.dashboard?.generated_at === right.dashboard?.generated_at &&
    left.route_comparison?.generated_at === right.route_comparison?.generated_at
  );
}

function writeHistory(nextRecord) {
  fs.mkdirSync(path.dirname(historyPath), { recursive: true });
  const previous = readHistory().records.filter((record) => !sameEvidence(record, nextRecord));
  const records = [nextRecord, ...previous]
    .sort((left, right) => right.generated_at.localeCompare(left.generated_at))
    .slice(0, 30);
  const history = {
    updated_at: nextRecord.generated_at,
    records,
  };
  fs.writeFileSync(historyPath, `${JSON.stringify(history, null, 2)}\n`);
  fs.writeFileSync(historyMarkdownPath, renderMarkdown(history));
  return history;
}

function renderMarkdown(history) {
  const latest = history.records[0];
  const lines = [
    "# Forge Public Release History",
    "",
    `Updated: ${history.updated_at}`,
    "",
  ];

  if (latest) {
    lines.push(
      "## Latest",
      "",
      `- Dashboard score: ${latest.dashboard.score} / 100`,
      `- Dashboard passed: ${latest.dashboard.passed}`,
      `- Required score: ${latest.dashboard.fail_under} / 100`,
      `- Public routes: ${latest.route_comparison.route_count}`,
      `- Total decoded bytes: ${latest.route_comparison.total_decoded_bytes} B`,
      `- Total Brotli estimate: ${latest.route_comparison.total_brotli_bytes} B`,
      `- Smallest Brotli route: \`${latest.route_comparison.lowest_brotli_route}\``,
      ""
    );

    lines.push(
      "| Route | Fixture | Delivery | Decoded | Brotli | HTTP median | Chrome load | Budget |",
      "| --- | --- | --- | ---: | ---: | ---: | ---: | --- |"
    );
    for (const route of latest.route_comparison.routes) {
      lines.push(
        `| ${route.route} | ${route.fixture_mode} | ${route.delivery} | ${route.decoded_bytes} B | ${route.brotli_bytes} B | ${route.http_route_median_ms} ms | ${route.chrome_load_event_ms} ms | ${formatBool(route.budget_passed)} |`
      );
    }
    lines.push("");
  }

  lines.push(
    "## Records",
    "",
    "| Recorded | Dashboard | Routes | Decoded | Brotli | Findings |",
    "| --- | ---: | ---: | ---: | ---: | ---: |"
  );
  for (const record of history.records) {
    lines.push(
      `| ${record.generated_at} | ${record.dashboard.score} | ${record.route_comparison.route_count} | ${record.route_comparison.total_decoded_bytes} B | ${record.route_comparison.total_brotli_bytes} B | ${record.dashboard.findings.length} |`
    );
  }
  lines.push("");

  return `${lines.join("\n")}\n`;
}

function formatBool(value) {
  if (value === null || value === undefined) {
    return "n/a";
  }
  return value ? "yes" : "no";
}

function main() {
  const dashboard = readJson(
    dashboardPath,
    "Forge release-dashboard JSON; run `dx forge release-dashboard --format json --output .dx/ci/forge-release-dashboard.json` first"
  );
  const routeComparison = readJson(routeComparisonPath, "Forge public route comparison JSON");
  const history = writeHistory(buildRecord(dashboard, routeComparison));

  console.log(
    JSON.stringify(
      {
        history: relativeToRoot(historyPath),
        markdown: relativeToRoot(historyMarkdownPath),
        records: history.records.length,
        latest_dashboard_score: history.records[0]?.dashboard.score ?? null,
        latest_route_count: history.records[0]?.route_comparison.route_count ?? null,
        latest_total_brotli_bytes:
          history.records[0]?.route_comparison.total_brotli_bytes ?? null,
      },
      null,
      2
    )
  );
}

main();
