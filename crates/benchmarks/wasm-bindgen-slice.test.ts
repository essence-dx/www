const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "wasm-bindgen");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("wasm-bindgen slice tracks real generated web glue surface", () => {
  const upstreamWeb = read(path.join(mirror, "crates", "cli", "tests", "reference", "targets-target-web.js"));
  const upstreamMemory = read(path.join(mirror, "crates", "cli", "tests", "reference", "wasm-export-types.js"));
  const upstreamDataView = read(path.join(mirror, "crates", "cli", "tests", "reference", "web-sys.bg.js"));
  const upstreamClosures = read(path.join(mirror, "crates", "cli", "tests", "reference", "closures.bg.js"));
  const upstreamTypes = read(path.join(mirror, "crates", "cli", "tests", "reference", "targets-target-web-atomics.d.ts"));
  const upstreamAllocatorTypes = read(path.join(mirror, "crates", "cli", "tests", "reference", "wasm-export-types.d.ts"));
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_wasm_bindgen.rs"));
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));
  const packageCatalog = read(path.join(root, "examples", "template", "package-catalog.ts"));
  const cliMod = read(path.join(root, "dx-www", "src", "cli", "mod.rs"));
  const studioManifest = read(path.join(root, "dx-www", "src", "cli", "studio_manifest.rs"));
  const launchShell = read(path.join(root, "examples", "template", "template-shell.tsx"));
  const launchProof = read(path.join(root, "examples", "template", "wasm-interop-status.tsx"));
  const runtimeLaunch = read(path.join(root, "tools", "launch", "runtime-template", "pages", "index.html"));
  const runtimeJs = read(path.join(root, "tools", "launch", "runtime-template", "assets", "launch-runtime.ts"));
  const editContract = read(path.join(root, "examples", "template", "dx-studio-edit-contract.ts"));
  const routeContract = read(path.join(root, "examples", "template", "template-route-contract.ts"));
  const runtimeMaterializer = read(path.join(root, "tools", "launch", "materialize-www-template.ts"));
  const dashboardReceipt = read(
    path.join(
      root,
      "examples",
      "www-template",
      ".dx",
      "forge",
      "receipts",
      "2026-05-22-wasm-bindgen-dashboard-workflow.json",
    ),
  );
  const dashboardPage = read(path.join(root, "examples", "dashboard", "src", "pages", "Dashboard.tsx"));
  const dashboardComponent = read(
    path.join(root, "examples", "dashboard", "src", "components", "WasmBindgenWorkflow.tsx"),
  );
  const dashboardLib = read(
    path.join(root, "examples", "dashboard", "src", "lib", "wasmBindgenDashboard.ts"),
  );
  const packageDoc = read(path.join(root, "docs", "packages", "wasm-bindgen.md"));
  const dashboardReadme = read(path.join(root, "examples", "dashboard", "README.md"));

  for (const marker of [
    "WebAssembly.instantiateStreaming",
    "module.headers.get('Content-Type') !== 'application/wasm'",
    "typeof module_or_path === 'string'",
    "module_or_path instanceof Request",
    "module_or_path instanceof URL",
    "fetch(module_or_path)",
    "__wbindgen_init_externref_table",
  ]) {
    assert.match(upstreamWeb, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing upstream web marker ${marker}`);
  }

  for (const marker of [
    "cachedUint8ArrayMemory0",
    "getUint8ArrayMemory0",
    "new Uint8Array(wasm.memory.buffer)",
    "getStringFromWasm0",
    "__wbg___wbindgen_throw",
    "throw new Error(getStringFromWasm0",
    "passStringToWasm0",
    "WASM_VECTOR_LEN",
    "TextEncoder",
    "TextDecoder('utf-8', { ignoreBOM: true, fatal: true })",
  ]) {
    assert.match(upstreamMemory, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing upstream memory marker ${marker}`);
  }

  for (const marker of [
    "cachedDataViewMemory0",
    "getDataViewMemory0",
    "new DataView(wasm.memory.buffer)",
  ]) {
    assert.match(upstreamDataView, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing upstream DataView marker ${marker}`);
  }

  for (const marker of [
    "FinalizationRegistry",
    "__wbindgen_destroy_closure",
    "CLOSURE_DTORS.register",
    "CLOSURE_DTORS.unregister",
    "__externref_table_alloc",
    "__externref_table_dealloc",
    "__wbindgen_exn_store",
  ]) {
    assert.match(upstreamClosures, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing upstream closure marker ${marker}`);
  }

  for (const marker of [
    "export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module",
    "export type SyncInitInput = BufferSource | WebAssembly.Module",
    "readonly __wbindgen_externrefs: WebAssembly.Table",
    "readonly __wbindgen_thread_destroy",
    "readonly __wbindgen_start",
  ]) {
    assert.match(upstreamTypes, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing upstream type marker ${marker}`);
  }

  for (const marker of [
    "readonly __wbindgen_malloc",
    "readonly __wbindgen_realloc",
    "readonly __wbindgen_free",
  ]) {
    assert.match(upstreamAllocatorTypes, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing upstream allocator type marker ${marker}`);
  }

  for (const marker of [
    '("js/wasm/bindgen/loader.ts", WASM_BINDGEN_LOADER_TS)',
    '("js/wasm/bindgen/react.tsx", WASM_BINDGEN_REACT_TSX)',
    '"js/wasm/bindgen/dashboard-workflow.tsx"',
    "WASM_BINDGEN_DASHBOARD_WORKFLOW_TSX",
    "WasmBindgenDashboardWorkflow",
    "dx-dashboard-wasm-bindgen",
    'officialName: "WebAssembly Bridge"',
    'upstreamPackage: "wasm-bindgen"',
    'upstreamVersion: "0.2.121"',
    "dxCheckVisibility",
    'honestyLabel: "ADAPTER-BOUNDARY"',
    "export type WasmBindgenBytes = ArrayBuffer | ArrayBufferView",
    "loadWasmBindgenModule",
    "reloadWasmBindgenModuleSync",
    "resetWasmBindgenModuleState",
    "getWasmBindgenModuleMemory",
    "getWasmBindgenUint8Memory",
    "getWasmBindgenDataViewMemory",
    "getWasmBindgenMemoryViews",
    "WasmBindgenMemoryViews",
    "encodeWasmBindgenString",
    "decodeWasmBindgenString",
    "throwWasmBindgenError",
    "WasmBindgenEncodedString",
    "WasmBindgenAllocatorModule",
    "WasmBindgenAllocation",
    "WasmBindgenStringAllocation",
    "allocateWasmBindgenBytes",
    "allocateWasmBindgenString",
    "reallocateWasmBindgenAllocation",
    "freeWasmBindgenAllocation",
    "assertWasmBindgenMalloc",
    "assertWasmBindgenRealloc",
    "assertWasmBindgenFree",
    "assertWasmBindgenAlignment",
    "assertWasmBindgenAllocationSize",
    "assertWasmBindgenMemoryRange",
    "memoryBytes",
    "source-owned Uint8Array memory view",
    "source-owned DataView memory view",
    "source-owned UTF-8 string encoding",
    "source-owned UTF-8 string decoding",
    "source-owned UTF-8 string allocation",
    "source-owned throw bridge",
    "validated memory string ranges",
    "source-owned byte allocation",
    "source-owned byte reallocation",
    "source-owned allocation free helper",
    "validated closure state cleanup",
    "validated closure-destroy export access",
    "allocator export diagnostics",
    "getWasmBindgenExternrefTable",
    "initializeWasmBindgenExternrefTable",
    "WasmBindgenExceptionModule",
    "allocateWasmBindgenExternref",
    "deallocateWasmBindgenExternref",
    "storeWasmBindgenException",
    "assertWasmBindgenExternrefIndex",
    "assertWasmBindgenExternrefAllocator",
    "assertWasmBindgenExternrefDeallocator",
    "assertWasmBindgenExceptionStore",
    "source-owned externref allocation",
    "source-owned externref deallocation",
    "source-owned exception storage bridge",
    "exception bridge diagnostics",
    "getWasmBindgenStartFunction",
    "destroyWasmBindgenThread",
    "destroyWasmBindgenClosure",
    "WasmBindgenClosureModule",
    "WasmBindgenClosureState",
    "assertWasmBindgenClosureState",
    "inspectWasmBindgenModule",
    "WasmBindgenModuleDiagnostics",
    "externrefTableLength",
    "source-mirrored externref table seeding",
    "inspectWasmBindgenResponse",
    "formatWasmBindgenResponseDiagnostics",
    "assertWasmBindgenResponse",
    "assertWasmBindgenResponseDiagnostics",
    "validated Response diagnostics input",
    "validated Response diagnostics formatter input",
    "formatted Response diagnostics",
    "hasWasmBindgenFetchInput",
    "useWasmBindgenModule",
    "derived hook start function",
    "derived hook diagnostics",
    'officialName: "WebAssembly Bridge"',
    'upstreamPackage: "wasm-bindgen"',
    'upstreamVersion: "0.2.121"',
    '"webassembly-bridge"',
    '"webassembly/bridge"',
    '"dx-forge/wasm-bindgen"',
    'sourceMirror: "G:\\\\WWW\\\\inspirations\\\\wasm-bindgen"',
    "crates/cli/tests/reference/targets-target-web.js",
    "crates/cli/tests/reference/wasm-export-types.d.ts",
    "exportedFiles",
    "requiredEnv: []",
    'dashboardWorkflow: "wasm/bindgen/dashboard-workflow.tsx"',
    "appOwnedBoundaries",
    "Rust crate source and #[wasm_bindgen] export design",
    "receiptPaths",
    ".dx/forge/receipts/wasm-bindgen.json",
    "dashboardUsage",
    "dashboardWorkflow:",
    "examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json",
    'sourceFile: "examples/dashboard/src/components/WasmBindgenWorkflow.tsx"',
    'launchSourceFile: "examples/template/wasm-interop-status.tsx"',
    'launchDashboardComponent: "launch-wasm-compute-dashboard-workflow"',
    'previewManifestSurface: "launch-runtime-wasm-compute-dashboard"',
    'productSurface: "launch-dashboard"',
    'data-dx-dashboard-card="local-compute"',
    'data-dx-dashboard-workflow="local-compute-readiness"',
    "local WebAssembly add(a, b) readiness check plus app-owned generated-module readiness",
    "wasm-compute-dashboard-workflow",
    "icons",
    "pack:wasm-bindgen",
  ]) {
    assert.match(slice, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing slice marker ${marker}`);
  }

  for (const marker of [
    '"wasm/bindgen"',
    '"webassembly-bridge"',
    '"webassembly/bridge"',
    '"wasm-bindgen"',
    '"dx-forge/wasm-bindgen"',
    "wasm-bindgen@0.2.121 local source mirror",
    'paths.contains(&"wasm/bindgen/dashboard-workflow.tsx")',
    "assert_eq!(paths.len(), 6)",
  ]) {
    assert.match(registry, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing registry marker ${marker}`);
  }

  for (const marker of [
    'packageId: "wasm/bindgen"',
    'officialName: "WebAssembly Bridge"',
    '"webassembly-bridge"',
    '"webassembly/bridge"',
    '"dx-forge/wasm-bindgen"',
    'upstreamPackage: "wasm-bindgen"',
    'upstreamVersion: "0.2.121"',
    "requiredEnv: []",
    'command: "dx add webassembly-bridge --write"',
    '"wasm/bindgen/dashboard-workflow.tsx"',
    '"examples/dashboard/src/components/WasmBindgenWorkflow.tsx"',
    '"examples/template/wasm-interop-status.tsx"',
    '"examples/template/dx-studio-edit-contract.ts#wasm-compute-dashboard-workflow"',
    '"tools/launch/materialize-www-template.ts#launch-runtime-wasm-compute-dashboard"',
    "G:/WWW/inspirations/wasm-bindgen README",
    '".dx/forge/receipts/wasm-bindgen.json"',
    '"examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json"',
    'sourceMirror: "G:/WWW/inspirations/wasm-bindgen"',
    'inspectedSourceFiles: [',
    '"Cargo.toml"',
    '"src/lib.rs"',
    '"crates/cli/tests/reference/targets-target-web-atomics.d.ts"',
    'selectedSurfaces: [',
    '"generated-module-loader"',
    '"launch-local-compute-dashboard"',
    'dxCheckVisibility: {',
    'currentStatus: "present"',
    '"unsupported-surface"',
    'honestyLabel: "ADAPTER-BOUNDARY"',
    'component: "LaunchWasmInteropStatus"',
    'previewManifestSurface: "launch-runtime-wasm-compute-dashboard"',
    'data-dx-dashboard-metric="wasm-compute"',
    'data-dx-wasm-action="run-local-add"',
    'dxIcon: "pack:wasm-bindgen"',
    "WasmBindgenDashboardWorkflow",
    "WebAssembly Bridge",
    '<dx-icon name=\\"pack:wasm-bindgen\\" />',
  ]) {
    assert.match(packageCatalog, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing package catalog marker ${marker}`);
  }

  for (const marker of [
    '"package_id": "wasm/bindgen"',
    '"official_name": "WebAssembly Bridge"',
    '"upstream_package": "wasm-bindgen"',
    '"upstream_version": "0.2.121"',
    '"command": "dx add webassembly-bridge --write"',
    '"source_mirror": "G:/WWW/inspirations/wasm-bindgen"',
    '"dx_check_visibility"',
    '"WebAssembly Bridge"',
    "dx add webassembly-bridge",
  ]) {
    assert.match(cliMod, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing CLI marker ${marker}`);
  }

  for (const marker of [
    '"package": "wasm/bindgen"',
    '"official_name": "WebAssembly Bridge"',
    '"upstream_package": "wasm-bindgen"',
    '"source_file": "examples/template/wasm-interop-status.tsx"',
  ]) {
    assert.match(studioManifest, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing Studio manifest marker ${marker}`);
  }

  for (const marker of [
    'data-dx-component="launch-wasm-compute-dashboard-workflow"',
    'data-dx-dashboard-workflow="local-compute-readiness"',
    'data-dx-dashboard-card="local-compute"',
    'data-dx-product-surface="launch-dashboard"',
    "WASM local compute dashboard workflow",
    "Open the account data dashboard above to run local compute",
    "<LaunchWasmInteropStatus />",
  ]) {
    assert.match(launchShell, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing launch dashboard marker ${marker}`);
  }

  for (const marker of [
    "@/wasm/bindgen/loader",
    "@/wasm/bindgen/react",
    "inspectWasmBindgenResponse",
    "formatWasmBindgenResponseDiagnostics",
    "useWasmBindgenModule",
    "WasmBindgenFactory",
    "dx-launch-wasm-status",
    "wasmResponse?: Response",
    "responseDiagnostics",
    "Wasm MIME: {formatWasmBindgenResponseDiagnostics(responseDiagnostics)}",
    "data-dx-wasm-bindgen-status",
    'data-dx-component="wasm-bindgen-readiness-workflow"',
    'data-dx-package="wasm/bindgen"',
    'data-dx-wasm-interaction="missing-module-state"',
    'data-dx-wasm-interaction="local-add-readiness"',
    'data-dx-wasm-action="run-local-add"',
    'data-dx-wasm-add-result={addResult ?? "idle"}',
    'name: "pack:wasm-bindgen"',
    '"data-dx-icon": "pack:wasm-bindgen"',
    "importLocalReadinessWasmBindgenModule",
    "WebAssembly.instantiate(localAddWasmBytes)",
    "No app-owned wasm-bindgen module is configured yet",
    "diagnostics?.memoryPages",
    "diagnostics?.memoryBytes",
    "UTF-8 encode/decode/allocation/throw helpers ready",
    "status.start",
    "status.externrefs",
    "status.diagnostics",
    "diagnostics?.externrefTableLength",
    "diagnostics?.canStoreException",
    "externref exception store available",
    "canResetState",
    "canDestroyThread",
    "canDestroyClosure",
    "canAllocateBytes",
    "canReallocateBytes",
    "canFreeBytes",
    "byte allocation available",
    "closure cleanup",
    "managed by generated init",
  ]) {
    assert.match(launchProof, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing launch template marker ${marker}`);
  }

  for (const marker of [
    'data-dx-component="wasm-bindgen-readiness-workflow"',
    'data-dx-dashboard-workflow="local-compute-readiness"',
    'data-dx-package="wasm/bindgen"',
    'data-dx-product-surface="launch-dashboard"',
    'data-dx-wasm-bindgen-status="missing-app-module"',
    'data-dx-wasm-interaction="missing-module-state"',
    'data-dx-wasm-interaction="local-add-readiness"',
    'data-dx-wasm-action="run-local-add"',
    'data-dx-wasm-add-result="idle"',
    'data-dx-wasm-mime-status="application/wasm"',
  ]) {
    assert.match(runtimeLaunch, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing live runtime marker ${marker}`);
  }

  for (const marker of [
    "function bindWasm()",
    "WebAssembly.instantiate(localAddWasmBytes)",
    "dxWasmBindgenStatus",
    "dxWasmAddResult",
    "dxDashboardWasmRuns",
    "mission-wasm-status",
    "WASM local compute updated the launch dashboard.",
    "bindWasm()",
  ]) {
    assert.match(runtimeJs, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing live runtime script marker ${marker}`);
  }

  for (const marker of [
    '"launch-runtime-wasm-compute-dashboard"',
    "'[data-dx-component=\"launch-wasm-compute-dashboard-workflow\"]'",
    '"wasm/bindgen"',
    '"insert_icon_media"',
    '".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json"',
    '"data-dx-wasm-add-result"',
  ]) {
    assert.match(runtimeMaterializer, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing runtime materializer marker ${marker}`);
  }

  for (const marker of [
    'id: "wasm-compute-dashboard-workflow"',
    'selector: \'[data-dx-component="launch-wasm-compute-dashboard-workflow"]\'',
    'packageIds: ["wasm/bindgen"]',
    '"move_reorder_section"',
    '"update_text_content"',
    '"insert_icon_media"',
    'layoutPolicy: "responsive-design-system-grid"',
    "absolutePositioning: false",
    "noNodeModulesRequired: true",
  ]) {
    assert.match(editContract, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing edit contract marker ${marker}`);
  }

  for (const marker of [
    '"schema": "dx.forge.package_dashboard_workflow_receipt"',
    '"package_id": "wasm/bindgen"',
    '"package_name": "WebAssembly Bridge"',
    '"upstream_package": "wasm-bindgen"',
    '"upstream_version": "0.2.121"',
    '"honesty_label": "ADAPTER-BOUNDARY"',
    '"selected_surfaces"',
    '"dx_check_visibility"',
    '"component": "launch-wasm-compute-dashboard-workflow"',
    '"workflow": "local-compute-readiness"',
    '"status": "source-coded-runtime-pending"',
    '"coding_score": 97',
    '"examples/template/wasm-interop-status.tsx"',
    '"data-dx-dashboard-metric=\\"wasm-compute\\""',
    '"data-dx-wasm-action=\\"run-local-add\\""',
    '"tools/launch/materialize-www-template.ts"',
    '"public/preview-.dx/build-cache/manifest.json#launch-runtime-wasm-compute-dashboard"',
    '"zed_preview_surface": "launch-runtime-wasm-compute-dashboard"',
    '"zed_preview_selector": "[data-dx-component=\\"launch-wasm-compute-dashboard-workflow\\"]"',
    '"required_env": []',
    '"app-owned Rust crate module import"',
    '"dx run --test .\\\\benchmarks\\\\wasm-bindgen-slice.test.ts"',
    '"no_runtime_execution": true',
  ]) {
    assert.match(dashboardReceipt, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing dashboard receipt marker ${marker}`);
  }

  for (const marker of [
    "wasmBindgenLocalComputeDashboard",
    'packageId: "wasm/bindgen"',
    'file: "examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json"',
    'component: "launch-wasm-compute-dashboard-workflow"',
    'dashboardWorkflow: "local-compute-readiness"',
    'previewManifestSurface: "launch-runtime-wasm-compute-dashboard"',
    'materializedReceiptFile:',
    '".dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json"',
    '"data-dx-dashboard-metric"',
    '"data-dx-wasm-action"',
    "dx run --test .\\\\benchmarks\\\\wasm-bindgen-slice.test.ts",
  ]) {
    assert.match(routeContract, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing route contract marker ${marker}`);
  }

  for (const marker of [
    "import { WasmBindgenWorkflow } from '../components/WasmBindgenWorkflow'",
    "<WasmBindgenWorkflow />",
  ]) {
    assert.match(dashboardPage, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing starter dashboard marker ${marker}`);
  }

  for (const marker of [
    'data-dx-package="wasm/bindgen"',
    'data-dx-component="dashboard-wasm-bindgen-workflow"',
    'data-dx-dashboard-workflow="wasm-interop"',
    'data-dx-wasm-dashboard-action="run-local-add"',
    'data-dx-style-surface="theme-token-card"',
    'data-dx-icon-search="wasm:bindgen"',
    '<dx-icon name="pack:wasm-bindgen"',
    "createWasmBindgenDashboardReceipt",
    "data-dx-wasm-dashboard-receipt",
    "data-dx-wasm-add-result",
  ]) {
    assert.match(dashboardComponent, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing dashboard workflow marker ${marker}`);
  }

  for (const marker of [
    "packageId: 'wasm/bindgen'",
    "officialName: 'WebAssembly Bridge'",
    "upstreamPackage: 'wasm-bindgen'",
    "upstreamVersion: '0.2.121'",
    "sourceMirror: 'G:/WWW/inspirations/wasm-bindgen'",
    "upstreamReference: 'wasm-bindgen@0.2.121 local source mirror'",
    "'crates/cli/tests/reference/targets-target-web.js'",
    "'wasm/bindgen/dashboard-workflow.tsx'",
    "'examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json'",
    "satisfies WasmBindgenDashboardPackage",
    "dxCheckVisibility",
    "WebAssembly.instantiate(localAddWasmBytes)",
    "status: 'missing-generated-module'",
    "status: 'local-readiness-ready'",
    "component: 'launch-wasm-compute-dashboard-workflow'",
    "dashboardCard: 'local-compute'",
    "previewManifestSurface: 'launch-runtime-wasm-compute-dashboard'",
    "Rust crate exports marked with #[wasm_bindgen]",
  ]) {
    assert.match(dashboardLib, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing dashboard lib marker ${marker}`);
  }

  assert.doesNotMatch(dashboardComponent, /#[0-9a-fA-F]{3,8}/, "dashboard workflow must not hardcode hex colors");
  assert.doesNotMatch(
    dashboardComponent,
    /\b(?:text|bg|border)-(?:slate|gray|zinc|neutral|stone|red|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose)-\d{2,3}\b/,
    "dashboard workflow must use dashboard/theme tokens instead of hardcoded Tailwind color classes",
  );

  for (const marker of [
    "### WebAssembly Bridge workflow",
    "`examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json`",
    "`WebAssembly.instantiate()` add action",
    "<dx-icon name=\"pack:wasm-bindgen\" />",
  ]) {
    assert.match(dashboardReadme, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing starter dashboard README marker ${marker}`);
  }

  for (const marker of [
    "# WebAssembly Bridge Forge Package",
    "Official DX package name: `WebAssembly Bridge`",
    "Package id: `wasm/bindgen`",
    "Upstream package: `wasm-bindgen`",
    "Upstream version: `0.2.121`",
    "Honesty label: `ADAPTER-BOUNDARY`",
    "Source mirror: `G:\\WWW\\inspirations\\wasm-bindgen`",
    "dx-check visibility: `present`, `stale`, `missing-receipt`, `blocked`, `unsupported-surface`",
    "`wasm/bindgen/dashboard-workflow.tsx`",
    "`examples/dashboard/src/components/WasmBindgenWorkflow.tsx`",
    '`data-dx-component="launch-wasm-compute-dashboard-workflow"`',
    "Surface id: `wasm-compute-dashboard-workflow`",
    '`data-dx-dashboard-metric="wasm-compute"`',
    "Discovery owners: package catalog, generated package metadata, launch route contract, dashboard receipt",
    "Generated starter receipt path: `.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json`",
    "`examples/template/.dx/forge/receipts/2026-05-22-wasm-bindgen-dashboard-workflow.json`",
    '`data-dx-component="dashboard-wasm-bindgen-workflow"`',
    "`WebAssembly.instantiate(localAddWasmBytes)`",
    "`benchmarks/wasm-bindgen-slice.test.ts`",
  ]) {
    assert.match(packageDoc, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")), `missing package doc marker ${marker}`);
  }
});
