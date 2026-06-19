const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceRoot = "G:\\WWW\\inspirations\\next-intl";
const helperPath = "core/src/ecosystem/project_check/internationalization_dx_check.rs";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(sourceRoot, relativePath), "utf8");
}

test("Internationalization has a focused Rust dx-check visibility emitter", () => {
  const upstreamPackage = JSON.parse(readMirror("packages/next-intl/package.json"));
  const provider = readMirror("packages/next-intl/src/shared/NextIntlClientProvider.tsx");
  const hooks = readMirror("packages/use-intl/src/react/index.tsx");
  const middleware = readMirror("packages/next-intl/src/middleware/middleware.tsx");
  const helper = read(helperPath);
  const projectCheck = read("core/src/ecosystem/project_check.rs");
  const packageDoc = read("docs/packages/next-intl.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");

  assert.equal(upstreamPackage.name, "next-intl");
  assert.equal(upstreamPackage.version, "4.12.0");
  assert.match(provider, /NextIntlClientProvider/);
  assert.match(hooks, /useTranslations/);
  assert.match(hooks, /useLocale/);
  assert.match(hooks, /useFormatter/);
  assert.match(middleware, /export default function createMiddleware/);

  assert.match(helper, /INTERNATIONALIZATION_PACKAGE_ID: &str = "i18n\/next-intl"/);
  assert.match(helper, /INTERNATIONALIZATION_OFFICIAL_NAME: &str = "Internationalization"/);
  assert.match(
    helper,
    /INTERNATIONALIZATION_PACKAGE_STATUS: &str = "\.dx\/forge\/package-status\.json"/,
  );
  assert.match(
    helper,
    /INTERNATIONALIZATION_DASHBOARD_RECEIPT: &str =\s*"\.dx\/forge\/receipts\/2026-05-22-i18n-next-intl-dashboard-locale\.json"/,
  );

  for (const metric of [
    "internationalization_package_present",
    "internationalization_receipt_present",
    "internationalization_receipt_stale",
    "internationalization_missing_receipt",
    "internationalization_blocked_surface",
    "internationalization_unsupported_surface",
    "internationalization_dx_style_compatibility_present",
    "internationalization_dx_style_compatibility_missing",
  ]) {
    assert.match(helper, new RegExp(metric));
  }

  for (const finding of [
    "internationalization-missing-package-status",
    "internationalization-missing-receipt",
    "internationalization-stale-receipt",
    "internationalization-blocked-surface",
    "internationalization-unsupported-surface",
    "internationalization-missing-dx-style-compatibility",
  ]) {
    assert.match(helper, new RegExp(finding));
  }

  assert.match(
    helper,
    /use super::file_hashes::count_sha256_file_hash_mismatches;/,
  );
  assert.match(helper, /count_sha256_file_hash_mismatches\(root, surface\)/);
  assert.doesNotMatch(helper, /use sha2::\{Digest, Sha256\};/);
  assert.doesNotMatch(helper, /fn count_hash_mismatches/);
  assert.doesNotMatch(helper, /fn sha256_project_file/);
  assert.doesNotMatch(helper, /fn sha256_hex/);
  assert.match(
    helper,
    /internationalization_hash_mismatch_metric_and_finding_are_byte_derived/,
  );
  assert.match(helper, /dx_style_compatibility_is_present/);

  assert.match(projectCheck, /mod internationalization_dx_check;/);
  assert.match(
    projectCheck,
    /use internationalization_dx_check::forge_internationalization_package_metrics;/,
  );
  assert.match(projectCheck, /forge_internationalization_package_metrics\(root, &manifest\)/);
  assert.match(projectCheck, /metrics\.extend\(internationalization_metrics\);/);
  assert.match(projectCheck, /findings\.extend\(internationalization_findings\);/);

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /Rust dx-check output/);
    assert.match(source, /internationalization_package_present/);
    assert.match(source, /internationalization-missing-receipt/);
    assert.match(source, /project_check\/file_hashes\.rs/);
    assert.match(source, /internationalization_dx_style_compatibility_present/);
    assert.match(source, /internationalization-missing-dx-style-compatibility/);
    assert.match(source, /without claiming live locale routing proof/);
  }
});
