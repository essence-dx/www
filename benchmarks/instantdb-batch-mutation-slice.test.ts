const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes batched transaction helpers from upstream examples", () => {
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
  const upstreamNextApp = read(
    path.join(mirror, "examples", "next-js-app-dir", "src", "app", "page.tsx"),
  );
  const upstreamViteApp = read(
    path.join(mirror, "examples", "vite-react", "src", "App.tsx"),
  );
  const slice = read(
    path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"),
  );
  const registry = read(
    path.join(root, "core", "src", "ecosystem", "forge_registry.rs"),
  );

  assert.match(upstreamReactCommon, /db\.transact\(\[/);
  assert.match(upstreamReactCommon, /transact = \(/);
  assert.match(upstreamNextApp, /const txs = completed\.map/);
  assert.match(upstreamNextApp, /db\.transact\(txs\)/);
  assert.match(upstreamViteApp, /const txs = completed\.map/);
  assert.match(upstreamViteApp, /db\.transact\(txs\)/);

  assert.match(slice, /export function clearCompletedInstantTodos/);
  assert.match(slice, /\.map\(\(todo\) => db\.tx\.todos\[todo\.id\]\.delete\(\)\)/);
  assert.match(slice, /db\.transact\(chunks\)/);
  assert.match(slice, /batchMutation: "clearCompletedInstantTodos\(todos\)"/);
  assert.match(slice, /Clear completed/);

  assert.match(registry, /mutations\.contains\("clearCompletedInstantTodos"\)/);
  assert.match(registry, /mutations\.contains\("db\.transact\(chunks\)"\)/);
});
