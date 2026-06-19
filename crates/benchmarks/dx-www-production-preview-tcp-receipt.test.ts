import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");
const collectorPath = path.join(root, "benchmarks", "dx-www-production-preview-tcp-receipt.ts");

function read(relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function okResponse(body = "abcdef"): string {
  return [
    "HTTP/1.1 200 OK",
    "Content-Type: text/html; charset=utf-8",
    "Cache-Control: no-store",
    'ETag: "dx-fixture-6"',
    "Last-Modified: Thu, 01 Jan 2026 00:00:00 GMT",
    "Accept-Ranges: bytes",
    `Content-Length: ${body.length}`,
    "Connection: close",
    "",
    body,
  ].join("\r\n");
}

function writeFixture(filePath: string, overrides: Record<string, unknown> = {}): void {
  const fixture = {
    schema: "dx.www.readiness.production_http_tcp_preview_fixture.v1",
    url: "http://127.0.0.1:4173",
    build_dir: "examples/template/.dx/www/output",
    dx_www_bin: "target/debug/dx-www.exe",
    responses: {
      first: okResponse(),
      conditional_etag: [
        "HTTP/1.1 304 Not Modified",
        'ETag: "dx-fixture-6"',
        "Content-Length: 0",
        "Connection: close",
        "",
        "",
      ].join("\r\n"),
      conditional_last_modified: [
        "HTTP/1.1 304 Not Modified",
        "Last-Modified: Thu, 01 Jan 2026 00:00:00 GMT",
        "Content-Length: 0",
        "Connection: close",
        "",
        "",
      ].join("\r\n"),
      head: [
        "HTTP/1.1 200 OK",
        "Content-Type: text/html; charset=utf-8",
        "Content-Length: 6",
        "Connection: close",
        "",
        "",
      ].join("\r\n"),
      range: [
        "HTTP/1.1 206 Partial Content",
        "Content-Range: bytes 1-3/6",
        "Content-Length: 3",
        "Connection: close",
        "",
        "bcd",
      ].join("\r\n"),
      invalid_range: [
        "HTTP/1.1 416 Range Not Satisfiable",
        "Content-Range: bytes */6",
        "Content-Length: 0",
        "Connection: close",
        "",
        "",
      ].join("\r\n"),
      if_range: [
        "HTTP/1.1 206 Partial Content",
        "Content-Range: bytes 1-3/6",
        "Content-Length: 3",
        "Connection: close",
        "",
        "bcd",
      ].join("\r\n"),
      stale_if_range: okResponse(),
      br_asset: [
        "HTTP/1.1 200 OK",
        "Content-Type: application/javascript; charset=utf-8",
        "Content-Encoding: br",
        "Vary: Accept-Encoding",
        "Content-Length: 5",
        "Connection: close",
        "",
        "br-js",
      ].join("\r\n"),
      gzip_asset: [
        "HTTP/1.1 200 OK",
        "Content-Type: application/javascript; charset=utf-8",
        "Content-Encoding: gzip",
        "Vary: Accept-Encoding",
        "Content-Length: 7",
        "Connection: close",
        "",
        "gzip-js",
      ].join("\r\n"),
      plain_asset: [
        "HTTP/1.1 200 OK",
        "Content-Type: application/javascript; charset=utf-8",
        "Vary: Accept-Encoding",
        "Content-Length: 8",
        "Connection: close",
        "",
        "plain-js",
      ].join("\r\n"),
      options_route: [
        "HTTP/1.1 204 No Content",
        "Allow: GET, HEAD, OPTIONS",
        "Content-Length: 0",
        "Connection: close",
        "",
        "",
      ].join("\r\n"),
      post_asset: [
        "HTTP/1.1 405 Method Not Allowed",
        "Allow: GET, HEAD, OPTIONS",
        "Content-Type: application/json",
        "Content-Length: 43",
        "Connection: close",
        "",
        '{"error":"static-asset-method-not-allowed"}',
      ].join("\r\n"),
    },
    ...overrides,
  };
  fs.writeFileSync(filePath, `${JSON.stringify(fixture, null, 2)}\n`);
}

test("production preview TCP collector is TypeScript, source-owned, and scoped honestly", () => {
  const collector = read("benchmarks/dx-www-production-preview-tcp-receipt.ts");

  for (const marker of [
    "dx.www.readiness.production_http_tcp_preview_fixture.v1",
    "dx.www.readiness.production_http_tcp_preview_receipt_contract",
    "dx-source-owned-production-preview-tcp-collector",
    "preview --production-contract",
    "chooseAssetProbePath",
    "asset_probe_path",
    "--build-dir",
    "--port",
    "sendRawHttp",
    "preview-tcp-server-parity",
    "browser-js-enabled-runtime-replay",
    "provider-bound-cdn-cache-replay",
    "hosted-provider-adapter-replay",
    "fixture-production-http-tcp-preview-not-live-proof",
    "local-production-http-tcp-preview-current",
  ]) {
    assert.match(collector, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.doesNotMatch(collector, /from\s+["'](?:playwright|puppeteer|axios)["']/);
  assert.doesNotMatch(collector, /require\(["'](?:playwright|puppeteer|axios)["']\)/);
});

test("production preview TCP collector fixture mode cannot claim live TCP proof", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-www-preview-tcp-"));
  const fixturePath = path.join(tempDir, "responses.json");
  const outPath = path.join(tempDir, "receipt.json");
  writeFixture(fixturePath);

  const output = execFileSync(
    process.execPath,
    [collectorPath, "--from-response-fixture", fixturePath, "--out", outPath],
    { cwd: root, encoding: "utf8" },
  );
  const summary = JSON.parse(output);
  const receipt = JSON.parse(fs.readFileSync(outPath, "utf8"));

  assert.equal(summary.current, false);
  assert.equal(summary.passed, true);
  assert.equal(summary.tcp_preview_server_started, false);
  assert.equal(receipt.passed, true);
  assert.equal(receipt.status, "fixture-production-http-tcp-preview-not-live-proof");
  assert.equal(receipt.tcp_preview_server_started, false);
  assert.equal(receipt.tcp_requests_executed, false);
  assert.equal(receipt.release_ready, false);
  assert.equal(receipt.fastest_world_claim, false);
  assert.deepEqual(receipt.cleared_external_proof_gap_ids, []);
  assert.ok(receipt.remaining_external_proof_gap_ids.includes("preview-tcp-server-parity"));
  assert.ok(receipt.checks.every((check: { passed: boolean }) => check.passed));
});

test("production preview TCP collector marks missing checks as failed", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-www-preview-tcp-fail-"));
  const fixturePath = path.join(tempDir, "responses.json");
  const outPath = path.join(tempDir, "receipt.json");
  writeFixture(fixturePath, {
    responses: {
      first: okResponse(),
      head: "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n",
    },
  });

  const output = execFileSync(
    process.execPath,
    [collectorPath, "--from-response-fixture", fixturePath, "--out", outPath],
    { cwd: root, encoding: "utf8" },
  );
  const summary = JSON.parse(output);
  const receipt = JSON.parse(fs.readFileSync(outPath, "utf8"));

  assert.equal(summary.current, false);
  assert.equal(summary.passed, false);
  assert.equal(receipt.status, "production-http-tcp-preview-failed");
  assert.equal(receipt.passed, false);
});
