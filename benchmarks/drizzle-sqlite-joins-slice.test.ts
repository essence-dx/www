const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { test } = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function readSource(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("drizzle sqlite slice exposes typed join helpers", () => {
  const forge = readSource("core/src/ecosystem/forge_drizzle.rs");
  const wwwTemplate = readSource("examples/template/data-status.tsx");
  const trustPolicy = readSource("core/src/ecosystem/forge_security.rs");

  assert.match(forge, /"js\/db\/drizzle\/joins\.ts"/);
  assert.match(forge, /const DRIZZLE_JOINS_TS/);
  assert.match(forge, /import \{ desc, eq, sql \} from "drizzle-orm"/);
  assert.match(forge, /export function listPublishedPostPreviews/);
  assert.match(forge, /\.from\(posts\)\s+\.innerJoin\(users, eq\(posts\.authorId, users\.id\)\)/);
  assert.match(forge, /export function listUsersWithOptionalPosts/);
  assert.match(forge, /\.from\(users\)\s+\.leftJoin\(posts, eq\(users\.id, posts\.authorId\)\)/);
  assert.match(forge, /joins: \{/);
  assert.match(forge, /publicApi: "select\(\)\.from\(\)\.leftJoin\/innerJoin"/);
  assert.match(forge, /Join shape, null-handling, and cross-table authorization stay app-owned/);

  assert.match(wwwTemplate, /SQLite joins/);
  assert.match(wwwTemplate, /dxDrizzlePackage\.joins\.listPostPreviews/);

  assert.match(trustPolicy, /db\/drizzle\/joins\.ts/);
  assert.match(trustPolicy, /join shape, null-handling, and cross-table authorization stay application-owned/);
});
