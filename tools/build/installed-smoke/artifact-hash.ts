const crypto = require("node:crypto");
const fs = require("node:fs");

const { blake3Hex } = require("./blake3-16.ts");

function hashMatchesArtifact(hash, algorithm, output) {
  if (!nonEmptyString(hash)) {
    return false;
  }

  if (!nonEmptyString(algorithm)) {
    return null;
  }

  const normalizedAlgorithm = normalizeHashAlgorithm(algorithm);
  if (normalizedAlgorithm === null) {
    return false;
  }

  if (!output.present || !output.absolutePath) {
    return false;
  }

  const bytes = fs.readFileSync(output.absolutePath);
  const digest =
    normalizedAlgorithm === "blake3-16"
      ? blake3Hex(bytes)
      : crypto.createHash(normalizedAlgorithm).update(bytes).digest("hex");
  const expected = hash.toLowerCase();

  return digest === expected || digest.startsWith(expected);
}

function normalizeHashAlgorithm(value) {
  if (typeof value !== "string") {
    return null;
  }

  const normalized = value.trim().toLowerCase();
  if (
    normalized === "sha256" ||
    normalized === "sha-256" ||
    normalized === "sha256-prefix"
  ) {
    return "sha256";
  }
  if (normalized === "blake3" || normalized === "blake3-16") {
    return "blake3-16";
  }

  return null;
}

function nonEmptyString(value) {
  return typeof value === "string" && value.length > 0;
}

module.exports = {
  hashMatchesArtifact,
};
