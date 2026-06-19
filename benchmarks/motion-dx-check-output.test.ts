const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function escaped(marker) {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

test("Motion & Animation dx-check output is wired into the Rust forge section", () => {
  const projectCheck = read("core/src/ecosystem/project_check.rs");
  const motionDxCheckPath = path.join(
    root,
    "core",
    "src",
    "ecosystem",
    "project_check",
    "motion_animation_dx_check.rs",
  );

  assert.ok(
    fs.existsSync(motionDxCheckPath),
    "missing Motion & Animation dx-check module",
  );

  const motionDxCheck = fs.readFileSync(motionDxCheckPath, "utf8");

  for (const marker of [
    "mod motion_animation_dx_check;",
    "use motion_animation_dx_check::forge_motion_animation_package_metrics;",
    "forge_motion_animation_package_metrics(root, &manifest)",
    "metrics.extend(motion_animation_metrics);",
    "findings.extend(motion_animation_findings);",
  ]) {
    assert.match(projectCheck, escaped(marker), `missing project_check marker ${marker}`);
  }

  for (const marker of [
    'MOTION_ANIMATION_PACKAGE_ID: &str = "animation/motion"',
    'MOTION_ANIMATION_OFFICIAL_NAME: &str = "Motion & Animation"',
    'MOTION_ANIMATION_PACKAGE_STATUS: &str = ".dx/forge/package-status.json"',
    "MOTION_ANIMATION_DASHBOARD_RECEIPT",
    '"examples/template/.dx/forge/receipts/2026-05-22-animation-motion-dashboard-workflow.json"',
    "pub(super) fn forge_motion_animation_package_metrics",
    "use super::file_hashes::{count_sha256_file_hash_mismatches, has_sha256_file_hashes};",
    "PathBuf",
    "json_array_entries(&package_status, &[\"package_lane_visibility\"])",
    "package_receipt_exists(root, package_receipt_path)",
    "has_sha256_file_hashes(surface)",
    "hash_mismatches += count_sha256_file_hash_mismatches(root, surface);",
    'check_metric("motion_animation_package_present", package_present)',
    'check_metric("motion_animation_receipt_present", receipt_present)',
    'check_metric("motion_animation_receipt_stale", stale_receipt)',
    'check_metric("motion_animation_missing_receipt", missing_receipt)',
    'check_metric("motion_animation_blocked_surface", blocked_surfaces)',
    '"motion_animation_unsupported_surface"',
    '"motion_animation_hash_manifest_present"',
    '"motion_animation_hash_mismatch"',
    '"motion-animation-missing-package-status"',
    '"motion-animation-stale-receipt"',
    '"motion-animation-missing-receipt"',
    '"motion-animation-blocked-surface"',
    '"motion-animation-unsupported-surface"',
    '"motion-animation-hash-mismatch"',
    "motion_animation_shared_hash_helper_matches_known_digest",
    "motion_animation_shared_hash_helper_detects_stale_bytes",
    "motion_animation_package_metrics_report_byte_derived_hash_mismatch",
    "motion_animation_package_metrics_honor_missing_receipt_status",
    'metric_value(&metrics, "motion_animation_hash_mismatch")',
    'finding.code == "motion-animation-hash-mismatch"',
    'Some("missing-receipt") => missing_receipt = 1',
    "SOURCE-ONLY",
    "ADAPTER-BOUNDARY",
  ]) {
    assert.match(motionDxCheck, escaped(marker), `missing module marker ${marker}`);
  }

  for (const marker of [
    "fn count_hash_mismatches",
    "fn sha256_project_file",
    "fn normalize_sha256_hash",
    "use sha2::{Digest, Sha256};",
  ]) {
    assert.doesNotMatch(
      motionDxCheck,
      escaped(marker),
      `Motion & Animation should use shared hash helper, not local marker ${marker}`,
    );
  }
});

test("Motion & Animation docs record Rust dx-check consumption", () => {
  const packageDoc = read("docs/packages/animation-motion.md");

  for (const marker of [
    "Rust dx-check output",
    "`core/src/ecosystem/project_check/motion_animation_dx_check.rs`",
    "`motion_animation_*`",
    "`motion-animation-stale-receipt`",
    "`motion-animation-missing-receipt`",
    "`motion-animation-hash-mismatch`",
    "byte-derived SHA-256",
    "shared `project_check/file_hashes.rs` helper",
    "without claiming live browser animation runtime proof",
  ]) {
    assert.match(packageDoc, escaped(marker), `missing package doc marker ${marker}`);
  }
});
