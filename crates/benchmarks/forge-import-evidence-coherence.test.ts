import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath) {
  return readFileSync(join(repoRoot, relativePath), "utf8");
}

function missingReferencedArtifacts(status, existingPaths) {
  const missing = [];
  const check = (field, value) => {
    if (value && !existingPaths.has(value)) {
      missing.push(`${field}:${value}`);
    }
  };

  check("package_lock.path", status.package_lock?.path);
  check("package_lock.manifest_path", status.package_lock?.manifest_path);
  check("remote_status.path", status.remote_status?.path);
  check(
    "remote_status.sync_plan_receipt_path",
    status.remote_status?.sync_plan_receipt_path,
  );

  for (const receipt of status.receipts?.examples ?? []) {
    check("receipts.examples", receipt);
  }

  return missing;
}

test("Forge import security model treats package status as a read model", () => {
  const model = read("docs/forge-import-security-model.md");
  const security = read("docs/SECURITY.md");
  const limitations = read("docs/forge-launch-limitations.md");

  assert.match(model, /`\.dx\/forge\/package-status\.json` is a read model/);
  assert.match(model, /not an authority artifact/);
  assert.match(model, /source manifest, package lock, current receipts/);
  assert.match(model, /trust policy/);
  assert.match(model, /remote status/);
  assert.match(model, /materialized files/);
  assert.match(model, /must be downgraded to `stale`/);
  assert.match(model, /BLAKE3 package and file integrity/);
  assert.match(model, /SHA-256 receipt freshness/);
  assert.match(
    model,
    /License and advisory data are declaration-only unless a receipt explicitly marks\s+them reviewed/,
  );

  assert.match(security, /Forge import security model/);
  assert.match(limitations, /package-status as a read model/);
});

test("status claims with missing references fail the coherence gate", () => {
  const status = {
    status: "lock-backed",
    package_lock: {
      path: ".dx/forge/package-lock.json",
      manifest_path: ".dx/forge/source-.dx/build-cache/manifest.json",
    },
    receipts: {
      examples: [".dx/forge/receipts/packages/demo.json"],
    },
    remote_status: {
      path: ".dx/forge/remote-status.json",
      sync_plan_receipt_path: ".dx/forge/receipts/remotes/sync-plan.json",
    },
  };

  assert.deepEqual(missingReferencedArtifacts(status, new Set()), [
    "package_lock.path:.dx/forge/package-lock.json",
    "package_lock.manifest_path:.dx/forge/source-.dx/build-cache/manifest.json",
    "remote_status.path:.dx/forge/remote-status.json",
    "remote_status.sync_plan_receipt_path:.dx/forge/receipts/remotes/sync-plan.json",
    "receipts.examples:.dx/forge/receipts/packages/demo.json",
  ]);

  assert.deepEqual(
    missingReferencedArtifacts(
      status,
      new Set([
        ".dx/forge/package-lock.json",
        ".dx/forge/source-.dx/build-cache/manifest.json",
        ".dx/forge/remote-status.json",
        ".dx/forge/receipts/remotes/sync-plan.json",
        ".dx/forge/receipts/packages/demo.json",
      ]),
    ),
    [],
  );
});
