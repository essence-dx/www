const fs = require("node:fs");
const path = require("node:path");
const { readText, relativeFromRepo } = require("./paths.js");

const CHECKED_SOURCE_ROOTS = ["core/src", "dx-www/src"];
const ALLOWED_SOURCE_FILES = new Set(["dx-www/src/next_rust.rs"]);
const DIRECT_VENDOR_SOURCE_REASON =
  "Vendored Next/Turbopack Rust source must be wrapped, not compiled directly into DX runtime";

function sourceRootPath(repoRoot, root) {
  return path.join(repoRoot, ...root.split("/"));
}

function sortedRustFiles(rootPath) {
  if (!fs.existsSync(rootPath)) {
    return [];
  }

  const entries = fs.readdirSync(rootPath, { withFileTypes: true });
  return entries
    .flatMap((entry) => {
      const childPath = path.join(rootPath, entry.name);
      if (entry.isDirectory()) {
        return sortedRustFiles(childPath);
      }
      return entry.isFile() && entry.name.endsWith(".rs") ? [childPath] : [];
    })
    .sort((left, right) => left.localeCompare(right));
}

function normalizeTarget(target) {
  return target.split("\\").join("/");
}

function collectSourceInclusions(relativePath, text) {
  const findings = [];
  const vendorTarget = String.raw`([^"']*vendor[\\/]next-rust[^"']*)`;
  const includePattern = new RegExp(String.raw`\b(include|include_str|include_bytes)!\s*\(\s*["']${vendorTarget}["']\s*\)`, "g");
  const pathAttributePattern = new RegExp(String.raw`#\s*\[\s*path\s*=\s*["']${vendorTarget}["']\s*\]`, "g");

  for (const match of text.matchAll(includePattern)) {
    findings.push({
      file: relativePath,
      pattern: `${match[1]}!`,
      target: normalizeTarget(match[2]),
      reason: DIRECT_VENDOR_SOURCE_REASON,
    });
  }

  for (const match of text.matchAll(pathAttributePattern)) {
    findings.push({
      file: relativePath,
      pattern: "#[path]",
      target: normalizeTarget(match[1]),
      reason: DIRECT_VENDOR_SOURCE_REASON,
    });
  }

  return findings;
}

function collectVendorSourceInclusion(repoRoot) {
  const checkedRoots = CHECKED_SOURCE_ROOTS.filter((root) => fs.existsSync(sourceRootPath(repoRoot, root)));
  const checkedFiles = [];
  const forbiddenInclusions = [];

  for (const root of checkedRoots) {
    for (const filePath of sortedRustFiles(sourceRootPath(repoRoot, root))) {
      const relativePath = relativeFromRepo(repoRoot, filePath);
      checkedFiles.push(relativePath);
      if (!ALLOWED_SOURCE_FILES.has(relativePath)) {
        forbiddenInclusions.push(...collectSourceInclusions(relativePath, readText(filePath)));
      }
    }
  }

  return {
    checkedRoots,
    checkedFiles,
    forbiddenInclusions,
  };
}

module.exports = {
  collectVendorSourceInclusion,
};
