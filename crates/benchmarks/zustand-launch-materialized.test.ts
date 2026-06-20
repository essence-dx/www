const assert = require("node:assert");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");

function read(file) {
  return fs.readFileSync(file, "utf8");
}

test("materialized launch route includes visible Zustand state and dashboard workflow proof", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-zustand-launch-"));
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
  const launch = read(path.join(dir, "pages", "index.html"));
  const runtime = read(path.join(dir, "public", "launch-runtime.js"));
  const manifest = JSON.parse(read(path.join(dir, "public", "preview-.dx/build-cache/manifest.json")));

  assert.equal(result.ok, true);
  assert.equal(result.noNodeModules, true);
  assert.ok(!fs.existsSync(path.join(dir, "node_modules")));
  assert.match(launch, /data-dx-route="\/"/);
  assert.match(launch, /data-dx-package="state\/zustand"/);
  assert.match(launch, /data-dx-component="zustand-state-card"/);
  assert.match(launch, /data-dx-zustand-store="launch-counter"/);
  assert.match(launch, /data-dx-zustand-count="0"/);
  assert.match(launch, /data-dx-zustand-toggle-state="disabled"/);
  assert.match(launch, /data-dx-zustand-persist-key="dx-launch-counter"/);
  assert.match(launch, /data-dx-zustand-action="increment"/);
  assert.match(launch, /data-dx-zustand-action="toggle-review-mode"/);
  assert.match(launch, /data-dx-zustand-action="reset"/);
  assert.match(launch, /data-dx-zustand-action="rehydrate"/);
  assert.match(launch, /data-dx-component="launch-dashboard-state-shell"/);
  assert.match(launch, /data-dx-component="launch-dashboard-state-summary"/);
  assert.match(launch, /data-dx-dashboard-workflow="ui-state-persistence-shell"/);
  assert.match(launch, /data-dx-component="launch-dashboard-state-workflow"/);
  assert.match(launch, /data-dx-zustand-store="launch-dashboard-settings"/);
  assert.match(launch, /data-dx-zustand-persist-key="dx-template-dashboard-settings"/);
  assert.match(launch, /data-dx-zustand-dashboard-density="comfortable"/);
  assert.match(launch, /data-dx-zustand-dashboard-focus="payment"/);
  assert.match(launch, /data-dx-zustand-dashboard-applied="false"/);
  assert.match(launch, /data-dx-zustand-action="set-dashboard-density"/);
  assert.match(launch, /data-dx-zustand-action="select-dashboard-focus"/);
  assert.match(launch, /data-dx-zustand-action="save-dashboard-settings"/);
  assert.match(launch, /data-dx-zustand-action="reset-dashboard-settings"/);
  assert.match(launch, /data-dx-zustand-action="rehydrate-dashboard-settings"/);
  assert.match(launch, /data-dx-zustand-hydration-event="onFinishHydration"/);
  assert.match(launch, /data-dx-zustand-rehydrate-state="idle"/);
  assert.match(launch, /data-dx-check-package-lane-row="state\/zustand"/);
  assert.match(launch, /data-dx-check-package-lane-name="State Management"/);
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-helper="examples\/template\/state-management-receipt-hashes\.ts"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-json-command="node tools\/launch\/run-template-receipt-helper\.js examples\/template\/state-management-receipt-hashes\.ts --check --json"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-zed="state-management:receipt-hash-refresh"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-current-metric="state_management_receipt_hash_refresh_current"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-stale-metric="state_management_receipt_hash_refresh_stale"/,
  );
  assert.match(
    launch,
    /data-dx-check-package-lane-hash-refresh-missing-metric="state_management_receipt_hash_refresh_missing"/,
  );
  assert.match(runtime, /localStorage\.getItem\("dx-launch-counter"\)/);
  assert.match(runtime, /localStorage\.setItem\(\s*"dx-launch-counter"/);
  assert.match(runtime, /localStorage\.getItem\("dx-template-dashboard-settings"\)/);
  assert.match(runtime, /localStorage\.setItem\(\s*"dx-template-dashboard-settings"/);
  assert.match(runtime, /function applyLaunchDashboardSettings/);
  assert.match(runtime, /function bindLaunchDashboardSettings/);
  assert.match(runtime, /function markLaunchDashboardHydration/);
  assert.match(runtime, /function markLaunchDashboardRehydrateState/);
  assert.match(runtime, /#dashboard-settings-rehydrate/);
  assert.match(runtime, /data-dx-zustand-rehydrate-state/);
  assert.match(runtime, /Rehydrated dx-template-dashboard-settings/);
  assert.match(runtime, /launch-dashboard-state-shell/);
  assert.match(runtime, /#zustand-dashboard-state-summary/);
  assert.match(runtime, /data-dx-zustand-dashboard-applied/);
  assert.match(runtime, /#state-rehydrate/);
  assert.match(runtime, /Rehydrated dx-launch-counter/);
  assert.ok(
    manifest.routes.some((route) =>
      route.route === "/" && route.forgePackages.includes("state/zustand"),
    ),
  );
  const launchRoute = manifest.routes.find((route) => route.route === "/");
  assert.ok(launchRoute, "preview manifest must include /launch");
  assert.ok(
    launchRoute.dataDxMarkers.includes("data-dx-check-package-lane-hash-refresh-helper"),
    "preview manifest /launch route must expose State Management hash helper marker",
  );
  assert.ok(
    launchRoute.dataDxMarkers.includes("data-dx-check-package-lane-hash-refresh-json-command"),
    "preview manifest /launch route must expose State Management helper JSON command marker",
  );
  assert.ok(
    launchRoute.dataDxMarkers.includes("data-dx-check-package-lane-hash-refresh-zed"),
    "preview manifest /launch route must expose State Management Zed helper marker",
  );
  assert.ok(
    Array.isArray(manifest.sourceGuardRunbookFixtures),
    "preview manifest must expose source-guard runbook fixtures",
  );
  const stateManagementRunbookFixture = manifest.sourceGuardRunbookFixtures.find(
    (fixture) => fixture.packageId === "state/zustand",
  );
  assert.ok(
    stateManagementRunbookFixture,
    "preview manifest must expose the State Management source-guard runbook fixture",
  );
  assert.equal(stateManagementRunbookFixture.officialPackageName, "State Management");
  assert.equal(
    stateManagementRunbookFixture.fixture,
    "docs/packages/state-zustand.source-guard-runbook.json",
  );
  assert.equal(
    stateManagementRunbookFixture.guardId,
    "state-management-generated-starter-materialization",
  );
  assert.equal(stateManagementRunbookFixture.route, "/");
  assert.equal(stateManagementRunbookFixture.honestyLabel, "SOURCE-ONLY");
  assert.equal(stateManagementRunbookFixture.runtimeProof, false);
  assert.equal(
    stateManagementRunbookFixture.zedVisibility,
    "state-management:receipt-hash-refresh",
  );
  assert.ok(
    launchRoute.sourceGuardRunbookFixtures.includes(
      "docs/packages/state-zustand.source-guard-runbook.json",
    ),
    "preview manifest /launch route must point at the State Management runbook fixture",
  );
  const dxCheckSurface = manifest.editContract.editableSurfaces.find(
    (surface) => surface.id === "launch-runtime-dx-check-panel",
  );
  assert.ok(dxCheckSurface, "preview manifest must expose the dx-check panel surface");
  assert.ok(
    dxCheckSurface.packageIds.includes("state/zustand"),
    "dx-check panel surface must be package-scoped to State Management",
  );
  assert.ok(
    dxCheckSurface.stateMarkers.includes(
      "data-dx-check-package-lane-hash-refresh-helper",
    ),
    "dx-check panel surface must expose State Management hash helper markers",
  );
  assert.ok(
    dxCheckSurface.stateMarkers.includes(
      "data-dx-check-package-lane-hash-refresh-json-command",
    ),
    "dx-check panel surface must expose State Management helper JSON command markers",
  );
  assert.ok(
    dxCheckSurface.stateMarkers.includes(
      "data-dx-check-package-lane-hash-refresh-zed",
    ),
    "dx-check panel surface must expose State Management Zed helper markers",
  );
  assert.ok(
    manifest.editContract.editableSurfaces.some(
      (surface) =>
        surface.id === "launch-runtime-dashboard-state-shell" &&
        surface.selector === '[data-dx-component="launch-dashboard-state-shell"]' &&
        surface.sourceFile === "pages/index.html" &&
        surface.packageIds.includes("state/zustand") &&
        surface.operations.includes("move_reorder_section") &&
        surface.operations.includes("update_text_content") &&
        surface.operations.includes("update_design_token") &&
        surface.interactionSelectors.includes('[data-dx-zustand-action="set-dashboard-density"]') &&
        surface.interactionSelectors.includes('[data-dx-zustand-action="select-dashboard-focus"]') &&
        surface.interactionSelectors.includes('[data-dx-zustand-action="rehydrate-dashboard-settings"]') &&
        surface.stateMarkers.includes("data-dx-zustand-dashboard-density") &&
        surface.stateMarkers.includes("data-dx-zustand-dashboard-focus") &&
        surface.stateMarkers.includes("data-dx-zustand-command-hints") &&
        surface.stateMarkers.includes("data-dx-zustand-hydration-event") &&
        surface.stateMarkers.includes("data-dx-zustand-rehydrate-state") &&
        surface.receiptPath ===
          ".dx/forge/receipts/2026-05-22-state-zustand-dashboard-workflow.json",
    ),
    "runtime preview manifest must expose the Zustand dashboard shell as editable",
  );
});
