const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { test } = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function readSource(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("drizzle sqlite slice exposes typed SQLite views", () => {
  const forge = readSource("core/src/ecosystem/forge_drizzle.rs");
  const wwwTemplate = readSource("examples/template/data-status.tsx");
  const trustPolicy = readSource("core/src/ecosystem/forge_security.rs");

  assert.match(forge, /"js\/db\/drizzle\/views\.ts"/);
  assert.match(forge, /const DRIZZLE_VIEWS_TS/);
  assert.match(forge, /getViewSelectedFields/);
  assert.match(forge, /sqliteView/);
  assert.match(forge, /export const publishedPostSummaries = sqliteView\("published_post_summaries"\)/);
  assert.match(forge, /\.innerJoin\(users, eq\(posts\.authorId, users\.id\)\)/);
  assert.match(forge, /bodyLength: sql<number>`length\(\$\{posts\.body\}\)`\.as\("body_length"\)/);
  assert.match(forge, /export const existingPublishedPostSummaries = sqliteView\("existing_published_post_summaries"/);
  assert.match(forge, /\.existing\(\)/);
  assert.match(forge, /export type PublishedPostSummary = typeof publishedPostSummaries\.\$inferSelect/);
  assert.match(forge, /export function listPublishedPostSummaries/);
  assert.match(forge, /\.from\(publishedPostSummaries\)/);
  assert.match(forge, /export function readPublishedPostSummaryFields/);
  assert.match(forge, /Object\.keys\(getViewSelectedFields\(publishedPostSummaries\)\)/);
  assert.match(forge, /views: \{/);
  assert.match(forge, /publicApi: "sqliteView\/sqliteView.existing\/getViewSelectedFields"/);
  assert.match(forge, /View SQL definitions, migration lifecycle, and compatibility with existing database views stay app-owned/);

  assert.match(wwwTemplate, /SQLite views/);
  assert.match(wwwTemplate, /dxDrizzlePackage\.views\.listPublishedPostSummaries/);

  assert.match(trustPolicy, /db\/drizzle\/views\.ts/);
  assert.match(trustPolicy, /View SQL definitions, migration lifecycle, and compatibility with existing database views stay application-owned/);
});
