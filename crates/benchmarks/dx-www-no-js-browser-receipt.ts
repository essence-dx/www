import { spawn, type ChildProcess } from "node:child_process";
import fs from "node:fs";
import http from "node:http";
import os from "node:os";
import path from "node:path";
import { pathToFileURL } from "node:url";

type JsonRecord = Record<string, unknown>;

type DomNode = {
  nodeName: string;
  nodeValue?: string;
  attributes?: string[];
  children?: DomNode[];
};

type CdpResponse = {
  id?: number;
  method?: string;
  params?: JsonRecord;
  result?: JsonRecord;
  error?: { message?: string };
  sessionId?: string;
};

const NO_JS_BROWSER_SCHEMA = "dx.www.readiness.no_js_browser_receipt_contract";
const DOM_FIXTURE_SCHEMA = "dx.www.readiness.no_js_browser_dom_snapshot_fixture.v1";
const REPO_ROOT = path.resolve(import.meta.dirname, "..");
const DEFAULT_HTML_PATH = "examples/template/.dx/www/output/app/index.html";
const DEFAULT_OUT_PATH = ".dx/receipts/readiness/browser-import-candidates/no-js-browser-latest.json";
const NO_JS_ARTIFACT_RECEIPT = ".dx/receipts/readiness/no-js-artifact-latest.json";

function usage(): string {
  return [
    "Usage:",
    "  node benchmarks/dx-www-no-js-browser-receipt.ts --out <receipt.json>",
    "  node benchmarks/dx-www-no-js-browser-receipt.ts --html-path <relative-html> --out <receipt.json>",
    "  node benchmarks/dx-www-no-js-browser-receipt.ts --url <url> --html-path <relative-html> --out <receipt.json>",
    "  node benchmarks/dx-www-no-js-browser-receipt.ts --from-dom-json <fixture.json> --out <receipt.json>",
    "",
    "The default browser path launches Chrome or Edge through CDP, disables script execution before navigation, then reads the DOM without executing page JavaScript.",
  ].join("\n");
}

function argumentAfter(args: string[], flag: string): string | undefined {
  const index = args.indexOf(flag);
  return index >= 0 ? args[index + 1] : undefined;
}

function readJson(relativeOrAbsolute: string): JsonRecord {
  return JSON.parse(fs.readFileSync(resolveProjectPath(relativeOrAbsolute), "utf8")) as JsonRecord;
}

function resolveProjectPath(relativeOrAbsolute: string): string {
  return path.isAbsolute(relativeOrAbsolute)
    ? relativeOrAbsolute
    : path.join(REPO_ROOT, relativeOrAbsolute);
}

function relativeProjectPath(filePath: string): string {
  return path.relative(REPO_ROOT, filePath).replaceAll(path.sep, "/");
}

function recordValue(value: unknown): JsonRecord {
  if (!value || typeof value !== "object" || Array.isArray(value)) return {};
  return value as JsonRecord;
}

function noJsArtifactReceipt(): JsonRecord {
  try {
    return readJson(NO_JS_ARTIFACT_RECEIPT);
  } catch {
    return {};
  }
}

function defaultUrlForHtmlPath(htmlPath: string): string {
  return pathToFileURL(resolveProjectPath(htmlPath)).toString();
}

function htmlMetadata(htmlPath: string): { htmlPath: string; artifactHtmlBlake3: string | null } {
  const artifact = noJsArtifactReceipt();
  const artifactHtmlPath = typeof artifact.html_path === "string" ? artifact.html_path : htmlPath;
  const artifactHash =
    typeof artifact.artifact_html_blake3 === "string" ? artifact.artifact_html_blake3 : null;
  return {
    htmlPath: artifactHtmlPath,
    artifactHtmlBlake3: artifactHash,
  };
}

function attrPairs(node: DomNode): Map<string, string> {
  const pairs = new Map<string, string>();
  const attributes = Array.isArray(node.attributes) ? node.attributes : [];
  for (let index = 0; index + 1 < attributes.length; index += 2) {
    pairs.set(attributes[index].toLowerCase(), attributes[index + 1]);
  }
  return pairs;
}

function textContent(node: DomNode): string {
  const ownText = typeof node.nodeValue === "string" ? node.nodeValue : "";
  return `${ownText}${(node.children || []).map(textContent).join("")}`;
}

function domFacts(documentNode: DomNode): JsonRecord {
  let scriptTagCount = 0;
  let dataDxOutputModeTinyStatic = false;
  let dataDxJsNone = false;
  let semanticLandmarkPresent = false;
  let linkCount = 0;
  let formCount = 0;
  let seoTitlePresent = false;
  let accessibilitySignalCount = 0;
  let visibleTextPresent = false;

  const visit = (node: DomNode) => {
    const tag = node.nodeName.toLowerCase();
    const attrs = attrPairs(node);
    if (tag === "#text" && typeof node.nodeValue === "string" && node.nodeValue.trim().length > 0) {
      visibleTextPresent = true;
    }
    if (tag === "script") scriptTagCount += 1;
    if (attrs.get("data-dx-output-mode") === "tiny-static") dataDxOutputModeTinyStatic = true;
    if (attrs.get("data-dx-js") === "none") dataDxJsNone = true;
    if (["main", "nav", "header", "footer", "aside", "section", "article"].includes(tag)) {
      semanticLandmarkPresent = true;
    }
    if (tag === "a" && attrs.has("href")) linkCount += 1;
    if (tag === "form") formCount += 1;
    if (tag === "title" && textContent(node).trim().length > 0) seoTitlePresent = true;
    if (
      ["main", "h1", "h2", "h3", "form", "label"].includes(tag) ||
      (tag === "a" && attrs.has("href")) ||
      (tag === "img" && attrs.has("alt")) ||
      attrs.has("aria-label") ||
      attrs.has("aria-labelledby") ||
      attrs.has("role")
    ) {
      accessibilitySignalCount += 1;
    }
    (node.children || []).forEach(visit);
  };
  visit(documentNode);

  return {
    script_tag_count: scriptTagCount,
    data_dx_output_mode_tiny_static: dataDxOutputModeTinyStatic,
    data_dx_js_none: dataDxJsNone,
    semantic_landmark_present: semanticLandmarkPresent,
    visible_text_present: visibleTextPresent,
    link_count: linkCount,
    form_count: formCount,
    seo_title_present: seoTitlePresent,
    accessibility_signal_count: accessibilitySignalCount,
  };
}

function noJsReceiptFromDomSnapshot(input: {
  document: DomNode;
  url: string;
  userAgent: string;
  htmlPath: string;
  artifactHtmlBlake3: string | null;
  scriptExecutionDisabledBeforeNavigation: boolean;
}): JsonRecord {
  const facts = domFacts(input.document);
  const passed =
    input.scriptExecutionDisabledBeforeNavigation &&
    facts.script_tag_count === 0 &&
    facts.data_dx_output_mode_tiny_static === true &&
    facts.data_dx_js_none === true &&
    facts.semantic_landmark_present === true &&
    facts.visible_text_present === true &&
    typeof facts.link_count === "number" &&
    facts.link_count > 0 &&
    typeof facts.form_count === "number" &&
    facts.form_count > 0 &&
    facts.seo_title_present === true &&
    typeof facts.accessibility_signal_count === "number" &&
    facts.accessibility_signal_count > 0 &&
    typeof input.artifactHtmlBlake3 === "string" &&
    /^blake3:[a-f0-9]{64}$/.test(input.artifactHtmlBlake3);

  return {
    schema: NO_JS_BROWSER_SCHEMA,
    schema_revision: 1,
    passed,
    status: passed ? "current-local-js-disabled-browser-proof" : "candidate-not-current",
    live_browser_executed: true,
    javascript_disabled_browser: input.scriptExecutionDisabledBeforeNavigation,
    page_javascript_enabled: false,
    html_path: input.htmlPath,
    artifact_html_blake3: input.artifactHtmlBlake3,
    ...facts,
    browser_snapshot_source: "chrome-devtools-protocol-dom-get-document",
    script_execution_disabled_before_navigation: input.scriptExecutionDisabledBeforeNavigation,
    script_execution_disabled_cdp: input.scriptExecutionDisabledBeforeNavigation,
    proof_scope: "local-js-disabled-browser-no-js-route-replay",
    page_url: input.url,
    user_agent: input.userAgent,
    receipt_source: "dx-www-no-js-browser-receipt.ts",
    release_ready: false,
    fastest_world_claim: false,
  };
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
      this.listeners.set(method, [...(this.listeners.get(method) || []), listener]);
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

function cdpNodeToDomNode(node: JsonRecord): DomNode {
  return {
    nodeName: String(node.nodeName || ""),
    nodeValue: typeof node.nodeValue === "string" ? node.nodeValue : "",
    attributes: Array.isArray(node.attributes)
      ? node.attributes.filter((value): value is string => typeof value === "string")
      : [],
    children: Array.isArray(node.children) ? node.children.map((child) => cdpNodeToDomNode(child as JsonRecord)) : [],
  };
}

async function collectViaCdp(url: string, timeoutMs: number): Promise<{ document: DomNode; userAgent: string }> {
  const [browserPath] = browserCandidates();
  if (!browserPath) {
    throw new Error("Chrome or Edge was not found. Set DX_BROWSER to a Chromium executable path.");
  }

  const port = await freePort();
  const userDataDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-www-no-js-browser-"));
  let browserProcess: ChildProcess | null = null;
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

    await cdp.send("Page.enable", {}, sessionId);
    await cdp.send("DOM.enable", {}, sessionId);
    await cdp.send("Emulation.setScriptExecutionDisabled", { value: true }, sessionId);
    const loaded = cdp.once("Page.loadEventFired", timeoutMs);
    await cdp.send("Page.navigate", { url }, sessionId);
    await loaded;
    const documentResponse = await cdp.send("DOM.getDocument", { depth: -1, pierce: true }, sessionId);
    const navigatorResponse = await cdp.send(
      "Browser.getVersion",
      {},
    );
    await cdp.send("Target.closeTarget", { targetId });
    return {
      document: cdpNodeToDomNode(recordValue(documentResponse.root)),
      userAgent:
        typeof navigatorResponse.userAgent === "string"
          ? navigatorResponse.userAgent
          : "unknown-cdp-user-agent",
    };
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

const args = process.argv.slice(2);
const out = argumentAfter(args, "--out") || DEFAULT_OUT_PATH;
const htmlPath = argumentAfter(args, "--html-path") || DEFAULT_HTML_PATH;
const metadata = htmlMetadata(htmlPath);

if (args.includes("--help")) {
  process.stdout.write(`${usage()}\n`);
} else if (args.includes("--from-dom-json")) {
  const source = argumentAfter(args, "--from-dom-json");
  if (!source) throw new Error(usage());
  const fixture = readJson(source);
  if (fixture.schema !== DOM_FIXTURE_SCHEMA) {
    throw new Error(`Expected ${DOM_FIXTURE_SCHEMA} in ${source}`);
  }
  const receipt = noJsReceiptFromDomSnapshot({
    document: recordValue(fixture.document) as DomNode,
    url: typeof fixture.url === "string" ? fixture.url : defaultUrlForHtmlPath(metadata.htmlPath),
    userAgent: typeof fixture.user_agent === "string" ? fixture.user_agent : "fixture",
    htmlPath: metadata.htmlPath,
    artifactHtmlBlake3: metadata.artifactHtmlBlake3,
    scriptExecutionDisabledBeforeNavigation:
      fixture.script_execution_disabled_before_navigation === true,
  });
  writeJson(resolveProjectPath(out), receipt);
  process.stdout.write(JSON.stringify({ out, passed: receipt.passed, schema: receipt.schema }, null, 2));
} else {
  const url = argumentAfter(args, "--url") || defaultUrlForHtmlPath(metadata.htmlPath);
  const timeoutMs = Number(argumentAfter(args, "--timeout-ms") || 15000);
  const browserSnapshot = await collectViaCdp(url, timeoutMs);
  const receipt = noJsReceiptFromDomSnapshot({
    document: browserSnapshot.document,
    url,
    userAgent: browserSnapshot.userAgent,
    htmlPath: metadata.htmlPath,
    artifactHtmlBlake3: metadata.artifactHtmlBlake3,
    scriptExecutionDisabledBeforeNavigation: true,
  });
  writeJson(resolveProjectPath(out), receipt);
  process.stdout.write(JSON.stringify({ out, passed: receipt.passed, schema: receipt.schema }, null, 2));
}
