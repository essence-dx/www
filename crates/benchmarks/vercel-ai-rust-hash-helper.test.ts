const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("AI SDK dx-check uses the shared materialized-file hash helper", () => {
  const aiDxCheck = read("core/src/ecosystem/project_check/ai_sdk_dx_check.rs");
  const aiDxCheckProduction = aiDxCheck.split("#[cfg(test)]")[0];
  const fileHashes = read("core/src/ecosystem/project_check/file_hashes.rs");
  const packageDoc = read("docs/packages/ai-vercel-ai.md");

  assert.match(
    fileHashes,
    /pub\(super\) fn has_sha256_file_hashes\(surface: &serde_json::Value\) -> bool/,
  );
  assert.match(
    fileHashes,
    /pub\(super\) fn count_materialized_sha256_file_hash_mismatches/,
  );
  assert.match(fileHashes, /upstream:/);
  assert.match(fileHashes, /examples\/template\//);

  assert.match(
    aiDxCheck,
    /use super::file_hashes::\{[^}]*count_materialized_sha256_file_hash_mismatches[^}]*has_sha256_file_hashes[^}]*\};/,
  );
  assert.match(
    aiDxCheck,
    /hash_mismatches \+= count_materialized_sha256_file_hash_mismatches\(root, surface\);/,
  );
  assert.doesNotMatch(aiDxCheckProduction, /sha2::\{Digest, Sha256\}/);
  assert.doesNotMatch(aiDxCheckProduction, /std::collections::BTreeSet/);
  assert.doesNotMatch(aiDxCheckProduction, /fn count_hash_mismatches\(/);
  assert.doesNotMatch(aiDxCheckProduction, /fn materialized_surface_files\(/);
  assert.doesNotMatch(aiDxCheckProduction, /fn is_provenance_hash\(/);
  assert.doesNotMatch(aiDxCheckProduction, /fn sha256_file\(/);

  assert.match(
    aiDxCheck,
    /fn ai_sdk_hash_mismatch_metric_and_finding_are_byte_derived\(\)/,
  );
  assert.match(aiDxCheck, /forge_ai_sdk_package_metrics\(dir\.path\(\), &manifest\)/);
  assert.match(aiDxCheck, /"ai-sdk-hash-mismatch"/);
  assert.match(aiDxCheck, /"components\/template-app\/ai-chat-status\.tsx"/);

  assert.match(packageDoc, /shared `file_hashes` helper/);
  assert.match(
    packageDoc,
    /ai_sdk_hash_mismatch_metric_and_finding_are_byte_derived/,
  );
});
