const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const receiptPath =
  "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json";

function read(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `missing ${relativePath}`);
  return fs.readFileSync(filePath, "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function sha256(relativePath) {
  return crypto.createHash("sha256").update(read(relativePath)).digest("hex");
}

function escaped(marker) {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

test("Payments billing workflow receipt exposes hash-backed freshness", () => {
  const receipt = readJson(receiptPath);
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = read("examples/template/forge-package-status-read-model.ts");
  const paymentsDxCheck = read("core/src/ecosystem/project_check/payments_dx_check.rs");
  const coreCargo = read("core/Cargo.toml");
  const packageDoc = read("docs/packages/payments-stripe-js.md");

  assert.equal(receipt.package_name, "Payments");
  assert.equal(receipt.package_id, "payments/stripe-js");
  assert.equal(receipt.hash_algorithm, "sha256");
  assert.ok(receipt.files.length >= 3, "receipt must track selected source files");
  assert.ok(
    receipt.hash_exclusions.some(
      (entry) =>
        entry.path === "tools/launch/runtime-template/pages/index.html" &&
        /shared launch runtime/i.test(entry.reason),
    ),
    "shared runtime page must be excluded from Payments-owned hash freshness",
  );

  for (const filePath of receipt.files) {
    assert.equal(
      receipt.file_hashes[filePath],
      sha256(filePath),
      `${filePath} hash is stale in Payments receipt`,
    );
  }

  const paymentsVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "payments/stripe-js",
  );
  assert.ok(paymentsVisibility, "Payments visibility row is missing");

  const selectedSurfaceFileHashes = {};
  for (const surface of paymentsVisibility.selected_surfaces) {
    assert.equal(surface.hash_algorithm, "sha256");
    for (const [filePath, fileHash] of Object.entries(surface.file_hashes ?? {})) {
      selectedSurfaceFileHashes[filePath] = fileHash;
    }
  }
  assert.deepEqual(selectedSurfaceFileHashes, receipt.file_hashes);

  for (const metric of [
    "payments_hash_manifest_present",
    "payments_hash_mismatch",
  ]) {
    assert.ok(
      paymentsVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from Payments visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, escaped(metric));
    assert.match(paymentsDxCheck, escaped(metric));
  }

  for (const marker of [
    "count_hash_mismatches(root, surface)",
    "payments-hash-mismatch",
    "hash_manifest_present",
    "hash_mismatches",
    "hash_project_file_sha256",
    "Sha256",
  ]) {
    assert.match(paymentsDxCheck, escaped(marker), `missing Rust marker ${marker}`);
  }

  assert.match(coreCargo, /sha2/);
  assert.match(packageDoc, /hash_algorithm: sha256/);
  assert.match(packageDoc, /file_hashes/);
  assert.match(packageDoc, /payments_hash_mismatch/);
});
