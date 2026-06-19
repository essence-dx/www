const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const helperPath = "core/src/ecosystem/project_check/forms_dx_check.rs";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("Forms has a focused Rust dx-check visibility emitter", () => {
  const upstreamUseForm = fs.readFileSync(
    path.resolve(root, "..", "..", "WWW/inspirations/react-hook-form/src/useForm.ts"),
    "utf8",
  );
  const upstreamFormContext = fs.readFileSync(
    path.resolve(
      root,
      "..",
      "..",
      "WWW/inspirations/react-hook-form/src/useFormContext.tsx",
    ),
    "utf8",
  );
  const upstreamController = fs.readFileSync(
    path.resolve(root, "..", "..", "WWW/inspirations/react-hook-form/src/controller.tsx"),
    "utf8",
  );
  const helper = read(helperPath);
  const projectCheck = read("core/src/ecosystem/project_check.rs");
  const packageDoc = read("docs/packages/forms-react-hook-form.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.match(upstreamUseForm, /export function useForm/);
  assert.match(upstreamFormContext, /export const FormProvider/);
  assert.match(upstreamController, /export \{ Controller \}/);

  assert.match(helper, /FORMS_PACKAGE_ID: &str = "forms\/react-hook-form"/);
  assert.match(helper, /FORMS_OFFICIAL_NAME: &str = "Forms"/);
  assert.match(helper, /FORMS_PACKAGE_STATUS: &str = "\.dx\/forge\/package-status\.json"/);
  assert.match(
    helper,
    /FORMS_DASHBOARD_RECEIPT: &str =\s*"\.dx\/forge\/receipts\/2026-05-22-forms-dashboard-workflow\.json"/,
  );

  for (const metric of [
    "forms_package_present",
    "forms_receipt_present",
    "forms_receipt_stale",
    "forms_missing_receipt",
    "forms_blocked_surface",
    "forms_unsupported_surface",
    "forms_hash_manifest_present",
    "forms_hash_mismatch",
    "forms_receipt_hash_refresh_current",
    "forms_receipt_hash_refresh_stale",
    "forms_receipt_hash_refresh_missing",
  ]) {
    assert.match(helper, new RegExp(metric));
  }

  for (const finding of [
    "forms-missing-package-status",
    "forms-missing-receipt",
    "forms-stale-receipt",
    "forms-blocked-surface",
    "forms-unsupported-surface",
    "forms-hash-mismatch",
  ]) {
    assert.match(helper, new RegExp(finding));
  }

  for (const marker of [
    "Sha256::digest",
    "sha256_project_file(root, relative_path)",
    "normalize_sha256_hash(expected_hash)",
    "count_surface_hash_mismatches(root, surface)",
    "receipt_hash_refresh_counts(visibility)",
    "json_string_array(refresh, \"stale_files\")",
    "json_string_array(refresh, \"stale_mirror_files\")",
    "json_string_array(refresh, \"missing_files\")",
    "json_string_array(refresh, \"missing_mirror_files\")",
    "forms_hash_mismatch_metric_and_finding_are_byte_derived",
    "forms_package_metrics_reports_helper_freshness_from_path_arrays",
  ]) {
    assert.match(helper, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(projectCheck, /mod forms_dx_check;/);
  assert.match(projectCheck, /use forms_dx_check::forge_forms_package_metrics;/);
  assert.match(projectCheck, /forge_forms_package_metrics\(root, &manifest\)/);
  assert.match(projectCheck, /metrics\.extend\(forms_metrics\);/);
  assert.match(projectCheck, /findings\.extend\(forms_findings\);/);

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /Rust dx-check output/);
    assert.match(source, /forms_package_present/);
    assert.match(source, /forms_receipt_hash_refresh_current/);
    assert.match(source, /receipt_hash_refresh/);
    assert.match(source, /forms-missing-receipt/);
    assert.match(source, /without claiming browser submission proof/);
  }
});
