import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import vm from "node:vm";

const root = path.resolve(import.meta.dirname, "..");

function read(relative: string): string {
  const absolute = path.join(root, relative);
  assert.ok(fs.existsSync(absolute), `missing ${relative}`);
  return fs.readFileSync(absolute, "utf8");
}

class FakeCustomEvent {
  type: string;
  detail: unknown;

  constructor(type: string, init: { detail?: unknown } = {}) {
    this.type = type;
    this.detail = init.detail;
  }
}

class FakeElement {
  localName: string;
  value = "";
  checked = false;
  private attributes = new Map<string, string>();
  private listeners = new Map<string, ((event: { preventDefault(): void }) => void)[]>();

  constructor(localName: string, attributes: Record<string, string> = {}) {
    this.localName = localName;
    for (const [name, value] of Object.entries(attributes)) {
      this.setAttribute(name, value);
    }
  }

  getAttribute(name: string): string | null {
    return this.attributes.get(name) ?? null;
  }

  setAttribute(name: string, value: string): void {
    this.attributes.set(name, String(value));
  }

  addEventListener(eventName: string, listener: (event: { preventDefault(): void }) => void): void {
    const existing = this.listeners.get(eventName) ?? [];
    existing.push(listener);
    this.listeners.set(eventName, existing);
  }

  listenerNames(): string[] {
    return Array.from(this.listeners.keys()).sort();
  }

  dispatch(eventName: string): void {
    for (const listener of this.listeners.get(eventName) ?? []) {
      listener({ preventDefault() {} });
    }
  }
}

function extractBinderScript(source: string, descriptorSets: unknown[], supportedEvents: string[]): string {
  const match = source.match(/r#"\(function \(\) \{\r?\n([\s\S]*?)\r?\n\}\)\(\);"#/);
  assert.ok(match, "DOM action binder script template should stay extractable");
  return `(function () {\n${match[1]}\n})();`
    .replace("__DX_DESCRIPTOR_SETS__", JSON.stringify(descriptorSets))
    .replace("__DX_SUPPORTED_DOM_EVENTS__", JSON.stringify(supportedEvents))
    .replace("__DX_SUPPORTED_DOM_EVENT_CATALOG_HASH__", JSON.stringify(`blake3:${"a".repeat(64)}`))
    .replace("__DX_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT__", JSON.stringify("dx.www.readiness.native_event_browser_binder_receipt_contract"))
    .replace("__DX_REACT_STYLE_EVENT_EXAMPLES__", JSON.stringify(["onClick", "onInput", "onPointerMove"]))
    .replace("__DX_DOM_EVENT_EXAMPLES__", JSON.stringify(["click", "input", "pointermove"]))
    .replace(
      "__DX_UNSUPPORTED_EVENT_POLICY__",
      JSON.stringify("diagnose unsupported React-style event attributes without attaching listeners or claiming React synthetic event parity"),
    );
}

function nativeDomEvents(): string[] {
  const source = read("core/src/delivery/dom_events.rs");
  const match = source.match(/NATIVE_DOM_EVENT_NAMES:\s*&\[&str\]\s*=\s*&\[\r?\n([\s\S]*?)\r?\n\];/);
  assert.ok(match, "native DOM event catalog should stay source-owned and extractable");
  return Array.from(match[1].matchAll(/"([^"]+)"/g), ([, eventName]) => eventName);
}

test("generated DOM action binder replays React-style native events without unsupported listeners", () => {
  const source = read("dx-www/src/cli/app_router_execution/source_render_parts/client_component.rs");
  assert.ok(
    !nativeDomEvents().includes("once_per"),
    "source-owned native event catalog must not include non-browser pseudo events",
  );
  const button = new FakeElement("button");
  const anchor = new FakeElement("a");
  const input = new FakeElement("input", { name: "title", type: "text" });
  input.value = "hello";
  const windowEvents: FakeCustomEvent[] = [];
  const runtimeDispatches: { eventId: string; payload: unknown }[] = [];
  const document = {
    querySelectorAll(tag: string) {
      return [button, anchor, input].filter((element) => element.localName === tag);
    },
  };
  const context = {
    window: {
      __DX_STATE_GRAPH_RUNTIME__: {
        dispatch(eventId: string, payload: unknown) {
          runtimeDispatches.push({ eventId, payload });
          return { ok: true, eventId };
        },
      },
      dispatchEvent(event: FakeCustomEvent) {
        windowEvents.push(event);
        return true;
      },
    },
    document,
    CustomEvent: FakeCustomEvent,
  };
  const descriptors = [
    {
      source_path: "app/page.tsx",
      descriptors: [
        {
          kind: "button-event",
          tag: "button",
          event_attributes: [
            { name: "onClick", dom_event: "click" },
            { name: "onPointerMove", dom_event: "pointermove" },
            { name: "onMagicGesture", dom_event: "magicgesture" },
            { name: "onOnce_per", dom_event: "once_per" },
          ],
          event_slots: [
            { id: "slot-click", event: "click" },
            { id: "slot-pointer", event: "pointermove" },
            { id: "slot-once-per", event: "once_per" },
          ],
        },
        {
          kind: "button-event",
          tag: "a",
          event_attributes: [{ name: "onClick", dom_event: "click" }],
          event_slots: [{ id: "slot-anchor-dblclick", event: "dblclick" }],
        },
        {
          kind: "input-binding",
          tag: "input",
          name: "title",
          type: "text",
          event_attributes: [{ name: "onInput", dom_event: "input" }],
          event_slots: [{ id: "slot-input", event: "input" }],
        },
      ],
    },
  ];

  vm.runInNewContext(extractBinderScript(source, descriptors, nativeDomEvents()), context);

  assert.deepEqual(button.listenerNames(), ["click", "pointermove"]);
  assert.deepEqual(anchor.listenerNames(), ["click", "dblclick"]);
  assert.deepEqual(input.listenerNames(), ["input"]);
  assert.ok(!button.listenerNames().includes("magicgesture"));
  assert.ok(!button.listenerNames().includes("once_per"));
  button.dispatch("click");
  button.dispatch("pointermove");
  button.dispatch("once_per");
  anchor.dispatch("click");
  input.dispatch("input");
  assert.deepEqual(
    runtimeDispatches.map((dispatch) => dispatch.eventId),
    ["slot-click", "slot-pointer", "slot-input"],
  );
  assert.equal(anchor.getAttribute("data-dx-state-runtime-dispatch"), "no-event-slot");
  assert.equal(button.getAttribute("data-dx-state-runtime-dispatch"), "dispatched");
  assert.equal(input.getAttribute("data-dx-state-runtime-dispatch"), "dispatched");
  assert.equal(
    windowEvents.filter((event) => event.type === "dx:dom-action-preview").length,
    4,
  );
  assert.equal(
    windowEvents.filter((event) => event.type === "dx:state-runtime-dispatch").length,
    3,
  );
  assert.ok(context.window.__DX_DOM_ACTION_BINDER__);
  assert.equal(context.window.__DX_DOM_ACTION_BINDER__.contract.catalog_source, "core/src/delivery/dom_events.rs::NATIVE_DOM_EVENT_NAMES");
  assert.match(context.window.__DX_DOM_ACTION_BINDER__.contract.catalog_hash, /^blake3:[a-f0-9]{64}$/);
  assert.equal(context.window.__DX_DOM_ACTION_BINDER__.contract.supported_event_count, nativeDomEvents().length);
  assert.deepEqual(
    JSON.parse(JSON.stringify(context.window.__DX_DOM_ACTION_BINDER__.contract.react_style_event_examples)),
    ["onClick", "onInput", "onPointerMove"],
  );
  assert.deepEqual(
    JSON.parse(JSON.stringify(context.window.__DX_DOM_ACTION_BINDER__.contract.dom_event_examples)),
    ["click", "input", "pointermove"],
  );
  assert.equal(
    context.window.__DX_DOM_ACTION_BINDER__.contract.unsupported_event_policy,
    "diagnose unsupported React-style event attributes without attaching listeners or claiming React synthetic event parity",
  );
  const snapshot = JSON.parse(JSON.stringify(context.window.__DX_DOM_ACTION_BINDER__.getSnapshot()));
  assert.deepEqual(snapshot, {
    schema: "dx.tsx.domActionBinderSnapshot",
    schema_revision: 1,
    readiness_receipt_contract: "dx.www.readiness.native_event_browser_binder_receipt_contract",
    descriptor_count: 3,
    bound_count: 3,
    binder_global_present: true,
    node_modules_required: false,
    catalog_source: "core/src/delivery/dom_events.rs::NATIVE_DOM_EVENT_NAMES",
    catalog_hash: `blake3:${"a".repeat(64)}`,
    supported_event_count: nativeDomEvents().length,
    listener_events: ["click", "dblclick", "input", "pointermove"],
    unsupported_listener_attached: false,
    preview_event_count: 4,
    state_dispatch_count: 3,
    interaction_class_application_count: 0,
    state_runtime_bridge: "preview-events-dispatch-to-state-runtime",
    release_ready: false,
    fastest_world_claim: false,
    full_react_hook_parity: false,
    full_react_event_parity: false,
    proof_scope: "local-browser-native-event-binder-replay-required",
  });
});

test("generated DOM action binder applies literal event interaction classes without state dispatch", () => {
  const source = read("dx-www/src/cli/app_router_execution/source_render_parts/client_component.rs");
  const button = new FakeElement("button");
  const windowEvents: FakeCustomEvent[] = [];
  const runtimeDispatches: { eventId: string; payload: unknown }[] = [];
  const document = {
    querySelectorAll(tag: string) {
      return [button].filter((element) => element.localName === tag);
    },
  };
  const context = {
    window: {
      __DX_STATE_GRAPH_RUNTIME__: {
        dispatch(eventId: string, payload: unknown) {
          runtimeDispatches.push({ eventId, payload });
          return { ok: true, eventId };
        },
      },
      dispatchEvent(event: FakeCustomEvent) {
        windowEvents.push(event);
        return true;
      },
    },
    document,
    CustomEvent: FakeCustomEvent,
  };
  const descriptors = [
    {
      source_path: "app/page.tsx",
      descriptors: [
        {
          kind: "button-event",
          tag: "button",
          event_attributes: [
            {
              name: "onClick",
              dom_event: "click",
              handler_kind: "interaction-classes",
              interaction_classes: ["scale-up", "bg-accent"],
            },
          ],
          event_slots: [],
        },
      ],
    },
  ];

  vm.runInNewContext(extractBinderScript(source, descriptors, nativeDomEvents()), context);
  button.dispatch("click");

  assert.equal(button.getAttribute("class"), "scale-up bg-accent");
  assert.equal(button.getAttribute("data-dx-interaction-class-applied"), "scale-up bg-accent");
  assert.equal(button.getAttribute("data-dx-state-runtime-dispatch"), "not-needed");
  assert.deepEqual(runtimeDispatches, []);
  assert.equal(
    windowEvents.filter((event) => event.type === "dx:dom-action-preview").length,
    1,
  );
  assert.equal(
    windowEvents.filter((event) => event.type === "dx:state-runtime-dispatch").length,
    0,
  );
  assert.equal(
    context.window.__DX_DOM_ACTION_BINDER__.getSnapshot().interaction_class_application_count,
    1,
  );
});

test("route-unit report keeps unsupported React-style events diagnostic-only", () => {
  const routeUnit = read("core/src/delivery/route_unit.rs");

  assert.match(routeUnit, /unsupported_react_event_attributes/);
  assert.match(routeUnit, /unsupported React-style event attribute/);
  assert.match(routeUnit, /no listener is attached and React synthetic event parity is not claimed/);

  const semantics = read("dx-www/src/cli/app_router_semantics.rs");
  assert.match(semantics, /react_event_diagnostic_code/);
  assert.match(semantics, /dx\.dom-event\.unsupported/);
  assert.match(semantics, /listener_attached/);
  assert.match(semantics, /adapter_boundary_required/);
  assert.match(semantics, /failure_reason/);
  assert.match(semantics, /not-in-native-dom-event-catalog/);
});
