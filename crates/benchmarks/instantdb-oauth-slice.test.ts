const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes real OAuth and token auth helpers", () => {
  const upstreamCore = read(path.join(mirror, "client", "packages", "core", "src", "index.ts"));
  const upstreamAuthSync = read(
    path.join(mirror, "client", "sandbox", "react-nextjs", "pages", "play", "authsync.tsx"),
  );
  const upstreamExpo = read(
    path.join(mirror, "client", "sandbox", "react-native-expo", "app", "play", "expo-auth-session.tsx"),
  );
  const upstreamLogin = read(
    path.join(mirror, "client", "sandbox", "react-nextjs", "app", "play", "app-router-login", "page.tsx"),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));
  const launchProof = read(path.join(root, "examples", "template", "instantdb-status.tsx"));

  assert.match(upstreamCore, /signInWithToken = \(token: AuthToken\)/);
  assert.match(upstreamCore, /createAuthorizationURL = \(params:/);
  assert.match(upstreamCore, /signInWithIdToken = \(/);
  assert.match(upstreamCore, /exchangeOAuthCode = \(/);
  assert.match(upstreamCore, /issuerURI = \(\): string/);
  assert.match(upstreamAuthSync, /db\.auth\.signInWithToken/);
  assert.match(upstreamExpo, /db\.auth\.issuerURI\(\)/);
  assert.match(upstreamExpo, /\.exchangeOAuthCode/);
  assert.match(upstreamLogin, /auth\.createAuthorizationURL/);

  assert.match(slice, /"js\/instant\/oauth\.ts"/);
  assert.match(slice, /createInstantLaunchAuthorizationUrl/);
  assert.match(slice, /db\.auth\.createAuthorizationURL/);
  assert.match(slice, /signInInstantLaunchWithIdToken/);
  assert.match(slice, /db\.auth\.signInWithIdToken/);
  assert.match(slice, /exchangeInstantLaunchOAuthCode/);
  assert.match(slice, /db\.auth\.exchangeOAuthCode/);
  assert.match(slice, /signInInstantLaunchWithToken/);
  assert.match(slice, /db\.auth\.signInWithToken/);
  assert.match(slice, /instantLaunchIssuerUri/);
  assert.match(slice, /db\.auth\.issuerURI/);
  assert.match(slice, /OAuth provider setup/);

  assert.match(registry, /lib\/instant\/oauth\.ts/);
  assert.match(registry, /createInstantLaunchAuthorizationUrl/);
  assert.match(registry, /db\.auth\.createAuthorizationURL/);

  assert.match(launchProof, /createInstantLaunchAuthorizationUrl/);
  assert.match(launchProof, /data-dx-instant-oauth/);
  assert.match(launchProof, /oauth helpers wired/);
});
