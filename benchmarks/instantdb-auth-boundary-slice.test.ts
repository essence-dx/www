const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes real auth boundary helpers from upstream API", () => {
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
  const upstreamGatedAuth = read(
    path.join(
      mirror,
      "client",
      "sandbox",
      "react-nextjs",
      "pages",
      "play",
      "gated-auth.tsx",
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

  assert.match(upstreamReactCommon, /useUser = \(\): User =>/);
  assert.match(upstreamReactCommon, /SignedIn: React\.FC/);
  assert.match(upstreamReactCommon, /SignedOut: React\.FC/);
  assert.match(upstreamGatedAuth, /db\.useUser\(\)/);
  assert.match(upstreamGatedAuth, /<db\.SignedIn>/);
  assert.match(upstreamGatedAuth, /<db\.SignedOut>/);

  assert.match(slice, /"js\/components\/instant\/instant-auth-boundary\.tsx"/);
  assert.match(slice, /InstantLaunchAuthBoundary/);
  assert.match(slice, /<db\.SignedIn>/);
  assert.match(slice, /<db\.SignedOut>/);
  assert.match(slice, /db\.useUser\(\)/);
  assert.match(slice, /InstantLaunchUserBadge/);
  assert.match(slice, /authBoundary: "InstantLaunchAuthBoundary \+ InstantLaunchUserBadge"/);

  assert.match(registry, /components\/instant\/instant-auth-boundary\.tsx/);
  assert.match(registry, /auth_boundary\.contains\("<db\.SignedIn>"\)/);
  assert.match(registry, /auth_boundary\.contains\("db\.useUser\(\)"\)/);

  assert.match(launchProof, /InstantLaunchAuthBoundary/);
  assert.match(launchProof, /InstantLaunchUserBadge/);
  assert.match(launchProof, /data-dx-instant-auth-boundary/);
  assert.match(launchProof, /auth boundary wired/);
});
