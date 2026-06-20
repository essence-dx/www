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

function sliceBetween(source: string, startNeedle: string, endNeedle: string): string {
  const start = source.indexOf(startNeedle);
  assert.notEqual(start, -1, `${startNeedle} must exist`);
  const end = source.indexOf(endNeedle, start + startNeedle.length);
  assert.notEqual(end, -1, `${endNeedle} must follow ${startNeedle}`);
  return source.slice(start, end);
}

function rustConstI64(source: string, name: string): number {
  const match = source.match(new RegExp(`const ${name}: i64 = (\\d+);`));
  assert.ok(match, `${name} must be a numeric i64 const`);
  return Number(match[1]);
}

test("tiny-static proof stays zero-JS and does not claim Astro parity", () => {
  const routeUnit = read("core/src/delivery/route_unit.rs");
  const contract = read("core/src/delivery/contract.rs");
  const appRoute = read("core/src/delivery/app_route.rs");
  const starterPage = read("examples/template/app/page.tsx");
  const starterStyles = read("examples/template/styles/globals.css");
  const readiness = read("dx-www/src/cli/readiness.rs");
  const proof = sliceBetween(routeUnit, "fn tiny_static_route_proof(", "fn runtime_candidate(");
  const noJsReceipt = sliceBetween(
    readiness,
    "fn write_readiness_no_js_artifact_receipt(",
    "fn write_readiness_primitive_proof_receipt(",
  );
  const noJsSrFields = sliceBetween(
    readiness,
    "fn readiness_no_js_artifact_sr_fields(",
    "fn readiness_proof_graph_sr_fields(",
  );

  assert.match(
    contract,
    /pub struct DxTinyStaticRouteProof[\s\S]*total_public_bytes[\s\S]*script_tag_count[\s\S]*runtime_required[\s\S]*no_js_capable[\s\S]*meaningful_html[\s\S]*semantic_landmark_present[\s\S]*link_count[\s\S]*form_count[\s\S]*seo_title_present[\s\S]*accessibility_signal_count[\s\S]*browser_api_required[\s\S]*astro_parity_status/,
    "tiny-static route proof should expose byte, JS, semantic HTML, browser API, and Astro parity boundaries",
  );
  assert.match(
    proof,
    /let lowercase_html = fallback\.html\.to_ascii_lowercase\(\);[\s\S]*let script_tag_count = lowercase_html\.matches\("<script"\)\.count\(\);/,
    "tiny-static proof should count script tags from emitted fallback HTML case-insensitively",
  );
  assert.match(
    proof,
    /let browser_api_required = delivery_mode != DxDeliveryMode::Static[\s\S]*!state\.slots\.is_empty\(\)[\s\S]*!state\.event_slots\.is_empty\(\)[\s\S]*!state\.effects\.is_empty\(\)[\s\S]*!state\.server_actions\.is_empty\(\)[\s\S]*streaming_enabled/,
    "tiny-static proof should reject browser APIs, state/event/effect/action runtime, and streaming requirements",
  );
  assert.match(
    proof,
    /let no_js_capable = delivery_mode == DxDeliveryMode::Static[\s\S]*&& !browser_api_required[\s\S]*&& script_tag_count == 0[\s\S]*&& meaningful_html;/,
    "tiny-static no-JS capability should require static mode, zero script tags, no browser APIs, and meaningful HTML",
  );
  expectAll(proof, [
    '"tiny-static".to_string()',
    '"none".to_string()',
    'astro_parity_status: "not_yet_claimed".to_string()',
  ]);
  expectAll(starterPage, [
    'href="/state-runtime"',
    'className="starter-form"',
    'action="/state-runtime"',
    'method="get"',
    'name="note"',
    'type="submit"',
  ]);
  expectAll(starterStyles, [
    ".starter-link",
    ".starter-form",
    ".starter-form-row",
    "hsl(var(--foreground))",
    "hsl(var(--surface))",
  ]);
  expectAll(appRoute, ['data-dx-output-mode="tiny-static"', 'data-dx-js="none"']);
  expectAll(readiness, [
    "foundation-wired-proof-needed",
    "same-machine-throughput-receipt-current-payload-paint-proof-needed",
    "same-machine-throughput-and-no-js-browser-current-paint-proof-needed",
    "same-machine throughput raceboard receipt, Lighthouse paint receipts, and Astro tiny-static payload parity",
    "JS-disabled browser receipt, Lighthouse JS-enabled/static-build paint receipts, and Astro tiny-static payload parity on the same machine",
    "source-only local tiny-static contract; no live Astro payload/paint/throughput replay receipt yet",
    "source-only HTML/CSS no-JS proof; not live Astro payload/paint/throughput parity",
    "links_forms_seo_accessibility_fact_status",
    "astro_parity_claimed=false",
    "live_astro_parity_receipt=missing",
  ]);
  assert.match(
    noJsReceipt,
    /let route_unit_tiny_static_proof = route_unit[\s\S]*pointer\("\/runtime_report\/tiny_static_route_proof"\)/,
    "no-JS artifact receipt should read the full route-unit tiny-static proof instead of a single boolean",
  );
  assert.match(
    noJsReceipt,
    /let route_unit_no_js_capable_current = route_unit_no_js_capable == Some\(true\)[\s\S]*route_unit_script_tag_count == Some\(0\)[\s\S]*route_unit_runtime_required == Some\(false\)[\s\S]*route_unit_browser_api_required == Some\(false\)[\s\S]*route_unit_output_mode == Some\("tiny-static"\)[\s\S]*route_unit_js == Some\("none"\)/,
    "no-JS artifact receipt should require the route-unit proof to agree on JS, scripts, runtime, browser API, and output mode",
  );
  expectAll(noJsReceipt, [
    "readiness_no_js_public_js_artifacts",
    "readiness_no_js_public_output_root",
    "readiness_no_js_public_js_artifact",
    "readiness_decoded_precompressed_artifact_path",
    "path_has_public_js_extension",
    "html_semantic_landmark_present",
    "html_accessibility_signal_count",
    '"semantic_landmark_present": semantic_landmark_present',
    '"link_count": link_count',
    '"form_count": form_count',
    '"seo_title_present": seo_title_present',
    '"accessibility_signal_count": accessibility_signal_count',
    '"link_form_navigation_proof_current": link_form_navigation_proof_current',
    '"route_unit_link_form_navigation_current": route_unit_link_form_navigation_current',
    '"html_css_links_forms_seo_accessibility_current": html_css_links_forms_seo_accessibility_current',
    '"route_unit_output_mode": route_unit_output_mode',
    '"route_unit_js": route_unit_js',
    '"route_unit_script_tag_count": route_unit_script_tag_count',
    '"route_unit_runtime_required": route_unit_runtime_required',
    '"route_unit_browser_api_required": route_unit_browser_api_required',
    '"route_unit_semantic_landmark_present": route_unit_semantic_landmark_present',
    '"route_unit_link_count": route_unit_link_count',
    '"route_unit_form_count": route_unit_form_count',
    '"route_unit_seo_title_present": route_unit_seo_title_present',
    '"route_unit_accessibility_signal_count": route_unit_accessibility_signal_count',
    '"route_unit_no_js_proof_current": route_unit_no_js_capable_current',
    '"links_forms_seo_accessibility_fact_status": links_forms_seo_accessibility_fact_status',
    "artifact-facts-current-not-browser-proof",
    "artifact-facts-incomplete-not-browser-proof",
    '"public_js_artifact_count": public_js_artifact_count',
    '"public_js_artifacts": public_js_artifacts',
    '"artifact_html_blake3": artifact_html_blake3',
  ]);
  expectAll(noJsSrFields, [
    '"route_unit_output_mode"',
    '"route_unit_js"',
    '"route_unit_script_tag_count"',
    '"route_unit_runtime_required"',
    '"route_unit_browser_api_required"',
    '"semantic_landmark_present"',
    '"link_count"',
    '"form_count"',
    '"seo_title_present"',
    '"accessibility_signal_count"',
    '"route_unit_semantic_landmark_present"',
    '"route_unit_link_count"',
    '"route_unit_form_count"',
    '"route_unit_seo_title_present"',
    '"route_unit_accessibility_signal_count"',
    '"route_unit_no_js_proof_current"',
    '"link_form_navigation_proof_current"',
    '"route_unit_link_form_navigation_current"',
    '"html_css_links_forms_seo_accessibility_current"',
    '"public_js_artifact_count"',
    '"artifact_html_blake3"',
  ]);
  assert.match(
    noJsReceipt,
    /walkdir::WalkDir::new\(&output_root\)[\s\S]*readiness_no_js_public_js_artifact\(project, &output_root, entry\.path\(\)\)/,
    "no-JS artifact receipt should recursively scan the public output root for shipped JS assets",
  );
  assert.match(
    noJsReceipt,
    /readiness_bundle_evidence_only_path\(&relative_to_output\)[\s\S]*readiness_decoded_precompressed_artifact_path\(path\)[\s\S]*path_has_public_js_extension\(&decoded_path\)/,
    "no-JS artifact receipt should ignore evidence paths and detect precompressed JS chunks by decoded extension",
  );
  assert.match(
    noJsReceipt,
    /&& public_js_artifact_count == 0/,
    "no-JS artifact receipt should fail if deployable route-local JS artifacts are present",
  );
});

test("Astro tiny-static raceboard remains an explicit unclaimed global speed gap", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");
  const scoreBreakdown = sliceBetween(
    readiness,
    "fn readiness_score_breakdown()",
    "fn readiness_delivery_tiers()",
  );
  const deliveryTiers = sliceBetween(
    readiness,
    "fn readiness_delivery_tiers()",
    "fn readiness_native_event_catalog(",
  );
  const currentHonestScore = rustConstI64(readiness, "READINESS_CURRENT_HONEST_SCORE");
  const targetScore = rustConstI64(readiness, "READINESS_TARGET_SCORE");

  assert.ok(
    currentHonestScore <= targetScore,
    "release-readiness score must not exceed the local target while Astro tiny-static raceboard proof is missing",
  );
  expectAll(scoreBreakdown, [
    '"fastest_world_claim": false',
    '"remaining_hardening": [',
    "broader Astro tiny-static payload and throughput parity beyond the current same-machine tiny route",
  ]);
  expectAll(deliveryTiers, [
    '"name": "static/no-JS"',
    '"output_mode": "tiny-static"',
    '"browser_runtime": false',
    '"browser_js_budget_bytes": 0',
    '"public_route_packet": false',
    '"source_route_evidence_only": true',
    '"astro_parity_claimed": false',
    '"live_astro_parity_receipt": "missing"',
    "source-only HTML/CSS no-JS proof; not live Astro payload/paint/throughput parity",
  ]);
  assert.doesNotMatch(
    deliveryTiers,
    /"astro_parity_claimed": true|"live_astro_parity_receipt": "(?!missing")/,
    "static/no-JS tier must not grow an Astro parity claim without a live raceboard receipt",
  );
});

test("tiny-static meaningful HTML proof requires visible content, not tags alone", () => {
  const routeUnit = read("core/src/delivery/route_unit.rs");
  const proof = sliceBetween(routeUnit, "fn tiny_static_route_proof(", "fn runtime_candidate(");
  const meaningfulHelper = sliceBetween(
    routeUnit,
    "fn tiny_static_has_meaningful_html(",
    "fn runtime_candidate(",
  );

  assert.match(
    proof,
    /let meaningful_html = tiny_static_has_meaningful_html\(&fallback\.html\);/,
    "tiny-static no-JS proof should use a helper that can reject empty semantic shells",
  );
  expectAll(proof, [
    "tiny_static_semantic_landmark_present",
    "tiny_static_tag_count",
    "tiny_static_accessibility_signal_count",
    "semantic_landmark_present",
    "link_count",
    "form_count",
    "seo_title_present",
    "accessibility_signal_count",
  ]);
  assert.match(
    meaningfulHelper,
    /let has_semantic_shell = html\.contains\("<main"\)[\s\S]*html_has_visible_text\(html\)/,
    "meaningful HTML should require both semantic shell tags and visible text outside markup",
  );
  assert.match(
    meaningfulHelper,
    /fn html_has_visible_text\(html: &str\) -> bool[\s\S]*in_tag[\s\S]*!ch\.is_whitespace\(\)/,
    "visible content detection should scan text outside tags instead of trusting tag presence",
  );
  assert.doesNotMatch(
    proof,
    /let meaningful_html = fallback\.html\.contains\("<main"\)[\s\S]*fallback\.html\.contains\("<h1"\)/,
    "empty h1/p/a/article/section tags should not be enough to claim no-JS capability",
  );
});

test("deploy partition keeps proof and route evidence out of public runtime bytes", () => {
  const deployAdapter = read("dx-www/src/cli/deploy_adapter_contract.rs");
  const readiness = read("dx-www/src/cli/readiness.rs");
  const uploadPlan = sliceBetween(
    deployAdapter,
    "fn provider_adapter_upload_plan(",
    "fn deploy_artifact_bundle(",
  );
  const bundleDecision = sliceBetween(
    deployAdapter,
    "fn deploy_artifact_bundle(",
    "fn deploy_artifact_evidence_only_path(",
  );
  const evidenceOnly = sliceBetween(
    deployAdapter,
    "fn deploy_artifact_evidence_only_path(",
    "fn deploy_artifact_cache_control(",
  );
  const cacheControl = sliceBetween(
    deployAdapter,
    "fn deploy_artifact_cache_control(",
    "fn deploy_bundle_partition(",
  );
  const partition = sliceBetween(
    deployAdapter,
    "fn bundle_partition_from_upload_plan(",
    "fn write_cache_manifest(",
  );

  assert.match(
    uploadPlan,
    /let bundle = deploy_artifact_bundle\(path, bundle\);[\s\S]*let cache_control = deploy_artifact_cache_control\(cache_control, bundle\);/,
    "every upload-plan artifact should pass through the public/evidence downgrade and cache-control gate",
  );
  assert.match(
    uploadPlan,
    /route\["packet"\][\s\S]*add_artifact\([\s\S]*"route-packet"[\s\S]*"public-runtime"[\s\S]*route\["client_islands_runtime"\][\s\S]*add_artifact\([\s\S]*"client-islands-runtime"[\s\S]*"public-runtime"[\s\S]*route\["server_data"\][\s\S]*add_artifact\(server_data, "server-data", "no-store", "public-runtime"\)/,
    "route artifacts that request public-runtime should still pass through the same downgrade gate",
  );
  assert.match(
    bundleDecision,
    /"public-runtime" if !deploy_artifact_evidence_only_path\(path\) => "public-runtime"[\s\S]*_ => "evidence"/,
    "requested public runtime artifacts should be downgraded when they match evidence-only paths",
  );
  expectAll(evidenceOnly, [
    "deploy_precompressed_source_path(&normalized)",
    'decoded.starts_with(".dx/")',
    'decoded.starts_with(".dx/build-cache/source-routes/")',
    'decoded.ends_with(".sr")',
    'decoded == ".dx/build-cache/deploy-adapter.json"',
    "DX_CLOUD_PROVIDER_ADAPTER_JSON",
    "PROVIDER_ADAPTER_SMOKE_MATRIX_JSON",
    "ROUTE_HANDLER_CONFORMANCE_MATRIX_JSON",
    "SERVER_ACTION_REPLAY_LEDGER_JSON",
    "READINESS_PROOF_GRAPH_RECEIPT",
    "CACHE_MANIFEST_JSON",
    'decoded == ".dx/build-cache/rollback.json"',
    'decoded == ".dx/build-cache/manifest.json"',
    'decoded == "page-graph.json"',
    'decoded.ends_with("/page-graph.json")',
    'decoded.ends_with("/app-router-execution.json")',
    'decoded.ends_with("/client-islands.json")',
    'decoded.ends_with("/streaming-plan.json")',
  ]);
  assert.match(
    cacheControl,
    /if bundle == "evidence"[\s\S]*"no-store"\.to_string\(\)/,
    "evidence artifacts should be no-store even when a caller requested public caching",
  );
  assert.match(
    partition,
    /"separation_enforced": true[\s\S]*"public_runtime_bundle"[\s\S]*"deployable": true[\s\S]*"excludes": \[READINESS_PROOF_GRAPH_RECEIPT, "\.dx\/receipts\/\*\*", "app-router-execution\.json", "deploy-adapter\.json", SERVER_ACTION_REPLAY_LEDGER_JSON\][\s\S]*"evidence_bundle"[\s\S]*"deployable_public_bytes": false[\s\S]*"cache_control": "no-store"/,
    "bundle partition should state that proof receipts and evidence stay outside deployable public bytes",
  );
  expectAll(readiness, [
    '"id": "public-vs-evidence-bundle"',
    "deploy-partition-foundation",
    "local-public-evidence-partition-current-provider-proof-needed",
    "public_runtime_bundle",
    "evidence_bundle",
    "READINESS_BUNDLE_PARTITION_RECEIPT_CONTRACT",
    "READINESS_BUNDLE_PARTITION_RECEIPT_SR",
    "READINESS_BUNDLE_PARTITION_RECEIPT_MACHINE",
    "local-public-evidence-partition-current",
    "bundle_partition_current",
    "readiness_bundle_partition_stale_reason",
    "bundle_partition_stale_reason_from_receipt",
    "bundle_partition_stale_fields",
    "bundle-partition-receipt-missing",
    "bundle-partition-local-contract-incomplete",
    "bundle-partition-hosted-provider-proof-missing",
    "bundle_partition_stale_reason",
    "precompressed_evidence_artifact_count",
    "precompressed_evidence_public_leak_count",
    "precompressed_evidence_path_samples",
    "precompressed_evidence_paths_no_store",
    "readiness_decoded_precompressed_path_str",
    '"bundle_partition_receipt": READINESS_BUNDLE_PARTITION_RECEIPT',
    '"stale_reason": bundle_partition_stale_reason',
    "source-only local deploy contract; no hosted multi-provider evidence-bundle replay receipt yet",
  ]);
});

test("upload-plan fixture proves stale proof paths are downgraded before public partitioning", () => {
  const deployAdapter = read("dx-www/src/cli/deploy_adapter_contract.rs");
  const forcedEvidenceGuard = sliceBetween(
    deployAdapter,
    "fn provider_upload_plan_forces_evidence_only_paths_out_of_public_runtime()",
    "fn provider_upload_plan_marks_precompressed_runtime_headers()",
  );

  expectAll(forcedEvidenceGuard, [
    '"packet": ".dx/receipts/readiness/proof-graph.sr"',
    '"execution_contract": "app/page-graph.json"',
    '"client_islands_runtime": ".dx/build-cache/source-routes/root/client-islands.js"',
    '"path": ".dx/build-cache/source-routes/root/route-unit.json"',
    '"path": ".dx/build-cache/deploy-adapter.json.br"',
    '"path": ".dx/build-cache/cache-manifest.json.gz"',
    '"path": ".dx/receipts/readiness/proof-graph.sr.gz"',
    "artifact_bundle(&upload_plan, \"app/index.html\")",
    "artifact_bundle(&upload_plan, \"app/server-data.json\")",
    "artifact_bundle(&upload_plan, \"chunks/app.mjs\")",
  ]);
  assert.match(
    forcedEvidenceGuard,
    /for path in \[[\s\S]*READINESS_PROOF_GRAPH_RECEIPT[\s\S]*"app\/page-graph\.json"[\s\S]*"source-routes\/root\/client-islands\.js"[\s\S]*"source-routes\/root\/route-unit\.json"[\s\S]*"deploy-adapter\.json\.br"[\s\S]*"cache-manifest\.json\.gz"[\s\S]*"\.dx\/receipts\/readiness\/proof-graph\.sr\.gz"[\s\S]*assert_eq!\(artifact\["bundle"\], "evidence"\);[\s\S]*assert_eq!\(artifact\["cache_control"\], "no-store"\);/,
    "representative proof, page-graph, route evidence, stale runtime paths, and encoded evidence paths should be asserted as evidence/no-store",
  );
  assert.match(
    forcedEvidenceGuard,
    /public_artifacts[\s\S]*\.all\(\|artifact\| \{[\s\S]*!deploy_artifact_evidence_only_path\([\s\S]*artifact\["path"\]\.as_str\(\)\.expect\("public artifact path"\)[\s\S]*\)/,
    "public-runtime partition should reject every path the evidence-only classifier would catch",
  );
});

test("tiny-static deploy route discovery does not invent stale public route data paths", () => {
  const deployAdapter = read("dx-www/src/cli/deploy_adapter_contract.rs");
  const appRouterBuild = read("dx-www/src/cli/app_router_build_command.rs");
  const readiness = read("dx-www/src/cli/readiness.rs");
  const deployRoutes = sliceBetween(
    deployAdapter,
    "fn deploy_routes(output_dir: &Path)",
    "fn deploy_source_route_evidence(",
  );
  const tinyStaticPacketGuard = sliceBetween(
    deployAdapter,
    "fn deploy_routes_do_not_invent_tiny_static_packet_paths()",
    "fn provider_upload_plan_partitions_public_runtime_from_evidence()",
  );

  assert.match(
    deployRoutes,
    /let packet = relative\.replace\("index\.html", "index\.dxpk"\);[\s\S]*if output_dir\.join\(&packet\)\.is_file\(\) && !tiny_static_no_js \{[\s\S]*route\["packet"\] = serde_json::json!\(packet\);[\s\S]*\}/,
    "deploy route discovery should only publish a route packet field when the packet artifact actually exists and the route is not tiny-static no-JS",
  );
  expectAll(deployRoutes, [
    "route_html_indicates_tiny_static_no_js",
    'data-dx-output-mode="tiny-static"',
    'data-dx-js="none"',
    "output_dir.join(&packet).is_file() && !tiny_static_no_js",
    "output_dir.join(&client_islands).is_file() && !tiny_static_no_js",
    "output_dir.join(&client_islands_runtime).is_file() && !tiny_static_no_js",
    "output_dir.join(&streaming_plan).is_file() && !tiny_static_no_js",
    "output_dir.join(&server_data).is_file() && !tiny_static_no_js",
  ]);
  assert.match(
    appRouterBuild,
    /if !streaming_plan_written \{[\s\S]*remove_stale_route_artifact\(&app_output_dir\.join\("streaming-plan\.json"\)\)\?;[\s\S]*let islands_compiled = write_app_client_islands_contract\([\s\S]*if islands_compiled == 0 \{[\s\S]*remove_stale_route_artifacts\([\s\S]*&app_output_dir,[\s\S]*&\["client-islands\.json", "client-islands\.js"\],[\s\S]*\)\?;/,
    "tiny-static rebuilds should remove stale streaming and client-island runtime artifacts before deploy route discovery can publish them",
  );
  assert.match(
    appRouterBuild,
    /fn remove_stale_route_artifacts\(app_output_dir: &Path, file_names: &\[&str\]\) -> DxResult<\(\)>[\s\S]*fn remove_stale_route_artifact\(path: &Path\) -> DxResult<\(\)>/,
    "stale route runtime cleanup should use one small shared helper instead of packet-only cleanup",
  );
  assert.match(
    tinyStaticPacketGuard,
    /data-dx-output-mode="tiny-static"[\s\S]*data-dx-js="none"[\s\S]*index\.dxpk[\s\S]*server-data\.json[\s\S]*client-islands\.json[\s\S]*client-islands\.js[\s\S]*streaming-plan\.json[\s\S]*let routes = deploy_routes\(output\);[\s\S]*assert_eq!\(root\["html"\], "app\/index\.html"\);[\s\S]*assert_eq!\(root\.get\("packet"\), None\);[\s\S]*assert_eq!\(root\.get\("server_data"\), None\);[\s\S]*assert_eq!\(root\.get\("client_islands"\), None\);[\s\S]*assert_eq!\(root\.get\("client_islands_runtime"\), None\);[\s\S]*assert_eq!\(root\.get\("streaming_plan"\), None\);/,
    "tiny-static deploy proof should keep generated public routes free of stale route runtime data when no runtime data was emitted",
  );
  expectAll(readiness, [
    "route_public_packet_required",
    "remove_stale_route_packet",
    "no public route packet for no_js_capable routes",
    "no stale index.dxpk for tiny-static no-JS routes",
    "deploy_routes_do_not_invent_tiny_static_packet_paths",
  ]);
});
