import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const dxRoot = path.resolve(repoRoot, "..");

function readForge(relativePath) {
  return fs.readFileSync(path.join(dxRoot, "forge", relativePath), "utf8");
}

test("flow-forge emits stable schema names with numeric format fields", () => {
  const packages = readForge("src/packages.rs");
  const checkout = readForge("src/cli/checkout.rs");
  const checkoutArchive = readForge("src/cli/checkout_archive.rs");
  const readme = readForge("README.md");
  const status = readForge("docs/FORGE_STATUS.md");

  assert.match(packages, /pub const PACKAGE_CONTRACT_FORMAT: u16 = 1;/);
  assert.match(packages, /pub const PACKAGE_MANIFEST_SCHEMA: &str = "forge\.package_manifest";/);
  assert.match(packages, /pub const PACKAGE_LOCK_SCHEMA: &str = "forge\.package_lock";/);
  assert.match(packages, /pub const PACKAGE_STATUS_SCHEMA: &str = "forge\.package_status_receipt";/);
  assert.match(packages, /pub const PACKAGE_ADD_RECEIPT_SCHEMA: &str = "forge\.package_add_receipt";/);
  assert.doesNotMatch(packages, /pub const [A-Z_]+_SCHEMA: &str = "forge\.[^"]+\.v1";/);
  assert.match(packages, /const LEGACY_PACKAGE_MANIFEST_SCHEMA: &str = "forge\.package_manifest\.v1";/);
  assert.match(packages, /pub format: u16,/);
  assert.match(packages, /format: PACKAGE_CONTRACT_FORMAT,/);

  assert.match(checkout, /const CHECKOUT_ARCHIVE_SCHEMA: &str = "forge\.checkout_archive_receipt";/);
  assert.match(checkout, /format: CHECKOUT_ARCHIVE_FORMAT,/);
  assert.doesNotMatch(checkout, /forge\.checkout_archive_receipt\.v1/);

  assert.match(
    checkoutArchive,
    /const CHECKOUT_ARCHIVE_RESTORE_SCHEMA: &str = "forge\.checkout_archive_restore_receipt";/,
  );
  assert.match(
    checkoutArchive,
    /const LEGACY_CHECKOUT_ARCHIVE_SCHEMA: &str = "forge\.checkout_archive_receipt\.v1";/,
  );
  assert.match(checkoutArchive, /#\[serde\(default = "checkout_archive_format"\)\]/);
  assert.match(checkoutArchive, /archive_receipt\.format != CHECKOUT_ARCHIVE_FORMAT/);

  assert.doesNotMatch(readme, /forge\.package_add_receipt\.v1/);
  assert.doesNotMatch(status, /forge\.package_add_receipt\.v1/);
  assert.match(`${readme}\n${status}`, /numeric `format: 1`/);
});
