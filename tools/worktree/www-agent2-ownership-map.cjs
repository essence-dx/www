"use strict";

const { execFileSync } = require("node:child_process");

const SAMPLE_LIMIT = 8;
const REPORT_SCHEMA = "dx.www.worktree.agent2OwnershipMap";

const DOMAINS = Object.freeze([
  {
    key: "deleted-root-junk",
    owner: "release-control",
    decision: "preserve deletion of the root junk file named '-'",
    patterns: [/^-$/],
  },
  {
    key: "workspace-junk",
    owner: "workspace-junk-quarantine",
    decision: "do not stage Windows-reserved or root junk paths; remove only with release-control approval",
    patterns: [/^(?:NUL|CON|PRN|AUX)(?:\..*)?$/i],
  },
  {
    key: "generated",
    owner: "artifact-quarantine",
    decision: "quarantine unless a lane explicitly asks to keep proof artifacts",
    patterns: [
      /^\.dx\//,
      /^\.tmp\//,
      /^benchmarks\/reports\//,
      /^examples\/template\/\.dx\/(?:build|receipts)\//,
      /^librust_out\.rlib$/,
    ],
  },
  {
    key: "forge",
    owner: "forge-lanes",
    decision: "commit only with Forge package/provenance/trust source work",
    patterns: [
      /^core\/src\/ecosystem\/forge_/,
      /^dx-www\/src\/cli\/forge_/,
      /^examples\/template\/\.dx\/forge\//,
      /^examples\/template\/forge-package-status-read-model\.ts$/,
      /^benchmarks\/.*forge.*\.(?:cjs|mjs|ts|tsx)$/,
    ],
  },
  {
    key: "dx-style",
    owner: "style-lanes",
    decision: "keep separate from dx-www core and template commits",
    patterns: [
      /^related-crates\/style\//,
      /^tools\/dx-style(?:\/|$)/,
      /^tools\/style(?:\/|$)/,
      /^benchmarks\/dx-style/,
      /^dx-www\/src\/cli\/dx_style_support\.rs$/,
    ],
  },
  {
    key: "build-smoke",
    owner: "build-artifact-lanes",
    decision: "commit with installed-smoke, readiness, or dx build artifact contract work",
    patterns: [
      /^tools\/build\/installed-smoke\//,
      /^tools\/build\/readiness-gate\//,
      /^tools\/launch\/launch-readiness-gate\.js$/,
      /^tools\/launch\/readiness-gate\//,
      /^benchmarks\/(?:dx-build|installed-smoke|source-build)/,
      /^dx-www\/tests\/(?:dx_build|source_build)/,
      /^dx-www\/src\/build\//,
      /^dx-www\/src\/cli\/preview_(?:command|contract)\.rs$/,
    ],
  },
  {
    key: "app-router-route-handlers",
    owner: "router-runtime-lanes",
    decision: "commit with App Router, route handler, metadata, or routing semantic work",
    patterns: [
      /^core\/src\/delivery\//,
      /^dx-www\/src\/cli\/app_/,
      /^dx-www\/src\/router\//,
      /^dx-www\/tests\/app_router/,
      /^benchmarks\/(?:app-|route-handler|tsx-app-router|metadata|nextjs-compatibility)/,
      /^router\//,
    ],
  },
  {
    key: "hot-reload-diagnostics",
    owner: "dev-feedback-lanes",
    decision: "commit with hot reload, overlay, diagnostics, or dev server work",
    patterns: [
      /^dx-www\/src\/dev\//,
      /^dx-www\/src\/diagnostics/,
      /^dx-www\/src\/hot_reload_protocol\.rs$/,
      /^dx-www\/tests\/diagnostics_cli\.rs$/,
      /^benchmarks\/(?:diagnostics|dx-dev|hot-reload)/,
    ],
  },
  {
    key: "resolver-linker",
    owner: "resolver-lanes",
    decision: "commit with source-first resolver and no-node_modules boundary work",
    patterns: [
      /^dx-www\/src\/build\/source_engine\/(?:module_|ecmascript|discovery|graph|ecosystem)/,
      /^tools\/build-graph\//,
      /^benchmarks\/(?:source-resolver|source-linker|module-linker)/,
    ],
  },
  {
    key: "dx-www-core",
    owner: "www-core-lanes",
    decision: "commit with Rust crate API, CLI, project, and build executable work",
    patterns: [
      /^dx-www\/Cargo\.toml$/,
      /^dx-www\/src\//,
      /^dx-www\/tests\//,
      /^core\/src\/lib\.rs$/,
      /^dx-www\/src\/lib\.rs$/,
    ],
  },
  {
    key: "template",
    owner: "template-product-lanes",
    decision: "commit with www-template product source, not generated .dx outputs",
    patterns: [
      /^examples\/template\//,
      /^examples\/conversion-proof\//,
      /^tools\/launch\/materialize-www-template\.ts$/,
      /^benchmarks\/www-template/,
      /^benchmarks\/default-www-template/,
    ],
  },
  {
    key: "docs-status",
    owner: "docs-and-truth-lanes",
    decision: "commit after source behavior is proven; avoid docs-only score movement",
    patterns: [
      /^(?:README|DX|TODO|CHANGELOG)\.md$/,
      /^docs\//,
      /^dx-www\/README\.md$/,
      /^examples\/template\/(?:README|TODO|CHANGELOG)\.md$/,
    ],
  },
  {
    key: "vendor-reference",
    owner: "scope-boundary-lanes",
    decision: "review as provenance/reference cleanup; do not bundle with runtime features",
    patterns: [
      /^tools\/vendor\//,
      /^tools\/next-rust-merge\//,
      /^core\/src\/devtools\.rs$/,
      /^dx-www\/src\/(?:next_rust|next_rust_|cli\/next_)/,
      /^benchmarks\/next-rust/,
      /^benchmarks\/public-framework/,
    ],
  },
  {
    key: "worker-control",
    owner: "coordination-lanes",
    decision: "commit with worker automation and worktree hygiene tooling",
    patterns: [
      /^\.gitignore$/,
      /^start-www-worker\.ps1$/,
      /^worker-lanes\//,
      /^tools\/launch-stabilize\/coordinator\.cjs$/,
      /^tools\/worktree\//,
      /^benchmarks\/.*ownership.*\.(?:cjs|mjs|ts|tsx)$/,
    ],
  },
  {
    key: "benchmark-tests",
    owner: "test-owner-lanes",
    decision: "commit with the source lane that owns the asserted behavior",
    patterns: [/^benchmarks\//],
  },
]);

const COMMIT_PLAN = Object.freeze([
  ["coordination-lanes", "worker-control plus ownership tooling after review"],
  ["www-core-lanes", "dx-www Rust source and crate-level API changes"],
  ["router-runtime-lanes", "App Router, route handlers, metadata, and routing tests"],
  ["resolver-lanes", "source-first resolver, module linker, and no-node boundary changes"],
  ["dev-feedback-lanes", "dev server, hot reload, overlay, and diagnostics changes"],
  ["style-lanes", "dx-style source, fixtures, and style-only checks"],
  ["build-artifact-lanes", "dx build contracts, installed-smoke tools, readiness gates"],
  ["forge-lanes", "Forge package/provenance/trust/source changes"],
  ["template-product-lanes", "www-template authored source and docs"],
  ["test-owner-lanes", "benchmark files staged only with their source-owner lane"],
  ["scope-boundary-lanes", "reference-only Next/Turbopack scope cleanup"],
  ["docs-and-truth-lanes", "docs/status updates after source proof"],
  ["artifact-quarantine", "generated outputs kept out unless explicitly accepted as proof"],
  ["release-control", "preserve the deleted root junk file named '-'"],
  ["workspace-junk-quarantine", "do not stage Windows-reserved root junk paths"],
  ["needs-human-routing", "do not stage until an owner lane claims the path"],
]);

const REVIEW_RISK_PLAN = Object.freeze({
  "source-deletions": {
    owner: "release-control",
    action: "route each source deletion to its lane owner before staging",
  },
  "untracked-legacy-js-tests": {
    owner: "test-owner-lanes",
    action: "review legacy .cjs/.mjs tests against the .ts-default rule before staging",
  },
  "broad-dx-www-core": {
    owner: "www-core-lanes",
    action: "split broad dx-www core changes by command/runtime owner before staging",
  },
});

function main(argv = process.argv.slice(2)) {
  const options = parseArgs(argv);
  const report = buildOwnershipReport(options);

  if (options.handoff && options.json) {
    process.stdout.write(`${JSON.stringify(report.handoff, null, 2)}\n`);
  } else if (options.handoff) {
    printHandoff(report.handoff);
  } else if (options.json) {
    process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
  } else {
    printReport(report);
  }

  if (options.strict && report.blockingRiskCount > 0) {
    process.exitCode = 2;
  }
}

function parseArgs(argv) {
  return {
    handoff: argv.includes("--handoff"),
    json: argv.includes("--json"),
    strict: argv.includes("--strict"),
    sampleLimit: readNumericOption(argv, "--sample-limit", SAMPLE_LIMIT),
  };
}

function readNumericOption(argv, flag, fallback) {
  const index = argv.indexOf(flag);
  if (index === -1 || index === argv.length - 1) {
    return fallback;
  }
  const value = Number.parseInt(argv[index + 1], 10);
  return Number.isFinite(value) && value > 0 ? value : fallback;
}

function buildOwnershipReport(options = {}) {
  const entries = readStatusEntries();
  const unmerged = readUnmergedPaths();
  const groups = groupEntries(entries, options);
  const risks = buildRiskList(entries, groups, unmerged, options);
  const commitPlan = buildCommitPlan(groups);
  const blockingRiskCount = risks.filter((risk) => risk.severity === "blocking").length;
  const report = {
    schema: REPORT_SCHEMA,
    format: 2,
    generatedAt: new Date().toISOString(),
    branch: readGit(["branch", "--show-current"]).trim(),
    shortstat: readGit(["diff", "--shortstat"]).trim(),
    dirtyEntryCount: entries.length,
    blockingRiskCount,
    entries,
    groups,
    risks,
    commitPlan,
    unmerged,
  };
  report.handoff = buildHandoff(report);
  return report;
}

function readStatusEntries() {
  const records = readGit(["status", "--porcelain=v1", "-z", "--untracked-files=all"])
    .split("\0")
    .filter(Boolean);
  const entries = [];

  for (let index = 0; index < records.length; index += 1) {
    const record = records[index];
    const status = record.slice(0, 2);
    const path = normalizePath(record.slice(3));
    let oldPath = null;

    if (isRenameOrCopyStatus(status) && index + 1 < records.length) {
      oldPath = normalizePath(records[index + 1]);
      index += 1;
    }

    entries.push({
      status,
      path,
      oldPath,
      displayPath: oldPath ? `${oldPath} -> ${path}` : path,
      domain: classify(path),
      kind: classifyChangeKind(status),
    });
  }

  return entries;
}

function readUnmergedPaths() {
  const output = readGit(["diff", "--name-only", "--diff-filter=U", "-z"]);
  return output
    .split("\0")
    .filter(Boolean)
    .map(normalizePath);
}

function readGit(args) {
  return execFileSync("git", args, {
    cwd: process.cwd(),
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });
}

function normalizePath(path) {
  return path.replace(/\\/g, "/");
}

function isRenameOrCopyStatus(status) {
  return status.includes("R") || status.includes("C");
}

function classify(path) {
  for (const domain of DOMAINS) {
    if (domain.patterns.some((pattern) => pattern.test(path))) {
      return domain;
    }
  }
  return {
    key: "unclassified",
    owner: "needs-human-routing",
    decision: "do not stage until an owner lane claims it",
  };
}

function classifyChangeKind(status) {
  if (status === "??") return "untracked";
  if (status.includes("U")) return "unmerged";
  if (status.includes("D")) return "deleted";
  if (status.includes("A")) return "added";
  if (status.includes("R")) return "renamed";
  if (status.includes("M")) return "modified";
  return "changed";
}

function groupEntries(entries, { sampleLimit = SAMPLE_LIMIT } = {}) {
  const groups = new Map();

  for (const entry of entries) {
    const key = entry.domain.key;
    if (!groups.has(key)) {
      groups.set(key, {
        key,
        owner: entry.domain.owner,
        decision: entry.domain.decision,
        total: 0,
        byKind: {},
        samples: [],
      });
    }
    const group = groups.get(key);
    group.total += 1;
    group.byKind[entry.kind] = (group.byKind[entry.kind] || 0) + 1;
    if (group.samples.length < sampleLimit) {
      group.samples.push(formatStatusPath(entry));
    }
  }

  return Array.from(groups.values()).sort((a, b) => {
    if (b.total !== a.total) return b.total - a.total;
    return a.key.localeCompare(b.key);
  });
}

function buildRiskList(entries, groups, unmerged, { sampleLimit = SAMPLE_LIMIT } = {}) {
  const risks = [];
  const rootJunk = entries.find((entry) => entry.path === "-");
  if (!rootJunk || !rootJunk.status.includes("D")) {
    risks.push({
      id: "root-junk-deletion-missing",
      severity: "blocking",
      count: 1,
      message: "Root junk file '-' is not represented as a preserved deletion.",
      paths: ["-"],
    });
  }
  if (unmerged.length > 0) {
    risks.push({
      id: "unmerged-paths",
      severity: "blocking",
      count: unmerged.length,
      message: `${unmerged.length} unmerged paths are present.`,
      paths: sampleList(unmerged, sampleLimit),
    });
  }

  const deletedSource = entries.filter(
    (entry) =>
      entry.kind === "deleted" &&
      entry.path !== "-" &&
      !["generated", "vendor-reference"].includes(entry.domain.key),
  );
  if (deletedSource.length > 0) {
    risks.push({
      id: "source-deletions",
      severity: "review",
      count: deletedSource.length,
      message: `${deletedSource.length} source deletions need lane-owner review.`,
      paths: sampleEntryPaths(deletedSource, sampleLimit),
    });
  }

  const generated = groups.find((group) => group.key === "generated");
  if (generated && generated.total > 0) {
    risks.push({
      id: "generated-artifacts",
      severity: "quarantine",
      count: generated.total,
      message: `${generated.total} generated artifact entries should be quarantined before source commits.`,
      paths: generated.samples,
    });
  }

  const workspaceJunk = groups.find((group) => group.key === "workspace-junk");
  if (workspaceJunk && workspaceJunk.total > 0) {
    risks.push({
      id: "workspace-junk-paths",
      severity: "blocking",
      count: workspaceJunk.total,
      message: `${workspaceJunk.total} Windows-reserved or root junk paths must not be staged.`,
      paths: workspaceJunk.samples,
    });
  }

  const untrackedLegacyTests = entries.filter(
    (entry) =>
      entry.status === "??" &&
      /^benchmarks\/.*\.(?:cjs|mjs)$/.test(entry.path),
  );
  if (untrackedLegacyTests.length > 0) {
    risks.push({
      id: "untracked-legacy-js-tests",
      severity: "review",
      count: untrackedLegacyTests.length,
      message:
        "Untracked legacy JS tests need review against the new .ts-default rule.",
      paths: sampleEntryPaths(untrackedLegacyTests, sampleLimit),
    });
  }

  const broadCore = groups.find((group) => group.key === "dx-www-core");
  if (broadCore && broadCore.total > 50) {
    risks.push({
      id: "broad-dx-www-core",
      severity: "review",
      count: broadCore.total,
      message: `dx-www core has ${broadCore.total} entries; split by command/runtime owner before staging.`,
      paths: broadCore.samples,
    });
  }

  const unclassified = groups.find((group) => group.key === "unclassified");
  if (unclassified && unclassified.total > 0) {
    risks.push({
      id: "unclassified-paths",
      severity: "blocking",
      count: unclassified.total,
      message: `${unclassified.total} entries are unclassified and must not be staged blind.`,
      paths: unclassified.samples,
    });
  }

  const plannedOwners = new Set(COMMIT_PLAN.map(([owner]) => owner));
  const unplannedOwners = groups.filter((group) => !plannedOwners.has(group.owner));
  if (unplannedOwners.length > 0) {
    risks.push({
      id: "unplanned-owner-groups",
      severity: "blocking",
      count: unplannedOwners.length,
      message: `${unplannedOwners.length} owner groups are missing from the commit plan.`,
      paths: unplannedOwners.map((group) => group.owner),
    });
  }

  return risks;
}

function sampleEntryPaths(entries, sampleLimit = SAMPLE_LIMIT) {
  return sampleList(
    entries.map((entry) => entry.displayPath || entry.path),
    sampleLimit,
  );
}

function sampleList(items, sampleLimit = SAMPLE_LIMIT) {
  const sample = items
    .slice(0, sampleLimit)
    .map((item) => String(item));
  if (items.length > sampleLimit) {
    sample.push("...");
  }
  return sample;
}

function buildCommitPlan(groups) {
  return COMMIT_PLAN.map(([owner, action]) => {
    const ownedGroups = groups.filter((group) => group.owner === owner);
    return {
      owner,
      count: ownedGroups.reduce((sum, group) => sum + group.total, 0),
      groups: ownedGroups.map((group) => group.key),
      disposition: dispositionForOwner(owner),
      action,
    };
  });
}

function dispositionForOwner(owner) {
  if (owner === "artifact-quarantine") return "quarantine";
  if (owner === "release-control") return "preserve-delete";
  if (owner === "workspace-junk-quarantine") return "blocked";
  if (owner === "needs-human-routing") return "blocked";
  return "stage-only-with-owner-lane";
}

function buildReviewCandidates(risks) {
  return risks
    .filter((risk) => risk.severity === "review")
    .map((risk) => {
      const plan = REVIEW_RISK_PLAN[risk.id] || {
        owner: "needs-human-routing",
        action: "review before staging",
      };
      return {
        riskId: risk.id,
        owner: plan.owner,
        count: risk.count || risk.paths.length,
        disposition: "review-before-staging",
        action: plan.action,
        samplePaths: risk.paths,
      };
    });
}

function buildHandoff(report) {
  const nonEmptyPlan = report.commitPlan.filter((item) => item.count > 0);
  const itemsByDisposition = (disposition) =>
    nonEmptyPlan
      .filter((item) => item.disposition === disposition)
      .map(summarizePlanItem);

  return {
    schema: `${REPORT_SCHEMA}.handoff`,
    format: 1,
    generatedAt: report.generatedAt,
    branch: report.branch,
    shortstat: report.shortstat,
    dirtyEntryCount: report.dirtyEntryCount,
    unmergedPathCount: report.unmerged.length,
    blockingRiskCount: report.blockingRiskCount,
    ownerGroupCount: report.groups.length,
    nonEmptyOwnerCount: nonEmptyPlan.length,
    stageCandidates: itemsByDisposition("stage-only-with-owner-lane"),
    quarantineCandidates: itemsByDisposition("quarantine"),
    preservedDeletions: itemsByDisposition("preserve-delete"),
    blockedCandidates: itemsByDisposition("blocked"),
    reviewCandidates: buildReviewCandidates(report.risks),
    risks: report.risks.map((risk) => ({
      id: risk.id,
      severity: risk.severity,
      count: risk.count || risk.paths.length,
      message: risk.message,
      samplePaths: risk.paths,
    })),
    nextActions: [
      "Stage only by owner lane; do not stage the whole worktree.",
      "Quarantine generated artifacts unless a lane explicitly accepts them as proof.",
      "Keep the root junk file named '-' as a preserved deletion.",
      "Do not stage Windows-reserved root junk paths such as NUL.",
      "Use --json for the complete manifest before any release-control staging plan.",
    ],
  };
}

function summarizePlanItem(item) {
  return {
    owner: item.owner,
    count: item.count,
    groups: item.groups,
    action: item.action,
  };
}

function formatStatusPath(entry) {
  return `${entry.status} ${entry.displayPath || entry.path}`;
}

function printHandoff(handoff) {
  console.log("DX-WWW Agent 2 Release Handoff");
  console.log(`Branch: ${handoff.branch || "(unknown)"}`);
  console.log(`Dirty entries: ${handoff.dirtyEntryCount}`);
  console.log(`Diff shortstat: ${handoff.shortstat || "(none)"}`);
  console.log(`Unmerged paths: ${handoff.unmergedPathCount}`);
  console.log(`Blocking risks: ${handoff.blockingRiskCount}`);
  console.log("");

  printPlanSection("Stage candidates by owner lane", handoff.stageCandidates);
  printPlanSection("Quarantine candidates", handoff.quarantineCandidates);
  printPlanSection("Preserved deletions", handoff.preservedDeletions);
  printPlanSection("Blocked candidates", handoff.blockedCandidates);
  printReviewSection("Review-before-staging candidates", handoff.reviewCandidates);

  console.log("Risks:");
  if (handoff.risks.length === 0) {
    console.log("- none");
  } else {
    for (const risk of handoff.risks) {
      const paths = risk.samplePaths.length > 0 ? `: ${risk.samplePaths.join(", ")}` : "";
      console.log(`- [${risk.severity}] ${risk.id} (${risk.count}): ${risk.message}${paths}`);
    }
  }
  console.log("");

  console.log("Next actions:");
  for (const action of handoff.nextActions) {
    console.log(`- ${action}`);
  }
}

function printPlanSection(title, items) {
  console.log(`${title}:`);
  if (items.length === 0) {
    console.log("- none");
  } else {
    for (const item of items) {
      console.log(`- ${item.owner}: ${item.count} entries; groups=${item.groups.join(",")}`);
    }
  }
  console.log("");
}

function printReviewSection(title, items) {
  console.log(`${title}:`);
  if (items.length === 0) {
    console.log("- none");
  } else {
    for (const item of items) {
      console.log(`- ${item.owner}: ${item.count} entries; risk=${item.riskId}`);
    }
  }
  console.log("");
}

function printReport({
  branch,
  shortstat,
  dirtyEntryCount,
  groups,
  risks,
  unmerged,
  commitPlan,
  blockingRiskCount,
}) {
  console.log("DX-WWW Agent 2 Worktree Ownership Map");
  console.log(`Branch: ${branch || "(unknown)"}`);
  console.log(`Dirty entries: ${dirtyEntryCount}`);
  console.log(`Diff shortstat: ${shortstat || "(none)"}`);
  console.log(`Unmerged paths: ${unmerged.length}`);
  console.log(`Blocking risks: ${blockingRiskCount}`);
  console.log("");

  console.log("Domain summary:");
  for (const group of groups) {
    console.log(
      `- ${group.key}: ${group.total} entries; owner=${group.owner}; kinds=${formatKindMap(
        group.byKind,
      )}`,
    );
    console.log(`  decision: ${group.decision}`);
    console.log(`  samples: ${group.samples.join(" | ")}`);
  }
  console.log("");

  console.log("Risk flags:");
  if (risks.length === 0) {
    console.log("- none");
  } else {
    for (const risk of risks) {
      const paths = risk.paths.length > 0 ? `: ${risk.paths.join(", ")}` : "";
      console.log(`- [${risk.severity}] ${risk.message}${paths}`);
    }
  }
  console.log("");

  console.log("Commit/quarantine plan:");
  for (const item of commitPlan) {
    console.log(
      `- ${item.owner}: ${item.count} entries; disposition=${item.disposition}; ${item.action}`,
    );
  }
}

function formatKindMap(byKind) {
  return Object.entries(byKind)
    .sort((a, b) => a[0].localeCompare(b[0]))
    .map(([kind, count]) => `${kind}:${count}`)
    .join(",");
}

if (require.main === module) {
  main();
}

module.exports = {
  buildCommitPlan,
  buildHandoff,
  buildOwnershipReport,
  buildReviewCandidates,
  buildRiskList,
  classify,
  classifyChangeKind,
  formatStatusPath,
  groupEntries,
  readStatusEntries,
};
