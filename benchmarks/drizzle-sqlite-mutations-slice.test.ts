const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { test } = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function readSource(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("drizzle sqlite slice exposes update and delete mutation helpers", () => {
  const forge = readSource("core/src/ecosystem/forge_drizzle.rs");
  const wwwTemplate = readSource("examples/template/data-status.tsx");
  const trustPolicy = readSource("core/src/ecosystem/forge_security.rs");

  assert.match(forge, /"js\/db\/drizzle\/mutations\.ts"/);
  assert.match(forge, /const DRIZZLE_MUTATIONS_TS/);
  assert.match(forge, /import \{ eq \} from "drizzle-orm"/);
  assert.match(forge, /export function updateUserRole/);
  assert.match(forge, /\.update\(users\)/);
  assert.match(forge, /\.set\(\{/);
  assert.match(forge, /\.where\(eq\(users\.email, input\.email\)\)/);
  assert.match(forge, /\.returning\(\)\.get\(\)/);
  assert.match(forge, /export function publishPost/);
  assert.match(forge, /\.update\(posts\)/);
  assert.match(forge, /status: "published"/);
  assert.match(forge, /export function deletePostBySlug/);
  assert.match(forge, /\.delete\(posts\)/);
  assert.match(forge, /\.where\(eq\(posts\.slug, slug\)\)/);
  assert.match(forge, /mutations: \{/);
  assert.match(forge, /publicApi: "db\.update\/db\.delete returning"/);
  assert.match(forge, /Mutation authorization, audit trail, and soft-delete policy stay app-owned/);

  assert.match(wwwTemplate, /SQLite mutations/);
  assert.match(wwwTemplate, /dxDrizzlePackage\.mutations\.updateUser/);

  assert.match(trustPolicy, /db\/drizzle\/mutations\.ts/);
  assert.match(trustPolicy, /mutation authorization, audit trail, and soft-delete policy/);
});
