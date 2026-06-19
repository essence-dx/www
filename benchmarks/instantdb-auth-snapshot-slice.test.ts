const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes one-time auth snapshot helpers from upstream API", () => {
  const upstreamReactCommon = read(
    path.join(
      mirror,
      "client",
      "packages",
      "react-common",
      "src",
      "InstantReactAbstractDatabase.tsx",
    ),
  );
  const upstreamCore = read(
    path.join(mirror, "client", "packages", "core", "src", "index.ts"),
  );
  const upstreamStatusBar = read(
    path.join(
      mirror,
      "examples",
      "solidjs-vite-advanced",
      "src",
      "components",
      "StatusBar.tsx",
    ),
  );
  const slice = read(
    path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"),
  );
  const registry = read(
    path.join(root, "core", "src", "ecosystem", "forge_registry.rs"),
  );
  const launchProof = read(
    path.join(root, "examples", "template", "instantdb-status.tsx"),
  );

  assert.match(upstreamReactCommon, /getAuth\(\): Promise<User \| null>/);
  assert.match(upstreamReactCommon, /return this\.core\.getAuth\(\)/);
  assert.match(upstreamCore, /getAuth\(\): Promise<User \| null>/);
  assert.match(upstreamStatusBar, /await db\.getAuth\(\)/);

  assert.match(slice, /export function getInstantLaunchAuth\(\)/);
  assert.match(slice, /return db\.getAuth\(\)/);
  assert.match(slice, /export async function requireInstantLaunchUser\(\)/);
  assert.match(slice, /authSnapshot: "getInstantLaunchAuth\(\)"/);
  assert.match(slice, /"db\.getAuth"/);

  assert.match(registry, /auth\.contains\("getInstantLaunchAuth"\)/);
  assert.match(registry, /auth\.contains\("db\.getAuth"\)/);
  assert.match(registry, /auth\.contains\("requireInstantLaunchUser"\)/);

  assert.match(launchProof, /getInstantLaunchAuth/);
  assert.match(launchProof, /requireInstantLaunchUser/);
  assert.match(launchProof, /data-dx-instant-auth-snapshot/);
  assert.match(launchProof, /getAuth helper wired/);
});
