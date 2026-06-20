const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const templateRoot = path.join(root, "examples", "template");

const packageId = "wasm/bindgen";
const cacheManifestPath =
  ".dx/forge/cache/wasm-bindgen/0.2.121-dx.0/.dx/build-cache/manifest.json";
const expectedFiles = [
  "wasm/bindgen/loader.ts",
  "wasm/bindgen/react.tsx",
  "wasm/bindgen/readiness.ts",
  "wasm/bindgen/metadata.ts",
  "wasm/bindgen/README.md",
];

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

test("wasm/bindgen is promoted into the lock-backed Forge package set without live-Wasm overclaims", () => {
  const sourceManifest = readJson(
    "examples/template/.dx/forge/source-.dx/build-cache/manifest.json",
  );
  const lock = readJson("examples/template/.dx/forge/package-lock.json");
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const cacheManifest = readJson(`examples/template/${cacheManifestPath}`);

  const sourcePackage = sourceManifest.packages.find(
    (entry) => entry.package_id === packageId,
  );
  assert.ok(sourcePackage, "source manifest should expose wasm/bindgen");
  assert.equal(sourcePackage.official_package_name, "WebAssembly Bridge");
  assert.equal(sourcePackage.upstream_name, "wasm-bindgen");
  assert.equal(sourcePackage.version, "0.2.121-dx.0");
  assert.equal(sourcePackage.source_kind, "curated-registry");
  assert.equal(sourcePackage.runtime_proof, false);
  assert.equal(sourcePackage.generated_wasm_artifact, false);
  assert.equal(sourcePackage.files.length, expectedFiles.length);
  assert.ok(sourcePackage.rollback_receipt, "wasm/bindgen needs rollback receipt");

  const packageEntry = lock.packages.find((entry) => entry.name === packageId);
  assert.ok(packageEntry, "package lock should expose wasm/bindgen");
  assert.equal(packageEntry.official_package_name, "WebAssembly Bridge");
  assert.equal(packageEntry.source_kind, "local-slice");
  assert.equal(packageEntry.source_locator, "wasm/bindgen/loader.ts");
  assert.equal(packageEntry.integrity_state, "valid");
  assert.equal(packageEntry.files.length, expectedFiles.length);
  assert.equal(packageEntry.integrity_hash, sourcePackage.integrity_hash);
  assert.equal(packageEntry.runtime_proof, false);
  assert.equal(packageEntry.generated_wasm_artifact, false);
  assert.match(
    packageEntry.provenance.note,
    /generated web-target JavaScript glue/i,
  );
  assert.match(packageEntry.provenance.note, /no generated \.wasm artifact/i);
  assert.ok(packageEntry.rollback_receipt_path);
  assert.ok(packageEntry.safety_archive_receipt_path);

  for (const expectedFile of expectedFiles) {
    assert.ok(
      packageEntry.files.some((file) => file.path === expectedFile),
      `missing wasm/bindgen locked file ${expectedFile}`,
    );
    assert.ok(
      fs.existsSync(path.join(templateRoot, expectedFile)),
      `missing materialized wasm/bindgen source ${expectedFile}`,
    );
    assert.ok(
      cacheManifest.cached_files.some((file) => file.path === expectedFile),
      `missing wasm/bindgen cached file ${expectedFile}`,
    );
  }

  assert.ok(
    cacheManifest.cached_files.every((file) =>
      fs.existsSync(path.join(templateRoot, file.cache_path)),
    ),
    "wasm/bindgen cache manifest should reference existing cached files",
  );

  const packageReceiptPath = packageEntry.receipt_paths.find((candidate) =>
    candidate.includes("receipts/packages/wasm-bindgen.json"),
  );
  assert.ok(packageReceiptPath, "wasm/bindgen should reference package-add receipt");

  const packageReceipt = readJson(`examples/template/${packageReceiptPath}`);
  assert.equal(packageReceipt.schema, "forge.package_add_receipt");
  assert.equal(packageReceipt.package.name, packageId);
  assert.equal(packageReceipt.package.official_package_name, "WebAssembly Bridge");
  assert.equal(
    packageReceipt.boundary,
    "forge-owned source slice; no generated .wasm artifact or package install performed",
  );
  assert.equal(packageReceipt.cache.cached_files.length, packageEntry.files.length);
  assert.equal(packageReceipt.runtime_proof, false);
  assert.equal(packageReceipt.generated_wasm_artifact, false);

  const safetyArchive = readJson(
    `examples/template/${packageEntry.safety_archive_receipt_path}`,
  );
  assert.equal(safetyArchive.schema, "forge.package_safety_archive_receipt");
  assert.equal(safetyArchive.package.name, packageId);
  assert.equal(safetyArchive.package.official_package_name, "WebAssembly Bridge");
  assert.equal(safetyArchive.archive.file_count, packageEntry.files.length);
  assert.equal(safetyArchive.runtime_proof, false);
  assert.equal(safetyArchive.generated_wasm_artifact, false);

  assert.ok(status.locked_package_names.includes(packageId));
  assert.equal(status.package_count, lock.packages.length);
  assert.equal(status.locked_package_count, lock.packages.length);
  assert.equal(
    status.cache.cache_file_count,
    lock.packages.reduce((count, entry) => count + entry.files.length, 0),
  );
  assert.ok(
    status.cache.manifests.includes(cacheManifestPath),
    "package-status cache manifests should include wasm/bindgen",
  );
  assert.equal(status.safety_archive.rollback_covered_package_count, lock.packages.length);
});
