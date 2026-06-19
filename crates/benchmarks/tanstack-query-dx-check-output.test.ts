const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath = "core/src/ecosystem/project_check/data_fetching_cache_dx_check.rs";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("Data Fetching & Cache has a focused Rust dx-check visibility emitter", () => {
  const upstreamQueryClient = fs.readFileSync(
    path.resolve(
      root,
      "..",
      "..",
      "WWW/inspirations/tanstack-query/packages/query-core/src/queryClient.ts",
    ),
    "utf8",
  );
  const helperFullPath = path.join(root, helperPath);

  assert.match(upstreamQueryClient, /setQueryDefaults/);
  assert.match(upstreamQueryClient, /getQueryDefaults/);
  assert.match(upstreamQueryClient, /invalidateQueries/);
  assert.match(upstreamQueryClient, /ensureQueryData/);
  assert.match(upstreamQueryClient, /prefetchQuery/);
  assert.match(upstreamQueryClient, /cancelQueries/);
  assert.ok(fs.existsSync(helperFullPath), "missing Data Fetching & Cache dx-check module");

  const helper = fs.readFileSync(helperFullPath, "utf8");
  const projectCheck = read("core/src/ecosystem/project_check.rs");
  const packageDoc = read("docs/packages/tanstack-query.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.match(helper, /DATA_FETCHING_CACHE_PACKAGE_ID: &str = "tanstack\/query"/);
  assert.match(helper, /DATA_FETCHING_CACHE_OFFICIAL_NAME: &str = "Data Fetching & Cache"/);
  assert.match(helper, /use super::file_hashes::count_sha256_file_hash_mismatches;/);
  assert.doesNotMatch(helper, /fn count_hash_mismatches\(/);
  assert.match(
    helper,
    /DATA_FETCHING_CACHE_PACKAGE_STATUS: &str = "\.dx\/forge\/package-status\.json"/,
  );
  assert.match(
    helper,
    /DATA_FETCHING_CACHE_DASHBOARD_RECEIPT: &str =\s*"\.dx\/forge\/receipts\/2026-05-22-tanstack-query-dashboard-data\.json"/,
  );

  for (const metric of [
    "data_fetching_cache_package_present",
    "data_fetching_cache_receipt_present",
    "data_fetching_cache_receipt_stale",
    "data_fetching_cache_missing_receipt",
    "data_fetching_cache_blocked_surface",
    "data_fetching_cache_unsupported_surface",
    "data_fetching_cache_hash_manifest_present",
    "data_fetching_cache_hash_mismatch",
  ]) {
    assert.match(helper, new RegExp(metric));
  }

  for (const finding of [
    "data-fetching-cache-missing-package-status",
    "data-fetching-cache-missing-receipt",
    "data-fetching-cache-stale-receipt",
    "data-fetching-cache-blocked-surface",
    "data-fetching-cache-unsupported-surface",
    "data-fetching-cache-hash-mismatch",
  ]) {
    assert.match(helper, new RegExp(finding));
  }

  assert.match(
    helper,
    /fn data_fetching_cache_hash_mismatch_flips_when_selected_file_changes\(/,
  );
  assert.match(
    helper,
    /metric_value\(&metrics, "data_fetching_cache_hash_mismatch"\)/,
  );
  assert.match(helper, /data-fetching-cache-hash-mismatch/);

  assert.match(projectCheck, /mod data_fetching_cache_dx_check;/);
  assert.match(projectCheck, /use data_fetching_cache_dx_check::forge_data_fetching_cache_package_metrics;/);
  assert.match(projectCheck, /forge_data_fetching_cache_package_metrics\(root, &manifest\)/);
  assert.match(projectCheck, /metrics\.extend\(data_fetching_cache_metrics\);/);
  assert.match(projectCheck, /findings\.extend\(data_fetching_cache_findings\);/);

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /Rust dx-check output/);
    assert.match(source, /data_fetching_cache_package_present/);
    assert.match(source, /data_fetching_cache_hash_mismatch/);
    assert.match(source, /hash_algorithm: sha256/);
    assert.match(source, /data-fetching-cache-missing-receipt/);
    assert.match(source, /data_fetching_cache_hash_mismatch_flips_when_selected_file_changes/);
    assert.match(source, /without claiming live QueryClient runtime proof/);
  }
});
