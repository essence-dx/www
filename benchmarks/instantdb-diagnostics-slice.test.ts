const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes diagnostics helpers from public exports", () => {
  const upstreamReact = read(path.join(mirror, "client", "packages", "react", "src", "index.ts"));
  const upstreamError = read(path.join(mirror, "client", "packages", "core", "src", "utils", "fetch.ts"));
  const upstreamWarnings = read(path.join(mirror, "client", "packages", "core", "src", "warningToggle.ts"));
  const upstreamLogin = read(path.join(mirror, "examples", "solidjs-vite-advanced", "src", "components", "Login.tsx"));
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));
  const launchProof = read(path.join(root, "examples", "template", "instantdb-status.tsx"));

  assert.match(upstreamReact, /InstantAPIError,/);
  assert.match(upstreamReact, /setInstantWarningsEnabled,/);
  assert.match(upstreamError, /export class InstantAPIError extends InstantError/);
  assert.match(upstreamWarnings, /export const setInstantWarningsEnabled/);
  assert.match(upstreamLogin, /err\.body\?\.message \|\| err\.message/);

  assert.match(slice, /"js\/instant\/diagnostics\.ts"/);
  assert.match(slice, /InstantAPIError/);
  assert.match(slice, /InstantError/);
  assert.match(slice, /setInstantLaunchWarningsEnabled/);
  assert.match(slice, /setInstantWarningsEnabled\(enabled\)/);
  assert.match(slice, /isInstantLaunchApiError/);
  assert.match(slice, /formatInstantLaunchError/);
  assert.match(slice, /error\.body\?\.message \?\? error\.message/);
  assert.match(slice, /diagnostics: "formatInstantLaunchError\(error\)"/);

  assert.match(registry, /lib\/instant\/diagnostics\.ts/);
  assert.match(registry, /formatInstantLaunchError/);
  assert.match(registry, /setInstantWarningsEnabled/);

  assert.match(launchProof, /formatInstantLaunchError/);
  assert.match(launchProof, /data-dx-instant-diagnostics/);
  assert.match(launchProof, /diagnostics helper wired/);
});
