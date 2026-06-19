import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");

function read(relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function expectAll(source: string, markers: string[]): void {
  for (const marker of markers) {
    assert.match(source, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
}

test("release-readiness reactivity model receipt is source-owned, serializer-backed, and honest", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");

  expectAll(readiness, [
    "READINESS_REACTIVITY_MODEL_SCHEMA",
    "dx.www.readiness.reactivity_model",
    "READINESS_REACTIVITY_MODEL_RECEIPT_CONTRACT",
    "dx.www.readiness.reactivity_model_receipt_contract",
    "READINESS_REACTIVITY_MODEL_RECEIPT",
    ".dx/receipts/readiness/reactivity-model-latest.json",
    "READINESS_REACTIVITY_MODEL_RECEIPT_SR",
    ".dx/receipts/readiness/reactivity-model-latest.sr",
    "READINESS_REACTIVITY_MODEL_RECEIPT_MACHINE",
    ".dx/serializer/receipts-readiness-reactivity-model-latest.machine",
    "write_readiness_reactivity_model_receipt",
    "readiness_reactivity_model_receipt_is_current",
    "readiness_reactivity_model_sr_fields",
    "readiness_reactivity_source_check",
    "source-owned-reactivity-model-foundation-current",
    "source-owned-reactivity-model-receipt-current-browser-proof-needed",
    "local-source-owned-reactivity-model-foundation",
    "reactivity_model_receipt_current",
    "reactivity-model-receipt-not-current",
    "node --test benchmarks/dx-www-readiness-reactivity-receipts.test.ts",
    "readiness_release_ready",
    "browser_proof_status",
    "foundation-not-release-proof",
    "source-guarded-not-real-browser-proof",
    "source-owned reactivity model foundation only; browser replay and hosted breadth remain separate release-readiness gates",
    "This receipt validates the source-owned DX-native reactivity model only; React hooks are adapter-only inventory unless exact DX state-slot lowering proves a safe compatibility case, and this receipt does not run a browser, execute arbitrary React hooks, or claim hosted provider parity.",
  ]);

  expectAll(readiness, [
    "state()",
    "derived()",
    "effect()",
    "action()",
    "React useState exact DX-state-slot adapter syntax only",
    "react_hook_inventory_api",
    "exact_lowering_required",
    "state_graph_has_exact_use_state_lowering",
    "unsupported_unlowerable_use_state_diagnostic",
    "adapter_boundary_required_when_unlowerable",
    "dx.react-hook.useState.missing-exact-state-slot",
    "useEffect",
    "useReducer",
    "useContext",
    "useTransition",
    "react_api_shim_executed",
    "full_react_hook_runtime",
    "browser_runtime_executed",
    "hosted_provider_proof",
    "readiness_release_ready",
    "browser_proof_status",
    "foundation-not-release-proof",
    "fastest_world_claim",
    "READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT",
    "READINESS_STATE_RUNTIME_BROWSER_RECEIPT",
    "READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR",
    "READINESS_STATE_RUNTIME_BROWSER_RECEIPT_MACHINE",
  ]);
});

test("release-readiness reactivity receipt watches the real DX-native runtime markers", () => {
  const stateRuntime = read("dx-www/src/cli/app_router_execution/state_runtime.rs");
  const sourceRender = read("dx-www/src/cli/app_router_execution/source_render.rs");
  const clientComponent = read(
    "dx-www/src/cli/app_router_execution/source_render_parts/client_component.rs",
  );
  const staticMarkup = read(
    "dx-www/src/cli/app_router_execution/source_render_parts/static_markup.rs",
  );

  expectAll(stateRuntime, [
    "dx.tsx.dxNativeReactivityCapabilities",
    "state()",
    "derived()",
    "effect()",
    "action()",
    "react_hook_policy",
    "DX-native state() slots, app-global store slots, and explicit state graph slots",
    "__DX_STATE_GRAPH_RUNTIME__",
    "reflectStateSlotToDom",
    "setRuntimeSlot",
    "refreshDerivedSlots",
    "scheduleEffectsForState",
    "dx:state-dom-reflection",
    "dx:derived-state-slot",
    "dx:effect-scheduled",
    "dx:state-runtime-diagnostic",
    "dx.state-runtime.operation.unsupported-react-like-operation",
    "dx.react-hook.useEffect.adapter-boundary-required",
    "react_api_shim_executed: false",
    "adapter_boundary_required: true",
    "full_react_hook_runtime: false",
  ]);

  const semantics = read("dx-www/src/cli/app_router_semantics.rs");
  expectAll(semantics, [
    "state_graph_has_exact_use_state_lowering",
    "source_events_have_exact_use_state_lowering",
    "count_use_state_setter_calls",
    "event_handler_has_exact_use_state_setter_operation",
    "is_lowerable_use_state_setter_argument",
    "dx.react-hook.useState.exact-dx-state-slot-lowering",
    "dx.react-hook.useState.missing-exact-state-slot",
    "useState is compatibility sugar only when the compiler can lower it exactly into DX state slots.",
    '"adapter_boundary_required": status != "compatibility-lowered"',
    "react-effect-boundary",
    "react-semantic-boundary",
    "callback bodies and cleanup are not executed with hidden React semantics",
  ]);

  expectAll(clientComponent, [
    "dispatchDomActionPreviewToStateRuntime",
    "state_runtime_dispatcher",
    "dx:state-runtime-dispatch",
    "full_react_hook_parity: false",
    "react_synthetic_events: false",
    "generated-dom-action-binder",
  ]);

  expectAll(staticMarkup, [
    "data-dx-state-read",
    "data-dx-state-value",
    "data-dx-state-checked",
    "data-dx-state-aria-*",
    "state_slot_visible_to_document",
    "matches!(slot.scope, DxStateScope::Global)",
    "The generated runtime hook updates only elements carrying compiler-owned data-dx-state-* markers.",
  ]);

  expectAll(sourceRender, [
    "static_dom_reflects_exact_global_store_reads_across_component_files",
    "components/counter-badge.tsx",
    "source_path: \"lib/stores/counter.ts\".to_string()",
    "name: \"counterStore.count\".to_string()",
    "scope: DxStateScope::Global",
    "assert_eq!(reflection_plan[\"reflection_count\"], 2)",
    "data-dx-state-read=\"counterStore.count\"",
    "data-dx-state-value=\"counterStore.count\"",
  ]);
});

test("Agent context exposes reactivity receipt status and blockers", () => {
  const agentContext = read("dx-www/src/cli/agent_context.rs");

  expectAll(agentContext, [
    "REACTIVITY_MODEL_RECEIPT",
    "REACTIVITY_MODEL_RECEIPT_SCHEMA",
    "REACTIVITY_MODEL_RECEIPT_SR",
    "REACTIVITY_MODEL_RECEIPT_MACHINE",
    "reactivity_model_receipt_status",
    "reactivity_model_receipt_is_current",
    "\"reactivity_model\"",
    "\"reactivity-model\"",
    "reactivity-model-receipt-missing",
    "readiness-reactivity-model-machine-contract-missing",
    "Source-owned DX-native reactivity release-readiness proof is missing or stale",
    "Source-owned DX-native reactivity release-readiness proof is not serializer-backed yet",
    "Browser state runtime replay remains a separate gate",
    "local-source-owned-reactivity-model-foundation",
    "useState(exact-dx-state-slot-only)",
    "compatibility_lowering_api",
    "exact_lowering_required",
    "use_state_lowering_rule",
    "unsupported_unlowerable_use_state_diagnostic",
    "adapter_boundary_required_when_unlowerable",
    "browser_proof_status",
    "foundation-not-release-proof",
    "source-guarded-not-real-browser-proof",
  ]);
});
