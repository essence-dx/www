const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes explicit create transactions for new records", () => {
  const upstreamTx = read(path.join(mirror, "client", "packages", "core", "src", "instatx.ts"));
  const upstreamNextExample = read(
    path.join(mirror, "client", "sandbox", "react-nextjs", "app", "play", "ssr", "AddTodo.tsx"),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));

  assert.match(upstreamTx, /Create objects\. Throws an error if the object with the provided ID already/);
  assert.match(upstreamTx, /create: \(/);
  assert.match(upstreamNextExample, /db\.tx\.todos\[id\(\)\]\.create\(\{/);

  assert.match(slice, /export function addInstantTodo/);
  assert.match(slice, /db\.tx\.todos\[id\(\)\]\.create\(\{/);
  assert.match(slice, /"db\.tx\.\*\.create"/);
  assert.match(slice, /mutationSurface: "db\.transact\(db\.tx\.todos\[id\(\)\]\.create\(\.\.\.\)\)"/);
  assert.match(slice, /explicit `\.create\(\)`/);

  assert.match(registry, /mutations\.contains\("\.create\(\{"\)/);
});
