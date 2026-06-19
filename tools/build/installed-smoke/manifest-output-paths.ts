const fs = require("node:fs");
const path = require("node:path");

function declaredOutput(root, outputPath) {
  const output = typeof outputPath === "string" ? outputPath : null;
  const safety = manifestPathSafety(output);
  const normalizedPath = safety.normalizedPath;
  const absoluteCandidate = normalizedPath ? path.resolve(root, normalizedPath) : null;
  const absolutePath =
    absoluteCandidate && pathIsInside(path.resolve(root), absoluteCandidate)
      ? absoluteCandidate
      : null;

  return {
    path: output,
    normalizedPath,
    pathSafe: output ? safety.safe : null,
    pathUnsafeReason: output ? safety.unsafeReason : null,
    absolutePath,
    present: absolutePath ? fs.existsSync(absolutePath) : false,
  };
}

function sourceOwnedOutputPath(sourcePath) {
  if (!sourcePath) return null;
  const clean = normalizeManifestPath(sourcePath);
  if (!clean) return null;
  return path.posix.join(".dx/build", clean);
}

function isPublicAssetSourcePath(sourcePath) {
  const normalized = normalizeManifestPath(sourcePath);
  return Boolean(normalized && normalized.startsWith("public/") && normalized.length > "public/".length);
}

function isPublicAssetOutputPath(outputPath) {
  const normalized = normalizeManifestPath(outputPath);
  const publicOutputRoot = ".dx/build/public/";
  return Boolean(
    normalized &&
      normalized.startsWith(publicOutputRoot) &&
      normalized.length > publicOutputRoot.length,
  );
}

function publicAssetOutputPathMatchesSourceOwnedAssetPath(sourcePath, outputPath, hash) {
  const source = normalizeManifestPath(sourcePath);
  const output = normalizeManifestPath(outputPath);
  if (
    !source ||
    !output ||
    !nonEmptyString(hash) ||
    !isPublicAssetSourcePath(source) ||
    !isPublicAssetOutputPath(output)
  ) {
    return false;
  }

  const expectedDirectory = path.posix.join(".dx/build", path.posix.dirname(source));
  const sourceExtension = path.posix.extname(source);
  const sourceStem = path.posix.basename(source, sourceExtension);
  const expectedFileName = `${sourceStem}-${hash}${sourceExtension}`;

  return (
    path.posix.dirname(output) === expectedDirectory &&
    path.posix.basename(output) === expectedFileName
  );
}

function manifestPathBaseName(value) {
  const normalized = normalizeManifestPath(value);
  return normalized ? path.posix.basename(normalized) : "";
}

function isSafeManifestPath(value) {
  return manifestPathSafety(value).safe;
}

function normalizeManifestPath(value) {
  return manifestPathSafety(value).normalizedPath;
}

function manifestPathSafety(value) {
  if (typeof value !== "string" || value.length === 0) {
    return unsafeManifestPath("not-a-nonempty-relative-path");
  }

  const slashed = value.replaceAll("\\", "/");
  if (path.win32.isAbsolute(slashed) || path.posix.isAbsolute(slashed)) {
    return unsafeManifestPath("absolute-path");
  }

  const segments = slashed.split("/");
  const unsafeSegmentReason = unsafeManifestPathSegmentReason(segments);
  if (unsafeSegmentReason) {
    return unsafeManifestPath(unsafeSegmentReason);
  }

  const normalized = path.posix.normalize(slashed);
  if (normalized === ".") {
    return unsafeManifestPath("empty-normalized-path");
  }

  return {
    normalizedPath: normalized,
    safe: true,
    unsafeReason: null,
  };
}

function unsafeManifestPath(reason) {
  return {
    normalizedPath: null,
    safe: false,
    unsafeReason: reason,
  };
}

function unsafeManifestPathSegmentReason(segments) {
  for (const segment of segments) {
    if (segment === "..") {
      return "parent-directory-segment";
    }
    if (segment.toLowerCase() === "node_modules") {
      return "node-modules-segment";
    }
  }
  return null;
}

function pathIsInside(root, target) {
  const relative = path.relative(root, target);
  return relative === "" || (!relative.startsWith("..") && !path.isAbsolute(relative));
}

function nonEmptyString(value) {
  return typeof value === "string" && value.length > 0;
}

module.exports = {
  declaredOutput,
  isPublicAssetOutputPath,
  isPublicAssetSourcePath,
  isSafeManifestPath,
  manifestPathSafety,
  manifestPathBaseName,
  normalizeManifestPath,
  pathIsInside,
  publicAssetOutputPathMatchesSourceOwnedAssetPath,
  sourceOwnedOutputPath,
};
