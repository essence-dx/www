const fs = require("fs");
const path = require("path");
const { spawnSync } = require("child_process");

const {
  buildReport: buildStaticCompetitorReport,
  renderMarkdown: renderStaticCompetitorMarkdown,
} = require("./compare-forge-static-competitors.ts");

const root = path.resolve(__dirname, "..");
const reportDir = path.join(__dirname, "reports");
const outJsonPath = path.join(reportDir, "forge-live-framework-harness.json");
const outMdPath = path.join(reportDir, "forge-live-framework-harness.md");

const defaultProfiles = [
  {
    framework: "Astro",
    project_dir: path.join(__dirname, "fair-counter", "astro"),
    package_manager: "npm",
    build_command: ["npm", ["run", "build"]],
    output_dirs: ["dist"],
    static_floor_id: "astro-static-html-floor",
  },
  {
    framework: "Svelte",
    project_dir: path.join(__dirname, "fair-counter", "svelte"),
    package_manager: "npm",
    build_command: ["npm", ["run", "build"]],
    output_dirs: ["dist", "build"],
    static_floor_id: "svelte-prerendered-static-floor",
  },
  {
    framework: "HTMX",
    project_dir: path.join(__dirname, "fair-counter", "htmx"),
    package_manager: "npm",
    build_command: null,
    output_dirs: ["public"],
    static_floor_id: "htmx-local-static-server",
  },
  {
    framework: "Next.js",
    project_dir: path.join(__dirname, "fair-counter", "next"),
    package_manager: "npm",
    build_command: ["npm", ["run", "build"]],
    output_dirs: [".next", "out"],
    static_floor_id: "next-static-export-floor",
  },
];

function buildReport(options = {}) {
  const generatedAt = options.generatedAt || new Date().toISOString();
  const liveEnabled = options.liveEnabled ?? process.env.DX_FORGE_LIVE_FRAMEWORKS === "1";
  const staticReport =
    options.staticReport ||
    buildStaticCompetitorReport({
      generatedAt,
      routeComparison: options.routeComparison,
      routeComparisonPath: options.routeComparisonPath,
    });
  const profiles = options.profiles || defaultProfiles;
  const detector = options.detectProject || detectFrameworkProject;
  const commandRunner = options.runCommand || runCommand;
  const liveBuilds = profiles.map((profile) =>
    liveBuildRow(profile, {
      liveEnabled,
      detector,
      commandRunner,
      timeoutMs: options.timeoutMs || 120000,
    })
  );
  const liveBuildsRun = liveBuilds.some((row) => row.status === "passed" || row.status === "failed");

  return {
    generated_at: generatedAt,
    report_id: "forge-live-framework-harness-v1",
    source: "benchmarks/compare-forge-live-frameworks.ts",
    static_floor_report: {
      source: staticReport.source_route_comparison,
      framework_count: staticReport.frameworks.length,
      route_count: staticReport.required_routes.length,
      dxwww_brotli_bytes: staticReport.frameworks[0].total_brotli_bytes,
      competitor_floor_brotli_bytes: Object.fromEntries(
        staticReport.frameworks
          .slice(1)
          .map((framework) => [framework.framework, framework.total_brotli_bytes])
      ),
      scope: staticReport.scope,
    },
    live_frameworks: {
      enabled: liveEnabled,
      builds_run: liveBuildsRun,
      package_installs_run: false,
      rows: liveBuilds,
    },
    separation_contract: [
      "Static-floor rows are deterministic local fixtures and are always generated.",
      "Live framework rows are opt-in and run only when DX_FORGE_LIVE_FRAMEWORKS=1.",
      "The live harness never runs npm install, pnpm install, yarn install, bun install, or package manager audit commands.",
      "Missing node_modules or package scripts produce skipped rows, not fake benchmark numbers.",
      "Do not merge static-floor and live-build rows into one winner without labeling the evidence type.",
    ],
    honest_scope: [
      "This harness proves the comparison workflow, not broad framework superiority.",
      "Live rows measure only already-installed local baseline projects.",
      "Static floors remain useful for conservative payload direction when live toolchains are unavailable.",
      "A production benchmark claim still needs route parity, browser timing, CDN transfer, and interaction evidence.",
    ],
  };
}

function liveBuildRow(profile, { liveEnabled, detector, commandRunner, timeoutMs }) {
  const detected = detector(profile);
  const base = {
    framework: profile.framework,
    project_dir: profile.project_dir,
    static_floor_id: profile.static_floor_id,
    live_enabled: liveEnabled,
    package_install_run: false,
    detected,
  };

  if (!liveEnabled) {
    return {
      ...base,
      status: "skipped",
      reason: "Set DX_FORGE_LIVE_FRAMEWORKS=1 to run installed local framework builds.",
      build_command: profile.build_command ? commandToString(profile.build_command) : null,
      output_summary: outputSummary(profile),
    };
  }

  if (!detected.project_exists) {
    return {
      ...base,
      status: "skipped",
      reason: "Framework project directory is missing.",
      build_command: profile.build_command ? commandToString(profile.build_command) : null,
      output_summary: outputSummary(profile),
    };
  }

  if (!detected.node_modules_exists) {
    return {
      ...base,
      status: "skipped",
      reason: "node_modules is missing; the live harness will not install packages.",
      build_command: profile.build_command ? commandToString(profile.build_command) : null,
      output_summary: outputSummary(profile),
    };
  }

  if (!profile.build_command) {
    return {
      ...base,
      status: "not-applicable",
      reason: "This baseline is served from checked-in static files and has no build command.",
      build_command: null,
      output_summary: outputSummary(profile),
    };
  }

  if (!detected.build_script_exists) {
    return {
      ...base,
      status: "skipped",
      reason: "package.json has no build script.",
      build_command: commandToString(profile.build_command),
      output_summary: outputSummary(profile),
    };
  }

  const started = Date.now();
  const result = commandRunner(profile.build_command[0], profile.build_command[1], {
    cwd: profile.project_dir,
    timeoutMs,
  });
  const durationMs = Date.now() - started;
  return {
    ...base,
    status: result.status === 0 ? "passed" : "failed",
    reason: result.status === 0 ? "build command passed" : "build command failed",
    build_command: commandToString(profile.build_command),
    duration_ms: durationMs,
    exit_code: result.status,
    stdout_tail: tail(result.stdout),
    stderr_tail: tail(result.stderr),
    output_summary: outputSummary(profile),
  };
}

function detectFrameworkProject(profile) {
  const packageJsonPath = path.join(profile.project_dir, "package.json");
  const packageJson = readPackageJson(packageJsonPath);
  const hasBuildCommand = Boolean(packageJson?.scripts?.build);
  return {
    project_exists: fs.existsSync(profile.project_dir),
    package_json_exists: fs.existsSync(packageJsonPath),
    node_modules_exists: fs.existsSync(path.join(profile.project_dir, "node_modules")),
    build_script_exists: hasBuildCommand,
    package_name: packageJson?.name || null,
    package_version: packageJson?.version || null,
  };
}

function outputSummary(profile) {
  return profile.output_dirs.map((relative) => {
    const dir = path.join(profile.project_dir, relative);
    return {
      path: dir,
      exists: fs.existsSync(dir),
      file_count: countFiles(dir),
      total_bytes: sumFileBytes(dir),
    };
  });
}

function runCommand(command, args, options) {
  const result = spawnSync(command, args, {
    cwd: options.cwd,
    timeout: options.timeoutMs,
    windowsHide: true,
    encoding: "utf8",
    shell: process.platform === "win32",
  });
  return {
    status: typeof result.status === "number" ? result.status : 1,
    stdout: result.stdout || "",
    stderr: result.stderr || result.error?.message || "",
  };
}

function readPackageJson(filePath) {
  try {
    return JSON.parse(fs.readFileSync(filePath, "utf8"));
  } catch (_) {
    return null;
  }
}

function countFiles(dir) {
  if (!fs.existsSync(dir)) {
    return 0;
  }
  let count = 0;
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      count += countFiles(fullPath);
    } else if (entry.isFile()) {
      count += 1;
    }
  }
  return count;
}

function sumFileBytes(dir) {
  if (!fs.existsSync(dir)) {
    return 0;
  }
  let total = 0;
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      total += sumFileBytes(fullPath);
    } else if (entry.isFile()) {
      total += fs.statSync(fullPath).size;
    }
  }
  return total;
}

function commandToString(command) {
  return [command[0], ...command[1]].join(" ");
}

function tail(value, maxLength = 1200) {
  const text = String(value || "").trim();
  if (text.length <= maxLength) {
    return text;
  }
  return text.slice(text.length - maxLength);
}

function renderMarkdown(report) {
  const lines = [
    "# Forge Live Framework Harness",
    "",
    `Generated: ${report.generated_at}`,
    "",
    "This report separates deterministic static-floor evidence from opt-in live framework builds.",
    "",
    "## Static Floor",
    "",
    `- Source: \`${report.static_floor_report.source}\``,
    `- Routes: \`${report.static_floor_report.route_count}\``,
    `- DX-WWW Brotli: \`${report.static_floor_report.dxwww_brotli_bytes}\` B`,
    `- Competitor builds run for static floor: \`${!report.static_floor_report.scope.competitor_builds_not_run}\``,
    `- Package installs run for static floor: \`${!report.static_floor_report.scope.no_package_install}\``,
    "",
    "## Live Builds",
    "",
    `- Enabled: \`${report.live_frameworks.enabled}\``,
    `- Builds run: \`${report.live_frameworks.builds_run}\``,
    `- Package installs run: \`${report.live_frameworks.package_installs_run}\``,
    "",
    "| Framework | Status | Project | Build command | Reason | Outputs |",
    "| --- | --- | --- | --- | --- | --- |",
    ...report.live_frameworks.rows.map((row) =>
      [
        row.framework,
        row.status,
        `\`${row.project_dir}\``,
        row.build_command ? `\`${row.build_command}\`` : "`none`",
        row.reason,
        row.output_summary
          .map((output) => `${path.basename(output.path)}:${output.exists ? output.total_bytes : "missing"}`)
          .join(", "),
      ].join(" | ")
    ).map((row) => `| ${row} |`),
    "",
    "## Separation Contract",
    "",
    ...report.separation_contract.map((item) => `- ${item}`),
    "",
    "## Honest Scope",
    "",
    ...report.honest_scope.map((item) => `- ${item}`),
    "",
    "## Static Competitor Detail",
    "",
    renderStaticCompetitorMarkdown({
      generated_at: report.generated_at,
      source_route_comparison: report.static_floor_report.source,
      scope: report.static_floor_report.scope,
      frameworks: [
        {
          framework: "DX-WWW",
          baseline_id: "measured-forge-public-routes",
          baseline_kind: "measured-static-routes",
          route_count: report.static_floor_report.route_count,
          total_decoded_bytes: 0,
          total_brotli_bytes: report.static_floor_report.dxwww_brotli_bytes,
          note: "Measured public Forge static routes from the source route comparison.",
        },
        ...Object.entries(report.static_floor_report.competitor_floor_brotli_bytes).map(
          ([framework, bytes]) => ({
            framework,
            baseline_id: `${framework.toLowerCase().replace(/[^a-z0-9]+/g, "-")}-static-floor`,
            baseline_kind: "static-floor",
            route_count: report.static_floor_report.route_count,
            total_decoded_bytes: 0,
            total_brotli_bytes: bytes,
            note: "Static-floor summary carried through from the competitor evidence report.",
          })
        ),
      ],
      route_comparisons: [],
      honest_findings: report.honest_scope,
      conclusion: "See the live-build rows above for opt-in installed framework evidence.",
    }).split("\n").slice(0, 12).join("\n"),
    "",
  ];
  return lines.join("\n");
}

function writeReport(report) {
  fs.mkdirSync(reportDir, { recursive: true });
  fs.writeFileSync(outJsonPath, `${JSON.stringify(report, null, 2)}\n`);
  fs.writeFileSync(outMdPath, renderMarkdown(report));
}

function main() {
  const report = buildReport();
  writeReport(report);
  console.log(
    JSON.stringify(
      {
        report: [path.relative(root, outJsonPath), path.relative(root, outMdPath)],
        live_enabled: report.live_frameworks.enabled,
        live_builds_run: report.live_frameworks.builds_run,
        package_installs_run: report.live_frameworks.package_installs_run,
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
  renderMarkdown,
  detectFrameworkProject,
  liveBuildRow,
};
