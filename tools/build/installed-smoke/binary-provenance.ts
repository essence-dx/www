const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const {
  DEFAULT_SOURCE_FRESHNESS_DIRS,
  DEFAULT_SOURCE_FRESHNESS_PATHS,
  discoverDefaultSourceFreshnessPaths,
  inspectBinarySourceFreshness,
} = require("./source-freshness.ts");

const DEFAULT_INSTALLED_BINARY = "G:\\Dx\\bin\\dx-www.exe";

function classifyBinary(binary) {
  const installedDefault =
    normalizeBinaryPath(binary) === normalizeBinaryPath(DEFAULT_INSTALLED_BINARY);

  return {
    defaultBinary: DEFAULT_INSTALLED_BINARY,
    override: !installedDefault,
    role: installedDefault ? "installed-default" : "candidate-override",
  };
}

function inspectBinary(binary) {
  const resolved = path.resolve(binary);
  try {
    const stat = fs.statSync(resolved);
    const isFile = stat.isFile();
    return {
      path: resolved,
      present: true,
      kind: isFile ? "file" : "non-file",
      byteLength: isFile ? stat.size : null,
      modifiedMs: Math.trunc(stat.mtimeMs),
      sha256: isFile ? sha256File(resolved) : null,
    };
  } catch (error) {
    return {
      path: resolved,
      present: false,
      kind: null,
      byteLength: null,
      modifiedMs: null,
      sha256: null,
      error: error && error.code ? error.code : "unreadable",
    };
  }
}

function sha256File(filePath) {
  return crypto.createHash("sha256").update(fs.readFileSync(filePath)).digest("hex");
}

function normalizeBinaryPath(binary) {
  return path.resolve(binary).replace(/\//g, "\\").toLowerCase();
}

module.exports = {
  classifyBinary,
  DEFAULT_INSTALLED_BINARY,
  DEFAULT_SOURCE_FRESHNESS_DIRS,
  DEFAULT_SOURCE_FRESHNESS_PATHS,
  discoverDefaultSourceFreshnessPaths,
  inspectBinary,
  inspectBinarySourceFreshness,
};
