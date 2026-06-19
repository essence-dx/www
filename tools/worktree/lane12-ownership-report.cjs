"use strict";

const {
  classifyStatusLines,
  stagedNameStatusToShortStatus,
} = require("./lane12-ownership-rules.cjs");

function summarizeOwnership(entries) {
  const groupsByOwner = new Map();
  const lane12StageablePaths = [];
  const preserveOnlyPaths = [];
  const foreignOwnerPaths = [];
  const unclassifiedPaths = [];
  const generatedArtifactPaths = [];
  const nodeModulesBoundaryPaths = [];
  let mixedCommitRisk = false;

  for (const entry of entries) {
    if (!groupsByOwner.has(entry.owner)) {
      groupsByOwner.set(entry.owner, {
        owner: entry.owner,
        lane: entry.lane,
        count: 0,
        paths: [],
      });
    }
    const group = groupsByOwner.get(entry.owner);
    group.count += 1;
    group.paths.push(entry.path);

    if (entry.sourceBoundary === "generated-artifact") {
      generatedArtifactPaths.push(entry.path);
    }
    if (entry.sourceBoundary === "node-modules-boundary") {
      nodeModulesBoundaryPaths.push(entry.path);
    }

    if (entry.lane12Stageable) {
      lane12StageablePaths.push(entry.path);
    } else if (entry.commitPolicy === "preserve-deletion") {
      preserveOnlyPaths.push(entry.path);
    } else {
      mixedCommitRisk = true;
      foreignOwnerPaths.push(entry.path);
      if (entry.owner === "unclassified") {
        unclassifiedPaths.push(entry.path);
      }
    }
  }

  return {
    entryCount: entries.length,
    mixedCommitRisk,
    lane12StageablePaths,
    preserveOnlyPaths,
    foreignOwnerPaths,
    unclassifiedPaths,
    generatedArtifactPaths,
    nodeModulesBoundaryPaths,
    generatedArtifactCount: generatedArtifactPaths.length,
    nodeModulesBoundaryCount: nodeModulesBoundaryPaths.length,
    groups: Array.from(groupsByOwner.values()).sort(compareGroups),
  };
}

function buildLane12OwnershipReport(
  input,
  {
    generatedAt = null,
    compact = false,
    sampleLimit = 8,
    executionMode = "read-only-status-classifier",
  } = {},
) {
  const entries = classifyStatusLines(input);
  const summary = summarizeOwnership(entries);
  const blockers = buildLane12Blockers(summary, { sampleLimit });
  const blockedLane12Commit = blockers.length > 0;

  const report = {
    schema: "dx.www.worktree.lane12Ownership",
    format: 1,
    lane: 12,
    laneName: "Scope cleanup, docs truth, and worktree coordination",
    executionMode,
    generatedAt,
    status: blockedLane12Commit ? "blocked" : "passed",
    blockedLane12Commit,
    entries,
    summary,
    blockers,
    nextAction: blockedLane12Commit
      ? "stage only summary.lane12StageablePaths; preserve summary.preserveOnlyPaths and leave foreign-owner paths to their lane owners"
      : "Lane 12 status is isolated; stage only summary.lane12StageablePaths",
  };

  if (!compact) {
    return report;
  }

  return compactLane12OwnershipReport(report, { sampleLimit });
}

function buildLane12StagedOwnershipReport(input, options = {}) {
  return buildLane12OwnershipReport(stagedNameStatusToShortStatus(input), {
    ...options,
    executionMode: "read-only-staged-index-classifier",
  });
}

function buildLane12Blockers(summary, { sampleLimit = 8 } = {}) {
  const blockers = [];
  const foreignOwnerPaths = summary.foreignOwnerPaths.filter(
    (path) => !summary.unclassifiedPaths.includes(path),
  );

  if (foreignOwnerPaths.length > 0) {
    blockers.push({
      id: "foreign-owner-dirty-files",
      severity: "blocking",
      count: foreignOwnerPaths.length,
      samplePaths: foreignOwnerPaths.slice(0, sampleLimit),
      message: "dirty files owned by other lanes must not be bundled into Lane 12 work",
    });
  }

  if (summary.generatedArtifactPaths.length > 0) {
    blockers.push({
      id: "generated-artifact-dirty-files",
      severity: "blocking",
      count: summary.generatedArtifactPaths.length,
      samplePaths: summary.generatedArtifactPaths.slice(0, sampleLimit),
      message:
        "generated .dx outputs, receipts, caches, and scratch artifacts require explicit artifact-owner review before staging",
    });
  }

  if (summary.nodeModulesBoundaryPaths.length > 0) {
    blockers.push({
      id: "node-modules-boundary-dirty-files",
      severity: "blocking",
      count: summary.nodeModulesBoundaryPaths.length,
      samplePaths: summary.nodeModulesBoundaryPaths.slice(0, sampleLimit),
      message:
        "node_modules paths must stay out of DX-WWW source and release-control commits",
    });
  }

  if (summary.unclassifiedPaths.length > 0) {
    blockers.push({
      id: "unclassified-dirty-files",
      severity: "blocking",
      count: summary.unclassifiedPaths.length,
      samplePaths: summary.unclassifiedPaths.slice(0, sampleLimit),
      message: "unclassified dirty files need an owner before Lane 12 staging",
    });
  }

  if (summary.preserveOnlyPaths.length > 0) {
    blockers.push({
      id: "preserve-only-deletions",
      severity: "blocking",
      count: summary.preserveOnlyPaths.length,
      samplePaths: summary.preserveOnlyPaths,
      message: "preserve-only deletions are intentional workspace state and should not be restored or mixed into this lane",
    });
  }

  return blockers;
}

function compactLane12OwnershipReport(report, { sampleLimit }) {
  return {
    schema: report.schema,
    format: report.format,
    lane: report.lane,
    laneName: report.laneName,
    executionMode: report.executionMode,
    generatedAt: report.generatedAt,
    compact: true,
    status: report.status,
    blockedLane12Commit: report.blockedLane12Commit,
    summary: compactSummary(report.summary, { sampleLimit }),
    blockers: report.blockers,
    nextAction: report.nextAction,
  };
}

function compactSummary(summary, { sampleLimit }) {
  return {
    entryCount: summary.entryCount,
    mixedCommitRisk: summary.mixedCommitRisk,
    lane12StageablePaths: summary.lane12StageablePaths,
    preserveOnlyPaths: summary.preserveOnlyPaths,
    foreignOwnerCount: foreignOwnerCount(summary),
    unclassifiedCount: summary.unclassifiedPaths.length,
    generatedArtifactCount: summary.generatedArtifactCount,
    generatedArtifactPaths: summary.generatedArtifactPaths,
    nodeModulesBoundaryCount: summary.nodeModulesBoundaryCount,
    nodeModulesBoundaryPaths: summary.nodeModulesBoundaryPaths,
    groups: summary.groups.map((group) => ({
      owner: group.owner,
      lane: group.lane,
      count: group.count,
      samplePaths: group.paths.slice(0, sampleLimit),
    })),
  };
}

function buildLane12StageablePathsReport(report) {
  return {
    schema: "dx.www.worktree.lane12StageablePaths",
    format: 1,
    lane: report.lane,
    laneName: report.laneName,
    generatedAt: report.generatedAt,
    status: report.status,
    blockedLane12Commit: report.blockedLane12Commit,
    lane12StageablePaths: report.summary.lane12StageablePaths,
    preserveOnlyPaths: report.summary.preserveOnlyPaths,
    foreignOwnerCount: foreignOwnerCount(report.summary),
    unclassifiedCount: report.summary.unclassifiedPaths.length,
    generatedArtifactCount: report.summary.generatedArtifactCount,
    generatedArtifactPaths: report.summary.generatedArtifactPaths,
    nodeModulesBoundaryCount: report.summary.nodeModulesBoundaryCount,
    nodeModulesBoundaryPaths: report.summary.nodeModulesBoundaryPaths,
    blockerIds: report.blockers.map((blocker) => blocker.id),
    nextAction: report.blockedLane12Commit
      ? "stage only lane12StageablePaths; preserve preserveOnlyPaths and leave foreign-owner paths to their lane owners"
      : "Lane 12 status is isolated; stage only lane12StageablePaths",
  };
}

function buildLane12CommitPlanReport(report, { sampleLimit = 8 } = {}) {
  const generatedArtifactPaths = new Set(report.summary.generatedArtifactPaths);
  const nodeModulesBoundaryPaths = new Set(report.summary.nodeModulesBoundaryPaths);
  const unclassifiedPaths = new Set(report.summary.unclassifiedPaths);
  const lane12StageablePaths = new Set(report.summary.lane12StageablePaths);

  const stageGroups = report.summary.groups
    .map((group) => {
      const paths = group.paths.filter((path) => lane12StageablePaths.has(path));
      return {
        owner: group.owner,
        lane: group.lane,
        action: "stage",
        count: paths.length,
        paths,
      };
    })
    .filter((group) => group.count > 0);

  const foreignOwnerGroups = report.summary.groups
    .filter((group) => group.lane !== 12 && group.owner !== "unclassified")
    .map((group) => {
      const paths = group.paths.filter(
        (path) =>
          !generatedArtifactPaths.has(path) &&
          !nodeModulesBoundaryPaths.has(path) &&
          !unclassifiedPaths.has(path),
      );
      return {
        owner: group.owner,
        lane: group.lane,
        action: "leave-to-owner-lane",
        count: paths.length,
        samplePaths: paths.slice(0, sampleLimit),
      };
    })
    .filter((group) => group.count > 0);

  const holdGroups = [];
  if (report.summary.preserveOnlyPaths.length > 0) {
    holdGroups.push({
      id: "preserve-only-deletions",
      action: "preserve-do-not-stage",
      count: report.summary.preserveOnlyPaths.length,
      paths: report.summary.preserveOnlyPaths,
    });
  }
  if (report.summary.generatedArtifactPaths.length > 0) {
    holdGroups.push({
      id: "generated-artifacts",
      action: "hold-for-artifact-owner-review",
      count: report.summary.generatedArtifactPaths.length,
      samplePaths: report.summary.generatedArtifactPaths.slice(0, sampleLimit),
    });
  }
  if (report.summary.nodeModulesBoundaryPaths.length > 0) {
    holdGroups.push({
      id: "node-modules-boundary",
      action: "remove-from-source-staging",
      count: report.summary.nodeModulesBoundaryPaths.length,
      samplePaths: report.summary.nodeModulesBoundaryPaths.slice(0, sampleLimit),
    });
  }
  if (report.summary.unclassifiedPaths.length > 0) {
    holdGroups.push({
      id: "unclassified",
      action: "assign-owner-before-staging",
      count: report.summary.unclassifiedPaths.length,
      samplePaths: report.summary.unclassifiedPaths.slice(0, sampleLimit),
    });
  }

  return {
    schema: "dx.www.worktree.commitPlan",
    format: 1,
    lane: report.lane,
    laneName: report.laneName,
    generatedAt: report.generatedAt,
    status: report.status,
    blockedLane12Commit: report.blockedLane12Commit,
    stageGroups,
    preserveOnlyPaths: report.summary.preserveOnlyPaths,
    foreignOwnerGroups,
    holdGroups,
    remainingBlockerIds: report.blockers.map((blocker) => blocker.id),
    nextAction: report.blockedLane12Commit
      ? "stage only stageGroups[].paths for Lane 12; leave foreignOwnerGroups and holdGroups untouched"
      : "stage only stageGroups[].paths",
  };
}

function buildLane12OwnerSummaryReport(report, { sampleLimit = 8 } = {}) {
  return {
    schema: "dx.www.worktree.ownerSummary",
    format: 1,
    lane: report.lane,
    laneName: report.laneName,
    generatedAt: report.generatedAt,
    status: report.status,
    blockedLane12Commit: report.blockedLane12Commit,
    entryCount: report.summary.entryCount,
    foreignOwnerCount: foreignOwnerCount(report.summary),
    unclassifiedCount: report.summary.unclassifiedPaths.length,
    generatedArtifactCount: report.summary.generatedArtifactCount,
    generatedArtifactSamplePaths: report.summary.generatedArtifactPaths.slice(0, sampleLimit),
    nodeModulesBoundaryCount: report.summary.nodeModulesBoundaryCount,
    nodeModulesBoundarySamplePaths: report.summary.nodeModulesBoundaryPaths.slice(0, sampleLimit),
    owners: [...report.summary.groups]
      .sort(compareGroupsForCoordination)
      .map((group) => ({
        owner: group.owner,
        lane: group.lane,
        count: group.count,
        samplePaths: group.paths.slice(0, sampleLimit),
      })),
    blockerIds: report.blockers.map((blocker) => blocker.id),
    nextAction: report.nextAction,
  };
}

function foreignOwnerCount(summary) {
  return Math.max(0, summary.foreignOwnerPaths.length - summary.unclassifiedPaths.length);
}

function compareGroups(left, right) {
  const leftLane = left.lane === null ? Number.MAX_SAFE_INTEGER : left.lane;
  const rightLane = right.lane === null ? Number.MAX_SAFE_INTEGER : right.lane;
  if (leftLane !== rightLane) {
    return leftLane - rightLane;
  }
  return left.owner.localeCompare(right.owner);
}

function compareGroupsForCoordination(left, right) {
  const leftRank = left.lane === 12 ? 0 : left.lane === null ? 2 : 1;
  const rightRank = right.lane === 12 ? 0 : right.lane === null ? 2 : 1;
  if (leftRank !== rightRank) {
    return leftRank - rightRank;
  }
  return compareGroups(left, right);
}

module.exports = {
  buildLane12CommitPlanReport,
  buildLane12OwnershipReport,
  buildLane12OwnerSummaryReport,
  buildLane12StagedOwnershipReport,
  buildLane12StageablePathsReport,
  summarizeOwnership,
};
