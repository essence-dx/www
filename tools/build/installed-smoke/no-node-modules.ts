const fs = require("node:fs");
const path = require("node:path");

const DEFAULT_SKIPPED_DIRECTORIES = new Set([".git", "target"]);
const DEFAULT_MAX_FINDINGS = 50;

function findNodeModulesDirs(root, options = {}) {
  const maxFindings = options.maxFindings || DEFAULT_MAX_FINDINGS;
  const findings = [];

  scan(root, "");
  return findings;

  function scan(absoluteDir, relativeDir) {
    let entries;
    try {
      entries = fs.readdirSync(absoluteDir, { withFileTypes: true });
    } catch {
      return false;
    }

    for (const entry of entries.sort((left, right) => left.name.localeCompare(right.name))) {
      const relativePath = relativeDir ? `${relativeDir}/${entry.name}` : entry.name;
      const isDirectoryLike = entry.isDirectory() || entry.isSymbolicLink();
      if (entry.name === "node_modules" && isDirectoryLike) {
        findings.push(relativePath);
        if (findings.length >= maxFindings) {
          return true;
        }
        continue;
      }
      if (!entry.isDirectory()) {
        continue;
      }
      if (DEFAULT_SKIPPED_DIRECTORIES.has(entry.name)) {
        continue;
      }
      if (scan(path.join(absoluteDir, entry.name), relativePath)) {
        return true;
      }
    }

    return false;
  }
}

function diffNodeModulesDirs(beforePaths, afterPaths) {
  const before = new Set(normalizePathList(beforePaths));
  return normalizePathList(afterPaths).filter((candidate) => !before.has(candidate));
}

function normalizePathList(paths) {
  return Array.isArray(paths)
    ? paths.filter((item) => typeof item === "string" && item.length > 0)
    : [];
}

module.exports = {
  diffNodeModulesDirs,
  findNodeModulesDirs,
};
