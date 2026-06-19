use std::path::Path;
use std::process::Command;

use chrono::Utc;
use serde_json::{Value, json};

use crate::error::{DxError, DxResult};

use super::{docs_doctor, readiness, www_root};

const DEVTOOLS_VISUAL_EDIT_RECEIPT: &str = readiness::READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT;
const DEVTOOLS_VISUAL_EDIT_RECEIPT_SCHEMA: &str =
    readiness::READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_CONTRACT;
const DEVTOOLS_VISUAL_EDIT_RECEIPT_SR: &str = readiness::READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_SR;
const DEVTOOLS_VISUAL_EDIT_RECEIPT_MACHINE: &str =
    readiness::READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_MACHINE;
const NATIVE_EVENT_CATALOG_RECEIPT: &str = readiness::READINESS_NATIVE_EVENT_CATALOG_RECEIPT;
const NATIVE_EVENT_CATALOG_RECEIPT_SCHEMA: &str =
    readiness::READINESS_NATIVE_EVENT_CATALOG_RECEIPT_CONTRACT;
const NATIVE_EVENT_CATALOG_RECEIPT_SR: &str = readiness::READINESS_NATIVE_EVENT_CATALOG_RECEIPT_SR;
const NATIVE_EVENT_CATALOG_RECEIPT_MACHINE: &str =
    readiness::READINESS_NATIVE_EVENT_CATALOG_RECEIPT_MACHINE;
const NO_JS_ARTIFACT_RECEIPT: &str = readiness::READINESS_NO_JS_ARTIFACT_RECEIPT;
const NO_JS_ARTIFACT_RECEIPT_SCHEMA: &str = readiness::READINESS_NO_JS_ARTIFACT_RECEIPT_CONTRACT;
const NO_JS_ARTIFACT_RECEIPT_SR: &str = readiness::READINESS_NO_JS_ARTIFACT_RECEIPT_SR;
const NO_JS_ARTIFACT_RECEIPT_MACHINE: &str = readiness::READINESS_NO_JS_ARTIFACT_RECEIPT_MACHINE;
const NO_JS_BROWSER_RECEIPT: &str = readiness::READINESS_NO_JS_BROWSER_RECEIPT;
const NO_JS_BROWSER_RECEIPT_SCHEMA: &str = readiness::READINESS_NO_JS_BROWSER_RECEIPT_CONTRACT;
const NO_JS_BROWSER_RECEIPT_SR: &str = readiness::READINESS_NO_JS_BROWSER_RECEIPT_SR;
const NO_JS_BROWSER_RECEIPT_MACHINE: &str = readiness::READINESS_NO_JS_BROWSER_RECEIPT_MACHINE;
const SAME_MACHINE_PERFORMANCE_RECEIPT: &str =
    readiness::READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT;
const SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT: &str =
    readiness::READINESS_SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT;
const SAME_MACHINE_PERFORMANCE_RECEIPT_SCHEMA: &str =
    readiness::READINESS_SAME_MACHINE_PERFORMANCE_SCHEMA;
const SAME_MACHINE_PERFORMANCE_RECEIPT_SR: &str =
    readiness::READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_SR;
const SAME_MACHINE_PERFORMANCE_RECEIPT_MACHINE: &str =
    readiness::READINESS_SAME_MACHINE_PERFORMANCE_RECEIPT_MACHINE;
const PRODUCTION_HTTP_RECEIPT: &str = readiness::READINESS_PRODUCTION_HTTP_RECEIPT;
const PRODUCTION_HTTP_RECEIPT_SCHEMA: &str = readiness::READINESS_PRODUCTION_HTTP_RECEIPT_CONTRACT;
const PRODUCTION_HTTP_RECEIPT_SR: &str = readiness::READINESS_PRODUCTION_HTTP_RECEIPT_SR;
const PRODUCTION_HTTP_RECEIPT_MACHINE: &str = readiness::READINESS_PRODUCTION_HTTP_RECEIPT_MACHINE;
const PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT: &str =
    readiness::READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT;
const PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_SCHEMA: &str =
    readiness::READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_CONTRACT;
const PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_SR: &str =
    readiness::READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_SR;
const PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_MACHINE: &str =
    readiness::READINESS_PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_MACHINE;
const BUNDLE_PARTITION_RECEIPT: &str = readiness::READINESS_BUNDLE_PARTITION_RECEIPT;
const BUNDLE_PARTITION_RECEIPT_SCHEMA: &str =
    readiness::READINESS_BUNDLE_PARTITION_RECEIPT_CONTRACT;
const BUNDLE_PARTITION_RECEIPT_SR: &str = readiness::READINESS_BUNDLE_PARTITION_RECEIPT_SR;
const BUNDLE_PARTITION_RECEIPT_MACHINE: &str =
    readiness::READINESS_BUNDLE_PARTITION_RECEIPT_MACHINE;
const BUNDLE_PROVIDER_REPLAY_RECEIPT: &str = readiness::READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT;
const BUNDLE_PROVIDER_REPLAY_RECEIPT_SCHEMA: &str =
    readiness::READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_CONTRACT;
const BUNDLE_PROVIDER_REPLAY_RECEIPT_SR: &str =
    readiness::READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_SR;
const BUNDLE_PROVIDER_REPLAY_RECEIPT_MACHINE: &str =
    readiness::READINESS_BUNDLE_PROVIDER_REPLAY_RECEIPT_MACHINE;
const SERVER_ACTION_REPLAY_LEDGER_RECEIPT: &str =
    readiness::READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT;
const SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SCHEMA: &str =
    readiness::READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_CONTRACT;
const SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SR: &str =
    readiness::READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SR;
const SERVER_ACTION_REPLAY_LEDGER_RECEIPT_MACHINE: &str =
    readiness::READINESS_SERVER_ACTION_REPLAY_LEDGER_RECEIPT_MACHINE;
const PRIMITIVE_PROOF_RECEIPT: &str = readiness::READINESS_PRIMITIVE_PROOF_RECEIPT;
const PRIMITIVE_PROOF_RECEIPT_SCHEMA: &str = readiness::READINESS_PRIMITIVE_PROOF_RECEIPT_CONTRACT;
const PRIMITIVE_PROOF_RECEIPT_SR: &str = readiness::READINESS_PRIMITIVE_PROOF_RECEIPT_SR;
const PRIMITIVE_PROOF_RECEIPT_MACHINE: &str = readiness::READINESS_PRIMITIVE_PROOF_RECEIPT_MACHINE;
const NATIVE_EVENT_BROWSER_BINDER_RECEIPT: &str =
    readiness::READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT;
const NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SCHEMA: &str =
    readiness::READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT;
const NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR: &str =
    readiness::READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR;
const NATIVE_EVENT_BROWSER_BINDER_RECEIPT_MACHINE: &str =
    readiness::READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_MACHINE;
const STATE_RUNTIME_BROWSER_RECEIPT: &str = readiness::READINESS_STATE_RUNTIME_BROWSER_RECEIPT;
const STATE_RUNTIME_BROWSER_RECEIPT_SCHEMA: &str =
    readiness::READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT;
const STATE_RUNTIME_BROWSER_RECEIPT_SR: &str =
    readiness::READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR;
const STATE_RUNTIME_BROWSER_RECEIPT_MACHINE: &str =
    readiness::READINESS_STATE_RUNTIME_BROWSER_RECEIPT_MACHINE;
const REACTIVITY_MODEL_RECEIPT: &str = readiness::READINESS_REACTIVITY_MODEL_RECEIPT;
const REACTIVITY_MODEL_RECEIPT_SCHEMA: &str =
    readiness::READINESS_REACTIVITY_MODEL_RECEIPT_CONTRACT;
const REACTIVITY_MODEL_RECEIPT_SR: &str = readiness::READINESS_REACTIVITY_MODEL_RECEIPT_SR;
const REACTIVITY_MODEL_RECEIPT_MACHINE: &str =
    readiness::READINESS_REACTIVITY_MODEL_RECEIPT_MACHINE;
const DOCS_ONBOARDING_RECEIPT: &str = readiness::READINESS_DOCS_ONBOARDING_RECEIPT;
const DOCS_ONBOARDING_RECEIPT_SCHEMA: &str = readiness::READINESS_DOCS_ONBOARDING_RECEIPT_CONTRACT;
const DOCS_ONBOARDING_RECEIPT_SR: &str = readiness::READINESS_DOCS_ONBOARDING_RECEIPT_SR;
const DOCS_ONBOARDING_RECEIPT_MACHINE: &str = readiness::READINESS_DOCS_ONBOARDING_RECEIPT_MACHINE;
const DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT: &str = docs_doctor::DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT;
const DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_SCHEMA: &str =
    docs_doctor::DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_CONTRACT;
const DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_SR: &str =
    docs_doctor::DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_SR;
const DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_MACHINE: &str =
    docs_doctor::DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_MACHINE;
const ISLAND_ABI_RECEIPT: &str = readiness::READINESS_ISLAND_ABI_RECEIPT;
const ISLAND_ABI_RECEIPT_SCHEMA: &str = readiness::READINESS_ISLAND_ABI_RECEIPT_CONTRACT;
const ISLAND_ABI_RECEIPT_SR: &str = readiness::READINESS_ISLAND_ABI_RECEIPT_SR;
const ISLAND_ABI_RECEIPT_MACHINE: &str = readiness::READINESS_ISLAND_ABI_RECEIPT_MACHINE;
const ISLAND_BROWSER_RECEIPT: &str = readiness::READINESS_ISLAND_BROWSER_RECEIPT;
const ISLAND_BROWSER_RECEIPT_SCHEMA: &str = readiness::READINESS_ISLAND_BROWSER_RECEIPT_CONTRACT;
const ISLAND_BROWSER_RECEIPT_SR: &str = readiness::READINESS_ISLAND_BROWSER_RECEIPT_SR;
const ISLAND_BROWSER_RECEIPT_MACHINE: &str = readiness::READINESS_ISLAND_BROWSER_RECEIPT_MACHINE;
const TEMPLATE_CHECK_RECEIPT: &str = "examples/template/.dx/receipts/check/check-latest.json";
const READINESS_PROOF_GRAPH_RECEIPT: &str = readiness::READINESS_PROOF_GRAPH_RECEIPT;
const READINESS_PROOF_GRAPH_RECEIPT_SCHEMA: &str = readiness::READINESS_PROOF_GRAPH_SCHEMA;
const READINESS_PROOF_GRAPH_RECEIPT_MACHINE: &str =
    readiness::READINESS_PROOF_GRAPH_RECEIPT_MACHINE;
const READINESS_INSPECT_COMMAND: &str = "dx www readiness --json --full";
const READINESS_WRITE_RECEIPTS_COMMAND: &str = "dx www readiness --write-receipts --json";
const BROWSER_RECEIPT_HARNESS_TEST_COMMAND: &str =
    "node --test benchmarks/dx-www-readiness-browser-receipt-harness.test.ts";
const BROWSER_RECEIPT_HARNESS_PAGE_SNAPSHOT_COMMAND: &str =
    "node benchmarks/dx-www-readiness-browser-receipt-harness.ts --print-page-collector";
const BROWSER_RECEIPT_HARNESS_DOM_SNAPSHOT_COMMAND: &str =
    "node benchmarks/dx-www-readiness-browser-receipt-harness.ts --print-dom-page-collector";
const BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND: &str =
    BROWSER_RECEIPT_HARNESS_PAGE_SNAPSHOT_COMMAND;
const BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND: &str = "node benchmarks/dx-www-readiness-browser-receipt-harness.ts --from-page-json <page-snapshot.json> --out-dir .dx/receipts/readiness/browser-import-candidates";
const BROWSER_PAGE_SNAPSHOT_IMPORT_COMMAND: &str =
    "dx www readiness --import-browser-page-snapshot <page-snapshot.json> --json --full";
const BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR: &str =
    ".dx/receipts/readiness/browser-import-candidates";
const CANONICAL_STARTER_BROWSER_ORIGIN: &str = "http://127.0.0.1:<port>";
const CANONICAL_STATE_RUNTIME_ROUTE: &str = "/state-runtime";
const CANONICAL_ISLANDS_ROUTE: &str = "/islands";
const CANONICAL_HOME_ROUTE: &str = "/";
const CANONICAL_STATE_RUNTIME_SOURCE: &str =
    "examples/template/proof-routes/state-runtime/page.tsx";
const CANONICAL_STATE_RUNTIME_COMPONENT_SOURCE: &str =
    "examples/template/components/state-runtime-probe.tsx";
const CANONICAL_ISLANDS_SOURCE: &str = "examples/template/proof-routes/islands/page.tsx";
const CANONICAL_ISLANDS_COMPONENT_SOURCE: &str =
    "examples/template/components/island-runtime-probe.tsx";
const CANONICAL_NO_JS_OUTPUT_HTML: &str = "examples/template/.dx/www/output/app/index.html";
const STATE_RUNTIME_BROWSER_CANDIDATE_RECEIPT: &str =
    ".dx/receipts/readiness/browser-import-candidates/state-runtime-browser-latest.json";
const ISLAND_BROWSER_CANDIDATE_RECEIPT: &str =
    ".dx/receipts/readiness/browser-import-candidates/island-browser-latest.json";
const VISUAL_EDIT_BROWSER_CANDIDATE_RECEIPT: &str =
    ".dx/receipts/readiness/browser-import-candidates/visual-edit-browser-workbench-latest.json";
const NO_JS_BROWSER_CANDIDATE_RECEIPT: &str =
    ".dx/receipts/readiness/browser-import-candidates/no-js-browser-latest.json";
const VISUAL_EDIT_BROWSER_IMPORT_COMMAND: &str =
    "dx www readiness --import-visual-edit-browser-receipt <browser-receipt.json> --json --full";
const NATIVE_EVENT_BROWSER_BINDER_IMPORT_COMMAND: &str = "dx www readiness --import-native-event-browser-binder-receipt <browser-receipt.json> --json --full";
const STATE_RUNTIME_BROWSER_IMPORT_COMMAND: &str =
    "dx www readiness --import-state-runtime-browser-receipt <browser-receipt.json> --json --full";
const NO_JS_BROWSER_IMPORT_COMMAND: &str =
    "dx www readiness --import-no-js-browser-receipt <browser-receipt.json> --json --full";
const ISLAND_BROWSER_IMPORT_COMMAND: &str =
    "dx www readiness --import-island-browser-receipt <browser-receipt.json> --json --full";
const NO_JS_BROWSER_COLLECT_COMMAND: &str = readiness::READINESS_NO_JS_BROWSER_COLLECT_COMMAND;
const VISUAL_EDIT_FOUNDATION_REPLAY_COMMAND: &str =
    "dx www readiness --write-visual-edit-replay --json";
const SAME_MACHINE_PERFORMANCE_REPLAY_COMMAND: &str =
    readiness::READINESS_SAME_MACHINE_PERFORMANCE_REPLAY_COMMAND;
const SAME_MACHINE_PERFORMANCE_RAW_REPLAY_COMMAND: &str =
    readiness::READINESS_SAME_MACHINE_PERFORMANCE_RAW_REPLAY_COMMAND;
const SAME_MACHINE_PERFORMANCE_DRY_RUN_COMMAND: &str =
    readiness::READINESS_SAME_MACHINE_PERFORMANCE_DRY_RUN_COMMAND;
const SAME_MACHINE_PERFORMANCE_IMPORT_COMMAND: &str =
    readiness::READINESS_SAME_MACHINE_PERFORMANCE_IMPORT_COMMAND;
const PRODUCTION_HTTP_REPLAY_COMMAND: &str = "dx www readiness --write-receipts --json";
const PRODUCTION_HTTP_CONTRACT_TEST_COMMAND: &str =
    "node --test benchmarks/dx-www-readiness-production-http-receipt.test.ts";
const BUNDLE_PARTITION_CONTRACT_TEST_COMMAND: &str =
    "node --test benchmarks/dx-www-tiny-static-public-partition-proof.test.ts";
const SERVER_ACTION_REPLAY_LEDGER_CONTRACT_TEST_COMMAND: &str =
    "node --test benchmarks/server-action-replay-ledger-honesty.test.ts";
const PRIMITIVE_PROOF_CONTRACT_TEST_COMMAND: &str =
    "node --test benchmarks/dx-www-readiness-primitive-receipts.test.ts";
const ISLAND_ABI_CONTRACT_TEST_COMMAND: &str =
    "node --test benchmarks/dx-www-islands-abi-camelcase.test.ts";

pub(super) fn cmd_agent_context(cwd: &Path, args: &[String]) -> DxResult<()> {
    let mut json_output = false;
    let mut full = false;
    for arg in args {
        match arg.as_str() {
            "--json" | "--format=json" => json_output = true,
            "--full" => full = true,
            "--help" | "-h" => {
                eprintln!("dx www agent-context --json [--full]");
                eprintln!(
                    "    Print compact machine-readable context for agents working on DX-WWW."
                );
                eprintln!(
                    "    Use --full to include the release readiness proof graph, event catalog, delivery tiers, and bundle split."
                );
                return Ok(());
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!("Unknown dx www agent-context option: {value}"),
                    field: Some("www agent-context".to_string()),
                });
            }
        }
    }

    let report = build_agent_context_report(&www_root::discover_www_repo_root(cwd), full);
    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).map_err(|error| {
                DxError::ConfigValidationError {
                    message: format!("Failed to render agent context JSON: {error}"),
                    field: Some("www agent-context".to_string()),
                }
            })?
        );
        return Ok(());
    }

    println!("DX-WWW agent context");
    println!(
        "Workspace: {}",
        report["workspace"]["path"].as_str().unwrap_or("")
    );
    println!(
        "Branch: {}",
        report["workspace"]["branch"].as_str().unwrap_or("unknown")
    );
    println!(
        "Dirty files: {}",
        report["workspace"]["dirty_count"]
            .as_u64()
            .unwrap_or_default()
    );
    println!(
        "Next action: {}",
        report["next_safe_actions"]
            .as_array()
            .and_then(|actions| actions.first())
            .and_then(Value::as_str)
            .unwrap_or("Run --json for full context.")
    );
    Ok(())
}

fn build_agent_context_report(cwd: &Path, full: bool) -> Value {
    let workspace = git_workspace_snapshot(cwd);
    let lane_state = lane_state_snapshot(cwd);
    let receipt_paths = receipt_path_statuses(cwd);
    let browser_receipt_actions = browser_receipt_actions(&receipt_paths);
    let readiness_contracts = readiness_contract_statuses(cwd, &receipt_paths);
    let active_blockers = active_blockers(cwd, &receipt_paths, &readiness_contracts);
    let next_safe_actions = next_safe_actions(&active_blockers);
    let readiness = readiness::readiness_agent_context_for_project(full, Some(cwd));
    let readiness_summary = readiness
        .get("readiness_summary")
        .cloned()
        .unwrap_or(Value::Null);
    let readiness_full = readiness
        .get("readiness_full")
        .cloned()
        .unwrap_or(Value::Null);
    let readiness_gate_status = readiness
        .get("readiness_gate_status")
        .cloned()
        .unwrap_or_else(readiness::readiness_gate_status);
    let readiness_replay_commands = readiness
        .get("readiness_replay_commands")
        .cloned()
        .unwrap_or_else(|| json!(readiness::readiness_replay_commands()));
    let missing_proof_gates = readiness_gate_status
        .get("missing_proof_gates")
        .cloned()
        .unwrap_or_else(|| json!([]));
    let remaining_proof_gates = readiness_gate_status
        .get("remaining_proof_gates")
        .cloned()
        .unwrap_or_else(|| json!([]));
    let release_ready = readiness_gate_status
        .get("release_ready")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let release_claim_allowed = readiness_gate_status
        .get("release_claim_allowed")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    json!({
        "schema": "dx.www.agent_context",
        "format": 1,
        "command": if full { "dx www agent-context --json --full" } else { "dx www agent-context --json" },
        "release_ready": release_ready,
        "relative_release_ready": release_ready,
        "release_ready_scope": readiness_gate_status.get("release_ready_scope").cloned().unwrap_or(Value::Null),
        "fastest_world_claim": false,
        "generated_at": Utc::now().to_rfc3339(),
        "workspace": workspace,
        "lane_state": lane_state,
        "allowed_checks": allowed_checks(),
        "active_blockers": active_blockers,
        "receipt_paths": receipt_paths,
        "browser_receipt_actions": browser_receipt_actions,
        "readiness_contracts": readiness_contracts,
        "next_safe_actions": next_safe_actions,
        "readiness_summary": readiness_summary,
        "readiness_gate_status": readiness_gate_status,
        "missing_proof_gates": missing_proof_gates,
        "remaining_proof_gates": remaining_proof_gates,
        "release_claim_allowed": release_claim_allowed,
        "global_speed_claim_allowed": false,
        "readiness_replay_commands": readiness_replay_commands,
        "replay_commands": readiness::readiness_replay_commands(),
        "readiness_full": readiness_full,
        "readiness": readiness,
        "source_owned_contract": true,
        "node_modules_required": false
    })
}

fn git_workspace_snapshot(cwd: &Path) -> Value {
    let status = run_git(cwd, &["status", "--short", "--branch"]).unwrap_or_default();
    let status_lines = status
        .lines()
        .map(str::trim_end)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    let branch_line = status_lines
        .first()
        .cloned()
        .unwrap_or_else(|| "## unknown".to_string());
    let status_short = status_lines.iter().skip(1).cloned().collect::<Vec<_>>();
    let branch = run_git(cwd, &["branch", "--show-current"])
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_string());
    let upstream = run_git(
        cwd,
        &["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"],
    )
    .map(|value| value.trim().to_string())
    .filter(|value| !value.is_empty())
    .unwrap_or_default();
    let diff_shortstat = run_git(cwd, &["diff", "--shortstat"])
        .map(|value| value.trim().to_string())
        .unwrap_or_default();

    json!({
        "path": cwd.to_string_lossy().replace('\\', "/"),
        "branch": branch,
        "upstream": upstream,
        "branch_line": branch_line,
        "status_short": status_short,
        "dirty_count": status_short.len(),
        "clean": status_short.is_empty(),
        "diff_shortstat": diff_shortstat
    })
}

fn lane_state_snapshot(cwd: &Path) -> Value {
    let state_root = cwd.join("worker-lanes/state");
    let counter_path = state_root.join("www-30-agent.counter.txt");
    let claims_path = state_root.join("www-30-agent.claims.jsonl");
    let current_lane_counter = std::fs::read_to_string(&counter_path)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok());
    let latest_claims = std::fs::read_to_string(&claims_path)
        .ok()
        .map(|contents| {
            contents
                .lines()
                .rev()
                .take(5)
                .filter_map(|line| serde_json::from_str::<Value>(line).ok())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    json!({
        "scope": "www-30-agent",
        "counter_path": "worker-lanes/state/www-30-agent.counter.txt",
        "claims_path": "worker-lanes/state/www-30-agent.claims.jsonl",
        "current_lane_counter": current_lane_counter,
        "max_lanes": 30,
        "round_full": current_lane_counter.is_some_and(|count| count >= 30),
        "latest_claims": latest_claims
    })
}

fn receipt_path_statuses(cwd: &Path) -> Vec<Value> {
    [
        ".dx/receipts/build/readiness.json",
        ".dx/receipts/build/installed-binary-smoke-latest.json",
        ".dx/receipts/check/web-perf/report.json",
        ".dx/receipts/check/web-perf/dev/report.json",
        ".dx/receipts/check/web-perf/static-build/report.json",
        ".dx/receipts/deploy/vercel.json",
        DEVTOOLS_VISUAL_EDIT_RECEIPT,
        NATIVE_EVENT_CATALOG_RECEIPT,
        NO_JS_ARTIFACT_RECEIPT,
        NO_JS_BROWSER_RECEIPT,
        SAME_MACHINE_PERFORMANCE_RECEIPT,
        SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT,
        PRODUCTION_HTTP_RECEIPT,
        PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT,
        BUNDLE_PARTITION_RECEIPT,
        BUNDLE_PROVIDER_REPLAY_RECEIPT,
        SERVER_ACTION_REPLAY_LEDGER_RECEIPT,
        PRIMITIVE_PROOF_RECEIPT,
        NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
        STATE_RUNTIME_BROWSER_RECEIPT,
        REACTIVITY_MODEL_RECEIPT,
        DOCS_ONBOARDING_RECEIPT,
        DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT,
        ISLAND_ABI_RECEIPT,
        ISLAND_BROWSER_RECEIPT,
        ".dx/receipts/style/build.json",
        ".dx/receipts/style/check.json",
        ".dx/receipts/icons/sync.json",
        ".dx/receipts/forge/packages-check.json",
        TEMPLATE_CHECK_RECEIPT,
        "examples/template/.dx/receipts/style/build.json",
        "examples/template/.dx/receipts/style/check.json",
        "examples/template/public/preview-manifest.json",
    ]
    .into_iter()
    .map(|relative| {
        let path = cwd.join(relative);
        let parsed = read_json(&path);
        let devtools_visual_edit_receipt_status = (relative == DEVTOOLS_VISUAL_EDIT_RECEIPT)
            .then(|| devtools_visual_edit_receipt_status(parsed.as_ref()));
        let native_event_catalog_receipt_status = (relative == NATIVE_EVENT_CATALOG_RECEIPT)
            .then(|| native_event_catalog_receipt_status(parsed.as_ref()));
        let no_js_artifact_receipt_status =
            (relative == NO_JS_ARTIFACT_RECEIPT).then(|| no_js_artifact_receipt_status(parsed.as_ref()));
        let no_js_browser_receipt_status =
            (relative == NO_JS_BROWSER_RECEIPT).then(|| no_js_browser_receipt_status(cwd, parsed.as_ref()));
        let same_machine_performance_receipt_status = (relative == SAME_MACHINE_PERFORMANCE_RECEIPT)
            .then(|| same_machine_performance_receipt_status(parsed.as_ref()));
        let production_http_receipt_status = (relative == PRODUCTION_HTTP_RECEIPT)
            .then(|| production_http_receipt_status(parsed.as_ref()));
        let production_http_tcp_preview_receipt_status =
            (relative == PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT)
                .then(|| production_http_tcp_preview_receipt_status(parsed.as_ref()));
        let bundle_partition_receipt_status = (relative == BUNDLE_PARTITION_RECEIPT)
            .then(|| bundle_partition_receipt_status(parsed.as_ref()));
        let bundle_provider_replay_receipt_status = (relative == BUNDLE_PROVIDER_REPLAY_RECEIPT)
            .then(|| bundle_provider_replay_receipt_status(parsed.as_ref()));
        let server_action_replay_ledger_receipt_status =
            (relative == SERVER_ACTION_REPLAY_LEDGER_RECEIPT)
                .then(|| server_action_replay_ledger_receipt_status(parsed.as_ref()));
        let primitive_proof_receipt_status = (relative == PRIMITIVE_PROOF_RECEIPT)
            .then(|| primitive_proof_receipt_status(parsed.as_ref()));
        let native_event_browser_binder_receipt_status =
            (relative == NATIVE_EVENT_BROWSER_BINDER_RECEIPT)
                .then(|| native_event_browser_binder_receipt_status(parsed.as_ref()));
        let state_runtime_browser_receipt_status = (relative == STATE_RUNTIME_BROWSER_RECEIPT)
            .then(|| state_runtime_browser_receipt_status(parsed.as_ref()));
        let reactivity_model_receipt_status =
            (relative == REACTIVITY_MODEL_RECEIPT).then(|| reactivity_model_receipt_status(parsed.as_ref()));
        let docs_onboarding_receipt_status =
            (relative == DOCS_ONBOARDING_RECEIPT).then(|| docs_onboarding_receipt_status(parsed.as_ref()));
        let docs_doctor_command_replay_receipt_status =
            (relative == DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT)
                .then(|| docs_doctor_command_replay_receipt_status(parsed.as_ref()));
        let island_abi_receipt_status =
            (relative == ISLAND_ABI_RECEIPT).then(|| island_abi_receipt_status(parsed.as_ref()));
        let island_browser_receipt_status = (relative == ISLAND_BROWSER_RECEIPT)
            .then(|| island_browser_receipt_status(parsed.as_ref()));
        let readiness_receipt_gate_status = parsed
            .as_ref()
            .map(readiness_receipt_gate_status)
            .unwrap_or_else(readiness_receipt_gate_status_missing);
        json!({
            "path": relative,
            "present": path.is_file(),
            "bytes": std::fs::metadata(&path).ok().map(|metadata| metadata.len()),
            "schema": parsed
                .as_ref()
                .and_then(|value| value.get("schema"))
                .and_then(Value::as_str),
            "passed": parsed
                .as_ref()
                .and_then(|value| value.get("passed"))
                .and_then(Value::as_bool),
            "score": parsed.as_ref().and_then(check_score_value),
            "max_score": parsed.as_ref().and_then(check_score_max),
            "release_ready": parsed
                .as_ref()
                .and_then(|value| value.get("release_ready"))
                .and_then(Value::as_bool),
            "fastest_world_claim": parsed
                .as_ref()
                .and_then(|value| value.get("fastest_world_claim"))
                .and_then(Value::as_bool),
            "devtools_visual_edit_receipt_status": devtools_visual_edit_receipt_status,
            "native_event_catalog_receipt_status": native_event_catalog_receipt_status,
            "no_js_artifact_receipt_status": no_js_artifact_receipt_status,
            "no_js_browser_receipt_status": no_js_browser_receipt_status,
            "same_machine_performance_receipt_status": same_machine_performance_receipt_status,
            "production_http_receipt_status": production_http_receipt_status,
            "production_http_tcp_preview_receipt_status": production_http_tcp_preview_receipt_status,
            "bundle_partition_receipt_status": bundle_partition_receipt_status,
            "bundle_provider_replay_receipt_status": bundle_provider_replay_receipt_status,
            "server_action_replay_ledger_receipt_status": server_action_replay_ledger_receipt_status,
            "primitive_proof_receipt_status": primitive_proof_receipt_status,
            "native_event_browser_binder_receipt_status": native_event_browser_binder_receipt_status,
            "state_runtime_browser_receipt_status": state_runtime_browser_receipt_status,
            "reactivity_model_receipt_status": reactivity_model_receipt_status,
            "docs_onboarding_receipt_status": docs_onboarding_receipt_status,
            "docs_doctor_command_replay_receipt_status": docs_doctor_command_replay_receipt_status,
            "island_abi_receipt_status": island_abi_receipt_status,
            "island_browser_receipt_status": island_browser_receipt_status,
            "readiness_receipt_gate_status": readiness_receipt_gate_status,
        })
    })
    .collect()
}

fn browser_receipt_actions(receipt_paths: &[Value]) -> Vec<Value> {
    [
        (
            "visual-edit-workbench-receipts",
            DEVTOOLS_VISUAL_EDIT_RECEIPT,
            "devtools_visual_edit_receipt_status",
            "browser_receipt_current",
            DEVTOOLS_VISUAL_EDIT_RECEIPT_SR,
            DEVTOOLS_VISUAL_EDIT_RECEIPT_MACHINE,
            VISUAL_EDIT_BROWSER_IMPORT_COMMAND,
            "visual-edit-browser-workbench-receipt-missing",
        ),
        (
            "native-event-browser-binder",
            NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
            "native_event_browser_binder_receipt_status",
            "current",
            NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR,
            NATIVE_EVENT_BROWSER_BINDER_RECEIPT_MACHINE,
            NATIVE_EVENT_BROWSER_BINDER_IMPORT_COMMAND,
            "native-event-browser-binder-receipt-missing",
        ),
        (
            "state-runtime-browser",
            STATE_RUNTIME_BROWSER_RECEIPT,
            "state_runtime_browser_receipt_status",
            "current",
            STATE_RUNTIME_BROWSER_RECEIPT_SR,
            STATE_RUNTIME_BROWSER_RECEIPT_MACHINE,
            STATE_RUNTIME_BROWSER_IMPORT_COMMAND,
            "state-runtime-browser-receipt-missing",
        ),
        (
            "tiny-static-no-js-browser",
            NO_JS_BROWSER_RECEIPT,
            "no_js_browser_receipt_status",
            "current",
            NO_JS_BROWSER_RECEIPT_SR,
            NO_JS_BROWSER_RECEIPT_MACHINE,
            NO_JS_BROWSER_IMPORT_COMMAND,
            "no-js-browser-receipt-missing",
        ),
        (
            "island-browser",
            ISLAND_BROWSER_RECEIPT,
            "island_browser_receipt_status",
            "current",
            ISLAND_BROWSER_RECEIPT_SR,
            ISLAND_BROWSER_RECEIPT_MACHINE,
            ISLAND_BROWSER_IMPORT_COMMAND,
            "island-browser-receipt-missing",
        ),
    ]
    .into_iter()
    .map(
        |(
            gate_id,
            receipt_path,
            status_key,
            current_key,
            source_contract,
            machine_contract,
            import_command,
            stale_reason,
        )| {
            let status = receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == receipt_path)
                .and_then(|receipt| receipt.get(status_key))
                .cloned()
                .unwrap_or(Value::Null);
            let current = status
                .get(current_key)
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let stale_reasons = status
                .get("browser_receipt_stale_reasons")
                .or_else(|| status.get("stale_reasons"))
                .cloned()
                .unwrap_or_else(|| json!([stale_reason]));
            let mut action = browser_receipt_action_metadata(
                gate_id,
                receipt_path,
                source_contract,
                machine_contract,
                import_command,
                stale_reason,
            );
            if let Some(object) = action.as_object_mut() {
                object.insert("current".to_string(), json!(current));
                object.insert("status".to_string(), status);
                object.insert("stale_reasons".to_string(), stale_reasons);
            }
            action
        },
    )
    .collect()
}

fn readiness_contract_statuses(cwd: &Path, receipt_paths: &[Value]) -> Value {
    json!({
        "schema": "dx.www.readiness.agent_context_contracts",
        "proof_graph": readiness_sr_contract_status(
            cwd,
            "proof-graph",
            READINESS_PROOF_GRAPH_RECEIPT_SCHEMA,
            READINESS_PROOF_GRAPH_RECEIPT,
            READINESS_PROOF_GRAPH_RECEIPT_MACHINE,
        ),
        "visual_edit": readiness_contract_status(
            cwd,
            "visual-edit-workbench-receipts",
            DEVTOOLS_VISUAL_EDIT_RECEIPT_SCHEMA,
            DEVTOOLS_VISUAL_EDIT_RECEIPT,
            DEVTOOLS_VISUAL_EDIT_RECEIPT_SR,
            DEVTOOLS_VISUAL_EDIT_RECEIPT_MACHINE,
            receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == DEVTOOLS_VISUAL_EDIT_RECEIPT)
                .and_then(|receipt| {
                    receipt
                        .get("devtools_visual_edit_receipt_status")
                        .and_then(|status| status.get("current"))
                        .and_then(Value::as_bool)
                })
                .unwrap_or(false),
        ),
        "native_events": readiness_contract_status(
            cwd,
            "native-events",
            NATIVE_EVENT_CATALOG_RECEIPT_SCHEMA,
            NATIVE_EVENT_CATALOG_RECEIPT,
            NATIVE_EVENT_CATALOG_RECEIPT_SR,
            NATIVE_EVENT_CATALOG_RECEIPT_MACHINE,
            receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == NATIVE_EVENT_CATALOG_RECEIPT)
                .and_then(|receipt| {
                    receipt
                        .get("native_event_catalog_receipt_status")
                        .and_then(|status| status.get("current"))
                        .and_then(Value::as_bool)
                })
                .unwrap_or(false),
        ),
        "no_js_artifact": readiness_contract_status(
            cwd,
            "tiny-static-no-js-artifact",
            NO_JS_ARTIFACT_RECEIPT_SCHEMA,
            NO_JS_ARTIFACT_RECEIPT,
            NO_JS_ARTIFACT_RECEIPT_SR,
            NO_JS_ARTIFACT_RECEIPT_MACHINE,
            receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == NO_JS_ARTIFACT_RECEIPT)
                .and_then(|receipt| {
                    receipt
                        .get("no_js_artifact_receipt_status")
                        .and_then(|status| status.get("current"))
                        .and_then(Value::as_bool)
                })
                .unwrap_or(false),
        ),
        "no_js_browser": readiness_contract_status(
            cwd,
            "tiny-static-no-js-browser",
            NO_JS_BROWSER_RECEIPT_SCHEMA,
            NO_JS_BROWSER_RECEIPT,
            NO_JS_BROWSER_RECEIPT_SR,
            NO_JS_BROWSER_RECEIPT_MACHINE,
            receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == NO_JS_BROWSER_RECEIPT)
                .and_then(|receipt| {
                    receipt
                        .get("no_js_browser_receipt_status")
                        .and_then(|status| status.get("current"))
                        .and_then(Value::as_bool)
                })
                .unwrap_or(false),
        ),
        "same_machine_performance": readiness_contract_status(
            cwd,
            "same-machine-performance",
            SAME_MACHINE_PERFORMANCE_RECEIPT_SCHEMA,
            SAME_MACHINE_PERFORMANCE_RECEIPT,
            SAME_MACHINE_PERFORMANCE_RECEIPT_SR,
            SAME_MACHINE_PERFORMANCE_RECEIPT_MACHINE,
            receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == SAME_MACHINE_PERFORMANCE_RECEIPT)
                .and_then(|receipt| {
                    receipt
                        .get("same_machine_performance_receipt_status")
                        .and_then(|status| status.get("current"))
                        .and_then(Value::as_bool)
                })
                .unwrap_or(false),
        ),
        "native_event_browser_binder": readiness_contract_status(
            cwd,
            "native-event-browser-binder",
            NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SCHEMA,
            NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
            NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR,
            NATIVE_EVENT_BROWSER_BINDER_RECEIPT_MACHINE,
            receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == NATIVE_EVENT_BROWSER_BINDER_RECEIPT)
                .and_then(|receipt| {
                    receipt
                        .get("native_event_browser_binder_receipt_status")
                        .and_then(|status| status.get("current"))
                        .and_then(Value::as_bool)
                })
                .unwrap_or(false),
        ),
        "state_runtime_browser": readiness_contract_status(
            cwd,
            "state-runtime-browser",
            STATE_RUNTIME_BROWSER_RECEIPT_SCHEMA,
            STATE_RUNTIME_BROWSER_RECEIPT,
            STATE_RUNTIME_BROWSER_RECEIPT_SR,
            STATE_RUNTIME_BROWSER_RECEIPT_MACHINE,
            receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == STATE_RUNTIME_BROWSER_RECEIPT)
                .and_then(|receipt| {
                    receipt
                        .get("state_runtime_browser_receipt_status")
                        .and_then(|status| status.get("current"))
                        .and_then(Value::as_bool)
                })
                .unwrap_or(false),
        ),
        "reactivity_model": readiness_contract_status(
            cwd,
            "reactivity-model",
            REACTIVITY_MODEL_RECEIPT_SCHEMA,
            REACTIVITY_MODEL_RECEIPT,
            REACTIVITY_MODEL_RECEIPT_SR,
            REACTIVITY_MODEL_RECEIPT_MACHINE,
            receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == REACTIVITY_MODEL_RECEIPT)
                .and_then(|receipt| {
                    receipt
                        .get("reactivity_model_receipt_status")
                        .and_then(|status| status.get("current"))
                        .and_then(Value::as_bool)
                })
                .unwrap_or(false),
        ),
        "docs_onboarding": readiness_contract_status(
            cwd,
            "docs-onboarding-doctor",
            DOCS_ONBOARDING_RECEIPT_SCHEMA,
            DOCS_ONBOARDING_RECEIPT,
            DOCS_ONBOARDING_RECEIPT_SR,
            DOCS_ONBOARDING_RECEIPT_MACHINE,
            receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == DOCS_ONBOARDING_RECEIPT)
                .and_then(|receipt| {
                    receipt
                        .get("docs_onboarding_receipt_status")
                        .and_then(|status| status.get("current"))
                        .and_then(Value::as_bool)
                })
                .unwrap_or(false),
        ),
        "docs_doctor_command_replay": readiness_contract_status(
            cwd,
            "docs-doctor-command-replay",
            DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_SCHEMA,
            DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT,
            DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_SR,
            DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_MACHINE,
            receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT)
                .and_then(|receipt| {
                    receipt
                        .get("docs_doctor_command_replay_receipt_status")
                        .and_then(|status| status.get("current"))
                        .and_then(Value::as_bool)
                })
                .unwrap_or(false),
        ),
        "island_abi": readiness_contract_status(
            cwd,
            "islands",
            ISLAND_ABI_RECEIPT_SCHEMA,
            ISLAND_ABI_RECEIPT,
            ISLAND_ABI_RECEIPT_SR,
            ISLAND_ABI_RECEIPT_MACHINE,
            receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == ISLAND_ABI_RECEIPT)
                .and_then(|receipt| {
                    receipt
                        .get("island_abi_receipt_status")
                        .and_then(|status| status.get("current"))
                        .and_then(Value::as_bool)
                })
                .unwrap_or(false),
        ),
        "island_browser": readiness_contract_status(
            cwd,
            "island-browser",
            ISLAND_BROWSER_RECEIPT_SCHEMA,
            ISLAND_BROWSER_RECEIPT,
            ISLAND_BROWSER_RECEIPT_SR,
            ISLAND_BROWSER_RECEIPT_MACHINE,
            receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == ISLAND_BROWSER_RECEIPT)
                .and_then(|receipt| {
                    receipt
                        .get("island_browser_receipt_status")
                        .and_then(|status| status.get("current"))
                        .and_then(Value::as_bool)
                })
                .unwrap_or(false),
        ),
        "server_action_replay_ledger": readiness_contract_status(
            cwd,
            "route-action-runtime",
            SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SCHEMA,
            SERVER_ACTION_REPLAY_LEDGER_RECEIPT,
            SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SR,
            SERVER_ACTION_REPLAY_LEDGER_RECEIPT_MACHINE,
            receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == SERVER_ACTION_REPLAY_LEDGER_RECEIPT)
                .and_then(|receipt| {
                    receipt
                        .get("server_action_replay_ledger_receipt_status")
                        .and_then(|status| status.get("current"))
                        .and_then(Value::as_bool)
                })
                .unwrap_or(false),
        ),
        "primitive_proof": readiness_contract_status(
            cwd,
            "primitive-proof",
            PRIMITIVE_PROOF_RECEIPT_SCHEMA,
            PRIMITIVE_PROOF_RECEIPT,
            PRIMITIVE_PROOF_RECEIPT_SR,
            PRIMITIVE_PROOF_RECEIPT_MACHINE,
            receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == PRIMITIVE_PROOF_RECEIPT)
                .and_then(|receipt| {
                    receipt
                        .get("primitive_proof_receipt_status")
                        .and_then(|status| status.get("current"))
                        .and_then(Value::as_bool)
                })
                .unwrap_or(false),
        ),
        "bundle_partition": readiness_contract_status(
            cwd,
            "public-vs-evidence-bundle",
            BUNDLE_PARTITION_RECEIPT_SCHEMA,
            BUNDLE_PARTITION_RECEIPT,
            BUNDLE_PARTITION_RECEIPT_SR,
            BUNDLE_PARTITION_RECEIPT_MACHINE,
            receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == BUNDLE_PARTITION_RECEIPT)
                .and_then(|receipt| {
                    receipt
                        .get("bundle_partition_receipt_status")
                        .and_then(|status| status.get("current"))
                        .and_then(Value::as_bool)
                })
                .unwrap_or(false),
        ),
        "bundle_provider_replay": readiness_contract_status(
            cwd,
            "public-vs-evidence-bundle-provider-replay",
            BUNDLE_PROVIDER_REPLAY_RECEIPT_SCHEMA,
            BUNDLE_PROVIDER_REPLAY_RECEIPT,
            BUNDLE_PROVIDER_REPLAY_RECEIPT_SR,
            BUNDLE_PROVIDER_REPLAY_RECEIPT_MACHINE,
            receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == BUNDLE_PROVIDER_REPLAY_RECEIPT)
                .and_then(|receipt| {
                    receipt
                        .get("bundle_provider_replay_receipt_status")
                        .and_then(|status| status.get("current"))
                        .and_then(Value::as_bool)
                })
                .unwrap_or(false),
        ),
        "rule": ".sr sources are opaque serializer contracts, generated .machine files are binary machine caches, and legacy JSON receipts remain compatibility read-models; agent-context must not parse .sr or .machine as JSON."
    })
}

fn readiness_contract_status(
    cwd: &Path,
    id: &str,
    schema: &str,
    json_read_model_path: &str,
    serializer_receipt_path: &str,
    machine_contract_path: &str,
    json_read_model_current: bool,
) -> Value {
    let source = file_contract_status(cwd, serializer_receipt_path, "opaque-sr");
    let machine = file_contract_status(cwd, machine_contract_path, "generated-machine-cache");
    let source_present = source["present"].as_bool() == Some(true);
    let machine_present = machine["present"].as_bool() == Some(true);
    let source_fingerprinted = source
        .get("content_blake3")
        .and_then(Value::as_str)
        .is_some();
    let machine_fingerprinted = machine
        .get("content_blake3")
        .and_then(Value::as_str)
        .is_some();
    let machine_fresh_against_source =
        machine_cache_fresh_against_source(cwd, serializer_receipt_path, machine_contract_path);
    let current = source_present
        && machine_present
        && source_fingerprinted
        && machine_fingerprinted
        && machine_fresh_against_source == Some(true)
        && json_read_model_current;

    json!({
        "id": id,
        "contract": schema,
        "current": current,
        "blocking_status": if current { "current" } else { "serializer-machine-proof-missing-or-stale" },
        "json_read_model_path": json_read_model_path,
        "json_read_model_current": json_read_model_current,
        "source_contract": source,
        "machine_contract": machine,
        "machine_fresh_against_source": machine_fresh_against_source,
        "required_for_claims": [
            "serializer .sr source present",
            "serializer .sr source content fingerprint available",
            "generated .machine cache present",
            "generated .machine cache content fingerprint available",
            "generated .machine cache is not older than the .sr source",
            "legacy JSON read-model current until consumers migrate"
        ],
    })
}

fn readiness_sr_contract_status(
    cwd: &Path,
    id: &str,
    schema: &str,
    serializer_receipt_path: &str,
    machine_contract_path: &str,
) -> Value {
    let source = file_contract_status(cwd, serializer_receipt_path, "opaque-sr");
    let machine = file_contract_status(cwd, machine_contract_path, "generated-machine-cache");
    let source_present = source["present"].as_bool() == Some(true);
    let machine_present = machine["present"].as_bool() == Some(true);
    let source_fingerprinted = source
        .get("content_blake3")
        .and_then(Value::as_str)
        .is_some();
    let machine_fingerprinted = machine
        .get("content_blake3")
        .and_then(Value::as_str)
        .is_some();
    let machine_fresh_against_source =
        machine_cache_fresh_against_source(cwd, serializer_receipt_path, machine_contract_path);
    let current = source_present
        && machine_present
        && source_fingerprinted
        && machine_fingerprinted
        && machine_fresh_against_source == Some(true);

    json!({
        "id": id,
        "contract": schema,
        "current": current,
        "blocking_status": if current { "current" } else { "serializer-machine-proof-missing-or-stale" },
        "source_contract": source,
        "machine_contract": machine,
        "machine_fresh_against_source": machine_fresh_against_source,
        "replay_command": "dx www readiness --write-receipts",
        "build_replay_command": "dx build",
        "proof_scopes": [
            "root release-readiness agent-context serializer proof",
            "build-output deploy-adapter proof graph"
        ],
        "inspect_command": "dx www readiness --json --full",
        "stale_reasons": if current {
            json!([])
        } else {
            json!(["proof-graph-receipt-not-regenerated"])
        },
        "required_for_claims": [
            "serializer .sr source present",
            "serializer .sr source content fingerprint available",
            "generated .machine cache present",
            "generated .machine cache content fingerprint available",
            "generated .machine cache is not older than the .sr source"
        ],
    })
}

fn file_contract_status(cwd: &Path, relative: &str, parse_mode: &str) -> Value {
    let path = cwd.join(relative);
    json!({
        "path": relative,
        "present": path.is_file(),
        "bytes": std::fs::metadata(&path).ok().map(|metadata| metadata.len()),
        "modified_unix_ms": file_modified_unix_ms(&path),
        "content_blake3": file_blake3_hex(&path),
        "parse_mode": parse_mode,
        "json_parse_attempted": false,
    })
}

fn machine_cache_fresh_against_source(
    cwd: &Path,
    source_relative: &str,
    machine_relative: &str,
) -> Option<bool> {
    let source_modified = std::fs::metadata(cwd.join(source_relative))
        .and_then(|metadata| metadata.modified())
        .ok()?;
    let machine_modified = std::fs::metadata(cwd.join(machine_relative))
        .and_then(|metadata| metadata.modified())
        .ok()?;
    Some(machine_modified >= source_modified)
}

fn file_modified_unix_ms(path: &Path) -> Option<u128> {
    std::fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()?
        .duration_since(std::time::UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_millis())
}

fn file_blake3_hex(path: &Path) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    Some(blake3::hash(&bytes).to_hex().to_string())
}

fn browser_receipt_proof_target(gate_id: &str) -> Value {
    match gate_id {
        "state-runtime-browser" => json!({
            "kind": "canonical-starter-browser-proof-target",
            "route": CANONICAL_STATE_RUNTIME_ROUTE,
            "route_url": format!("{CANONICAL_STARTER_BROWSER_ORIGIN}{CANONICAL_STATE_RUNTIME_ROUTE}"),
            "starter_source_file": CANONICAL_STATE_RUNTIME_SOURCE,
            "starter_component_source_file": CANONICAL_STATE_RUNTIME_COMPONENT_SOURCE,
            "materialized_route_file": "app/state-runtime/page.tsx",
            "canonical_receipt": STATE_RUNTIME_BROWSER_RECEIPT,
            "candidate_receipt": STATE_RUNTIME_BROWSER_CANDIDATE_RECEIPT,
            "import_command": STATE_RUNTIME_BROWSER_IMPORT_COMMAND,
            "snapshot_capture_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
            "page_snapshot_capture_command": BROWSER_RECEIPT_HARNESS_PAGE_SNAPSHOT_COMMAND,
            "dom_snapshot_capture_command": BROWSER_RECEIPT_HARNESS_DOM_SNAPSHOT_COMMAND,
            "snapshot_to_candidate_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
            "page_snapshot_import_command": BROWSER_PAGE_SNAPSHOT_IMPORT_COMMAND,
            "snapshot_capture_modes": ["full-replay-page-collector", "read-only-dom-after-browser-interactions"],
            "required_snapshot_fields": [
                "browser_runtime_executed",
                "state_runtime.present",
                "state_runtime.route",
                "state_runtime.state_reflection_event_count",
                "state_runtime.derived_reflection_event_count",
                "state_runtime.effect_scheduled_event_count",
                "state_runtime.action_dispatch_count"
            ],
            "required_selectors": [
                r#"[data-dx-route="/state-runtime"]"#,
                "[data-dx-state-read]"
            ],
            "required_events": [
                "dx:state-runtime-dispatch",
                "dx:state-dom-reflection",
                "dx:derived-state-slot",
                "dx:effect-scheduled"
            ],
            "capture_note": "Open the canonical starter /state-runtime route in a real local browser, evaluate the page collector there, then import the generated state-runtime browser candidate.",
            "snapshot_claims_proof": false,
            "import_validation_required": true,
            "release_claim_allowed": false,
            "global_speed_claim_allowed": false
        }),
        "island-browser" => json!({
            "kind": "canonical-starter-browser-proof-target",
            "route": CANONICAL_ISLANDS_ROUTE,
            "route_url": format!("{CANONICAL_STARTER_BROWSER_ORIGIN}{CANONICAL_ISLANDS_ROUTE}"),
            "starter_source_file": CANONICAL_ISLANDS_SOURCE,
            "starter_component_source_file": CANONICAL_ISLANDS_COMPONENT_SOURCE,
            "materialized_route_file": "app/islands/page.tsx",
            "canonical_receipt": ISLAND_BROWSER_RECEIPT,
            "candidate_receipt": ISLAND_BROWSER_CANDIDATE_RECEIPT,
            "import_command": ISLAND_BROWSER_IMPORT_COMMAND,
            "snapshot_capture_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
            "page_snapshot_capture_command": BROWSER_RECEIPT_HARNESS_PAGE_SNAPSHOT_COMMAND,
            "dom_snapshot_capture_command": BROWSER_RECEIPT_HARNESS_DOM_SNAPSHOT_COMMAND,
            "snapshot_to_candidate_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
            "page_snapshot_import_command": BROWSER_PAGE_SNAPSHOT_IMPORT_COMMAND,
            "snapshot_capture_modes": ["full-replay-page-collector", "read-only-dom-after-browser-interactions"],
            "required_snapshot_fields": [
                "browser_runtime_executed",
                "islands.bridge_present",
                "islands.source_owned_bridge",
                "islands.abi_schema",
                "islands.directive_style",
                "islands.no_js_fallback_preserved",
                "islands.island_count",
                "islands.source_owned_island_count",
                "islands.client_island_event_count",
                "islands.event_replay_results"
            ],
            "required_selectors": [
                r#"[data-dx-route="/islands"]"#,
                "[data-dx-client-island-bridge]",
                "[data-dx-island]",
                r#"[data-dx-component="island-runtime-probe"]"#,
                "[data-dx-event-id][data-dx-event]"
            ],
            "required_events": ["dx:client-island-event"],
            "capture_note": "Open the canonical starter /islands route in a real local browser, evaluate the page collector there, then import the generated island browser candidate.",
            "snapshot_claims_proof": false,
            "import_validation_required": true,
            "release_claim_allowed": false,
            "global_speed_claim_allowed": false
        }),
        "visual-edit-workbench-receipts" => json!({
            "kind": "canonical-starter-browser-proof-target",
            "route": CANONICAL_STATE_RUNTIME_ROUTE,
            "route_url": format!("{CANONICAL_STARTER_BROWSER_ORIGIN}{CANONICAL_STATE_RUNTIME_ROUTE}"),
            "starter_source_file": CANONICAL_STATE_RUNTIME_SOURCE,
            "starter_component_source_file": CANONICAL_STATE_RUNTIME_COMPONENT_SOURCE,
            "canonical_receipt": DEVTOOLS_VISUAL_EDIT_RECEIPT,
            "candidate_receipt": VISUAL_EDIT_BROWSER_CANDIDATE_RECEIPT,
            "import_command": VISUAL_EDIT_BROWSER_IMPORT_COMMAND,
            "snapshot_capture_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
            "page_snapshot_capture_command": BROWSER_RECEIPT_HARNESS_PAGE_SNAPSHOT_COMMAND,
            "dom_snapshot_capture_command": BROWSER_RECEIPT_HARNESS_DOM_SNAPSHOT_COMMAND,
            "snapshot_to_candidate_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
            "page_snapshot_import_command": BROWSER_PAGE_SNAPSHOT_IMPORT_COMMAND,
            "snapshot_capture_modes": ["full-replay-page-collector", "read-only-dom-after-browser-interactions"],
            "required_snapshot_fields": [
                "browser_runtime_executed",
                "visual_edit.devtools_global_present",
                "visual_edit.browser_workbench_replay",
                "visual_edit_replay_attempt.status"
            ],
            "required_selectors": [r#"[data-dx-component="state-runtime-probe"]"#],
            "capture_note": "Use a real local browser page snapshot from the canonical /state-runtime starter route for the visual edit workbench candidate.",
            "snapshot_claims_proof": false,
            "import_validation_required": true,
            "release_claim_allowed": false,
            "global_speed_claim_allowed": false
        }),
        "tiny-static-no-js-browser" => json!({
            "kind": "canonical-starter-no-js-browser-proof-target",
            "route": CANONICAL_HOME_ROUTE,
            "output_html": CANONICAL_NO_JS_OUTPUT_HTML,
            "canonical_receipt": NO_JS_BROWSER_RECEIPT,
            "candidate_receipt": NO_JS_BROWSER_CANDIDATE_RECEIPT,
            "collector_command": NO_JS_BROWSER_COLLECT_COMMAND,
            "import_command": NO_JS_BROWSER_IMPORT_COMMAND,
            "snapshot_capture_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
            "page_snapshot_capture_command": BROWSER_RECEIPT_HARNESS_PAGE_SNAPSHOT_COMMAND,
            "dom_snapshot_capture_command": BROWSER_RECEIPT_HARNESS_DOM_SNAPSHOT_COMMAND,
            "snapshot_to_candidate_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
            "page_snapshot_import_command": BROWSER_PAGE_SNAPSHOT_IMPORT_COMMAND,
            "snapshot_capture_modes": ["full-replay-page-collector", "read-only-dom-after-browser-interactions"],
            "required_snapshot_fields": [
                "no_js.javascript_disabled_browser",
                "no_js.meaningful_html_without_js",
                "no_js.link_count",
                "no_js.form_count"
            ],
            "capture_note": "Capture the canonical no-JS starter output as a JS-disabled browser receipt; source HTML alone does not claim live browser proof.",
            "snapshot_claims_proof": false,
            "import_validation_required": true,
            "release_claim_allowed": false,
            "global_speed_claim_allowed": false
        }),
        _ => Value::Null,
    }
}

fn browser_receipt_action_metadata(
    gate_id: &str,
    receipt: &str,
    source_contract: &str,
    machine_contract: &str,
    import_command: &str,
    stale_reason: &str,
) -> Value {
    json!({
        "gate_id": gate_id,
        "blocks_release": true,
        "release_claim_allowed": false,
        "global_speed_claim_allowed": false,
        "receipt": receipt,
        "source_contract": source_contract,
        "machine_contract": machine_contract,
        "replay_command": READINESS_INSPECT_COMMAND,
        "write_receipts_command": READINESS_WRITE_RECEIPTS_COMMAND,
        "import_command": import_command,
        "canonical_browser_proof_target": browser_receipt_proof_target(gate_id),
        "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
        "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
        "harness_page_snapshot_command": BROWSER_RECEIPT_HARNESS_PAGE_SNAPSHOT_COMMAND,
        "harness_dom_snapshot_command": BROWSER_RECEIPT_HARNESS_DOM_SNAPSHOT_COMMAND,
        "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
        "snapshot_capture_modes": ["full-replay-page-collector", "read-only-dom-after-browser-interactions"],
        "snapshot_import_command": BROWSER_PAGE_SNAPSHOT_IMPORT_COMMAND,
        "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
        "missing_stale_reason": stale_reason,
        "score_honesty": "browser receipts can clear local proof handoff only; release_ready and global_speed_claim_allowed stay false until hosted/provider breadth gates are proven"
    })
}

fn active_blockers(cwd: &Path, receipt_paths: &[Value], readiness_contracts: &Value) -> Vec<Value> {
    let mut blockers = Vec::new();
    if readiness_contracts
        .pointer("/proof_graph/current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        blockers.push(json!({
            "id": "readiness-proof-graph-machine-contract-missing",
            "severity": "high",
            "message": "Proof graph release-readiness serializer proof is missing or stale; do not claim proof graph receipt freshness until dx www readiness --write-receipts regenerates the root .sr source and generated .machine contract. dx build still regenerates the build-output deploy-adapter proof graph.",
            "source_contract": READINESS_PROOF_GRAPH_RECEIPT,
            "machine_contract": READINESS_PROOF_GRAPH_RECEIPT_MACHINE,
            "replay_command": "dx www readiness --write-receipts",
            "build_replay_command": "dx build",
            "inspect_command": "dx www readiness --json --full",
            "stale_reasons": ["proof-graph-receipt-not-regenerated"],
            "contract_status": readiness_contracts["proof_graph"]
        }));
    }

    if receipt_paths.iter().any(|receipt| {
        receipt["path"] == ".dx/receipts/check/web-perf/dev/report.json"
            && receipt["present"] == true
    }) && !receipt_paths.iter().any(|receipt| {
        receipt["path"] == ".dx/receipts/check/web-perf/static-build/report.json"
            && receipt["present"] == true
    }) {
        blockers.push(json!({
            "id": "web-perf-static-receipt-missing",
            "severity": "medium",
            "message": "Dev web-perf and static/build web-perf receipts are split; static/build proof has not been collected yet."
        }));
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == DEVTOOLS_VISUAL_EDIT_RECEIPT)
    {
        if receipt
            .pointer("/devtools_visual_edit_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "devtools-visual-edit-receipt-missing",
                "severity": "high",
                "message": "Devtools visual-edit release-readiness proof is missing or stale; do not claim visual edit workbench proof until the latest receipt is current.",
                "receipt": DEVTOOLS_VISUAL_EDIT_RECEIPT,
                "required_schema": DEVTOOLS_VISUAL_EDIT_RECEIPT_SCHEMA,
                "browser_receipt_gate": browser_receipt_action_metadata(
                    "visual-edit-workbench-receipts",
                    DEVTOOLS_VISUAL_EDIT_RECEIPT,
                    DEVTOOLS_VISUAL_EDIT_RECEIPT_SR,
                    DEVTOOLS_VISUAL_EDIT_RECEIPT_MACHINE,
                    VISUAL_EDIT_BROWSER_IMPORT_COMMAND,
                    "visual-edit-browser-workbench-receipt-missing",
                ),
                "replay_command": READINESS_INSPECT_COMMAND,
                "write_replay_command": VISUAL_EDIT_FOUNDATION_REPLAY_COMMAND,
                "import_command": VISUAL_EDIT_BROWSER_IMPORT_COMMAND,
                "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
                "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
                "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
                "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
                "stale_reasons": receipt
                    .pointer("/devtools_visual_edit_receipt_status/stale_reasons")
                    .cloned()
                    .unwrap_or_else(|| json!(["visual-edit-browser-workbench-receipt-missing"])),
                "receipt_status": receipt["devtools_visual_edit_receipt_status"]
            }));
        }
        if receipt
            .pointer("/devtools_visual_edit_receipt_status/browser_receipt_current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "visual-edit-browser-workbench-receipt-missing",
                "severity": "high",
                "message": "Visual-edit browser workbench proof is missing or stale; safe preview/apply/undo receipts do not prove inspect/cascade/preview/apply/undo in a real browser workbench.",
                "receipt": DEVTOOLS_VISUAL_EDIT_RECEIPT,
                "required_schema": DEVTOOLS_VISUAL_EDIT_RECEIPT_SCHEMA,
                "browser_receipt_gate": browser_receipt_action_metadata(
                    "visual-edit-workbench-receipts",
                    DEVTOOLS_VISUAL_EDIT_RECEIPT,
                    DEVTOOLS_VISUAL_EDIT_RECEIPT_SR,
                    DEVTOOLS_VISUAL_EDIT_RECEIPT_MACHINE,
                    VISUAL_EDIT_BROWSER_IMPORT_COMMAND,
                    "visual-edit-browser-workbench-receipt-missing",
                ),
                "replay_command": READINESS_INSPECT_COMMAND,
                "write_replay_command": VISUAL_EDIT_FOUNDATION_REPLAY_COMMAND,
                "import_command": VISUAL_EDIT_BROWSER_IMPORT_COMMAND,
                "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
                "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
                "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
                "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
                "stale_reasons": receipt
                    .pointer("/devtools_visual_edit_receipt_status/browser_receipt_stale_reasons")
                    .cloned()
                    .unwrap_or_else(|| json!(["visual-edit-browser-workbench-receipt-missing"])),
                "receipt_status": receipt["devtools_visual_edit_receipt_status"]
            }));
        }
    }

    if readiness_contracts
        .pointer("/visual_edit/current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        blockers.push(json!({
            "id": "readiness-visual-edit-machine-contract-missing",
            "severity": "high",
            "message": "Devtools visual-edit release-readiness proof is not serializer-backed yet; do not claim durable visual edit proof until the .sr source and generated .machine contract exist and the JSON read-model receipt is current.",
            "source_contract": DEVTOOLS_VISUAL_EDIT_RECEIPT_SR,
            "machine_contract": DEVTOOLS_VISUAL_EDIT_RECEIPT_MACHINE,
            "legacy_json_read_model": DEVTOOLS_VISUAL_EDIT_RECEIPT,
            "browser_receipt_gate": browser_receipt_action_metadata(
                "visual-edit-workbench-receipts",
                DEVTOOLS_VISUAL_EDIT_RECEIPT,
                DEVTOOLS_VISUAL_EDIT_RECEIPT_SR,
                DEVTOOLS_VISUAL_EDIT_RECEIPT_MACHINE,
                VISUAL_EDIT_BROWSER_IMPORT_COMMAND,
                "visual-edit-browser-workbench-receipt-missing",
            ),
            "replay_command": READINESS_INSPECT_COMMAND,
            "write_replay_command": VISUAL_EDIT_FOUNDATION_REPLAY_COMMAND,
            "import_command": VISUAL_EDIT_BROWSER_IMPORT_COMMAND,
            "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
            "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
            "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
            "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
            "stale_reasons": ["visual-edit-browser-workbench-receipt-missing"],
            "contract_status": readiness_contracts["visual_edit"]
        }));
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == NATIVE_EVENT_CATALOG_RECEIPT)
    {
        if receipt
            .pointer("/native_event_catalog_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "native-event-catalog-receipt-missing",
                "severity": "high",
                "message": "Native DOM event release-readiness receipt is missing or stale; local MDN/catalog freshness requires a current native-events receipt, and full browser binder proof remains a separate missing gate.",
                "receipt": NATIVE_EVENT_CATALOG_RECEIPT,
                "required_schema": NATIVE_EVENT_CATALOG_RECEIPT_SCHEMA,
                "receipt_status": receipt["native_event_catalog_receipt_status"]
            }));
        }
    }

    if readiness_contracts
        .pointer("/native_events/current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        blockers.push(json!({
            "id": "readiness-native-events-machine-contract-missing",
            "severity": "high",
            "message": "Native DOM event release-readiness serializer proof is not current; do not claim local MDN/catalog freshness from durable receipts until the .sr source, generated .machine contract, and JSON read-model are current. Full browser binder proof remains a separate missing gate.",
            "source_contract": NATIVE_EVENT_CATALOG_RECEIPT_SR,
            "machine_contract": NATIVE_EVENT_CATALOG_RECEIPT_MACHINE,
            "legacy_json_read_model": NATIVE_EVENT_CATALOG_RECEIPT,
            "contract_status": readiness_contracts["native_events"]
        }));
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == NO_JS_ARTIFACT_RECEIPT)
    {
        if receipt
            .pointer("/no_js_artifact_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "tiny-static-no-js-artifact-receipt-missing",
                "severity": "high",
                "message": "Tiny-static/no-JS release-readiness artifact proof is missing or stale; do not claim Tier 0 no-JS output until the latest no-JS artifact receipt is current.",
                "receipt": NO_JS_ARTIFACT_RECEIPT,
                "required_schema": NO_JS_ARTIFACT_RECEIPT_SCHEMA,
                "receipt_status": receipt["no_js_artifact_receipt_status"]
            }));
        }
    }

    if readiness_contracts
        .pointer("/no_js_artifact/current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        blockers.push(json!({
            "id": "readiness-no-js-artifact-machine-contract-missing",
            "severity": "high",
            "message": "Tiny-static/no-JS release-readiness proof is not serializer-backed yet; do not claim durable Tier 0 output proof until the JSON receipt, .sr source, and generated .machine contract are current.",
            "source_contract": NO_JS_ARTIFACT_RECEIPT_SR,
            "machine_contract": NO_JS_ARTIFACT_RECEIPT_MACHINE,
            "legacy_json_read_model": NO_JS_ARTIFACT_RECEIPT,
            "contract_status": readiness_contracts["no_js_artifact"]
        }));
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == NO_JS_BROWSER_RECEIPT)
    {
        if receipt
            .pointer("/no_js_browser_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "no-js-browser-receipt-missing",
                "severity": "high",
                "message": "Tiny-static/no-JS browser proof is missing or stale; artifact-only receipts are not enough to claim a real JS-disabled browser pass.",
                "receipt": NO_JS_BROWSER_RECEIPT,
                "required_schema": NO_JS_BROWSER_RECEIPT_SCHEMA,
                "browser_receipt_gate": browser_receipt_action_metadata(
                    "tiny-static-no-js-browser",
                    NO_JS_BROWSER_RECEIPT,
                    NO_JS_BROWSER_RECEIPT_SR,
                    NO_JS_BROWSER_RECEIPT_MACHINE,
                    NO_JS_BROWSER_IMPORT_COMMAND,
                    "no-js-browser-receipt-missing",
                ),
                "replay_command": READINESS_INSPECT_COMMAND,
                "collector_command": NO_JS_BROWSER_COLLECT_COMMAND,
                "import_command": NO_JS_BROWSER_IMPORT_COMMAND,
                "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
                "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
                "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
                "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
                "stale_reasons": receipt
                    .pointer("/no_js_browser_receipt_status/stale_reasons")
                    .cloned()
                    .unwrap_or_else(|| json!(["no-js-browser-receipt-missing"])),
                "receipt_status": receipt["no_js_browser_receipt_status"]
            }));
        }
    }

    if readiness_contracts
        .pointer("/no_js_browser/current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        blockers.push(json!({
            "id": "readiness-no-js-browser-machine-contract-missing",
            "severity": "high",
            "message": "Tiny-static/no-JS browser proof is not serializer-backed yet; do not claim real JS-disabled browser proof until the JSON receipt, .sr source, and generated .machine contract are current.",
            "source_contract": NO_JS_BROWSER_RECEIPT_SR,
            "machine_contract": NO_JS_BROWSER_RECEIPT_MACHINE,
            "legacy_json_read_model": NO_JS_BROWSER_RECEIPT,
            "browser_receipt_gate": browser_receipt_action_metadata(
                "tiny-static-no-js-browser",
                NO_JS_BROWSER_RECEIPT,
                NO_JS_BROWSER_RECEIPT_SR,
                NO_JS_BROWSER_RECEIPT_MACHINE,
                NO_JS_BROWSER_IMPORT_COMMAND,
                "no-js-browser-receipt-missing",
            ),
            "replay_command": READINESS_INSPECT_COMMAND,
            "collector_command": NO_JS_BROWSER_COLLECT_COMMAND,
            "import_command": NO_JS_BROWSER_IMPORT_COMMAND,
            "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
            "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
            "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
            "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
            "stale_reasons": ["no-js-browser-receipt-missing"],
            "contract_status": readiness_contracts["no_js_browser"]
        }));
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == SAME_MACHINE_PERFORMANCE_RECEIPT)
    {
        if receipt
            .pointer("/same_machine_performance_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "same-machine-performance-receipt-missing",
                "severity": "high",
                "message": "Same-machine WWW/Next/Svelte/Astro throughput proof is missing or stale; do not claim Astro tiny-static parity, release readiness, or global speed leadership from source-only no-JS receipts.",
                "receipt": SAME_MACHINE_PERFORMANCE_RECEIPT,
                "required_schema": SAME_MACHINE_PERFORMANCE_RECEIPT_SCHEMA,
                "collection_receipt": SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT,
                "serializer_receipt": SAME_MACHINE_PERFORMANCE_RECEIPT_SR,
                "machine_contract": SAME_MACHINE_PERFORMANCE_RECEIPT_MACHINE,
                "replay_command": SAME_MACHINE_PERFORMANCE_REPLAY_COMMAND,
                "raw_replay_command": SAME_MACHINE_PERFORMANCE_RAW_REPLAY_COMMAND,
                "dry_run_command": SAME_MACHINE_PERFORMANCE_DRY_RUN_COMMAND,
                "import_command": SAME_MACHINE_PERFORMANCE_IMPORT_COMMAND,
                "stale_reasons": receipt
                    .pointer("/same_machine_performance_receipt_status/stale_reasons")
                    .cloned()
                    .unwrap_or_else(|| json!(["same-machine-performance-receipt-missing"])),
                "receipt_status": receipt["same_machine_performance_receipt_status"]
            }));
        }
    }

    if readiness_contracts
        .pointer("/same_machine_performance/current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        blockers.push(json!({
            "id": "readiness-same-machine-performance-machine-contract-missing",
            "severity": "high",
            "message": "Same-machine performance proof is not durable serializer-backed evidence yet; import the target benchmark receipt into the canonical .dx JSON/SR/machine receipt before using it as readiness evidence.",
            "source_contract": SAME_MACHINE_PERFORMANCE_RECEIPT_SR,
            "machine_contract": SAME_MACHINE_PERFORMANCE_RECEIPT_MACHINE,
            "legacy_json_read_model": SAME_MACHINE_PERFORMANCE_RECEIPT,
            "collection_receipt": SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT,
            "replay_command": SAME_MACHINE_PERFORMANCE_REPLAY_COMMAND,
            "raw_replay_command": SAME_MACHINE_PERFORMANCE_RAW_REPLAY_COMMAND,
            "dry_run_command": SAME_MACHINE_PERFORMANCE_DRY_RUN_COMMAND,
            "import_command": SAME_MACHINE_PERFORMANCE_IMPORT_COMMAND,
            "contract_test_command": "node --test benchmarks/dx-www-same-machine-performance-receipt.test.ts",
            "stale_reasons": ["same-machine-performance-receipt-missing"],
            "contract_status": readiness_contracts["same_machine_performance"]
        }));
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == PRODUCTION_HTTP_RECEIPT)
    {
        if receipt
            .pointer("/production_http_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "production-http-local-replay-receipt-missing",
                "severity": "high",
                "message": "Production HTTP local wire replay proof is missing or stale; do not claim ETag, 304, Range, method guard, or precompressed asset readiness until the current local replay receipt exists.",
                "receipt": PRODUCTION_HTTP_RECEIPT,
                "required_schema": PRODUCTION_HTTP_RECEIPT_SCHEMA,
                "serializer_receipt": PRODUCTION_HTTP_RECEIPT_SR,
                "machine_contract": PRODUCTION_HTTP_RECEIPT_MACHINE,
                "replay_command": READINESS_WRITE_RECEIPTS_COMMAND,
                "contract_test_command": PRODUCTION_HTTP_CONTRACT_TEST_COMMAND,
                "stale_reasons": receipt
                    .pointer("/production_http_receipt_status/stale_reasons")
                    .cloned()
                    .unwrap_or_else(|| json!(["production-http-local-replay-receipt-missing"])),
                "receipt_status": receipt["production_http_receipt_status"]
            }));
        }
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT)
    {
        if receipt
            .pointer("/production_http_tcp_preview_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "production-http-tcp-preview-receipt-missing",
                "severity": "high",
                "message": "Production HTTP TCP preview proof is missing or stale; local wire replay does not prove the running dx preview TCP server.",
                "receipt": PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT,
                "required_schema": PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_SCHEMA,
                "serializer_receipt": PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_SR,
                "machine_contract": PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_MACHINE,
                "collect_command": readiness::READINESS_PRODUCTION_HTTP_TCP_PREVIEW_COLLECT_COMMAND,
                "import_command": "dx www readiness --import-production-http-tcp-preview-receipt <tcp-receipt.json> --json --full",
                "contract_test_command": "node --test benchmarks/dx-www-production-preview-tcp-receipt.test.ts",
                "stale_reasons": receipt
                    .pointer("/production_http_tcp_preview_receipt_status/stale_reasons")
                    .cloned()
                    .unwrap_or_else(|| json!(["production-http-tcp-preview-receipt-missing"])),
                "receipt_status": receipt["production_http_tcp_preview_receipt_status"]
            }));
        }
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == BUNDLE_PARTITION_RECEIPT)
    {
        if receipt
            .pointer("/bundle_partition_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "bundle-partition-receipt-missing",
                "severity": "high",
                "message": "Public runtime versus evidence bundle proof is missing or stale; do not claim production deploy cleanliness until the local partition receipt is current.",
                "receipt": BUNDLE_PARTITION_RECEIPT,
                "required_schema": BUNDLE_PARTITION_RECEIPT_SCHEMA,
                "serializer_receipt": BUNDLE_PARTITION_RECEIPT_SR,
                "machine_contract": BUNDLE_PARTITION_RECEIPT_MACHINE,
                "replay_command": READINESS_WRITE_RECEIPTS_COMMAND,
                "contract_test_command": BUNDLE_PARTITION_CONTRACT_TEST_COMMAND,
                "stale_reasons": receipt
                    .pointer("/bundle_partition_receipt_status/stale_reasons")
                    .cloned()
                    .unwrap_or_else(|| json!(["bundle-partition-receipt-missing"])),
                "receipt_status": receipt["bundle_partition_receipt_status"]
            }));
        }
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == BUNDLE_PROVIDER_REPLAY_RECEIPT)
    {
        if receipt
            .pointer("/bundle_provider_replay_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "bundle-provider-replay-receipt-missing",
                "severity": "high",
                "message": "Hosted public runtime versus evidence bundle replay proof is missing or stale; do not claim production deploy cleanliness from local partition receipts alone.",
                "receipt": BUNDLE_PROVIDER_REPLAY_RECEIPT,
                "required_schema": BUNDLE_PROVIDER_REPLAY_RECEIPT_SCHEMA,
                "serializer_receipt": BUNDLE_PROVIDER_REPLAY_RECEIPT_SR,
                "machine_contract": BUNDLE_PROVIDER_REPLAY_RECEIPT_MACHINE,
                "collect_command": readiness::READINESS_BUNDLE_PROVIDER_REPLAY_COLLECT_COMMAND,
                "import_command": "dx www readiness --import-bundle-provider-replay-receipt <bundle-provider-replay-receipt.json> --json --full",
                "contract_test_command": "node --test benchmarks/dx-www-hosted-bundle-replay.test.ts",
                "stale_reasons": receipt
                    .pointer("/bundle_provider_replay_receipt_status/stale_reasons")
                    .cloned()
                    .unwrap_or_else(|| json!(["bundle-provider-replay-receipt-missing"])),
                "receipt_status": receipt["bundle_provider_replay_receipt_status"]
            }));
        }
    }

    if readiness_contracts
        .pointer("/bundle_partition/current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        blockers.push(json!({
            "id": "readiness-bundle-partition-machine-contract-missing",
            "severity": "high",
            "message": "Public runtime versus evidence bundle proof is not serializer-backed yet; do not clear the production-output cleanliness gate until the JSON receipt, .sr source, and generated .machine contract are current.",
            "source_contract": BUNDLE_PARTITION_RECEIPT_SR,
            "machine_contract": BUNDLE_PARTITION_RECEIPT_MACHINE,
            "legacy_json_read_model": BUNDLE_PARTITION_RECEIPT,
            "replay_command": READINESS_WRITE_RECEIPTS_COMMAND,
            "contract_test_command": BUNDLE_PARTITION_CONTRACT_TEST_COMMAND,
            "stale_reasons": ["bundle-partition-receipt-missing"],
            "contract_status": readiness_contracts["bundle_partition"]
        }));
    }

    if readiness_contracts
        .pointer("/bundle_provider_replay/current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        blockers.push(json!({
            "id": "readiness-bundle-provider-replay-machine-contract-missing",
            "severity": "high",
            "message": "Hosted public runtime versus evidence bundle replay proof is not serializer-backed yet; do not clear the production-output cleanliness gate until the hosted replay JSON receipt, .sr source, and generated .machine contract are current.",
            "source_contract": BUNDLE_PROVIDER_REPLAY_RECEIPT_SR,
            "machine_contract": BUNDLE_PROVIDER_REPLAY_RECEIPT_MACHINE,
            "legacy_json_read_model": BUNDLE_PROVIDER_REPLAY_RECEIPT,
            "collect_command": readiness::READINESS_BUNDLE_PROVIDER_REPLAY_COLLECT_COMMAND,
            "import_command": "dx www readiness --import-bundle-provider-replay-receipt <bundle-provider-replay-receipt.json> --json --full",
            "contract_test_command": "node --test benchmarks/dx-www-hosted-bundle-replay.test.ts",
            "stale_reasons": ["bundle-provider-replay-receipt-missing"],
            "contract_status": readiness_contracts["bundle_provider_replay"]
        }));
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == SERVER_ACTION_REPLAY_LEDGER_RECEIPT)
    {
        if receipt
            .pointer("/server_action_replay_ledger_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "server-action-replay-ledger-receipt-missing",
                "severity": "high",
                "message": "Server-action replay ledger proof is missing or stale; do not claim route/action runtime maturity until the local production-preview replay ledger receipt is current.",
                "receipt": SERVER_ACTION_REPLAY_LEDGER_RECEIPT,
                "required_schema": SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SCHEMA,
                "serializer_receipt": SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SR,
                "machine_contract": SERVER_ACTION_REPLAY_LEDGER_RECEIPT_MACHINE,
                "replay_command": READINESS_WRITE_RECEIPTS_COMMAND,
                "contract_test_command": SERVER_ACTION_REPLAY_LEDGER_CONTRACT_TEST_COMMAND,
                "stale_reasons": receipt
                    .pointer("/server_action_replay_ledger_receipt_status/stale_reasons")
                    .cloned()
                    .unwrap_or_else(|| json!(["server-action-replay-ledger-receipt-missing"])),
                "receipt_status": receipt["server_action_replay_ledger_receipt_status"]
            }));
        }
    }

    if readiness_contracts
        .pointer("/server_action_replay_ledger/current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        blockers.push(json!({
            "id": "readiness-server-action-replay-ledger-machine-contract-missing",
            "severity": "high",
            "message": "Server-action replay ledger proof is not serializer-backed yet; do not clear the route/action runtime gate until the JSON receipt, .sr source, and generated .machine contract are current.",
            "source_contract": SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SR,
            "machine_contract": SERVER_ACTION_REPLAY_LEDGER_RECEIPT_MACHINE,
            "legacy_json_read_model": SERVER_ACTION_REPLAY_LEDGER_RECEIPT,
            "replay_command": READINESS_WRITE_RECEIPTS_COMMAND,
            "contract_test_command": SERVER_ACTION_REPLAY_LEDGER_CONTRACT_TEST_COMMAND,
            "stale_reasons": ["server-action-replay-ledger-receipt-missing"],
            "contract_status": readiness_contracts["server_action_replay_ledger"]
        }));
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == PRIMITIVE_PROOF_RECEIPT)
    {
        if receipt
            .pointer("/primitive_proof_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "primitive-proof-receipt-missing",
                "severity": "high",
                "message": "Primitive Image/Font/Script/Wasm proof is missing or stale; do not claim primitive maturity until the source-owned primitive proof receipt is current.",
                "receipt": PRIMITIVE_PROOF_RECEIPT,
                "required_schema": PRIMITIVE_PROOF_RECEIPT_SCHEMA,
                "serializer_receipt": PRIMITIVE_PROOF_RECEIPT_SR,
                "machine_contract": PRIMITIVE_PROOF_RECEIPT_MACHINE,
                "replay_command": READINESS_WRITE_RECEIPTS_COMMAND,
                "contract_test_command": PRIMITIVE_PROOF_CONTRACT_TEST_COMMAND,
                "stale_reasons": receipt
                    .pointer("/primitive_proof_receipt_status/stale_reasons")
                    .cloned()
                    .unwrap_or_else(|| json!(["primitive-proof-receipt-missing"])),
                "receipt_status": receipt["primitive_proof_receipt_status"]
            }));
        }
    }

    if readiness_contracts
        .pointer("/primitive_proof/current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        blockers.push(json!({
            "id": "readiness-primitive-proof-machine-contract-missing",
            "severity": "high",
            "message": "Primitive proof is not serializer-backed yet; do not clear the primitive gate until the JSON receipt, .sr source, and generated .machine contract are current.",
            "source_contract": PRIMITIVE_PROOF_RECEIPT_SR,
            "machine_contract": PRIMITIVE_PROOF_RECEIPT_MACHINE,
            "legacy_json_read_model": PRIMITIVE_PROOF_RECEIPT,
            "replay_command": READINESS_WRITE_RECEIPTS_COMMAND,
            "contract_test_command": PRIMITIVE_PROOF_CONTRACT_TEST_COMMAND,
            "stale_reasons": ["primitive-proof-receipt-missing"],
            "contract_status": readiness_contracts["primitive_proof"]
        }));
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == NATIVE_EVENT_BROWSER_BINDER_RECEIPT)
    {
        if receipt
            .pointer("/native_event_browser_binder_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "native-event-browser-binder-receipt-missing",
                "severity": "high",
                "message": "Native DOM event browser binder proof is missing or stale; Node VM replay is not enough for the local browser replay gate.",
                "receipt": NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
                "required_schema": NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SCHEMA,
                "browser_receipt_gate": browser_receipt_action_metadata(
                    "native-event-browser-binder",
                    NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
                    NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR,
                    NATIVE_EVENT_BROWSER_BINDER_RECEIPT_MACHINE,
                    NATIVE_EVENT_BROWSER_BINDER_IMPORT_COMMAND,
                    "native-event-browser-binder-receipt-missing",
                ),
                "replay_command": READINESS_INSPECT_COMMAND,
                "import_command": NATIVE_EVENT_BROWSER_BINDER_IMPORT_COMMAND,
                "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
                "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
                "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
                "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
                "stale_reasons": receipt
                    .pointer("/native_event_browser_binder_receipt_status/stale_reasons")
                    .cloned()
                    .unwrap_or_else(|| json!(["native-event-browser-binder-receipt-missing"])),
                "receipt_status": receipt["native_event_browser_binder_receipt_status"]
            }));
        }
    }

    if readiness_contracts
        .pointer("/native_event_browser_binder/current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        blockers.push(json!({
            "id": "readiness-native-event-browser-binder-machine-contract-missing",
            "severity": "high",
            "message": "Native DOM event browser binder proof is not serializer-backed yet; do not claim durable browser binder proof until the JSON receipt, .sr source, and generated .machine contract are current.",
            "source_contract": NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR,
            "machine_contract": NATIVE_EVENT_BROWSER_BINDER_RECEIPT_MACHINE,
            "legacy_json_read_model": NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
            "browser_receipt_gate": browser_receipt_action_metadata(
                "native-event-browser-binder",
                NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
                NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR,
                NATIVE_EVENT_BROWSER_BINDER_RECEIPT_MACHINE,
                NATIVE_EVENT_BROWSER_BINDER_IMPORT_COMMAND,
                "native-event-browser-binder-receipt-missing",
            ),
            "replay_command": READINESS_INSPECT_COMMAND,
            "import_command": NATIVE_EVENT_BROWSER_BINDER_IMPORT_COMMAND,
            "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
            "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
            "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
            "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
            "stale_reasons": ["native-event-browser-binder-receipt-missing"],
            "contract_status": readiness_contracts["native_event_browser_binder"]
        }));
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == STATE_RUNTIME_BROWSER_RECEIPT)
    {
        if receipt
            .pointer("/state_runtime_browser_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "state-runtime-browser-receipt-missing",
                "severity": "high",
                "message": "DX-native state runtime browser proof is missing or stale; Node VM fake-DOM replay is not enough for the local state/derived/effect/action browser gate.",
                "receipt": STATE_RUNTIME_BROWSER_RECEIPT,
                "required_schema": STATE_RUNTIME_BROWSER_RECEIPT_SCHEMA,
                "browser_receipt_gate": browser_receipt_action_metadata(
                    "state-runtime-browser",
                    STATE_RUNTIME_BROWSER_RECEIPT,
                    STATE_RUNTIME_BROWSER_RECEIPT_SR,
                    STATE_RUNTIME_BROWSER_RECEIPT_MACHINE,
                    STATE_RUNTIME_BROWSER_IMPORT_COMMAND,
                    "state-runtime-browser-receipt-missing",
                ),
                "replay_command": READINESS_INSPECT_COMMAND,
                "import_command": STATE_RUNTIME_BROWSER_IMPORT_COMMAND,
                "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
                "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
                "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
                "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
                "stale_reasons": receipt
                    .pointer("/state_runtime_browser_receipt_status/stale_reasons")
                    .cloned()
                    .unwrap_or_else(|| json!(["state-runtime-browser-receipt-missing"])),
                "receipt_status": receipt["state_runtime_browser_receipt_status"]
            }));
        }
    }

    if readiness_contracts
        .pointer("/state_runtime_browser/current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        blockers.push(json!({
            "id": "readiness-state-runtime-browser-machine-contract-missing",
            "severity": "high",
            "message": "DX-native state runtime browser proof is not serializer-backed yet; do not claim durable reactivity browser proof until the JSON receipt, .sr source, and generated .machine contract are current.",
            "source_contract": STATE_RUNTIME_BROWSER_RECEIPT_SR,
            "machine_contract": STATE_RUNTIME_BROWSER_RECEIPT_MACHINE,
            "legacy_json_read_model": STATE_RUNTIME_BROWSER_RECEIPT,
            "browser_receipt_gate": browser_receipt_action_metadata(
                "state-runtime-browser",
                STATE_RUNTIME_BROWSER_RECEIPT,
                STATE_RUNTIME_BROWSER_RECEIPT_SR,
                STATE_RUNTIME_BROWSER_RECEIPT_MACHINE,
                STATE_RUNTIME_BROWSER_IMPORT_COMMAND,
                "state-runtime-browser-receipt-missing",
            ),
            "replay_command": READINESS_INSPECT_COMMAND,
            "import_command": STATE_RUNTIME_BROWSER_IMPORT_COMMAND,
            "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
            "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
            "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
            "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
            "stale_reasons": ["state-runtime-browser-receipt-missing"],
            "contract_status": readiness_contracts["state_runtime_browser"]
        }));
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == REACTIVITY_MODEL_RECEIPT)
    {
        if receipt
            .pointer("/reactivity_model_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "reactivity-model-receipt-missing",
                "severity": "high",
                "message": "Source-owned DX-native reactivity release-readiness proof is missing or stale; do not claim state/derived/effect/action source receipt freshness until the latest reactivity-model receipt is current. Browser state runtime replay remains a separate gate.",
                "receipt": REACTIVITY_MODEL_RECEIPT,
                "required_schema": REACTIVITY_MODEL_RECEIPT_SCHEMA,
                "receipt_status": receipt["reactivity_model_receipt_status"]
            }));
        }
    }

    if readiness_contracts
        .pointer("/reactivity_model/current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        blockers.push(json!({
            "id": "readiness-reactivity-model-machine-contract-missing",
            "severity": "high",
            "message": "Source-owned DX-native reactivity release-readiness proof is not serializer-backed yet; do not claim durable reactivity model proof until the JSON receipt, .sr source, and generated .machine contract are current. Browser state runtime replay remains a separate gate.",
            "source_contract": REACTIVITY_MODEL_RECEIPT_SR,
            "machine_contract": REACTIVITY_MODEL_RECEIPT_MACHINE,
            "legacy_json_read_model": REACTIVITY_MODEL_RECEIPT,
            "contract_status": readiness_contracts["reactivity_model"]
        }));
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == DOCS_ONBOARDING_RECEIPT)
    {
        if receipt
            .pointer("/docs_onboarding_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "docs-onboarding-receipt-missing",
                "severity": "high",
                "message": "Source-owned docs/onboarding release-readiness proof is missing or stale; do not claim docs-doctor/onboarding receipt freshness until the latest docs-onboarding receipt is current. Generated/archive cleanup and docs-doctor command replay remain separate gates.",
                "receipt": DOCS_ONBOARDING_RECEIPT,
                "required_schema": DOCS_ONBOARDING_RECEIPT_SCHEMA,
                "receipt_status": receipt["docs_onboarding_receipt_status"]
            }));
        } else {
            let status = &receipt["docs_onboarding_receipt_status"];
            let warning_surfaces_clean = status
                .get("generated_archived_warning_surfaces_clean")
                .and_then(Value::as_bool)
                == Some(true);
            let warning_surfaces_promoted = status
                .get("generated_archived_warning_surfaces_promoted")
                .and_then(Value::as_bool)
                == Some(true);
            if !warning_surfaces_clean && !warning_surfaces_promoted {
                blockers.push(json!({
                    "id": "docs-onboarding-generated-archived-warning-cleanup",
                    "severity": "high",
                    "message": "Docs-doctor generated/archive compatibility surfaces remain warning-only; do not claim docs/onboarding release readiness until generated archives are cleaned or promoted with explicit ownership.",
                    "receipt": DOCS_ONBOARDING_RECEIPT,
                    "replay_command": "dx www docs-doctor --json",
                    "receipt_status": status
                }));
            }
        }
    }

    if readiness_contracts
        .pointer("/docs_onboarding/current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        blockers.push(json!({
            "id": "readiness-docs-onboarding-machine-contract-missing",
            "severity": "high",
            "message": "Source-owned docs/onboarding release-readiness proof is not serializer-backed yet; do not claim durable docs/onboarding proof until the JSON receipt, .sr source, and generated .machine contract are current. Generated/archive cleanup and docs-doctor command replay remain separate gates.",
            "source_contract": DOCS_ONBOARDING_RECEIPT_SR,
            "machine_contract": DOCS_ONBOARDING_RECEIPT_MACHINE,
            "legacy_json_read_model": DOCS_ONBOARDING_RECEIPT,
            "contract_status": readiness_contracts["docs_onboarding"]
        }));
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT)
    {
        if receipt
            .pointer("/docs_doctor_command_replay_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "docs-doctor-command-replay-receipt-missing",
                "severity": "high",
                "message": "Docs-doctor command replay proof is missing or stale; run dx www docs-doctor --json --write-receipt before claiming docs/onboarding command replay freshness.",
                "receipt": DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT,
                "required_schema": DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_SCHEMA,
                "receipt_status": receipt["docs_doctor_command_replay_receipt_status"]
            }));
        }
    }

    if readiness_contracts
        .pointer("/docs_doctor_command_replay/current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        blockers.push(json!({
            "id": "readiness-docs-doctor-command-replay-machine-contract-missing",
            "severity": "high",
            "message": "Docs-doctor command replay proof is not serializer-backed yet; do not clear the docs-doctor replay gate until the JSON receipt, .sr source, and generated .machine contract are current.",
            "source_contract": DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_SR,
            "machine_contract": DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_MACHINE,
            "legacy_json_read_model": DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT,
            "contract_status": readiness_contracts["docs_doctor_command_replay"]
        }));
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == ISLAND_ABI_RECEIPT)
    {
        if receipt
            .pointer("/island_abi_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "island-abi-receipt-missing",
                "severity": "high",
                "message": "Source-owned islands ABI release-readiness proof is missing or stale; do not claim camelCase islands ABI receipt freshness until the latest island-abi receipt is current.",
                "receipt": ISLAND_ABI_RECEIPT,
                "required_schema": ISLAND_ABI_RECEIPT_SCHEMA,
                "serializer_receipt": ISLAND_ABI_RECEIPT_SR,
                "machine_contract": ISLAND_ABI_RECEIPT_MACHINE,
                "replay_command": READINESS_WRITE_RECEIPTS_COMMAND,
                "contract_test_command": ISLAND_ABI_CONTRACT_TEST_COMMAND,
                "stale_reasons": receipt
                    .pointer("/island_abi_receipt_status/stale_reasons")
                    .cloned()
                    .unwrap_or_else(|| json!(["island-abi-receipt-missing"])),
                "receipt_status": receipt["island_abi_receipt_status"]
            }));
        }
    }

    if readiness_contracts
        .pointer("/island_abi/current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        blockers.push(json!({
            "id": "readiness-island-abi-machine-contract-missing",
            "severity": "high",
            "message": "Source-owned islands ABI release-readiness proof is not serializer-backed yet; do not claim durable islands ABI proof until the JSON receipt, .sr source, and generated .machine contract are current.",
            "source_contract": ISLAND_ABI_RECEIPT_SR,
            "machine_contract": ISLAND_ABI_RECEIPT_MACHINE,
            "legacy_json_read_model": ISLAND_ABI_RECEIPT,
            "replay_command": READINESS_WRITE_RECEIPTS_COMMAND,
            "contract_test_command": ISLAND_ABI_CONTRACT_TEST_COMMAND,
            "stale_reasons": ["island-abi-receipt-missing"],
            "contract_status": readiness_contracts["island_abi"]
        }));
    }

    if let Some(receipt) = receipt_paths
        .iter()
        .find(|receipt| receipt["path"] == ISLAND_BROWSER_RECEIPT)
    {
        if receipt
            .pointer("/island_browser_receipt_status/current")
            .and_then(Value::as_bool)
            != Some(true)
        {
            blockers.push(json!({
                "id": "island-browser-receipt-missing",
                "severity": "high",
                "message": "Source-owned island browser replay proof is missing or stale; capture the canonical /islands starter route in a real local browser and import the generated receipt before claiming local island runtime execution.",
                "receipt": ISLAND_BROWSER_RECEIPT,
                "required_schema": ISLAND_BROWSER_RECEIPT_SCHEMA,
                "browser_receipt_gate": browser_receipt_action_metadata(
                    "island-browser",
                    ISLAND_BROWSER_RECEIPT,
                    ISLAND_BROWSER_RECEIPT_SR,
                    ISLAND_BROWSER_RECEIPT_MACHINE,
                    ISLAND_BROWSER_IMPORT_COMMAND,
                    "island-browser-receipt-missing",
                ),
                "replay_command": READINESS_INSPECT_COMMAND,
                "import_command": ISLAND_BROWSER_IMPORT_COMMAND,
                "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
                "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
                "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
                "snapshot_import_command": BROWSER_PAGE_SNAPSHOT_IMPORT_COMMAND,
                "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
                "canonical_browser_proof_target": browser_receipt_proof_target("island-browser"),
                "stale_reasons": receipt
                    .pointer("/island_browser_receipt_status/stale_reasons")
                    .cloned()
                    .unwrap_or_else(|| json!(["island-browser-receipt-missing"])),
                "receipt_status": receipt["island_browser_receipt_status"]
            }));
        }
    }

    if readiness_contracts
        .pointer("/island_browser/current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        blockers.push(json!({
            "id": "readiness-island-browser-machine-contract-missing",
            "severity": "high",
            "message": "Source-owned island browser replay proof is not serializer-backed yet; do not claim durable island browser proof until the JSON receipt, .sr source, and generated .machine contract are current.",
            "source_contract": ISLAND_BROWSER_RECEIPT_SR,
            "machine_contract": ISLAND_BROWSER_RECEIPT_MACHINE,
            "legacy_json_read_model": ISLAND_BROWSER_RECEIPT,
            "browser_receipt_gate": browser_receipt_action_metadata(
                "island-browser",
                ISLAND_BROWSER_RECEIPT,
                ISLAND_BROWSER_RECEIPT_SR,
                ISLAND_BROWSER_RECEIPT_MACHINE,
                ISLAND_BROWSER_IMPORT_COMMAND,
                "island-browser-receipt-missing",
            ),
            "replay_command": READINESS_INSPECT_COMMAND,
            "import_command": ISLAND_BROWSER_IMPORT_COMMAND,
            "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
            "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
            "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
            "snapshot_import_command": BROWSER_PAGE_SNAPSHOT_IMPORT_COMMAND,
            "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
            "canonical_browser_proof_target": browser_receipt_proof_target("island-browser"),
            "stale_reasons": ["island-browser-receipt-missing"],
            "contract_status": readiness_contracts["island_browser"]
        }));
    }

    if let Some(installed) =
        read_json(&cwd.join(".dx/receipts/build/installed-binary-smoke-latest.json"))
    {
        if installed.get("passed").and_then(Value::as_bool) == Some(false) {
            blockers.push(json!({
                "id": "installed-binary-smoke-failed",
                "severity": "high",
                "message": "Installed DX binary smoke receipt is present but failing.",
                "receipt": ".dx/receipts/build/installed-binary-smoke-latest.json"
            }));
        }
    }

    match read_json(&cwd.join(TEMPLATE_CHECK_RECEIPT)) {
        Some(check) => {
            if !readiness_receipt_gate_metadata_current(&check) {
                blockers.push(json!({
                    "id": "template-check-readiness-gate-stale",
                    "severity": "high",
                    "message": "Template dx-check receipt is missing safe release-readiness metadata or replay commands.",
                    "receipt": TEMPLATE_CHECK_RECEIPT,
                    "readiness_receipt_gate_status": readiness_receipt_gate_status(&check)
                }));
            }

            let score = check_score_value(&check);
            let max_score = check_score_max(&check).unwrap_or(500);
            let launch_floor = if max_score <= 100 { 90 } else { 450 };
            if score.is_some_and(|value| value < launch_floor) {
                blockers.push(json!({
                    "id": "template-check-score-below-launch-bar",
                    "severity": "medium",
                    "message": format!("Template dx-check receipt is below the {launch_floor}/{max_score} launch bar."),
                    "score": score,
                    "max_score": max_score,
                    "receipt": TEMPLATE_CHECK_RECEIPT
                }));
            }
        }
        None => blockers.push(json!({
            "id": "template-check-readiness-gate-stale",
            "severity": "high",
            "message": "Template dx-check receipt is missing, unreadable, or missing safe release-readiness metadata or replay commands.",
            "receipt": TEMPLATE_CHECK_RECEIPT,
            "readiness_receipt_gate_status": readiness_receipt_gate_status_missing()
        })),
    }

    blockers
}

fn devtools_visual_edit_receipt_status(receipt: Option<&Value>) -> Value {
    let stale_reasons = receipt
        .map(devtools_visual_edit_receipt_stale_reasons)
        .unwrap_or_else(|| {
            vec![
                "visual-edit-browser-workbench-receipt-missing".to_string(),
                "receipt-missing".to_string(),
            ]
        });
    let browser_receipt_stale_reasons = receipt
        .map(devtools_visual_edit_browser_receipt_stale_reasons)
        .unwrap_or_else(|| vec!["visual-edit-browser-workbench-receipt-missing".to_string()]);
    json!({
        "contract": DEVTOOLS_VISUAL_EDIT_RECEIPT_SCHEMA,
        "current": stale_reasons.is_empty(),
        "browser_receipt_gate": browser_receipt_action_metadata(
            "visual-edit-workbench-receipts",
            DEVTOOLS_VISUAL_EDIT_RECEIPT,
            DEVTOOLS_VISUAL_EDIT_RECEIPT_SR,
            DEVTOOLS_VISUAL_EDIT_RECEIPT_MACHINE,
            VISUAL_EDIT_BROWSER_IMPORT_COMMAND,
            "visual-edit-browser-workbench-receipt-missing",
        ),
        "canonical_browser_proof_target": browser_receipt_proof_target("visual-edit-workbench-receipts"),
        "browser_receipt_current": browser_receipt_stale_reasons.is_empty(),
        "browser_receipt_required": true,
        "browser_receipt_stale_reasons": browser_receipt_stale_reasons,
        "replay_command": READINESS_INSPECT_COMMAND,
        "write_replay_command": VISUAL_EDIT_FOUNDATION_REPLAY_COMMAND,
        "write_receipts_command": READINESS_WRITE_RECEIPTS_COMMAND,
        "import_command": VISUAL_EDIT_BROWSER_IMPORT_COMMAND,
        "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
        "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
        "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
        "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "operation": receipt
            .and_then(|value| value.get("operation"))
            .and_then(Value::as_str),
        "applied": receipt
            .and_then(|value| value.get("applied"))
            .and_then(Value::as_bool),
        "undone": receipt
            .and_then(|value| value.get("undone"))
            .and_then(Value::as_bool),
        "source_mutated": receipt
            .and_then(|value| value.get("source_mutated"))
            .and_then(Value::as_bool),
        "source_path": receipt
            .and_then(|value| value.get("source_path"))
            .and_then(Value::as_str),
        "receipt_durability": receipt
            .and_then(|value| value.get("receipt_durability"))
            .and_then(Value::as_str),
        "receipt_write_status": receipt
            .and_then(|value| value.get("receipt_write_status"))
            .and_then(Value::as_str),
        "undo_supported": receipt
            .and_then(|value| value.get("undo_supported"))
            .and_then(Value::as_bool),
        "undo_receipt_status": receipt
            .and_then(|value| value.get("undo_receipt_status"))
            .and_then(Value::as_str),
        "browser_workbench_replay": receipt
            .and_then(|value| value.get("browser_workbench_replay"))
            .and_then(Value::as_str),
        "release_ready": receipt
            .and_then(|value| value.get("release_ready"))
            .and_then(Value::as_bool),
        "fastest_world_claim": receipt
            .and_then(|value| value.get("fastest_world_claim"))
            .and_then(Value::as_bool),
        "passed": receipt
            .and_then(|value| value.get("passed"))
            .and_then(Value::as_bool),
        "status": receipt
            .and_then(|value| value.get("status"))
            .and_then(Value::as_str),
        "receipt_freshness": receipt
            .and_then(|value| value.get("receipt_freshness"))
            .and_then(Value::as_str),
        "stale_reasons": stale_reasons,
    })
}

fn devtools_visual_edit_receipt_is_current(receipt: &Value) -> bool {
    devtools_visual_edit_receipt_stale_reasons(receipt).is_empty()
}

fn devtools_visual_edit_receipt_stale_reasons(receipt: &Value) -> Vec<String> {
    if devtools_visual_edit_browser_workbench_receipt_is_current(receipt) {
        return Vec::new();
    }

    let mut reasons = Vec::new();
    let operation = receipt.get("operation").and_then(Value::as_str);
    let source_mutated = receipt.get("source_mutated").and_then(Value::as_bool);
    let undo_receipt_status = receipt.get("undo_receipt_status").and_then(Value::as_str);

    if receipt.get("schema").and_then(Value::as_str) != Some(DEVTOOLS_VISUAL_EDIT_RECEIPT_SCHEMA) {
        reasons.push("schema-mismatch".to_string());
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false) {
        reasons.push("release-ready-claim-not-false".to_string());
    }
    if receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false) {
        reasons.push("global-speed-claim-not-false".to_string());
    }
    if !matches!(operation, Some("style-apply" | "style-undo")) {
        reasons.push("unsupported-visual-edit-operation".to_string());
    }
    if operation == Some("style-apply")
        && receipt.get("applied").and_then(Value::as_bool) != Some(true)
    {
        reasons.push("style-apply-not-applied".to_string());
    }
    if operation == Some("style-undo")
        && receipt.get("undone").and_then(Value::as_bool) != Some(true)
    {
        reasons.push("style-undo-not-undone".to_string());
    }
    if source_mutated != Some(true) {
        reasons.push("source-not-mutated".to_string());
    }
    if receipt.get("source_path").and_then(Value::as_str).is_none() {
        reasons.push("missing-source-path".to_string());
    }
    if receipt.get("receipt_durability").and_then(Value::as_str) != Some("json-sr-machine-written")
    {
        reasons.push("receipt-durability-not-written".to_string());
    }
    if receipt.get("receipt_write_status").and_then(Value::as_str)
        != Some("json-sr-machine-written")
    {
        reasons.push("receipt-write-status-not-written".to_string());
    }
    if receipt.get("undo_supported").and_then(Value::as_bool) != Some(true) {
        reasons.push("undo-not-supported".to_string());
    }
    if operation == Some("style-apply") && undo_receipt_status != Some("pending") {
        reasons.push("style-apply-undo-status-not-pending".to_string());
    }
    if operation == Some("style-undo") && undo_receipt_status != Some("json-sr-machine-written") {
        reasons.push("style-undo-receipt-status-not-written".to_string());
    }
    if receipt
        .get("browser_workbench_replay")
        .and_then(Value::as_str)
        .is_none()
    {
        reasons.push("missing-browser-workbench-replay-status".to_string());
    }

    reasons
}

fn devtools_visual_edit_browser_workbench_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str) == Some(DEVTOOLS_VISUAL_EDIT_RECEIPT_SCHEMA)
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("visual_replay_attempted")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt.get("visual_replay_status").and_then(Value::as_str) == Some("current")
        && receipt
            .get("devtools_global_present")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("browser_workbench_replay")
            .and_then(Value::as_str)
            == Some("current")
        && receipt.get("proof_scope").and_then(Value::as_str)
            == Some("local-in-app-browser-visual-edit-workbench-replay")
        && receipt_string_array_contains(
            receipt.get("workbench_phases").unwrap_or(&Value::Null),
            "inspect",
        )
        && receipt_string_array_contains(
            receipt.get("workbench_phases").unwrap_or(&Value::Null),
            "cascade",
        )
        && receipt_string_array_contains(
            receipt.get("workbench_phases").unwrap_or(&Value::Null),
            "preview",
        )
        && receipt_string_array_contains(
            receipt.get("workbench_phases").unwrap_or(&Value::Null),
            "apply",
        )
        && receipt_string_array_contains(
            receipt.get("workbench_phases").unwrap_or(&Value::Null),
            "undo",
        )
        && receipt_string_array_contains(
            receipt.get("workbench_phases").unwrap_or(&Value::Null),
            "receipt",
        )
        && receipt
            .get("missing_workbench_phases")
            .and_then(Value::as_array)
            .is_some_and(Vec::is_empty)
        && receipt
            .get("inspected_element_present")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt.get("cascade_inspected").and_then(Value::as_bool) == Some(true)
        && receipt
            .get("preview_source_mutated")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt.get("apply_source_mutated").and_then(Value::as_bool) == Some(true)
        && receipt.get("undo_source_restored").and_then(Value::as_bool) == Some(true)
        && receipt
            .get("safe_local_source_target_known")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("apply_receipt_written")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt.get("undo_receipt_written").and_then(Value::as_bool) == Some(true)
        && receipt.get("receipt_durability").and_then(Value::as_str)
            == Some("json-sr-machine-written")
        && receipt_snapshot_hash_is_current(receipt.get("browser_snapshot_hash"))
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
}

fn devtools_visual_edit_browser_receipt_stale_reasons(receipt: &Value) -> Vec<String> {
    let mut reasons = Vec::new();

    if receipt
        .get("browser_runtime_executed")
        .and_then(Value::as_bool)
        != Some(true)
    {
        reasons.push("visual-edit-browser-runtime-not-executed".to_string());
    }
    if receipt
        .get("browser_workbench_replay")
        .and_then(Value::as_str)
        != Some("current")
    {
        reasons.push("visual-edit-browser-workbench-replay-missing".to_string());
    }
    if receipt.get("proof_scope").and_then(Value::as_str)
        != Some("local-in-app-browser-visual-edit-workbench-replay")
    {
        reasons.push("visual-edit-proof-scope-not-local-browser-workbench-replay".to_string());
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false) {
        reasons.push("release-ready-claim-not-false".to_string());
    }
    if receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false) {
        reasons.push("global-speed-claim-not-false".to_string());
    }

    reasons
}

fn receipt_snapshot_hash_is_current(value: Option<&Value>) -> bool {
    value.and_then(Value::as_str).is_some_and(|hash| {
        let digest = hash.strip_prefix("sha256:").unwrap_or(hash);
        digest.len() == 64 && digest.chars().all(|ch| ch.is_ascii_hexdigit())
    })
}

fn native_event_catalog_receipt_status(receipt: Option<&Value>) -> Value {
    let catalog_integrity = readiness::native_dom_event_catalog_integrity();
    let stale_reasons = receipt
        .map(native_event_catalog_receipt_stale_reasons)
        .unwrap_or_else(|| vec!["native-event-catalog-receipt-missing".to_string()]);
    json!({
        "contract": NATIVE_EVENT_CATALOG_RECEIPT_SCHEMA,
        "current": receipt.is_some_and(native_event_catalog_receipt_is_current),
        "stale_reasons": stale_reasons,
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "schema_revision": receipt
            .and_then(|value| value.get("schema_revision"))
            .and_then(Value::as_u64),
        "id": receipt
            .and_then(|value| value.get("id"))
            .and_then(Value::as_str),
        "passed": receipt
            .and_then(|value| value.get("passed"))
            .and_then(Value::as_bool),
        "status": receipt
            .and_then(|value| value.get("status"))
            .and_then(Value::as_str),
        "receipt_freshness": receipt
            .and_then(|value| value.get("receipt_freshness"))
            .and_then(Value::as_str),
        "catalog_hash": receipt
            .and_then(|value| value.get("catalog_hash"))
            .and_then(Value::as_str),
        "catalog_count": receipt
            .and_then(|value| value.get("catalog_count"))
            .and_then(Value::as_u64),
        "expected_catalog_count": catalog_integrity.catalog_count,
        "expected_catalog_hash": catalog_integrity.catalog_hash,
        "mdn_snapshot_status": receipt
            .and_then(|value| value.get("mdn_snapshot_status"))
            .and_then(Value::as_str),
        "mdn_event_count": receipt
            .and_then(|value| value.pointer("/mdn_event_freshness/mdn_event_count"))
            .and_then(Value::as_u64),
        "compiler_event_count": receipt
            .and_then(|value| value.pointer("/mdn_event_freshness/compiler_event_count"))
            .and_then(Value::as_u64),
        "missing_from_compiler_count": receipt
            .and_then(|value| value.pointer("/mdn_event_freshness/missing_from_compiler_count"))
            .and_then(Value::as_u64),
        "extra_in_compiler_count": receipt
            .and_then(|value| value.pointer("/mdn_event_freshness/extra_in_compiler_count"))
            .and_then(Value::as_u64),
        "mdn_exact_match": receipt
            .and_then(|value| value.pointer("/mdn_event_freshness/exact_match"))
            .and_then(Value::as_bool),
    })
}

fn native_event_catalog_receipt_is_current(receipt: &Value) -> bool {
    native_event_catalog_receipt_stale_reasons(receipt).is_empty()
}

fn native_event_catalog_receipt_stale_reasons(receipt: &Value) -> Vec<String> {
    let catalog_integrity = readiness::native_dom_event_catalog_integrity();
    let mut stale_reasons = Vec::new();

    if receipt.get("schema").and_then(Value::as_str) != Some(NATIVE_EVENT_CATALOG_RECEIPT_SCHEMA) {
        stale_reasons.push("schema-mismatch-or-missing".to_string());
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("passed-not-true".to_string());
    }
    if receipt.get("catalog_count").and_then(Value::as_u64)
        != Some(catalog_integrity.catalog_count as u64)
    {
        stale_reasons.push("native-event-catalog-count-stale".to_string());
    }
    if receipt.get("catalog_hash").and_then(Value::as_str)
        != Some(catalog_integrity.catalog_hash.as_str())
    {
        stale_reasons.push("native-event-catalog-hash-stale".to_string());
    }
    if !(native_event_catalog_receipt_status_is_current(
        receipt.get("status").and_then(Value::as_str),
    ) || native_event_catalog_receipt_status_is_current(
        receipt.get("receipt_freshness").and_then(Value::as_str),
    )) {
        stale_reasons.push("native-event-catalog-status-not-current".to_string());
    }

    stale_reasons
}

fn native_event_catalog_receipt_status_is_current(status: Option<&str>) -> bool {
    matches!(
        status,
        Some("current" | "compiler-catalog-valid-mdn-current")
    )
}

fn no_js_artifact_receipt_status(receipt: Option<&Value>) -> Value {
    json!({
        "contract": NO_JS_ARTIFACT_RECEIPT_SCHEMA,
        "current": receipt.is_some_and(no_js_artifact_receipt_is_current),
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "passed": receipt
            .and_then(|value| value.get("passed"))
            .and_then(Value::as_bool),
        "status": receipt
            .and_then(|value| value.get("status"))
            .and_then(Value::as_str)
            .unwrap_or("missing-no-js-artifact-receipt"),
        "artifact_root": receipt
            .and_then(|value| value.get("artifact_root"))
            .and_then(Value::as_str),
        "artifact_source": receipt
            .and_then(|value| value.get("artifact_source"))
            .and_then(Value::as_str),
        "artifact_path_resolution": receipt
            .and_then(|value| value.get("artifact_path_resolution"))
            .and_then(Value::as_str),
        "html_present": receipt
            .and_then(|value| value.get("html_present"))
            .and_then(Value::as_bool),
        "html_bytes": receipt
            .and_then(|value| value.get("html_bytes"))
            .and_then(Value::as_u64),
        "artifact_html_blake3": receipt
            .and_then(|value| value.get("artifact_html_blake3"))
            .and_then(Value::as_str),
        "script_tag_count": receipt
            .and_then(|value| value.get("script_tag_count"))
            .and_then(Value::as_u64),
        "data_dx_output_mode_tiny_static": receipt
            .and_then(|value| value.get("data_dx_output_mode_tiny_static"))
            .and_then(Value::as_bool),
        "data_dx_js_none": receipt
            .and_then(|value| value.get("data_dx_js_none"))
            .and_then(Value::as_bool),
        "public_packet_present": receipt
            .and_then(|value| value.get("public_packet_present"))
            .and_then(Value::as_bool),
        "public_js_artifact_count": receipt
            .and_then(|value| value.get("public_js_artifact_count"))
            .and_then(Value::as_u64),
        "public_js_artifacts": receipt
            .and_then(|value| value.get("public_js_artifacts"))
            .cloned()
            .unwrap_or(Value::Null),
        "meaningful_html_without_js": receipt
            .and_then(|value| value.get("meaningful_html_without_js"))
            .and_then(Value::as_bool),
        "route_unit_no_js_capable": receipt
            .and_then(|value| value.get("route_unit_no_js_capable"))
            .and_then(Value::as_bool),
        "route_unit_present": receipt
            .and_then(|value| value.get("route_unit_present"))
            .and_then(Value::as_bool),
        "astro_parity_claimed": receipt
            .and_then(|value| value.get("astro_parity_claimed"))
            .and_then(Value::as_bool),
        "live_browser_executed": receipt
            .and_then(|value| value.get("live_browser_executed"))
            .and_then(Value::as_bool),
        "javascript_disabled_browser": receipt
            .and_then(|value| value.get("javascript_disabled_browser"))
            .and_then(Value::as_bool),
        "live_astro_parity_receipt": receipt
            .and_then(|value| value.get("live_astro_parity_receipt"))
            .and_then(Value::as_str),
    })
}

fn no_js_artifact_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str) == Some(NO_JS_ARTIFACT_RECEIPT_SCHEMA)
        && receipt.get("schema_revision").and_then(Value::as_u64) == Some(1)
        && receipt.get("id").and_then(Value::as_str) == Some("tiny-static-no-js-artifact")
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt.get("status").and_then(Value::as_str) == Some("artifact-current")
        && receipt.get("html_present").and_then(Value::as_bool) == Some(true)
        && receipt
            .get("artifact_html_blake3")
            .and_then(Value::as_str)
            .is_some_and(|hash| {
                hash.strip_prefix("blake3:")
                    .is_some_and(|digest| digest.len() == 64)
            })
        && receipt.get("script_tag_count").and_then(Value::as_u64) == Some(0)
        && receipt
            .get("data_dx_output_mode_tiny_static")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt.get("data_dx_js_none").and_then(Value::as_bool) == Some(true)
        && receipt.get("main_present").and_then(Value::as_bool) == Some(true)
        && receipt.get("visible_text_present").and_then(Value::as_bool) == Some(true)
        && receipt
            .get("public_packet_present")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("public_js_artifact_count")
            .and_then(Value::as_u64)
            == Some(0)
        && receipt.get("route_unit_present").and_then(Value::as_bool) == Some(true)
        && receipt
            .get("route_unit_no_js_capable")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("meaningful_html_without_js")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
        && receipt.get("astro_parity_claimed").and_then(Value::as_bool) == Some(false)
        && receipt
            .get("live_browser_executed")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("javascript_disabled_browser")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("live_astro_parity_receipt")
            .and_then(Value::as_str)
            == Some("missing")
}

fn no_js_browser_receipt_status(cwd: &Path, receipt: Option<&Value>) -> Value {
    let stale_reasons = receipt
        .map(|value| no_js_browser_receipt_stale_reasons(cwd, value))
        .unwrap_or_else(|| vec!["no-js-browser-receipt-missing".to_string()]);
    let current = stale_reasons.is_empty();

    json!({
        "contract": NO_JS_BROWSER_RECEIPT_SCHEMA,
        "current": current,
        "browser_receipt_gate": browser_receipt_action_metadata(
            "tiny-static-no-js-browser",
            NO_JS_BROWSER_RECEIPT,
            NO_JS_BROWSER_RECEIPT_SR,
            NO_JS_BROWSER_RECEIPT_MACHINE,
            NO_JS_BROWSER_IMPORT_COMMAND,
            "no-js-browser-receipt-missing",
        ),
        "canonical_browser_proof_target": browser_receipt_proof_target("tiny-static-no-js-browser"),
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "status": if current {
            "current-local-js-disabled-browser-proof"
        } else {
            receipt
                .and_then(|value| value.get("status"))
                .and_then(Value::as_str)
                .unwrap_or("missing-no-js-browser-receipt")
        },
        "browser_receipt_required": true,
        "replay_command": READINESS_INSPECT_COMMAND,
        "write_receipts_command": READINESS_WRITE_RECEIPTS_COMMAND,
        "collector_command": NO_JS_BROWSER_COLLECT_COMMAND,
        "import_command": NO_JS_BROWSER_IMPORT_COMMAND,
        "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
        "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
        "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
        "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
        "stale_reasons": stale_reasons,
        "html_path": receipt
            .and_then(|value| value.get("html_path"))
            .and_then(Value::as_str),
        "artifact_html_blake3": receipt
            .and_then(|value| value.get("artifact_html_blake3"))
            .and_then(Value::as_str),
        "live_browser_executed": receipt
            .and_then(|value| value.get("live_browser_executed"))
            .and_then(Value::as_bool),
        "javascript_disabled_browser": receipt
            .and_then(|value| value.get("javascript_disabled_browser"))
            .and_then(Value::as_bool),
        "page_javascript_enabled": receipt
            .and_then(|value| value.get("page_javascript_enabled"))
            .and_then(Value::as_bool),
        "script_tag_count": receipt
            .and_then(|value| value.get("script_tag_count"))
            .and_then(Value::as_u64),
        "data_dx_output_mode_tiny_static": receipt
            .and_then(|value| value.get("data_dx_output_mode_tiny_static"))
            .and_then(Value::as_bool),
        "data_dx_js_none": receipt
            .and_then(|value| value.get("data_dx_js_none"))
            .and_then(Value::as_bool),
        "semantic_landmark_present": receipt
            .and_then(|value| value.get("semantic_landmark_present"))
            .and_then(Value::as_bool),
        "visible_text_present": receipt
            .and_then(|value| value.get("visible_text_present"))
            .and_then(Value::as_bool),
        "link_count": receipt
            .and_then(|value| value.get("link_count"))
            .and_then(Value::as_u64),
        "form_count": receipt
            .and_then(|value| value.get("form_count"))
            .and_then(Value::as_u64),
        "seo_title_present": receipt
            .and_then(|value| value.get("seo_title_present"))
            .and_then(Value::as_bool),
        "accessibility_signal_count": receipt
            .and_then(|value| value.get("accessibility_signal_count"))
            .and_then(Value::as_u64),
        "release_ready": receipt
            .and_then(|value| value.get("release_ready"))
            .and_then(Value::as_bool),
        "fastest_world_claim": receipt
            .and_then(|value| value.get("fastest_world_claim"))
            .and_then(Value::as_bool),
    })
}

fn no_js_browser_receipt_is_current(cwd: &Path, receipt: &Value) -> bool {
    no_js_browser_receipt_stale_reasons(cwd, receipt).is_empty()
}

fn no_js_browser_receipt_stale_reasons(cwd: &Path, receipt: &Value) -> Vec<String> {
    let mut stale_reasons = Vec::new();

    if receipt.get("schema").and_then(Value::as_str) != Some(NO_JS_BROWSER_RECEIPT_SCHEMA) {
        stale_reasons.push("no-js-browser-schema-mismatch".to_string());
    }
    if !no_js_browser_artifact_hash_matches(cwd, receipt) {
        stale_reasons.push("no-js-browser-artifact-hash-mismatch".to_string());
    }
    if receipt
        .get("live_browser_executed")
        .and_then(Value::as_bool)
        != Some(true)
        || receipt
            .get("javascript_disabled_browser")
            .and_then(Value::as_bool)
            != Some(true)
        || receipt
            .get("page_javascript_enabled")
            .and_then(Value::as_bool)
            != Some(false)
    {
        stale_reasons.push("no-js-browser-execution-flags-invalid".to_string());
    }
    if receipt.get("script_tag_count").and_then(Value::as_u64) != Some(0)
        || receipt
            .get("data_dx_output_mode_tiny_static")
            .and_then(Value::as_bool)
            != Some(true)
        || receipt.get("data_dx_js_none").and_then(Value::as_bool) != Some(true)
    {
        stale_reasons.push("no-js-browser-static-markers-invalid".to_string());
    }
    if receipt
        .get("semantic_landmark_present")
        .and_then(Value::as_bool)
        != Some(true)
        || receipt.get("visible_text_present").and_then(Value::as_bool) != Some(true)
        || receipt
            .get("link_count")
            .and_then(Value::as_u64)
            .is_none_or(|count| count == 0)
        || receipt
            .get("form_count")
            .and_then(Value::as_u64)
            .is_none_or(|count| count == 0)
        || receipt.get("seo_title_present").and_then(Value::as_bool) != Some(true)
        || receipt
            .get("accessibility_signal_count")
            .and_then(Value::as_u64)
            .is_none_or(|count| count == 0)
    {
        stale_reasons.push("no-js-browser-meaningful-html-incomplete".to_string());
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false)
        || receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false)
    {
        stale_reasons.push("no-js-browser-claim-overstated".to_string());
    }

    stale_reasons
}

fn no_js_browser_artifact_hash_matches(cwd: &Path, receipt: &Value) -> bool {
    let Some(html_path) = receipt.get("html_path").and_then(Value::as_str) else {
        return false;
    };
    let html_relative = Path::new(html_path);
    if html_relative.is_absolute()
        || html_relative
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return false;
    }
    file_blake3_hex(&cwd.join(html_relative))
        .map(|hash| format!("blake3:{hash}"))
        .as_deref()
        == receipt.get("artifact_html_blake3").and_then(Value::as_str)
}

fn same_machine_performance_receipt_status(receipt: Option<&Value>) -> Value {
    let stale_reasons = receipt
        .map(same_machine_performance_receipt_stale_reasons)
        .unwrap_or_else(|| vec!["same-machine-performance-receipt-missing".to_string()]);
    json!({
        "contract": SAME_MACHINE_PERFORMANCE_RECEIPT_SCHEMA,
        "current": stale_reasons.is_empty(),
        "benchmark_receipt_required": true,
        "collection_receipt": SAME_MACHINE_PERFORMANCE_COLLECTION_RECEIPT,
        "serializer_receipt": SAME_MACHINE_PERFORMANCE_RECEIPT_SR,
        "machine_contract": SAME_MACHINE_PERFORMANCE_RECEIPT_MACHINE,
        "stale_reasons": stale_reasons,
        "raceboard": same_machine_performance_raceboard_from_receipt(receipt),
        "replay_command": SAME_MACHINE_PERFORMANCE_REPLAY_COMMAND,
        "raw_replay_command": SAME_MACHINE_PERFORMANCE_RAW_REPLAY_COMMAND,
        "dry_run_command": SAME_MACHINE_PERFORMANCE_DRY_RUN_COMMAND,
        "import_command": SAME_MACHINE_PERFORMANCE_IMPORT_COMMAND,
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "dry_run": receipt
            .and_then(|value| value.get("dry_run"))
            .and_then(Value::as_bool),
        "measurement_executed": receipt
            .and_then(|value| value.get("measurement_executed"))
            .and_then(Value::as_bool),
        "http_requests_executed": receipt
            .and_then(|value| value.get("http_requests_executed"))
            .and_then(Value::as_bool),
        "round_count": receipt
            .and_then(|value| value.get("round_count"))
            .and_then(Value::as_u64),
        "target_summary_count": receipt
            .and_then(|value| value.get("target_summaries"))
            .and_then(Value::as_array)
            .map(Vec::len),
        "faster_than_upstream_claimed": receipt
            .and_then(|value| value.get("faster_than_upstream_claimed"))
            .and_then(Value::as_bool),
        "same_machine_replay_required_for_speed_claim": receipt
            .and_then(|value| value.get("same_machine_replay_required_for_speed_claim"))
            .and_then(Value::as_bool),
    })
}

fn same_machine_performance_raceboard_from_receipt(receipt: Option<&Value>) -> Value {
    let ranking = receipt
        .map(same_machine_performance_ranking)
        .unwrap_or_default();
    let winner = ranking
        .first()
        .and_then(|entry| entry.get("name"))
        .and_then(Value::as_str)
        .map(str::to_string);
    let www_rank = ranking
        .iter()
        .find(|entry| entry.get("name").and_then(Value::as_str) == Some("www"))
        .and_then(|entry| entry.get("rank"))
        .and_then(Value::as_u64);
    let www_median_rps = receipt.and_then(|receipt| same_machine_target_median_rps(receipt, "www"));
    let next_median_rps =
        receipt.and_then(|receipt| same_machine_target_median_rps(receipt, "next"));
    let svelte_median_rps =
        receipt.and_then(|receipt| same_machine_target_median_rps(receipt, "svelte"));
    let astro_median_rps =
        receipt.and_then(|receipt| same_machine_target_median_rps(receipt, "astro"));

    json!({
        "current": receipt
            .map(same_machine_performance_receipt_stale_reasons)
            .is_some_and(|reasons| reasons.is_empty()),
        "ranked_by": "median_requests_per_second",
        "winner": winner,
        "www_rank": www_rank,
        "www_median_rps": www_median_rps,
        "www_vs_next_median_rps_ratio": same_machine_rps_ratio(www_median_rps, next_median_rps),
        "www_vs_svelte_median_rps_ratio": same_machine_rps_ratio(www_median_rps, svelte_median_rps),
        "www_vs_astro_median_rps_ratio": same_machine_rps_ratio(www_median_rps, astro_median_rps),
        "smallest_public_bytes_target": same_machine_smallest_public_bytes_target(&ranking),
        "ranking": ranking,
        "claim_boundary": "This is a local same-machine runtime raceboard only; it is not a global fastest-world claim or Astro tiny-static payload parity proof.",
    })
}

fn same_machine_performance_ranking(receipt: &Value) -> Vec<Value> {
    let mut entries = receipt
        .get("target_summaries")
        .and_then(Value::as_array)
        .into_iter()
        .flat_map(|summaries| summaries.iter())
        .filter_map(|summary| {
            let name = summary.get("name").and_then(Value::as_str)?;
            let median_rps = summary
                .pointer("/requests_per_second/median")
                .and_then(Value::as_f64)?;
            if !median_rps.is_finite() || median_rps <= 0.0 {
                return None;
            }
            Some((
                name.to_string(),
                median_rps,
                summary
                    .pointer("/latency_ms/p95/median")
                    .and_then(Value::as_f64),
                summary.get("bytes_total").and_then(Value::as_u64),
                same_machine_first_response_bytes(receipt, name),
            ))
        })
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| {
        right
            .1
            .partial_cmp(&left.1)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    entries
        .into_iter()
        .enumerate()
        .map(
            |(
                index,
                (name, median_rps, p95_latency_ms, public_bytes_total, first_response_bytes),
            )| {
                json!({
                    "rank": index + 1,
                    "name": name,
                    "median_requests_per_second": median_rps,
                    "median_p95_latency_ms": p95_latency_ms,
                    "first_response_bytes": first_response_bytes,
                    "public_bytes_total": public_bytes_total,
                })
            },
        )
        .collect()
}

fn same_machine_first_response_bytes(receipt: &Value, target: &str) -> Option<u64> {
    receipt
        .get("output_fixtures")
        .and_then(Value::as_array)
        .into_iter()
        .flat_map(|fixtures| fixtures.iter())
        .find(|fixture| fixture.get("name").and_then(Value::as_str) == Some(target))
        .and_then(|fixture| fixture.get("bytes").and_then(Value::as_u64))
}

fn same_machine_target_median_rps(receipt: &Value, target: &str) -> Option<f64> {
    receipt
        .get("target_summaries")
        .and_then(Value::as_array)
        .into_iter()
        .flat_map(|summaries| summaries.iter())
        .find(|summary| summary.get("name").and_then(Value::as_str) == Some(target))
        .and_then(|summary| {
            summary
                .pointer("/requests_per_second/median")
                .and_then(Value::as_f64)
        })
        .filter(|value| value.is_finite() && *value > 0.0)
}

fn same_machine_rps_ratio(numerator: Option<f64>, denominator: Option<f64>) -> Option<f64> {
    let numerator = numerator?;
    let denominator = denominator?;
    (denominator > 0.0).then_some((numerator / denominator * 100.0).round() / 100.0)
}

fn same_machine_smallest_public_bytes_target(ranking: &[Value]) -> Option<String> {
    ranking
        .iter()
        .filter_map(|entry| {
            Some((
                entry.get("name").and_then(Value::as_str)?,
                entry
                    .get("first_response_bytes")
                    .and_then(Value::as_u64)
                    .or_else(|| entry.get("public_bytes_total").and_then(Value::as_u64))?,
            ))
        })
        .min_by_key(|(_, bytes)| *bytes)
        .map(|(name, _)| name.to_string())
}

fn same_machine_performance_receipt_stale_reasons(receipt: &Value) -> Vec<String> {
    let mut stale_reasons = Vec::new();
    if receipt.get("schema").and_then(Value::as_str)
        != Some(SAME_MACHINE_PERFORMANCE_RECEIPT_SCHEMA)
    {
        stale_reasons.push("schema-mismatch-or-missing".to_string());
    }
    if receipt.get("dry_run").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("dry-run-not-false".to_string());
    }
    if receipt.get("measurement_executed").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("measurement-not-executed".to_string());
    }
    if receipt
        .get("http_requests_executed")
        .and_then(Value::as_bool)
        != Some(true)
    {
        stale_reasons.push("http-requests-not-executed".to_string());
    }
    if receipt
        .get("preflight_http_requests_executed")
        .and_then(Value::as_bool)
        != Some(true)
    {
        stale_reasons.push("preflight-http-requests-not-executed".to_string());
    }
    if receipt
        .get("measurement_http_requests_executed")
        .and_then(Value::as_bool)
        != Some(true)
    {
        stale_reasons.push("measurement-http-requests-not-executed".to_string());
    }
    if receipt
        .get("round_count")
        .and_then(Value::as_u64)
        .is_none_or(|round_count| round_count == 0)
    {
        stale_reasons.push("round-count-not-positive".to_string());
    }
    if receipt
        .pointer("/benchmark/script_path")
        .and_then(Value::as_str)
        != Some("benchmarks/dx-runtime-throughput-benchmark.ts")
    {
        stale_reasons.push("script-path-not-canonical-throughput-benchmark".to_string());
    }
    if receipt
        .pointer("/benchmark/script_sha256")
        .and_then(Value::as_str)
        .is_none_or(str::is_empty)
    {
        stale_reasons.push("script-hash-missing".to_string());
    }
    if receipt
        .get("same_machine_replay_required_for_speed_claim")
        .and_then(Value::as_bool)
        != Some(true)
    {
        stale_reasons.push("same-machine-replay-required-not-true".to_string());
    }
    if receipt
        .get("faster_than_upstream_claimed")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("faster-than-upstream-claimed-not-false".to_string());
    }
    if receipt
        .pointer("/no_claims/no_claim_framework_absolute_superiority")
        .and_then(Value::as_bool)
        != Some(true)
    {
        stale_reasons.push("framework-superiority-no-claim-missing".to_string());
    }
    if !same_machine_performance_binary_hash_current(receipt) {
        stale_reasons.push("same-machine-performance-binary-hash-missing".to_string());
    }
    let preflight_failures = same_machine_performance_preflight_failures(receipt);
    if !preflight_failures.is_empty() {
        stale_reasons.push("same-machine-performance-preflight-failed".to_string());
        stale_reasons.extend(
            preflight_failures
                .into_iter()
                .map(|target| format!("preflight-output-invalid-{target}")),
        );
    }
    let target_error_targets = same_machine_performance_target_error_targets(receipt);
    if !target_error_targets.is_empty() {
        stale_reasons.push("same-machine-performance-target-errors".to_string());
        stale_reasons.extend(
            target_error_targets
                .into_iter()
                .map(|target| format!("target-errors-present-{target}")),
        );
    }
    for target in ["www", "next", "svelte", "astro"] {
        if !same_machine_receipt_has_measured_target(receipt, target) {
            stale_reasons.push(format!("missing-measured-target-{target}"));
        }
    }
    stale_reasons
}

fn same_machine_performance_binary_hash_current(receipt: &Value) -> bool {
    receipt
        .pointer("/dx_www_binary/exists")
        .and_then(Value::as_bool)
        == Some(true)
        && receipt
            .pointer("/dx_www_binary/hash_status")
            .and_then(Value::as_str)
            == Some("captured")
        && json_non_empty_string(receipt.pointer("/dx_www_binary/sha256"))
}

fn same_machine_performance_preflight_failures(receipt: &Value) -> Vec<&'static str> {
    ["www", "next", "svelte", "astro"]
        .into_iter()
        .filter(|target| {
            !receipt
                .get("output_fixtures")
                .and_then(Value::as_array)
                .into_iter()
                .flat_map(|fixtures| fixtures.iter())
                .any(|fixture| {
                    fixture.get("name").and_then(Value::as_str) == Some(*target)
                        && fixture.get("ok").and_then(Value::as_bool) == Some(true)
                        && fixture
                            .get("status")
                            .and_then(Value::as_u64)
                            .is_some_and(|status| (200..400).contains(&status))
                        && json_non_empty_string(fixture.get("sha256"))
                })
        })
        .collect()
}

fn same_machine_performance_target_error_targets(receipt: &Value) -> Vec<&'static str> {
    ["www", "next", "svelte", "astro"]
        .into_iter()
        .filter(|target| {
            !receipt
                .get("target_summaries")
                .and_then(Value::as_array)
                .into_iter()
                .flat_map(|summaries| summaries.iter())
                .any(|summary| {
                    let round_count = summary
                        .get("round_count")
                        .and_then(Value::as_u64)
                        .unwrap_or(0);
                    summary.get("name").and_then(Value::as_str) == Some(*target)
                        && round_count > 0
                        && summary
                            .get("successful_round_count")
                            .and_then(Value::as_u64)
                            == Some(round_count)
                        && summary
                            .get("errors_total")
                            .or_else(|| summary.get("errors"))
                            .and_then(Value::as_u64)
                            == Some(0)
                })
        })
        .collect()
}

fn same_machine_receipt_has_measured_target(receipt: &Value, target: &str) -> bool {
    receipt
        .get("target_summaries")
        .and_then(Value::as_array)
        .into_iter()
        .flat_map(|summaries| summaries.iter())
        .any(|summary| {
            summary.get("name").and_then(Value::as_str) == Some(target)
                && summary
                    .get("round_count")
                    .and_then(Value::as_u64)
                    .is_some_and(|round_count| round_count > 0)
                && summary
                    .pointer("/requests_per_second/median")
                    .and_then(Value::as_f64)
                    .is_some_and(|value| value.is_finite() && value > 0.0)
        })
}

fn json_non_empty_string(value: Option<&Value>) -> bool {
    value
        .and_then(Value::as_str)
        .is_some_and(|text| !text.trim().is_empty())
}

fn json_string_array_contains(value: Option<&Value>, expected: &str) -> bool {
    value
        .and_then(Value::as_array)
        .is_some_and(|items| items.iter().any(|item| item.as_str() == Some(expected)))
}

fn production_http_receipt_status(receipt: Option<&Value>) -> Value {
    let stale_reasons = receipt
        .map(production_http_receipt_stale_reasons)
        .unwrap_or_else(|| vec!["production-http-local-replay-receipt-missing".to_string()]);
    json!({
        "contract": PRODUCTION_HTTP_RECEIPT_SCHEMA,
        "current": stale_reasons.is_empty(),
        "local_replay_receipt_required": true,
        "hosted_provider_proof_required": true,
        "stale_reasons": stale_reasons,
        "replay_command": PRODUCTION_HTTP_REPLAY_COMMAND,
        "contract_test_command": PRODUCTION_HTTP_CONTRACT_TEST_COMMAND,
        "serializer_receipt": PRODUCTION_HTTP_RECEIPT_SR,
        "machine_contract": PRODUCTION_HTTP_RECEIPT_MACHINE,
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "passed": receipt
            .and_then(|value| value.get("passed"))
            .and_then(Value::as_bool),
        "status": receipt
            .and_then(|value| value.get("status"))
            .and_then(Value::as_str),
        "proof_scope": receipt
            .and_then(|value| value.get("proof_scope"))
            .and_then(Value::as_str),
        "wire_responder": receipt
            .and_then(|value| value.get("wire_responder"))
            .and_then(Value::as_str),
        "tcp_preview_server_started": receipt
            .and_then(|value| value.get("tcp_preview_server_started"))
            .and_then(Value::as_bool),
        "browser_runtime_executed": receipt
            .and_then(|value| value.get("browser_runtime_executed"))
            .and_then(Value::as_bool),
        "hosted_provider_proof": receipt
            .and_then(|value| value.get("hosted_provider_proof"))
            .and_then(Value::as_bool),
        "provider_bound_cdn_executed": receipt
            .and_then(|value| value.get("provider_bound_cdn_executed"))
            .and_then(Value::as_bool),
        "axum_responder_source_parity": readiness::readiness_production_http_axum_source_parity(),
        "remaining_external_proof_gap_ids": readiness::READINESS_PRODUCTION_HTTP_EXTERNAL_PROOF_GAP_IDS,
        "external_proof_gap_ids": receipt
            .and_then(|value| value.get("external_proof_gap_ids"))
            .cloned()
            .unwrap_or(Value::Null),
        "missing_check_ids": receipt
            .map(production_http_missing_check_ids)
            .unwrap_or_else(|| {
                production_http_expected_check_ids()
                    .iter()
                    .map(|check| check.to_string())
                    .collect::<Vec<_>>()
            }),
    })
}

fn production_http_tcp_preview_receipt_status(receipt: Option<&Value>) -> Value {
    let stale_reasons = receipt
        .map(production_http_tcp_preview_receipt_stale_reasons)
        .unwrap_or_else(|| vec!["production-http-tcp-preview-receipt-missing".to_string()]);
    json!({
        "contract": PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_SCHEMA,
        "current": stale_reasons.is_empty(),
        "tcp_preview_receipt_required": true,
        "hosted_provider_proof_required": true,
        "stale_reasons": stale_reasons,
        "collect_command": readiness::READINESS_PRODUCTION_HTTP_TCP_PREVIEW_COLLECT_COMMAND,
        "import_command": "dx www readiness --import-production-http-tcp-preview-receipt <tcp-receipt.json> --json --full",
        "contract_test_command": "node --test benchmarks/dx-www-production-preview-tcp-receipt.test.ts",
        "serializer_receipt": PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_SR,
        "machine_contract": PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_MACHINE,
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "passed": receipt
            .and_then(|value| value.get("passed"))
            .and_then(Value::as_bool),
        "status": receipt
            .and_then(|value| value.get("status"))
            .and_then(Value::as_str),
        "proof_scope": receipt
            .and_then(|value| value.get("proof_scope"))
            .and_then(Value::as_str),
        "tcp_preview_server_started": receipt
            .and_then(|value| value.get("tcp_preview_server_started"))
            .and_then(Value::as_bool),
        "tcp_requests_executed": receipt
            .and_then(|value| value.get("tcp_requests_executed"))
            .and_then(Value::as_bool),
        "browser_runtime_executed": receipt
            .and_then(|value| value.get("browser_runtime_executed"))
            .and_then(Value::as_bool),
        "axum_responder_source_parity": readiness::readiness_production_http_axum_source_parity(),
        "remaining_external_proof_gap_ids": receipt
            .and_then(|value| value.get("remaining_external_proof_gap_ids"))
            .cloned()
            .unwrap_or(Value::Null),
        "cleared_external_proof_gap_ids": receipt
            .and_then(|value| value.get("cleared_external_proof_gap_ids"))
            .cloned()
            .unwrap_or(Value::Null),
        "missing_check_ids": receipt
            .map(production_http_missing_check_ids)
            .unwrap_or_else(|| {
                production_http_expected_check_ids()
                    .iter()
                    .map(|check| check.to_string())
                    .collect::<Vec<_>>()
            }),
    })
}

fn production_http_tcp_preview_receipt_stale_reasons(receipt: &Value) -> Vec<String> {
    let mut stale_reasons = Vec::new();
    if receipt.get("schema").and_then(Value::as_str)
        != Some(PRODUCTION_HTTP_TCP_PREVIEW_RECEIPT_SCHEMA)
    {
        stale_reasons.push("schema-mismatch-or-missing".to_string());
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("passed-not-true".to_string());
    }
    if receipt.get("status").and_then(Value::as_str)
        != Some("local-production-http-tcp-preview-current")
    {
        stale_reasons.push("status-not-current".to_string());
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("release-ready-overclaimed".to_string());
    }
    if receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("fastest-world-claim-overclaimed".to_string());
    }
    if receipt.get("proof_scope").and_then(Value::as_str)
        != Some("local-production-preview-tcp-server")
    {
        stale_reasons.push("proof-scope-not-tcp-preview".to_string());
    }
    if receipt
        .get("tcp_preview_server_started")
        .and_then(Value::as_bool)
        != Some(true)
    {
        stale_reasons.push("tcp-preview-server-not-started".to_string());
    }
    if receipt
        .get("tcp_requests_executed")
        .and_then(Value::as_bool)
        != Some(true)
    {
        stale_reasons.push("tcp-requests-not-executed".to_string());
    }
    if receipt
        .get("browser_runtime_executed")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("browser-runtime-overclaimed".to_string());
    }
    if receipt
        .get("hosted_provider_proof")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("hosted-provider-overclaimed".to_string());
    }
    if receipt
        .get("provider_bound_cdn_executed")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("provider-cdn-overclaimed".to_string());
    }
    if !json_string_array_contains(
        receipt.get("cleared_external_proof_gap_ids"),
        "preview-tcp-server-parity",
    ) {
        stale_reasons.push("preview-tcp-server-parity-not-cleared".to_string());
    }
    if json_string_array_contains(
        receipt.get("remaining_external_proof_gap_ids"),
        "preview-tcp-server-parity",
    ) {
        stale_reasons.push("preview-tcp-server-parity-still-remaining".to_string());
    }
    for check_id in production_http_missing_check_ids(receipt) {
        stale_reasons.push(format!("missing-or-failing-check-{check_id}"));
    }
    stale_reasons
}

fn production_http_receipt_stale_reasons(receipt: &Value) -> Vec<String> {
    let mut stale_reasons = Vec::new();
    if receipt.get("schema").and_then(Value::as_str) != Some(PRODUCTION_HTTP_RECEIPT_SCHEMA) {
        stale_reasons.push("schema-mismatch-or-missing".to_string());
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("passed-not-true".to_string());
    }
    if receipt.get("status").and_then(Value::as_str)
        != Some("local-production-http-wire-replay-current")
    {
        stale_reasons.push("status-not-current".to_string());
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("release-ready-overclaimed".to_string());
    }
    if receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("fastest-world-claim-overclaimed".to_string());
    }
    if receipt.get("proof_scope").and_then(Value::as_str)
        != Some("local-production-contract-wire-replay")
    {
        stale_reasons.push("proof-scope-not-local-wire-replay".to_string());
    }
    if receipt.get("wire_responder").and_then(Value::as_str)
        != Some("production_contract_wire_response")
    {
        stale_reasons.push("wire-responder-not-canonical".to_string());
    }
    for field in [
        "tcp_preview_server_started",
        "browser_runtime_executed",
        "hosted_provider_proof",
        "provider_bound_cdn_executed",
    ] {
        if receipt.get(field).and_then(Value::as_bool) != Some(false) {
            stale_reasons.push(format!("{field}-must-be-false-for-local-replay"));
        }
    }
    for gap_id in readiness::READINESS_PRODUCTION_HTTP_EXTERNAL_PROOF_GAP_IDS {
        if !receipt
            .get("external_proof_gap_ids")
            .is_some_and(|value| receipt_string_array_contains(value, gap_id))
        {
            stale_reasons.push(format!("missing-external-proof-gap-{gap_id}"));
        }
    }
    for check_id in production_http_missing_check_ids(receipt) {
        stale_reasons.push(format!("missing-or-failing-check-{check_id}"));
    }
    stale_reasons
}

fn production_http_missing_check_ids(receipt: &Value) -> Vec<String> {
    production_http_expected_check_ids()
        .iter()
        .copied()
        .filter(|check_id| !production_http_check_passed(receipt, check_id))
        .map(str::to_string)
        .collect()
}

fn production_http_check_passed(receipt: &Value, expected_id: &str) -> bool {
    receipt
        .get("checks")
        .and_then(Value::as_array)
        .is_some_and(|checks| {
            checks.iter().any(|check| {
                check.get("id").and_then(Value::as_str) == Some(expected_id)
                    && check.get("passed").and_then(Value::as_bool) == Some(true)
            })
        })
}

fn production_http_expected_check_ids() -> &'static [&'static str] {
    &[
        "etag-present",
        "if-none-match-304",
        "if-modified-since-304",
        "head-omits-body",
        "range-206",
        "range-416",
        "if-range-206",
        "stale-if-range-falls-back-to-full-body",
        "br-negotiation",
        "gzip-negotiation",
        "plain-asset-vary",
        "static-options-204-allow-header",
        "static-post-405-allow-header",
        "precompressed-decoded-content-type",
    ]
}

fn bundle_partition_receipt_status(receipt: Option<&Value>) -> Value {
    let stale_reasons = receipt
        .map(bundle_partition_receipt_stale_reasons)
        .unwrap_or_else(|| vec!["bundle-partition-receipt-missing".to_string()]);
    json!({
        "contract": BUNDLE_PARTITION_RECEIPT_SCHEMA,
        "current": stale_reasons.is_empty(),
        "local_partition_receipt_required": true,
        "hosted_provider_proof_required": true,
        "stale_reasons": stale_reasons,
        "replay_command": READINESS_WRITE_RECEIPTS_COMMAND,
        "contract_test_command": BUNDLE_PARTITION_CONTRACT_TEST_COMMAND,
        "serializer_receipt": BUNDLE_PARTITION_RECEIPT_SR,
        "machine_contract": BUNDLE_PARTITION_RECEIPT_MACHINE,
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "passed": receipt
            .and_then(|value| value.get("passed"))
            .and_then(Value::as_bool),
        "status": receipt
            .and_then(|value| value.get("status"))
            .and_then(Value::as_str),
        "public_runtime_artifact_count": receipt
            .and_then(|value| value.get("public_runtime_artifact_count"))
            .and_then(Value::as_u64),
        "evidence_artifact_count": receipt
            .and_then(|value| value.get("evidence_artifact_count"))
            .and_then(Value::as_u64),
        "public_runtime_evidence_path_count": receipt
            .and_then(|value| value.get("public_runtime_evidence_path_count"))
            .and_then(Value::as_u64),
        "evidence_artifacts_no_store": receipt
            .and_then(|value| value.get("evidence_artifacts_no_store"))
            .and_then(Value::as_bool),
        "evidence_bundle_deployable_public_bytes": receipt
            .and_then(|value| value.get("evidence_bundle_deployable_public_bytes"))
            .and_then(Value::as_bool),
        "stale_fields": receipt
            .map(bundle_partition_stale_fields)
            .unwrap_or_else(|| vec![
                "public_runtime_artifact_count".to_string(),
                "evidence_artifact_count".to_string(),
                "deploy_partition_present".to_string(),
                "provider_adapter_present".to_string(),
            ]),
    })
}

fn bundle_partition_receipt_stale_reasons(receipt: &Value) -> Vec<String> {
    let mut stale_reasons = Vec::new();
    if receipt.get("schema").and_then(Value::as_str) != Some(BUNDLE_PARTITION_RECEIPT_SCHEMA) {
        stale_reasons.push("schema-mismatch-or-missing".to_string());
    }
    if receipt.get("id").and_then(Value::as_str) != Some("bundle-partition") {
        stale_reasons.push("id-not-bundle-partition".to_string());
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("passed-not-true".to_string());
    }
    if receipt.get("status").and_then(Value::as_str)
        != Some("local-public-evidence-partition-current")
    {
        stale_reasons.push("status-not-current".to_string());
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("release-ready-overclaimed".to_string());
    }
    if receipt
        .get("hosted_provider_proof")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("hosted-provider-proof-overclaimed".to_string());
    }
    for field in bundle_partition_stale_fields(receipt) {
        stale_reasons.push(format!("stale-partition-field-{field}"));
    }
    stale_reasons
}

fn bundle_provider_replay_receipt_status(receipt: Option<&Value>) -> Value {
    let stale_reasons = receipt
        .map(bundle_provider_replay_receipt_stale_reasons)
        .unwrap_or_else(|| vec!["bundle-provider-replay-receipt-missing".to_string()]);
    json!({
        "contract": BUNDLE_PROVIDER_REPLAY_RECEIPT_SCHEMA,
        "current": stale_reasons.is_empty(),
        "hosted_provider_proof_required": true,
        "stale_reasons": stale_reasons,
        "collect_command": readiness::READINESS_BUNDLE_PROVIDER_REPLAY_COLLECT_COMMAND,
        "import_command": "dx www readiness --import-bundle-provider-replay-receipt <bundle-provider-replay-receipt.json> --json --full",
        "serializer_receipt": BUNDLE_PROVIDER_REPLAY_RECEIPT_SR,
        "machine_contract": BUNDLE_PROVIDER_REPLAY_RECEIPT_MACHINE,
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "passed": receipt
            .and_then(|value| value.get("passed"))
            .and_then(Value::as_bool),
        "status": receipt
            .and_then(|value| value.get("status"))
            .and_then(Value::as_str),
        "hosted_provider_proof": receipt
            .and_then(|value| value.get("hosted_provider_proof"))
            .and_then(Value::as_bool),
        "local_base_url": receipt
            .and_then(|value| value.get("local_base_url"))
            .and_then(Value::as_bool),
        "public_runtime_artifact_count": receipt
            .and_then(|value| value.get("public_runtime_artifact_count"))
            .and_then(Value::as_u64),
        "evidence_artifact_count": receipt
            .and_then(|value| value.get("evidence_artifact_count"))
            .and_then(Value::as_u64),
        "public_failure_count": receipt
            .and_then(|value| value.get("public_failure_count"))
            .and_then(Value::as_u64),
        "evidence_public_leak_count": receipt
            .and_then(|value| value.get("evidence_public_leak_count"))
            .and_then(Value::as_u64),
    })
}

fn bundle_provider_replay_receipt_stale_reasons(receipt: &Value) -> Vec<String> {
    let mut stale_reasons = Vec::new();
    if receipt.get("schema").and_then(Value::as_str) != Some(BUNDLE_PROVIDER_REPLAY_RECEIPT_SCHEMA)
    {
        stale_reasons.push("schema-mismatch-or-missing".to_string());
    }
    if receipt.get("id").and_then(Value::as_str) != Some("bundle-provider-replay") {
        stale_reasons.push("id-not-bundle-provider-replay".to_string());
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("passed-not-true".to_string());
    }
    if receipt.get("status").and_then(Value::as_str)
        != Some("hosted-public-evidence-bundle-replay-current")
    {
        stale_reasons.push("status-not-current-hosted-provider".to_string());
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("release-ready-overclaimed".to_string());
    }
    if receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("fastest-world-overclaimed".to_string());
    }
    if receipt
        .get("hosted_provider_proof")
        .and_then(Value::as_bool)
        != Some(true)
    {
        stale_reasons.push("hosted-provider-proof-missing".to_string());
    }
    if receipt.get("local_base_url").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("local-base-url-not-hosted-proof".to_string());
    }
    if receipt
        .get("public_runtime_artifact_count")
        .and_then(Value::as_u64)
        .is_none_or(|count| count == 0)
    {
        stale_reasons.push("public-runtime-artifact-count-missing".to_string());
    }
    if receipt
        .get("evidence_artifact_count")
        .and_then(Value::as_u64)
        .is_none_or(|count| count == 0)
    {
        stale_reasons.push("evidence-artifact-count-missing".to_string());
    }
    if receipt.get("public_failure_count").and_then(Value::as_u64) != Some(0) {
        stale_reasons.push("public-runtime-artifact-not-public".to_string());
    }
    if receipt
        .get("evidence_public_leak_count")
        .and_then(Value::as_u64)
        != Some(0)
    {
        stale_reasons.push("evidence-artifact-public-leak".to_string());
    }
    stale_reasons
}

fn bundle_partition_stale_fields(receipt: &Value) -> Vec<String> {
    let mut fields = Vec::new();
    if receipt
        .get("public_runtime_artifact_count")
        .and_then(Value::as_u64)
        .is_none_or(|count| count == 0)
    {
        fields.push("public_runtime_artifact_count".to_string());
    }
    if receipt
        .get("evidence_artifact_count")
        .and_then(Value::as_u64)
        .is_none_or(|count| count == 0)
    {
        fields.push("evidence_artifact_count".to_string());
    }
    if receipt
        .get("public_runtime_evidence_path_count")
        .and_then(Value::as_u64)
        != Some(0)
    {
        fields.push("public_runtime_evidence_path_count".to_string());
    }
    for (field, expected) in [
        ("evidence_artifacts_no_store", true),
        ("public_runtime_deployable", true),
        ("deploy_partition_present", true),
        ("provider_adapter_present", true),
    ] {
        if receipt.get(field).and_then(Value::as_bool) != Some(expected) {
            fields.push(field.to_string());
        }
    }
    if receipt
        .get("evidence_bundle_deployable_public_bytes")
        .and_then(Value::as_bool)
        != Some(false)
    {
        fields.push("evidence_bundle_deployable_public_bytes".to_string());
    }
    fields
}

fn server_action_replay_ledger_receipt_status(receipt: Option<&Value>) -> Value {
    let stale_reasons = receipt
        .map(server_action_replay_ledger_receipt_stale_reasons)
        .unwrap_or_else(|| vec!["server-action-replay-ledger-receipt-missing".to_string()]);
    json!({
        "contract": SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SCHEMA,
        "current": stale_reasons.is_empty(),
        "local_replay_receipt_required": true,
        "hosted_provider_proof_required": true,
        "stale_reasons": stale_reasons,
        "replay_command": READINESS_WRITE_RECEIPTS_COMMAND,
        "contract_test_command": SERVER_ACTION_REPLAY_LEDGER_CONTRACT_TEST_COMMAND,
        "serializer_receipt": SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SR,
        "machine_contract": SERVER_ACTION_REPLAY_LEDGER_RECEIPT_MACHINE,
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "passed": receipt
            .and_then(|value| value.get("passed"))
            .and_then(Value::as_bool),
        "status": receipt
            .and_then(|value| value.get("status"))
            .and_then(Value::as_str),
        "ledger_path": receipt
            .and_then(|value| value.get("ledger_path"))
            .and_then(Value::as_str),
        "ledger_present": receipt
            .and_then(|value| value.get("ledger_present"))
            .and_then(Value::as_bool),
        "distributed": receipt
            .and_then(|value| value.get("distributed"))
            .and_then(Value::as_bool),
        "provider_hosted": receipt
            .and_then(|value| value.get("provider_hosted"))
            .and_then(Value::as_bool),
        "hosted_provider_proof": receipt
            .and_then(|value| value.get("hosted_provider_proof"))
            .and_then(Value::as_bool),
        "provider_proof_status": receipt
            .and_then(|value| value.get("provider_proof_status"))
            .and_then(Value::as_str),
        "production_proof_scope": receipt
            .and_then(|value| value.get("production_proof_scope"))
            .and_then(Value::as_str),
        "provider_hosted_replay_required": receipt
            .and_then(|value| value.get("provider_hosted_replay_required"))
            .and_then(Value::as_bool),
        "remaining_provider_gap_ids": readiness::READINESS_SERVER_ACTION_PROVIDER_GAP_IDS,
        "provider_proof_gap_ids": receipt
            .and_then(|value| value.get("provider_proof_gap_ids"))
            .cloned()
            .unwrap_or(Value::Null),
    })
}

fn server_action_replay_ledger_receipt_stale_reasons(receipt: &Value) -> Vec<String> {
    let mut stale_reasons = Vec::new();
    if receipt.get("schema").and_then(Value::as_str)
        != Some(SERVER_ACTION_REPLAY_LEDGER_RECEIPT_SCHEMA)
    {
        stale_reasons.push("schema-mismatch-or-missing".to_string());
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("passed-not-true".to_string());
    }
    if receipt.get("status").and_then(Value::as_str)
        != Some("local-replay-ledger-current-provider-proof-needed")
    {
        stale_reasons.push("status-not-current".to_string());
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("release-ready-overclaimed".to_string());
    }
    if receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("fastest-world-claim-overclaimed".to_string());
    }
    if receipt.get("distributed").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("distributed-must-be-false-for-local-ledger".to_string());
    }
    if receipt.get("provider_hosted").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("provider-hosted-must-be-false-for-local-ledger".to_string());
    }
    if receipt
        .get("hosted_provider_proof")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("hosted-provider-proof-must-be-false-for-local-ledger".to_string());
    }
    if receipt.get("provider_proof_status").and_then(Value::as_str)
        != Some("not-run-local-preview-only")
    {
        stale_reasons.push("provider-proof-status-not-local-preview-only".to_string());
    }
    if receipt
        .get("production_proof_scope")
        .and_then(Value::as_str)
        != Some("local-production-preview-only")
    {
        stale_reasons.push("production-proof-scope-not-local-preview-only".to_string());
    }
    if receipt
        .get("provider_hosted_replay_required")
        .and_then(Value::as_bool)
        != Some(true)
    {
        stale_reasons.push("provider-hosted-replay-required-not-true".to_string());
    }
    for gap_id in readiness::READINESS_SERVER_ACTION_PROVIDER_GAP_IDS {
        if !receipt
            .get("provider_proof_gap_ids")
            .is_some_and(|value| receipt_string_array_contains(value, gap_id))
        {
            stale_reasons.push(format!("missing-provider-proof-gap-{gap_id}"));
        }
    }
    stale_reasons
}

fn primitive_proof_receipt_status(receipt: Option<&Value>) -> Value {
    let stale_reasons = receipt
        .map(primitive_proof_receipt_stale_reasons)
        .unwrap_or_else(|| vec!["primitive-proof-receipt-missing".to_string()]);
    json!({
        "contract": PRIMITIVE_PROOF_RECEIPT_SCHEMA,
        "current": stale_reasons.is_empty(),
        "source_owned_receipt_required": true,
        "hosted_browser_proof_required": true,
        "stale_reasons": stale_reasons,
        "replay_command": READINESS_WRITE_RECEIPTS_COMMAND,
        "contract_test_command": PRIMITIVE_PROOF_CONTRACT_TEST_COMMAND,
        "serializer_receipt": PRIMITIVE_PROOF_RECEIPT_SR,
        "machine_contract": PRIMITIVE_PROOF_RECEIPT_MACHINE,
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "passed": receipt
            .and_then(|value| value.get("passed"))
            .and_then(Value::as_bool),
        "status": receipt
            .and_then(|value| value.get("status"))
            .and_then(Value::as_str),
        "primitive_count": receipt
            .and_then(|value| value.get("primitive_count"))
            .and_then(Value::as_u64),
        "primitive_current_count": receipt
            .and_then(|value| value.get("primitive_current_count"))
            .and_then(Value::as_u64),
        "browser_runtime_executed": receipt
            .and_then(|value| value.get("browser_runtime_executed"))
            .and_then(Value::as_bool),
        "hosted_provider_proof": receipt
            .and_then(|value| value.get("hosted_provider_proof"))
            .and_then(Value::as_bool),
        "live_browser_executed": receipt
            .and_then(|value| value.get("live_browser_executed"))
            .and_then(Value::as_bool),
        "proof_scope": receipt
            .and_then(|value| value.get("proof_scope"))
            .and_then(Value::as_str),
        "missing_primitives": receipt
            .map(primitive_proof_missing_primitives)
            .unwrap_or_else(|| primitive_proof_expected_ids().iter().map(|id| id.to_string()).collect::<Vec<_>>()),
    })
}

fn primitive_proof_receipt_stale_reasons(receipt: &Value) -> Vec<String> {
    let mut stale_reasons = Vec::new();
    if receipt.get("schema").and_then(Value::as_str) != Some(PRIMITIVE_PROOF_RECEIPT_SCHEMA) {
        stale_reasons.push("schema-mismatch-or-missing".to_string());
    }
    if receipt.get("id").and_then(Value::as_str) != Some("primitive-proof") {
        stale_reasons.push("id-not-primitive-proof".to_string());
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("passed-not-true".to_string());
    }
    if receipt.get("status").and_then(Value::as_str)
        != Some("source-owned-primitive-foundation-current")
    {
        stale_reasons.push("status-not-current".to_string());
    }
    if receipt.get("source_owned").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("source-owned-not-true".to_string());
    }
    if receipt.get("primitive_count").and_then(Value::as_u64) != Some(4) {
        stale_reasons.push("primitive-count-not-four".to_string());
    }
    if receipt
        .get("primitive_current_count")
        .and_then(Value::as_u64)
        != Some(4)
    {
        stale_reasons.push("primitive-current-count-not-four".to_string());
    }
    if receipt
        .get("browser_runtime_executed")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("browser-runtime-executed-overclaimed".to_string());
    }
    if receipt
        .get("hosted_provider_proof")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("hosted-provider-proof-overclaimed".to_string());
    }
    if receipt
        .get("live_browser_executed")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("live-browser-executed-overclaimed".to_string());
    }
    if receipt.get("proof_scope").and_then(Value::as_str)
        != Some("local-source-owned-primitive-foundation")
    {
        stale_reasons.push("proof-scope-not-source-owned-foundation".to_string());
    }
    for primitive in primitive_proof_missing_primitives(receipt) {
        stale_reasons.push(format!("missing-or-stale-primitive-{primitive}"));
    }
    stale_reasons
}

fn primitive_proof_missing_primitives(receipt: &Value) -> Vec<String> {
    primitive_proof_expected_ids()
        .iter()
        .copied()
        .filter(|id| !primitive_proof_has_current_primitive(receipt, id))
        .map(str::to_string)
        .collect()
}

fn primitive_proof_has_current_primitive(receipt: &Value, expected_id: &str) -> bool {
    receipt
        .get("primitives")
        .and_then(Value::as_array)
        .is_some_and(|primitives| {
            primitives.iter().any(|primitive| {
                primitive.get("id").and_then(Value::as_str) == Some(expected_id)
                    && primitive.get("passed").and_then(Value::as_bool) == Some(true)
                    && primitive.get("source_owned").and_then(Value::as_bool) == Some(true)
            })
        })
}

fn primitive_proof_expected_ids() -> &'static [&'static str] {
    &["image", "font", "script", "wasm"]
}

fn native_event_browser_binder_receipt_status(receipt: Option<&Value>) -> Value {
    let stale_reasons = receipt
        .map(native_event_browser_binder_receipt_stale_reasons)
        .unwrap_or_else(|| vec!["native-event-browser-binder-receipt-missing".to_string()]);
    json!({
        "contract": NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SCHEMA,
        "current": stale_reasons.is_empty(),
        "browser_receipt_gate": browser_receipt_action_metadata(
            "native-event-browser-binder",
            NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
            NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR,
            NATIVE_EVENT_BROWSER_BINDER_RECEIPT_MACHINE,
            NATIVE_EVENT_BROWSER_BINDER_IMPORT_COMMAND,
            "native-event-browser-binder-receipt-missing",
        ),
        "browser_receipt_required": true,
        "stale_reasons": stale_reasons,
        "replay_command": READINESS_INSPECT_COMMAND,
        "write_receipts_command": READINESS_WRITE_RECEIPTS_COMMAND,
        "import_command": NATIVE_EVENT_BROWSER_BINDER_IMPORT_COMMAND,
        "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
        "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
        "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
        "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "passed": receipt
            .and_then(|value| value.get("passed"))
            .and_then(Value::as_bool),
        "status": receipt
            .map(native_event_browser_binder_status_from_receipt)
            .unwrap_or("missing-browser-binder-receipt"),
        "browser_runtime_executed": receipt
            .and_then(|value| value.get("browser_runtime_executed"))
            .and_then(Value::as_bool),
        "binder_global_present": receipt
            .and_then(|value| value.get("binder_global_present"))
            .and_then(Value::as_bool),
        "supported_event_count": receipt
            .and_then(|value| value.get("supported_event_count"))
            .and_then(Value::as_u64),
        "expected_supported_event_count": readiness::native_dom_event_names().len(),
        "catalog_hash": receipt
            .and_then(|value| value.get("catalog_hash"))
            .and_then(Value::as_str),
        "expected_catalog_hash": readiness::native_dom_event_catalog_integrity().catalog_hash,
        "listener_events": receipt
            .and_then(|value| value.get("listener_events"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default(),
        "unsupported_listener_attached": receipt
            .and_then(|value| value.get("unsupported_listener_attached"))
            .and_then(Value::as_bool),
        "preview_event_count": receipt
            .and_then(|value| value.get("preview_event_count"))
            .and_then(Value::as_u64),
        "state_dispatch_count": receipt
            .and_then(|value| value.get("state_dispatch_count"))
            .and_then(Value::as_u64),
        "proof_scope": receipt
            .and_then(|value| value.get("proof_scope"))
            .and_then(Value::as_str),
    })
}

fn native_event_browser_binder_status_from_receipt(receipt: &Value) -> &'static str {
    if native_event_browser_binder_receipt_is_current(receipt) {
        "browser-binder-replay-current"
    } else {
        "browser-binder-replay-stale"
    }
}

fn native_event_browser_binder_receipt_is_current(receipt: &Value) -> bool {
    native_event_browser_binder_receipt_stale_reasons(receipt).is_empty()
}

fn native_event_browser_binder_receipt_stale_reasons(receipt: &Value) -> Vec<String> {
    let expected_integrity = readiness::native_dom_event_catalog_integrity();
    let listener_events = receipt.get("listener_events").unwrap_or(&Value::Null);
    let mut reasons = Vec::new();

    if receipt.get("schema").and_then(Value::as_str)
        != Some(NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SCHEMA)
    {
        reasons.push("schema-mismatch".to_string());
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true) {
        reasons.push("passed-not-true".to_string());
    }
    if receipt.get("supported_event_count").and_then(Value::as_u64)
        != Some(readiness::native_dom_event_names().len() as u64)
    {
        reasons.push("supported-event-count-mismatch".to_string());
    }
    if receipt.get("catalog_hash").and_then(Value::as_str)
        != Some(expected_integrity.catalog_hash.as_str())
    {
        reasons.push("catalog-hash-mismatch".to_string());
    }
    if receipt
        .get("browser_runtime_executed")
        .and_then(Value::as_bool)
        != Some(true)
    {
        reasons.push("browser-runtime-not-executed".to_string());
    }
    if receipt
        .get("binder_global_present")
        .and_then(Value::as_bool)
        != Some(true)
    {
        reasons.push("binder-global-missing".to_string());
    }
    if receipt
        .get("unsupported_listener_attached")
        .and_then(Value::as_bool)
        != Some(false)
    {
        reasons.push("unsupported-listener-attached".to_string());
    }
    if receipt
        .get("preview_event_count")
        .and_then(Value::as_u64)
        .is_none_or(|count| count < 3)
    {
        reasons.push("preview-event-count-too-low".to_string());
    }
    if receipt
        .get("state_dispatch_count")
        .and_then(Value::as_u64)
        .is_none_or(|count| count < 3)
    {
        reasons.push("state-dispatch-count-too-low".to_string());
    }
    for event_name in ["click", "pointermove", "input"] {
        if !receipt_string_array_contains(listener_events, event_name) {
            reasons.push(format!("listener-event-missing-{event_name}"));
        }
    }
    if receipt.get("proof_scope").and_then(Value::as_str)
        != Some("local-in-app-browser-native-event-binder-replay")
    {
        reasons.push("proof-scope-not-local-browser-native-event-binder-replay".to_string());
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false) {
        reasons.push("release-ready-claim-not-false".to_string());
    }
    if receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false) {
        reasons.push("global-speed-claim-not-false".to_string());
    }

    reasons
}

fn state_runtime_browser_receipt_status(receipt: Option<&Value>) -> Value {
    let stale_reasons = receipt
        .map(state_runtime_browser_receipt_stale_reasons)
        .unwrap_or_else(|| vec!["state-runtime-browser-receipt-missing".to_string()]);
    json!({
        "contract": STATE_RUNTIME_BROWSER_RECEIPT_SCHEMA,
        "current": stale_reasons.is_empty(),
        "browser_receipt_gate": browser_receipt_action_metadata(
            "state-runtime-browser",
            STATE_RUNTIME_BROWSER_RECEIPT,
            STATE_RUNTIME_BROWSER_RECEIPT_SR,
            STATE_RUNTIME_BROWSER_RECEIPT_MACHINE,
            STATE_RUNTIME_BROWSER_IMPORT_COMMAND,
            "state-runtime-browser-receipt-missing",
        ),
        "canonical_browser_proof_target": browser_receipt_proof_target("state-runtime-browser"),
        "browser_receipt_required": true,
        "replay_command": READINESS_INSPECT_COMMAND,
        "write_receipts_command": READINESS_WRITE_RECEIPTS_COMMAND,
        "import_command": STATE_RUNTIME_BROWSER_IMPORT_COMMAND,
        "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
        "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
        "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
        "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "passed": receipt
            .and_then(|value| value.get("passed"))
            .and_then(Value::as_bool),
        "status": receipt
            .map(state_runtime_browser_status_from_receipt)
            .unwrap_or("missing-state-runtime-browser-receipt"),
        "receipt_freshness": receipt
            .and_then(|value| value.get("receipt_freshness"))
            .and_then(Value::as_str),
        "stale_reasons": stale_reasons,
        "browser_runtime_executed": receipt
            .and_then(|value| value.get("browser_runtime_executed"))
            .and_then(Value::as_bool),
        "runtime_global_present": receipt
            .and_then(|value| value.get("runtime_global_present"))
            .and_then(Value::as_bool),
        "state_reflection_event_count": receipt
            .and_then(|value| value.get("state_reflection_event_count"))
            .and_then(Value::as_u64),
        "derived_reflection_event_count": receipt
            .and_then(|value| value.get("derived_reflection_event_count"))
            .and_then(Value::as_u64),
        "effect_scheduled_event_count": receipt
            .and_then(|value| value.get("effect_scheduled_event_count"))
            .and_then(Value::as_u64),
        "action_dispatch_count": receipt
            .and_then(|value| value.get("action_dispatch_count"))
            .and_then(Value::as_u64),
        "proof_scope": receipt
            .and_then(|value| value.get("proof_scope"))
            .and_then(Value::as_str),
    })
}

fn state_runtime_browser_receipt_stale_reasons(receipt: &Value) -> Vec<String> {
    let api_methods = receipt.get("api_methods").unwrap_or(&Value::Null);
    let mut reasons = Vec::new();

    if receipt.get("schema").and_then(Value::as_str) != Some(STATE_RUNTIME_BROWSER_RECEIPT_SCHEMA) {
        reasons.push("schema-mismatch".to_string());
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true) {
        reasons.push("passed-not-true".to_string());
    }
    if receipt
        .get("browser_runtime_executed")
        .and_then(Value::as_bool)
        != Some(true)
    {
        reasons.push("browser-runtime-not-executed".to_string());
    }
    if receipt
        .get("runtime_global_present")
        .and_then(Value::as_bool)
        != Some(true)
    {
        reasons.push("runtime-global-missing".to_string());
    }
    if receipt
        .get("full_react_hook_runtime")
        .and_then(Value::as_bool)
        != Some(false)
    {
        reasons.push("full-react-hook-runtime-claimed".to_string());
    }
    if receipt
        .get("react_api_shim_executed")
        .and_then(Value::as_bool)
        != Some(false)
    {
        reasons.push("react-api-shim-executed".to_string());
    }
    if receipt
        .get("state_reflection_event_count")
        .and_then(Value::as_u64)
        .is_none_or(|count| count < 3)
    {
        reasons.push("state-reflection-event-count-too-low".to_string());
    }
    if receipt
        .get("derived_reflection_event_count")
        .and_then(Value::as_u64)
        .is_none_or(|count| count < 2)
    {
        reasons.push("derived-reflection-event-count-too-low".to_string());
    }
    if receipt
        .get("effect_scheduled_event_count")
        .and_then(Value::as_u64)
        .is_none_or(|count| count < 2)
    {
        reasons.push("effect-scheduled-event-count-too-low".to_string());
    }
    if receipt
        .get("action_dispatch_count")
        .and_then(Value::as_u64)
        .is_none_or(|count| count < 3)
    {
        reasons.push("action-dispatch-count-too-low".to_string());
    }
    for method in [
        "getSnapshot",
        "setSlot",
        "dispatch",
        "refreshDerivedSlots",
        "scheduleEffectsForState",
    ] {
        if !receipt_string_array_contains(api_methods, method) {
            reasons.push(format!("api-method-missing-{method}"));
        }
    }
    if receipt.get("proof_scope").and_then(Value::as_str)
        != Some("local-in-app-browser-state-runtime-replay")
    {
        reasons.push("proof-scope-not-local-browser-state-runtime-replay".to_string());
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false) {
        reasons.push("release-ready-claim-not-false".to_string());
    }
    if receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false) {
        reasons.push("global-speed-claim-not-false".to_string());
    }

    reasons
}

fn state_runtime_browser_status_from_receipt(receipt: &Value) -> &'static str {
    if state_runtime_browser_receipt_is_current(receipt) {
        "state-runtime-browser-replay-current"
    } else {
        "state-runtime-browser-replay-stale"
    }
}

fn state_runtime_browser_receipt_is_current(receipt: &Value) -> bool {
    let api_methods = receipt.get("api_methods").unwrap_or(&Value::Null);

    receipt.get("schema").and_then(Value::as_str) == Some(STATE_RUNTIME_BROWSER_RECEIPT_SCHEMA)
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("runtime_global_present")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("full_react_hook_runtime")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("react_api_shim_executed")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("state_reflection_event_count")
            .and_then(Value::as_u64)
            .is_some_and(|count| count >= 3)
        && receipt
            .get("derived_reflection_event_count")
            .and_then(Value::as_u64)
            .is_some_and(|count| count >= 2)
        && receipt
            .get("effect_scheduled_event_count")
            .and_then(Value::as_u64)
            .is_some_and(|count| count >= 2)
        && receipt
            .get("action_dispatch_count")
            .and_then(Value::as_u64)
            .is_some_and(|count| count >= 3)
        && receipt_string_array_contains(api_methods, "getSnapshot")
        && receipt_string_array_contains(api_methods, "setSlot")
        && receipt_string_array_contains(api_methods, "dispatch")
        && receipt_string_array_contains(api_methods, "refreshDerivedSlots")
        && receipt_string_array_contains(api_methods, "scheduleEffectsForState")
        && receipt.get("proof_scope").and_then(Value::as_str)
            == Some("local-in-app-browser-state-runtime-replay")
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
}

fn reactivity_model_receipt_status(receipt: Option<&Value>) -> Value {
    json!({
        "contract": REACTIVITY_MODEL_RECEIPT_SCHEMA,
        "current": receipt.is_some_and(reactivity_model_receipt_is_current),
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "passed": receipt
            .and_then(|value| value.get("passed"))
            .and_then(Value::as_bool),
        "status": receipt
            .and_then(|value| value.get("status"))
            .and_then(Value::as_str)
            .unwrap_or("missing-reactivity-model-receipt"),
        "reactivity_model_schema": receipt
            .and_then(|value| value.get("reactivity_model_schema"))
            .and_then(Value::as_str),
        "public_runtime": receipt
            .and_then(|value| value.get("public_runtime"))
            .and_then(Value::as_str),
        "source_owned_runtime": receipt
            .and_then(|value| value.get("source_owned_runtime"))
            .and_then(Value::as_bool),
        "runtime_capabilities_schema": receipt
            .and_then(|value| value.get("runtime_capabilities_schema"))
            .and_then(Value::as_str),
        "unsupported_react_api_policy": receipt
            .and_then(|value| value.get("unsupported_react_api_policy"))
            .and_then(Value::as_str),
        "compatibility_lowering": receipt
            .and_then(|value| value.get("compatibility_lowering"))
            .cloned(),
        "compatibility_lowering_api": receipt
            .and_then(|value| value.get("compatibility_lowering_api"))
            .cloned(),
        "exact_lowering_required": receipt
            .and_then(|value| value.get("exact_lowering_required"))
            .and_then(Value::as_bool),
        "use_state_lowering_rule": receipt
            .and_then(|value| value.get("use_state_lowering_rule"))
            .and_then(Value::as_str),
        "unsupported_unlowerable_use_state_diagnostic": receipt
            .and_then(|value| value.get("unsupported_unlowerable_use_state_diagnostic"))
            .and_then(Value::as_str),
        "adapter_boundary_required_when_unlowerable": receipt
            .and_then(|value| value.get("adapter_boundary_required_when_unlowerable"))
            .and_then(Value::as_bool),
        "browser_proof_status": receipt
            .and_then(|value| value.get("browser_proof_status"))
            .and_then(Value::as_str),
        "browser_replay_receipt_contract": receipt
            .and_then(|value| value.get("browser_replay_receipt_contract"))
            .and_then(Value::as_str),
        "browser_replay_receipt": receipt
            .and_then(|value| value.get("browser_replay_receipt"))
            .and_then(Value::as_str),
        "browser_replay_receipt_sr": receipt
            .and_then(|value| value.get("browser_replay_receipt_sr"))
            .and_then(Value::as_str),
        "browser_replay_receipt_machine": receipt
            .and_then(|value| value.get("browser_replay_receipt_machine"))
            .and_then(Value::as_str),
        "node_vm_state_runtime_replay_status": receipt
            .and_then(|value| value.get("node_vm_state_runtime_replay_status"))
            .and_then(Value::as_str),
        "browser_runtime_executed": receipt
            .and_then(|value| value.get("browser_runtime_executed"))
            .and_then(Value::as_bool),
        "react_api_shim_executed": receipt
            .and_then(|value| value.get("react_api_shim_executed"))
            .and_then(Value::as_bool),
        "full_react_hook_runtime": receipt
            .and_then(|value| value.get("full_react_hook_runtime"))
            .and_then(Value::as_bool),
        "source_check_count": receipt
            .and_then(|value| value.get("source_check_count"))
            .and_then(Value::as_u64),
        "source_check_current_count": receipt
            .and_then(|value| value.get("source_check_current_count"))
            .and_then(Value::as_u64),
        "proof_scope": receipt
            .and_then(|value| value.get("proof_scope"))
            .and_then(Value::as_str),
    })
}

fn reactivity_model_receipt_is_current(receipt: &Value) -> bool {
    let dx_native_api = receipt.get("dx_native_api").unwrap_or(&Value::Null);
    let compatibility_lowering = receipt
        .get("compatibility_lowering")
        .unwrap_or(&Value::Null);
    let unsupported_react_hooks = receipt
        .get("unsupported_react_hooks")
        .unwrap_or(&Value::Null);

    receipt.get("schema").and_then(Value::as_str) == Some(REACTIVITY_MODEL_RECEIPT_SCHEMA)
        && receipt.get("id").and_then(Value::as_str) == Some("reactivity")
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt.get("status").and_then(Value::as_str)
            == Some("source-owned-reactivity-model-foundation-current")
        && receipt
            .get("reactivity_model_schema")
            .and_then(Value::as_str)
            == Some(readiness::READINESS_REACTIVITY_MODEL_SCHEMA)
        && receipt
            .get("runtime_capabilities_schema")
            .and_then(Value::as_str)
            == Some("dx.tsx.dxNativeReactivityCapabilities")
        && receipt.get("source_owned").and_then(Value::as_bool) == Some(true)
        && receipt.get("source_owned_runtime").and_then(Value::as_bool) == Some(true)
        && receipt_string_array_contains(dx_native_api, "state()")
        && receipt_string_array_contains(dx_native_api, "derived()")
        && receipt_string_array_contains(dx_native_api, "effect()")
        && receipt_string_array_contains(dx_native_api, "action()")
        && receipt_string_array_contains(
            compatibility_lowering,
            "useState(exact-dx-state-slot-only)",
        )
        && receipt_string_array_contains(
            receipt
                .get("compatibility_lowering_api")
                .unwrap_or(&Value::Null),
            "useState",
        )
        && receipt
            .get("exact_lowering_required")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("use_state_lowering_rule")
            .and_then(Value::as_str)
            == Some(
                "lower useState only when state_graph_has_exact_use_state_lowering proves every binding maps to a compiler-owned DX state slot",
            )
        && receipt
            .get("unsupported_unlowerable_use_state_diagnostic")
            .and_then(Value::as_str)
            == Some("dx.react-hook.useState.missing-exact-state-slot")
        && receipt
            .get("adapter_boundary_required_when_unlowerable")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt_string_array_contains(unsupported_react_hooks, "useReducer")
        && receipt_string_array_contains(unsupported_react_hooks, "useContext")
        && receipt_string_array_contains(unsupported_react_hooks, "useEffect")
        && receipt_string_array_contains(unsupported_react_hooks, "useTransition")
        && receipt
            .get("node_modules_required")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("react_api_shim_executed")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("full_react_hook_runtime")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("hosted_provider_proof")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt.get("browser_proof_status").and_then(Value::as_str)
            == Some("foundation-not-release-proof")
        && receipt
            .get("node_vm_state_runtime_replay_status")
            .and_then(Value::as_str)
            == Some("source-guarded-not-real-browser-proof")
        && receipt
            .get("browser_replay_receipt_contract")
            .and_then(Value::as_str)
            == Some(STATE_RUNTIME_BROWSER_RECEIPT_SCHEMA)
        && receipt
            .get("browser_replay_receipt")
            .and_then(Value::as_str)
            == Some(STATE_RUNTIME_BROWSER_RECEIPT)
        && receipt
            .get("browser_replay_receipt_sr")
            .and_then(Value::as_str)
            == Some(STATE_RUNTIME_BROWSER_RECEIPT_SR)
        && receipt
            .get("browser_replay_receipt_machine")
            .and_then(Value::as_str)
            == Some(STATE_RUNTIME_BROWSER_RECEIPT_MACHINE)
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt
            .get("readiness_release_ready")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
        && receipt.get("proof_scope").and_then(Value::as_str)
            == Some("local-source-owned-reactivity-model-foundation")
        && receipt.get("source_check_count").and_then(Value::as_u64) == Some(6)
        && receipt
            .get("source_check_current_count")
            .and_then(Value::as_u64)
            == Some(6)
}

fn docs_onboarding_receipt_status(receipt: Option<&Value>) -> Value {
    json!({
        "contract": DOCS_ONBOARDING_RECEIPT_SCHEMA,
        "current": receipt.is_some_and(docs_onboarding_receipt_is_current),
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "passed": receipt
            .and_then(|value| value.get("passed"))
            .and_then(Value::as_bool),
        "status": receipt
            .and_then(|value| value.get("status"))
            .and_then(Value::as_str)
            .unwrap_or("missing-docs-onboarding-receipt"),
        "docs_onboarding_schema": receipt
            .and_then(|value| value.get("docs_onboarding_schema"))
            .and_then(Value::as_str),
        "docs_doctor_schema": receipt
            .and_then(|value| value.get("docs_doctor_schema"))
            .and_then(Value::as_str),
        "docs_doctor_command": receipt
            .and_then(|value| value.get("docs_doctor_command"))
            .and_then(Value::as_str),
        "docs_doctor_report_evaluated": receipt
            .and_then(|value| value.get("docs_doctor_report_evaluated"))
            .and_then(Value::as_bool),
        "docs_doctor_runtime_executed": receipt
            .and_then(|value| value.get("docs_doctor_runtime_executed"))
            .and_then(Value::as_bool),
        "docs_doctor_error_count": receipt
            .and_then(|value| value.get("docs_doctor_error_count"))
            .and_then(Value::as_u64),
        "docs_doctor_warning_count": receipt
            .and_then(|value| value.get("docs_doctor_warning_count"))
            .and_then(Value::as_u64),
        "public_docs_source_guarded": receipt
            .and_then(|value| value.get("public_docs_source_guarded"))
            .and_then(Value::as_bool),
        "compatibility_surfaces_warning_only": receipt
            .and_then(|value| value.get("compatibility_surfaces_warning_only"))
            .and_then(Value::as_bool),
        "generated_archived_warning_surfaces_clean": receipt
            .and_then(|value| value.get("generated_archived_warning_surfaces_clean"))
            .and_then(Value::as_bool),
        "generated_archived_warning_surfaces_promoted": receipt
            .and_then(|value| value.get("generated_archived_warning_surfaces_promoted"))
            .and_then(Value::as_bool),
        "generated_archived_warning_finding_count": receipt
            .and_then(|value| value.get("generated_archived_warning_finding_count"))
            .and_then(Value::as_u64),
        "source_check_count": receipt
            .and_then(|value| value.get("source_check_count"))
            .and_then(Value::as_u64),
        "source_check_current_count": receipt
            .and_then(|value| value.get("source_check_current_count"))
            .and_then(Value::as_u64),
        "proof_scope": receipt
            .and_then(|value| value.get("proof_scope"))
            .and_then(Value::as_str),
    })
}

fn docs_onboarding_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str) == Some(DOCS_ONBOARDING_RECEIPT_SCHEMA)
        && receipt.get("id").and_then(Value::as_str) == Some("docs-onboarding-doctor")
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt.get("status").and_then(Value::as_str)
            == Some("source-owned-docs-onboarding-foundation-current")
        && receipt
            .get("docs_onboarding_schema")
            .and_then(Value::as_str)
            == Some(readiness::READINESS_DOCS_ONBOARDING_SCHEMA)
        && receipt.get("docs_doctor_schema").and_then(Value::as_str) == Some("dx.www.docs_doctor")
        && receipt.get("source_owned").and_then(Value::as_bool) == Some(true)
        && receipt
            .get("docs_doctor_runtime_executed")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .get("docs_doctor_report_evaluated")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("docs_doctor_error_count")
            .and_then(Value::as_u64)
            == Some(0)
        && receipt
            .get("public_docs_source_guarded")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("compatibility_surfaces_warning_only")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("generated_archived_warning_surfaces_clean")
            .and_then(Value::as_bool)
            .is_some()
        && receipt
            .get("generated_archived_warning_surfaces_promoted")
            .and_then(Value::as_bool)
            .is_some()
        && receipt
            .get("generated_archived_warning_finding_count")
            .and_then(Value::as_u64)
            .is_some()
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt
            .get("readiness_release_ready")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
        && receipt.get("proof_scope").and_then(Value::as_str)
            == Some("local-source-owned-docs-onboarding-foundation")
        && receipt.get("source_check_count").and_then(Value::as_u64) == Some(5)
        && receipt
            .get("source_check_current_count")
            .and_then(Value::as_u64)
            == Some(5)
}

fn docs_doctor_command_replay_receipt_status(receipt: Option<&Value>) -> Value {
    json!({
        "contract": DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_SCHEMA,
        "current": receipt.is_some_and(docs_doctor_command_replay_receipt_is_current),
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "passed": receipt
            .and_then(|value| value.get("passed"))
            .and_then(Value::as_bool),
        "status": receipt
            .and_then(|value| value.get("status"))
            .and_then(Value::as_str)
            .unwrap_or("missing-docs-doctor-command-replay-receipt"),
        "command": receipt
            .and_then(|value| value.get("command"))
            .and_then(Value::as_str),
        "docs_doctor_schema": receipt
            .and_then(|value| value.get("docs_doctor_schema"))
            .and_then(Value::as_str),
        "docs_doctor_runtime_executed": receipt
            .and_then(|value| value.get("docs_doctor_runtime_executed"))
            .and_then(Value::as_bool),
        "command_replay_executed": receipt
            .and_then(|value| value.get("command_replay_executed"))
            .and_then(Value::as_bool),
        "docs_doctor_error_count": receipt
            .and_then(|value| value.get("docs_doctor_error_count"))
            .and_then(Value::as_u64),
        "docs_doctor_warning_count": receipt
            .and_then(|value| value.get("docs_doctor_warning_count"))
            .and_then(Value::as_u64),
        "generated_archived_warning_finding_count": receipt
            .and_then(|value| value.get("generated_archived_warning_finding_count"))
            .and_then(Value::as_u64),
        "proof_scope": receipt
            .and_then(|value| value.get("proof_scope"))
            .and_then(Value::as_str),
    })
}

fn docs_doctor_command_replay_receipt_is_current(receipt: &Value) -> bool {
    docs_doctor::docs_doctor_command_replay_receipt_is_current(receipt)
}

fn island_abi_receipt_status(receipt: Option<&Value>) -> Value {
    let stale_reasons = receipt
        .map(island_abi_receipt_stale_reasons)
        .unwrap_or_else(|| vec!["island-abi-receipt-missing".to_string()]);
    json!({
        "contract": ISLAND_ABI_RECEIPT_SCHEMA,
        "current": stale_reasons.is_empty(),
        "source_owned_receipt_required": true,
        "browser_adapter_proof_required": true,
        "browser_import_command": ISLAND_BROWSER_IMPORT_COMMAND,
        "canonical_browser_proof_target": browser_receipt_proof_target("island-browser"),
        "stale_reasons": stale_reasons,
        "replay_command": READINESS_WRITE_RECEIPTS_COMMAND,
        "contract_test_command": ISLAND_ABI_CONTRACT_TEST_COMMAND,
        "serializer_receipt": ISLAND_ABI_RECEIPT_SR,
        "machine_contract": ISLAND_ABI_RECEIPT_MACHINE,
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "passed": receipt
            .and_then(|value| value.get("passed"))
            .and_then(Value::as_bool),
        "status": receipt
            .and_then(|value| value.get("status"))
            .and_then(Value::as_str)
            .unwrap_or("missing-island-abi-receipt"),
        "directive_style_id": receipt
            .and_then(|value| value.get("directive_style_id"))
            .and_then(Value::as_str),
        "core_directives": receipt
            .and_then(|value| value.get("core_directives"))
            .cloned(),
        "additional_supported_directives": receipt
            .and_then(|value| value.get("additional_supported_directives"))
            .cloned(),
        "route_unit_proof_metadata": receipt
            .and_then(|value| value.get("route_unit_proof_metadata"))
            .and_then(Value::as_str),
        "route_streaming_island_metadata": receipt
            .and_then(|value| value.get("route_streaming_island_metadata"))
            .cloned(),
        "no_js_fallback_required": receipt
            .and_then(|value| value.get("no_js_fallback_required"))
            .and_then(Value::as_bool),
        "node_modules_required": receipt
            .and_then(|value| value.get("node_modules_required"))
            .and_then(Value::as_bool),
        "full_react_hydration": receipt
            .and_then(|value| value.get("full_react_hydration"))
            .and_then(Value::as_bool),
        "browser_runtime_executed": receipt
            .and_then(|value| value.get("browser_runtime_executed"))
            .and_then(Value::as_bool),
        "hosted_provider_proof": receipt
            .and_then(|value| value.get("hosted_provider_proof"))
            .and_then(Value::as_bool),
        "provider_adapter_executed": receipt
            .and_then(|value| value.get("provider_adapter_executed"))
            .and_then(Value::as_bool),
        "source_check_count": receipt
            .and_then(|value| value.get("source_check_count"))
            .and_then(Value::as_u64),
        "source_check_current_count": receipt
            .and_then(|value| value.get("source_check_current_count"))
            .and_then(Value::as_u64),
        "proof_scope": receipt
            .and_then(|value| value.get("proof_scope"))
            .and_then(Value::as_str),
    })
}

fn island_abi_receipt_stale_reasons(receipt: &Value) -> Vec<String> {
    let mut stale_reasons = Vec::new();
    let directives = receipt.get("directives").unwrap_or(&Value::Null);
    let core_directives = receipt.get("core_directives").unwrap_or(&Value::Null);
    let supported_directives = receipt.get("supported_directives").unwrap_or(&Value::Null);
    let unsupported = receipt
        .get("unsupported_directive_syntax")
        .unwrap_or(&Value::Null);
    let streaming_metadata = receipt
        .get("route_streaming_island_metadata")
        .unwrap_or(&Value::Null);

    if receipt.get("schema").and_then(Value::as_str) != Some(ISLAND_ABI_RECEIPT_SCHEMA) {
        stale_reasons.push("schema-mismatch-or-missing".to_string());
    }
    if receipt.get("id").and_then(Value::as_str) != Some("islands") {
        stale_reasons.push("id-not-islands".to_string());
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("passed-not-true".to_string());
    }
    if receipt.get("status").and_then(Value::as_str)
        != Some("source-owned-island-abi-foundation-current")
    {
        stale_reasons.push("status-not-current".to_string());
    }
    if receipt.get("source_owned").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("source-owned-not-true".to_string());
    }
    if receipt.get("source_owned_runtime").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("source-owned-runtime-not-true".to_string());
    }
    if receipt.get("directive_style_id").and_then(Value::as_str) != Some("camelCase-jsx-props") {
        stale_reasons.push("directive-style-not-camel-case-jsx-props".to_string());
    }
    for directive in ["clientLoad", "clientVisible", "clientIdle", "clientOnly"] {
        if !receipt_string_array_contains(directives, directive)
            || !receipt_string_array_contains(core_directives, directive)
            || !receipt_string_array_contains(supported_directives, directive)
        {
            stale_reasons.push(format!("missing-core-directive-{directive}"));
        }
    }
    for directive in ["clientMedia", "clientInteraction"] {
        if !receipt_string_array_contains(supported_directives, directive) {
            stale_reasons.push(format!("missing-supported-directive-{directive}"));
        }
    }
    for syntax in [
        "client:load",
        "client:visible",
        "client:idle",
        "client:only",
    ] {
        if !receipt_string_array_contains(unsupported, syntax) {
            stale_reasons.push(format!("missing-unsupported-syntax-{syntax}"));
        }
    }
    if receipt
        .get("no_js_fallback_required")
        .and_then(Value::as_bool)
        != Some(true)
    {
        stale_reasons.push("no-js-fallback-required-not-true".to_string());
    }
    if receipt
        .get("node_modules_required")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("node-modules-required-overclaimed".to_string());
    }
    if receipt.get("full_react_hydration").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("full-react-hydration-overclaimed".to_string());
    }
    if receipt
        .get("browser_runtime_executed")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("browser-runtime-executed-overclaimed".to_string());
    }
    if receipt
        .get("hosted_provider_proof")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("hosted-provider-proof-overclaimed".to_string());
    }
    if receipt
        .get("provider_adapter_executed")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("provider-adapter-executed-overclaimed".to_string());
    }
    if receipt.get("source_check_count").and_then(Value::as_u64) != Some(5) {
        stale_reasons.push("source-check-count-not-five".to_string());
    }
    if receipt
        .get("source_check_current_count")
        .and_then(Value::as_u64)
        != Some(5)
    {
        stale_reasons.push("source-check-current-count-not-five".to_string());
    }
    if receipt
        .get("route_unit_proof_metadata")
        .and_then(Value::as_str)
        != Some("DxRouteReceipt.client_island_abi")
    {
        stale_reasons.push("route-unit-proof-metadata-missing".to_string());
    }
    if !receipt_string_array_contains(streaming_metadata, "directive_style_id")
        || !receipt_string_array_contains(streaming_metadata, "no_js_fallback_required")
    {
        stale_reasons.push("route-streaming-island-metadata-incomplete".to_string());
    }
    if receipt.get("proof_scope").and_then(Value::as_str)
        != Some("local-source-owned-island-abi-foundation")
    {
        stale_reasons.push("proof-scope-not-source-owned-island-foundation".to_string());
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("release-ready-overclaimed".to_string());
    }
    if receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("fastest-world-claim-overclaimed".to_string());
    }
    stale_reasons
}

fn island_abi_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("schema_revision").and_then(Value::as_u64) == Some(1)
        && island_abi_receipt_stale_reasons(receipt).is_empty()
}

fn island_browser_receipt_status(receipt: Option<&Value>) -> Value {
    let stale_reasons = receipt
        .map(island_browser_receipt_stale_reasons)
        .unwrap_or_else(|| vec!["island-browser-receipt-missing".to_string()]);
    json!({
        "contract": ISLAND_BROWSER_RECEIPT_SCHEMA,
        "current": stale_reasons.is_empty(),
        "browser_receipt_required": true,
        "hosted_provider_proof_required": true,
        "browser_receipt_gate": browser_receipt_action_metadata(
            "island-browser",
            ISLAND_BROWSER_RECEIPT,
            ISLAND_BROWSER_RECEIPT_SR,
            ISLAND_BROWSER_RECEIPT_MACHINE,
            ISLAND_BROWSER_IMPORT_COMMAND,
            "island-browser-receipt-missing",
        ),
        "canonical_browser_proof_target": browser_receipt_proof_target("island-browser"),
        "stale_reasons": stale_reasons,
        "replay_command": READINESS_INSPECT_COMMAND,
        "write_receipts_command": READINESS_WRITE_RECEIPTS_COMMAND,
        "import_command": ISLAND_BROWSER_IMPORT_COMMAND,
        "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
        "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
        "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
        "snapshot_import_command": BROWSER_PAGE_SNAPSHOT_IMPORT_COMMAND,
        "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
        "serializer_receipt": ISLAND_BROWSER_RECEIPT_SR,
        "machine_contract": ISLAND_BROWSER_RECEIPT_MACHINE,
        "schema": receipt
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_str),
        "passed": receipt
            .and_then(|value| value.get("passed"))
            .and_then(Value::as_bool),
        "status": receipt
            .and_then(|value| value.get("status"))
            .and_then(Value::as_str)
            .unwrap_or("missing-island-browser-receipt"),
        "browser_runtime_executed": receipt
            .and_then(|value| value.get("browser_runtime_executed"))
            .and_then(Value::as_bool),
        "island_count": receipt
            .and_then(|value| value.get("island_count"))
            .and_then(Value::as_u64),
        "source_owned_island_count": receipt
            .and_then(|value| value.get("source_owned_island_count"))
            .and_then(Value::as_u64),
        "directives_seen": receipt
            .and_then(|value| value.get("directives_seen"))
            .cloned(),
        "missing_core_directives": receipt
            .and_then(|value| value.get("missing_core_directives"))
            .cloned(),
        "event_node_count": receipt
            .and_then(|value| value.get("event_node_count"))
            .and_then(Value::as_u64),
        "client_island_event_count": receipt
            .and_then(|value| value.get("client_island_event_count"))
            .and_then(Value::as_u64),
        "proof_scope": receipt
            .and_then(|value| value.get("proof_scope"))
            .and_then(Value::as_str),
    })
}

fn island_browser_receipt_stale_reasons(receipt: &Value) -> Vec<String> {
    let mut stale_reasons = Vec::new();
    if receipt.get("schema").and_then(Value::as_str) != Some(ISLAND_BROWSER_RECEIPT_SCHEMA) {
        stale_reasons.push("schema-mismatch-or-missing".to_string());
    }
    if receipt.get("schema_revision").and_then(Value::as_u64) != Some(1) {
        stale_reasons.push("schema-revision-not-one".to_string());
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("passed-not-true".to_string());
    }
    if receipt.get("status").and_then(Value::as_str)
        != Some("source-owned-island-browser-replay-current")
    {
        stale_reasons.push("status-not-current".to_string());
    }
    if receipt
        .get("browser_runtime_executed")
        .and_then(Value::as_bool)
        != Some(true)
    {
        stale_reasons.push("browser-runtime-not-executed".to_string());
    }
    if receipt.get("source_owned_bridge").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("source-owned-bridge-missing".to_string());
    }
    if receipt.get("bridge_abi_style").and_then(Value::as_str) != Some("camelCase") {
        stale_reasons.push("bridge-abi-style-not-camelcase".to_string());
    }
    if receipt.get("directive_style").and_then(Value::as_str) != Some("camelCase-jsx-props") {
        stale_reasons.push("directive-style-not-camelcase-jsx-props".to_string());
    }
    for directive in ["clientLoad", "clientVisible", "clientIdle", "clientOnly"] {
        if !receipt_string_array_contains(
            receipt.get("directives_seen").unwrap_or(&Value::Null),
            directive,
        ) {
            stale_reasons.push(format!("missing-browser-directive-{directive}"));
        }
    }
    if !receipt
        .get("missing_core_directives")
        .and_then(Value::as_array)
        .is_some_and(Vec::is_empty)
    {
        stale_reasons.push("missing-core-directives-not-empty".to_string());
    }
    if receipt
        .get("island_count")
        .and_then(Value::as_u64)
        .is_none_or(|count| count == 0)
    {
        stale_reasons.push("island-count-zero".to_string());
    }
    if receipt
        .get("source_owned_island_count")
        .and_then(Value::as_u64)
        != receipt.get("island_count").and_then(Value::as_u64)
    {
        stale_reasons.push("source-owned-island-count-mismatch".to_string());
    }
    if receipt
        .get("event_node_count")
        .and_then(Value::as_u64)
        .is_none_or(|count| count == 0)
    {
        stale_reasons.push("event-node-count-zero".to_string());
    }
    if receipt
        .get("missed_event_replay_count")
        .and_then(Value::as_u64)
        != Some(0)
    {
        stale_reasons.push("missed-event-replay-count-not-zero".to_string());
    }
    if receipt.get("proof_scope").and_then(Value::as_str)
        != Some("local-in-app-browser-source-owned-island-replay")
    {
        stale_reasons.push("proof-scope-not-local-island-browser-replay".to_string());
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("release-ready-overclaimed".to_string());
    }
    if receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("fastest-world-claim-overclaimed".to_string());
    }
    stale_reasons
}

fn readiness_receipt_gate_status(receipt: &Value) -> Value {
    let stale_reasons = readiness_receipt_gate_stale_reasons(receipt);
    let metadata_current = stale_reasons.is_empty();
    let replay_verified_current = readiness_receipt_gate_replay_verified_current(receipt);
    let missing_replay_commands = readiness_required_replay_commands()
        .iter()
        .filter_map(|(field, command)| {
            if receipt_string_array_contains(receipt.get(field).unwrap_or(&Value::Null), command) {
                None
            } else {
                Some(json!(command))
            }
        })
        .collect::<Vec<_>>();
    json!({
        "current": metadata_current,
        "metadata_current": metadata_current,
        "replay_verified_current": replay_verified_current,
        "proof_status": if !metadata_current {
            "missing-or-unsafe-readiness-gate-metadata"
        } else if replay_verified_current {
            "replay-verified-current"
        } else if readiness_receipt_has_scoped_release_ready(receipt) {
            "relative-local-proof-backed-release-ready"
        } else {
            "static-advisory-not-release-proof"
        },
        "stale_reasons": stale_reasons,
        "release_ready": readiness_receipt_has_scoped_release_ready(receipt),
        "fastest_world_claim": false,
        "gate_release_ready": readiness_receipt_has_scoped_release_ready(receipt),
        "gate_fastest_world_claim": false,
        "claimed_release_ready": receipt.get("release_ready").cloned().unwrap_or(Value::Null),
        "claimed_fastest_world_claim": receipt.get("fastest_world_claim").cloned().unwrap_or(Value::Null),
        "claimed_gate_release_ready": receipt.pointer("/readiness_gate_status/release_ready").cloned().unwrap_or(Value::Null),
        "claimed_gate_fastest_world_claim": receipt.pointer("/readiness_gate_status/fastest_world_claim").cloned().unwrap_or(Value::Null),
        "score_kind": receipt.pointer("/readiness_gate_status/score_kind").cloned().unwrap_or(Value::Null),
        "verified_from_replay_receipts": receipt.pointer("/readiness_gate_status/verified_from_replay_receipts").cloned().unwrap_or(Value::Null),
        "receipt_freshness": receipt.pointer("/readiness_gate_status/receipt_freshness").cloned().unwrap_or(Value::Null),
        "has_readiness_replay": receipt_string_array_contains(
            receipt.get("readiness_replay_commands").unwrap_or(&Value::Null),
            "dx www readiness --json --full",
        ),
        "has_agent_context_replay": receipt_string_array_contains(
            receipt.get("replay_commands").unwrap_or(&Value::Null),
            "dx www agent-context --json --full",
        ),
        "has_docs_doctor_replay": receipt_string_array_contains(
            receipt.get("replay_commands").unwrap_or(&Value::Null),
            "dx www docs-doctor --json",
        ),
        "missing_replay_commands": missing_replay_commands,
    })
}

fn readiness_receipt_gate_status_missing() -> Value {
    json!({
        "current": false,
        "metadata_current": false,
        "replay_verified_current": false,
        "proof_status": "missing-readiness-gate-metadata",
        "stale_reasons": [{
            "code": "missing-readiness-gate-metadata",
            "path": TEMPLATE_CHECK_RECEIPT
        }],
        "release_ready": Value::Null,
        "fastest_world_claim": Value::Null,
        "gate_release_ready": Value::Null,
        "gate_fastest_world_claim": Value::Null,
        "claimed_release_ready": Value::Null,
        "claimed_fastest_world_claim": Value::Null,
        "claimed_gate_release_ready": Value::Null,
        "claimed_gate_fastest_world_claim": Value::Null,
        "score_kind": Value::Null,
        "verified_from_replay_receipts": Value::Null,
        "receipt_freshness": Value::Null,
        "has_readiness_replay": false,
        "has_agent_context_replay": false,
        "has_docs_doctor_replay": false,
        "missing_replay_commands": readiness_required_replay_commands()
            .iter()
            .map(|(_, command)| json!(command))
            .collect::<Vec<_>>(),
    })
}

fn readiness_receipt_gate_metadata_current(receipt: &Value) -> bool {
    readiness_receipt_gate_stale_reasons(receipt).is_empty()
}

fn readiness_receipt_gate_stale_reasons(receipt: &Value) -> Vec<Value> {
    let mut stale_reasons = Vec::new();
    let scoped_release_ready = readiness_receipt_has_scoped_release_ready(receipt);

    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false) && !scoped_release_ready
    {
        stale_reasons.push(json!({
            "code": "release-ready-claim-unsafe",
            "field": "release_ready",
            "expected": "false or scoped local proof-backed release readiness",
            "actual": receipt.get("release_ready").cloned().unwrap_or(Value::Null)
        }));
    }
    if receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push(json!({
            "code": "global-speed-claim-unsafe-for-static-advisory",
            "field": "fastest_world_claim",
            "expected": false,
            "actual": receipt.get("fastest_world_claim").cloned().unwrap_or(Value::Null)
        }));
    }
    if receipt
        .pointer("/readiness_gate_status/release_ready")
        .and_then(Value::as_bool)
        != Some(false)
        && !scoped_release_ready
    {
        stale_reasons.push(json!({
            "code": "gate-release-ready-claim-unsafe",
            "field": "readiness_gate_status.release_ready",
            "expected": "false or scoped local proof-backed release readiness",
            "actual": receipt.pointer("/readiness_gate_status/release_ready").cloned().unwrap_or(Value::Null)
        }));
    }
    if receipt
        .pointer("/readiness_gate_status/fastest_world_claim")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push(json!({
            "code": "gate-global-speed-claim-unsafe-for-static-advisory",
            "field": "readiness_gate_status.fastest_world_claim",
            "expected": false,
            "actual": receipt.pointer("/readiness_gate_status/fastest_world_claim").cloned().unwrap_or(Value::Null)
        }));
    }
    let score_kind = receipt
        .pointer("/readiness_gate_status/score_kind")
        .and_then(Value::as_str);
    if !matches!(
        score_kind,
        Some("static-advisory-not-release-proof" | "relative-local-proof-backed-release-ready")
    ) {
        stale_reasons.push(json!({
            "code": "score-kind-unsafe-for-readiness",
            "field": "readiness_gate_status.score_kind",
            "expected": "static-advisory-not-release-proof or relative-local-proof-backed-release-ready",
            "actual": receipt.pointer("/readiness_gate_status/score_kind").cloned().unwrap_or(Value::Null)
        }));
    }
    let replay_receipts_verified = receipt
        .pointer("/readiness_gate_status/verified_from_replay_receipts")
        .and_then(Value::as_bool);
    if replay_receipts_verified != Some(scoped_release_ready) {
        stale_reasons.push(json!({
            "code": "verified-from-replay-receipts-unsafe-for-readiness",
            "field": "readiness_gate_status.verified_from_replay_receipts",
            "expected": scoped_release_ready,
            "actual": receipt.pointer("/readiness_gate_status/verified_from_replay_receipts").cloned().unwrap_or(Value::Null)
        }));
    }
    if !readiness_receipt_freshness_safe(
        receipt
            .pointer("/readiness_gate_status/receipt_freshness")
            .and_then(Value::as_str),
        scoped_release_ready,
    ) {
        stale_reasons.push(json!({
            "code": "receipt-freshness-unsafe-for-readiness",
            "field": "readiness_gate_status.receipt_freshness",
            "expected": if scoped_release_ready { "current" } else { "not-evaluated-in-this-command or local-receipts-evaluated" },
            "actual": receipt.pointer("/readiness_gate_status/receipt_freshness").cloned().unwrap_or(Value::Null)
        }));
    }

    for (field, command) in readiness_required_replay_commands().iter() {
        if !receipt_string_array_contains(receipt.get(field).unwrap_or(&Value::Null), command) {
            stale_reasons.push(json!({
                "code": "missing-replay-command",
                "field": field,
                "command": command
            }));
        }
    }

    stale_reasons
}

fn static_advisory_receipt_freshness_safe(value: Option<&str>) -> bool {
    readiness_receipt_freshness_safe(value, false)
}

fn readiness_receipt_freshness_safe(value: Option<&str>, scoped_release_ready: bool) -> bool {
    if scoped_release_ready {
        return value == Some("current");
    }
    matches!(
        value,
        Some("not-evaluated-in-this-command" | "local-receipts-evaluated")
    )
}

fn readiness_receipt_has_scoped_release_ready(receipt: &Value) -> bool {
    receipt.get("release_ready").and_then(Value::as_bool) == Some(true)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
        && receipt
            .get("global_speed_claim_allowed")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .pointer("/readiness_gate_status/release_ready")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .pointer("/readiness_gate_status/fastest_world_claim")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .pointer("/readiness_gate_status/release_claim_allowed")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .pointer("/readiness_gate_status/global_speed_claim_allowed")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt
            .pointer("/readiness_gate_status/score_kind")
            .and_then(Value::as_str)
            == Some("relative-local-proof-backed-release-ready")
        && receipt
            .pointer("/readiness_gate_status/verified_from_replay_receipts")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .pointer("/readiness_gate_status/receipt_freshness")
            .and_then(Value::as_str)
            == Some("current")
        && receipt
            .pointer("/readiness_gate_status/release_ready_scope")
            .and_then(Value::as_str)
            == Some("local-proof-backed-www-release")
}

fn readiness_required_replay_commands() -> Vec<(&'static str, &'static str)> {
    readiness::readiness_replay_commands()
        .into_iter()
        .flat_map(|command| {
            [
                ("readiness_replay_commands", command),
                ("replay_commands", command),
            ]
        })
        .collect()
}

fn readiness_receipt_gate_replay_verified_current(receipt: &Value) -> bool {
    readiness_receipt_gate_metadata_current(receipt)
        && receipt
            .pointer("/readiness_gate_status/verified_from_replay_receipts")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .pointer("/readiness_gate_status/receipt_freshness")
            .and_then(Value::as_str)
            == Some("current")
}

fn receipt_string_array_contains(value: &Value, expected: &str) -> bool {
    value
        .as_array()
        .is_some_and(|items| items.iter().any(|item| item.as_str() == Some(expected)))
}

fn allowed_checks() -> Vec<Value> {
    vec![
        json!({
            "id": "style",
            "command": "dx style build --json",
            "cost": "light",
            "purpose": "Regenerate source-owned CSS and style receipts."
        }),
        json!({
            "id": "icons",
            "command": "dx icons sync --json",
            "cost": "light",
            "purpose": "Regenerate source-owned icon wrappers from <icon> tags."
        }),
        json!({
            "id": "readiness",
            "command": "dx www readiness --json --full",
            "cost": "light",
            "purpose": "Replay the release readiness proof graph, readiness proof breakdown, and known non-claims."
        }),
        json!({
            "id": "readiness-receipts",
            "command": "dx www readiness --write-receipts --json",
            "cost": "light",
            "purpose": "Write source-owned release readiness receipts that can be proven from the current checkout without inventing missing visual/browser proof."
        }),
        json!({
            "id": "docs-doctor",
            "command": "dx www docs-doctor --json",
            "cost": "light",
            "purpose": "Scan public WWW docs for stale App Router, output-path, and proof claims."
        }),
        json!({
            "id": "web-perf-dev",
            "command": "dx check web-perf --url http://127.0.0.1:3000 --device desktop --receipt-mode dev --json",
            "cost": "medium",
            "purpose": "Collect dev-server web performance receipt without mixing it with static/build proof."
        }),
        json!({
            "id": "web-perf-static",
            "command": "dx check web-perf --url http://127.0.0.1:3000 --device desktop --receipt-mode static-build --json",
            "cost": "medium",
            "purpose": "Collect static/build web performance receipt after serving .dx/www/output."
        }),
        json!({
            "id": "compile-cli",
            "command": "cargo check -p dx-www --no-default-features --features cli --bin dx-www -j1",
            "cost": "heavy",
            "purpose": "Compile-proof the CLI without overloading low-end machines."
        }),
    ]
}

fn next_safe_actions(active_blockers: &[Value]) -> Vec<String> {
    let mut actions = vec![
        "Keep app/ as the public authoring truth; do not revive pages/runtime-pages as official surfaces.".to_string(),
        "Run dx style build/check and dx icons sync/check after TSX edits.".to_string(),
        "Use split web-perf receipts so dev-mode scores never masquerade as static/build release proof.".to_string(),
    ];
    if !active_blockers.is_empty() {
        actions.push(
            "Resolve high-severity active_blockers before claiming release readiness.".to_string(),
        );
    }
    actions
}

fn read_json(path: &Path) -> Option<Value> {
    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn check_score_value(value: &Value) -> Option<u64> {
    value
        .pointer("/view_model/score_value")
        .or_else(|| value.pointer("/zed/score_value"))
        .or_else(|| value.get("score"))
        .and_then(Value::as_u64)
}

fn check_score_max(value: &Value) -> Option<u64> {
    value
        .pointer("/view_model/score_max")
        .or_else(|| value.pointer("/zed/score_max"))
        .or_else(|| value.get("max_score"))
        .and_then(Value::as_u64)
}

fn run_git(cwd: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_context_json_contains_required_sections() {
        let temp = tempfile::tempdir().expect("temp dir");
        let report = build_agent_context_report(temp.path(), false);

        assert_eq!(report["schema"], "dx.www.agent_context");
        assert!(report.get("workspace").is_some());
        assert!(report.get("allowed_checks").is_some());
        assert!(report.get("active_blockers").is_some());
        assert!(report.get("receipt_paths").is_some());
        assert!(report.get("readiness_contracts").is_some());
        assert!(report.get("next_safe_actions").is_some());
        assert!(report.get("readiness").is_some());
        assert_eq!(report["release_ready"], true);
        assert_eq!(report["relative_release_ready"], true);
        assert_eq!(report["readiness_gate_status"]["release_ready"], true);
        assert_eq!(
            report["readiness_gate_status"]["fastest_world_claim"],
            false
        );
        assert!(
            report["readiness_replay_commands"]
                .as_array()
                .expect("release readiness replay commands")
                .iter()
                .any(|command| command == "dx check --latest-receipt --json")
        );
    }

    #[test]
    fn template_check_score_floor_follows_receipt_scale() {
        let temp = tempfile::tempdir().expect("temp dir");
        let receipt_path = temp
            .path()
            .join("examples/template/.dx/receipts/check/check-latest.json");
        std::fs::create_dir_all(receipt_path.parent().expect("parent")).expect("receipt dir");
        std::fs::write(
            &receipt_path,
            r#"{"score":98,"max_score":100,"zed":{"score_value":98,"score_max":100}}"#,
        )
        .expect("receipt");

        let receipt_paths = receipt_path_statuses(temp.path());
        let readiness_contracts = readiness_contract_statuses(temp.path(), &receipt_paths);
        let blockers = active_blockers(temp.path(), &receipt_paths, &readiness_contracts);

        assert!(
            !blockers
                .iter()
                .any(|blocker| blocker["id"] == "template-check-score-below-launch-bar")
        );
        assert!(blockers.iter().any(|blocker| {
            blocker["id"] == "template-check-readiness-gate-stale" && blocker["severity"] == "high"
        }));
    }

    #[test]
    fn missing_browser_receipt_gates_expose_actionable_replay_metadata() {
        let temp = tempfile::tempdir().expect("temp dir");
        let receipt_paths = receipt_path_statuses(temp.path());
        let readiness_contracts = readiness_contract_statuses(temp.path(), &receipt_paths);
        let blockers = active_blockers(temp.path(), &receipt_paths, &readiness_contracts);

        for (receipt_path, status_key, import_command, stale_reason) in [
            (
                DEVTOOLS_VISUAL_EDIT_RECEIPT,
                "devtools_visual_edit_receipt_status",
                "dx www readiness --import-visual-edit-browser-receipt <browser-receipt.json> --json --full",
                "visual-edit-browser-workbench-receipt-missing",
            ),
            (
                NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
                "native_event_browser_binder_receipt_status",
                "dx www readiness --import-native-event-browser-binder-receipt <browser-receipt.json> --json --full",
                "native-event-browser-binder-receipt-missing",
            ),
            (
                STATE_RUNTIME_BROWSER_RECEIPT,
                "state_runtime_browser_receipt_status",
                "dx www readiness --import-state-runtime-browser-receipt <browser-receipt.json> --json --full",
                "state-runtime-browser-receipt-missing",
            ),
            (
                NO_JS_BROWSER_RECEIPT,
                "no_js_browser_receipt_status",
                "dx www readiness --import-no-js-browser-receipt <browser-receipt.json> --json --full",
                "no-js-browser-receipt-missing",
            ),
            (
                ISLAND_BROWSER_RECEIPT,
                "island_browser_receipt_status",
                "dx www readiness --import-island-browser-receipt <browser-receipt.json> --json --full",
                "island-browser-receipt-missing",
            ),
        ] {
            let receipt = receipt_paths
                .iter()
                .find(|receipt| receipt["path"] == receipt_path)
                .expect("browser receipt path tracked");
            let status = &receipt[status_key];
            assert_eq!(status["current"], false);
            assert_eq!(status["import_command"], import_command);
            assert_eq!(
                status["harness_snapshot_command"],
                "node benchmarks/dx-www-readiness-browser-receipt-harness.ts --print-page-collector"
            );
            assert_eq!(
                status["harness_import_command"],
                "node benchmarks/dx-www-readiness-browser-receipt-harness.ts --from-page-json <page-snapshot.json> --out-dir .dx/receipts/readiness/browser-import-candidates"
            );
            if status_key == "state_runtime_browser_receipt_status" {
                assert_eq!(
                    status["canonical_browser_proof_target"]["route"],
                    "/state-runtime"
                );
                assert_eq!(
                    status["canonical_browser_proof_target"]["candidate_receipt"],
                    ".dx/receipts/readiness/browser-import-candidates/state-runtime-browser-latest.json"
                );
                assert_eq!(
                    status["canonical_browser_proof_target"]["snapshot_claims_proof"],
                    false
                );
            }
            if status_key == "island_browser_receipt_status" {
                assert_eq!(
                    status["canonical_browser_proof_target"]["route"],
                    "/islands"
                );
                assert_eq!(
                    status["canonical_browser_proof_target"]["candidate_receipt"],
                    ".dx/receipts/readiness/browser-import-candidates/island-browser-latest.json"
                );
                assert_eq!(
                    status["canonical_browser_proof_target"]["snapshot_claims_proof"],
                    false
                );
            }
            assert!(
                status["stale_reasons"]
                    .as_array()
                    .expect("stale reasons")
                    .iter()
                    .any(|reason| reason == stale_reason),
                "{status_key} should expose the missing browser proof reason"
            );
        }

        for (blocker_id, import_command, stale_reason) in [
            (
                "devtools-visual-edit-receipt-missing",
                "dx www readiness --import-visual-edit-browser-receipt <browser-receipt.json> --json --full",
                "visual-edit-browser-workbench-receipt-missing",
            ),
            (
                "native-event-browser-binder-receipt-missing",
                "dx www readiness --import-native-event-browser-binder-receipt <browser-receipt.json> --json --full",
                "native-event-browser-binder-receipt-missing",
            ),
            (
                "state-runtime-browser-receipt-missing",
                "dx www readiness --import-state-runtime-browser-receipt <browser-receipt.json> --json --full",
                "state-runtime-browser-receipt-missing",
            ),
            (
                "no-js-browser-receipt-missing",
                "dx www readiness --import-no-js-browser-receipt <browser-receipt.json> --json --full",
                "no-js-browser-receipt-missing",
            ),
            (
                "island-browser-receipt-missing",
                "dx www readiness --import-island-browser-receipt <browser-receipt.json> --json --full",
                "island-browser-receipt-missing",
            ),
        ] {
            let blocker = blockers
                .iter()
                .find(|blocker| blocker["id"] == blocker_id)
                .expect("missing browser receipt blocker");
            assert_eq!(blocker["import_command"], import_command);
            assert_eq!(blocker["replay_command"], "dx www readiness --json --full");
            assert_eq!(blocker["stale_reasons"][0], stale_reason);
            assert_eq!(
                blocker["browser_receipt_gate"]["canonical_browser_proof_target"]["snapshot_claims_proof"],
                false
            );
        }
    }
}
