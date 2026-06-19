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

function trackedPathSet(status) {
  return new Set(status.tracked_files.map((file) => file.path));
}

test("launch template publishes a real Forge VCS status and snapshot receipt", () => {
  const statusProjectPath = ".dx/forge/vcs-status.json";
  const snapshotProjectPath =
    ".dx/forge/receipts/vcs/www-template-snapshot.json";
  const statusPath = "examples/template/.dx/forge/vcs-status.json";
  const snapshotPath =
    "examples/template/.dx/forge/receipts/vcs/www-template-snapshot.json";
  const vcsStatus = readJson(statusPath);
  const snapshotReceipt = readJson(snapshotPath);

  assert.strictEqual(
    vcsStatus.schema,
    "dx.www.template.forge_vcs_status",
  );
  assert.strictEqual(snapshotReceipt.schema, "forge.vcs_snapshot_receipt");
  assert.strictEqual(vcsStatus.status, "clean");
  assert.match(vcsStatus.snapshot_id, /^[a-f0-9]{64}$/);
  assert.strictEqual(snapshotReceipt.snapshot_id, vcsStatus.snapshot_id);
  assert.strictEqual(vcsStatus.snapshot_receipt_path, snapshotProjectPath);
  assert.strictEqual(snapshotReceipt.status_path, statusProjectPath);

  assert.ok(
    vcsStatus.summary.tracked_file_count >= 60,
    "VCS receipt should cover package files plus www-template source and media",
  );
  assert.strictEqual(
    vcsStatus.summary.tracked_file_count,
    vcsStatus.tracked_files.length,
  );
  assert.strictEqual(vcsStatus.summary.missing_file_count, 0);
  assert.strictEqual(vcsStatus.summary.modified_file_count, 0);
  assert.ok(vcsStatus.summary.media_file_count >= 1);
  assert.ok(vcsStatus.summary.code_file_count >= 1);
  assert.deepStrictEqual(snapshotReceipt.summary, vcsStatus.summary);

  for (const source of [
    "forge-package-lock",
    "forge-media-status",
    "www-template-source-surface",
  ]) {
    assert.ok(
      vcsStatus.tracked_file_sources.includes(source),
      `${source} missing from tracked file sources`,
    );
    assert.ok(
      snapshotReceipt.tracked_file_sources.includes(source),
      `${source} missing from snapshot tracked file sources`,
    );
  }

  const paths = trackedPathSet(vcsStatus);
  for (const requiredPath of [
    "components/ui/button.tsx",
    "lib/forge/state/zustand/index.ts",
    "lib/query/client.ts",
    "template-shell.tsx",
    "package-catalog.ts",
    "tools/launch/runtime-template/pages/index.html",
    "tools/launch/runtime-template/assets/favicon.svg",
  ]) {
    assert.ok(paths.has(requiredPath), `${requiredPath} missing from VCS status`);
  }

  for (const file of vcsStatus.tracked_files) {
    assert.strictEqual(file.exists, true, `${file.path} should exist`);
    assert.strictEqual(file.state, "clean", `${file.path} should be clean`);
    assert.match(file.content_hash, /^[a-f0-9]{64}$/);
    assert.strictEqual(file.hash_algorithm, "sha256");
  }

  assert.ok(
    snapshotReceipt.restore_plan.steps.some((step) =>
      /operator review/i.test(step),
    ),
    "snapshot receipt should include an operator-reviewed restore plan",
  );
  assert.match(snapshotReceipt.boundary, /not a full Forge commit graph/i);

  const packageStatus = readJson(
    "examples/template/.dx/forge/package-status.json",
  );
  assert.strictEqual(packageStatus.vcs_status.path, statusProjectPath);
  assert.strictEqual(packageStatus.vcs_status.snapshot_id, vcsStatus.snapshot_id);
  assert.strictEqual(
    packageStatus.vcs_status.snapshot_receipt_path,
    snapshotProjectPath,
  );
  assert.ok(
    packageStatus.dx_check_metrics.includes("forge_vcs_snapshot_present"),
  );
  assert.ok(
    packageStatus.dx_check_metrics.includes(
      "forge_vcs_snapshot_receipt_present",
    ),
  );

  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  assert.match(readModel, /export type LaunchForgeVcsStatus/);
  assert.match(readModel, /vcsStatus:/);
  assert.match(readModel, /trackedFileCount:/);
  assert.match(readModel, /snapshotReceiptPath:/);

  const statusSource = read("examples/template/forge-package-status.ts");
  assert.match(statusSource, /vcsStatus: receiptBackedStatus\.vcsStatus/);
});
