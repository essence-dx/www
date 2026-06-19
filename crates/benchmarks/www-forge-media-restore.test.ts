const assert = require("assert");
const crypto = require("crypto");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readBytes(relativePath) {
  return fs.readFileSync(path.join(root, relativePath));
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function sha256(bytes) {
  return crypto.createHash("sha256").update(bytes).digest("hex");
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

test("launch template publishes a media status, restore receipt, and cache-backed restore input", () => {
  const mediaStatusProjectPath = ".dx/forge/media-status.json";
  const restoreReceiptProjectPath =
    ".dx/forge/receipts/media/www-template-favicon-restore.json";

  const sourceBytes = readBytes(
    "tools/launch/runtime-template/assets/favicon.svg",
  );
  const sourceHash = sha256(sourceBytes);
  const expectedCacheProjectPath = `.dx/forge/media-cache/www-template-favicon/sha256-${sourceHash}/favicon.svg`;
  const cacheBytes = readBytes(
    `examples/template/${expectedCacheProjectPath}`,
  );

  assert.deepStrictEqual(cacheBytes, sourceBytes, "media cache must match source");

  const mediaStatus = readJson(
    "examples/template/.dx/forge/media-status.json",
  );
  const restoreReceipt = readJson(
    "examples/template/.dx/forge/receipts/media/www-template-favicon-restore.json",
  );

  assert.strictEqual(mediaStatus.schema, "dx.www.template.forge_media_status");
  assert.strictEqual(restoreReceipt.schema, "forge.media_restore_receipt");
  assert.strictEqual(mediaStatus.summary.asset_count, 1);
  assert.strictEqual(mediaStatus.summary.tracked_asset_count, 1);
  assert.strictEqual(mediaStatus.summary.cached_asset_count, 1);
  assert.strictEqual(mediaStatus.summary.restore_receipt_count, 1);
  assert.strictEqual(mediaStatus.summary.dedupe_key_count, 1);
  assert.ok(mediaStatus.summary.chunk_count >= 1);

  const asset = mediaStatus.assets.find(
    (entry) => entry.asset_id === "www-template-favicon",
  );
  assert.ok(asset, "www-template-favicon asset missing");
  assert.strictEqual(asset.path, "tools/launch/runtime-template/assets/favicon.svg");
  assert.strictEqual(asset.media_type, "image/svg+xml");
  assert.strictEqual(asset.content_hash, sourceHash);
  assert.strictEqual(asset.hash_algorithm, "sha256");
  assert.strictEqual(asset.cache_path, expectedCacheProjectPath);
  assert.strictEqual(asset.restore_receipt_path, restoreReceiptProjectPath);
  assert.strictEqual(asset.preview_receipt, ".dx/forge/package-status.json");
  assert.strictEqual(asset.metadata.dedupe_key, sourceHash);
  assert.match(asset.metadata.forge_chunking_note, /MP4, EXR, UAsset, and CSP/);
  assert.ok(asset.chunk_map.length >= 1, "chunk map missing");
  assert.strictEqual(asset.chunk_map[0].hash, sourceHash);

  assert.strictEqual(restoreReceipt.asset.asset_id, asset.asset_id);
  assert.strictEqual(restoreReceipt.asset.path, asset.path);
  assert.strictEqual(restoreReceipt.asset.content_hash, sourceHash);
  assert.strictEqual(restoreReceipt.cache.cache_path, expectedCacheProjectPath);
  assert.strictEqual(restoreReceipt.cache.cache_hash, sourceHash);
  assert.strictEqual(restoreReceipt.restore_plan.state, "ready-for-review");
  assert.ok(restoreReceipt.restore_plan.steps.length >= 4);
  assert.match(restoreReceipt.boundary, /not a full Git LFS replacement/i);

  const packageStatus = readJson(
    "examples/template/.dx/forge/package-status.json",
  );
  assert.strictEqual(packageStatus.media_status.path, mediaStatusProjectPath);
  assert.strictEqual(
    packageStatus.media_status.restore_receipt_path,
    restoreReceiptProjectPath,
  );
  assert.strictEqual(packageStatus.media_status.cache_path, expectedCacheProjectPath);
  for (const metric of [
    "forge_media_status_present",
    "forge_media_restore_receipts",
    "forge_media_cache_files",
    "forge_media_cache_missing_files",
  ]) {
    assert.ok(packageStatus.dx_check_metrics.includes(metric), `${metric} missing`);
  }

  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  assert.match(readModel, /export type LaunchForgeMediaRestoreStatus/);
  assert.match(readModel, /mediaStatus:/);
  assert.match(readModel, new RegExp(escapeRegExp(restoreReceiptProjectPath)));
  assert.match(readModel, new RegExp(escapeRegExp(expectedCacheProjectPath)));

  const statusSource = read("examples/template/forge-package-status.ts");
  assert.match(statusSource, /mediaStatus: receiptBackedStatus\.mediaStatus/);

  const projectCheckSource = read("core/src/ecosystem/project_check.rs");
  assert.match(projectCheckSource, /MEDIA_STATUS_PATH/);
  assert.match(projectCheckSource, /forge_media_restore_receipts/);
});
