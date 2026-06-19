import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");

function read(relative: string): string {
  const absolute = path.join(root, relative);
  assert.ok(fs.existsSync(absolute), `missing ${relative}`);
  return fs.readFileSync(absolute, "utf8");
}

test("WWW exposes framework-owned global stores instead of template-only state packages", () => {
  const contract = read("core/src/delivery/contract.rs");
  const routeUnit = read("core/src/delivery/route_unit.rs");
  const globalStore = read("core/src/delivery/global_store.rs");
  const coreTests = read("core/src/delivery/tests.rs");
  const cliSources = read("dx-www/src/cli/mod_parts/cli_core_impl.rs");
  const sourceRender = read("dx-www/src/cli/app_router_execution/source_render.rs");
  const stateRuntime = read("dx-www/src/cli/app_router_execution/state_runtime.rs");
  const staticMarkup = read(
    "dx-www/src/cli/app_router_execution/source_render_parts/static_markup.rs",
  );
  const templateSources = read("dx-www/src/cli/default_template_sources.rs");
  const templateProbe = read("examples/template/components/state-runtime-probe.tsx");

  assert.match(contract, /pub global_stores: Vec<DxGlobalStore>/);
  assert.match(contract, /pub struct DxGlobalStore/);
  assert.match(contract, /pub struct DxGlobalStoreAction/);
  assert.match(globalStore, /pub\(super\) fn global_store_bindings/);
  assert.match(globalStore, /state\(/);
  assert.match(globalStore, /derived\(/);
  assert.match(globalStore, /effect\(/);
  assert.match(globalStore, /action\(/);
  assert.match(routeUnit, /global_store_bindings\(&source\.source_path, &source\.source\)/);
  assert.match(routeUnit, /DxStateScope::Global/);
  assert.match(coreTests, /route_unit_links_imported_global_store_actions_across_sources/);
  assert.match(coreTests, /components\/CounterPanel\.tsx/);
  assert.match(coreTests, /lib\/stores\/counter\.ts/);
  assert.match(coreTests, /slot\.name == "counterStore\.count" && slot\.scope == DxStateScope::Global/);
  assert.match(coreTests, /event\.action\.as_deref\(\) == Some\("counterStore\.increment"\)/);
  assert.match(stateRuntime, /dx-global-store-action/);
  assert.match(stateRuntime, /global_store_actions/);
  assert.match(staticMarkup, /state_slot_visible_to_document/);
  assert.match(staticMarkup, /matches!\(slot\.scope, DxStateScope::Global\)/);
  assert.match(staticMarkup, /expression_refs_state_slot\(expression, &slot\.name\)/);
  assert.match(sourceRender, /static_dom_reflects_exact_global_store_reads_across_component_files/);
  assert.match(sourceRender, /components\/counter-badge\.tsx/);
  assert.match(sourceRender, /source_path: "lib\/stores\/counter\.ts"\.to_string\(\)/);
  assert.match(sourceRender, /name: "counterStore\.count"\.to_string\(\)/);
  assert.match(sourceRender, /scope: DxStateScope::Global/);
  assert.match(sourceRender, /assert_eq!\(reflection_plan\["reflection_count"\], 2\)/);
  assert.match(sourceRender, /data-dx-state-read="counterStore\.count"/);
  assert.match(sourceRender, /data-dx-state-value="counterStore\.count"/);
  assert.match(cliSources, /"components", "lib\/stores"/);
  assert.match(templateSources, /lib\/stores\/counter\.ts/);
  assert.match(templateProbe, /stateRuntimeProbeStore = store/);
  assert.doesNotMatch(templateProbe, /from "react"/);
  assert.match(templateProbe, /counterStore\.increment\(counterStore\)/);
});

test("literal React-style events lower to interaction classes while expression handlers stay logic", () => {
  const routeUnit = read("core/src/delivery/route_unit.rs");
  const staticExpression = read("dx-www/src/cli/app_router_execution/source_render_parts/static_expression.rs");
  const clientComponent = read("dx-www/src/cli/app_router_execution/source_render_parts/client_component.rs");
  const templateProbe = read("examples/template/components/state-runtime-probe.tsx");

  assert.match(routeUnit, /attribute\.expression\.is_none\(\) && attribute\.value\.is_some\(\)/);
  assert.match(staticExpression, /data-dx-on-\{suffix\}-class/);
  assert.match(staticExpression, /safe_interaction_class_value/);
  assert.match(clientComponent, /function interactionClassesFor/);
  assert.match(clientComponent, /function applyInteractionClasses/);
  assert.match(clientComponent, /handler_kind": handler_kind/);
  assert.match(templateProbe, /onClick=\{.+stateRuntimeProbeStore\.increment/s);
  assert.match(templateProbe, /onClick="scale-up bg-accent"/);
});

test("WWW motion points at DX Style animation and group features without hardcoded presets", () => {
  const staticExpression = read("dx-www/src/cli/app_router_execution/source_render_parts/static_expression.rs");
  const dxStyleTool = read("dx-www/src/cli/public_framework_tools/dx_style.rs");
  const styleAnimation = read("related-crates/style/src/core/animation/mod.rs");
  const groupParser = read("related-crates/style/src/parser/mod.rs");
  const templateProbe = read("examples/template/components/state-runtime-probe.tsx");

  assert.match(staticExpression, /data-dx-motion="dx-style"/);
  assert.match(staticExpression, /data-dx-motion-engine="dx-style-animation-generator"/);
  assert.match(staticExpression, /data-dx-motion-group-syntax/);
  assert.match(dxStyleTool, /motion=\\"/);
  assert.match(dxStyleTool, /dxMotion=\\"/);
  assert.match(dxStyleTool, /data-dx-motion-class=\\"/);
  assert.match(styleAnimation, /fn named_animation_css/);
  assert.match(styleAnimation, /animation-pop\(opacity:0\.\.1,scale:0\.96\.\.1\)/);
  assert.match(styleAnimation, /from\(/);
  assert.match(styleAnimation, /via\(/);
  assert.match(styleAnimation, /to\(/);
  assert.match(groupParser, /expand_grouping_into/);
  assert.match(templateProbe, /motion="button\(transition-transform duration-200\) animation-pop\(opacity:0\.\.1,scale:0\.96\.\.1\)"/);
});

test("native DOM event catalog remains MDN-sized and source-owned", () => {
  const domEvents = read("core/src/delivery/dom_events.rs");
  const readiness = read("dx-www/src/cli/readiness.rs");
  const events = Array.from(
    domEvents.matchAll(/NATIVE_DOM_EVENT_NAMES:\s*&\[&str\]\s*=\s*&\[[\s\S]*?\];/g),
  );
  assert.equal(events.length, 1);
  const names = Array.from(events[0][0].matchAll(/"([^"]+)"/g), ([, name]) => name);

  assert.ok(names.length >= 250, `expected broad MDN-style native event catalog, got ${names.length}`);
  assert.ok(names.includes("click"));
  assert.ok(names.includes("input"));
  assert.ok(names.includes("pointermove"));
  assert.match(domEvents, /react_style_event_attribute_to_dom_event/);
  assert.match(readiness, /mdn_browser_compat_event_freshness/);
  assert.match(readiness, /target\/mdn-browser-compat-data/);
});
