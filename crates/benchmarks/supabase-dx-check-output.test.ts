const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath =
  "core/src/ecosystem/project_check/backend_platform_client_dx_check.rs";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("Backend Platform Client has a focused Rust dx-check visibility emitter", () => {
  const upstreamAccountForm = fs.readFileSync(
    path.resolve(
      root,
      "..",
      "..",
      "WWW/inspirations/supabase/examples/user-management/nextjs-user-management/app/account/account-form.tsx",
    ),
    "utf8",
  );
  const upstreamBrowserClient = fs.readFileSync(
    path.resolve(
      root,
      "..",
      "..",
      "WWW/inspirations/supabase/examples/user-management/nextjs-user-management/lib/supabase/client.ts",
    ),
    "utf8",
  );
  const upstreamServerClient = fs.readFileSync(
    path.resolve(
      root,
      "..",
      "..",
      "WWW/inspirations/supabase/examples/user-management/nextjs-user-management/lib/supabase/server.ts",
    ),
    "utf8",
  );
  const helperFullPath = path.join(root, helperPath);

  assert.match(upstreamBrowserClient, /createBrowserClient/);
  assert.match(upstreamServerClient, /createServerClient/);
  assert.match(upstreamAccountForm, /\.from\('profiles'\)/);
  assert.match(upstreamAccountForm, /\.select\(`full_name, username, website, avatar_url`\)/);
  assert.match(upstreamAccountForm, /\.upsert\(\{/);
  assert.ok(
    fs.existsSync(helperFullPath),
    "missing Backend Platform Client dx-check module",
  );

  const helper = fs.readFileSync(helperFullPath, "utf8");
  const projectCheck = read("core/src/ecosystem/project_check.rs");
  const packageDoc = read("docs/packages/supabase-client.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.match(helper, /BACKEND_PLATFORM_CLIENT_PACKAGE_ID: &str = "supabase\/client"/);
  assert.match(
    helper,
    /BACKEND_PLATFORM_CLIENT_OFFICIAL_NAME: &str = "Backend Platform Client"/,
  );
  assert.match(
    helper,
    /BACKEND_PLATFORM_CLIENT_PACKAGE_STATUS: &str = "\.dx\/forge\/package-status\.json"/,
  );
  assert.match(
    helper,
    /BACKEND_PLATFORM_CLIENT_DASHBOARD_RECEIPT: &str =\s*"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-supabase-client-dashboard-workflow\.json"/,
  );

  for (const metric of [
    "backend_platform_client_package_present",
    "backend_platform_client_receipt_present",
    "backend_platform_client_receipt_stale",
    "backend_platform_client_missing_receipt",
    "backend_platform_client_blocked_surface",
    "backend_platform_client_unsupported_surface",
    "backend_platform_client_hash_manifest_present",
    "backend_platform_client_hash_mismatch",
    "backend_platform_client_receipt_hash_refresh_current",
    "backend_platform_client_receipt_hash_refresh_stale",
    "backend_platform_client_receipt_hash_refresh_missing",
    "backend_platform_client_dx_style_compatibility_present",
    "backend_platform_client_dx_style_compatibility_missing",
  ]) {
    assert.match(helper, new RegExp(metric));
  }

  assert.match(
    helper,
    /use super::file_hashes::count_sha256_file_hash_mismatches;/,
  );
  assert.match(
    helper,
    /hash_mismatches \+= count_sha256_file_hash_mismatches\(root, surface\);/,
  );
  assert.doesNotMatch(
    helper,
    /fn count_hash_mismatches/,
    "Backend Platform Client should use the shared byte-derived SHA-256 comparator",
  );
  assert.match(
    helper,
    /fn backend_platform_client_hash_mismatch_metric_and_finding_are_byte_derived\(\)/,
  );
  assert.match(helper, /tempfile::tempdir\(\)\.expect\("tempdir"\)/);
  assert.match(helper, /metric_value\(&metrics, "backend_platform_client_hash_manifest_present"\)/);
  assert.match(helper, /metric_value\(&metrics, "backend_platform_client_receipt_stale"\)/);
  assert.match(helper, /metric_value\(&metrics, "backend_platform_client_hash_mismatch"\)/);
  assert.match(
    helper,
    /metric_value\(\s*&metrics,\s*"backend_platform_client_dx_style_compatibility_present"\s*\)/,
  );
  assert.match(
    helper,
    /metric_value\(\s*&metrics,\s*"backend_platform_client_dx_style_compatibility_missing"\s*\)/,
  );
  assert.match(helper, /let \(refresh_current, refresh_stale, refresh_missing\) = receipt_hash_refresh_counts\(visibility\);/);
  assert.match(helper, /backend_platform_client_hash_refresh_stale_helper_keeps_source_hash_clean/);
  assert.match(
    helper,
    /metric_value\(\s*&stale_metrics,\s*"backend_platform_client_receipt_hash_refresh_stale"\s*\)/,
  );
  assert.match(
    helper,
    /metric_value\(\s*&stale_metrics,\s*"backend_platform_client_hash_mismatch"\s*\),\s*Some\(0\)/,
  );
  assert.match(helper, /finding\.code == "backend-platform-client-hash-mismatch"/);
  assert.match(
    helper,
    /fn backend_platform_client_dx_style_missing_metric_and_finding_flip\(\)/,
  );
  assert.match(helper, /dx_style_compatibility_is_present\(visibility\)/);
  assert.match(helper, /dx\.forge\.package\.dx_style_compatibility/);
  assert.match(
    helper,
    /finding\.code == "backend-platform-client-missing-dx-style-compatibility"/,
  );

  for (const finding of [
    "backend-platform-client-missing-package-status",
    "backend-platform-client-missing-receipt",
    "backend-platform-client-stale-receipt",
    "backend-platform-client-blocked-surface",
    "backend-platform-client-unsupported-surface",
    "backend-platform-client-hash-mismatch",
    "backend-platform-client-missing-dx-style-compatibility",
  ]) {
    assert.match(helper, new RegExp(finding));
  }

  assert.match(projectCheck, /mod backend_platform_client_dx_check;/);
  assert.match(
    projectCheck,
    /use backend_platform_client_dx_check::forge_backend_platform_client_package_metrics;/,
  );
  assert.match(
    projectCheck,
    /forge_backend_platform_client_package_metrics\(root, &manifest\)/,
  );
  assert.match(projectCheck, /metrics\.extend\(backend_platform_client_metrics\);/);
  assert.match(projectCheck, /findings\.extend\(backend_platform_client_findings\);/);

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /Rust dx-check output/);
    assert.match(source, /backend_platform_client_package_present/);
    assert.match(source, /backend_platform_client_dx_style_compatibility_present/);
    assert.match(source, /backend_platform_client_receipt_hash_refresh_current/);
    assert.match(source, /backend_platform_client_hash_refresh_stale_helper_keeps_source_hash_clean/);
    assert.match(source, /backend-platform-client-missing-dx-style-compatibility/);
    assert.match(source, /backend_platform_client_dx_style_missing_metric_and_finding_flip/);
    assert.match(source, /backend-platform-client-missing-receipt/);
    assert.match(source, /byte-derived SHA-256|SHA-256 byte comparison/);
    assert.match(source, /without claiming hosted Supabase runtime proof/);
  }
});
