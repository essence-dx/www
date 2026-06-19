const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceRoot = "G:\\WWW\\inspirations\\next-intl";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(sourceRoot, relativePath), "utf8");
}

test("Internationalization package-lane row exposes dx-style check-panel visibility", () => {
  const upstreamPackage = JSON.parse(readMirror("packages/next-intl/package.json"));
  const provider = readMirror(
    "packages/next-intl/src/shared/NextIntlClientProvider.tsx",
  );
  const hooks = readMirror("packages/use-intl/src/react/index.tsx");
  const reader = read("core/src/ecosystem/dx_check_receipt.rs");
  const packageDoc = read("docs/packages/next-intl.md");
  const dx = read("DX.md");
  const todo = read("TODO.md");
  const changelog = read("CHANGELOG.md");
  const intlRow = reader.slice(
    reader.indexOf("fn internationalization_package_lane_row"),
    reader.indexOf("fn internationalization_missing_receipt_row"),
  );

  assert.equal(upstreamPackage.name, "next-intl");
  assert.equal(upstreamPackage.version, "4.12.0");
  assert.match(provider, /NextIntlClientProvider/);
  assert.match(hooks, /useTranslations/);
  assert.match(hooks, /useLocale/);
  assert.match(hooks, /useFormatter/);

  assert.match(reader, /INTERNATIONALIZATION_PACKAGE_ID: &str = "i18n\/next-intl"/);
  assert.match(reader, /INTERNATIONALIZATION_OFFICIAL_NAME: &str = "Internationalization"/);
  assert.match(reader, /INTERNATIONALIZATION_UPSTREAM_PACKAGE: &str = "next-intl"/);
  assert.match(reader, /INTERNATIONALIZATION_UPSTREAM_VERSION: &str = "4\.12\.0"/);
  assert.match(
    reader,
    /INTERNATIONALIZATION_PACKAGE_STATUS_PATH: &str = "\.dx\/forge\/package-status\.json"/,
  );
  assert.match(
    reader,
    /INTERNATIONALIZATION_PACKAGE_RECEIPT_PATH: &str =\s*"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-i18n-next-intl-dashboard-locale\.json"/,
  );
  assert.match(reader, /rows\.extend\(internationalization_package_lane_row\(root, package_status\)\)/);
  assert.match(reader, /internationalization_package_lane_row\(\s*root: &Path,\s*package_status: Option<&ForgePackageStatusReadModel>,/);
  assert.match(reader, /internationalization_hash_manifest_present/);
  assert.match(reader, /internationalization_hash_mismatch/);
  assert.match(reader, /const INTERNATIONALIZATION_METRICS: \[&str; 13\]/);
  assert.match(reader, /internationalization_receipt_hash_refresh_current/);
  assert.match(reader, /internationalization_receipt_hash_refresh_stale/);
  assert.match(reader, /internationalization_receipt_hash_refresh_missing/);
  assert.match(reader, /internationalization_dx_style_compatibility_present/);
  assert.match(reader, /internationalization_dx_style_compatibility_missing/);
  assert.match(
    intlRow,
    /let receipt_hash_refresh = package_lane_hash_refresh\(package\);/,
  );
  assert.match(
    intlRow,
    /let \(refresh_current, refresh_stale, refresh_missing\) = receipt_hash_refresh_counts\(package\);/,
  );
  assert.match(
    intlRow,
    /let stale_receipt = u64::from\(\s*matches!\(visibility_status, "stale"\)\s*\|\| matches!\(receipt_status, "stale"\)\s*\|\| hash_mismatches > 0\s*\|\| refresh_stale > 0,/,
  );
  assert.match(
    intlRow,
    /receipt_hash_refresh: receipt_hash_refresh\.clone\(\),/,
  );
  assert.match(
    reader,
    /internationalization_next_action\(\s*status,\s*refresh_stale,\s*refresh_missing,\s*dx_style_compatibility_missing,/,
  );
  assert.match(
    reader,
    /fn internationalization_next_action\(\s*status: &str,\s*refresh_stale: u64,\s*refresh_missing: u64,\s*dx_style_compatibility_missing: u64,/,
  );
  assert.match(reader, /dx_style_compatibility_is_present\(package\)/);
  assert.match(
    reader,
    /dx_check_latest_panel_exposes_internationalization_package_lane_style_row/,
  );
  assert.match(reader, /stale_helper_internationalization/);
  assert.match(
    reader,
    /stale_helper_package_status\["package_lane_visibility"\]\[0\]\["receipt_hash_refresh"\]\["stale_file_count"\]/,
  );
  assert.match(reader, /helper_stale_metric_value\("internationalization_receipt_hash_refresh_stale"\)/);
  assert.match(reader, /helper_stale_metric_value\("internationalization_hash_mismatch"\),\s*0/);

  for (const source of [packageDoc, dx, todo, changelog]) {
    assert.match(source, /DX Studio\/check-panel Internationalization row/);
    assert.match(source, /internationalization_hash_manifest_present/);
    assert.match(source, /internationalization_hash_mismatch/);
    assert.match(source, /internationalization_receipt_hash_refresh_current/);
    assert.match(source, /internationalization_receipt_hash_refresh_stale/);
    assert.match(source, /internationalization_receipt_hash_refresh_missing/);
    assert.match(source, /receiptHashRefresh|receipt_hash_refresh/);
    assert.match(source, /stale-helper-only|stale helper only/);
    assert.match(source, /internationalization_dx_style_compatibility_present/);
    assert.match(source, /internationalization_dx_style_compatibility_missing/);
    assert.match(source, /without claiming live locale routing proof/);
  }
});
