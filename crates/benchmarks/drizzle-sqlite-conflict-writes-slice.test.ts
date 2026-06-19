const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { test } = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function readSource(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("drizzle sqlite slice exposes conflict write helpers", () => {
  const forge = readSource("core/src/ecosystem/forge_drizzle.rs");
  const wwwTemplate = readSource("examples/template/data-status.tsx");
  const trustPolicy = readSource("core/src/ecosystem/forge_security.rs");

  assert.match(forge, /"js\/db\/drizzle\/upserts\.ts"/);
  assert.match(forge, /const DRIZZLE_UPSERTS_TS/);
  assert.match(forge, /import \{ sql \} from "drizzle-orm"/);
  assert.match(forge, /export function upsertUserByEmail/);
  assert.match(forge, /\.onConflictDoUpdate\(\{/);
  assert.match(forge, /target: users\.email/);
  assert.match(forge, /sql`excluded\.name`/);
  assert.match(forge, /\.returning\(\)\.get\(\)/);
  assert.match(forge, /export function createUserIfAbsent/);
  assert.match(forge, /\.onConflictDoNothing\(\{ target: users\.email \}\)/);
  assert.match(forge, /export function upsertPostBySlug/);
  assert.match(forge, /target: posts\.slug/);
  assert.match(forge, /conflictWrites: \{/);
  assert.match(forge, /publicApi: "onConflictDoUpdate\/onConflictDoNothing"/);
  assert.match(forge, /Conflict targets and merge policy stay app-owned/);

  assert.match(wwwTemplate, /SQLite conflict writes/);
  assert.match(wwwTemplate, /dxDrizzlePackage\.conflictWrites\.upsertUser/);

  assert.match(trustPolicy, /db\/drizzle\/upserts\.ts/);
  assert.match(trustPolicy, /conflict targets and merge policy/);
});
