import { spawn, type ChildProcess } from "node:child_process";
import fs from "node:fs";
import net from "node:net";
import path from "node:path";

type JsonRecord = Record<string, unknown>;

const REPO_ROOT = path.resolve(import.meta.dirname, "..");
const FIXTURE_SCHEMA = "dx.www.readiness.production_http_tcp_preview_fixture.v1";
const RECEIPT_SCHEMA = "dx.www.readiness.production_http_tcp_preview_receipt_contract";
const DEFAULT_OUT =
  ".dx/receipts/readiness/browser-import-candidates/production-http-tcp-preview-latest.json";
const DEFAULT_BUILD_DIR = "examples/template/.dx/www/output";
const DEFAULT_DX_WWW_BIN = "target/debug/dx-www.exe";

const REMAINING_EXTERNAL_GAPS = [
  "browser-js-enabled-runtime-replay",
  "browser-js-disabled-runtime-replay",
  "axum-static-responder-parity",
  "provider-bound-cdn-cache-replay",
  "hosted-provider-adapter-replay",
];

function usage(): string {
  return [
    "Usage:",
    "  node benchmarks/dx-www-production-preview-tcp-receipt.ts --dx-www-bin <dx-www.exe> --build-dir <.dx/www/output> --out <receipt.json>",
    "  node benchmarks/dx-www-production-preview-tcp-receipt.ts --from-response-fixture <fixture.json> --out <receipt.json>",
    "",
    "Starts dx preview --production-contract, sends raw TCP HTTP requests, and writes a source-owned production HTTP TCP preview receipt candidate.",
  ].join("\n");
}

function argumentAfter(args: string[], flag: string): string | undefined {
  const index = args.indexOf(flag);
  return index >= 0 ? args[index + 1] : undefined;
}

function resolveProjectPath(relativeOrAbsolute: string): string {
  return path.isAbsolute(relativeOrAbsolute)
    ? relativeOrAbsolute
    : path.join(REPO_ROOT, relativeOrAbsolute);
}

function readJson(relativeOrAbsolute: string): JsonRecord {
  return JSON.parse(fs.readFileSync(resolveProjectPath(relativeOrAbsolute), "utf8")) as JsonRecord;
}

function recordValue(value: unknown): JsonRecord {
  if (!value || typeof value !== "object" || Array.isArray(value)) return {};
  return value as JsonRecord;
}

function stringValue(value: unknown): string | null {
  return typeof value === "string" ? value : null;
}

function chooseAssetProbePath(buildDir: string): string {
  const deploy = JSON.parse(fs.readFileSync(path.join(buildDir, ".dx/build-cache/deploy-adapter.json"), "utf8")) as JsonRecord;
  const assets = Array.isArray(deploy.immutable_assets) ? deploy.immutable_assets : [];
  const paths = assets
    .map((asset) => recordValue(asset).path)
    .filter((candidate): candidate is string => typeof candidate === "string");
  const pathSet = new Set(paths);
  const precompressedBase = paths.find((candidate) => {
    if (candidate.endsWith(".br") || candidate.endsWith(".gz")) return false;
    return pathSet.has(`${candidate}.br`) && pathSet.has(`${candidate}.gz`);
  });
  if (precompressedBase) return precompressedBase;
  return paths.find((candidate) => /\.(mjs|js|css)$/i.test(candidate)) || paths[0] || "chunks/app.mjs";
}

function headerValue(response: string, name: string): string | null {
  const prefix = `${name.toLowerCase()}:`;
  for (const line of response.split(/\r?\n/)) {
    if (line.toLowerCase().startsWith(prefix)) {
      return line.slice(line.indexOf(":") + 1).trim();
    }
  }
  return null;
}

function responseBody(response: string | null): string {
  if (typeof response !== "string") return "";
  const split = response.indexOf("\r\n\r\n");
  return split >= 0 ? response.slice(split + 4) : "";
}

function has(response: string | null, value: string): boolean {
  return typeof response === "string" && response.includes(value);
}

function check(id: string, passed: boolean): JsonRecord {
  return { id, passed };
}

function checksFromResponses(responses: Record<string, string | null>): JsonRecord[] {
  const etag = headerValue(responses.first || "", "ETag");
  const lastModified = headerValue(responses.first || "", "Last-Modified");
  const firstBody = responseBody(responses.first);
  const firstBodyLength = firstBody.length;
  const firstRangeBody = firstBody.slice(1, 4);
  const firstBodyRangeable = firstBodyLength >= 4;
  const plainAssetContentType = headerValue(responses.plain_asset || "", "Content-Type");
  const brAssetContentType = headerValue(responses.br_asset || "", "Content-Type");
  const gzipAssetContentType = headerValue(responses.gzip_asset || "", "Content-Type");
  const precompressedDecodedContentType =
    Boolean(plainAssetContentType) &&
    brAssetContentType === plainAssetContentType &&
    gzipAssetContentType === plainAssetContentType;

  return [
    check("etag-present", Boolean(etag && etag.startsWith("\"dx-"))),
    check(
      "if-none-match-304",
      Boolean(
        etag &&
          has(responses.conditional_etag, "HTTP/1.1 304 Not Modified") &&
          has(responses.conditional_etag, "Content-Length: 0") &&
          has(responses.conditional_etag, `ETag: ${etag}`),
      ),
    ),
    check(
      "if-modified-since-304",
      Boolean(
        lastModified &&
          has(responses.conditional_last_modified, "HTTP/1.1 304 Not Modified") &&
          has(responses.conditional_last_modified, "Content-Length: 0") &&
          has(responses.conditional_last_modified, `Last-Modified: ${lastModified}`),
      ),
    ),
    check(
      "head-omits-body",
      has(responses.head, "HTTP/1.1 200 OK") &&
        has(responses.head, `Content-Length: ${firstBodyLength}`) &&
        Boolean(responses.head?.endsWith("\r\n\r\n")) &&
        responseBody(responses.head).length === 0,
    ),
    check(
      "range-206",
      firstBodyRangeable &&
      has(responses.range, "HTTP/1.1 206 Partial Content") &&
        has(responses.range, `Content-Range: bytes 1-3/${firstBodyLength}`) &&
        responseBody(responses.range) === firstRangeBody,
    ),
    check(
      "range-416",
      has(responses.invalid_range, "HTTP/1.1 416 Range Not Satisfiable") &&
        has(responses.invalid_range, `Content-Range: bytes */${firstBodyLength}`),
    ),
    check(
      "if-range-206",
      firstBodyRangeable &&
      has(responses.if_range, "HTTP/1.1 206 Partial Content") &&
        has(responses.if_range, `Content-Range: bytes 1-3/${firstBodyLength}`) &&
        responseBody(responses.if_range) === firstRangeBody,
    ),
    check(
      "stale-if-range-falls-back-to-full-body",
      has(responses.stale_if_range, "HTTP/1.1 200 OK") &&
        !has(responses.stale_if_range, "Content-Range:") &&
        responseBody(responses.stale_if_range) === firstBody,
    ),
    check(
      "br-negotiation",
      has(responses.br_asset, "HTTP/1.1 200 OK") &&
      has(responses.br_asset, "Content-Encoding: br") &&
        responseBody(responses.br_asset).length > 0,
    ),
    check(
      "gzip-negotiation",
      has(responses.gzip_asset, "HTTP/1.1 200 OK") &&
      has(responses.gzip_asset, "Content-Encoding: gzip") &&
        responseBody(responses.gzip_asset).length > 0,
    ),
    check(
      "plain-asset-vary",
      has(responses.plain_asset, "HTTP/1.1 200 OK") &&
        !has(responses.plain_asset, "Content-Encoding:") &&
        has(responses.plain_asset, "Vary: Accept-Encoding") &&
        responseBody(responses.plain_asset).length > 0,
    ),
    check(
      "static-options-204-allow-header",
      has(responses.options_route, "HTTP/1.1 204 No Content") &&
        has(responses.options_route, "Allow: GET, HEAD, OPTIONS") &&
        has(responses.options_route, "Content-Length: 0"),
    ),
    check(
      "static-post-405-allow-header",
      has(responses.post_asset, "HTTP/1.1 405 Method Not Allowed") &&
        has(responses.post_asset, "Allow: GET, HEAD, OPTIONS") &&
        has(responses.post_asset, "\"error\":\"static-asset-method-not-allowed\""),
    ),
    check("precompressed-decoded-content-type", precompressedDecodedContentType),
  ];
}

function receiptFromResponses(input: {
  url: string;
  buildDir: string;
  dxWwwBin: string;
  previewCommand: string;
  assetProbePath: string;
  responses: Record<string, string | null>;
  tcpPreviewServerStarted: boolean;
  tcpRequestsExecuted: boolean;
  source: string;
}): JsonRecord {
  const checks = checksFromResponses(input.responses);
  const passed = checks.every((candidate) => candidate.passed === true);
  const current = passed && input.tcpPreviewServerStarted && input.tcpRequestsExecuted;
  return {
    schema: RECEIPT_SCHEMA,
    schema_revision: 1,
    id: "production-http-tcp-preview",
    collector: "dx-source-owned-production-preview-tcp-collector",
    generated_at: new Date().toISOString(),
    project: REPO_ROOT.replaceAll(path.sep, "/"),
    passed,
    status: current
      ? "local-production-http-tcp-preview-current"
      : passed
        ? "fixture-production-http-tcp-preview-not-live-proof"
        : "production-http-tcp-preview-failed",
    release_ready: false,
    fastest_world_claim: false,
    proof_scope: "local-production-preview-tcp-server",
    source: input.source,
    url: input.url,
    build_dir: input.buildDir,
    dx_www_bin: input.dxWwwBin,
    preview_command: input.previewCommand,
    asset_probe_path: input.assetProbePath,
    tcp_preview_server_started: input.tcpPreviewServerStarted,
    tcp_requests_executed: input.tcpRequestsExecuted,
    browser_runtime_executed: false,
    provider_bound_cdn_executed: false,
    hosted_provider_proof: false,
    cleared_external_proof_gap_ids: current ? ["preview-tcp-server-parity"] : [],
    remaining_external_proof_gap_ids: current
      ? REMAINING_EXTERNAL_GAPS
      : ["preview-tcp-server-parity", ...REMAINING_EXTERNAL_GAPS],
    checks,
    etag: headerValue(input.responses.first || "", "ETag"),
    last_modified: headerValue(input.responses.first || "", "Last-Modified"),
    rule: "Local TCP preview proof clears only the preview-tcp-server-parity sub-gate; browser runtime, Axum parity, CDN, and hosted-provider proof remain separate gates.",
  };
}

function writeJson(filePath: string, value: JsonRecord): void {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`);
}

function freePort(): Promise<number> {
  return new Promise((resolve, reject) => {
    const server = net.createServer();
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      server.close(() => {
        if (address && typeof address === "object") resolve(address.port);
        else reject(new Error("Unable to allocate a local preview TCP port"));
      });
    });
    server.on("error", reject);
  });
}

function sendRawHttp(port: number, request: string, timeoutMs: number): Promise<string> {
  return new Promise((resolve, reject) => {
    const socket = net.createConnection({ host: "127.0.0.1", port });
    const chunks: Buffer[] = [];
    const timeout = setTimeout(() => {
      socket.destroy();
      reject(new Error(`Timed out waiting for TCP response on ${port}`));
    }, timeoutMs);
    socket.on("connect", () => socket.write(request));
    socket.on("data", (chunk) => chunks.push(chunk));
    socket.on("end", () => {
      clearTimeout(timeout);
      resolve(Buffer.concat(chunks).toString("utf8"));
    });
    socket.on("error", (error) => {
      clearTimeout(timeout);
      reject(error);
    });
  });
}

async function waitForPreview(port: number, timeoutMs: number): Promise<void> {
  const started = Date.now();
  let lastError: unknown = null;
  while (Date.now() - started < timeoutMs) {
    try {
      const response = await sendRawHttp(port, "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n", 750);
      if (response.startsWith("HTTP/1.1")) return;
    } catch (error) {
      lastError = error;
    }
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
  throw new Error(`Timed out waiting for dx preview TCP server on ${port}: ${String(lastError)}`);
}

async function collectLive(input: {
  dxWwwBin: string;
  buildDir: string;
  out: string;
  timeoutMs: number;
}): Promise<JsonRecord> {
  const dxWwwBin = resolveProjectPath(input.dxWwwBin);
  const buildDir = resolveProjectPath(input.buildDir);
  if (!fs.existsSync(dxWwwBin)) throw new Error(`Missing dx-www binary: ${dxWwwBin}`);
  if (!fs.existsSync(path.join(buildDir, ".dx/build-cache/deploy-adapter.json"))) {
    throw new Error(`Missing .dx/build-cache/deploy-adapter.json in build dir: ${buildDir}`);
  }
  const port = await freePort();
  const previewCommand = `${dxWwwBin} preview --production-contract --build-dir ${buildDir} --port ${port}`;
  let child: ChildProcess | null = null;
  try {
    child = spawn(dxWwwBin, [
      "preview",
      "--production-contract",
      "--build-dir",
      buildDir,
      "--port",
      String(port),
    ], { cwd: REPO_ROOT, stdio: "ignore" });
    await waitForPreview(port, input.timeoutMs);
    const url = `http://127.0.0.1:${port}`;
    const assetProbePath = chooseAssetProbePath(buildDir);
    const assetRequestPath = assetProbePath.startsWith("/") ? assetProbePath : `/${assetProbePath}`;
    const first = await sendRawHttp(port, "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n", input.timeoutMs);
    const etag = headerValue(first, "ETag") || "\"dx-missing\"";
    const lastModified = headerValue(first, "Last-Modified") || "Thu, 01 Jan 1970 00:00:00 GMT";
    const responses = {
      first,
      conditional_etag: await sendRawHttp(port, `GET / HTTP/1.1\r\nHost: localhost\r\nIf-None-Match: ${etag}\r\n\r\n`, input.timeoutMs),
      conditional_last_modified: await sendRawHttp(port, `GET / HTTP/1.1\r\nHost: localhost\r\nIf-Modified-Since: ${lastModified}\r\n\r\n`, input.timeoutMs),
      head: await sendRawHttp(port, "HEAD / HTTP/1.1\r\nHost: localhost\r\n\r\n", input.timeoutMs),
      range: await sendRawHttp(port, "GET / HTTP/1.1\r\nHost: localhost\r\nRange: bytes=1-3\r\n\r\n", input.timeoutMs),
      invalid_range: await sendRawHttp(port, "GET / HTTP/1.1\r\nHost: localhost\r\nRange: bytes=999999-1000000\r\n\r\n", input.timeoutMs),
      if_range: await sendRawHttp(port, `GET / HTTP/1.1\r\nHost: localhost\r\nRange: bytes=1-3\r\nIf-Range: ${etag}\r\n\r\n`, input.timeoutMs),
      stale_if_range: await sendRawHttp(port, "GET / HTTP/1.1\r\nHost: localhost\r\nRange: bytes=1-3\r\nIf-Range: \"dx-stale\"\r\n\r\n", input.timeoutMs),
      br_asset: await sendRawHttp(port, `GET ${assetRequestPath} HTTP/1.1\r\nHost: localhost\r\nAccept-Encoding: gzip;q=0.5, br\r\n\r\n`, input.timeoutMs),
      gzip_asset: await sendRawHttp(port, `GET ${assetRequestPath} HTTP/1.1\r\nHost: localhost\r\nAccept-Encoding: br;q=0, gzip;q=1\r\n\r\n`, input.timeoutMs),
      plain_asset: await sendRawHttp(port, `GET ${assetRequestPath} HTTP/1.1\r\nHost: localhost\r\n\r\n`, input.timeoutMs),
      options_route: await sendRawHttp(port, "OPTIONS / HTTP/1.1\r\nHost: localhost\r\n\r\n", input.timeoutMs),
      post_asset: await sendRawHttp(port, `POST ${assetRequestPath} HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\n\r\n`, input.timeoutMs),
    };
    return receiptFromResponses({
      url,
      buildDir: input.buildDir,
      dxWwwBin: input.dxWwwBin,
      previewCommand,
      assetProbePath,
      responses,
      tcpPreviewServerStarted: true,
      tcpRequestsExecuted: true,
      source: "dx-preview-production-contract-tcp",
    });
  } finally {
    if (child) child.kill();
  }
}

const args = process.argv.slice(2);
if (args.includes("--help")) {
  process.stdout.write(`${usage()}\n`);
} else if (args.includes("--from-response-fixture")) {
  const fixturePath = argumentAfter(args, "--from-response-fixture");
  if (!fixturePath) throw new Error(usage());
  const fixture = readJson(fixturePath);
  if (fixture.schema !== FIXTURE_SCHEMA) {
    throw new Error(`Expected ${FIXTURE_SCHEMA} in ${fixturePath}`);
  }
  const out = argumentAfter(args, "--out") || DEFAULT_OUT;
  const responses = recordValue(fixture.responses);
  const receipt = receiptFromResponses({
    url: String(fixture.url || "fixture://production-http-tcp-preview"),
    buildDir: String(fixture.build_dir || DEFAULT_BUILD_DIR),
    dxWwwBin: String(fixture.dx_www_bin || DEFAULT_DX_WWW_BIN),
    previewCommand: "fixture-response-map",
    assetProbePath: String(fixture.asset_probe_path || "chunks/app.mjs"),
    responses: Object.fromEntries(
      Object.entries(responses).map(([key, value]) => [key, stringValue(value)]),
    ),
    tcpPreviewServerStarted: false,
    tcpRequestsExecuted: false,
    source: fixturePath,
  });
  writeJson(resolveProjectPath(out), receipt);
  process.stdout.write(JSON.stringify({
    out,
    current: receipt.status === "local-production-http-tcp-preview-current",
    passed: receipt.passed === true,
    tcp_preview_server_started: receipt.tcp_preview_server_started === true,
  }, null, 2));
} else {
  const out = argumentAfter(args, "--out") || DEFAULT_OUT;
  const dxWwwBin = argumentAfter(args, "--dx-www-bin") || DEFAULT_DX_WWW_BIN;
  const buildDir = argumentAfter(args, "--build-dir") || DEFAULT_BUILD_DIR;
  const timeoutMs = Number(argumentAfter(args, "--timeout-ms") || 10000);
  const receipt = await collectLive({ dxWwwBin, buildDir, out, timeoutMs });
  writeJson(resolveProjectPath(out), receipt);
  process.stdout.write(JSON.stringify({
    out,
    current: receipt.status === "local-production-http-tcp-preview-current",
    passed: receipt.passed === true,
    tcp_preview_server_started: receipt.tcp_preview_server_started === true,
  }, null, 2));
}
