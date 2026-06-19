const fs = require("fs");
const path = require("path");

const reportDir = path.join(__dirname, "reports");
const historyDir = path.join(reportDir, "vertical-proof-history");
const historyIndexPath = path.join(historyDir, "index.json");
const outJsonPath = path.join(reportDir, "forge-launch-delivery-comparison.json");
const outMdPath = path.join(reportDir, "forge-launch-delivery-comparison.md");

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

function hasStaticDelivery(snapshot) {
  return (
    snapshot.fixture_mode === "forge-site" &&
    (snapshot.route_delivery === "static" || metric(snapshot, ["delivery", "route_mode"]) === "static") &&
    metric(snapshot, ["delivery", "runtime_asset_written"], false) === false
  );
}

function hasDxpkRuntimeDelivery(snapshot) {
  if (snapshot.fixture_mode !== "forge-site" || hasStaticDelivery(snapshot)) {
    return false;
  }
  return (
    metric(snapshot, ["delivery", "runtime_asset_written"], false) === true ||
    metric(snapshot, ["chrome", "dx_packet_applied"], snapshot.dx_packet_applied) === true ||
    metric(snapshot, ["http", "resource_count"], 1) > 1
  );
}

function isPackageVertical(snapshot) {
  return ["forge-combo", "forge-package", "forge-icon"].includes(snapshot.fixture_mode);
}

function pickModeSnapshots(history) {
  const staticNoRuntime = history.find(hasStaticDelivery);
  const dxpkRuntime = history.find(hasDxpkRuntimeDelivery);
  const packageVertical =
    history.find((snapshot) => snapshot.fixture_mode === "forge-combo") ||
    history.find((snapshot) => snapshot.fixture_mode === "forge-package") ||
    history.find((snapshot) => snapshot.fixture_mode === "forge-icon") ||
    history.find(isPackageVertical);
  return { staticNoRuntime, dxpkRuntime, packageVertical };
}

function modeRow(mode, snapshot, note) {
  if (!snapshot) {
    return null;
  }
  return {
    mode,
    note,
    generated_at: snapshot.generated_at,
    fixture_mode: snapshot.fixture_mode,
    fixture: snapshot.fixture,
    markdown: snapshot.markdown,
    route_delivery: snapshot.route_delivery || metric(snapshot, ["delivery", "route_mode"], "dxpk-runtime"),
    runtime_asset_written: metric(snapshot, ["delivery", "runtime_asset_written"], null),
    packet_artifact_written: metric(snapshot, ["delivery", "packet_artifact_written"], true),
    http_resources: metric(snapshot, ["http", "resource_count"], null),
    chrome_scripts: metric(snapshot, ["chrome", "scripts"], null),
    forge_packages: snapshot.forge_packages,
    forge_files_tracked: snapshot.forge_files_tracked,
    decoded_bytes: snapshot.decoded_bytes,
    brotli_bytes: snapshot.brotli_bytes,
    http_route_median_ms: snapshot.http_route_median_ms,
    chrome_load_event_ms: snapshot.chrome_load_event_ms,
    dx_packet_applied: snapshot.dx_packet_applied,
    interaction_works: snapshot.interaction_works,
    budget_passed: snapshot.budget_passed ?? null,
  };
}

function buildDelta(from, to) {
  if (!from || !to) {
    return null;
  }
  return {
    from: from.mode,
    to: to.mode,
    decoded_bytes_delta: to.decoded_bytes - from.decoded_bytes,
    decoded_bytes_pct: percentDelta(from.decoded_bytes, to.decoded_bytes),
    brotli_bytes_delta: to.brotli_bytes - from.brotli_bytes,
    brotli_bytes_pct: percentDelta(from.brotli_bytes, to.brotli_bytes),
    http_route_median_ms_delta: nullableDelta(from.http_route_median_ms, to.http_route_median_ms),
    chrome_load_event_ms_delta: nullableDelta(from.chrome_load_event_ms, to.chrome_load_event_ms),
  };
}

function nullableDelta(left, right) {
  if (typeof left !== "number" || typeof right !== "number") {
    return null;
  }
  return Number((right - left).toFixed(3));
}

function percentDelta(left, right) {
  if (typeof left !== "number" || left === 0 || typeof right !== "number") {
    return null;
  }
  return Number((((right - left) / left) * 100).toFixed(1));
}

function buildReport(history) {
  const picked = pickModeSnapshots(history);
  const modes = [
    modeRow(
      "static-no-runtime",
      picked.staticNoRuntime,
      "Current public /forge route. Ships crawlable HTML and keeps DXPK as a proof artifact without a browser runtime asset."
    ),
    modeRow(
      "dxpk-runtime",
      picked.dxpkRuntime,
      "Historical /forge route with DXPK browser runtime applied. Kept as a regression baseline for the static/no-runtime planner."
    ),
    modeRow(
      "package-vertical",
      picked.packageVertical,
      "Interactive source-owned package route. This is not the public /forge page, but it proves the runtime path still works when interaction exists."
    ),
  ].filter(Boolean);

  const missing = [
    ["static-no-runtime", picked.staticNoRuntime],
    ["dxpk-runtime", picked.dxpkRuntime],
    ["package-vertical", picked.packageVertical],
  ]
    .filter(([, snapshot]) => !snapshot)
    .map(([name]) => name);

  if (missing.length > 0) {
    throw new Error(`Missing required delivery comparison mode(s): ${missing.join(", ")}`);
  }

  const staticMode = modes.find((mode) => mode.mode === "static-no-runtime");
  const runtimeMode = modes.find((mode) => mode.mode === "dxpk-runtime");
  const packageMode = modes.find((mode) => mode.mode === "package-vertical");
  const lowestBrotli = [...modes].sort((left, right) => left.brotli_bytes - right.brotli_bytes)[0];

  return {
    generated_at: new Date().toISOString(),
    source_history_index: "vertical-proof-history/index.json",
    mode_count: modes.length,
    lowest_brotli_mode: lowestBrotli.mode,
    public_forge_route_mode: staticMode.route_delivery,
    modes,
    deltas: [
      buildDelta(runtimeMode, staticMode),
      buildDelta(packageMode, staticMode),
    ].filter(Boolean),
    conclusion:
      "The public /forge route should stay static/no-runtime while it has no interaction. DXPK runtime remains justified for interactive package verticals, and the report keeps the historical runtime baseline visible so payload regressions are obvious.",
  };
}

function renderMarkdown(report) {
  return [
    "# Forge Launch Delivery Comparison",
    "",
    `Generated: ${report.generated_at}`,
    `History source: \`${report.source_history_index}\``,
    "",
    "| Mode | Fixture | Generated | Delivery | Runtime asset | Resources | Scripts | Packages | Files | Decoded | Brotli | HTTP median | Chrome load | DXPK | Interaction | Evidence |",
    "| --- | --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- | --- |",
    ...report.modes.map((mode) =>
      [
        mode.mode,
        mode.fixture_mode,
        mode.generated_at,
        mode.route_delivery || "unknown",
        formatBool(mode.runtime_asset_written),
        formatNullable(mode.http_resources),
        formatNullable(mode.chrome_scripts),
        mode.forge_packages,
        mode.forge_files_tracked,
        `${mode.decoded_bytes} B`,
        `${mode.brotli_bytes} B`,
        `${mode.http_route_median_ms} ms`,
        formatNullable(mode.chrome_load_event_ms, " ms"),
        formatBool(mode.dx_packet_applied),
        formatBool(mode.interaction_works),
        mode.markdown ? `[md](${mode.markdown.replace(/^vertical-proof-history\//, "vertical-proof-history/")})` : "n/a",
      ].join(" | ")
    ).map((row) => `| ${row} |`),
    "",
    "## Deltas To Static / No Runtime",
    "",
    "| Comparison | Decoded | Decoded % | Brotli | Brotli % | HTTP median | Chrome load |",
    "| --- | ---: | ---: | ---: | ---: | ---: | ---: |",
    ...report.deltas.map((delta) =>
      [
        `${delta.from} to ${delta.to}`,
        formatSigned(delta.decoded_bytes_delta, " B"),
        formatSigned(delta.decoded_bytes_pct, "%"),
        formatSigned(delta.brotli_bytes_delta, " B"),
        formatSigned(delta.brotli_bytes_pct, "%"),
        formatSigned(delta.http_route_median_ms_delta, " ms"),
        formatSigned(delta.chrome_load_event_ms_delta, " ms"),
      ].join(" | ")
    ).map((row) => `| ${row} |`),
    "",
    "## Notes",
    "",
    ...report.modes.map((mode) => `- ${mode.mode}: ${mode.note}`),
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

function formatSigned(value, suffix = "") {
  if (value === null || value === undefined) {
    return "n/a";
  }
  const numeric = typeof value === "number" ? Number(value.toFixed(3)) : value;
  return `${numeric > 0 ? "+" : ""}${numeric}${suffix}`;
}

function main() {
  const history = readHistory();
  const report = buildReport(history);
  fs.writeFileSync(outJsonPath, `${JSON.stringify(report, null, 2)}\n`);
  fs.writeFileSync(outMdPath, renderMarkdown(report));
  console.log(
    JSON.stringify(
      {
        report: {
          jsonPath: outJsonPath,
          mdPath: outMdPath,
        },
        modes: report.modes.map((mode) => ({
          mode: mode.mode,
          fixture_mode: mode.fixture_mode,
          decoded_bytes: mode.decoded_bytes,
          brotli_bytes: mode.brotli_bytes,
          chrome_load_event_ms: mode.chrome_load_event_ms,
        })),
        lowest_brotli_mode: report.lowest_brotli_mode,
      },
      null,
      2
    )
  );
}

main();
