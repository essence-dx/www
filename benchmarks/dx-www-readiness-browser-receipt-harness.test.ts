import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");
const harnessPath = path.join(root, "benchmarks", "dx-www-readiness-browser-receipt-harness.ts");

function read(relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function nativeEventNames(): string[] {
  const source = read("core/src/delivery/dom_events.rs");
  const match = source.match(/const NATIVE_DOM_EVENT_NAMES:\s*&\[&str\]\s*=\s*&\[([\s\S]*?)\];/);
  assert.ok(match, "native event catalog source should be parseable");
  return Array.from(match[1].matchAll(/"([^"]+)"/g), (item) => item[1]);
}

function nativeEventCatalogReceipt(): { count: number; hash: string } {
  const receipt = JSON.parse(read(".dx/receipts/readiness/native-events-latest.json"));
  assert.equal(typeof receipt.catalog_count, "number");
  assert.match(receipt.catalog_hash, /^blake3:[a-f0-9]{64}$/);
  return {
    count: receipt.catalog_count,
    hash: receipt.catalog_hash,
  };
}

function noJsArtifactReceipt(): { htmlPath: string; htmlBlake3: string } {
  const receipt = JSON.parse(read(".dx/receipts/readiness/no-js-artifact-latest.json"));
  assert.equal(typeof receipt.html_path, "string");
  assert.match(receipt.artifact_html_blake3, /^blake3:[a-f0-9]{64}$/);
  return {
    htmlPath: receipt.html_path,
    htmlBlake3: receipt.artifact_html_blake3,
  };
}

test("release-readiness browser receipt harness is TypeScript and converts only real page snapshots", () => {
  const harness = read("benchmarks/dx-www-readiness-browser-receipt-harness.ts");
  const readiness = read("dx-www/src/cli/readiness.rs");
  const binder = read("dx-www/src/cli/app_router_execution/source_render_parts/client_component.rs");

  for (const marker of [
    "collectReadinessBrowserPageSnapshot",
    "collectReadinessBrowserDomSnapshot",
    "--print-dom-page-collector",
    "read-only-dom-after-browser-interactions",
    "window.__DX_DOM_ACTION_BINDER__",
    "window.__DX_STATE_GRAPH_RUNTIME__",
    "window.__DX_READINESS_VISUAL_EDIT_REPLAY__",
    "window.__DX_DEVTOOLS_READINESS_VISUAL_EDIT_REPLAY__",
    "visual_edit_replay_attempt",
    "dx:dom-action-preview",
    "dx:state-runtime-dispatch",
    "dx:state-dom-reflection",
    "dx:derived-state-slot",
    "dx:effect-scheduled",
    "dx:state-event",
    "local-in-app-browser-native-event-binder-replay",
    "local-in-app-browser-state-runtime-replay",
    "local-in-app-browser-source-owned-island-replay",
    "local-in-app-browser-visual-edit-workbench-replay",
    "local-js-disabled-browser-no-js-route-replay",
    "island-browser-latest.json",
    "islandBrowserReceiptFromPageSnapshot",
    "REQUIRED_ISLAND_DIRECTIVES",
    "dx:client-island-event",
    "data-dx-client-island-bridge",
    "dxIslandDirectives",
    "visual-edit-browser-workbench-latest.json",
    "no-js-browser-latest.json",
    "browser_snapshot_hash",
    "missing_listener_events",
    "missing_contract_events",
    "missing_replay_events",
    "missing_api_methods",
    "missing_workbench_phases",
    "browser_event_constructors",
    "browser_event_replay_results",
    "catalog_contract_current",
    "expected_catalog_hash",
    "native-events-catalog-receipt-missing",
    "NO_JS_ARTIFACT_RECEIPT_PATH",
    "noJsBrowserReceiptFromPageSnapshot",
    "window.__DX_READINESS_BROWSER_FLAGS__",
    "javascriptDisabledBrowser",
    "artifactHtmlBlake3",
    "dx www readiness --import-no-js-browser-receipt",
    "dx www readiness --import-island-browser-receipt",
    ".filter((item) => item.previewed === true)",
    "release_ready: false",
    "fastest_world_claim: false",
    "dx www readiness validates them before canonical writes",
  ]) {
    assert.match(harness, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(binder, /catalog_hash: __DX_SUPPORTED_DOM_EVENT_CATALOG_HASH__/);
  assert.match(binder, /supported_event_count: supportedEvents\.length/);
  assert.match(readiness, /dx-www-readiness-browser-receipt-harness\.test\.ts/);
  assert.match(readiness, /dx-www-readiness-browser-receipt-harness\.ts --print-page-collector/);
  assert.match(readiness, /dx-www-readiness-browser-receipt-harness\.ts --print-dom-page-collector/);
  assert.match(readiness, /--from-page-json <page-snapshot\.json> --out-dir \.dx\/receipts\/readiness\/browser-import-candidates/);
  assert.match(readiness, /"page_snapshot_capture_command": READINESS_BROWSER_PAGE_COLLECT_COMMAND/);
  assert.match(readiness, /"dom_snapshot_capture_command": READINESS_BROWSER_DOM_COLLECT_COMMAND/);
  assert.match(readiness, /"snapshot_capture_modes": \["full-replay-page-collector", "read-only-dom-after-browser-interactions"\]/);
});

test("release-readiness browser receipt collector invokes source-owned Devtools replay before snapshotting", () => {
  const collector = execFileSync(
    process.execPath,
    [harnessPath, "--print-page-collector"],
    { cwd: root, encoding: "utf8" },
  );

  assert.match(collector, /async function collectReadinessBrowserPageSnapshot/);
  assert.match(collector, /window\.__DX_DEVTOOLS_READINESS_VISUAL_EDIT_REPLAY__/);
  assert.match(collector, /await visualReplayRunner\(\)/);
  assert.match(collector, /new MouseEvent/);
  assert.match(collector, /new PointerEvent/);
  assert.match(collector, /new InputEvent/);
  assert.match(collector, /browser_event_constructors/);
  assert.match(collector, /browser_event_replay_results/);
  assert.match(collector, /data-dx-client-island-bridge/);
  assert.match(collector, /dxIslandDirectives/);
  assert.match(collector, /dx:client-island-event/);
  assert.match(collector, /islandEventReplayResults/);
  assert.match(collector, /visual_edit: window\.__DX_READINESS_VISUAL_EDIT_REPLAY__ \|\| null/);
  assert.match(collector, /visual_edit_replay_attempt/);
  assert.match(collector, /no_js:/);
  assert.match(collector, /javascriptDisabledBrowser/);
  assert.match(collector, /document\.scripts\.length/);
  assert.match(collector, /data_dx_output_mode_tiny_static/);
  assert.match(collector, /missing-entrypoint/);
  assert.match(collector, /source-owned-devtools-replay-completed/);
  assert.match(collector, /source-owned-devtools-replay-did-not-become-current/);
});

test("release-readiness browser receipt DOM collector can read browser island proof without unsafe page mutation", () => {
  const collector = execFileSync(
    process.execPath,
    [harnessPath, "--print-dom-page-collector"],
    { cwd: root, encoding: "utf8" },
  );

  assert.match(collector, /async function collectReadinessBrowserDomSnapshot/);
  assert.match(collector, /read-only-dom-after-browser-interactions/);
  assert.match(collector, /data-dx-client-island-bridge/);
  assert.match(collector, /data-dx-client-island-event-log/);
  assert.match(collector, /eventReplayResults/);
  assert.match(collector, /browser_runtime_executed: eventReplayResults\.length > 0/);
  assert.doesNotMatch(collector, /dispatchEvent\(createBrowserEvent/);
  assert.doesNotMatch(collector, /window\.addEventListener/);
});

test("release-readiness browser receipt harness writes honest import candidates from a page snapshot", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-www-readiness-browser-"));
  const snapshotPath = path.join(tempDir, "page-snapshot.json");
  const outDir = path.join(tempDir, "receipts");
  const catalogEvents = nativeEventNames();
  const catalogReceipt = nativeEventCatalogReceipt();
  const noJsArtifact = noJsArtifactReceipt();
  const catalogReceiptMatchesSource = catalogReceipt.count === catalogEvents.length;
  const snapshot = {
    schema: "dx.www.readiness.browser_receipt_page_snapshot.v1",
    schema_revision: 1,
    browser_runtime_executed: true,
    url: "http://127.0.0.1:3000/state-runtime",
    canonical_route: "/state-runtime",
    title: "DX WWW",
    user_agent: "Mozilla/5.0 release-readinessHarness",
    viewport: {
      width: 1280,
      height: 720,
      device_pixel_ratio: 1,
    },
    binder: {
      present: true,
      contract: {
        supported_events: catalogEvents,
        supported_event_count: catalogEvents.length,
        catalog_hash: catalogReceipt.hash,
      },
      required_events: ["click", "pointermove", "input"],
      listener_events: ["click", "input", "pointermove"],
      browser_event_constructors: {
        click: "MouseEvent",
        input: "InputEvent",
        pointermove: "PointerEvent",
      },
      browser_event_replay_results: [
        { event: "click", tag: "button", previewed: true },
        { event: "pointermove", tag: "button", previewed: true },
        { event: "input", tag: "input", previewed: true },
      ],
      preview_event_count: 3,
      state_dispatch_count: 3,
      unsupported_listener_attached: false,
    },
    state_runtime: {
      present: true,
      route: "/state-runtime",
      api_methods: [
        "getSnapshot",
        "setSlot",
        "dispatch",
        "refreshDerivedSlots",
        "scheduleEffectsForState",
      ],
      full_react_hook_runtime: false,
      react_api_shim_executed: false,
      state_reflection_event_count: 3,
      derived_reflection_event_count: 2,
      effect_scheduled_event_count: 2,
      action_dispatch_count: 3,
      slot_count: 3,
      event_count: 3,
    },
    islands: {
      bridge_present: true,
      source_owned_bridge: true,
      bridge_abi_style: "camelCase",
      abi_schema: "dx.react.clientIsland.abi",
      directive_style: "camelCase-jsx-props",
      no_js_fallback_preserved: true,
      island_count: 4,
      source_owned_island_count: 4,
      directives_seen: ["clientIdle", "clientLoad", "clientOnly", "clientVisible"],
      hydration_strategies: ["idle", "load", "only", "visible"],
      event_node_count: 1,
      client_island_event_count: 1,
      event_replay_results: [{ event_id: "island-event-1", event: "click", replayed: true }],
      client_only_adapter_values: ["none", "react"],
      provider_adapter_values: ["not-executed"],
      browser_runtime_proof_values: ["not-claimed"],
      full_react_hydration: false,
      node_modules_required: false,
      react_synthetic_events: false,
      provider_adapter_executed: false,
    },
    visual_edit: {
      devtools_global_present: true,
      browser_workbench_replay: "current",
      workbench_phases: ["inspect", "cascade", "preview", "apply", "undo", "receipt"],
      inspected_element_present: true,
      cascade_inspected: true,
      preview_source_mutated: false,
      apply_source_mutated: true,
      undo_source_restored: true,
      safe_local_source_target_known: true,
      apply_receipt_written: true,
      undo_receipt_written: true,
      receipt_durability: "json-sr-machine-written",
      inspected_selector: "[data-dx-component=\"state-runtime-probe\"]",
      inspected_element_fingerprint: "state-runtime-probe:section.state-runtime-probe",
      style_property: "--ring",
      style_value: "0 0% 84%",
      computed_style_before: {
        property: "--ring",
        value: "0 0% 83%",
      },
      computed_style_after_preview: {
        property: "--ring",
        value: "0 0% 84%",
      },
      computed_style_after_undo: {
        property: "--ring",
        value: "0 0% 83%",
      },
      source_target: {
        relativePath: "examples/template/styles/theme.css",
        kind: "css-custom-property",
        range: {
          startByte: 381,
          endByte: 400,
          expectedText: "  --ring: 0 0% 83%;",
        },
      },
    },
    visual_edit_replay_attempt: {
      attempted: true,
      status: "current",
      ok: true,
      reason: "source-owned-devtools-replay-completed",
    },
    no_js: {
      route: "/",
      live_browser_executed: true,
      javascript_disabled_browser: true,
      page_javascript_enabled: false,
      html_path: noJsArtifact.htmlPath,
      script_tag_count: 0,
      data_dx_output_mode_tiny_static: true,
      data_dx_js_none: true,
      semantic_landmark_present: true,
      visible_text_present: true,
      link_count: 1,
      form_count: 1,
      seo_title_present: true,
      accessibility_signal_count: 5,
    },
  };
  fs.writeFileSync(snapshotPath, `${JSON.stringify(snapshot, null, 2)}\n`);

  const output = execFileSync(
    process.execPath,
    [harnessPath, "--from-page-json", snapshotPath, "--out-dir", outDir],
    { cwd: root, encoding: "utf8" },
  );
  const report = JSON.parse(output);
  assert.equal(report.release_ready, false);
  assert.equal(report.fastest_world_claim, false);
  assert.equal(report.receipts[0].passed, catalogReceiptMatchesSource);
  assert.equal(report.receipts[1].passed, true);
  assert.equal(report.receipts[2].passed, true);
  assert.equal(report.receipts[3].passed, true);
  assert.equal(report.receipts[4].passed, true);
  assert.equal(report.receipts[0].canonical_route, "/state-runtime");
  assert.equal(report.receipts[0].proof_target.kind, "native-event-binder");
  assert.equal(report.receipts[1].canonical_route, "/state-runtime");
  assert.equal(report.receipts[1].proof_target.kind, "state-runtime");
  assert.equal(report.receipts[2].canonical_route, "/state-runtime");
  assert.equal(report.receipts[2].proof_target.kind, "source-owned-client-islands");
  assert.equal(report.receipts[3].canonical_route, "/state-runtime");
  assert.equal(report.receipts[3].proof_target.kind, "visual-edit-workbench");
  assert.equal(report.receipts[4].canonical_route, "/");
  assert.equal(report.receipts[4].proof_target.kind, "tiny-static-no-js-route");

  const nativeReceipt = JSON.parse(
    fs.readFileSync(path.join(outDir, "native-event-browser-binder-latest.json"), "utf8"),
  );
  const stateReceipt = JSON.parse(
    fs.readFileSync(path.join(outDir, "state-runtime-browser-latest.json"), "utf8"),
  );
  const visualReceipt = JSON.parse(
    fs.readFileSync(path.join(outDir, "visual-edit-browser-workbench-latest.json"), "utf8"),
  );
  const islandReceipt = JSON.parse(
    fs.readFileSync(path.join(outDir, "island-browser-latest.json"), "utf8"),
  );
  const noJsBrowserReceipt = JSON.parse(
    fs.readFileSync(path.join(outDir, "no-js-browser-latest.json"), "utf8"),
  );
  assert.equal(nativeReceipt.schema, "dx.www.readiness.native_event_browser_binder_receipt_contract");
  assert.equal(nativeReceipt.browser_runtime_executed, true);
  assert.equal(nativeReceipt.canonical_route, "/state-runtime");
  assert.equal(nativeReceipt.proof_target.route, "/state-runtime");
  assert.equal(nativeReceipt.proof_target.global, "__DX_DOM_ACTION_BINDER__");
  assert.equal(nativeReceipt.hosted_provider_proof, false);
  assert.equal(nativeReceipt.provider_adapter_executed, false);
  assert.equal(nativeReceipt.unsupported_listener_attached, false);
  assert.equal(nativeReceipt.react_synthetic_events, false);
  assert.equal(nativeReceipt.full_react_event_parity, false);
  assert.equal(nativeReceipt.supported_event_count, catalogEvents.length);
  assert.equal(nativeReceipt.catalog_hash, catalogReceipt.hash);
  assert.equal(nativeReceipt.expected_catalog_count, catalogReceipt.count);
  assert.equal(nativeReceipt.catalog_contract_current, catalogReceiptMatchesSource);
  assert.deepEqual(nativeReceipt.listener_events, ["click", "input", "pointermove"]);
  assert.deepEqual(nativeReceipt.missing_listener_events, []);
  assert.deepEqual(nativeReceipt.missing_contract_events, []);
  assert.deepEqual(nativeReceipt.missing_replay_events, []);
  assert.equal(nativeReceipt.browser_event_constructors.pointermove, "PointerEvent");
  assert.match(nativeReceipt.browser_snapshot_hash, /^sha256:[a-f0-9]{64}$/);
  assert.equal(stateReceipt.schema, "dx.www.readiness.state_runtime_browser_receipt_contract");
  assert.equal(stateReceipt.route, "/state-runtime");
  assert.equal(stateReceipt.canonical_route, "/state-runtime");
  assert.equal(stateReceipt.proof_target.route, "/state-runtime");
  assert.equal(stateReceipt.proof_target.global, "__DX_STATE_GRAPH_RUNTIME__");
  assert.equal(stateReceipt.hosted_provider_proof, false);
  assert.equal(stateReceipt.provider_adapter_executed, false);
  assert.equal(stateReceipt.full_react_hook_runtime, false);
  assert.equal(stateReceipt.react_api_shim_executed, false);
  assert.deepEqual(stateReceipt.missing_api_methods, []);
  assert.equal(stateReceipt.slot_count, 3);
  assert.equal(stateReceipt.event_count, 3);
  assert.match(stateReceipt.browser_snapshot_hash, /^sha256:[a-f0-9]{64}$/);
  assert.equal(islandReceipt.schema, "dx.www.readiness.island_browser_receipt_contract");
  assert.equal(islandReceipt.passed, true);
  assert.equal(islandReceipt.status, "source-owned-island-browser-replay-current");
  assert.equal(islandReceipt.browser_runtime_executed, true);
  assert.equal(islandReceipt.canonical_route, "/state-runtime");
  assert.equal(islandReceipt.proof_target.route, "/state-runtime");
  assert.equal(islandReceipt.proof_target.selector, "[data-dx-client-island-bridge]");
  assert.equal(islandReceipt.hosted_provider_proof, false);
  assert.equal(islandReceipt.source_owned_bridge, true);
  assert.equal(islandReceipt.bridge_abi_style, "camelCase");
  assert.equal(islandReceipt.directive_style, "camelCase-jsx-props");
  assert.deepEqual(islandReceipt.missing_core_directives, []);
  assert.equal(islandReceipt.island_count, 4);
  assert.equal(islandReceipt.source_owned_island_count, 4);
  assert.equal(islandReceipt.event_node_count, 1);
  assert.equal(islandReceipt.client_island_event_count, 1);
  assert.equal(islandReceipt.missed_event_replay_count, 0);
  assert.equal(islandReceipt.full_react_hydration, false);
  assert.equal(islandReceipt.node_modules_required, false);
  assert.equal(islandReceipt.react_synthetic_events, false);
  assert.equal(islandReceipt.provider_adapter_executed, false);
  assert.match(islandReceipt.browser_snapshot_hash, /^sha256:[a-f0-9]{64}$/);
  assert.equal(islandReceipt.proof_scope, "local-in-app-browser-source-owned-island-replay");
  assert.equal(visualReceipt.schema, "dx.www.readiness.visual_edit_workbench_receipt_contract");
  assert.equal(visualReceipt.canonical_route, "/state-runtime");
  assert.equal(visualReceipt.proof_target.route, "/state-runtime");
  assert.equal(
    visualReceipt.proof_target.source_target_relative_path,
    "examples/template/styles/theme.css",
  );
  assert.equal(visualReceipt.hosted_provider_proof, false);
  assert.equal(visualReceipt.provider_adapter_executed, false);
  assert.equal(visualReceipt.browser_workbench_replay, "current");
  assert.deepEqual(visualReceipt.missing_workbench_phases, []);
  assert.equal(visualReceipt.visual_replay_attempted, true);
  assert.equal(visualReceipt.visual_replay_status, "current");
  assert.equal(visualReceipt.visual_replay_reason, "source-owned-devtools-replay-completed");
  assert.equal(visualReceipt.receipt_source, "dx.www.readiness.browser_receipt_page_snapshot.v1");
  assert.equal(visualReceipt.preview_source_mutated, false);
  assert.equal(visualReceipt.apply_source_mutated, true);
  assert.equal(visualReceipt.undo_source_restored, true);
  assert.match(visualReceipt.browser_snapshot_hash, /^sha256:[a-f0-9]{64}$/);
  assert.equal(visualReceipt.source_target.relativePath, "examples/template/styles/theme.css");
  assert.equal(noJsBrowserReceipt.schema, "dx.www.readiness.no_js_browser_receipt_contract");
  assert.equal(noJsBrowserReceipt.passed, true);
  assert.equal(noJsBrowserReceipt.status, "current-local-js-disabled-browser-proof");
  assert.equal(noJsBrowserReceipt.route, "/");
  assert.equal(noJsBrowserReceipt.canonical_route, "/");
  assert.equal(noJsBrowserReceipt.proof_target.route, "/");
  assert.equal(noJsBrowserReceipt.proof_target.html_path, noJsArtifact.htmlPath);
  assert.equal(noJsBrowserReceipt.hosted_provider_proof, false);
  assert.equal(noJsBrowserReceipt.provider_adapter_executed, false);
  assert.equal(noJsBrowserReceipt.javascript_disabled_browser, true);
  assert.equal(noJsBrowserReceipt.page_javascript_enabled, false);
  assert.equal(noJsBrowserReceipt.script_tag_count, 0);
  assert.equal(noJsBrowserReceipt.html_path, noJsArtifact.htmlPath);
  assert.equal(noJsBrowserReceipt.artifact_html_blake3, noJsArtifact.htmlBlake3);
  assert.equal(noJsBrowserReceipt.data_dx_output_mode_tiny_static, true);
  assert.equal(noJsBrowserReceipt.data_dx_js_none, true);
  assert.equal(noJsBrowserReceipt.link_count, 1);
  assert.equal(noJsBrowserReceipt.form_count, 1);
  assert.equal(noJsBrowserReceipt.release_ready, false);
  assert.equal(noJsBrowserReceipt.fastest_world_claim, false);
  assert.match(noJsBrowserReceipt.browser_snapshot_hash, /^sha256:[a-f0-9]{64}$/);
  assert.equal(noJsBrowserReceipt.proof_scope, "local-js-disabled-browser-no-js-route-replay");
});

test("release-readiness browser receipt harness rejects event replays that did not preview", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-www-readiness-browser-unpreviewed-"));
  const snapshotPath = path.join(tempDir, "page-snapshot.json");
  const outDir = path.join(tempDir, "receipts");
  const snapshot = {
    schema: "dx.www.readiness.browser_receipt_page_snapshot.v1",
    schema_revision: 1,
    browser_runtime_executed: true,
    url: "http://127.0.0.1:3000/",
    title: "DX WWW",
    user_agent: "Mozilla/5.0 release-readinessHarness",
    viewport: {
      width: 1280,
      height: 720,
      device_pixel_ratio: 1,
    },
    binder: {
      present: true,
      contract: {
        supported_events: ["click", "input", "pointermove"],
        supported_event_count: 3,
        catalog_hash: "blake3:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
      },
      required_events: ["click", "pointermove", "input"],
      listener_events: ["click", "input", "pointermove"],
      browser_event_constructors: {
        click: "MouseEvent",
        input: "InputEvent",
        pointermove: "PointerEvent",
      },
      browser_event_replay_results: [
        { event: "click", tag: "button", previewed: true },
        { event: "pointermove", tag: "button", previewed: false },
        { event: "input", tag: "input", previewed: true },
      ],
      preview_event_count: 3,
      state_dispatch_count: 3,
      unsupported_listener_attached: false,
    },
    state_runtime: {
      present: false,
      api_methods: [],
      full_react_hook_runtime: false,
      react_api_shim_executed: false,
      state_reflection_event_count: 0,
      derived_reflection_event_count: 0,
      effect_scheduled_event_count: 0,
      action_dispatch_count: 0,
    },
    visual_edit: null,
    visual_edit_replay_attempt: {
      attempted: false,
      status: "missing-entrypoint",
      ok: false,
      reason: "window.__DX_DEVTOOLS_READINESS_VISUAL_EDIT_REPLAY__ is not present",
    },
  };
  fs.writeFileSync(snapshotPath, `${JSON.stringify(snapshot, null, 2)}\n`);

  const output = execFileSync(
    process.execPath,
    [harnessPath, "--from-page-json", snapshotPath, "--out-dir", outDir],
    { cwd: root, encoding: "utf8" },
  );
  const report = JSON.parse(output);
  assert.equal(report.receipts[0].passed, false);

  const nativeReceipt = JSON.parse(
    fs.readFileSync(path.join(outDir, "native-event-browser-binder-latest.json"), "utf8"),
  );
  assert.equal(nativeReceipt.catalog_contract_current, false);
  assert.deepEqual(nativeReceipt.missing_listener_events, []);
  assert.deepEqual(nativeReceipt.missing_contract_events, []);
  assert.deepEqual(nativeReceipt.missing_replay_events, ["pointermove"]);
});

test("release-readiness browser receipt harness keeps visual receipt stale when replay did not become current", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-www-readiness-browser-stale-"));
  const snapshotPath = path.join(tempDir, "page-snapshot.json");
  const outDir = path.join(tempDir, "receipts");
  const snapshot = {
    schema: "dx.www.readiness.browser_receipt_page_snapshot.v1",
    schema_revision: 1,
    browser_runtime_executed: true,
    url: "http://127.0.0.1:3000/",
    title: "DX WWW",
    user_agent: "Mozilla/5.0 release-readinessHarness",
    viewport: {
      width: 1280,
      height: 720,
      device_pixel_ratio: 1,
    },
    binder: {
      present: false,
      contract: {},
      required_events: ["click", "pointermove", "input"],
      listener_events: [],
      browser_event_constructors: {},
      browser_event_replay_results: [],
      preview_event_count: 0,
      state_dispatch_count: 0,
      unsupported_listener_attached: false,
    },
    state_runtime: {
      present: false,
      api_methods: [],
      full_react_hook_runtime: false,
      react_api_shim_executed: false,
      state_reflection_event_count: 0,
      derived_reflection_event_count: 0,
      effect_scheduled_event_count: 0,
      action_dispatch_count: 0,
    },
    visual_edit: {
      devtools_global_present: true,
      browser_workbench_replay: "missing",
      workbench_phases: ["inspect", "cascade", "preview"],
      inspected_element_present: true,
      cascade_inspected: true,
      preview_source_mutated: false,
      apply_source_mutated: false,
      undo_source_restored: false,
      safe_local_source_target_known: true,
      apply_receipt_written: false,
      undo_receipt_written: false,
      receipt_durability: "candidate-only-not-written",
      inspected_selector: "[data-dx-component=\"state-runtime-probe\"]",
      inspected_element_fingerprint: "state-runtime-probe:section.state-runtime-probe",
      style_property: "--ring",
      style_value: "0 0% 84%",
      computed_style_before: { property: "--ring", value: "0 0% 83%" },
      computed_style_after_preview: { property: "--ring", value: "0 0% 84%" },
      computed_style_after_undo: { property: "--ring", value: "" },
      source_target: {
        relativePath: "examples/template/styles/theme.css",
        kind: "css-custom-property",
        range: {
          startByte: 381,
          endByte: 400,
          expectedText: "  --ring: 0 0% 83%;",
        },
      },
    },
    visual_edit_replay_attempt: {
      attempted: true,
      status: "not-current",
      ok: false,
      reason: "source-owned-devtools-replay-did-not-become-current",
    },
  };
  fs.writeFileSync(snapshotPath, `${JSON.stringify(snapshot, null, 2)}\n`);

  const output = execFileSync(
    process.execPath,
    [harnessPath, "--from-page-json", snapshotPath, "--out-dir", outDir],
    { cwd: root, encoding: "utf8" },
  );
  const report = JSON.parse(output);
  const visualReceipt = JSON.parse(
    fs.readFileSync(path.join(outDir, "visual-edit-browser-workbench-latest.json"), "utf8"),
  );

  assert.equal(report.receipts[3].passed, false);
  assert.equal(visualReceipt.passed, false);
  assert.equal(visualReceipt.browser_workbench_replay, "missing");
  assert.equal(visualReceipt.apply_source_mutated, false);
  assert.equal(visualReceipt.undo_source_restored, false);
  assert.deepEqual(visualReceipt.missing_workbench_phases, ["apply", "undo", "receipt"]);
});

test("release-readiness browser receipt harness rejects stale visual globals without a current replay attempt", () => {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-www-readiness-browser-stale-global-"));
  const snapshotPath = path.join(tempDir, "page-snapshot.json");
  const outDir = path.join(tempDir, "receipts");
  const snapshot = {
    schema: "dx.www.readiness.browser_receipt_page_snapshot.v1",
    schema_revision: 1,
    browser_runtime_executed: true,
    url: "http://127.0.0.1:3000/",
    title: "DX WWW",
    user_agent: "Mozilla/5.0 release-readinessHarness",
    viewport: {
      width: 1280,
      height: 720,
      device_pixel_ratio: 1,
    },
    binder: {
      present: true,
      contract: {
        supported_events: ["click", "input", "pointermove"],
        supported_event_count: 3,
        catalog_hash: "blake3:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
      },
      required_events: ["click", "pointermove", "input"],
      listener_events: ["click", "input", "pointermove"],
      browser_event_constructors: {
        click: "MouseEvent",
        input: "InputEvent",
        pointermove: "PointerEvent",
      },
      browser_event_replay_results: [
        { event: "click", tag: "button", previewed: true },
        { event: "pointermove", tag: "button", previewed: true },
        { event: "input", tag: "input", previewed: true },
      ],
      preview_event_count: 3,
      state_dispatch_count: 3,
      unsupported_listener_attached: false,
    },
    state_runtime: {
      present: true,
      api_methods: [
        "getSnapshot",
        "setSlot",
        "dispatch",
        "refreshDerivedSlots",
        "scheduleEffectsForState",
      ],
      full_react_hook_runtime: false,
      react_api_shim_executed: false,
      state_reflection_event_count: 3,
      derived_reflection_event_count: 2,
      effect_scheduled_event_count: 2,
      action_dispatch_count: 3,
    },
    visual_edit: {
      devtools_global_present: true,
      browser_workbench_replay: "current",
      workbench_phases: ["inspect", "cascade", "preview", "apply", "undo", "receipt"],
      inspected_element_present: true,
      cascade_inspected: true,
      preview_source_mutated: false,
      apply_source_mutated: true,
      undo_source_restored: true,
      safe_local_source_target_known: true,
      apply_receipt_written: true,
      undo_receipt_written: true,
      receipt_durability: "json-sr-machine-written",
      inspected_selector: "[data-dx-component=\"state-runtime-probe\"]",
      inspected_element_fingerprint: "state-runtime-probe:section.state-runtime-probe",
      style_property: "--ring",
      style_value: "0 0% 84%",
      computed_style_before: { property: "--ring", value: "0 0% 83%" },
      computed_style_after_preview: { property: "--ring", value: "0 0% 84%" },
      computed_style_after_undo: { property: "--ring", value: "0 0% 83%" },
      source_target: {
        relativePath: "examples/template/styles/theme.css",
        kind: "css-custom-property",
        range: {
          startByte: 381,
          endByte: 400,
          expectedText: "  --ring: 0 0% 83%;",
        },
      },
    },
    visual_edit_replay_attempt: {
      attempted: true,
      status: "error",
      ok: false,
      reason: "Devtools replay failed after a stale global was already present",
    },
  };
  fs.writeFileSync(snapshotPath, `${JSON.stringify(snapshot, null, 2)}\n`);

  const output = execFileSync(
    process.execPath,
    [harnessPath, "--from-page-json", snapshotPath, "--out-dir", outDir],
    { cwd: root, encoding: "utf8" },
  );
  const report = JSON.parse(output);
  const visualReceipt = JSON.parse(
    fs.readFileSync(path.join(outDir, "visual-edit-browser-workbench-latest.json"), "utf8"),
  );

  assert.equal(report.receipts[3].passed, false);
  assert.equal(visualReceipt.passed, false);
  assert.equal(visualReceipt.browser_workbench_replay, "current");
  assert.equal(visualReceipt.visual_replay_status, "error");
  assert.match(visualReceipt.visual_replay_reason, /stale global/);
});
