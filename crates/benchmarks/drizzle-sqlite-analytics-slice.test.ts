const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { test } = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function readSource(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("drizzle sqlite slice exposes aggregate analytics helpers", () => {
  const forge = readSource("core/src/ecosystem/forge_drizzle.rs");
  const wwwTemplate = readSource("examples/template/data-status.tsx");
  const trustPolicy = readSource("core/src/ecosystem/forge_security.rs");

  assert.match(forge, /"js\/db\/drizzle\/analytics\.ts"/);
  assert.match(forge, /const DRIZZLE_ANALYTICS_TS/);
  assert.match(forge, /import \{ avg, count, countDistinct, eq, sql \} from "drizzle-orm"/);
  assert.match(forge, /export function readLaunchDatabaseStats/);
  assert.match(forge, /db\.select\(\{ value: count\(\) \}\)\.from\(users\)\.get\(\)/);
  assert.match(forge, /db\.select\(\{ value: countDistinct\(users\.role\) \}\)/);
  assert.match(forge, /db\.select\(\{ value: avg\(posts\.id\) \}\)\.from\(posts\)\.get\(\)/);
  assert.match(forge, /sql<number>`count\(\*\) filter \(where \$\{posts\.status\} = 'published'\)`/);
  assert.match(forge, /analytics: \{/);
  assert.match(forge, /publicApi: "count\/countDistinct\/avg\/sql aggregate"/);
  assert.match(forge, /Analytics definitions and business KPIs stay app-owned/);

  assert.match(wwwTemplate, /SQLite analytics/);
  assert.match(wwwTemplate, /dxDrizzlePackage\.analytics\.readStats/);

  assert.match(trustPolicy, /db\/drizzle\/analytics\.ts/);
  assert.match(trustPolicy, /business KPI definitions stay application-owned/);
});
