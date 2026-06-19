const fs = require("node:fs");
const path = require("node:path");

const DEFAULT_SOURCE_FRESHNESS_PATHS = [
  "Cargo.toml", "Cargo.lock", "binary/Cargo.toml", "core/Cargo.toml",
  "core/src/lib.rs", "dom/Cargo.toml", "dx-www/Cargo.toml",
  "dx-www/src/cli/mod.rs", "dx-www/src/lib.rs", "dx-www/src/main.rs",
  "error/Cargo.toml", "reactor/Cargo.toml", "related-crates/markdown/Cargo.toml",
  "related-crates/media-icon/Cargo.toml", "related-crates/media-icon/src/lib.rs",
  "related-crates/security/Cargo.toml", "../serializer/Cargo.toml",
  "../serializer/src/lib.rs", "related-crates/style/Cargo.toml",
  "related-crates/style/src/core/engine/mod.rs",
  "related-crates/style/src/core/engine/utility/mod.rs", "related-crates/style/style.css",
  "router/Cargo.toml", "server/Cargo.toml",
];
const DEFAULT_SOURCE_FRESHNESS_DIRS = [
  "dx-www/src", "binary/src", "core/src", "dom/src", "error/src", "reactor/src",
  "router/src", "server/src", "related-crates/markdown/src", "related-crates/media-icon/src",
  "related-crates/security/src", "../serializer/src", "related-crates/style/src",
];
const IGNORED_SOURCE_DIRS = new Set([".dx", ".git", "node_modules", "target"]);

function inspectBinarySourceFreshness(binaryIdentity, options = {}) {
  const repoRoot = path.resolve(options.repoRoot || path.join(__dirname, "..", "..", ".."));
  const sourcePaths = Array.isArray(options.sourcePaths)
    ? options.sourcePaths
    : discoverDefaultSourceFreshnessPaths(repoRoot);
  const base = baseFreshnessResult(repoRoot);

  if (
    !binaryIdentity ||
    binaryIdentity.present !== true ||
    binaryIdentity.kind !== "file" ||
    !Number.isFinite(binaryIdentity.modifiedMs)
  ) {
    return { ...base, reason: "binary-unavailable" };
  }

  const { sources, missingSourcePaths } = collectFreshnessSourceStats(repoRoot, sourcePaths);
  const newestSource = newestFreshnessSource(sources);
  if (!newestSource) {
    return {
      ...base,
      checked: true,
      binaryModifiedMs: binaryIdentity.modifiedMs,
      missingSourcePaths,
      reason: "no-tracked-sources",
    };
  }

  return {
    ...base,
    checked: true,
    fresh: binaryIdentity.modifiedMs >= newestSource.modifiedMs,
    binaryModifiedMs: binaryIdentity.modifiedMs,
    newestSourcePath: newestSource.path,
    newestSourceModifiedMs: newestSource.modifiedMs,
    trackedSourceCount: sources.length,
    trackedSourcePaths: sources.map((source) => source.path),
    missingSourcePaths,
  };
}

function baseFreshnessResult(repoRoot) {
  return {
    required: true,
    repoRoot,
    checked: false,
    fresh: null,
    binaryModifiedMs: null,
    newestSourcePath: null,
    newestSourceModifiedMs: null,
    trackedSourceCount: 0,
    trackedSourcePaths: [],
    missingSourcePaths: [],
  };
}

function collectFreshnessSourceStats(repoRoot, sourcePaths) {
  const sources = [];
  const missingSourcePaths = [];
  for (const sourcePath of sourcePaths) {
    const source = inspectFreshnessPath(repoRoot, sourcePath);
    if (source) {
      sources.push(source);
    } else {
      missingSourcePaths.push(normalizeSourcePath(sourcePath));
    }
  }

  return { sources, missingSourcePaths };
}

function inspectFreshnessPath(repoRoot, sourcePath) {
  const absolutePath = path.join(repoRoot, sourcePath);
  try {
    const stat = fs.statSync(absolutePath);
    if (!stat.isFile()) {
      return null;
    }
    return {
      path: normalizeSourcePath(sourcePath),
      modifiedMs: Math.trunc(stat.mtimeMs),
    };
  } catch {
    return null;
  }
}

function newestFreshnessSource(sources) {
  return sources.reduce((newest, source) => (
    !newest || source.modifiedMs > newest.modifiedMs ? source : newest
  ), null);
}

function discoverDefaultSourceFreshnessPaths(repoRoot) {
  const sourcePaths = new Set(DEFAULT_SOURCE_FRESHNESS_PATHS.map(normalizeSourcePath));
  for (const sourceDir of DEFAULT_SOURCE_FRESHNESS_DIRS) {
    collectRustSourcePaths(repoRoot, sourceDir, sourcePaths);
  }
  return Array.from(sourcePaths).sort();
}

function collectRustSourcePaths(repoRoot, sourceDir, sourcePaths) {
  const absoluteDir = path.join(repoRoot, sourceDir);
  let entries;
  try {
    entries = fs.readdirSync(absoluteDir, { withFileTypes: true });
  } catch {
    return;
  }

  for (const entry of entries) {
    if (IGNORED_SOURCE_DIRS.has(entry.name)) continue;

    const absolutePath = path.join(absoluteDir, entry.name);
    const relativePath = normalizeSourcePath(path.relative(repoRoot, absolutePath));
    if (entry.isDirectory()) {
      collectRustSourcePaths(repoRoot, relativePath, sourcePaths);
    } else if (entry.isFile() && entry.name.endsWith(".rs")) {
      sourcePaths.add(relativePath);
    }
  }
}

function normalizeSourcePath(sourcePath) {
  return String(sourcePath).replace(/\\/g, "/");
}

module.exports = {
  DEFAULT_SOURCE_FRESHNESS_DIRS,
  DEFAULT_SOURCE_FRESHNESS_PATHS,
  discoverDefaultSourceFreshnessPaths,
  inspectBinarySourceFreshness,
};
