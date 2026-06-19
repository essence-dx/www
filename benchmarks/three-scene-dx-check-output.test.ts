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

test("3D Scene System dx-check output is wired into the Rust forge section", () => {
  const projectCheck = read("core/src/ecosystem/project_check.rs");
  const threeSceneDxCheckPath = path.join(
    root,
    "core",
    "src",
    "ecosystem",
    "project_check",
    "three_scene_system_dx_check.rs",
  );

  assert.ok(
    fs.existsSync(threeSceneDxCheckPath),
    "missing 3D Scene System dx-check module",
  );

  const threeSceneDxCheck = fs.readFileSync(threeSceneDxCheckPath, "utf8");

  for (const marker of [
    "mod three_scene_system_dx_check;",
    "use three_scene_system_dx_check::forge_three_scene_system_package_metrics;",
    "forge_three_scene_system_package_metrics(root, &manifest)",
    "metrics.extend(three_scene_system_metrics);",
    "findings.extend(three_scene_system_findings);",
  ]) {
    assert.match(projectCheck, escaped(marker), `missing project_check marker ${marker}`);
  }

  for (const marker of [
    'THREE_SCENE_SYSTEM_PACKAGE_ID: &str = "3d/launch-scene"',
    'THREE_SCENE_SYSTEM_OFFICIAL_NAME: &str = "3D Scene System"',
    'THREE_SCENE_SYSTEM_PACKAGE_STATUS: &str = ".dx/forge/package-status.json"',
    "THREE_SCENE_SYSTEM_DASHBOARD_RECEIPT",
    '".dx/forge/receipts/3d-launch-scene-dashboard-workflow.json"',
    "pub(super) fn forge_three_scene_system_package_metrics",
    "json_array_entries(&package_status, &[\"package_lane_visibility\"])",
    "package_receipt_exists(root, package_receipt_path)",
    "hash_manifest_present = 1;",
    "use super::file_hashes::count_sha256_file_hash_mismatches;",
    "hash_mismatches += count_sha256_file_hash_mismatches(root, surface);",
    '"three_scene_system_receipt_present"',
    '"three_scene_system_receipt_stale"',
    '"three_scene_system_missing_receipt"',
    '"three_scene_system_blocked_surface"',
    '"three_scene_system_unsupported_surface"',
    '"three_scene_system_hash_manifest_present"',
    '"three_scene_system_hash_mismatch"',
    '"three_scene_system_receipt_hash_refresh_current"',
    '"three_scene_system_receipt_hash_refresh_stale"',
    '"three_scene_system_receipt_hash_refresh_missing"',
    '"three_scene_system_dx_style_compatibility_present"',
    '"three_scene_system_dx_style_compatibility_missing"',
    '"three-scene-system-missing-package-status"',
    '"three-scene-system-stale-receipt"',
    '"three-scene-system-missing-receipt"',
    '"three-scene-system-blocked-surface"',
    '"three-scene-system-unsupported-surface"',
    '"three-scene-system-hash-mismatch"',
    '"three-scene-system-missing-dx-style-compatibility"',
    "dx_style_compatibility_is_present(visibility)",
    "fn dx_style_compatibility_is_present",
    "fn receipt_hash_refresh_counts",
    "SOURCE-ONLY",
  ]) {
    assert.match(threeSceneDxCheck, escaped(marker), `missing module marker ${marker}`);
  }
  assert.match(
    threeSceneDxCheck,
    /let\s*\(\s*refresh_current,\s*refresh_stale,\s*refresh_missing\s*\)\s*=\s*receipt_hash_refresh_counts\(visibility\);/,
    "missing rustfmt-safe receipt_hash_refresh_counts assignment",
  );
});

test("3D Scene System owns a package-level hash mismatch fixture", () => {
  const threeSceneDxCheck = read(
    "core/src/ecosystem/project_check/three_scene_system_dx_check.rs",
  );

  for (const marker of [
    "three_scene_system_hash_mismatch_metric_and_finding_are_byte_derived",
    "write_three_scene_system_receipt(dir.path());",
    "write_three_scene_system_package_status(dir.path(), &expected_hash);",
    'metric_value(&metrics, "three_scene_system_hash_mismatch")',
    'metric_value(&stale_metrics, "three_scene_system_receipt_stale")',
    'finding.code == "three-scene-system-hash-mismatch"',
    "three_scene_system_dx_style_compatibility_missing_is_reported",
    "write_three_scene_system_package_status_without_dx_style",
    'finding.code == "three-scene-system-missing-dx-style-compatibility"',
    "three_scene_system_hash_refresh_stale_helper_keeps_source_hash_clean",
    "fn sha256_file(path: &Path) -> String",
  ]) {
    assert.match(threeSceneDxCheck, escaped(marker), `missing fixture marker ${marker}`);
  }
  for (const [name, metric] of [
    ["current helper metric", "three_scene_system_receipt_hash_refresh_current"],
    ["stale helper metric", "three_scene_system_receipt_hash_refresh_stale"],
    ["clean source hash metric", "three_scene_system_hash_mismatch"],
  ]) {
    assert.match(
      threeSceneDxCheck,
      new RegExp(
        `metric_value\\(\\s*&(?:current_metrics|stale_metrics),\\s*"${metric}"\\s*\\)`,
        "m",
      ),
      `missing fixture ${name}`,
    );
  }
});

test("3D Scene System docs record Rust dx-check consumption", () => {
  const packageDoc = read("docs/packages/3d-scene-system.md");

  for (const marker of [
    "Rust dx-check output",
    "`core/src/ecosystem/project_check/three_scene_system_dx_check.rs`",
    "`three_scene_system_*`",
    "`three-scene-system-stale-receipt`",
    "`three-scene-system-missing-receipt`",
    "`three-scene-system-hash-mismatch`",
    "`three_scene_system_dx_style_compatibility_present`",
    "`three_scene_system_dx_style_compatibility_missing`",
    "`three_scene_system_receipt_hash_refresh_current`",
    "`three_scene_system_receipt_hash_refresh_stale`",
    "`three_scene_system_receipt_hash_refresh_missing`",
    "`three-scene-system-missing-dx-style-compatibility`",
    "`three_scene_system_hash_refresh_stale_helper_keeps_source_hash_clean`",
    "byte-derived SHA-256",
    "`dx.forge.package.dx_style_compatibility`",
    "without claiming browser, WebGL, or screenshot proof",
  ]) {
    assert.match(packageDoc, escaped(marker), `missing package doc marker ${marker}`);
  }
});
