const assert = require("node:assert");
const test = require("node:test");

const baseUrl = process.env.DX_LIVE_BASE_URL ?? "http://127.0.0.1:3000";

async function get(path) {
  const response = await fetch(`${baseUrl}${path}`);
  const text = await response.text();
  return { response, text };
}

test("live root route renders package proof instead of fallback", async () => {
  const { response, text } = await get("/");
  assert.equal(response.status, 200);
  assert.doesNotMatch(
    text,
    /data-dx-tsx-static-snapshot="layout-children-fallback"|>\s*\{children\}\s*</,
    "live root route must not render literal children fallback",
  );
  assert.match(text, /data-dx-route="\/"/);
  assert.ok((text.match(/data-dx-/g) ?? []).length >= 40, "expected many data-dx markers");
  assert.match(text, /<form id="launch-form"/);
  assert.match(text, /<canvas[\s\S]*id="dx-launch-scene"/);

  for (const proof of [
    "auth-session-card",
    "shadcn-dashboard-controls-runtime",
    "launch-billing-checkout-workflow",
    "zod-form-card",
    "zustand-state-card",
    "tanstack-query-dashboard-data-workflow",
    "launch-trpc-api-dashboard-workflow",
    "launch-motion-dashboard-summary",
    "motion-animation-card",
    "launch-scene-webgl-proof",
    "markdown-docs-card",
    "launch-automation-connector-workflow",
    "launch-automation-catalog-summary",
    "wasm-bindgen-readiness-workflow",
    "supabase-schema-query-workflow",
    "instantdb-runtime-dashboard-workflow",
    "launch-drizzle-data-workflow",
    "database-backend-card",
    "dx-studio-edit-markers",
  ]) {
    assert.match(text, new RegExp(`data-dx-component="${proof}"`), `missing ${proof}`);
  }

  assert.match(text, /data-dx-package="shadcn\/ui\/button"/);
  assert.match(text, /data-dx-dashboard-workflow="operator-controls"/);
  assert.match(text, /data-dx-shadcn-dashboard-action="set-density"/);
  assert.match(text, /data-dx-shadcn-dashboard-action="select-queue"/);
  assert.match(text, /data-dx-shadcn-dashboard-action="preview-dashboard-receipt"/);
  assert.match(text, /data-dx-shadcn-dashboard-keyboard="arrow-roving-focus"/);
  assert.match(text, /data-dx-shadcn-dashboard-action="focus-target-card"/);
  assert.match(text, /data-dx-shadcn-dashboard-controls-target="mission-payment-status"/);
  assert.doesNotMatch(text, /data-dx-shadcn-proof="runtime-source-primitives"/);
  assert.doesNotMatch(text, /data-dx-shadcn-action="select-primitive"/);
  assert.doesNotMatch(text, /data-dx-shadcn-action="prepare-local-source-edit"/);
  assert.doesNotMatch(text, /data-dx-shadcn-preview="local-source-edit"/);
  assert.match(text, /data-dx-package="tanstack\/query"/);
  assert.match(text, /data-dx-dashboard-workflow="query-backed-dashboard-data"/);
  assert.match(text, /data-dx-query-dashboard-source="launch-runtime-catalog"/);
  assert.match(text, /data-dx-query-dashboard-queue="package-readiness"/);
  assert.match(text, /data-dx-query-dashboard-package-count="30"/);
  assert.match(text, /data-dx-query-dashboard-role-count="22"/);
  assert.match(text, /data-dx-query-dashboard-row="tanstack\/query"/);
  assert.match(text, /data-dx-query-package-id="tanstack\/query"/);
  assert.match(text, /data-dx-query-package-status="ready"/);
  assert.match(text, /data-dx-query-action="refresh-dashboard-data"/);
  assert.match(text, /data-dx-query-safe-action="read-dashboard-catalog"/);
  assert.match(text, /data-dx-query-result-status="idle"/);
  assert.match(text, /id="query-package-count"/);
  assert.match(text, /id="mission-query-status"/);
  assert.doesNotMatch(text, /data-dx-query-endpoint="\/api\/health"/);
  assert.match(text, /data-dx-package="automations\/n8n"/);
  assert.match(text, /data-dx-automation-workflow="connector-readiness"/);
  assert.doesNotMatch(text, /data-dx-automation-demo=/);
  assert.match(text, /data-dx-automation-interaction="connector-picker"/);
  assert.match(text, /data-dx-automation-interaction="workflow-readiness"/);
  assert.match(text, /data-dx-automation-action="preview-run-receipt"/);
  assert.match(text, /data-dx-automation-missing-config="false"/);
  assert.match(text, /data-dx-package="api\/trpc"/);
  assert.match(text, /data-dx-trpc-workflow="launch-api-readiness"/);
  assert.match(text, /data-trpc-interaction="local-launch-event-mutation"/);
  assert.match(text, /data-dx-package="animation\/motion"/);
  assert.match(text, /data-dx-component="launch-motion-dashboard-summary"/);
  assert.match(text, /data-dx-dashboard-card="animation"/);
  assert.match(text, /id="mission-motion-status"/);
  assert.match(text, /id="mission-motion-detail"/);
  assert.match(text, /data-dx-component="motion-animation-card"/);
  assert.match(text, /data-dx-dashboard-workflow="motion-panel-orchestration"/);
  assert.match(text, /data-dx-product-surface="launch-dashboard"/);
  assert.match(text, /data-dx-motion-state="source-owned"/);
  assert.match(text, /data-dx-motion-progress="34"/);
  assert.match(text, /data-dx-motion-interaction="advance-stage"/);
  assert.match(text, /data-dx-motion-interaction="reverse-order"/);
  assert.match(text, /data-dx-motion-interaction="reset-proof"/);
  assert.match(text, /data-dx-motion-progress-bar/);
  assert.match(text, /data-dx-package="3d\/launch-scene"/);
  assert.match(text, /data-dx-scene-canvas/);
  assert.match(text, /data-dx-scene-workflow-controls/);
  assert.match(text, /data-dx-scene-workflow-action="select-node"/);
  assert.match(text, /data-dx-scene-workflow-action="regress-performance"/);
  assert.match(text, /data-dx-scene-workflow-action="reset-performance"/);
  assert.match(text, /data-dx-package="wasm\/bindgen"/);
  assert.match(text, /data-dx-wasm-bindgen-status="missing-app-module"/);
  assert.match(text, /data-dx-wasm-interaction="local-add-readiness"/);
  assert.match(text, /data-dx-wasm-action="run-local-add"/);
  assert.match(text, /data-dx-package="supabase\/client"/);
  assert.match(text, /data-dx-supabase-readiness="client-readiness"/);
  assert.doesNotMatch(text, /data-dx-supabase-demo=/);
  assert.match(text, /data-dx-supabase-interaction="config-readiness"/);
  assert.match(text, /data-dx-supabase-action="run-local-schema-query"/);
  assert.match(text, /data-dx-supabase-config-status="missing-config"/);
  assert.match(text, /data-dx-supabase-query-state="idle"/);
  assert.match(text, /data-dx-component="launch-instantdb-runtime-dashboard-workflow"/);
  assert.match(text, /data-dx-component="instantdb-runtime-dashboard-workflow"/);
  assert.match(text, /data-dx-package="instantdb\/react"/);
  assert.match(text, /data-dx-dashboard-workflow="realtime-data-readiness"/);
  assert.match(text, /data-dx-instant-readiness="runtime-dashboard-readiness"/);
  assert.doesNotMatch(text, /data-dx-instant-demo=/);
  assert.match(text, /data-dx-instant-required-env="NEXT_PUBLIC_INSTANT_APP_ID"/);
  assert.match(text, /data-dx-instant-action="prepare-local-schema-receipt"/);
  assert.match(text, /data-dx-instant-local-receipt="idle"/);
});

test("live preview manifest exposes TanStack Query dashboard workflow", async () => {
  const { response, text } = await get("/public/preview-manifest.json");
  assert.equal(response.status, 200);

  const manifest = JSON.parse(text);
  assert.equal(manifest.noNodeModulesRequired, true);

  const rootRoute = manifest.routes.find((route) => route.route === "/");
  assert.ok(rootRoute, "missing root preview route");
  assert.ok(rootRoute.forgePackages.includes("tanstack/query"));
  assert.ok(rootRoute.dataDxMarkers.includes("data-dx-query-dashboard-source"));
  assert.ok(rootRoute.dataDxMarkers.includes("data-dx-query-dashboard-queue"));
  assert.ok(rootRoute.dataDxMarkers.includes("data-dx-query-package-id"));
  assert.ok(rootRoute.dataDxMarkers.includes("data-dx-query-package-status"));
  assert.ok(rootRoute.dataDxMarkers.includes("data-dx-query-action"));
  assert.ok(rootRoute.dataDxMarkers.includes("data-dx-query-safe-action"));
  assert.ok(rootRoute.dataDxMarkers.includes("data-dx-query-result-status"));

  assert.ok(
    manifest.editContract.editableSurfaces.some(
      (surface) =>
        surface.id === "launch-runtime-query-dashboard-data" &&
        surface.selector === '[data-dx-component="tanstack-query-dashboard-data-workflow"]' &&
        surface.packageIds.includes("tanstack/query") &&
        surface.stateMarkers.includes("data-dx-query-dashboard-queue") &&
        surface.stateMarkers.includes("data-dx-query-package-id") &&
        surface.receiptPath ===
          ".dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json",
    ),
  );
  assert.ok(
    manifest.files.includes(
      ".dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json",
    ),
  );
});

test("manifest-advertised runtime routes return visible runtime pages", async () => {
  for (const route of ["/automations", "/ui", "/database", "/backend"]) {
    const { response, text } = await get(route);
    assert.equal(response.status, 200, `${route} should not 404`);
    assert.match(text, new RegExp(`data-dx-route="${route}"`));
    assert.doesNotMatch(text, /Page not found|404 Not Found/);
  }
});
