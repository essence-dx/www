const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");
const {
  scanConflictMarkers,
} = require("../next-rust-merge/conflict-marker-check.cjs");
const {
  runSchemaStatusNoiseCheck,
} = require("../next-rust-merge/schema-status-noise-check.cjs");
const {
  DATABASE_API_READINESS_ROUTE_PROBE,
  DEFAULT_LAUNCH_ROUTE_PROBES,
  probeLaunchRouteCandidates,
  probeLaunchRoutes,
} = require("./route-probe.cjs");

const LAUNCH_COORDINATOR_SCHEMA = "dx.www.launchStabilize.coordinator";
const AUDIT_FLOOR_SCORE = 40;
const DIRTY_OWNER_SAMPLE_PATH_LIMIT = 5;

const REQUIRED_MINIMAL_SOURCE_PATHS = Object.freeze([
  "examples/template/app",
  "examples/template/components",
  "examples/template/server",
  "examples/template/styles",
]);

const REQUIRED_TEMPLATE_ROUTE_SOURCE_FILES = Object.freeze([
  "examples/template/app/page.tsx",
  "examples/template/app/dashboard/page.tsx",
  "examples/template/app/login/page.tsx",
  "examples/template/app/logout/page.tsx",
]);

const TEMPLATE_NODE_MODULE_PATHS = Object.freeze([
  "examples/template/node_modules",
  ".dx/template-app-browser-preview/node_modules",
]);

const TEMPLATE_ARTIFACT_ROOTS = Object.freeze([
  "examples/template",
]);

const FORBIDDEN_TEMPLATE_EXTENSIONS = new Set([".pg", ".cp", ".cjs"]);

const GENERATED_NOISE_PREFIXES = Object.freeze([
  ".tmp/",
  ".dx/receipts/next-rust/",
  ".dx/template-app-browser-preview/",
  "examples/template/.dx/forge/cache/",
  "examples/template/.dx/forge/receipts/202",
  "examples/template/.dx/forge/receipts/safety/",
]);

const LANE14_OUTPUT_PREFIXES = Object.freeze([
  "tools/launch-stabilize/",
  "tools/launch/launch-route-smoke.js",
  "benchmarks/launch-stabilize-",
  "tools/next-rust-merge/",
  "benchmarks/next-rust-",
]);

const LAUNCH_PUBLIC_SCHEMA_NOISE_FILES = Object.freeze([
  "docs/NEXTJS_COMPATIBILITY_MAP.md",
  "dx-www/src/diagnostics.rs",
  "dx-www/src/cli/app_route_handler_receipt.rs",
  "dx-www/src/cli/mod.rs",
  "core/src/delivery/server_contract.rs",
  "core/src/ecosystem/dx_check_receipt.rs",
]);

const LAUNCH_PUBLIC_SCHEMA_NOISE_ROOTS = Object.freeze([
  "docs",
  "dx-www/src",
  "core/src",
  "tools/launch-stabilize",
]);

const LAUNCH_PUBLIC_SCHEMA_NOISE_EXTENSIONS = new Set([
  ".cjs",
  ".js",
  ".json",
  ".md",
  ".rs",
  ".ts",
  ".tsx",
]);

const LAUNCH_PUBLIC_SCHEMA_NOISE_IGNORED_DIRS = new Set([
  ".dx",
  ".git",
  "node_modules",
  "target",
  "vendor",
]);

const LAUNCH_STATUS_OVERCLAIM_RULE_IDS = new Set([
  "full-next-parity-overclaim",
  "production-merge-ready-overclaim",
]);

const DEFAULT_LIVE_ROUTE_BASE_URL = "http://127.0.0.1:3000";
const ACCEPTED_PROVEN_CHECK_IDS = new Set([
  "dx-style-compile-proof",
  "dx-build-tiny-app-proof",
  "server-data-json-proof",
  "giant-cli-mod-risk",
]);

function buildLaunchCoordinatorReport({
  cwd = process.cwd(),
  generatedAt = new Date().toISOString(),
  gitStatusText = null,
  provenCheckIds = [],
  routeProbeReport = null,
  gitDiffCheckReport = null,
  requireGitDiffCheck = false,
} = {}) {
  const proven = new Set(provenCheckIds);
  const schemaStatusNoiseReport = launchSchemaStatusNoiseReport(cwd);
  const checks = [
    minimalSourceLayoutCheck(cwd),
    requiredTemplateRouteSourcesCheck(cwd),
    noTemplateNodeModulesCheck(cwd),
    noTemplateFakeArtifactsCheck(cwd),
    mergeConflictMarkersCheck(cwd),
    ...(gitDiffCheckReport
      ? [gitDiffWhitespaceCheck(gitDiffCheckReport)]
      : requireGitDiffCheck
        ? [missingGitDiffWhitespaceCheck()]
        : []),
    liveRouteProbeCheck(routeProbeReport),
    proofRequiredCheck({
      id: "dx-style-compile-proof",
      ownerLane: 1,
      proven,
      evidence:
        "dx-style blocks launch until the focused style crate compile/check proof passes",
      nextAction:
        "fix related-crates/style compile blockers and run the smallest dx-style cargo proof",
    }),
    proofRequiredCheck({
      id: "dx-build-tiny-app-proof",
      ownerLane: 2,
      proven,
      evidence:
        "dx build is not launch-proven for a tiny app with app, components, server, styles, CSS, assets, and receipts",
      nextAction:
        "prove dx build on a tiny source-owned app without template-local node_modules",
    }),
    proofRequiredCheck({
      id: "server-data-json-proof",
      ownerLane: 3,
      proven,
      evidence:
        "server-data.json source exists but launch requires a focused emission proof",
      nextAction:
        "run or repair the App Router server-data build contract until .dx/build/app/server-data.json is proven",
    }),
    generatedPreviewNoiseCheck(cwd, gitStatusText),
    dirtyWorktreeLaneRiskCheck(cwd, gitStatusText),
    dirtyWorktreeOwnerCoverageCheck(cwd, gitStatusText),
    publicSchemaVersionNoiseCheck(cwd, schemaStatusNoiseReport),
    launchStatusOverclaimsCheck(cwd, schemaStatusNoiseReport),
    giantCliModRiskCheck(cwd, proven),
  ];
  const blockers = checks.filter((entry) => entry.blocking && entry.status !== "passed");
  const readinessGaps = checks.filter((entry) => entry.status !== "passed");
  const nonBlockingReadinessGaps = readinessGaps.filter((entry) => !entry.blocking);
  const ownerLaneSummary = summarizeReadinessByOwner(readinessGaps);
  const laneCleanupPlan = buildLaneCleanupPlan(checks);
  const launchScore =
    blockers.length === 0 ? 100 : Math.max(AUDIT_FLOOR_SCORE, 100 - blockers.length * 12);
  const finalReadinessScore =
    readinessGaps.length === 0
      ? 100
      : Math.max(
          AUDIT_FLOOR_SCORE,
          100 - blockers.length * 12 - nonBlockingReadinessGaps.length * 8,
        );

  return {
    schema: LAUNCH_COORDINATOR_SCHEMA,
    format: 1,
    lane: 14,
    laneName: "Final Launch Coordinator",
    mission: "launch-stabilize",
    featureImplementation: false,
    executionMode: "launch-stabilize-read-only",
    status: blockers.length === 0 ? "passing" : "blocked",
    generatedAt,
    auditFloorScore: AUDIT_FLOOR_SCORE,
    launchScore,
    finalReadinessScore,
    hundredPercentReady: readinessGaps.length === 0,
    readinessSummary: {
      blockerCount: blockers.length,
      nonBlockingGapCount: nonBlockingReadinessGaps.length,
      totalGapCount: readinessGaps.length,
      finalReadinessScore,
    },
    ownerLaneSummary,
    laneCleanupPlan,
    architecture: launchArchitecture(),
    checks,
    blockers,
    readinessGaps,
    nextActionForCoordinator:
      blockers[0]?.nextAction ||
      readinessGaps[0]?.nextAction ||
      "run the next stronger launch smoke after lane proofs land",
  };
}

async function runLaunchCoordinatorCli({
  cwd = process.cwd(),
  argv = process.argv.slice(2),
  stdout = process.stdout,
  stderr = process.stderr,
  generatedAt = new Date().toISOString(),
} = {}) {
  try {
    const options = parseCoordinatorCliArgs(argv);
    const routeProbeReport = options.probeLiveRoutes
      ? await runLiveRouteProbe({ options, generatedAt })
      : null;
    const gitDiffCheckReport = options.checkGitDiff ? runGitDiffCheck(cwd) : null;
    const report = buildLaunchCoordinatorReport({
      cwd,
      generatedAt,
      provenCheckIds: options.provenCheckIds,
      routeProbeReport,
      gitDiffCheckReport,
      requireGitDiffCheck: options.strictFinal || options.checkGitDiff,
    });

    stdout.write(`${JSON.stringify(report, null, 2)}\n`);
    return { exitCode: coordinatorExitCode(report, options), report };
  } catch (error) {
    stderr.write(`${error.message}\n`);
    return { exitCode: 1, report: null };
  }
}

function coordinatorExitCode(report, options) {
  if (options.strictFinal) {
    return report.hundredPercentReady ? 0 : 1;
  }

  return report.status === "passing" ? 0 : 1;
}

function parseCoordinatorCliArgs(argv) {
  const options = {
    probeLiveRoutes: false,
    baseUrl: DEFAULT_LIVE_ROUTE_BASE_URL,
    baseUrlCandidates: null,
    timeoutMs: 2000,
    provenCheckIds: [],
    routes: DEFAULT_LAUNCH_ROUTE_PROBES,
    strictFinal: false,
    checkGitDiff: false,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--probe-live-routes") {
      options.probeLiveRoutes = true;
    } else if (arg === "--base-url") {
      options.baseUrl = requireCliValue(argv, (index += 1), arg);
    } else if (arg === "--base-url-candidates") {
      options.baseUrlCandidates = parseBaseUrlCandidates(
        requireCliValue(argv, (index += 1), arg),
      );
    } else if (arg === "--timeout-ms") {
      options.timeoutMs = Number(requireCliValue(argv, (index += 1), arg));
    } else if (
      arg === "--include-database-readiness" ||
      arg === "--include-database-readiness-boundary"
    ) {
      options.routes = withDatabaseApiReadinessProbe(options.routes);
    } else if (arg === "--proven-check") {
      options.provenCheckIds.push(requireCliValue(argv, (index += 1), arg));
    } else if (arg === "--strict-final") {
      options.strictFinal = true;
    } else if (arg === "--check-git-diff") {
      options.checkGitDiff = true;
    } else if (arg === "--json") {
      continue;
    } else {
      throw new Error(`Unknown launch coordinator argument: ${arg}`);
    }
  }

  validateProvenCheckIds(options.provenCheckIds);

  if (!Number.isFinite(options.timeoutMs) || options.timeoutMs < 1) {
    throw new Error("--timeout-ms must be a positive number");
  }

  return options;
}

function withDatabaseApiReadinessProbe(routes) {
  const values = Array.isArray(routes) ? routes : DEFAULT_LAUNCH_ROUTE_PROBES;
  if (values.some((route) => route?.path === DATABASE_API_READINESS_ROUTE_PROBE.path)) {
    return values;
  }

  return [...values, DATABASE_API_READINESS_ROUTE_PROBE];
}

function validateProvenCheckIds(provenCheckIds) {
  for (const id of provenCheckIds) {
    if (!ACCEPTED_PROVEN_CHECK_IDS.has(id)) {
      throw new Error(`Unknown launch coordinator proof id: ${id}`);
    }
  }
}

async function runLiveRouteProbe({ options, generatedAt }) {
  if (!options.baseUrlCandidates) {
    return probeLaunchRoutes({
      baseUrl: options.baseUrl,
      routes: options.routes,
      timeoutMs: options.timeoutMs,
      generatedAt,
    });
  }

  const candidateReport = await probeLaunchRouteCandidates({
    baseUrls: options.baseUrlCandidates,
    routes: options.routes,
    timeoutMs: options.timeoutMs,
    generatedAt,
  });

  return {
    ...candidateReport.selectedReport,
    candidateProbeSchema: candidateReport.schema,
    candidateAttempts: candidateReport.attempts,
  };
}

function parseBaseUrlCandidates(value) {
  const candidates = value
    .split(",")
    .map((entry) => entry.trim())
    .filter(Boolean);

  if (candidates.length === 0) {
    throw new Error("--base-url-candidates requires at least one URL");
  }

  return candidates;
}

function requireCliValue(argv, index, flag) {
  const value = argv[index];
  if (!value || value.startsWith("--")) {
    throw new Error(`${flag} requires a value`);
  }
  return value;
}

function minimalSourceLayoutCheck(cwd) {
  const missing = REQUIRED_MINIMAL_SOURCE_PATHS.filter(
    (entry) => !fs.existsSync(path.join(cwd, entry)),
  );

  return launchCheck({
    id: "minimal-source-layout",
    ownerLane: 4,
    status: missing.length === 0 ? "passed" : "blocked",
    evidence:
      missing.length === 0
        ? "minimal launch source directories exist"
        : `missing minimal source directories: ${missing.join(", ")}`,
    nextAction: "restore app, components, server, and styles in the source-owned launch template",
    details: { requiredPaths: [...REQUIRED_MINIMAL_SOURCE_PATHS], missing },
  });
}

function requiredTemplateRouteSourcesCheck(cwd) {
  const missing = REQUIRED_TEMPLATE_ROUTE_SOURCE_FILES.filter(
    (entry) => !fs.existsSync(path.join(cwd, entry)),
  );
  const sourceOnlyShadows = REQUIRED_TEMPLATE_ROUTE_SOURCE_FILES.map(
    (entry) => `${entry}.source-only`,
  ).filter((entry) => fs.existsSync(path.join(cwd, entry)));
  const blocked = missing.length > 0 || sourceOnlyShadows.length > 0;

  return launchCheck({
    id: "required-template-route-sources",
    ownerLane: 10,
    status: blocked ? "blocked" : "passed",
    evidence: blocked
      ? `missing route source files: ${missing.length}; source-only route shadows: ${sourceOnlyShadows.length}`
      : "launch-critical template route source files are present without source-only shadows",
    nextAction:
      "restore real App Router TSX source files for /, /dashboard, /login, and /logout before final www signoff",
    details: {
      requiredFiles: [...REQUIRED_TEMPLATE_ROUTE_SOURCE_FILES],
      missing,
      sourceOnlyShadows,
    },
  });
}

function noTemplateNodeModulesCheck(cwd) {
  const present = TEMPLATE_NODE_MODULE_PATHS.filter((entry) =>
    fs.existsSync(path.join(cwd, entry)),
  );

  return launchCheck({
    id: "no-template-node-modules",
    ownerLane: 9,
    status: present.length === 0 ? "passed" : "blocked",
    evidence:
      present.length === 0
        ? "template-local node_modules directories are absent"
        : `template-local node_modules directories exist: ${present.join(", ")}`,
    nextAction: "remove template-local node_modules assumptions from launch paths",
    details: { checkedPaths: [...TEMPLATE_NODE_MODULE_PATHS], present },
  });
}

function noTemplateFakeArtifactsCheck(cwd) {
  const artifacts = findForbiddenTemplateArtifacts(cwd);

  return launchCheck({
    id: "no-template-fake-artifacts",
    ownerLane: 4,
    status: artifacts.length === 0 ? "passed" : "blocked",
    evidence:
      artifacts.length === 0
        ? "no .pg, .cp, or .cjs template artifacts were found in launch template roots"
        : `forbidden template artifacts found: ${artifacts.slice(0, 8).join(", ")}`,
    nextAction: "move generated/template artifacts behind source materializers",
    details: { forbiddenExtensions: [...FORBIDDEN_TEMPLATE_EXTENSIONS], artifacts },
  });
}

function mergeConflictMarkersCheck(cwd) {
  const report = scanConflictMarkers({ cwd });

  return launchCheck({
    id: "merge-conflict-markers",
    ownerLane: 14,
    status: report.status === "passed" ? "passed" : "blocked",
    evidence:
      report.status === "passed"
        ? `no merge conflict markers in ${report.scannedFileCount} merge-sensitive files`
        : `merge conflict markers or missing scan targets found: ${report.markers.length} markers, ${report.missingTargets.length} missing targets`,
    nextAction: "resolve conflict markers before launch scoring can be trusted",
    details: {
      scannedFileCount: report.scannedFileCount,
      missingTargets: report.missingTargets,
      markers: report.markers,
    },
  });
}

function gitDiffWhitespaceCheck(report) {
  const exitCode = Number(report.exitCode ?? 1);
  const passed = exitCode === 0;

  return launchCheck({
    id: "git-diff-whitespace",
    ownerLane: 14,
    status: passed ? "passed" : "blocked",
    evidence: passed
      ? "git diff --check passed"
      : `git diff --check failed with exit code ${exitCode}`,
    nextAction:
      "fix whitespace errors reported by git diff --check before final launch signoff",
    details: {
      command: report.command || "git diff --check",
      exitCode,
      outputLines: gitDiffCheckOutputLines(report),
    },
  });
}

function missingGitDiffWhitespaceCheck() {
  return launchCheck({
    id: "git-diff-whitespace",
    ownerLane: 14,
    status: "blocked",
    evidence: "strict final launch requires git diff --check proof",
    nextAction:
      "rerun the launch coordinator with --check-git-diff before final launch signoff",
    details: {
      supplied: false,
      requiredCommand: "git diff --check",
    },
  });
}

function liveRouteProbeCheck(routeProbeReport) {
  if (!routeProbeReport) {
    return launchCheck({
      id: "live-route-probe",
      ownerLane: 14,
      blocking: false,
      status: "skipped",
      evidence:
        "no live route probe report was supplied; run the probe when the Rust dev server is already running",
      nextAction:
        "node tools/launch-stabilize/route-probe.cjs --base-url http://127.0.0.1:3000 --json",
      details: { supplied: false },
    });
  }

  const failedCount = Number(routeProbeReport.failedCount || 0);
  const adapterBoundaryCount = Number(routeProbeReport.adapterBoundaryCount || 0);
  const passedCount = Number(routeProbeReport.passedCount || 0);
  const connectivity = routeProbeReport.connectivity || null;
  const routes = Array.isArray(routeProbeReport.routes) ? routeProbeReport.routes : [];
  const failedRoutes = summarizeRouteProbeRoutes(routes, "failed");
  const adapterBoundaryRoutes = summarizeRouteProbeRoutes(routes, "adapter-boundary");
  const passedRoutes = routes
    .filter((route) => route?.status === "passed" && typeof route.path === "string")
    .map((route) => route.path);
  const failed = routeProbeReport.status === "failed" || failedCount > 0;
  const status = failed
    ? "failed"
    : adapterBoundaryCount > 0
      ? "adapter-boundary"
      : "passed";

  return launchCheck({
    id: "live-route-probe",
    ownerLane: 14,
    handoffLane: failed ? 8 : null,
    blocking: failed,
    status,
    evidence: liveRouteProbeEvidence(
      status,
      passedCount,
      adapterBoundaryCount,
      failedCount,
      connectivity,
      failedRoutes,
    ),
    nextAction: failed
      ? "restore the Rust/Axum dev server and route responses on port 3000 before final launch signoff"
      : adapterBoundaryCount > 0
        ? "keep adapter-boundary 501 routes visible until route-owner lanes replace them"
        : "keep the live route probe in the final launch gate",
    details: {
      supplied: true,
      schema: routeProbeReport.schema,
      format: routeProbeReport.format,
      baseUrl: routeProbeReport.baseUrl,
      passedCount,
      adapterBoundaryCount,
      failedCount,
      connectivity,
      failedRoutes,
      adapterBoundaryRoutes,
      passedRoutes,
      candidateProbeSchema: routeProbeReport.candidateProbeSchema || null,
      candidateAttempts: Array.isArray(routeProbeReport.candidateAttempts)
        ? routeProbeReport.candidateAttempts
        : [],
      routes,
    },
  });
}

function liveRouteProbeEvidence(
  status,
  passedCount,
  adapterBoundaryCount,
  failedCount,
  connectivity = null,
  failedRoutes = [],
) {
  if (connectivity?.status === "unreachable") {
    return `live launch server is unreachable: ${connectivity.evidence}`;
  }

  if (status === "passed") {
    return `${passedCount} live launch route probes passed`;
  }

  if (status === "adapter-boundary") {
    return `${passedCount} live route probes passed and ${adapterBoundaryCount} expected adapter-boundary route probes remain visible`;
  }

  const failedPathSummary =
    failedRoutes.length > 0
      ? `: ${failedRoutes.map((route) => route.path).join(", ")}`
      : "";
  const connectivitySummary =
    connectivity?.evidence && connectivity.status !== "reachable"
      ? ` (${connectivity.evidence})`
      : "";

  return `${failedCount} live launch route probes failed${failedPathSummary}${connectivitySummary}`;
}

function summarizeRouteProbeRoutes(routes, status) {
  return routes
    .filter((route) => route?.status === status && typeof route.path === "string")
    .map((route) => ({
      path: route.path,
      label: route.label || route.path,
      actualStatus: route.actualStatus ?? null,
      elapsedMs: Number.isFinite(Number(route.elapsedMs)) ? Number(route.elapsedMs) : null,
      evidence: typeof route.evidence === "string" ? route.evidence : "",
    }));
}

function proofRequiredCheck({ id, ownerLane, proven, evidence, nextAction }) {
  return launchCheck({
    id,
    ownerLane,
    status: proven.has(id) ? "passed" : "blocked",
    evidence: proven.has(id) ? "focused proof was supplied to the launch coordinator" : evidence,
    nextAction,
  });
}

function generatedPreviewNoiseCheck(cwd, gitStatusText) {
  const statusText = gitStatusText === null ? readGitStatusShort(cwd) : gitStatusText;
  const noisyEntries = parseGitStatusPaths(statusText).filter((entry) =>
    isGeneratedNoisePath(entry.path),
  );

  return launchCheck({
    id: "generated-preview-noise",
    ownerLane: 7,
    status: noisyEntries.length === 0 ? "passed" : "blocked",
    evidence:
      noisyEntries.length === 0
        ? "no generated preview/cache/receipt churn is present in git status"
        : `${noisyEntries.length} generated preview/cache/receipt status entries are present`,
    nextAction:
      "move or ignore generated preview, cache, and receipt churn so source materializers stay authoritative",
    details: { noisyEntries: noisyEntries.slice(0, 25) },
  });
}

function dirtyWorktreeLaneRiskCheck(cwd, gitStatusText) {
  const statusText = gitStatusText === null ? readGitStatusShort(cwd) : gitStatusText;
  const entries = parseGitStatusPaths(statusText);
  const generatedNoiseEntries = entries.filter((entry) => isGeneratedNoisePath(entry.path));
  const lane14Entries = entries.filter((entry) => isLane14OutputPath(entry.path));
  const sourceOrOtherEntries = entries.filter(
    (entry) => !isGeneratedNoisePath(entry.path) && !isLane14OutputPath(entry.path),
  );

  return launchCheck({
    id: "dirty-worktree-lane-risk",
    ownerLane: 14,
    status: entries.length === 0 ? "passed" : "blocked",
    evidence:
      entries.length === 0
        ? "git status is clean for final launch coordination"
        : `${entries.length} git status entries require lane-safe cleanup before final launch signoff`,
    nextAction:
      "land, shelve, or explicitly assign dirty lane outputs before final launch scoring",
    details: {
      totalCount: entries.length,
      generatedNoiseCount: generatedNoiseEntries.length,
      lane14OutputCount: lane14Entries.length,
      sourceOrOtherCount: sourceOrOtherEntries.length,
      sourceOrOtherEntries: sourceOrOtherEntries.slice(0, 25),
      generatedNoiseEntries: generatedNoiseEntries.slice(0, 25),
      lane14OutputEntries: lane14Entries.slice(0, 25),
      ownerHints: summarizeDirtyOwnerHints(entries),
    },
  });
}

function dirtyWorktreeOwnerCoverageCheck(cwd, gitStatusText) {
  const statusText = gitStatusText === null ? readGitStatusShort(cwd) : gitStatusText;
  const entries = parseGitStatusPaths(statusText);
  const unclassifiedEntries = entries.filter(
    (entry) => dirtyEntryOwnerHint(entry.path).ownerLane === null,
  );

  return launchCheck({
    id: "dirty-worktree-owner-coverage",
    ownerLane: 14,
    status: unclassifiedEntries.length === 0 ? "passed" : "blocked",
    evidence:
      unclassifiedEntries.length === 0
        ? "every dirty status entry has a launch lane owner hint"
        : `${unclassifiedEntries.length} dirty status ${
            unclassifiedEntries.length === 1 ? "entry has" : "entries have"
          } no lane owner`,
    nextAction:
      "add a narrow lane owner hint, land the owning lane output, or remove the stray dirty file before final coordination",
    details: {
      unclassifiedCount: unclassifiedEntries.length,
      unclassifiedEntries: unclassifiedEntries.slice(0, 25),
    },
  });
}

function launchSchemaStatusNoiseReport(cwd) {
  return runSchemaStatusNoiseCheck({
    cwd,
    files: collectLaunchPublicSchemaNoiseFiles(cwd),
  });
}

function publicSchemaVersionNoiseCheck(cwd, report = launchSchemaStatusNoiseReport(cwd)) {
  const violations = report.violations.filter(
    (violation) => violation.id === "public-schema-version-suffix",
  );

  return launchCheck({
    id: "public-schema-version-noise",
    ownerLane: 14,
    status: violations.length === 0 ? "passed" : "blocked",
    evidence:
      violations.length === 0
        ? "launch-sensitive source files do not expose public schema version suffix names"
        : `${violations.length} public schema version suffix references remain in launch-sensitive files`,
    nextAction:
      "replace public schema suffix names with stable names plus numeric format fields, or document narrow compatibility exceptions",
    details: {
      scannedFiles: report.scannedFiles.map((filePath) =>
        normalizePath(path.relative(cwd, filePath)),
      ),
      violationCount: violations.length,
      violations: violations.slice(0, 25).map((violation) =>
        sanitizeSchemaNoiseViolation(cwd, violation),
      ),
    },
  });
}

function launchStatusOverclaimsCheck(cwd, report = launchSchemaStatusNoiseReport(cwd)) {
  const violations = report.violations.filter((violation) =>
    LAUNCH_STATUS_OVERCLAIM_RULE_IDS.has(violation.id),
  );

  return launchCheck({
    id: "launch-status-overclaims",
    ownerLane: 14,
    status: violations.length === 0 ? "passed" : "blocked",
    evidence:
      violations.length === 0
        ? "launch-sensitive source files do not contain final-readiness overclaims"
        : `${violations.length} launch status overclaim references remain in launch-sensitive files`,
    nextAction:
      "replace launch overclaims with proven, adapter-boundary, skipped, or unimplemented status language",
    details: {
      scannedFiles: report.scannedFiles.map((filePath) =>
        normalizePath(path.relative(cwd, filePath)),
      ),
      violationCount: violations.length,
      violations: violations.slice(0, 25).map((violation) =>
        sanitizeSchemaNoiseViolation(cwd, violation),
      ),
    },
  });
}

function collectLaunchPublicSchemaNoiseFiles(cwd) {
  const files = new Set(LAUNCH_PUBLIC_SCHEMA_NOISE_FILES);

  for (const root of LAUNCH_PUBLIC_SCHEMA_NOISE_ROOTS) {
    collectSchemaNoiseFilesFromRoot(cwd, root, files);
  }

  return [...files].sort();
}

function collectSchemaNoiseFilesFromRoot(cwd, relativeRoot, files) {
  const absoluteRoot = path.join(cwd, relativeRoot);
  if (!fs.existsSync(absoluteRoot)) return;

  const entry = fs.statSync(absoluteRoot);
  if (entry.isFile()) {
    if (shouldScanSchemaNoiseFile(relativeRoot)) {
      files.add(normalizePath(relativeRoot));
    }
    return;
  }

  if (!entry.isDirectory()) return;

  const children = fs.readdirSync(absoluteRoot, { withFileTypes: true });
  for (const child of children) {
    if (child.isDirectory() && LAUNCH_PUBLIC_SCHEMA_NOISE_IGNORED_DIRS.has(child.name)) {
      continue;
    }

    const childRelativePath = normalizePath(path.join(relativeRoot, child.name));
    const childAbsolutePath = path.join(cwd, childRelativePath);
    if (child.isDirectory()) {
      collectSchemaNoiseFilesFromRoot(cwd, childRelativePath, files);
    } else if (child.isFile() && shouldScanSchemaNoiseFile(childRelativePath)) {
      files.add(childRelativePath);
    } else if (!child.isFile() && fs.statSync(childAbsolutePath).isDirectory()) {
      collectSchemaNoiseFilesFromRoot(cwd, childRelativePath, files);
    }
  }
}

function shouldScanSchemaNoiseFile(relativePath) {
  return LAUNCH_PUBLIC_SCHEMA_NOISE_EXTENSIONS.has(path.extname(relativePath));
}

function giantCliModRiskCheck(cwd, proven) {
  const id = "giant-cli-mod-risk";
  const filePath = path.join(cwd, "dx-www/src/cli/mod.rs");
  const lineCount = fs.existsSync(filePath)
    ? fs.readFileSync(filePath, "utf8").split(/\r?\n/).length
    : 0;
  const passed = proven.has(id) || (lineCount > 0 && lineCount <= 1800);

  return launchCheck({
    id,
    ownerLane: 6,
    status: passed ? "passed" : "blocked",
    evidence: passed
      ? "cli/mod.rs is under the launch coordinator risk threshold or proof was supplied"
      : `dx-www/src/cli/mod.rs is still high-risk at ${lineCount} lines`,
    nextAction:
      "extract one coherent CLI area into a small module before adding more launch behavior",
    details: { file: "dx-www/src/cli/mod.rs", lineCount, riskThreshold: 1800 },
  });
}

function sanitizeSchemaNoiseViolation(cwd, violation) {
  return {
    id: violation.id,
    file: normalizePath(path.relative(cwd, violation.file)),
    line: violation.line,
    column: violation.column,
    matchLength: typeof violation.match === "string" ? violation.match.length : 0,
  };
}

function summarizeReadinessByOwner(readinessGaps) {
  const summaries = new Map();

  for (const gap of readinessGaps) {
    const ownerLane = Number.isInteger(gap.ownerLane) ? gap.ownerLane : null;
    const key = ownerLane === null ? "unassigned" : String(ownerLane);
    const summary = summaries.get(key) || {
      ownerLane,
      blockerCount: 0,
      nonBlockingGapCount: 0,
      totalGapCount: 0,
      checkIds: [],
      nextActions: [],
    };

    if (gap.blocking) {
      summary.blockerCount += 1;
    } else {
      summary.nonBlockingGapCount += 1;
    }
    summary.totalGapCount += 1;

    if (summary.checkIds.length < 8 && !summary.checkIds.includes(gap.id)) {
      summary.checkIds.push(gap.id);
    }
    if (
      typeof gap.nextAction === "string" &&
      gap.nextAction.length > 0 &&
      summary.nextActions.length < 5 &&
      !summary.nextActions.includes(gap.nextAction)
    ) {
      summary.nextActions.push(gap.nextAction);
    }

    summaries.set(key, summary);
  }

  return [...summaries.values()].sort((left, right) => {
    if (right.blockerCount !== left.blockerCount) {
      return right.blockerCount - left.blockerCount;
    }
    if (right.totalGapCount !== left.totalGapCount) {
      return right.totalGapCount - left.totalGapCount;
    }
    return ownerLaneSortValue(left.ownerLane) - ownerLaneSortValue(right.ownerLane);
  });
}

function ownerLaneSortValue(ownerLane) {
  return ownerLane === null ? Number.MAX_SAFE_INTEGER : ownerLane;
}

function buildLaneCleanupPlan(checks) {
  const lanes = new Map();

  for (const check of checks) {
    if (check.status === "passed") continue;
    const entry = laneCleanupEntry(lanes, check.handoffLane ?? check.ownerLane);
    if (check.blocking) {
      entry.blockingCheckIds.push(check.id);
    } else {
      entry.nonBlockingCheckIds.push(check.id);
    }
    if (typeof check.nextAction === "string") {
      entry.nextActions.push(check.nextAction);
    }
  }

  const dirtyRisk = checks.find((entry) => entry.id === "dirty-worktree-lane-risk");
  const ownerHints = Array.isArray(dirtyRisk?.details?.ownerHints)
    ? dirtyRisk.details.ownerHints
    : [];

  for (const hint of ownerHints) {
    const entry = laneCleanupEntry(lanes, hint.ownerLane);
    entry.dirtyCount += hint.count;
    entry.dirtyBuckets.push({
      bucket: hint.bucket,
      count: hint.count,
      samplePaths: hint.samplePaths,
    });
  }

  return [...lanes.values()]
    .map((entry) => ({
      ...entry,
      blockingCheckIds: uniqueStrings(entry.blockingCheckIds),
      nonBlockingCheckIds: uniqueStrings(entry.nonBlockingCheckIds),
      nextActions: uniqueStrings(entry.nextActions).slice(0, 5),
      dirtyBuckets: entry.dirtyBuckets.sort((left, right) => {
        if (right.count !== left.count) return right.count - left.count;
        return left.bucket.localeCompare(right.bucket);
      }),
    }))
    .sort((left, right) => {
      if (right.dirtyCount !== left.dirtyCount) return right.dirtyCount - left.dirtyCount;
      if (right.blockingCheckIds.length !== left.blockingCheckIds.length) {
        return right.blockingCheckIds.length - left.blockingCheckIds.length;
      }
      return ownerLaneSortValue(left.ownerLane) - ownerLaneSortValue(right.ownerLane);
    });
}

function laneCleanupEntry(lanes, ownerLane) {
  const key = ownerLane === null ? "unassigned" : String(ownerLane);
  const existing = lanes.get(key);
  if (existing) return existing;

  const entry = {
    ownerLane,
    dirtyCount: 0,
    dirtyBuckets: [],
    blockingCheckIds: [],
    nonBlockingCheckIds: [],
    nextActions: [],
  };
  lanes.set(key, entry);
  return entry;
}

function uniqueStrings(values) {
  return [...new Set(values.filter((value) => typeof value === "string" && value.length > 0))];
}

function summarizeDirtyOwnerHints(entries) {
  const summaries = new Map();

  for (const entry of entries) {
    const hint = dirtyEntryOwnerHint(entry.path);
    const key = `${hint.ownerLane}:${hint.bucket}`;
    const summary = summaries.get(key) || {
      ...hint,
      count: 0,
      samplePaths: [],
    };

    summary.count += 1;
    if (summary.samplePaths.length < DIRTY_OWNER_SAMPLE_PATH_LIMIT) {
      summary.samplePaths.push(entry.path);
    }
    summaries.set(key, summary);
  }

  return [...summaries.values()].sort((left, right) => {
    if (right.count !== left.count) return right.count - left.count;
    return left.priority - right.priority;
  }).map(({ priority: _priority, ...summary }) => summary);
}

function dirtyEntryOwnerHint(pathValue) {
  if (isGeneratedNoisePath(pathValue)) {
    return {
      ownerLane: 7,
      bucket: "generated-preview-noise",
      priority: 10,
    };
  }

  if (isLane14OutputPath(pathValue)) {
    return {
      ownerLane: 14,
      bucket: "final-launch-coordinator",
      priority: 20,
    };
  }

  if (isLaunchStatusHandoffPath(pathValue)) {
    return {
      ownerLane: 14,
      bucket: "launch-status-handoff",
      priority: 22,
    };
  }

  if (isPublicFrameworkContractPath(pathValue)) {
    return {
      ownerLane: 14,
      bucket: "public-framework-contract",
      priority: 23,
    };
  }

  if (isNextRustReferenceBoundaryPath(pathValue)) {
    return {
      ownerLane: 14,
      bucket: "next-rust-reference-boundary",
      priority: 24,
    };
  }

  if (isCompileTestGatePath(pathValue)) {
    return {
      ownerLane: 1,
      bucket: "compile-test-gate",
      priority: 26,
    };
  }

  if (
    pathValue.startsWith(".dx/postcss-compat-target/") ||
    pathValue.startsWith("related-crates/style/") ||
    pathValue.startsWith("tools/dx-style/") ||
    pathValue.startsWith("tools/style/") ||
    pathValue.startsWith("benchmarks/dx-style") ||
    pathValue === "dx-www/src/parser/style.rs"
  ) {
    return {
      ownerLane: 7,
      bucket: "dx-style-runtime-integration",
      priority: 25,
    };
  }

  if (
    pathValue.startsWith("core/src/delivery/route_handler") ||
    pathValue.startsWith("benchmarks/route-handler") ||
    pathValue === "benchmarks/app-api-route-handler-extensions.test.ts" ||
    pathValue === "benchmarks/dx-api-router-http-method-detection.test.ts" ||
    pathValue === "benchmarks/ai-route-handler-provider-boundary.test.ts" ||
    pathValue === "benchmarks/automation-route-handler-provider-boundary.test.ts" ||
    pathValue === "benchmarks/fumadocs-llms-route-handler-contract.test.ts" ||
    pathValue === "benchmarks/fumadocs-route-handler-proxy-query-contract.test.ts" ||
    pathValue === "benchmarks/provider-route-handler-boundary-truth.test.ts" ||
    pathValue === "benchmarks/trpc-launch-runtime-proof.test.ts" ||
    pathValue.endsWith("-route-handler-slice.test.ts") ||
    pathValue.startsWith("dx-www/tests/route_handler") ||
    pathValue.startsWith("dx-www/src/api/") ||
    pathValue.startsWith("dx-www/src/cli/app_api_routes")
  ) {
    return {
      ownerLane: 3,
      bucket: "route-handler-execution",
      priority: 28,
    };
  }

  if (
    pathValue.startsWith("dx-www/src/cli/app_router_server_data") ||
    pathValue.startsWith("dx-www/src/data/") ||
    pathValue.startsWith("dx-www/tests/app_router_server_data") ||
    pathValue.startsWith("benchmarks/app-router-server-data")
  ) {
    return {
      ownerLane: 5,
      bucket: "server-data-safe-loaders",
      priority: 29,
    };
  }

  if (
    pathValue.startsWith("dx-www/src/cli/app_router_execution") ||
    pathValue.startsWith("dx-www/src/cli/app_router_semantics") ||
    pathValue.startsWith("dx-www/src/router/") ||
    pathValue === "benchmarks/app-router-page-extensions-build-loop.test.ts" ||
    pathValue.startsWith("benchmarks/dx-app-router-") ||
    pathValue === "benchmarks/dx-router-request-normalization.test.ts" ||
    pathValue.startsWith("benchmarks/tsx-app-router") ||
    pathValue.startsWith("benchmarks/lane4-") ||
    pathValue === "core/src/delivery/app_route.rs"
  ) {
    return {
      ownerLane: 4,
      bucket: "app-router-runtime-parity",
      priority: 30,
    };
  }

  if (
    pathValue === "dx-www/src/cli/mod.rs" ||
    pathValue.startsWith("dx-www/src/cli/") ||
    pathValue.startsWith("benchmarks/cli-")
  ) {
    return {
      ownerLane: 13,
      bucket: "cli-architecture-split",
      priority: 30,
    };
  }

  if (
    pathValue.startsWith("dx-www/src/build/source_engine/module_resolver") ||
    pathValue.startsWith("dx-www/tests/source_resolver") ||
    pathValue.startsWith("benchmarks/source-resolver") ||
    pathValue === "tools/build-graph/resolver.ts"
  ) {
    return {
      ownerLane: 6,
      bucket: "resolver-module-linker",
      priority: 35,
    };
  }

  if (
    pathValue.startsWith("dx-www/src/build/") ||
    pathValue.startsWith("dx-www/tests/source_build") ||
    pathValue.startsWith("dx-www/tests/dx_build") ||
    pathValue.startsWith("tools/build/") ||
    pathValue.startsWith("tools/build-graph/") ||
    pathValue.startsWith("benchmarks/dx-build") ||
    isSourceBuildProofPath(pathValue)
  ) {
    return {
      ownerLane: 2,
      bucket: "dx-build-end-to-end",
      priority: 40,
    };
  }

  if (
    pathValue.startsWith("dx-www/src/dev/") ||
    pathValue === "dx-www/src/hot_reload_protocol.rs" ||
    pathValue === "benchmarks/dx-dev-feedback-contract.test.ts" ||
    pathValue === "benchmarks/dx-dev-feedback-contract.test.ts" ||
    pathValue.startsWith("benchmarks/dx-hot-reload") ||
    pathValue === "benchmarks/dx-hot-reload-issue-stream.test.ts" ||
    pathValue.startsWith("benchmarks/dx-dev-")
  ) {
    return {
      ownerLane: 8,
      bucket: "dev-server-hmr",
      priority: 50,
    };
  }

  if (
    pathValue === "dx-www/src/diagnostics.rs" ||
    pathValue.startsWith("dx-www/src/diagnostics/") ||
    pathValue.startsWith("dx-www/tests/diagnostics") ||
    pathValue.includes("_diagnostics.") ||
    pathValue.startsWith("benchmarks/diagnostics")
  ) {
    return {
      ownerLane: 9,
      bucket: "diagnostics-error-ux",
      priority: 55,
    };
  }

  if (
    pathValue.startsWith("examples/template/.dx/forge/") ||
    pathValue.startsWith("docs/packages/") ||
    pathValue.startsWith("core/src/ecosystem/") ||
    pathValue.startsWith("benchmarks/www-forge") ||
    pathValue.startsWith("benchmarks/www-template-forge") ||
    pathValue.includes("package-status-read-model") ||
    isForgePackageProofPath(pathValue)
  ) {
    return {
      ownerLane: 11,
      bucket: "forge-package-template-truth",
      priority: 58,
    };
  }

  if (
    pathValue.startsWith("examples/template/") ||
    pathValue.startsWith("benchmarks/www-template-") ||
    isDefaultTemplateProductPath(pathValue)
  ) {
    return {
      ownerLane: 10,
      bucket: "default-template-product-quality",
      priority: 60,
    };
  }

  if (isConversionProofStaticPath(pathValue)) {
    return {
      ownerLane: 10,
      bucket: "conversion-proof-static-parity",
      priority: 61,
    };
  }

  if (isFlowForgeIntegrationPath(pathValue)) {
    return {
      ownerLane: 11,
      bucket: "flow-forge-integration",
      priority: 62,
    };
  }

  return {
    ownerLane: null,
    bucket: "unclassified-source-or-doc",
    priority: 100,
  };
}

function isLaunchStatusHandoffPath(pathValue) {
  return (
    pathValue === "README.md" ||
    pathValue === "DX.md" ||
    pathValue === "TODO.md" ||
    pathValue === "CHANGELOG.md" ||
    pathValue === "docs/DX_WWW_MANAGER_HANDOFF.md" ||
    pathValue === "benchmarks/launch-docs-honesty.test.ts" ||
    pathValue === "benchmarks/launch-readme-overclaim-guard.test.ts"
  );
}

function isPublicFrameworkContractPath(pathValue) {
  return (
    pathValue === "docs/DX_WWW_FRAMEWORK_STRUCTURE.md" ||
    pathValue === "docs/dx-www-developer-contract.md" ||
    pathValue === "dx-www/README.md" ||
    pathValue === "benchmarks/public-framework-contract.test.ts" ||
    pathValue === "benchmarks/public-framework-tools.test.ts" ||
    pathValue === "benchmarks/dx-scope-removal-contract.test.ts" ||
    pathValue === "benchmarks/dx-www-architecture-scope-contract.test.ts" ||
    pathValue === "benchmarks/dx-www-architecture-scope.test.ts" ||
    pathValue === "benchmarks/dx-www-parser-launch-extensions.test.ts" ||
    pathValue === "benchmarks/react-starter-benchmark-scope.test.ts" ||
    pathValue === "benchmarks/public-v1-error-wording.test.ts" ||
    pathValue === "core/src/delivery/server_contract.rs" ||
    pathValue === "core/src/ecosystem/dx_check_receipt.rs" ||
    pathValue === "core/src/ecosystem/dx_style_receipts.rs" ||
    pathValue === "dx-www/src/project.rs" ||
    pathValue === "dx-www/src/parser/mod.rs" ||
    pathValue === "dx-www/src/production/mod.rs"
  );
}

function isNextRustReferenceBoundaryPath(pathValue) {
  return (
    pathValue === "docs/next-rust-merge-checkpoint.md" ||
    pathValue === "docs/NEXTJS_COMPATIBILITY_MAP.md" ||
    pathValue === "vendor/next-rust/README.md" ||
    pathValue === "dx-www/src/next_rust.rs" ||
    pathValue === "dx-www/src/next_rust_task_adapter.rs" ||
    pathValue === "dx-www/src/next_rust_source_map_adapter.rs" ||
    pathValue === "benchmarks/nextjs-compatibility-map.test.ts" ||
    pathValue === "benchmarks/next-custom-transforms-receipt.test.ts" ||
    pathValue.startsWith("tools/vendor/")
  );
}

function isBuildGraphAdapterPath(pathValue) {
  return (
    pathValue === "tools/build-graph/dx-graph.ts" ||
    pathValue === "tools/build-graph/index.js" ||
    pathValue === "tools/build-graph/reader.ts" ||
    pathValue === "tools/build-graph/receipt.ts" ||
    pathValue === "tools/build-graph/scanner.ts" ||
    pathValue === "tools/build-graph/asset-references.ts" ||
    pathValue === "tools/build-graph/vendor-root.ts"
  );
}

function isCompileTestGatePath(pathValue) {
  return (
    pathValue === "Cargo.lock" ||
    pathValue === "Cargo.toml" ||
    pathValue === "dx-www/Cargo.toml" ||
    pathValue === "dx-www/src/lib.rs" ||
    pathValue === "dx-www/src/main.rs" ||
    pathValue === "dx-www/src/config.rs" ||
    pathValue === "dx-www/src/config_source.rs" ||
    pathValue === "dx-www/src/error.rs" ||
    pathValue === "core/src/delivery/mod.rs" ||
    pathValue === "core/src/delivery/tests.rs" ||
    pathValue === "tools/launch/launch-compile-gate.js" ||
    pathValue === "tools/launch/launch-readiness-gate.js" ||
    pathValue.startsWith("tools/launch/readiness-gate/") ||
    pathValue.startsWith("benchmarks/launch-compile-gate") ||
    pathValue.startsWith("benchmarks/launch-readiness-gate")
  );
}

function isSourceBuildProofPath(pathValue) {
  return (
    pathValue.startsWith("benchmarks/fixtures/build-graph/") ||
    pathValue.startsWith("benchmarks/source-build-") ||
    pathValue === "docs/build-graph-model.md" ||
    pathValue === "benchmarks/mdx-docs-source-build-contract.test.ts" ||
    pathValue === "benchmarks/source-build-image-metadata-source-guard.test.ts" ||
    pathValue.startsWith("dx-www/tests/dx_build_")
  );
}

function isForgePackageProofPath(pathValue) {
  return (
    pathValue.startsWith("examples/dashboard/src/lib/forge/") ||
    pathValue === "examples/dashboard/src/components/BetterAuthAccountWorkflow.tsx" ||
    pathValue.startsWith("examples/dashboard/src/components/AutomationWorkflowPanel") ||
    pathValue.startsWith("examples/dashboard/src/lib/wasmBindgenDashboard") ||
    pathValue.startsWith("integrations/n8n-nodes-base/") ||
    /^benchmarks\/[a-z0-9-]+-dx-check-package-lane-panel\.test\.ts$/.test(pathValue) ||
    /^benchmarks\/[a-z0-9-]+-receipt-hash-refresh\.test\.ts$/.test(pathValue) ||
    /^benchmarks\/(automations|drizzle|fumadocs|instantdb|next-intl|stripe-payment|supabase|wasm-bindgen|zod)-.*\.test\.ts$/.test(
      pathValue,
    ) ||
    pathValue === "benchmarks/forge-golden-path-launch-proof.test.ts" ||
    pathValue === "benchmarks/forge-source-owned-package-review.test.ts" ||
    pathValue === "benchmarks/forge-status-panel-token-classes.test.ts" ||
    pathValue === "benchmarks/launch-package-slices.test.ts" ||
    pathValue === "benchmarks/measure-forge-package-update-rehearsal.ts" ||
    pathValue === "benchmarks/motion-launch-materialized.test.ts" ||
    pathValue === "benchmarks/tanstack-query-slice.test.ts" ||
    pathValue === "benchmarks/zustand-launch-materialized.test.ts" ||
    isLaunchPackageRealityProofPath(pathValue)
  );
}

function isLaunchPackageRealityProofPath(pathValue) {
  return (
    pathValue === "benchmarks/authentication-lock-backed-template.test.ts" ||
    pathValue === "benchmarks/better-auth-dashboard-workflow.test.ts" ||
    pathValue === "benchmarks/better-auth-live-runtime.test.ts" ||
    pathValue === "benchmarks/better-auth-session-helper.test.ts" ||
    pathValue === "benchmarks/forge-package-row-maturity-classification.test.ts" ||
    pathValue === "benchmarks/instantdb-dx-check-visibility.test.ts" ||
    pathValue === "benchmarks/launch-package-maturity-classification.test.ts" ||
    pathValue === "benchmarks/template-forms-validation-receipt-wiring.test.ts" ||
    pathValue === "benchmarks/template-readiness-execution-proof.test.ts" ||
    pathValue === "benchmarks/three-scene-package-doc.test.ts" ||
    pathValue === "benchmarks/vercel-ai-launch-visible-proof.test.ts" ||
    pathValue === "benchmarks/zod-dashboard-settings-workflow.test.ts" ||
    pathValue.startsWith("benchmarks/lane7-3d-") ||
    pathValue.startsWith("benchmarks/lane7-motion-") ||
    pathValue.startsWith("benchmarks/lane7-webassembly-")
  );
}

function isDefaultTemplateProductPath(pathValue) {
  return (
    pathValue === "benchmarks/default-www-template-contract.test.ts" ||
    pathValue === "benchmarks/launch-live-runtime-guard.test.ts" ||
    pathValue === "benchmarks/launch-runtime-materializer.test.ts" ||
    pathValue === "benchmarks/launch-scene-runtime.test.ts" ||
    pathValue === "benchmarks/lane7-default-template-surface.test.ts" ||
    pathValue === "tools/launch/materialize-www-template.ts" ||
    pathValue === "tools/launch/run-template-receipt-helper.js"
  );
}

function isFlowForgeIntegrationPath(pathValue) {
  return (
    pathValue.startsWith("../forge/") ||
    pathValue.startsWith("forge/") ||
    pathValue.startsWith("benchmarks/flow-forge-")
  );
}

function isConversionProofStaticPath(pathValue) {
  return (
    pathValue === "benchmarks/dx-www-conversion-proof.test.ts" ||
    pathValue === "examples/conversion-proof/.dx/forge/source-manifest.json" ||
    pathValue ===
      "examples/conversion-proof/forge/route-discovery/conversion-routes.json" ||
    pathValue.startsWith("examples/conversion-proof/.dx/vercel-landing/") ||
    pathValue.startsWith("examples/conversion-proof/pages/") ||
    pathValue.startsWith("examples/conversion-proof/public/")
  );
}

function isGeneratedNoisePath(pathValue) {
  return (
    pathValue === "-" ||
    GENERATED_NOISE_PREFIXES.some((prefix) => pathValue.startsWith(prefix))
  );
}

function isLane14OutputPath(pathValue) {
  return LANE14_OUTPUT_PREFIXES.some((prefix) => pathValue.startsWith(prefix));
}

function launchCheck({
  id,
  ownerLane,
  handoffLane = null,
  status,
  evidence,
  nextAction,
  details = {},
  blocking = true,
}) {
  return {
    id,
    ownerLane,
    ...(Number.isInteger(handoffLane) ? { handoffLane } : {}),
    blocking,
    status,
    evidence,
    nextAction,
    details,
  };
}

function findForbiddenTemplateArtifacts(cwd) {
  const artifacts = [];

  for (const root of TEMPLATE_ARTIFACT_ROOTS) {
    const absoluteRoot = path.join(cwd, root);
    if (fs.existsSync(absoluteRoot)) {
      collectForbiddenTemplateArtifacts(absoluteRoot, cwd, artifacts);
    }
  }

  return artifacts.sort();
}

function collectForbiddenTemplateArtifacts(directory, cwd, artifacts) {
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    if (entry.name === ".git" || entry.name === "node_modules") continue;

    const absolute = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      collectForbiddenTemplateArtifacts(absolute, cwd, artifacts);
    } else if (entry.isFile() && FORBIDDEN_TEMPLATE_EXTENSIONS.has(path.extname(entry.name))) {
      artifacts.push(normalizePath(path.relative(cwd, absolute)));
    }
  }
}

function readGitStatusShort(cwd) {
  const result = spawnSync("git", ["status", "--short"], {
    cwd,
    encoding: "utf8",
    windowsHide: true,
  });

  if (result.status !== 0) {
    return "";
  }

  return result.stdout;
}

function runGitDiffCheck(cwd) {
  const result = spawnSync("git", ["diff", "--check"], {
    cwd,
    encoding: "utf8",
    windowsHide: true,
  });

  return {
    command: "git diff --check",
    exitCode: Number.isInteger(result.status) ? result.status : 1,
    stdout: result.stdout || "",
    stderr: result.stderr || "",
    error: result.error?.message || null,
  };
}

function gitDiffCheckOutputLines(report) {
  return [report.stdout, report.stderr, report.error]
    .filter((value) => typeof value === "string" && value.length > 0)
    .join("\n")
    .split(/\r?\n/)
    .map((line) => line.trimEnd())
    .filter(Boolean)
    .slice(0, 25);
}

function parseGitStatusPaths(statusText) {
  return statusText
    .split(/\r?\n/)
    .map((line) => line.trimEnd())
    .filter(Boolean)
    .map((line) => {
      const status = line.slice(0, 2).trim();
      const rawPath = line.slice(3).trim();
      const renamePath = rawPath.includes(" -> ") ? rawPath.split(" -> ").pop() : rawPath;
      return { status, path: normalizePath(renamePath) };
    });
}

function launchArchitecture() {
  return {
    dxRuntimeAuthoritative: true,
    publicTurbopackDependency: false,
    reactRequiredCore: false,
    nodeModulesRequired: false,
    nodeNapiFoundation: false,
    generatedPreviewAuthoritative: false,
  };
}

function normalizePath(value) {
  return value.replace(/\\/g, "/");
}

if (require.main === module) {
  runLaunchCoordinatorCli().then(({ exitCode }) => {
    process.exitCode = exitCode;
  });
}

module.exports = {
  LAUNCH_COORDINATOR_SCHEMA,
  buildLaunchCoordinatorReport,
  parseGitStatusPaths,
  runLaunchCoordinatorCli,
};
