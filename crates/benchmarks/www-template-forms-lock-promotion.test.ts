const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const templateRoot = path.join(root, "examples", "template");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

test("forms/react-hook-form is promoted into the lock-backed Forge package set", () => {
  const sourceManifest = readJson(
    "examples/template/.dx/forge/source-.dx/build-cache/manifest.json",
  );
  const lock = readJson("examples/template/.dx/forge/package-lock.json");
  const status = readJson("examples/template/.dx/forge/package-status.json");

  const sourcePackage = sourceManifest.packages.find(
    (entry) => entry.package_id === "forms/react-hook-form",
  );
  assert.ok(sourcePackage, "source manifest should expose forms/react-hook-form");
  assert.equal(sourcePackage.upstream_name, "react-hook-form");
  assert.equal(sourcePackage.version, "7.75.0-dx.0");
  assert.equal(sourcePackage.files.length, 7);
  assert.ok(sourcePackage.rollback_receipt, "forms/react-hook-form needs rollback receipt");

  const packageEntry = lock.packages.find((entry) => entry.name === "forms/react-hook-form");
  assert.ok(packageEntry, "package lock should expose forms/react-hook-form");
  assert.equal(packageEntry.source_kind, "local-slice");
  assert.equal(packageEntry.source_locator, "lib/forms/react-hook-form/form.tsx");
  assert.equal(packageEntry.integrity_state, "valid");
  assert.equal(packageEntry.files.length, sourcePackage.files.length);
  assert.equal(packageEntry.integrity_hash, sourcePackage.integrity_hash);
  assert.ok(packageEntry.rollback_receipt_path);
  assert.ok(packageEntry.safety_archive_receipt_path);

  for (const expectedFile of [
    "lib/forms/react-hook-form/form.tsx",
    "lib/forms/react-hook-form/fields.tsx",
    "lib/forms/react-hook-form/dry-run-receipt.ts",
    "lib/forms/react-hook-form/resolver.ts",
    "lib/forms/react-hook-form/metadata.ts",
    "lib/forms/react-hook-form/README.md",
  ]) {
    assert.ok(
      packageEntry.files.some((file) => file.path === expectedFile),
      `missing forms/react-hook-form locked file ${expectedFile}`,
    );
    assert.ok(
      fs.existsSync(path.join(templateRoot, expectedFile)),
      `missing materialized forms/react-hook-form source ${expectedFile}`,
    );
  }

  const packageReceiptPath = packageEntry.receipt_paths.find((candidate) =>
    candidate.includes("receipts/packages/forms-react-hook-form.json"),
  );
  assert.ok(packageReceiptPath, "forms/react-hook-form should reference package-add receipt");

  const packageReceipt = readJson(`examples/template/${packageReceiptPath}`);
  assert.equal(packageReceipt.schema, "forge.package_add_receipt");
  assert.equal(packageReceipt.package.name, "forms/react-hook-form");
  assert.equal(
    packageReceipt.boundary,
    "forge-owned source slice; no node_modules install performed",
  );
  assert.equal(packageReceipt.cache.cached_files.length, packageEntry.files.length);

  const safetyArchive = readJson(
    `examples/template/${packageEntry.safety_archive_receipt_path}`,
  );
  assert.equal(safetyArchive.schema, "forge.package_safety_archive_receipt");
  assert.equal(safetyArchive.package.name, "forms/react-hook-form");
  assert.equal(safetyArchive.archive.file_count, packageEntry.files.length);
  assert.ok(
    safetyArchive.archive.files.every((file) =>
      fs.existsSync(path.join(templateRoot, file.cache_path)),
    ),
    "forms/react-hook-form archive should reference existing cache files",
  );

  assert.ok(status.locked_package_names.includes("forms/react-hook-form"));
  assert.equal(status.package_count, lock.packages.length);
  assert.equal(status.locked_package_count, lock.packages.length);
  assert.equal(
    status.cache.cache_file_count,
    lock.packages.reduce((count, entry) => count + entry.files.length, 0),
  );
  assert.ok(
    status.cache.manifests.includes(
      ".dx/forge/cache/forms-react-hook-form/7.75.0-dx.0/.dx/build-cache/manifest.json",
    ),
    "package-status cache manifests should include forms/react-hook-form",
  );
  assert.equal(status.safety_archive.rollback_covered_package_count, lock.packages.length);
});
