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
  const start = source.indexOf(name);
  assert.notEqual(start, -1, `${name} must exist`);
  const arrayStart = source.indexOf("&[", start);
  const arrayEnd = source.indexOf("];", arrayStart);
  assert.notEqual(arrayStart, -1, `${name} must have an array start`);
  assert.notEqual(arrayEnd, -1, `${name} must have an array end`);
  return rustStringLiterals(source.slice(arrayStart, arrayEnd));
}

function readinessNativeGroupEvents(readiness: string): string[] {
  const start = readiness.indexOf("fn native_event_groups()");
  const end = readiness.indexOf("fn native_event_catalog_integrity", start);
  assert.notEqual(start, -1, "native_event_groups must exist");
  assert.notEqual(end, -1, "native_event_catalog_integrity must follow native_event_groups");
  const grouped = new Set<string>();
  const source = readiness.slice(start, end);

  for (const match of source.matchAll(/"events":\s*\[([^\]]*)\]/g)) {
    for (const eventName of rustStringLiterals(match[1])) {
      grouped.add(eventName);
    }
  }

  return [...grouped].sort();
}

function readinessGateIds(readiness: string): string[] {
  const start = readiness.indexOf('"gate_summary": [');
  const end = readiness.indexOf('"proof_node_ids": [', start);
  assert.notEqual(start, -1, "gate_summary must exist");
  assert.notEqual(end, -1, "proof_node_ids must follow gate_summary");
  return [...readiness.slice(start, end).matchAll(/"id":\s*"([^"]+)"/g)].map(
    (match) => match[1],
  );
}

function readinessProofNodeIds(readiness: string): string[] {
  const start = readiness.indexOf('"proof_node_ids": [');
  const end = readiness.indexOf('"replay_commands": readiness_replay_commands()', start);
  assert.notEqual(start, -1, "proof_node_ids must exist");
  assert.notEqual(end, -1, "replay_commands must follow proof_node_ids");
  return rustStringLiterals(readiness.slice(start, end));
}

function readinessProofGraphNodeIds(readiness: string): string[] {
  const start = readiness.indexOf('"proof_nodes": [');
  const end = readiness.indexOf('"replay_commands": readiness_replay_commands()', start);
  assert.notEqual(start, -1, "proof_nodes must exist");
  assert.notEqual(end, -1, "replay_commands must follow proof_nodes");
  return [...readiness.slice(start, end).matchAll(/readiness_proof_node\(\s*"([^"]+)"/g)].map(
    (match) => match[1],
  );
}

test("readiness browser receipt metadata names canonical starter route proof targets", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");

  expectAll(readiness, [
    "READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE",
    '"/state-runtime"',
    "READINESS_STATE_RUNTIME_CANONICAL_STARTER_SOURCE",
    "examples/template/proof-routes/state-runtime/page.tsx",
    "READINESS_STATE_RUNTIME_CANONICAL_STARTER_DEV_URL",
    "http://127.0.0.1:3000/state-runtime",
    "READINESS_ISLANDS_CANONICAL_STARTER_ROUTE",
    '"/islands"',
    "READINESS_ISLANDS_CANONICAL_STARTER_SOURCE",
    "examples/template/proof-routes/islands/page.tsx",
    "READINESS_ISLANDS_CANONICAL_STARTER_DEV_URL",
    "http://127.0.0.1:3000/islands",
    "readiness_browser_receipt_proof_targets",
    '"browser_receipt_proof_targets": readiness_browser_receipt_proof_targets()',
    '"canonical_starter_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE',
    '"canonical_starter_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE',
    '"canonical_starter_source": READINESS_STATE_RUNTIME_CANONICAL_STARTER_SOURCE',
    '"canonical_starter_source": READINESS_ISLANDS_CANONICAL_STARTER_SOURCE',
    '"canonical_local_dev_url": READINESS_STATE_RUNTIME_CANONICAL_STARTER_DEV_URL',
    '"canonical_local_dev_url": READINESS_ISLANDS_CANONICAL_STARTER_DEV_URL',
    '"browser_runtime_executed_by_readiness": false',
    '"canonical_proof_target_route": READINESS_STATE_RUNTIME_CANONICAL_STARTER_ROUTE',
    '"canonical_proof_target_route": READINESS_ISLANDS_CANONICAL_STARTER_ROUTE',
    "dx www readiness --import-state-runtime-browser-receipt <browser-receipt.json> --json --full",
    "dx www readiness --import-island-browser-receipt <browser-receipt.json> --json --full",
    "local-in-app-browser-state-runtime-replay",
    "local-in-app-browser-source-owned-island-replay",
  ]);
  assert.doesNotMatch(readiness, /"browser_runtime_executed_by_readiness": true/);
});

test("release-readiness proof contract is source-owned and separates public bytes from evidence", () => {
  const cli = read("dx-www/src/cli/mod.rs");
  const readiness = read("dx-www/src/cli/readiness.rs");
  const deploy = read("dx-www/src/cli/deploy_adapter_contract.rs");
  const buildCommand = read("dx-www/src/cli/app_router_build_command.rs");
  const runtimeCommand = read("dx-www/src/cli/app_router_runtime_command.rs");
  const execution = read("dx-www/src/cli/app_router_execution.rs");
  const routeHandlerBuild = read("dx-www/src/cli/app_route_handler_build_output.rs");

  assert.match(cli, /mod readiness;/);
  expectAll(readiness, [
    "www_root::discover_www_repo_root(cwd)",
    "dx.www.readiness.proof_graph",
    "dx.www.readiness.score_breakdown",
    "dx.www.readiness.delivery_tiers",
    "dx.www.readiness.native_event_catalog",
    "READINESS_CURRENT_HONEST_SCORE",
    "READINESS_TARGET_SCORE",
    "dx.www.readiness.native_event_catalog_receipt_contract",
    ".dx/receipts/readiness/native-events-latest.json",
    ".dx/receipts/readiness/native-events-latest.sr",
    ".dx/serializer/receipts-readiness-native-events-latest.machine",
    "dx.www.readiness.native_event_browser_binder_receipt_contract",
    ".dx/receipts/readiness/native-event-browser-binder-latest.json",
    ".dx/receipts/readiness/native-event-browser-binder-latest.sr",
    ".dx/serializer/receipts-readiness-native-event-browser-binder-latest.machine",
    "dx.www.readiness.state_runtime_browser_receipt_contract",
    ".dx/receipts/readiness/state-runtime-browser-latest.json",
    ".dx/receipts/readiness/state-runtime-browser-latest.sr",
    ".dx/serializer/receipts-readiness-state-runtime-browser-latest.machine",
    "dx.www.readiness.reactivity_model",
    "dx.www.readiness.reactivity_model_receipt_contract",
    ".dx/receipts/readiness/reactivity-model-latest.json",
    ".dx/receipts/readiness/reactivity-model-latest.sr",
    ".dx/serializer/receipts-readiness-reactivity-model-latest.machine",
    "dx.www.readiness.docs_onboarding",
    "dx.www.readiness.docs_onboarding_receipt_contract",
    ".dx/receipts/readiness/docs-onboarding-latest.json",
    ".dx/receipts/readiness/docs-onboarding-latest.sr",
    ".dx/serializer/receipts-readiness-docs-onboarding-latest.machine",
    "dx.www.readiness.server_action_replay_ledger_receipt_contract",
    ".dx/receipts/readiness/server-action-replay-ledger-latest.json",
    ".dx/receipts/readiness/server-action-replay-ledger-latest.sr",
    ".dx/serializer/receipts-readiness-server-action-replay-ledger-latest.machine",
    "dx.www.readiness.no_js_artifact_receipt_contract",
    ".dx/receipts/readiness/no-js-artifact-latest.json",
    ".dx/receipts/readiness/no-js-artifact-latest.sr",
    ".dx/serializer/receipts-readiness-no-js-artifact-latest.machine",
    "dx.www.readiness.no_js_browser_receipt_contract",
    ".dx/receipts/readiness/no-js-browser-latest.json",
    ".dx/receipts/readiness/no-js-browser-latest.sr",
    ".dx/serializer/receipts-readiness-no-js-browser-latest.machine",
    "dx.www.readiness.bundle_partition",
    "dx.www.readiness.bundle_partition_receipt_contract",
    ".dx/receipts/readiness/bundle-partition-latest.json",
    ".dx/receipts/readiness/bundle-partition-latest.sr",
    ".dx/serializer/receipts-readiness-bundle-partition-latest.machine",
    "dx.www.readiness.production_http",
    "dx.www.readiness.route_action_runtime",
    "dx.www.readiness.primitive_proof",
    ".dx/receipts/readiness/proof-graph.sr",
    "visual_edit_workbench_receipt_contract",
    "visual_edit_workbench_receipt",
    "visual_edit_workbench_receipt_sr",
    "visual_edit_workbench_receipt_machine",
    "native_event_catalog_receipt_contract",
    "native_event_catalog_receipt",
    "native_event_catalog_receipt_sr",
    "native_event_catalog_receipt_machine",
    "state_runtime_browser_receipt_contract",
    "state_runtime_browser_receipt",
    "state_runtime_browser_receipt_sr",
    "state_runtime_browser_receipt_machine",
    "reactivity_model_receipt_contract",
    "reactivity_model_receipt",
    "reactivity_model_receipt_sr",
    "reactivity_model_receipt_machine",
    "docs_onboarding_receipt_contract",
    "docs_onboarding_receipt",
    "docs_onboarding_receipt_sr",
    "docs_onboarding_receipt_machine",
    "server_action_replay_ledger_receipt_contract",
    "server_action_replay_ledger_receipt",
    "server_action_replay_ledger_receipt_sr",
    "server_action_replay_ledger_receipt_machine",
    "tiny-static",
    "tier-0-static-no-js-source-only",
    "tiny_static_route_proof",
    "no_js_capable",
    "script_tag_count",
    "semantic_landmark_present",
    "link_count",
    "form_count",
    "seo_title_present",
    "accessibility_signal_count",
    "artifact_html_blake3",
    "links_forms_seo_accessibility_fact_status",
    "route_unit_present",
    "route_unit_no_js_capable_current",
    "missing-route-unit-proof",
    "route-unit-not-no-js-capable",
    "public-packet-present",
    "browser_js_budget_bytes",
    "runtime_required == false",
    "browser_api_required == false",
    "astro_parity_status",
    "astro_parity_claimed",
    "live_astro_parity_receipt",
    "route_public_packet_required",
    "remove_stale_route_packet",
    "no public route packet for no_js_capable routes",
    "deploy_routes_do_not_invent_tiny_static_packet_paths",
    "dx_preview_production_contract_serves_only_deploy_adapter_outputs",
    "node --test benchmarks/public-framework-tools.test.ts",
    "node --test benchmarks/dx-devtools-framework-integration.test.ts",
    "node --test benchmarks/dx-www-docs-doctor.test.ts",
    "normalized_public_artifact_path_rejects_evidence_and_dot_dx_paths",
    "public_runtime_artifact_plan_counts_evidence_but_returns_only_public_paths",
    "copy_public_runtime_artifacts_leaves_receipts_outside_vercel_static",
    "cargo test -j 1 -p dx-www --no-default-features --features cli deploy_routes_do_not_invent_tiny_static_packet_paths -- --nocapture",
    ".dx/build-cache/source-routes/**/route-unit.json is evidence-only",
    "source-only local tiny-static contract; no live Astro payload/paint/throughput replay receipt yet",
    "source-only HTML/CSS no-JS proof; not live Astro payload/paint/throughput parity",
    "no_js_browser_receipt_current",
    "no_js_browser_stale_reason",
    "no-js-browser-receipt-missing",
    "current-local-js-disabled-browser-proof",
    "READINESS_NO_JS_BROWSER_COLLECT_COMMAND",
    "node --test benchmarks/dx-www-no-js-browser-receipt.test.ts",
    "node benchmarks/dx-www-no-js-browser-receipt.ts --html-path examples/template/.dx/www/output/app/index.html",
    "collector_command",
    "dx www readiness --import-no-js-browser-receipt <browser-receipt.json> --json --full",
    "dx.www.same_machine_performance_receipt",
    "target/framework-comparison-20260531/throughput.json",
    "same_machine_performance_receipt_current",
    "same-machine-throughput-receipt-current-payload-paint-proof-needed",
    "node --test benchmarks/dx-runtime-throughput-receipt-contract.test.ts",
    "node --test benchmarks/dx-www-lighthouse-runtime-guard.test.ts",
    "node --test benchmarks/dx-www-cdp-paint-receipt.test.ts",
    "node benchmarks/dx-www-cdp-paint-receipt.ts --url http://127.0.0.1:3000 --receipt-mode dev --out examples/template/.dx/receipts/check/web-perf/dev/report.json",
    "node benchmarks/dx-www-cdp-paint-receipt.ts --url http://127.0.0.1:4173 --receipt-mode static-build --out examples/template/.dx/receipts/check/web-perf/static-build/report.json",
    "source-owned-cdp-paint-receipts-current-lighthouse-parity-needed",
    "node benchmarks/dx-runtime-throughput-benchmark.ts --rounds 3 --requests 240 --concurrency 16",
    "public_runtime_bundle",
    "evidence_bundle",
    "source-only local deploy contract; no hosted multi-provider evidence-bundle replay receipt yet",
    "dx deploy vercel copies public-runtime artifacts only",
    ".dx/build-cache/provider-adapter.dx-cloud.json upload_plan",
    ".vercel/output/static",
    "vercel_build_output.evidence_excluded_from_public_output",
    "public_runtime_content_hash",
    "public_runtime_artifact_count",
    "evidence_artifact_count",
    "bundle_partition_current",
    "local-public-evidence-partition-current",
    "local-public-evidence-partition-current-provider-proof-needed",
    "bundle_partition_source",
    "page-graph.json evidence-only",
    "normalized.ends_with(\"/page-graph.json\")",
    "materialize_vercel_build_output_keeps_tiny_static_public_and_evidence_private",
    ".dx/build-cache/cache-manifest.json",
    ".dx/build-cache/provider-adapter-smoke-matrix.json",
    "precompressed_assets",
    "route_action_runtime",
    "primitive_proof",
    "READINESS_PRODUCTION_HTTP_TCP_PREVIEW_COLLECT_COMMAND",
    "dx.www.readiness.production_http_tcp_preview_receipt_contract",
    ".dx/receipts/readiness/production-http-tcp-preview-latest.json",
    ".dx/receipts/readiness/production-http-tcp-preview-latest.sr",
    ".dx/serializer/receipts-readiness-production-http-tcp-preview-latest.machine",
    "node --test benchmarks/dx-www-production-preview-tcp-receipt.test.ts",
    "node benchmarks/dx-www-production-preview-tcp-receipt.ts --dx-www-bin target/release/dx-www.exe --build-dir examples/template/.dx/www/output --out .dx/receipts/readiness/browser-import-candidates/production-http-tcp-preview-latest.json",
    "dx www readiness --import-production-http-tcp-preview-receipt <tcp-receipt.json> --json --full",
    "production_http_tcp_preview_current",
    "production_http_tcp_preview_stale_reason",
    "preview-tcp-server-parity",
    "server-action-protocols.json",
    "server-action-runtime.json",
    ".dx/build-cache/server-action-replay-ledger.json",
    ".dx/build-cache/route-handler-conformance-matrix.json",
    "local-route-handler-conformance-foundation",
    "csrf_hook",
    "session_hook",
    "replay_protection",
    "405 Method Not Allowed",
    "400 Bad Request",
    "starter_score",
    "runtime_proof_score",
    "framework_maturity_score",
    "release_readiness_score",
    "fastest_world_claim",
    "Astro tiny-static parity remains a proof gate",
    "dx.react.clientIsland.abi",
    "camelCase-jsx-props",
    "framework_adapter_count",
    "unsupported_directive_syntax",
    "dx.www.readiness.command",
    "dx www readiness --json --full",
    "docs-onboarding-doctor",
    "dx www docs-doctor --json",
    "benchmarks/dx-www-docs-doctor.test.ts",
    "benchmarks/dx-www-readiness-docs-onboarding-receipts.test.ts",
    "docs_doctor_coverage_scope",
    "current-public-www-docs-plus-compatibility-generated-archive-warning-surfaces",
    "docs_doctor_coverage_gap",
    "generated-archived-warning-cleanup-and-ownership-promotion",
    "docs_doctor_generated_archived_policy",
    "warning-only-generated-archive-coverage",
    "MONITORED_GENERATED_ARCHIVED_CLAIM_ROOTS",
    "generated_archived_claim_surface_policy",
    "generated-archived-stale-claim",
    "docs_doctor_replay_command",
    "docs_doctor_report_schema",
    "dx.www.docs_doctor",
    "starter_check_receipt",
    "starter_inventory",
    "receipt-score-mismatch",
    "missing-starter-file-claim",
    "write_sr_artifact(output_dir, READINESS_PROOF_GRAPH_RECEIPT, &fields)",
    "readiness_proof_graph_sr_fields",
    "serializer_machine_generated",
    "machine_path",
    "machine_path_within_root",
    "serializer_provenance",
    "dx.www.readiness.serializer_provenance",
    "source_blake3",
    "machine_blake3",
    "file_blake3_hex",
    "artifact_path_within_root",
    "--write-receipts",
    "--write-visual-edit-replay",
    "write_readiness_local_receipts",
    "write_readiness_visual_edit_replay_receipt",
    "visual_edit_replay",
    "source_restored",
    "write_readiness_no_js_artifact_receipt",
    "readiness_no_js_artifact_paths",
    "root-output-first-then-canonical-starter",
    "examples/template/.dx/www/output/app/index.html",
    "examples-template-starter",
    "project-root-output",
    "/runtime_report/tiny_static_route_proof",
    "readiness_no_js_artifact_sr_fields",
    "write_readiness_native_event_catalog_receipt",
    "write_readiness_native_event_browser_binder_sr_receipt",
    "native_event_browser_binder_receipt_is_current",
    "readiness_native_event_catalog_sr_fields",
    "readiness_native_event_browser_binder_sr_fields",
    "compiler-catalog-valid-mdn-current",
    "current-against-local-mdn-browser-compat-data",
    "mdn_event_freshness",
    "mdn_event_count",
    "event_entry_count",
    "missing_from_compiler_count",
    "extra_in_compiler_count",
    "target/mdn-browser-compat-data/api *_event entries",
    "browser_binder_proof_status",
    "missing-browser-binder-receipt",
    "browser-binder-replay-current",
    "Node VM replay is not browser proof",
    "node_vm_binder_replay_status",
    "benchmarks/dx-www-native-dom-event-binder-replay.test.ts",
    "\"written_count\": written_count",
    "let written_count = receipts.len()",
    "\"receipts\": receipts",
    "\"id\": \"visual-edit-workbench-receipts\"",
    "dx www readiness does not invent visual-edit proof",
    "\"live_browser_executed\": false",
    "\"javascript_disabled_browser\": false",
    "\"astro_parity_claimed\": false",
  ]);

  expectAll(deploy, [
    "readiness_deploy_contract",
    "write_readiness_proof_graph_receipt",
    "READINESS_PROOF_GRAPH_RECEIPT",
    "readiness",
    "bundle_partition",
    "cache_manifest",
    "cdn_headers",
    "dx.www.deploy.bundle_partition",
    "public-runtime",
    "evidence",
    "deployable_public_bytes",
    "Content-Encoding",
    "Accept-Encoding",
    "request_schema",
    "response_schema",
    "preview_error_policy",
    "PROVIDER_ADAPTER_SMOKE_MATRIX_JSON",
    "deploy_source_route_evidence",
    "source_route_evidence",
    ".dx/build-cache/source-routes/root/route-unit.json",
    "source-route-unit",
    "dx.www.deploy.provider_adapter_smoke_matrix",
    "dx.www.deploy.route_handler_conformance_matrix",
    "hosted_provider_proof",
    "automatic_route_handler_options_response",
    "local-replay-passing-foundation",
    "account-free-fixture",
    "upload-plan-only",
    "deploy_routes_do_not_invent_tiny_static_packet_paths",
    "output_dir.join(&packet).is_file()",
  ]);
  expectAll(buildCommand, [
    "react_route_component_sources",
    "route_public_packet_required",
    "remove_stale_route_packet",
    "tiny_static_route_proof",
    "no_js_capable",
  ]);
  expectAll(runtimeCommand, [
    "react_route_component_sources",
    "route_value_import_specifiers",
    "resolve_component_import",
    "components_by_path",
    "parse_tsx_module",
  ]);
  expectAll(execution, [
    "tiny_static_route_proof.no_js_capable",
    "Value::Null",
    '"packet_path": public_packet_path',
  ]);

  expectAll(routeHandlerBuild, [
    "declared_methods",
    "implicit_methods",
    "safe_build_methods",
    "source_method",
  ]);
});

test("agent context exposes compact and full release-readiness handoff data", () => {
  const agentContext = read("dx-www/src/cli/agent_context.rs");
  const cliCore = read("dx-www/src/cli/mod_parts/cli_core_impl.rs");

  expectAll(agentContext, [
    "--full",
    "dx www agent-context --json --full",
    "readiness",
    "readiness_summary",
    "readiness_full",
    "readiness_agent_context_for_project(full, Some(cwd))",
    "dx www readiness --json --full",
  ]);
  expectAll(cliCore, [
    'Some("readiness") => {',
    "readiness::cmd_readiness(&self.cwd, &args[1..])",
    "www readiness --json",
  ]);
  assert.match(cliCore, /www agent-context --json\s+Compact agent handoff context/);
});

test("release-readiness exposes route-handler and server-action proof gaps to check context", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");

  expectAll(readiness, [
    "dx.www.readiness.route_handler_server_action_gaps",
    "route-handler-server-action-proof-gaps",
    "route-handler-provider-conformance-matrix",
    ".dx/build-cache/route-handler-conformance-matrix.json",
    "local-route-handler-conformance-foundation",
    "release_ready",
    "\"release_ready\": false",
    "breadth-proof-gap",
    "not_yet_claimed",
    "provider-deployed route-handler conformance matrix",
    "provider-hosted route-handler conformance is not broad enough for release-readiness claims",
    "Local route-handler/server-action foundations are proven; do not claim release readiness server runtime maturity until provider-hosted breadth proof passes.",
    "server-action-distributed-replay-store",
    ".dx/build-cache/server-action-replay-ledger.json",
    "hash-only local preview replay ledger",
    "local-preview-hash-ledger",
    "gap_count",
    "server_action_provider_gap_ids",
    "provider_proof_gap_ids",
    "provider-request-cancellation-replay",
    "distributed idempotency store",
    "provider-hosted CSRF/session",
    "request cancellation",
    "durable replay matrix",
    "production-contract-adapter-smoke-matrix",
    "dx_build_emits_hosted_preview_bundle_with_forge_receipts",
    "dx_server_action_post_endpoints_run_in_dev_and_preview_with_receipts",
    "dx_preview_production_contract_serves_only_deploy_adapter_outputs",
    ".dx/build-cache/provider-adapter-smoke-matrix.json",
    "local-smoke-matrix-emitted",
    "local replay/account-free/upload-plan proof",
    "server-action-protocols.json",
    ".dx/build-cache/server-action-replay-ledger.json",
    "dx check --latest-receipt --json",
    "dx www agent-context --json --full",
  ]);
  assert.doesNotMatch(
    readiness,
    /route-handler-hosted-preview-fixture-stale|server-action-protocol-receipt-missing|production-contract-health-json-gap/,
  );
});

test("release-readiness proof surfaces keep honest check and agent-context gate markers visible", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");
  const agentContext = read("dx-www/src/cli/agent_context.rs");
  const combined = `${readiness}\n${agentContext}`;

  expectAll(combined, [
    "readiness_gate_status",
    "release_claim_allowed",
    "global_speed_claim_allowed",
    "missing_proof_gates",
    "remaining_proof_gates",
    "relative_release_ready",
    "release_ready_scope",
    "release_ready",
    "\"release_ready\": true",
    "fastest_world_claim",
    "fastest_world_claim\": false",
    "replay_commands",
    "dx check --latest-receipt --json",
    "dx www readiness --json --full",
    "dx www readiness --write-visual-edit-replay --json",
    "dx www agent-context --json --full",
    "known non-claims",
    "not_yet_claimed",
    "relative-local-proof-backed-release-ready",
    "local-proof-backed-www-release",
    "post-release proof hardening gates",
  ]);

  assert.doesNotMatch(combined, /fastest_world_claim":\s*true/);
  assert.doesNotMatch(readiness, /fn readiness_proof_graph_sr\(/);
});

test("Release-readiness proof graph exposes evidence status without drift-prone gap metadata", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");

  expectAll(readiness, [
    "readiness_proof_node",
    "proof_evidence_kind",
    "source-receipt-contract",
    "runtime-receipt-contract",
    "static-source-guard",
    "\"last_verified_at\": Value::Null",
    "\"replay_evaluated\": false",
    "\"receipt_freshness\": \"not-evaluated-in-this-command\"",
    "required_receipts",
    "blocking_readiness_gate",
    "readiness_route_handler_server_action_gap_ids(&gaps)",
    "readiness_route_handler_server_action_gap_count(&gaps)",
    "as_array().map_or(0, Vec::len)",
    "filter_map(|gap| gap.get(\"id\").and_then(Value::as_str))",
  ]);

  assert.doesNotMatch(readiness, /"gap_count":\s*3/);
  assert.doesNotMatch(
    readiness,
    /"gap_ids":\s*\[\s*"route-handler-provider-conformance-matrix"/,
  );
});

test("release-readiness blocking gates all have proof graph nodes", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");
  const gateIds = readinessGateIds(readiness);
  const proofNodeIds = new Set(readinessProofNodeIds(readiness));
  const proofGraphNodeIds = new Set(readinessProofGraphNodeIds(readiness));

  assert.ok(gateIds.length >= 10, "release-readiness should expose the full blocking gate list");
  assert.deepEqual(
    gateIds.filter((id) => !proofNodeIds.has(id)),
    [],
    "every blocking gate must be listed in proof_node_ids",
  );
  assert.deepEqual(
    gateIds.filter((id) => !proofGraphNodeIds.has(id)),
    [],
    "every blocking gate must have a matching proof graph node",
  );
  expectAll(readiness, [
    "manifest-directive-and-abi-foundation",
    "island_abi_receipt_current",
    "island_browser_receipt_current",
    "source-owned-island-abi-and-browser-replay-current-hosted-proof-needed",
    "source-owned-island-abi-receipt-current-hosted-proof-needed",
    "per-directive browser proof, no-JS fallback proof, and explicit framework adapter receipts",
    "hosted/provider island breadth, explicit framework adapter execution receipts, and release-proof promotion",
    "durable source-owned island ABI receipt, then per-directive browser proof, no-JS fallback proof, and explicit framework adapter receipts",
    "source-receipt-contract",
    "clientLoad/clientVisible/clientIdle/clientOnly directives",
    "client-island manifest",
    "Local source-owned island browser replay is receipt-addressable when present; hosted/provider adapter breadth and explicit framework adapter receipts remain required.",
    "benchmarks/dx-www-islands-abi-camelcase.test.ts",
    "visual-edit-workbench-receipts",
    "inspect-preview-apply-undo-receipt-foundation-browser-proof-missing",
    "dx.www.readiness.visual_edit_workbench_receipt_contract",
    ".dx/receipts/devtools/visual-edit-latest.json",
    ".dx/receipts/devtools/visual-edit-latest.sr",
    ".dx/serializer/receipts-devtools-visual-edit-latest.machine",
    "json_read_model_path",
    "serializer_receipt_path",
    "machine_contract_path",
    "/_dx/devtools/style-preview",
    "/_dx/devtools/style-apply",
    "/_dx/devtools/style-undo",
    "preview-only / not writable guard",
    "browser workbench replay remains the release readiness gate",
  ]);
});

test("release-readiness native-event catalog validates report groups against compiler-owned events", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");

  expectAll(readiness, [
    "native_event_catalog_integrity",
    "dx.www.readiness.native_event_catalog_receipt_contract",
    ".dx/receipts/readiness/native-events-latest.json",
    ".dx/receipts/readiness/native-events-latest.sr",
    ".dx/serializer/receipts-readiness-native-events-latest.machine",
    ".dx/receipts/readiness/native-event-browser-binder-latest.json",
    ".dx/receipts/readiness/native-event-browser-binder-latest.sr",
    ".dx/serializer/receipts-readiness-native-event-browser-binder-latest.machine",
    "json_read_model_path",
    "serializer_receipt_path",
    "machine_contract_path",
    "grouped_event_names",
    "unknown_grouped_events",
    "ungrouped_catalog_events",
    "native_event_catalog_complete",
    "events.iter().copied().collect::<BTreeSet<_>>()",
    "difference(&catalog_events)",
    "difference(&grouped_events)",
    "\"mdn_snapshot_status\": \"durable-local-receipt-supported\"",
    "\"catalog_source\": \"compiler-owned-static-snapshot\"",
    "\"source_freshness\": \"evaluated-by-dx-www-readiness-write-receipts-when-local-mdn-checkout-exists\"",
    "\"group\": \"mdn-other\"",
    "current-against-local-mdn-browser-compat-data",
    "local-mdn-browser-compat-data-commit-recorded",
    "dx.www.readiness.mdn_browser_compat_event_freshness",
    "mdn_browser_compat_key_is_dom_event",
    "\"once_per_event\"",
    "browser_binder_receipt_contract",
    "\"browser_binder_proof_status\": \"missing-browser-binder-receipt\"",
    "\"node_vm_binder_replay_status\": \"source-guarded-not-real-browser-proof\"",
    "benchmarks/dx-www-native-dom-event-binder-replay.test.ts",
  ]);
  const routeUnit = read("core/src/delivery/route_unit.rs");
  expectAll(routeUnit, [
    "tier-0 no-JS proof is source-only; Astro parity requires a separate live payload/paint/throughput receipt",
    "source-only no-JS shell; Astro parity is not claimed by route-unit proof",
  ]);
});

test("release-readiness native-event report groups cover every compiler-owned event", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");
  const compilerEvents = read("core/src/delivery/dom_events.rs");

  const catalogEvents = rustStaticStringArray(compilerEvents, "NATIVE_DOM_EVENT_NAMES");
  const groupedEvents = readinessNativeGroupEvents(readiness);
  const catalogSet = new Set(catalogEvents);
  const groupedSet = new Set(groupedEvents);

  assert.ok(catalogEvents.length > 100, "native event catalog should be broad");
  assert.deepEqual(
    groupedEvents.filter((eventName) => !catalogSet.has(eventName)),
    [],
    "release-readiness native-event groups must not claim events outside the compiler catalog",
  );
  const dynamicallyCovered = catalogEvents.filter(
    (eventName) => !groupedSet.has(eventName),
  );
  assert.ok(
    dynamicallyCovered.length > 0,
    "the broad MDN catalog should leave non-curated events for mdn-other",
  );
  expectAll(readiness, [
    "let mdn_other = catalog_events",
    "difference(&grouped_events)",
    "\"group\": \"mdn-other\"",
    "\"Automatically covers compiler-owned MDN event names that do not belong to the curated UI groups yet.\"",
  ]);
  assert.ok(
    catalogEvents.length >= 300,
    "native event catalog should track the broad MDN event surface",
  );
});

test("dev and preview server-action validation errors share the 400 contract", () => {
  const runtime = read("dx-www/src/cli/server_action_runtime.rs");
  const cliCore = read("dx-www/src/cli/mod_parts/cli_core_impl.rs");
  const preview = read("dx-www/src/cli/preview_contract.rs");
  const integration = read("dx-www/src/cli/tests/part_02.rs");

  expectAll(runtime, [
    "server_action_error_status",
    "server_action_redacted_error",
    "400 Bad Request",
    "500 Internal Server Error",
    "csrf token",
    "session id",
    "idempotency key",
    "SERVER_ACTION_REPLAY_LEDGER_JSON",
    "local-preview-hash-ledger",
    "distributed",
    "provider_hosted",
    "duplicate",
    "replay_key_hash",
    "raw payload, session, csrf, and idempotency values are not persisted",
    "action_id does not match",
    "expected ",
    "validation failed",
  ]);

  expectAll(cliCore, [
    "server_action_runtime::server_action_error_status(&error)",
    "server_action_runtime::server_action_redacted_error(&error)",
  ]);
  expectAll(preview, [
    "server_action_runtime::server_action_error_status(error)",
    "server_action_runtime::server_action_redacted_error(error)",
  ]);
  expectAll(integration, [
    "let (invalid_status, invalid_content_type, invalid_dev_body)",
    'assert_eq!(invalid_status, "400 Bad Request")',
    "let invalid_preview = preview_contract::handle_production_contract_http_request",
    'assert_eq!(invalid_preview.status, "400 Bad Request")',
    'assert_eq!(invalid_preview_response["receipt_written"], false)',
    'assert_eq!(invalid_preview_response["replay_safe"], false)',
  ]);
  assert.doesNotMatch(
    integration,
    /assert_eq!\(invalid_status,\s*"500 Internal Server Error"\)/,
  );
});

test("native DOM event catalog drives React-style event collection and browser binder support", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");
  const semantics = read("dx-www/src/cli/app_router_semantics.rs");
  const clientComponent = read(
    "dx-www/src/cli/app_router_execution/source_render_parts/client_component.rs",
  );
  const compilerEvents = read("core/src/delivery/dom_events.rs");
  const deliveryMod = read("core/src/delivery/mod.rs");
  const routeUnit = read("core/src/delivery/route_unit.rs");

  expectAll(readiness, [
    "click",
    "dblclick",
    "beforeinput",
    "pointerdown",
    "pointermove",
    "pointerup",
    "keydown",
    "keyup",
    "compositionstart",
    "compositionupdate",
    "compositionend",
    "animationstart",
    "animationiteration",
    "transitionend",
    "wheel",
    "dragstart",
    "drop",
    "touchstart",
    "touchmove",
    "touchend",
  ]);

  assert.match(semantics, /on\[A-Z\]\[A-Za-z0-9_\]\*/);
  expectAll(clientComponent, [
    "native_dom_event_names",
    "react_style_event_attribute_to_dom_event",
    "__DX_SUPPORTED_DOM_EVENTS__",
    "supportedEvents.includes",
    "support_status",
    "native-dom-event-supported",
    "unsupported-react-event-diagnostic",
    "Use a native DOM event from the DX catalog or add an explicit adapter boundary.",
    "readiness_receipt_contract",
    "dx.www.readiness.native_event_browser_binder_receipt_contract",
    "react_style_event_examples",
    "onClick",
    "onInput",
    "onPointerMove",
    "dom_event_examples",
    "pointermove",
    "unsupported_event_policy",
    "diagnose unsupported React-style event attributes without attaching listeners or claiming React synthetic event parity",
    "receipt_ready_fields",
    "binder_global_present",
    "catalog_source",
    "catalog_hash",
    "listener_events",
    "unsupported_listener_attached",
    "preview_event_count",
    "state_dispatch_count",
    "local-browser-native-event-binder-replay-required",
    "eventAttributeCount === 0",
  ]);
  expectAll(readiness, [
    "react_style_event_attribute_to_dom_event",
    "current compiler delivery module is the source-owned foundation catalog",
    '"react_style_event_examples": ["onClick", "onInput", "onPointerMove"]',
    '"dom_event_examples": ["click", "input", "pointermove"]',
  ]);
  assert.match(
    readiness,
    /fn native_event_browser_binder_receipt_is_current[\s\S]*receipt\.get\("proof_scope"\)\.and_then\(Value::as_str\)\s*== Some\("local-in-app-browser-native-event-binder-replay"\)/,
  );
  assert.match(
    readiness,
    /fn native_event_browser_binder_receipt_is_current[\s\S]*receipt\s*\.get\("full_react_event_parity"\)\s*\.and_then\(Value::as_bool\)\s*== Some\(false\)/,
  );
  assert.match(
    readiness,
    /fn native_event_browser_binder_receipt_is_current[\s\S]*receipt\s*\.get\("react_synthetic_events"\)\s*\.and_then\(Value::as_bool\)\s*== Some\(false\)/,
  );
  expectAll(compilerEvents, [
    "pub struct NativeDomEventCatalogIntegrity",
    "pub fn native_dom_event_catalog_integrity() -> NativeDomEventCatalogIntegrity",
    "NATIVE_DOM_EVENT_NAMES",
    "react_style_event_attribute_to_dom_event",
    "onDoubleClick",
    "dblclick",
    "input",
    "pointermove",
    "beforeinput",
    "catalog_count",
    "sorted_unique",
    "duplicate_events",
    "catalog_hash",
  ]);
  expectAll(deliveryMod, [
    "NativeDomEventCatalogIntegrity",
    "native_dom_event_catalog_integrity",
  ]);
  expectAll(readiness, [
    "native_dom_event_catalog_integrity",
    '"compiler_integrity": native_dom_event_catalog_integrity()',
    '"source_of_truth": "core/src/delivery/dom_events.rs::NATIVE_DOM_EVENT_NAMES"',
    '"sorted_unique"',
    '"duplicate_events"',
    '"catalog_hash"',
  ]);
  expectAll(routeUnit, [
    "react_style_event_attribute_to_dom_event(&attribute.name)",
    "unsupported_react_event_attributes",
    "unsupported React-style event attribute",
    "no listener is attached and React synthetic event parity is not claimed",
    "DxStateEventSlot",
    "event: dom_event",
  ]);
  assert.doesNotMatch(routeUnit, /fn react_dom_event_name/);
  assert.doesNotMatch(clientComponent, /Attaches only click, input, change, and native submit/);
  assert.doesNotMatch(
    clientComponent,
    /descriptor\.kind === "button-event" && names\.size === 0\) names\.add\("click"\)/,
  );
});

test("reactivity and island ABI stay DX-native while preserving React-friendly authoring", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");
  const stateRuntime = read("dx-www/src/cli/app_router_execution/state_runtime.rs");
  const semantics = read("dx-www/src/cli/app_router_semantics.rs");
  const clientIsland = read("core/src/delivery/client_island.rs");
  const buildOutput = read("dx-www/src/cli/app_router_build_output.rs");

  expectAll(readiness, [
    "\"island_abi\": readiness_island_abi()",
    "readiness_island_abi",
    "dx.www.readiness.island_abi",
    "compiler_abi_schema",
    "dx.react.clientIsland.abi",
    "compiler_capabilities_schema",
    "clientLoad",
    "clientVisible",
    "clientIdle",
    "clientOnly",
    "clientMedia",
    "clientInteraction",
    "core_directives",
    "additional_supported_directives",
    "release_core_directives",
    "route_unit_proof_metadata",
    "DxRouteReceipt.client_island_abi",
    "route_streaming_island_metadata",
    "camelCase",
    "camelCase-jsx-props",
    "directive_style_id",
    "unsupported_directive_syntax",
    "client:load",
    "client:visible",
    "client:idle",
    "client:only",
    "abi_fields",
    "source_owned_runtime",
    "node_modules_required",
    "full_react_hydration",
    "no_js_fallback_required",
    "source_owned_island_count",
    "dynamic_import_count",
    "explicit_frameworks",
    "adapter-boundary",
    "no_js_fallback",
    "required for every island route before the ABI can claim release readiness",
    "node_vm_state_runtime_replay_status",
    "browser_replay_receipt_contract",
    "state_runtime_browser_replay",
    "benchmarks/tsx-app-router-state-runtime-operations.test.ts",
    "useState(exact-dx-state-slot-only)",
    "compatibility_lowering_api",
    "exact_lowering_required",
    "state_graph_has_exact_use_state_lowering",
    "unsupported_unlowerable_use_state_diagnostic",
    "adapter_boundary_required_when_unlowerable",
  ]);

  expectAll(clientIsland, [
    "DxReactClientIslandAbiCapabilities",
    "react_client_island_abi_capabilities() -> DxReactClientIslandAbiCapabilities",
    "dx.react.clientIsland.abi.capabilities",
    "readiness_release_ready",
    "browser_proof_status",
    "adapter_boundary_required",
    "DxReactClientIslandDirective",
    "clientVisible",
    "clientIdle",
    "clientOnly",
    "framework-adapter-client-only",
    "component_route_directives",
    "is_client_island_directive",
    "dx:client-island-event",
    "data-dx-event-id",
    "CustomEvent",
  ]);
  assert.doesNotMatch(clientIsland, /\(\) => \{\}/);

  expectAll(buildOutput, [
    "stamp_client_island_hydration_markers",
    "data-dx-client-island-bridge",
    "data-dx-island",
    "data-dx-event-id",
  ]);

  expectAll(stateRuntime, [
    "dx_native_reactivity_capabilities",
    "dx.tsx.dxNativeReactivityCapabilities",
    "capability_matrix",
    "unsupported_react_api_policy",
    "adapter_boundary_required",
    "browser_proof_status",
    "state()",
    "derived()",
    "effect()",
    "action()",
    "derived_slots",
    "derived_dom_reflection",
    "refreshDerivedSlots",
    "dx:derived-state-slot",
    "derived slots use same data-dx-state-* markers",
    "react_hook_policy",
    "DX-native state() slots, app-global store slots, and explicit state graph slots",
    "dx.react-hook.useEffect.adapter-boundary-required",
  ]);

  expectAll(semantics, [
    "dx.tsx.reactEventSupport",
    "dx.tsx.reactHookSupport",
    "react_api_shim_executed",
    "native-dom-event-supported",
    "unsupported-react-event-diagnostic",
    "unsupported-react-event",
    "compatibility-lowered",
    "state_graph_has_exact_use_state_lowering",
    "source_events_have_exact_use_state_lowering",
    "event_handler_has_exact_use_state_setter_operation",
    "is_lowerable_use_state_setter_argument",
    "useState was detected, but the compiler did not find an exact DX state slot for this source.",
    "effect-boundary-scheduled",
    "react-effect-boundary",
    "react-semantic-boundary",
    "callback bodies and cleanup are not executed with hidden React semantics",
    "unsupported-react-hook-diagnostic",
    "unsupported-react-hook",
  ]);
  expectAll(semantics, [
    "collect_use_state_bindings",
    "source_without_comments_and_strings",
    "react_hook_call_patterns",
    "react_named_hook_imports",
    "react_namespace_imports",
    "source_slots.len() == bindings.len()",
    "slot.name.as_str() == binding.state_name.as_str()",
    "slot.setter.as_deref() == binding.setter_name.as_deref()",
    "dx.react-hook.useState.missing-exact-state-slot",
    "dx.react-hook.unsupported.",
    "not executed as React runtime semantics and not treated as a no-op",
  ]);
  assert.doesNotMatch(
    semantics,
    /"useState"\s+if\s+state_graph_has_slot_for_source\(state_graph,\s*source_path\)/,
  );

  expectAll(readiness, [
    "compiler_capabilities",
    "dx_compiler::delivery::react_client_island_abi_capabilities()",
    "dx.react.clientIsland.abi.capabilities",
    "readiness_release_ready",
    "browser_proof_status",
    "runtime_capabilities",
    "app_router_execution::dx_native_reactivity_capabilities()",
    "dx.tsx.dxNativeReactivityCapabilities",
    "unsupported_react_api_policy",
    "browser_proof_status",
    "full_react_hook_runtime",
    '"readiness_release_ready": false',
  ]);
});

test("production preview has concrete HTTP cache and range proof hooks", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");
  const preview = read("dx-www/src/cli/preview_contract.rs");
  const deploy = read("dx-www/src/cli/deploy_adapter_contract.rs");

  expectAll(preview, [
    "ETag",
    "If-None-Match",
    "304 Not Modified",
    "Last-Modified",
    "If-Modified-Since",
    "http_date_not_after",
    "Accept-Ranges: bytes",
    "206 Partial Content",
    "If-Range",
    "416 Range Not Satisfiable",
    "Content-Range: bytes */",
    "Content-Range: bytes",
    "ByteRangeSelection",
    "parse_single_byte_range",
    "Content-Encoding",
    "Accept-Encoding",
    "accepted_precompressed_asset_suffixes",
    "accept_encoding_quality",
    "Vary: Accept-Encoding",
    "production_contract_content_encoding",
    "production_contract_decoded_path",
    "server-action-method-not-allowed",
    "production-health-method-not-allowed",
    "server_action_failed_response",
    "receipt_written",
    "server_action_runtime::server_action_error_status(error)",
  ]);

  expectAll(readiness, [
    "Last-Modified and If-Modified-Since 304 responses",
    "If-Range gating for partial responses",
    "416 Range Not Satisfiable with Content-Range: bytes */length",
    "Vary: Accept-Encoding for encoded assets and negotiable plain assets",
    "source-owned Axum adapter parity delegates to the canonical production wire responder",
    "live Axum/server transport proof for the preview cache/range policy",
  ]);

  expectAll(deploy, [
    'decoded_path.starts_with(".dx/build-cache/source-routes/")',
    ".dx/build-cache/source-routes/root/modules/app-page.mjs",
    ".dx/build-cache/source-routes/root/index.dxpk",
  ]);
});

test("getting started docs describe the current App Router and proof workflow", () => {
  const gettingStarted = read("docs/getting-started.md");

  expectAll(gettingStarted, [
    "dx new my-app",
    "dx dev --host 127.0.0.1 --port 3000",
    "app/layout.tsx",
    "app/page.tsx",
    "app/api/**/route.ts",
    "emits only the root page",
    "Add routes only when the product needs them",
    "Browser receipts still need to be captured and imported",
    "styles/generated.css",
    ".dx/www/output",
    "dx check . --json",
    "dx www agent-context --json --full",
    "Static/no-JS HTML and CSS",
    "adapter-boundary islands",
    "not full React or Next.js runtime parity",
  ]);

  for (const staleMarker of [
    "dx init",
    "src/App.tsx",
    "HTIP binary",
    "dx.config.json",
    "runtime\": \"micro",
    "dx serve --port",
  ]) {
    assert.doesNotMatch(gettingStarted, new RegExp(staleMarker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});
