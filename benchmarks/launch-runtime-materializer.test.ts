import assert from "node:assert";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");

function read(file) {
  return fs.readFileSync(file, "utf8");
}

test("launch runtime materializer creates live pages without node_modules", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-launch-runtime-"));
  fs.mkdirSync(path.join(dir, "app", "launch"), { recursive: true });
  fs.writeFileSync(
    path.join(dir, "app", "launch", "page.tsx"),
    "export default function Page(){ return <>{children}</>; }\n",
  );

  const output = execFileSync(process.execPath, [materializer, dir], {
    cwd: root,
    encoding: "utf8",
  });
  const result = JSON.parse(output);

  assert.equal(result.ok, true);
  assert.equal(result.noNodeModules, true);
  assert.ok(!fs.existsSync(path.join(dir, "node_modules")));
  assert.ok(fs.existsSync(path.join(dir, "app", "launch", "page.tsx.source-only")));
  assert.ok(!fs.existsSync(path.join(dir, "app", "launch", "page.tsx")));
  assert.ok(fs.existsSync(path.join(dir, "public", "favicon.svg")));
  assert.ok(fs.existsSync(path.join(dir, "favicon.svg")));
  assert.ok(fs.existsSync(path.join(dir, "pages", "favicon.svg.html")));
  assert.equal(read(path.join(dir, "pages", "_layout.html")).trim(), "<template>{children}</template>");

  const expectedPages = [
    ["index", "/"],
    ["automations", "/automations"],
    ["ui", "/ui"],
    ["database", "/database"],
    ["backend", "/backend"],
  ];
  for (const [fileName, route] of expectedPages) {
    const page = path.join(dir, "pages", `${fileName}.html`);
    assert.ok(fs.existsSync(page), `expected ${fileName}.html to exist`);
    assert.match(read(page), new RegExp(`data-dx-route="${route}"`));
  }

  const launch = read(path.join(dir, "pages", "index.html"));
  assert.doesNotMatch(launch, /\{children\}/);
  assert.match(launch, /data-dx-route="\/"/);
  assert.match(launch, /data-dx-component="auth-session-card"/);
  assert.doesNotMatch(launch, /data-dx-component="shadcn-ui-runtime-proof"/);
  assert.doesNotMatch(launch, /data-dx-shadcn-proof="runtime-source-primitives"/);
  assert.doesNotMatch(launch, /data-dx-shadcn-action="prepare-local-source-edit"/);
  assert.match(launch, /data-dx-component="launch-operating-dashboard"/);
  assert.match(launch, /data-dx-product-surface="launch-dashboard"/);
  assert.match(launch, /data-dx-dashboard-card="session"/);
  assert.match(launch, /data-dx-dashboard-card="payment"/);
  assert.match(launch, /data-dx-dashboard-card="automation"/);
  assert.match(launch, /data-dx-dashboard-card="database"/);
  assert.match(launch, /data-dx-dashboard-card="controls"/);
  assert.match(launch, /data-dx-component="launch-motion-dashboard-summary"/);
  assert.match(launch, /data-dx-dashboard-card="animation"/);
  assert.match(launch, /id="mission-motion-status"/);
  assert.match(launch, /id="mission-motion-detail"/);
  assert.match(launch, /data-dx-dashboard-action="sync-from-runtime"/);
  assert.match(launch, /data-dx-component="shadcn-dashboard-controls-runtime"/);
  assert.match(launch, /data-dx-package="shadcn\/ui\/button"/);
  assert.match(launch, /data-dx-dashboard-workflow="operator-controls"/);
  assert.match(launch, /data-dx-shadcn-dashboard-action="set-density"/);
  assert.match(launch, /data-dx-shadcn-dashboard-action="select-queue"/);
  assert.match(launch, /data-dx-shadcn-dashboard-keyboard="arrow-roving-focus"/);
  assert.match(launch, /data-dx-shadcn-dashboard-action="focus-target-card"/);
  assert.match(launch, /data-dx-shadcn-dashboard-controls-target="mission-payment-status"/);
  assert.match(launch, /data-dx-shadcn-dashboard-action="preview-dashboard-receipt"/);
  assert.match(launch, /data-dx-shadcn-dashboard-receipt="idle"/);
  assert.match(launch, /data-dx-component="launch-billing-checkout-workflow"/);
  assert.match(launch, /data-dx-dashboard-flow="billing-checkout"/);
  assert.match(launch, /data-dx-component="zod-form-card"/);
  assert.match(launch, /data-dx-package="state\/zustand"/);
  assert.match(launch, /data-dx-component="zustand-state-card"/);
  assert.match(launch, /data-dx-zustand-store="launch-counter"/);
  assert.match(launch, /data-dx-zustand-persist-key="dx-launch-counter"/);
  assert.match(launch, /data-dx-zustand-action="increment"/);
  assert.match(launch, /data-dx-zustand-action="toggle-review-mode"/);
  assert.match(launch, /data-dx-zustand-action="reset"/);
  assert.match(launch, /data-dx-zustand-action="rehydrate"/);
  assert.match(launch, /data-dx-component="launch-dashboard-state-workflow"/);
  assert.match(launch, /data-dx-zustand-store="launch-dashboard-settings"/);
  assert.match(launch, /data-dx-zustand-persist-key="dx-template-dashboard-settings"/);
  assert.match(launch, /data-dx-zustand-action="set-dashboard-density"/);
  assert.match(launch, /data-dx-zustand-action="select-dashboard-focus"/);
  assert.match(launch, /data-dx-zustand-action="save-dashboard-settings"/);
  assert.match(launch, /data-dx-zustand-action="reset-dashboard-settings"/);
  assert.match(launch, /data-dx-component="forge-safety-archive-status"/);
  assert.match(launch, /data-dx-safety-archive-contract="dx\.forge\.safety_archive_contract"/);
  assert.match(launch, /data-dx-forge-status-surface="safety-archive-status"/);
  assert.match(launch, /data-dx-zed-surface="safety-archive-status"/);
  assert.match(launch, /data-dx-safety-archive-operation="archive-before-delete"/);
  assert.match(launch, /data-dx-safety-archive-state="covered"/);
  assert.match(launch, /data-dx-safety-archive-safe-delete="true"/);
  assert.match(launch, /data-dx-safety-archive-rollback-coverage="100"/);
  assert.match(launch, /data-dx-safety-archive-receipt-count="3"/);
  assert.match(launch, /data-dx-safety-archive-boundary="local-cache-restore-inputs-no-remote-rollback"/);
  assert.match(launch, /Local package archive receipts exist/);
  assert.match(launch, /data-dx-package="api\/trpc"/);
  assert.match(launch, /data-dx-component="launch-trpc-api-dashboard-workflow"/);
  assert.match(launch, /data-dx-dashboard-workflow="typed-api-readiness"/);
  assert.match(launch, /data-dx-trpc-workflow="launch-api-readiness"/);
  assert.match(launch, /data-dx-trpc-action="check-health"/);
  assert.match(launch, /data-dx-trpc-action="prepare-launch-event"/);
  assert.match(launch, /data-trpc-interaction="health-query"/);
  assert.match(launch, /data-trpc-interaction="local-launch-event-mutation"/);
  assert.match(launch, /data-trpc-mutation-state="idle"/);
  assert.match(launch, /data-dx-package="animation\/motion"/);
  assert.match(launch, /data-dx-component="motion-animation-card"/);
  assert.match(launch, /data-dx-dashboard-workflow="motion-panel-orchestration"/);
  assert.match(launch, /data-dx-product-surface="launch-dashboard"/);
  assert.match(launch, /data-dx-motion-interaction="runtime-local-workflow"/);
  assert.doesNotMatch(launch, /data-dx-motion-interaction="runtime-local-demo"/);
  assert.match(launch, /data-dx-motion-interaction="advance-stage"/);
  assert.match(launch, /data-dx-motion-interaction="reverse-order"/);
  assert.match(launch, /data-dx-motion-interaction="reset-proof"/);
  assert.match(launch, /data-dx-motion-progress-bar/);
  assert.match(launch, /id="fumadocs-workflow"/);
  assert.match(launch, /class="card wide docs-workflow"/);
  assert.match(launch, /data-dx-component="launch-fumadocs-docs-workflow"/);
  assert.match(launch, /data-dx-dashboard-workflow="docs-help-changelog"/);
  assert.match(launch, /data-dx-product-surface="dashboard-help-content"/);
  assert.match(launch, /data-dx-fumadocs-action="safe-local-route-preview"/);
  assert.match(launch, /data-dx-fumadocs-local-response="idle"/);
  assert.match(launch, /data-dx-fumadocs-receipt-route="none"/);
  assert.match(launch, /aria-pressed="true"/);
  assert.match(launch, /role="status"[\s\S]{0,80}aria-live="polite"/);
  assert.doesNotMatch(launch, /id="fumadocs-proof"/);
  assert.doesNotMatch(launch, /docs-proof/);
  assert.doesNotMatch(launch, /data-dx-component="fumadocs-docs-navigation-proof"/);
  assert.match(launch, /data-dx-package="3d\/launch-scene"/);
  assert.match(launch, /data-dx-component="launch-scene-webgl-proof"/);
  assert.match(launch, /data-dx-scene-canvas/);
  assert.match(launch, /data-dx-scene-workflow-controls/);
  assert.match(launch, /data-dx-scene-workflow-selected-node="none"/);
  assert.match(launch, /data-dx-scene-workflow-action="select-node"/);
  assert.match(launch, /data-dx-scene-workflow-action="regress-performance"/);
  assert.match(launch, /data-dx-scene-workflow-action="reset-performance"/);
  assert.match(launch, /data-dx-component="launch-automation-connector-workflow"/);
  assert.match(launch, /data-dx-component="launch-automation-catalog-summary"/);
  assert.doesNotMatch(launch, /data-dx-component="automations-n8n-summary"/);
  assert.match(launch, /data-dx-package="automations\/n8n"/);
  assert.match(launch, /data-dx-automation-workflow="connector-readiness"/);
  assert.doesNotMatch(launch, /data-dx-automation-demo=/);
  assert.match(launch, /data-dx-automation-interaction="connector-picker"/);
  assert.match(launch, /data-dx-automation-interaction="workflow-readiness"/);
  assert.match(launch, /data-dx-automation-action="preview-run-receipt"/);
  assert.match(launch, /data-dx-automation-local-receipt="draft-workflow-receipt"/);
  assert.doesNotMatch(launch, /data-dx-automation-local-demo=/);
  assert.match(launch, /data-dx-automation-receipt-status="idle"/);
  assert.match(launch, /data-dx-component="wasm-bindgen-readiness-workflow"/);
  assert.match(launch, /data-dx-package="wasm\/bindgen"/);
  assert.match(launch, /data-dx-wasm-bindgen-status="missing-app-module"/);
  assert.match(launch, /data-dx-wasm-interaction="missing-module-state"/);
  assert.match(launch, /data-dx-wasm-interaction="local-add-readiness"/);
  assert.match(launch, /data-dx-wasm-action="run-local-add"/);
  assert.match(launch, /data-dx-wasm-add-result="idle"/);
  assert.match(launch, /<dx-icon name="pack:wasm-bindgen"/);
  assert.match(launch, /data-icon-source="dx-icons"/);
  assert.match(launch, /data-dx-component="dx-icon-runtime-markers"/);
  assert.ok(
    (launch.match(/data-dx-icon="/g) || []).length >= 6,
    "runtime page must preserve DX icon proof markers after icon rendering",
  );
  assert.match(launch, /data-dx-component="launch-drizzle-data-workflow"/);
  assert.match(launch, /data-dx-dashboard-workflow="sqlite-read-model"/);
  assert.match(launch, /data-dx-drizzle-action="apply-read-model"/);
  assert.match(launch, /data-dx-drizzle-action="select-read-model"/);
  assert.match(launch, /data-dx-drizzle-sql-preview/);
  assert.match(launch, /data-dx-component="launch-instantdb-runtime-dashboard-workflow"/);
  assert.match(launch, /data-dx-component="instantdb-runtime-dashboard-workflow"/);
  assert.match(launch, /data-dx-package="instantdb\/react"/);
  assert.match(launch, /data-dx-dashboard-workflow="realtime-data-readiness"/);
  assert.match(launch, /data-dx-instant-readiness="runtime-dashboard-readiness"/);
  assert.doesNotMatch(launch, /data-dx-instant-demo=/);
  assert.match(launch, /data-dx-instant-action="prepare-local-schema-receipt"/);
  assert.match(launch, /data-dx-instant-local-receipt="idle"/);
  assert.match(launch, /data-dx-supabase-readiness="client-readiness"/);
  assert.doesNotMatch(launch, /data-dx-supabase-demo=/);
  assert.match(launch, /<form[\s\S]*id="launch-form"/);
  assert.match(launch, /<canvas[\s\S]*id="dx-launch-scene"/);

  const runtime = read(path.join(dir, "public", "launch-runtime.js"));
  assert.doesNotMatch(runtime, /function bindShadcnProof\(\)/);
  assert.match(runtime, /function bindShadcnDashboardControls\(\)/);
  assert.doesNotMatch(runtime, /data-dx-shadcn-action='select-primitive'/);
  assert.match(runtime, /data-dx-shadcn-dashboard-action='set-density'/);
  assert.match(runtime, /data-dx-shadcn-dashboard-action='preview-dashboard-receipt'/);
  assert.match(runtime, /data-dx-shadcn-dashboard-action='focus-target-card'/);
  assert.doesNotMatch(runtime, /#shadcn-prepare-edit/);
  assert.match(runtime, /dxShadcnDashboardReceipt/);
  assert.doesNotMatch(runtime, /dxShadcnReceiptState/);
  assert.doesNotMatch(runtime, /bindShadcnProof\(\)/);
  assert.match(runtime, /bindShadcnDashboardControls\(\)/);
  assert.match(runtime, /function bindScene\(\)/);
  assert.match(runtime, /data-dx-scene-pixel-proof/);
  assert.match(runtime, /markScenePixelProof/);
  assert.match(runtime, /"aria-pressed"/);
  assert.match(runtime, /sceneNodes/);
  assert.match(runtime, /data-dx-scene-workflow-selected-node/);
  assert.match(runtime, /data-dx-scene-performance-band/);
  assert.match(runtime, /function renderMotionProof\(\)/);
  assert.match(runtime, /dashboard\.dataset\.dxDashboardMotion/);
  assert.match(runtime, /setText\(\s*"#mission-motion-status"/);
  assert.match(runtime, /setText\(\s*"#mission-motion-detail"/);
  assert.match(runtime, /data-dx-motion-interaction='advance-stage'/);
  assert.match(runtime, /data-dx-motion-interaction='reverse-order'/);
  assert.match(runtime, /data-dx-motion-interaction='reset-proof'/);
  assert.match(runtime, /localStorage\.getItem\("dx-launch-counter"\)/);
  assert.match(runtime, /localStorage\.setItem\(\s*"dx-launch-counter"/);
  assert.match(runtime, /localStorage\.getItem\("dx-template-dashboard-settings"\)/);
  assert.match(runtime, /localStorage\.setItem\(\s*"dx-template-dashboard-settings"/);
  assert.match(runtime, /function applyLaunchDashboardSettings/);
  assert.match(runtime, /function bindLaunchDashboardSettings/);
  assert.match(runtime, /#state-rehydrate/);
  assert.match(runtime, /Rehydrated dx-launch-counter/);
  assert.match(runtime, /function bindMissionControl\(\)/);
  assert.match(runtime, /function updateMissionControl\(/);
  assert.match(runtime, /data-dx-dashboard-action='sync-from-runtime'/);

  const runtimeScript = read(path.join(dir, "public", "launch-runtime.js"));
  assert.match(runtimeScript, /function bindAutomations\(\)/);
  assert.match(runtimeScript, /data-dx-automation-connector/);
  assert.match(runtimeScript, /dxAutomationReceiptStatus/);
  assert.match(runtimeScript, /#mission-trpc-health-check/);
  assert.match(runtimeScript, /#mission-trpc-launch-event/);
  assert.match(runtimeScript, /setTrpcWorkflow/);
  assert.match(runtimeScript, /function bindWasm\(\)/);
  assert.match(runtimeScript, /WebAssembly\.instantiate\(localAddWasmBytes\)/);
  assert.match(runtimeScript, /dxWasmAddResult/);
  assert.match(runtimeScript, /bindWasm\(\)/);
  assert.match(runtimeScript, /instantdbReceiptRuns: 0/);
  assert.match(runtimeScript, /function bindInstantDbRuntimeProof\(\)/);
  assert.match(runtimeScript, /dx-instantdb-local-/);
  assert.match(runtimeScript, /bindInstantDbRuntimeProof\(\)/);
  assert.ok(
    result.files.includes("favicon.svg"),
    "materializer must copy a root favicon for /favicon.svg compatibility",
  );
  assert.ok(
    result.files.includes("pages/favicon.svg.html"),
    "materializer must bridge /favicon.svg for stale dev servers that do not map public root assets",
  );

  for (const routeHandler of [
    "app/api/auth/session/route.ts",
    "app/api/checkout/route.ts",
    "app/api/ai/chat/route.ts",
    "app/api/trpc/health/route.ts",
  ]) {
    const file = path.join(dir, routeHandler);
    assert.ok(fs.existsSync(file), `expected ${routeHandler}`);
    assert.doesNotMatch(read(file), /export const \{?\s*(GET|POST)/);
  }

  const manifest = JSON.parse(read(path.join(dir, "public", "preview-manifest.json")));
  assert.equal(manifest.noNodeModulesRequired, true);
  assert.ok(manifest.routes.some((route) => route.route === "/"));
  assert.ok(
    manifest.routes.some((route) =>
      route.route === "/" && route.forgePackages.includes("automations/n8n"),
    ),
  );
  assert.ok(
    manifest.routes.some((route) =>
      route.route === "/" && route.forgePackages.includes("shadcn/ui/button"),
    ),
  );
  const launchRoute = manifest.routes.find((route) => route.route === "/");
  assert.ok(launchRoute, "expected / route metadata");
  assert.ok(launchRoute.forgePackages.includes("tanstack/query"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-query-dashboard-source"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-query-action"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-query-safe-action"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-query-result-status"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-safety-archive-contract"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-safety-archive-state"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-safety-archive-rollback-coverage"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-safety-archive-receipt-count"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-safety-archive-safe-delete"));
  assert.ok(
    result.files.includes(
      ".dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json",
    ),
    "materializer must copy the TanStack Query dashboard workflow receipt",
  );
  assert.ok(
    fs.existsSync(
      path.join(
        dir,
        ".dx",
        "forge",
        "receipts",
        "2026-05-22-tanstack-query-dashboard-data.json",
      ),
    ),
  );
  assert.ok(
    result.files.includes(
      ".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
    ),
    "materializer must copy the wasm-bindgen dashboard workflow receipt",
  );
  assert.ok(
    fs.existsSync(
      path.join(
        dir,
        ".dx",
        "forge",
        "receipts",
        "2026-05-22-wasm-bindgen-dashboard-workflow.json",
      ),
    ),
  );
  assert.ok(manifest.editableOperations.includes("insert_component"));
  assert.equal(manifest.editContract.schema, "dx.studio.launch_edit_contract");
  assert.equal(manifest.editContract.layoutPolicy, "responsive-design-system-grid");
  assert.equal(manifest.editContract.absolutePositioning, false);
  assert.equal(manifest.editContract.noNodeModulesRequired, true);
  assert.ok(
    manifest.editContract.editableSurfaces.some(
      (surface) =>
        surface.id === "launch-runtime-proof-grid" &&
        surface.selector === '[data-dx-editable-section="launch-package-proof-grid"]' &&
        surface.sourceFile === "pages/index.html" &&
        surface.packageIds.includes("shadcn/ui/button") &&
        surface.operations.includes("insert_component") &&
        surface.operations.includes("move_reorder_section"),
    ),
  );
  assert.ok(
    manifest.editContract.editableSurfaces.some(
      (surface) =>
        surface.id === "launch-runtime-dashboard" &&
        surface.selector === '[data-dx-editable-section="launch-dashboard"]' &&
        surface.sourceFile === "pages/index.html" &&
        surface.packageIds.includes("shadcn/ui/button") &&
        surface.packageIds.includes("shadcn/ui/item") &&
        surface.packageIds.includes("auth/better-auth") &&
        surface.packageIds.includes("payments/stripe-js") &&
        surface.packageIds.includes("state/zustand") &&
        surface.packageIds.includes("instantdb/react") &&
        surface.packageIds.includes("wasm/bindgen") &&
        surface.packageIds.includes("animation/motion") &&
        surface.operations.includes("move_reorder_section") &&
        surface.operations.includes("update_text_content"),
    ),
  );
  assert.ok(
    manifest.editContract.editableSurfaces.some(
      (surface) =>
        surface.id === "launch-runtime-dashboard-state-workflow" &&
        surface.selector === '[data-dx-component="launch-dashboard-state-workflow"]' &&
        surface.sourceFile === "pages/index.html" &&
        surface.packageIds.includes("state/zustand") &&
        surface.operations.includes("move_reorder_section") &&
        surface.operations.includes("update_text_content") &&
        surface.operations.includes("update_design_token"),
    ),
  );
  assert.ok(
    manifest.editContract.editableSurfaces.some(
      (surface) =>
        surface.id === "launch-runtime-forge-safety-archive" &&
        surface.selector === '[data-dx-component="forge-safety-archive-status"]' &&
        surface.sourceFile === "pages/index.html" &&
        surface.packageIds.includes("migration/static-site") &&
        !surface.packageIds.includes("www/template") &&
        surface.stateMarkers.includes("data-dx-safety-archive-contract") &&
        surface.stateMarkers.includes("data-dx-safety-archive-state") &&
        surface.stateMarkers.includes("data-dx-safety-archive-rollback-coverage") &&
        surface.stateMarkers.includes("data-dx-safety-archive-receipt-count") &&
        surface.stateMarkers.includes("data-dx-safety-archive-safe-delete") &&
        surface.operations.includes("move_reorder_section") &&
        surface.operations.includes("update_text_content"),
    ),
    "runtime preview manifest must expose the Forge safety/archive row for Studio inspection",
  );
  assert.ok(
    manifest.editContract.editableSurfaces.some(
      (surface) =>
        surface.id === "launch-runtime-query-dashboard-data" &&
        surface.selector === '[data-dx-component="tanstack-query-dashboard-data-workflow"]' &&
        surface.sourceFile === "pages/index.html" &&
        surface.packageIds.includes("tanstack/query") &&
        surface.operations.includes("move_reorder_section") &&
        surface.operations.includes("update_text_content") &&
        surface.operations.includes("update_design_token") &&
        surface.interactionSelectors.includes('[data-dx-query-action="refresh-dashboard-data"]') &&
        surface.stateMarkers.includes("data-dx-query-dashboard-source") &&
        surface.stateMarkers.includes("data-dx-query-dashboard-queue") &&
        surface.stateMarkers.includes("data-dx-query-package-id") &&
        surface.stateMarkers.includes("data-dx-query-package-status") &&
        surface.receiptPath ===
          ".dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json",
    ),
    "runtime preview manifest must expose the TanStack Query dashboard data workflow as editable",
  );
  assert.ok(
    manifest.editContract.editableSurfaces.some(
      (surface) =>
        surface.id === "launch-runtime-instantdb-dashboard" &&
        surface.selector === '[data-dx-component="instantdb-runtime-dashboard-workflow"]' &&
        surface.sourceFile === "pages/index.html" &&
        surface.packageIds.includes("instantdb/react") &&
        surface.operations.includes("move_reorder_section") &&
        surface.operations.includes("update_text_content") &&
        surface.operations.includes("insert_icon_media"),
    ),
  );
  assert.ok(
    manifest.editContract.editableSurfaces.some(
      (surface) =>
        surface.id === "launch-runtime-wasm-compute-dashboard" &&
        surface.selector === '[data-dx-component="launch-wasm-compute-dashboard-workflow"]' &&
        surface.sourceFile === "pages/index.html" &&
        surface.packageIds.includes("wasm/bindgen") &&
        surface.operations.includes("move_reorder_section") &&
        surface.operations.includes("update_text_content") &&
        surface.operations.includes("insert_icon_media") &&
        surface.interactionSelectors.includes('[data-dx-wasm-action="run-local-add"]') &&
        surface.stateMarkers.includes("data-dx-dashboard-metric") &&
        surface.stateMarkers.includes("data-dx-wasm-add-result") &&
        surface.receiptPath ===
          ".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
    ),
    "runtime preview manifest must expose wasm-bindgen local compute dashboard workflow as editable",
  );
  assert.ok(
    manifest.editContract.editableSurfaces.some(
      (surface) =>
        surface.id === "launch-runtime-docs" &&
        surface.selector === '[data-dx-component="launch-fumadocs-docs-workflow"]' &&
        surface.sourceFile === "pages/index.html" &&
        surface.packageIds.includes("content/fumadocs-next") &&
        surface.packageIds.includes("content/react-markdown") &&
        surface.operations.includes("move_reorder_section") &&
        surface.operations.includes("update_text_content") &&
        surface.operations.includes("insert_icon_media") &&
        surface.interactionSelectors.includes(
          '[data-dx-fumadocs-interaction="page-tree-selector"]',
        ) &&
        surface.interactionSelectors.includes(
          '[data-dx-fumadocs-action="safe-local-route-preview"]',
        ) &&
        surface.stateMarkers.includes("data-dx-fumadocs-rendered-route") &&
        surface.stateMarkers.includes("data-dx-fumadocs-local-response") &&
        surface.receiptPath ===
          ".dx/forge/receipts/2026-05-22-content-fumadocs-dashboard-workflow.json",
    ),
    "runtime preview manifest must expose Fumadocs docs workflow as editable",
  );
  assert.ok(
    !manifest.editContract.editableSurfaces.some((surface) =>
      surface.selector.includes("fumadocs-docs-navigation-proof"),
    ),
    "runtime preview manifest must not advertise old Fumadocs proof selector",
  );
  assert.ok(
    manifest.editContract.operations.some(
      (operation) =>
        operation.operation === "insert_icon_media" &&
        operation.selector === "[data-dx-media-slot]" &&
        operation.responsivePolicy === "use-existing-grid-and-design-tokens" &&
        operation.requiresNodeModules === false,
    ),
  );
});
