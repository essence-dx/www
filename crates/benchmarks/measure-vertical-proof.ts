const fs = require("fs");
const http = require("http");
const os = require("os");
const path = require("path");
const { spawn, spawnSync } = require("child_process");
const { performance } = require("perf_hooks");
const zlib = require("zlib");
const { dxWwwCargoRunArgs } = require("./dx-www-cli-paths.ts");

const root = path.resolve(__dirname, "..");
const fixtureRoot = path.join(__dirname, "vertical-proof");
const reportDir = path.join(__dirname, "reports");
const historyDir = path.join(reportDir, "vertical-proof-history");
const sampleCount = Number(process.env.DX_VERTICAL_CHROME_SAMPLES || 5);
const fixtureMode = process.env.DX_VERTICAL_PROOF_MODE || "forge-package";
const enforceBudgetGate = process.env.DX_VERTICAL_BUDGET_GATE === "1";
const defaultBudgetConfigPath =
  process.env.DX_VERTICAL_BUDGET_CONFIG ||
  path.join(root, "dx.config.toml");

fs.mkdirSync(reportDir, { recursive: true });
fs.mkdirSync(historyDir, { recursive: true });

const fixtureModes = {
  component: {
    fixture: "benchmarks/vertical-proof",
    packageArgs: ["--component", "components/ProofCard.tsx"],
    description: "older local .tsx component fixture",
  },
  "forge-package": {
    fixture: "generated:forge-package",
    packageArgs: ["--package", "ui/button"],
    description: "source-owned Forge UI button package fixture",
  },
  "forge-icon": {
    fixture: "generated:forge-icon",
    packageArgs: ["--package", "icon/search"],
    description: "source-owned dx/icon/search selected-icon package fixture",
  },
  "forge-combo": {
    fixture: "generated:forge-combo",
    packageArgs: ["--package", "ui/button", "--package", "icon/search"],
    description: "combined source-owned Forge UI button plus dx/icon/search fixture",
  },
  "forge-site": {
    fixture: "generated:forge-site",
    packageArgs: [],
    routePath: "/forge",
    description: "compact public /forge launch page generated from release proof",
  },
  "forge-scorecard": {
    fixture: "generated:forge-scorecard",
    packageArgs: [],
    routePath: "/forge/scorecard",
    description: "public /forge/scorecard package scorecard page generated from Forge evidence",
  },
  "forge-ci": {
    fixture: "generated:forge-ci",
    packageArgs: [],
    routePath: "/forge/ci",
    description: "public /forge/ci CI evidence page generated from Forge smoke and readiness models",
  },
  "forge-evidence": {
    fixture: "generated:forge-evidence",
    packageArgs: [],
    routePath: "/forge/evidence",
    description: "public /forge/evidence index page generated from Forge public evidence links",
  },
  "forge-releases": {
    fixture: "generated:forge-releases",
    packageArgs: [],
    routePath: "/forge/releases",
    description: "public /forge/releases history page generated from Forge release-history evidence",
  },
  "forge-changelog": {
    fixture: "generated:forge-changelog",
    packageArgs: [],
    routePath: "/forge/changelog",
    description: "public /forge/changelog page generated from Forge launch-changelog evidence",
  },
  "forge-adoption": {
    fixture: "generated:forge-adoption",
    packageArgs: [],
    routePath: "/forge/adoption",
    description: "public /forge/adoption page generated from local Forge adoption evidence",
  },
};

const forgeRouteBudgetProfiles = {
  "forge-site": {
    configKey: "site",
    profile: "compact-forge-launch",
    envPrefix: "DX_FORGE_SITE_MAX",
    defaults: {
      decoded_bytes: 30_000,
      brotli_bytes: 5_500,
      http_route_median_ms: 5,
      chrome_load_event_ms: 75,
    },
  },
  "forge-scorecard": {
    configKey: "scorecard",
    profile: "compact-forge-scorecard",
    envPrefix: "DX_FORGE_SCORECARD_MAX",
    defaults: {
      decoded_bytes: 20_000,
      brotli_bytes: 3_500,
      http_route_median_ms: 5,
      chrome_load_event_ms: 75,
    },
  },
  "forge-ci": {
    configKey: "ci",
    profile: "compact-forge-ci",
    envPrefix: "DX_FORGE_CI_MAX",
    defaults: {
      decoded_bytes: 16_000,
      brotli_bytes: 3_000,
      http_route_median_ms: 5,
      chrome_load_event_ms: 75,
    },
  },
  "forge-evidence": {
    configKey: "evidence",
    profile: "compact-forge-evidence",
    envPrefix: "DX_FORGE_EVIDENCE_MAX",
    defaults: {
      decoded_bytes: 24_000,
      brotli_bytes: 4_000,
      http_route_median_ms: 5,
      chrome_load_event_ms: 75,
    },
  },
  "forge-releases": {
    configKey: "releases",
    profile: "compact-forge-releases",
    envPrefix: "DX_FORGE_RELEASES_MAX",
    defaults: {
      decoded_bytes: 18_000,
      brotli_bytes: 3_000,
      http_route_median_ms: 5,
      chrome_load_event_ms: 75,
    },
  },
  "forge-changelog": {
    configKey: "changelog",
    profile: "compact-forge-changelog",
    envPrefix: "DX_FORGE_CHANGELOG_MAX",
    defaults: {
      decoded_bytes: 18_000,
      brotli_bytes: 3_000,
      http_route_median_ms: 5,
      chrome_load_event_ms: 75,
    },
  },
  "forge-adoption": {
    configKey: "adoption",
    profile: "compact-forge-adoption",
    envPrefix: "DX_FORGE_ADOPTION_MAX",
    defaults: {
      decoded_bytes: 30_000,
      brotli_bytes: 5_500,
      http_route_median_ms: 5,
      chrome_load_event_ms: 75,
    },
  },
};

function writeForgePackageFixture(tempRoot) {
  fs.mkdirSync(path.join(tempRoot, "pages"), { recursive: true });
  fs.writeFileSync(
    path.join(tempRoot, "pages", "index.html"),
    String.raw`<main class="mx-auto grid max-w-3xl gap-4 p-6">
        <section class="rounded-lg border border-neutral-200 bg-neutral-50 p-5">
            <p class="text-sm font-medium text-neutral-500">Phase 3 Forge package proof</p>
            <h1 class="mt-2 text-3xl font-semibold text-neutral-950">DX-WWW source-owned package route</h1>
            <p class="mt-3 text-base text-neutral-700">
                This page materializes the Forge UI button through DX Forge, renders crawlable fallback HTML,
                emits a canonical DXPK packet, and keeps the measured route free of node_modules.
            </p>
        </section>

        <section class="rounded-lg border border-neutral-200 bg-white p-4 shadow-sm">
            <p class="text-xs font-semibold uppercase tracking-wide text-neutral-500">Forge package</p>
            <h2 class="mt-2 text-lg font-semibold text-neutral-950">Materialized Button</h2>
            <div class="mt-3">
                <button class="rounded-md bg-neutral-950 px-3 py-2 text-sm font-medium text-white" data-forge-package="ui-components">
                    Launch with Forge
                </button>
            </div>
        </section>

        <section class="rounded-lg border border-neutral-200 bg-white p-4 shadow-sm">
            <p class="text-xs font-semibold uppercase tracking-wide text-neutral-500">Interaction</p>
            <h2 class="mt-2 text-lg font-semibold text-neutral-950">Source-owned control surface</h2>
            <p class="mt-3 text-sm text-neutral-700">Interactive proof belongs in the TSX App Router runtime lane.</p>
        </section>
    </main>
`
  );
}

function writeForgeIconFixture(tempRoot) {
  fs.mkdirSync(path.join(tempRoot, "pages"), { recursive: true });
  fs.writeFileSync(
    path.join(tempRoot, "pages", "index.html"),
    String.raw`<main class="mx-auto grid max-w-3xl gap-4 p-6">
        <section class="rounded-lg border border-neutral-200 bg-neutral-50 p-5">
            <p class="text-sm font-medium text-neutral-500">Phase 3 selected-icon proof</p>
            <h1 class="mt-2 text-3xl font-semibold text-neutral-950">DX-WWW source-owned icon route</h1>
            <p class="mt-3 text-base text-neutral-700">
                This page materializes only the Search icon plus its helper files through DX Forge,
                emits a canonical DXPK packet, and keeps the measured route free of node_modules.
            </p>
        </section>

        <section class="rounded-lg border border-neutral-200 bg-white p-4 shadow-sm">
            <p class="text-xs font-semibold uppercase tracking-wide text-neutral-500">Forge package</p>
            <h2 class="mt-2 text-lg font-semibold text-neutral-950">Materialized icon</h2>
            <button class="mt-3 inline-flex items-center gap-2 rounded-md border border-neutral-200 px-3 py-2 text-sm font-medium text-neutral-950">
                <icon name="pack:search" />
                <span>Search the registry</span>
            </button>
        </section>

        <section class="rounded-lg border border-neutral-200 bg-white p-4 shadow-sm">
            <p class="text-xs font-semibold uppercase tracking-wide text-neutral-500">Interaction</p>
            <h2 class="mt-2 text-lg font-semibold text-neutral-950">Source-owned control surface</h2>
            <p class="mt-3 text-sm text-neutral-700">Interactive proof belongs in the TSX App Router runtime lane.</p>
        </section>
    </main>
`
  );
}

function writeForgeComboFixture(tempRoot) {
  fs.mkdirSync(path.join(tempRoot, "pages"), { recursive: true });
  fs.writeFileSync(
    path.join(tempRoot, "pages", "index.html"),
    String.raw`<main class="mx-auto grid max-w-3xl gap-4 p-6">
        <section class="rounded-lg border border-neutral-200 bg-neutral-50 p-5">
            <p class="text-sm font-medium text-neutral-500">Phase 3 multi-package proof</p>
            <h1 class="mt-2 text-3xl font-semibold text-neutral-950">DX-WWW combined Forge route</h1>
            <p class="mt-3 text-base text-neutral-700">
                This page materializes a source-owned Forge UI button and selected Search icon together,
                proving that Forge can compose editable packages without node_modules.
            </p>
        </section>

        <section class="rounded-lg border border-neutral-200 bg-white p-4 shadow-sm">
            <p class="text-xs font-semibold uppercase tracking-wide text-neutral-500">Forge packages</p>
            <h2 class="mt-2 text-lg font-semibold text-neutral-950">Button plus icon</h2>
            <div class="mt-3">
                <button class="inline-flex items-center gap-2 rounded-md bg-neutral-950 px-3 py-2 text-sm font-medium text-white" data-forge-package="ui-components dx-icons">
                    <icon name="pack:search" />
                    Search with Forge
                </button>
            </div>
        </section>

        <section class="rounded-lg border border-neutral-200 bg-white p-4 shadow-sm">
            <p class="text-xs font-semibold uppercase tracking-wide text-neutral-500">Interaction</p>
            <h2 class="mt-2 text-lg font-semibold text-neutral-950">Source-owned control surface</h2>
            <p class="mt-3 text-sm text-neutral-700">Interactive proof belongs in the TSX App Router runtime lane.</p>
        </section>
    </main>
`
  );
}

function copyForgeReleaseHistoryFixture(tempRoot) {
  const sourceDir = path.join(root, "benchmarks", "reports");
  const targetDir = path.join(tempRoot, "benchmarks", "reports");
  const jsonSource = path.join(sourceDir, "forge-public-release-history.json");
  const mdSource = path.join(sourceDir, "forge-public-release-history.md");

  if (!fs.existsSync(jsonSource)) {
    throw new Error(`Missing Forge release history fixture: ${jsonSource}`);
  }

  fs.mkdirSync(targetDir, { recursive: true });
  fs.copyFileSync(jsonSource, path.join(targetDir, "forge-public-release-history.json"));
  if (fs.existsSync(mdSource)) {
    fs.copyFileSync(mdSource, path.join(targetDir, "forge-public-release-history.md"));
  }
}

function copyFixture(tempRoot) {
  fs.mkdirSync(tempRoot, { recursive: true });
  if (fixtureMode === "component") {
    fs.tsxSync(path.join(fixtureRoot, "pages"), path.join(tempRoot, "pages"), { recursive: true });
    fs.tsxSync(path.join(fixtureRoot, "components"), path.join(tempRoot, "components"), { recursive: true });
    return;
  }
  if (fixtureMode === "forge-package") {
    writeForgePackageFixture(tempRoot);
    return;
  }
  if (fixtureMode === "forge-icon") {
    writeForgeIconFixture(tempRoot);
    return;
  }
  if (fixtureMode === "forge-combo") {
    writeForgeComboFixture(tempRoot);
    return;
  }
  if (["forge-releases", "forge-changelog"].includes(fixtureMode)) {
    copyForgeReleaseHistoryFixture(tempRoot);
    return;
  }
  if (["forge-site", "forge-scorecard", "forge-ci", "forge-evidence"].includes(fixtureMode)) {
    return;
  }
  if (fixtureMode === "forge-adoption") {
    return;
  }
  if (!fixtureModes[fixtureMode]) {
    throw new Error(`Unsupported DX_VERTICAL_PROOF_MODE: ${fixtureMode}`);
  }
}

function runDxProve(tempRoot) {
  const cargoFlags = ["-q", "--no-default-features", "--features", "cli"];
  if (
    [
      "forge-site",
      "forge-scorecard",
      "forge-ci",
      "forge-evidence",
      "forge-releases",
      "forge-changelog",
      "forge-adoption",
    ].includes(fixtureMode)
  ) {
    if (fixtureMode === "forge-adoption") {
      const adoptionResult = spawnSync(
        "cargo",
        [
          ...dxWwwCargoRunArgs(root, [], cargoFlags),
          "forge",
          "adoption-smoke",
          "--project",
          ".",
          "--format",
          "json",
          "--fail-under",
          "90",
          "--quiet",
        ],
        {
          cwd: tempRoot,
          encoding: "utf8",
          windowsHide: true,
        }
      );
      if (adoptionResult.status !== 0) {
        throw new Error(
          [
            "dx forge adoption-smoke failed",
            `exit: ${adoptionResult.status}`,
            adoptionResult.stdout,
            adoptionResult.stderr,
          ].join("\n")
        );
      }
    }

    const args = [
      ...dxWwwCargoRunArgs(root, [], cargoFlags),
      "prove",
      "vertical",
      "--fixture",
      fixtureMode,
      "--out",
      "proof",
      "--write",
      "--format",
      "json",
      "--quiet",
    ];
    const result = spawnSync("cargo", args, {
      cwd: tempRoot,
      encoding: "utf8",
      windowsHide: true,
    });
    if (result.status !== 0) {
      throw new Error(
        [
          `dx prove vertical ${fixtureMode} failed`,
          `exit: ${result.status}`,
          result.stdout,
          result.stderr,
        ].join("\n")
      );
    }
    return;
  }

  const args = [
    ...dxWwwCargoRunArgs(root, [], cargoFlags),
    "prove",
    "vertical",
    "--page",
    "pages/index.html",
    ...fixtureModes[fixtureMode].packageArgs,
    "--out",
    "proof",
    "--write",
    "--format",
    "json",
    "--quiet",
  ];
  const result = spawnSync("cargo", args, {
    cwd: tempRoot,
    encoding: "utf8",
    windowsHide: true,
  });
  if (result.status !== 0) {
    throw new Error(
      [
        "dx prove vertical failed",
        `exit: ${result.status}`,
        result.stdout,
        result.stderr,
      ].join("\n")
    );
  }
}

function mimeType(filePath) {
  switch (path.extname(filePath).toLowerCase()) {
    case ".css":
      return "text/css; charset=utf-8";
    case ".html":
      return "text/html; charset=utf-8";
    case ".js":
      return "application/javascript; charset=utf-8";
    case ".json":
      return "application/json; charset=utf-8";
    case ".wasm":
      return "application/wasm";
    default:
      return "application/octet-stream";
  }
}

function createStaticServer(baseDir) {
  return http.createServer((req, res) => {
    const url = new URL(req.url, "http://127.0.0.1");
    const base = path.resolve(baseDir);
    const requestedPath = url.pathname === "/" ? "/index.html" : decodeURIComponent(url.pathname);
    const candidatePaths = [requestedPath];
    if (!path.extname(requestedPath)) {
      candidatePaths.push(`${requestedPath}.html`);
    }
    const filePath = candidatePaths
      .map((candidate) => path.resolve(path.normalize(path.join(baseDir, candidate))))
      .find(
        (candidate) =>
          candidate.startsWith(base) && fs.existsSync(candidate) && fs.statSync(candidate).isFile()
      );
    if (!filePath) {
      res.writeHead(404, { "content-type": "text/plain; charset=utf-8" });
      res.end("Not found");
      return;
    }

    const body = fs.readFileSync(filePath);
    res.writeHead(200, {
      "cache-control": "no-cache",
      "content-length": body.length,
      "content-type": mimeType(filePath),
    });
    res.end(body);
  });
}

function listen(server) {
  return new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const { port } = server.address();
      resolve({
        baseUrl: `http://127.0.0.1:${port}`,
        close: () =>
          new Promise((done) => {
            server.closeIdleConnections?.();
            server.closeAllConnections?.();
            server.close(done);
          }),
      });
    });
  });
}

async function fetchResource(url) {
  const started = performance.now();
  const response = await fetch(url, { headers: { "cache-control": "no-cache" } });
  const buffer = Buffer.from(await response.arrayBuffer());
  return {
    url,
    status: response.status,
    ok: response.ok,
    decoded_bytes: buffer.length,
    elapsed_ms: Number((performance.now() - started).toFixed(3)),
    content_length: Number(response.headers.get("content-length") || buffer.length),
    buffer,
  };
}

function extractAssets(html, baseUrl) {
  const urls = new Set();
  const pattern = /(?:src|href)="([^"]+)"/g;
  let match;
  while ((match = pattern.exec(html))) {
    const value = match[1];
    if (value.startsWith("data:") || value.startsWith("#")) {
      continue;
    }
    const url = new URL(value, baseUrl);
    if (url.origin === new URL(baseUrl).origin) {
      urls.add(url.toString());
    }
  }
  return [...urls].sort();
}

function compress(buffer) {
  return {
    raw_bytes: buffer.length,
    gzip_bytes: zlib.gzipSync(buffer, { level: 9 }).length,
    brotli_bytes: zlib.brotliCompressSync(buffer, {
      params: { [zlib.constants.BROTLI_PARAM_QUALITY]: 11 },
    }).length,
  };
}

function summarize(values) {
  const sorted = [...values].sort((a, b) => a - b);
  const sum = values.reduce((total, value) => total + value, 0);
  const pick = (percentile) =>
    sorted[Math.min(sorted.length - 1, Math.max(0, Math.ceil(sorted.length * percentile) - 1))];
  return {
    samples: values.length,
    min: Number(sorted[0].toFixed(3)),
    median: Number(pick(0.5).toFixed(3)),
    mean: Number((sum / values.length).toFixed(3)),
    p95: Number(pick(0.95).toFixed(3)),
    max: Number(sorted[sorted.length - 1].toFixed(3)),
  };
}

async function measureHttp(baseUrl, routePath = "/") {
  const routeUrl = new URL(routePath, `${baseUrl}/`).toString();
  const index = await fetchResource(routeUrl);
  if (!index.ok) {
    throw new Error(`vertical proof returned HTTP ${index.status}`);
  }
  const assets = await Promise.all(
    extractAssets(index.buffer.toString("utf8"), routeUrl).map((assetUrl) => fetchResource(assetUrl))
  );
  const resources = [index, ...assets];
  const body = Buffer.concat(resources.map((resource) => resource.buffer));
  const routeSamples = [];
  for (let index = 0; index < 12; index += 1) {
    routeSamples.push((await fetchResource(routeUrl)).elapsed_ms);
  }
  return {
    route_url: routeUrl,
    route_path: new URL(routeUrl).pathname,
    resource_count: resources.length,
    total_decoded_bytes: resources.reduce((sum, item) => sum + item.decoded_bytes, 0),
    content_length_bytes: resources.reduce((sum, item) => sum + item.content_length, 0),
    compression_estimate: compress(body),
    route_timing_ms: summarize(routeSamples),
    resources: resources.map(({ buffer, url, ...resource }) => ({
      path: new URL(url).pathname,
      ...resource,
    })),
  };
}

function findChrome() {
  const candidates = [
    path.join(process.env.ProgramFiles || "", "Google", "Chrome", "Application", "chrome.exe"),
    path.join(process.env["ProgramFiles(x86)"] || "", "Google", "Chrome", "Application", "chrome.exe"),
    path.join(process.env.LOCALAPPDATA || "", "Google", "Chrome", "Application", "chrome.exe"),
    path.join(process.env.ProgramFiles || "", "Microsoft", "Edge", "Application", "msedge.exe"),
    path.join(process.env["ProgramFiles(x86)"] || "", "Microsoft", "Edge", "Application", "msedge.exe"),
  ];
  return candidates.find((candidate) => candidate && fs.existsSync(candidate));
}

async function freePort() {
  const server = http.createServer();
  return new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const { port } = server.address();
      server.close(() => resolve(port));
    });
  });
}

async function waitFor(url, timeoutMs = 20000) {
  const started = Date.now();
  let lastError;
  while (Date.now() - started < timeoutMs) {
    try {
      const response = await fetch(url);
      if (response.ok) {
        await response.arrayBuffer();
        return;
      }
    } catch (error) {
      lastError = error;
    }
    await new Promise((resolve) => setTimeout(resolve, 250));
  }
  throw lastError || new Error(`Timed out waiting for ${url}`);
}

async function startChrome() {
  if (typeof WebSocket === "undefined") {
    return null;
  }
  const chromePath = findChrome();
  if (!chromePath) {
    return null;
  }

  const port = await freePort();
  const userDataDir = path.join(os.tmpdir(), `dx-www-vertical-chrome-${Date.now()}`);
  const child = spawn(
    chromePath,
    [
      "--headless=new",
      "--disable-gpu",
      "--no-first-run",
      "--no-default-browser-check",
      "--disable-background-networking",
      "--disable-features=Translate,BackForwardCache",
      `--remote-debugging-port=${port}`,
      `--user-data-dir=${userDataDir}`,
      "about:blank",
    ],
    { stdio: "ignore", windowsHide: true }
  );
  const baseUrl = `http://127.0.0.1:${port}`;
  await waitFor(`${baseUrl}/json/version`);
  return {
    baseUrl,
    close: () => {
      killProcessTree(child);
      try {
        fs.rmSync(userDataDir, { recursive: true, force: true });
      } catch {
        // Windows can keep Chrome profile stores locked briefly after process exit.
      }
    },
  };
}

function killProcessTree(child) {
  if (process.platform === "win32") {
    const result = spawnSync("taskkill", ["/PID", String(child.pid), "/T", "/F"], {
      stdio: "ignore",
      timeout: 5000,
      windowsHide: true,
    });
    if (result.error) {
      child.kill("SIGKILL");
    }
  } else {
    child.kill("SIGTERM");
  }
}

async function chromeNewPage(chrome) {
  let response = await fetch(`${chrome.baseUrl}/json/new`, { method: "PUT" });
  if (!response.ok) {
    response = await fetch(`${chrome.baseUrl}/json/new`);
  }
  if (!response.ok) {
    throw new Error(`Chrome refused a new page: ${response.status}`);
  }
  return response.json();
}

function connectCdp(wsUrl) {
  return new Promise((resolve, reject) => {
    const ws = new WebSocket(wsUrl);
    let id = 0;
    const pending = new Map();
    const waiters = new Map();
    const timer = setTimeout(() => reject(new Error("Timed out connecting to Chrome CDP")), 10000);

    ws.addEventListener("open", () => {
      clearTimeout(timer);
      resolve({
        send(method, params = {}) {
          id += 1;
          const messageId = id;
          ws.send(JSON.stringify({ id: messageId, method, params }));
          return new Promise((resolveSend, rejectSend) => {
            pending.set(messageId, { resolve: resolveSend, reject: rejectSend });
          });
        },
        waitFor(method, timeoutMs) {
          return new Promise((resolveEvent, rejectEvent) => {
            const timeout = setTimeout(() => {
              rejectEvent(new Error(`Timed out waiting for ${method}`));
            }, timeoutMs);
            waiters.set(method, { resolve: resolveEvent, timeout });
          });
        },
        close() {
          ws.close();
        },
      });
    });
    ws.addEventListener("error", reject);
    ws.addEventListener("message", (event) => {
      const message = JSON.parse(event.data);
      if (message.id && pending.has(message.id)) {
        const request = pending.get(message.id);
        pending.delete(message.id);
        if (message.error) {
          request.reject(new Error(message.error.message));
        } else {
          request.resolve(message.result);
        }
      }
      if (message.method && waiters.has(message.method)) {
        const waiter = waiters.get(message.method);
        waiters.delete(message.method);
        clearTimeout(waiter.timeout);
        waiter.resolve(message.params || {});
      }
    });
  });
}

async function measureChromeOnce(chrome, url) {
  const tab = await chromeNewPage(chrome);
  const client = await connectCdp(tab.webSocketDebuggerUrl);
  try {
    await client.send("Page.enable");
    await client.send("Runtime.enable");
    await client.send("Network.enable");
    await client.send("Network.setCacheDisabled", { cacheDisabled: true });
    const loaded = client.waitFor("Page.loadEventFired", 20000);
    await client.send("Page.navigate", { url });
    await loaded;
    await new Promise((resolve) => setTimeout(resolve, 50));
    const result = await client.send("Runtime.evaluate", {
      awaitPromise: true,
      returnByValue: true,
      expression: `(async () => {
        await Promise.race([
          window.__DX_PACKET_APPLIED__ || Promise.resolve(null),
          new Promise((resolve) => setTimeout(resolve, 1000))
        ]);
        const nav = performance.getEntriesByType("navigation")[0];
        const resources = performance.getEntriesByType("resource");
        const target = document.getElementById("dx-state-count");
        const button = document.querySelector("button[data-dx-on-click]");
        const before = target ? target.textContent : null;
        const clickStarted = performance.now();
        if (button) button.click();
        const clickUpdateMs = performance.now() - clickStarted;
        const after = target ? target.textContent : null;
        return {
          title: document.title,
          dom_nodes: document.getElementsByTagName("*").length,
          scripts: document.scripts.length,
          buttons: document.querySelectorAll("button").length,
          nav_duration_ms: nav ? nav.duration : 0,
          dom_content_loaded_ms: nav ? nav.domContentLoadedEventEnd - nav.startTime : 0,
          load_event_ms: nav ? nav.loadEventEnd - nav.startTime : 0,
          transfer_size: nav ? nav.transferSize + Array.from(resources).reduce((sum, item) => sum + item.transferSize, 0) : 0,
          encoded_body_size: nav ? nav.encodedBodySize + Array.from(resources).reduce((sum, item) => sum + item.encodedBodySize, 0) : 0,
          decoded_body_size: nav ? nav.decodedBodySize + Array.from(resources).reduce((sum, item) => sum + item.decodedBodySize, 0) : 0,
          resource_count: resources.length + 1,
          dx_packet_status: document.documentElement.dataset.dxPacketStatus || null,
          dx_packet_sections: Number(document.documentElement.dataset.dxPacketSections || 0),
          dx_packet_payload_bytes: Number(document.documentElement.dataset.dxPacketPayloadBytes || 0),
          dx_packet_template_count: Number(document.documentElement.dataset.dxPacketTemplateCount || 0),
          dx_packet_route: document.documentElement.dataset.dxPacketRoute || null,
          dx_packet_applied: document.documentElement.dataset.dxPacketStatus === "applied",
          counter_before: before,
          counter_after: after,
          counter_click_update_ms: clickUpdateMs,
          interaction_works: before === "0" && after === "1"
        };
      })()`,
    });
    return normalizeMetrics(result.result.value);
  } finally {
    client.close();
    await fetch(`${chrome.baseUrl}/json/close/${tab.id}`).catch(() => {});
  }
}

function normalizeMetrics(metrics) {
  return Object.fromEntries(
    Object.entries(metrics).map(([key, value]) => [
      key,
      typeof value === "number" ? Number(value.toFixed(3)) : value,
    ])
  );
}

async function measureChrome(chrome, url) {
  if (!chrome) {
    return null;
  }
  const warmup = await measureChromeOnce(chrome, url);
  const samples = [];
  for (let index = 0; index < sampleCount; index += 1) {
    samples.push(await measureChromeOnce(chrome, url));
  }
  const median = (key) => summarize(samples.map((sample) => sample[key])).median;
  const last = samples[samples.length - 1];
  return {
    enabled: true,
    samples: samples.length,
    warmup_excluded: true,
    title: last.title,
    dom_nodes: last.dom_nodes,
    scripts: last.scripts,
    buttons: last.buttons,
    nav_duration_ms: median("nav_duration_ms"),
    dom_content_loaded_ms: median("dom_content_loaded_ms"),
    load_event_ms: median("load_event_ms"),
    transfer_size: median("transfer_size"),
    encoded_body_size: median("encoded_body_size"),
    decoded_body_size: median("decoded_body_size"),
    dx_packet_status: last.dx_packet_status,
    dx_packet_sections: last.dx_packet_sections,
    dx_packet_payload_bytes: last.dx_packet_payload_bytes,
    dx_packet_template_count: last.dx_packet_template_count,
    dx_packet_route: last.dx_packet_route,
    dx_packet_applied: samples.every((sample) => sample.dx_packet_applied),
    counter_click_update_ms: median("counter_click_update_ms"),
    interaction_works: samples.every((sample) => sample.interaction_works),
    warmup,
    raw_samples: samples,
  };
}

function writeReports(report) {
  const jsonPath = path.join(reportDir, "vertical-proof-measurement.json");
  const mdPath = path.join(reportDir, "vertical-proof-measurement.md");
  const snapshot = snapshotPaths(report);
  report.snapshot = {
    json: relativeReportPath(snapshot.jsonPath),
    markdown: relativeReportPath(snapshot.mdPath),
    index_json: relativeReportPath(snapshot.indexPath),
    index_markdown: relativeReportPath(snapshot.indexMdPath),
  };
  const comparisonRows = report.comparison
    ? [
        "",
        "## Previous Report Comparison",
        "",
        "| Metric | Previous | Current | Delta |",
        "| --- | ---: | ---: | ---: |",
        `| Fixture mode | ${report.comparison.previous_fixture_mode} | ${report.fixture_mode} | - |`,
        `| Forge packages | ${report.comparison.previous_forge_packages} | ${report.comparison.current_forge_packages} | ${formatDelta(report.comparison.delta_forge_packages)} |`,
        `| Decoded bytes | ${report.comparison.previous_decoded_bytes} B | ${report.comparison.current_decoded_bytes} B | ${formatDelta(report.comparison.delta_decoded_bytes)} B |`,
        `| Brotli estimate | ${report.comparison.previous_brotli_bytes} B | ${report.comparison.current_brotli_bytes} B | ${formatDelta(report.comparison.delta_brotli_bytes)} B |`,
        `| HTTP route median | ${report.comparison.previous_http_route_median_ms} ms | ${report.comparison.current_http_route_median_ms} ms | ${formatDelta(report.comparison.delta_http_route_median_ms)} ms |`,
        `| Chrome load event median | ${formatNullable(report.comparison.previous_chrome_load_event_ms, " ms")} | ${formatNullable(report.comparison.current_chrome_load_event_ms, " ms")} | ${formatNullable(report.comparison.delta_chrome_load_event_ms, " ms", true)} |`,
      ]
    : [];
  const markdown = renderReportMarkdown(report, comparisonRows);
  const json = `${JSON.stringify(report, null, 2)}\n`;

  fs.writeFileSync(jsonPath, json);
  fs.writeFileSync(mdPath, markdown);
  fs.writeFileSync(snapshot.jsonPath, json);
  fs.writeFileSync(snapshot.mdPath, markdown);
  updateHistoryIndex(report, snapshot);
  const triagePath = report.budget ? path.join(reportDir, "vertical-proof-triage.md") : null;
  if (triagePath) {
    fs.writeFileSync(triagePath, renderBudgetTriageMarkdown(report));
  }

  return {
    jsonPath,
    mdPath,
    triagePath,
    snapshotJsonPath: snapshot.jsonPath,
    snapshotMdPath: snapshot.mdPath,
    historyIndexPath: snapshot.indexPath,
    historyIndexMarkdownPath: snapshot.indexMdPath,
  };
}

function renderReportMarkdown(report, comparisonRows) {
  const budgetRows = report.budget
    ? [
        "",
        "## Delivery Budget",
        "",
        `Profile: \`${report.budget.profile}\``,
        `Config: ${
          report.budget.config_path ? `\`${report.budget.config_path}\`` : "defaults and environment"
        }`,
        `Enforced: ${report.budget.enforced ? "yes" : "no"}`,
        "",
        "| Metric | Current | Max | Status |",
        "| --- | ---: | ---: | --- |",
        ...report.budget.checks.map(
          (check) =>
            `| ${check.metric} | ${formatNullable(check.value, check.unit)} | ${formatNullable(
              check.max,
              check.unit
            )} | ${check.passed ? "pass" : "fail"} |`
        ),
        "",
        `Budget passed: ${report.budget.passed}`,
      ]
    : [];
  return [
    "# DX-WWW Vertical Proof Measurement",
    "",
    `Generated: ${report.generated_at}`,
    `Snapshot JSON: ${report.snapshot.json}`,
    `Snapshot Markdown: ${report.snapshot.markdown}`,
    "",
    "| Metric | Value |",
    "| --- | ---: |",
    `| HTTP resources | ${report.http.resource_count} |`,
    `| HTTP route | ${report.http.route_path} |`,
    `| Route delivery | ${report.delivery.route_mode} |`,
    `| DXPK proof artifact | ${report.delivery.packet_artifact_written} |`,
    `| Runtime asset | ${report.delivery.runtime_asset_written} |`,
    `| Decoded bytes | ${report.http.total_decoded_bytes} B |`,
    `| Content-Length bytes | ${report.http.content_length_bytes} B |`,
    `| gzip estimate | ${report.http.compression_estimate.gzip_bytes} B |`,
    `| Brotli estimate | ${report.http.compression_estimate.brotli_bytes} B |`,
    `| HTTP route median | ${report.http.route_timing_ms.median} ms |`,
    `| Chrome enabled | ${report.chrome ? "yes" : "no"} |`,
    `| Chrome load event median | ${report.chrome ? `${report.chrome.load_event_ms} ms` : "n/a"} |`,
    `| Chrome transfer size | ${report.chrome ? `${report.chrome.transfer_size} B` : "n/a"} |`,
    `| DXPK applied in DOM | ${report.chrome ? report.chrome.dx_packet_applied : "n/a"} |`,
    `| DXPK sections | ${report.chrome ? report.chrome.dx_packet_sections : "n/a"} |`,
    `| Counter click update median | ${report.chrome ? `${report.chrome.counter_click_update_ms} ms` : "n/a"} |`,
    `| Interaction works | ${report.chrome ? report.chrome.interaction_works : "n/a"} |`,
    `| Forge packages | ${report.proof.forge_packages.length} |`,
    `| Delivery budget passed | ${report.budget ? report.budget.passed : "n/a"} |`,
    ...budgetRows,
    ...comparisonRows,
    "",
    `This measures the compiler-backed vertical slice generated by \`dx prove vertical\`, served over localhost HTTP, and loaded in headless Chrome when Chrome is available. The current fixture mode is \`${fixtureMode}\` (${fixtureModes[fixtureMode].description}). Set \`DX_VERTICAL_PROOF_MODE=component\`, \`forge-package\`, \`forge-icon\`, \`forge-combo\`, \`forge-site\`, \`forge-scorecard\`, \`forge-ci\`, \`forge-evidence\`, \`forge-releases\`, \`forge-changelog\`, or \`forge-adoption\` to compare verticals.`,
    "",
  ].join("\n");
}

function renderBudgetTriageMarkdown(report) {
  const budget = report.budget;
  const failedChecks = budget ? budget.checks.filter((check) => !check.passed) : [];
  const status = budget ? (budget.passed ? "passing" : "failing") : "not configured";
  const lines = [
    "# DX-WWW Launch Budget Triage",
    "",
    `Generated: ${report.generated_at || "n/a"}`,
    `Fixture mode: ${report.fixture_mode || "n/a"}`,
    `Status: ${status}`,
  ];

  if (!budget) {
    return [
      ...lines,
      "",
      "No delivery budget was configured for this fixture mode.",
      "",
    ].join("\n");
  }

  lines.push(
    `Profile: ${budget.profile}`,
    `Config: ${budget.config_path || "defaults and environment"}`,
    `Enforced: ${budget.enforced ? "yes" : "no"}`,
    "",
    "## Failed Checks",
    ""
  );

  if (failedChecks.length === 0) {
    lines.push("- `pass`: no launch budget failures.");
  } else {
    for (const check of failedChecks) {
      lines.push(
        `- \`fail\` ${check.metric}: ${formatNullable(check.value, check.unit)} > ${formatNullable(
          check.max,
          check.unit
        )}`
      );
    }
  }

  lines.push(
    "",
    "## All Checks",
    "",
    "| Metric | Current | Max | Status |",
    "| --- | ---: | ---: | --- |"
  );
  for (const check of budget.checks) {
    lines.push(
      `| ${check.metric} | ${formatNullable(check.value, check.unit)} | ${formatNullable(
        check.max,
        check.unit
      )} | ${check.passed ? "pass" : "fail"} |`
    );
  }

  lines.push("", "## First Actions", "");
  if (failedChecks.length === 0) {
    lines.push("- Keep this triage artifact as the clean launch-budget baseline.");
  } else {
    if (failedChecks.some((check) => /decoded|brotli/i.test(check.metric))) {
      lines.push("- Inspect route payload size, non-critical evidence detail, and static HTML output before relaxing byte thresholds.");
    }
    if (failedChecks.some((check) => /http route/i.test(check.metric))) {
      lines.push("- Re-run on an idle runner, then inspect the local static server timing path if HTTP median stays high.");
    }
    if (failedChecks.some((check) => /chrome/i.test(check.metric))) {
      lines.push("- Re-run once on an idle machine, then inspect rendered resource count and runtime/script injection.");
    }
    lines.push("- Keep `DX_VERTICAL_BUDGET_GATE=1` failing until the launch threshold or implementation change is reviewed.");
  }

  return `${lines.join("\n")}\n`;
}

function snapshotPaths(report) {
  const stem = `${snapshotStamp(report.generated_at)}-${report.fixture_mode}`;
  return {
    jsonPath: path.join(historyDir, `${stem}.json`),
    mdPath: path.join(historyDir, `${stem}.md`),
    indexPath: path.join(historyDir, "index.json"),
    indexMdPath: path.join(historyDir, "index.md"),
  };
}

function snapshotStamp(iso) {
  return iso.replace(/\.\d{3}Z$/, "Z").replace(/[^0-9TZ]/g, "");
}

function relativeReportPath(filePath) {
  return path.relative(reportDir, filePath).replaceAll(path.sep, "/");
}

function updateHistoryIndex(report, snapshot) {
  const current = readHistoryIndex(snapshot.indexPath);
  const entry = historyIndexEntry(report, snapshot);
  const snapshots = [
    entry,
    ...current.snapshots.filter(
      (item) => !(item.generated_at === entry.generated_at && item.fixture_mode === entry.fixture_mode)
    ),
  ].sort((left, right) => right.generated_at.localeCompare(left.generated_at));

  fs.writeFileSync(
    snapshot.indexPath,
    `${JSON.stringify(
      {
        updated_at: report.generated_at,
        snapshots,
      },
      null,
      2
    )}\n`
  );
  fs.writeFileSync(snapshot.indexMdPath, renderHistoryIndexMarkdown(report.generated_at, snapshots));
}

function readHistoryIndex(indexPath) {
  if (!fs.existsSync(indexPath)) {
    return { snapshots: [] };
  }
  try {
    const parsed = JSON.parse(fs.readFileSync(indexPath, "utf8"));
    return { snapshots: Array.isArray(parsed.snapshots) ? parsed.snapshots : [] };
  } catch {
    return { snapshots: [] };
  }
}

function historyIndexEntry(report, snapshot) {
  return {
    generated_at: report.generated_at,
    fixture_mode: report.fixture_mode,
    fixture: report.fixture,
    json: relativeReportPath(snapshot.jsonPath),
    markdown: relativeReportPath(snapshot.mdPath),
    route_delivery: report.delivery.route_mode,
    forge_packages: report.proof.forge_packages.length,
    forge_files_tracked: report.proof.forge_packages.reduce(
      (sum, forgePackage) => sum + (forgePackage.files_tracked || 0),
      0
    ),
    decoded_bytes: report.http.total_decoded_bytes,
    brotli_bytes: report.http.compression_estimate.brotli_bytes,
    http_route_median_ms: report.http.route_timing_ms.median,
    chrome_load_event_ms: report.chrome ? report.chrome.load_event_ms : null,
    dx_packet_applied: report.chrome ? report.chrome.dx_packet_applied : null,
    interaction_works: report.chrome ? report.chrome.interaction_works : null,
    budget_passed: report.budget ? report.budget.passed : null,
    budget_profile: report.budget ? report.budget.profile : null,
  };
}

function renderHistoryIndexMarkdown(updatedAt, snapshots) {
  return [
    "# DX-WWW Vertical Proof History",
    "",
    `Updated: ${updatedAt}`,
    "",
    "| Generated | Fixture | Delivery | Packages | Files | Decoded | Brotli | HTTP median | Chrome load | DXPK | Interaction | Budget | Report |",
    "| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- | --- | --- |",
    ...snapshots.map((snapshot) =>
      [
        snapshot.generated_at,
        snapshot.fixture_mode,
        snapshot.route_delivery || "unknown",
        snapshot.forge_packages,
        snapshot.forge_files_tracked,
        `${snapshot.decoded_bytes} B`,
        `${snapshot.brotli_bytes} B`,
        `${snapshot.http_route_median_ms} ms`,
        formatNullable(snapshot.chrome_load_event_ms, " ms"),
        snapshot.dx_packet_applied,
        snapshot.interaction_works,
        snapshot.budget_passed ?? "n/a",
        `[md](${path.basename(snapshot.markdown)})`,
      ].join(" | ")
    ).map((row) => `| ${row} |`),
    "",
  ].join("\n");
}

function formatDelta(value) {
  if (value > 0) {
    return `+${Number(value.toFixed?.(3) ?? value)}`;
  }
  if (value < 0) {
    return `${Number(value.toFixed?.(3) ?? value)}`;
  }
  return "0";
}

function formatNullable(value, suffix = "", signed = false) {
  if (value === null || value === undefined) {
    return "n/a";
  }
  const numeric = typeof value === "number" ? Number(value.toFixed(3)) : value;
  if (signed && typeof numeric === "number" && numeric > 0) {
    return `+${numeric}${suffix}`;
  }
  return `${numeric}${suffix}`;
}

function readPreviousReport() {
  const jsonPath = path.join(reportDir, "vertical-proof-measurement.json");
  if (!fs.existsSync(jsonPath)) {
    return null;
  }
  try {
    return JSON.parse(fs.readFileSync(jsonPath, "utf8"));
  } catch {
    return null;
  }
}

function compareToPrevious(previous, current) {
  if (!previous) {
    return null;
  }
  const previousChromeLoad = previous.chrome?.load_event_ms ?? null;
  const currentChromeLoad = current.chrome?.load_event_ms ?? null;
  return {
    previous_generated_at: previous.generated_at,
    previous_fixture_mode: previous.fixture_mode,
    current_fixture_mode: current.fixture_mode,
    previous_forge_packages: previous.proof?.forge_packages?.length ?? 0,
    current_forge_packages: current.proof.forge_packages.length,
    delta_forge_packages:
      current.proof.forge_packages.length - (previous.proof?.forge_packages?.length ?? 0),
    previous_decoded_bytes: previous.http?.total_decoded_bytes ?? 0,
    current_decoded_bytes: current.http.total_decoded_bytes,
    delta_decoded_bytes:
      current.http.total_decoded_bytes - (previous.http?.total_decoded_bytes ?? 0),
    previous_brotli_bytes: previous.http?.compression_estimate?.brotli_bytes ?? 0,
    current_brotli_bytes: current.http.compression_estimate.brotli_bytes,
    delta_brotli_bytes:
      current.http.compression_estimate.brotli_bytes -
      (previous.http?.compression_estimate?.brotli_bytes ?? 0),
    previous_http_route_median_ms: previous.http?.route_timing_ms?.median ?? 0,
    current_http_route_median_ms: current.http.route_timing_ms.median,
    delta_http_route_median_ms:
      current.http.route_timing_ms.median - (previous.http?.route_timing_ms?.median ?? 0),
    previous_chrome_load_event_ms: previousChromeLoad,
    current_chrome_load_event_ms: currentChromeLoad,
    delta_chrome_load_event_ms:
      previousChromeLoad === null || currentChromeLoad === null
        ? null
        : currentChromeLoad - previousChromeLoad,
  };
}

function summarizeForge(forge) {
  if (!forge) {
    return null;
  }
  return {
    package_id: forge.package_id,
    action: forge.action,
    risk_score: forge.risk_score,
    traffic: forge.traffic,
    files_tracked: forge.files_tracked,
  };
}

function summarizeDelivery(proof) {
  const written = proof.written || {};
  return {
    route_mode: written.runtime_path ? "dxpk-runtime" : "static",
    packet_artifact_written: Boolean(written.packet_path),
    runtime_asset_written: Boolean(written.runtime_path),
  };
}

function budgetNumber(name, fallback, env = process.env) {
  const value = env[name];
  if (value === undefined || value === "") {
    return fallback;
  }
  const parsed = Number(value);
  if (!Number.isFinite(parsed) || parsed <= 0) {
    throw new Error(`${name} must be a positive number when set`);
  }
  return parsed;
}

function stripTomlComment(line) {
  let inString = false;
  for (let index = 0; index < line.length; index += 1) {
    const char = line[index];
    if (char === '"' && line[index - 1] !== "\\") {
      inString = !inString;
    }
    if (char === "#" && !inString) {
      return line.slice(0, index);
    }
  }
  return line;
}

function parseTomlScalar(value, configPath, key) {
  const trimmed = stripTomlComment(value).trim();
  if (trimmed.startsWith('"') && trimmed.endsWith('"')) {
    return trimmed.slice(1, -1);
  }
  const number = Number(trimmed);
  if (Number.isFinite(number)) {
    return number;
  }
  throw new Error(`Unsupported dx.config.toml value for ${key} in ${configPath}`);
}

function normalizeLaunchBudgetKey(key) {
  return (
    {
      decoded_bytes: "decoded_bytes",
      max_decoded_bytes: "decoded_bytes",
      route_bytes: "decoded_bytes",
      max_route_bytes: "decoded_bytes",
      brotli_bytes: "brotli_bytes",
      max_brotli_bytes: "brotli_bytes",
      http_route_median_ms: "http_route_median_ms",
      max_http_route_median_ms: "http_route_median_ms",
      chrome_load_event_ms: "chrome_load_event_ms",
      max_chrome_load_event_ms: "chrome_load_event_ms",
      chrome_load_ms: "chrome_load_event_ms",
      max_chrome_load_ms: "chrome_load_event_ms",
      profile: "profile",
    }[key] || null
  );
}

function readLaunchBudgetConfigs(configPath = defaultBudgetConfigPath) {
  if (!configPath || !fs.existsSync(configPath)) {
    return {};
  }

  const text = fs.readFileSync(configPath, "utf8");
  let activeBudgetKey = null;
  const configs = {};

  for (const rawLine of text.split(/\r?\n/)) {
    const line = stripTomlComment(rawLine).trim();
    if (!line) {
      continue;
    }
    const section = line.match(/^\[([^\]]+)\]$/);
    if (section) {
      activeBudgetKey = launchBudgetSectionKey(section[1]);
      if (activeBudgetKey && !configs[activeBudgetKey]) {
        configs[activeBudgetKey] = {};
      }
      continue;
    }
    if (!activeBudgetKey) {
      continue;
    }

    const equalsIndex = line.indexOf("=");
    if (equalsIndex === -1) {
      continue;
    }
    const key = line.slice(0, equalsIndex).trim();
    const normalized = normalizeLaunchBudgetKey(key);
    if (!normalized) {
      continue;
    }
    const value = parseTomlScalar(line.slice(equalsIndex + 1), configPath, key);
    if (normalized === "profile") {
      if (typeof value !== "string" || value.trim() === "") {
        throw new Error(
          `forge.launch_budget.${activeBudgetKey}.profile must be a non-empty string in ${configPath}`
        );
      }
      configs[activeBudgetKey].profile = value;
      continue;
    }
    if (typeof value !== "number" || !Number.isFinite(value) || value <= 0) {
      throw new Error(
        `forge.launch_budget.${activeBudgetKey}.${key} must be a positive number in ${configPath}`
      );
    }
    configs[activeBudgetKey][normalized] = value;
  }

  return configs;
}

function launchBudgetSectionKey(section) {
  if (section === "forge.launch_budget" || section === "forge.launch_budget.site") {
    return "site";
  }
  if (section === "forge.launch_budget.scorecard") {
    return "scorecard";
  }
  if (section === "forge.launch_budget.ci") {
    return "ci";
  }
  if (section === "forge.launch_budget.evidence") {
    return "evidence";
  }
  if (section === "forge.launch_budget.releases") {
    return "releases";
  }
  if (section === "forge.launch_budget.changelog") {
    return "changelog";
  }
  return null;
}

function readLaunchBudgetConfig(configPath = defaultBudgetConfigPath, budgetKey = "site") {
  return readLaunchBudgetConfigs(configPath)[budgetKey] || {};
}

function budgetForFixture(mode, options = {}) {
  const profile = forgeRouteBudgetProfiles[mode];
  if (!profile) {
    return null;
  }
  const env = options.env || process.env;
  const configPath =
    options.configPath ||
    env.DX_VERTICAL_BUDGET_CONFIG ||
    defaultBudgetConfigPath;
  const config = readLaunchBudgetConfig(configPath, profile.configKey);
  return {
    profile: config.profile || profile.profile,
    thresholds: {
      decoded_bytes: budgetNumber(
        `${profile.envPrefix}_DECODED_BYTES`,
        config.decoded_bytes ?? profile.defaults.decoded_bytes,
        env
      ),
      brotli_bytes: budgetNumber(
        `${profile.envPrefix}_BROTLI_BYTES`,
        config.brotli_bytes ?? profile.defaults.brotli_bytes,
        env
      ),
      http_route_median_ms: budgetNumber(
        `${profile.envPrefix}_HTTP_MEDIAN_MS`,
        config.http_route_median_ms ?? profile.defaults.http_route_median_ms,
        env
      ),
      chrome_load_event_ms: budgetNumber(
        `${profile.envPrefix}_CHROME_LOAD_MS`,
        config.chrome_load_event_ms ?? profile.defaults.chrome_load_event_ms,
        env
      ),
    },
    config_path: fs.existsSync(configPath) ? configPath : null,
  };
}

function budgetCheck(metric, value, max, unit = "") {
  const numeric = typeof value === "number" && Number.isFinite(value) ? value : null;
  return {
    metric,
    value: numeric,
    max,
    unit,
    passed: numeric !== null && numeric <= max,
  };
}

function evaluateDeliveryBudget(report) {
  const budget = budgetForFixture(report.fixture_mode);
  if (!budget) {
    return null;
  }
  const checks = [
    budgetCheck(
      "decoded bytes",
      report.http.total_decoded_bytes,
      budget.thresholds.decoded_bytes,
      " B"
    ),
    budgetCheck(
      "Brotli estimate",
      report.http.compression_estimate.brotli_bytes,
      budget.thresholds.brotli_bytes,
      " B"
    ),
    budgetCheck(
      "HTTP route median",
      report.http.route_timing_ms.median,
      budget.thresholds.http_route_median_ms,
      " ms"
    ),
    budgetCheck(
      "Chrome load event median",
      report.chrome ? report.chrome.load_event_ms : null,
      budget.thresholds.chrome_load_event_ms,
      " ms"
    ),
  ];
  return {
    profile: budget.profile,
    config_path: budget.config_path,
    enforced: enforceBudgetGate,
    passed: checks.every((check) => check.passed),
    checks,
  };
}

async function main() {
  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-www-vertical-proof-"));
  const previousReport = readPreviousReport();
  let server;
  let chrome;
  try {
    copyFixture(tempRoot);
    runDxProve(tempRoot);
    server = await listen(createStaticServer(path.join(tempRoot, "proof")));
    const routePath = fixtureModes[fixtureMode].routePath || "/";
    const routeUrl = new URL(routePath, `${server.baseUrl}/`).toString();
    const httpResult = await measureHttp(server.baseUrl, routePath);
    chrome = await startChrome();
    const chromeResult = await measureChrome(chrome, routeUrl);
    const proof = JSON.parse(fs.readFileSync(path.join(tempRoot, "proof", "proof.json"), "utf8"));
    const delivery = summarizeDelivery(proof);
    const report = {
      generated_at: new Date().toISOString(),
      fixture: fixtureModes[fixtureMode].fixture,
      fixture_mode: fixtureMode,
      delivery,
      proof: {
        route: proof.route,
        fallback: proof.fallback,
        packet: proof.packet,
        browser_packet: proof.browser_packet,
        interaction: proof.interaction,
        forge: summarizeForge(proof.forge),
        forge_packages: proof.forge_packages || [],
      },
      http: httpResult,
      chrome: chromeResult,
    };
    report.budget = evaluateDeliveryBudget(report);
    report.comparison = compareToPrevious(previousReport, report);
    const paths = writeReports(report);
    console.log(
      JSON.stringify(
        {
          report: paths,
          decoded_bytes: report.http.total_decoded_bytes,
          brotli_bytes: report.http.compression_estimate.brotli_bytes,
          http_median_ms: report.http.route_timing_ms.median,
          chrome_load_event_ms: report.chrome ? report.chrome.load_event_ms : null,
          chrome_transfer_size: report.chrome ? report.chrome.transfer_size : null,
          route_delivery: report.delivery.route_mode,
          dx_packet_applied: report.chrome ? report.chrome.dx_packet_applied : null,
          interaction_works: report.chrome ? report.chrome.interaction_works : null,
          budget_passed: report.budget ? report.budget.passed : null,
          budget_enforced: report.budget ? report.budget.enforced : false,
          budget_triage: paths.triagePath || null,
        },
        null,
        2
      )
    );
    if (report.budget?.enforced && !report.budget.passed) {
      const failed = report.budget.checks
        .filter((check) => !check.passed)
        .map((check) => check.metric)
        .join(", ");
      throw new Error(`DX vertical proof budget failed: ${failed}`);
    }
  } finally {
    if (chrome) {
      chrome.close();
    }
    if (server) {
      await server.close();
    }
    fs.rmSync(tempRoot, { recursive: true, force: true });
  }
}

if (require.main === module) {
  main()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error(error);
      process.exit(1);
    });
}

module.exports = {
  budgetForFixture,
  readLaunchBudgetConfig,
  renderBudgetTriageMarkdown,
};
