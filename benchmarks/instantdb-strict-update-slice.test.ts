const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes strict update transactions for existing records", () => {
  const upstreamTx = read(path.join(mirror, "client", "packages", "core", "src", "instatx.ts"));
  const upstreamTodoExample = read(
    path.join(mirror, "client", "sandbox", "react-nextjs", "pages", "play", "todo.jsx"),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));

  assert.match(upstreamTx, /strict update/);
  assert.match(upstreamTx, /\{upsert: false\}/);
  assert.match(upstreamTodoExample, /update\(\{ done: !todo\.done \}, \{ upsert: false \}\)/);
  assert.match(upstreamTodoExample, /todos\.map\(\(todo\) =>/);

  assert.match(slice, /toggleInstantTodo/);
  assert.match(slice, /update\(\{ done: !todo\.done \}, \{ upsert: false \}\)/);
  assert.match(slice, /export function toggleAllInstantTodos/);
  assert.match(slice, /todos\.map\(\(todo\) => db\.tx\.todos\[todo\.id\]\.update\(\{ done \}, \{ upsert: false \}\)\)/);
  assert.match(slice, /"db\.tx\.\*\.update\(\.\.\., \{ upsert: false \}\)"/);
  assert.match(slice, /strictUpdateMutation: "toggleInstantTodo\(todo\)"/);
  assert.match(slice, /bulkStrictUpdateMutation: "toggleAllInstantTodos\(todos\)"/);
  assert.match(slice, />\s*Toggle all\s*<\/button>/);

  assert.match(registry, /mutations\.contains\("\{ upsert: false \}"\)/);
  assert.match(registry, /mutations\.contains\("toggleAllInstantTodos"\)/);
});
