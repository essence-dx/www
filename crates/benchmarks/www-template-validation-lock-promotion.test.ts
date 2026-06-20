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

test("validation/zod is promoted into the lock-backed Forge package set", () => {
  const sourceManifest = readJson(
    "examples/template/.dx/forge/source-.dx/build-cache/manifest.json",
  );
  const lock = readJson("examples/template/.dx/forge/package-lock.json");
  const status = readJson("examples/template/.dx/forge/package-status.json");

  const sourcePackage = sourceManifest.packages.find(
    (entry) => entry.package_id === "validation/zod",
  );
  assert.ok(sourcePackage, "source manifest should expose validation/zod");
  assert.equal(sourcePackage.upstream_name, "zod");
  assert.equal(sourcePackage.version, "4.4.3-dx.13");
  assert.equal(sourcePackage.files.length, 20);
  assert.ok(sourcePackage.rollback_receipt, "validation/zod needs rollback receipt");

  const packageEntry = lock.packages.find((entry) => entry.name === "validation/zod");
  assert.ok(packageEntry, "package lock should expose validation/zod");
  assert.equal(packageEntry.source_kind, "local-slice");
  assert.equal(packageEntry.source_locator, "lib/validation/zod/dashboard-settings.ts");
  assert.equal(packageEntry.integrity_state, "valid");
  assert.equal(packageEntry.files.length, sourcePackage.files.length);
  assert.equal(packageEntry.integrity_hash, sourcePackage.integrity_hash);
  assert.ok(packageEntry.rollback_receipt_path);
  assert.ok(packageEntry.safety_archive_receipt_path);

  for (const expectedFile of [
    "lib/validation/zod/schemas.ts",
    "lib/validation/zod/parse.ts",
    "lib/validation/zod/dashboard-settings.ts",
    "lib/validation/zod/template-forms.ts",
    "lib/validation/zod/metadata.ts",
    "lib/validation/zod/README.md",
  ]) {
    assert.ok(
      packageEntry.files.some((file) => file.path === expectedFile),
      `missing validation/zod locked file ${expectedFile}`,
    );
    assert.ok(
      fs.existsSync(path.join(templateRoot, expectedFile)),
      `missing materialized validation/zod source ${expectedFile}`,
    );
  }

  const packageReceiptPath = packageEntry.receipt_paths.find((candidate) =>
    candidate.includes("receipts/packages/validation-zod.json"),
  );
  assert.ok(packageReceiptPath, "validation/zod should reference package-add receipt");

  const packageReceipt = readJson(`examples/template/${packageReceiptPath}`);
  assert.equal(packageReceipt.schema, "forge.package_add_receipt");
  assert.equal(packageReceipt.package.name, "validation/zod");
  assert.equal(
    packageReceipt.boundary,
    "forge-owned source slice; no node_modules install performed",
  );
  assert.equal(packageReceipt.cache.cached_files.length, packageEntry.files.length);

  const safetyArchive = readJson(
    `examples/template/${packageEntry.safety_archive_receipt_path}`,
  );
  assert.equal(safetyArchive.schema, "forge.package_safety_archive_receipt");
  assert.equal(safetyArchive.package.name, "validation/zod");
  assert.equal(safetyArchive.archive.file_count, packageEntry.files.length);
  assert.ok(
    safetyArchive.archive.files.every((file) =>
      fs.existsSync(path.join(templateRoot, file.cache_path)),
    ),
    "validation/zod archive should reference existing cache files",
  );

  assert.ok(status.locked_package_names.includes("validation/zod"));
  assert.equal(status.package_count, lock.packages.length);
  assert.equal(status.locked_package_count, lock.packages.length);
  assert.equal(
    status.cache.cache_file_count,
    lock.packages.reduce((count, entry) => count + entry.files.length, 0),
  );
  assert.ok(
    status.cache.manifests.includes(
      ".dx/forge/cache/validation-zod/4.4.3-dx.13/.dx/build-cache/manifest.json",
    ),
    "package-status cache manifests should include validation/zod",
  );
  assert.equal(status.safety_archive.rollback_covered_package_count, lock.packages.length);
});
