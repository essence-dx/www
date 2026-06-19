const fs = require("fs");
const http = require("http");
const os = require("os");
const path = require("path");
const { spawn, spawnSync } = require("child_process");
const { performance } = require("perf_hooks");
const zlib = require("zlib");

const root = path.resolve(__dirname, "..");
const suiteRoot = path.join(__dirname, "fair-counter");
const reportDir = path.join(__dirname, "reports");
fs.mkdirSync(reportDir, { recursive: true });

const frameworks = ["DX-WWW", "Astro", "Svelte", "HTMX", "Next.js"];
const HTTP_ROUTE_SAMPLES = 12;
const CHROME_SAMPLES = Number(process.env.DX_BENCH_CHROME_SAMPLES || 7);
const routes = [
  { key: "small-counter", label: "Small counter", path: "/" },
  { key: "medium-docs", label: "Medium docs, 160 sections", path: "/medium-docs" },
  { key: "medium-cards", label: "Medium cards, 180 cards + filter", path: "/medium-cards" },
  { key: "big-dashboard", label: "Big dashboard, 1,200 rows + filter", path: "/big-dashboard" },
];

const MIME = {
  ".css": "text/css; charset=utf-8",
  ".html": "text/html; charset=utf-8",
  ".js": "application/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".wasm": "application/wasm",
  ".woff2": "font/woff2",
};

function fileType(filePath) {
  return MIME[path.extname(filePath).toLowerCase()] || "application/octet-stream";
}

function createStaticServer(baseDir, options = {}) {
  return http.createServer((req, res) => {
    const url = new URL(req.url, "http://127.0.0.1");
    const pathname = decodeURIComponent(url.pathname);
    const cleanPath = pathname === "/" ? "/index.html" : pathname;
    const resolvedBase = path.resolve(baseDir);
    let resolvedFile = path.resolve(path.normalize(path.join(baseDir, cleanPath)));

    if (!resolvedFile.startsWith(resolvedBase) || !fs.existsSync(resolvedFile) || !fs.statSync(resolvedFile).isFile()) {
      const indexPath = path.resolve(path.join(baseDir, pathname, "index.html"));
      if (indexPath.startsWith(resolvedBase) && fs.existsSync(indexPath) && fs.statSync(indexPath).isFile()) {
        resolvedFile = indexPath;
      } else if (options.fallbackIndex) {
        resolvedFile = path.resolve(path.join(baseDir, "index.html"));
      } else {
        res.writeHead(404, { "content-type": "text/plain; charset=utf-8" });
        res.end("Not found");
        return;
      }
    }

    const body = fs.readFileSync(resolvedFile);
    res.writeHead(200, {
      "content-type": fileType(resolvedFile),
      "content-length": body.length,
      "cache-control": "no-cache",
    });
    res.end(body);
  });
}

function createHtmxServer() {
  const htmxPath = path.join(suiteRoot, "htmx", "node_modules", "htmx.org", "dist", "htmx.min.js");
  const publicDir = path.join(suiteRoot, "htmx", "public");
  let count = 0;

  return http.createServer((req, res) => {
    if (req.url === "/" || req.url === "/index.html") {
      sendBuffer(res, fs.readFileSync(path.join(publicDir, "index.html")), "text/html; charset=utf-8");
      return;
    }
    if (req.url === "/medium-docs") {
      sendText(res, renderDocsPage("HTMX", "HTMX + server HTML"), "text/html; charset=utf-8");
      return;
    }
    if (req.url === "/medium-cards") {
      sendText(res, renderCardsPage("HTMX", "HTMX + inline filter"), "text/html; charset=utf-8");
      return;
    }
    if (req.url === "/big-dashboard") {
      sendText(res, renderDashboardPage("HTMX", "HTMX + inline filter", 1200), "text/html; charset=utf-8");
      return;
    }
    if (req.url === "/htmx.min.js") {
      sendBuffer(res, fs.readFileSync(htmxPath), "application/javascript; charset=utf-8");
      return;
    }
    if (req.method === "POST" && req.url === "/counter/increment") {
      count += 1;
      sendText(res, String(count), "text/plain; charset=utf-8");
      return;
    }
    if (req.method === "POST" && req.url === "/counter/decrement") {
      count -= 1;
      sendText(res, String(count), "text/plain; charset=utf-8");
      return;
    }
    if (req.method === "POST" && req.url === "/counter/reset") {
      count = 0;
      sendText(res, "0", "text/plain; charset=utf-8");
      return;
    }
    res.writeHead(404, { "content-type": "text/plain; charset=utf-8" });
    res.end("Not found");
  });
}

function sendText(res, text, contentType) {
  sendBuffer(res, Buffer.from(text), contentType);
}

function sendBuffer(res, body, contentType) {
  res.writeHead(200, {
    "content-type": contentType,
    "content-length": body.length,
    "cache-control": "no-cache",
  });
  res.end(body);
}

function benchmarkHead(title) {
  return `<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>${title}</title><script src="/htmx.min.js"></script><style>${benchmarkCss()}</style></head><body>`;
}

function benchmarkCss() {
  return "*{box-sizing:border-box}body{margin:0;background:#f7f8fb;color:#14171f;font-family:system-ui,-apple-system,BlinkMacSystemFont,\"Segoe UI\",sans-serif;line-height:1.5}.bench-shell{max-width:1180px;margin:0 auto;padding:24px}.bench-hero{align-items:flex-end;border-bottom:1px solid #dfe3ea;display:flex;gap:16px;justify-content:space-between;margin-bottom:20px;padding-bottom:16px}.eyebrow{color:#5b6472;font-size:13px;font-weight:700;letter-spacing:.08em;text-transform:uppercase}h1{font-size:32px;line-height:1.1;margin:6px 0 0}.metric-row,.toolbar{display:flex;flex-wrap:wrap;gap:8px}.metric{background:#fff;border:1px solid #dfe3ea;border-radius:8px;min-width:110px;padding:8px 10px}.metric b{display:block;font-size:20px}.doc-grid,.card-grid{display:grid;gap:10px;grid-template-columns:repeat(auto-fit,minmax(240px,1fr))}.doc-section,.card,.panel{background:#fff;border:1px solid #dfe3ea;border-radius:8px;margin-bottom:10px;padding:14px}.toolbar{margin-bottom:14px}.toolbar input{border:1px solid #cad0da;border-radius:6px;flex:1;font:inherit;padding:10px 12px}.tag{background:#edf2ff;border-radius:999px;color:#2c55c7;display:inline-block;font-size:12px;font-weight:700;margin-top:10px;padding:3px 8px}table{border-collapse:collapse;font-size:13px;width:100%}th,td{border-bottom:1px solid #e6e9ef;padding:8px;text-align:left}th{background:#f0f3f8;color:#4b5565;font-size:12px;text-transform:uppercase}.status-ok{color:#087443;font-weight:700}.status-risk{color:#a15c00;font-weight:700}@media(max-width:700px){.bench-shell{padding:16px}.bench-hero{display:block}h1{font-size:26px}}";
}

function renderDocsPage(framework, _runtime) {
  let html = benchmarkHead(`${framework} Medium Docs Benchmark`);
  html += "<main class=\"bench-shell\"><header class=\"bench-hero\"><div><div class=\"eyebrow\">Medium route</div><h1>Framework documentation system</h1></div><div class=\"metric-row\">";
  html += metric("Sections", "160") + metric("Runtime", framework);
  html += "</div></header><section class=\"doc-grid\">";
  for (let index = 1; index <= 160; index += 1) {
    const principle = ["routing", "serialization", "styling", "deployment"][index % 4];
    html += `<article class="doc-section"><h2>Binary web principle ${index}</h2><p>This block covers cache behavior, payload shape, routing boundaries, and production maintenance for the same generated content.</p><span class="tag">${principle}</span></article>`;
  }
  return `${html}</section></main></body></html>`;
}

function renderCardsPage(framework, _runtime) {
  let html = benchmarkHead(`${framework} Medium Cards Benchmark`);
  html += "<main class=\"bench-shell\"><header class=\"bench-hero\"><div><div class=\"eyebrow\">Medium interactive route</div><h1>Component registry catalog</h1></div><div class=\"metric-row\">";
  html += metric("Cards", "180") + metric("Runtime", framework);
  html += "</div></header><div class=\"toolbar\"><input id=\"card-filter\" type=\"search\" placeholder=\"Filter registry cards\" aria-label=\"Filter registry cards\"></div><section id=\"card-grid\" class=\"card-grid\">";
  for (let index = 1; index <= 180; index += 1) {
    const category = ["auth", "dashboard", "commerce", "editor", "analytics"][index % 5];
    html += `<article class="card" data-search="${category} component ${index}"><h2>${category} component ${index}</h2><p>Editable source-owned component packaging with predictable upgrade metadata.</p><span class="tag">${category}</span></article>`;
  }
  html += "</section></main><script>const cardInput=document.getElementById('card-filter');const cards=[...document.querySelectorAll('[data-search]')];cardInput.addEventListener('input',()=>{const q=cardInput.value.toLowerCase();for(const card of cards){card.hidden=!card.dataset.search.includes(q)}});</script></body></html>";
  return html;
}

function renderDashboardPage(framework, runtime, rows) {
  let html = benchmarkHead(`${framework} Big Dashboard Benchmark`);
  html += "<main class=\"bench-shell\"><header class=\"bench-hero\"><div><div class=\"eyebrow\">Big interactive route</div><h1>Revenue operations dashboard</h1></div><div class=\"metric-row\">";
  html += metric("Rows", String(rows)) + metric("Runtime", framework);
  html += "</div></header><div class=\"toolbar\"><input id=\"row-filter\" type=\"search\" placeholder=\"Filter customers or status\" aria-label=\"Filter dashboard rows\"></div><section class=\"panel\"><table><thead><tr><th>Account</th><th>Plan</th><th>Region</th><th>Status</th><th>MRR</th><th>Risk</th></tr></thead><tbody>";
  for (let index = 1; index <= rows; index += 1) {
    const plan = ["Enterprise", "Pro", "Team", "Starter"][index % 4];
    const region = ["APAC", "EU", "NA", "LATAM", "MEA"][index % 5];
    const status = index % 9 === 0 ? "Review" : "Healthy";
    const statusClass = status === "Review" ? "status-risk" : "status-ok";
    const mrr = 400 + (index % 37) * 91;
    html += `<tr data-search="account ${index} ${plan} ${region} ${status}"><td>Account ${index}</td><td>${plan}</td><td>${region}</td><td class="${statusClass}">${status}</td><td>$${mrr}</td><td>${(index * 7) % 100}</td></tr>`;
  }
  html += "</tbody></table></section></main><script>const rowInput=document.getElementById('row-filter');const rows=[...document.querySelectorAll('tbody tr')];rowInput.addEventListener('input',()=>{const q=rowInput.value.toLowerCase();for(const row of rows){row.hidden=!row.dataset.search.includes(q)}});</script></body></html>";
  return html;
}

function metric(label, value) {
  return `<div class="metric"><span>${label}</span><b>${value}</b></div>`;
}

function listen(server) {
  return new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const { port } = server.address();
      resolve({
        baseUrl: `http://127.0.0.1:${port}`,
        close: () => new Promise((done) => server.close(done)),
      });
    });
  });
}

function freePort() {
  const probe = http.createServer();
  return new Promise((resolve, reject) => {
    probe.once("error", reject);
    probe.listen(0, "127.0.0.1", () => {
      const selected = probe.address().port;
      probe.close(() => resolve(selected));
    });
  });
}

async function waitFor(url, timeoutMs = 30000) {
  const started = Date.now();
  let lastError;
  while (Date.now() - started < timeoutMs) {
    try {
      const response = await fetch(url, { cache: "no-store" });
      if (response.ok) {
        await response.arrayBuffer();
        return;
      }
    } catch (error) {
      lastError = error;
    }
    await new Promise((resolve) => setTimeout(resolve, 300));
  }
  throw lastError || new Error(`Timed out waiting for ${url}`);
}

async function startNext() {
  const port = await freePort();
  const bin = path.join(suiteRoot, "next", "node_modules", ".bin", process.platform === "win32" ? "next.cmd" : "next");
  const command = process.platform === "win32" ? "cmd.exe" : bin;
  const args = process.platform === "win32" ? ["/c", bin, "start", "-p", String(port)] : ["start", "-p", String(port)];
  const child = spawn(command, args, {
    cwd: path.join(suiteRoot, "next"),
    stdio: "ignore",
    windowsHide: true,
  });
  const baseUrl = `http://127.0.0.1:${port}`;
  await waitFor(`${baseUrl}/`);
  return {
    baseUrl,
    close: () => killProcessTree(child),
  };
}

async function startDxWww() {
  const port = await freePort();
  const args = [
    "run",
    "--release",
    "--target-dir",
    path.join(root, "www", "target-bench-dx-www-release"),
    "-p",
    "dx-www-demo",
    "--bin",
    "demo-server",
  ];
  const child = spawn("cargo", args, {
    cwd: path.join(root, "www"),
    env: { ...process.env, PORT: String(port) },
    stdio: "ignore",
    windowsHide: true,
  });
  const baseUrl = `http://127.0.0.1:${port}`;
  await waitFor(`${baseUrl}/fair-counter`, 300000);
  return {
    baseUrl,
    close: () => killProcessTree(child),
  };
}

function killProcessTree(child) {
  if (!child || child.killed) {
    return;
  }
  if (process.platform === "win32") {
    spawnSync("taskkill", ["/PID", String(child.pid), "/T", "/F"], { stdio: "ignore" });
  } else {
    child.kill("SIGTERM");
  }
}

async function fetchResource(url, options = {}) {
  const started = performance.now();
  const response = await fetch(url, {
    headers: { "cache-control": "no-cache", ...(options.headers || {}) },
    method: options.method || "GET",
  });
  const buffer = Buffer.from(await response.arrayBuffer());
  return {
    url,
    status: response.status,
    ok: response.ok,
    content_type: response.headers.get("content-type"),
    content_length_header: response.headers.get("content-length"),
    decoded_bytes: buffer.length,
    elapsed_ms: Number((performance.now() - started).toFixed(3)),
    buffer,
  };
}

function redactExternalNextBundlerResource(resource) {
  if (typeof resource.url !== "string" || !/\/_next\/static\/chunks\/[^/]+\.js$/.test(resource.url)) {
    return resource;
  }
  return {
    ...resource,
    url: "external-next-bundler-runtime-chunk",
    url_redacted: true,
    url_redaction_reason: "external-framework-baseline-runtime-asset",
  };
}

function extractAssets(html, baseUrl) {
  const urls = new Set();
  const attrPattern = /(?:src|href)="([^"]+)"/g;
  let match;
  while ((match = attrPattern.exec(html))) {
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

function sumCompression(resources) {
  return resources.reduce(
    (acc, item) => {
      const compressed = compress(item.buffer);
      acc.raw_bytes += compressed.raw_bytes;
      acc.gzip_bytes += compressed.gzip_bytes;
      acc.brotli_bytes += compressed.brotli_bytes;
      return acc;
    },
    { raw_bytes: 0, gzip_bytes: 0, brotli_bytes: 0 }
  );
}

function summarize(values) {
  const sorted = [...values].sort((a, b) => a - b);
  const sum = values.reduce((a, b) => a + b, 0);
  const pick = (p) => sorted[Math.min(sorted.length - 1, Math.max(0, Math.ceil(sorted.length * p) - 1))];
  return {
    samples: values.length,
    min: Number(sorted[0].toFixed(3)),
    median: Number(pick(0.5).toFixed(3)),
    mean: Number((sum / values.length).toFixed(3)),
    p95: Number(pick(0.95).toFixed(3)),
    max: Number(sorted[sorted.length - 1].toFixed(3)),
  };
}

async function measureHttp(target, route, sampleCount = HTTP_ROUTE_SAMPLES) {
  const pathForTarget = target.name === "DX-WWW" && route.key === "small-counter" ? "/fair-counter" : route.path;
  const url = new URL(pathForTarget, target.baseUrl).toString();
  const index = await fetchResource(url);
  if (!index.ok) {
    throw new Error(`${target.name} ${route.key} returned ${index.status}`);
  }
  const html = index.buffer.toString("utf8");
  const assetUrls = extractAssets(html, url);
  const assets = await Promise.all(assetUrls.map((assetUrl) => fetchResource(assetUrl)));
  const resources = [index, ...assets];
  const routeSamples = [];
  const fullSamples = [];

  for (let i = 0; i < sampleCount; i += 1) {
    routeSamples.push((await fetchResource(url)).elapsed_ms);
  }
  for (let i = 0; i < Math.max(5, Math.floor(sampleCount / 2)); i += 1) {
    const started = performance.now();
    const first = await fetchResource(url);
    const firstAssets = extractAssets(first.buffer.toString("utf8"), url);
    await Promise.all(firstAssets.map((assetUrl) => fetchResource(assetUrl)));
    fullSamples.push(performance.now() - started);
  }

  return {
    url,
    resource_count: resources.length,
    resources: resources.map((resource) => {
      const { buffer, ...item } = redactExternalNextBundlerResource(resource);
      return item;
    }),
    total_decoded_bytes: resources.reduce((sum, item) => sum + item.decoded_bytes, 0),
    compression_estimate: sumCompression(resources),
    route_timing_ms: summarize(routeSamples),
    full_route_timing_ms: summarize(fullSamples),
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

async function startChrome() {
  const chromePath = findChrome();
  if (!chromePath) {
    return null;
  }
  const port = await freePort();
  const userDataDir = path.join(os.tmpdir(), `dx-www-chrome-${Date.now()}-${Math.random().toString(16).slice(2)}`);
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
  await waitFor(`${baseUrl}/json/version`, 20000);
  return {
    baseUrl,
    close: () => {
      killProcessTree(child);
      try {
        fs.rmSync(userDataDir, { recursive: true, force: true });
      } catch {
        // Chrome can keep Crashpad metrics locked for a moment on Windows.
      }
    },
  };
}

async function chromeNewPage(chrome) {
  let response = await fetch(`${chrome.baseUrl}/json/new`, { method: "PUT" });
  if (!response.ok) {
    response = await fetch(`${chrome.baseUrl}/json/new`);
  }
  if (!response.ok) {
    throw new Error(`Chrome refused new page: ${response.status}`);
  }
  return response.json();
}

async function measureChrome(chrome, url, sampleCount = CHROME_SAMPLES) {
  if (!chrome) {
    return null;
  }
  const samples = [];
  for (let index = 0; index < sampleCount; index += 1) {
    samples.push(await measureChromeOnce(chrome, url));
  }
  const numericMedian = (key) => summarize(samples.map((item) => item[key])).median;
  const last = samples[samples.length - 1];
  return {
    title: last.title,
    samples: samples.length,
    dom_nodes: last.dom_nodes,
    rows: last.rows,
    cards: last.cards,
    sections: last.sections,
    scripts: last.scripts,
    stylesheets: last.stylesheets,
    nav_duration_ms: numericMedian("nav_duration_ms"),
    dom_content_loaded_ms: numericMedian("dom_content_loaded_ms"),
    load_event_ms: numericMedian("load_event_ms"),
    transfer_size: numericMedian("transfer_size"),
    encoded_body_size: numericMedian("encoded_body_size"),
    decoded_body_size: numericMedian("decoded_body_size"),
    resource_count: last.resource_count,
    raw_samples: samples,
  };
}

async function measureChromeOnce(chrome, url) {
  if (!chrome) {
    return null;
  }
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
    await new Promise((resolve) => setTimeout(resolve, 75));
    const expression = `(() => {
      const nav = performance.getEntriesByType("navigation")[0];
      const resources = performance.getEntriesByType("resource");
      return {
        title: document.title,
        dom_nodes: document.getElementsByTagName("*").length,
        rows: document.querySelectorAll("tbody tr").length,
        cards: document.querySelectorAll(".card").length,
        sections: document.querySelectorAll(".doc-section").length,
        scripts: document.scripts.length,
        stylesheets: document.styleSheets.length,
        nav_duration_ms: nav ? nav.duration : 0,
        dom_content_loaded_ms: nav ? nav.domContentLoadedEventEnd - nav.startTime : 0,
        load_event_ms: nav ? nav.loadEventEnd - nav.startTime : 0,
        transfer_size: nav ? nav.transferSize + Array.from(resources).reduce((sum, item) => sum + item.transferSize, 0) : 0,
        encoded_body_size: nav ? nav.encodedBodySize + Array.from(resources).reduce((sum, item) => sum + item.encodedBodySize, 0) : 0,
        decoded_body_size: nav ? nav.decodedBodySize + Array.from(resources).reduce((sum, item) => sum + item.decodedBodySize, 0) : 0,
        resource_count: resources.length + 1
      };
    })()`;
    const result = await client.send("Runtime.evaluate", {
      expression,
      returnByValue: true,
      awaitPromise: true,
    });
    return normalizeChromeMetrics(result.result.value);
  } finally {
    client.close();
    await fetch(`${chrome.baseUrl}/json/close/${tab.id}`).catch(() => {});
  }
}

function normalizeChromeMetrics(metrics) {
  return Object.fromEntries(
    Object.entries(metrics).map(([key, value]) => [key, typeof value === "number" ? Number(value.toFixed(3)) : value])
  );
}

function connectCdp(wsUrl) {
  return new Promise((resolve, reject) => {
    const ws = new WebSocket(wsUrl);
    let id = 0;
    const pending = new Map();
    const eventWaiters = new Map();
    const timeout = setTimeout(() => reject(new Error("Timed out connecting to Chrome CDP")), 10000);

    ws.addEventListener("open", () => {
      clearTimeout(timeout);
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
            const timer = setTimeout(() => {
              rejectEvent(new Error(`Timed out waiting for ${method}`));
            }, timeoutMs);
            const waiters = eventWaiters.get(method) || [];
            waiters.push((params) => {
              clearTimeout(timer);
              resolveEvent(params);
            });
            eventWaiters.set(method, waiters);
          });
        },
        close() {
          ws.close();
        },
      });
    });

    ws.addEventListener("message", (event) => {
      const message = JSON.parse(event.data);
      if (message.id && pending.has(message.id)) {
        const item = pending.get(message.id);
        pending.delete(message.id);
        if (message.error) {
          item.reject(new Error(message.error.message));
        } else {
          item.resolve(message.result);
        }
        return;
      }
      if (message.method && eventWaiters.has(message.method)) {
        const waiters = eventWaiters.get(message.method);
        eventWaiters.delete(message.method);
        for (const waiter of waiters) {
          waiter(message.params);
        }
      }
    });

    ws.addEventListener("error", reject);
  });
}

function lowerIsBetterScore(value, best) {
  if (!Number.isFinite(value) || value <= 0) {
    return 0;
  }
  return Math.max(1, Math.min(100, Number(((best / value) * 100).toFixed(1))));
}

function average(values) {
  return Number((values.reduce((sum, value) => sum + value, 0) / values.length).toFixed(1));
}

function bytes(value) {
  if (value < 1024) {
    return `${value} B`;
  }
  return `${(value / 1024).toFixed(2)} KB`;
}

function stars(score) {
  const full = Math.max(0, Math.min(5, Math.round(score / 20)));
  return `${"\u2605".repeat(full)}${"\u2606".repeat(5 - full)}`;
}

async function main() {
  const dxServer = await startDxWww();
  const svelteServer = await listen(createStaticServer(path.join(suiteRoot, "svelte", "dist"), { fallbackIndex: true }));
  const astroServer = await listen(createStaticServer(path.join(suiteRoot, "astro", "dist")));
  const htmxServer = await listen(createHtmxServer());
  const nextServer = await startNext();
  const chrome = await startChrome();
  const servers = [
    { name: "DX-WWW", baseUrl: dxServer.baseUrl, close: dxServer.close },
    { name: "Astro", baseUrl: astroServer.baseUrl, close: astroServer.close },
    { name: "Svelte", baseUrl: svelteServer.baseUrl, close: svelteServer.close },
    { name: "HTMX", baseUrl: htmxServer.baseUrl, close: htmxServer.close },
    { name: "Next.js", baseUrl: nextServer.baseUrl, close: nextServer.close },
  ];

  try {
    const results = [];
    for (const route of routes) {
      for (const target of servers) {
        const httpResult = await measureHttp(target, route);
        const chromeResult = await measureChrome(chrome, httpResult.url);
        results.push({
          framework: target.name,
          route: route.key,
          label: route.label,
          http: httpResult,
          chrome: chromeResult,
        });
      }
    }
    writeReports(results, Boolean(chrome));
  } finally {
    if (chrome) {
      chrome.close();
    }
    for (const server of servers.reverse()) {
      await server.close();
    }
  }
}

function writeReports(results, chrome_enabled) {
  const generatedAt = new Date().toISOString();
  const byRoute = Object.fromEntries(routes.map((route) => [route.key, results.filter((item) => item.route === route.key)]));
  const routeScores = {};

  for (const route of routes) {
    const rows = byRoute[route.key];
    const bestBrotli = Math.min(...rows.map((item) => item.http.compression_estimate.brotli_bytes));
    const bestHttpMedian = Math.min(...rows.map((item) => item.http.full_route_timing_ms.median));
    const bestChromeLoad = chrome_enabled
      ? Math.min(...rows.map((item) => item.chrome.load_event_ms || item.chrome.nav_duration_ms || Infinity))
      : null;
    routeScores[route.key] = Object.fromEntries(
      rows.map((item) => {
        const parts = [
          lowerIsBetterScore(item.http.compression_estimate.brotli_bytes, bestBrotli),
          lowerIsBetterScore(item.http.full_route_timing_ms.median, bestHttpMedian),
        ];
        if (chrome_enabled) {
          parts.push(lowerIsBetterScore(item.chrome.load_event_ms || item.chrome.nav_duration_ms, bestChromeLoad));
        }
        return [item.framework, average(parts)];
      })
    );
  }

  const overall = Object.fromEntries(
    frameworks.map((framework) => {
      const score = average(routes.map((route) => routeScores[route.key][framework]));
      return [framework, { score, stars: stars(score) }];
    })
  );
  const ranked = [...frameworks].sort((a, b) => overall[b].score - overall[a].score);

  const report = {
    generated_at: generatedAt,
    method: {
      description:
        "Real local production route benchmark over small, medium, and big pages. Counts first-route same-origin resources and captures Chrome headless load/DOM metrics when Chrome is available.",
      chrome_enabled,
      chrome_samples: chrome_enabled ? CHROME_SAMPLES : 0,
      http_route_samples: HTTP_ROUTE_SAMPLES,
      community_excluded: true,
      routes,
    },
    results,
    route_scores: routeScores,
    overall,
    ranked,
    honest_gaps: [
      "Static HTML routes leave less room for DX-WWW to beat Astro by huge margins because both ultimately ship HTML.",
      "Svelte here is a Vite CSR app, not SvelteKit SSR; payload may look small on content-heavy routes while browser render work moves client-side.",
      "The big-dashboard full DOM route is intentionally demanding. DX-WWW needs its adaptive/binary route wired into production output to turn the lab win into a real route win.",
    ],
  };

  fs.writeFileSync(path.join(reportDir, "real-route-comparison.json"), `${JSON.stringify(report, null, 2)}\n`);
  fs.writeFileSync(path.join(reportDir, "real-route-comparison.md"), renderMarkdown(report));
  console.log(
    JSON.stringify(
      {
        reports: [
          path.join(reportDir, "real-route-comparison.md"),
          path.join(reportDir, "real-route-comparison.json"),
        ],
        ranked: ranked.map((name) => ({ name, ...overall[name] })),
      },
      null,
      2
    )
  );
}

function renderMarkdown(report) {
  let markdown = "# Real Route Framework Comparison\n\n";
  markdown += `Generated: ${report.generated_at}\n\n`;
  markdown += "Community/adoption is deliberately excluded.\n\n";
  markdown += `Chrome headless metrics: ${report.method.chrome_enabled ? "enabled" : "not available"}.\n\n`;
  markdown += `Samples: ${report.method.http_route_samples} HTTP route timings, ${report.method.chrome_samples} Chrome page loads per framework/route.\n\n`;
  markdown += "## Overall\n\n";
  markdown += "| Framework | Real-route score | Stars |\n";
  markdown += "| --- | ---: | --- |\n";
  for (const name of report.ranked) {
    markdown += `| ${name} | ${report.overall[name].score} | ${report.overall[name].stars} |\n`;
  }
  markdown += "\n## Route Scores\n\n";
  markdown += "| Route | DX-WWW | Astro | Svelte | HTMX | Next.js |\n";
  markdown += "| --- | ---: | ---: | ---: | ---: | ---: |\n";
  for (const route of routes) {
    const scores = report.route_scores[route.key];
    markdown += `| ${route.label} | ${scores["DX-WWW"]} | ${scores.Astro} | ${scores.Svelte} | ${scores.HTMX} | ${scores["Next.js"]} |\n`;
  }
  markdown += "\n## Payload And Timing\n\n";
  for (const route of routes) {
    markdown += `### ${route.label}\n\n`;
    markdown += "| Framework | Brotli | Raw decoded | Resources | HTTP full median | Chrome load | DOM nodes |\n";
    markdown += "| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n";
    for (const item of report.results.filter((entry) => entry.route === route.key)) {
      markdown += `| ${item.framework} | ${bytes(item.http.compression_estimate.brotli_bytes)} | ${bytes(item.http.total_decoded_bytes)} | ${item.http.resource_count} | ${item.http.full_route_timing_ms.median} ms | ${item.chrome ? `${item.chrome.load_event_ms} ms` : "n/a"} | ${item.chrome ? item.chrome.dom_nodes : "n/a"} |\n`;
    }
    markdown += "\n";
  }
  markdown += "## Demanding Gaps\n\n";
  for (const gap of report.honest_gaps) {
    markdown += `- ${gap}\n`;
  }
  return markdown;
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
