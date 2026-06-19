import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const templateRoot = path.join(root, "examples", "template");

const packageId = "3d/launch-scene";
const cacheManifestPath =
  ".dx/forge/cache/3d-launch-scene/0.184.0-r3f10-dx.0/manifest.json";
const expectedFiles = [
  "components/scene/launch-scene.tsx",
  "lib/scene/index.ts",
  "lib/scene/types.ts",
  "lib/scene/preset.ts",
  "lib/scene/interaction.ts",
  "lib/scene/dashboard-workflow.ts",
  "lib/scene/dashboard-controls.ts",
  "lib/scene/frame-sample.ts",
  "lib/scene/capability-report.ts",
  "lib/scene/viewport-report.ts",
  "lib/scene/bounds-report.ts",
  "lib/scene/raycast-report.ts",
  "lib/scene/preview-readiness.ts",
  "lib/scene/performance-monitor.ts",
  "lib/scene/renderer-handoff.ts",
  "lib/scene/r3f-renderer-adapter.ts",
  "lib/scene/webgl-runtime.ts",
  "lib/scene/metadata.ts",
  "lib/scene/README.md",
] as const;

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath: string) {
  return JSON.parse(read(relativePath));
}

test("3D Scene System is promoted into the lock-backed Forge package set without WebGL overclaims", () => {
  const sourceManifest = readJson(
    "examples/template/.dx/forge/source-manifest.json",
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
  assert.ok(sourcePackage, "source manifest should expose 3d/launch-scene");
  assert.equal(sourcePackage.official_package_name, "3D Scene System");
  assert.equal(
    sourcePackage.upstream_name,
    "three + @react-three/fiber + @react-three/drei",
  );
  assert.equal(sourcePackage.version, "0.184.0-r3f10-dx.0");
  assert.equal(sourcePackage.source_kind, "curated-registry");
  assert.equal(sourcePackage.runtime_proof, false);
  assert.equal(sourcePackage.webgl_runtime_proof, false);
  assert.equal(sourcePackage.browser_screenshot_proof, false);
  assert.equal(sourcePackage.files.length, expectedFiles.length);
  assert.ok(sourcePackage.rollback_receipt, "3d/launch-scene needs rollback receipt");

  const packageEntry = lock.packages.find(
    (entry: { name?: string }) => entry.name === packageId,
  );
  assert.ok(packageEntry, "package lock should expose 3d/launch-scene");
  assert.equal(packageEntry.official_package_name, "3D Scene System");
  assert.equal(packageEntry.source_kind, "local-slice");
  assert.equal(packageEntry.source_locator, "components/scene/launch-scene.tsx");
  assert.equal(packageEntry.integrity_state, "valid");
  assert.equal(packageEntry.files.length, expectedFiles.length);
  assert.equal(packageEntry.integrity_hash, sourcePackage.integrity_hash);
  assert.equal(packageEntry.runtime_proof, false);
  assert.equal(packageEntry.webgl_runtime_proof, false);
  assert.equal(packageEntry.browser_screenshot_proof, false);
  assert.match(packageEntry.provenance.note, /Three\/R3F\/Drei/i);
  assert.match(packageEntry.provenance.note, /no browser WebGL proof/i);
  assert.ok(packageEntry.rollback_receipt_path);
  assert.ok(packageEntry.safety_archive_receipt_path);
  assert.ok(
    packageEntry.dependency_constraints.some(
      (dependency: { name?: string; boundary?: string }) =>
        dependency.name === "three" &&
        /app-owned runtime/.test(dependency.boundary ?? ""),
    ),
    "3d/launch-scene should record the real Three adapter boundary",
  );

  for (const expectedFile of expectedFiles) {
    assert.ok(
      packageEntry.files.some((file: { path?: string }) => file.path === expectedFile),
      `missing 3d/launch-scene locked file ${expectedFile}`,
    );
    assert.ok(
      fs.existsSync(path.join(templateRoot, expectedFile)),
      `missing materialized 3d/launch-scene source ${expectedFile}`,
    );
    assert.ok(
      cacheManifest.cached_files.some(
        (file: { path?: string }) => file.path === expectedFile,
      ),
      `missing 3d/launch-scene cached file ${expectedFile}`,
    );
  }

  assert.ok(
    cacheManifest.cached_files.every((file: { cache_path: string }) =>
      fs.existsSync(path.join(templateRoot, file.cache_path)),
    ),
    "3d/launch-scene cache manifest should reference existing cached files",
  );

  const packageReceiptPath = packageEntry.receipt_paths.find((candidate: string) =>
    candidate.includes("receipts/packages/3d-launch-scene.json"),
  );
  assert.ok(packageReceiptPath, "3d/launch-scene should reference package-add receipt");

  const packageReceipt = readJson(`examples/template/${packageReceiptPath}`);
  assert.equal(packageReceipt.schema, "forge.package_add_receipt");
  assert.equal(packageReceipt.package.name, packageId);
  assert.equal(packageReceipt.package.official_package_name, "3D Scene System");
  assert.equal(
    packageReceipt.boundary,
    "forge-owned source slice; no node_modules install, browser WebGL proof, or screenshot proof performed",
  );
  assert.equal(packageReceipt.cache.cached_files.length, packageEntry.files.length);
  assert.equal(packageReceipt.runtime_proof, false);
  assert.equal(packageReceipt.webgl_runtime_proof, false);
  assert.equal(packageReceipt.browser_screenshot_proof, false);

  const safetyArchive = readJson(
    `examples/template/${packageEntry.safety_archive_receipt_path}`,
  );
  assert.equal(safetyArchive.schema, "forge.package_safety_archive_receipt");
  assert.equal(safetyArchive.package.name, packageId);
  assert.equal(safetyArchive.package.official_package_name, "3D Scene System");
  assert.equal(safetyArchive.archive.file_count, packageEntry.files.length);
  assert.equal(safetyArchive.runtime_proof, false);
  assert.equal(safetyArchive.webgl_runtime_proof, false);
  assert.equal(safetyArchive.browser_screenshot_proof, false);

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
    "package-status cache manifests should include 3d/launch-scene",
  );
  assert.equal(
    status.safety_archive.rollback_covered_package_count,
    lock.packages.length,
  );

  assert.match(packageLockReality, /"3d\/launch-scene"/);
  assert.match(packageReality, /"3d\/launch-scene": 84/);
  assert.match(
    packageReality,
    /Lock\/cache proof plus the default LandingSceneSurface/,
  );
});
