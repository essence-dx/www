const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("InstantDB slice exposes real auth and client capability APIs", () => {
  const slice = read("core/src/ecosystem/forge_instantdb.rs");
  const launchProof = read("examples/template/instantdb-status.tsx");

  assert.match(slice, /db\.useAuth/);
  assert.match(slice, /db\.auth/);
  assert.match(slice, /db\.storage/);
  assert.match(slice, /db\.streams/);
  assert.match(slice, /db\.transact\(\[\.\.\.\]\)/);
  assert.match(slice, /clearCompletedInstantTodos/);
  assert.match(slice, /db\.transact\(chunks\)/);
  assert.match(slice, /createDxInstantCapabilities/);
  assert.match(slice, /authState: "db\.useAuth\(\)"/);
  assert.match(slice, /clientCapabilities: "db\.auth \+ db\.storage \+ db\.streams"/);
  assert.match(slice, /Instant auth flows/);

  assert.match(launchProof, /db\.useAuth\(\)/);
  assert.match(launchProof, /data-dx-instant-auth/);
});
