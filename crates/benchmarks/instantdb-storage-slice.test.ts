const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes real storage helpers from upstream API", () => {
  const upstreamCore = read(path.join(mirror, "client", "packages", "core", "src", "index.ts"));
  const upstreamStorage = read(
    path.join(mirror, "client", "packages", "core", "src", "StorageAPI.ts"),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const launchProof = read(
    path.join(root, "examples", "template", "instantdb-status.tsx"),
  );
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));

  assert.match(upstreamCore, /uploadFile = \(/);
  assert.match(upstreamCore, /@deprecated Use `db\.transact` to delete files instead/);
  assert.match(upstreamCore, /db\.transact\(db\.tx\.\$files\[lookup\('path', 'photos\/demo\.png'\)\]\.delete\(\)\)/);
  assert.match(upstreamCore, /@deprecated\. getDownloadUrl will be removed in the future/);
  assert.match(upstreamCore, /Use `useQuery` instead to query and fetch for valid urls/);
  assert.match(upstreamStorage, /type UploadFileResponse/);
  assert.match(upstreamStorage, /type DeleteFileResponse/);

  assert.match(slice, /"js\/instant\/storage\.ts"/);
  assert.match(slice, /export type DxInstantStorageUploadInput/);
  assert.match(slice, /export type DxInstantFileRecord/);
  assert.match(slice, /uploadInstantLaunchFile/);
  assert.match(slice, /instantLaunchFileQuery/);
  assert.match(slice, /queryInstantLaunchFile/);
  assert.match(slice, /deleteInstantLaunchFile/);
  assert.match(slice, /db\.storage\.uploadFile/);
  assert.match(slice, /fileClient\(\)\.queryOnce/);
  assert.match(slice, /fileClient\(\)\.transact/);
  assert.match(slice, /lookup\("path", path\)/);
  assert.doesNotMatch(slice, /db\.storage\.delete/);
  assert.doesNotMatch(slice, /db\.storage\.getDownloadUrl/);
  assert.match(slice, /storageHelper: "uploadInstantLaunchFile\(file\)"/);
  assert.match(slice, /fileLookupHelper: "queryInstantLaunchFile\(path\)"/);
  assert.match(slice, /file access rules/);

  assert.match(launchProof, /uploadInstantLaunchFile/);
  assert.match(launchProof, /queryInstantLaunchFile/);
  assert.match(launchProof, /deleteInstantLaunchFile/);
  assert.match(launchProof, /data-dx-instant-storage/);

  assert.match(registry, /lib\/instant\/storage\.ts/);
});
