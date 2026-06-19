const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes a typed permissions starter from upstream rules API", () => {
  const upstreamReact = read(path.join(mirror, "client", "packages", "react", "src", "index.ts"));
  const upstreamExample = read(path.join(mirror, "examples", "next-js-app-dir", "src", "instant.perms.ts"));
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));

  assert.match(upstreamReact, /type InstantRules/);
  assert.match(upstreamExample, /import type \{ InstantRules \} from "@instantdb\/react"/);
  assert.match(upstreamExample, /satisfies InstantRules/);

  assert.match(slice, /"js\/instant\/perms\.ts"/);
  assert.match(slice, /import type \{ InstantRules \} from "@instantdb\/react"/);
  assert.match(slice, /todos: \{/);
  assert.match(slice, /\$files: \{/);
  assert.match(slice, /satisfies InstantRules/);
  assert.match(slice, /permissions: "rules satisfies InstantRules"/);
  assert.match(slice, /deployed product rules/);

  assert.match(registry, /lib\/instant\/perms\.ts/);
  assert.match(registry, /assert_eq!\(paths\.len\(\), 25\)/);
});
