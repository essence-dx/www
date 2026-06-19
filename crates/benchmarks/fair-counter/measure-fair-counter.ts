const fs = require("fs");
const http = require("http");
const path = require("path");
const { spawn, spawnSync } = require("child_process");
const { performance } = require("perf_hooks");
const zlib = require("zlib");

const root = path.resolve(__dirname, "..", "..");
const suiteRoot = __dirname;
const reportDir = path.join(root, "benchmarks", "reports");
fs.mkdirSync(reportDir, { recursive: true });

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

function createStaticServer(baseDir, extraRoutes = {}) {
  return http.createServer((req, res) => {
    if (extraRoutes[req.url]) {
      extraRoutes[req.url](req, res);
      return;
    }

    const url = new URL(req.url, "http://127.0.0.1");
    const pathname = decodeURIComponent(url.pathname);
    const cleanPath = pathname === "/" ? "/index.html" : pathname;
    const filePath = path.normalize(path.join(baseDir, cleanPath));
    const resolvedBase = path.resolve(baseDir);
    const resolvedFile = path.resolve(filePath);

    if (!resolvedFile.startsWith(resolvedBase) || !fs.existsSync(resolvedFile) || !fs.statSync(resolvedFile).isFile()) {
      res.writeHead(404, { "content-type": "text/plain; charset=utf-8" });
      res.end("Not found");
      return;
    }

    const body = fs.readFileSync(resolvedFile);
    res.writeHead(200, {
      "content-type": fileType(resolvedFile),
      "content-length": body.length,
    });
    res.end(body);
  });
}

function createHtmxServer() {
  const publicDir = path.join(suiteRoot, "htmx", "public");
  const htmxPath = path.join(suiteRoot, "htmx", "node_modules", "htmx.org", "dist", "htmx.min.js");
  let count = 0;

  return http.createServer((req, res) => {
    if (req.url === "/" || req.url === "/index.html") {
      const body = fs.readFileSync(path.join(publicDir, "index.html"));
      res.writeHead(200, {
        "content-type": "text/html; charset=utf-8",
        "content-length": body.length,
      });
      res.end(body);
      return;
    }

    if (req.url === "/htmx.min.js") {
      const body = fs.readFileSync(htmxPath);
      res.writeHead(200, {
        "content-type": "application/javascript; charset=utf-8",
        "content-length": body.length,
      });
      res.end(body);
      return;
    }

    if (req.method === "POST" && req.url === "/counter/increment") {
      count += 1;
      sendText(res, String(count));
      return;
    }

    if (req.method === "POST" && req.url === "/counter/decrement") {
      count -= 1;
      sendText(res, String(count));
      return;
    }

    if (req.method === "POST" && req.url === "/counter/reset") {
      count = 0;
      sendText(res, "0");
      return;
    }

    res.writeHead(404, { "content-type": "text/plain; charset=utf-8" });
    res.end("Not found");
  });
}

function sendText(res, text) {
  const body = Buffer.from(text);
  res.writeHead(200, {
    "content-type": "text/plain; charset=utf-8",
    "content-length": body.length,
  });
  res.end(body);
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

function stripBuffer(resource) {
  const { buffer, ...rest } = resource;
  return rest;
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
    min_ms: Number(sorted[0].toFixed(3)),
    median_ms: Number(pick(0.5).toFixed(3)),
    mean_ms: Number((sum / values.length).toFixed(3)),
    p95_ms: Number(pick(0.95).toFixed(3)),
    max_ms: Number(sorted[sorted.length - 1].toFixed(3)),
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

function formatBytes(value) {
  return `${value.toLocaleString()} B`;
}

async function waitFor(url, timeoutMs = 30000) {
  const started = Date.now();
  let lastError;
  while (Date.now() - started < timeoutMs) {
    try {
      const result = await fetchResource(url);
      if (result.ok) {
        return;
      }
    } catch (error) {
      lastError = error;
    }
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
  throw lastError || new Error(`Timed out waiting for ${url}`);
}

async function startNext() {
  const probe = http.createServer();
  const port = await new Promise((resolve, reject) => {
    probe.once("error", reject);
    probe.listen(0, "127.0.0.1", () => {
      const selected = probe.address().port;
      probe.close(() => resolve(selected));
    });
  });

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
    close: () => {
      if (process.platform === "win32") {
        spawnSync("taskkill", ["/PID", String(child.pid), "/T", "/F"], { stdio: "ignore" });
      } else {
        child.kill("SIGTERM");
      }
    },
  };
}

async function startDxWww() {
  const probe = http.createServer();
  const port = await new Promise((resolve, reject) => {
    probe.once("error", reject);
    probe.listen(0, "127.0.0.1", () => {
      const selected = probe.address().port;
      probe.close(() => resolve(selected));
    });
  });

  const command = "cargo";
  const args = [
    "run",
    "--release",
    "--target-dir",
    path.join(root, "target-bench-dx-www-release"),
    "-p",
    "dx-www-demo",
    "--bin",
    "demo-server",
  ];
  const child = spawn(command, args, {
    cwd: root,
    env: { ...process.env, PORT: String(port) },
    stdio: "ignore",
    windowsHide: true,
  });
  const baseUrl = `http://127.0.0.1:${port}`;
  await waitFor(`${baseUrl}/fair-counter`, 300000);

  return {
    baseUrl,
    close: () => {
      if (process.platform === "win32") {
        spawnSync("taskkill", ["/PID", String(child.pid), "/T", "/F"], { stdio: "ignore" });
      } else {
        child.kill("SIGTERM");
      }
    },
  };
}

function targetUrl(target) {
  return new URL(target.path || "/", target.baseUrl).toString();
}

async function measureTarget(target) {
  const indexUrl = targetUrl(target);
  const index = await fetchResource(indexUrl);
  const html = index.buffer.toString("utf8");
  const assetUrls = new Set(extractAssets(html, indexUrl));
  for (const asset of target.extraAssets || []) {
    assetUrls.add(new URL(asset, target.baseUrl).toString());
  }

  const assets = await Promise.all([...assetUrls].map((url) => fetchResource(url)));
  const resources = [index, ...assets];
  const compression = sumCompression(resources);
  const rootSamples = [];
  const fullSamples = [];

  for (let i = 0; i < 25; i += 1) {
    rootSamples.push((await fetchResource(indexUrl)).elapsed_ms);
  }

  for (let i = 0; i < 15; i += 1) {
    const started = performance.now();
    const first = await fetchResource(indexUrl);
    const fullAssetUrls = new Set(extractAssets(first.buffer.toString("utf8"), indexUrl));
    for (const asset of target.extraAssets || []) {
      fullAssetUrls.add(new URL(asset, target.baseUrl).toString());
    }
    await Promise.all([...fullAssetUrls].map((url) => fetchResource(url)));
    fullSamples.push(performance.now() - started);
  }

  const action = target.action
    ? await fetchResource(new URL(target.action.path, target.baseUrl).toString(), { method: target.action.method })
    : null;

  return {
    name: target.name,
    version: target.version,
    model: target.model,
    resources: resources.map(stripBuffer),
    resource_count: resources.length,
    total_decoded_bytes: resources.reduce((sum, item) => sum + item.decoded_bytes, 0),
    compression_estimate: compression,
    root_timing: summarize(rootSamples),
    full_route_timing: summarize(fullSamples),
    action: action ? stripBuffer(action) : null,
  };
}

function packageVersion(project, packageName) {
  const pkg = JSON.parse(fs.readFileSync(path.join(suiteRoot, project, "package.json"), "utf8"));
  return {
    dependencies: pkg.dependencies || {},
    devDependencies: pkg.devDependencies || {},
  }.dependencies[packageName] || {
    dependencies: pkg.dependencies || {},
    devDependencies: pkg.devDependencies || {},
  }.devDependencies[packageName];
}

function writeReports(results) {
  const generatedAt = new Date().toISOString();
  const rankedRaw = [...results].sort((a, b) => a.total_decoded_bytes - b.total_decoded_bytes);
  const rankedGzip = [...results].sort((a, b) => a.compression_estimate.gzip_bytes - b.compression_estimate.gzip_bytes);
  const rankedTiming = [...results].sort((a, b) => a.full_route_timing.median_ms - b.full_route_timing.median_ms);
  const dx = results.find((item) => item.name === "DX-WWW");
  const ratios = Object.fromEntries(
    results.map((item) => [
      item.name,
      {
        raw_vs_dx: Number((item.total_decoded_bytes / dx.total_decoded_bytes).toFixed(2)),
        gzip_vs_dx: Number((item.compression_estimate.gzip_bytes / dx.compression_estimate.gzip_bytes).toFixed(2)),
        brotli_vs_dx: Number((item.compression_estimate.brotli_bytes / dx.compression_estimate.brotli_bytes).toFixed(2)),
        full_median_vs_dx: Number((item.full_route_timing.median_ms / dx.full_route_timing.median_ms).toFixed(2)),
      },
    ])
  );

  const report = {
    generated_at: generatedAt,
    method: {
      description:
        "Equivalent minimal counter page, first-route HTML plus same-origin script/style/runtime assets. DX-WWW is served by its real Rust demo route; Next.js uses next start; Svelte/Astro use static local servers; HTMX uses a small local fragment server.",
      dx_note: "DX-WWW uses the compiler's micro-JS/no-WASM path for this tiny interaction.",
      timing_note: "Localhost HTTP fetch timing is a sanity check, not a browser Lighthouse or production CDN benchmark.",
    },
    versions: {
      next: packageVersion("next", "next"),
      react: packageVersion("next", "react"),
      svelte: packageVersion("svelte", "svelte"),
      vite: packageVersion("svelte", "vite"),
      astro: packageVersion("astro", "astro"),
      htmx: packageVersion("htmx", "htmx.org"),
    },
    results,
    ratios,
    rankings: {
      raw_bytes: rankedRaw.map((item) => item.name),
      gzip_bytes: rankedGzip.map((item) => item.name),
      full_route_median_ms: rankedTiming.map((item) => item.name),
    },
  };

  fs.writeFileSync(path.join(reportDir, "fair-counter-comparison.json"), `${JSON.stringify(report, null, 2)}\n`);

  const rows = results
    .map(
      (item) =>
        `| ${item.name} | ${item.model} | ${item.resource_count} | ${formatBytes(item.total_decoded_bytes)} | ${formatBytes(item.compression_estimate.gzip_bytes)} | ${formatBytes(item.compression_estimate.brotli_bytes)} | ${item.full_route_timing.median_ms} ms |`
    )
    .join("\n");

  const ratioRows = results
    .map(
      (item) =>
        `| ${item.name} | ${ratios[item.name].raw_vs_dx}x | ${ratios[item.name].gzip_vs_dx}x | ${ratios[item.name].brotli_vs_dx}x | ${ratios[item.name].full_median_vs_dx}x |`
    )
    .join("\n");

  const markdown = `# Fair Counter Comparison

Generated: ${generatedAt}

## Method

Equivalent minimal counter pages were built for DX-WWW, Next.js, Svelte, Astro, and HTMX. The measurement counts first-route HTML plus same-origin JS/CSS/runtime assets. DX-WWW is served by its real Rust demo route, Next.js uses \`next start\`, Svelte/Astro use static local servers, and HTMX uses a small local fragment server. DX-WWW uses the micro-JS/no-WASM path for this tiny interaction. Local timings are localhost HTTP sanity checks, not browser Lighthouse or production CDN results.

## Versions

- Next.js ${report.versions.next} / React ${report.versions.react}
- Svelte ${report.versions.svelte} / Vite ${report.versions.vite}
- Astro ${report.versions.astro}
- HTMX ${report.versions.htmx}

## Payload And Timing

| Target | Model | Requests | Raw decoded | gzip estimate | Brotli estimate | Full route median |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
${rows}

## Ratio Against DX-WWW

| Target | Raw | gzip | Brotli | Full route median |
| --- | ---: | ---: | ---: | ---: |
${ratioRows}

## Rankings

- Smallest raw payload: ${rankedRaw.map((item) => item.name).join(" -> ")}
- Smallest gzip estimate: ${rankedGzip.map((item) => item.name).join(" -> ")}
- Fastest localhost full-route median: ${rankedTiming.map((item) => item.name).join(" -> ")}

## Notes

- Astro uses static HTML plus a tiny vanilla script, which is idiomatic for this kind of page and is a very strong baseline.
- Svelte compiles the counter into a small client bundle, but it still ships more client JavaScript than the Astro static version.
- HTMX ships the htmx runtime and moves state mutation to the server endpoint, so it is not the same client-state model as DX-WWW, Svelte, or React.
- Next.js ships much more runtime JavaScript for this tiny client island, even after removing Tailwind, fonts, images, and extra demo content.
`;

  fs.writeFileSync(path.join(reportDir, "fair-counter-comparison.md"), markdown);
  console.log(JSON.stringify({ generated_at: generatedAt, results: results.map((item) => ({ name: item.name, raw: item.total_decoded_bytes, gzip: item.compression_estimate.gzip_bytes, brotli: item.compression_estimate.brotli_bytes, median_ms: item.full_route_timing.median_ms, requests: item.resource_count })), reports: [path.join(reportDir, "fair-counter-comparison.md"), path.join(reportDir, "fair-counter-comparison.json")] }, null, 2));
}

async function main() {
  const dxServer = await startDxWww();
  const svelteServer = await listen(createStaticServer(path.join(suiteRoot, "svelte", "dist")));
  const astroServer = await listen(createStaticServer(path.join(suiteRoot, "astro", "dist")));
  const htmxServer = await listen(createHtmxServer());
  const nextServer = await startNext();
  const servers = [dxServer, svelteServer, astroServer, htmxServer, nextServer];

  try {
    const targets = [
      {
        name: "DX-WWW",
        version: "current local",
        model: "Rust route + static HTML + micro JS",
        baseUrl: dxServer.baseUrl,
        path: "/fair-counter",
      },
      {
        name: "Next.js",
        version: packageVersion("next", "next"),
        model: "App Router + React client island",
        baseUrl: nextServer.baseUrl,
      },
      {
        name: "Svelte",
        version: packageVersion("svelte", "svelte"),
        model: "Svelte compiled client bundle",
        baseUrl: svelteServer.baseUrl,
      },
      {
        name: "Astro",
        version: packageVersion("astro", "astro"),
        model: "Static HTML + inline script",
        baseUrl: astroServer.baseUrl,
      },
      {
        name: "HTMX",
        version: packageVersion("htmx", "htmx.org"),
        model: "HTML + htmx runtime + server fragment",
        baseUrl: htmxServer.baseUrl,
        action: { method: "POST", path: "/counter/increment" },
      },
    ];

    const results = [];
    for (const target of targets) {
      results.push(await measureTarget(target));
    }
    writeReports(results);
  } finally {
    for (const server of servers.reverse()) {
      await server.close();
    }
  }
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
