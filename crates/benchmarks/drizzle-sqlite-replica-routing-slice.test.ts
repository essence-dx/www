const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const sourceMirror = "G:/WWW/inspirations/drizzle-orm";

function readSource(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readMirror(relativePath) {
  return fs.readFileSync(path.join(sourceMirror, relativePath), "utf8");
}

test("Database ORM slice exposes upstream SQLite replica routing as an app-owned boundary", () => {
  const upstreamDb = readMirror("drizzle-orm/src/sqlite-core/db.ts");
  const forge = readSource("core/src/ecosystem/forge_drizzle.rs");
  const packageDoc = readSource("docs/packages/db-drizzle-sqlite.md");
  const packageCatalog = readSource("examples/template/package-catalog.ts");

  assert.match(upstreamDb, /export const withReplicas = </);
  assert.match(upstreamDb, /const selectDistinct: Q\['selectDistinct'\]/);
  assert.match(upstreamDb, /const \$count: Q\['\$count'\]/);
  assert.match(upstreamDb, /update,\s*\n\t\tinsert,\s*\n\t\tdelete: \$delete/);

  assert.match(forge, /"js\/db\/drizzle\/replicas\.ts"/);
  assert.match(forge, /const DRIZZLE_REPLICAS_TS/);
  assert.match(forge, /import \{ withReplicas \} from "drizzle-orm\/sqlite-core"/);
  assert.match(forge, /export type DxDrizzleReplicaRoutingStatus = "missing-replica" \| "configured"/);
  assert.match(forge, /export function createDxDrizzleReplicaSet/);
  assert.match(forge, /withReplicas\(primary, replicas, chooseReplica\)/);
  assert.match(forge, /export function readDxDrizzleReplicaReadiness/);
  assert.match(forge, /readApis: \["select", "selectDistinct", "\$count", "with", "query"\]/);
  assert.match(forge, /writeApis: \["insert", "update", "delete", "transaction", "run"\]/);
  assert.match(forge, /officialPackageName: "Database ORM"/);
  assert.match(forge, /"db\/drizzle\/replicas\.ts"/);
  assert.match(forge, /"createDxDrizzleReplicaSet"/);
  assert.match(forge, /"readDxDrizzleReplicaReadiness"/);
  assert.match(forge, /"withReplicas"/);
  assert.match(forge, /"selectDistinct"/);
  assert.match(forge, /"\$count"/);
  assert.match(forge, /replicaRouting: \{/);
  assert.match(forge, /publicApi: "withReplicas"/);
  assert.match(forge, /Read-replica topology, replica health, routing policy, and write-after-read consistency stay app-owned/);

  assert.match(packageDoc, /# Database ORM/);
  assert.match(packageDoc, /Package id: `db\/drizzle-sqlite`/);
  assert.match(packageDoc, /Upstream package: `drizzle-orm`/);
  assert.match(packageDoc, /`withReplicas`/);
  assert.match(packageDoc, /`selectDistinct`/);
  assert.match(packageDoc, /`\$count`/);
  assert.match(packageDoc, /read replicas/i);
  assert.match(packageDoc, /Read-replica topology, replica health, routing policy, and write-after-read consistency/);

  assert.match(packageCatalog, /officialPackageName: "Database ORM"/);
  assert.match(packageCatalog, /name: "Database ORM Dashboard Workflow"/);
  assert.match(packageCatalog, /db\/drizzle\/replicas\.ts/);
  assert.match(packageCatalog, /withReplicas/);
});
