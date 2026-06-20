import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const rootWindowsDeviceArtifacts = [
  "NUL",
  "nul",
  "CON",
  "con",
  "PRN",
  "prn",
  "AUX",
  "aux",
  "COM1",
  "com9",
  "LPT1",
  "lpt9",
];
const nestedWindowsDeviceSentinels = [
  "benchmarks/fixtures/NUL",
  "docs/CON",
  "examples/template/AUX",
  "worker-lanes/PRN",
  "cache/LPT1",
];

function isIgnored(relativePath) {
  const result = spawnSync("git", ["check-ignore", "-q", "--", relativePath], {
    cwd: root,
    encoding: "utf8",
  });

  if (result.status === 0) return true;
  if (result.status === 1) return false;
  const detail = result.error?.message || result.stderr.trim() || `exit ${result.status}`;
  throw new Error(`git check-ignore failed for ${relativePath}: ${detail}`);
}

test("temp and cache hygiene ignores only local worker/build byproducts", () => {
  for (const localArtifact of [
    ".tmp/agent25-pass3-installed-smoke.json",
    ".dx/cache/probe.json",
    ".dx/build/routes.json",
    ".dx/www/forge-package-status.machine",
    ".dx/www/forge-package-status.machine.meta.json",
    ".dx/www/style-check-receipt.machine",
    ".dx/performance/json-machine-cache-receipts/forge-package-status.json",
    ".dx/performance/json-machine-cache-receipts/media-icon-raw-index.json",
    ".dx/performance/json-machine-cache-receipts/media-icon-engine-startup.json",
    ".dx/icon/machine/v1/catalog.machine",
    ".dx/icon/machine/v1/catalog.machine.meta.json",
    ".dx/icon/machine/v1/prefix.machine",
    ".dx/icon/machine/v1/prefix.machine.meta.json",
    ".dx/icon/machine/v1/pack-body.machine",
    ".dx/icon/machine/v1/pack-body.machine.meta.json",
    ".dx/receipts/build/installed-binary-smoke-latest.json",
    "examples/template/.dx/build/route-.dx/build-cache/manifest.json",
    "examples/template/.dx/receipts/build/latest.json",
    "examples/template/.dx/receipts/graph/latest.json",
    "worker-lanes/state/www-worker.json",
    ...rootWindowsDeviceArtifacts,
  ]) {
    assert.equal(isIgnored(localArtifact), true, `${localArtifact} should stay local-only`);
  }

  for (const sourceOrCoordinationFile of [
    "start-www-worker.ps1",
    "worker-lanes/claim-www-lane.ps1",
    "worker-lanes/WWW_30_AGENT_AUTO_LANE_PROMPT.md",
    "cache/Cargo.toml",
    "examples/template/.dx/forge/package-status.json",
    ...nestedWindowsDeviceSentinels,
  ]) {
    assert.equal(
      isIgnored(sourceOrCoordinationFile),
      false,
      `${sourceOrCoordinationFile} must remain visible to release control`,
    );
  }
});
