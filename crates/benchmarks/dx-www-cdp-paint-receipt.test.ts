import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");
const collectorPath = path.join(root, "benchmarks", "dx-www-cdp-paint-receipt.ts");

function read(relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function writeFixture(filePath: string, overrides: Record<string, unknown> = {}): void {
  const fixture = {
    schema: "dx.www.readiness.cdp_paint_fixture.v1",
    receipt_mode: "dev",
    url: "http://127.0.0.1:3000/",
    device: "desktop",
    first_contentful_paint_ms: 42.4,
    largest_contentful_paint_ms: 88.8,
    cumulative_layout_shift: 0,
    total_blocking_time_ms: 0,
    speed_index_ms: null,
    request_count: 3,
    transfer_size_bytes: 4096,
    user_agent: "fixture-cdp-browser",
    observer_errors: [],
    ...overrides,
  };
  fs.writeFileSync(filePath, `${JSON.stringify(fixture, null, 2)}\n`);
}

test("source-owned CDP paint collector is TypeScript and avoids Lighthouse/npm/browser UI packages", () => {
  const collector = read("benchmarks/dx-www-cdp-paint-receipt.ts");

  for (const marker of [
    "dx.www.readiness.cdp_paint_fixture.v1",
    "dx-source-owned-cdp-paint-collector",
    "measured-from-source-owned-cdp",
    "source-owned-cdp-browser-paint",
    "metrics_complete",
    "browser_runtime_executed",
    "Page.addScriptToEvaluateOnNewDocument",
    "PerformanceObserver",
    "largest-contentful-paint",
    "Network.loadingFinished",
    "Chrome or Edge was not found. Set DX_BROWSER",
    "lighthouse_parity: false",
    "source-owned CDP paint proof does not claim Lighthouse category totals",
  ]) {
    assert.match(collector, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.doesNotMatch(collector, /from\s+["'](?:playwright|puppeteer)["']/);
  assert.doesNotMatch(collector, /require\(["'](?:playwright|puppeteer)["']\)/);
  assert.doesNotMatch(collector, /\bnpx\b|--lighthouse|lighthouse\.json/);
});

test("source-owned CDP paint collector writes fixture metrics without claiming browser proof", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-www-cdp-paint-"));
  const fixturePath = path.join(tempDir, "paint-fixture.json");
  const outPath = path.join(tempDir, "report.json");
  writeFixture(fixturePath);

  const output = execFileSync(
    process.execPath,
    [collectorPath, "--from-fixture", fixturePath, "--out", outPath],
    { cwd: root, encoding: "utf8" },
  );
  const summary = JSON.parse(output);
  const receipt = JSON.parse(fs.readFileSync(outPath, "utf8"));

  assert.equal(summary.current, false);
  assert.equal(summary.metrics_complete, true);
  assert.equal(summary.browser_runtime_executed, false);
  assert.equal(receipt.tool, "dx check web-perf");
  assert.equal(receipt.collector, "dx-source-owned-cdp-paint-collector");
  assert.equal(receipt.measurement_status, "fixture-cdp-paint-not-browser-proof");
  assert.equal(receipt.paint_proof_kind, "source-owned-cdp-browser-paint");
  assert.equal(receipt.metrics_complete, true);
  assert.equal(receipt.browser_runtime_executed, false);
  assert.equal(receipt.lighthouse_parity, false);
  assert.equal(receipt.release_ready, false);
  assert.equal(receipt.fastest_world_claim, false);
  assert.equal(receipt.score_completeness.complete, false);
  assert.equal(receipt.core_web_vitals.first_contentful_paint_ms, 42.4);
  assert.equal(receipt.core_web_vitals.largest_contentful_paint_ms, 88.8);
  assert.equal(receipt.network.request_count, 3);
  assert.equal(receipt.network.transfer_size_bytes, 4096);
});

test("source-owned CDP paint collector keeps partial receipts non-current when LCP is missing", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-www-cdp-paint-partial-"));
  const fixturePath = path.join(tempDir, "paint-fixture.json");
  const outPath = path.join(tempDir, "report.json");
  writeFixture(fixturePath, { largest_contentful_paint_ms: null });

  const output = execFileSync(
    process.execPath,
    [collectorPath, "--from-fixture", fixturePath, "--out", outPath],
    { cwd: root, encoding: "utf8" },
  );
  const summary = JSON.parse(output);
  const receipt = JSON.parse(fs.readFileSync(outPath, "utf8"));

  assert.equal(summary.current, false);
  assert.equal(summary.metrics_complete, false);
  assert.equal(summary.browser_runtime_executed, false);
  assert.equal(receipt.collector, "dx-source-owned-cdp-paint-collector");
  assert.equal(receipt.measurement_status, "partial-cdp-paint-missing-fcp-or-lcp");
  assert.equal(receipt.metrics_complete, false);
  assert.equal(receipt.browser_runtime_executed, false);
  assert.equal(receipt.core_web_vitals.largest_contentful_paint_ms, null);
});
