const assert = require("assert");
const crypto = require("crypto");
const fs = require("fs");
const path = require("path");
const test = require("node:test");
const { pathToFileURL } = require("url");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function sha256(relativePath) {
  return crypto.createHash("sha256").update(read(relativePath)).digest("hex");
}

test("launch template typed package-lane order matches the package-status receipt", async () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = await import(
    pathToFileURL(
      path.join(root, "examples/template/forge-package-status-read-model.ts"),
    ).href
  );
  const snapshot = readModel.readLaunchForgePackageStatus();

  assert.deepStrictEqual(
    snapshot.packageLaneVisibility.map((lane) => lane.packageId),
    status.package_lane_visibility.map((lane) => lane.package_id),
  );
  assert.deepStrictEqual(
    snapshot.dashboardSummary.lockedPackageNames,
    status.locked_package_names,
  );
  assert.strictEqual(
    snapshot.dashboardSummary.noNodeModulesRequired,
    status.no_node_modules_required,
  );
  assert.deepStrictEqual(
    snapshot.packages.map((row) => ({
      name: row.name,
      version: row.version,
      integrityState: row.integrityState,
      integrityHash: row.integrityHash,
      fileCount: row.fileCount,
      packageReceiptPath: row.packageReceiptPath,
    })),
    status.packages.map((row) => ({
      name: row.name,
      version: row.version,
      integrityState: row.integrity_state,
      integrityHash: row.integrity_hash,
      fileCount: row.file_count,
      packageReceiptPath: row.receipt_paths[0],
    })),
  );
  assert.deepStrictEqual(snapshot.dxCheckMetrics, status.dx_check_metrics);
  assert.deepStrictEqual(snapshot.zedReceiptSurfaces, status.zed_receipt_surfaces);

  for (const lane of snapshot.packageLaneVisibility) {
    assert.strictEqual(lane.launchClassification.runtimeProof, false);
    assert.strictEqual(lane.launchClassification.browserProof, false);
    assert.strictEqual(lane.launchClassification.liveProviderProof, false);
  }
});

test("launch template exposes the Forge package-status receipt as a typed read model", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const safetyArchiveContract = read(
    "examples/template/forge-safety-archive-contract.ts",
  );

  assert.match(readModel, /export type LaunchForgePackageStatusReadModel/);
  assert.match(readModel, /export function readLaunchForgePackageStatus/);
  assert.match(readModel, /export function launchForgePackageRows/);
  assert.match(readModel, /sourceReceipt: "\.dx\/forge\/package-status\.json"/);
  assert.match(readModel, /noNodeModulesRequired: true/);
  assert.match(
    readModel,
    new RegExp(`lockedPackageCount: ${status.locked_package_count}`),
  );
  assert.match(
    readModel,
    new RegExp(`cacheFileCount: ${status.package_lock.cache_file_count}`),
  );
  assert.match(
    readModel,
    new RegExp(`remoteCount: ${status.package_lock.remote_count}`),
  );
  assert.match(
    readModel,
    new RegExp(`mediaAssetCount: ${status.package_lock.media_asset_count}`),
  );

  for (const packageName of status.locked_package_names) {
    assert.match(
      readModel,
      new RegExp(packageName.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `${packageName} missing from the typed package status read model`,
    );
  }

  for (const surface of status.zed_receipt_surfaces) {
    assert.match(
      readModel,
      new RegExp(surface.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      `${surface} missing from Zed receipt surfaces`,
    );
  }

  const statusSource = read("examples/template/forge-package-status.ts");
  assert.match(statusSource, /readLaunchForgePackageStatus/);
  assert.match(statusSource, /receiptBackedStatus/);
  assert.match(statusSource, /safetyArchiveStatus: receiptBackedStatus\.safetyArchive/);
  assert.match(statusSource, /packageRows: launchForgePackageRows\(\)/);
  assert.match(readModel, /export type LaunchForgeSafetyArchiveStatus/);
  assert.match(safetyArchiveContract, /dx\.forge\.safety_archive_contract/);
  assert.match(safetyArchiveContract, /zedSurface: "safety-archive-status"/);
  assert.match(safetyArchiveContract, /operationSafetySurface: "archive-before-delete"/);
  assert.match(safetyArchiveContract, /safeForDestructivePackageOperations/);
  assert.match(safetyArchiveContract, /rollbackCoveragePercent/);

  const routeContract = read("examples/template/template-route-contract.ts");
  assert.match(routeContract, /components\/template-app\/forge-safety-archive-contract\.ts/);
  assert.match(routeContract, /components\/template-app\/forge-package-status-read-model\.ts/);

  const cliSource = read("dx-www/src/cli/mod.rs");
  assert.match(cliSource, /NEXT_FAMILIAR_FORGE_PACKAGE_STATUS_TS/);
  assert.match(cliSource, /NEXT_FAMILIAR_FORGE_PACKAGE_STATUS_READ_MODEL_TS/);
  assert.match(cliSource, /components\/template-app\/forge-package-status\.ts/);
  assert.match(cliSource, /components\/template-app\/forge-package-status-read-model\.ts/);
});

test("launch template exposes remote object HEAD health to Zed and dx-check consumers", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const remoteHeadHealth = readJson(
    "examples/template/.dx/forge/receipts/remotes/www-template-r2-head-health.json",
  );
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const remoteHeadContract = read(
    "examples/template/forge-remote-head-health-contract.ts",
  );
  const remoteHeadPanel = read(
    "examples/template/forge-remote-head-health-panel.tsx",
  );
  const routeContract = read("examples/template/template-route-contract.ts");

  assert.strictEqual(
    remoteHeadHealth.schema_version,
    "dx.forge.remote_object_head_health",
  );
  assert.strictEqual(remoteHeadHealth.provider_kind, "s3-compatible-object-storage");
  assert.strictEqual(remoteHeadHealth.package_id, "auth/better-auth");
  assert.strictEqual(remoteHeadHealth.safe_for_remote_install, false);
  assert.strictEqual(remoteHeadHealth.blocking_check_count, 2);
  assert.match(remoteHeadHealth.boundary, /without claiming any live network read/i);

  const statusRow = status.remote_object_head_health.find(
    (row) =>
      row.source_receipt_path ===
      ".dx/forge/receipts/remotes/www-template-r2-head-health.json",
  );
  assert.ok(statusRow, "package-status remote_object_head_health row is missing");
  assert.strictEqual(statusRow.safe_for_remote_install, false);
  assert.strictEqual(statusRow.check_count, remoteHeadHealth.checks.length);
  assert.strictEqual(
    statusRow.blocking_check_count,
    remoteHeadHealth.blocking_check_count,
  );

  for (const metric of [
    "forge_remote_head_health_receipts",
    "forge_remote_head_health_safe_receipts",
    "forge_remote_head_health_blocking_checks",
    "forge_remote_head_health_missing_required",
    "forge_remote_head_health_missing_optional",
    "forge_remote_head_health_byte_mismatches",
  ]) {
    assert.ok(status.dx_check_metrics.includes(metric), `${metric} missing`);
    assert.match(readModel, new RegExp(metric));
  }

  assert.ok(
    status.zed_receipt_surfaces.includes("remote-object-head-health"),
    "remote-object-head-health Zed receipt surface is missing",
  );
  assert.match(readModel, /export type LaunchForgeRemoteObjectHeadHealthStatus/);
  assert.match(readModel, /remoteObjectHeadHealth:/);
  assert.match(readModel, /remoteObjectHeadHealthBlockingCount: 2/);
  assert.match(statusSource, /remoteObjectHeadHealth: receiptBackedStatus\.remoteObjectHeadHealth/);
  assert.match(statusSource, /remoteObjectHeadHealthBlockingCount/);
  assert.match(remoteHeadContract, /dx\.forge\.remote_head_health_panel_contract/);
  assert.match(remoteHeadContract, /zedSurface: "remote-object-head-health"/);
  assert.match(remoteHeadContract, /blockingCheckCount: primary\?\.blockingCheckCount \?\? 0/);
  assert.match(remoteHeadContract, /sourceReceiptPath: primary\?\.sourceReceiptPath \?\? missingReceiptPath/);
  assert.match(remoteHeadPanel, /data-dx-remote-head-contract=\{panelContract\.schema\}/);
  assert.match(remoteHeadPanel, /data-dx-remote-head-blocking-count=\{panelContract\.blockingCheckCount\}/);
  assert.match(routeContract, /components\/template-app\/forge-remote-head-health-contract\.ts/);
  assert.match(routeContract, /www-template-r2-head-health\.json/);
});

test("State Management exposes package-lane dx-check visibility", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const packageReceipt = readJson(
    "examples/template/.dx/forge/receipts/packages/state-zustand.json",
  );
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const packageDoc = read("docs/packages/state-zustand.md");

  assert.match(readModel, /export type LaunchForgePackageLaneVisibilityState/);
  assert.match(readModel, /export const stateManagementPackageVisibility/);
  assert.match(statusSource, /stateManagementPackageVisibility/);
  assert.match(packageDoc, /## dx-check Visibility/);

  const statusVocabulary = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
  ];

  const stateVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "state/zustand",
  );

  assert.ok(stateVisibility, "State Management visibility row is missing");
  assert.strictEqual(stateVisibility.official_package_name, "State Management");
  assert.strictEqual(stateVisibility.upstream_package, "zustand");
  assert.strictEqual(stateVisibility.status, "present");
  assert.strictEqual(stateVisibility.receipt_status, "present");
  assert.deepStrictEqual(stateVisibility.status_vocabulary, statusVocabulary);
  assert.ok(
    stateVisibility.selected_surfaces.some(
      (surface) => surface.surface_id === "launch-dashboard-state-workflow",
    ),
    "State Management dashboard workflow surface is missing",
  );

  for (const metric of [
    "state_management_receipt_present",
    "state_management_receipt_stale",
    "state_management_missing_receipt",
    "state_management_blocked_surface",
    "state_management_unsupported_surface",
  ]) {
    assert.ok(
      stateVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from State Management visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  assert.strictEqual(
    packageReceipt.package.official_package_name,
    "State Management",
  );
  assert.strictEqual(packageReceipt.package.package_id, "state/zustand");
  assert.strictEqual(packageReceipt.package.upstream_package, "zustand");
  assert.deepStrictEqual(
    packageReceipt.package.dx_check_visibility.status_vocabulary,
    statusVocabulary,
  );
});

test("WebAssembly Bridge exposes package-lane dx-check visibility", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
  );
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const packageDoc = read("docs/packages/wasm-bindgen.md");

  const statusVocabulary = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
  ];

  const wasmVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "wasm/bindgen",
  );

  assert.ok(wasmVisibility, "WebAssembly Bridge visibility row is missing");
  assert.strictEqual(wasmVisibility.official_package_name, "WebAssembly Bridge");
  assert.strictEqual(wasmVisibility.upstream_package, "wasm-bindgen");
  assert.strictEqual(wasmVisibility.upstream_version, "0.2.121");
  assert.strictEqual(
    wasmVisibility.source_mirror,
    "G:/WWW/inspirations/wasm-bindgen",
  );
  assert.strictEqual(wasmVisibility.status, "present");
  assert.strictEqual(wasmVisibility.receipt_status, "present");
  assert.strictEqual(
    wasmVisibility.package_receipt_path,
    ".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
  );
  assert.deepStrictEqual(wasmVisibility.status_vocabulary, statusVocabulary);
  assert.ok(
    status.zed_receipt_surfaces.includes("wasm-bindgen-dashboard-workflow"),
    "WebAssembly Bridge Zed receipt surface is missing",
  );

  for (const surfaceId of [
    "dashboard-wasm-bindgen-workflow",
    "launch-wasm-compute-dashboard-workflow",
    "wasm-bindgen-readiness-workflow",
  ]) {
    assert.ok(
      wasmVisibility.selected_surfaces.some(
        (surface) => surface.surface_id === surfaceId,
      ),
      `${surfaceId} missing from WebAssembly Bridge visibility row`,
    );
  }

  for (const marker of [
    'data-dx-package="wasm/bindgen"',
    'data-dx-component="dashboard-wasm-bindgen-workflow"',
    'data-dx-component="launch-wasm-compute-dashboard-workflow"',
    'data-dx-component="wasm-bindgen-readiness-workflow"',
    'data-dx-wasm-action="run-local-add"',
    "data-dx-wasm-add-result",
  ]) {
    const sourceMarkers = wasmVisibility.selected_surfaces.flatMap(
      (surface) => surface.source_markers,
    );

    assert.ok(
      sourceMarkers.includes(marker),
      `${marker} missing from WebAssembly Bridge visibility markers`,
    );
  }

  for (const metric of [
    "webassembly_bridge_receipt_present",
    "webassembly_bridge_receipt_stale",
    "webassembly_bridge_missing_receipt",
    "webassembly_bridge_blocked_surface",
    "webassembly_bridge_unsupported_surface",
  ]) {
    assert.ok(
      wasmVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from WebAssembly Bridge visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  assert.match(readModel, /export const webAssemblyBridgePackageVisibility/);
  assert.match(statusSource, /webAssemblyBridgePackageVisibility/);
  assert.match(statusSource, /webAssemblyBridgeVisibility/);
  assert.match(packageDoc, /dx-check visibility/);
  assert.match(packageDoc, /package-status read model/);
  assert.deepStrictEqual(receipt.dx_check_visibility.statuses, statusVocabulary);
});

test("3D Scene System exposes hash-backed package-lane dx-check visibility", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/3d-launch-scene-dashboard-workflow.json",
  );
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const packageDoc = read("docs/packages/3d-scene-system.md");
  const sourceGuardRunbookFixture =
    "docs/packages/3d-scene-system.source-guard-runbook.json";
  const previewManifestMaterializer =
    "tools/launch/materialize-www-template.ts";

  const statusVocabulary = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
  ];

  const sceneVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "3d/launch-scene",
  );

  assert.ok(sceneVisibility, "3D Scene System visibility row is missing");
  assert.strictEqual(sceneVisibility.official_package_name, "3D Scene System");
  assert.strictEqual(
    sceneVisibility.upstream_package,
    "three + @react-three/fiber + @react-three/drei",
  );
  assert.strictEqual(
    sceneVisibility.source_mirror,
    "G:/WWW/inspirations/three.js; G:/WWW/inspirations/react-three-fiber; G:/WWW/inspirations/drei",
  );
  assert.strictEqual(sceneVisibility.status, "present");
  assert.strictEqual(sceneVisibility.receipt_status, "present");
  assert.strictEqual(
    sceneVisibility.package_receipt_path,
    ".dx/forge/receipts/3d-launch-scene-dashboard-workflow.json",
  );
  assert.deepStrictEqual(sceneVisibility.status_vocabulary, statusVocabulary);
  assert.ok(
    status.zed_receipt_surfaces.includes(
      "3d-scene-system:launch-scene-dashboard-workflow",
    ),
    "3D Scene System Zed receipt surface is missing",
  );

  const dashboardSurface = sceneVisibility.selected_surfaces.find(
    (surface) => surface.surface_id === "launch-scene-dashboard-workflow",
  );

  assert.ok(
    dashboardSurface,
    "3D Scene System dashboard workflow surface is missing",
  );
  assert.strictEqual(dashboardSurface.status, "present");
  assert.strictEqual(
    dashboardSurface.receipt_path,
    ".dx/forge/receipts/3d-launch-scene-dashboard-workflow.json",
  );
  assert.strictEqual(dashboardSurface.hash_algorithm, "sha256");
  const dashboardSurfaceFiles = new Set(dashboardSurface.files);
  assert.deepStrictEqual(
    dashboardSurface.file_hashes,
    Object.fromEntries(
      Object.entries(receipt.file_hashes).filter(([filePath]) =>
        dashboardSurfaceFiles.has(filePath),
      ),
    ),
  );

  for (const filePath of dashboardSurface.files) {
    assert.strictEqual(
      dashboardSurface.file_hashes[filePath],
      sha256(filePath),
      `${filePath} hash is stale in 3D Scene System visibility row`,
    );
  }

  const runbookSurface = sceneVisibility.selected_surfaces.find(
    (surface) => surface.surface_id === "three-scene-system-source-guard-runbook",
  );
  assert.ok(runbookSurface, "3D Scene System runbook surface is missing");
  assert.deepStrictEqual(runbookSurface.files, [sourceGuardRunbookFixture]);
  assert.strictEqual(
    runbookSurface.file_hashes[sourceGuardRunbookFixture],
    receipt.file_hashes[sourceGuardRunbookFixture],
  );
  assert.strictEqual(
    receipt.source_guard_runbook_fixture,
    sourceGuardRunbookFixture,
  );
  assert.strictEqual(
    receipt.preview_manifest_materializer,
    previewManifestMaterializer,
  );

  const materializerSurface = sceneVisibility.selected_surfaces.find(
    (surface) =>
      surface.surface_id === "three-scene-system-preview-manifest-materializer",
  );
  assert.ok(
    materializerSurface,
    "3D Scene System preview-manifest materializer surface is missing",
  );
  assert.deepStrictEqual(materializerSurface.files, [
    previewManifestMaterializer,
    "public/preview-.dx/build-cache/manifest.json",
  ]);
  assert.strictEqual(
    materializerSurface.file_hashes[previewManifestMaterializer],
    receipt.file_hashes[previewManifestMaterializer],
  );

  for (const marker of [
    'data-dx-package="3d/launch-scene"',
    'data-dx-component="launch-scene-webgl-proof"',
    'data-dx-component="launch-scene-dashboard-workflow"',
    'data-dx-dashboard-workflow="scene-visual-ops"',
    'data-dx-style-surface="launch-scene"',
    'data-dx-token-scope="3d/launch-scene"',
  ]) {
    assert.ok(
      dashboardSurface.source_markers.includes(marker),
      `${marker} missing from 3D Scene System visibility markers`,
    );
  }

  for (const metric of [
    "three_scene_system_receipt_present",
    "three_scene_system_receipt_stale",
    "three_scene_system_missing_receipt",
    "three_scene_system_blocked_surface",
    "three_scene_system_unsupported_surface",
    "three_scene_system_hash_manifest_present",
    "three_scene_system_hash_mismatch",
    "three_scene_system_receipt_hash_refresh_current",
    "three_scene_system_receipt_hash_refresh_stale",
    "three_scene_system_receipt_hash_refresh_missing",
    "three_scene_system_dx_style_compatibility_present",
    "three_scene_system_dx_style_compatibility_missing",
  ]) {
    assert.ok(
      sceneVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from 3D Scene System visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  assert.strictEqual(
    sceneVisibility.dx_style_compatibility.schema,
    "dx.forge.package.dx_style_compatibility",
  );
  assert.strictEqual(sceneVisibility.dx_style_compatibility.status, "present");
  assert.strictEqual(
    sceneVisibility.dx_style_compatibility.token_source,
    "examples/template/launch-scene.tsx",
  );
  assert.strictEqual(
    sceneVisibility.dx_style_compatibility.generated_css,
    "examples/template/styles/globals.css",
  );
  assert.strictEqual(sceneVisibility.dx_style_compatibility.runtime_proof, false);
  assert.ok(
    sceneVisibility.dx_style_compatibility.visible_surfaces.includes(
      "launch-scene-dashboard-workflow",
    ),
    "3D Scene System dx-style visible surface is missing",
  );
  for (const marker of [
    'data-dx-style-surface="launch-scene"',
    'data-dx-token-scope="3d/launch-scene"',
  ]) {
    assert.ok(
      sceneVisibility.dx_style_compatibility.data_dx_markers.includes(marker),
      `${marker} missing from 3D Scene System dx-style compatibility markers`,
    );
  }

  assert.match(readModel, /export const threeDSceneSystemPackageVisibility/);
  assert.match(readModel, /hashAlgorithm: "sha256"/);
  assert.match(readModel, /dxStyleCompatibility: \{/);
  assert.match(statusSource, /threeDSceneSystemPackageVisibility/);
  assert.match(statusSource, /threeDSceneSystemVisibility/);
  assert.match(packageDoc, /package-status read model/);
  assert.match(packageDoc, /three_scene_system_hash_mismatch/);
  assert.deepStrictEqual(receipt.dx_check_visibility.statuses, statusVocabulary);
});

test("Data Fetching & Cache exposes package-lane dx-check visibility", () => {
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const receipt = readJson(
    "examples/template/.dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json",
  );
  const readModel = read(
    "examples/template/forge-package-status-read-model.ts",
  );
  const statusSource = read("examples/template/forge-package-status.ts");
  const packageDoc = read("docs/packages/tanstack-query.md");

  const statusVocabulary = [
    "present",
    "stale",
    "missing-receipt",
    "blocked",
    "unsupported-surface",
  ];

  const queryVisibility = status.package_lane_visibility.find(
    (entry) => entry.package_id === "tanstack/query",
  );

  assert.ok(queryVisibility, "Data Fetching & Cache visibility row is missing");
  assert.strictEqual(
    queryVisibility.official_package_name,
    "Data Fetching & Cache",
  );
  assert.strictEqual(queryVisibility.upstream_package, "@tanstack/react-query");
  assert.strictEqual(queryVisibility.upstream_version, "5.100.10");
  assert.strictEqual(
    queryVisibility.source_mirror,
    "G:/WWW/inspirations/tanstack-query",
  );
  assert.strictEqual(queryVisibility.status, "present");
  assert.strictEqual(queryVisibility.receipt_status, "present");
  assert.strictEqual(
    queryVisibility.package_receipt_path,
    ".dx/forge/receipts/2026-05-22-tanstack-query-dashboard-data.json",
  );
  assert.deepStrictEqual(queryVisibility.status_vocabulary, statusVocabulary);
  assert.ok(
    status.zed_receipt_surfaces.includes(
      "data-fetching-cache-query-dashboard-workflow",
    ),
    "Data Fetching & Cache Zed receipt surface is missing",
  );

  for (const surfaceId of [
    "data-fetching-cache-query-dashboard-workflow",
    "data-fetching-cache-starter-dashboard-workflow",
  ]) {
    assert.ok(
      queryVisibility.selected_surfaces.some(
        (surface) => surface.surface_id === surfaceId,
      ),
      `${surfaceId} missing from Data Fetching & Cache visibility row`,
    );
  }

  const sourceMarkers = queryVisibility.selected_surfaces.flatMap(
    (surface) => surface.source_markers,
  );

  for (const marker of [
    'data-dx-package="tanstack/query"',
    'data-dx-component="tanstack-query-dashboard-data-workflow"',
    'data-dx-dashboard-workflow="query-backed-dashboard-data"',
    'data-dx-component="dashboard-tanstack-query-workflow"',
    "data-dx-query-check-visibility",
  ]) {
    assert.ok(
      sourceMarkers.includes(marker),
      `${marker} missing from Data Fetching & Cache visibility markers`,
    );
  }

  for (const metric of [
    "data_fetching_cache_receipt_present",
    "data_fetching_cache_receipt_stale",
    "data_fetching_cache_missing_receipt",
    "data_fetching_cache_blocked_surface",
    "data_fetching_cache_unsupported_surface",
  ]) {
    assert.ok(
      queryVisibility.dx_check_metrics.includes(metric),
      `${metric} missing from Data Fetching & Cache visibility row`,
    );
    assert.ok(
      status.dx_check_metrics.includes(metric),
      `${metric} missing from package-status dx_check_metrics`,
    );
    assert.match(readModel, new RegExp(metric));
  }

  assert.match(readModel, /export const dataFetchingCachePackageVisibility/);
  assert.match(statusSource, /dataFetchingCachePackageVisibility/);
  assert.match(statusSource, /dataFetchingCacheVisibility/);
  assert.match(packageDoc, /package-status read model/);
  assert.strictEqual(receipt.dx_check_visibility.current_status, "present");
  assert.deepStrictEqual(
    receipt.dx_check_visibility.status_legend.map((entry) => entry.status),
    statusVocabulary,
  );
});
