const DX_SOURCE_BUILD_HASH_ALGORITHM = "blake3-16";

function summarizeManifestHashAlgorithm(entry, declaredAlgorithm, hash) {
  const declared = stringOrNull(declaredAlgorithm);
  if (declared) {
    return {
      algorithm: declared,
      inferred: false,
    };
  }

  if (entry?.source_owned_contract === true && isBlake3Prefix(hash)) {
    return {
      algorithm: DX_SOURCE_BUILD_HASH_ALGORITHM,
      inferred: true,
    };
  }

  return {
    algorithm: null,
    inferred: false,
  };
}

function isBlake3Prefix(hash) {
  return typeof hash === "string" && /^[0-9a-f]{16}$/i.test(hash);
}

function stringOrNull(value) {
  return typeof value === "string" ? value : null;
}

module.exports = {
  DX_SOURCE_BUILD_HASH_ALGORITHM,
  summarizeManifestHashAlgorithm,
};
