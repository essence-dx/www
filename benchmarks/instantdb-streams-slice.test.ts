const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes real stream helpers from upstream API", () => {
  const upstreamCore = read(path.join(mirror, "client", "packages", "core", "src", "index.ts"));
  const upstreamExample = read(
    path.join(mirror, "client", "sandbox", "solid-vite", "src", "pages", "Streams.tsx"),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const launchProof = read(path.join(root, "examples", "template", "instantdb-status.tsx"));
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));

  assert.match(upstreamCore, /createReadStream = \(/);
  assert.match(upstreamCore, /createWriteStream = \(/);
  assert.match(upstreamExample, /db\.streams\.createWriteStream/);
  assert.match(upstreamExample, /db\.streams\.createReadStream/);

  assert.match(slice, /"js\/instant\/streams\.ts"/);
  assert.match(slice, /export type DxInstantStreamTarget/);
  assert.match(slice, /createInstantLaunchWriteStream/);
  assert.match(slice, /createInstantLaunchReadStream/);
  assert.match(slice, /writeInstantLaunchText/);
  assert.match(slice, /db\.streams\.createWriteStream/);
  assert.match(slice, /db\.streams\.createReadStream/);
  assert.match(slice, /streamHelper: "createInstantLaunchWriteStream\(clientId\)"/);
  assert.match(slice, /stream lifecycle/);
  assert.match(launchProof, /createInstantLaunchWriteStream/);
  assert.match(launchProof, /data-dx-instant-streams/);

  assert.match(registry, /lib\/instant\/streams\.ts/);
});
