import assert from "node:assert";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `missing ${relativePath}`);
  return fs.readFileSync(filePath, "utf8");
}

test("default WWW starter does not hide the production fallback shell", () => {
  const globals = read("examples/template/styles/globals.css");
  const templateSources = read("dx-www/src/cli/default_template_sources.rs");

  assert.match(templateSources, /examples\/template\/styles\/globals\.css/);
  assert.doesNotMatch(
    globals,
    /data-dx-template=["']next-familiar["'][\s\S]{0,160}display\s*:\s*none/i,
    "the source-owned starter CSS must not hide the server-rendered next-familiar fallback shell",
  );
  assert.doesNotMatch(
    globals,
    /main\.dx-shell\.dx-page-shell[\s\S]{0,160}display\s*:\s*none/i,
    "the starter must keep the compiled server shell paintable for Lighthouse and no-JS browsers",
  );
});

test("production route shell stays source-owned and does not pull remote fonts", () => {
  const appRouteDelivery = read("core/src/delivery/app_route.rs");

  assert.doesNotMatch(appRouteDelivery, /fonts\.googleapis\.com/);
  assert.doesNotMatch(appRouteDelivery, /fonts\.gstatic\.com/);
  assert.doesNotMatch(appRouteDelivery, /JetBrains\+Mono/);
  assert.match(
    read("examples/template/styles/theme.css"),
    /--font-mono:\s*"JetBrains Mono"/,
    "JetBrains Mono should remain the preferred local/system font family without a render-blocking remote font request",
  );
});

test("production preview server uses a startup cache for contract files", () => {
  const previewCommand = read("dx-www/src/cli/preview_command.rs");
  const previewContract = read("dx-www/src/cli/preview_contract.rs");

  assert.match(previewCommand, /load_production_preview_cache/);
  assert.match(previewCommand, /Arc::clone\(&preview_cache\)/);
  assert.match(previewContract, /struct DxProductionPreviewCache/);
  assert.match(previewContract, /production_contract_wire_response_cached/);
  assert.match(previewContract, /production_contract_static_paths/);
});

test("production preview defaults and asset contracts match WWW build output", () => {
  const previewOptions = read("dx-www/src/cli/preview_options.rs");
  const previewCommand = read("dx-www/src/cli/preview_command.rs");
  const deployContract = read("dx-www/src/cli/deploy_adapter_contract.rs");

  assert.match(previewOptions, /DEFAULT_OUTPUT_DIR/);
  assert.doesNotMatch(previewOptions, /cwd\.join\("\.dx\/build"\)/);
  assert.match(previewCommand, /\.dx\/www\/output\/deploy-adapter\.json/);

  for (const extension of ["mjs", "webmanifest", "ico", "avif", "wasm", "woff2"]) {
    assert.match(
      deployContract,
      new RegExp(`"${extension}"`),
      `${extension} assets should be listed in the production deploy contract`,
    );
  }
});

test("production preview handles real HTTP semantics", () => {
  const previewCommand = read("dx-www/src/cli/preview_command.rs");
  const previewContract = read("dx-www/src/cli/preview_contract.rs");

  assert.match(previewCommand, /read_preview_http_request/);
  assert.match(previewCommand, /Content-Length/i);
  assert.match(previewCommand, /MAX_PREVIEW_REQUEST_BYTES/);
  assert.match(previewContract, /request_meta\.method == "HEAD"/);
  assert.match(previewContract, /wire_response_bytes\(/);
  assert.match(previewContract, /production_contract_cached_conditional_get_returns_304_for_matching_etag/);
  assert.match(previewContract, /application\/manifest\+json/);
  assert.match(previewContract, /application\/wasm/);
});

test("readiness tracks Lighthouse paint receipts without turning them into release claims", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");

  for (const marker of [
    "READINESS_LIGHTHOUSE_DEV_WEB_PERF_RECEIPT",
    "READINESS_LIGHTHOUSE_STATIC_WEB_PERF_RECEIPT",
    "READINESS_LIGHTHOUSE_DEV_WEB_PERF_COMMAND",
    "READINESS_LIGHTHOUSE_STATIC_WEB_PERF_COMMAND",
    "READINESS_CDP_PAINT_DEV_WEB_PERF_COMMAND",
    "READINESS_CDP_PAINT_STATIC_WEB_PERF_COMMAND",
    "readiness_lighthouse_paint_receipts_status",
    "browser_paint_receipt_is_current",
    "lighthouse_paint_receipt_is_current",
    "source_owned_cdp_paint_receipt_is_current",
    "dx.www.readiness.lighthouse_paint_receipts",
    "lighthouse_paint_receipts_current",
    "lighthouse_paint_receipts",
    "source_owned_cdp_current",
    "source_owned_cdp_paint",
    "metrics_complete",
    "browser_runtime_executed",
    "lighthouse-paint-receipts-missing",
    "lighthouse-paint-receipts-stale",
    "source-owned-cdp-paint-receipts-current-lighthouse-parity-needed",
    "first_contentful_paint_ms",
    "largest_contentful_paint_ms",
    "dx-source-owned-cdp-paint-collector",
    "measured-from-source-owned-cdp",
    "source-owned-cdp-browser-paint",
    "official-lighthouse-json-import",
    "measured-from-lighthouse-json",
    "node --test benchmarks/dx-www-cdp-paint-receipt.test.ts",
    "Local source-owned CDP or Lighthouse paint receipts can clear only the paint sub-gate",
  ]) {
    assert.match(readiness, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});
