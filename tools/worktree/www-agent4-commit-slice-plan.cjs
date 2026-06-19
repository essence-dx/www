"use strict";

const { execFileSync } = require("node:child_process");
const fs = require("node:fs");

const {
  readStatusEntries,
} = require("./www-agent2-ownership-map.cjs");

const SAMPLE_LIMIT = 10;
const GIT_OUTPUT_BUFFER_BYTES = 64 * 1024 * 1024;
const LARGE_SOURCE_FILE_BYTES = 100 * 1024;

const GENERATED_IGNORE_PATTERNS = [
  /^\.dx\//,
  /^\.tmp\//,
  /^examples\/template\/\.dx\/build\//,
  /^examples\/template\/\.dx\/receipts\//,
];

const SLICE_DEFINITIONS = Object.freeze({
  "coordination": {
    title: "Worker coordination and release-control tooling",
    policy: "commit only with worker-lane automation and worktree planning",
    domains: ["worker-control"],
    checks: ["node --check tools/worktree/www-agent4-commit-slice-plan.cjs"],
  },
  "www-core": {
    title: "DX-WWW core executable and shared Rust crate surface",
    policy: "commit after cargo check/build owner confirms warnings and compile proof",
    domains: ["dx-www-core"],
    checks: ["cargo check -p dx-www --no-default-features --features cli --bin dx-www -j 1"],
  },
  "router-runtime": {
    title: "App Router, route handlers, routing, and runtime semantics",
    policy: "split further if App Router and route-handler files are both present",
    domains: ["app-router-route-handlers"],
    checks: ["focused App Router and route-handler benchmarks owned by the source lane"],
  },
  "dx-style": {
    title: "DX-owned style engine and template token compatibility",
    policy: "keep separate from template product and dx-www core commits",
    domains: ["dx-style"],
    checks: ["cargo check -p dx-style --lib -j 1", "focused dx-style benchmark"],
  },
  "build-artifacts": {
    title: "dx build source graph, installed-smoke, manifests, and readiness gates",
    policy: "commit source/tools separately from generated .dx outputs",
    domains: ["build-smoke"],
    checks: ["real dx build or installed-smoke focused check"],
  },
  "forge": {
    title: "Forge package/provenance/trust source changes",
    policy: "commit with Forge owner review; do not mix with generated Forge receipts",
    domains: ["forge"],
    checks: ["focused Forge split/provenance checks"],
  },
  "template-product": {
    title: "Launch template authored product files",
    policy: "commit authored template source only; keep .dx outputs quarantined",
    domains: ["template"],
    checks: ["HTTP/browser or www-template focused check"],
  },
  "dev-feedback": {
    title: "Dev server, hot reload, diagnostics, and overlay feedback",
    policy: "commit with dev/hot-reload owner proof",
    domains: ["hot-reload-diagnostics"],
    checks: ["focused hot reload or diagnostics benchmark"],
  },
  "resolver-build-graph": {
    title: "Resolver, linker, and build graph adapters",
    policy: "commit with resolver/build-graph owner proof",
    domains: ["resolver-linker"],
    checks: ["focused resolver/build graph checks"],
  },
  "docs-status": {
    title: "Docs, status, scorecards, and public claim truth",
    policy: "commit after source proof; never as standalone score movement",
    domains: ["docs-status"],
    checks: ["scope/claim source guard"],
  },
  "scope-boundary": {
    title: "Next/Turbopack reference-only and removed-scope cleanup",
    policy: "manager-reviewed deletion slice; do not bundle with runtime features",
    domains: ["vendor-reference"],
    checks: ["scope removal guard"],
  },
  "test-evidence": {
    title: "Cross-lane benchmark evidence",
    policy: "move each test into the commit for the source behavior it asserts",
    domains: ["benchmark-tests"],
    checks: ["node --check or node --test for each touched benchmark"],
  },
  "hold-generated": {
    title: "Generated artifacts and local proof output",
    policy: "hold out of commits unless an artifact owner explicitly accepts proof files",
    domains: ["generated"],
    checks: ["artifact owner inventory"],
  },
  "hold-preserve": {
    title: "Preserved workspace-only deletions",
    policy: "do not restore and do not mix with feature commits",
    domains: ["deleted-root-junk"],
    checks: ["git status confirms root '-' remains deleted"],
  },
  "hold-unclassified": {
    title: "Unclassified dirty files",
    policy: "assign an owner before staging",
    domains: ["unclassified"],
    checks: ["human release-control review"],
  },
});

const SLICE_PATH_OVERRIDES = Object.freeze([
  {
    pattern: /^tools\/launch\/launch-readiness-gate\.js$/,
    slice: "build-artifacts",
    reason: "launch readiness gate source belongs with build/readiness proof tooling",
  },
  {
    pattern: /^tools\/launch\/materialize-www-template\.ts$/,
    slice: "template-product",
    reason: "launch runtime page materializer belongs with authored template product work",
  },
]);

const DOMAIN_TO_SLICE = Object.freeze(
  Object.fromEntries(
    Object.entries(SLICE_DEFINITIONS).flatMap(([sliceKey, definition]) =>
      definition.domains.map((domain) => [domain, sliceKey]),
    ),
  ),
);

const SLICE_ORDER = Object.freeze([
  "coordination",
  "www-core",
  "router-runtime",
  "dx-style",
  "build-artifacts",
  "forge",
  "template-product",
  "dev-feedback",
  "resolver-build-graph",
  "docs-status",
  "scope-boundary",
  "test-evidence",
  "hold-generated",
  "hold-preserve",
  "hold-unclassified",
]);

function main() {
  const options = parseArgs(process.argv.slice(2));
  const plan = buildCommitSlicePlan();
  if (options.writePathspec) {
    writeSlicePathspec(plan, options.writePathspec);
    return;
  }
  if (options.paths) {
    printSlicePaths(plan, options.paths, { nul: options.nulPaths });
    return;
  }
  if (options.handoff && options.json) {
    console.log(JSON.stringify(plan.handoff, null, 2));
    return;
  }
  if (options.handoff) {
    printHandoff(plan);
    return;
  }
  if (options.json) {
    console.log(JSON.stringify(plan, null, 2));
    return;
  }
  printTextPlan(plan);
}

function parseArgs(argv) {
  const options = {
    handoff: false,
    json: false,
    nulPaths: false,
    paths: null,
    writePathspec: null,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--handoff") {
      options.handoff = true;
    } else if (arg === "--json") {
      options.json = true;
    } else if (arg === "--paths" || arg === "--slice-paths") {
      options.paths = argv[index + 1] || "";
      index += 1;
    } else if (arg === "--paths-z" || arg === "--slice-paths-z") {
      options.paths = argv[index + 1] || "";
      options.nulPaths = true;
      index += 1;
    } else if (arg === "--write-pathspec") {
      options.writePathspec = {
        slice: argv[index + 1] || "",
        outputPath: argv[index + 2] || "",
      };
      index += 2;
    } else if (arg.startsWith("--paths=")) {
      options.paths = arg.slice("--paths=".length);
    } else if (arg.startsWith("--slice-paths=")) {
      options.paths = arg.slice("--slice-paths=".length);
    } else if (arg.startsWith("--paths-z=")) {
      options.paths = arg.slice("--paths-z=".length);
      options.nulPaths = true;
    } else if (arg.startsWith("--slice-paths-z=")) {
      options.paths = arg.slice("--slice-paths-z=".length);
      options.nulPaths = true;
    }
  }

  return options;
}

function buildCommitSlicePlan() {
  const entries = readStatusEntries();
  const slices = buildSlices(entries);
  const ignoredGenerated = readIgnoredGeneratedArtifacts();
  const stagedEntries = readNameStatusEntries(["diff", "--cached", "--name-status"]);
  const unmerged = readGit(["diff", "--name-only", "--diff-filter=U", "-z"])
    .split("\0")
    .filter(Boolean)
    .map(normalizePath);
  const riskFlags = buildRiskFlags({
    entries,
    slices,
    ignoredGenerated,
    unmerged,
    stagedEntries,
  });
  const generatedAt = new Date().toISOString();

  return {
    schema: "dx.www.worktree.agent4CommitSlicePlan",
    format: 1,
    lane: 4,
    laneName: "Commit Slice Planner",
    generatedAt,
    branch: readGit(["branch", "--show-current"]).trim(),
    shortstat: readGit(["diff", "--shortstat"]).trim(),
    statusEntryCount: entries.length,
    stagedEntryCount: stagedEntries.length,
    stagedEntries: stagedEntries.slice(0, SAMPLE_LIMIT),
    unmergedPaths: unmerged,
    ignoredGenerated,
    slices,
    riskFlags,
    handoff: buildReleaseHandoff({
      generatedAt,
      slices,
      riskFlags,
      unmerged,
      ignoredGenerated,
      stagedEntries,
    }),
    nextAction:
      "Stage one slice at a time with explicit paths; leave hold-* slices and foreign generated output untouched.",
  };
}

function buildSlices(entries) {
  const slices = new Map();
  for (const key of SLICE_ORDER) {
    const definition = SLICE_DEFINITIONS[key];
    slices.set(key, {
      key,
      title: definition.title,
      policy: definition.policy,
      checks: definition.checks,
      total: 0,
      byKind: {},
      paths: [],
      samplePaths: [],
    });
  }

  for (const entry of entries) {
    const sliceKey = classifySlice(entry);
    const slice = slices.get(sliceKey);
    slice.total += 1;
    slice.byKind[entry.kind] = (slice.byKind[entry.kind] || 0) + 1;
    slice.paths.push(entry.path);
    if (slice.samplePaths.length < SAMPLE_LIMIT) {
      slice.samplePaths.push(`${entry.status} ${entry.path}`);
    }
  }

  return Array.from(slices.values())
    .filter((slice) => slice.total > 0)
    .sort((left, right) => SLICE_ORDER.indexOf(left.key) - SLICE_ORDER.indexOf(right.key));
}

function classifySlice(entry) {
  const override = SLICE_PATH_OVERRIDES.find((candidate) =>
    candidate.pattern.test(entry.path),
  );
  if (override) {
    return override.slice;
  }

  const domainKey = entry.domain?.key || "unclassified";
  return DOMAIN_TO_SLICE[domainKey] || "hold-unclassified";
}

function buildRiskFlags({ entries, slices, ignoredGenerated, unmerged, stagedEntries }) {
  const risks = [];

  if (stagedEntries.length > 0) {
    risks.push({
      id: "staged-index-not-empty",
      severity: "blocking",
      count: stagedEntries.length,
      message:
        "The index already has staged entries. Inspect or clear intentional staging before preparing release-control slices.",
      samplePaths: stagedEntries.slice(0, SAMPLE_LIMIT).map(formatNameStatusEntry),
    });
  }

  if (unmerged.length > 0) {
    risks.push({
      id: "unmerged-index",
      severity: "blocking",
      message: "Resolve unmerged paths before any release-control staging.",
      samplePaths: unmerged.slice(0, SAMPLE_LIMIT),
    });
  }

  const rootJunk = entries.find((entry) => entry.path === "-");
  if (!rootJunk || !rootJunk.status.includes("D")) {
    risks.push({
      id: "root-junk-delete-not-preserved",
      severity: "blocking",
      message: "The junk root file named '-' must stay deleted unless a coordinator handles it.",
    });
  }

  const holdSlices = slices.filter((slice) => slice.key.startsWith("hold-"));
  for (const slice of holdSlices) {
    risks.push({
      id: slice.key,
      severity: "blocking",
      count: slice.total,
      message: slice.policy,
      samplePaths: slice.samplePaths.map((sample) => sample.replace(/^.. /, "")),
    });
  }

  const sourceDeletions = entries.filter(
    (entry) =>
      entry.kind === "deleted" &&
      entry.path !== "-" &&
      !/^tools\/build-graph\/turbo-tasks-/.test(entry.path) &&
      entry.path !== "core/src/devtools.rs" &&
      entry.path !== "dx-www/src/cli/next_parity_fixtures.rs",
  );
  if (sourceDeletions.length > 0) {
    risks.push({
      id: "source-deletions-need-owner-review",
      severity: "review",
      count: sourceDeletions.length,
      message: "Non-scope source deletions must be reviewed in their owner slice before staging.",
      samplePaths: sourceDeletions.slice(0, SAMPLE_LIMIT).map((entry) => entry.path),
    });
  }

  const legacyUntrackedTests = entries.filter(
    (entry) =>
      entry.status === "??" &&
      /^benchmarks\/.*\.(?:cjs|mjs)$/.test(entry.path),
  );
  if (legacyUntrackedTests.length > 0) {
    risks.push({
      id: "untracked-legacy-js-tests",
      severity: "review",
      count: legacyUntrackedTests.length,
      message: "New tests default to .ts; untracked .cjs/.mjs tests need source-lane justification.",
      samplePaths: legacyUntrackedTests.slice(0, SAMPLE_LIMIT).map((entry) => entry.path),
    });
  }

  const largeLaunchSourceFiles = entries
    .filter((entry) => /^tools\/launch\/.*\.(?:cjs|js|mjs|ts)$/.test(entry.path))
    .map((entry) => ({ entry, bytes: readFileSize(entry.path) }))
    .filter((item) => item.bytes >= LARGE_SOURCE_FILE_BYTES);
  if (largeLaunchSourceFiles.length > 0) {
    risks.push({
      id: "large-launch-source-files",
      severity: "review",
      count: largeLaunchSourceFiles.length,
      message:
        "Large launch tooling files need owner review before staging so generated-materializer code does not hide inside a broad commit.",
      samplePaths: largeLaunchSourceFiles
        .slice(0, SAMPLE_LIMIT)
        .map((item) => `${item.entry.path} (${item.bytes} bytes)`),
    });
  }

  if (ignoredGenerated.total > 0) {
    risks.push({
      id: "ignored-generated-artifacts-present",
      severity: "hold",
      count: ignoredGenerated.total,
      message: "Ignored generated outputs are present locally; keep them out of source commits.",
      samplePaths: ignoredGenerated.samplePaths,
    });
  }

  if (slices.length > 1) {
    risks.push({
      id: "mixed-slice-worktree",
      severity: "blocking",
      count: slices.length,
      message: "The worktree contains multiple independent commit slices; never stage all.",
      samplePaths: slices.slice(0, SAMPLE_LIMIT).map((slice) => slice.key),
    });
  }

  return risks;
}

function readFileSize(path) {
  try {
    return fs.statSync(path).size;
  } catch {
    return 0;
  }
}

function readIgnoredGeneratedArtifacts() {
  const output = readGit(["ls-files", "--others", "--ignored", "--exclude-standard", "-z"]);
  const paths = output
    .split("\0")
    .filter(Boolean)
    .map(normalizePath)
    .filter((path) => GENERATED_IGNORE_PATTERNS.some((pattern) => pattern.test(path)));
  return {
    total: paths.length,
    samplePaths: paths.slice(0, SAMPLE_LIMIT),
  };
}

function readNameStatusEntries(args) {
  const records = readGit([...args, "-z"]).split("\0").filter(Boolean);
  const entries = [];

  for (let index = 0; index < records.length; index += 1) {
    const status = records[index];
    let path = normalizePath(records[index + 1]);
    let oldPath = null;
    index += 1;

    if (/^[RC]/.test(status)) {
      oldPath = path;
      path = normalizePath(records[index + 1]);
      index += 1;
    }

    entries.push({ status, path, oldPath });
  }

  return entries;
}

function buildReleaseHandoff({ generatedAt, slices, riskFlags, unmerged, ignoredGenerated, stagedEntries }) {
  const stageOrder = slices
    .filter((slice) => !slice.key.startsWith("hold-"))
    .map((slice) => ({
      key: slice.key,
      title: slice.title,
      count: slice.total,
      disposition: slice.key === "docs-status" ? "stage-after-source-proof" : "stage-with-owner-lane",
      policy: slice.policy,
      checks: slice.checks,
      pathListCommand: `node tools/worktree/www-agent4-commit-slice-plan.cjs --paths ${slice.key}`,
      pathListNulCommand: `node tools/worktree/www-agent4-commit-slice-plan.cjs --paths-z ${slice.key}`,
      stageCommand:
        `$p = Join-Path $env:TEMP 'dx-www-${slice.key}.pathspec'; ` +
        `node tools/worktree/www-agent4-commit-slice-plan.cjs --write-pathspec ${slice.key} $p; ` +
        `git add --pathspec-from-file=$p --pathspec-file-nul; ` +
        `Remove-Item -LiteralPath $p`,
    }));
  const holdSlices = slices
    .filter((slice) => slice.key.startsWith("hold-"))
    .map((slice) => ({
      key: slice.key,
      title: slice.title,
      count: slice.total,
      policy: slice.policy,
      samplePaths: slice.samplePaths,
    }));

  return {
    schema: "dx.www.worktree.agent4ReleaseHandoff",
    format: 1,
    generatedAt,
    currentIndexStagedEntryCount: stagedEntries.length,
    currentIndexStagedSamples: stagedEntries.slice(0, SAMPLE_LIMIT).map(formatNameStatusEntry),
    unmergedPathCount: unmerged.length,
    ignoredGeneratedArtifactCount: ignoredGenerated.total,
    stageOrder,
    holdSlices,
    mustNotCommit: buildMustNotCommitInventory({ holdSlices, ignoredGenerated }),
    blockingRisks: riskFlags.filter((risk) => risk.severity === "blocking"),
    reviewRisks: riskFlags.filter((risk) => risk.severity !== "blocking"),
    nextActions: [
      "Run the path list command first and inspect the slice before staging.",
      "Use the stage command only for one owner-approved slice at a time.",
      "Do not emit or stage hold-* paths with --paths; use --json for their inventory.",
      "Leave generated outputs quarantined unless an artifact owner explicitly accepts them.",
    ],
  };
}

function buildMustNotCommitInventory({ holdSlices, ignoredGenerated }) {
  const inventory = holdSlices.map((slice) => ({
    source: slice.key,
    reason: slice.policy,
    count: slice.count,
    samplePaths: slice.samplePaths.map((sample) => sample.replace(/^.. /, "")),
  }));

  if (ignoredGenerated.total > 0) {
    inventory.push({
      source: "ignored-generated-artifacts",
      reason: "local generated outputs are ignored and should stay out of source commits",
      count: ignoredGenerated.total,
      samplePaths: ignoredGenerated.samplePaths,
    });
  }

  return inventory;
}

function printSlicePaths(plan, sliceKey, { nul = false } = {}) {
  const slice = findStageableSlice(plan, sliceKey);
  if (!slice) {
    return;
  }

  if (nul) {
    if (slice.paths.length > 0) {
      process.stdout.write(`${slice.paths.join("\0")}\0`);
    }
    return;
  }

  for (const path of slice.paths) {
    console.log(path);
  }
}

function writeSlicePathspec(plan, { slice: sliceKey, outputPath }) {
  const slice = findStageableSlice(plan, sliceKey);
  if (!slice) {
    return;
  }
  if (!outputPath) {
    console.error("--write-pathspec requires an output path.");
    process.exitCode = 2;
    return;
  }

  fs.writeFileSync(outputPath, slice.paths.length > 0 ? `${slice.paths.join("\0")}\0` : "", "utf8");
  console.error(`Wrote ${slice.paths.length} pathspecs for '${slice.key}' to ${outputPath}.`);
}

function findStageableSlice(plan, sliceKey) {
  const slice = plan.slices.find((candidate) => candidate.key === sliceKey);
  if (!slice) {
    console.error(`Unknown slice '${sliceKey}'. Known slices: ${plan.slices.map((item) => item.key).join(", ")}`);
    process.exitCode = 2;
    return null;
  }
  if (slice.key.startsWith("hold-")) {
    console.error(`Refusing to emit paths for hold slice '${slice.key}'. Use --json for inventory.`);
    process.exitCode = 2;
    return null;
  }
  return slice;
}

function printTextPlan(plan) {
  console.log("DX-WWW Agent 4 Commit Slice Plan");
  console.log(`Branch: ${plan.branch || "(unknown)"}`);
  console.log(`Dirty entries: ${plan.statusEntryCount}`);
  console.log(`Staged entries: ${plan.stagedEntryCount}`);
  console.log(`Diff shortstat: ${plan.shortstat || "(none)"}`);
  console.log(`Unmerged paths: ${plan.unmergedPaths.length}`);
  console.log(`Ignored generated artifacts: ${plan.ignoredGenerated.total}`);
  console.log("");

  console.log("Slices:");
  for (const slice of plan.slices) {
    console.log(
      `- ${slice.key}: ${slice.total} entries; kinds=${formatKindMap(slice.byKind)}`,
    );
    console.log(`  title: ${slice.title}`);
    console.log(`  policy: ${slice.policy}`);
    console.log(`  checks: ${slice.checks.join(" | ")}`);
    console.log(`  samples: ${slice.samplePaths.join(" | ")}`);
  }
  console.log("");

  console.log("Risk flags:");
  if (plan.riskFlags.length === 0) {
    console.log("- none");
  } else {
    for (const risk of plan.riskFlags) {
      const count = Number.isFinite(risk.count) ? ` count=${risk.count};` : "";
      console.log(`- [${risk.severity}] ${risk.id};${count} ${risk.message}`);
      if (risk.samplePaths?.length > 0) {
        console.log(`  samples: ${risk.samplePaths.join(" | ")}`);
      }
    }
  }
  console.log("");
  console.log(`Next action: ${plan.nextAction}`);
}

function printHandoff(plan) {
  const handoff = plan.handoff;
  console.log("DX-WWW Agent 4 Release-Control Handoff");
  console.log(`Branch: ${plan.branch || "(unknown)"}`);
  console.log(`Dirty entries: ${plan.statusEntryCount}`);
  console.log(`Staged entries: ${plan.stagedEntryCount}`);
  console.log(`Diff shortstat: ${plan.shortstat || "(none)"}`);
  console.log(`Unmerged paths: ${handoff.unmergedPathCount}`);
  console.log(`Ignored generated artifacts: ${handoff.ignoredGeneratedArtifactCount}`);
  console.log("");

  console.log("Stage order:");
  for (const item of handoff.stageOrder) {
    console.log(`- ${item.key}: ${item.count} entries; disposition=${item.disposition}`);
    console.log(`  policy: ${item.policy}`);
    console.log(`  inspect: ${item.pathListCommand}`);
    console.log(`  inspect-nul: ${item.pathListNulCommand}`);
    console.log(`  stage: ${item.stageCommand}`);
  }
  console.log("");

  if (handoff.currentIndexStagedEntryCount > 0) {
    console.log("Current staged index:");
    console.log(`- ${handoff.currentIndexStagedEntryCount} staged entries already exist`);
    console.log(`  samples: ${handoff.currentIndexStagedSamples.join(" | ")}`);
    console.log("");
  }

  console.log("Hold slices:");
  if (handoff.holdSlices.length === 0) {
    console.log("- none");
  } else {
    for (const item of handoff.holdSlices) {
      console.log(`- ${item.key}: ${item.count} entries; ${item.policy}`);
      if (item.samplePaths.length > 0) {
        console.log(`  samples: ${item.samplePaths.join(" | ")}`);
      }
    }
  }
  console.log("");

  console.log("Must-not-commit inventory:");
  if (handoff.mustNotCommit.length === 0) {
    console.log("- none");
  } else {
    for (const item of handoff.mustNotCommit) {
      console.log(`- ${item.source}: ${item.count} paths; ${item.reason}`);
      if (item.samplePaths.length > 0) {
        console.log(`  samples: ${item.samplePaths.join(" | ")}`);
      }
    }
  }
  console.log("");

  console.log("Blocking risks:");
  printRiskList(handoff.blockingRisks);
  console.log("");

  console.log("Review risks:");
  printRiskList(handoff.reviewRisks);
  console.log("");

  console.log("Next actions:");
  for (const action of handoff.nextActions) {
    console.log(`- ${action}`);
  }
}

function printRiskList(risks) {
  if (risks.length === 0) {
    console.log("- none");
    return;
  }
  for (const risk of risks) {
    const count = Number.isFinite(risk.count) ? ` count=${risk.count};` : "";
    console.log(`- [${risk.severity}] ${risk.id};${count} ${risk.message}`);
    if (risk.samplePaths?.length > 0) {
      console.log(`  samples: ${risk.samplePaths.join(" | ")}`);
    }
  }
}

function formatKindMap(byKind) {
  return Object.entries(byKind)
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([kind, count]) => `${kind}:${count}`)
    .join(",");
}

function formatNameStatusEntry(entry) {
  return entry.oldPath
    ? `${entry.status} ${entry.oldPath} -> ${entry.path}`
    : `${entry.status} ${entry.path}`;
}

function readGit(args) {
  return execFileSync("git", args, {
    cwd: process.cwd(),
    encoding: "utf8",
    maxBuffer: GIT_OUTPUT_BUFFER_BYTES,
    stdio: ["ignore", "pipe", "pipe"],
  });
}

function normalizePath(path) {
  return String(path || "").replaceAll("\\", "/");
}

if (require.main === module) {
  main();
}

module.exports = {
  buildCommitSlicePlan,
  buildReleaseHandoff,
  buildMustNotCommitInventory,
  classifySlice,
  parseArgs,
};
