const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes real merge transaction API", () => {
  const upstreamTx = read(path.join(mirror, "client", "packages", "core", "src", "instatx.ts"));
  const upstreamPatchExample = read(
    path.join(mirror, "client", "sandbox", "react-nextjs", "pages", "play", "patch.tsx"),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));

  assert.match(upstreamTx, /Similar to `update`, but instead of overwriting/);
  assert.match(upstreamTx, /db\.tx\.goals\[goalId\]\.merge/);
  assert.match(upstreamPatchExample, /\.merge\(\{/);
  assert.match(upstreamPatchExample, /nestedData/);

  assert.match(slice, /details: i\.json\(\)\.optional\(\)/);
  assert.match(slice, /mergeInstantTodoLaunchDetails/);
  assert.match(slice, /\.merge\(\{/);
  assert.match(slice, /touchedAt: Date\.now\(\)/);
  assert.match(slice, /mergeMutation: "mergeInstantTodoLaunchDetails\(todo\)"/);
  assert.match(slice, /nested `\.merge\(\)`/);
  assert.match(slice, />\s*Touch\s*<\/button>/);

  assert.match(registry, /details: i\.json\(\)\.optional\(\)/);
  assert.match(registry, /mergeInstantTodoLaunchDetails/);
  assert.match(registry, /\.merge\(\{/);
});
