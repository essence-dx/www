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

function rustStringLiterals(source: string): string[] {
  return [...source.matchAll(/"([^"]+)"/g)].map((match) => match[1]);
}

function rustStaticStringArray(source: string, name: string): string[] {
  const body = functionBody(source, name);
  assert.match(body, /&\[/, `${name} must return a static array`);
  return rustStringLiterals(body);
}

function functionBody(source: string, name: string): string {
  const start = source.indexOf(`fn ${name}(`);
  assert.notEqual(start, -1, `${name} must exist`);
  const nextFunction = source.indexOf("\nfn ", start + 1);
  return source.slice(start, nextFunction === -1 ? undefined : nextFunction);
}

test("WWW islands ABI exposes camelCase directives without adopting Astro colon syntax", () => {
  const clientIsland = read("core/src/delivery/client_island.rs");

  const directives = rustStaticStringArray(clientIsland, "client_island_directive_names");
  for (const directive of ["clientLoad", "clientVisible", "clientIdle", "clientOnly"]) {
    assert.ok(directives.includes(directive), `${directive} must be a supported WWW island directive`);
  }
  assert.equal(directives.some((directive) => directive.includes(":")), false);

  assert.deepEqual(
    rustStaticStringArray(clientIsland, "unsupported_client_island_directive_syntax"),
    ["client:load", "client:visible", "client:idle", "client:only"],
  );

  const capabilities = functionBody(clientIsland, "react_client_island_abi_capabilities");
  assert.match(capabilities, /directive_style:\s*"camelCase-jsx-props"/);
  assert.match(capabilities, /directive_style_id:\s*"camelCase-jsx-props"/);
  assert.match(capabilities, /no_js_fallback_required:\s*true/);
  assert.match(capabilities, /full_react_hydration:\s*false/);
  assert.match(capabilities, /readiness_release_ready:\s*false/);
  assert.match(capabilities, /preview-only/);
  assert.match(capabilities, /executable framework adapter receipts/);
});

test("release-readiness islands ABI durable receipt stays source-owned and honest", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");

  expectAll(readiness, [
    "READINESS_ISLAND_ABI_RECEIPT_CONTRACT",
    "dx.www.readiness.island_abi_receipt_contract",
    "READINESS_ISLAND_ABI_RECEIPT",
    ".dx/receipts/readiness/island-abi-latest.json",
    "READINESS_ISLAND_ABI_RECEIPT_SR",
    ".dx/receipts/readiness/island-abi-latest.sr",
    "READINESS_ISLAND_ABI_RECEIPT_MACHINE",
    ".dx/serializer/receipts-readiness-island-abi-latest.machine",
    "READINESS_ISLAND_BROWSER_RECEIPT_CONTRACT",
    "dx.www.readiness.island_browser_receipt_contract",
    "READINESS_ISLAND_BROWSER_RECEIPT",
    ".dx/receipts/readiness/island-browser-latest.json",
    "READINESS_ISLAND_BROWSER_RECEIPT_SR",
    ".dx/receipts/readiness/island-browser-latest.sr",
    "READINESS_ISLAND_BROWSER_RECEIPT_MACHINE",
    ".dx/serializer/receipts-readiness-island-browser-latest.machine",
    "import_readiness_island_browser_receipt",
    "readiness_island_browser_sr_fields",
    "island_browser_receipt_is_current",
    "write_readiness_island_abi_receipt",
    "readiness_island_abi_sr_fields",
    "readiness_island_abi_stale_reason",
    "island_abi_stale_reason_from_receipt",
    "island_abi_missing_directives",
    "island-abi-receipt-missing",
    "island-abi-source-coverage-incomplete",
    "island-abi-browser-adapter-proof-missing",
    "island-browser-receipt-missing",
    "island-browser-directive-coverage-incomplete",
    "island-browser-runtime-event-replay-incomplete",
    "island_abi_stale_reason",
    "island_browser_stale_reason",
    "island_gate_stale_reason",
    "island_abi_receipt_current",
    "island_browser_receipt_current",
    "source-owned-island-abi-and-browser-replay-current-hosted-proof-needed",
    "source-owned-island-abi-receipt-current-hosted-proof-needed",
    "source-owned-island-abi-foundation-current",
    "source-owned-island-abi-foundation-stale",
    "local-source-owned-island-abi-foundation",
    "clientLoad/clientVisible/clientIdle/clientOnly directives",
  ]);

  const islandAbi = functionBody(readiness, "readiness_island_abi");
  expectAll(islandAbi, [
    '"schema": "dx.www.readiness.island_abi"',
    '"readiness_release_ready": false',
    '"browser_proof_status": "foundation-not-release-proof"',
    '"attribute_style": "camelCase"',
    '"directive_style_id": "camelCase-jsx-props"',
    '"clientLoad"',
    '"clientVisible"',
    '"clientIdle"',
    '"clientOnly"',
    '"unsupported_directive_syntax": ["client:load", "client:visible", "client:idle", "client:only"]',
    '"adapter_boundary": "source-owned client islands by default; explicit framework adapters only through clientOnly"',
    '"no_js_fallback": "required for every island route before the ABI can claim release readiness"',
  ]);

  const writer = functionBody(readiness, "write_readiness_island_abi_receipt");
  expectAll(writer, [
    '"schema": READINESS_ISLAND_ABI_RECEIPT_CONTRACT',
    '"id": "islands"',
    '"island_abi_schema": "dx.www.readiness.island_abi"',
    '"source_owned": true',
    '"directive_style_id": "camelCase-jsx-props"',
    '"clientLoad"',
    '"clientVisible"',
    '"clientIdle"',
    '"clientOnly"',
    '"client:load"',
    '"client:visible"',
    '"client:idle"',
    '"client:only"',
    '"browser_runtime_executed": false',
    '"hosted_provider_proof": false',
    '"provider_adapter_executed": false',
    '"release_ready": false',
    '"fastest_world_claim": false',
    '"proof_scope": "local-source-owned-island-abi-foundation"',
    "write_sr_artifact(",
    "READINESS_ISLAND_ABI_RECEIPT_SR",
    "readiness_island_abi_sr_fields(&receipt)",
    "serializer_provenance_json(project, &sr_artifact)",
    "write_readiness_json_receipt(",
    "READINESS_ISLAND_ABI_RECEIPT",
    '"json_read_model_path": READINESS_ISLAND_ABI_RECEIPT',
    '"serializer_receipt_path": READINESS_ISLAND_ABI_RECEIPT_SR',
    '"machine_path": relative_artifact_path(project, &sr_artifact.machine)',
    '"machine_path_within_root": artifact_path_within_root(project, &sr_artifact.machine)',
  ]);

  const srFields = functionBody(readiness, "readiness_island_abi_sr_fields");
  expectAll(srFields, [
    'sr_string(READINESS_ISLAND_ABI_RECEIPT_CONTRACT)',
    '"island_abi_schema"',
    'sr_string("dx.www.readiness.island_abi")',
    '("release_ready", sr_bool(false))',
    '("fastest_world_claim", sr_bool(false))',
    '("source_owned", sr_bool(true))',
    '("browser_runtime_executed", sr_bool(false))',
    '("hosted_provider_proof", sr_bool(false))',
    '("provider_adapter_executed", sr_bool(false))',
    '("directive_style_id", sr_string("camelCase-jsx-props"))',
    '"directives"',
    'sr_string("clientLoad;clientVisible;clientIdle;clientOnly")',
    '"core_directives"',
    '"supported_directives"',
    "clientMedia;clientInteraction",
    '"additional_supported_directives"',
    '"release_core_directives"',
    '"route_unit_proof_metadata"',
    "DxRouteReceipt.client_island_abi",
    '"route_streaming_island_metadata"',
    '"unsupported_directive_syntax"',
    'sr_string("client:load;client:visible;client:idle;client:only")',
    '"proof_scope"',
    "local-source-owned-island-abi-foundation",
    "no browser/provider adapter execution is claimed by this source-owned islands ABI receipt",
  ]);

  const gateStatus = functionBody(readiness, "readiness_gate_status_for_project");
  expectAll(gateStatus, [
    "island_abi_receipt_current",
    "island_browser_receipt_current",
    '"island_abi_receipt_current": island_abi_receipt_current',
    '"island_browser_receipt_current": island_browser_receipt_current',
    "let island_abi_status = if island_abi_receipt_current && island_browser_receipt_current",
    '"source-owned-island-abi-and-browser-replay-current-hosted-proof-needed"',
    '"source-owned-island-abi-receipt-current-hosted-proof-needed"',
    '"manifest-directive-and-abi-foundation"',
    '"id": "islands"',
    '"status": island_abi_status',
    '"island_abi_receipt": READINESS_ISLAND_ABI_RECEIPT',
    '"island_browser_receipt": READINESS_ISLAND_BROWSER_RECEIPT',
    '"stale_reason": island_gate_stale_reason',
    '"per-directive browser proof, no-JS fallback proof, and explicit framework adapter receipts"',
  ]);
});

test("camelCase directives drive source-owned strategies and remain separate from normal props", () => {
  const clientIsland = read("core/src/delivery/client_island.rs");

  const props = functionBody(clientIsland, "component_props_from_doc");
  assert.match(props, /is_client_island_directive\(&attribute\.name\)/);
  assert.match(props, /is_unsupported_client_island_directive_syntax\(&attribute\.name\)/);
  assert.match(props, /return None;/);

  const directives = functionBody(clientIsland, "component_directives_from_doc");
  assert.match(directives, /is_client_island_directive\(&attribute\.name\)/);
  assert.match(directives, /DxReactClientIslandDirective/);
  assert.doesNotMatch(
    directives,
    /is_unsupported_client_island_directive_syntax\(&attribute\.name\)[\s\S]*DxReactClientIslandDirective/,
    "Astro-style colon directives must not become supported WWW island directives",
  );

  const unsupported = functionBody(clientIsland, "is_unsupported_client_island_directive_syntax");
  assert.match(unsupported, /unsupported_client_island_directive_syntax\(\)\.contains\(&name\)/);

  const strategy = functionBody(clientIsland, "hydration_strategy");
  assert.match(strategy, /directive\.name == "clientOnly"[\s\S]*"framework-adapter-client-only"/);
  assert.match(strategy, /directive\.name == "clientVisible"[\s\S]*"visible"/);
  assert.match(strategy, /directive\.name == "clientIdle"[\s\S]*"idle"/);
  assert.match(strategy, /directive\.name == "clientLoad"[\s\S]*"load"/);
  assert.match(strategy, /directive\.name == "clientMedia"[\s\S]*"media-recognized-not-executed"/);
  assert.match(
    strategy,
    /directive\.name == "clientInteraction"[\s\S]*"interaction-recognized-not-executed"/,
  );
});

test("source render and route-unit proof keep islands source-owned with no-JS fallback boundaries", () => {
  const appRoute = read("core/src/delivery/app_route.rs");
  const contract = read("core/src/delivery/contract.rs");
  const sourceRender = read("dx-www/src/cli/app_router_execution/source_render.rs");
  const clientComponent = read(
    "dx-www/src/cli/app_router_execution/source_render_parts/client_component.rs",
  );
  const routeUnit = read("core/src/delivery/route_unit.rs");

  assert.match(appRoute, /DxReactClientIslandInput/);
  assert.match(appRoute, /compile_react_client_islands\(DxReactClientIslandInput/);
  assert.match(appRoute, /react_client_island_abi_capabilities\(\)/);
  assert.match(appRoute, /pub directive_style_id: String/);
  assert.match(appRoute, /pub directives: Vec<String>/);
  assert.match(appRoute, /pub hydration_strategy: String/);
  assert.match(appRoute, /pub no_js_fallback_required: bool/);
  assert.match(appRoute, /pub browser_proof_status: String/);
  assert.match(appRoute, /pub framework_adapter: String/);
  assert.match(appRoute, /"preview-only"/);
  assert.match(appRoute, /"local-source-owned-island-abi-foundation"/);

  assert.match(contract, /pub struct DxRouteClientIslandAbiReceipt/);
  assert.match(contract, /pub client_island_abi: Option<DxRouteClientIslandAbiReceipt>/);
  assert.match(contract, /pub directive_style_id: String/);
  assert.match(contract, /pub core_directives: Vec<String>/);
  assert.match(contract, /pub no_js_fallback_required: bool/);
  assert.match(contract, /pub browser_proof_status: String/);

  assert.match(sourceRender, /"client_islands": client_islands/);
  assert.match(sourceRender, /"Bind generated state\/event runtime operations to the real rendered DOM for client islands\."/);

  const buildOutput = read("dx-www/src/cli/app_router_build_output.rs");
  assert.match(buildOutput, /data-dx-client-island-bridge="source-owned"/);
  assert.match(buildOutput, /data-dx-client-island-abi="camelCase"/);
  assert.match(buildOutput, /data-dx-no-js-fallback="preserved"/);
  assert.ok((buildOutput.match(/data-dx-no-js-fallback="preserved"/g) ?? []).length >= 2);
  assert.match(buildOutput, /data-dx-client-only-adapters="preview-only"/);
  assert.match(buildOutput, /data-dx-browser-proof="not-claimed"/);
  assert.match(buildOutput, /data-dx-browser-runtime-proof="not-claimed"/);
  assert.ok((buildOutput.match(/data-dx-browser-runtime-proof="not-claimed"/g) ?? []).length >= 2);
  assert.match(buildOutput, /data-dx-provider-runtime-proof="not-claimed"/);
  assert.ok((buildOutput.match(/data-dx-provider-runtime-proof="not-claimed"/g) ?? []).length >= 2);
  assert.match(buildOutput, /data-dx-provider-adapters="not-executed"/);
  assert.match(buildOutput, /data-dx-provider-adapter="not-executed"/);
  assert.match(buildOutput, /data-dx-island-hydration-strategy=/);
  assert.match(buildOutput, /data-dx-island-directives=/);
  assert.match(buildOutput, /data-dx-island-abi-schema="dx\.react\.clientIsland\.abi"/);
  assert.match(buildOutput, /data-dx-island-directive-style="camelCase-jsx-props"/);
  assert.match(buildOutput, /data-dx-client-only-adapter=/);
  assert.match(buildOutput, /data-dx-client-load=/);
  assert.match(buildOutput, /data-dx-client-visible=/);
  assert.match(buildOutput, /data-dx-client-idle=/);
  assert.match(buildOutput, /data-dx-client-media=/);
  assert.match(buildOutput, /data-dx-client-interaction=/);
  assert.match(buildOutput, /data-dx-client-media-support="recognized-not-executed"/);
  assert.match(buildOutput, /data-dx-client-interaction-support="recognized-not-executed"/);
  assert.match(buildOutput, /fn island_directive_names/);
  assert.match(buildOutput, /fn island_client_only_adapter_status/);
  assert.match(buildOutput, /fn island_client_load_status/);
  assert.match(buildOutput, /fn island_client_visible_status/);
  assert.match(buildOutput, /fn island_client_idle_status/);
  assert.match(buildOutput, /fn island_client_media_status/);
  assert.match(buildOutput, /fn island_client_interaction_status/);
  assert.match(buildOutput, /"clientOnly"[\s\S]*"preview-only"/);
  assert.match(buildOutput, /"clientLoad"[\s\S]*"observed"/);
  assert.match(buildOutput, /"clientVisible"[\s\S]*"observed"/);
  assert.match(buildOutput, /"clientIdle"[\s\S]*"observed"/);
  assert.match(buildOutput, /"clientMedia"[\s\S]*"recognized-not-executed"/);
  assert.match(buildOutput, /"clientInteraction"[\s\S]*"recognized-not-executed"/);
  assert.match(buildOutput, /"not-requested"/);

  assert.match(clientComponent, /"node_modules_required": false/);
  assert.match(clientComponent, /"full_react_hydration": false/);
  assert.match(clientComponent, /"react_synthetic_events": false/);
  assert.match(clientComponent, /fn client_island_abi_metadata/);
  assert.match(clientComponent, /"abi": client_island_abi_metadata\(\)/);
  assert.match(clientComponent, /react_client_island_abi_capabilities\(\)/);
  assert.match(clientComponent, /"directive_style": capabilities\.directive_style/);
  assert.match(clientComponent, /"supported_directives": capabilities\.supported_directives/);
  assert.match(clientComponent, /"unsupported_directive_syntax": capabilities\.unsupported_directive_syntax/);
  assert.match(clientComponent, /"framework_adapters": "preview-only"/);
  assert.match(clientComponent, /"Does not execute arbitrary client components, effects, context providers, or full React hydration\."/);

  assert.match(
    routeUnit,
    /let lowercase_html = fallback\.html\.to_ascii_lowercase\(\);[\s\S]*let script_tag_count = lowercase_html\.matches\("<script"\)\.count\(\);/,
  );
  assert.match(routeUnit, /let no_js_capable = delivery_mode == DxDeliveryMode::Static[\s\S]*script_tag_count == 0[\s\S]*meaningful_html;/);
  assert.match(routeUnit, /astro_parity_status: "not_yet_claimed"\.to_string\(\)/);
  assert.match(routeUnit, /client_island_abi: route_client_island_abi_receipt\(streaming\)/);
  assert.match(routeUnit, /fn route_client_island_abi_receipt/);
  assert.match(routeUnit, /directive_style_id: "camelCase-jsx-props"\.to_string\(\)/);
  assert.match(routeUnit, /core_directives: vec!\[/);
  assert.match(routeUnit, /"clientLoad"\.to_string\(\)/);
  assert.match(routeUnit, /"clientVisible"\.to_string\(\)/);
  assert.match(routeUnit, /"clientIdle"\.to_string\(\)/);
  assert.match(routeUnit, /"clientOnly"\.to_string\(\)/);
  assert.match(routeUnit, /browser_proof_status: "foundation-not-release-proof"\.to_string\(\)/);
});
