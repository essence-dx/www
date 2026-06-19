const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceRoot = "G:\\WWW\\inspirations\\fumadocs";
const helperPath = "core/src/ecosystem/project_check/documentation_system_dx_check.rs";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(sourceRoot, relativePath), "utf8");
}

test("Documentation System has a focused Rust dx-check visibility emitter", () => {
  const upstreamPackage = JSON.parse(readMirror("packages/core/package.json"));
  const loader = readMirror("packages/core/src/source/loader.ts");
  const breadcrumb = readMirror("packages/core/src/breadcrumb.tsx");
  const llms = readMirror("packages/core/src/source/llms.ts");
  const searchServer = readMirror("packages/core/src/search/orama/create-server.ts");
  const searchClient = readMirror("packages/core/src/search/client.ts");
  const openapi = readMirror("packages/openapi/src/server/index.tsx");
  const helper = read(helperPath);
  const projectCheck = read("core/src/ecosystem/project_check.rs");
  const packageDoc = read("docs/packages/content-fumadocs-next.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(upstreamPackage.name, "fumadocs-core");
  assert.equal(upstreamPackage.version, "16.8.12");
  assert.match(loader, /export function loader/);
  assert.match(breadcrumb, /export function getBreadcrumbItems/);
  assert.match(llms, /export function llms/);
  assert.match(searchServer, /export function createFromSource/);
  assert.match(searchClient, /export function useDocsSearch/);
  assert.match(openapi, /export function createOpenAPI/);

  assert.match(helper, /DOCUMENTATION_SYSTEM_PACKAGE_ID: &str = "content\/fumadocs-next"/);
  assert.match(helper, /DOCUMENTATION_SYSTEM_OFFICIAL_NAME: &str = "Documentation System"/);
  assert.match(
    helper,
    /DOCUMENTATION_SYSTEM_PACKAGE_STATUS: &str = "\.dx\/forge\/package-status\.json"/,
  );
  assert.match(
    helper,
    /DOCUMENTATION_SYSTEM_DASHBOARD_RECEIPT: &str =\s*"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-content-fumadocs-dashboard-workflow\.json"/,
  );

  for (const metric of [
    "documentation_system_package_present",
    "documentation_system_receipt_present",
    "documentation_system_receipt_stale",
    "documentation_system_missing_receipt",
    "documentation_system_blocked_surface",
    "documentation_system_unsupported_surface",
    "documentation_system_hash_manifest_present",
    "documentation_system_hash_mismatch",
    "documentation_system_receipt_hash_refresh_current",
    "documentation_system_receipt_hash_refresh_stale",
    "documentation_system_receipt_hash_refresh_missing",
    "documentation_system_dx_style_compatibility_present",
    "documentation_system_dx_style_compatibility_missing",
  ]) {
    assert.match(helper, new RegExp(metric));
  }

  for (const finding of [
    "documentation-system-missing-package-status",
    "documentation-system-missing-receipt",
    "documentation-system-stale-receipt",
    "documentation-system-blocked-surface",
    "documentation-system-unsupported-surface",
    "documentation-system-hash-mismatch",
    "documentation-system-missing-dx-style-compatibility",
  ]) {
    assert.match(helper, new RegExp(finding));
  }

  assert.match(
    helper,
    /use super::file_hashes::\{\s*count_sha256_file_hash_mismatches,\s*count_sha256_path_hash_mismatches,?\s*\};/s,
  );
  assert.match(
    helper,
    /fn documentation_system_hash_mismatch_metric_and_finding_are_byte_derived/,
  );
  assert.match(
    helper,
    /forge_documentation_system_package_metrics\(dir\.path\(\), &manifest\)/,
  );
  assert.match(helper, /examples\/template\/content\/docs\/index\.mdx/);
  assert.match(helper, /fn receipt_hash_refresh_counts/);
  assert.match(helper, /receipt_hash_refresh_stale > 0/);
  assert.match(
    helper,
    /json_text\(refresh, &\["schema"\]\) != Some\("dx\.forge\.package\.receipt_hash_refresh"\)/,
  );
  assert.doesNotMatch(helper, /use sha2::/);
  assert.doesNotMatch(helper, /fn sha256_project_file/);
  assert.doesNotMatch(helper, /fn normalize_sha256_hash/);

  assert.match(projectCheck, /mod documentation_system_dx_check;/);
  assert.match(
    projectCheck,
    /use documentation_system_dx_check::forge_documentation_system_package_metrics;/,
  );
  assert.match(projectCheck, /forge_documentation_system_package_metrics\(root, &manifest\)/);
  assert.match(projectCheck, /metrics\.extend\(documentation_system_metrics\);/);
  assert.match(projectCheck, /findings\.extend\(documentation_system_findings\);/);

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /Rust dx-check output/);
    assert.match(source, /documentation_system_package_present/);
    assert.match(source, /documentation_system_receipt_hash_refresh_stale/);
    assert.match(source, /documentation-system-missing-receipt/);
    assert.match(source, /documentation_system_hash_mismatch_metric_and_finding_are_byte_derived/);
    assert.match(source, /without claiming live Fumadocs renderer runtime proof/);
  }
});
