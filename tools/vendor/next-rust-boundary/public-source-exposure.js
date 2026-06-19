const fs = require("node:fs");
const path = require("node:path");
const { readText } = require("./paths.js");

const PUBLIC_SOURCE_PATHS = [
  "dx-www/src/lib.rs",
  "dx-www/src/main.rs",
  "dx-www/src/public_api.ts",
  "dx-www/src/public_api.tsx",
  "dx-www/src/index.ts",
  "dx-www/src/index.tsx",
];

const NEXT_RUNTIME_REASON =
  "Next.js runtime internals must not be re-exported from DX-WWW public source";
const TURBOPACK_ADAPTER_REASON =
  "Turbopack internals require a DX-owned adapter before public source exposure";
const NEXT_PACKAGE_REASON =
  "Next.js runtime modules must not become DX-WWW public API imports";
const REACT_PACKAGE_REASON =
  "React/RSC modules must not become required DX-WWW public API imports";

const FORBIDDEN_RUST_SYMBOLS = {
  next_core: NEXT_RUNTIME_REASON,
  next_napi_bindings: NEXT_RUNTIME_REASON,
  turbopack_nodejs: NEXT_RUNTIME_REASON,
  turbo_tasks: TURBOPACK_ADAPTER_REASON,
  turbopack_core: TURBOPACK_ADAPTER_REASON,
  turbopack_css: TURBOPACK_ADAPTER_REASON,
  turbopack_ecmascript: TURBOPACK_ADAPTER_REASON,
  turbopack_resolve: TURBOPACK_ADAPTER_REASON,
};

const PACKAGE_REASONS = new Map([
  ["next", NEXT_PACKAGE_REASON],
  ["react", REACT_PACKAGE_REASON],
  ["react-dom", REACT_PACKAGE_REASON],
  ["react-server-dom-webpack", REACT_PACKAGE_REASON],
]);

function sourcePath(repoRoot, relativePath) {
  return path.join(repoRoot, ...relativePath.split("/"));
}

function hasRustSymbol(text, symbol) {
  const escaped = symbol.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  return new RegExp(`\\b${escaped}\\b`).test(text);
}

function collectRustExposures(relativePath, text) {
  return Object.entries(FORBIDDEN_RUST_SYMBOLS)
    .filter(([symbol]) => hasRustSymbol(text, symbol))
    .map(([exposure, reason]) => ({
      file: relativePath,
      exposure,
      reason,
    }));
}

function reasonForPackageSpecifier(specifier) {
  if (specifier === "next" || specifier.startsWith("next/")) {
    return NEXT_PACKAGE_REASON;
  }
  if (PACKAGE_REASONS.has(specifier)) {
    return PACKAGE_REASONS.get(specifier);
  }
  if (specifier.startsWith("@vercel/turbopack") || specifier === "turbopack" || specifier.startsWith("turbopack/")) {
    return TURBOPACK_ADAPTER_REASON;
  }
  return null;
}

function collectPackageSpecifiers(text) {
  const specifiers = [];
  const importExportPattern = /\b(?:import|export)\b[^'"]*['"]([^'"]+)['"]/g;
  const requirePattern = /\brequire\(\s*['"]([^'"]+)['"]\s*\)/g;
  const dynamicImportPattern = /\bimport\(\s*['"]([^'"]+)['"]\s*\)/g;

  for (const pattern of [importExportPattern, requirePattern, dynamicImportPattern]) {
    for (const match of text.matchAll(pattern)) {
      specifiers.push(match[1]);
    }
  }

  return [...new Set(specifiers)];
}

function collectTypeScriptExposures(relativePath, text) {
  return collectPackageSpecifiers(text)
    .map((exposure) => ({
      file: relativePath,
      exposure,
      reason: reasonForPackageSpecifier(exposure),
    }))
    .filter((finding) => finding.reason !== null);
}

function collectFileExposures(relativePath, text) {
  if (relativePath.endsWith(".rs")) {
    return collectRustExposures(relativePath, text);
  }
  return collectTypeScriptExposures(relativePath, text);
}

function collectPublicSourceExposure(repoRoot) {
  const checkedFiles = [];
  const forbiddenExposures = [];

  for (const relativePath of PUBLIC_SOURCE_PATHS) {
    const absolutePath = sourcePath(repoRoot, relativePath);
    if (!fs.existsSync(absolutePath)) {
      continue;
    }

    const text = readText(absolutePath);
    checkedFiles.push(relativePath);
    forbiddenExposures.push(...collectFileExposures(relativePath, text));
  }

  return {
    checkedFiles,
    forbiddenExposures,
  };
}

module.exports = {
  collectPublicSourceExposure,
};
