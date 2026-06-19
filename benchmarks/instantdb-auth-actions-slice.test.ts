const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes real auth action helpers from upstream API", () => {
  const upstreamCore = read(path.join(mirror, "client", "packages", "core", "src", "index.ts"));
  const upstreamExample = read(
    path.join(mirror, "client", "sandbox", "react-nextjs", "pages", "play", "guest.tsx"),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));
  const launchProof = read(path.join(root, "examples", "template", "instantdb-status.tsx"));

  assert.match(upstreamCore, /sendMagicCode = \(/);
  assert.match(upstreamCore, /signInWithMagicCode = \(/);
  assert.match(upstreamCore, /signInAsGuest = \(/);
  assert.match(upstreamCore, /signOut = \(/);
  assert.match(upstreamExample, /auth\.sendMagicCode/);
  assert.match(upstreamExample, /auth\.signInWithMagicCode/);
  assert.match(upstreamExample, /auth\.signInAsGuest/);
  assert.match(upstreamExample, /auth\.signOut/);

  assert.match(slice, /"js\/instant\/auth\.ts"/);
  assert.match(slice, /sendInstantLaunchMagicCode/);
  assert.match(slice, /verifyInstantLaunchMagicCode/);
  assert.match(slice, /signInInstantLaunchGuest/);
  assert.match(slice, /signOutInstantLaunchUser/);
  assert.match(slice, /db\.auth\.sendMagicCode/);
  assert.match(slice, /db\.auth\.signInWithMagicCode/);
  assert.match(slice, /db\.auth\.signInAsGuest/);
  assert.match(slice, /db\.auth\.signOut/);
  assert.match(slice, /authHelpers: "sendInstantLaunchMagicCode\(email\) \+ verifyInstantLaunchMagicCode\(code\)"/);
  assert.match(slice, /Applications still own Instant auth flows/);

  assert.match(registry, /lib\/instant\/auth\.ts/);
  assert.match(registry, /sendInstantLaunchMagicCode/);
  assert.match(registry, /db\.auth\.sendMagicCode/);

  assert.match(launchProof, /sendInstantLaunchMagicCode/);
  assert.match(launchProof, /verifyInstantLaunchMagicCode/);
  assert.match(launchProof, /data-dx-instant-auth-actions/);
  assert.match(launchProof, /auth action helpers wired/);
});
