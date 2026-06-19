const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { test } = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function readSource(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("drizzle sqlite slice exposes prepared query helpers", () => {
  const forge = readSource("core/src/ecosystem/forge_drizzle.rs");
  const wwwTemplate = readSource("examples/template/data-status.tsx");
  const trustPolicy = readSource("core/src/ecosystem/forge_security.rs");

  assert.match(forge, /"js\/db\/drizzle\/prepared-queries\.ts"/);
  assert.match(forge, /const DRIZZLE_PREPARED_QUERIES_TS/);
  assert.match(forge, /import \{ desc, eq, placeholder \} from "drizzle-orm"/);
  assert.match(forge, /export function prepareUsersByRole/);
  assert.match(forge, /\.where\(eq\(users\.role, placeholder\("role"\)\)\)/);
  assert.match(forge, /\.limit\(placeholder\("limit"\)\)/);
  assert.match(forge, /\.prepare\(\)/);
  assert.match(forge, /export function listPreparedUsersByRole/);
  assert.match(forge, /\.all\(input\)/);
  assert.match(forge, /export function preparePostBySlug/);
  assert.match(forge, /\.where\(eq\(posts\.slug, placeholder\("slug"\)\)\)/);
  assert.match(forge, /export function getPreparedPostBySlug/);
  assert.match(forge, /\.get\(input\)/);
  assert.match(forge, /publicApi: "placeholder\(\) \+ \.prepare\(\)"/);
  assert.match(forge, /Prepared statement lifetime and invalidation stay app-owned/);

  assert.match(wwwTemplate, /SQLite prepared/);
  assert.match(wwwTemplate, /dxDrizzlePackage\.preparedQueries\.listUsers/);

  assert.match(trustPolicy, /db\/drizzle\/prepared-queries\.ts/);
  assert.match(trustPolicy, /prepared statement lifetime/);
});
