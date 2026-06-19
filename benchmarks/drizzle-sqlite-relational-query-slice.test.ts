const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("drizzle sqlite slice exposes relational query builder helpers", () => {
  const drizzle = read("core/src/ecosystem/forge_drizzle.rs");
  const dataStatus = read("examples/template/data-status.tsx");

  assert.match(drizzle, /"js\/db\/drizzle\/relational-queries\.ts"/);
  assert.match(drizzle, /DRIZZLE_RELATIONAL_QUERIES_TS/);
  assert.match(drizzle, /db\.query\.users\.findMany/);
  assert.match(drizzle, /with: \{\s*posts: true,\s*\}/s);
  assert.match(drizzle, /db\.query\.posts\.findFirst/);
  assert.match(drizzle, /with: \{\s*author: true,\s*\}/s);
  assert.match(drizzle, /listUsersWithPosts/);
  assert.match(drizzle, /findPostWithAuthor/);
  assert.match(drizzle, /relationalQueries: \{/);
  assert.match(drizzle, /publicApi: "db\.query\.\*\.findMany\/findFirst"/);
  assert.match(drizzle, /Relational query shape stays schema-owned/);

  assert.match(dataStatus, /SQLite relations/);
  assert.match(dataStatus, /dxDrizzlePackage\.relationalQueries\.listUsers/);
});
