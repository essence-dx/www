const fs = require("fs");
const path = require("path");

const reportDir = path.join(__dirname, "reports");
const historyDir = path.join(reportDir, "vertical-proof-history");
const historyIndexPath = path.join(historyDir, "index.json");
const outJsonPath = path.join(reportDir, "forge-public-route-comparison.json");
const outMdPath = path.join(reportDir, "forge-public-route-comparison.md");

const publicRoutes = [
  {
    route: "/forge",
    fixture_mode: "forge-site",
    role: "Launch evidence",
  },
  {
    route: "/forge/scorecard",
    fixture_mode: "forge-scorecard",
    role: "Package scorecard",
  },
  {
    route: "/forge/ci",
    fixture_mode: "forge-ci",
    role: "CI evidence",
  },
  {
    route: "/forge/evidence",
    fixture_mode: "forge-evidence",
    role: "Evidence index",
  },
  {
    route: "/forge/releases",
    fixture_mode: "forge-releases",
    role: "Release history",
  },
  {
    route: "/forge/changelog",
    fixture_mode: "forge-changelog",
    role: "Launch changelog",
  },
  {
    route: "/forge/adoption",
    fixture_mode: "forge-adoption",
    role: "Adoption evidence",
  },
];

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

function readHistory() {
  if (!fs.existsSync(historyIndexPath)) {
    throw new Error(`Missing vertical proof history index: ${historyIndexPath}`);
  }
  const parsed = readJson(historyIndexPath);
  if (!Array.isArray(parsed.snapshots)) {
    throw new Error("vertical-proof-history/index.json must contain a snapshots array");
  }
  return parsed.snapshots
    .map((snapshot) => hydrateSnapshot(snapshot))
    .sort((left, right) => right.generated_at.localeCompare(left.generated_at));
}

function hydrateSnapshot(snapshot) {
  const fullPath = path.join(reportDir, snapshot.json || "");
  if (!snapshot.json || !fs.existsSync(fullPath)) {
    return { ...snapshot, full: null };
  }
  return { ...snapshot, full: readJson(fullPath) };
}

function latestForRoute(history, fixtureMode) {
  return history.find((snapshot) => snapshot.fixture_mode === fixtureMode) || null;
}

function metric(snapshot, pathParts, fallback = null) {
  let current = snapshot.full;
  for (const part of pathParts) {
    if (current === null || current === undefined) {
      return fallback;
    }
    current = current[part];
  }
  return current === undefined ? fallback : current;
}

function routeRow(route, snapshot) {
  if (!snapshot) {
    return {
      ...route,
      status: "missing",
    };
  }

  return {
    ...route,
    status: "measured",
    generated_at: snapshot.generated_at,
    route_delivery:
      snapshot.route_delivery || metric(snapshot, ["delivery", "route_mode"], "unknown"),
    runtime_asset_written: metric(snapshot, ["delivery", "runtime_asset_written"], null),
    packet_artifact_written: metric(snapshot, ["delivery", "packet_artifact_written"], null),
    http_resources: metric(snapshot, ["http", "resource_count"], null),
    forge_packages: snapshot.forge_packages,
    forge_files_tracked: snapshot.forge_files_tracked,
    decoded_bytes: snapshot.decoded_bytes,
    brotli_bytes: snapshot.brotli_bytes,
    http_route_median_ms: snapshot.http_route_median_ms,
    chrome_load_event_ms: snapshot.chrome_load_event_ms,
    dx_packet_applied: snapshot.dx_packet_applied,
    interaction_works: snapshot.interaction_works,
    budget_passed: snapshot.budget_passed ?? null,
    markdown: snapshot.markdown,
    json: snapshot.json,
  };
}

function buildReport(history) {
  const routes = publicRoutes.map((route) =>
    routeRow(route, latestForRoute(history, route.fixture_mode))
  );
  const missing = routes.filter((route) => route.status !== "measured");
  if (missing.length > 0) {
    throw new Error(
      `Missing public Forge route measurement(s): ${missing
        .map((route) => route.fixture_mode)
        .join(", ")}`
    );
  }

  const lowestBrotli = [...routes].sort((left, right) => left.brotli_bytes - right.brotli_bytes)[0];
  const totalDecoded = routes.reduce((sum, route) => sum + route.decoded_bytes, 0);
  const totalBrotli = routes.reduce((sum, route) => sum + route.brotli_bytes, 0);

  return {
    generated_at: new Date().toISOString(),
    source_history_index: "vertical-proof-history/index.json",
    route_count: routes.length,
    total_decoded_bytes: totalDecoded,
    total_brotli_bytes: totalBrotli,
    lowest_brotli_route: lowestBrotli.route,
    routes,
    conclusion:
      "The public Forge launch, scorecard, CI, evidence-index, release-history, launch-changelog, and adoption evidence routes are all static/no-runtime compiler outputs. Keep them measured together so launch evidence, package claims, CI proof, public evidence links, release history, human changelog copy, and local adoption evidence cannot grow independently without visibility. This is reproducible local adoption evidence, not deployed-user proof or a market-usage claim.",
  };
}

function renderMarkdown(report) {
  return [
    "# Forge Public Route Comparison",
    "",
    `Generated: ${report.generated_at}`,
    `History source: \`${report.source_history_index}\``,
    `Total decoded: ${report.total_decoded_bytes} B`,
    `Total Brotli estimate: ${report.total_brotli_bytes} B`,
    `Smallest Brotli route: \`${report.lowest_brotli_route}\``,
    "",
    "| Route | Role | Fixture | Generated | Delivery | Runtime asset | Resources | Packages | Files | Decoded | Brotli | HTTP median | Chrome load | DXPK | Budget | Evidence |",
    "| --- | --- | --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- | --- |",
    ...report.routes
      .map((route) =>
        [
          route.route,
          route.role,
          route.fixture_mode,
          route.generated_at,
          route.route_delivery,
          formatBool(route.runtime_asset_written),
          formatNullable(route.http_resources),
          formatNullable(route.forge_packages),
          formatNullable(route.forge_files_tracked),
          `${route.decoded_bytes} B`,
          `${route.brotli_bytes} B`,
          `${route.http_route_median_ms} ms`,
          formatNullable(route.chrome_load_event_ms, " ms"),
          formatBool(route.dx_packet_applied),
          formatBool(route.budget_passed),
          route.markdown ? `[md](${route.markdown})` : "n/a",
        ].join(" | ")
      )
      .map((row) => `| ${row} |`),
    "",
    report.conclusion,
    "",
  ].join("\n");
}

function formatBool(value) {
  if (value === null || value === undefined) {
    return "n/a";
  }
  return value ? "yes" : "no";
}

function formatNullable(value, suffix = "") {
  if (value === null || value === undefined) {
    return "n/a";
  }
  return `${value}${suffix}`;
}

function main() {
  const report = buildReport(readHistory());
  fs.writeFileSync(outJsonPath, `${JSON.stringify(report, null, 2)}\n`);
  fs.writeFileSync(outMdPath, renderMarkdown(report));
  console.log(
    JSON.stringify(
      {
        report: [outMdPath, outJsonPath],
        route_count: report.route_count,
        total_decoded_bytes: report.total_decoded_bytes,
        total_brotli_bytes: report.total_brotli_bytes,
        lowest_brotli_route: report.lowest_brotli_route,
      },
      null,
      2
    )
  );
}

if (require.main === module) {
  main();
}

module.exports = {
  buildReport,
  publicRoutes,
  renderMarkdown,
};
