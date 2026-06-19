const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-instantdb-realtime-dashboard.json";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function sha256(relativePath) {
  return crypto.createHash("sha256").update(read(relativePath)).digest("hex");
}

test("Realtime App Database exposes receipt hash evidence to package-status and Rust dx-check", () => {
  const receipt = readJson(receiptPath);
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = read("examples/template/forge-package-status-read-model.ts");
  const packageCatalog = read("examples/template/package-catalog.ts");
  const packageDoc = read("docs/packages/instantdb-react.md");
  const rustHelper = read(
    "core/src/ecosystem/project_check/realtime_app_database_dx_check.rs",
  );
  const upstreamReactIndex = fs.readFileSync(
    path.resolve(
      root,
      "..",
      "..",
      "WWW/inspirations/instantdb/client/packages/react/src/index.ts",
    ),
    "utf8",
  );
  const upstreamCoreIndex = fs.readFileSync(
    path.resolve(
      root,
      "..",
      "..",
      "WWW/inspirations/instantdb/client/packages/core/src/index.ts",
    ),
    "utf8",
  );

  assert.match(upstreamReactIndex, /SyncTableCallbackEventType/);
  assert.match(upstreamCoreIndex, /_syncTableExperimental/);

  assert.equal(receipt.package_id, "instantdb/react");
  assert.equal(receipt.package_name, "Realtime App Database");
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.ok(Array.isArray(receipt.files), "receipt files manifest is missing");
  assert.ok(receipt.file_hashes, "receipt file_hashes manifest is missing");

  for (const filePath of receipt.files) {
    assert.equal(
      receipt.file_hashes[filePath],
      sha256(filePath),
      `${filePath} hash is stale in the Realtime App Database receipt`,
    );
  }

  const visibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "instantdb/react",
  );
  assert.ok(visibility, "Realtime App Database package-status row is missing");

  const packageStatusHashedFiles = new Set();
  for (const surfaceId of [
    "instantdb-runtime-dashboard-workflow",
    "dashboard-instantdb-workflow",
    "sync-table-events",
  ]) {
    const surface = visibility.selected_surfaces.find(
      (candidate) => candidate.surface_id === surfaceId,
    );
    assert.ok(surface, `${surfaceId} is missing from package-status`);
    assert.equal(surface.hash_algorithm, "sha256");
    assert.ok(
      surface.file_hashes && Object.keys(surface.file_hashes).length > 0,
      `${surfaceId} is missing hash-backed file evidence`,
    );
    for (const [filePath, digest] of Object.entries(surface.file_hashes)) {
      assert.equal(
        digest,
        receipt.file_hashes[filePath],
        `${surfaceId} has stale or receipt-divergent hash evidence for ${filePath}`,
      );
      packageStatusHashedFiles.add(filePath);
    }
  }

  for (const filePath of receipt.files) {
    assert.ok(
      packageStatusHashedFiles.has(filePath),
      `${filePath} is missing from package-status selected-surface hash evidence`,
    );
  }

  for (const metric of [
    "realtime_app_database_hash_manifest_present",
    "realtime_app_database_hash_mismatch",
  ]) {
    assert.ok(
      visibility.dx_check_metrics.includes(metric),
      `${metric} missing from Realtime App Database package-status row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
    assert.match(rustHelper, new RegExp(metric));
    assert.match(packageDoc, new RegExp(metric));
  }

  assert.match(packageCatalog, /receiptIntegrity: \{/);
  assert.match(packageCatalog, /trackedFiles: \[/);
  assert.match(packageCatalog, /hashAlgorithm: "sha256"/);
  assert.match(rustHelper, /use super::file_hashes::count_sha256_file_hash_mismatches/);
  assert.match(rustHelper, /hash_manifest_present = 1/);
  assert.match(
    rustHelper,
    /hash_mismatches \+= count_sha256_file_hash_mismatches\(root, surface\)/,
  );
  assert.match(rustHelper, /realtime-app-database-hash-mismatch/);
});
