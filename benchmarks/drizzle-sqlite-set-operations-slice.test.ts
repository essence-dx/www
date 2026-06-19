const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { test } = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function readSource(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("drizzle sqlite slice exposes real set operation helpers", () => {
  const forge = readSource("core/src/ecosystem/forge_drizzle.rs");
  const wwwTemplate = readSource("examples/template/data-status.tsx");
  const trustPolicy = readSource("core/src/ecosystem/forge_security.rs");

  assert.match(forge, /"js\/db\/drizzle\/set-operations\.ts"/);
  assert.match(forge, /const DRIZZLE_SET_OPERATIONS_TS/);
  assert.match(forge, /import \{ except, intersect, union, unionAll \} from "drizzle-orm\/sqlite-core"/);
  assert.match(forge, /export function listLaunchAudience/);
  assert.match(forge, /union\(\s*selectAdminAudience\(db\),\s*selectPublishedAuthorAudience\(db\),\s*\)/);
  assert.match(forge, /export function listPublicationCandidates/);
  assert.match(forge, /unionAll\(\s*selectPublishedPostCandidates\(db\),\s*selectDraftPostCandidates\(db\),\s*\)/);
  assert.match(forge, /export function listAuthorsWhoCanPublish/);
  assert.match(forge, /intersect\(\s*selectAuthorsWithPublishedPosts\(db\),\s*selectAdminAuthors\(db\),\s*\)/);
  assert.match(forge, /export function listUnpublishedPostIdentities/);
  assert.match(forge, /except\(\s*selectAllPostIdentities\(db\),\s*selectPublishedPostIdentities\(db\),\s*\)/);
  assert.match(forge, /setOperations: \{/);
  assert.match(forge, /publicApi: "union\/unionAll\/intersect\/except"/);
  assert.match(forge, /Set operation operand order, duplicate policy, and pagination stay app-owned/);

  assert.match(wwwTemplate, /SQLite set ops/);
  assert.match(wwwTemplate, /dxDrizzlePackage\.setOperations\.listAudience/);

  assert.match(trustPolicy, /db\/drizzle\/set-operations\.ts/);
  assert.match(trustPolicy, /set operation operand order, duplicate policy, result ordering, and pagination stay application-owned/);
});
