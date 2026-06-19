import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import vm from "node:vm";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const stateRuntimePath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_router_execution",
  "state_runtime.rs",
);

function read(filePath) {
  assert.ok(fs.existsSync(filePath), `missing ${path.relative(root, filePath)}`);
  return fs.readFileSync(filePath, "utf8");
}

class DomVmCustomEvent {
  type;
  detail;

  constructor(type, init = {}) {
    this.type = type;
    this.detail = init.detail;
  }
}

class DomVmElement {
  localName;
  attributes = new Map();
  textContent = "";
  value = "";
  checked = false;

  constructor(localName, attributes = {}) {
    this.localName = localName;
    for (const [name, value] of Object.entries(attributes)) {
      this.setAttribute(name, value);
    }
  }

  getAttribute(name) {
    return this.attributes.get(name) ?? null;
  }

  setAttribute(name, value) {
    this.attributes.set(name, String(value));
  }

  removeAttribute(name) {
    this.attributes.delete(name);
  }
}

function extractStateRuntimeScript(source, program) {
  const match = source.match(/r#"<script[^>]*>\r?\n([\s\S]*?)\r?\n<\/script>"#/);
  assert.ok(match, "state runtime script template should stay extractable");
  return match[1]
    .replaceAll("{{", "{")
    .replaceAll("}}", "}")
    .replace("{program_json}", JSON.stringify(program));
}

function createDocument(elements) {
  const events = [];
  return {
    events,
    querySelectorAll(selector) {
      const match = selector.match(/^\[(.+?)(~=|=)"(.*)"\]$/);
      assert.ok(match, `unexpected selector ${selector}`);
      const [, attribute, operator, expected] = match;
      return elements.filter((element) => {
        const actual = element.getAttribute(attribute);
        if (actual == null) return false;
        return operator === "~="
          ? actual.split(/\s+/).includes(expected)
          : actual === expected;
      });
    },
    dispatchEvent(event) {
      events.push(event);
      return true;
    },
  };
}

function browserSlotEvidence(evidence, name, kind = "state") {
  const collection =
    kind === "derived" ? evidence.dom.derived_slots : evidence.dom.state_slots;
  const match = collection.find((slot) => slot.name === name);
  assert.ok(match, `missing ${kind} browser evidence for ${name}`);
  return match;
}

test("App Router state runtime lowers safe string literals and functional toggles", () => {
  const stateRuntime = read(stateRuntimePath);

  assert.match(stateRuntime, /read_complete_js_string_literal/);
  assert.match(stateRuntime, /exact_setter_call_arguments/);
  assert.match(stateRuntime, /infer_functional_toggle/);
  assert.match(stateRuntime, /state_runtime_lowers_safe_string_literal_setters/);
  assert.match(stateRuntime, /state_runtime_lowers_functional_boolean_toggles/);
  assert.match(stateRuntime, /state_runtime_rejects_non_exact_use_state_handler_bodies/);
  assert.match(stateRuntime, /unsupported_react_api_policy/);
  assert.match(stateRuntime, /react_api_shim_executed": false/);
  assert.match(stateRuntime, /full_react_hook_runtime": false/);
  assert.match(stateRuntime, /node_modules_required": false/);
  assert.match(stateRuntime, /stateRuntimeBrowserEvidence/);
  assert.match(stateRuntime, /getBrowserEvidence/);
});

test("App Router state runtime replays DOM reflection in an in-memory DOM VM", () => {
  const stateRuntime = read(stateRuntimePath);
  const program = {
    schema: "dx.tsx.stateRuntime",
    schema_revision: 1,
    route: "/",
    lowering_status: "runtime-emitted",
    counts: { slots: 4, derived_slots: 1, events: 0 },
    slots: [
      { name: "count", initial_value: 1 },
      { name: "name", initial_value: "Initial" },
      { name: "open", initial_value: false },
      { name: "counterStore.count", initial_value: 1 },
    ],
    derived_slots: [
      {
        name: "double",
        dependencies: ["count"],
        evaluation_status: "safe-runtime-lowered",
        operation: { kind: "binary", dependency: "count", operator: "*", right: 2 },
      },
    ],
    events: [],
    effects: [],
    effect_scheduler: { effects: [] },
    state_dom_reflection: {
      event: "dx:state-dom-reflection",
      derived_event: "dx:derived-state-slot",
    },
  };
  const countText = new DomVmElement("span", { "data-dx-state-read": "count" });
  const nameInput = new DomVmElement("input", { "data-dx-state-value": "name" });
  const openInput = new DomVmElement("input", { "data-dx-state-checked": "open" });
  const openButton = new DomVmElement("button", {
    "data-dx-state-disabled": "open",
    "data-dx-state-aria-expanded": "open",
  });
  const counterStoreText = new DomVmElement("span", {
    "data-dx-state-read": "counterStore.count",
  });
  const counterStoreInput = new DomVmElement("input", {
    "data-dx-state-value": "counterStore.count",
  });
  const doubleText = new DomVmElement("span", { "data-dx-state-read": "double" });
  const document = createDocument([
    countText,
    nameInput,
    openInput,
    openButton,
    counterStoreText,
    counterStoreInput,
    doubleText,
  ]);
  const context = {
    window: {},
    document,
    CustomEvent: DomVmCustomEvent,
  };
  context.window = context;

  vm.runInNewContext(extractStateRuntimeScript(stateRuntime, program), context);
  const runtime = context.window.__DX_STATE_GRAPH_RUNTIME__;

  assert.equal(countText.textContent, "1");
  assert.equal(nameInput.value, "Initial");
  assert.equal(openInput.checked, false);
  assert.equal(openButton.disabled, false);
  assert.equal(openButton.getAttribute("aria-expanded"), "false");
  assert.equal(counterStoreText.textContent, "1");
  assert.equal(counterStoreInput.value, "1");
  assert.equal(doubleText.textContent, "2");
  const readyEvent = document.events.find((event) => event.type === "dx:state-runtime-ready");
  assert.equal(readyEvent.detail.browser_evidence.schema, "dx.tsx.stateRuntimeBrowserEvidence");
  assert.equal(readyEvent.detail.browser_evidence.reason, "ready");
  assert.equal(
    readyEvent.detail.browser_evidence.unsupported_runtime_claims
      .native_listener_binding_complete,
    false,
  );
  const initialEvidence = runtime.getBrowserEvidence("test-initial");
  assert.deepEqual({ ...initialEvidence.snapshot }, {
    count: 1,
    name: "Initial",
    open: false,
    "counterStore.count": 1,
    double: 2,
  });
  assert.equal(initialEvidence.event_contracts.runtime_ready, "dx:state-runtime-ready");
  assert.equal(initialEvidence.event_contracts.state_dom_reflection, "dx:state-dom-reflection");
  assert.equal(browserSlotEvidence(initialEvidence, "count").targets.text_content, 1);
  assert.equal(browserSlotEvidence(initialEvidence, "name").targets.form_control_value, 1);
  assert.equal(browserSlotEvidence(initialEvidence, "open").targets.form_control_checked, 1);
  assert.equal(
    browserSlotEvidence(initialEvidence, "open").targets.boolean_attributes[
      "data-dx-state-disabled"
    ],
    1,
  );
  assert.equal(
    browserSlotEvidence(initialEvidence, "open").targets.aria_attributes["aria-expanded"],
    1,
  );
  assert.equal(browserSlotEvidence(initialEvidence, "counterStore.count").targets.text_content, 1);
  assert.equal(
    browserSlotEvidence(initialEvidence, "counterStore.count").targets.form_control_value,
    1,
  );
  assert.equal(browserSlotEvidence(initialEvidence, "double", "derived").targets.text_content, 1);
  assert.equal(runtime.setSlot("count", 3), true);
  assert.equal(countText.textContent, "3");
  assert.equal(doubleText.textContent, "6");
  assert.equal(runtime.setSlot("name", "Ada"), true);
  assert.equal(nameInput.value, "Ada");
  assert.equal(runtime.setSlot("open", true), true);
  assert.equal(openInput.checked, true);
  assert.equal(openButton.disabled, true);
  assert.equal(openButton.getAttribute("disabled"), "");
  assert.equal(openButton.getAttribute("aria-expanded"), "true");
  assert.equal(runtime.setSlot("counterStore.count", 4), true);
  assert.equal(counterStoreText.textContent, "4");
  assert.equal(counterStoreInput.value, "4");
  const updatedEvidence = runtime.getBrowserEvidence("after-browser-mutations");
  assert.equal(updatedEvidence.snapshot.count, 3);
  assert.equal(updatedEvidence.snapshot.name, "Ada");
  assert.equal(updatedEvidence.snapshot.open, true);
  assert.equal(updatedEvidence.snapshot["counterStore.count"], 4);
  assert.equal(updatedEvidence.snapshot.double, 6);
  assert.equal(browserSlotEvidence(updatedEvidence, "counterStore.count").value, 4);
  assert.equal(browserSlotEvidence(updatedEvidence, "open").value, true);
  assert.ok(
    document.events.some((event) => event.type === "dx:state-dom-reflection"),
    "runtime should dispatch real state DOM reflection events",
  );
  assert.ok(
    document.events.some((event) => event.type === "dx:derived-state-slot"),
    "runtime should dispatch derived state reflection events",
  );
});

test("App Router state runtime dispatches action metadata and dependent effects without unsupported React shims", () => {
  const stateRuntime = read(stateRuntimePath);
  assert.doesNotMatch(
    stateRuntime,
    /\b(?:function|const|let|var)\s+use(?:Effect|LayoutEffect|InsertionEffect|Reducer|Context|Transition|Optimistic|ActionState)\b/,
    "state runtime must not install unsupported React hook shims",
  );
  const program = {
    schema: "dx.tsx.stateRuntime",
    schema_revision: 1,
    route: "/counter",
    lowering_status: "runtime-emitted",
    counts: { slots: 1, derived_slots: 1, events: 4, effects: 1, server_actions: 1 },
    slots: [{ name: "count", initial_value: 0 }],
    derived_slots: [
      {
        name: "ready",
        dependencies: ["count"],
        evaluation_status: "safe-runtime-lowered",
        operation: { kind: "comparison", dependency: "count", operator: ">=", right: 1 },
      },
    ],
    events: [
      {
        id: "event:add",
        source_path: "components/Counter.tsx",
        event: "click",
        handler: "() => setCount(count + 1)",
        action: null,
        operation: { kind: "add", slot: "count", delta: 1 },
      },
      {
        id: "event:save",
        source_path: "components/Counter.tsx",
        event: "submit",
        handler: "() => saveCount()",
        action: "saveCount",
        operation: { kind: "server-action", action: "saveCount" },
      },
      {
        id: "event:unsafe",
        source_path: "components/Counter.tsx",
        event: "click",
        handler: "() => deleteEverything()",
        action: "deleteEverything",
        operation: { kind: "server-action", action: "deleteEverything" },
      },
      {
        id: "event:reducer-dispatch",
        source_path: "components/Counter.tsx",
        event: "click",
        handler: "() => dispatch({ type: 'increment' })",
        action: null,
        operation: { kind: "useReducer-dispatch", slot: "count" },
      },
    ],
    effects: [],
    effect_scheduler: {
      effects: [
        {
          id: "effect:count",
          source_path: "components/Counter.tsx",
          dependencies: ["count"],
          order: 0,
        },
      ],
    },
    server_actions: [
      {
        id: "action:saveCount",
        action: "saveCount",
        source_path: "components/Counter.tsx",
        import_source: "../actions",
        event_id: "event:save",
      },
    ],
    state_dom_reflection: {
      event: "dx:state-dom-reflection",
      derived_event: "dx:derived-state-slot",
    },
  };
  const countText = new DomVmElement("span", { "data-dx-state-read": "count" });
  const readyText = new DomVmElement("span", { "data-dx-state-read": "ready" });
  const document = createDocument([countText, readyText]);
  const context = {
    window: {},
    document,
    CustomEvent: DomVmCustomEvent,
  };
  context.window = context;

  vm.runInNewContext(extractStateRuntimeScript(stateRuntime, program), context);
  const runtime = context.window.__DX_STATE_GRAPH_RUNTIME__;

  assert.equal(runtime.dispatch("event:add", { kind: "click" }).ok, true);
  assert.equal(countText.textContent, "1");
  assert.equal(readyText.textContent, "true");
  assert.ok(
    document.events.some(
      (event) =>
        event.type === "dx:effect-scheduled" &&
        event.detail.changed_slot === "count" &&
        event.detail.executed === false &&
        event.detail.full_react_effect_body_execution === false,
    ),
    "state changes should schedule dependent effect records without executing React effect bodies",
  );

  const actionResult = runtime.dispatch("event:save", { kind: "submit" });
  assert.equal(actionResult.ok, true);
  assert.equal(actionResult.state_result.action, "saveCount");
  assert.equal(actionResult.state_result.server_action_invoked, false);
  const actionEvents = document.events.filter((event) => event.type === "dx:server-action-edge");
  assert.equal(actionEvents.length, 1);
  assert.equal(actionEvents[0].detail.action, "saveCount");
  assert.equal(actionEvents[0].detail.edge_status, "matched-source-owned-edge");
  assert.equal(actionEvents[0].detail.edge_id, "action:saveCount");
  assert.equal(actionEvents[0].detail.import_source, "../actions");
  assert.equal(actionEvents[0].detail.status, "metadata-only");
  assert.equal(actionEvents[0].detail.source_owned, true);
  assert.equal(actionEvents[0].detail.server_action_invoked, false);
  assert.equal(actionEvents[0].detail.node_modules_required, false);
  assert.equal(actionResult.state_result.edge.id, "action:saveCount");
  assert.equal(actionResult.state_result.edge.import_source, "../actions");

  const unsafeResult = runtime.dispatch("event:unsafe", { kind: "click" });
  assert.equal(unsafeResult.ok, false);
  assert.equal(
    unsafeResult.state_result.reason,
    "unmatched-source-owned-server-action-edge",
  );
  assert.equal(unsafeResult.state_result.status, "diagnostic-only");
  assert.equal(unsafeResult.state_result.server_action_invoked, false);
  const unsafeEvents = document.events.filter(
    (event) =>
      event.type === "dx:server-action-edge" &&
      event.detail.action === "deleteEverything",
  );
  assert.equal(unsafeEvents.length, 1);
  assert.equal(unsafeEvents[0].detail.status, "diagnostic-only");
  assert.equal(unsafeEvents[0].detail.edge_status, "unmatched-action-metadata");
  assert.equal(
    unsafeEvents[0].detail.diagnostic_code,
    "dx.state-runtime.action.unmatched-source-edge",
  );
  assert.equal(unsafeEvents[0].detail.server_action_invoked, false);

  const reducerResult = runtime.dispatch("event:reducer-dispatch", {
    type: "click",
  });
  assert.equal(reducerResult.ok, false);
  assert.equal(reducerResult.state_result.status, "diagnostic-only");
  assert.equal(
    reducerResult.state_result.diagnostic_code,
    "dx.state-runtime.operation.unsupported-react-like-operation",
  );
  assert.equal(reducerResult.state_result.full_react_hook_runtime, false);
  assert.equal(reducerResult.state_result.react_api_shim_executed, false);
  assert.equal(reducerResult.state_result.adapter_boundary_required, true);
  const operationDiagnostics = document.events.filter(
    (event) => event.type === "dx:state-runtime-diagnostic",
  );
  assert.equal(operationDiagnostics.length, 1);
  assert.equal(operationDiagnostics[0].detail.operation_kind, "useReducer-dispatch");
  assert.equal(operationDiagnostics[0].detail.status, "diagnostic-only");
  assert.equal(
    operationDiagnostics[0].detail.reason,
    "unsupported-react-like-state-operation",
  );
  assert.equal(operationDiagnostics[0].detail.full_react_hook_runtime, false);
  assert.equal(operationDiagnostics[0].detail.react_api_shim_executed, false);
  assert.equal(operationDiagnostics[0].detail.adapter_boundary_required, true);
});
