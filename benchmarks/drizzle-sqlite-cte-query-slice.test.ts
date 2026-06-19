const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { test } = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function readSource(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("drizzle sqlite slice exposes CTE and subquery helpers", () => {
  const forge = readSource("core/src/ecosystem/forge_drizzle.rs");
  const wwwTemplate = readSource("examples/template/data-status.tsx");
  const trustPolicy = readSource("core/src/ecosystem/forge_security.rs");

  assert.match(forge, /"js\/db\/drizzle\/cte-queries\.ts"/);
  assert.match(forge, /const DRIZZLE_CTE_QUERIES_TS/);
  assert.match(forge, /export function listAuthorsWithPostCounts/);
  assert.match(forge, /db\.\$with\("post_counts"\)\.as\(/);
  assert.match(forge, /sql<number>`count\(\$\{posts\.id\}\)`\.as\("post_count"\)/);
  assert.match(forge, /\.groupBy\(posts\.authorId\)/);
  assert.match(forge, /db\s*\.\s*with\(postCounts\)/);
  assert.match(forge, /\.leftJoin\(postCounts, eq\(users\.id, postCounts\.authorId\)\)/);
  assert.match(forge, /export function listRecentPublishedPostSlugs/);
  assert.match(forge, /\.as\("recent_published_posts"\)/);
  assert.match(forge, /\.from\(recentPublishedPosts\)/);
  assert.match(forge, /cteQueries: \{/);
  assert.match(forge, /publicApi: "db\.\$with\/db\.with\/subquery\.as"/);
  assert.match(forge, /CTE names, SQL aliases, aggregation semantics, and subquery pagination stay app-owned/);

  assert.match(wwwTemplate, /SQLite CTEs/);
  assert.match(wwwTemplate, /dxDrizzlePackage\.cteQueries\.listAuthorsWithPostCounts/);

  assert.match(trustPolicy, /db\/drizzle\/cte-queries\.ts/);
  assert.match(trustPolicy, /CTE names, SQL aliases, aggregation semantics, and subquery pagination stay application-owned/);
});
