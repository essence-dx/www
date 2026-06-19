const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes imperative subscription helpers from public core API", () => {
  const upstreamCore = read(path.join(mirror, "client", "packages", "core", "src", "index.ts"));
  const upstreamVanilla = read(path.join(mirror, "examples", "vite-vanilla", "src", "main.ts"));
  const upstreamSandbox = read(path.join(mirror, "client", "sandbox", "vanilla-js-vite", "src", "main.ts"));
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));
  const launchProof = read(path.join(root, "examples", "template", "instantdb-status.tsx"));

  assert.match(upstreamCore, /subscribeQuery<[\s\S]*return this\._reactor\.subscribeQuery/);
  assert.match(upstreamCore, /subscribeAuth\(cb: \(auth: AuthResult\) => void\)/);
  assert.match(upstreamCore, /subscribeConnectionStatus\(/);
  assert.match(upstreamVanilla, /db\.subscribeQuery\(\{ todos: \{\} \}, \(resp\) =>/);
  assert.match(upstreamSandbox, /db\.subscribeAuth\(\(auth\) =>/);

  assert.match(slice, /"js\/instant\/subscriptions\.ts"/);
  assert.match(slice, /subscribeInstantLaunchTodos/);
  assert.match(slice, /db\.core\.subscribeQuery\(instantLaunchTodosQuery, callback\)/);
  assert.match(slice, /subscribeInstantLaunchAuth/);
  assert.match(slice, /db\.core\.subscribeAuth\(callback\)/);
  assert.match(slice, /subscribeInstantLaunchConnectionStatus/);
  assert.match(slice, /db\.core\.subscribeConnectionStatus\(callback\)/);
  assert.match(slice, /subscriptions: "subscribeInstantLaunchTodos\(callback\)"/);

  assert.match(registry, /lib\/instant\/subscriptions\.ts/);
  assert.match(registry, /subscribeInstantLaunchTodos/);
  assert.match(registry, /db\.core\.subscribeQuery/);

  assert.match(launchProof, /subscribeInstantLaunchTodos/);
  assert.match(launchProof, /data-dx-instant-subscriptions/);
  assert.match(launchProof, /subscription helpers wired/);
});
