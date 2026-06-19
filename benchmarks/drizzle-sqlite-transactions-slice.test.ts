const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { test } = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function readSource(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("drizzle sqlite slice exposes a real transaction helper", () => {
  const forge = readSource("core/src/ecosystem/forge_drizzle.rs");
  const wwwTemplate = readSource("examples/template/data-status.tsx");
  const trustPolicy = readSource("core/src/ecosystem/forge_security.rs");

  assert.match(forge, /"js\/db\/drizzle\/transactions\.ts"/);
  assert.match(forge, /const DRIZZLE_TRANSACTIONS_TS/);
  assert.match(forge, /SQLiteTransactionConfig/);
  assert.match(forge, /export function createUserWithPost/);
  assert.match(forge, /db\.transaction\(\(tx\) =>/);
  assert.match(forge, /tx\.insert\(users\)/);
  assert.match(forge, /tx\.insert\(posts\)/);
  assert.match(forge, /publicApi: "db\.transaction"/);
  assert.match(forge, /transactionHelper: "createUserWithPost"/);
  assert.match(forge, /Atomic write helpers stay app-owned when business rules diverge/);

  assert.match(wwwTemplate, /SQLite transactions/);
  assert.match(wwwTemplate, /dxDrizzlePackage\.transactions\.transactionHelper/);

  assert.match(trustPolicy, /db\/drizzle\/transactions\.ts/);
  assert.match(trustPolicy, /transaction boundaries/);
});
