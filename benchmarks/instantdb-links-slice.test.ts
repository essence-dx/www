const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes real link and unlink relationship APIs", () => {
  const upstreamTx = read(path.join(mirror, "client", "packages", "core", "src", "instatx.ts"));
  const upstreamLinkExample = read(
    path.join(mirror, "client", "sandbox", "react-nextjs", "pages", "play", "linkunlink.tsx"),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));

  assert.match(upstreamTx, /Link two objects together/);
  assert.match(upstreamTx, /db\.tx\.goals\[goalId\]\.link\(\{todos: todoId\}\)/);
  assert.match(upstreamTx, /db\.tx\.goals\[goalId\]\.unlink\(\{todos: todoId\}\)/);
  assert.match(upstreamLinkExample, /links: \{/);
  assert.match(upstreamLinkExample, /\.link\(\{ members: userId \}\)/);
  assert.match(upstreamLinkExample, /\.unlink\(\{ members: userId \}\)/);

  assert.match(slice, /labels: i\.entity/);
  assert.match(slice, /todoLabels: \{/);
  assert.match(slice, /label: "labels"/);
  assert.match(slice, /labels: \{\}/);
  assert.match(slice, /labelInstantTodoForLaunch/);
  assert.match(slice, /\.link\(\{/);
  assert.match(slice, /unlabelInstantTodoForLaunch/);
  assert.match(slice, /\.unlink\(\{/);
  assert.match(slice, /linkMutation: "labelInstantTodoForLaunch\(todo\)"/);
  assert.match(slice, /relationship `\.link\(\)` \/ `\.unlink\(\)`/);

  assert.match(registry, /labelInstantTodoForLaunch/);
  assert.match(registry, /\.unlink\(\{/);
});
