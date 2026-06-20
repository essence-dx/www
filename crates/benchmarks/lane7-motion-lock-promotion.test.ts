import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const templateRoot = path.join(root, "examples", "template");

const packageId = "animation/motion";
const cacheManifestPath =
  ".dx/forge/cache/animation-motion/12.38.0-dx.12/.dx/build-cache/manifest.json";
const expectedFiles = [
  "motion/presets.ts",
  "motion/provider.tsx",
  "motion/controls.tsx",
  "motion/frame.tsx",
  "motion/layout.tsx",
  "motion/lazy.tsx",
  "motion/motion-values.tsx",
  "motion/page-visibility.tsx",
  "motion/presence.tsx",
  "motion/reorder.tsx",
  "motion/reveal.tsx",
  "motion/scoped-animate.tsx",
  "motion/scroll-progress.tsx",
  "motion/will-change.tsx",
  "motion/dashboard-workflow.ts",
  "motion/metadata.ts",
  "motion/README.md",
] as const;

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath: string) {
  return JSON.parse(read(relativePath));
}

test("Motion & Animation is promoted into the lock-backed Forge package set without browser-motion overclaims", () => {
  const sourceManifest = readJson(
    "examples/template/.dx/forge/source-.dx/build-cache/manifest.json",
  );
  const lock = readJson("examples/template/.dx/forge/package-lock.json");
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const cacheManifest = readJson(`examples/template/${cacheManifestPath}`);
  const packageLockReality = read(
    "examples/template/components/template-app/package-lock-reality.ts",
  );
  const packageReality = read(
    "examples/template/components/template-app/package-reality.ts",
  );

  const sourcePackage = sourceManifest.packages.find(
    (entry: { package_id?: string }) => entry.package_id === packageId,
  );
  assert.ok(sourcePackage, "source manifest should expose animation/motion");
  assert.equal(sourcePackage.official_package_name, "Motion & Animation");
  assert.equal(sourcePackage.upstream_name, "motion");
  assert.equal(sourcePackage.version, "12.38.0-dx.12");
  assert.equal(sourcePackage.source_kind, "curated-registry");
  assert.equal(sourcePackage.runtime_proof, false);
  assert.equal(sourcePackage.files.length, expectedFiles.length);
  assert.ok(sourcePackage.rollback_receipt, "animation/motion needs rollback receipt");

  const packageEntry = lock.packages.find(
    (entry: { name?: string }) => entry.name === packageId,
  );
  assert.ok(packageEntry, "package lock should expose animation/motion");
  assert.equal(packageEntry.official_package_name, "Motion & Animation");
  assert.equal(packageEntry.source_kind, "local-slice");
  assert.equal(packageEntry.source_locator, "motion/provider.tsx");
  assert.equal(packageEntry.integrity_state, "valid");
  assert.equal(packageEntry.files.length, expectedFiles.length);
  assert.equal(packageEntry.integrity_hash, sourcePackage.integrity_hash);
  assert.equal(packageEntry.runtime_proof, false);
  assert.match(packageEntry.provenance.note, /Motion React public APIs/i);
  assert.match(packageEntry.provenance.note, /no live browser animation proof/i);
  assert.ok(packageEntry.rollback_receipt_path);
  assert.ok(packageEntry.safety_archive_receipt_path);
  assert.ok(
    packageEntry.dependency_constraints.some(
      (dependency: { name?: string; boundary?: string }) =>
        dependency.name === "motion" &&
        /app-owned runtime/.test(dependency.boundary ?? ""),
    ),
    "animation/motion should record the real Motion adapter boundary",
  );

  for (const expectedFile of expectedFiles) {
    assert.ok(
      packageEntry.files.some((file: { path?: string }) => file.path === expectedFile),
      `missing animation/motion locked file ${expectedFile}`,
    );
    assert.ok(
      fs.existsSync(path.join(templateRoot, expectedFile)),
      `missing materialized animation/motion source ${expectedFile}`,
    );
    assert.ok(
      cacheManifest.cached_files.some(
        (file: { path?: string }) => file.path === expectedFile,
      ),
      `missing animation/motion cached file ${expectedFile}`,
    );
  }

  assert.ok(
    cacheManifest.cached_files.every((file: { cache_path: string }) =>
      fs.existsSync(path.join(templateRoot, file.cache_path)),
    ),
    "animation/motion cache manifest should reference existing cached files",
  );

  const packageReceiptPath = packageEntry.receipt_paths.find((candidate: string) =>
    candidate.includes("receipts/packages/animation-motion.json"),
  );
  assert.ok(packageReceiptPath, "animation/motion should reference package-add receipt");

  const packageReceipt = readJson(`examples/template/${packageReceiptPath}`);
  assert.equal(packageReceipt.schema, "forge.package_add_receipt");
  assert.equal(packageReceipt.package.name, packageId);
  assert.equal(
    packageReceipt.package.official_package_name,
    "Motion & Animation",
  );
  assert.equal(
    packageReceipt.boundary,
    "forge-owned source slice; no node_modules install or browser animation proof performed",
  );
  assert.equal(packageReceipt.cache.cached_files.length, packageEntry.files.length);
  assert.equal(packageReceipt.runtime_proof, false);

  const safetyArchive = readJson(
    `examples/template/${packageEntry.safety_archive_receipt_path}`,
  );
  assert.equal(safetyArchive.schema, "forge.package_safety_archive_receipt");
  assert.equal(safetyArchive.package.name, packageId);
  assert.equal(safetyArchive.package.official_package_name, "Motion & Animation");
  assert.equal(safetyArchive.archive.file_count, packageEntry.files.length);
  assert.equal(safetyArchive.runtime_proof, false);

  assert.ok(status.locked_package_names.includes(packageId));
  assert.equal(status.package_count, lock.packages.length);
  assert.equal(status.locked_package_count, lock.packages.length);
  assert.equal(
    status.cache.cache_file_count,
    lock.packages.reduce(
      (count: number, entry: { files?: readonly unknown[] }) =>
        count + (entry.files?.length ?? 0),
      0,
    ),
  );
  assert.ok(
    status.cache.manifests.includes(cacheManifestPath),
    "package-status cache manifests should include animation/motion",
  );
  assert.equal(
    status.safety_archive.rollback_covered_package_count,
    lock.packages.length,
  );

  assert.match(packageLockReality, /"animation\/motion"/);
  assert.match(packageReality, /"animation\/motion": 72/);
  assert.match(packageReality, /Lock\/cache proof for editable Motion & Animation helpers/);
});
