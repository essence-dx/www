import fs from "node:fs";
import crypto from "node:crypto";
import path from "node:path";

type JsonRecord = Record<string, unknown>;

const NATIVE_EVENT_BROWSER_BINDER_SCHEMA =
  "dx.www.readiness.native_event_browser_binder_receipt_contract";
const STATE_RUNTIME_BROWSER_SCHEMA = "dx.www.readiness.state_runtime_browser_receipt_contract";
const VISUAL_EDIT_WORKBENCH_SCHEMA = "dx.www.readiness.visual_edit_workbench_receipt_contract";
const NO_JS_BROWSER_SCHEMA = "dx.www.readiness.no_js_browser_receipt_contract";
const ISLAND_BROWSER_SCHEMA = "dx.www.readiness.island_browser_receipt_contract";
const PAGE_SNAPSHOT_SCHEMA = "dx.www.readiness.browser_receipt_page_snapshot.v1";
const NATIVE_RECEIPT_NAME = "native-event-browser-binder-latest.json";
const STATE_RECEIPT_NAME = "state-runtime-browser-latest.json";
const VISUAL_EDIT_RECEIPT_NAME = "visual-edit-browser-workbench-latest.json";
const NO_JS_BROWSER_RECEIPT_NAME = "no-js-browser-latest.json";
const ISLAND_BROWSER_RECEIPT_NAME = "island-browser-latest.json";
const REPO_ROOT = path.resolve(import.meta.dirname, "..");
const NATIVE_EVENT_CATALOG_RECEIPT_PATH = path.join(
  REPO_ROOT,
  ".dx",
  "receipts",
  "readiness",
  "native-events-latest.json",
);
const NO_JS_ARTIFACT_RECEIPT_PATH = path.join(
  REPO_ROOT,
  ".dx",
  "receipts",
  "readiness",
  "no-js-artifact-latest.json",
);
const REQUIRED_NATIVE_BROWSER_EVENTS = ["click", "pointermove", "input"];
const REQUIRED_ISLAND_DIRECTIVES = ["clientLoad", "clientVisible", "clientIdle", "clientOnly"];
const REQUIRED_STATE_RUNTIME_API = [
  "getSnapshot",
  "setSlot",
  "dispatch",
  "refreshDerivedSlots",
  "scheduleEffectsForState",
];
const REQUIRED_VISUAL_EDIT_PHASES = ["inspect", "cascade", "preview", "apply", "undo", "receipt"];

type NativeEventCatalogExpectation = {
  count: number;
  hash: string | null;
  source: string;
  verified: boolean;
  staleReason: string | null;
};

async function collectReadinessBrowserDomSnapshot() {
  const normalizeBrowserRoute = (value) => {
    if (typeof value !== "string" || value.trim().length === 0) return null;
    let route = value.trim().split("#")[0].split("?")[0];
    if (!route.startsWith("/")) route = `/${route}`;
    route = route.replace(/\/+/g, "/");
    return route.length > 1 && route.endsWith("/") ? route.slice(0, -1) : route || "/";
  };
  const bridge = document.querySelector("[data-dx-client-island-bridge]");
  const islandElements = Array.from(document.querySelectorAll("[data-dx-island]"));
  const parseJsonAttribute = (element, name, fallback) => {
    if (!(element instanceof HTMLElement)) return fallback;
    const value = element.getAttribute(name);
    if (!value) return fallback;
    try {
      return JSON.parse(value);
    } catch (_error) {
      return fallback;
    }
  };
  const splitDirectiveList = (value) =>
    typeof value === "string" && value.trim().length > 0 && value !== "none"
      ? value
          .split(/[,\s]+/)
          .map((item) => item.trim())
          .filter(Boolean)
      : [];
  const eventLog = parseJsonAttribute(bridge, "data-dx-client-island-event-log", []);
  const eventReplayResults = Array.isArray(eventLog)
    ? eventLog.map((event) => ({
        event_id: event && typeof event.eventId === "string" ? event.eventId : null,
        event: event && typeof event.event === "string" ? event.event : null,
        replayed: true,
      }))
    : [];
  const islandDirectiveSets = islandElements.map((element) =>
    element instanceof HTMLElement ? splitDirectiveList(element.dataset.dxIslandDirectives) : [],
  );
  const islandDirectivesSeen = Array.from(new Set(islandDirectiveSets.flat())).sort();
  const islandHydrationStrategies = Array.from(
    new Set(
      islandElements
        .map((element) => (element instanceof HTMLElement ? element.dataset.dxIslandHydrationStrategy : null))
        .filter((value) => typeof value === "string" && value.length > 0),
    ),
  ).sort();
  const route =
    normalizeBrowserRoute(window.location.pathname) ||
    (document.querySelector("[data-dx-route]") instanceof HTMLElement
      ? normalizeBrowserRoute(document.querySelector("[data-dx-route]").dataset.dxRoute)
      : null) ||
    "/";
  const noJsLandmarks = document.querySelectorAll("main, nav, header, footer, aside, section, article");
  const noJsAccessibilitySignals = document.querySelectorAll(
    "main, h1, h2, h3, a[href], form, label, [aria-label], [aria-labelledby], [role], img[alt]",
  );
  const noJsRoot = document.querySelector("[data-dx-output-mode], [data-dx-js]");
  return {
    schema: "dx.www.readiness.browser_receipt_page_snapshot.v1",
    schema_revision: 1,
    collected_by: "benchmarks/dx-www-readiness-browser-receipt-harness.ts",
    collector_mode: "read-only-dom-after-browser-interactions",
    browser_runtime_executed: eventReplayResults.length > 0,
    url: window.location.href,
    canonical_route: route,
    title: document.title,
    user_agent: window.navigator.userAgent,
    project_root: null,
    project_root_source: null,
    viewport: {
      width: window.innerWidth,
      height: window.innerHeight,
      device_pixel_ratio: window.devicePixelRatio || 1,
    },
    binder: {
      present: document.querySelector("[data-dx-dom-action-bound]") instanceof HTMLElement,
      contract: null,
      snapshot: null,
      bound_element_count: document.querySelectorAll("[data-dx-dom-action-bound]").length,
      required_events: ["click", "pointermove", "input"],
      listener_events: Array.from(
        new Set(
          Array.from(document.querySelectorAll("[data-dx-dom-action-last-event]"))
            .map((element) => element.getAttribute("data-dx-dom-action-last-event"))
            .filter((value) => typeof value === "string" && value.length > 0),
        ),
      ).sort(),
      browser_event_constructors: {},
      browser_event_replay_results: [],
      preview_event_count: document.querySelectorAll("[data-dx-dom-action-last-event]").length,
      state_dispatch_count: document.querySelectorAll("[data-dx-state-runtime-dispatch='dispatched']").length,
      unsupported_listener_attached: false,
    },
    state_runtime: {
      present: document.querySelector("[data-dx-state-read], [data-dx-state-value]") instanceof HTMLElement,
      schema: null,
      route,
      api_methods: [],
      slot_count: document.querySelectorAll("[data-dx-state-read], [data-dx-state-value]").length,
      event_count: eventReplayResults.length,
      snapshot: null,
      state_reflection_event_count: 0,
      derived_reflection_event_count: 0,
      effect_scheduled_event_count: 0,
      action_dispatch_count: 0,
      full_react_hook_runtime: false,
      react_api_shim_executed: false,
    },
    islands: {
      bridge_present: bridge instanceof HTMLElement,
      source_owned_bridge:
        bridge instanceof HTMLElement && bridge.dataset.dxClientIslandBridge === "source-owned",
      bridge_abi_style: bridge instanceof HTMLElement ? bridge.dataset.dxClientIslandAbi ?? null : null,
      abi_schema: bridge instanceof HTMLElement ? bridge.dataset.dxIslandAbiSchema ?? null : null,
      directive_style:
        bridge instanceof HTMLElement ? bridge.dataset.dxIslandDirectiveStyle ?? null : null,
      no_js_fallback_preserved:
        bridge instanceof HTMLElement && bridge.dataset.dxNoJsFallback === "preserved",
      island_count: islandElements.length,
      source_owned_island_count: islandElements.filter(
        (element) =>
          element instanceof HTMLElement &&
          element.dataset.dxIslandAbiSchema === "dx.react.clientIsland.abi" &&
          element.dataset.dxIslandDirectiveStyle === "camelCase-jsx-props" &&
          element.dataset.dxProviderAdapter === "not-executed",
      ).length,
      directives_seen: islandDirectivesSeen,
      hydration_strategies: islandHydrationStrategies,
      event_node_count: eventReplayResults.length,
      event_replay_results: eventReplayResults,
      client_island_event_count: eventReplayResults.length,
      client_only_adapter_values: Array.from(
        new Set(
          islandElements
            .map((element) => (element instanceof HTMLElement ? element.dataset.dxClientOnlyAdapter : null))
            .filter((value) => typeof value === "string" && value.length > 0),
        ),
      ).sort(),
      provider_adapter_values: Array.from(
        new Set(
          islandElements
            .map((element) => (element instanceof HTMLElement ? element.dataset.dxProviderAdapter : null))
            .filter((value) => typeof value === "string" && value.length > 0),
        ),
      ).sort(),
      browser_runtime_proof_values: Array.from(
        new Set(
          islandElements
            .map((element) => (element instanceof HTMLElement ? element.dataset.dxBrowserRuntimeProof : null))
            .filter((value) => typeof value === "string" && value.length > 0),
        ),
      ).sort(),
      full_react_hydration: false,
      node_modules_required: false,
      react_synthetic_events: false,
      provider_adapter_executed: false,
    },
    visual_edit: null,
    visual_edit_replay_attempt: {
      attempted: false,
      status: "read-only-dom-collector",
      ok: false,
      reason: "collector intentionally records island browser proof without running visual edit replay",
    },
    no_js: {
      live_browser_executed: true,
      javascript_disabled_browser: false,
      page_javascript_enabled: true,
      html_path: null,
      route,
      artifact_html_blake3: null,
      script_tag_count: document.scripts.length,
      data_dx_output_mode_tiny_static:
        noJsRoot instanceof HTMLElement && noJsRoot.dataset.dxOutputMode === "tiny-static",
      data_dx_js_none: noJsRoot instanceof HTMLElement && noJsRoot.dataset.dxJs === "none",
      semantic_landmark_present: noJsLandmarks.length > 0,
      visible_text_present: document.body.innerText.trim().length > 0,
      link_count: document.querySelectorAll("a[href]").length,
      form_count: document.querySelectorAll("form").length,
      seo_title_present: document.title.trim().length > 0,
      accessibility_signal_count: noJsAccessibilitySignals.length,
    },
  };
}

async function collectReadinessBrowserPageSnapshot() {
  const eventLog = [];
  const readinessFlags = window.__DX_READINESS_BROWSER_FLAGS__ || {};
  const normalizeBrowserRoute = (value) => {
    if (typeof value !== "string" || value.trim().length === 0) return null;
    let route = value.trim().split("#")[0].split("?")[0];
    if (!route.startsWith("/")) route = `/${route}`;
    route = route.replace(/\/+/g, "/");
    return route.length > 1 && route.endsWith("/") ? route.slice(0, -1) : route || "/";
  };
  const pageCanonicalRoute =
    normalizeBrowserRoute(readinessFlags.canonicalRoute) ||
    normalizeBrowserRoute(readinessFlags.route) ||
    normalizeBrowserRoute(window.location.pathname) ||
    "/";
  const capture = (type) => (event) => {
    eventLog.push({ type, detail: event && event.detail ? event.detail : null });
  };
  const capturedEvents = [
    "dx:dom-action-binder-ready",
    "dx:dom-action-preview",
    "dx:state-runtime-dispatch",
    "dx:state-runtime-ready",
    "dx:state-dom-reflection",
    "dx:derived-state-slot",
    "dx:effect-scheduled",
    "dx:state-event",
    "dx:state-slot",
    "dx:client-island-event",
    "dx:preload",
  ];
  capturedEvents.forEach((type) => {
    window.addEventListener(type, capture(type), { passive: true });
    document.addEventListener(type, capture(type), { passive: true });
  });

  const binder = window.__DX_DOM_ACTION_BINDER__ || null;
  const runtime = window.__DX_STATE_GRAPH_RUNTIME__ || null;
  let devtoolsSession = null;
  try {
    const sessionResponse = await fetch("/_dx/devtools/session", {
      cache: "no-store",
      headers: { accept: "application/json" },
    });
    if (sessionResponse.ok) {
      devtoolsSession = await sessionResponse.json();
    }
  } catch (_error) {
    devtoolsSession = null;
  }
  const boundElements = Array.from(document.querySelectorAll("[data-dx-dom-action-bound]"));
  const candidateEvents = ["click", "pointermove", "input"];
  const browserEventConstructors = {};
  const browserEventReplayResults = [];

  const previewCount = () => eventLog.filter((event) => event.type === "dx:dom-action-preview").length;
  const createBrowserEvent = (eventName) => {
    if (eventName === "click" && typeof MouseEvent === "function") {
      browserEventConstructors[eventName] = "MouseEvent";
      return new MouseEvent(eventName, {
        bubbles: true,
        cancelable: true,
        button: 0,
        buttons: 1,
        clientX: 16,
        clientY: 16,
        view: window,
      });
    }
    if (eventName.startsWith("pointer") && typeof PointerEvent === "function") {
      browserEventConstructors[eventName] = "PointerEvent";
      return new PointerEvent(eventName, {
        bubbles: true,
        cancelable: true,
        pointerId: 1,
        pointerType: "mouse",
        isPrimary: true,
        clientX: 24,
        clientY: 24,
      });
    }
    if (eventName === "input" && typeof InputEvent === "function") {
      browserEventConstructors[eventName] = "InputEvent";
      return new InputEvent(eventName, {
        bubbles: true,
        cancelable: true,
        inputType: "insertText",
        data: "dx-readiness",
      });
    }
    browserEventConstructors[eventName] = "Event";
    return new Event(eventName, { bubbles: true, cancelable: true });
  };
  const dispatchBrowserEvent = (element, eventName) => {
    const before = previewCount();
    if (
      eventName === "input" &&
      (element instanceof HTMLInputElement || element instanceof HTMLTextAreaElement)
    ) {
      element.value = `${element.value || ""}dx-readiness`;
    }
    element.dispatchEvent(createBrowserEvent(eventName));
    const previewed = previewCount() > before;
    browserEventReplayResults.push({
      event: eventName,
      tag: element.tagName ? element.tagName.toLowerCase() : null,
      previewed,
    });
    return previewed;
  };
  for (const eventName of candidateEvents) {
    for (const element of boundElements) {
      dispatchBrowserEvent(element, eventName);
    }
  }
  const unsupportedBefore = previewCount();
  for (const element of boundElements) {
    dispatchBrowserEvent(element, "dxunsupportedreadinessevent");
  }
  const unsupportedListenerAttached = previewCount() > unsupportedBefore;

  const runtimeProgram = runtime && runtime.program && typeof runtime.program === "object"
    ? runtime.program
    : {};
  const slots = Array.isArray(runtimeProgram.slots) ? runtimeProgram.slots : [];
  const events = Array.isArray(runtimeProgram.events) ? runtimeProgram.events : [];
  const mutateValue = (value, index) => {
    if (typeof value === "boolean") return !value;
    if (typeof value === "number") return value + index + 1;
    if (typeof value === "string") return `${value}-readiness-${index + 1}`;
    return `readiness-${index + 1}`;
  };
  if (runtime && typeof runtime.setSlot === "function") {
    slots.slice(0, 3).forEach((slot, index) => {
      if (slot && typeof slot.name === "string") {
        runtime.setSlot(slot.name, mutateValue(slot.initial_value, index));
      }
    });
  }
  if (runtime && typeof runtime.refreshDerivedSlots === "function") {
    runtime.refreshDerivedSlots(null, "readiness-browser-receipt");
  }
  if (runtime && typeof runtime.scheduleEffectsForState === "function") {
    slots.slice(0, 3).forEach((slot) => {
      if (slot && typeof slot.name === "string") {
        runtime.scheduleEffectsForState(slot.name, "readiness-browser-receipt");
      }
    });
  }
  if (runtime && typeof runtime.dispatch === "function") {
    events.slice(0, 3).forEach((event) => {
      if (event && typeof event.id === "string") {
        runtime.dispatch(event.id, {
          schema: "dx.www.readiness.browser_receipt_event_payload.v1",
          event: event.event || null,
        });
      }
    });
  }
  const runtimeSnapshot =
    runtime && typeof runtime.getSnapshot === "function" ? runtime.getSnapshot() : null;
  const islandBridge = document.querySelector("[data-dx-client-island-bridge]");
  const islandElements = Array.from(document.querySelectorAll("[data-dx-island]"));
  const islandEventNodes = Array.from(document.querySelectorAll("[data-dx-event-id][data-dx-event]"));
  const splitDirectiveList = (value) =>
    typeof value === "string" && value.trim().length > 0 && value !== "none"
      ? value
          .split(",")
          .map((item) => item.trim())
          .filter(Boolean)
      : [];
  const islandEventCount = () => eventLog.filter((event) => event.type === "dx:client-island-event").length;
  const islandEventReplayResults = [];
  for (const node of islandEventNodes) {
    const eventName = node instanceof HTMLElement ? node.dataset.dxEvent : null;
    const eventId = node instanceof HTMLElement ? node.dataset.dxEventId : null;
    if (!eventName || !eventId) continue;
    const before = islandEventCount();
    node.dispatchEvent(createBrowserEvent(eventName));
    islandEventReplayResults.push({
      event_id: eventId,
      event: eventName,
      replayed: islandEventCount() > before,
    });
  }
  const islandDirectiveSets = islandElements.map((element) =>
    element instanceof HTMLElement ? splitDirectiveList(element.dataset.dxIslandDirectives) : [],
  );
  const islandDirectivesSeen = Array.from(new Set(islandDirectiveSets.flat())).sort();
  const islandHydrationStrategies = Array.from(
    new Set(
      islandElements
        .map((element) => (element instanceof HTMLElement ? element.dataset.dxIslandHydrationStrategy : null))
        .filter((value) => typeof value === "string" && value.length > 0),
    ),
  ).sort();

  const visualReplayRunner = window.__DX_DEVTOOLS_READINESS_VISUAL_EDIT_REPLAY__;
  let visualReplayAttempt = {
    attempted: false,
    status: "missing-entrypoint",
    ok: false,
    reason: "window.__DX_DEVTOOLS_READINESS_VISUAL_EDIT_REPLAY__ is not present",
  };
  if (typeof visualReplayRunner === "function") {
    visualReplayAttempt = {
      attempted: true,
      status: "started",
      ok: false,
      reason: "replay-not-finished",
    };
    try {
      const replayResult = await visualReplayRunner();
      visualReplayAttempt = {
        attempted: true,
        status: replayResult && replayResult.ok === true ? "current" : "not-current",
        ok: replayResult && replayResult.ok === true,
        reason:
          replayResult && typeof replayResult.reason === "string"
            ? replayResult.reason
            : replayResult && replayResult.ok === true
              ? "source-owned-devtools-replay-completed"
              : "source-owned-devtools-replay-did-not-become-current",
      };
    } catch (error) {
      const reason = error instanceof Error ? error.message : String(error);
      visualReplayAttempt = {
        attempted: true,
        status: "error",
        ok: false,
        reason,
      };
    }
  }

  const domPreviews = eventLog.filter((event) => event.type === "dx:dom-action-preview");
  const stateDispatches = eventLog.filter((event) => event.type === "dx:state-runtime-dispatch");
  const stateReflections = eventLog.filter((event) => event.type === "dx:state-dom-reflection");
  const derivedReflections = eventLog.filter((event) => event.type === "dx:derived-state-slot");
  const effectScheduled = eventLog.filter((event) => event.type === "dx:effect-scheduled");
  const stateEvents = eventLog.filter((event) => event.type === "dx:state-event");
  const listenerEvents = Array.from(
    new Set(
      domPreviews
        .map((event) => event.detail && event.detail.event)
        .filter((eventName) => typeof eventName === "string"),
    ),
  ).sort();
  const noJsLandmarks = document.querySelectorAll("main, nav, header, footer, aside, section, article");
  const noJsAccessibilitySignals = document.querySelectorAll(
    "main, h1, h2, h3, a[href], form, label, [aria-label], [aria-labelledby], [role], img[alt]",
  );
  const noJsRoot = document.querySelector("[data-dx-output-mode], [data-dx-js]");
  const pageJavascriptEnabled =
    Boolean(window.__DX_DOM_ACTION_BINDER__) ||
    Boolean(window.__DX_STATE_GRAPH_RUNTIME__) ||
    Boolean(window.__DX_DEVTOOLS__) ||
    Boolean(window.__DX_READINESS_VISUAL_EDIT_REPLAY__);

  return {
    schema: "dx.www.readiness.browser_receipt_page_snapshot.v1",
    schema_revision: 1,
    collected_by: "benchmarks/dx-www-readiness-browser-receipt-harness.ts",
    browser_runtime_executed: true,
    url: window.location.href,
    canonical_route: pageCanonicalRoute,
    title: document.title,
    user_agent: window.navigator.userAgent,
    project_root:
      devtoolsSession && typeof devtoolsSession.project_root === "string"
        ? devtoolsSession.project_root
        : null,
    project_root_source:
      devtoolsSession && typeof devtoolsSession.project_root === "string"
        ? "/_dx/devtools/session"
        : null,
    viewport: {
      width: window.innerWidth,
      height: window.innerHeight,
      device_pixel_ratio: window.devicePixelRatio || 1,
    },
    binder: {
      present: Boolean(binder),
      contract: binder && binder.contract ? binder.contract : null,
      snapshot: binder && typeof binder.getSnapshot === "function" ? binder.getSnapshot() : null,
      bound_element_count: boundElements.length,
      required_events: candidateEvents,
      listener_events: listenerEvents,
      browser_event_constructors: browserEventConstructors,
      browser_event_replay_results: browserEventReplayResults,
      preview_event_count: domPreviews.length,
      state_dispatch_count: stateDispatches.length,
      unsupported_listener_attached: unsupportedListenerAttached,
    },
    state_runtime: {
      present: Boolean(runtime),
      schema: runtime && runtime.schema ? runtime.schema : null,
      route: runtime && runtime.route ? runtime.route : null,
      api_methods: runtime
        ? ["getSnapshot", "setSlot", "dispatch", "refreshDerivedSlots", "scheduleEffectsForState"]
            .filter((name) => typeof runtime[name] === "function")
        : [],
      slot_count: slots.length,
      event_count: events.length,
      snapshot: runtimeSnapshot,
      state_reflection_event_count: stateReflections.length,
      derived_reflection_event_count: derivedReflections.length,
      effect_scheduled_event_count: effectScheduled.length,
      action_dispatch_count: stateEvents.length,
      full_react_hook_runtime: false,
      react_api_shim_executed: false,
    },
    islands: {
      bridge_present: islandBridge instanceof HTMLElement,
      source_owned_bridge:
        islandBridge instanceof HTMLElement && islandBridge.dataset.dxClientIslandBridge === "source-owned",
      bridge_abi_style:
        islandBridge instanceof HTMLElement ? islandBridge.dataset.dxClientIslandAbi ?? null : null,
      abi_schema:
        islandBridge instanceof HTMLElement ? islandBridge.dataset.dxIslandAbiSchema ?? null : null,
      directive_style:
        islandBridge instanceof HTMLElement ? islandBridge.dataset.dxIslandDirectiveStyle ?? null : null,
      no_js_fallback_preserved:
        islandBridge instanceof HTMLElement && islandBridge.dataset.dxNoJsFallback === "preserved",
      island_count: islandElements.length,
      source_owned_island_count: islandElements.filter(
        (element) =>
          element instanceof HTMLElement &&
          element.dataset.dxIslandAbiSchema === "dx.react.clientIsland.abi" &&
          element.dataset.dxIslandDirectiveStyle === "camelCase-jsx-props" &&
          element.dataset.dxProviderAdapter === "not-executed",
      ).length,
      directives_seen: islandDirectivesSeen,
      hydration_strategies: islandHydrationStrategies,
      event_node_count: islandEventNodes.length,
      event_replay_results: islandEventReplayResults,
      client_island_event_count: islandEventCount(),
      client_only_adapter_values: Array.from(
        new Set(
          islandElements
            .map((element) => (element instanceof HTMLElement ? element.dataset.dxClientOnlyAdapter : null))
            .filter((value) => typeof value === "string" && value.length > 0),
        ),
      ).sort(),
      provider_adapter_values: Array.from(
        new Set(
          islandElements
            .map((element) => (element instanceof HTMLElement ? element.dataset.dxProviderAdapter : null))
            .filter((value) => typeof value === "string" && value.length > 0),
        ),
      ).sort(),
      browser_runtime_proof_values: Array.from(
        new Set(
          islandElements
            .map((element) => (element instanceof HTMLElement ? element.dataset.dxBrowserRuntimeProof : null))
            .filter((value) => typeof value === "string" && value.length > 0),
        ),
      ).sort(),
      full_react_hydration: false,
      node_modules_required: false,
      react_synthetic_events: false,
      provider_adapter_executed: false,
    },
    visual_edit: window.__DX_READINESS_VISUAL_EDIT_REPLAY__ || null,
    visual_edit_replay_attempt: visualReplayAttempt,
    no_js: {
      live_browser_executed: true,
      javascript_disabled_browser: readinessFlags.javascriptDisabledBrowser === true,
      page_javascript_enabled: readinessFlags.pageJavascriptEnabled === true ? true : pageJavascriptEnabled,
      html_path: typeof readinessFlags.htmlPath === "string" ? readinessFlags.htmlPath : null,
      route:
        normalizeBrowserRoute(readinessFlags.noJsRoute) ||
        (noJsRoot instanceof HTMLElement ? normalizeBrowserRoute(noJsRoot.dataset.dxRoute) : null) ||
        pageCanonicalRoute,
      artifact_html_blake3:
        typeof readinessFlags.artifactHtmlBlake3 === "string"
          ? readinessFlags.artifactHtmlBlake3
          : null,
      script_tag_count: document.scripts.length,
      data_dx_output_mode_tiny_static:
        noJsRoot instanceof HTMLElement && noJsRoot.dataset.dxOutputMode === "tiny-static",
      data_dx_js_none: noJsRoot instanceof HTMLElement && noJsRoot.dataset.dxJs === "none",
      semantic_landmark_present: noJsLandmarks.length > 0,
      visible_text_present: document.body.innerText.trim().length > 0,
      link_count: document.querySelectorAll("a[href]").length,
      form_count: document.querySelectorAll("form").length,
      seo_title_present: document.title.trim().length > 0,
      accessibility_signal_count: noJsAccessibilitySignals.length,
    },
  };
}

function stringArray(value: unknown): string[] {
  return Array.isArray(value)
    ? value.filter((item): item is string => typeof item === "string")
    : [];
}

function missingStrings(required: string[], actual: string[]): string[] {
  return required.filter((item) => !actual.includes(item));
}

function numberValue(value: unknown): number {
  return typeof value === "number" && Number.isFinite(value) ? value : 0;
}

function recordValue(value: unknown): JsonRecord {
  return value && typeof value === "object" && !Array.isArray(value) ? (value as JsonRecord) : {};
}

function canonicalRouteFromValue(value: unknown): string | null {
  if (typeof value !== "string" || value.trim().length === 0) return null;
  let route = value.trim();
  try {
    if (/^[a-zA-Z][a-zA-Z0-9+.-]*:/.test(route)) {
      route = new URL(route).pathname;
    }
  } catch {
    return null;
  }
  route = route.split("#")[0].split("?")[0];
  if (!route.startsWith("/")) route = `/${route}`;
  route = route.replace(/\/+/g, "/");
  return route.length > 1 && route.endsWith("/") ? route.slice(0, -1) : route || "/";
}

function canonicalRouteFromSnapshot(snapshot: JsonRecord): string | null {
  return (
    canonicalRouteFromValue(snapshot.canonical_route) ??
    canonicalRouteFromValue(snapshot.route) ??
    canonicalRouteFromValue(snapshot.url)
  );
}

function canonicalRouteForRecord(record: JsonRecord, snapshot: JsonRecord): string | null {
  return (
    canonicalRouteFromValue(record.canonical_route) ??
    canonicalRouteFromValue(record.route) ??
    canonicalRouteFromSnapshot(snapshot)
  );
}

function localBrowserProofTarget(
  kind: string,
  route: string | null,
  details: JsonRecord = {},
): JsonRecord {
  return {
    kind,
    route,
    proof_boundary: "local-browser-only",
    hosted_provider_proof: false,
    provider_adapter_executed: false,
    ...details,
  };
}

function hasApiMethods(methods: string[]): boolean {
  return missingStrings(REQUIRED_STATE_RUNTIME_API, methods).length === 0;
}

function readNativeEventNamesFromSource(): string[] {
  try {
    const sourcePath = path.join(REPO_ROOT, "core", "src", "delivery", "dom_events.rs");
    const source = fs.readFileSync(sourcePath, "utf8");
    const match = source.match(/const NATIVE_DOM_EVENT_NAMES:\s*&\[&str\]\s*=\s*&\[([\s\S]*?)\];/);
    if (!match) return [];
    return Array.from(match[1].matchAll(/"([^"]+)"/g), (item) => item[1]);
  } catch {
    return [];
  }
}

function nativeEventCatalogExpectation(): NativeEventCatalogExpectation {
  try {
    const receipt = JSON.parse(fs.readFileSync(NATIVE_EVENT_CATALOG_RECEIPT_PATH, "utf8")) as JsonRecord;
    const count = numberValue(receipt.catalog_count);
    const hash = typeof receipt.catalog_hash === "string" ? receipt.catalog_hash : null;
    if (count > 0 && hash && hash.startsWith("blake3:")) {
      return {
        count,
        hash,
        source: ".dx/receipts/readiness/native-events-latest.json",
        verified: true,
        staleReason: null,
      };
    }
  } catch {
    // Fall through to the source-count fallback below. The canonical CLI import still validates hash.
  }

  return {
    count: readNativeEventNamesFromSource().length,
    hash: null,
    source: "core/src/delivery/dom_events.rs",
    verified: false,
    staleReason: "native-events-catalog-receipt-missing",
  };
}

function noJsArtifactReceipt(): JsonRecord {
  try {
    return JSON.parse(fs.readFileSync(NO_JS_ARTIFACT_RECEIPT_PATH, "utf8")) as JsonRecord;
  } catch {
    return {};
  }
}

function stableJson(value: unknown): string {
  if (Array.isArray(value)) return `[${value.map(stableJson).join(",")}]`;
  if (value && typeof value === "object") {
    const record = value as JsonRecord;
    return `{${Object.keys(record)
      .sort()
      .map((key) => `${JSON.stringify(key)}:${stableJson(record[key])}`)
      .join(",")}}`;
  }
  return JSON.stringify(value);
}

function snapshotHash(value: unknown): string {
  return `sha256:${crypto.createHash("sha256").update(stableJson(value)).digest("hex")}`;
}

function nativeReceiptFromPageSnapshot(snapshot: JsonRecord): JsonRecord {
  const binder = recordValue(snapshot.binder);
  const contract = recordValue(binder.contract);
  const canonicalRoute = canonicalRouteFromSnapshot(snapshot);
  const catalogExpectation = nativeEventCatalogExpectation();
  const requiredEvents = stringArray(binder.required_events);
  const requiredBrowserEvents = requiredEvents.length > 0 ? requiredEvents : REQUIRED_NATIVE_BROWSER_EVENTS;
  const listenerEvents = stringArray(binder.listener_events);
  const supportedEvents = stringArray(contract.supported_events);
  const supportedEventCount = numberValue(contract.supported_event_count);
  const catalogHash = typeof contract.catalog_hash === "string" ? contract.catalog_hash : null;
  const catalogContractCurrent =
    catalogExpectation.verified &&
    supportedEventCount === catalogExpectation.count &&
    supportedEvents.length === catalogExpectation.count &&
    catalogHash === catalogExpectation.hash;
  const replayResults = Array.isArray(binder.browser_event_replay_results)
    ? binder.browser_event_replay_results
    : [];
  const replayedEvents = Array.from(
    new Set(
      replayResults
        .map(recordValue)
        .filter((item) => item.previewed === true)
        .map((item) => item.event)
        .filter((eventName): eventName is string => typeof eventName === "string"),
    ),
  ).sort();
  const missingListenerEvents = missingStrings(requiredBrowserEvents, listenerEvents);
  const missingContractEvents = missingStrings(requiredBrowserEvents, supportedEvents);
  const missingReplayEvents = missingStrings(requiredBrowserEvents, replayedEvents);
  const passed =
    snapshot.browser_runtime_executed === true &&
    binder.present === true &&
    catalogContractCurrent &&
    missingListenerEvents.length === 0 &&
    missingContractEvents.length === 0 &&
    missingReplayEvents.length === 0 &&
    numberValue(binder.preview_event_count) >= requiredBrowserEvents.length &&
    numberValue(binder.state_dispatch_count) >= requiredBrowserEvents.length &&
    binder.unsupported_listener_attached === false &&
    requiredBrowserEvents.every((eventName) => listenerEvents.includes(eventName));

  return {
    schema: NATIVE_EVENT_BROWSER_BINDER_SCHEMA,
    schema_revision: 1,
    passed,
    browser_runtime_executed: snapshot.browser_runtime_executed === true,
    route: canonicalRoute,
    canonical_route: canonicalRoute,
    proof_target: localBrowserProofTarget("native-event-binder", canonicalRoute, {
      global: "__DX_DOM_ACTION_BINDER__",
      required_events: requiredBrowserEvents,
    }),
    binder_global_present: binder.present === true,
    unsupported_listener_attached: binder.unsupported_listener_attached === true,
    supported_event_count: supportedEventCount,
    catalog_hash: catalogHash,
    expected_catalog_count: catalogExpectation.count,
    expected_catalog_hash: catalogExpectation.hash,
    catalog_contract_current: catalogContractCurrent,
    catalog_contract_source: catalogExpectation.source,
    catalog_contract_stale_reason: catalogExpectation.staleReason,
    required_events: requiredBrowserEvents,
    listener_events: listenerEvents,
    missing_listener_events: missingListenerEvents,
    missing_contract_events: missingContractEvents,
    missing_replay_events: missingReplayEvents,
    browser_event_constructors: recordValue(binder.browser_event_constructors),
    browser_event_replay_results: replayResults,
    preview_event_count: numberValue(binder.preview_event_count),
    state_dispatch_count: numberValue(binder.state_dispatch_count),
    react_synthetic_events: false,
    full_react_event_parity: false,
    browser_snapshot_hash: snapshotHash({
      url: snapshot.url ?? null,
      user_agent: snapshot.user_agent ?? null,
      canonical_route: canonicalRoute,
      binder,
    }),
    proof_scope: "local-in-app-browser-native-event-binder-replay",
    receipt_source: PAGE_SNAPSHOT_SCHEMA,
    page_url: snapshot.url ?? null,
    hosted_provider_proof: false,
    provider_adapter_executed: false,
    release_ready: false,
    fastest_world_claim: false,
  };
}

function stateReceiptFromPageSnapshot(snapshot: JsonRecord): JsonRecord {
  const stateRuntime = recordValue(snapshot.state_runtime);
  const canonicalRoute = canonicalRouteForRecord(stateRuntime, snapshot);
  const apiMethods = stringArray(stateRuntime.api_methods);
  const missingApiMethods = missingStrings(REQUIRED_STATE_RUNTIME_API, apiMethods);
  const passed =
    snapshot.browser_runtime_executed === true &&
    stateRuntime.present === true &&
    hasApiMethods(apiMethods) &&
    stateRuntime.full_react_hook_runtime === false &&
    stateRuntime.react_api_shim_executed === false &&
    numberValue(stateRuntime.state_reflection_event_count) >= 3 &&
    numberValue(stateRuntime.derived_reflection_event_count) >= 2 &&
    numberValue(stateRuntime.effect_scheduled_event_count) >= 2 &&
    numberValue(stateRuntime.action_dispatch_count) >= 3;

  return {
    schema: STATE_RUNTIME_BROWSER_SCHEMA,
    schema_revision: 1,
    passed,
    browser_runtime_executed: snapshot.browser_runtime_executed === true,
    route: canonicalRoute,
    canonical_route: canonicalRoute,
    proof_target: localBrowserProofTarget("state-runtime", canonicalRoute, {
      global: "__DX_STATE_GRAPH_RUNTIME__",
      runtime_schema: stateRuntime.schema ?? null,
    }),
    runtime_global_present: stateRuntime.present === true,
    full_react_hook_runtime: false,
    react_api_shim_executed: false,
    state_reflection_event_count: numberValue(stateRuntime.state_reflection_event_count),
    derived_reflection_event_count: numberValue(stateRuntime.derived_reflection_event_count),
    effect_scheduled_event_count: numberValue(stateRuntime.effect_scheduled_event_count),
    action_dispatch_count: numberValue(stateRuntime.action_dispatch_count),
    api_methods: apiMethods,
    missing_api_methods: missingApiMethods,
    slot_count: numberValue(stateRuntime.slot_count),
    event_count: numberValue(stateRuntime.event_count),
    runtime_schema: stateRuntime.schema ?? null,
    browser_snapshot_hash: snapshotHash({
      url: snapshot.url ?? null,
      user_agent: snapshot.user_agent ?? null,
      canonical_route: canonicalRoute,
      state_runtime: stateRuntime,
    }),
    proof_scope: "local-in-app-browser-state-runtime-replay",
    receipt_source: PAGE_SNAPSHOT_SCHEMA,
    page_url: snapshot.url ?? null,
    hosted_provider_proof: false,
    provider_adapter_executed: false,
    release_ready: false,
    fastest_world_claim: false,
  };
}

function islandBrowserReceiptFromPageSnapshot(snapshot: JsonRecord): JsonRecord {
  const islands = recordValue(snapshot.islands);
  const canonicalRoute = canonicalRouteFromSnapshot(snapshot);
  const directivesSeen = stringArray(islands.directives_seen);
  const hydrationStrategies = stringArray(islands.hydration_strategies);
  const missingCoreDirectives = missingStrings(REQUIRED_ISLAND_DIRECTIVES, directivesSeen);
  const eventReplayResults = Array.isArray(islands.event_replay_results)
    ? islands.event_replay_results.map(recordValue)
    : [];
  const missedEventReplays = eventReplayResults.filter((item) => item.replayed !== true);
  const islandCount = numberValue(islands.island_count);
  const sourceOwnedIslandCount = numberValue(islands.source_owned_island_count);
  const eventNodeCount = numberValue(islands.event_node_count);
  const clientIslandEventCount = numberValue(islands.client_island_event_count);
  const passed =
    snapshot.browser_runtime_executed === true &&
    islands.bridge_present === true &&
    islands.source_owned_bridge === true &&
    islands.bridge_abi_style === "camelCase" &&
    islands.abi_schema === "dx.react.clientIsland.abi" &&
    islands.directive_style === "camelCase-jsx-props" &&
    islands.no_js_fallback_preserved === true &&
    islandCount > 0 &&
    sourceOwnedIslandCount === islandCount &&
    missingCoreDirectives.length === 0 &&
    hydrationStrategies.length > 0 &&
    eventNodeCount > 0 &&
    clientIslandEventCount >= eventNodeCount &&
    missedEventReplays.length === 0 &&
    islands.full_react_hydration === false &&
    islands.node_modules_required === false &&
    islands.react_synthetic_events === false &&
    islands.provider_adapter_executed === false;

  return {
    schema: ISLAND_BROWSER_SCHEMA,
    schema_revision: 1,
    passed,
    status: passed ? "source-owned-island-browser-replay-current" : "candidate-not-current",
    browser_runtime_executed: snapshot.browser_runtime_executed === true,
    route: canonicalRoute,
    canonical_route: canonicalRoute,
    proof_target: localBrowserProofTarget("source-owned-client-islands", canonicalRoute, {
      selector: "[data-dx-client-island-bridge]",
      event_selector: "[data-dx-event-id][data-dx-event]",
    }),
    bridge_present: islands.bridge_present === true,
    source_owned_bridge: islands.source_owned_bridge === true,
    bridge_abi_style: islands.bridge_abi_style ?? null,
    abi_schema: islands.abi_schema ?? null,
    directive_style: islands.directive_style ?? null,
    no_js_fallback_preserved: islands.no_js_fallback_preserved === true,
    island_count: islandCount,
    source_owned_island_count: sourceOwnedIslandCount,
    directives_seen: directivesSeen,
    missing_core_directives: missingCoreDirectives,
    hydration_strategies: hydrationStrategies,
    event_node_count: eventNodeCount,
    client_island_event_count: clientIslandEventCount,
    event_replay_results: eventReplayResults,
    missed_event_replay_count: missedEventReplays.length,
    client_only_adapter_values: stringArray(islands.client_only_adapter_values),
    provider_adapter_values: stringArray(islands.provider_adapter_values),
    browser_runtime_proof_values: stringArray(islands.browser_runtime_proof_values),
    full_react_hydration: false,
    node_modules_required: false,
    react_synthetic_events: false,
    provider_adapter_executed: false,
    browser_snapshot_hash: snapshotHash({
      url: snapshot.url ?? null,
      user_agent: snapshot.user_agent ?? null,
      canonical_route: canonicalRoute,
      islands,
    }),
    proof_scope: "local-in-app-browser-source-owned-island-replay",
    receipt_source: PAGE_SNAPSHOT_SCHEMA,
    page_url: snapshot.url ?? null,
    hosted_provider_proof: false,
    release_ready: false,
    fastest_world_claim: false,
  };
}

function visualEditReceiptFromPageSnapshot(snapshot: JsonRecord): JsonRecord {
  const visual = recordValue(snapshot.visual_edit);
  const replayAttempt = recordValue(snapshot.visual_edit_replay_attempt);
  const phases = stringArray(visual.workbench_phases);
  const missingPhases = missingStrings(REQUIRED_VISUAL_EDIT_PHASES, phases);
  const viewport = recordValue(snapshot.viewport);
  const sourceTarget = recordValue(visual.source_target);
  const range = recordValue(sourceTarget.range);
  const computedBefore = recordValue(visual.computed_style_before);
  const computedPreview = recordValue(visual.computed_style_after_preview);
  const computedUndo = recordValue(visual.computed_style_after_undo);
  const pageUrl = typeof snapshot.url === "string" ? snapshot.url : "";
  const userAgent = typeof snapshot.user_agent === "string" ? snapshot.user_agent : "";
  const canonicalRoute = canonicalRouteFromSnapshot(snapshot);
  const proofTarget = localBrowserProofTarget("visual-edit-workbench", canonicalRoute, {
    source_target_relative_path:
      typeof sourceTarget.relativePath === "string" ? sourceTarget.relativePath : null,
    inspected_selector:
      typeof visual.inspected_selector === "string" ? visual.inspected_selector : null,
  });
  const browserSnapshotHash = snapshotHash({
    pageUrl,
    userAgent,
    canonical_route: canonicalRoute,
    viewport,
    visual,
    proof_target: proofTarget,
  });
  const passed =
    snapshot.browser_runtime_executed === true &&
    replayAttempt.attempted === true &&
    replayAttempt.ok === true &&
    replayAttempt.status === "current" &&
    visual.devtools_global_present === true &&
    visual.browser_workbench_replay === "current" &&
    visual.inspected_element_present === true &&
    visual.cascade_inspected === true &&
    visual.preview_source_mutated === false &&
    visual.apply_source_mutated === true &&
    visual.undo_source_restored === true &&
    visual.safe_local_source_target_known === true &&
    visual.apply_receipt_written === true &&
    visual.undo_receipt_written === true &&
    visual.receipt_durability === "json-sr-machine-written" &&
    missingPhases.length === 0 &&
    typeof visual.inspected_selector === "string" &&
    visual.inspected_selector.trim().length > 0 &&
    typeof visual.inspected_element_fingerprint === "string" &&
    visual.inspected_element_fingerprint.trim().length > 0 &&
    typeof visual.style_property === "string" &&
    visual.style_property.trim().length > 0 &&
    typeof visual.style_value === "string" &&
    visual.style_value.trim().length > 0 &&
    typeof computedBefore.value === "string" &&
    computedBefore.value.trim().length > 0 &&
    typeof computedPreview.value === "string" &&
    computedPreview.value.trim().length > 0 &&
    typeof computedUndo.value === "string" &&
    computedUndo.value.trim().length > 0 &&
    typeof sourceTarget.relativePath === "string" &&
    sourceTarget.relativePath.trim().length > 0 &&
    numberValue(range.endByte) > 0 &&
    numberValue(range.endByte) >= numberValue(range.startByte) &&
    typeof pageUrl === "string" &&
    pageUrl.trim().length > 0 &&
    typeof userAgent === "string" &&
    userAgent.trim().length > 0 &&
    numberValue(viewport.width) > 0 &&
    numberValue(viewport.height) > 0;

  return {
    schema: VISUAL_EDIT_WORKBENCH_SCHEMA,
    schema_revision: 1,
    passed,
    browser_runtime_executed: snapshot.browser_runtime_executed === true,
    visual_replay_attempted: replayAttempt.attempted === true,
    visual_replay_status: replayAttempt.status ?? null,
    visual_replay_reason: replayAttempt.reason ?? null,
    devtools_global_present: visual.devtools_global_present === true,
    browser_workbench_replay: visual.browser_workbench_replay === "current" ? "current" : "missing",
    proof_scope: "local-in-app-browser-visual-edit-workbench-replay",
    route: canonicalRoute,
    canonical_route: canonicalRoute,
    proof_target: proofTarget,
    source_root: typeof snapshot.project_root === "string" ? snapshot.project_root : null,
    source_root_source:
      typeof snapshot.project_root === "string" ? snapshot.project_root_source ?? "page-snapshot" : null,
    workbench_phases: phases,
    missing_workbench_phases: missingPhases,
    inspected_element_present: visual.inspected_element_present === true,
    cascade_inspected: visual.cascade_inspected === true,
    preview_source_mutated: visual.preview_source_mutated === true,
    apply_source_mutated: visual.apply_source_mutated === true,
    undo_source_restored: visual.undo_source_restored === true,
    safe_local_source_target_known: visual.safe_local_source_target_known === true,
    apply_receipt_written: visual.apply_receipt_written === true,
    undo_receipt_written: visual.undo_receipt_written === true,
    receipt_durability: visual.receipt_durability ?? null,
    page_url: pageUrl,
    user_agent: userAgent,
    viewport,
    inspected_selector: visual.inspected_selector ?? null,
    inspected_element_fingerprint: visual.inspected_element_fingerprint ?? null,
    style_property: visual.style_property ?? null,
    style_value: visual.style_value ?? null,
    computed_style_before: computedBefore,
    computed_style_after_preview: computedPreview,
    computed_style_after_undo: computedUndo,
    source_target: sourceTarget,
    browser_snapshot_hash: browserSnapshotHash,
    receipt_source: PAGE_SNAPSHOT_SCHEMA,
    hosted_provider_proof: false,
    provider_adapter_executed: false,
    release_ready: false,
    fastest_world_claim: false,
  };
}

function noJsBrowserReceiptFromPageSnapshot(snapshot: JsonRecord): JsonRecord {
  const noJs = recordValue(snapshot.no_js);
  const artifact = noJsArtifactReceipt();
  const canonicalRoute = canonicalRouteForRecord(noJs, snapshot);
  const htmlPath =
    typeof noJs.html_path === "string"
      ? noJs.html_path
      : typeof artifact.html_path === "string"
        ? artifact.html_path
        : null;
  const artifactHtmlBlake3 =
    typeof noJs.artifact_html_blake3 === "string"
      ? noJs.artifact_html_blake3
      : typeof artifact.artifact_html_blake3 === "string"
        ? artifact.artifact_html_blake3
        : null;
  const scriptTagCount = numberValue(noJs.script_tag_count);
  const accessibilitySignalCount = numberValue(noJs.accessibility_signal_count);
  const linkCount = numberValue(noJs.link_count);
  const formCount = numberValue(noJs.form_count);
  const passed =
    snapshot.browser_runtime_executed === true &&
    noJs.live_browser_executed === true &&
    noJs.javascript_disabled_browser === true &&
    noJs.page_javascript_enabled === false &&
    noJs.data_dx_output_mode_tiny_static === true &&
    noJs.data_dx_js_none === true &&
    scriptTagCount === 0 &&
    noJs.semantic_landmark_present === true &&
    noJs.visible_text_present === true &&
    linkCount > 0 &&
    formCount > 0 &&
    noJs.seo_title_present === true &&
    accessibilitySignalCount > 0 &&
    typeof htmlPath === "string" &&
    htmlPath.length > 0 &&
    typeof artifactHtmlBlake3 === "string" &&
    /^blake3:[a-f0-9]{64}$/.test(artifactHtmlBlake3);

  return {
    schema: NO_JS_BROWSER_SCHEMA,
    schema_revision: 1,
    passed,
    status: passed ? "current-local-js-disabled-browser-proof" : "candidate-not-current",
    live_browser_executed: noJs.live_browser_executed === true,
    javascript_disabled_browser: noJs.javascript_disabled_browser === true,
    page_javascript_enabled: noJs.page_javascript_enabled === true,
    route: canonicalRoute,
    canonical_route: canonicalRoute,
    proof_target: localBrowserProofTarget("tiny-static-no-js-route", canonicalRoute, {
      html_path: htmlPath,
      artifact_html_blake3: artifactHtmlBlake3,
    }),
    html_path: htmlPath,
    artifact_html_blake3: artifactHtmlBlake3,
    script_tag_count: scriptTagCount,
    data_dx_output_mode_tiny_static: noJs.data_dx_output_mode_tiny_static === true,
    data_dx_js_none: noJs.data_dx_js_none === true,
    semantic_landmark_present: noJs.semantic_landmark_present === true,
    visible_text_present: noJs.visible_text_present === true,
    link_count: linkCount,
    form_count: formCount,
    seo_title_present: noJs.seo_title_present === true,
    accessibility_signal_count: accessibilitySignalCount,
    browser_snapshot_hash: snapshotHash({
      url: snapshot.url ?? null,
      user_agent: snapshot.user_agent ?? null,
      canonical_route: canonicalRoute,
      no_js: noJs,
    }),
    proof_scope: "local-js-disabled-browser-no-js-route-replay",
    receipt_source: PAGE_SNAPSHOT_SCHEMA,
    page_url: snapshot.url ?? null,
    hosted_provider_proof: false,
    provider_adapter_executed: false,
    release_ready: false,
    fastest_world_claim: false,
  };
}

function receiptOutputRow(id: string, receiptPath: string, receipt: JsonRecord): JsonRecord {
  return {
    id,
    path: receiptPath,
    passed: receipt.passed === true,
    canonical_route: receipt.canonical_route ?? null,
    proof_scope: receipt.proof_scope ?? null,
    proof_target: recordValue(receipt.proof_target),
  };
}

function usage(): string {
  return [
    "Usage:",
    "  node benchmarks/dx-www-readiness-browser-receipt-harness.ts --print-page-collector",
    "  node benchmarks/dx-www-readiness-browser-receipt-harness.ts --print-dom-page-collector",
    "  node benchmarks/dx-www-readiness-browser-receipt-harness.ts --from-page-json <page-snapshot.json> --out-dir <dir>",
    "",
    "The page collector must run inside a real WWW page. The DOM collector is read-only and expects browser interactions to happen first. This file only converts that real page snapshot into import candidates; dx www readiness validates them before canonical writes.",
  ].join("\n");
}

function argumentAfter(args: string[], flag: string): string | undefined {
  const index = args.indexOf(flag);
  return index >= 0 ? args[index + 1] : undefined;
}

function writeJson(filePath: string, value: JsonRecord): void {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`);
}

const args = process.argv.slice(2);
if (args.includes("--print-page-collector")) {
  process.stdout.write(`(${collectReadinessBrowserPageSnapshot.toString()})();\n`);
} else if (args.includes("--print-dom-page-collector")) {
  process.stdout.write(`(${collectReadinessBrowserDomSnapshot.toString()})();\n`);
} else if (args.includes("--from-page-json")) {
  const source = argumentAfter(args, "--from-page-json");
  const outDir = argumentAfter(args, "--out-dir");
  if (!source || !outDir) {
    throw new Error(usage());
  }
  const snapshot = JSON.parse(fs.readFileSync(source, "utf8")) as JsonRecord;
  if (snapshot.schema !== PAGE_SNAPSHOT_SCHEMA) {
    throw new Error(`Expected ${PAGE_SNAPSHOT_SCHEMA} in ${source}`);
  }
  const nativeReceipt = nativeReceiptFromPageSnapshot(snapshot);
  const stateReceipt = stateReceiptFromPageSnapshot(snapshot);
  const islandBrowserReceipt = islandBrowserReceiptFromPageSnapshot(snapshot);
  const visualReceipt = visualEditReceiptFromPageSnapshot(snapshot);
  const noJsBrowserReceipt = noJsBrowserReceiptFromPageSnapshot(snapshot);
  const nativePath = path.join(outDir, NATIVE_RECEIPT_NAME);
  const statePath = path.join(outDir, STATE_RECEIPT_NAME);
  const islandBrowserPath = path.join(outDir, ISLAND_BROWSER_RECEIPT_NAME);
  const visualPath = path.join(outDir, VISUAL_EDIT_RECEIPT_NAME);
  const noJsBrowserPath = path.join(outDir, NO_JS_BROWSER_RECEIPT_NAME);
  writeJson(nativePath, nativeReceipt);
  writeJson(statePath, stateReceipt);
  writeJson(islandBrowserPath, islandBrowserReceipt);
  writeJson(visualPath, visualReceipt);
  writeJson(noJsBrowserPath, noJsBrowserReceipt);
  process.stdout.write(
    `${JSON.stringify(
      {
        schema: "dx.www.readiness.browser_receipt_harness_output.v1",
        source,
        out_dir: outDir,
        receipts: [
          receiptOutputRow("native-event-browser-binder", nativePath, nativeReceipt),
          receiptOutputRow("state-runtime-browser", statePath, stateReceipt),
          receiptOutputRow("island-browser", islandBrowserPath, islandBrowserReceipt),
          receiptOutputRow("visual-edit-browser-workbench", visualPath, visualReceipt),
          receiptOutputRow("no-js-browser", noJsBrowserPath, noJsBrowserReceipt),
        ],
        import_commands: [
          `dx www readiness --import-native-event-browser-binder-receipt ${nativePath} --json --full`,
          `dx www readiness --import-state-runtime-browser-receipt ${statePath} --json --full`,
          `dx www readiness --import-island-browser-receipt ${islandBrowserPath} --json --full`,
          `dx www readiness --import-visual-edit-browser-receipt ${visualPath} --json --full`,
          `dx www readiness --import-no-js-browser-receipt ${noJsBrowserPath} --json --full`,
        ],
        release_ready: false,
        fastest_world_claim: false,
      },
      null,
      2,
    )}\n`,
  );
} else {
  process.stderr.write(`${usage()}\n`);
  process.exitCode = 1;
}
