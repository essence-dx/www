const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("Realtime App Database slice materializes the upstream InstantDB Sync Table surface", () => {
  const upstreamReact = read(
    path.join(mirror, "client", "packages", "react", "src", "index.ts"),
  );
  const upstreamCore = read(
    path.join(mirror, "client", "packages", "core", "src", "index.ts"),
  );
  const upstreamSandbox = read(
    path.join(
      mirror,
      "client",
      "sandbox",
      "react-nextjs",
      "pages",
      "play",
      "sync-table.tsx",
    ),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));
  const catalog = read(
    path.join(root, "examples", "template", "package-catalog.ts"),
  );
  const dashboardModel = read(
    path.join(root, "examples", "dashboard", "src", "lib", "instantdbDashboard.ts"),
  );
  const packageDoc = read(path.join(root, "docs", "packages", "instantdb-react.md"));
  const workflowReceipt = read(
    path.join(
      root,
      "examples",
      "www-template",
      ".dx",
      "forge",
      "receipts",
      "2026-05-22-instantdb-realtime-dashboard.json",
    ),
  );

  assert.match(upstreamReact, /SyncTableCallbackEventType/);
  assert.match(upstreamReact, /type SyncTableCallbackEvent/);
  assert.match(upstreamReact, /StoreInterface/);
  assert.match(upstreamCore, /_syncTableExperimental<Q extends ValidQuery<Q, Schema>>/);
  assert.match(upstreamSandbox, /db\.core\._syncTableExperimental/);
  assert.match(upstreamSandbox, /SyncTableCallbackEventType\.InitialSyncBatch/);
  assert.match(upstreamSandbox, /SyncTableCallbackEventType\.SyncTransaction/);
  assert.match(upstreamSandbox, /SyncTableCallbackEventType\.Error/);

  assert.match(slice, /"js\/instant\/sync-table\.ts"/);
  assert.match(slice, /INSTANTDB_SYNC_TABLE_TS/);
  assert.match(slice, /SyncTableCallbackEventType/);
  assert.match(slice, /type SyncTableCallbackEvent/);
  assert.match(slice, /type StoreInterfaceStoreName/);
  assert.match(slice, /subscribeInstantLaunchSyncTable/);
  assert.match(slice, /db\.core\._syncTableExperimental\(\s*query,\s*callback,/);
  assert.match(slice, /summarizeInstantLaunchSyncTableEvent/);
  assert.match(slice, /syncTable: "subscribeInstantLaunchSyncTable/);
  assert.match(slice, /experimental Sync Table subscriptions/);

  assert.match(registry, /lib\/instant\/sync-table\.ts/);
  assert.match(registry, /subscribeInstantLaunchSyncTable/);
  assert.match(catalog, /lib\/instant\/sync-table\.ts/);
  assert.match(catalog, /SyncTableCallbackEventType/);
  assert.match(dashboardModel, /lib\/instant\/sync-table\.ts/);
  assert.match(dashboardModel, /db\.core\._syncTableExperimental/);
  assert.match(packageDoc, /Sync Table/);
  assert.match(packageDoc, /subscribeInstantLaunchSyncTable/);
  assert.match(workflowReceipt, /"lib\/instant\/sync-table\.ts"/);
});
