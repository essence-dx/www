import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");
const read = (relative: string) => fs.readFileSync(path.join(root, relative), "utf8");

function extractReadinessReplayCommands(source: string): string[] {
  const body = source.match(
    /pub\(crate\) fn readiness_replay_commands\(\) -> Vec<&'static str> \{\s*vec!\[([\s\S]*?)\]\s*\}/,
  )?.[1];
  assert.ok(body, "readiness_replay_commands() should stay source-owned and visible");
  return Array.from(body.matchAll(/"([^"\n]+)"/g), (match) => match[1]);
}

test("dx www agent-context exposes compact JSON handoff contract", () => {
  const mod = read("dx-www/src/cli/mod.rs");
  const cliCore = read("dx-www/src/cli/mod_parts/cli_core_impl.rs");
  const helpText = read("dx-www/src/cli/help_text.rs");
  const cli = `${mod}\n${cliCore}`;
  const agentContext = read("dx-www/src/cli/agent_context.rs");
  const readiness = read("dx-www/src/cli/readiness.rs");
  const agentContextContractSource = `${agentContext}\n${readiness}`;

  assert.match(mod, /mod agent_context;/);
  assert.match(cli, /"agent-context"\s*=>\s*\{/);
  assert.match(cli, /Some\("agent-context"\)\s*=>\s*agent_context::cmd_agent_context/);
  assert.match(helpText, /www agent-context --json\s+Compact agent handoff context/);

  for (const marker of [
    "dx.www.agent_context",
    "status_short",
    "dirty_count",
    "allowed_checks",
    "dx www readiness --json --full",
    "dx www readiness --write-receipts",
    "dx www readiness --write-visual-edit-replay --json",
    "dx www docs-doctor --json",
    "www_root::discover_www_repo_root(cwd)",
    ".dx/receipts/deploy/vercel.json",
    ".dx/receipts/devtools/visual-edit-latest.json",
    ".dx/receipts/devtools/visual-edit-latest.sr",
    ".dx/serializer/receipts-devtools-visual-edit-latest.machine",
    ".dx/receipts/readiness/native-events-latest.json",
    ".dx/receipts/readiness/native-events-latest.sr",
    ".dx/serializer/receipts-readiness-native-events-latest.machine",
    ".dx/receipts/readiness/no-js-artifact-latest.json",
    ".dx/receipts/readiness/no-js-artifact-latest.sr",
    ".dx/serializer/receipts-readiness-no-js-artifact-latest.machine",
    ".dx/receipts/readiness/no-js-browser-latest.json",
    ".dx/receipts/readiness/no-js-browser-latest.sr",
    ".dx/serializer/receipts-readiness-no-js-browser-latest.machine",
    "target/framework-comparison-20260531/throughput.json",
    "dx.www.same_machine_performance_receipt",
    ".dx/receipts/readiness/production-http-local-replay-latest.json",
    ".dx/receipts/readiness/production-http-local-replay-latest.sr",
    ".dx/serializer/receipts-readiness-production-http-local-replay-latest.machine",
    ".dx/receipts/readiness/production-http-tcp-preview-latest.json",
    ".dx/receipts/readiness/production-http-tcp-preview-latest.sr",
    ".dx/serializer/receipts-readiness-production-http-tcp-preview-latest.machine",
    "dx.www.readiness.production_http_local_replay_receipt_contract",
    "dx.www.readiness.production_http_tcp_preview_receipt_contract",
    "production_http_receipt_status",
    "production_http_receipt_stale_reasons",
    "production_http_missing_check_ids",
    "production_http_expected_check_ids",
    "production-http-local-replay-receipt-missing",
    "dx www readiness --write-receipts --json",
    "node --test benchmarks/dx-www-readiness-production-http-receipt.test.ts",
    "node --test benchmarks/dx-www-production-preview-tcp-receipt.test.ts",
    "node benchmarks/dx-www-production-preview-tcp-receipt.ts --dx-www-bin target/release/dx-www.exe --build-dir examples/template/.dx/www/output --out .dx/receipts/readiness/browser-import-candidates/production-http-tcp-preview-latest.json",
    "READINESS_PRODUCTION_HTTP_TCP_PREVIEW_COLLECT_COMMAND",
    "dx www readiness --import-production-http-tcp-preview-receipt <tcp-receipt.json> --json --full",
    "production_http_tcp_preview_current",
    "production_http_tcp_preview_stale_reason",
    "axum_responder_source_parity",
    "source-owned-axum-adapter-parity-current-local",
    "live Axum/server transport proof",
    "missing-or-failing-check-{check_id}",
    ".dx/receipts/readiness/bundle-partition-latest.json",
    ".dx/receipts/readiness/bundle-partition-latest.sr",
    ".dx/serializer/receipts-readiness-bundle-partition-latest.machine",
    "dx.www.readiness.bundle_partition_receipt_contract",
    "bundle_partition_receipt_status",
    "bundle_partition_receipt_stale_reasons",
    "bundle_partition_stale_fields",
    "bundle-partition-receipt-missing",
    "readiness-bundle-partition-machine-contract-missing",
    "node --test benchmarks/dx-www-tiny-static-public-partition-proof.test.ts",
    "stale-partition-field-{field}",
    ".dx/receipts/readiness/server-action-replay-ledger-latest.json",
    ".dx/receipts/readiness/server-action-replay-ledger-latest.sr",
    ".dx/serializer/receipts-readiness-server-action-replay-ledger-latest.machine",
    "dx.www.readiness.server_action_replay_ledger_receipt_contract",
    "server_action_replay_ledger_receipt_status",
    "server_action_replay_ledger_receipt_stale_reasons",
    "server-action-replay-ledger-receipt-missing",
    "readiness-server-action-replay-ledger-machine-contract-missing",
    "node --test benchmarks/server-action-replay-ledger-honesty.test.ts",
    "provider-hosted-replay-required-not-true",
    ".dx/receipts/readiness/primitive-proof-latest.json",
    ".dx/receipts/readiness/primitive-proof-latest.sr",
    ".dx/serializer/receipts-readiness-primitive-proof-latest.machine",
    "dx.www.readiness.primitive_proof_receipt_contract",
    "primitive_proof_receipt_status",
    "primitive_proof_receipt_stale_reasons",
    "primitive_proof_missing_primitives",
    "primitive-proof-receipt-missing",
    "readiness-primitive-proof-machine-contract-missing",
    "node --test benchmarks/dx-www-readiness-primitive-receipts.test.ts",
    "missing-or-stale-primitive-{primitive}",
    ".dx/receipts/readiness/native-event-browser-binder-latest.json",
    ".dx/receipts/readiness/native-event-browser-binder-latest.sr",
    ".dx/serializer/receipts-readiness-native-event-browser-binder-latest.machine",
    ".dx/receipts/readiness/state-runtime-browser-latest.json",
    ".dx/receipts/readiness/state-runtime-browser-latest.sr",
    ".dx/serializer/receipts-readiness-state-runtime-browser-latest.machine",
    ".dx/receipts/readiness/reactivity-model-latest.json",
    ".dx/receipts/readiness/reactivity-model-latest.sr",
    ".dx/serializer/receipts-readiness-reactivity-model-latest.machine",
    ".dx/receipts/readiness/docs-onboarding-latest.json",
    ".dx/receipts/readiness/docs-onboarding-latest.sr",
    ".dx/serializer/receipts-readiness-docs-onboarding-latest.machine",
    ".dx/receipts/readiness/island-abi-latest.json",
    ".dx/receipts/readiness/island-abi-latest.sr",
    ".dx/serializer/receipts-readiness-island-abi-latest.machine",
    "active_blockers",
    "receipt_paths",
    "readiness_contracts",
    "readiness_contracts",
    "dx.www.readiness.agent_context_contracts",
    "readiness-proof-graph-machine-contract-missing",
    "dx www readiness --write-receipts regenerates the root .sr source",
    "build-output deploy-adapter proof graph",
    "readiness-visual-edit-machine-contract-missing",
    "readiness-native-events-machine-contract-missing",
    "readiness-no-js-artifact-machine-contract-missing",
    "readiness-no-js-browser-machine-contract-missing",
    "readiness-native-event-browser-binder-machine-contract-missing",
    "readiness-state-runtime-browser-machine-contract-missing",
    "readiness-reactivity-model-machine-contract-missing",
    "readiness-docs-onboarding-machine-contract-missing",
    "readiness-island-abi-machine-contract-missing",
    "serializer-machine-proof-missing-or-stale",
    "source_contract",
    "machine_contract",
    "opaque-sr",
    "generated-machine-cache",
    "generated .machine cache is not older than the .sr source",
    "machine_fresh_against_source",
    "machine_cache_fresh_against_source",
    "modified_unix_ms",
    "content_blake3",
    "file_blake3_hex",
    "source content fingerprint available",
    "machine cache content fingerprint available",
    "json_parse_attempted",
    "legacy JSON read-model current until consumers migrate",
    "agent-context must not parse .sr or .machine as JSON",
    "next_safe_actions",
    "template-check-readiness-gate-stale",
    "devtools-visual-edit-receipt-missing",
    "native-event-catalog-receipt-missing",
    "tiny-static-no-js-artifact-receipt-missing",
    "same-machine-performance-receipt-missing",
    "same_machine_performance_receipt_status",
    "same_machine_performance_raceboard",
    "same_machine_performance_raceboard_from_receipt",
    "ranked_by",
    "www_vs_next_median_rps_ratio",
    "same_machine_performance_receipt_stale_reasons",
    "same_machine_receipt_has_measured_target",
    "same_machine_performance_binary_hash_current",
    "same_machine_performance_preflight_failures",
    "same_machine_performance_target_error_targets",
    "same-machine-performance-binary-hash-missing",
    "same-machine-performance-preflight-failed",
    "same-machine-performance-target-errors",
    "node benchmarks/dx-runtime-throughput-orchestrator.ts --mode all --jobs 6 --rounds 3 --requests 240 --concurrency 16 --out target/framework-comparison-20260531/throughput.json",
    "raw_replay_command",
    "node benchmarks/dx-runtime-throughput-benchmark.ts --rounds 3 --requests 240 --concurrency 16 --www-url http://127.0.0.1:42104/fair-counter --dx-www-bin target/release/dx-www.exe --out target/framework-comparison-20260531/throughput.json",
    "node benchmarks/dx-runtime-throughput-benchmark.ts --dry-run --rounds 2 --out target/framework-comparison-20260531/throughput-dry-run.json",
    "node --test benchmarks/dx-www-cdp-paint-receipt.test.ts",
    "node benchmarks/dx-www-cdp-paint-receipt.ts --url http://127.0.0.1:3000 --receipt-mode dev --out examples/template/.dx/receipts/check/web-perf/dev/report.json",
    "node benchmarks/dx-www-cdp-paint-receipt.ts --url http://127.0.0.1:4173 --receipt-mode static-build --out examples/template/.dx/receipts/check/web-perf/static-build/report.json",
    "source-owned-cdp-paint-receipts-current-lighthouse-parity-needed",
    "browser_runtime_executed",
    "metrics_complete",
    "dx check web-perf --url http://127.0.0.1:3000 --device desktop --receipt-mode dev --lighthouse --json",
    "dx check web-perf --url http://127.0.0.1:4173 --device desktop --receipt-mode static-build --lighthouse --json",
    "missing-measured-target-{target}",
    "native-event-browser-binder-receipt-missing",
    "state-runtime-browser-receipt-missing",
    "no-js-browser-receipt-missing",
    "island-abi-receipt-missing",
    "devtools_visual_edit_receipt_is_current",
    "receipt_durability",
    "receipt_write_status",
    "source_mutated",
    "undo_receipt_status",
    "browser_workbench_replay",
    "stale_reasons",
    "style-apply-not-applied",
    "style-undo-receipt-status-not-written",
    "native_event_catalog_receipt_is_current",
    "native_event_catalog_receipt_stale_reasons",
    "native-event-catalog-count-stale",
    "native-event-catalog-hash-stale",
    "expected_catalog_count",
    "expected_catalog_hash",
    "native_event_catalog_receipt_status_is_current",
    "no_js_artifact_receipt_status",
    "no_js_artifact_receipt_is_current",
    "missing-no-js-artifact-receipt",
    "artifact-current",
    "artifact_root",
    "artifact_source",
    "artifact_path_resolution",
    "artifact_html_blake3",
    "meaningful_html_without_js",
    "schema_revision",
    "tiny-static-no-js-artifact",
    "route_unit_present",
    "route_unit_no_js_capable",
    "live_browser_executed",
    "javascript_disabled_browser",
    "live_astro_parity_receipt",
    "no_js_browser_receipt_status",
    "no_js_browser_receipt_is_current",
    "no_js_browser_receipt_stale_reasons",
    "no_js_browser_artifact_hash_matches",
    "current-local-js-disabled-browser-proof",
    "NO_JS_BROWSER_COLLECT_COMMAND",
    "collector_command",
    "node benchmarks/dx-www-no-js-browser-receipt.ts --html-path examples/template/.dx/www/output/app/index.html",
    "node --test benchmarks/dx-www-no-js-browser-receipt.test.ts",
    "dx www readiness --import-no-js-browser-receipt <browser-receipt.json> --json --full",
    "BROWSER_RECEIPT_HARNESS_PAGE_SNAPSHOT_COMMAND",
    "BROWSER_RECEIPT_HARNESS_DOM_SNAPSHOT_COMMAND",
    "node benchmarks/dx-www-readiness-browser-receipt-harness.ts --print-page-collector",
    "node benchmarks/dx-www-readiness-browser-receipt-harness.ts --print-dom-page-collector",
    "harness_page_snapshot_command",
    "harness_dom_snapshot_command",
    "page_snapshot_capture_command",
    "dom_snapshot_capture_command",
    "snapshot_capture_modes",
    "full-replay-page-collector",
    "read-only-dom-after-browser-interactions",
    "no-js-browser-artifact-hash-mismatch",
    "no-js-browser-execution-flags-invalid",
    "no-js-browser-static-markers-invalid",
    "no-js-browser-meaningful-html-incomplete",
    "native_event_browser_binder_receipt_status",
    "native_event_browser_binder_receipt_is_current",
    "browser-binder-replay-current",
    "browser-binder-replay-stale",
    "state_runtime_browser_receipt_status",
    "state_runtime_browser_receipt_is_current",
    "state-runtime-browser-replay-current",
    "state-runtime-browser-replay-stale",
    "CANONICAL_STATE_RUNTIME_ROUTE",
    '"/state-runtime"',
    "CANONICAL_STATE_RUNTIME_SOURCE",
    "examples/template/proof-routes/state-runtime/page.tsx",
    "CANONICAL_STATE_RUNTIME_COMPONENT_SOURCE",
    "examples/template/components/state-runtime-probe.tsx",
    "STATE_RUNTIME_BROWSER_CANDIDATE_RECEIPT",
    ".dx/receipts/readiness/browser-import-candidates/state-runtime-browser-latest.json",
    "canonical_browser_proof_target",
    "snapshot_claims_proof",
    "import_validation_required",
    "dx www readiness --import-browser-page-snapshot <page-snapshot.json> --json --full",
    "reactivity_model_receipt_status",
    "reactivity_model_receipt_is_current",
    "source-owned-reactivity-model-foundation-current",
    "missing-reactivity-model-receipt",
    "reactivity-model-receipt-missing",
    "docs_onboarding_receipt_status",
    "docs_onboarding_receipt_is_current",
    "source-owned-docs-onboarding-foundation-current",
    'receipt.get("source_check_count").and_then(Value::as_u64) == Some(6)',
    'receipt.get("source_check_count").and_then(Value::as_u64) == Some(5)',
    "docs_doctor_report_evaluated",
    "docs_doctor_error_count",
    "generated_archived_warning_finding_count",
    "missing-docs-onboarding-receipt",
    "docs-onboarding-receipt-missing",
    "docs-onboarding-generated-archived-warning-cleanup",
    "generated_archived_warning_surfaces_clean",
    "generated_archived_warning_surfaces_promoted",
    "docs_doctor_command",
    "docs-doctor command replay remain separate gates",
    "island_abi_receipt_status",
    "island_abi_receipt_is_current",
    "island_abi_receipt_stale_reasons",
    "island_browser_receipt_status",
    "island_browser_receipt_stale_reasons",
    "source-owned-island-browser-replay-current",
    "missing-island-browser-receipt",
    "island-browser-receipt-missing",
    "readiness-island-browser-machine-contract-missing",
    "dx www readiness --import-island-browser-receipt <browser-receipt.json> --json --full",
    "CANONICAL_ISLANDS_ROUTE",
    '"/islands"',
    "CANONICAL_ISLANDS_SOURCE",
    "examples/template/proof-routes/islands/page.tsx",
    "CANONICAL_ISLANDS_COMPONENT_SOURCE",
    "examples/template/components/island-runtime-probe.tsx",
    "ISLAND_BROWSER_CANDIDATE_RECEIPT",
    ".dx/receipts/readiness/browser-import-candidates/island-browser-latest.json",
    "source-owned-island-abi-foundation-current",
    "missing-island-abi-receipt",
    "camelCase-jsx-props",
    "node --test benchmarks/dx-www-islands-abi-camelcase.test.ts",
    "missing-core-directive-{directive}",
    "island-abi-receipt-missing",
    "compiler-catalog-valid-mdn-current",
    "mdn_event_count",
    "compiler_event_count",
    "missing_from_compiler_count",
    "extra_in_compiler_count",
    "mdn_exact_match",
    "dx.www.readiness.visual_edit_workbench_receipt_contract",
    "dx.www.readiness.native_event_catalog_receipt_contract",
    "dx.www.readiness.state_runtime_browser_receipt_contract",
    "dx.www.readiness.reactivity_model_receipt_contract",
    "dx.www.readiness.docs_onboarding_receipt_contract",
    "dx.www.readiness.no_js_artifact_receipt_contract",
    "dx.www.readiness.no_js_browser_receipt_contract",
    "dx.www.readiness.island_abi_receipt_contract",
    "dx.www.readiness.island_browser_receipt_contract",
    "VISUAL_EDIT_BROWSER_CANDIDATE_RECEIPT",
    ".dx/receipts/readiness/browser-import-candidates/visual-edit-browser-workbench-latest.json",
    "NO_JS_BROWSER_CANDIDATE_RECEIPT",
    ".dx/receipts/readiness/browser-import-candidates/no-js-browser-latest.json",
    "readiness_receipt_gate_status",
    "metadata_current",
    "replay_verified_current",
    "static-advisory-not-release-proof",
    "missing-or-unsafe-readiness-gate-metadata",
    "release_ready",
    "fastest_world_claim",
    "score_kind",
    "readiness_gate_status",
    "readiness_agent_context_for_project",
    "readiness::readiness_agent_context_for_project(full, Some(cwd))",
    "readiness_gate_status_for_project(project)",
    "readiness_replay_commands",
    "readiness_replay_commands",
    "readiness::readiness_replay_commands()",
    "missing_replay_commands",
    "verified_from_replay_receipts",
    "receipt_freshness",
    "replay_commands",
    "relative_release_ready",
    "release_ready_scope",
    "remaining_proof_gates",
    "fastest_world_claim",
    "fastest_world_claim\": false",
    "check_score_value",
    "check_score_max",
    "launch_floor",
    "dx check web-perf --url http://127.0.0.1:3000 --device desktop --receipt-mode dev --json",
    "dx check web-perf --url http://127.0.0.1:3000 --device desktop --receipt-mode static-build --json",
  ]) {
    assert.match(
      agentContextContractSource,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
  }

  assert.match(
    agentContext,
    /"state-runtime-browser" => json!\(\{[\s\S]*"route": CANONICAL_STATE_RUNTIME_ROUTE[\s\S]*"starter_source_file": CANONICAL_STATE_RUNTIME_SOURCE[\s\S]*"candidate_receipt": STATE_RUNTIME_BROWSER_CANDIDATE_RECEIPT[\s\S]*"import_command": STATE_RUNTIME_BROWSER_IMPORT_COMMAND[\s\S]*"snapshot_claims_proof": false/,
    "agent-context should expose the exact /state-runtime browser snapshot target without claiming proof",
  );
  assert.match(
    agentContext,
    /"island-browser" => json!\(\{[\s\S]*"route": CANONICAL_ISLANDS_ROUTE[\s\S]*"starter_source_file": CANONICAL_ISLANDS_SOURCE[\s\S]*"candidate_receipt": ISLAND_BROWSER_CANDIDATE_RECEIPT[\s\S]*"import_command": ISLAND_BROWSER_IMPORT_COMMAND[\s\S]*"snapshot_claims_proof": false/,
    "agent-context should expose the exact /islands browser snapshot target without claiming proof",
  );
  for (const [statusFunction, importCommand] of [
    [
      "state_runtime_browser_receipt_status",
      "dx www readiness --import-state-runtime-browser-receipt <browser-receipt.json> --json --full",
    ],
    [
      "no_js_browser_receipt_status",
      "dx www readiness --import-no-js-browser-receipt <browser-receipt.json> --json --full",
    ],
    [
      "island_browser_receipt_status",
      "dx www readiness --import-island-browser-receipt <browser-receipt.json> --json --full",
    ],
    [
      "devtools_visual_edit_receipt_status",
      "dx www readiness --import-visual-edit-browser-receipt <browser-receipt.json> --json --full",
    ],
  ] as const) {
    assert.match(
      agentContext,
      new RegExp(
        `fn ${statusFunction}[\\s\\S]*${importCommand.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}`,
      ),
      `${statusFunction} should expose the current import command`,
    );
  }

  assert.match(
    agentContext,
    /receipt\["path"\] == "\.dx\/receipts\/check\/web-perf\/dev\/report\.json"[\s\S]*receipt\["present"\] == true/,
  );
  assert.match(readiness, /pub\(crate\) fn readiness_gate_status\(\) -> Value/);
  assert.match(readiness, /"score_kind": READINESS_SCORE_KIND/);
  assert.match(readiness, /"verified_from_replay_receipts": true/);
  assert.match(readiness, /"public-vs-evidence-bundle"/);
  assert.match(readiness, /"production-http-preview"/);
  assert.ok(
    extractReadinessReplayCommands(readiness).length >= 30,
    "agent-context should consume the full release-readiness replay command list",
  );
  assert.doesNotMatch(
    agentContext,
    /for \(field, command\) in \[/,
    "agent-context should not validate release-readiness replay freshness from a hard-coded subset",
  );
});

test("dx check writes a latest receipt that uses the current 500-point panel contract", () => {
  const mod = read("dx-www/src/cli/mod.rs");
  const cliCore = read("dx-www/src/cli/mod_parts/cli_core_impl.rs");
  const cliForgeC = read("dx-www/src/cli/mod_parts/cli_forge_commands_c.rs");
  const cli = `${mod}\n${cliCore}\n${cliForgeC}`;
  const latestReceipt = read("dx-www/src/cli/dx_check_latest_receipt.rs");
  const panel = read("core/src/ecosystem/dx_check_receipt/panel.rs");
  const agentContext = read("dx-www/src/cli/agent_context.rs");
  const docsDoctor = read("dx-www/src/cli/docs_doctor.rs");
  const starterCheck = JSON.parse(
    read("examples/template/.dx/receipts/check/check-latest.json"),
  );

  assert.match(mod, /mod dx_check_latest_receipt;/);
  assert.match(cli, /write_dx_check_latest_receipt\(&path, &report\)\?/);

  for (const marker of [
    "DX_CHECK_LATEST_RECEIPT_PATH",
    "DX_CHECK_ZED_PANEL_SCHEMA_VERSION",
    "DX_CHECK_WEIGHT_PROFILE",
    "score_value",
    "score_max",
    "score_percent",
    "project_health_score",
    "project_health_score_max",
    "project_health_score_percent",
    "project_health_score_estimated",
    "dx_check_score",
    "dx_check_score_max",
    "dx_check_score_percent",
    "dx_check_score_estimated",
    "readiness_score",
    "readiness_score_max",
    "readiness_score_kind",
    "readiness_score_estimated",
    "current_honest_score",
    "readiness::readiness_gate_status_for_project(Some(project))",
    "readiness_gate_status",
    "readiness_replay_commands",
    "readiness_evidence_freshness",
    "release-readiness-evidence-not-dx-check-health",
    "visual_edit_stale_reason",
    "missing_proof_gate_count",
    "release_claim_allowed",
    "global_speed_claim_allowed",
    "missing_proof_gates",
    "remaining_proof_gates",
    "relative_release_ready",
    "release_ready_scope",
    "readiness_replay_commands",
    "www-release-readiness-gate-blocked",
    "readiness_release_gate_ready",
    "zed_status(report.traffic, blocker_findings.len())",
    "\"release_ready\": readiness_release_gate_ready",
    "fastest_world_claim",
    "fastest_world_claim\": false",
    "dx check --latest-receipt --json",
  ]) {
    assert.match(latestReceipt, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(latestReceipt, /u16::from\(report\.score\)\.saturating_mul\(5\)/);
  assert.match(
    latestReceipt,
    /let readiness_score = readiness_gate_status[\s\S]*\.get\("current_honest_score"\)/,
  );
  assert.match(
    latestReceipt,
    /"project_health_score": score_value[\s\S]*"dx_check_score": score_value[\s\S]*"readiness_score": readiness_score/,
  );
  assert.equal(starterCheck.project_health_score, starterCheck.score);
  assert.equal(starterCheck.project_health_score_max, 500);
  assert.equal(starterCheck.dx_check_score, starterCheck.score);
  assert.equal(starterCheck.dx_check_score_max, 500);
  assert.equal(starterCheck.readiness_score, starterCheck.readiness_gate_status.current_honest_score);
  assert.equal(starterCheck.readiness_score_max, 100);
  assert.ok(
    ["static-advisory-not-release-proof", "relative-local-proof-backed-release-ready"].includes(
      starterCheck.readiness_score_kind,
    ),
  );
  assert.equal(starterCheck.readiness_score_estimated, false);
  assert.match(panel, /pub readiness_gate_status: Option<Value>/);
  assert.match(panel, /pub readiness_replay_commands: Vec<String>/);
  assert.match(panel, /readiness_gate_status: None/);
  assert.match(panel, /latest_receipt_readiness_gate_metadata_current/);
  assert.match(cliForgeC, /serde_json::to_value\(&panel\)/);
  assert.match(cliForgeC, /readiness::readiness_gate_status\(\)/);
  assert.match(cliForgeC, /static_readiness_gate_advisory/);
  assert.match(cliForgeC, /static_readiness_replay_commands/);
  assert.match(cliForgeC, /entry\("readiness_replay_commands"\.to_string\(\)\)/);
  assert.match(cliForgeC, /entry\("replay_commands"\.to_string\(\)\)/);
  assert.doesNotMatch(
    cliForgeC,
    /\.insert\(\s*"readiness_gate_status"\.to_string\(\),\s*readiness::readiness_gate_status\(\),\s*\)/,
  );
  assert.match(cliForgeC, /"fastest_world_claim"\.to_string\(\), serde_json::json!\(false\)/);
  assert.doesNotMatch(`${latestReceipt}\n${cliForgeC}`, /fastest_world_claim":\s*true/);

  for (const [name, source] of [
    ["panel", panel],
    ["agent-context", agentContext],
    ["docs-doctor", docsDoctor],
  ] as const) {
    assert.match(
      source,
      /verified_from_replay_receipts[\s\S]*Some\(false\)/,
      `${name} must reject static-advisory receipts that pretend replay proof is current`,
    );
    assert.match(
      source,
      /static_advisory_receipt_freshness_safe/,
      `${name} must accept only explicitly safe static-advisory receipt freshness states`,
    );
    assert.match(
      source,
      /stale_reasons/,
      `${name} must keep unsafe release-readiness gate reasons visible`,
    );
    assert.match(
      source,
      /verified-from-replay-receipts-unsafe-for-(?:static-advisory|readiness)/,
      `${name} must explain unsafe replay-current metadata`,
    );
    assert.match(
      source,
      /receipt-freshness-unsafe-for-(?:static-advisory|readiness)/,
      `${name} must explain unsafe freshness metadata`,
    );
  }

  for (const [name, source] of [
    ["agent-context", agentContext],
    ["docs-doctor", docsDoctor],
  ] as const) {
    assert.match(
      source,
      /claimed_release_ready/,
      `${name} must keep raw release-ready claims as evidence instead of safe status`,
    );
    assert.match(
      source,
      /claimed_gate_release_ready/,
      `${name} must keep raw gate release-ready claims as evidence instead of safe status`,
    );
  }
});
