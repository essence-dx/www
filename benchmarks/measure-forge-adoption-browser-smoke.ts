const fs = require("fs");
const http = require("http");
const os = require("os");
const path = require("path");
const { spawn, spawnSync } = require("child_process");
const { performance } = require("perf_hooks");
const { dxWwwCargoRunArgs } = require("./dx-www-cli-paths.ts");

const root = path.resolve(__dirname, "..");
const reportDir = path.join(__dirname, "reports");
const outJsonPath = path.join(reportDir, "forge-adoption-browser-smoke.json");
const outMdPath = path.join(reportDir, "forge-adoption-browser-smoke.md");

const routeDefinitions = [
  {
    route: "/forge",
    html: "forge.html",
    clean_html: "forge/index.html",
    artifacts: ["forge.dxp", "forge-proof.json", "forge.claims.json", "forge.evidence.json"],
  },
  {
    route: "/forge/scorecard",
    html: "forge/scorecard.html",
    clean_html: "forge/scorecard/index.html",
    artifacts: ["forge/scorecard.dxp", "forge/scorecard.proof.json"],
  },
  {
    route: "/forge/ci",
    html: "forge/ci.html",
    clean_html: "forge/ci/index.html",
    artifacts: ["forge/ci.dxp", "forge/ci.proof.json", "forge/ci.claims.json"],
  },
  {
    route: "/forge/evidence",
    html: "forge/evidence.html",
    clean_html: "forge/evidence/index.html",
    artifacts: ["forge/evidence.dxp", "forge/evidence.proof.json", "forge/evidence.claims.json"],
  },
  {
    route: "/forge/releases",
    html: "forge/releases.html",
    clean_html: "forge/releases/index.html",
    artifacts: ["forge/releases.dxp", "forge/releases.proof.json", "forge/releases.claims.json"],
  },
  {
    route: "/forge/changelog",
    html: "forge/changelog.html",
    clean_html: "forge/changelog/index.html",
    artifacts: ["forge/changelog.dxp", "forge/changelog.proof.json", "forge/changelog.claims.json"],
  },
];

async function buildReport(options = {}) {
  const generatedAt = options.generatedAt || new Date().toISOString();
  const explicitProjectDir = options.projectDir || process.env.DX_FORGE_ADOPTION_BROWSER_PROJECT || null;
  const projectDir = path.resolve(explicitProjectDir || path.join(root, ".dx", "adoption-browser-smoke"));
  const prepare =
    options.prepare ?? process.env.DX_FORGE_ADOPTION_BROWSER_PREPARE !== "0";
  const measure = options.measure ?? true;
  const chromeEnabled =
    options.chromeEnabled ?? process.env.DX_FORGE_ADOPTION_BROWSER_DISABLE_CHROME !== "1";
  const routeSamples = Number(options.routeSamples || process.env.DX_FORGE_ADOPTION_BROWSER_SAMPLES || 3);

  let prepare_result = {
    enabled: false,
    status: "skipped",
    reason: "prepare disabled",
  };
  if (prepare) {
    if ((options.resetProject ?? !explicitProjectDir) && projectDir.startsWith(path.join(root, ".dx"))) {
      fs.rmSync(projectDir, { recursive: true, force: true });
    }
    prepare_result = prepareAdoptionProject(projectDir, options.runCommand || runCommand);
  }

  const publicDir = path.join(projectDir, "public");
  const routes = inspectPublicRoutes(publicDir);
  const noNodeModules = !fs.existsSync(path.join(projectDir, "node_modules"));
  const findings = collectFindings(routes, noNodeModules, prepare_result);

  let httpEvidence = {
    enabled: false,
    reason: measure ? "public directory missing" : "measurement disabled",
    routes: [],
  };
  let browserEvidence = {
    enabled: false,
    reason: measure ? "not attempted" : "measurement disabled",
    routes: [],
  };

  if (measure && fs.existsSync(publicDir)) {
    const server = await startStaticServer(publicDir);
    try {
      httpEvidence = await measureHttpRoutes(server.baseUrl, routeDefinitions, routeSamples);
      if (chromeEnabled) {
        browserEvidence = await measureBrowserRoutes(server.baseUrl, routeDefinitions);
      } else {
        browserEvidence = {
          enabled: false,
          reason: "disabled by DX_FORGE_ADOPTION_BROWSER_DISABLE_CHROME",
          routes: [],
        };
      }
    } finally {
      await server.close();
    }
  }

  const score = scoreReport(routes, noNodeModules, findings, browserEvidence);
  return {
    generated_at: generatedAt,
    report_id: "forge-adoption-browser-smoke-v1",
    source: "benchmarks/measure-forge-adoption-browser-smoke.ts",
    project_dir: projectDir,
    public_dir: publicDir,
    score,
    passed: score >= 90 && findings.length === 0,
    no_node_modules: noNodeModules,
    prepare_result,
    route_count: routes.length,
    routes,
    http: httpEvidence,
    browser: browserEvidence,
    findings,
    honest_scope: [
      "This is local browser smoke evidence for the generated Forge adoption app routes.",
      "It does not claim live customer traffic, CDN performance, or broad framework replacement.",
      "The harness never runs package installs and treats node_modules as a release-risk finding.",
      "Chrome timing is collected only when a local Chrome or Edge executable is available.",
    ],
  };
}

function prepareAdoptionProject(projectDir, commandRunner) {
  fs.mkdirSync(path.join(projectDir, ".dx", "forge", "adoption-smoke"), { recursive: true });
  const outputPath = path.join(projectDir, ".dx", "forge", "adoption-smoke", "forge-smoke.json");
  const started = Date.now();
  const result = commandRunner("cargo", dxWwwCargoRunArgs(root, [
    "forge",
    "adoption-smoke",
    "--project",
    projectDir,
    "--format",
    "json",
    "--output",
    outputPath,
    "--fail-under",
    "90",
    "--quiet",
  ]));
  const durationMs = Date.now() - started;
  return {
    enabled: true,
    status: result.status === 0 ? "passed" : "failed",
    duration_ms: durationMs,
    output: outputPath,
    exit_code: result.status,
    stdout_tail: tail(result.stdout),
    stderr_tail: tail(result.stderr),
  };
}

function runCommand(command, args) {
  const result = spawnSync(command, args, {
    cwd: root,
    encoding: "utf8",
    windowsHide: true,
    timeout: 180000,
  });
  return {
    status: typeof result.status === "number" ? result.status : 1,
    stdout: result.stdout || "",
    stderr: result.stderr || result.error?.message || "",
  };
}

function inspectPublicRoutes(publicDir) {
  return routeDefinitions.map((definition) => {
    const htmlPath = path.join(publicDir, definition.html);
    const cleanHtmlPath = path.join(publicDir, definition.clean_html);
    const htmlSource = readText(htmlPath) || readText(cleanHtmlPath) || "";
    const links = extractLinks(htmlSource);
    const scripts = extractScripts(htmlSource);
    const headings = extractHeadings(htmlSource);
    const artifactRows = definition.artifacts.map((artifact) => {
      const artifactPath = path.join(publicDir, artifact);
      return {
        path: artifact,
        exists: fs.existsSync(artifactPath),
        bytes: fs.existsSync(artifactPath) ? fs.statSync(artifactPath).size : 0,
      };
    });
    const runtimeAsset = runtimeAssetPath(definition);
    const runtimeAssetExists = fs.existsSync(path.join(publicDir, runtimeAsset));
    const unsafeLinks = links.filter((link) => isUnsafeLink(link.href));
    const staticNoRuntime = scripts.length === 0 && !runtimeAssetExists;
    const passed =
      fs.existsSync(htmlPath) &&
      fs.existsSync(cleanHtmlPath) &&
      staticNoRuntime &&
      headings.h1_count >= 1 &&
      unsafeLinks.length === 0 &&
      artifactRows.every((artifact) => artifact.exists);

    return {
      route: definition.route,
      html: relativePath(htmlPath),
      clean_html: relativePath(cleanHtmlPath),
      html_exists: fs.existsSync(htmlPath),
      clean_html_exists: fs.existsSync(cleanHtmlPath),
      decoded_bytes: Buffer.byteLength(htmlSource),
      heading_counts: headings,
      link_count: links.length,
      unsafe_link_count: unsafeLinks.length,
      scripts: scripts.length,
      runtime_asset: runtimeAsset,
      runtime_asset_exists: runtimeAssetExists,
      static_no_runtime: staticNoRuntime,
      artifacts: artifactRows,
      passed,
    };
  });
}

function collectFindings(routes, noNodeModules, prepareResult) {
  const findings = [];
  if (!noNodeModules) {
    findings.push("node_modules exists in the adoption smoke project");
  }
  if (prepareResult.enabled && prepareResult.status !== "passed") {
    findings.push("adoption-smoke prepare command failed");
  }
  for (const route of routes) {
    if (!route.html_exists) findings.push(`${route.route} missing primary HTML`);
    if (!route.clean_html_exists) findings.push(`${route.route} missing clean index HTML`);
    if (!route.static_no_runtime) findings.push(`${route.route} is not static/no-runtime`);
    if (route.heading_counts.h1_count < 1) findings.push(`${route.route} missing h1`);
    if (route.unsafe_link_count > 0) findings.push(`${route.route} has unsafe links`);
    for (const artifact of route.artifacts) {
      if (!artifact.exists) findings.push(`${route.route} missing artifact ${artifact.path}`);
    }
  }
  return findings;
}

function scoreReport(routes, noNodeModules, findings, browserEvidence) {
  let score = 100;
  score -= findings.length * 8;
  if (!noNodeModules) score -= 20;
  if (routes.some((route) => !route.passed)) score -= 10;
  if (!browserEvidence.enabled) score -= 4;
  return Math.max(0, Math.min(100, score));
}

async function startStaticServer(publicDir) {
  const server = http.createServer((request, response) => {
    const filePath = resolvePublicPath(publicDir, request.url || "/");
    if (!filePath) {
      response.writeHead(404);
      response.end("not found");
      return;
    }
    response.writeHead(200, { "content-type": contentType(filePath) });
    response.end(fs.readFileSync(filePath));
  });
  await new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", resolve);
  });
  const { port } = server.address();
  return {
    baseUrl: `http://127.0.0.1:${port}`,
    close: () => new Promise((resolve) => server.close(resolve)),
  };
}

function resolvePublicPath(publicDir, requestUrl) {
  const parsed = new URL(requestUrl, "http://127.0.0.1");
  const decoded = decodeURIComponent(parsed.pathname);
  const clean = decoded.replace(/^\/+/, "");
  const candidates = [];
  if (!clean) {
    candidates.push("index.html");
  } else {
    candidates.push(clean, `${clean}.html`, path.join(clean, "index.html"));
  }
  for (const candidate of candidates) {
    const fullPath = path.resolve(publicDir, candidate);
    if (fullPath.startsWith(path.resolve(publicDir)) && fs.existsSync(fullPath) && fs.statSync(fullPath).isFile()) {
      return fullPath;
    }
  }
  return null;
}

async function measureHttpRoutes(baseUrl, definitions, samples) {
  const routes = [];
  for (const definition of definitions) {
    const url = new URL(definition.route, `${baseUrl}/`).toString();
    const timings = [];
    let status = 0;
    let bytes = 0;
    for (let index = 0; index < samples; index += 1) {
      const started = performance.now();
      const response = await fetch(url);
      const buffer = Buffer.from(await response.arrayBuffer());
      timings.push(performance.now() - started);
      status = response.status;
      bytes = buffer.length;
    }
    routes.push({
      route: definition.route,
      status,
      decoded_bytes: bytes,
      timing_ms: summarize(timings),
    });
  }
  return {
    enabled: true,
    samples,
    routes,
  };
}

async function measureBrowserRoutes(baseUrl, definitions) {
  if (typeof WebSocket === "undefined") {
    return { enabled: false, reason: "global WebSocket is unavailable in this Node.js runtime", routes: [] };
  }
  const chrome = await startChrome();
  if (!chrome) {
    return { enabled: false, reason: "Chrome or Edge executable not found", routes: [] };
  }
  try {
    const routes = [];
    for (const definition of definitions) {
      const url = new URL(definition.route, `${baseUrl}/`).toString();
      routes.push(await measureBrowserRoute(chrome, definition.route, url));
    }
    return { enabled: true, routes };
  } finally {
    chrome.close();
  }
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
  if (!chromePath) return null;
  const port = await freePort();
  const userDataDir = path.join(os.tmpdir(), `dx-forge-adoption-chrome-${Date.now()}`);
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

async function measureBrowserRoute(chrome, route, url) {
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
    const result = await client.send("Runtime.evaluate", {
      awaitPromise: true,
      returnByValue: true,
      expression: `(() => {
        const nav = performance.getEntriesByType("navigation")[0];
        const resources = performance.getEntriesByType("resource");
        return {
          title: document.title,
          h1_count: document.querySelectorAll("h1").length,
          link_count: document.querySelectorAll("a[href]").length,
          script_count: document.scripts.length,
          resource_count: resources.length + 1,
          dom_nodes: document.getElementsByTagName("*").length,
          load_event_ms: nav ? nav.loadEventEnd - nav.startTime : 0,
          dom_content_loaded_ms: nav ? nav.domContentLoadedEventEnd - nav.startTime : 0,
          transfer_size: nav ? nav.transferSize + Array.from(resources).reduce((sum, item) => sum + item.transferSize, 0) : 0,
          decoded_body_size: nav ? nav.decodedBodySize + Array.from(resources).reduce((sum, item) => sum + item.decodedBodySize, 0) : 0
        };
      })()`,
    });
    return { route, ...normalizeMetrics(result.result.value) };
  } finally {
    client.close();
    await fetch(`${chrome.baseUrl}/json/close/${tab.id}`).catch(() => {});
  }
}

async function freePort() {
  const server = http.createServer();
  await new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", resolve);
  });
  const { port } = server.address();
  await new Promise((resolve) => server.close(resolve));
  return port;
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

async function chromeNewPage(chrome) {
  let response = await fetch(`${chrome.baseUrl}/json/new`, { method: "PUT" });
  if (!response.ok) response = await fetch(`${chrome.baseUrl}/json/new`);
  if (!response.ok) throw new Error(`Chrome refused a new page: ${response.status}`);
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
            const timeout = setTimeout(() => rejectEvent(new Error(`Timed out waiting for ${method}`)), timeoutMs);
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
        message.error ? request.reject(new Error(message.error.message)) : request.resolve(message.result);
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

function killProcessTree(child) {
  if (process.platform === "win32") {
    const result = spawnSync("taskkill", ["/PID", String(child.pid), "/T", "/F"], {
      stdio: "ignore",
      timeout: 5000,
      windowsHide: true,
    });
    if (result.error) child.kill("SIGKILL");
  } else {
    child.kill("SIGTERM");
  }
}

function extractHeadings(html) {
  const counts = { h1_count: 0, h2_count: 0, h3_count: 0, h4_count: 0, h5_count: 0, h6_count: 0 };
  for (const match of html.matchAll(/<h([1-6])\b/gi)) {
    counts[`h${match[1]}_count`] += 1;
  }
  return counts;
}

function extractLinks(html) {
  return [...html.matchAll(/<a\b[^>]*\bhref=["']([^"']+)["'][^>]*>/gi)].map((match) => ({ href: match[1] }));
}

function extractScripts(html) {
  return [...html.matchAll(/<script\b/gi)];
}

function isUnsafeLink(href) {
  return /^javascript:/i.test(href) || /^data:/i.test(href);
}

function runtimeAssetPath(definition) {
  const withoutHtml = definition.html.replace(/\.html$/, "");
  return `${withoutHtml}.dxp.js`;
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

function normalizeMetrics(metrics) {
  return Object.fromEntries(
    Object.entries(metrics).map(([key, value]) => [
      key,
      typeof value === "number" ? Number(value.toFixed(3)) : value,
    ])
  );
}

function renderMarkdown(report) {
  const lines = [
    "# Forge Adoption Browser Smoke",
    "",
    `Generated: ${report.generated_at}`,
    `Project: \`${report.project_dir}\``,
    `Score: \`${report.score}\` / \`100\``,
    `Passed: \`${report.passed}\``,
    `No node_modules: \`${report.no_node_modules}\``,
    "",
    "## Routes",
    "",
    "| Route | Passed | Static/no-runtime | H1 | Links | Scripts | Artifacts | HTTP median | Browser load |",
    "| --- | --- | --- | --- | --- | --- | --- | --- | --- |",
    ...report.routes.map((route) => {
      const http = report.http.routes.find((item) => item.route === route.route);
      const browser = report.browser.routes.find((item) => item.route === route.route);
      const artifacts = route.artifacts.filter((artifact) => artifact.exists).length;
      return [
        route.route,
        route.passed,
        route.static_no_runtime,
        route.heading_counts.h1_count,
        route.link_count,
        route.scripts,
        `${artifacts}/${route.artifacts.length}`,
        http ? `${http.timing_ms.median} ms` : "n/a",
        browser ? `${browser.load_event_ms} ms` : "n/a",
      ].join(" | ");
    }).map((line) => `| ${line} |`),
    "",
    "## Findings",
    "",
    ...(report.findings.length ? report.findings.map((finding) => `- ${finding}`) : ["- none"]),
    "",
    "## Honest Scope",
    "",
    ...report.honest_scope.map((item) => `- ${item}`),
    "",
  ];
  return lines.join("\n");
}

function writeReport(report) {
  fs.mkdirSync(reportDir, { recursive: true });
  fs.writeFileSync(outJsonPath, `${JSON.stringify(report, null, 2)}\n`);
  fs.writeFileSync(outMdPath, renderMarkdown(report));
}

function readText(filePath) {
  try {
    return fs.readFileSync(filePath, "utf8");
  } catch (_) {
    return null;
  }
}

function relativePath(filePath) {
  return path.relative(root, filePath);
}

function contentType(filePath) {
  if (filePath.endsWith(".html")) return "text/html; charset=utf-8";
  if (filePath.endsWith(".json")) return "application/json; charset=utf-8";
  if (filePath.endsWith(".dxp")) return "application/octet-stream";
  return "text/plain; charset=utf-8";
}

function tail(value, maxLength = 1200) {
  const text = String(value || "").trim();
  return text.length <= maxLength ? text : text.slice(text.length - maxLength);
}

async function main() {
  const report = await buildReport();
  writeReport(report);
  console.log(
    JSON.stringify(
      {
        report: [path.relative(root, outJsonPath), path.relative(root, outMdPath)],
        score: report.score,
        passed: report.passed,
        browser_enabled: report.browser.enabled,
        route_count: report.route_count,
      },
      null,
      2
    )
  );
  if (!report.passed) process.exitCode = 1;
}

if (require.main === module) {
  main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
  });
}

module.exports = {
  buildReport,
  inspectPublicRoutes,
  renderMarkdown,
  routeDefinitions,
};
