const fs = require("node:fs");
const path = require("node:path");
const { readText } = require("./paths.js");

const PUBLIC_MANIFEST_PATHS = [
  "Cargo.toml",
  "dx-www/Cargo.toml",
  "package.json",
  "dx-www/package.json",
];

const FORBIDDEN_PACKAGE_DEPENDENCIES = {
  next: "Next.js must not become a public DX-WWW dependency foundation",
  react: "React/RSC must not become a required DX-WWW core dependency",
  "react-dom": "React/RSC must not become a required DX-WWW core dependency",
  "react-server-dom-webpack": "React/RSC must not become a required DX-WWW core dependency",
  turbo: "Turborepo/npm tooling must not become the default DX-WWW foundation",
  turborepo: "Turborepo/npm tooling must not become the default DX-WWW foundation",
};

const FORBIDDEN_CARGO_DEPENDENCIES = {
  "next-core": "Next.js runtime crates must stay outside the DX-WWW core",
  "next-napi-bindings": "Node/NAPI bindings must stay outside the DX-WWW foundation",
  "turbopack-nodejs": "Node-backed Turbopack runtime crates must stay quarantined",
  "turbo-tasks": "turbo-tasks requires an adapter before Cargo workspace wiring",
  "turbopack-core": "Turbopack crates require an adapter before Cargo workspace wiring",
  "turbopack-css": "Turbopack CSS must not replace dx-style as the public CSS core",
  "turbopack-ecmascript": "Turbopack ECMAScript must not replace DX-owned TSX analysis",
  "turbopack-resolve": "Turbopack resolver rules must not override Forge/source rules",
};

function manifestPath(repoRoot, relativePath) {
  return path.join(repoRoot, ...relativePath.split("/"));
}

function packageDependencySections(packageJson) {
  return [
    "dependencies",
    "devDependencies",
    "peerDependencies",
    "optionalDependencies",
  ].flatMap((scope) => {
    const entries = packageJson[scope] && typeof packageJson[scope] === "object"
      ? Object.keys(packageJson[scope])
      : [];
    return entries.map((dependency) => ({ scope, dependency }));
  });
}

function collectPackageDependencyFindings(relativePath, text) {
  const parsed = JSON.parse(text);
  return packageDependencySections(parsed)
    .filter(({ dependency }) => {
      return (
        Object.prototype.hasOwnProperty.call(FORBIDDEN_PACKAGE_DEPENDENCIES, dependency) ||
        dependency.startsWith("@next/")
      );
    })
    .map(({ scope, dependency }) => ({
      manifest: relativePath,
      scope,
      dependency,
      reason: FORBIDDEN_PACKAGE_DEPENDENCIES[dependency] ||
        "Next.js internals must not become public DX-WWW dependencies",
    }));
}

function collectCargoDependencyFindings(relativePath, text) {
  return Object.entries(FORBIDDEN_CARGO_DEPENDENCIES)
    .filter(([dependency]) => {
      const escaped = dependency.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
      return new RegExp(`(^|\\n)\\s*${escaped}\\s*=`, "m").test(text);
    })
    .map(([dependency, reason]) => ({
      manifest: relativePath,
      scope: "cargo.dependencies",
      dependency,
      reason,
    }));
}

function collectPublicDependencyClaims(repoRoot) {
  const checkedManifests = [];
  const forbiddenDependencies = [];

  for (const relativePath of PUBLIC_MANIFEST_PATHS) {
    const absolutePath = manifestPath(repoRoot, relativePath);
    if (!fs.existsSync(absolutePath)) {
      continue;
    }

    const text = readText(absolutePath);
    checkedManifests.push(relativePath);
    if (relativePath.endsWith("package.json")) {
      forbiddenDependencies.push(...collectPackageDependencyFindings(relativePath, text));
    } else {
      forbiddenDependencies.push(...collectCargoDependencyFindings(relativePath, text));
    }
  }

  return {
    checkedManifests,
    forbiddenDependencies,
  };
}

module.exports = {
  collectPublicDependencyClaims,
};
