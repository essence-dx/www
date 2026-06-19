const fs = require("node:fs");
const path = require("node:path");
const { EXPECTED_IMPORTED_GROUPS } = require("./constants.js");
const { readText } = require("./paths.js");

const FORBIDDEN_VENDOR_CARGO_DEPENDENCIES = {
  "next-core": "Vendored build-layer groups must not depend on Next.js runtime core",
  "next-napi-bindings": "Vendored build-layer groups must not depend on Node/NAPI runtime bindings",
  "turbopack-nodejs": "Vendored build-layer groups must not depend on Node-backed Turbopack runtime",
};

function manifestPath(repoRoot, group) {
  return path.join(repoRoot, "vendor", "next-rust", ...group.split("/"), "Cargo.toml");
}

function manifestDisplayPath(group) {
  return `vendor/next-rust/${group}/Cargo.toml`;
}

function cargoDependencyPresent(text, dependency) {
  const escaped = dependency.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const inlineDependency = new RegExp(`(^|\\n)\\s*${escaped}\\s*=`, "m");
  const dependencyTable = new RegExp(`(^|\\n)\\s*\\[[^\\]\\n]*\\.${escaped}\\]\\s*(?=\\r?\\n|$)`, "m");
  return inlineDependency.test(text) || dependencyTable.test(text);
}

function collectVendoredCargoDependencyFindings(relativePath, text) {
  return Object.entries(FORBIDDEN_VENDOR_CARGO_DEPENDENCIES)
    .filter(([dependency]) => cargoDependencyPresent(text, dependency))
    .map(([dependency, reason]) => ({
      manifest: relativePath,
      dependency,
      reason,
    }));
}

function collectVendoredCargoDependencyClaims(repoRoot) {
  const checkedManifests = [];
  const forbiddenDependencies = [];

  for (const group of EXPECTED_IMPORTED_GROUPS) {
    const absolutePath = manifestPath(repoRoot, group);
    if (!fs.existsSync(absolutePath)) {
      continue;
    }

    const relativePath = manifestDisplayPath(group);
    checkedManifests.push(relativePath);
    forbiddenDependencies.push(
      ...collectVendoredCargoDependencyFindings(relativePath, readText(absolutePath)),
    );
  }

  return {
    checkedManifests,
    forbiddenDependencies,
  };
}

module.exports = {
  collectVendoredCargoDependencyClaims,
};
