const assert = require("node:assert/strict");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const packageStatusPath =
  "examples/template/.dx/forge/package-status.json";
const readModelPath = "examples/template/forge-package-status-read-model.ts";

const receiptCases = [
  {
    packageId: "automations/n8n",
    helperPath: "examples/template/automation-connectors-receipt-hashes.ts",
    receiptPath:
      "examples/template/.dx/forge/receipts/2026-05-22-automation-connectors-launch-workflow.json",
    zedVisibility: "automation-connectors:receipt-hash-refresh",
  },
  {
    packageId: "db/drizzle-sqlite",
    helperPath: "examples/template/database-orm-receipt-hashes.ts",
    receiptPath:
      "examples/template/.dx/forge/receipts/2026-05-22-db-drizzle-sqlite-dashboard-workflow.json",
    zedVisibility: "database-orm:receipt-hash-refresh",
  },
  {
    packageId: "forms/react-hook-form",
    helperPath: "examples/template/forms-receipt-hashes.ts",
    receiptPath:
      "examples/template/.dx/forge/receipts/2026-05-22-forms-dashboard-workflow.json",
    zedVisibility: "forms:receipt-hash-refresh",
  },
  {
    packageId: "payments/stripe-js",
    helperPath: "examples/template/payments-receipt-hashes.ts",
    receiptPath:
      "examples/template/.dx/forge/receipts/2026-05-22-payments-stripe-js-billing-workflow.json",
    zedVisibility: "payments:receipt-hash-refresh",
  },
];

test("Forge dashboard receipts stay current without runtime proof overclaims", () => {
  const packageStatus = readJson(packageStatusPath);
  const readModel = read(readModelPath);

  for (const receiptCase of receiptCases) {
    const report = runReceiptHelper(receiptCase.helperPath);
    assert.equal(report.status, "current", receiptCase.packageId);
    assert.equal(report.receipt_path, receiptCase.receiptPath);
    assert.equal(report.runtime_execution, false);
    assert.equal(report.secret_access, false);
    assert.deepEqual(report.stale_files ?? [], []);
    assert.deepEqual(report.missing_files ?? [], []);
    assert.deepEqual(report.stale_mirror_files ?? [], []);
    assert.deepEqual(report.missing_mirror_files ?? [], []);

    const receipt = readJson(receiptCase.receiptPath);
    assert.equal(receipt.package_id, receiptCase.packageId);
    assert.equal(receipt.hash_algorithm, "sha256");
    assertNoRuntimeProofOverclaim(receipt, receiptCase.packageId);
    assertRootFileHashesMatch(receipt, receiptCase.packageId);

    assert.ok(
      packageStatus.zed_receipt_surfaces.includes(receiptCase.zedVisibility),
      `${receiptCase.zedVisibility} missing from package-status`,
    );
    assert.match(readModel, new RegExp(escapeRegex(receiptCase.zedVisibility)));
  }
});

function runReceiptHelper(helperPath) {
  const result = spawnSync(
    process.execPath,
    [
      "tools/launch/run-template-receipt-helper.js",
      helperPath,
      "--check",
      "--json",
    ],
    {
      cwd: root,
      encoding: "utf8",
      windowsHide: true,
    },
  );
  assert.equal(result.status, 0, result.stdout + result.stderr);
  return JSON.parse(result.stdout);
}

function assertRootFileHashesMatch(receipt, packageId) {
  assert.ok(receipt.file_hashes, `${packageId} receipt is missing file_hashes`);
  for (const [relativePath, expectedHash] of fileHashEntries(receipt.file_hashes)) {
    assert.match(expectedHash, /^[a-f0-9]{64}$/, relativePath);
    assert.equal(sha256(relativePath), expectedHash, `${packageId}: ${relativePath}`);
  }
}

function fileHashEntries(fileHashes) {
  if (Array.isArray(fileHashes)) {
    return fileHashes.map((entry) => [entry.path, entry.sha256]);
  }
  return Object.entries(fileHashes);
}

function assertNoRuntimeProofOverclaim(receipt, packageId) {
  if (Object.prototype.hasOwnProperty.call(receipt, "runtime_execution")) {
    assert.equal(receipt.runtime_execution, false, packageId);
  }
  if (Object.prototype.hasOwnProperty.call(receipt, "no_runtime_execution")) {
    assert.equal(receipt.no_runtime_execution, true, packageId);
  }

  const serialized = JSON.stringify(receipt);
  assert.doesNotMatch(serialized, /"runtime_execution"\s*:\s*true/, packageId);
  assert.doesNotMatch(serialized, /"runtime_proof"\s*:\s*true/, packageId);
  assert.doesNotMatch(serialized, /"secret_access"\s*:\s*true/, packageId);
  assert.match(serialized, /(SOURCE-ONLY|ADAPTER-BOUNDARY)/, packageId);
}

function sha256(relativePath) {
  return crypto
    .createHash("sha256")
    .update(fs.readFileSync(path.join(root, relativePath)))
    .digest("hex");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function escapeRegex(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
