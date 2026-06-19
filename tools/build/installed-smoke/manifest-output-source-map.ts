const fs = require("node:fs");
const path = require("node:path");

function summarizeSourceMapArtifact(absolutePath, present) {
  if (!present || !absolutePath) {
    return {
      jsonValid: false,
      sourceCount: 0,
      sources: [],
      unsafeSourceCount: 0,
      unsafeSources: [],
      error: "missing",
    };
  }
  try {
    const sourceMap = JSON.parse(fs.readFileSync(absolutePath, "utf8"));
    const sourceEntries = sourceMapSourceEntries(sourceMap);
    const sources = sourceEntries.map((entry) => entry.source);
    const unsafeSources = sourceEntries
      .filter((entry) => entry.safe !== true)
      .map((entry) => entry.source);
    return {
      jsonValid: true,
      sourceCount: sources.length,
      sources,
      unsafeSourceCount: unsafeSources.length,
      unsafeSources,
      error: null,
    };
  } catch (error) {
    return {
      jsonValid: false,
      sourceCount: 0,
      sources: [],
      unsafeSourceCount: 0,
      unsafeSources: [],
      error: error && error.message ? error.message : "invalid-json",
    };
  }
}

function sourceMapSourceEntries(sourceMap) {
  if (!sourceMap || typeof sourceMap !== "object") {
    return [];
  }
  const sourceRoot = typeof sourceMap.sourceRoot === "string" ? sourceMap.sourceRoot : "";
  const directSources = Array.isArray(sourceMap.sources)
    ? sourceMap.sources
        .filter((source) => typeof source === "string" && source.length > 0)
        .map((source) => sourceMapSourceEntry(sourceRoot, source))
        .filter((entry) => entry.source.length > 0)
    : [];
  const sectionSources = Array.isArray(sourceMap.sections)
    ? sourceMap.sections.flatMap((section) => sourceMapSourceEntries(section?.map))
    : [];

  return [...directSources, ...sectionSources];
}

function sourceMapSourceEntry(sourceRoot, source) {
  const normalized = normalizeSourceMapSource(sourceRoot, source);
  return {
    source: normalized,
    safe:
      sourceMapPartIsSafe(sourceRoot) &&
      sourceMapPartIsSafe(source) &&
      normalizedSourceMapSourceIsSafe(normalized),
  };
}

function normalizeSourceMapSource(sourceRoot, source) {
  const normalizedSource = normalizeSourceMapPart(source);
  if (!normalizedSource) {
    return "";
  }

  const normalizedRoot = normalizeSourceMapPart(sourceRoot);
  const joined = normalizedRoot
    ? path.posix.join(normalizedRoot, normalizedSource)
    : normalizedSource;
  const normalized = path.posix.normalize(joined);
  return normalized === "." ? "" : normalized.replace(/^\.\//, "");
}

function sourceMapPartIsSafe(value) {
  if (typeof value !== "string" || value.length === 0) {
    return true;
  }

  const slashed = value.replaceAll("\\", "/");
  if (
    /^[a-z][a-z0-9+.-]*:/i.test(slashed) ||
    /^[a-zA-Z]:\//.test(slashed) ||
    slashed.startsWith("//") ||
    path.posix.isAbsolute(slashed)
  ) {
    return false;
  }

  const rawSegments = slashed.split("/");
  return !rawSegments.includes("..") && normalizedSourceMapSourceIsSafe(normalizeSourceMapPart(value));
}

function normalizedSourceMapSourceIsSafe(source) {
  if (typeof source !== "string" || source.length === 0) {
    return true;
  }

  const segments = source.split("/");
  return source !== ".." && !source.startsWith("../") && !segments.includes("node_modules");
}

function normalizeSourceMapPart(value) {
  if (typeof value !== "string" || value.length === 0) {
    return "";
  }

  const withoutScheme = value.replace(/^[a-z][a-z0-9+.-]*:\/+/i, "");
  const normalized = path.posix.normalize(withoutScheme.replaceAll("\\", "/"));
  return normalized === "." ? "" : normalized.replace(/^\.\//, "");
}

module.exports = { summarizeSourceMapArtifact };
