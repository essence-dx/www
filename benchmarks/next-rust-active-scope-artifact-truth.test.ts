import assert from "node:assert/strict";
import { createRequire } from "node:module";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const require = createRequire(import.meta.url);
const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const {
  buildActiveScopeReportFromBoundary,
  buildActiveScopeSummary,
} = require("../tools/vendor/next-rust-boundary/active-scope.js");

function readJson(relativePath: string) {
  return JSON.parse(fs.readFileSync(path.join(repoRoot, relativePath), "utf8"));
}

test("checked-in Next/Turbopack receipts carry active-scope artifact truth", () => {
  const sourceReceipt = readJson(".dx/receipts/next-rust/vendor-boundary.json");
  const consumerReceipt = readJson(
    ".dx/receipts/next-rust/vendor-boundary-consumer.json",
  );
  const activeScope = buildActiveScopeReportFromBoundary(
    consumerReceipt.status,
    consumerReceipt.snapshot.boundary,
  );
  const activeScopeSummary = buildActiveScopeSummary(activeScope);

  assert.equal(sourceReceipt.schema, "dx.nextRust.vendorBoundary");
  assert.equal(sourceReceipt.status, "ok");

  assert.equal(
    consumerReceipt.schema,
    "dx.nextRust.vendorBoundary.consumerReceipt",
  );
  assert.equal(consumerReceipt.status, "ok");
  assert.equal(consumerReceipt.snapshot.sourceReceipt.fresh, true);
  assert.equal(consumerReceipt.snapshot.sourceReceipt.mismatchCount, 0);
  assert.equal(consumerReceipt.snapshot.boundary.removedTargetsBlocked, true);
  assert.equal(
    consumerReceipt.snapshot.boundary.excludedRuntimeTargetsBlocked,
    true,
  );

  assert.equal(activeScope.status, "ok");
  assert.equal(activeScope.referenceOnlyNextRust, true);
  assert.equal(activeScope.runtimeBuildAdoption, false);
  assert.equal(activeScope.turbopackPublicArchitecture, false);
  assert.equal(activeScope.removedTargetsBlocked, true);
  assert.equal(activeScope.excludedRuntimeTargetsBlocked, true);
  assert.equal(activeScope.devFeedbackEndpoint, "/_dx/feedback");
  assert.deepEqual(activeScope.publicClaimChecks, [
    "next-devtools-clone-target",
    "dx-devtools-removed-target",
    "turbopack-runtime-build-adoption",
    "external-bundler-execution-proof-target",
  ]);

  assert.deepEqual(activeScopeSummary, {
    schema: "dx.nextRust.vendorBoundary.activeScopeSummary",
    status: "ok",
    scopeStatus: "ok",
    mismatchCount: 0,
    mismatches: [],
  });
});
