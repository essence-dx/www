const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

test("launch template publishes a safe multi-remote status and sync-plan receipt", () => {
  const statusProjectPath = ".dx/forge/remote-status.json";
  const planProjectPath =
    ".dx/forge/receipts/remotes/www-template-sync-plan.json";
  const remoteStatus = readJson(
    "examples/template/.dx/forge/remote-status.json",
  );
  const syncPlan = readJson(
    "examples/template/.dx/forge/receipts/remotes/www-template-sync-plan.json",
  );

  assert.strictEqual(
    remoteStatus.schema,
    "dx.www.template.forge_remote_status",
  );
  assert.strictEqual(syncPlan.schema, "forge.remote_sync_plan_receipt");
  assert.match(remoteStatus.plan_id, /^[a-f0-9]{64}$/);
  assert.strictEqual(syncPlan.plan_id, remoteStatus.plan_id);
  assert.strictEqual(remoteStatus.sync_plan_receipt_path, planProjectPath);
  assert.strictEqual(syncPlan.status_path, statusProjectPath);

  assert.strictEqual(remoteStatus.summary.remote_count, 5);
  assert.strictEqual(remoteStatus.summary.safe_remote_count, 5);
  assert.strictEqual(remoteStatus.summary.unsafe_remote_count, 0);
  assert.strictEqual(remoteStatus.summary.plaintext_secret_count, 0);
  assert.strictEqual(remoteStatus.summary.executable_remote_count, 1);
  assert.strictEqual(remoteStatus.summary.boundary_remote_count, 4);
  assert.strictEqual(remoteStatus.summary.local_provider_ready, true);

  const kinds = new Set(remoteStatus.remotes.map((remote) => remote.kind));
  for (const kind of [
    "local-filesystem",
    "git-compatible",
    "s3-compatible",
    "database-backed",
    "forge-remote-adapter",
  ]) {
    assert.ok(kinds.has(kind), `${kind} remote missing`);
  }

  const local = remoteStatus.remotes.find((remote) => remote.name === "local-cache");
  assert.ok(local, "local-cache remote missing");
  assert.strictEqual(local.executable_now, true);
  assert.strictEqual(local.health, "ready");
  assert.strictEqual(local.sync_state, "local-ready");
  assert.strictEqual(local.secret_policy, "no-plaintext-secrets");

  for (const remote of remoteStatus.remotes) {
    assert.strictEqual(remote.secrets_safe, true, `${remote.name} not safe`);
    assert.doesNotMatch(remote.locator, /token=|secret=|password=|@/i);
    assert.ok(remote.capabilities.length > 0, `${remote.name} missing capabilities`);
  }

  assert.strictEqual(syncPlan.status, "partial");
  assert.strictEqual(syncPlan.summary.actions, 5);
  assert.strictEqual(syncPlan.summary.executable_actions, 1);
  assert.strictEqual(syncPlan.summary.boundary_actions, 4);
  assert.ok(
    syncPlan.actions.some(
      (action) =>
        action.remote === "local-cache" && action.state === "ready-to-execute",
    ),
    "sync plan should include a ready local-cache action",
  );
  assert.ok(
    syncPlan.actions.every((action) => action.secrets_safe === true),
    "sync actions must not expose plaintext secrets",
  );
  assert.match(syncPlan.boundary, /no live network sync is claimed/i);

  const packageStatus = readJson(
    "examples/template/.dx/forge/package-status.json",
  );
  assert.strictEqual(packageStatus.remote_status.path, statusProjectPath);
  assert.strictEqual(packageStatus.remote_status.plan_id, remoteStatus.plan_id);
  assert.strictEqual(
    packageStatus.remote_status.sync_plan_receipt_path,
    planProjectPath,
  );
  for (const metric of [
    "forge_remote_status_present",
    "forge_remote_sync_plan_present",
    "forge_remote_safe_count",
    "forge_remote_boundary_count",
  ]) {
    assert.ok(packageStatus.dx_check_metrics.includes(metric), `${metric} missing`);
  }

  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  assert.match(readModel, /export type LaunchForgeRemoteSyncStatus/);
  assert.match(readModel, /remoteStatus:/);
  assert.match(readModel, new RegExp(escapeRegExp(planProjectPath)));

  const statusSource = read("examples/template/forge-package-status.ts");
  assert.match(statusSource, /remoteStatus: receiptBackedStatus\.remoteStatus/);
});
