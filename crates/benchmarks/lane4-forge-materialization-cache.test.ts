import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

import { databaseApiLaneMaterializationReality } from "../examples/template/components/template-app/package-lock-reality.ts";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const templateRoot = path.join(root, "examples", "template");

type ForgeFile = {
  readonly path: string;
  readonly bytes: number;
  readonly content_hash?: string;
  readonly expected_hash?: string;
  readonly hash?: string;
  readonly hash_algorithm?: string;
  readonly cache_path?: string;
};

type ForgePackage = {
  readonly name?: string;
  readonly package_id?: string;
  readonly version: string;
  readonly source_kind?: string;
  readonly integrity_state?: string;
  readonly file_count?: number;
  readonly files: readonly ForgeFile[];
};

function readJson(relativePath: string) {
  return JSON.parse(fs.readFileSync(path.join(templateRoot, relativePath), "utf8"));
}

function packageIdOf(packageEntry: ForgePackage) {
  return packageEntry.package_id ?? packageEntry.name;
}

function findPackage(
  packages: readonly ForgePackage[],
  packageId: string,
): ForgePackage {
  const packageEntry = packages.find((entry) => packageIdOf(entry) === packageId);
  assert.ok(packageEntry, `${packageId} should be present`);
  return packageEntry;
}

function filesByPath(files: readonly ForgeFile[]) {
  return new Map(files.map((file) => [file.path, file]));
}

function assertTemplatePathIsSourceOwned(filePath: string) {
  assert.equal(
    path.isAbsolute(filePath),
    false,
    `${filePath} must be a template-relative source path`,
  );
  assert.equal(
    filePath.includes("node_modules"),
    false,
    `${filePath} must not require node_modules ownership`,
  );
  assert.doesNotMatch(filePath, /(^|[/.])(?:pg|cp|cjs)(?:$|[/.])/);
  assert.doesNotMatch(filePath, /(?:^|[/.])v1(?:$|[/.])/);
}

test("Database + API lane lock entries match materialized source and Forge cache bytes", () => {
  const packageLock = readJson(".dx/forge/package-lock.json");
  const packageStatus = readJson(".dx/forge/package-status.json");
  const sourceManifest = readJson(".dx/forge/source-.dx/build-cache/manifest.json");

  assert.deepEqual(databaseApiLaneMaterializationReality.packageIds, [
    "db/drizzle-sqlite",
    "instantdb/react",
    "supabase/client",
    "api/trpc",
  ]);
  assert.equal(packageStatus.package_lock.integrity_valid, true);
  assert.equal(packageStatus.no_node_modules_required, true);

  for (const packageId of databaseApiLaneMaterializationReality.packageIds) {
    const lockPackage = findPackage(packageLock.packages, packageId);
    const statusPackage = findPackage(packageStatus.packages, packageId);
    const sourcePackage = findPackage(sourceManifest.packages, packageId);
    const expectedCacheManifest =
      databaseApiLaneMaterializationReality.cacheManifestByPackageId[packageId];
    const cacheManifest = readJson(expectedCacheManifest);
    const sourceFiles = filesByPath(sourcePackage.files);
    const cacheFiles = filesByPath(cacheManifest.cached_files);

    assert.equal(lockPackage.source_kind, "local-slice");
    assert.equal(lockPackage.integrity_state, "valid");
    assert.equal(statusPackage.integrity_state, "valid");
    assert.equal(statusPackage.file_count, lockPackage.files.length);
    assert.equal(sourcePackage.files.length, lockPackage.files.length);
    assert.equal(cacheManifest.cached_files.length, lockPackage.files.length);
    assert.ok(
      packageStatus.cache.manifests.includes(expectedCacheManifest),
      `${packageId} cache manifest should be visible in package-status`,
    );

    for (const filePath of databaseApiLaneMaterializationReality.frontFacingFilesByPackageId[
      packageId
    ]) {
      assert.ok(
        lockPackage.files.some((file) => file.path === filePath),
        `${packageId} should lock ${filePath}`,
      );
    }

    for (const lockFile of lockPackage.files) {
      assertTemplatePathIsSourceOwned(lockFile.path);
      const sourceFile = sourceFiles.get(lockFile.path);
      const cacheFile = cacheFiles.get(lockFile.path);
      assert.ok(sourceFile, `${packageId} source manifest missing ${lockFile.path}`);
      assert.ok(cacheFile, `${packageId} cache manifest missing ${lockFile.path}`);
      assert.equal(lockFile.hash_algorithm, "blake3");
      assert.equal(cacheFile.hash_algorithm, "blake3");
      assert.equal(sourceFile.hash, lockFile.content_hash);
      assert.equal(lockFile.expected_hash, lockFile.content_hash);
      assert.equal(cacheFile.content_hash, lockFile.content_hash);
      assert.equal(sourceFile.bytes, lockFile.bytes);
      assert.equal(cacheFile.bytes, lockFile.bytes);

      const materializedPath = path.join(templateRoot, lockFile.path);
      const cachePath = path.join(templateRoot, cacheFile.cache_path ?? "");
      const materializedBytes = fs.readFileSync(materializedPath);
      const cachedBytes = fs.readFileSync(cachePath);
      assert.equal(
        materializedBytes.byteLength,
        lockFile.bytes,
        `${packageId} ${lockFile.path} byte count should match package-lock`,
      );
      assert.equal(
        cachedBytes.byteLength,
        lockFile.bytes,
        `${packageId} ${lockFile.path} byte count should match cache manifest`,
      );
      assert.ok(
        materializedBytes.equals(cachedBytes),
        `${packageId} ${lockFile.path} must match its Forge cache copy byte-for-byte`,
      );
    }
  }
});

test("Database + API readiness contract is promoted into the Type-Safe API Forge package", () => {
  const packageLock = readJson(".dx/forge/package-lock.json");
  const packageStatus = readJson(".dx/forge/package-status.json");
  const sourceManifest = readJson(".dx/forge/source-.dx/build-cache/manifest.json");
  const apiPackageId = "api/trpc";
  const promotedFiles = [
    "lib/database-api/source-contract.ts",
    "server/database-api/readiness.ts",
    "app/api/database-api/readiness/route.ts",
  ];

  const lockPackage = findPackage(packageLock.packages, apiPackageId);
  const statusPackage = findPackage(packageStatus.packages, apiPackageId);
  const sourcePackage = findPackage(sourceManifest.packages, apiPackageId);
  const cacheManifest = readJson(
    databaseApiLaneMaterializationReality.cacheManifestByPackageId[apiPackageId],
  );
  const lockFiles = filesByPath(lockPackage.files);
  const sourceFiles = filesByPath(sourcePackage.files);
  const cacheFiles = filesByPath(cacheManifest.cached_files);

  assert.equal(statusPackage.file_count, lockPackage.files.length);
  assert.equal(sourcePackage.files.length, lockPackage.files.length);
  assert.equal(cacheManifest.cached_files.length, lockPackage.files.length);

  for (const filePath of promotedFiles) {
    assertTemplatePathIsSourceOwned(filePath);
    assert.ok(lockFiles.has(filePath), `package-lock should include ${filePath}`);
    assert.ok(sourceFiles.has(filePath), `source manifest should include ${filePath}`);
    assert.ok(cacheFiles.has(filePath), `cache manifest should include ${filePath}`);

    const lockFile = lockFiles.get(filePath);
    const sourceFile = sourceFiles.get(filePath);
    const cacheFile = cacheFiles.get(filePath);
    assert.equal(lockFile?.hash_algorithm, "blake3");
    assert.equal(cacheFile?.hash_algorithm, "blake3");
    assert.equal(lockFile?.content_hash, sourceFile?.hash);
    assert.equal(cacheFile?.content_hash, sourceFile?.hash);
    assert.equal(lockFile?.bytes, sourceFile?.bytes);
    assert.equal(cacheFile?.bytes, sourceFile?.bytes);

    const materializedBytes = fs.readFileSync(path.join(templateRoot, filePath));
    const cachedBytes = fs.readFileSync(path.join(templateRoot, cacheFile?.cache_path ?? ""));
    assert.equal(materializedBytes.byteLength, sourceFile?.bytes);
    assert.ok(materializedBytes.equals(cachedBytes), `${filePath} cache copy should match source`);
  }
});
