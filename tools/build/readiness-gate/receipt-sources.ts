const fs = require("node:fs");
const path = require("node:path");

const {
  CHECK_LAUNCH_RECEIPT,
  INSTALLED_BINARY_SMOKE_RECEIPT,
  NEXT_RUST_BOUNDARY_RECEIPT,
  READINESS_RECEIPT,
  SOURCE_BUILD_RECEIPT,
  ZED_HANDOFF_RECEIPT,
} = require("./constants.ts");
const { normalizePath } = require("./io.ts");

function resolveReceiptSources(projectRoot) {
  return {
    checkLaunch: firstExisting(projectRoot, [
      hubCandidate(projectRoot, CHECK_LAUNCH_RECEIPT),
      workspaceCandidate(projectRoot, "www", CHECK_LAUNCH_RECEIPT),
    ]),
    installedBinarySmoke: firstExisting(projectRoot, [
      hubCandidate(projectRoot, INSTALLED_BINARY_SMOKE_RECEIPT),
    ]),
    nextRustBoundary: firstExisting(projectRoot, [
      hubCandidate(projectRoot, NEXT_RUST_BOUNDARY_RECEIPT),
      workspaceCandidate(projectRoot, "www", NEXT_RUST_BOUNDARY_RECEIPT),
    ]),
    readiness: firstExisting(projectRoot, [
      hubCandidate(projectRoot, READINESS_RECEIPT),
      workspaceCandidate(projectRoot, "www", READINESS_RECEIPT),
    ]),
    sourceBuild: firstExisting(projectRoot, [
      hubCandidate(projectRoot, SOURCE_BUILD_RECEIPT),
      workspaceCandidate(projectRoot, "www", SOURCE_BUILD_RECEIPT),
    ]),
    zedHandoff: firstExisting(projectRoot, [
      hubCandidate(projectRoot, ZED_HANDOFF_RECEIPT),
      workspaceCandidate(projectRoot, "www", ZED_HANDOFF_RECEIPT),
    ]),
  };
}

function hubCandidate(projectRoot, relativePath) {
  return {
    absolutePath: path.join(projectRoot, relativePath),
    relativePath,
    source: {
      kind: "hub",
      workspace: null,
      path: normalizePath(relativePath),
    },
  };
}

function workspaceCandidate(projectRoot, workspace, relativePath) {
  const workspacePath = path.join(workspace, relativePath);
  return {
    absolutePath: path.join(projectRoot, workspacePath),
    relativePath: normalizePath(workspacePath),
    source: {
      kind: "workspace",
      workspace,
      path: normalizePath(workspacePath),
    },
  };
}

function firstExisting(projectRoot, candidates) {
  const found = candidates.find((candidate) => fs.existsSync(candidate.absolutePath));
  return found || candidates[0] || null;
}

module.exports = {
  resolveReceiptSources,
};
