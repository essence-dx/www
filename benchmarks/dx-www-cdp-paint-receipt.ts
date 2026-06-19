import { spawn, type ChildProcess } from "node:child_process";
import fs from "node:fs";
import http from "node:http";
import os from "node:os";
import path from "node:path";

type JsonRecord = Record<string, unknown>;

type CdpResponse = {
  id?: number;
  method?: string;
  params?: JsonRecord;
  result?: JsonRecord;
  error?: { message?: string };
  sessionId?: string;
};

const PAINT_FIXTURE_SCHEMA = "dx.www.readiness.cdp_paint_fixture.v1";
const REPO_ROOT = path.resolve(import.meta.dirname, "..");
const DEFAULT_OUT_PATH = ".dx/receipts/check/web-perf/dev/report.json";

function usage(): string {
  return [
    "Usage:",
    "  node benchmarks/dx-www-cdp-paint-receipt.ts --url <url> --receipt-mode <dev|static-build>",
    "  node benchmarks/dx-www-cdp-paint-receipt.ts --url <url> --out <receipt.json>",
    "  node benchmarks/dx-www-cdp-paint-receipt.ts --from-fixture <fixture.json> --out <receipt.json>",
    "",
    "Launches Chrome or Edge through CDP, records source-owned browser paint metrics, and writes a dx check web-perf receipt without using Lighthouse or npm packages.",
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

function numberValue(value: unknown): number | null {
  return typeof value === "number" && Number.isFinite(value) && value >= 0 ? value : null;
}

function browserCandidates(): string[] {
  const explicit = process.env.DX_BROWSER || process.env.CHROME || process.env.CHROME_PATH;
  return [
    explicit,
    "C:/Program Files/Google/Chrome/Application/chrome.exe",
    "C:/Program Files (x86)/Google/Chrome/Application/chrome.exe",
    "C:/Program Files/Microsoft/Edge/Application/msedge.exe",
    "C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe",
  ].filter((candidate): candidate is string => Boolean(candidate && fs.existsSync(candidate)));
}

function freePort(): Promise<number> {
  return new Promise((resolve, reject) => {
    const server = http.createServer();
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      server.close(() => {
        if (address && typeof address === "object") resolve(address.port);
        else reject(new Error("Unable to allocate a local CDP port"));
      });
    });
    server.on("error", reject);
  });
}

async function waitForJson(url: string, timeoutMs: number): Promise<JsonRecord> {
  const started = Date.now();
  let lastError: unknown = null;
  while (Date.now() - started < timeoutMs) {
    try {
      const response = await fetch(url);
      if (response.ok) return (await response.json()) as JsonRecord;
    } catch (error) {
      lastError = error;
    }
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
  throw new Error(`Timed out waiting for ${url}: ${String(lastError)}`);
}

class CdpClient {
  private nextId = 1;
  private pending = new Map<number, { resolve: (value: JsonRecord) => void; reject: (error: Error) => void }>();
  private listeners = new Map<string, Array<(event: CdpResponse) => void>>();
  private readonly socket: WebSocket;

  constructor(socket: WebSocket) {
    this.socket = socket;
    this.socket.addEventListener("message", (event) => {
      const message = JSON.parse(String(event.data)) as CdpResponse;
      if (typeof message.id === "number") {
        const pending = this.pending.get(message.id);
        if (!pending) return;
        this.pending.delete(message.id);
        if (message.error) {
          pending.reject(new Error(message.error.message || `CDP command ${message.id} failed`));
        } else {
          pending.resolve(message.result || {});
        }
        return;
      }
      if (message.method) {
        for (const listener of this.listeners.get(message.method) || []) listener(message);
      }
    });
  }

  send(method: string, params: JsonRecord = {}, sessionId?: string): Promise<JsonRecord> {
    const id = this.nextId++;
    const payload = sessionId ? { id, method, params, sessionId } : { id, method, params };
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      this.socket.send(JSON.stringify(payload));
    });
  }

  on(method: string, listener: (event: CdpResponse) => void): void {
    this.listeners.set(method, [...(this.listeners.get(method) || []), listener]);
  }

  once(method: string, timeoutMs: number): Promise<CdpResponse> {
    return new Promise((resolve, reject) => {
      const timeout = setTimeout(() => reject(new Error(`Timed out waiting for ${method}`)), timeoutMs);
      const listener = (event: CdpResponse) => {
        clearTimeout(timeout);
        this.listeners.set(
          method,
          (this.listeners.get(method) || []).filter((candidate) => candidate !== listener),
        );
        resolve(event);
      };
      this.on(method, listener);
    });
  }
}

async function openCdp(webSocketUrl: string): Promise<CdpClient> {
  if (typeof WebSocket !== "function") {
    throw new Error("This Node.js runtime does not expose the WebSocket global required for CDP.");
  }
  const socket = new WebSocket(webSocketUrl);
  await new Promise<void>((resolve, reject) => {
    socket.addEventListener("open", () => resolve(), { once: true });
    socket.addEventListener("error", () => reject(new Error("Failed to connect to browser CDP WebSocket")), {
      once: true,
    });
  });
  return new CdpClient(socket);
}

const paintObserverScript = String.raw`
(() => {
  const state = {
    firstContentfulPaintMs: null,
    largestContentfulPaintMs: null,
    cumulativeLayoutShift: 0,
    totalBlockingTimeMs: 0,
    observerErrors: []
  };
  Object.defineProperty(window, "__DX_CDP_PAINT__", { value: state, configurable: true });
  const observe = (type, callback) => {
    try {
      const observer = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) callback(entry);
      });
      observer.observe({ type, buffered: true });
    } catch (error) {
      state.observerErrors.push(type + ":" + String(error && error.message || error));
    }
  };
  observe("paint", (entry) => {
    if (entry.name === "first-contentful-paint") state.firstContentfulPaintMs = entry.startTime;
  });
  observe("largest-contentful-paint", (entry) => {
    state.largestContentfulPaintMs = entry.startTime;
  });
  observe("layout-shift", (entry) => {
    if (!entry.hadRecentInput) state.cumulativeLayoutShift += entry.value || 0;
  });
  observe("longtask", (entry) => {
    state.totalBlockingTimeMs += Math.max(0, (entry.duration || 0) - 50);
  });
})();
`;

function receiptFromMetrics(input: {
  url: string;
  device: string;
  source: string;
  firstContentfulPaintMs: number | null;
  largestContentfulPaintMs: number | null;
  cumulativeLayoutShift: number | null;
  totalBlockingTimeMs: number | null;
  speedIndexMs: number | null;
  requestCount: number | null;
  transferSizeBytes: number | null;
  userAgent: string;
  observerErrors: unknown[];
  browserRuntimeExecuted: boolean;
}): JsonRecord {
  const metricsComplete =
    input.firstContentfulPaintMs !== null && input.largestContentfulPaintMs !== null;
  const current = metricsComplete && input.browserRuntimeExecuted;
  return {
    tool: "dx check web-perf",
    version: 1,
    generated_at: new Date().toISOString(),
    project: REPO_ROOT.replaceAll(path.sep, "/"),
    collector: "dx-source-owned-cdp-paint-collector",
    measurement_status: current
      ? "measured-from-source-owned-cdp"
      : metricsComplete
        ? "fixture-cdp-paint-not-browser-proof"
      : "partial-cdp-paint-missing-fcp-or-lcp",
    device: input.device,
    url: input.url,
    input: input.source,
    scores: {
      performance: null,
      accessibility: null,
      best_practices: null,
      seo: null,
      total: null,
    },
    score_completeness: {
      complete: false,
      required_categories: ["performance", "accessibility", "best-practices", "seo"],
      missing_categories: ["performance", "accessibility", "best-practices", "seo"],
      policy: "source-owned CDP paint proof does not claim Lighthouse category totals",
    },
    core_web_vitals: {
      first_contentful_paint_ms: input.firstContentfulPaintMs,
      largest_contentful_paint_ms: input.largestContentfulPaintMs,
      cumulative_layout_shift: input.cumulativeLayoutShift,
      total_blocking_time_ms: input.totalBlockingTimeMs,
      speed_index_ms: input.speedIndexMs,
    },
    network: {
      request_count: input.requestCount,
      transfer_size_bytes: input.transferSizeBytes,
    },
    cdp_domains: ["Browser", "Target", "Page", "Performance", "Network", "Runtime"],
    paint_proof_kind: "source-owned-cdp-browser-paint",
    metrics_complete: metricsComplete,
    browser_runtime_executed: input.browserRuntimeExecuted,
    lighthouse_parity: false,
    release_ready: false,
    fastest_world_claim: false,
    user_agent: input.userAgent,
    observer_errors: input.observerErrors,
    score_scale: "0-400",
    receipts: {
      report: ".dx/receipts/check/web-perf/report.json",
    },
    receipt_path: ".dx/receipts/check/web-perf/report.json",
  };
}

async function collectViaCdp(url: string, device: string, timeoutMs: number): Promise<JsonRecord> {
  const [browserPath] = browserCandidates();
  if (!browserPath) {
    throw new Error("Chrome or Edge was not found. Set DX_BROWSER to a Chromium executable path.");
  }

  const port = await freePort();
  const userDataDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-www-cdp-paint-"));
  let browserProcess: ChildProcess | null = null;
  let requestCount = 0;
  let transferSizeBytes = 0;

  try {
    browserProcess = spawn(browserPath, [
      "--headless=new",
      "--disable-gpu",
      "--no-first-run",
      "--no-default-browser-check",
      `--user-data-dir=${userDataDir}`,
      `--remote-debugging-port=${port}`,
      "about:blank",
    ], { stdio: "ignore" });

    const version = await waitForJson(`http://127.0.0.1:${port}/json/version`, timeoutMs);
    const webSocketUrl = version.webSocketDebuggerUrl;
    if (typeof webSocketUrl !== "string") throw new Error("CDP version endpoint did not expose a WebSocket URL.");
    const cdp = await openCdp(webSocketUrl);
    const target = await cdp.send("Target.createTarget", { url: "about:blank" });
    const targetId = target.targetId;
    if (typeof targetId !== "string") throw new Error("Target.createTarget did not return a target id.");
    const attached = await cdp.send("Target.attachToTarget", { targetId, flatten: true });
    const sessionId = attached.sessionId;
    if (typeof sessionId !== "string") throw new Error("Target.attachToTarget did not return a session id.");

    cdp.on("Network.requestWillBeSent", () => {
      requestCount += 1;
    });
    cdp.on("Network.loadingFinished", (event) => {
      const encoded = recordValue(event.params).encodedDataLength;
      if (typeof encoded === "number" && Number.isFinite(encoded) && encoded >= 0) {
        transferSizeBytes += encoded;
      }
    });

    await cdp.send("Page.enable", {}, sessionId);
    await cdp.send("Network.enable", {}, sessionId);
    await cdp.send("Performance.enable", {}, sessionId);
    await cdp.send("Runtime.enable", {}, sessionId);
    await cdp.send("Network.setCacheDisabled", { cacheDisabled: true }, sessionId);
    await cdp.send("Page.addScriptToEvaluateOnNewDocument", { source: paintObserverScript }, sessionId);

    const loaded = cdp.once("Page.loadEventFired", timeoutMs);
    await cdp.send("Page.navigate", { url }, sessionId);
    await loaded;
    await new Promise((resolve) => setTimeout(resolve, 250));

    const metricsResponse = await cdp.send("Runtime.evaluate", {
      expression: "(() => { const paint = window.__DX_CDP_PAINT__ || {}; return { firstContentfulPaintMs: paint.firstContentfulPaintMs ?? null, largestContentfulPaintMs: paint.largestContentfulPaintMs ?? null, cumulativeLayoutShift: paint.cumulativeLayoutShift ?? null, totalBlockingTimeMs: paint.totalBlockingTimeMs ?? null, observerErrors: paint.observerErrors || [], userAgent: navigator.userAgent }; })()",
      returnByValue: true,
    }, sessionId);
    await cdp.send("Target.closeTarget", { targetId });

    const result = recordValue(metricsResponse.result);
    const value = recordValue(result.value);
    return receiptFromMetrics({
      url,
      device,
      source: "chrome-devtools-protocol",
      firstContentfulPaintMs: numberValue(value.firstContentfulPaintMs),
      largestContentfulPaintMs: numberValue(value.largestContentfulPaintMs),
      cumulativeLayoutShift: numberValue(value.cumulativeLayoutShift),
      totalBlockingTimeMs: numberValue(value.totalBlockingTimeMs),
      speedIndexMs: null,
      requestCount,
      transferSizeBytes,
      userAgent: typeof value.userAgent === "string" ? value.userAgent : "unknown-cdp-user-agent",
      observerErrors: Array.isArray(value.observerErrors) ? value.observerErrors : [],
      browserRuntimeExecuted: true,
    });
  } finally {
    if (browserProcess) browserProcess.kill();
    try {
      fs.rmSync(userDataDir, { recursive: true, force: true });
    } catch {
      // Windows can briefly hold the Chromium profile directory after process shutdown.
    }
  }
}

function writeJson(filePath: string, value: JsonRecord): void {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`);
}

function outputPathForMode(mode: string): string {
  return `.dx/receipts/check/web-perf/${mode}/report.json`;
}

const args = process.argv.slice(2);
if (args.includes("--help")) {
  process.stdout.write(`${usage()}\n`);
} else if (args.includes("--from-fixture")) {
  const fixturePath = argumentAfter(args, "--from-fixture");
  if (!fixturePath) throw new Error(usage());
  const fixture = readJson(fixturePath);
  if (fixture.schema !== PAINT_FIXTURE_SCHEMA) {
    throw new Error(`Expected ${PAINT_FIXTURE_SCHEMA} in ${fixturePath}`);
  }
  const receiptMode = argumentAfter(args, "--receipt-mode") || String(fixture.receipt_mode || "dev");
  const out = argumentAfter(args, "--out") || outputPathForMode(receiptMode);
  const receipt = receiptFromMetrics({
    url: typeof fixture.url === "string" ? fixture.url : "fixture://dx-www-cdp-paint",
    device: typeof fixture.device === "string" ? fixture.device : "desktop",
    source: fixturePath,
    firstContentfulPaintMs: numberValue(fixture.first_contentful_paint_ms),
    largestContentfulPaintMs: numberValue(fixture.largest_contentful_paint_ms),
    cumulativeLayoutShift: numberValue(fixture.cumulative_layout_shift),
    totalBlockingTimeMs: numberValue(fixture.total_blocking_time_ms),
    speedIndexMs: numberValue(fixture.speed_index_ms),
    requestCount: numberValue(fixture.request_count),
    transferSizeBytes: numberValue(fixture.transfer_size_bytes),
    userAgent: typeof fixture.user_agent === "string" ? fixture.user_agent : "fixture",
    observerErrors: Array.isArray(fixture.observer_errors) ? fixture.observer_errors : [],
    browserRuntimeExecuted: false,
  });
  writeJson(resolveProjectPath(out), receipt);
  process.stdout.write(JSON.stringify({
    out,
    current: receipt.measurement_status === "measured-from-source-owned-cdp" && receipt.browser_runtime_executed === true,
    metrics_complete: receipt.metrics_complete === true,
    browser_runtime_executed: receipt.browser_runtime_executed === true,
  }, null, 2));
} else {
  const url = argumentAfter(args, "--url");
  if (!url) throw new Error(usage());
  const receiptMode = argumentAfter(args, "--receipt-mode") || "dev";
  if (!["dev", "static-build"].includes(receiptMode)) throw new Error("--receipt-mode must be dev or static-build");
  const out = argumentAfter(args, "--out") || outputPathForMode(receiptMode);
  const timeoutMs = Number(argumentAfter(args, "--timeout-ms") || 15000);
  const device = argumentAfter(args, "--device") || "desktop";
  const receipt = await collectViaCdp(url, device, timeoutMs);
  writeJson(resolveProjectPath(out), receipt);
  process.stdout.write(JSON.stringify({
    out,
    current: receipt.measurement_status === "measured-from-source-owned-cdp" && receipt.browser_runtime_executed === true,
    metrics_complete: receipt.metrics_complete === true,
    browser_runtime_executed: receipt.browser_runtime_executed === true,
  }, null, 2));
}
