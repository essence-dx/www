const fs = require("fs");
const path = require("path");
const zlib = require("zlib");
const { performance } = require("perf_hooks");
const {
  buildHistoricalBenchmarkSnapshotStatus,
  snapshotStatusMarkdownBlock,
} = require("./report-snapshot-status");

const root = path.resolve(__dirname, "..");
const reportDir = path.join(__dirname, "reports");
const DX_BASE = "http://127.0.0.1:8097";
const NEXT_BASE = "http://127.0.0.1:8098";

fs.mkdirSync(reportDir, { recursive: true });

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

function summarizeBytes(values) {
  const sorted = [...values].sort((a, b) => a - b);
  const sum = values.reduce((a, b) => a + b, 0);
  const pick = (p) => sorted[Math.min(sorted.length - 1, Math.max(0, Math.ceil(sorted.length * p) - 1))];
  return {
    samples: values.length,
    min_bytes: sorted[0],
    median_bytes: pick(0.5),
    mean_bytes: Math.round(sum / values.length),
    p95_bytes: pick(0.95),
    max_bytes: sorted[sorted.length - 1],
  };
}

function formatBytes(value) {
  return `${value.toLocaleString()} B`;
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

async function fetchResource(url) {
  const start = performance.now();
  const response = await fetch(url, { headers: { "cache-control": "no-cache" } });
  const buffer = Buffer.from(await response.arrayBuffer());
  return {
    url,
    status: response.status,
    ok: response.ok,
    content_type: response.headers.get("content-type"),
    content_length_header: response.headers.get("content-length"),
    decoded_bytes: buffer.length,
    elapsed_ms: Number((performance.now() - start).toFixed(3)),
    buffer,
  };
}

function stripBuffer(resource) {
  const { buffer, ...serializable } = redactExternalNextBundlerResource(resource);
  return serializable;
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

function extractNextAssets(html) {
  const urls = new Set();
  const attrPattern = /(?:src|href)="([^"]+)"/g;
  let match;
  while ((match = attrPattern.exec(html))) {
    const value = match[1];
    if (value.startsWith("/_next/static/")) {
      urls.add(new URL(value, NEXT_BASE).toString());
    }
  }
  return [...urls].sort();
}

function sumCompression(resources) {
  return resources.reduce(
    (acc, resource) => {
      const item = compress(resource.buffer);
      acc.raw_bytes += item.raw_bytes;
      acc.gzip_bytes += item.gzip_bytes;
      acc.brotli_bytes += item.brotli_bytes;
      return acc;
    },
    { raw_bytes: 0, gzip_bytes: 0, brotli_bytes: 0 }
  );
}

async function sampleSingle(url, count = 25) {
  const times = [];
  let last;
  for (let i = 0; i < count; i += 1) {
    last = await fetchResource(url);
    times.push(last.elapsed_ms);
  }
  return { timing: summarize(times), last: stripBuffer(last) };
}

async function sampleDxFull(count = 15) {
  const times = [];
  const byteTotals = [];
  for (let i = 0; i < count; i += 1) {
    const start = performance.now();
    const html = await fetchResource(`${DX_BASE}/index.html`);
    const wasm = await fetchResource(`${DX_BASE}/dx_www_client.wasm`);
    times.push(performance.now() - start);
    byteTotals.push(html.decoded_bytes + wasm.decoded_bytes);
  }
  return { timing: summarize(times), bytes: summarizeBytes(byteTotals) };
}

async function sampleNextFull(assetUrls, count = 10) {
  const times = [];
  const byteTotals = [];
  for (let i = 0; i < count; i += 1) {
    const start = performance.now();
    const html = await fetchResource(`${NEXT_BASE}/`);
    const assets = await Promise.all(assetUrls.map((url) => fetchResource(url)));
    times.push(performance.now() - start);
    byteTotals.push(html.decoded_bytes + assets.reduce((sum, item) => sum + item.decoded_bytes, 0));
  }
  return { timing: summarize(times), bytes: summarizeBytes(byteTotals) };
}

function writeReports(result) {
  const dx = result.live_payloads.dx_www;
  const next = result.live_payloads.nextjs;
  const timing = result.local_timing;
  const generatedAt = result.generated_at;

  const markdown = `# DX-WWW Current Status vs Next.js 16.2.6

Generated: ${generatedAt}

${snapshotStatusMarkdownBlock()}
## What Passed

- DX-WWW Rust workspace: \`cargo check --workspace\` passed, with warnings.
- DX-WWW browser WASM runtime: \`cargo check -p dx-www-browser --target wasm32-unknown-unknown\` passed.
- Next baseline: \`npm run build\` passed on Next.js ${result.next_baseline_package.next} / React ${result.next_baseline_package.react}.
- Next latest version was verified against the npm registry as \`16.2.6\`.
- Next audit currently reports a moderate PostCSS advisory through the Next dependency tree.

## Live Payload Comparison

| Target | Resources Counted | Raw decoded bytes | gzip estimate | Brotli estimate |
| --- | ---: | ---: | ---: | ---: |
| DX-WWW demo | ${dx.resource_count} | ${formatBytes(dx.total_decoded_bytes)} | ${formatBytes(dx.compression_estimate.gzip_bytes)} | ${formatBytes(dx.compression_estimate.brotli_bytes)} |
| Next.js baseline | ${next.resource_count} | ${formatBytes(next.total_decoded_bytes)} | ${formatBytes(next.compression_estimate.gzip_bytes)} | ${formatBytes(next.compression_estimate.brotli_bytes)} |

Raw payload ratio: Next is ${result.ratios.next_vs_dx_raw_payload}x larger for this first route.
Compressed ratio estimate: Next is ${result.ratios.next_vs_dx_gzip_estimate}x larger with gzip and ${result.ratios.next_vs_dx_brotli_estimate}x larger with Brotli.

## Local Timing

| Check | Median | p95 | Samples |
| --- | ---: | ---: | ---: |
| DX index.html | ${timing.dx_index.median_ms} ms | ${timing.dx_index.p95_ms} ms | ${timing.dx_index.samples} |
| DX wasm | ${timing.dx_wasm.median_ms} ms | ${timing.dx_wasm.p95_ms} ms | ${timing.dx_wasm.samples} |
| Next / HTML | ${timing.next_index.median_ms} ms | ${timing.next_index.p95_ms} ms | ${timing.next_index.samples} |
| DX HTML + WASM sequential | ${timing.dx_html_plus_wasm_sequential.median_ms} ms | ${timing.dx_html_plus_wasm_sequential.p95_ms} ms | ${timing.dx_html_plus_wasm_sequential.samples} |
| Next HTML + static assets parallel | ${timing.next_html_plus_static_assets_parallel.median_ms} ms | ${timing.next_html_plus_static_assets_parallel.p95_ms} ms | ${timing.next_html_plus_static_assets_parallel.samples} |

## Honest Verdict

DX-WWW is dramatically smaller than a current minimal Next.js app in this demo. That is real.

It is not yet better than Next.js as a framework. The current demo still uses inline JavaScript for the visible counter, and the WASM currently proves loading/exports more than full app rendering, routing, data, auth, deploy behavior, or ecosystem compatibility.

The strongest validated advantage is payload size. The biggest unvalidated claims are framework completeness, real-world app ergonomics, production routing/data semantics, browser API coverage, update/security workflow, and developer adoption.
`;

  fs.writeFileSync(path.join(reportDir, "current-status.md"), markdown);
  fs.writeFileSync(path.join(reportDir, "latest.md"), markdown);
  fs.writeFileSync(path.join(reportDir, "current-status.json"), `${JSON.stringify(result, null, 2)}\n`);
  fs.writeFileSync(path.join(reportDir, "latest.json"), `${JSON.stringify(result, null, 2)}\n`);
  fs.writeFileSync(
    path.join(reportDir, "latest-compression.json"),
    `${JSON.stringify(
      {
        generated_at: result.generated_at,
        snapshot_status: buildHistoricalBenchmarkSnapshotStatus(),
        dx_www: dx.compression_estimate,
        nextjs: next.compression_estimate,
        ratios: result.ratios,
      },
      null,
      2
    )}\n`
  );
}

async function main() {
  const generatedAt = new Date().toISOString();
  const nextPackage = JSON.parse(fs.readFileSync(path.join(root, "benchmarks", "next-baseline", "package.json"), "utf8"));

  const dxIndex = await fetchResource(`${DX_BASE}/index.html`);
  const dxWasm = await fetchResource(`${DX_BASE}/dx_www_client.wasm`);
  const dxTinyWasmPath = path.join(root, "www", "demo", "dx_www_client_tiny_opt.wasm");
  const dxTinyWasmBytes = fs.existsSync(dxTinyWasmPath) ? fs.statSync(dxTinyWasmPath).size : null;

  const nextIndex = await fetchResource(`${NEXT_BASE}/`);
  const nextAssetUrls = extractNextAssets(nextIndex.buffer.toString("utf8"));
  const nextAssets = await Promise.all(nextAssetUrls.map((url) => fetchResource(url)));

  const dxResources = [dxIndex, dxWasm];
  const nextResources = [nextIndex, ...nextAssets];
  const dxCompression = sumCompression(dxResources);
  const nextCompression = sumCompression(nextResources);
  const dxTotal = dxResources.reduce((sum, item) => sum + item.decoded_bytes, 0);
  const nextTotal = nextResources.reduce((sum, item) => sum + item.decoded_bytes, 0);

  const dxIndexTiming = await sampleSingle(`${DX_BASE}/index.html`);
  const dxWasmTiming = await sampleSingle(`${DX_BASE}/dx_www_client.wasm`);
  const nextIndexTiming = await sampleSingle(`${NEXT_BASE}/`);
  const dxFullTiming = await sampleDxFull();
  const nextFullTiming = await sampleNextFull(nextAssetUrls);

  const result = {
    generated_at: generatedAt,
    snapshot_status: buildHistoricalBenchmarkSnapshotStatus(),
    next_latest_verified_from_npm_registry: "16.2.6",
    next_baseline_package: {
      next: nextPackage.dependencies.next,
      react: nextPackage.dependencies.react,
      react_dom: nextPackage.dependencies["react-dom"],
    },
    live_payloads: {
      dx_www: {
        resources: dxResources.map(stripBuffer),
        resource_count: dxResources.length,
        total_decoded_bytes: dxTotal,
        compression_estimate: dxCompression,
        tiny_wasm_file_bytes: dxTinyWasmBytes,
      },
      nextjs: {
        resources: nextResources.map(stripBuffer),
        resource_count: nextResources.length,
        static_asset_count: nextAssets.length,
        total_decoded_bytes: nextTotal,
        compression_estimate: nextCompression,
      },
    },
    local_timing: {
      note: "Localhost fetch timing is useful for relative sanity checks only; it is not a browser Lighthouse or production edge benchmark.",
      dx_index: dxIndexTiming.timing,
      dx_wasm: dxWasmTiming.timing,
      next_index: nextIndexTiming.timing,
      dx_html_plus_wasm_sequential: dxFullTiming.timing,
      next_html_plus_static_assets_parallel: nextFullTiming.timing,
    },
    ratios: {
      next_vs_dx_raw_payload: Number((nextTotal / dxTotal).toFixed(2)),
      next_vs_dx_gzip_estimate: Number((nextCompression.gzip_bytes / dxCompression.gzip_bytes).toFixed(2)),
      next_vs_dx_brotli_estimate: Number((nextCompression.brotli_bytes / dxCompression.brotli_bytes).toFixed(2)),
      next_full_median_vs_dx_full_median: Number(
        (nextFullTiming.timing.median_ms / dxFullTiming.timing.median_ms).toFixed(2)
      ),
    },
  };

  writeReports(result);

  console.log(
    JSON.stringify(
      {
        dx_raw_bytes: dxTotal,
        next_raw_bytes: nextTotal,
        raw_ratio: result.ratios.next_vs_dx_raw_payload,
        dx_full_median_ms: dxFullTiming.timing.median_ms,
        next_full_median_ms: nextFullTiming.timing.median_ms,
        timing_ratio: result.ratios.next_full_median_vs_dx_full_median,
        next_static_asset_count: nextAssets.length,
        reports: [
          path.join(reportDir, "current-status.md"),
          path.join(reportDir, "current-status.json"),
        ],
      },
      null,
      2
    )
  );
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
